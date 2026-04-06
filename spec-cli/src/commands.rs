use crate::config::{WorkspaceConfig, load_workspace_config};
use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand, ValueEnum};
use serde::Serialize;
use spec_core::export::build_export_bundle;
use spec_core::generator::{
    GenerateOptions, clean_output_dir, generate_code_with_options, generate_mod_rs,
    safe_output_path, write_generated_file,
};
use spec_core::loader::{is_unit_spec, load_directory_report, load_file};
use spec_core::normalizer::normalize_spec;
use spec_core::passport::{
    PassportEvidence, PassportTestResult, build_passport_with_evidence, compute_contract_hash,
    ensure_gitignore_entry, read_passport, rfc3339_now, write_passport,
};
use spec_core::pipeline::{
    ParsedCargoTestResult, cargo_available, parse_cargo_test_output, run_cargo_build,
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

type CollectedSpecs = (
    Vec<LoadedSpec>,
    Vec<spec_core::SpecError>,
    Vec<spec_core::SpecWarning>,
    usize,
);
type DiagnosticMap = BTreeMap<String, Vec<String>>;

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
}

#[derive(Serialize)]
struct JsonStatusUnit {
    id: String,
    status: &'static str,
    errors: Vec<JsonErrorEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_at: Option<String>,
    stale: bool,
}

#[derive(Serialize)]
struct JsonErrorEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    code: String,
    path: String,
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
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Validate(args) => validate_command(&args.path, args.no_strict, args.format),
            Self::Status(args) => status_command(&args.path, args.format),
            Self::Generate(args) => generate_command(&args.path, &args.output),
            Self::Build(args) => {
                let config = load_workspace_config(&args.path)?;
                build_command(
                    &args.path,
                    &args.output,
                    args.crate_root.as_deref(),
                    &config,
                )
            }
            Self::Test(args) => {
                let config = load_workspace_config(&args.path)?;
                test_command(
                    &args.path,
                    &args.output,
                    args.crate_root.as_deref(),
                    &config,
                )
            }
            Self::Export(args) => export_command(&args.path, args.output.as_deref()),
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
    #[arg(long, default_value = "generated/spec")]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(
        value_name = "PATH",
        help = "Directory containing .unit.spec files, or a single .unit.spec file"
    )]
    pub path: PathBuf,
    #[arg(long, default_value = "generated/spec")]
    pub output: PathBuf,
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
    #[arg(long, default_value = "generated/spec")]
    pub output: PathBuf,
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
                schema_version: 1,
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

    let mut errors_by_path: HashMap<String, Vec<JsonErrorEntry>> = HashMap::new();
    for err in &validation_errors {
        for path in error_paths(err) {
            errors_by_path
                .entry(path)
                .or_default()
                .push(spec_error_to_json_entry(err, &id_by_path));
        }
    }

    if has_loader_errors {
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
                    schema_version: 1,
                    units: vec![],
                };
                let json = serde_json::to_string_pretty(&response)?;
                print!("{json}");
                std::io::stdout().flush()?;
            }
        }
        return Ok(());
    }

    let mut units = Vec::with_capacity(specs.len());
    let mut has_invalid_or_stale = has_loader_errors;

    for spec in &specs {
        let source_path = Path::new(&spec.source.file_path);
        let passport = match read_passport(source_path) {
            Ok(passport) => passport,
            Err(err) => {
                eprintln!(
                    "⚠ failed to read passport for {}: {err}",
                    source_path.display()
                );
                None
            }
        };
        let live_hash = compute_contract_hash(spec);
        let errors = errors_by_path
            .remove(&spec.source.file_path)
            .unwrap_or_default();
        let invalid = !errors.is_empty();
        let stale = !invalid
            && passport
                .as_ref()
                .and_then(|p| p.contract_hash.as_ref())
                .is_some_and(|passport_hash| live_hash.as_deref() != Some(passport_hash.as_str()));
        let evidence_at = if invalid {
            None
        } else {
            passport
                .as_ref()
                .and_then(|p| p.evidence.as_ref())
                .map(|e| e.observed_at.clone())
        };
        let status = if invalid {
            "invalid"
        } else if stale {
            "stale"
        } else {
            "valid"
        };

        if invalid || stale {
            has_invalid_or_stale = true;
        }

        units.push(JsonStatusUnit {
            id: spec.spec.id.clone(),
            status,
            errors,
            evidence_at,
            stale,
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
                schema_version: 1,
                units,
            };
            let json = serde_json::to_string_pretty(&response)?;
            print!("{json}");
            std::io::stdout().flush()?;
        }
    }

    if has_invalid_or_stale {
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

    let bundle = build_export_bundle(&specs, &rfc3339_now());
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

fn generate_command(path: &Path, output: &Path) -> Result<()> {
    let generated = generate_specs(path, output)?;
    if !generated.specs.is_empty() {
        let passport_root = if path.is_file() {
            path.parent().unwrap_or(path)
        } else {
            path
        };
        finalize_passports(
            passport_root,
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
        let passport = build_passport_with_evidence(
            spec,
            generated_at,
            evidence_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned(),
            contract_hash_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned(),
        );
        let source_path = Path::new(&spec.source.file_path);
        write_passport(&passport, source_path)
            .with_context(|| format!("Failed to write passport for {}", spec.source.id))?;
    }

    Ok(())
}

struct PipelineContext {
    crate_root: PathBuf,
    cargo_target_dir: PathBuf,
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
        _temp_dir: temp_dir,
    })
}

fn build_command(
    path: &Path,
    output: &Path,
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

    let generated = generate_specs(path, output)?;
    if !generated.specs.is_empty() {
        finalize_passports(path, &generated.specs, &generated.generated_at, None, None)?;
    }

    let result = run_cargo_build(&ctx.crate_root, &ctx.cargo_target_dir)?;
    print!("{}", result.stdout);
    eprint!("{}", result.stderr);
    if result.exit_code != 0 {
        bail!("❌ cargo build failed");
    }
    Ok(())
}

fn test_command(
    path: &Path,
    output: &Path,
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

    let generated = generate_specs(spec_root, output)?;
    if generated.specs.is_empty() {
        return Ok(());
    }

    if target_spec.is_none() {
        finalize_passports(path, &generated.specs, &generated.generated_at, None, None)?;
    }

    let output_prefix = if target_spec.is_some() {
        Some(output_module_prefix(output)?)
    } else {
        None
    };
    let filter = match (target_spec.as_ref(), output_prefix.as_deref()) {
        (Some(target), Some(prefix)) => {
            let resolved = ResolvedSpec::from_spec(target.spec.clone());
            Some(cargo_test_filter_for(&resolved, prefix))
        }
        _ => None,
    };

    let build_result = run_cargo_build(&ctx.crate_root, &ctx.cargo_target_dir)?;
    print!("{}", build_result.stdout);
    eprint!("{}", build_result.stderr);
    if build_result.exit_code != 0 {
        let observed_at = rfc3339_now();
        if let Some(target_spec) = target_spec.as_ref() {
            let evidence_by_spec =
                build_failure_evidence(std::slice::from_ref(target_spec), &observed_at);
            let contract_hash_by_spec = contract_hashes_for(std::slice::from_ref(target_spec));
            finalize_passports(
                spec_root,
                std::slice::from_ref(target_spec),
                &generated.generated_at,
                Some(&evidence_by_spec),
                contract_hash_by_spec.as_ref(),
            )?;
        } else {
            let evidence_by_spec = build_failure_evidence(&generated.specs, &observed_at);
            let contract_hash_by_spec = contract_hashes_for(&generated.specs);
            finalize_passports(
                path,
                &generated.specs,
                &generated.generated_at,
                Some(&evidence_by_spec),
                contract_hash_by_spec.as_ref(),
            )?;
        }
        bail!("❌ cargo build failed");
    }

    let test_result = run_cargo_test(&ctx.crate_root, &ctx.cargo_target_dir, filter.as_deref())?;
    print!("{}", test_result.stdout);
    eprint!("{}", test_result.stderr);

    if target_spec.is_some() && zero_tests_ran(&test_result.stdout) {
        bail!("❌ cargo test matched 0 tests");
    }

    let parsed_test_results = parse_cargo_test_output(&test_result.stdout);
    let observed_at = rfc3339_now();
    if let Some(target_spec) = target_spec.as_ref() {
        let evidence_by_spec = build_test_evidence(
            std::slice::from_ref(target_spec),
            output,
            &parsed_test_results,
            &observed_at,
        )?;
        let contract_hash_by_spec = contract_hashes_for(std::slice::from_ref(target_spec));
        finalize_passports(
            spec_root,
            std::slice::from_ref(target_spec),
            &generated.generated_at,
            Some(&evidence_by_spec),
            contract_hash_by_spec.as_ref(),
        )?;
    } else {
        let evidence_by_spec =
            build_test_evidence(&generated.specs, output, &parsed_test_results, &observed_at)?;
        let contract_hash_by_spec = contract_hashes_for(&generated.specs);
        finalize_passports(
            path,
            &generated.specs,
            &generated.generated_at,
            Some(&evidence_by_spec),
            contract_hash_by_spec.as_ref(),
        )?;
    }
    if test_result.exit_code != 0 {
        bail!("❌ cargo test failed");
    }
    Ok(())
}

fn build_failure_evidence(
    specs: &[LoadedSpec],
    observed_at: &str,
) -> BTreeMap<String, PassportEvidence> {
    specs
        .iter()
        .map(|spec| {
            (
                spec.spec.id.clone(),
                PassportEvidence {
                    build_status: "fail".to_string(),
                    test_results: vec![],
                    observed_at: observed_at.to_string(),
                },
            )
        })
        .collect()
}

fn build_test_evidence(
    specs: &[LoadedSpec],
    output: &Path,
    parsed_test_results: &BTreeMap<String, ParsedCargoTestResult>,
    observed_at: &str,
) -> Result<BTreeMap<String, PassportEvidence>> {
    let output_prefix = output_module_prefix(output)?;
    let mut evidence_by_spec = BTreeMap::new();

    for spec in specs {
        let resolved = ResolvedSpec::from_spec(spec.spec.clone());
        let mut test_results = Vec::new();

        for local_test in &spec.spec.local_tests {
            let full_name = expected_cargo_test_name(&resolved, &output_prefix, &local_test.id);
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
            },
        );
    }

    Ok(evidence_by_spec)
}

fn output_module_prefix(output: &Path) -> Result<String> {
    output
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(|name| name.to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "❌ could not determine output module prefix from {}",
                output.display()
            )
        })
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

fn spec_error_to_json_entry(
    err: &spec_core::SpecError,
    id_by_path: &HashMap<String, String>,
) -> JsonErrorEntry {
    let code = match err {
        spec_core::SpecError::Io(_) => "Io",
        spec_core::SpecError::InvalidUtf8 { .. } => "InvalidUtf8",
        spec_core::SpecError::YamlParse { .. } => "YamlParse",
        spec_core::SpecError::Json(_) => "Json",
        spec_core::SpecError::SchemaValidation { .. } => "SchemaValidation",
        spec_core::SpecError::SemanticValidation { .. } => "SemanticValidation",
        spec_core::SpecError::RustKeyword { .. } => "RustKeyword",
        spec_core::SpecError::DuplicateId { .. } => "DuplicateId",
        spec_core::SpecError::DepCollision { .. } => "DepCollision",
        spec_core::SpecError::MissingDep { .. } => "MissingDep",
        spec_core::SpecError::CyclicDep { .. } => "CyclicDep",
        spec_core::SpecError::UseStatementInBody { .. } => "UseStatementInBody",
        spec_core::SpecError::BodyRustMustBeBlock { .. } => "BodyRustMustBeBlock",
        spec_core::SpecError::BodyRustLooksLikeFnDeclaration { .. } => {
            "BodyRustLooksLikeFnDeclaration"
        }
        spec_core::SpecError::LocalTestExpectNotExpr { .. } => "LocalTestExpectNotExpr",
        spec_core::SpecError::DuplicateLocalTestId { .. } => "DuplicateLocalTestId",
        spec_core::SpecError::ContractTypeInvalid { .. } => "ContractTypeInvalid",
        spec_core::SpecError::ContractInputNameInvalid { .. } => "ContractInputNameInvalid",
        spec_core::SpecError::Traversal { .. } => "Traversal",
        spec_core::SpecError::Generator { .. } => "Generator",
        spec_core::SpecError::OutputDir { .. } => "OutputDir",
        spec_core::SpecError::MissingMarker { .. } => "MissingMarker",
    }
    .to_string();

    let (unit, path, dep, field, value, message, id, path2, cycle) = match err {
        spec_core::SpecError::Io(_) => (
            None,
            String::new(),
            None,
            None,
            None,
            Some(err.to_string()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::InvalidUtf8 { path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::YamlParse { message, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::Json(_) => (
            None,
            String::new(),
            None,
            None,
            None,
            Some(err.to_string()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::SchemaValidation { message, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::SemanticValidation { message, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::RustKeyword { path, segment, id } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            Some(segment.clone()),
            None,
            Some(id.clone()),
            None,
            None,
        ),
        spec_core::SpecError::DuplicateId { id, file1, file2 } => (
            id_by_path.get(file1).cloned(),
            file1.clone(),
            None,
            None,
            None,
            None,
            Some(id.clone()),
            Some(file2.clone()),
            None,
        ),
        spec_core::SpecError::DepCollision { dep1, dep2, fn_name, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            Some(dep1.clone()),
            None,
            Some(fn_name.clone()),
            None,
            None,
            Some(dep2.clone()),
            None,
        ),
        spec_core::SpecError::MissingDep { dep, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            Some(dep.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::CyclicDep { cycle_path, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(cycle_path.clone()),
        ),
        spec_core::SpecError::UseStatementInBody { path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::BodyRustMustBeBlock { path, message } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::LocalTestExpectNotExpr { id, path, message } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            Some(message.clone()),
            Some(id.clone()),
            None,
            None,
        ),
        spec_core::SpecError::DuplicateLocalTestId { id, path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            Some(id.clone()),
            None,
            None,
        ),
        spec_core::SpecError::ContractTypeInvalid {
            field,
            type_str,
            path,
            ..
        } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            Some(format!("contract.{field}")),
            Some(type_str.clone()),
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::ContractInputNameInvalid { name, path, .. } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            Some(format!("contract.inputs.{name}")),
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::Traversal { path, .. } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        spec_core::SpecError::Generator { message } => (
            None,
            String::new(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::OutputDir { message } => (
            None,
            String::new(),
            None,
            None,
            None,
            Some(message.clone()),
            None,
            None,
            None,
        ),
        spec_core::SpecError::MissingMarker { path } => (
            id_by_path.get(path).cloned(),
            path.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };

    JsonErrorEntry {
        unit,
        code,
        path,
        dep,
        field,
        value,
        message,
        id,
        path2,
        cycle,
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
    let symbol = match unit.status {
        "valid" => {
            if unit.evidence_at.is_some() {
                "✓"
            } else {
                "—"
            }
        }
        "invalid" => "✗",
        "stale" => "~",
        _ => "?",
    };

    let detail = match unit.status {
        "invalid" => format!(
            "({} error{})",
            unit.errors.len(),
            pluralize(unit.errors.len())
        ),
        "stale" => match &unit.evidence_at {
            Some(ts) => format!("evidence:{ts}  (contract changed)"),
            None => "no-evidence  (contract changed)".to_string(),
        },
        _ => match &unit.evidence_at {
            Some(ts) => format!("evidence:{ts}"),
            None => "no-evidence".to_string(),
        },
    };

    println!("{symbol} {:<32} {:<7} {detail}", unit.id, unit.status);
    if unit.status == "invalid" {
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

        generate_command(&units_dir, &output_dir).unwrap();

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
        generate_command(&units_dir, &output_dir).unwrap();

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
        generate_command(&units_dir, &output_dir).unwrap();

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
}
