use crate::config::{WorkspaceConfig, load_workspace_config};
use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand, ValueEnum};
use serde::{Serialize, Serializer};
use spec_core::export::build_export_bundle;
use spec_core::generator::{
    GenerateOptions, clean_output_dir, generate_code_with_options, generate_mod_rs,
    safe_output_path, write_generated_file,
};
use spec_core::loader::{is_unit_spec, load_directory_report, load_file};
use spec_core::normalizer::normalize_spec;
use spec_core::passport::{
    ArtifactProvenance, PassportEvidence, PassportTestResult, build_passport_with_evidence,
    compute_contract_hash, ensure_gitignore_entry, read_passport, rfc3339_now, write_passport,
};
use spec_core::pipeline::{
    ParsedCargoTestResult, Verbosity, cargo_available, parse_cargo_test_output, run_cargo_build,
    run_cargo_test, workspace_root_for, zero_tests_ran,
};
use spec_core::types::{LoadedSpec, ResolvedSpec};
use spec_core::validator::{
    ValidationOptions, check_spec_versions, validate_deps_exist_with_options,
    validate_full_with_options, validate_no_duplicate_ids,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type CollectedSpecs = (
    Vec<LoadedSpec>,
    Vec<spec_core::SpecError>,
    Vec<spec_core::SpecWarning>,
    usize,
);
type DiagnosticMap = BTreeMap<String, Vec<String>>;

const JSON_SCHEMA_VERSION: u8 = 2;
const CONCURRENT_PASSPORT_WRITER_TTL_SECS: u64 = 300;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Serialize)]
struct JsonValidateResponse {
    schema_version: u8,
    status: &'static str,
    errors: Vec<JsonErrorEntry>,
    warnings: Vec<String>,
}

#[derive(Serialize)]
struct JsonStatusResponse {
    schema_version: u8,
    units: Vec<JsonStatusUnit>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    loader_errors: Vec<JsonErrorEntry>,
}

#[derive(Serialize)]
struct JsonStatusUnit {
    id: String,
    status: HealthState,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    errors: Vec<JsonErrorEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_at: Option<String>,
}

#[derive(Serialize)]
struct JsonErrorEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dep: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cycle: Option<Vec<String>>,
}

#[derive(Default)]
struct ErrorFields {
    unit: Option<String>,
    path: Option<String>,
    dep: Option<String>,
    field: Option<String>,
    value: Option<String>,
    message: Option<String>,
    id: Option<String>,
    path2: Option<String>,
    cycle: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HealthState {
    Invalid,
    Failing,
    Stale,
    Incomplete,
    Untested,
    Valid,
}

impl HealthState {
    fn as_str(self) -> &'static str {
        match self {
            HealthState::Invalid => "invalid",
            HealthState::Failing => "failing",
            HealthState::Stale => "stale",
            HealthState::Incomplete => "incomplete",
            HealthState::Untested => "untested",
            HealthState::Valid => "valid",
        }
    }

    fn is_valid(self) -> bool {
        matches!(self, HealthState::Valid)
    }

    fn symbol(self) -> &'static str {
        match self {
            HealthState::Valid => "✓",
            HealthState::Untested => "—",
            HealthState::Incomplete => "?",
            HealthState::Stale => "~",
            HealthState::Failing | HealthState::Invalid => "✗",
        }
    }
}

impl Serialize for HealthState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

struct PassportWritePlan<'a> {
    passport_root: &'a Path,
    specs: &'a [LoadedSpec],
}

struct ConcurrentPassportWriteGuard {
    marker_path: Option<PathBuf>,
}

impl ConcurrentPassportWriteGuard {
    fn begin(passport_root: &Path) -> Self {
        match Self::begin_in(
            passport_root,
            &std::env::temp_dir(),
            std::process::id(),
            SystemTime::now(),
        ) {
            Ok((guard, other_writers)) => {
                if let Some(warning) =
                    concurrent_passport_write_warning_message(passport_root, other_writers)
                {
                    eprintln!("{warning}");
                }
                guard
            }
            Err(_) => Self { marker_path: None },
        }
    }

    fn begin_in(
        passport_root: &Path,
        registry_base: &Path,
        pid: u32,
        now: SystemTime,
    ) -> Result<(Self, usize)> {
        let registry_dir = concurrent_passport_writer_registry_dir(passport_root, registry_base);
        fs::create_dir_all(&registry_dir)
            .with_context(|| format!("Failed to create {}", registry_dir.display()))?;

        let marker_path = registry_dir.join(concurrent_passport_writer_marker_name(pid, now));
        fs::write(&marker_path, "")
            .with_context(|| format!("Failed to write {}", marker_path.display()))?;

        let other_writers = count_other_active_passport_writers(&registry_dir, pid, now)?;
        Ok((
            Self {
                marker_path: Some(marker_path),
            },
            other_writers,
        ))
    }
}

impl Drop for ConcurrentPassportWriteGuard {
    fn drop(&mut self) {
        let Some(marker_path) = self.marker_path.take() else {
            return;
        };

        let _ = fs::remove_file(&marker_path);
        if let Some(parent) = marker_path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Validate .unit.spec files")]
    Validate(ValidateArgs),
    #[command(about = "Show per-unit validation, passport, and staleness status")]
    Status(StatusArgs),
    #[command(about = "Generate Rust source files from .unit.spec files")]
    Generate(GenerateArgs),
    #[command(about = "Validate, generate, and run cargo build")]
    Build(BuildArgs),
    #[command(about = "Validate, generate, run cargo build and cargo test")]
    Test(TestArgs),
    #[command(about = "Export spec metadata as a JSON bundle")]
    Export(ExportArgs),
    #[command(about = "Print shell completion script to stdout")]
    Completions(CompletionsArgs),
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Validate(args) => validate_command(&args.path, args.no_strict, args.format),
            Self::Status(args) => status_command(&args.path, args.format),
            Self::Generate(args) => generate_command(&args.path, args.output.as_deref()),
            Self::Build(args) => {
                let config = load_workspace_config(&args.path)?;
                build_command(
                    &args.path,
                    args.output.as_deref(),
                    args.crate_root.as_deref(),
                    &config,
                )
            }
            Self::Test(args) => {
                let config = load_workspace_config(&args.path)?;
                test_command(
                    &args.path,
                    args.output.as_deref(),
                    args.crate_root.as_deref(),
                    &config,
                )
            }
            Self::Export(args) => export_command(&args.path, args.output.as_deref()),
            Self::Completions(_) => unreachable!("handled in main"),
        }
    }
}

#[derive(Args, Debug)]
pub struct ValidateArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Downgrade missing-dep errors to warnings and exit 0 (validation only)"
    )]
    pub no_strict: bool,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Output directory for generated Rust files (default: {crate_root}/src/generated)"
    )]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Output directory for generated Rust files (default: {crate_root}/src/generated)"
    )]
    pub output: Option<PathBuf>,
    #[arg(
        long,
        help = "Path to the Cargo project root (overrides spec.toml and ancestor walk)"
    )]
    pub crate_root: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Output directory for generated Rust files (default: {crate_root}/src/generated)"
    )]
    pub output: Option<PathBuf>,
    #[arg(
        long,
        help = "Path to the Cargo project root (overrides spec.toml and ancestor walk)"
    )]
    pub crate_root: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(long, help = "Write JSON bundle to FILE instead of stdout")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: clap_complete::Shell,
}

fn validate_command(path: &Path, no_strict: bool, format: OutputFormat) -> Result<()> {
    let (specs, loader_errors, loader_warnings, total_files) = collect_specs(path)?;
    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: !no_strict,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (validation_errors, validation_warnings) = finish_validation(&specs, &validation_options);

    match format {
        OutputFormat::Text => {
            let mut errors = DiagnosticMap::new();
            let mut warnings = DiagnosticMap::new();

            for err in loader_errors {
                push_error(&mut errors, err);
            }
            for err in validation_errors {
                push_error(&mut errors, err);
            }
            for warning in loader_warnings {
                push_warning(&mut warnings, warning);
            }
            for warning in validation_warnings {
                push_warning(&mut warnings, warning);
            }

            let warning_count = count_messages(&warnings);

            if !warnings.is_empty() {
                print_diagnostics(&warnings);
            }

            if errors.is_empty() {
                if total_files == 0 {
                    println!("0 units found, nothing to validate.");
                } else {
                    println!(
                        "✅ {total_files} unit{} valid{}",
                        pluralize(total_files),
                        if warning_count == 0 {
                            String::new()
                        } else {
                            format!(" with {warning_count} warning{}", pluralize(warning_count))
                        }
                    );
                }
                return Ok(());
            }

            print_diagnostics(&errors);
            let file_count = count_unique_files(&errors);
            bail!(
                "❌ {} file{}, {} error{}",
                file_count,
                pluralize(file_count),
                count_messages(&errors),
                pluralize(count_messages(&errors))
            );
        }
        OutputFormat::Json => {
            let id_by_path: HashMap<String, String> = specs
                .iter()
                .map(|s| (s.source.file_path.clone(), s.spec.id.clone()))
                .collect();
            let mut errors = Vec::with_capacity(loader_errors.len() + validation_errors.len());
            errors.extend(
                loader_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );
            errors.extend(
                validation_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );

            let warnings = loader_warnings
                .into_iter()
                .chain(validation_warnings)
                .map(|warning| warning.to_string())
                .collect();
            let has_errors = !errors.is_empty();

            let response = JsonValidateResponse {
                schema_version: JSON_SCHEMA_VERSION,
                status: if has_errors { "invalid" } else { "valid" },
                errors,
                warnings,
            };
            let json = serde_json::to_string_pretty(&response)?;
            print!("{json}");
            std::io::stdout().flush()?;

            if has_errors {
                std::process::exit(1);
            } else {
                Ok(())
            }
        }
    }
}

struct HealthStatus {
    status: HealthState,
    reason: Option<String>,
    evidence_at: Option<String>,
}

fn compute_health_status(
    errors: &[JsonErrorEntry],
    passport: Option<&spec_core::passport::Passport>,
    live_hash: Option<&str>,
) -> HealthStatus {
    // 1. invalid
    if !errors.is_empty() {
        return HealthStatus { status: HealthState::Invalid, reason: None, evidence_at: None };
    }

    let evidence = passport.and_then(|p| p.evidence.as_ref());
    let evidence_at = evidence.map(|e| e.observed_at.clone());

    // 2. failing — build failure or any test fail (requires evidence; failing beats stale)
    if let Some(ev) = evidence {
        let build_failed = ev.build_status != "pass";
        let any_test_failed = ev.test_results.iter().any(|r| r.status == "fail");
        if build_failed || any_test_failed {
            let reason = if build_failed {
                match ev.build_status.as_str() {
                    "timeout" => "build timed out".to_string(),
                    _ => "build failed".to_string(),
                }
            } else {
                let n = ev.test_results.iter().filter(|r| r.status == "fail").count();
                format!("{} test{} failed", n, pluralize(n))
            };
            return HealthStatus {
                status: HealthState::Failing,
                reason: Some(reason),
                evidence_at,
            };
        }
    }

    // 3. stale — contract hash changed since last test, including added/removed contracts.
    //    Only fires when a passport exists (i.e. the unit has been tested before).
    //    Without a passport there is nothing to compare against — falls through to untested.
    let stored_hash = passport.and_then(|p| p.contract_hash.as_deref());
    if passport.is_some() {
        let hash_changed = match (stored_hash, live_hash) {
            (Some(stored), Some(live)) => stored != live,
            (None, Some(_)) | (Some(_), None) => true, // contract added or removed since last test
            (None, None) => false,
        };
        if hash_changed {
            return HealthStatus {
                status: HealthState::Stale,
                reason: Some("contract changed since last test".to_string()),
                evidence_at,
            };
        }
    }

    // 4. incomplete — unobserved tests (requires evidence)
    if let Some(ev) = evidence {
        let unknown_count = ev.test_results.iter().filter(|r| r.status == "unknown").count();
        if unknown_count > 0 {
            return HealthStatus {
                status: HealthState::Incomplete,
                reason: Some(format!(
                    "{} test{} not observed in cargo output",
                    unknown_count,
                    pluralize(unknown_count)
                )),
                evidence_at,
            };
        }
    }

    // 5. untested — no passport or no evidence
    if evidence.is_none() {
        return HealthStatus {
            status: HealthState::Untested,
            reason: Some("no evidence".to_string()),
            evidence_at: None,
        };
    }

    // 6. valid
    HealthStatus { status: HealthState::Valid, reason: None, evidence_at }
}

fn status_command(path: &Path, format: OutputFormat) -> Result<()> {
    let (specs, loader_errors, _loader_warnings, total_files) = collect_specs(path)?;
    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (validation_errors, _validation_warnings) = finish_validation(&specs, &validation_options);
    let id_by_path: HashMap<String, String> = specs
        .iter()
        .map(|s| (s.source.file_path.clone(), s.spec.id.clone()))
        .collect();
    let has_loader_errors = !loader_errors.is_empty();

    // Convert loader errors to JSON entries (non-consuming borrow) so they can be
    // surfaced in JSON mode where print_diagnostics is not called.
    let loader_error_entries: Vec<JsonErrorEntry> = loader_errors
        .iter()
        .map(|err| spec_error_to_json_entry(err, &id_by_path))
        .collect();

    let mut errors_by_path: HashMap<String, Vec<JsonErrorEntry>> = HashMap::new();
    for err in &validation_errors {
        for path in error_paths(err) {
            errors_by_path
                .entry(path)
                .or_default()
                .push(spec_error_to_json_entry(err, &id_by_path));
        }
    }

    // Text mode emits loader errors as human-readable diagnostics;
    // JSON mode surfaces them in the response's loader_errors field.
    if has_loader_errors && matches!(format, OutputFormat::Text) {
        let mut diagnostics = DiagnosticMap::new();
        for err in loader_errors {
            push_error(&mut diagnostics, err);
        }
        print_diagnostics(&diagnostics);
    }

    if total_files == 0 && specs.is_empty() && !has_loader_errors {
        match format {
            OutputFormat::Text => {
                println!("0 units found, nothing to status.");
            }
            OutputFormat::Json => {
                let response = JsonStatusResponse {
                    schema_version: JSON_SCHEMA_VERSION,
                    units: vec![],
                    loader_errors: vec![],
                };
                let json = serde_json::to_string_pretty(&response)?;
                print!("{json}");
                std::io::stdout().flush()?;
            }
        }
        return Ok(());
    }

    let mut units = Vec::with_capacity(specs.len());
    let mut needs_nonzero_exit = has_loader_errors;

    for spec in &specs {
        let source_path = Path::new(&spec.source.file_path);
        let passport = match read_passport(source_path) {
            Ok(passport) => passport,
            Err(err) => {
                if matches!(format, OutputFormat::Text) {
                    eprintln!(
                        "⚠ failed to read passport for {}: {err}",
                        source_path.display()
                    );
                }
                None
            }
        };
        let live_hash = compute_contract_hash(spec);
        let errors = errors_by_path
            .remove(&spec.source.file_path)
            .unwrap_or_default();
        let health = compute_health_status(&errors, passport.as_ref(), live_hash.as_deref());

        if !health.status.is_valid() {
            needs_nonzero_exit = true;
        }

        units.push(JsonStatusUnit {
            id: spec.spec.id.clone(),
            status: health.status,
            reason: health.reason,
            errors,
            evidence_at: health.evidence_at,
        });
    }

    match format {
        OutputFormat::Text => {
            for unit in &units {
                print_status_unit(unit);
            }
        }
        OutputFormat::Json => {
            let response = JsonStatusResponse {
                schema_version: JSON_SCHEMA_VERSION,
                units,
                loader_errors: loader_error_entries,
            };
            let json = serde_json::to_string_pretty(&response)?;
            print!("{json}");
            std::io::stdout().flush()?;
        }
    }

    if needs_nonzero_exit {
        std::process::exit(1);
    }

    Ok(())
}

fn export_command(path: &Path, output: Option<&Path>) -> Result<()> {
    let (specs, loader_errors, loader_warnings, _total_files) = collect_specs(path)?;
    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (validation_errors, validation_warnings) = finish_validation(&specs, &validation_options);
    let mut errors = DiagnosticMap::new();
    let mut warnings = DiagnosticMap::new();
    for err in loader_errors {
        push_error(&mut errors, err);
    }
    for err in validation_errors {
        push_error(&mut errors, err);
    }
    for warning in loader_warnings {
        push_warning(&mut warnings, warning);
    }
    for warning in validation_warnings {
        push_warning(&mut warnings, warning);
    }

    if !warnings.is_empty() {
        print_diagnostics(&warnings);
    }
    if !errors.is_empty() {
        print_diagnostics(&errors);
        let file_count = count_unique_files(&errors);
        bail!(
            "❌ {} file{}, {} error{}",
            file_count,
            pluralize(file_count),
            count_messages(&errors),
            pluralize(count_messages(&errors))
        );
    }

    let provenance = resolve_git_provenance(if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    });
    let bundle = build_export_bundle(&specs, &rfc3339_now(), provenance.as_ref());
    let json = serde_json::to_string_pretty(&bundle)?;

    match output {
        Some(path) => {
            validate_export_output_path(path)?;
            fs::write(path, json)
                .with_context(|| format!("Failed to write export bundle to {}", path.display()))?;
        }
        None => {
            print!("{json}");
        }
    }

    Ok(())
}

fn generate_command(path: &Path, output: Option<&Path>) -> Result<()> {
    let spec_root = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    let resolved_output: PathBuf = match output {
        Some(p) => p.to_path_buf(),
        None => {
            // Use the same crate-root precedence as build/test:
            // spec.toml [pipeline] crate_root → ancestor Cargo.toml walk.
            let config = load_workspace_config(path)?;
            let crate_root = match config.pipeline.crate_root.as_deref() {
                Some(p) => p.to_path_buf(),
                None => workspace_root_for(spec_root)?,
            };
            crate_root.join("src/generated")
        }
    };
    let generated = generate_specs(path, &resolved_output)?;
    if !generated.specs.is_empty() {
        finalize_passports(
            spec_root,
            &generated.specs,
            &generated.generated_at,
            None,
            None,
        )?;
    }
    Ok(())
}

fn generate_specs(path: &Path, output: &Path) -> Result<GeneratedSpecs> {
    let (specs, loader_errors, loader_warnings, total_files) = collect_specs(path)?;
    if total_files == 0 {
        let mut warnings = DiagnosticMap::new();
        for warning in loader_warnings {
            push_warning(&mut warnings, warning);
        }
        if !warnings.is_empty() {
            print_diagnostics(&warnings);
        }
        println!("0 units found, nothing to generate.");
        return Ok(GeneratedSpecs {
            specs,
            generated_at: rfc3339_now(),
        });
    }

    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (validation_errors, validation_warnings) = finish_validation(&specs, &validation_options);
    let mut errors = DiagnosticMap::new();
    let mut warnings = DiagnosticMap::new();
    for err in loader_errors {
        push_error(&mut errors, err);
    }
    for err in validation_errors {
        push_error(&mut errors, err);
    }
    for warning in loader_warnings {
        push_warning(&mut warnings, warning);
    }
    for warning in validation_warnings {
        push_warning(&mut warnings, warning);
    }
    if !warnings.is_empty() {
        print_diagnostics(&warnings);
    }
    if !errors.is_empty() {
        print_diagnostics(&errors);
        let file_count = count_unique_files(&errors);
        bail!(
            "❌ {} file{}, {} error{}",
            file_count,
            pluralize(file_count),
            count_messages(&errors),
            pluralize(count_messages(&errors))
        );
    }

    let mut resolved_specs = Vec::new();
    for spec in &specs {
        resolved_specs.push(
            normalize_spec(spec.spec.clone())
                .with_context(|| format!("Failed to normalize {}", spec.source.file_path))?,
        );
    }

    let mut generated_rs_rel_paths = HashSet::<PathBuf>::new();
    for spec in &resolved_specs {
        generated_rs_rel_paths.insert(path_for_spec(spec));
    }

    // Include every generated mod.rs (root + nested modules) in the owned set.
    let namespaces = build_namespaces(&resolved_specs);
    for module_path in namespaces.keys() {
        let mod_rs_rel = if module_path.is_empty() {
            PathBuf::from("mod.rs")
        } else {
            PathBuf::from(module_path.replace('/', std::path::MAIN_SEPARATOR_STR)).join("mod.rs")
        };
        generated_rs_rel_paths.insert(mod_rs_rel);
    }

    let output_base = ensure_output_marker(output)?;
    let generate_options = GenerateOptions {
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };

    for spec in &resolved_specs {
        let content = generate_code_with_options(spec, &generate_options)
            .with_context(|| format!("Failed to generate Rust for {}", spec.id))?;
        let output_path = output_base.join(path_for_spec(spec));
        write_generated_file(&output_path.display().to_string(), &content)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;
    }

    for (module_path, namespace) in &namespaces {
        let content = generate_mod_rs(
            &namespace.unit_files.iter().cloned().collect::<Vec<_>>(),
            &namespace.subdirs.iter().cloned().collect::<Vec<_>>(),
        )
        .with_context(|| format!("Failed to generate mod.rs for module '{module_path}'"))?;

        let mod_rs_rel = if module_path.is_empty() {
            PathBuf::from("mod.rs")
        } else {
            PathBuf::from(module_path.replace('/', std::path::MAIN_SEPARATOR_STR)).join("mod.rs")
        };
        let mod_rs_path = output_base.join(mod_rs_rel);

        write_generated_file(&mod_rs_path.display().to_string(), &content)
            .with_context(|| format!("Failed to write {}", mod_rs_path.display()))?;
    }

    clean_output_dir(&output_base, &generated_rs_rel_paths)
        .with_context(|| format!("Failed to clean output directory {}", output_base.display()))?;

    let generated_at = rfc3339_now();

    println!(
        "Generated {} file{}",
        resolved_specs.len() + namespaces.len(),
        pluralize(resolved_specs.len() + namespaces.len())
    );
    Ok(GeneratedSpecs {
        specs,
        generated_at,
    })
}

fn finalize_passports(
    passport_root: &Path,
    specs: &[LoadedSpec],
    generated_at: &str,
    evidence_by_spec: Option<&BTreeMap<String, PassportEvidence>>,
    contract_hash_by_spec: Option<&BTreeMap<String, String>>,
) -> Result<()> {
    if specs.is_empty() {
        return Ok(());
    }

    let _writer_guard = ConcurrentPassportWriteGuard::begin(passport_root);
    write_passports(specs, generated_at, evidence_by_spec, contract_hash_by_spec)?;
    ensure_gitignore_entry(passport_root)
        .with_context(|| "Failed to update .gitignore for passport files")?;
    Ok(())
}

fn contract_hashes_for(specs: &[LoadedSpec]) -> Option<BTreeMap<String, String>> {
    let mut hashes = BTreeMap::new();
    for spec in specs {
        if let Some(hash) = compute_contract_hash(spec) {
            hashes.insert(spec.spec.id.clone(), hash);
        }
    }

    if hashes.is_empty() {
        None
    } else {
        Some(hashes)
    }
}

fn write_passports(
    specs: &[LoadedSpec],
    generated_at: &str,
    evidence_by_spec: Option<&BTreeMap<String, PassportEvidence>>,
    contract_hash_by_spec: Option<&BTreeMap<String, String>>,
) -> Result<()> {
    for spec in specs {
        let source_path = Path::new(&spec.source.file_path);

        let (evidence, contract_hash) = if evidence_by_spec.is_none() {
            // Non-test caller (spec generate / spec build): preserve any evidence
            // and contract_hash already on disk so we don't erase data written by
            // a prior `spec test` run.  If no baseline hash exists yet (fresh
            // project or first generate), compute one from the current contract so
            // that stale detection works before the first `spec test` run.
            let existing = read_passport(source_path).ok().flatten();
            let ev = existing.as_ref().and_then(|p| p.evidence.clone());
            let hash = existing
                .and_then(|p| p.contract_hash)
                .or_else(|| compute_contract_hash(spec));
            (ev, hash)
        } else {
            // Test caller: always use freshly-computed values (None is correct for
            // specs that have no contract).
            let ev = evidence_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned();
            let hash = contract_hash_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned();
            (ev, hash)
        };

        let passport = build_passport_with_evidence(spec, generated_at, evidence, contract_hash);
        write_passport(&passport, source_path)
            .with_context(|| format!("Failed to write passport for {}", spec.source.id))?;
    }

    Ok(())
}

struct PipelineContext {
    crate_root: PathBuf,
    cargo_target_dir: PathBuf,
    timeout: Option<Duration>,
    // Holds the tempdir alive for the duration of the command when we own one.
    _temp_dir: Option<tempfile::TempDir>,
}

struct GeneratedSpecs {
    specs: Vec<LoadedSpec>,
    generated_at: String,
}

fn resolve_pipeline_context(
    path: &Path,
    crate_root_flag: Option<&Path>,
    config: &WorkspaceConfig,
) -> Result<PipelineContext> {
    let crate_root = match crate_root_flag.or(config.pipeline.crate_root.as_deref()) {
        Some(p) => p.to_path_buf(),
        None => workspace_root_for(path)?,
    };

    let mut temp_dir: Option<tempfile::TempDir> = None;
    let cargo_target_dir = if let Some(p) = &config.pipeline.cargo_target_dir {
        p.clone()
    } else if let Ok(env_val) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(env_val)
    } else {
        let td = tempfile::TempDir::new()
            .with_context(|| "Failed to create temporary CARGO_TARGET_DIR")?;
        let path = td.path().to_path_buf();
        temp_dir = Some(td);
        path
    };

    Ok(PipelineContext {
        crate_root,
        cargo_target_dir,
        timeout: config.pipeline.timeout_secs.map(Duration::from_secs),
        _temp_dir: temp_dir,
    })
}

fn build_command(
    path: &Path,
    output: Option<&Path>,
    crate_root_flag: Option<&Path>,
    config: &WorkspaceConfig,
) -> Result<()> {
    if path.is_file() {
        bail!(
            "❌ spec build requires a directory path — pass the units directory, not a single file"
        );
    }

    if !cargo_available() {
        bail!("❌ cargo not found — install Rust or ensure cargo is on PATH");
    }

    let ctx = resolve_pipeline_context(path, crate_root_flag, config)?;
    let resolved_output = output
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.crate_root.join("src/generated"));

    let generated = generate_specs(path, &resolved_output)?;
    if !generated.specs.is_empty() {
        finalize_passports(path, &generated.specs, &generated.generated_at, None, None)?;
    }

    let result = run_cargo_build(
        &ctx.crate_root,
        &ctx.cargo_target_dir,
        ctx.timeout,
        Verbosity::Normal,
    )?;
    print!("{}", result.stdout);
    eprint!("{}", result.stderr);
    if result.timed_out {
        bail!("❌ cargo build timed out{}", timeout_suffix(ctx.timeout));
    }
    if result.exit_code != 0 {
        bail!("❌ cargo build failed");
    }
    Ok(())
}

fn test_command(
    path: &Path,
    output: Option<&Path>,
    crate_root_flag: Option<&Path>,
    config: &WorkspaceConfig,
) -> Result<()> {
    if !cargo_available() {
        bail!("❌ cargo not found — install Rust or ensure cargo is on PATH");
    }

    let (spec_root, target_spec) = if path.is_file() {
        if !is_unit_spec(path) {
            bail!("{} is not a .unit.spec file", path.display());
        }
        (
            path.parent().unwrap_or(path),
            Some(load_file(path).with_context(|| format!("Failed to load {}", path.display()))?),
        )
    } else {
        (path, None)
    };

    let ctx = resolve_pipeline_context(spec_root, crate_root_flag, config)?;
    let resolved_output = output
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.crate_root.join("src/generated"));

    let generated = generate_specs(spec_root, &resolved_output)?;
    if generated.specs.is_empty() {
        return Ok(());
    }

    if target_spec.is_none() {
        finalize_passports(path, &generated.specs, &generated.generated_at, None, None)?;
    }

    let passport_write_plan =
        passport_write_plan(path, spec_root, &generated.specs, target_spec.as_ref());

    // Resolve the module prefix once — used for both the cargo test filter and
    // evidence lookup. A single resolved value ensures they always agree.
    let effective_prefix = match &config.pipeline.generated_module_prefix {
        Some(explicit) => explicit.clone(),
        None => output_module_prefix(&resolved_output, &ctx.crate_root)?,
    };
    let filter = target_spec.as_ref().map(|target| {
        let resolved = ResolvedSpec::from_spec(target.spec.clone());
        cargo_test_filter_for(&resolved, &effective_prefix)
    });

    let provenance = resolve_git_provenance(&ctx.crate_root);
    let build_result = run_cargo_build(
        &ctx.crate_root,
        &ctx.cargo_target_dir,
        ctx.timeout,
        Verbosity::Normal,
    )?;
    print!("{}", build_result.stdout);
    eprint!("{}", build_result.stderr);
    if build_result.timed_out {
        let observed_at = rfc3339_now();
        let evidence_by_spec =
            build_timeout_evidence(passport_write_plan.specs, &observed_at, provenance.as_ref());
        let contract_hash_by_spec = contract_hashes_for(passport_write_plan.specs);
        finalize_test_passports(
            &passport_write_plan,
            &generated.generated_at,
            &evidence_by_spec,
            contract_hash_by_spec.as_ref(),
        )?;
        bail!("❌ cargo build timed out{}", timeout_suffix(ctx.timeout));
    }
    if build_result.exit_code != 0 {
        let observed_at = rfc3339_now();
        let evidence_by_spec =
            build_failure_evidence(passport_write_plan.specs, &observed_at, provenance.as_ref());
        let contract_hash_by_spec = contract_hashes_for(passport_write_plan.specs);
        finalize_test_passports(
            &passport_write_plan,
            &generated.generated_at,
            &evidence_by_spec,
            contract_hash_by_spec.as_ref(),
        )?;
        bail!("❌ cargo build failed");
    }

    let test_result = run_cargo_test(
        &ctx.crate_root,
        &ctx.cargo_target_dir,
        filter.as_deref(),
        ctx.timeout,
        Verbosity::Normal,
    )?;
    print!("{}", test_result.stdout);
    eprint!("{}", test_result.stderr);
    if test_result.timed_out {
        let observed_at = rfc3339_now();
        let evidence_by_spec =
            build_timeout_evidence(passport_write_plan.specs, &observed_at, provenance.as_ref());
        let contract_hash_by_spec = contract_hashes_for(passport_write_plan.specs);
        finalize_test_passports(
            &passport_write_plan,
            &generated.generated_at,
            &evidence_by_spec,
            contract_hash_by_spec.as_ref(),
        )?;
        bail!("❌ cargo test timed out{}", timeout_suffix(ctx.timeout));
    }

    if target_spec.is_some() && zero_tests_ran(&test_result.stdout) {
        bail!("❌ cargo test matched 0 tests");
    }

    let parsed_test_results = parse_cargo_test_output(&test_result.stdout);
    let observed_at = rfc3339_now();
    let evidence_by_spec = build_test_evidence(
        passport_write_plan.specs,
        &effective_prefix,
        &parsed_test_results,
        &observed_at,
        provenance.as_ref(),
    )?;
    let contract_hash_by_spec = contract_hashes_for(passport_write_plan.specs);
    finalize_test_passports(
        &passport_write_plan,
        &generated.generated_at,
        &evidence_by_spec,
        contract_hash_by_spec.as_ref(),
    )?;
    if test_result.exit_code != 0 {
        bail!("❌ cargo test failed");
    }
    Ok(())
}

fn timeout_suffix(timeout: Option<Duration>) -> String {
    match timeout {
        Some(timeout) => format!(" after {}s", timeout.as_secs()),
        None => String::new(),
    }
}

fn build_failure_evidence(
    specs: &[LoadedSpec],
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> BTreeMap<String, PassportEvidence> {
    build_incomplete_evidence(specs, "fail", observed_at, provenance)
}

fn build_timeout_evidence(
    specs: &[LoadedSpec],
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> BTreeMap<String, PassportEvidence> {
    build_incomplete_evidence(specs, "timeout", observed_at, provenance)
}

fn build_incomplete_evidence(
    specs: &[LoadedSpec],
    build_status: &str,
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> BTreeMap<String, PassportEvidence> {
    specs
        .iter()
        .map(|spec| {
            (
                spec.spec.id.clone(),
                PassportEvidence {
                    build_status: build_status.to_string(),
                    test_results: vec![],
                    observed_at: observed_at.to_string(),
                    provenance: provenance.cloned(),
                },
            )
        })
        .collect()
}

fn passport_write_plan<'a>(
    requested_path: &'a Path,
    spec_root: &'a Path,
    generated_specs: &'a [LoadedSpec],
    target_spec: Option<&'a LoadedSpec>,
) -> PassportWritePlan<'a> {
    if let Some(target_spec) = target_spec {
        PassportWritePlan {
            passport_root: spec_root,
            specs: std::slice::from_ref(target_spec),
        }
    } else {
        PassportWritePlan {
            passport_root: requested_path,
            specs: generated_specs,
        }
    }
}

fn finalize_test_passports(
    plan: &PassportWritePlan<'_>,
    generated_at: &str,
    evidence_by_spec: &BTreeMap<String, PassportEvidence>,
    contract_hash_by_spec: Option<&BTreeMap<String, String>>,
) -> Result<()> {
    finalize_passports(
        plan.passport_root,
        plan.specs,
        generated_at,
        Some(evidence_by_spec),
        contract_hash_by_spec,
    )
}

fn concurrent_passport_writer_registry_dir(passport_root: &Path, registry_base: &Path) -> PathBuf {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // DefaultHasher::new() uses a fixed zero seed (not per-process randomized), so two
    // concurrent processes produce the same hash for the same path within a single Rust
    // version. The algorithm is documented as unstable across Rust versions — if two spec
    // binaries compiled with different Rust versions run concurrently, they may hash to
    // different registry dirs and miss each other. For a best-effort warn-only feature
    // this is acceptable; use a stable hasher here if the guarantee ever matters.
    let canonical_root = passport_root
        .canonicalize()
        .unwrap_or_else(|_| passport_root.to_path_buf());
    let mut hasher = DefaultHasher::new();
    canonical_root.hash(&mut hasher);
    let hash = hasher.finish();
    registry_base.join(format!("spec-passport-writers-{hash:016x}"))
}

fn concurrent_passport_writer_marker_name(pid: u32, now: SystemTime) -> String {
    let now_secs = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    format!("{pid}-{now_secs}.active")
}

fn parse_concurrent_passport_writer_marker(file_name: &str) -> Option<(u32, u64)> {
    let file_name = file_name.strip_suffix(".active")?;
    let (pid, started_at) = file_name.split_once('-')?;
    Some((pid.parse().ok()?, started_at.parse().ok()?))
}

fn count_other_active_passport_writers(
    registry_dir: &Path,
    current_pid: u32,
    now: SystemTime,
) -> Result<usize> {
    let now_secs = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let mut other_writers = HashSet::new();

    for entry in fs::read_dir(registry_dir)
        .with_context(|| format!("Failed to read {}", registry_dir.display()))?
    {
        let entry = entry?;
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        let Some((pid, started_at)) = parse_concurrent_passport_writer_marker(file_name) else {
            continue;
        };

        if now_secs.saturating_sub(started_at) > CONCURRENT_PASSPORT_WRITER_TTL_SECS {
            let _ = fs::remove_file(entry.path());
            continue;
        }

        if pid != current_pid {
            other_writers.insert(pid);
        }
    }

    Ok(other_writers.len())
}

fn concurrent_passport_write_warning_message(
    passport_root: &Path,
    other_writers: usize,
) -> Option<String> {
    if other_writers == 0 {
        return None;
    }

    Some(format!(
        "⚠ detected {other_writers} other spec process{} writing passports under {}; concurrent passport writes are best-effort only (no locking)",
        pluralize(other_writers),
        passport_root.display()
    ))
}

fn build_test_evidence(
    specs: &[LoadedSpec],
    output_prefix: &str,
    parsed_test_results: &HashMap<String, ParsedCargoTestResult>,
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> Result<BTreeMap<String, PassportEvidence>> {
    let mut evidence_by_spec = BTreeMap::new();

    for spec in specs {
        let resolved = ResolvedSpec::from_spec(spec.spec.clone());
        let mut test_results = Vec::new();

        for local_test in &spec.spec.local_tests {
            let full_name = expected_cargo_test_name(&resolved, output_prefix, &local_test.id);
            // This lookup runs once per local test after parsing cargo stdout, so
            // keep it on a hash-based map for large repos where thousands of test
            // names may be correlated back into passport evidence in one command.
            let observed = parsed_test_results.get(&full_name);
            let (status, reason) = match observed {
                Some(result) => (result.status.clone(), result.reason.clone()),
                None => (
                    "unknown".to_string(),
                    Some("test not found in cargo output".to_string()),
                ),
            };

            test_results.push(PassportTestResult {
                id: local_test.id.clone(),
                status,
                reason,
            });
        }

        evidence_by_spec.insert(
            spec.spec.id.clone(),
            PassportEvidence {
                build_status: "pass".to_string(),
                test_results,
                observed_at: observed_at.to_string(),
                provenance: provenance.cloned(),
            },
        );
    }

    Ok(evidence_by_spec)
}

fn resolve_git_provenance(path: &Path) -> Option<ArtifactProvenance> {
    let sha = resolve_git_commit_sha(path)?;
    Some(ArtifactProvenance {
        git_commit_sha: sha,
    })
}

fn resolve_git_commit_sha(path: &Path) -> Option<String> {
    let output = std::process::Command::new("git")
        .current_dir(path)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let sha = String::from_utf8(output.stdout).ok()?;
    let sha = sha.trim();
    if sha.is_empty() {
        None
    } else {
        Some(sha.to_string())
    }
}

fn output_module_prefix(output: &Path, crate_root: &Path) -> Result<String> {
    // Strip `{crate_root}/src/` prefix from the output path so that
    // `{crate_root}/src/generated` → `"generated"` (not `"generated::spec"`).
    // Falls back to stripping a leading `"src"` component for relative paths
    // (test helpers, explicit `--output src/generated`).
    let src_root = crate_root.join("src");
    let relative = output.strip_prefix(&src_root).unwrap_or_else(|_| {
        // Relative path fallback: strip a leading "src" component if present.
        let mut comps = output.components();
        if comps
            .next()
            .map(|c| c.as_os_str() == "src")
            .unwrap_or(false)
        {
            output.strip_prefix("src").unwrap_or(output)
        } else {
            output
        }
    });

    let parts: Vec<&str> = relative
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .collect();

    if parts.is_empty() {
        return Err(anyhow::anyhow!(
            "❌ could not determine output module prefix from {}",
            output.display()
        ));
    }

    Ok(parts.join("::"))
}

fn cargo_test_filter_for(spec: &ResolvedSpec, output_prefix: &str) -> String {
    if spec.module_path.is_empty() {
        format!("{output_prefix}::{}::tests::", spec.fn_name)
    } else {
        format!(
            "{output_prefix}::{}::{}::tests::",
            spec.module_path.replace('/', "::"),
            spec.fn_name
        )
    }
}

fn expected_cargo_test_name(spec: &ResolvedSpec, output_prefix: &str, test_id: &str) -> String {
    if spec.module_path.is_empty() {
        format!("{output_prefix}::{}::tests::test_{test_id}", spec.fn_name)
    } else {
        format!(
            "{output_prefix}::{}::{}::tests::test_{test_id}",
            spec.module_path.replace('/', "::"),
            spec.fn_name
        )
    }
}

#[derive(Default)]
struct Namespace {
    unit_files: BTreeSet<String>,
    subdirs: BTreeSet<String>,
}

fn build_namespaces(specs: &[ResolvedSpec]) -> BTreeMap<String, Namespace> {
    let mut namespaces = BTreeMap::<String, Namespace>::new();
    namespaces.entry(String::new()).or_default();

    for spec in specs {
        namespaces
            .entry(spec.module_path.clone())
            .or_default()
            .unit_files
            .insert(spec.fn_name.clone());

        let segments: Vec<&str> = spec.id.split('/').collect();
        let module_segments = &segments[..segments.len() - 1];

        for depth in 0..module_segments.len() {
            let parent = if depth == 0 {
                String::new()
            } else {
                module_segments[..depth].join("/")
            };
            namespaces
                .entry(parent)
                .or_default()
                .subdirs
                .insert(module_segments[depth].to_string());
        }
    }

    namespaces
}

fn path_for_spec(spec: &ResolvedSpec) -> PathBuf {
    let mut path = PathBuf::new();
    if !spec.module_path.is_empty() {
        path.push(spec.module_path.replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    path.push(format!("{}.rs", spec.fn_name));
    path
}

fn ensure_output_marker(output: &Path) -> Result<PathBuf> {
    let output_base = safe_output_path(output)?;

    if output_base.exists() && !output_base.is_dir() {
        bail!(
            "Refusing to generate into {}: output path exists and is not a directory",
            output_base.display()
        );
    }

    let marker = output_base.join(".spec-generated");
    if !marker.exists() && output_base.exists() && !dir_is_empty(&output_base)? {
        bail!(
            "Refusing to generate into {}: non-empty directory missing .spec-generated marker",
            output_base.display()
        );
    }

    if !output_base.exists() {
        fs::create_dir_all(&output_base).with_context(|| {
            format!(
                "Failed to create output directory {}",
                output_base.display()
            )
        })?;
    }

    if !marker.exists() {
        fs::write(&marker, "")
            .with_context(|| format!("Failed to create marker {}", marker.display()))?;
    }

    Ok(output_base)
}

fn validate_export_output_path(output: &Path) -> Result<()> {
    if output.is_dir() {
        bail!("❌ --output must be a file path, not a directory");
    }

    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty())
        && !parent.exists()
    {
        bail!("❌ output directory does not exist: {}", parent.display());
    }

    Ok(())
}

fn dir_is_empty(path: &Path) -> Result<bool> {
    let mut entries =
        fs::read_dir(path).with_context(|| format!("Failed to read dir {}", path.display()))?;
    Ok(entries.next().is_none())
}

fn collect_specs(path: &Path) -> Result<CollectedSpecs> {
    if path.is_file() {
        let total_files = usize::from(is_unit_spec(path));
        if !is_unit_spec(path) {
            bail!("{} is not a .unit.spec file", path.display());
        }
        return match load_file(path) {
            Ok(spec) => Ok((vec![spec], Vec::new(), Vec::new(), total_files)),
            Err(err) => Ok((Vec::new(), vec![err], Vec::new(), total_files)),
        };
    }

    if !path.is_dir() {
        bail!("{} does not exist", path.display());
    }

    let report = load_directory_report(path);
    Ok((
        report.specs,
        report.errors,
        report.warnings,
        report.total_files,
    ))
}

fn finish_validation(
    specs: &[LoadedSpec],
    options: &ValidationOptions,
) -> (Vec<spec_core::SpecError>, Vec<spec_core::SpecWarning>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    errors.extend(validate_no_duplicate_ids(specs));

    for spec in specs {
        if let Err(err) = validate_full_with_options(spec, options) {
            errors.push(err);
        }
    }

    let (dep_errors, dep_warnings) = validate_deps_exist_with_options(specs, options);
    errors.extend(dep_errors);
    warnings.extend(dep_warnings);
    warnings.extend(check_spec_versions(specs));

    (errors, warnings)
}

fn print_diagnostics(diagnostics: &DiagnosticMap) {
    for (path, messages) in diagnostics {
        eprintln!("{path}:");
        for message in messages {
            eprintln!("  - {message}");
        }
    }
}

fn push_error(diagnostics: &mut DiagnosticMap, err: spec_core::SpecError) {
    let key = error_key(&err);
    diagnostics.entry(key).or_default().push(err.to_string());
}

fn push_warning(diagnostics: &mut DiagnosticMap, warning: spec_core::SpecWarning) {
    let key = warning_key(&warning);
    diagnostics
        .entry(key)
        .or_default()
        .push(warning.to_string());
}

fn spec_error_code(err: &spec_core::SpecError) -> &'static str {
    match err {
        spec_core::SpecError::Io(_) => "SPEC_IO",
        spec_core::SpecError::InvalidUtf8 { .. } => "SPEC_INVALID_UTF8",
        spec_core::SpecError::YamlParse { .. } => "SPEC_YAML_PARSE",
        spec_core::SpecError::Json(_) => "SPEC_JSON",
        spec_core::SpecError::SchemaValidation { .. } => "SPEC_SCHEMA_VALIDATION",
        spec_core::SpecError::SemanticValidation { .. } => "SPEC_SEMANTIC_VALIDATION",
        spec_core::SpecError::RustKeyword { .. } => "SPEC_RUST_KEYWORD",
        spec_core::SpecError::DuplicateId { .. } => "SPEC_DUPLICATE_ID",
        spec_core::SpecError::DepCollision { .. } => "SPEC_DEP_COLLISION",
        spec_core::SpecError::MissingDep { .. } => "SPEC_MISSING_DEP",
        spec_core::SpecError::CyclicDep { .. } => "SPEC_CYCLIC_DEP",
        spec_core::SpecError::UseStatementInBody { .. } => "SPEC_USE_STATEMENT_IN_BODY",
        spec_core::SpecError::BodyRustMustBeBlock { .. } => "SPEC_BODY_RUST_MUST_BE_BLOCK",
        spec_core::SpecError::BodyRustLooksLikeFnDeclaration { .. } => {
            "SPEC_BODY_RUST_LOOKS_LIKE_FN_DECLARATION"
        }
        spec_core::SpecError::LocalTestExpectNotExpr { .. } => "SPEC_LOCAL_TEST_EXPECT_NOT_EXPR",
        spec_core::SpecError::DuplicateLocalTestId { .. } => "SPEC_DUPLICATE_LOCAL_TEST_ID",
        spec_core::SpecError::ContractTypeInvalid { .. } => "SPEC_CONTRACT_TYPE_INVALID",
        spec_core::SpecError::ContractInputNameInvalid { .. } => "SPEC_CONTRACT_INPUT_NAME_INVALID",
        spec_core::SpecError::Traversal { .. } => "SPEC_TRAVERSAL",
        spec_core::SpecError::Generator { .. } => "SPEC_GENERATOR",
        spec_core::SpecError::OutputDir { .. } => "SPEC_OUTPUT_DIR",
        spec_core::SpecError::MissingMarker { .. } => "SPEC_MISSING_MARKER",
    }
}

fn spec_error_to_json_entry(
    err: &spec_core::SpecError,
    id_by_path: &HashMap<String, String>,
) -> JsonErrorEntry {
    let code = spec_error_code(err).to_string();

    let fields = match err {
        spec_core::SpecError::Io(_) => ErrorFields {
            message: Some(err.to_string()),
            ..Default::default()
        },
        spec_core::SpecError::InvalidUtf8 { path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::YamlParse { message, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::Json(_) => ErrorFields {
            message: Some(err.to_string()),
            ..Default::default()
        },
        spec_core::SpecError::SchemaValidation { message, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::SemanticValidation { message, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::RustKeyword { path, segment, id } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            value: Some(segment.clone()),
            id: Some(id.clone()),
            ..Default::default()
        },
        spec_core::SpecError::DuplicateId { id, file1, file2 } => ErrorFields {
            unit: id_by_path.get(file1).cloned(),
            path: Some(file1.clone()),
            id: Some(id.clone()),
            path2: Some(file2.clone()),
            ..Default::default()
        },
        spec_core::SpecError::DepCollision {
            dep1,
            dep2,
            fn_name,
            path,
        } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            dep: Some(dep1.clone()),
            value: Some(fn_name.clone()),
            path2: Some(dep2.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MissingDep { dep, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            dep: Some(dep.clone()),
            ..Default::default()
        },
        spec_core::SpecError::CyclicDep { cycle_path, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            cycle: Some(cycle_path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::UseStatementInBody { path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::BodyRustMustBeBlock { path, message } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::LocalTestExpectNotExpr { id, path, message } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            id: Some(id.clone()),
            ..Default::default()
        },
        spec_core::SpecError::DuplicateLocalTestId { id, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            id: Some(id.clone()),
            ..Default::default()
        },
        spec_core::SpecError::ContractTypeInvalid {
            field,
            type_str,
            path,
            ..
        } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            field: Some(format!("contract.{field}")),
            value: Some(type_str.clone()),
            ..Default::default()
        },
        spec_core::SpecError::ContractInputNameInvalid { name, path, .. } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            field: Some(format!("contract.inputs.{name}")),
            ..Default::default()
        },
        spec_core::SpecError::Traversal { message, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::Generator { message } => ErrorFields {
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::OutputDir { message } => ErrorFields {
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MissingMarker { path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            ..Default::default()
        },
    };

    JsonErrorEntry {
        unit: fields.unit,
        code,
        path: fields.path,
        dep: fields.dep,
        field: fields.field,
        value: fields.value,
        message: fields.message,
        id: fields.id,
        path2: fields.path2,
        cycle: fields.cycle,
    }
}

fn error_paths(err: &spec_core::SpecError) -> Vec<String> {
    match err {
        spec_core::SpecError::DuplicateId { file1, file2, .. } => {
            vec![file1.clone(), file2.clone()]
        }
        spec_core::SpecError::InvalidUtf8 { path }
        | spec_core::SpecError::YamlParse { path, .. }
        | spec_core::SpecError::SchemaValidation { path, .. }
        | spec_core::SpecError::SemanticValidation { path, .. }
        | spec_core::SpecError::RustKeyword { path, .. }
        | spec_core::SpecError::DepCollision { path, .. }
        | spec_core::SpecError::MissingDep { path, .. }
        | spec_core::SpecError::CyclicDep { path, .. }
        | spec_core::SpecError::UseStatementInBody { path }
        | spec_core::SpecError::BodyRustMustBeBlock { path, .. }
        | spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path }
        | spec_core::SpecError::LocalTestExpectNotExpr { path, .. }
        | spec_core::SpecError::DuplicateLocalTestId { path, .. }
        | spec_core::SpecError::ContractTypeInvalid { path, .. }
        | spec_core::SpecError::ContractInputNameInvalid { path, .. }
        | spec_core::SpecError::Traversal { path, .. }
        | spec_core::SpecError::MissingMarker { path } => vec![path.clone()],
        spec_core::SpecError::Generator { .. }
        | spec_core::SpecError::OutputDir { .. }
        | spec_core::SpecError::Io(_)
        | spec_core::SpecError::Json(_) => Vec::new(),
    }
}

fn print_status_unit(unit: &JsonStatusUnit) {
    let detail = match unit.status {
        HealthState::Invalid => format!(
            "({} error{})",
            unit.errors.len(),
            pluralize(unit.errors.len())
        ),
        _ => match &unit.reason {
            Some(r) => r.clone(),
            None => match &unit.evidence_at {
                Some(ts) => format!("evidence:{ts}"),
                None => String::new(),
            },
        },
    };

    // Width 10 accommodates "incomplete" (longest state = 10 chars)
    println!(
        "{} {:<32} {:<10} {detail}",
        unit.status.symbol(),
        unit.id,
        unit.status.as_str()
    );
    if unit.status == HealthState::Invalid {
        for entry in &unit.errors {
            println!("  · {}", json_error_entry_to_human(entry));
        }
    }
}

fn json_error_entry_to_human(entry: &JsonErrorEntry) -> String {
    if let Some(message) = &entry.message {
        return format!("{}: {message}", entry.code);
    }

    if let Some(dep) = &entry.dep {
        return format!("{}: dep '{dep}' not found in this spec set", entry.code);
    }

    if let Some(field) = &entry.field {
        if let Some(value) = &entry.value {
            return format!("{}: {field}: invalid type '{value}'", entry.code);
        }
        return format!("{}: {field}", entry.code);
    }

    if let Some(id) = &entry.id {
        if let Some(path2) = &entry.path2 {
            return format!("{}: '{id}' also in {path2}", entry.code);
        }
        return format!("{}: {id}", entry.code);
    }

    entry.code.clone()
}

fn error_key(err: &spec_core::SpecError) -> String {
    match err {
        spec_core::SpecError::InvalidUtf8 { path }
        | spec_core::SpecError::YamlParse { path, .. }
        | spec_core::SpecError::SchemaValidation { path, .. }
        | spec_core::SpecError::SemanticValidation { path, .. }
        | spec_core::SpecError::RustKeyword { path, .. }
        | spec_core::SpecError::DepCollision { path, .. }
        | spec_core::SpecError::MissingDep { path, .. }
        | spec_core::SpecError::CyclicDep { path, .. }
        | spec_core::SpecError::UseStatementInBody { path }
        | spec_core::SpecError::BodyRustMustBeBlock { path, .. }
        | spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path }
        | spec_core::SpecError::LocalTestExpectNotExpr { path, .. }
        | spec_core::SpecError::DuplicateLocalTestId { path, .. }
        | spec_core::SpecError::ContractTypeInvalid { path, .. }
        | spec_core::SpecError::ContractInputNameInvalid { path, .. }
        | spec_core::SpecError::Traversal { path, .. }
        | spec_core::SpecError::MissingMarker { path } => path.clone(),
        spec_core::SpecError::DuplicateId { file1, file2, .. } => format!("{file1} | {file2}"),
        spec_core::SpecError::Generator { .. } | spec_core::SpecError::OutputDir { .. } => {
            "generation".to_string()
        }
        spec_core::SpecError::Io(_) | spec_core::SpecError::Json(_) => "validation".to_string(),
    }
}

fn warning_key(warning: &spec_core::SpecWarning) -> String {
    match warning {
        spec_core::SpecWarning::MissingDep { path, .. }
        | spec_core::SpecWarning::SymlinkCycleSkipped { path }
        | spec_core::SpecWarning::MissingSpecVersion { path, .. } => path.clone(),
    }
}

fn count_messages(diagnostics: &DiagnosticMap) -> usize {
    diagnostics.values().map(Vec::len).sum()
}

fn count_unique_files(errors: &DiagnosticMap) -> usize {
    let mut files = std::collections::BTreeSet::new();
    for key in errors.keys() {
        for part in key.split(" | ") {
            files.insert(part.trim());
        }
    }
    files.len()
}

fn pluralize(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command as ProcessCommand;
    use std::time::Instant;
    use tempfile::TempDir;

    fn write_spec(dir: &Path, relative_path: &str, body: &str) {
        let path = dir.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    fn copy_dir_all(src: &Path, dst: &Path) {
        fs::create_dir_all(dst).unwrap();

        for entry in fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if entry.file_type().unwrap().is_dir() {
                copy_dir_all(&entry_path, &dst_path);
            } else {
                fs::copy(&entry_path, &dst_path).unwrap();
            }
        }
    }

    fn benchmark_loaded_spec(index: usize, tests_per_spec: usize) -> LoadedSpec {
        let id = format!("pricing/bench_{index:04}");
        LoadedSpec {
            source: spec_core::types::SpecSource {
                file_path: format!("units/pricing/bench_{index:04}.unit.spec"),
                id: id.clone(),
            },
            spec: spec_core::types::SpecStruct {
                id,
                kind: "function".to_string(),
                intent: spec_core::types::Intent {
                    why: format!("Benchmark unit {index}"),
                },
                contract: None,
                deps: Vec::new(),
                imports: Vec::new(),
                body: spec_core::types::Body {
                    rust: "{ true }".to_string(),
                },
                local_tests: (0..tests_per_spec)
                    .map(|test_index| spec_core::types::LocalTest {
                        id: format!("case_{test_index:02}"),
                        expect: "true".to_string(),
                    })
                    .collect(),
                links: None,
                spec_version: None,
            },
        }
    }

    fn benchmark_specs(spec_count: usize, tests_per_spec: usize) -> Vec<LoadedSpec> {
        (0..spec_count)
            .map(|index| benchmark_loaded_spec(index, tests_per_spec))
            .collect()
    }

    fn benchmark_stdout(specs: &[LoadedSpec], output: &Path, crate_root: &Path) -> String {
        let output_prefix = output_module_prefix(output, crate_root).unwrap();
        let mut stdout = String::from("running synthetic benchmark tests\n");

        for spec in specs {
            let resolved = ResolvedSpec::from_spec(spec.spec.clone());
            for (test_index, local_test) in spec.spec.local_tests.iter().enumerate() {
                let full_name = expected_cargo_test_name(&resolved, &output_prefix, &local_test.id);
                let status = if test_index % 11 == 0 { "FAILED" } else { "ok" };
                stdout.push_str("test ");
                stdout.push_str(&full_name);
                stdout.push_str(" ... ");
                stdout.push_str(status);
                stdout.push('\n');
            }
        }

        stdout
    }

    fn parse_cargo_test_output_btree_baseline(
        stdout: &str,
    ) -> BTreeMap<String, ParsedCargoTestResult> {
        let mut results: BTreeMap<String, ParsedCargoTestResult> = BTreeMap::new();

        for line in stdout.lines() {
            let Some(rest) = line.strip_prefix("test ") else {
                continue;
            };
            let Some((full_name, terminal_status)) = rest.split_once(" ... ") else {
                continue;
            };

            let parsed = match terminal_status.trim() {
                "ok" => ParsedCargoTestResult {
                    status: "pass".to_string(),
                    reason: None,
                },
                "FAILED" => ParsedCargoTestResult {
                    status: "fail".to_string(),
                    reason: None,
                },
                other => ParsedCargoTestResult {
                    status: "error".to_string(),
                    reason: Some(other.to_string()),
                },
            };

            match results.get_mut(full_name) {
                Some(existing) => {
                    existing.status = "error".to_string();
                    existing.reason = Some("multiple matching cargo results".to_string());
                }
                None => {
                    results.insert(full_name.to_string(), parsed);
                }
            }
        }

        results
    }

    fn build_test_evidence_btree_baseline(
        specs: &[LoadedSpec],
        output_prefix: &str,
        parsed_test_results: &BTreeMap<String, ParsedCargoTestResult>,
        observed_at: &str,
        provenance: Option<&ArtifactProvenance>,
    ) -> Result<BTreeMap<String, PassportEvidence>> {
        let mut evidence_by_spec = BTreeMap::new();

        for spec in specs {
            let resolved = ResolvedSpec::from_spec(spec.spec.clone());
            let mut test_results = Vec::new();

            for local_test in &spec.spec.local_tests {
                let full_name = expected_cargo_test_name(&resolved, output_prefix, &local_test.id);
                let observed = parsed_test_results.get(&full_name);
                let (status, reason) = match observed {
                    Some(result) => (result.status.clone(), result.reason.clone()),
                    None => (
                        "unknown".to_string(),
                        Some("test not found in cargo output".to_string()),
                    ),
                };

                test_results.push(PassportTestResult {
                    id: local_test.id.clone(),
                    status,
                    reason,
                });
            }

            evidence_by_spec.insert(
                spec.spec.id.clone(),
                PassportEvidence {
                    build_status: "pass".to_string(),
                    test_results,
                    observed_at: observed_at.to_string(),
                    provenance: provenance.cloned(),
                },
            );
        }

        Ok(evidence_by_spec)
    }

    #[test]
    fn generate_command_bootstraps_marker_and_writes_files() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let units_dir = temp_dir.path().join("units");
        let output_dir = temp_dir.path().join("generated/spec");
        write_spec(
            &units_dir,
            "pricing/apply_discount.unit.spec",
            r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    {
        round(Decimal::ZERO)
    }
"#,
        );

        generate_command(&units_dir, Some(&output_dir)).unwrap();

        assert!(output_dir.join(".spec-generated").exists());
        assert!(output_dir.join("pricing/apply_discount.rs").exists());
        assert!(output_dir.join("pricing/mod.rs").exists());
        assert!(output_dir.join("mod.rs").exists());
    }

    #[test]
    fn validate_command_collects_directory_errors() {
        let temp_dir = TempDir::new().unwrap();
        let units_dir = temp_dir.path().join("units");
        write_spec(
            &units_dir,
            "pricing/good.unit.spec",
            r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    pub fn apply_discount() {}
"#,
        );
        write_spec(
            &units_dir,
            "pricing/bad.unit.spec",
            r#"
id: pricing/type
kind: function
intent:
  why: Should fail.
body:
  rust: |
    use std::fmt;
    pub fn type() {}
extra_field: nope
"#,
        );

        let result = validate_command(&units_dir, false, OutputFormat::Text);
        assert!(result.is_err());
        let error_text = format!("{:#}", result.unwrap_err());
        assert!(error_text.contains("error"));
    }

    #[test]
    fn generate_command_writes_doc_comments_for_ecommerce_units() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let fixture_src = repo_root.join("examples/ecommerce");
        let fixture_dst = temp_dir.path().join("ecommerce");
        copy_dir_all(&fixture_src, &fixture_dst);

        let units_dir = fixture_dst.join("units");
        let output_dir = fixture_dst.join("src/generated");
        generate_command(&units_dir, Some(&output_dir)).unwrap();

        let apply_tax = fs::read_to_string(output_dir.join("pricing/apply_tax.rs")).unwrap();
        assert!(apply_tax.contains(
            "/// Add sales tax to a subtotal using a rate expressed as a decimal fraction.\n"
        ));
        assert!(apply_tax.contains("pub fn apply_tax("));
    }

    #[test]
    fn cargo_doc_succeeds_for_generated_ecommerce_docs() {
        if !cargo_available() {
            return;
        }

        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let fixture_src = repo_root.join("examples/ecommerce");
        let fixture_dst = temp_dir.path().join("ecommerce");
        copy_dir_all(&fixture_src, &fixture_dst);

        let units_dir = fixture_dst.join("units");
        let output_dir = fixture_dst.join("src/generated");
        generate_command(&units_dir, Some(&output_dir)).unwrap();

        let output = ProcessCommand::new("cargo")
            .current_dir(&fixture_dst)
            .env("CARGO_TARGET_DIR", temp_dir.path().join("cargo-target"))
            .env("CARGO_TERM_COLOR", "never")
            .args(["doc", "--no-deps"])
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "cargo doc failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn spec_error_code_namespace_is_stable_and_exhaustive_for_current_variants() {
        let io_error = std::io::Error::other("boom");
        let json_error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        let errors = vec![
            spec_core::SpecError::Io(io_error),
            spec_core::SpecError::InvalidUtf8 {
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::YamlParse {
                message: "bad yaml".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::Json(json_error),
            spec_core::SpecError::SchemaValidation {
                message: "bad schema".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::SemanticValidation {
                message: "bad semantics".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::RustKeyword {
                segment: "type".to_string(),
                id: "pricing/type".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::DuplicateId {
                id: "pricing/apply_discount".to_string(),
                file1: "units/a.unit.spec".to_string(),
                file2: "units/b.unit.spec".to_string(),
            },
            spec_core::SpecError::DepCollision {
                dep1: "money/round".to_string(),
                dep2: "money/format".to_string(),
                fn_name: "money".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::MissingDep {
                dep: "money/round".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::CyclicDep {
                cycle_path: vec!["a".to_string(), "b".to_string()],
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::UseStatementInBody {
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::BodyRustMustBeBlock {
                message: "expected block".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::BodyRustLooksLikeFnDeclaration {
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::LocalTestExpectNotExpr {
                id: "happy_path".to_string(),
                message: "not expr".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::DuplicateLocalTestId {
                id: "happy_path".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::ContractTypeInvalid {
                field: "contract.returns".to_string(),
                type_str: "Vec<".to_string(),
                message: "bad type".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::ContractInputNameInvalid {
                name: "bad-name".to_string(),
                message: "bad identifier".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::Traversal {
                message: "walk failed".to_string(),
                path: "units".to_string(),
            },
            spec_core::SpecError::Generator {
                message: "gen failed".to_string(),
            },
            spec_core::SpecError::OutputDir {
                message: "outside root".to_string(),
            },
            spec_core::SpecError::MissingMarker {
                path: "generated/spec".to_string(),
            },
        ];

        let codes = errors
            .iter()
            .map(spec_error_code)
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(codes.len(), errors.len());
        assert!(codes.iter().all(|code| code.starts_with("SPEC_")));
        assert!(codes.iter().all(|code| !code.is_empty()));
    }

    #[test]
    fn spec_error_to_json_entry_preserves_multi_field_variants() {
        let mut id_by_path = HashMap::new();
        id_by_path.insert(
            "units/pricing/apply_discount.unit.spec".to_string(),
            "pricing/apply_discount".to_string(),
        );

        let duplicate = spec_error_to_json_entry(
            &spec_core::SpecError::DuplicateId {
                id: "pricing/apply_discount".to_string(),
                file1: "units/pricing/apply_discount.unit.spec".to_string(),
                file2: "units/pricing/apply_tax.unit.spec".to_string(),
            },
            &id_by_path,
        );
        assert_eq!(duplicate.unit.as_deref(), Some("pricing/apply_discount"));
        assert_eq!(
            duplicate.path.as_deref(),
            Some("units/pricing/apply_discount.unit.spec")
        );
        assert_eq!(duplicate.id.as_deref(), Some("pricing/apply_discount"));
        assert_eq!(
            duplicate.path2.as_deref(),
            Some("units/pricing/apply_tax.unit.spec")
        );

        let dep_collision = spec_error_to_json_entry(
            &spec_core::SpecError::DepCollision {
                dep1: "money/round".to_string(),
                dep2: "money/format".to_string(),
                fn_name: "money".to_string(),
                path: "units/pricing/apply_discount.unit.spec".to_string(),
            },
            &id_by_path,
        );
        assert_eq!(dep_collision.dep.as_deref(), Some("money/round"));
        assert_eq!(dep_collision.value.as_deref(), Some("money"));
        assert_eq!(dep_collision.path2.as_deref(), Some("money/format"));
    }

    #[test]
    fn output_module_prefix_absolute_crate_root_strips_src() {
        // Primary production path: absolute crate_root + absolute output under {crate_root}/src/
        let crate_root = Path::new("/home/user/myproject");
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/generated"),
                crate_root
            )
            .unwrap(),
            "generated"
        );
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/generated/spec"),
                crate_root
            )
            .unwrap(),
            "generated::spec"
        );
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/api/gen"),
                crate_root
            )
            .unwrap(),
            "api::gen"
        );
    }

    #[test]
    fn output_module_prefix_relative_path_fallback_strips_src_component() {
        // Fallback path: relative output (e.g., explicit --output src/generated with relative CWD)
        let crate_root = Path::new("");
        assert_eq!(
            output_module_prefix(Path::new("src/generated"), crate_root).unwrap(),
            "generated"
        );
        assert_eq!(
            output_module_prefix(Path::new("src/generated/spec"), crate_root).unwrap(),
            "generated::spec"
        );
    }

    #[test]
    fn output_module_prefix_no_src_prefix_preserved() {
        // Output not under src/ — kept as-is (user likely set generated_module_prefix explicitly)
        let crate_root = Path::new("/home/user/myproject");
        assert_eq!(
            output_module_prefix(Path::new("generated"), crate_root).unwrap(),
            "generated"
        );
    }

    #[test]
    fn build_test_evidence_preserves_found_missing_and_duplicate_statuses() {
        let output = Path::new("src/generated");
        let crate_root = Path::new("");
        let spec = benchmark_loaded_spec(0, 3);
        let resolved = ResolvedSpec::from_spec(spec.spec.clone());
        let output_prefix = output_module_prefix(output, crate_root).unwrap();

        let mut parsed_test_results = HashMap::new();
        parsed_test_results.insert(
            expected_cargo_test_name(&resolved, &output_prefix, "case_00"),
            ParsedCargoTestResult {
                status: "pass".to_string(),
                reason: None,
            },
        );
        parsed_test_results.insert(
            expected_cargo_test_name(&resolved, &output_prefix, "case_01"),
            ParsedCargoTestResult {
                status: "error".to_string(),
                reason: Some("multiple matching cargo results".to_string()),
            },
        );

        let evidence = build_test_evidence(
            std::slice::from_ref(&spec),
            &output_prefix,
            &parsed_test_results,
            "2026-04-11T12:00:00Z",
            None,
        )
        .unwrap();

        let test_results = &evidence["pricing/bench_0000"].test_results;
        assert_eq!(test_results[0].status, "pass");
        assert_eq!(test_results[0].reason, None);
        assert_eq!(test_results[1].status, "error");
        assert_eq!(
            test_results[1].reason.as_deref(),
            Some("multiple matching cargo results")
        );
        assert_eq!(test_results[2].status, "unknown");
        assert_eq!(
            test_results[2].reason.as_deref(),
            Some("test not found in cargo output")
        );
    }

    #[test]
    #[ignore = "manual benchmark for Priority 4 parse/evidence ship gate"]
    fn benchmark_parse_and_evidence_hash_lookup_against_btree_baseline() {
        let output = Path::new("src/generated");
        let crate_root = Path::new("");
        let output_prefix = output_module_prefix(output, crate_root).unwrap();
        let specs = benchmark_specs(600, 8);
        let stdout = benchmark_stdout(&specs, output, crate_root);
        let observed_at = "2026-04-11T12:00:00Z";

        let baseline_evidence = build_test_evidence_btree_baseline(
            &specs,
            &output_prefix,
            &parse_cargo_test_output_btree_baseline(&stdout),
            observed_at,
            None,
        )
        .unwrap();
        let hash_evidence = build_test_evidence(
            &specs,
            &output_prefix,
            &parse_cargo_test_output(&stdout),
            observed_at,
            None,
        )
        .unwrap();
        assert_eq!(hash_evidence, baseline_evidence);

        const ITERS: usize = 75;

        for _ in 0..5 {
            let _ = std::hint::black_box(parse_cargo_test_output_btree_baseline(&stdout));
            let _ = std::hint::black_box(parse_cargo_test_output(&stdout));
        }

        let btree_started = Instant::now();
        for _ in 0..ITERS {
            let parsed = parse_cargo_test_output_btree_baseline(std::hint::black_box(&stdout));
            let evidence = build_test_evidence_btree_baseline(
                &specs,
                &output_prefix,
                &parsed,
                observed_at,
                None,
            )
            .unwrap();
            std::hint::black_box(evidence);
        }
        let btree_elapsed = btree_started.elapsed();

        let hash_started = Instant::now();
        for _ in 0..ITERS {
            let parsed = parse_cargo_test_output(std::hint::black_box(&stdout));
            let evidence =
                build_test_evidence(&specs, &output_prefix, &parsed, observed_at, None).unwrap();
            std::hint::black_box(evidence);
        }
        let hash_elapsed = hash_started.elapsed();

        let speedup = btree_elapsed.as_secs_f64() / hash_elapsed.as_secs_f64();
        eprintln!(
            "Priority 4 benchmark: btree={btree_elapsed:?}, hash={hash_elapsed:?}, speedup={speedup:.2}x, specs={}, tests_per_spec={}",
            specs.len(),
            specs[0].spec.local_tests.len()
        );
    }

    #[test]
    fn concurrent_passport_write_guard_detects_other_active_writer() {
        let temp_dir = TempDir::new().unwrap();
        let passport_root = temp_dir.path().join("units");
        fs::create_dir_all(&passport_root).unwrap();

        let registry_dir = concurrent_passport_writer_registry_dir(&passport_root, temp_dir.path());
        fs::create_dir_all(&registry_dir).unwrap();
        fs::write(
            registry_dir.join(concurrent_passport_writer_marker_name(7, SystemTime::now())),
            "",
        )
        .unwrap();

        let (_guard, other_writers) = ConcurrentPassportWriteGuard::begin_in(
            &passport_root,
            temp_dir.path(),
            42,
            SystemTime::now(),
        )
        .unwrap();

        assert_eq!(other_writers, 1);
        let warning =
            concurrent_passport_write_warning_message(&passport_root, other_writers).unwrap();
        assert!(warning.contains("1 other spec process"), "{warning}");
        assert!(
            warning.contains(passport_root.to_str().unwrap()),
            "{warning}"
        );
    }

    #[test]
    fn concurrent_passport_write_guard_ignores_stale_markers() {
        let temp_dir = TempDir::new().unwrap();
        let passport_root = temp_dir.path().join("units");
        fs::create_dir_all(&passport_root).unwrap();

        let stale_now = UNIX_EPOCH + Duration::from_secs(10);
        let registry_dir = concurrent_passport_writer_registry_dir(&passport_root, temp_dir.path());
        fs::create_dir_all(&registry_dir).unwrap();
        fs::write(
            registry_dir.join(concurrent_passport_writer_marker_name(7, stale_now)),
            "",
        )
        .unwrap();

        let (_guard, other_writers) = ConcurrentPassportWriteGuard::begin_in(
            &passport_root,
            temp_dir.path(),
            42,
            UNIX_EPOCH + Duration::from_secs(10 + CONCURRENT_PASSPORT_WRITER_TTL_SECS + 1),
        )
        .unwrap();

        assert_eq!(other_writers, 0);
    }
}
