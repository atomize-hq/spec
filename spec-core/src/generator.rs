//! Generator module: Generate Rust code from ResolvedSpec
//!
//! Implements the M1 generation path from PLAN.md:
//! - prepend `use ...` imports for imports + deps
//! - write generated `.rs` files
//! - generate `mod.rs` contents
//! - owned-tree orphan cleanup with `.spec-generated` marker safety rails

use crate::types::ResolvedSpec;
use crate::{Result, SpecError};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

const GENERATED_MARKER: &str = ".spec-generated";

fn build_fn_signature(spec: &ResolvedSpec) -> String {
    let params = spec
        .contract
        .as_ref()
        .and_then(|c| c.inputs.as_ref())
        .map(|inputs| {
            inputs
                .iter()
                .map(|(name, ty)| format!("{name}: {ty}"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();

    let return_type = spec
        .contract
        .as_ref()
        .and_then(|c| c.returns.as_ref())
        .map(|r| format!(" -> {}", r));

    match return_type {
        Some(ret) => format!("pub fn {}({}){}", spec.fn_name, params, ret),
        None => format!("pub fn {}({})", spec.fn_name, params),
    }
}

pub fn generate_code(spec: &ResolvedSpec) -> Result<String> {
    let (import_statements, dep_statements) = build_use_groups(spec)?;
    let mut output = String::new();

    for statement in import_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !spec.imports.is_empty() && !spec.deps.is_empty() {
        output.push('\n');
    }

    for statement in dep_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !spec.imports.is_empty() || !spec.deps.is_empty() {
        output.push('\n');
    }

    let signature = build_fn_signature(spec);
    let block = spec.body_rust.trim();
    output.push_str(&format!("{signature} {block}"));
    output.push('\n');

    if !spec.local_tests.is_empty() {
        // One blank line between the generated unit body and the tests module.
        output.push('\n');
        output.push_str("#[cfg(test)]\n");
        output.push_str("mod tests {\n");
        output.push_str("    use super::*;\n\n");

        for (index, local_test) in spec.local_tests.iter().enumerate() {
            let expect = local_test.expect.trim();
            output.push_str("    #[test]\n");
            output.push_str(&format!("    fn test_{}() {{\n", local_test.id));
            output.push_str(&format!("        assert!({expect});\n"));
            output.push_str("    }\n");

            if index + 1 != spec.local_tests.len() {
                output.push('\n');
            }
        }

        output.push_str("}\n");
    }
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
    } else {
        return Err(SpecError::Generator {
            message: format!(
                "Unable to write {}: missing parent directory",
                path.display()
            ),
        });
    }

    let parent_dir = path
        .parent()
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "Unable to write {}: missing parent directory",
                path.display()
            ),
        })?
        .to_path_buf();

    // Write to a temp file in the same directory and rename into place (per-file atomic).
    let mut tmp = tempfile::Builder::new()
        .prefix(".spec-tmp-")
        .suffix(".tmp")
        .tempfile_in(&parent_dir)
        .map_err(|err| SpecError::Generator {
            message: format!(
                "Unable to create temp file in {}: {}",
                parent_dir.display(),
                err
            ),
        })?;

    tmp.write_all(content.as_bytes())
        .map_err(|err| SpecError::Generator {
            message: format!("Unable to write temp file for {}: {}", path.display(), err),
        })?;

    if !content.ends_with('\n') {
        tmp.write_all(b"\n").map_err(|err| SpecError::Generator {
            message: format!(
                "Unable to finalize temp file for {}: {}",
                path.display(),
                err
            ),
        })?;
    }

    tmp.flush().map_err(|err| SpecError::Generator {
        message: format!("Unable to flush temp file for {}: {}", path.display(), err),
    })?;

    // On Windows, renaming over an existing file fails; remove it first.
    if cfg!(windows) && path.exists() {
        fs::remove_file(path).map_err(|err| SpecError::Generator {
            message: format!("Unable to remove existing {}: {}", path.display(), err),
        })?;
    }

    let tmp_path = tmp.into_temp_path();
    fs::rename(&tmp_path, path).map_err(|err| SpecError::Generator {
        message: format!(
            "Unable to rename temp file into {}: {}",
            path.display(),
            err
        ),
    })?;

    Ok(())
}

pub fn clean_output_dir(
    output_base: &Path,
    generated_rs_rel_paths: &HashSet<PathBuf>,
) -> Result<()> {
    let base = safe_output_path(output_base)?;

    let marker = base.join(GENERATED_MARKER);
    if !marker.exists() {
        return Err(SpecError::MissingMarker {
            path: base.display().to_string(),
        });
    }

    // Remove orphaned `.rs` files (anything not in the generated set).
    for entry in WalkDir::new(&base).follow_links(false) {
        let entry = entry.map_err(SpecError::from)?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let rel = path
            .strip_prefix(&base)
            .map_err(|err| SpecError::Generator {
                message: format!(
                    "Unable to compute relative path for {}: {}",
                    path.display(),
                    err
                ),
            })?;

        if !generated_rs_rel_paths.contains(rel) {
            fs::remove_file(path).map_err(|err| SpecError::Generator {
                message: format!("Unable to remove {}: {}", path.display(), err),
            })?;
        }
    }

    // Remove empty directories bottom-up (but never remove the base itself).
    for entry in WalkDir::new(&base).follow_links(false).contents_first(true) {
        let entry = entry.map_err(SpecError::from)?;
        if !entry.file_type().is_dir() || entry.file_type().is_symlink() {
            continue;
        }
        let path = entry.path();
        if path == base {
            continue;
        }

        let mut entries = fs::read_dir(path).map_err(|err| SpecError::Generator {
            message: format!("Unable to read dir {}: {}", path.display(), err),
        })?;
        if entries.next().is_none() {
            fs::remove_dir(path).map_err(|err| SpecError::Generator {
                message: format!("Unable to remove dir {}: {}", path.display(), err),
            })?;
        }
    }

    File::create(&marker).map_err(|err| SpecError::Generator {
        message: format!("Unable to recreate marker {}: {}", marker.display(), err),
    })?;

    Ok(())
}

pub fn generate_mod_rs(unit_files: &[String], subdirs: &[String]) -> Result<String> {
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

fn build_use_groups(spec: &ResolvedSpec) -> Result<(Vec<String>, Vec<String>)> {
    if let Some((dep1, dep2)) = ResolvedSpec::has_dep_collision(&spec.deps) {
        return Err(SpecError::DepCollision {
            dep1: dep1.clone(),
            dep2: dep2.clone(),
            fn_name: ResolvedSpec::dep_fn_name(dep1).to_string(),
            path: spec.id.clone(),
        });
    }

    let mut import_seen = HashSet::new();
    let mut import_statements = Vec::new();

    for import_path in &spec.imports {
        if import_seen.insert(import_path.clone()) {
            import_statements.push(format!("use {};", import_path));
        }
    }

    let mut dep_seen = HashSet::new();
    let mut dep_statements = Vec::new();

    for dep in &spec.deps {
        if dep_seen.insert(dep.clone()) {
            dep_statements.push(format!("use {}", ResolvedSpec::dep_to_use_path(dep)));
        }
    }

    Ok((import_statements, dep_statements))
}

fn module_item_name(fragment: &str) -> Option<String> {
    Path::new(fragment)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.trim_end_matches(".rs").to_string())
        .filter(|name| !name.is_empty())
}

pub fn safe_output_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let project_root = canonicalize_existing_path(&std::env::current_dir().map_err(|err| {
        SpecError::OutputDir {
            message: format!("Unable to determine project root: {err}"),
        }
    })?)?;
    let output_base = canonicalize_output_path(path)?;

    if !output_base.starts_with(&project_root) {
        return Err(SpecError::OutputDir {
            message: format!(
                "Refusing to generate into {}: output path is outside the project root {}",
                output_base.display(),
                project_root.display()
            ),
        });
    }

    Ok(output_base)
}

pub fn normalized_absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
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

fn canonicalize_output_path(path: &Path) -> Result<PathBuf> {
    let absolute = normalized_absolute_path(path);
    if absolute.exists() {
        return canonicalize_existing_path(&absolute);
    }

    let mut current = absolute.as_path();
    let mut missing_segments = Vec::new();

    while !current.exists() {
        let segment = current.file_name().ok_or_else(|| SpecError::OutputDir {
            message: format!(
                "Unable to resolve output path {}: no existing ancestor found",
                absolute.display()
            ),
        })?;
        missing_segments.push(segment.to_os_string());
        current = current.parent().ok_or_else(|| SpecError::OutputDir {
            message: format!(
                "Unable to resolve output path {}: no existing ancestor found",
                absolute.display()
            ),
        })?;
    }

    let mut resolved = canonicalize_existing_path(current)?;
    for segment in missing_segments.iter().rev() {
        resolved.push(segment);
    }

    Ok(resolved)
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize().map_err(|err| SpecError::OutputDir {
        message: format!("Unable to canonicalize {}: {}", path.display(), err),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Intent, LocalTest, ResolvedSpec, SpecStruct};
    #[cfg(unix)]
    use std::os::unix::fs as unix_fs;
    use tempfile::TempDir;

    fn test_spec_with(deps: Vec<&str>, imports: Vec<&str>, body: &str) -> ResolvedSpec {
        ResolvedSpec::from_spec(SpecStruct {
            id: "pricing/apply_discount".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Apply a percentage discount.".to_string(),
            },
            contract: None,
            deps: deps.into_iter().map(|dep| dep.to_string()).collect(),
            imports: imports
                .into_iter()
                .map(|import| import.to_string())
                .collect(),
            body: Body {
                rust: body.to_string(),
            },
            local_tests: vec![],
            links: None,
            spec_version: None,
        })
    }

    fn test_spec(deps: Vec<&str>, body: &str) -> ResolvedSpec {
        test_spec_with(deps, vec![], body)
    }

    #[test]
    fn test_generate_code_prepends_use_statements() {
        let spec = test_spec(
            vec!["money/round", "utils/math/normalize"],
            "{\n    round(Decimal::ONE)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use crate::money::round::round;\nuse crate::utils::math::normalize::normalize;\n\npub fn apply_discount() {\n    round(Decimal::ONE)\n}\n"
        );
    }

    #[test]
    fn test_generate_code_rejects_dep_collision() {
        let spec = test_spec(
            vec!["money/round", "utils/round"],
            "{\n    round(Decimal::ONE)\n}",
        );

        let err = generate_code(&spec).unwrap_err();
        assert!(err.to_string().contains("Dep fn_name collision"));
    }

    #[test]
    fn imports_field_generates_correct_use_statement() {
        let spec = test_spec_with(
            vec![],
            vec!["rust_decimal::Decimal"],
            "{\n    Decimal::ZERO\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use rust_decimal::Decimal;\n\npub fn apply_discount() {\n    Decimal::ZERO\n}\n"
        );
    }

    #[test]
    fn deps_unchanged_after_imports_split() {
        let spec = test_spec_with(
            vec!["money/round"],
            vec![],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert!(code.contains("use crate::money::round::round;"));
    }

    #[test]
    fn imports_emitted_before_deps_in_use_statements() {
        let spec = test_spec_with(
            vec!["money/round"],
            vec!["rust_decimal::Decimal"],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use rust_decimal::Decimal;\n\nuse crate::money::round::round;\n\npub fn apply_discount() {\n    round(Decimal::ZERO)\n}\n"
        );
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
    fn generate_local_tests_produces_cfg_test_block() {
        let mut spec = test_spec_with(
            vec![],
            vec!["rust_decimal::Decimal"],
            "{\n    Decimal::ZERO\n}",
        );
        spec.local_tests = vec![LocalTest {
            id: "happy_path".to_string(),
            expect: "apply_discount() == Decimal::ZERO".to_string(),
        }];

        let code = generate_code(&spec).unwrap();
        assert!(code.contains("#[cfg(test)]\nmod tests {"));
        assert!(code.contains("use super::*;"));
        assert!(code.contains("fn test_happy_path() {"));
        assert!(code.contains("assert!(apply_discount() == Decimal::ZERO);"));
    }

    #[test]
    fn generate_no_local_tests_produces_no_test_block() {
        let spec = test_spec_with(vec![], vec![], "{ }");
        let code = generate_code(&spec).unwrap();
        assert!(!code.contains("#[cfg(test)]"));
        assert!(!code.contains("mod tests {"));
    }

    #[test]
    fn clean_output_dir_removes_stale_module_from_prior_run() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        let pricing = base.join("pricing");
        let test_mod = base.join("test");
        fs::create_dir_all(&pricing).unwrap();
        fs::create_dir_all(&test_mod).unwrap();
        fs::write(base.join(GENERATED_MARKER), "").unwrap();

        // "Current run" generated files.
        fs::write(base.join("mod.rs"), "pub mod pricing;\n").unwrap();
        fs::write(pricing.join("apply_discount.rs"), "fn a() {}\n").unwrap();
        fs::write(pricing.join("mod.rs"), "pub mod apply_discount;\n").unwrap();

        // Stale module files from a prior run should be removed (and the empty dir pruned).
        fs::write(test_mod.join("foo.rs"), "fn stale() {}\n").unwrap();
        fs::write(test_mod.join("mod.rs"), "pub mod foo;\n").unwrap();

        let mut generated = HashSet::new();
        generated.insert(PathBuf::from("pricing/apply_discount.rs"));
        generated.insert(PathBuf::from("pricing/mod.rs"));
        generated.insert(PathBuf::from("mod.rs"));

        clean_output_dir(&base, &generated).unwrap();

        assert!(pricing.join("apply_discount.rs").exists());
        assert!(pricing.join("mod.rs").exists());

        assert!(!test_mod.join("foo.rs").exists());
        assert!(!test_mod.join("mod.rs").exists());
        assert!(!test_mod.exists(), "stale module dir should be removed");

        assert!(base.join("mod.rs").exists());
        assert!(base.join(GENERATED_MARKER).exists());
    }

    #[test]
    fn test_clean_output_dir_refuses_without_marker() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        fs::create_dir_all(&base).unwrap();

        let generated = HashSet::new();
        let err = clean_output_dir(&base, &generated).unwrap_err();
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

        let generated = HashSet::new();
        clean_output_dir(&base, &generated).unwrap();

        assert!(!pricing.join("apply_discount.rs").exists());
        assert!(
            outside_rs.exists(),
            "clean_output_dir must not delete files through symlinks"
        );
        assert!(base.join(GENERATED_MARKER).exists());
    }

    #[test]
    fn safe_output_path_accepts_existing_path_inside_project_root() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let path = temp_dir.path().join("generated/spec");
        fs::create_dir_all(&path).unwrap();

        let resolved = safe_output_path(&path).unwrap();
        assert_eq!(resolved, path.canonicalize().unwrap());
    }

    #[test]
    fn safe_output_path_resolves_nonexistent_nested_path_from_existing_ancestor() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated");
        fs::create_dir_all(&base).unwrap();
        let path = base.join("spec/pricing");

        let resolved = safe_output_path(&path).unwrap();
        assert_eq!(
            resolved,
            base.canonicalize().unwrap().join("spec").join("pricing")
        );
    }

    #[test]
    #[cfg(unix)]
    fn safe_output_path_rejects_symlink_escape_in_nonexistent_path() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let outside = tempfile::TempDir::new().unwrap();
        let link = temp_dir.path().join("escape");
        unix_fs::symlink(outside.path(), &link).unwrap();

        let err = safe_output_path(link.join("generated/spec")).unwrap_err();
        assert!(err.to_string().contains("outside the project root"));
    }
}
