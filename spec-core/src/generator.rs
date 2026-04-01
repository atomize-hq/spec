//! Generator module: Generate Rust code from ResolvedSpec
//!
//! Implements the M1 generation path from PLAN.md:
//! - prepend `use crate::...` imports for deps
//! - write generated `.rs` files
//! - generate `mod.rs` contents
//! - scoped clean with `.spec-generated` marker safety rails

use crate::types::ResolvedSpec;
use crate::{Result, SpecError};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

const GENERATED_MARKER: &str = ".spec-generated";

pub fn generate_code(spec: &ResolvedSpec) -> Result<String> {
    let use_statements = build_use_statements(spec)?;
    let mut output = String::new();

    for statement in use_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !spec.deps.is_empty() {
        output.push('\n');
    }

    output.push_str(spec.body_rust.trim_end_matches('\n'));
    output.push('\n');
    Ok(output)
}

pub fn write_generated_file(output_path: &str, content: &str) -> Result<()> {
    let path = Path::new(output_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| SpecError::OutputDir {
            message: format!(
                "Unable to create output directory {}: {}",
                parent.display(),
                err
            ),
        })?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .map_err(|err| SpecError::Generator {
            message: format!("Unable to open {} for writing: {}", path.display(), err),
        })?;

    file.write_all(content.as_bytes())
        .map_err(|err| SpecError::Generator {
            message: format!("Unable to write {}: {}", path.display(), err),
        })?;

    if !content.ends_with('\n') {
        file.write_all(b"\n").map_err(|err| SpecError::Generator {
            message: format!("Unable to finalize {}: {}", path.display(), err),
        })?;
    }

    file.flush().map_err(|err| SpecError::Generator {
        message: format!("Unable to flush {}: {}", path.display(), err),
    })?;

    Ok(())
}

pub fn clean_output_dir(output_base: &str, module_paths: &[String]) -> Result<()> {
    let base = normalized_absolute_path(output_base);
    let project_root = normalized_absolute_path(".");

    if !base.starts_with(&project_root) {
        return Err(SpecError::OutputDir {
            message: format!(
                "Refusing to clean {}: output path is outside the project root {}",
                base.display(),
                project_root.display()
            ),
        });
    }

    let marker = base.join(GENERATED_MARKER);
    if !marker.exists() {
        return Err(SpecError::MissingMarker {
            path: base.display().to_string(),
        });
    }

    let mut targets = normalize_target_dirs(&base, module_paths)?;
    targets.sort();
    targets.dedup();
    targets = prune_nested_targets(targets);

    for target in targets {
        if !target.exists() {
            continue;
        }

        for entry in WalkDir::new(&target).follow_links(false) {
            let entry = entry.map_err(SpecError::from)?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                fs::remove_file(path).map_err(|err| SpecError::Generator {
                    message: format!("Unable to remove {}: {}", path.display(), err),
                })?;
            }
        }
    }

    File::create(&marker).map_err(|err| SpecError::Generator {
        message: format!("Unable to recreate marker {}: {}", marker.display(), err),
    })?;

    Ok(())
}

pub fn generate_mod_rs(
    _output_base: &str,
    _module_path: &str,
    unit_files: &[String],
    subdirs: &[String],
) -> Result<String> {
    let mut seen = HashSet::new();
    let mut unit_mods = Vec::new();
    let mut subdir_mods = Vec::new();

    for unit_file in unit_files {
        if let Some(name) = module_item_name(unit_file) {
            let decl = format!("pub mod {};", name);
            if seen.insert(decl.clone()) {
                unit_mods.push(decl);
            }
        }
    }

    for subdir in subdirs {
        if let Some(name) = module_item_name(subdir) {
            let decl = format!("pub mod {};", name);
            if seen.insert(decl.clone()) {
                subdir_mods.push(decl);
            }
        }
    }

    unit_mods.sort();
    subdir_mods.sort();

    let mut output = String::new();
    for line in &unit_mods {
        output.push_str(line);
        output.push('\n');
    }

    if !unit_mods.is_empty() && !subdir_mods.is_empty() {
        output.push('\n');
    }

    for line in &subdir_mods {
        output.push_str(line);
        output.push('\n');
    }

    Ok(output)
}

fn build_use_statements(spec: &ResolvedSpec) -> Result<Vec<String>> {
    if let Some((dep1, dep2)) = ResolvedSpec::has_dep_collision(&spec.deps) {
        return Err(SpecError::DepCollision {
            dep1: dep1.clone(),
            dep2: dep2.clone(),
            fn_name: ResolvedSpec::dep_fn_name(dep1).to_string(),
            path: spec.id.clone(),
        });
    }

    let mut seen = HashSet::new();
    let mut statements = Vec::new();

    for dep in &spec.deps {
        if seen.insert(dep.clone()) {
            statements.push(format!("use {}", ResolvedSpec::dep_to_use_path(dep)));
        }
    }

    Ok(statements)
}

fn module_item_name(fragment: &str) -> Option<String> {
    Path::new(fragment)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.trim_end_matches(".rs").to_string())
        .filter(|name| !name.is_empty())
}

fn normalize_target_dirs(base: &Path, module_paths: &[String]) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    if module_paths.is_empty() {
        dirs.push(base.to_path_buf());
        return Ok(dirs);
    }

    for module_path in module_paths {
        dirs.push(join_module_path(base, module_path)?);
    }

    Ok(dirs)
}

fn prune_nested_targets(mut targets: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut pruned = Vec::new();

    'outer: for target in targets.drain(..) {
        for existing in &pruned {
            if target.starts_with(existing) {
                continue 'outer;
            }
        }
        pruned.push(target);
    }

    pruned
}

fn join_module_path(base: &Path, module_path: &str) -> Result<PathBuf> {
    let mut path = base.to_path_buf();
    for segment in module_path.split('/').filter(|segment| !segment.is_empty()) {
        if segment == "." || segment == ".." {
            return Err(SpecError::OutputDir {
                message: format!(
                    "Refusing to clean {}: invalid module path '{}'",
                    base.display(),
                    module_path
                ),
            });
        }
        path.push(segment);
    }

    let normalized = normalized_absolute_path(path);
    if !normalized.starts_with(base) {
        return Err(SpecError::OutputDir {
            message: format!(
                "Refusing to clean {}: resolved module path '{}' escaped the output base",
                normalized.display(),
                module_path
            ),
        });
    }

    Ok(normalized)
}

fn normalized_absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut normalized = if path.is_absolute() {
        PathBuf::new()
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(segment) => normalized.push(segment),
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Intent, ResolvedSpec, SpecStruct};
    #[cfg(unix)]
    use std::os::unix::fs as unix_fs;
    use tempfile::TempDir;

    fn test_spec(deps: Vec<&str>, body: &str) -> ResolvedSpec {
        ResolvedSpec::from_spec(SpecStruct {
            id: "pricing/apply_discount".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Apply a percentage discount.".to_string(),
            },
            contract: None,
            deps: deps.into_iter().map(|dep| dep.to_string()).collect(),
            body: Body {
                rust: body.to_string(),
            },
            local_tests: vec![],
            links: None,
        })
    }

    #[test]
    fn test_generate_code_prepends_use_statements() {
        let spec = test_spec(
            vec!["money/round", "utils/math/normalize"],
            "pub fn apply_discount() -> Decimal {\n    round(Decimal::ONE)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use crate::money::round::round;\nuse crate::utils::math::normalize::normalize;\n\npub fn apply_discount() -> Decimal {\n    round(Decimal::ONE)\n}\n"
        );
    }

    #[test]
    fn test_generate_code_rejects_dep_collision() {
        let spec = test_spec(
            vec!["money/round", "utils/round"],
            "pub fn apply_discount() -> Decimal {\n    round(Decimal::ONE)\n}",
        );

        let err = generate_code(&spec).unwrap_err();
        assert!(err.to_string().contains("Dep fn_name collision"));
    }

    #[test]
    fn test_write_generated_file_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir
            .path()
            .join("generated/spec/pricing/apply_discount.rs");

        write_generated_file(
            file_path.to_str().unwrap(),
            "pub fn apply_discount() -> Decimal { Decimal::ZERO }\n",
        )
        .unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            contents,
            "pub fn apply_discount() -> Decimal { Decimal::ZERO }\n"
        );
    }

    #[test]
    fn test_generate_mod_rs_lists_units_and_subdirs() {
        let content = generate_mod_rs(
            "./generated/spec",
            "pricing",
            &["apply_discount.rs".to_string(), "refund.rs".to_string()],
            &["taxes".to_string(), "discounts".to_string()],
        )
        .unwrap();

        assert_eq!(
            content,
            "pub mod apply_discount;\npub mod refund;\n\npub mod discounts;\npub mod taxes;\n"
        );
    }

    #[test]
    fn test_clean_output_dir_scoped_and_marker_safe() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        let pricing = base.join("pricing");
        let money = base.join("money");
        fs::create_dir_all(&pricing).unwrap();
        fs::create_dir_all(&money).unwrap();
        fs::write(base.join(GENERATED_MARKER), "").unwrap();
        fs::write(pricing.join("apply_discount.rs"), "fn a() {}\n").unwrap();
        fs::write(pricing.join("mod.rs"), "pub mod apply_discount;\n").unwrap();
        fs::write(money.join("round.rs"), "fn r() {}\n").unwrap();

        clean_output_dir(base.to_str().unwrap(), &[String::from("pricing")]).unwrap();

        assert!(!pricing.join("apply_discount.rs").exists());
        assert!(!pricing.join("mod.rs").exists());
        assert!(money.join("round.rs").exists());
        assert!(base.join(GENERATED_MARKER).exists());
    }

    #[test]
    fn test_clean_output_dir_refuses_without_marker() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        fs::create_dir_all(&base).unwrap();

        let err = clean_output_dir(base.to_str().unwrap(), &[String::from("pricing")]).unwrap_err();
        assert!(matches!(err, SpecError::MissingMarker { .. }));
    }

    #[test]
    #[cfg(unix)]
    fn test_clean_output_dir_does_not_follow_symlink_dirs() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        let pricing = base.join("pricing");
        fs::create_dir_all(&pricing).unwrap();
        fs::write(base.join(GENERATED_MARKER), "").unwrap();
        fs::write(pricing.join("apply_discount.rs"), "fn a() {}\n").unwrap();

        let outside_dir = temp_dir.path().join("outside");
        fs::create_dir_all(&outside_dir).unwrap();
        let outside_rs = outside_dir.join("outside.rs");
        fs::write(&outside_rs, "fn outside() {}\n").unwrap();

        unix_fs::symlink(&outside_dir, pricing.join("link")).unwrap();

        clean_output_dir(base.to_str().unwrap(), &[String::from("pricing")]).unwrap();

        assert!(!pricing.join("apply_discount.rs").exists());
        assert!(
            outside_rs.exists(),
            "clean_output_dir must not delete files through symlinks"
        );
        assert!(base.join(GENERATED_MARKER).exists());
    }
}
