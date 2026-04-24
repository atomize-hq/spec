use crate::config::{
    ResolvedLibrary, WorkspaceConfigError, WorkspaceContext, load_workspace_context,
    read_workspace_config_file, repo_root_for, rewrite_workspace_config_relative_paths,
};
use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand, ValueEnum};
use serde::{Serialize, Serializer};
use spec_core::escape_hatch::{EscapeHatchGate, EscapeHatchGateStatus};
use spec_core::export::{build_export_bundle, build_plan_export_bundle};
use spec_core::generator::{
    GenerateOptions, clean_output_dir, generate_and_write_molecule_tests, generate_mod_rs,
    generate_unit_code_with_options, safe_output_path_with_project_root, write_generated_file,
};
use spec_core::graph::{ProjectedUnitRef, project_unit, top_level_deps};
use spec_core::loader::{
    DirectoryLoadReport, discover_library_roots_bounded, is_molecule_test_spec, is_unit_spec,
    load_directory_report, load_directory_report_bounded, load_file, load_molecule_test_directory,
    load_molecule_test_directory_report, load_molecule_test_directory_report_bounded,
    load_molecule_test_file, load_plan_file,
};
use spec_core::molecule_evidence::{
    MoleculeEvidence, MoleculeEvidenceStatus, build_molecule_evidence,
    ensure_gitignore_entry as ensure_molecule_evidence_gitignore_entry,
    molecule_evidence_is_current_pass, molecule_evidence_is_stale, read_molecule_evidence,
    write_molecule_evidence,
};
use spec_core::normalizer::normalize_unit;
use spec_core::passport::{
    ArtifactProvenance, PassportEvidence, PassportFreshness, PassportMarker,
    PassportProjectionContext, PassportTestResult, apply_projected_passport_truth,
    build_passport_preserving_proof_state, build_passport_with_evidence, compute_contract_hash,
    ensure_gitignore_entry, project_passport_truth, read_passport, rfc3339_now, write_passport,
};
use spec_core::pipeline::{
    ParsedCargoTestResult, Verbosity, cargo_available, output_module_prefix,
    parse_cargo_test_output, run_cargo_build, run_cargo_test, workspace_root_for, zero_tests_ran,
};
use spec_core::plan::{
    PlanAcceptanceClosure, PlanAcceptanceClosureStatus, PlanComputedImpact, build_plan_report,
};
use spec_core::semantic_review::{
    SemanticHealthEffect, SemanticProjectionMode, SemanticReview, semantic_health_effect,
    semantic_review_summary,
};
#[cfg(test)]
use spec_core::types::ResolvedSpec;
use spec_core::types::{
    DepRef, LoadedMoleculeTest, LoadedSpec, NormalizedUnit, QualifiedUnitRef, ResolvedMoleculeTest,
};
use spec_core::validator::{
    QualifiedLoadedSpec, ValidationOptions, check_spec_versions, validate_full_with_options,
    validate_molecule_test_covers, validate_molecule_test_semantic,
    validate_no_duplicate_molecule_test_ids, validate_no_duplicate_qualified_ids,
    validate_qualified_deps_exist_with_options,
};
#[cfg(test)]
use spec_core::validator::{validate_deps_exist_with_options, validate_no_duplicate_ids};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
type PlanValidationInputs = (
    ValidationSpecCollection,
    Vec<spec_core::SpecError>,
    Vec<spec_core::SpecWarning>,
    Vec<LoadedMoleculeTest>,
);
type DiagnosticMap = BTreeMap<String, Vec<String>>;

struct ImportedLibrarySpecs {
    alias: String,
    specs: Vec<LoadedSpec>,
}

struct ValidationSpecCollection {
    root_specs: Vec<LoadedSpec>,
    support_specs: Vec<LoadedSpec>,
    imported_libraries: Vec<ImportedLibrarySpecs>,
    loader_errors: Vec<spec_core::SpecError>,
    loader_warnings: Vec<spec_core::SpecWarning>,
    total_files: usize,
}

impl ValidationSpecCollection {
    fn local_specs(&self) -> Vec<&LoadedSpec> {
        let mut specs = Vec::with_capacity(self.root_specs.len() + self.support_specs.len());
        specs.extend(self.root_specs.iter());
        specs.extend(self.support_specs.iter());
        specs
    }

    fn all_specs(&self) -> Vec<&LoadedSpec> {
        let mut specs = Vec::with_capacity(
            self.root_specs.len()
                + self.support_specs.len()
                + self
                    .imported_libraries
                    .iter()
                    .map(|library| library.specs.len())
                    .sum::<usize>(),
        );
        specs.extend(self.root_specs.iter());
        specs.extend(self.support_specs.iter());
        for library in &self.imported_libraries {
            specs.extend(library.specs.iter());
        }
        specs
    }
}

const VALIDATE_JSON_SCHEMA_VERSION: u8 = 3;
const STATUS_JSON_SCHEMA_VERSION: u8 = 3;
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
    warnings: Vec<JsonWarningEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    computed_impact: Option<PlanComputedImpact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acceptance_closure: Option<PlanAcceptanceClosure>,
}

#[derive(Serialize)]
struct JsonStatusResponse {
    schema_version: u8,
    roots: Vec<JsonStatusRoot>,
    units: Vec<JsonStatusUnit>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    loader_errors: Vec<JsonErrorEntry>,
}

#[derive(Serialize)]
struct JsonStatusRoot {
    root: String,
    units: Vec<JsonStatusUnit>,
    molecule_tests: Vec<JsonStatusMoleculeTest>,
}

type JsonStatusUnit = JsonStatusEntry;
type JsonStatusMoleculeTest = JsonStatusEntry;

#[derive(Clone, Serialize)]
struct JsonStatusEntry {
    id: String,
    status: HealthState,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    errors: Vec<JsonErrorEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    freshness: Option<PassportFreshness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    markers: Option<Vec<PassportMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    escape_hatch_gate: Option<EscapeHatchGate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_review: Option<SemanticReview>,
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
struct JsonWarningEntry {
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    message: String,
}

fn workspace_config_error_to_json_entry(err: &WorkspaceConfigError) -> JsonErrorEntry {
    JsonErrorEntry {
        unit: None,
        code: err.code().to_string(),
        path: Some(err.config_path().display().to_string()),
        dep: None,
        field: None,
        value: None,
        message: Some(err.detail_message()),
        id: None,
        path2: None,
        cycle: None,
    }
}

fn emit_json_validate_response(response: &JsonValidateResponse) -> Result<()> {
    let json = serde_json::to_string_pretty(response)?;
    print!("{json}");
    std::io::stdout().flush()?;
    Ok(())
}

fn emit_json_status_response(response: &JsonStatusResponse) -> Result<()> {
    let json = serde_json::to_string_pretty(response)?;
    print!("{json}");
    std::io::stdout().flush()?;
    Ok(())
}

fn emit_json_validate_workspace_config_failure(err: &WorkspaceConfigError) -> Result<()> {
    emit_json_validate_response(&JsonValidateResponse {
        schema_version: VALIDATE_JSON_SCHEMA_VERSION,
        status: "invalid",
        errors: vec![workspace_config_error_to_json_entry(err)],
        warnings: vec![],
        plan_id: None,
        computed_impact: None,
        acceptance_closure: None,
    })?;
    std::process::exit(1);
}

fn emit_json_status_workspace_config_failure(err: &WorkspaceConfigError) -> Result<()> {
    emit_json_status_response(&JsonStatusResponse {
        schema_version: STATUS_JSON_SCHEMA_VERSION,
        roots: vec![],
        units: vec![],
        loader_errors: vec![workspace_config_error_to_json_entry(err)],
    })?;
    std::process::exit(1);
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
    gate_root: &'a Path,
    specs: &'a [LoadedSpec],
    gate_specs: &'a [LoadedSpec],
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
    #[command(about = "Validate and export .plan.spec files")]
    Plan(PlanArgs),
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
                let context = load_workspace_context(&args.path)?;
                build_command(
                    &args.path,
                    args.output.as_deref(),
                    args.crate_root.as_deref(),
                    &context,
                )
            }
            Self::Test(args) => {
                let context = load_workspace_context(&args.path)?;
                test_command(
                    &args.path,
                    args.output.as_deref(),
                    args.crate_root.as_deref(),
                    &context,
                )
            }
            Self::Export(args) => export_command(&args.path, args.output.as_deref()),
            Self::Plan(args) => match args.command {
                PlanCommand::Validate(args) => plan_validate_command(&args.path, args.format),
                PlanCommand::Export(args) => {
                    plan_export_command(&args.path, args.output.as_deref())
                }
            },
            Self::Completions(_) => unreachable!("handled in main"),
        }
    }
}

#[derive(Args, Debug)]
pub struct PlanArgs {
    #[command(subcommand)]
    pub command: PlanCommand,
}

#[derive(Subcommand, Debug)]
pub enum PlanCommand {
    #[command(about = "Validate one .plan.spec file and compute derived impact")]
    Validate(PlanValidateArgs),
    #[command(about = "Export one .plan.spec file as a dedicated JSON bundle")]
    Export(PlanExportArgs),
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
    #[arg(value_name = "PATH", help = "Directory containing .unit.spec files")]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Output directory for generated Rust files (default: {crate_root}/src/generated)"
    )]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(value_name = "PATH", help = "Directory containing .unit.spec files")]
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
        help = "Directory containing .unit.spec files, a single .unit.spec file, or a single .test.spec file"
    )]
    pub path: PathBuf,
    #[arg(
        long,
        help = "Output directory for generated Rust files (directory-scoped test only; default: {crate_root}/src/generated)"
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
pub struct PlanValidateArgs {
    #[arg(value_name = "FILE", help = "Path to a single .plan.spec file")]
    pub path: PathBuf,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Args, Debug)]
pub struct PlanExportArgs {
    #[arg(value_name = "FILE", help = "Path to a single .plan.spec file")]
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
    let context = match load_workspace_context(path) {
        Ok(context) => context,
        Err(err) if matches!(format, OutputFormat::Json) => {
            if let Some(config_err) = err.downcast_ref::<WorkspaceConfigError>() {
                return emit_json_validate_workspace_config_failure(config_err);
            }
            return Err(err);
        }
        Err(err) => return Err(err),
    };
    let validation_specs = collect_validation_specs(path, &context)?;
    let config = context.config.clone();
    let validation_options = ValidationOptions {
        strict_deps: !no_strict,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (mut validation_errors, validation_warnings) =
        finish_validation_with_imports(&validation_specs, &validation_options);
    validation_errors.extend(validate_library_crate_aliases(
        validation_specs.local_specs(),
        path,
        &context,
    ));

    let (molecule_errors, molecule_warnings, molecule_loader_errors) =
        if includes_directory_molecule_tests(path) {
            let molecule_report = load_molecule_test_directory_report(path);
            let (errors, warnings) =
                validate_molecule_tests(&molecule_report.tests, &validation_specs.root_specs);
            (errors, warnings, molecule_report.errors)
        } else {
            (Vec::new(), Vec::new(), Vec::new())
        };

    match format {
        OutputFormat::Text => {
            let mut errors = DiagnosticMap::new();
            let mut warnings = DiagnosticMap::new();

            for err in validation_specs.loader_errors {
                push_error(&mut errors, err);
            }
            for err in validation_errors {
                push_error(&mut errors, err);
            }
            for err in molecule_loader_errors {
                push_error(&mut errors, err);
            }
            for err in molecule_errors {
                push_error(&mut errors, err);
            }
            for warning in validation_specs.loader_warnings {
                push_warning(&mut warnings, warning);
            }
            for warning in validation_warnings {
                push_warning(&mut warnings, warning);
            }
            for warning in molecule_warnings {
                push_warning(&mut warnings, warning);
            }

            let warning_count = count_messages(&warnings);

            if !warnings.is_empty() {
                print_diagnostics(&warnings);
            }

            if errors.is_empty() {
                if validation_specs.total_files == 0 {
                    println!("0 units found, nothing to validate.");
                } else {
                    println!(
                        "✅ {} unit{} valid{}",
                        validation_specs.total_files,
                        pluralize(validation_specs.total_files),
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
            let id_by_path: HashMap<String, String> = validation_specs
                .all_specs()
                .into_iter()
                .map(|s| (s.source.file_path.clone(), s.spec.id.clone()))
                .collect();
            let mut errors = Vec::with_capacity(
                validation_specs.loader_errors.len()
                    + validation_errors.len()
                    + molecule_loader_errors.len()
                    + molecule_errors.len(),
            );
            errors.extend(
                validation_specs
                    .loader_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );
            errors.extend(
                validation_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );
            errors.extend(
                molecule_loader_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );
            errors.extend(
                molecule_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, &id_by_path)),
            );

            let warnings = validation_specs
                .loader_warnings
                .into_iter()
                .chain(validation_warnings)
                .chain(molecule_warnings)
                .map(|warning| spec_warning_to_json_entry(&warning))
                .collect();
            let has_errors = !errors.is_empty();

            let response = JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: if has_errors { "invalid" } else { "valid" },
                errors,
                warnings,
                plan_id: None,
                computed_impact: None,
                acceptance_closure: None,
            };
            emit_json_validate_response(&response)?;

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
    freshness: Option<&PassportFreshness>,
) -> HealthStatus {
    // 1. invalid
    if !errors.is_empty() {
        return HealthStatus {
            status: HealthState::Invalid,
            reason: None,
            evidence_at: None,
        };
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
                let n = ev
                    .test_results
                    .iter()
                    .filter(|r| r.status == "fail")
                    .count();
                format!("{} test{} failed", n, pluralize(n))
            };
            return HealthStatus {
                status: HealthState::Failing,
                reason: Some(reason),
                evidence_at,
            };
        }
    }

    // 3. stale — authored/backend freshness changed since last test.
    if passport.is_some()
        && let Some(reason) = freshness_stale_reason(freshness)
    {
        return HealthStatus {
            status: HealthState::Stale,
            reason: Some(reason),
            evidence_at,
        };
    }

    // 4. incomplete — unobserved tests (requires evidence)
    if let Some(ev) = evidence {
        let unknown_count = ev
            .test_results
            .iter()
            .filter(|r| r.status == "unknown")
            .count();
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
    HealthStatus {
        status: HealthState::Valid,
        reason: None,
        evidence_at,
    }
}

fn apply_escape_hatch_gate_to_health(
    mut health: HealthStatus,
    gate: Option<&EscapeHatchGate>,
) -> HealthStatus {
    if health.status == HealthState::Valid
        && let Some(gate) = gate
        && gate.status == EscapeHatchGateStatus::Open
    {
        health.status = HealthState::Incomplete;
        health.reason = gate.reason.clone();
    }
    health
}

fn apply_semantic_review_to_health(
    mut health: HealthStatus,
    semantic_review: Option<&SemanticReview>,
) -> HealthStatus {
    if health.status != HealthState::Valid {
        return health;
    }

    match semantic_health_effect(semantic_review) {
        SemanticHealthEffect::KeepBase => health,
        SemanticHealthEffect::DemoteIncomplete => {
            health.status = HealthState::Incomplete;
            health.reason = semantic_review.map(semantic_review_summary);
            health
        }
        SemanticHealthEffect::DemoteFailing => {
            health.status = HealthState::Failing;
            health.reason = semantic_review.map(semantic_review_summary);
            health
        }
    }
}

fn freshness_stale_reason(freshness: Option<&PassportFreshness>) -> Option<String> {
    let freshness = freshness?;
    match (
        freshness.authored_truth_status,
        freshness.backend_execution_status,
    ) {
        (
            spec_core::passport::FreshnessStatus::Stale,
            spec_core::passport::FreshnessStatus::Stale,
        ) => Some("authored truth and backend execution changed since last test".to_string()),
        (spec_core::passport::FreshnessStatus::Stale, _) => {
            Some("authored truth changed since last test".to_string())
        }
        (_, spec_core::passport::FreshnessStatus::Stale) => {
            Some("backend execution changed since last test".to_string())
        }
        _ => None,
    }
}

fn plan_validate_has_failures(report: &spec_core::plan::PlanReport) -> bool {
    matches!(
        report.acceptance_closure.status,
        PlanAcceptanceClosureStatus::Open
    )
}

fn compute_molecule_health_status(
    errors: &[JsonErrorEntry],
    evidence: Option<&MoleculeEvidence>,
    test: &LoadedMoleculeTest,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> HealthStatus {
    if !errors.is_empty() {
        return HealthStatus {
            status: HealthState::Invalid,
            reason: Some(format!("{} error{}", errors.len(), pluralize(errors.len()))),
            evidence_at: None,
        };
    }

    let Some(evidence) = evidence else {
        return HealthStatus {
            status: HealthState::Untested,
            reason: Some("no molecule evidence".to_string()),
            evidence_at: None,
        };
    };

    let evidence_at = Some(evidence.observed_at.clone());
    if matches!(evidence.status, MoleculeEvidenceStatus::Stale)
        || molecule_evidence_is_stale(evidence, test, specs_by_id)
    {
        return HealthStatus {
            status: HealthState::Stale,
            reason: Some(evidence.reason.clone().unwrap_or_else(|| {
                "covered unit contract changed since last molecule test".to_string()
            })),
            evidence_at,
        };
    }
    if molecule_evidence_is_current_pass(evidence, test, specs_by_id) {
        return HealthStatus {
            status: HealthState::Valid,
            reason: None,
            evidence_at,
        };
    }

    match evidence.status {
        MoleculeEvidenceStatus::BuildFail => HealthStatus {
            status: HealthState::Failing,
            reason: Some(
                evidence
                    .reason
                    .clone()
                    .unwrap_or_else(|| "build failed".to_string()),
            ),
            evidence_at,
        },
        MoleculeEvidenceStatus::Timeout => HealthStatus {
            status: HealthState::Failing,
            reason: Some(
                evidence
                    .reason
                    .clone()
                    .unwrap_or_else(|| "build timed out".to_string()),
            ),
            evidence_at,
        },
        MoleculeEvidenceStatus::Fail => HealthStatus {
            status: HealthState::Failing,
            reason: Some(
                evidence
                    .reason
                    .clone()
                    .unwrap_or_else(|| "molecule test failed".to_string()),
            ),
            evidence_at,
        },
        MoleculeEvidenceStatus::Unknown => HealthStatus {
            status: HealthState::Incomplete,
            reason: Some(
                evidence
                    .reason
                    .clone()
                    .unwrap_or_else(|| "molecule test result unknown".to_string()),
            ),
            evidence_at,
        },
        MoleculeEvidenceStatus::Pass => unreachable!("current pass handled above"),
        MoleculeEvidenceStatus::Stale => unreachable!("handled above"),
    }
}

#[derive(Clone)]
struct StatusRootScope {
    collection_path: PathBuf,
    library_root: Option<PathBuf>,
    display_root: String,
    target_molecule_path: Option<PathBuf>,
}

struct ResolvedStatusScopes {
    scopes: Vec<StatusRootScope>,
    loader_errors: Vec<spec_core::SpecError>,
}

fn resolve_status_roots(path: &Path, context: &WorkspaceContext) -> Result<ResolvedStatusScopes> {
    let absolute_path = absolutize_from_current_dir(path)?;

    if absolute_path.is_file() {
        if is_unit_spec(&absolute_path) {
            let library_root = resolve_unit_library_root(&absolute_path, context);
            return Ok(ResolvedStatusScopes {
                scopes: vec![StatusRootScope {
                    collection_path: absolute_path.clone(),
                    library_root,
                    display_root: ".".to_string(),
                    target_molecule_path: None,
                }],
                loader_errors: Vec::new(),
            });
        }

        if is_molecule_test_spec(&absolute_path) {
            let library_root = resolve_molecule_test_library_root(&absolute_path, context);
            return Ok(ResolvedStatusScopes {
                scopes: vec![StatusRootScope {
                    collection_path: library_root
                        .clone()
                        .unwrap_or_else(|| absolute_path.clone()),
                    library_root,
                    display_root: ".".to_string(),
                    target_molecule_path: Some(absolute_path.clone()),
                }],
                loader_errors: Vec::new(),
            });
        }

        bail!(
            "{} is not a .unit.spec or .test.spec file",
            absolute_path.display()
        );
    }

    if absolute_path.join("units").is_dir() {
        let root = canonicalize_existing_dir(&absolute_path)?;
        return Ok(ResolvedStatusScopes {
            scopes: vec![StatusRootScope {
                collection_path: root.clone(),
                library_root: Some(root.clone()),
                display_root: ".".to_string(),
                target_molecule_path: None,
            }],
            loader_errors: Vec::new(),
        });
    }

    if absolute_path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "units")
    {
        let root = canonicalize_existing_dir(absolute_path.parent().unwrap_or(&absolute_path))?;
        return Ok(ResolvedStatusScopes {
            scopes: vec![StatusRootScope {
                collection_path: root.clone(),
                library_root: Some(root.clone()),
                display_root: ".".to_string(),
                target_molecule_path: None,
            }],
            loader_errors: Vec::new(),
        });
    }

    let search_root = canonicalize_existing_dir(&absolute_path)?;
    let discovery = discover_library_roots_bounded(&search_root, &search_root)?;
    let scopes = discovery
        .roots
        .into_iter()
        .map(|root| {
            let relative = root
                .strip_prefix(&search_root)
                .ok()
                .map(Path::to_path_buf)
                .filter(|relative| !relative.as_os_str().is_empty())
                .map(|relative| relative.display().to_string())
                .unwrap_or_else(|| ".".to_string());
            Ok(StatusRootScope {
                collection_path: root.clone(),
                library_root: Some(root.clone()),
                display_root: relative,
                target_molecule_path: None,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(ResolvedStatusScopes {
        scopes,
        loader_errors: discovery.errors,
    })
}

fn zero_roots_status_entry(path: &Path) -> JsonErrorEntry {
    JsonErrorEntry {
        unit: None,
        code: "SPEC_NO_LIBRARY_ROOTS".to_string(),
        path: Some(path.display().to_string()),
        dep: None,
        field: None,
        value: None,
        message: Some(format!(
            "no library roots discovered under {}",
            path.display()
        )),
        id: None,
        path2: None,
        cycle: None,
    }
}

fn status_command(path: &Path, format: OutputFormat) -> Result<()> {
    let root_context = match load_workspace_context(path) {
        Ok(context) => context,
        Err(err) if matches!(format, OutputFormat::Json) => {
            if let Some(config_err) = err.downcast_ref::<WorkspaceConfigError>() {
                return emit_json_status_workspace_config_failure(config_err);
            }
            return Err(err);
        }
        Err(err) => return Err(err),
    };
    let resolved_scopes = resolve_status_roots(path, &root_context)?;
    let scopes = resolved_scopes.scopes;

    let mut roots = Vec::new();
    let mut top_level_loader_errors: Vec<JsonErrorEntry> = resolved_scopes
        .loader_errors
        .iter()
        .map(|err| spec_error_to_json_entry(err, &HashMap::new()))
        .collect();
    let mut needs_nonzero_exit = scopes.is_empty();
    if scopes.is_empty() {
        top_level_loader_errors.push(zero_roots_status_entry(path));
    }

    for scope in scopes {
        let scope_context = match load_workspace_context(&scope.collection_path) {
            Ok(context) => context,
            Err(err) => {
                if let Some(config_err) = err.downcast_ref::<WorkspaceConfigError>() {
                    top_level_loader_errors.push(workspace_config_error_to_json_entry(config_err));
                    needs_nonzero_exit = true;
                    continue;
                }
                return Err(err);
            }
        };
        let validation_options = ValidationOptions {
            strict_deps: true,
            allow_unsafe_local_test_expect: scope_context
                .config
                .validation
                .allow_unsafe_local_test_expect,
        };
        let mut validation_specs =
            collect_validation_specs(&scope.collection_path, &scope_context)?;
        let loader_errors = std::mem::take(&mut validation_specs.loader_errors);
        let selected_libraries: Vec<ResolvedLibrary> = scope_context
            .libraries
            .iter()
            .filter(|library| {
                validation_specs
                    .imported_libraries
                    .iter()
                    .any(|imported| imported.alias == library.alias)
            })
            .cloned()
            .collect();
        let failed_import_aliases =
            imported_library_aliases_with_loader_errors(&loader_errors, &selected_libraries);
        let (validation_errors, _validation_warnings) =
            finish_validation_with_imports(&validation_specs, &validation_options);
        let mut validation_errors = suppress_cross_library_dep_not_found_for_failed_imports(
            validation_errors,
            &failed_import_aliases,
        );
        validation_errors.extend(validate_library_crate_aliases(
            validation_specs.local_specs(),
            &scope.collection_path,
            &scope_context,
        ));

        let molecule_report = if let Some(target_molecule_path) = &scope.target_molecule_path {
            let test = load_molecule_test_file(target_molecule_path)
                .with_context(|| format!("Failed to load {}", target_molecule_path.display()))?;
            spec_core::loader::MoleculeTestLoadReport {
                tests: vec![test],
                ..Default::default()
            }
        } else if includes_directory_molecule_tests(&scope.collection_path) {
            let allowed_root = scope
                .library_root
                .as_deref()
                .unwrap_or(scope.collection_path.as_path());
            load_molecule_test_directory_report_bounded(&scope.collection_path, allowed_root)?
        } else {
            spec_core::loader::MoleculeTestLoadReport::default()
        };
        let mut molecule_report = molecule_report;
        validation_errors.extend(std::mem::take(&mut molecule_report.errors));
        let (molecule_errors, _molecule_warnings) =
            validate_molecule_tests(&molecule_report.tests, &validation_specs.root_specs);
        validation_errors.extend(molecule_errors);

        let id_by_path: HashMap<String, String> = validation_specs
            .all_specs()
            .into_iter()
            .map(|spec| (spec.source.file_path.clone(), spec.spec.id.clone()))
            .chain(
                molecule_report
                    .tests
                    .iter()
                    .map(|test| (test.source.file_path.clone(), test.test.id.clone())),
            )
            .collect();
        let unit_paths: HashSet<String> = validation_specs
            .root_specs
            .iter()
            .map(|spec| spec.source.file_path.clone())
            .collect();
        let molecule_paths: HashSet<String> = molecule_report
            .tests
            .iter()
            .map(|test| test.source.file_path.clone())
            .collect();

        let mut unit_errors_by_path: HashMap<String, Vec<JsonErrorEntry>> = HashMap::new();
        let mut molecule_errors_by_path: HashMap<String, Vec<JsonErrorEntry>> = HashMap::new();
        let mut global_errors = loader_errors;
        for err in validation_errors {
            let entry = spec_error_to_json_entry(&err, &id_by_path);
            let error_paths_for_entry = error_paths(&err);
            let mut attached = false;
            for error_path in &error_paths_for_entry {
                if unit_paths.contains(error_path) {
                    unit_errors_by_path
                        .entry(error_path.clone())
                        .or_default()
                        .push(entry.clone());
                    attached = true;
                }
                if molecule_paths.contains(error_path) {
                    molecule_errors_by_path
                        .entry(error_path.clone())
                        .or_default()
                        .push(entry.clone());
                    attached = true;
                }
            }
            if !attached {
                global_errors.push(err);
            }
        }

        top_level_loader_errors.extend(
            global_errors
                .iter()
                .map(|err| spec_error_to_json_entry(err, &id_by_path)),
        );

        let specs_by_id: HashMap<String, LoadedSpec> = validation_specs
            .root_specs
            .iter()
            .map(|spec| (spec.spec.id.clone(), spec.clone()))
            .collect();
        let molecule_evidence_by_id: HashMap<String, MoleculeEvidence> = molecule_report
            .tests
            .iter()
            .filter_map(|test| {
                read_molecule_evidence(Path::new(&test.source.file_path))
                    .ok()
                    .flatten()
                    .map(|evidence| (test.test.id.clone(), evidence))
            })
            .collect();
        let projection_context = PassportProjectionContext {
            molecule_tests: &molecule_report.tests,
            molecule_evidence_by_id: &molecule_evidence_by_id,
            specs_by_id: &specs_by_id,
            semantic_projection_mode: SemanticProjectionMode::Preserve,
        };

        let mut units = Vec::with_capacity(validation_specs.root_specs.len());
        for spec in &validation_specs.root_specs {
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
            let projected_truth =
                project_passport_truth(spec, passport.as_ref(), &projection_context);
            let freshness = projected_truth.freshness.clone();
            let markers = projected_truth.markers.clone();
            let escape_hatch_gate = projected_truth.escape_hatch_gate.clone();
            let semantic_review = projected_truth.semantic_review.clone();
            let errors = unit_errors_by_path
                .remove(&spec.source.file_path)
                .unwrap_or_default();
            let health = apply_semantic_review_to_health(
                apply_escape_hatch_gate_to_health(
                    compute_health_status(&errors, passport.as_ref(), freshness.as_ref()),
                    escape_hatch_gate.as_ref(),
                ),
                semantic_review.as_ref(),
            );
            if !health.status.is_valid() {
                needs_nonzero_exit = true;
            }

            units.push(JsonStatusUnit {
                id: spec.spec.id.clone(),
                status: health.status,
                reason: health.reason,
                errors,
                evidence_at: health.evidence_at,
                freshness,
                markers,
                escape_hatch_gate,
                semantic_review,
            });
        }

        let mut molecule_tests = Vec::with_capacity(molecule_report.tests.len());
        for test in &molecule_report.tests {
            let errors = molecule_errors_by_path
                .remove(&test.source.file_path)
                .unwrap_or_default();
            let evidence = match read_molecule_evidence(Path::new(&test.source.file_path)) {
                Ok(evidence) => evidence,
                Err(err) => {
                    let mut errors = errors;
                    errors.push(spec_error_to_json_entry(&err, &id_by_path));
                    let health = compute_molecule_health_status(&errors, None, test, &specs_by_id);
                    needs_nonzero_exit = true;
                    molecule_tests.push(JsonStatusMoleculeTest {
                        id: test.test.id.clone(),
                        status: health.status,
                        reason: health.reason,
                        errors,
                        evidence_at: health.evidence_at,
                        freshness: None,
                        markers: None,
                        escape_hatch_gate: None,
                        semantic_review: None,
                    });
                    continue;
                }
            };
            let health =
                compute_molecule_health_status(&errors, evidence.as_ref(), test, &specs_by_id);
            if !health.status.is_valid() {
                needs_nonzero_exit = true;
            }
            molecule_tests.push(JsonStatusMoleculeTest {
                id: test.test.id.clone(),
                status: health.status,
                reason: health.reason,
                errors,
                evidence_at: health.evidence_at,
                freshness: None,
                markers: None,
                escape_hatch_gate: None,
                semantic_review: None,
            });
        }

        roots.push(JsonStatusRoot {
            root: scope.display_root,
            units,
            molecule_tests,
        });
    }

    let has_top_level_errors = !top_level_loader_errors.is_empty();

    match format {
        OutputFormat::Text => {
            if !top_level_loader_errors.is_empty() {
                let mut diagnostics = DiagnosticMap::new();
                for error in &top_level_loader_errors {
                    diagnostics
                        .entry(error.path.clone().unwrap_or_else(|| "<global>".to_string()))
                        .or_default()
                        .push(json_error_entry_to_human(error));
                }
                for (path, messages) in diagnostics {
                    eprintln!("{path}");
                    for message in messages {
                        eprintln!("  · {message}");
                    }
                }
            }
            if roots.is_empty() {
                eprintln!("✗ no library roots discovered under {}", path.display());
            }
            for root in &roots {
                println!("Root: {}", root.root);
                println!("UNITS");
                for unit in &root.units {
                    print_status_unit(unit);
                }
                if !root.molecule_tests.is_empty() {
                    println!("MOLECULE TESTS");
                    for molecule_test in &root.molecule_tests {
                        print_status_unit(molecule_test);
                    }
                }
            }
        }
        OutputFormat::Json => {
            let flat_units = roots
                .iter()
                .flat_map(|root| root.units.iter().cloned())
                .collect();
            emit_json_status_response(&JsonStatusResponse {
                schema_version: STATUS_JSON_SCHEMA_VERSION,
                roots,
                units: flat_units,
                loader_errors: top_level_loader_errors,
            })?;
        }
    }

    if needs_nonzero_exit || has_top_level_errors {
        std::process::exit(1);
    }

    Ok(())
}

fn imported_library_aliases_with_loader_errors(
    loader_errors: &[spec_core::SpecError],
    libraries: &[ResolvedLibrary],
) -> HashSet<String> {
    libraries
        .iter()
        .filter(|library| {
            loader_errors.iter().any(|err| {
                error_paths(err)
                    .into_iter()
                    .any(|path| Path::new(&path).starts_with(&library.root))
            })
        })
        .map(|library| library.alias.clone())
        .collect()
}

fn suppress_cross_library_dep_not_found_for_failed_imports(
    errors: Vec<spec_core::SpecError>,
    failed_import_aliases: &HashSet<String>,
) -> Vec<spec_core::SpecError> {
    errors
        .into_iter()
        .filter(|err| match err {
            spec_core::SpecError::CrossLibraryDepNotFound { dep, .. } => DepRef::parse(dep)
                .ok()
                .and_then(|dep_ref| dep_ref.library_alias().map(str::to_string))
                .is_none_or(|alias| !failed_import_aliases.contains(&alias)),
            _ => true,
        })
        .collect()
}

fn export_command(path: &Path, output: Option<&Path>) -> Result<()> {
    let context = load_workspace_context(path)?;
    let mut validation_specs = collect_validation_specs(path, &context)?;
    let loader_errors = std::mem::take(&mut validation_specs.loader_errors);
    let loader_warnings = std::mem::take(&mut validation_specs.loader_warnings);
    let config = context.config.clone();
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (mut validation_errors, validation_warnings) =
        finish_validation_with_imports(&validation_specs, &validation_options);
    validation_errors.extend(validate_library_crate_aliases(
        validation_specs.local_specs(),
        path,
        &context,
    ));
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

    let export_dir = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    let provenance = resolve_git_provenance(export_dir);
    let molecule_tests = if includes_directory_molecule_tests(path) {
        load_molecule_test_directory(export_dir).with_context(|| {
            format!(
                "Failed to load molecule tests from {}",
                export_dir.display()
            )
        })?
    } else {
        Vec::new()
    };

    // Validate molecule tests before including them in the export bundle.
    // export_command validates unit specs above but previously skipped molecule test validation,
    // allowing tests with unknown covers IDs or invalid bodies to silently enter the export JSON.
    let (molecule_errors, molecule_warnings) =
        validate_molecule_tests(&molecule_tests, &validation_specs.root_specs);
    for err in molecule_errors {
        push_error(&mut errors, err);
    }
    // Print molecule warnings immediately and separately — unit warnings were already printed
    // above. Adding molecule warnings to the shared map would re-print unit warnings alongside
    // molecule ones if there are molecule errors.
    if !molecule_warnings.is_empty() {
        let mut mol_warn_map = DiagnosticMap::new();
        for warning in molecule_warnings {
            push_warning(&mut mol_warn_map, warning);
        }
        print_diagnostics(&mol_warn_map);
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

    let bundle = build_export_bundle(
        &validation_specs.root_specs,
        &molecule_tests,
        &rfc3339_now(),
        provenance.as_ref(),
    );
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

fn plan_validate_command(path: &Path, format: OutputFormat) -> Result<()> {
    let context = match load_workspace_context(path) {
        Ok(context) => context,
        Err(err) if matches!(format, OutputFormat::Json) => {
            if let Some(config_err) = err.downcast_ref::<WorkspaceConfigError>() {
                return emit_json_validate_workspace_config_failure(config_err);
            }
            return Err(err);
        }
        Err(err) => return Err(err),
    };

    let (plan_path, library_root) = match resolve_plan_library_root(path, &context) {
        Ok(result) => result,
        Err(err) if matches!(format, OutputFormat::Json) => {
            emit_json_validate_response(&JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: "invalid",
                errors: vec![spec_error_to_json_entry(&err, &HashMap::new())],
                warnings: vec![],
                plan_id: None,
                computed_impact: None,
                acceptance_closure: None,
            })?;
            std::process::exit(1);
        }
        Err(err) => return Err(err.into()),
    };

    let plan = match load_plan_file(&plan_path) {
        Ok(plan) => plan,
        Err(err) if matches!(format, OutputFormat::Json) => {
            emit_json_validate_response(&JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: "invalid",
                errors: vec![spec_error_to_json_entry(&err, &HashMap::new())],
                warnings: vec![],
                plan_id: None,
                computed_impact: None,
                acceptance_closure: None,
            })?;
            std::process::exit(1);
        }
        Err(err) => return Err(err.into()),
    };

    let (validation_specs, validation_errors, warnings, molecule_tests) =
        match plan_validation_inputs(&library_root, &context) {
            Ok(inputs) => inputs,
            Err(err) if matches!(format, OutputFormat::Json) => {
                let Some(spec_err) = err.downcast_ref::<spec_core::SpecError>() else {
                    return Err(err);
                };
                emit_json_validate_response(&JsonValidateResponse {
                    schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                    status: "invalid",
                    errors: vec![spec_error_to_json_entry(spec_err, &HashMap::new())],
                    warnings: vec![],
                    plan_id: None,
                    computed_impact: None,
                    acceptance_closure: None,
                })?;
                std::process::exit(1);
            }
            Err(err) => return Err(err),
        };
    let mut id_by_path: HashMap<String, String> = validation_specs
        .all_specs()
        .into_iter()
        .map(|spec| (spec.source.file_path.clone(), spec.spec.id.clone()))
        .collect();
    id_by_path.insert(plan.source.file_path.clone(), plan.plan.id.clone());

    if !validation_errors.is_empty() {
        return emit_plan_validate_failure(validation_errors, warnings, &id_by_path, format);
    }

    let report = match build_plan_report(&plan, &validation_specs.root_specs, &molecule_tests) {
        Ok(report) => report,
        Err(err) if matches!(format, OutputFormat::Json) => {
            emit_json_validate_response(&JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: "invalid",
                errors: vec![spec_error_to_json_entry(&err, &id_by_path)],
                warnings: warnings.iter().map(spec_warning_to_json_entry).collect(),
                plan_id: None,
                computed_impact: None,
                acceptance_closure: None,
            })?;
            std::process::exit(1);
        }
        Err(err) => return Err(err.into()),
    };
    let has_plan_failures = plan_validate_has_failures(&report);

    match format {
        OutputFormat::Text => {
            if has_plan_failures {
                println!("❌ plan '{}' invalid", report.plan_id);
            } else {
                println!("✅ plan '{}' valid", report.plan_id);
            }
            println!(
                "computed impact: {} unit{}, {} molecule test{}, {} unresolved",
                report.computed_impact.units.len(),
                pluralize(report.computed_impact.units.len()),
                report.computed_impact.molecule_tests.len(),
                pluralize(report.computed_impact.molecule_tests.len()),
                report.computed_impact.unresolved.len()
            );
            if !report.acceptance_closure.missing_validate.is_empty() {
                println!(
                    "missing validate coverage: {}",
                    report.acceptance_closure.missing_validate.join(", ")
                );
            }
            if !report.acceptance_closure.missing_molecule_tests.is_empty() {
                println!(
                    "missing molecule coverage: {}",
                    report.acceptance_closure.missing_molecule_tests.join(", ")
                );
            }
            if !report.acceptance_closure.extra_validate.is_empty() {
                println!(
                    "extra validate coverage: {}",
                    report.acceptance_closure.extra_validate.join(", ")
                );
            }
            if !report.acceptance_closure.extra_molecule_tests.is_empty() {
                println!(
                    "extra molecule coverage: {}",
                    report.acceptance_closure.extra_molecule_tests.join(", ")
                );
            }
            if has_plan_failures {
                std::process::exit(1);
            }
            Ok(())
        }
        OutputFormat::Json => {
            let response = JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: if has_plan_failures {
                    "invalid"
                } else {
                    "valid"
                },
                errors: vec![],
                warnings: warnings.iter().map(spec_warning_to_json_entry).collect(),
                plan_id: Some(report.plan_id),
                computed_impact: Some(report.computed_impact),
                acceptance_closure: Some(report.acceptance_closure),
            };
            emit_json_validate_response(&response)?;
            if has_plan_failures {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

fn plan_export_command(path: &Path, output: Option<&Path>) -> Result<()> {
    let context = load_workspace_context(path)?;
    let (plan_path, library_root) = resolve_plan_library_root(path, &context)?;
    let plan = load_plan_file(&plan_path)?;
    let (validation_specs, validation_errors, warnings, molecule_tests) =
        plan_validation_inputs(&library_root, &context)?;

    if !validation_errors.is_empty() {
        let mut diagnostics = DiagnosticMap::new();
        for err in validation_errors {
            push_error(&mut diagnostics, err);
        }
        print_diagnostics(&diagnostics);
        let file_count = count_unique_files(&diagnostics);
        bail!(
            "❌ {} file{}, {} error{}",
            file_count,
            pluralize(file_count),
            count_messages(&diagnostics),
            pluralize(count_messages(&diagnostics))
        );
    }

    let report = build_plan_report(&plan, &validation_specs.root_specs, &molecule_tests)?;
    let mut bundle = build_plan_export_bundle(&plan, &report, &rfc3339_now());
    bundle.warnings = warnings
        .into_iter()
        .map(|warning| warning.to_string())
        .collect();
    let json = serde_json::to_string_pretty(&bundle)?;

    match output {
        Some(path) => {
            validate_export_output_path(path)?;
            fs::write(path, json)
                .with_context(|| format!("Failed to write export bundle to {}", path.display()))?;
        }
        None => print!("{json}"),
    }

    Ok(())
}

fn emit_plan_validate_failure(
    validation_errors: Vec<spec_core::SpecError>,
    warnings: Vec<spec_core::SpecWarning>,
    id_by_path: &HashMap<String, String>,
    format: OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Text => {
            let mut diagnostics = DiagnosticMap::new();
            for err in validation_errors {
                push_error(&mut diagnostics, err);
            }
            print_diagnostics(&diagnostics);
            let file_count = count_unique_files(&diagnostics);
            bail!(
                "❌ {} file{}, {} error{}",
                file_count,
                pluralize(file_count),
                count_messages(&diagnostics),
                pluralize(count_messages(&diagnostics))
            );
        }
        OutputFormat::Json => {
            let response = JsonValidateResponse {
                schema_version: VALIDATE_JSON_SCHEMA_VERSION,
                status: "invalid",
                errors: validation_errors
                    .iter()
                    .map(|err| spec_error_to_json_entry(err, id_by_path))
                    .collect(),
                warnings: warnings.iter().map(spec_warning_to_json_entry).collect(),
                plan_id: None,
                computed_impact: None,
                acceptance_closure: None,
            };
            emit_json_validate_response(&response)?;
            std::process::exit(1);
        }
    }
}

fn generate_command(path: &Path, output: Option<&Path>) -> Result<()> {
    if path.is_file() {
        bail!(
            "❌ spec generate requires a directory path — pass the units directory, not a single file"
        );
    }

    let context = load_workspace_context(path)?;
    let mut validation_specs = collect_validation_specs(path, &context)?;
    let loader_errors = std::mem::take(&mut validation_specs.loader_errors);
    let loader_warnings = std::mem::take(&mut validation_specs.loader_warnings);
    if !loader_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in loader_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ unable to load units before generation");
    }
    if !loader_warnings.is_empty() {
        let mut warnings = DiagnosticMap::new();
        for warning in loader_warnings {
            push_warning(&mut warnings, warning);
        }
        print_diagnostics(&warnings);
    }
    let alias_errors =
        validate_library_crate_aliases(validation_specs.local_specs(), path, &context);
    if !alias_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in alias_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ cross-library crate alias validation failed");
    }

    let spec_root = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    let explicit_output = output.map(PathBuf::from);
    let crate_root = match explicit_output {
        Some(_) => resolve_default_crate_root(spec_root, &context).ok(),
        None => Some(resolve_default_crate_root(spec_root, &context)?),
    };
    let project_root = context
        .repo_root
        .clone()
        .or_else(|| context.workspace_root.clone())
        .or_else(|| crate_root.clone())
        .unwrap_or(absolutize_from_current_dir(Path::new("."))?);
    let resolved_output: PathBuf = explicit_output.unwrap_or_else(|| {
        crate_root
            .expect("missing default crate root")
            .join("src/generated")
    });
    let generated = generate_specs(path, &resolved_output, &project_root)?;
    if !generated.specs.is_empty() {
        finalize_passports(
            &PassportWritePlan {
                passport_root: spec_root,
                gate_root: spec_root,
                specs: &generated.specs,
                gate_specs: &generated.specs,
            },
            &generated.generated_at,
            None,
            None,
            None,
            false,
        )?;
    }
    Ok(())
}

fn generate_specs(path: &Path, output: &Path, project_root: &Path) -> Result<GeneratedSpecs> {
    let context = load_workspace_context(path)?;
    let mut validation_specs = collect_validation_specs(path, &context)?;
    let specs: Vec<LoadedSpec> = validation_specs
        .local_specs()
        .into_iter()
        .cloned()
        .collect();
    let total_files = validation_specs.total_files;
    let loader_errors = std::mem::take(&mut validation_specs.loader_errors);
    let loader_warnings = std::mem::take(&mut validation_specs.loader_warnings);
    if total_files == 0 {
        let mut errors = DiagnosticMap::new();
        let mut warnings = DiagnosticMap::new();
        for err in loader_errors {
            push_error(&mut errors, err);
        }
        for warning in loader_warnings {
            push_warning(&mut warnings, warning);
        }

        let mut has_molecule_tests = false;
        if includes_directory_molecule_tests(path) {
            let molecule_report = load_molecule_test_directory_report(path);
            has_molecule_tests =
                !molecule_report.tests.is_empty() || !molecule_report.errors.is_empty();

            let (molecule_errors, molecule_warnings) =
                validate_molecule_tests(&molecule_report.tests, &specs);

            for err in molecule_report.errors {
                push_error(&mut errors, err);
            }
            for err in molecule_errors {
                push_error(&mut errors, err);
            }
            for warning in molecule_report.warnings {
                push_warning(&mut warnings, warning);
            }
            for warning in molecule_warnings {
                push_warning(&mut warnings, warning);
            }
        }

        let output_base = ensure_output_marker(output, project_root)?;
        let generated_rs_rel_paths = HashSet::<PathBuf>::new();
        clean_output_dir(&output_base, &generated_rs_rel_paths, project_root).with_context(
            || format!("Failed to clean output directory {}", output_base.display()),
        )?;

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

        if has_molecule_tests {
            bail!("❌ 0 unit specs found; molecule tests require unit specs to validate covers");
        }

        println!("0 units found, nothing to generate.");
        return Ok(GeneratedSpecs {
            specs,
            generated_at: rfc3339_now(),
        });
    }

    let config = context.config;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (validation_errors, validation_warnings) =
        finish_validation_with_imports(&validation_specs, &validation_options);
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

    let mut normalized_units = Vec::new();
    for spec in &specs {
        let unit = normalize_unit(spec.spec.clone())
            .with_context(|| format!("Failed to normalize {}", spec.source.file_path))?;
        normalized_units.push(unit);
    }

    let molecule_tests = if includes_directory_molecule_tests(path) {
        let spec_dir = path;
        load_molecule_test_directory(spec_dir)
            .with_context(|| format!("Failed to load molecule tests from {}", spec_dir.display()))?
    } else {
        Vec::new()
    };

    // Validate molecule tests before generating code from them so module trees and
    // molecule_tests.rs files are derived from the same validated input set.
    let (mol_errors, mol_warnings) = validate_molecule_tests(&molecule_tests, &specs);
    if !mol_warnings.is_empty() {
        let mut warn_map = DiagnosticMap::new();
        for w in mol_warnings {
            push_warning(&mut warn_map, w);
        }
        print_diagnostics(&warn_map);
    }
    if !mol_errors.is_empty() {
        let mut err_map = DiagnosticMap::new();
        for e in mol_errors {
            push_error(&mut err_map, e);
        }
        print_diagnostics(&err_map);
        let file_count = count_unique_files(&err_map);
        bail!(
            "❌ {} file{}, {} error{}",
            file_count,
            pluralize(file_count),
            count_messages(&err_map),
            pluralize(count_messages(&err_map))
        );
    }

    let resolved_molecule_tests: Vec<ResolvedMoleculeTest> = molecule_tests
        .iter()
        .map(ResolvedMoleculeTest::from_loaded)
        .collect();

    let mut generated_rs_rel_paths = HashSet::<PathBuf>::new();
    for unit in &normalized_units {
        generated_rs_rel_paths.insert(path_for_unit(unit));
    }

    // Include every generated mod.rs (root + nested modules) in the owned set.
    let namespaces = build_namespaces(&normalized_units, &resolved_molecule_tests);
    for module_path in namespaces.keys() {
        let mod_rs_rel = if module_path.is_empty() {
            PathBuf::from("mod.rs")
        } else {
            PathBuf::from(module_path.replace('/', std::path::MAIN_SEPARATOR_STR)).join("mod.rs")
        };
        generated_rs_rel_paths.insert(mod_rs_rel);
    }

    let output_base = ensure_output_marker(output, project_root)?;
    let generate_options = GenerateOptions {
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };

    for unit in &normalized_units {
        let content = generate_unit_code_with_options(unit, &generate_options)
            .with_context(|| format!("Failed to generate Rust for {}", unit.id()))?;
        let output_path = output_base.join(path_for_unit(unit));
        write_generated_file(&output_path.display().to_string(), &content)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;
    }

    for (module_path, namespace) in &namespaces {
        let content = generate_mod_rs(
            &namespace.unit_files.iter().cloned().collect::<Vec<_>>(),
            &namespace.subdirs.iter().cloned().collect::<Vec<_>>(),
            namespace.has_molecule_tests,
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
    let units_by_id: HashMap<&str, &NormalizedUnit> = normalized_units
        .iter()
        .map(|unit| (unit.id(), unit))
        .collect();
    let molecule_test_paths =
        generate_and_write_molecule_tests(&resolved_molecule_tests, &units_by_id, &output_base)
            .with_context(|| "Failed to generate molecule test files")?;
    let molecule_test_file_count = molecule_test_paths.len();
    generated_rs_rel_paths.extend(molecule_test_paths);

    clean_output_dir(&output_base, &generated_rs_rel_paths, project_root)
        .with_context(|| format!("Failed to clean output directory {}", output_base.display()))?;

    let generated_at = rfc3339_now();

    println!(
        "Generated {} file{}",
        normalized_units.len() + namespaces.len() + molecule_test_file_count,
        pluralize(normalized_units.len() + namespaces.len() + molecule_test_file_count)
    );
    Ok(GeneratedSpecs {
        specs,
        generated_at,
    })
}

fn finalize_passports(
    plan: &PassportWritePlan<'_>,
    generated_at: &str,
    evidence_by_spec: Option<&BTreeMap<String, PassportEvidence>>,
    contract_hash_by_spec: Option<&BTreeMap<String, String>>,
    molecule_evidence_overrides: Option<&BTreeMap<String, MoleculeEvidence>>,
    refresh_semantic_review: bool,
) -> Result<()> {
    if plan.specs.is_empty() {
        return Ok(());
    }

    let _writer_guard = ConcurrentPassportWriteGuard::begin(plan.passport_root);
    let gate_context = build_live_escape_hatch_context(
        plan.gate_root,
        plan.gate_specs,
        molecule_evidence_overrides,
    );
    write_passports(
        plan.specs,
        generated_at,
        evidence_by_spec,
        contract_hash_by_spec,
        &gate_context,
        refresh_semantic_review,
    )?;
    ensure_gitignore_entry(plan.passport_root)
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
    gate_context: &LiveEscapeHatchContext,
    refresh_semantic_review: bool,
) -> Result<()> {
    let projection_context = PassportProjectionContext {
        molecule_tests: &gate_context.molecule_tests,
        molecule_evidence_by_id: &gate_context.molecule_evidence_by_id,
        specs_by_id: &gate_context.specs_by_id,
        semantic_projection_mode: if refresh_semantic_review {
            SemanticProjectionMode::Refresh
        } else {
            SemanticProjectionMode::Preserve
        },
    };
    for spec in specs {
        let source_path = Path::new(&spec.source.file_path);

        let mut passport = if evidence_by_spec.is_none() {
            // Non-test caller (spec generate / spec build): preserve any evidence
            // and freshness anchors already on disk so we don't erase data written by
            // a prior `spec test` run. If no baseline hash exists yet (fresh
            // project or first generate), bootstrap one from the current
            // authored truth so legacy stale detection still works before the
            // first `spec test` run.
            let existing = read_passport(source_path).ok().flatten();
            let contract_hash = existing
                .as_ref()
                .and_then(|passport| passport.contract_hash.clone())
                .or_else(|| compute_contract_hash(spec));
            build_passport_preserving_proof_state(
                spec,
                generated_at,
                existing.as_ref(),
                contract_hash,
            )
        } else {
            // Test caller: always use freshly-computed values (None is correct for
            // specs that have no contract).
            let evidence = evidence_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned();
            let contract_hash = contract_hash_by_spec
                .and_then(|map| map.get(&spec.spec.id))
                .cloned();
            build_passport_with_evidence(spec, generated_at, evidence, contract_hash)
        };
        let projected_truth = project_passport_truth(spec, Some(&passport), &projection_context);
        apply_projected_passport_truth(&mut passport, projected_truth);
        write_passport(&passport, source_path)
            .with_context(|| format!("Failed to write passport for {}", spec.source.id))?;
    }

    Ok(())
}

struct LiveEscapeHatchContext {
    molecule_tests: Vec<LoadedMoleculeTest>,
    molecule_evidence_by_id: HashMap<String, MoleculeEvidence>,
    specs_by_id: HashMap<String, LoadedSpec>,
}

fn build_live_escape_hatch_context(
    gate_root: &Path,
    gate_specs: &[LoadedSpec],
    molecule_evidence_overrides: Option<&BTreeMap<String, MoleculeEvidence>>,
) -> LiveEscapeHatchContext {
    let molecule_tests = load_molecule_test_directory(gate_root).unwrap_or_default();
    let mut molecule_evidence_by_id: HashMap<String, MoleculeEvidence> = molecule_tests
        .iter()
        .filter_map(|test| {
            read_molecule_evidence(Path::new(&test.source.file_path))
                .ok()
                .flatten()
                .map(|evidence| (test.test.id.clone(), evidence))
        })
        .collect();
    if let Some(overrides) = molecule_evidence_overrides {
        molecule_evidence_by_id.extend(
            overrides
                .iter()
                .map(|(id, evidence)| (id.clone(), evidence.clone())),
        );
    }

    LiveEscapeHatchContext {
        molecule_tests,
        molecule_evidence_by_id,
        specs_by_id: gate_specs
            .iter()
            .map(|spec| (spec.spec.id.clone(), spec.clone()))
            .collect(),
    }
}

struct PipelineContext {
    crate_root: PathBuf,
    project_root: PathBuf,
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
    context: &WorkspaceContext,
) -> Result<PipelineContext> {
    let crate_root = match crate_root_flag {
        Some(path) => absolutize_from_current_dir(path)?,
        None => resolve_default_crate_root(path, context)?,
    };
    let crate_root = canonicalize_existing_dir(&crate_root)?;
    let project_root = resolve_project_root(context, &crate_root);

    let mut temp_dir: Option<tempfile::TempDir> = None;
    let cargo_target_dir = if let Some(path) = context.pipeline_cargo_target_dir() {
        path
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
        project_root,
        cargo_target_dir,
        timeout: context
            .config
            .pipeline
            .timeout_secs
            .map(Duration::from_secs),
        _temp_dir: temp_dir,
    })
}

fn absolutize_from_current_dir(path: &Path) -> Result<PathBuf> {
    Ok(if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("failed to resolve current working directory")?
            .join(path)
    })
}

fn resolve_default_crate_root(path: &Path, context: &WorkspaceContext) -> Result<PathBuf> {
    match context.pipeline_crate_root() {
        Some(path) => canonicalize_existing_dir(&path),
        None => workspace_root_for(path),
    }
}

fn resolve_plan_library_root(
    path: &Path,
    context: &WorkspaceContext,
) -> std::result::Result<(PathBuf, PathBuf), spec_core::SpecError> {
    let absolute_path =
        absolutize_from_current_dir(path).map_err(|err| spec_core::SpecError::Traversal {
            message: err.to_string(),
            path: path.display().to_string(),
        })?;

    if absolute_path.is_dir() {
        return Err(spec_core::SpecError::PlanDirectoryInput {
            path: absolute_path.display().to_string(),
        });
    }

    let canonical_plan =
        absolute_path
            .canonicalize()
            .map_err(|err| spec_core::SpecError::Traversal {
                message: err.to_string(),
                path: absolute_path.display().to_string(),
            })?;

    let repo_root = context
        .repo_root
        .clone()
        .or_else(|| repo_root_for(&absolute_path));

    if let Some(repo_root) = &repo_root {
        if absolute_path.starts_with(repo_root) && !canonical_plan.starts_with(repo_root) {
            return Err(spec_core::SpecError::PlanSymlinkEscape {
                path: absolute_path.display().to_string(),
            });
        }

        if !canonical_plan.starts_with(repo_root) {
            return Err(spec_core::SpecError::PlanOutsideLibraryRoot {
                path: canonical_plan.display().to_string(),
            });
        }
    }

    let mut current = canonical_plan.parent();
    while let Some(dir) = current {
        if let Some(repo_root) = &repo_root
            && !dir.starts_with(repo_root)
        {
            break;
        }

        if dir.join("units").is_dir() {
            return Ok((canonical_plan.clone(), dir.to_path_buf()));
        }

        current = dir.parent();
    }

    Err(spec_core::SpecError::PlanOutsideLibraryRoot {
        path: canonical_plan.display().to_string(),
    })
}

fn resolve_project_root(context: &WorkspaceContext, crate_root: &Path) -> PathBuf {
    if let Some(repo_root) = &context.repo_root {
        return repo_root.clone();
    }

    if let Some(workspace_root) = &context.workspace_root {
        return common_ancestor_path(workspace_root, crate_root)
            .unwrap_or_else(|| workspace_root.clone());
    }

    crate_root.to_path_buf()
}

fn common_ancestor_path(left: &Path, right: &Path) -> Option<PathBuf> {
    left.ancestors()
        .find(|candidate| right.starts_with(candidate))
        .map(Path::to_path_buf)
}

fn canonicalize_existing_dir(path: &Path) -> Result<PathBuf> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Failed to resolve {}", path.display()))?;
    if !canonical.is_dir() {
        bail!("{} is not a directory", canonical.display());
    }
    Ok(canonical)
}

fn build_command(
    path: &Path,
    output: Option<&Path>,
    crate_root_flag: Option<&Path>,
    context: &WorkspaceContext,
) -> Result<()> {
    if path.is_file() {
        bail!(
            "❌ spec build requires a directory path — pass the units directory, not a single file"
        );
    }

    if !cargo_available() {
        bail!("❌ cargo not found — install Rust or ensure cargo is on PATH");
    }

    let (root_specs, loader_errors, loader_warnings, _total_files) = collect_specs(path)?;
    if !loader_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in loader_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ unable to load units before build");
    }
    if !loader_warnings.is_empty() {
        let mut warnings = DiagnosticMap::new();
        for warning in loader_warnings {
            push_warning(&mut warnings, warning);
        }
        print_diagnostics(&warnings);
    }
    let alias_errors = validate_library_crate_aliases(&root_specs, path, context);
    if !alias_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in alias_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ cross-library crate alias validation failed");
    }

    let ctx = resolve_pipeline_context(path, crate_root_flag, context)?;
    let resolved_output = output
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.crate_root.join("src/generated"));

    let generated = generate_specs(path, &resolved_output, &ctx.project_root)?;
    if !generated.specs.is_empty() {
        finalize_passports(
            &PassportWritePlan {
                passport_root: path,
                gate_root: path,
                specs: &generated.specs,
                gate_specs: &generated.specs,
            },
            &generated.generated_at,
            None,
            None,
            None,
            false,
        )?;
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
    context: &WorkspaceContext,
) -> Result<()> {
    if !cargo_available() {
        bail!("❌ cargo not found — install Rust or ensure cargo is on PATH");
    }

    let mut single_file_generation_scope: Option<tempfile::TempDir> = None;
    let mut single_file_test_project_scope: Option<tempfile::TempDir> = None;
    let mut target_spec: Option<LoadedSpec> = None;
    let mut target_molecule_test: Option<LoadedMoleculeTest> = None;
    let mut molecule_evidence_root: Option<PathBuf> = None;
    let mut molecule_tests_for_evidence = Vec::<LoadedMoleculeTest>::new();
    let mut molecule_specs_by_id = HashMap::<String, LoadedSpec>::new();
    let mut gate_root = path.to_path_buf();

    let (generation_scope, pipeline_scope) = if path.is_file() {
        if output.is_some() {
            bail!(
                "❌ spec test does not accept --output for a single file — file-scoped tests use an isolated internal output tree"
            );
        }

        if is_unit_spec(path) {
            let validation_specs = collect_validation_specs(path, context)?;
            let generation_specs: Vec<LoadedSpec> = validation_specs
                .root_specs
                .iter()
                .chain(validation_specs.support_specs.iter())
                .cloned()
                .collect();
            let library_root = resolve_unit_library_root(path, context).ok_or_else(|| {
                anyhow::anyhow!("failed to resolve library root for {}", path.display())
            })?;
            gate_root = library_root.clone();
            let generation_tempdir =
                materialize_single_file_generation_scope(&library_root, &generation_specs, &[])?;
            let generation_scope = generation_tempdir.path().join("units");
            single_file_generation_scope = Some(generation_tempdir);
            target_spec = Some(
                load_file(path).with_context(|| format!("Failed to load {}", path.display()))?,
            );
            (
                generation_scope,
                path.parent().unwrap_or(path).to_path_buf(),
            )
        } else if is_molecule_test_spec(path) {
            let target_test = load_molecule_test_file(path)
                .with_context(|| format!("Failed to load {}", path.display()))?;
            let library_root =
                resolve_molecule_test_library_root(path, context).ok_or_else(|| {
                    anyhow::anyhow!("failed to resolve library root for {}", path.display())
                })?;
            gate_root = library_root.clone();
            let report = load_directory_report_bounded(&library_root, &library_root)?;
            if !report.errors.is_empty() {
                let mut errors = DiagnosticMap::new();
                for err in report.errors {
                    push_error(&mut errors, err);
                }
                print_diagnostics(&errors);
                bail!("❌ unable to load units before test");
            }
            if !report.warnings.is_empty() {
                let mut warnings = DiagnosticMap::new();
                for warning in report.warnings {
                    push_warning(&mut warnings, warning);
                }
                print_diagnostics(&warnings);
            }

            let generation_tempdir =
                materialize_single_file_generation_scope(&library_root, &report.specs, &[path])?;
            let generation_scope = generation_tempdir.path().join("units");
            single_file_generation_scope = Some(generation_tempdir);
            molecule_specs_by_id = report
                .specs
                .iter()
                .map(|spec| (spec.spec.id.clone(), spec.clone()))
                .collect();
            molecule_evidence_root = Some(library_root.join("units"));
            molecule_tests_for_evidence.push(target_test.clone());
            target_molecule_test = Some(target_test);
            (generation_scope, library_root)
        } else {
            bail!("{} is not a .unit.spec or .test.spec file", path.display());
        }
    } else {
        if includes_directory_molecule_tests(path) {
            let molecule_tests = load_molecule_test_directory(path).with_context(|| {
                format!("Failed to load molecule tests from {}", path.display())
            })?;
            molecule_tests_for_evidence = molecule_tests;
            molecule_evidence_root = Some(if path.join("units").is_dir() {
                path.join("units")
            } else {
                path.to_path_buf()
            });
        }
        (path.to_path_buf(), path.to_path_buf())
    };

    let (root_specs, loader_errors, loader_warnings, _total_files) =
        collect_specs(&generation_scope)?;
    if !loader_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in loader_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ unable to load units before test");
    }
    if !loader_warnings.is_empty() {
        let mut warnings = DiagnosticMap::new();
        for warning in loader_warnings {
            push_warning(&mut warnings, warning);
        }
        print_diagnostics(&warnings);
    }
    let alias_errors = validate_library_crate_aliases(&root_specs, path, context);
    if !alias_errors.is_empty() {
        let mut errors = DiagnosticMap::new();
        for err in alias_errors {
            push_error(&mut errors, err);
        }
        print_diagnostics(&errors);
        bail!("❌ cross-library crate alias validation failed");
    }

    let ctx = resolve_pipeline_context(&pipeline_scope, crate_root_flag, context)?;
    let mut resolved_output = output
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.crate_root.join("src/generated"));
    let mut effective_project_root = ctx.project_root.clone();
    let mut effective_crate_root = ctx.crate_root.clone();
    if path.is_file() {
        let isolated_project =
            materialize_single_file_test_project_scope(&ctx.project_root, &ctx.crate_root)?;
        effective_project_root = isolated_project.path().to_path_buf();
        effective_crate_root = isolated_project
            .path()
            .join(path_relative_to(&ctx.crate_root, &ctx.project_root)?);
        resolved_output = effective_crate_root.join("src/generated");
        single_file_test_project_scope = Some(isolated_project);
    }

    let generated = generate_specs(&generation_scope, &resolved_output, &effective_project_root)?;
    let _single_file_generation_scope = single_file_generation_scope;
    let _single_file_test_project_scope = single_file_test_project_scope;
    if target_spec.is_none() && target_molecule_test.is_none() {
        finalize_passports(
            &PassportWritePlan {
                passport_root: path,
                gate_root: path,
                specs: &generated.specs,
                gate_specs: &generated.specs,
            },
            &generated.generated_at,
            None,
            None,
            None,
            false,
        )?;
    }

    let passport_write_plan = passport_write_plan(
        path,
        &pipeline_scope,
        &gate_root,
        &generated.specs,
        target_spec.as_ref(),
    );

    // Resolve the module prefix once — used for both the cargo test filter and
    // evidence lookup. A single resolved value ensures they always agree.
    let effective_prefix = match &context.config.pipeline.generated_module_prefix {
        Some(explicit) => explicit.clone(),
        None => output_module_prefix(
            &resolved_output,
            &effective_crate_root,
            &std::env::current_dir().context("failed to resolve current working directory")?,
        )?,
    };
    let filter = if let Some(target) = target_spec.as_ref() {
        let resolved = normalize_unit(target.spec.clone())
            .with_context(|| format!("Failed to normalize {}", target.source.file_path))?;
        Some(cargo_test_filter_for_unit(&resolved, &effective_prefix))
    } else {
        target_molecule_test.as_ref().map(|target| {
            let resolved = ResolvedMoleculeTest::from_loaded(target);
            cargo_test_filter_for_molecule(&resolved, &effective_prefix)
        })
    };

    let provenance = resolve_git_provenance(&ctx.crate_root);
    let build_result = run_cargo_build(
        &effective_crate_root,
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
        let molecule_evidence = if !molecule_tests_for_evidence.is_empty() {
            let specs_by_id = if molecule_specs_by_id.is_empty() {
                generated
                    .specs
                    .iter()
                    .map(|spec| (spec.spec.id.clone(), spec.clone()))
                    .collect()
            } else {
                molecule_specs_by_id.clone()
            };
            Some(build_molecule_incomplete_evidence(
                &molecule_tests_for_evidence,
                &specs_by_id,
                MoleculeEvidenceStatus::Timeout,
                Some("cargo build timed out"),
                &observed_at,
                provenance.as_ref(),
            ))
        } else {
            None
        };
        if target_molecule_test.is_none() {
            finalize_test_passports(
                &passport_write_plan,
                &generated.generated_at,
                &evidence_by_spec,
                contract_hash_by_spec.as_ref(),
                molecule_evidence.as_ref(),
            )?;
        }
        if let Some(molecule_evidence) = molecule_evidence.as_ref() {
            finalize_molecule_evidence(
                molecule_evidence_root.as_deref().unwrap_or(path),
                &molecule_tests_for_evidence,
                molecule_evidence,
            )?;
            if target_molecule_test.is_some() {
                let gate_specs = gate_specs_for_refresh(&generated.specs, &molecule_specs_by_id);
                refresh_covered_seam_passports(
                    molecule_evidence_root.as_deref().unwrap_or(path),
                    &gate_root,
                    &gate_specs,
                    &generated.generated_at,
                    &molecule_tests_for_evidence,
                    molecule_evidence,
                )?;
            }
        }
        bail!("❌ cargo build timed out{}", timeout_suffix(ctx.timeout));
    }
    if build_result.exit_code != 0 {
        let observed_at = rfc3339_now();
        let evidence_by_spec =
            build_failure_evidence(passport_write_plan.specs, &observed_at, provenance.as_ref());
        let contract_hash_by_spec = contract_hashes_for(passport_write_plan.specs);
        let molecule_evidence = if !molecule_tests_for_evidence.is_empty() {
            let specs_by_id = if molecule_specs_by_id.is_empty() {
                generated
                    .specs
                    .iter()
                    .map(|spec| (spec.spec.id.clone(), spec.clone()))
                    .collect()
            } else {
                molecule_specs_by_id.clone()
            };
            Some(build_molecule_incomplete_evidence(
                &molecule_tests_for_evidence,
                &specs_by_id,
                MoleculeEvidenceStatus::BuildFail,
                Some("cargo build failed"),
                &observed_at,
                provenance.as_ref(),
            ))
        } else {
            None
        };
        if target_molecule_test.is_none() {
            finalize_test_passports(
                &passport_write_plan,
                &generated.generated_at,
                &evidence_by_spec,
                contract_hash_by_spec.as_ref(),
                molecule_evidence.as_ref(),
            )?;
        }
        if let Some(molecule_evidence) = molecule_evidence.as_ref() {
            finalize_molecule_evidence(
                molecule_evidence_root.as_deref().unwrap_or(path),
                &molecule_tests_for_evidence,
                molecule_evidence,
            )?;
            if target_molecule_test.is_some() {
                let gate_specs = gate_specs_for_refresh(&generated.specs, &molecule_specs_by_id);
                refresh_covered_seam_passports(
                    molecule_evidence_root.as_deref().unwrap_or(path),
                    &gate_root,
                    &gate_specs,
                    &generated.generated_at,
                    &molecule_tests_for_evidence,
                    molecule_evidence,
                )?;
            }
        }
        bail!("❌ cargo build failed");
    }

    let test_result = run_cargo_test(
        &effective_crate_root,
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
        let molecule_evidence = if !molecule_tests_for_evidence.is_empty() {
            let specs_by_id = if molecule_specs_by_id.is_empty() {
                generated
                    .specs
                    .iter()
                    .map(|spec| (spec.spec.id.clone(), spec.clone()))
                    .collect()
            } else {
                molecule_specs_by_id.clone()
            };
            Some(build_molecule_incomplete_evidence(
                &molecule_tests_for_evidence,
                &specs_by_id,
                MoleculeEvidenceStatus::Timeout,
                Some("cargo test timed out"),
                &observed_at,
                provenance.as_ref(),
            ))
        } else {
            None
        };
        if target_molecule_test.is_none() {
            finalize_test_passports(
                &passport_write_plan,
                &generated.generated_at,
                &evidence_by_spec,
                contract_hash_by_spec.as_ref(),
                molecule_evidence.as_ref(),
            )?;
        }
        if let Some(molecule_evidence) = molecule_evidence.as_ref() {
            finalize_molecule_evidence(
                molecule_evidence_root.as_deref().unwrap_or(path),
                &molecule_tests_for_evidence,
                molecule_evidence,
            )?;
            if target_molecule_test.is_some() {
                let gate_specs = gate_specs_for_refresh(&generated.specs, &molecule_specs_by_id);
                refresh_covered_seam_passports(
                    molecule_evidence_root.as_deref().unwrap_or(path),
                    &gate_root,
                    &gate_specs,
                    &generated.generated_at,
                    &molecule_tests_for_evidence,
                    molecule_evidence,
                )?;
            }
        }
        bail!("❌ cargo test timed out{}", timeout_suffix(ctx.timeout));
    }

    if (target_spec.is_some() || target_molecule_test.is_some())
        && zero_tests_ran(&test_result.stdout)
    {
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
    let molecule_evidence = if !molecule_tests_for_evidence.is_empty() {
        let specs_by_id = if molecule_specs_by_id.is_empty() {
            generated
                .specs
                .iter()
                .map(|spec| (spec.spec.id.clone(), spec.clone()))
                .collect()
        } else {
            molecule_specs_by_id.clone()
        };
        Some(build_test_molecule_evidence(
            &molecule_tests_for_evidence,
            &specs_by_id,
            &effective_prefix,
            &parsed_test_results,
            &observed_at,
            provenance.as_ref(),
        ))
    } else {
        None
    };
    if target_molecule_test.is_none() {
        finalize_test_passports(
            &passport_write_plan,
            &generated.generated_at,
            &evidence_by_spec,
            contract_hash_by_spec.as_ref(),
            molecule_evidence.as_ref(),
        )?;
    }
    if let Some(molecule_evidence) = molecule_evidence.as_ref() {
        finalize_molecule_evidence(
            molecule_evidence_root.as_deref().unwrap_or(path),
            &molecule_tests_for_evidence,
            molecule_evidence,
        )?;
        if target_molecule_test.is_some() {
            let gate_specs = gate_specs_for_refresh(&generated.specs, &molecule_specs_by_id);
            refresh_covered_seam_passports(
                molecule_evidence_root.as_deref().unwrap_or(path),
                &gate_root,
                &gate_specs,
                &generated.generated_at,
                &molecule_tests_for_evidence,
                molecule_evidence,
            )?;
        }
    }
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

fn build_molecule_incomplete_evidence(
    tests: &[LoadedMoleculeTest],
    specs_by_id: &HashMap<String, LoadedSpec>,
    status: MoleculeEvidenceStatus,
    reason: Option<&str>,
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> BTreeMap<String, MoleculeEvidence> {
    tests
        .iter()
        .map(|test| {
            (
                test.test.id.clone(),
                build_molecule_evidence(
                    test,
                    status.clone(),
                    reason.map(str::to_string),
                    observed_at,
                    specs_by_id,
                    provenance,
                ),
            )
        })
        .collect()
}

fn finalize_molecule_evidence(
    evidence_root: &Path,
    tests: &[LoadedMoleculeTest],
    evidence_by_id: &BTreeMap<String, MoleculeEvidence>,
) -> Result<()> {
    if tests.is_empty() {
        return Ok(());
    }

    for test in tests {
        if let Some(evidence) = evidence_by_id.get(&test.test.id) {
            write_molecule_evidence(evidence, Path::new(&test.source.file_path)).with_context(
                || format!("Failed to write molecule evidence for {}", test.test.id),
            )?;
        }
    }

    ensure_molecule_evidence_gitignore_entry(evidence_root)
        .with_context(|| "Failed to update .gitignore for molecule evidence files")?;
    Ok(())
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
    gate_root: &'a Path,
    generated_specs: &'a [LoadedSpec],
    target_spec: Option<&'a LoadedSpec>,
) -> PassportWritePlan<'a> {
    if let Some(target_spec) = target_spec {
        PassportWritePlan {
            passport_root: spec_root,
            gate_root,
            specs: std::slice::from_ref(target_spec),
            gate_specs: generated_specs,
        }
    } else {
        PassportWritePlan {
            passport_root: requested_path,
            gate_root,
            specs: generated_specs,
            gate_specs: generated_specs,
        }
    }
}

fn finalize_test_passports(
    plan: &PassportWritePlan<'_>,
    generated_at: &str,
    evidence_by_spec: &BTreeMap<String, PassportEvidence>,
    contract_hash_by_spec: Option<&BTreeMap<String, String>>,
    molecule_evidence_overrides: Option<&BTreeMap<String, MoleculeEvidence>>,
) -> Result<()> {
    finalize_passports(
        plan,
        generated_at,
        Some(evidence_by_spec),
        contract_hash_by_spec,
        molecule_evidence_overrides,
        true,
    )
}

fn refresh_covered_seam_passports(
    passport_root: &Path,
    gate_root: &Path,
    gate_specs: &[LoadedSpec],
    generated_at: &str,
    molecule_tests: &[LoadedMoleculeTest],
    molecule_evidence_by_id: &BTreeMap<String, MoleculeEvidence>,
) -> Result<()> {
    let covered_ids: HashSet<&str> = molecule_tests
        .iter()
        .flat_map(|test| test.test.covers.iter().map(String::as_str))
        .collect();
    let covered_specs: Vec<LoadedSpec> = gate_specs
        .iter()
        .filter(|spec| covered_ids.contains(spec.spec.id.as_str()))
        .cloned()
        .collect();

    finalize_passports(
        &PassportWritePlan {
            passport_root,
            gate_root,
            specs: &covered_specs,
            gate_specs,
        },
        generated_at,
        None,
        None,
        Some(molecule_evidence_by_id),
        true,
    )
}

fn gate_specs_for_refresh(
    generated_specs: &[LoadedSpec],
    original_specs_by_id: &HashMap<String, LoadedSpec>,
) -> Vec<LoadedSpec> {
    if original_specs_by_id.is_empty() {
        generated_specs.to_vec()
    } else {
        original_specs_by_id.values().cloned().collect()
    }
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
        let resolved = normalize_unit(spec.spec.clone()).map_err(|err| anyhow::anyhow!(err))?;
        let mut test_results = Vec::new();

        for local_test in &spec.spec.local_tests {
            let full_name =
                expected_cargo_test_name_for_unit(&resolved, output_prefix, &local_test.id);
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

fn build_test_molecule_evidence(
    tests: &[LoadedMoleculeTest],
    specs_by_id: &HashMap<String, LoadedSpec>,
    output_prefix: &str,
    parsed_test_results: &HashMap<String, ParsedCargoTestResult>,
    observed_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> BTreeMap<String, MoleculeEvidence> {
    tests
        .iter()
        .map(|test| {
            let resolved = ResolvedMoleculeTest::from_loaded(test);
            let full_name = expected_cargo_molecule_test_name(&resolved, output_prefix);
            let observed = parsed_test_results.get(&full_name);
            let (status, reason) = match observed {
                Some(result) => match result.status.as_str() {
                    "pass" => (MoleculeEvidenceStatus::Pass, result.reason.clone()),
                    "fail" => (MoleculeEvidenceStatus::Fail, result.reason.clone()),
                    _ => (MoleculeEvidenceStatus::Unknown, result.reason.clone()),
                },
                None => (
                    MoleculeEvidenceStatus::Unknown,
                    Some("test not found in cargo output".to_string()),
                ),
            };

            (
                test.test.id.clone(),
                build_molecule_evidence(test, status, reason, observed_at, specs_by_id, provenance),
            )
        })
        .collect()
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

fn unit_module_path(unit: &NormalizedUnit) -> &str {
    unit.module_path()
}

fn unit_file_stem(unit: &NormalizedUnit) -> &str {
    spec_core::types::callable_name(unit.id())
}

fn cargo_test_filter_for_unit(unit: &NormalizedUnit, output_prefix: &str) -> String {
    if unit_module_path(unit).is_empty() {
        format!("{output_prefix}::{}::tests::", unit_file_stem(unit))
    } else {
        format!(
            "{output_prefix}::{}::{}::tests::",
            unit_module_path(unit).replace('/', "::"),
            unit_file_stem(unit)
        )
    }
}

fn cargo_test_filter_for_molecule(test: &ResolvedMoleculeTest, output_prefix: &str) -> String {
    if test.module_path.is_empty() {
        format!("{output_prefix}::molecule_tests::test_{}", test.fn_name)
    } else {
        format!(
            "{output_prefix}::{}::molecule_tests::test_{}",
            test.module_path.replace('/', "::"),
            test.fn_name
        )
    }
}

#[cfg(test)]
fn expected_cargo_test_name(spec: &ResolvedSpec, output_prefix: &str, test_id: &str) -> String {
    expected_cargo_test_name_for_unit(
        &NormalizedUnit::Function(spec.clone()),
        output_prefix,
        test_id,
    )
}

fn expected_cargo_test_name_for_unit(
    unit: &NormalizedUnit,
    output_prefix: &str,
    test_id: &str,
) -> String {
    if unit_module_path(unit).is_empty() {
        format!(
            "{output_prefix}::{}::tests::test_{test_id}",
            unit_file_stem(unit)
        )
    } else {
        format!(
            "{output_prefix}::{}::{}::tests::test_{test_id}",
            unit_module_path(unit).replace('/', "::"),
            unit_file_stem(unit)
        )
    }
}

fn expected_cargo_molecule_test_name(test: &ResolvedMoleculeTest, output_prefix: &str) -> String {
    if test.module_path.is_empty() {
        format!("{output_prefix}::molecule_tests::test_{}", test.fn_name)
    } else {
        format!(
            "{output_prefix}::{}::molecule_tests::test_{}",
            test.module_path.replace('/', "::"),
            test.fn_name
        )
    }
}

#[derive(Default)]
struct Namespace {
    unit_files: BTreeSet<String>,
    subdirs: BTreeSet<String>,
    has_molecule_tests: bool,
}

fn record_namespace_branch(module_path: &str, namespaces: &mut BTreeMap<String, Namespace>) {
    namespaces.entry(String::new()).or_default();

    if module_path.is_empty() {
        return;
    }

    let segments: Vec<&str> = module_path.split('/').collect();
    for depth in 0..segments.len() {
        let parent = if depth == 0 {
            String::new()
        } else {
            segments[..depth].join("/")
        };
        namespaces
            .entry(parent)
            .or_default()
            .subdirs
            .insert(segments[depth].to_string());

        let current = segments[..=depth].join("/");
        namespaces.entry(current).or_default();
    }
}

fn build_namespaces(
    units: &[NormalizedUnit],
    molecule_tests: &[ResolvedMoleculeTest],
) -> BTreeMap<String, Namespace> {
    let mut namespaces = BTreeMap::<String, Namespace>::new();
    namespaces.entry(String::new()).or_default();

    for unit in units {
        record_namespace_branch(unit_module_path(unit), &mut namespaces);
        namespaces
            .entry(unit_module_path(unit).to_string())
            .or_default()
            .unit_files
            .insert(unit_file_stem(unit).to_string());
    }

    for test in molecule_tests {
        record_namespace_branch(&test.module_path, &mut namespaces);
        namespaces
            .entry(test.module_path.clone())
            .or_default()
            .has_molecule_tests = true;
    }

    namespaces
}

fn path_for_unit(unit: &NormalizedUnit) -> PathBuf {
    let mut path = PathBuf::new();
    if !unit_module_path(unit).is_empty() {
        path.push(unit_module_path(unit).replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    path.push(format!("{}.rs", unit_file_stem(unit)));
    path
}

fn ensure_output_marker(output: &Path, project_root: &Path) -> Result<PathBuf> {
    let output_base = safe_output_path_with_project_root(output, project_root)?;

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
            let hint = if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".test.spec"))
            {
                " (to validate molecule tests, pass the containing directory)"
            } else {
                ""
            };
            bail!("{} is not a .unit.spec file{}", path.display(), hint);
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

fn collect_validation_specs(
    path: &Path,
    context: &WorkspaceContext,
) -> Result<ValidationSpecCollection> {
    let (root_specs, mut loader_errors, mut loader_warnings, mut total_files) = if path.is_dir() {
        if let Some(library_root) = resolve_directory_library_root(path) {
            let report = load_directory_report_bounded(path, &library_root)?;
            (
                report.specs,
                report.errors,
                report.warnings,
                report.total_files,
            )
        } else {
            collect_specs(path)?
        }
    } else {
        collect_specs(path)?
    };
    let support_specs = collect_local_support_specs(path, context, &root_specs)?;
    let (
        _selected_libraries,
        imported_libraries,
        imported_errors,
        imported_warnings,
        imported_total_files,
    ) = load_referenced_validation_specs(
        root_specs.iter().chain(support_specs.iter()),
        &context.libraries,
    );
    total_files += imported_total_files;
    loader_errors.extend(imported_errors);
    loader_warnings.extend(imported_warnings);

    Ok(ValidationSpecCollection {
        root_specs,
        support_specs,
        imported_libraries,
        loader_errors,
        loader_warnings,
        total_files,
    })
}

fn resolve_directory_library_root(path: &Path) -> Option<PathBuf> {
    if path.join("units").is_dir() {
        canonicalize_existing_dir(path).ok()
    } else if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "units")
    {
        canonicalize_existing_dir(path.parent().unwrap_or(path)).ok()
    } else {
        None
    }
}

fn collect_local_support_specs(
    path: &Path,
    context: &WorkspaceContext,
    root_specs: &[LoadedSpec],
) -> Result<Vec<LoadedSpec>> {
    if !path.is_file() || !is_unit_spec(path) {
        return Ok(Vec::new());
    }

    let Some(library_root) = resolve_unit_library_root(path, context) else {
        return Ok(Vec::new());
    };

    let report = load_directory_report_bounded(&library_root, &library_root)?;
    let specs_by_id: HashMap<String, LoadedSpec> = report
        .specs
        .into_iter()
        .map(|spec| (spec.spec.id.clone(), spec))
        .collect();
    let mut visited: HashSet<String> = root_specs.iter().map(|spec| spec.spec.id.clone()).collect();
    let mut queue = VecDeque::new();
    let mut support_specs = Vec::new();

    for spec in root_specs {
        for dep in local_dep_ids(spec) {
            queue.push_back(dep);
        }
    }

    while let Some(dep_id) = queue.pop_front() {
        if !visited.insert(dep_id.clone()) {
            continue;
        }
        let Some(spec) = specs_by_id.get(&dep_id) else {
            continue;
        };
        for child_dep in local_dep_ids(spec) {
            if !visited.contains(&child_dep) {
                queue.push_back(child_dep);
            }
        }
        support_specs.push(spec.clone());
    }

    support_specs.sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    Ok(support_specs)
}

fn resolve_spec_library_root(path: &Path, context: &WorkspaceContext) -> Option<PathBuf> {
    let absolute_path = absolutize_from_current_dir(path).ok()?;
    let repo_root = context
        .repo_root
        .clone()
        .or_else(|| repo_root_for(&absolute_path));
    let mut current = absolute_path.parent();

    while let Some(dir) = current {
        if let Some(repo_root) = &repo_root
            && !dir.starts_with(repo_root)
        {
            break;
        }

        let units_dir = dir.join("units");
        if units_dir.is_dir() && absolute_path.starts_with(&units_dir) {
            return dir.canonicalize().ok();
        }

        current = dir.parent();
    }

    None
}

fn resolve_unit_library_root(path: &Path, context: &WorkspaceContext) -> Option<PathBuf> {
    if is_unit_spec(path) {
        resolve_spec_library_root(path, context)
    } else {
        None
    }
}

fn resolve_molecule_test_library_root(path: &Path, context: &WorkspaceContext) -> Option<PathBuf> {
    if is_molecule_test_spec(path) {
        resolve_spec_library_root(path, context)
    } else {
        None
    }
}

fn local_dep_ids(spec: &LoadedSpec) -> Vec<String> {
    project_unit(ProjectedUnitRef::Loaded(spec)).local_dep_ids()
}

fn materialize_single_file_generation_scope(
    library_root: &Path,
    specs: &[LoadedSpec],
    extra_files: &[&Path],
) -> Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new_in(library_root).with_context(|| {
        format!(
            "Failed to create temporary generation scope in {}",
            library_root.display()
        )
    })?;

    let spec_toml = library_root.join("spec.toml");
    if spec_toml.is_file() {
        let config = read_workspace_config_file(&spec_toml)?;
        let rewritten = rewrite_workspace_config_relative_paths(&config, library_root);
        let serialized = toml::to_string_pretty(&rewritten)
            .with_context(|| format!("Failed to serialize {}", spec_toml.display()))?;
        fs::write(temp_dir.path().join("spec.toml"), serialized)
            .with_context(|| format!("Failed to write {}", spec_toml.display()))?;
    }

    for spec in specs {
        let source_path = absolutize_from_current_dir(Path::new(&spec.source.file_path))?
            .canonicalize()
            .with_context(|| {
                format!(
                    "Failed to resolve source path for {}",
                    spec.source.file_path
                )
            })?;
        let rel_path = source_path.strip_prefix(library_root).with_context(|| {
            format!(
                "Failed to place {} inside temporary generation scope rooted at {}",
                source_path.display(),
                library_root.display()
            )
        })?;
        let dest_path = temp_dir.path().join(rel_path);
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        fs::copy(&source_path, &dest_path).with_context(|| {
            format!(
                "Failed to copy {} into {}",
                source_path.display(),
                dest_path.display()
            )
        })?;
    }

    for source_path in extra_files {
        let source_path = absolutize_from_current_dir(source_path)?
            .canonicalize()
            .with_context(|| {
                format!(
                    "Failed to resolve source path for {}",
                    source_path.display()
                )
            })?;
        let rel_path = source_path.strip_prefix(library_root).with_context(|| {
            format!(
                "Failed to place {} inside temporary generation scope rooted at {}",
                source_path.display(),
                library_root.display()
            )
        })?;
        let dest_path = temp_dir.path().join(rel_path);
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        fs::copy(&source_path, &dest_path).with_context(|| {
            format!(
                "Failed to copy {} into {}",
                source_path.display(),
                dest_path.display()
            )
        })?;
    }

    Ok(temp_dir)
}

fn path_relative_to(path: &Path, root: &Path) -> Result<PathBuf> {
    path.strip_prefix(root).map(PathBuf::from).with_context(|| {
        format!(
            "Failed to resolve {} relative to {}",
            path.display(),
            root.display()
        )
    })
}

fn materialize_single_file_test_project_scope(
    project_root: &Path,
    crate_root: &Path,
) -> Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new().with_context(|| {
        format!(
            "Failed to create isolated single-file test project rooted at {}",
            project_root.display()
        )
    })?;
    copy_project_tree_for_single_file_test(project_root, temp_dir.path(), crate_root)?;
    Ok(temp_dir)
}

fn copy_project_tree_for_single_file_test(
    source_root: &Path,
    dest_root: &Path,
    crate_root: &Path,
) -> Result<()> {
    fs::create_dir_all(dest_root)
        .with_context(|| format!("Failed to create {}", dest_root.display()))?;
    let generated_dir = crate_root.join("src/generated");
    copy_project_tree_for_single_file_test_inner(source_root, dest_root, &generated_dir)
}

fn copy_project_tree_for_single_file_test_inner(
    source_dir: &Path,
    dest_dir: &Path,
    generated_dir: &Path,
) -> Result<()> {
    for entry in fs::read_dir(source_dir)
        .with_context(|| format!("Failed to read {}", source_dir.display()))?
    {
        let entry =
            entry.with_context(|| format!("Failed to read entry in {}", source_dir.display()))?;
        let source_path = entry.path();
        if should_skip_single_file_test_copy(&source_path, generated_dir)? {
            continue;
        }

        let dest_path = dest_dir.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to read file type for {}", source_path.display()))?;
        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)
                .with_context(|| format!("Failed to create {}", dest_path.display()))?;
            copy_project_tree_for_single_file_test_inner(&source_path, &dest_path, generated_dir)?;
        } else {
            fs::copy(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy {} into {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn should_skip_single_file_test_copy(path: &Path, generated_dir: &Path) -> Result<bool> {
    if path.starts_with(generated_dir) {
        return Ok(true);
    }

    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("Failed to inspect {}", path.display()))?;
    if metadata.is_dir() {
        return Ok(path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| matches!(name, "target" | ".git")));
    }

    Ok(false)
}

fn collect_plan_validation_specs(
    library_root: &Path,
    libraries: &[ResolvedLibrary],
) -> Result<ValidationSpecCollection> {
    let units_root = library_root.join("units");
    let report = load_directory_report_bounded(&units_root, library_root)?;
    let (
        _selected_libraries,
        imported_libraries,
        imported_errors,
        imported_warnings,
        imported_total_files,
    ) = load_referenced_validation_specs(&report.specs, libraries);

    let mut loader_errors = report.errors;
    let mut loader_warnings = report.warnings;
    loader_errors.extend(imported_errors);
    loader_warnings.extend(imported_warnings);

    Ok(ValidationSpecCollection {
        root_specs: report.specs,
        support_specs: Vec::new(),
        imported_libraries,
        loader_errors,
        loader_warnings,
        total_files: report.total_files + imported_total_files,
    })
}

fn plan_validation_inputs(
    library_root: &Path,
    context: &WorkspaceContext,
) -> Result<PlanValidationInputs> {
    let units_root = library_root.join("units");
    let mut validation_specs = collect_plan_validation_specs(library_root, &context.libraries)?;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: context.config.validation.allow_unsafe_local_test_expect,
    };
    let (mut validation_errors, validation_warnings) =
        finish_validation_with_imports(&validation_specs, &validation_options);
    validation_errors.extend(validate_library_crate_aliases(
        &validation_specs.root_specs,
        library_root,
        context,
    ));

    let molecule_report = if includes_directory_molecule_tests(&units_root) {
        load_molecule_test_directory_report_bounded(&units_root, library_root)?
    } else {
        spec_core::loader::MoleculeTestLoadReport::default()
    };
    validation_errors.extend(molecule_report.errors);

    let (molecule_errors, molecule_warnings) =
        validate_molecule_tests(&molecule_report.tests, &validation_specs.root_specs);
    validation_errors.extend(molecule_errors);

    let loader_warnings = std::mem::take(&mut validation_specs.loader_warnings);
    let warnings = loader_warnings
        .into_iter()
        .chain(validation_warnings)
        .chain(molecule_warnings)
        .chain(molecule_report.warnings)
        .collect();

    Ok((
        validation_specs,
        validation_errors,
        warnings,
        molecule_report.tests,
    ))
}

fn load_referenced_validation_specs<'a>(
    root_specs: impl IntoIterator<Item = &'a LoadedSpec>,
    libraries: &[ResolvedLibrary],
) -> (
    Vec<ResolvedLibrary>,
    Vec<ImportedLibrarySpecs>,
    Vec<spec_core::SpecError>,
    Vec<spec_core::SpecWarning>,
    usize,
) {
    let mut selected_libraries = Vec::new();
    let mut imported_libraries = Vec::new();
    let mut loader_errors = Vec::new();
    let mut loader_warnings = Vec::new();
    let mut total_files = 0;
    let direct_root_aliases: BTreeSet<String> = direct_root_library_aliases(root_specs)
        .into_keys()
        .collect();

    for library in libraries
        .iter()
        .filter(|library| direct_root_aliases.contains(&library.alias))
        .cloned()
    {
        // Only root-spec aliases participate in library discovery. Imported libraries may refer to
        // additional aliases, but those stay unresolved during this invocation rather than
        // recursively loading transitive libraries.
        let report =
            load_directory_report_bounded(&library.root, &library.root).unwrap_or_else(|err| {
                DirectoryLoadReport {
                    errors: vec![err],
                    ..Default::default()
                }
            });
        total_files += report.total_files;
        loader_errors.extend(report.errors);
        loader_warnings.extend(report.warnings);
        selected_libraries.push(library.clone());
        imported_libraries.push(ImportedLibrarySpecs {
            alias: library.alias,
            specs: report.specs,
        });
    }

    (
        selected_libraries,
        imported_libraries,
        loader_errors,
        loader_warnings,
        total_files,
    )
}

#[derive(serde::Deserialize)]
struct CargoManifest {
    #[serde(default)]
    dependencies: BTreeMap<String, toml::Value>,
}

fn validate_library_crate_aliases<'a>(
    root_specs: impl IntoIterator<Item = &'a LoadedSpec>,
    path: &Path,
    context: &WorkspaceContext,
) -> Vec<spec_core::SpecError> {
    let referenced_aliases = direct_root_library_aliases(root_specs);
    if referenced_aliases.is_empty() {
        return Vec::new();
    }

    let (manifest_path, dependency_aliases) =
        match load_root_cargo_dependency_aliases(path, context) {
            Ok(result) => result,
            Err(err) => return vec![err],
        };

    referenced_aliases
        .into_iter()
        .filter_map(|(alias, source_path)| {
            if dependency_aliases.contains(&alias) {
                None
            } else {
                Some(spec_core::SpecError::LibraryCrateAliasMissing {
                    alias,
                    cargo_toml: manifest_path.display().to_string(),
                    path: source_path,
                })
            }
        })
        .collect()
}

fn direct_root_library_aliases<'a>(
    root_specs: impl IntoIterator<Item = &'a LoadedSpec>,
) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();

    for spec in root_specs {
        for dep in top_level_deps(spec) {
            let Ok(dep_ref) = DepRef::parse(&dep) else {
                continue;
            };
            if let Some(alias) = dep_ref.library_alias() {
                aliases
                    .entry(alias.to_string())
                    .or_insert_with(|| spec.source.file_path.clone());
            }
        }
    }

    aliases
}

fn resolved_crate_manifest_path(path: &Path, context: &WorkspaceContext) -> Result<PathBuf> {
    let spec_root = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    let crate_root = resolve_default_crate_root(spec_root, context)?;
    Ok(crate_root.join("Cargo.toml"))
}

fn load_root_cargo_dependency_aliases(
    path: &Path,
    context: &WorkspaceContext,
) -> std::result::Result<(PathBuf, HashSet<String>), spec_core::SpecError> {
    let manifest_path = resolved_crate_manifest_path(path, context).map_err(|err| {
        spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: None,
            message: format!("Failed to resolve Cargo.toml for library alias validation: {err}"),
        }
    })?;
    let dependency_aliases = load_cargo_dependency_aliases(&manifest_path).map_err(|err| {
        spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: Some(manifest_path.display().to_string()),
            message: err.to_string(),
        }
    })?;
    Ok((manifest_path, dependency_aliases))
}

fn load_cargo_dependency_aliases(manifest_path: &Path) -> Result<HashSet<String>> {
    let manifest = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;
    let parsed: CargoManifest = toml::from_str(&manifest)
        .with_context(|| format!("Failed to parse {}", manifest_path.display()))?;
    Ok(parsed.dependencies.into_keys().collect())
}

fn includes_directory_molecule_tests(path: &Path) -> bool {
    path.is_dir()
}

fn validate_molecule_tests(
    tests: &[LoadedMoleculeTest],
    specs: &[LoadedSpec],
) -> (Vec<spec_core::SpecError>, Vec<spec_core::SpecWarning>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Per-test semantic validation
    for test in tests {
        if let Err(e) = validate_molecule_test_semantic(test) {
            errors.push(e);
        }
    }

    // Cross-set: covers must reference real unit IDs
    let unit_ids: std::collections::HashSet<&str> =
        specs.iter().map(|s| s.spec.id.as_str()).collect();
    for test in tests {
        let (errs, warns) = validate_molecule_test_covers(test, &unit_ids);
        errors.extend(errs);
        warnings.extend(warns);
    }

    // Duplicate IDs
    errors.extend(validate_no_duplicate_molecule_test_ids(tests));

    (errors, warnings)
}

#[cfg(test)]
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

fn finish_validation_with_imports(
    specs: &ValidationSpecCollection,
    options: &ValidationOptions,
) -> (Vec<spec_core::SpecError>, Vec<spec_core::SpecWarning>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for spec in &specs.root_specs {
        if let Err(err) = validate_full_with_options(spec, options) {
            errors.push(err);
        }
    }
    for spec in &specs.support_specs {
        if let Err(err) = validate_full_with_options(spec, options) {
            errors.push(err);
        }
    }
    for library in &specs.imported_libraries {
        for spec in &library.specs {
            if let Err(err) = validate_full_with_options(spec, options) {
                errors.push(err);
            }
        }
    }

    let alias_set: HashSet<&str> = specs
        .imported_libraries
        .iter()
        .map(|library| library.alias.as_str())
        .collect();
    let qualified_specs = build_qualified_validation_specs(specs, &alias_set, &mut errors);

    errors.extend(validate_no_duplicate_qualified_ids(&qualified_specs));

    let (mut dep_errors, dep_warnings) =
        validate_qualified_deps_exist_with_options(&qualified_specs, options);
    errors.append(&mut dep_errors);
    warnings.extend(dep_warnings);
    warnings.extend(check_spec_versions(&specs.root_specs));
    warnings.extend(check_spec_versions(&specs.support_specs));
    for library in &specs.imported_libraries {
        warnings.extend(check_spec_versions(&library.specs));
    }

    (errors, warnings)
}

fn build_qualified_validation_specs<'a>(
    specs: &'a ValidationSpecCollection,
    known_library_aliases: &HashSet<&str>,
    errors: &mut Vec<spec_core::SpecError>,
) -> Vec<QualifiedLoadedSpec<'a>> {
    let mut qualified_specs = Vec::new();

    for spec in &specs.root_specs {
        qualified_specs.push(qualify_loaded_spec(
            spec,
            None,
            known_library_aliases,
            errors,
        ));
    }
    for spec in &specs.support_specs {
        qualified_specs.push(qualify_loaded_spec(
            spec,
            None,
            known_library_aliases,
            errors,
        ));
    }
    for library in &specs.imported_libraries {
        for spec in &library.specs {
            qualified_specs.push(qualify_loaded_spec(
                spec,
                Some(library.alias.as_str()),
                known_library_aliases,
                errors,
            ));
        }
    }

    qualified_specs
}

fn qualify_loaded_spec<'a>(
    loaded: &'a LoadedSpec,
    current_library: Option<&str>,
    known_library_aliases: &HashSet<&str>,
    errors: &mut Vec<spec_core::SpecError>,
) -> QualifiedLoadedSpec<'a> {
    let authored_deps = top_level_deps(loaded);
    let mut qualified_deps = Vec::with_capacity(authored_deps.len());

    for authored_dep in &authored_deps {
        let dep = match DepRef::parse(authored_dep) {
            Ok(dep) => dep,
            Err(err) => {
                errors.push(spec_core::SpecError::SemanticValidation {
                    message: err.to_string(),
                    path: loaded.source.file_path.clone(),
                });
                continue;
            }
        };

        if let Some(alias) = dep.library_alias()
            && !known_library_aliases.contains(alias)
        {
            errors.push(spec_core::SpecError::UnknownLibraryNamespace {
                alias: alias.to_string(),
                dep: dep.authored(),
                path: loaded.source.file_path.clone(),
            });
            continue;
        }

        qualified_deps.push(dep.to_qualified(current_library));
    }

    QualifiedLoadedSpec {
        loaded,
        qualified_id: QualifiedUnitRef::new(
            current_library.map(str::to_string),
            loaded.spec.id.clone(),
        ),
        qualified_deps,
    }
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
        spec_core::SpecError::UnknownLibraryNamespace { .. } => "SPEC_UNKNOWN_LIBRARY_NAMESPACE",
        spec_core::SpecError::CrossLibraryDepNotFound { .. } => "SPEC_CROSS_LIBRARY_DEP_NOT_FOUND",
        spec_core::SpecError::LibraryCrateAliasMissing { .. } => "SPEC_LIBRARY_CRATE_ALIAS_MISSING",
        spec_core::SpecError::LibraryCrateManifestError { .. } => {
            "SPEC_LIBRARY_CRATE_MANIFEST_ERROR"
        }
        spec_core::SpecError::CyclicDep { .. } => "SPEC_CYCLIC_DEP",
        spec_core::SpecError::CrossLibraryCycle { .. } => "SPEC_CROSS_LIBRARY_CYCLE",
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
        spec_core::SpecError::MoleculeCoversNotFound { .. } => "SPEC_MOLECULE_COVERS_NOT_FOUND",
        spec_core::SpecError::CrossLibraryMoleculeCoverUnsupported { .. } => {
            "SPEC_MOLECULE_CROSS_LIBRARY_COVERS_UNSUPPORTED"
        }
        spec_core::SpecError::DuplicateMoleculeTestId { .. } => "SPEC_DUPLICATE_MOLECULE_ID",
        spec_core::SpecError::MoleculeCoversCollision { .. } => "SPEC_MOLECULE_COVERS_COLLISION",
        spec_core::SpecError::MoleculeBodyRustMustBeBlock { .. } => {
            "SPEC_MOLECULE_BODY_RUST_MUST_BE_BLOCK"
        }
        spec_core::SpecError::MoleculeBodyContainsUnsafe { .. } => {
            "SPEC_MOLECULE_BODY_CONTAINS_UNSAFE"
        }
        spec_core::SpecError::ReservedUnitName { .. } => "SPEC_RESERVED_UNIT_NAME",
        spec_core::SpecError::PlanDirectoryInput { .. } => "SPEC_PLAN_DIRECTORY_INPUT",
        spec_core::SpecError::PlanOutsideLibraryRoot { .. } => "SPEC_PLAN_OUTSIDE_LIBRARY_ROOT",
        spec_core::SpecError::PlanSymlinkEscape { .. } => "SPEC_PLAN_SYMLINK_ESCAPE",
        spec_core::SpecError::PlanCrossLibraryUnit { .. } => "SPEC_PLAN_CROSS_LIBRARY_UNIT",
        spec_core::SpecError::PlanDuplicateChangeUnit { .. } => "SPEC_PLAN_DUPLICATE_CHANGE_UNIT",
        spec_core::SpecError::PlanUnitMissingForAction { .. } => {
            "SPEC_PLAN_UNIT_MISSING_FOR_ACTION"
        }
        spec_core::SpecError::PlanUnitAlreadyExistsForAdd { .. } => {
            "SPEC_PLAN_UNIT_ALREADY_EXISTS_FOR_ADD"
        }
        spec_core::SpecError::PlanMoleculeTestNotFound { .. } => {
            "SPEC_PLAN_MOLECULE_TEST_NOT_FOUND"
        }
        spec_core::SpecError::MoleculeEvidenceMalformed { .. } => {
            "SPEC_MOLECULE_EVIDENCE_MALFORMED"
        }
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
        spec_core::SpecError::UnknownLibraryNamespace { alias, dep, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            dep: Some(dep.clone()),
            value: Some(alias.clone()),
            ..Default::default()
        },
        spec_core::SpecError::CrossLibraryDepNotFound { dep, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            dep: Some(dep.clone()),
            ..Default::default()
        },
        spec_core::SpecError::LibraryCrateAliasMissing {
            alias,
            cargo_toml,
            path,
        } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            value: Some(alias.clone()),
            path2: Some(cargo_toml.clone()),
            ..Default::default()
        },
        spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml,
            message,
        } => ErrorFields {
            path: cargo_toml.clone(),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::CyclicDep { cycle_path, path } => ErrorFields {
            unit: id_by_path.get(path).cloned(),
            path: Some(path.clone()),
            cycle: Some(cycle_path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::CrossLibraryCycle { cycle_path, path } => ErrorFields {
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
            field: Some(field.clone()),
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
        spec_core::SpecError::MoleculeCoversNotFound {
            cover_id,
            test_id,
            test_path,
        } => ErrorFields {
            path: Some(test_path.clone()),
            id: Some(test_id.clone()),
            dep: Some(cover_id.clone()),
            ..Default::default()
        },
        spec_core::SpecError::CrossLibraryMoleculeCoverUnsupported {
            cover_id,
            test_id,
            test_path,
        } => ErrorFields {
            path: Some(test_path.clone()),
            id: Some(test_id.clone()),
            dep: Some(cover_id.clone()),
            message: Some(err.to_string()),
            ..Default::default()
        },
        spec_core::SpecError::DuplicateMoleculeTestId { id, file1, file2 } => ErrorFields {
            path: Some(file1.clone()),
            id: Some(id.clone()),
            path2: Some(file2.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MoleculeCoversCollision {
            cover1,
            cover2,
            fn_name,
            test_id,
            test_path,
        } => ErrorFields {
            path: Some(test_path.clone()),
            id: Some(test_id.clone()),
            dep: Some(cover1.clone()),
            value: Some(fn_name.clone()),
            path2: Some(cover2.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MoleculeBodyRustMustBeBlock { message, test_path } => ErrorFields {
            path: Some(test_path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MoleculeBodyContainsUnsafe { test_path } => ErrorFields {
            path: Some(test_path.clone()),
            ..Default::default()
        },
        spec_core::SpecError::ReservedUnitName { segment, path } => ErrorFields {
            path: Some(path.clone()),
            value: Some(segment.clone()),
            ..Default::default()
        },
        spec_core::SpecError::MoleculeEvidenceMalformed { path, message } => ErrorFields {
            path: Some(path.clone()),
            message: Some(message.clone()),
            ..Default::default()
        },
        spec_core::SpecError::PlanDirectoryInput { path }
        | spec_core::SpecError::PlanOutsideLibraryRoot { path }
        | spec_core::SpecError::PlanSymlinkEscape { path } => ErrorFields {
            path: Some(path.clone()),
            message: Some(err.to_string()),
            ..Default::default()
        },
        spec_core::SpecError::PlanCrossLibraryUnit { unit, path }
        | spec_core::SpecError::PlanDuplicateChangeUnit { unit, path }
        | spec_core::SpecError::PlanUnitAlreadyExistsForAdd { unit, path } => ErrorFields {
            path: Some(path.clone()),
            id: Some(unit.clone()),
            ..Default::default()
        },
        spec_core::SpecError::PlanUnitMissingForAction { unit, action, path } => ErrorFields {
            path: Some(path.clone()),
            id: Some(unit.clone()),
            value: Some(action.clone()),
            ..Default::default()
        },
        spec_core::SpecError::PlanMoleculeTestNotFound { test_id, path } => ErrorFields {
            path: Some(path.clone()),
            id: Some(test_id.clone()),
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

fn spec_warning_code(warning: &spec_core::SpecWarning) -> &'static str {
    match warning {
        spec_core::SpecWarning::MissingDep { .. } => "SPEC_MISSING_DEP_WARNING",
        spec_core::SpecWarning::SymlinkCycleSkipped { .. } => "SPEC_SYMLINK_CYCLE_SKIPPED",
        spec_core::SpecWarning::MissingSpecVersion { .. } => "SPEC_MISSING_SPEC_VERSION",
        spec_core::SpecWarning::MoleculeTestNoCoveredUnits { .. } => {
            "SPEC_MOLECULE_TEST_NO_COVERED_UNITS"
        }
        spec_core::SpecWarning::MoleculeImplicitImportsDeprecated { .. } => {
            "SPEC_MOLECULE_IMPLICIT_IMPORTS_DEPRECATED"
        }
    }
}

fn spec_warning_to_json_entry(warning: &spec_core::SpecWarning) -> JsonWarningEntry {
    let (path, id) = match warning {
        spec_core::SpecWarning::MissingDep { path, .. }
        | spec_core::SpecWarning::SymlinkCycleSkipped { path }
        | spec_core::SpecWarning::MissingSpecVersion { path, .. } => (Some(path.clone()), None),
        spec_core::SpecWarning::MoleculeTestNoCoveredUnits { test_id, test_path }
        | spec_core::SpecWarning::MoleculeImplicitImportsDeprecated { test_id, test_path } => {
            (Some(test_path.clone()), Some(test_id.clone()))
        }
    };

    JsonWarningEntry {
        code: spec_warning_code(warning).to_string(),
        path,
        id,
        message: warning.to_string(),
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
        | spec_core::SpecError::UnknownLibraryNamespace { path, .. }
        | spec_core::SpecError::CrossLibraryDepNotFound { path, .. }
        | spec_core::SpecError::LibraryCrateAliasMissing { path, .. }
        | spec_core::SpecError::CyclicDep { path, .. }
        | spec_core::SpecError::CrossLibraryCycle { path, .. }
        | spec_core::SpecError::UseStatementInBody { path }
        | spec_core::SpecError::BodyRustMustBeBlock { path, .. }
        | spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path }
        | spec_core::SpecError::LocalTestExpectNotExpr { path, .. }
        | spec_core::SpecError::DuplicateLocalTestId { path, .. }
        | spec_core::SpecError::ContractTypeInvalid { path, .. }
        | spec_core::SpecError::ContractInputNameInvalid { path, .. }
        | spec_core::SpecError::Traversal { path, .. }
        | spec_core::SpecError::MissingMarker { path }
        | spec_core::SpecError::PlanDirectoryInput { path }
        | spec_core::SpecError::PlanOutsideLibraryRoot { path }
        | spec_core::SpecError::PlanSymlinkEscape { path }
        | spec_core::SpecError::PlanCrossLibraryUnit { path, .. }
        | spec_core::SpecError::PlanDuplicateChangeUnit { path, .. }
        | spec_core::SpecError::PlanUnitMissingForAction { path, .. }
        | spec_core::SpecError::PlanUnitAlreadyExistsForAdd { path, .. }
        | spec_core::SpecError::PlanMoleculeTestNotFound { path, .. }
        | spec_core::SpecError::MoleculeEvidenceMalformed { path, .. } => vec![path.clone()],
        spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: Some(path),
            ..
        } => vec![path.clone()],
        spec_core::SpecError::Generator { .. }
        | spec_core::SpecError::OutputDir { .. }
        | spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: None, ..
        }
        | spec_core::SpecError::Io(_)
        | spec_core::SpecError::Json(_) => Vec::new(),
        spec_core::SpecError::MoleculeCoversNotFound { test_path, .. }
        | spec_core::SpecError::CrossLibraryMoleculeCoverUnsupported { test_path, .. }
        | spec_core::SpecError::MoleculeCoversCollision { test_path, .. }
        | spec_core::SpecError::MoleculeBodyRustMustBeBlock { test_path, .. }
        | spec_core::SpecError::MoleculeBodyContainsUnsafe { test_path } => {
            vec![test_path.clone()]
        }
        spec_core::SpecError::DuplicateMoleculeTestId { file1, file2, .. } => {
            vec![file1.clone(), file2.clone()]
        }
        spec_core::SpecError::ReservedUnitName { path, .. } => vec![path.clone()],
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
    if let Some(review) = &unit.semantic_review {
        println!("  · {}", semantic_review_summary(review));
    }
    if unit.status == HealthState::Invalid {
        for entry in &unit.errors {
            println!("  · {}", json_error_entry_to_human(entry));
        }
    }
}

fn json_error_entry_to_human(entry: &JsonErrorEntry) -> String {
    if entry.code == "SPEC_DEP_COLLISION"
        && let (Some(dep1), Some(dep2), Some(fn_name)) = (&entry.dep, &entry.path2, &entry.value)
    {
        return format!(
            "{}: '{}' and '{}' both resolve to '{}'",
            entry.code, dep1, dep2, fn_name
        );
    }

    if entry.code == "SPEC_MOLECULE_COVERS_COLLISION"
        && let (Some(cover1), Some(cover2), Some(fn_name), Some(test_id)) =
            (&entry.dep, &entry.path2, &entry.value, &entry.id)
    {
        return format!(
            "{}: '{}' and '{}' both resolve to '{}' in {}",
            entry.code, cover1, cover2, fn_name, test_id
        );
    }

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
        | spec_core::SpecError::UnknownLibraryNamespace { path, .. }
        | spec_core::SpecError::CrossLibraryDepNotFound { path, .. }
        | spec_core::SpecError::LibraryCrateAliasMissing { path, .. }
        | spec_core::SpecError::CyclicDep { path, .. }
        | spec_core::SpecError::CrossLibraryCycle { path, .. }
        | spec_core::SpecError::UseStatementInBody { path }
        | spec_core::SpecError::BodyRustMustBeBlock { path, .. }
        | spec_core::SpecError::BodyRustLooksLikeFnDeclaration { path }
        | spec_core::SpecError::LocalTestExpectNotExpr { path, .. }
        | spec_core::SpecError::DuplicateLocalTestId { path, .. }
        | spec_core::SpecError::ContractTypeInvalid { path, .. }
        | spec_core::SpecError::ContractInputNameInvalid { path, .. }
        | spec_core::SpecError::Traversal { path, .. }
        | spec_core::SpecError::MissingMarker { path }
        | spec_core::SpecError::PlanDirectoryInput { path }
        | spec_core::SpecError::PlanOutsideLibraryRoot { path }
        | spec_core::SpecError::PlanSymlinkEscape { path }
        | spec_core::SpecError::PlanCrossLibraryUnit { path, .. }
        | spec_core::SpecError::PlanDuplicateChangeUnit { path, .. }
        | spec_core::SpecError::PlanUnitMissingForAction { path, .. }
        | spec_core::SpecError::PlanUnitAlreadyExistsForAdd { path, .. }
        | spec_core::SpecError::PlanMoleculeTestNotFound { path, .. }
        | spec_core::SpecError::MoleculeEvidenceMalformed { path, .. } => path.clone(),
        spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: Some(path),
            ..
        } => path.clone(),
        spec_core::SpecError::DuplicateId { file1, file2, .. } => format!("{file1} | {file2}"),
        spec_core::SpecError::Generator { .. } | spec_core::SpecError::OutputDir { .. } => {
            "generation".to_string()
        }
        spec_core::SpecError::Io(_)
        | spec_core::SpecError::Json(_)
        | spec_core::SpecError::LibraryCrateManifestError {
            cargo_toml: None, ..
        } => "validation".to_string(),
        spec_core::SpecError::MoleculeCoversNotFound { test_path, .. }
        | spec_core::SpecError::CrossLibraryMoleculeCoverUnsupported { test_path, .. }
        | spec_core::SpecError::MoleculeCoversCollision { test_path, .. }
        | spec_core::SpecError::MoleculeBodyRustMustBeBlock { test_path, .. }
        | spec_core::SpecError::MoleculeBodyContainsUnsafe { test_path } => test_path.clone(),
        spec_core::SpecError::DuplicateMoleculeTestId { file1, file2, .. } => {
            format!("{file1} | {file2}")
        }
        spec_core::SpecError::ReservedUnitName { path, .. } => path.clone(),
    }
}

fn warning_key(warning: &spec_core::SpecWarning) -> String {
    match warning {
        spec_core::SpecWarning::MissingDep { path, .. }
        | spec_core::SpecWarning::SymlinkCycleSkipped { path }
        | spec_core::SpecWarning::MissingSpecVersion { path, .. } => path.clone(),
        spec_core::SpecWarning::MoleculeTestNoCoveredUnits { test_path, .. }
        | spec_core::SpecWarning::MoleculeImplicitImportsDeprecated { test_path, .. } => {
            test_path.clone()
        }
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
                extensions: spec_core::types::UnitExtensions::default(),
            },
        }
    }

    fn benchmark_specs(spec_count: usize, tests_per_spec: usize) -> Vec<LoadedSpec> {
        (0..spec_count)
            .map(|index| benchmark_loaded_spec(index, tests_per_spec))
            .collect()
    }

    fn benchmark_stdout(specs: &[LoadedSpec], output: &Path, crate_root: &Path) -> String {
        let output_prefix = output_module_prefix(output, crate_root, Path::new("")).unwrap();
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
        let temp_dir = TempDir::new_in(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
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
        let temp_dir = TempDir::new_in(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
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
    fn generate_specs_rejects_reserved_molecule_tests_namespace_segment() {
        let temp_dir = TempDir::new_in(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        let units_dir = temp_dir.path().join("units");
        let output_dir = temp_dir.path().join("generated/spec");

        write_spec(
            &units_dir,
            "qa/molecule_tests/foo.unit.spec",
            r#"
id: qa/molecule_tests/foo
kind: function
intent:
  why: Reproduce reserved namespace collision.
body:
  rust: |
    {
        true
    }
"#,
        );
        write_spec(
            &units_dir,
            "qa/flow.test.spec",
            r#"
id: qa/flow
intent:
  why: Exercise qa molecule test generation.
covers:
  - qa/molecule_tests/foo
body:
  rust: |
    {
        assert!(foo());
    }
"#,
        );

        let (specs, loader_errors, loader_warnings, total_files) =
            collect_specs(&units_dir).unwrap();
        assert_eq!(loader_errors.len(), 0);
        assert_eq!(loader_warnings.len(), 0);
        assert_eq!(total_files, 1);

        let validation_options = ValidationOptions {
            strict_deps: true,
            allow_unsafe_local_test_expect: false,
        };
        let (validation_errors, _validation_warnings) =
            finish_validation(&specs, &validation_options);
        assert!(
            validation_errors.iter().any(|err| {
                matches!(err, spec_core::SpecError::ReservedUnitName { segment, .. } if segment == "molecule_tests")
                    && spec_error_code(err) == "SPEC_RESERVED_UNIT_NAME"
            }),
            "expected SPEC_RESERVED_UNIT_NAME, got: {validation_errors:?}"
        );

        let err = match generate_specs(&units_dir, &output_dir, temp_dir.path()) {
            Ok(_) => panic!("expected reserved namespace validation to fail"),
            Err(err) => err.to_string(),
        };
        assert!(
            err.contains("1 error"),
            "expected generation to fail before output, got: {err}"
        );
        assert!(
            !output_dir.join("qa/molecule_tests.rs").exists(),
            "generator should fail before writing molecule_tests.rs"
        );
        assert!(
            !output_dir.join("qa/molecule_tests/mod.rs").exists(),
            "generator should fail before writing conflicting module output"
        );
    }

    #[test]
    fn cargo_doc_succeeds_for_generated_ecommerce_docs() {
        if !cargo_available() {
            return;
        }

        let temp_dir = TempDir::new_in(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
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
    fn materialize_single_file_generation_scope_rewrites_relative_library_paths_in_temp_config() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let app_root = repo_root.join("app-spec");
        let shared_root = repo_root.join("shared-spec");
        fs::create_dir_all(&app_root).unwrap();
        fs::create_dir_all(&shared_root).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/spec-tests\n").unwrap();
        fs::write(
            app_root.join("spec.toml"),
            "[pipeline]\ncrate_root = \"../app-crate\"\ncargo_target_dir = \"../target/spec\"\n[libraries]\nshared = \"../shared-spec\"\n",
        )
        .unwrap();

        let temp_scope = materialize_single_file_generation_scope(&app_root, &[], &[]).unwrap();
        let materialized =
            read_workspace_config_file(&temp_scope.path().join("spec.toml")).unwrap();

        assert_eq!(
            materialized.libraries.get("shared"),
            Some(&repo_root.join("shared-spec"))
        );
        assert_eq!(
            materialized.pipeline.crate_root,
            Some(repo_root.join("app-crate"))
        );
        assert_eq!(
            materialized.pipeline.cargo_target_dir,
            Some(repo_root.join("target/spec"))
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
            spec_core::SpecError::UnknownLibraryNamespace {
                alias: "shared".to_string(),
                dep: "shared::money/round".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::CrossLibraryDepNotFound {
                dep: "shared::money/round".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::LibraryCrateAliasMissing {
                alias: "shared".to_string(),
                cargo_toml: "Cargo.toml".to_string(),
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::LibraryCrateManifestError {
                cargo_toml: Some("Cargo.toml".to_string()),
                message: "Failed to parse Cargo.toml".to_string(),
            },
            spec_core::SpecError::CyclicDep {
                cycle_path: vec!["a".to_string(), "b".to_string()],
                path: "units/a.unit.spec".to_string(),
            },
            spec_core::SpecError::CrossLibraryCycle {
                cycle_path: vec!["a".to_string(), "shared::b".to_string()],
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
            spec_core::SpecError::MoleculeCoversNotFound {
                cover_id: "pricing/apply_discount".to_string(),
                test_id: "pricing/discount_flow".to_string(),
                test_path: "units/pricing/discount_flow.test.spec".to_string(),
            },
            spec_core::SpecError::DuplicateMoleculeTestId {
                id: "pricing/discount_flow".to_string(),
                file1: "units/pricing/a.test.spec".to_string(),
                file2: "units/pricing/b.test.spec".to_string(),
            },
            spec_core::SpecError::MoleculeCoversCollision {
                cover1: "money/round".to_string(),
                cover2: "utils/round".to_string(),
                fn_name: "round".to_string(),
                test_id: "pricing/discount_flow".to_string(),
                test_path: "units/pricing/discount_flow.test.spec".to_string(),
            },
            spec_core::SpecError::MoleculeBodyRustMustBeBlock {
                message: "expected block".to_string(),
                test_path: "units/pricing/discount_flow.test.spec".to_string(),
            },
            spec_core::SpecError::MoleculeBodyContainsUnsafe {
                test_path: "units/pricing/discount_flow.test.spec".to_string(),
            },
            spec_core::SpecError::ReservedUnitName {
                segment: "molecule_tests".to_string(),
                path: "units/pricing/molecule_tests.unit.spec".to_string(),
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

        let molecule_collision = spec_error_to_json_entry(
            &spec_core::SpecError::MoleculeCoversCollision {
                cover1: "money/round".to_string(),
                cover2: "utils/round".to_string(),
                fn_name: "round".to_string(),
                test_id: "pricing/rounding_flow".to_string(),
                test_path: "units/pricing/rounding_flow.test.spec".to_string(),
            },
            &id_by_path,
        );
        assert_eq!(
            molecule_collision.path.as_deref(),
            Some("units/pricing/rounding_flow.test.spec")
        );
        assert_eq!(
            molecule_collision.id.as_deref(),
            Some("pricing/rounding_flow")
        );
        assert_eq!(molecule_collision.dep.as_deref(), Some("money/round"));
        assert_eq!(molecule_collision.value.as_deref(), Some("round"));
        assert_eq!(molecule_collision.path2.as_deref(), Some("utils/round"));
    }

    #[test]
    fn workspace_config_error_json_entry_uses_stable_codes_and_config_path() {
        let config_path = PathBuf::from("/tmp/spec.toml");
        let cases = vec![
            (
                WorkspaceConfigError::LibraryPathNotFound {
                    config_path: config_path.clone(),
                    alias: "shared".to_string(),
                    candidate: PathBuf::from("/tmp/missing-spec"),
                },
                "SPEC_LIBRARY_PATH_NOT_FOUND",
            ),
            (
                WorkspaceConfigError::LibraryOutOfRoot {
                    config_path: config_path.clone(),
                    alias: "shared".to_string(),
                    resolved_root: PathBuf::from("/outside/shared-spec"),
                },
                "SPEC_LIBRARY_OUT_OF_ROOT",
            ),
            (
                WorkspaceConfigError::LibraryAliasSelf {
                    config_path: config_path.clone(),
                    alias: "app".to_string(),
                },
                "SPEC_LIBRARY_ALIAS_SELF",
            ),
            (
                WorkspaceConfigError::DuplicateLibraryRoot {
                    config_path: config_path.clone(),
                    existing_alias: "shared".to_string(),
                    alias: "shared_copy".to_string(),
                    resolved_root: PathBuf::from("/repo/shared-spec"),
                },
                "SPEC_DUPLICATE_LIBRARY_ROOT",
            ),
        ];

        for (err, expected_code) in cases {
            let entry = workspace_config_error_to_json_entry(&err);
            assert_eq!(entry.unit, None);
            assert_eq!(entry.code, expected_code);
            assert_eq!(entry.path.as_deref(), Some("/tmp/spec.toml"));
            assert_eq!(
                entry.message.as_deref(),
                Some(err.detail_message().as_str())
            );
        }
    }

    #[test]
    fn compute_health_status_uses_shared_legacy_passport_freshness_resolution() {
        let original = LoadedSpec {
            source: spec_core::types::SpecSource {
                file_path: "units/pricing/apply_tax.unit.spec".to_string(),
                id: "pricing/apply_tax".to_string(),
            },
            spec: spec_core::types::SpecStruct {
                id: "pricing/apply_tax".to_string(),
                kind: "function".to_string(),
                intent: spec_core::types::Intent {
                    why: "apply tax".to_string(),
                },
                contract: Some(spec_core::types::Contract {
                    inputs: Some(
                        [("subtotal".to_string(), "i32".to_string())]
                            .into_iter()
                            .collect(),
                    ),
                    returns: Some("i32".to_string()),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![],
                body: spec_core::types::Body {
                    rust: "{ subtotal }".to_string(),
                },
                local_tests: vec![spec_core::types::LocalTest {
                    id: "basic".to_string(),
                    expect: "true".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: spec_core::types::UnitExtensions::default(),
            },
        };
        let mut changed = original.clone();
        changed.spec.contract.as_mut().unwrap().returns = Some("i64".to_string());

        let mut legacy_passport = spec_core::passport::build_passport_with_evidence(
            &original,
            "2026-04-05T00:00:00Z",
            Some(spec_core::passport::PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![spec_core::passport::PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-05T00:00:00Z".to_string(),
                provenance: None,
            }),
            spec_core::passport::compute_contract_hash(&original),
        );
        legacy_passport.freshness = None;

        let freshness =
            spec_core::passport::resolve_passport_freshness(&changed, Some(&legacy_passport));
        let health = compute_health_status(&[], Some(&legacy_passport), freshness.as_ref());

        assert_eq!(health.status, HealthState::Stale);
        assert_eq!(
            health.reason.as_deref(),
            Some("authored truth changed since last test")
        );
    }

    #[test]
    fn escape_hatch_gate_demotes_only_otherwise_valid_units() {
        let valid_health = HealthStatus {
            status: HealthState::Valid,
            reason: None,
            evidence_at: Some("2026-04-21T00:00:00Z".to_string()),
        };
        let open_gate = EscapeHatchGate {
            status: EscapeHatchGateStatus::Open,
            required_surfaces: vec![
                spec_core::escape_hatch::EscapeHatchProofSurface::Atom,
                spec_core::escape_hatch::EscapeHatchProofSurface::Molecule,
            ],
            present_surfaces: vec![spec_core::escape_hatch::EscapeHatchProofSurface::Atom],
            missing_surfaces: vec![spec_core::escape_hatch::EscapeHatchProofSurface::Molecule],
            reason: Some("missing required escape-hatch proof: molecule".to_string()),
        };

        let demoted = apply_escape_hatch_gate_to_health(valid_health, Some(&open_gate));
        assert_eq!(demoted.status, HealthState::Incomplete);
        assert_eq!(
            demoted.reason.as_deref(),
            Some("missing required escape-hatch proof: molecule")
        );

        let stale_health = HealthStatus {
            status: HealthState::Stale,
            reason: Some("authored truth changed since last test".to_string()),
            evidence_at: Some("2026-04-21T00:00:00Z".to_string()),
        };
        let preserved = apply_escape_hatch_gate_to_health(stale_health, Some(&open_gate));
        assert_eq!(preserved.status, HealthState::Stale);
        assert_eq!(
            preserved.reason.as_deref(),
            Some("authored truth changed since last test")
        );
    }

    #[test]
    fn semantic_review_demotes_only_otherwise_valid_units() {
        let supported_incomplete_review = SemanticReview {
            verdict: spec_core::semantic_review::SemanticVerdict::UnderSpecified,
            compatibility_key: "data.checkout_quote.v1".to_string(),
            reason_codes: vec![spec_core::semantic_review::SemanticReasonCode::MissingSemanticMethods],
            summary: "authored semantic surfaces are too weak for honest evaluation".to_string(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: spec_core::semantic_review::EvaluatorScope::SupportedDataSurface,
        };
        let supported_failing_review = SemanticReview {
            verdict: spec_core::semantic_review::SemanticVerdict::SemanticDrift,
            compatibility_key: "data.checkout_quote.v1".to_string(),
            reason_codes: vec![
                spec_core::semantic_review::SemanticReasonCode::MethodBodyMissingCapBehavior,
            ],
            summary: "executable lowering contradicts authored semantic claims".to_string(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: spec_core::semantic_review::EvaluatorScope::SupportedDataSurface,
        };

        let valid_health = HealthStatus {
            status: HealthState::Valid,
            reason: None,
            evidence_at: Some("2026-04-21T00:00:00Z".to_string()),
        };
        let incomplete = apply_semantic_review_to_health(
            HealthStatus {
                status: valid_health.status,
                reason: valid_health.reason.clone(),
                evidence_at: valid_health.evidence_at.clone(),
            },
            Some(&supported_incomplete_review),
        );
        assert_eq!(incomplete.status, HealthState::Incomplete);
        assert_eq!(
            incomplete.reason.as_deref(),
            Some("semantic under-specified: authored semantic surfaces are too weak for honest evaluation")
        );

        let failing = apply_semantic_review_to_health(valid_health, Some(&supported_failing_review));
        assert_eq!(failing.status, HealthState::Failing);
        assert_eq!(
            failing.reason.as_deref(),
            Some("semantic drift: executable lowering contradicts authored semantic claims")
        );

        let stale_health = HealthStatus {
            status: HealthState::Stale,
            reason: Some("authored truth changed since last test".to_string()),
            evidence_at: Some("2026-04-21T00:00:00Z".to_string()),
        };
        let stale_preserved =
            apply_semantic_review_to_health(stale_health, Some(&supported_failing_review));
        assert_eq!(stale_preserved.status, HealthState::Stale);
        assert_eq!(
            stale_preserved.reason.as_deref(),
            Some("authored truth changed since last test")
        );

        let base_failure = HealthStatus {
            status: HealthState::Failing,
            reason: Some("build failed".to_string()),
            evidence_at: Some("2026-04-21T00:00:00Z".to_string()),
        };
        let failure_preserved =
            apply_semantic_review_to_health(base_failure, Some(&supported_incomplete_review));
        assert_eq!(failure_preserved.status, HealthState::Failing);
        assert_eq!(failure_preserved.reason.as_deref(), Some("build failed"));
    }

    #[test]
    fn plan_validate_has_failures_ignores_partial_impact_when_closure_is_closed() {
        let report = spec_core::plan::PlanReport {
            plan_id: "add-only".to_string(),
            computed_impact: spec_core::plan::PlanComputedImpact {
                status: spec_core::plan::PlanComputedImpactStatus::Partial,
                units: vec![],
                molecule_tests: vec![],
                unresolved: vec![spec_core::plan::PlanUnresolvedImpact {
                    unit: "pricing/tiered_rate".to_string(),
                    action: spec_core::plan::PlanChangeAction::Add,
                    reason: "current graph has no node for action=add".to_string(),
                }],
            },
            acceptance_closure: spec_core::plan::PlanAcceptanceClosure {
                status: spec_core::plan::PlanAcceptanceClosureStatus::Closed,
                missing_validate: vec![],
                missing_molecule_tests: vec![],
                extra_validate: vec!["pricing/tiered_rate".to_string()],
                extra_molecule_tests: vec![],
            },
            change_reports: vec![],
        };

        assert!(!plan_validate_has_failures(&report));
    }

    #[test]
    fn output_module_prefix_absolute_crate_root_strips_src() {
        // Primary production path: absolute crate_root + absolute output under {crate_root}/src/
        let crate_root = Path::new("/home/user/myproject");
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/generated"),
                crate_root,
                Path::new("/home/user")
            )
            .unwrap(),
            "generated"
        );
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/generated/spec"),
                crate_root,
                Path::new("/home/user")
            )
            .unwrap(),
            "generated::spec"
        );
        assert_eq!(
            output_module_prefix(
                &PathBuf::from("/home/user/myproject/src/api/gen"),
                crate_root,
                Path::new("/home/user")
            )
            .unwrap(),
            "api::gen"
        );
    }

    #[test]
    fn output_module_prefix_relative_path_fallback_strips_src_component() {
        let cwd = Path::new("/repo");
        let crate_root = Path::new("/repo/examples/ecommerce");
        assert_eq!(
            output_module_prefix(Path::new("src/generated"), crate_root, cwd).unwrap(),
            "generated"
        );
        assert_eq!(
            output_module_prefix(Path::new("src/generated/spec"), crate_root, cwd).unwrap(),
            "generated::spec"
        );
    }

    #[test]
    fn output_module_prefix_no_src_prefix_preserved() {
        // Output not under src/ — kept as-is (user likely set generated_module_prefix explicitly)
        let crate_root = Path::new("/home/user/myproject");
        assert_eq!(
            output_module_prefix(Path::new("generated"), crate_root, Path::new("")).unwrap(),
            "generated"
        );
    }

    #[test]
    fn build_test_evidence_preserves_found_missing_and_duplicate_statuses() {
        let output = Path::new("src/generated");
        let crate_root = Path::new("");
        let spec = benchmark_loaded_spec(0, 3);
        let resolved = ResolvedSpec::from_spec(spec.spec.clone());
        let output_prefix = output_module_prefix(output, crate_root, Path::new("")).unwrap();

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
        let output_prefix = output_module_prefix(output, crate_root, Path::new("")).unwrap();
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
