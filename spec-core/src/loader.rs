//! Loader module: Read and parse .unit.spec files from the filesystem
//!
//! Functions:
//! - Load single .unit.spec file
//! - Load directory recursively
//! - UTF-8 validation before YAML parsing
//! - Error tracking with file paths

use crate::plan::{LoadedPlan, PlanSource, PlanStruct, validate_raw_plan_yaml};
use crate::types::{
    LoadedMoleculeTest, LoadedSpec, MoleculeTestSource, MoleculeTestStruct, SpecSource, SpecStruct,
};
use crate::validator::{validate_raw_molecule_test_yaml, validate_raw_yaml};
use crate::{Result, SpecError};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

#[cfg(test)]
use crate::validator::validate_semantic;

/// Result of a collect-all directory load.
#[derive(Debug, Default)]
pub struct DirectoryLoadReport {
    pub specs: Vec<LoadedSpec>,
    pub errors: Vec<SpecError>,
    pub warnings: Vec<crate::SpecWarning>,
    pub total_files: usize,
}

/// Result of bounded library-root discovery under a search path.
#[derive(Debug, Default)]
pub struct LibraryRootDiscoveryReport {
    pub roots: Vec<PathBuf>,
    pub errors: Vec<SpecError>,
    pub warnings: Vec<crate::SpecWarning>,
}

fn canonicalize_scan_root(path: &Path) -> Result<PathBuf> {
    path.canonicalize().map_err(|err| SpecError::Traversal {
        message: err.to_string(),
        path: path.display().to_string(),
    })
}

fn canonicalize_scan_entry(path: &Path) -> Result<PathBuf> {
    path.canonicalize().map_err(|err| SpecError::Traversal {
        message: err.to_string(),
        path: path.display().to_string(),
    })
}

fn escaped_plan_scan_error(path: &Path) -> SpecError {
    SpecError::PlanSymlinkEscape {
        path: path.display().to_string(),
    }
}

fn subtree_already_rejected(path: &Path, rejected_roots: &[PathBuf]) -> bool {
    rejected_roots.iter().any(|root| path.starts_with(root))
}

fn should_descend_scan_entry(entry: &walkdir::DirEntry, root: &Path) -> bool {
    if entry.path() == root {
        return true;
    }

    let Some(name) = entry.file_name().to_str() else {
        return true;
    };

    !name.starts_with('.') && name != "target"
}

fn read_yaml_value<P: AsRef<Path>>(path: P) -> Result<(String, serde_yaml_bw::Value)> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy().to_string();

    // Read file as bytes for UTF-8 validation
    let bytes = fs::read(path)?;

    // Validate UTF-8
    if std::str::from_utf8(&bytes).is_err() {
        return Err(SpecError::InvalidUtf8 { path: path_str });
    }

    // Parse YAML to Value (preserves raw author input)
    let yaml_value: serde_yaml_bw::Value =
        serde_yaml_bw::from_slice(&bytes).map_err(|e| SpecError::YamlParse {
            message: e.to_string(),
            path: path_str.clone(),
        })?;

    Ok((path_str, yaml_value))
}

/// Load a single .unit.spec file
///
/// Returns the parsed SpecStruct with its source file information.
/// Performs UTF-8 validation before YAML parsing.
pub fn load_file<P: AsRef<Path>>(path: P) -> Result<LoadedSpec> {
    let (path_str, yaml_value) = read_yaml_value(path)?;

    // Validate the raw authored YAML before serde can normalize or drop fields.
    validate_raw_yaml(&yaml_value, &path_str)?;

    // Deserialize to SpecStruct
    let spec: SpecStruct =
        serde_yaml_bw::from_value(yaml_value).map_err(|e| SpecError::YamlParse {
            message: e.to_string(),
            path: path_str.clone(),
        })?;

    Ok(LoadedSpec {
        source: SpecSource {
            file_path: path_str,
            id: spec.id.clone(),
        },
        spec,
    })
}

/// Load a single `.plan.spec` file.
pub fn load_plan_file<P: AsRef<Path>>(path: P) -> Result<LoadedPlan> {
    let (path_str, yaml_value) = read_yaml_value(path)?;

    validate_raw_plan_yaml(&yaml_value, &path_str)?;

    let plan: PlanStruct =
        serde_yaml_bw::from_value(yaml_value).map_err(|e| SpecError::YamlParse {
            message: e.to_string(),
            path: path_str.clone(),
        })?;

    Ok(LoadedPlan {
        source: PlanSource {
            file_path: path_str,
            id: plan.id.clone(),
        },
        plan,
    })
}

/// Load all .unit.spec files from a directory recursively
///
/// Returns a vector of LoadedSpec, sorted by file path.
/// Non-.unit.spec files are skipped.
/// Empty directories return an empty vec (not an error).
pub fn load_directory<P: AsRef<Path>>(dir: P) -> Result<Vec<LoadedSpec>> {
    let report = load_directory_report(dir);
    if let Some(err) = report.errors.into_iter().next() {
        return Err(err);
    }
    Ok(report.specs)
}

/// Load all .unit.spec files from a directory recursively, collecting traversal
/// warnings and continuing past symlink cycles.
pub fn load_directory_report<P: AsRef<Path>>(dir: P) -> DirectoryLoadReport {
    let dir = dir.as_ref();
    let mut report = DirectoryLoadReport::default();

    for entry in WalkDir::new(dir).follow_links(true) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };

                if !name.ends_with(".unit.spec") {
                    continue;
                }

                report.total_files += 1;
                match load_file(path) {
                    Ok(spec) => report.specs.push(spec),
                    Err(err) => report.errors.push(err),
                }
            }
            Err(err) => {
                if let Some(warning) = walkdir_cycle_warning(&err) {
                    report.warnings.push(warning);
                } else {
                    report.errors.push(walkdir_error(err));
                }
            }
        }
    }

    report
        .specs
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    report
}

/// Load all `.unit.spec` files from a directory recursively while enforcing
/// that every visited file or followed symlink stays within `allowed_root`.
pub fn load_directory_report_bounded<P: AsRef<Path>, R: AsRef<Path>>(
    dir: P,
    allowed_root: R,
) -> Result<DirectoryLoadReport> {
    let dir = dir.as_ref();
    let allowed_root = canonicalize_scan_root(allowed_root.as_ref())?;
    let mut report = DirectoryLoadReport::default();
    let mut rejected_subtrees = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| should_descend_scan_entry(entry, dir))
    {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if subtree_already_rejected(path, &rejected_subtrees) {
                    continue;
                }

                let canonical_path = match canonicalize_scan_entry(path) {
                    Ok(path) => path,
                    Err(err) => {
                        report.errors.push(err);
                        continue;
                    }
                };

                if !canonical_path.starts_with(&allowed_root) {
                    report.errors.push(escaped_plan_scan_error(path));
                    if entry.file_type().is_dir() {
                        rejected_subtrees.push(path.to_path_buf());
                    }
                    continue;
                }

                if !path.is_file() {
                    continue;
                }

                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };

                if !name.ends_with(".unit.spec") {
                    continue;
                }

                report.total_files += 1;
                match load_file(path) {
                    Ok(spec) => report.specs.push(spec),
                    Err(err) => report.errors.push(err),
                }
            }
            Err(err) => {
                if let Some(warning) = walkdir_cycle_warning(&err) {
                    report.warnings.push(warning);
                } else {
                    report.errors.push(walkdir_error(err));
                }
            }
        }
    }

    report
        .specs
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    Ok(report)
}

/// Discover library roots while enforcing that both the candidate directory and its
/// owned `units/` directory stay within `allowed_root`, even when reached through symlinks.
pub fn discover_library_roots_bounded<P: AsRef<Path>, R: AsRef<Path>>(
    dir: P,
    allowed_root: R,
) -> Result<LibraryRootDiscoveryReport> {
    let dir = dir.as_ref();
    let allowed_root = canonicalize_scan_root(allowed_root.as_ref())?;
    let mut report = LibraryRootDiscoveryReport::default();
    let mut rejected_subtrees = Vec::new();
    let mut roots = BTreeSet::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| should_descend_scan_entry(entry, dir))
    {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if subtree_already_rejected(path, &rejected_subtrees) {
                    continue;
                }

                if !entry.file_type().is_dir() {
                    continue;
                }

                let canonical_path = match canonicalize_scan_entry(path) {
                    Ok(path) => path,
                    Err(err) => {
                        report.errors.push(err);
                        continue;
                    }
                };

                if !canonical_path.starts_with(&allowed_root) {
                    report.errors.push(escaped_plan_scan_error(path));
                    rejected_subtrees.push(path.to_path_buf());
                    continue;
                }

                let units_path = path.join("units");
                if !units_path.is_dir() {
                    continue;
                }

                let canonical_units = match canonicalize_scan_entry(&units_path) {
                    Ok(path) => path,
                    Err(err) => {
                        report.errors.push(err);
                        continue;
                    }
                };

                if !canonical_units.starts_with(&allowed_root) {
                    report.errors.push(escaped_plan_scan_error(&units_path));
                    continue;
                }

                roots.insert(canonical_path);
            }
            Err(err) => {
                if let Some(warning) = walkdir_cycle_warning(&err) {
                    report.warnings.push(warning);
                } else {
                    report.errors.push(walkdir_error(err));
                }
            }
        }
    }

    report.roots = roots.into_iter().collect();
    Ok(report)
}

/// Load all .unit.spec files from a directory recursively and collect all errors.
///
/// Unlike `load_directory`, this helper continues after failures so callers can
/// present grouped diagnostics for the full directory.
#[cfg(test)]
pub(crate) fn load_directory_collect_all<P: AsRef<Path>>(dir: P) -> DirectoryLoadReport {
    let mut report = load_directory_report(dir);
    let loaded_specs = std::mem::take(&mut report.specs);

    for spec in loaded_specs {
        match validate_semantic(&spec) {
            Ok(()) => report.specs.push(spec),
            Err(err) => report.errors.push(err),
        }
    }

    report
        .specs
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    report
}

fn walkdir_cycle_warning(err: &walkdir::Error) -> Option<crate::SpecWarning> {
    err.loop_ancestor()
        .map(|_| crate::SpecWarning::SymlinkCycleSkipped {
            path: err
                .path()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<unknown>".to_string()),
        })
}

fn walkdir_error(err: walkdir::Error) -> SpecError {
    SpecError::Traversal {
        message: err.to_string(),
        path: err
            .path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unknown>".to_string()),
    }
}

/// Result of a collect-all molecule test directory load.
#[derive(Debug, Default)]
pub struct MoleculeTestLoadReport {
    pub tests: Vec<LoadedMoleculeTest>,
    pub errors: Vec<SpecError>,
    pub warnings: Vec<crate::SpecWarning>,
    pub total_files: usize,
}

/// Load a single .test.spec file
///
/// Returns the parsed molecule test with its source file information.
/// Performs UTF-8 validation and schema validation before YAML parsing.
pub fn load_molecule_test_file<P: AsRef<Path>>(path: P) -> Result<LoadedMoleculeTest> {
    let (path_str, yaml_value) = read_yaml_value(path)?;

    // Validate against test.spec.json schema before serde can normalize or drop fields.
    validate_raw_molecule_test_yaml(&yaml_value, &path_str)?;

    // Deserialize to MoleculeTestStruct
    let test: MoleculeTestStruct =
        serde_yaml_bw::from_value(yaml_value).map_err(|e| SpecError::YamlParse {
            message: e.to_string(),
            path: path_str.clone(),
        })?;

    Ok(LoadedMoleculeTest {
        source: MoleculeTestSource {
            file_path: path_str,
            id: test.id.clone(),
        },
        test,
    })
}

/// Load all .test.spec files from a directory recursively (fail-fast).
///
/// Returns a vector of LoadedMoleculeTest, sorted by file path.
/// Non-.test.spec files are skipped.
/// Returns error on first failure.
pub fn load_molecule_test_directory<P: AsRef<Path>>(dir: P) -> Result<Vec<LoadedMoleculeTest>> {
    let report = load_molecule_test_directory_report(dir);
    if let Some(err) = report.errors.into_iter().next() {
        return Err(err);
    }
    Ok(report.tests)
}

/// Load all .test.spec files from a directory recursively, collecting all errors.
///
/// Unlike `load_molecule_test_directory`, continues past failures so callers can
/// present grouped diagnostics for the full directory (used by validate command).
pub fn load_molecule_test_directory_report<P: AsRef<Path>>(dir: P) -> MoleculeTestLoadReport {
    let dir = dir.as_ref();
    let mut report = MoleculeTestLoadReport::default();

    // If path is a file, skip molecule test loading (only directories are scanned)
    if dir.is_file() {
        return report;
    }

    for entry in WalkDir::new(dir).follow_links(true) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };

                if !name.ends_with(".test.spec") {
                    continue;
                }

                report.total_files += 1;
                match load_molecule_test_file(path) {
                    Ok(test) => report.tests.push(test),
                    Err(err) => report.errors.push(err),
                }
            }
            Err(err) => {
                if let Some(warning) = walkdir_cycle_warning(&err) {
                    report.warnings.push(warning);
                } else {
                    report.errors.push(walkdir_error(err));
                }
            }
        }
    }

    report
        .tests
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    report
}

/// Load all `.test.spec` files from a directory recursively while enforcing
/// that every visited file or followed symlink stays within `allowed_root`.
pub fn load_molecule_test_directory_report_bounded<P: AsRef<Path>, R: AsRef<Path>>(
    dir: P,
    allowed_root: R,
) -> Result<MoleculeTestLoadReport> {
    let dir = dir.as_ref();
    let allowed_root = canonicalize_scan_root(allowed_root.as_ref())?;
    let mut report = MoleculeTestLoadReport::default();
    let mut rejected_subtrees = Vec::new();

    if dir.is_file() {
        return Ok(report);
    }

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| should_descend_scan_entry(entry, dir))
    {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if subtree_already_rejected(path, &rejected_subtrees) {
                    continue;
                }

                let canonical_path = match canonicalize_scan_entry(path) {
                    Ok(path) => path,
                    Err(err) => {
                        report.errors.push(err);
                        continue;
                    }
                };

                if !canonical_path.starts_with(&allowed_root) {
                    report.errors.push(escaped_plan_scan_error(path));
                    if entry.file_type().is_dir() {
                        rejected_subtrees.push(path.to_path_buf());
                    }
                    continue;
                }

                if !path.is_file() {
                    continue;
                }

                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };

                if !name.ends_with(".test.spec") {
                    continue;
                }

                report.total_files += 1;
                match load_molecule_test_file(path) {
                    Ok(test) => report.tests.push(test),
                    Err(err) => report.errors.push(err),
                }
            }
            Err(err) => {
                if let Some(warning) = walkdir_cycle_warning(&err) {
                    report.warnings.push(warning);
                } else {
                    report.errors.push(walkdir_error(err));
                }
            }
        }
    }

    report
        .tests
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    Ok(report)
}

/// Check if a path is a .unit.spec file
pub fn is_unit_spec(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.ends_with(".unit.spec"))
        .unwrap_or(false)
}

/// Check if a path is a `.test.spec` file.
pub fn is_molecule_test_spec(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.ends_with(".test.spec"))
        .unwrap_or(false)
}

/// Get the output directory for a generated file based on its module path
///
/// Returns the directory path where the .rs file should be written.
/// E.g., for ID "pricing/apply_discount" with output base "./generated/spec",
/// returns "./generated/spec/pricing"
#[cfg(test)]
pub(crate) fn output_dir_for_spec(output_base: impl AsRef<Path>, module_path: &str) -> PathBuf {
    let mut path = output_base.as_ref().to_path_buf();
    if !module_path.is_empty() {
        path = path.join(module_path.replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    path
}

/// Get the file path for a generated .rs file
///
/// E.g., for ID "pricing/apply_discount" with output base "./generated/spec",
/// returns "./generated/spec/pricing/apply_discount.rs"
#[cfg(test)]
pub(crate) fn output_file_path(output_base: impl AsRef<Path>, id: &str) -> PathBuf {
    let parts: Vec<&str> = id.split('/').collect();
    let mut path = output_base.as_ref().to_path_buf();

    if parts.len() > 1 {
        // All but last segment form the directory path
        for segment in &parts[..parts.len() - 1] {
            path = path.join(segment);
        }
    }

    // Last segment is the file name
    let fn_name = parts.last().unwrap_or(&id);
    path.push(format!("{fn_name}.rs"));

    path
}

/// Get the directory path for a module's mod.rs file
///
/// E.g., for module path "pricing" with output base "./generated/spec",
/// returns "./generated/spec/pricing"
#[cfg(test)]
pub(crate) fn mod_rs_dir(output_base: impl AsRef<Path>, module_path: &str) -> PathBuf {
    if module_path.is_empty() {
        output_base.as_ref().to_path_buf()
    } else {
        output_base
            .as_ref()
            .join(module_path.replace('/', std::path::MAIN_SEPARATOR_STR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[test]
    fn test_load_valid_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let yaml = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
body:
  rust: |
    pub fn apply_discount(subtotal: f64, rate: f64) -> f64 {
        subtotal - subtotal * rate
    }
"#;
        temp_file.write_all(yaml.as_bytes()).unwrap();

        let loaded = load_file(temp_file.path()).unwrap();
        assert_eq!(loaded.spec.id, "pricing/apply_discount");
        assert_eq!(loaded.spec.kind, "function");
        assert_eq!(loaded.spec.intent.why, "Apply a percentage discount.");
    }

    #[test]
    fn test_load_file_rejects_unknown_fields_before_deserialization() {
        let mut temp_file = NamedTempFile::with_suffix(".unit.spec").unwrap();
        let yaml = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
body:
  rust: |
    pub fn apply_discount(subtotal: f64, rate: f64) -> f64 {
        subtotal - subtotal * rate
    }
extra_field: should_fail
"#;
        temp_file.write_all(yaml.as_bytes()).unwrap();

        let result = load_file(temp_file.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Schema validation failed"));
        assert!(err.contains("unknown field"));
    }

    #[test]
    fn test_load_file_not_found() {
        let result = load_file("/nonexistent/file.unit.spec");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No such file"));
    }

    #[test]
    fn test_load_invalid_yaml() {
        let mut temp_file = NamedTempFile::with_suffix("spec").unwrap();
        temp_file.write_all(b"invalid: [").unwrap();
        temp_file.flush().unwrap();

        let result = load_file(temp_file.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("parse") || err_msg.contains("YAML") || err_msg.contains("mapping")
        );
    }

    #[test]
    fn test_load_non_utf8() {
        let mut temp_file = NamedTempFile::with_suffix(".unit.spec").unwrap();
        // Write invalid UTF-8 bytes
        temp_file.write_all(&[0x80, 0x81, 0x82, 0x83]).unwrap();
        temp_file.flush().unwrap();

        let result = load_file(temp_file.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("File is not valid UTF-8"));
    }

    #[test]
    fn test_load_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create valid .unit.spec file
        let file1 = temp_dir.path().join("pricing.unit.spec");
        fs::write(
            &file1,
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: pub fn apply() {}
"#,
        )
        .unwrap();

        // Create nested subdirectory with spec
        let subdir = temp_dir.path().join("utils");
        fs::create_dir(&subdir).unwrap();
        let file2 = subdir.join("math.unit.spec");
        fs::write(
            &file2,
            r#"
id: utils/math/round
kind: function
intent:
  why: Round numbers.
body:
  rust: pub fn round() {}
"#,
        )
        .unwrap();

        // Create a non-.unit.spec file (should be skipped)
        let other_file = temp_dir.path().join("readme.txt");
        fs::write(&other_file, "# Readme").unwrap();

        let specs = load_directory(temp_dir.path()).unwrap();
        assert_eq!(specs.len(), 2);
    }

    #[test]
    fn test_load_directory_collect_all() {
        let temp_dir = TempDir::new().unwrap();

        fs::write(
            temp_dir.path().join("good.unit.spec"),
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: "{ }"
"#,
        )
        .unwrap();

        fs::write(
            temp_dir.path().join("bad.unit.spec"),
            r#"
id: pricing/type
kind: function
intent:
  why: Bad keyword id.
body:
  rust: "{ }"
"#,
        )
        .unwrap();

        fs::write(temp_dir.path().join("notes.txt"), "ignore me").unwrap();

        let report = load_directory_collect_all(temp_dir.path());
        assert_eq!(report.specs.len(), 1);
        assert_eq!(report.errors.len(), 1);
        assert!(
            report.errors[0]
                .to_string()
                .contains("Rust reserved keyword")
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_load_directory_report_skips_symlink_cycle_with_warning() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().unwrap();
        let units_dir = temp_dir.path().join("units");
        fs::create_dir_all(units_dir.join("pricing")).unwrap();
        fs::write(
            units_dir.join("pricing/apply.unit.spec"),
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: "{ }"
"#,
        )
        .unwrap();

        unix_fs::symlink(&units_dir, units_dir.join("loop")).unwrap();

        let report = load_directory_report(&units_dir);
        assert_eq!(report.specs.len(), 1);
        assert!(report.errors.is_empty());
        assert_eq!(report.warnings.len(), 1);
        assert!(
            report.warnings[0]
                .to_string()
                .contains("skipped symlink cycle")
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_load_directory_report_bounded_allows_in_root_symlink_target() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().unwrap();
        let library_root = temp_dir.path();
        let units_dir = library_root.join("units");
        let shared_dir = library_root.join("shared");
        fs::create_dir_all(&units_dir).unwrap();
        fs::create_dir_all(shared_dir.join("pricing")).unwrap();
        fs::write(
            shared_dir.join("pricing/apply.unit.spec"),
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();

        unix_fs::symlink(&shared_dir, units_dir.join("linked-shared")).unwrap();

        let report = load_directory_report_bounded(&units_dir, library_root).unwrap();
        assert_eq!(report.specs.len(), 1);
        assert!(report.errors.is_empty(), "{report:?}");
        assert!(report.warnings.is_empty(), "{report:?}");
    }

    #[test]
    #[cfg(unix)]
    fn test_load_directory_report_bounded_rejects_out_of_root_symlink_target() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().unwrap();
        let units_dir = temp_dir.path().join("units");
        fs::create_dir_all(&units_dir).unwrap();

        let outside_dir = TempDir::new().unwrap();
        let rogue_spec = outside_dir.path().join("rogue.unit.spec");
        fs::write(
            &rogue_spec,
            r#"
id: pricing/rogue
kind: function
intent:
  why: Escape the root.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();

        unix_fs::symlink(&rogue_spec, units_dir.join("rogue.unit.spec")).unwrap();

        let report = load_directory_report_bounded(&units_dir, temp_dir.path()).unwrap();
        assert!(report.specs.is_empty(), "{report:?}");
        assert_eq!(report.errors.len(), 1, "{report:?}");
        assert!(matches!(
            report.errors[0],
            SpecError::PlanSymlinkEscape { .. }
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_load_directory_report_bounded_skips_symlink_cycle_with_warning() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().unwrap();
        let library_root = temp_dir.path();
        let units_dir = library_root.join("units");
        fs::create_dir_all(units_dir.join("pricing")).unwrap();
        fs::write(
            units_dir.join("pricing/apply.unit.spec"),
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();

        unix_fs::symlink(&units_dir, units_dir.join("loop")).unwrap();

        let report = load_directory_report_bounded(&units_dir, library_root).unwrap();
        assert_eq!(report.specs.len(), 1);
        assert!(report.errors.is_empty(), "{report:?}");
        assert_eq!(report.warnings.len(), 1, "{report:?}");
        assert!(
            report.warnings[0]
                .to_string()
                .contains("skipped symlink cycle")
        );
    }

    #[test]
    fn test_load_directory_report_bounded_skips_hidden_scratch_subtrees() {
        let temp_dir = TempDir::new().unwrap();
        let library_root = temp_dir.path();
        let units_dir = library_root.join("units");
        fs::create_dir_all(units_dir.join("pricing")).unwrap();
        fs::create_dir_all(units_dir.join(".scratch/pricing")).unwrap();
        fs::create_dir_all(units_dir.join(".tmp-cache/pricing")).unwrap();
        fs::write(
            units_dir.join("pricing/apply.unit.spec"),
            r#"
id: pricing/apply
kind: function
intent:
  why: Apply pricing.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();
        fs::write(
            units_dir.join(".scratch/pricing/duplicate.unit.spec"),
            r#"
id: pricing/duplicate
kind: function
intent:
  why: Hidden scratch copy.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();
        fs::write(
            units_dir.join(".tmp-cache/pricing/ghost.unit.spec"),
            r#"
id: pricing/ghost
kind: function
intent:
  why: Hidden temp copy.
body:
  rust: "{ true }"
"#,
        )
        .unwrap();

        let report = load_directory_report_bounded(&units_dir, library_root).unwrap();
        assert_eq!(report.specs.len(), 1, "{report:?}");
        assert_eq!(report.specs[0].spec.id, "pricing/apply");
        assert!(report.errors.is_empty(), "{report:?}");
        assert!(report.warnings.is_empty(), "{report:?}");
    }

    #[test]
    #[cfg(unix)]
    fn test_load_molecule_test_directory_report_bounded_rejects_out_of_root_symlink_target() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().unwrap();
        let units_dir = temp_dir.path().join("units");
        fs::create_dir_all(&units_dir).unwrap();

        let outside_dir = TempDir::new().unwrap();
        let rogue_test = outside_dir.path().join("rogue.test.spec");
        fs::write(
            &rogue_test,
            r#"
id: pricing/rogue_flow
covers:
  - pricing/apply
body:
  rust: |
    {
        assert!(true);
    }
"#,
        )
        .unwrap();

        unix_fs::symlink(&rogue_test, units_dir.join("rogue.test.spec")).unwrap();

        let report =
            load_molecule_test_directory_report_bounded(&units_dir, temp_dir.path()).unwrap();
        assert!(report.tests.is_empty(), "{report:?}");
        assert_eq!(report.errors.len(), 1, "{report:?}");
        assert!(matches!(
            report.errors[0],
            SpecError::PlanSymlinkEscape { .. }
        ));
    }

    #[test]
    fn test_load_molecule_test_directory_report_bounded_skips_hidden_scratch_subtrees() {
        let temp_dir = TempDir::new().unwrap();
        let library_root = temp_dir.path();
        let units_dir = library_root.join("units");
        fs::create_dir_all(units_dir.join("pricing")).unwrap();
        fs::create_dir_all(units_dir.join(".scratch/pricing")).unwrap();
        fs::create_dir_all(units_dir.join(".tmp-cache/pricing")).unwrap();
        fs::write(
            units_dir.join("pricing/checkout_flow.test.spec"),
            r#"
id: pricing/checkout_flow
intent:
  why: Visible molecule test.
covers:
  - pricing/apply
body:
  rust: |
    {
        assert!(true);
    }
"#,
        )
        .unwrap();
        fs::write(
            units_dir.join(".scratch/pricing/duplicate.test.spec"),
            r#"
id: pricing/duplicate_flow
intent:
  why: Hidden scratch molecule test.
covers:
  - pricing/apply
body:
  rust: |
    {
        assert!(true);
    }
"#,
        )
        .unwrap();
        fs::write(
            units_dir.join(".tmp-cache/pricing/ghost.test.spec"),
            r#"
id: pricing/ghost_flow
intent:
  why: Hidden temp molecule test.
covers:
  - pricing/apply
body:
  rust: |
    {
        assert!(true);
    }
"#,
        )
        .unwrap();

        let report = load_molecule_test_directory_report_bounded(&units_dir, library_root).unwrap();
        assert_eq!(report.tests.len(), 1, "{report:?}");
        assert_eq!(report.tests[0].test.id, "pricing/checkout_flow");
        assert!(report.errors.is_empty(), "{report:?}");
        assert!(report.warnings.is_empty(), "{report:?}");
    }

    #[test]
    fn test_load_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let specs = load_directory(temp_dir.path()).unwrap();
        assert!(specs.is_empty());
    }

    #[test]
    fn test_is_unit_spec_requires_exact_suffix() {
        assert!(is_unit_spec(Path::new("pricing/apply_discount.unit.spec")));
        assert!(!is_unit_spec(Path::new(
            "pricing/apply_discount.unit.spec.bak"
        )));
        assert!(!is_unit_spec(Path::new("pricing/apply_discount.spec")));
    }

    #[test]
    fn test_output_dir_for_spec() {
        let base = Path::new("./generated/spec");

        assert_eq!(
            output_dir_for_spec(base, "pricing"),
            PathBuf::from("./generated/spec/pricing")
        );

        assert_eq!(
            output_dir_for_spec(base, "utils/math"),
            PathBuf::from("./generated/spec/utils/math")
        );

        assert_eq!(
            output_dir_for_spec(base, ""),
            PathBuf::from("./generated/spec")
        );
    }

    #[test]
    fn test_output_file_path() {
        let base = Path::new("./generated/spec");

        assert_eq!(
            output_file_path(base, "pricing/apply_discount"),
            PathBuf::from("./generated/spec/pricing/apply_discount.rs")
        );

        assert_eq!(
            output_file_path(base, "utils/math/round"),
            PathBuf::from("./generated/spec/utils/math/round.rs")
        );
    }

    #[test]
    fn test_mod_rs_dir() {
        let base = Path::new("./generated/spec");

        assert_eq!(
            mod_rs_dir(base, "pricing"),
            PathBuf::from("./generated/spec/pricing")
        );

        assert_eq!(mod_rs_dir(base, ""), PathBuf::from("./generated/spec"));
    }

    #[test]
    fn test_empty_file() {
        let mut temp_file = NamedTempFile::with_suffix(".unit.spec").unwrap();
        temp_file.write_all(b"").unwrap();
        temp_file.flush().unwrap();

        let result = load_file(temp_file.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("missing")
                || err.contains("EOF")
                || err.contains("end of file")
                || err.contains("Unknown entry")
                || err.contains("Schema validation failed")
        );
    }
}
