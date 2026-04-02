use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use spec_core::generator::{
    clean_output_dir, generate_code, generate_mod_rs, write_generated_file,
};
use spec_core::loader::{is_unit_spec, load_file};
use spec_core::normalizer::normalize_spec;
use spec_core::types::{LoadedSpec, ResolvedSpec};
use spec_core::validator::{validate_full, validate_no_duplicate_ids};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

type CollectedSpecs = (Vec<LoadedSpec>, BTreeMap<String, Vec<String>>, usize);

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
            Self::Validate(args) => validate_command(&args.path),
            Self::Generate(args) => generate_command(&args.path, &args.output),
        }
    }
}

#[derive(Args, Debug)]
pub struct ValidateArgs {
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub path: PathBuf,
    #[arg(long, default_value = "generated/spec")]
    pub output: PathBuf,
}

fn validate_command(path: &Path) -> Result<()> {
    let (specs, errors, total_files) = collect_specs(path)?;
    let errors = finish_validation(specs, errors);

    if errors.is_empty() {
        if total_files == 0 {
            println!("0 units found, nothing to validate.");
        } else {
            println!("✅ {total_files} unit{} valid", pluralize(total_files));
        }
        return Ok(());
    }

    print_errors(&errors);
    let file_count = count_unique_files(&errors);
    bail!(
        "❌ {} file{}, {} error{}",
        file_count,
        pluralize(file_count),
        count_errors(&errors),
        pluralize(count_errors(&errors))
    );
}

fn generate_command(path: &Path, output: &Path) -> Result<()> {
    let (specs, errors, total_files) = collect_specs(path)?;
    if total_files == 0 {
        println!("0 units found, nothing to generate.");
        return Ok(());
    }

    let errors = finish_validation(specs.clone(), errors);
    if !errors.is_empty() {
        print_errors(&errors);
        let file_count = count_unique_files(&errors);
        bail!(
            "❌ {} file{}, {} error{}",
            file_count,
            pluralize(file_count),
            count_errors(&errors),
            pluralize(count_errors(&errors))
        );
    }

    let mut resolved_specs = Vec::new();
    for spec in specs {
        resolved_specs.push(
            normalize_spec(spec.spec)
                .with_context(|| format!("Failed to normalize {}", spec.source.file_path))?,
        );
    }

    let module_paths: Vec<String> = resolved_specs
        .iter()
        .map(|spec| spec.module_path.clone())
        .collect();

    ensure_output_marker(output)?;
    clean_output_dir(&output.display().to_string(), &module_paths)
        .with_context(|| format!("Failed to clean output directory {}", output.display()))?;

    for spec in &resolved_specs {
        let content = generate_code(spec)
            .with_context(|| format!("Failed to generate Rust for {}", spec.id))?;
        let output_path = output.join(path_for_spec(spec));
        write_generated_file(&output_path.display().to_string(), &content)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;
    }

    for (module_path, namespace) in build_namespaces(&resolved_specs) {
        let content = generate_mod_rs(
            &namespace.unit_files.into_iter().collect::<Vec<_>>(),
            &namespace.subdirs.into_iter().collect::<Vec<_>>(),
        )
        .with_context(|| format!("Failed to generate mod.rs for module '{module_path}'"))?;

        let mod_rs_path = if module_path.is_empty() {
            output.join("mod.rs")
        } else {
            output
                .join(module_path.replace('/', std::path::MAIN_SEPARATOR_STR))
                .join("mod.rs")
        };

        write_generated_file(&mod_rs_path.display().to_string(), &content)
            .with_context(|| format!("Failed to write {}", mod_rs_path.display()))?;
    }

    println!(
        "Generated {} file{}",
        resolved_specs.len(),
        pluralize(resolved_specs.len())
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

fn ensure_output_marker(output: &Path) -> Result<()> {
    let marker = output.join(".spec-generated");
    if marker.exists() {
        return Ok(());
    }

    if !output.exists() {
        fs::create_dir_all(output)
            .with_context(|| format!("Failed to create output directory {}", output.display()))?;
    }

    fs::write(&marker, "")
        .with_context(|| format!("Failed to create marker {}", marker.display()))?;
    Ok(())
}

fn collect_specs(path: &Path) -> Result<CollectedSpecs> {
    if path.is_file() {
        let total_files = usize::from(is_unit_spec(path));
        if !is_unit_spec(path) {
            bail!("{} is not a .unit.spec file", path.display());
        }
        return match load_file(path) {
            Ok(spec) => Ok((vec![spec], BTreeMap::new(), total_files)),
            Err(err) => {
                let mut errors = BTreeMap::new();
                errors.insert(path.display().to_string(), vec![err.to_string()]);
                Ok((Vec::new(), errors, total_files))
            }
        };
    }

    if !path.is_dir() {
        bail!("{} does not exist", path.display());
    }

    let mut specs = Vec::new();
    let mut errors = BTreeMap::<String, Vec<String>>::new();
    let mut total_files = 0usize;

    for entry in WalkDir::new(path).follow_links(true) {
        let entry = entry?;
        let entry_path = entry.path();
        if !entry_path.is_file() || !is_unit_spec(entry_path) {
            continue;
        }

        total_files += 1;
        match load_file(entry_path) {
            Ok(spec) => specs.push(spec),
            Err(err) => errors
                .entry(entry_path.display().to_string())
                .or_default()
                .push(err.to_string()),
        }
    }

    specs.sort_by(|left, right| left.source.file_path.cmp(&right.source.file_path));
    Ok((specs, errors, total_files))
}

fn finish_validation(
    specs: Vec<LoadedSpec>,
    mut errors: BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    if let Err(err) = validate_no_duplicate_ids(&specs) {
        let key = duplicate_path_key(&err);
        errors.entry(key).or_default().push(err.to_string());
    }

    for spec in specs {
        if let Err(err) = validate_full(&spec) {
            errors
                .entry(spec.source.file_path.clone())
                .or_default()
                .push(err.to_string());
        }
    }

    errors
}

fn duplicate_path_key(err: &spec_core::SpecError) -> String {
    match err {
        spec_core::SpecError::DuplicateId { file1, file2, .. } => format!("{file1} | {file2}"),
        _ => "validation".to_string(),
    }
}

fn print_errors(errors: &BTreeMap<String, Vec<String>>) {
    for (path, path_errors) in errors {
        eprintln!("{path}:");
        for error in path_errors {
            eprintln!("  - {error}");
        }
    }
}

fn count_errors(errors: &BTreeMap<String, Vec<String>>) -> usize {
    errors.values().map(Vec::len).sum()
}

fn count_unique_files(errors: &BTreeMap<String, Vec<String>>) -> usize {
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
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount() -> Decimal {
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

        let result = validate_command(&units_dir);
        assert!(result.is_err());
        let error_text = format!("{:#}", result.unwrap_err());
        assert!(error_text.contains("error"));
    }
}
