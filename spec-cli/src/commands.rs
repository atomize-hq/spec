use crate::config::load_workspace_config;
use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use spec_core::generator::{
    clean_output_dir, generate_code, generate_mod_rs, safe_output_path, write_generated_file,
};
use spec_core::loader::{is_unit_spec, load_directory_report, load_file};
use spec_core::normalizer::normalize_spec;
use spec_core::passport::{build_passport, ensure_gitignore_entry, rfc3339_now, write_passport};
use spec_core::types::{LoadedSpec, ResolvedSpec};
use spec_core::validator::{
    ValidationOptions, check_spec_versions, validate_deps_exist_with_options,
    validate_full_with_options, validate_no_duplicate_ids,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

type CollectedSpecs = (
    Vec<LoadedSpec>,
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, Vec<String>>,
    usize,
);
type DiagnosticMap = BTreeMap<String, Vec<String>>;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Validate .unit.spec files")]
    Validate(ValidateArgs),
    #[command(about = "Generate Rust source files from .unit.spec files")]
    Generate(GenerateArgs),
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Validate(args) => validate_command(&args.path, args.no_strict),
            Self::Generate(args) => generate_command(&args.path, &args.output, args.no_strict),
        }
    }
}

#[derive(Args, Debug)]
pub struct ValidateArgs {
    pub path: PathBuf,
    #[arg(long)]
    pub no_strict: bool,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub path: PathBuf,
    #[arg(long, default_value = "generated/spec")]
    pub output: PathBuf,
    #[arg(long)]
    pub no_strict: bool,
}

fn validate_command(path: &Path, no_strict: bool) -> Result<()> {
    let (specs, errors, mut warnings, total_files) = collect_specs(path)?;
    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: !no_strict,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (errors, validation_warnings) = finish_validation(&specs, errors, &validation_options);
    merge_diagnostics(&mut warnings, validation_warnings);
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

fn generate_command(path: &Path, output: &Path, no_strict: bool) -> Result<()> {
    if no_strict {
        bail!(
            "❌ --no-strict is not valid for spec generate — use spec validate to check without strict enforcement"
        );
    }

    let (specs, errors, mut warnings, total_files) = collect_specs(path)?;
    if total_files == 0 {
        if !warnings.is_empty() {
            print_diagnostics(&warnings);
        }
        println!("0 units found, nothing to generate.");
        return Ok(());
    }

    let config = load_workspace_config(path)?;
    let validation_options = ValidationOptions {
        strict_deps: true,
        allow_unsafe_local_test_expect: config.validation.allow_unsafe_local_test_expect,
    };
    let (errors, validation_warnings) = finish_validation(&specs, errors, &validation_options);
    merge_diagnostics(&mut warnings, validation_warnings);
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

    for spec in &resolved_specs {
        let content = generate_code(spec)
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

    // Passport phase: only reached after all generation succeeds (atomicity guarantee).
    let generated_at = rfc3339_now();
    for spec in &specs {
        let passport = build_passport(spec, &generated_at);
        let source_path = Path::new(&spec.source.file_path);
        write_passport(&passport, source_path)
            .with_context(|| format!("Failed to write passport for {}", spec.source.id))?;
    }
    let gitignore_root = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    ensure_gitignore_entry(gitignore_root)
        .with_context(|| "Failed to update .gitignore for passport files")?;

    println!(
        "Generated {} file{}",
        resolved_specs.len() + namespaces.len(),
        pluralize(resolved_specs.len() + namespaces.len())
    );
    Ok(())
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
            Ok(spec) => Ok((vec![spec], BTreeMap::new(), BTreeMap::new(), total_files)),
            Err(err) => {
                let mut errors = BTreeMap::new();
                errors.insert(path.display().to_string(), vec![err.to_string()]);
                Ok((Vec::new(), errors, BTreeMap::new(), total_files))
            }
        };
    }

    if !path.is_dir() {
        bail!("{} does not exist", path.display());
    }

    let report = load_directory_report(path);
    let mut errors = BTreeMap::<String, Vec<String>>::new();
    let mut warnings = BTreeMap::<String, Vec<String>>::new();

    for err in report.errors {
        push_error(&mut errors, err);
    }
    for warning in report.warnings {
        push_warning(&mut warnings, warning);
    }

    Ok((report.specs, errors, warnings, report.total_files))
}

fn finish_validation(
    specs: &[LoadedSpec],
    mut errors: DiagnosticMap,
    options: &ValidationOptions,
) -> (DiagnosticMap, DiagnosticMap) {
    let mut warnings = DiagnosticMap::new();

    for err in validate_no_duplicate_ids(specs) {
        push_error(&mut errors, err);
    }

    for spec in specs {
        if let Err(err) = validate_full_with_options(spec, options) {
            push_error(&mut errors, err);
        }
    }

    let (dep_errors, dep_warnings) = validate_deps_exist_with_options(specs, options);
    for err in dep_errors {
        push_error(&mut errors, err);
    }
    for warning in dep_warnings {
        push_warning(&mut warnings, warning);
    }

    for warning in check_spec_versions(specs) {
        push_warning(&mut warnings, warning);
    }

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
        | spec_core::SpecWarning::MissingSpecVersion { path } => path.clone(),
    }
}

fn merge_diagnostics(target: &mut DiagnosticMap, source: DiagnosticMap) {
    for (path, mut messages) in source {
        target.entry(path).or_default().append(&mut messages);
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
    use tempfile::TempDir;

    fn write_spec(dir: &Path, relative_path: &str, body: &str) {
        let path = dir.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
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

        generate_command(&units_dir, &output_dir, false).unwrap();

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

        let result = validate_command(&units_dir, false);
        assert!(result.is_err());
        let error_text = format!("{:#}", result.unwrap_err());
        assert!(error_text.contains("error"));
    }
}
