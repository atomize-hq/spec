//! Loader module: Read and parse .unit.spec files from the filesystem
//!
//! Functions:
//! - Load single .unit.spec file
//! - Load directory recursively
//! - UTF-8 validation before YAML parsing
//! - Error tracking with file paths

use crate::types::{LoadedSpec, SpecSource, SpecStruct};
use crate::validator::validate_raw_yaml;
use crate::{Result, SpecError};
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use walkdir::WalkDir;

#[cfg(test)]
use crate::validator::validate_semantic;

/// Result of a collect-all directory load.
#[derive(Debug, Default)]
pub struct DirectoryLoadReport {
    pub specs: Vec<LoadedSpec>,
    pub errors: Vec<SpecError>,
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

/// Load all .unit.spec files from a directory recursively
///
/// Returns a vector of LoadedSpec, sorted by file path.
/// Non-.unit.spec files are skipped.
/// Empty directories return an empty vec (not an error).
pub fn load_directory<P: AsRef<Path>>(dir: P) -> Result<Vec<LoadedSpec>> {
    let dir = dir.as_ref();
    let mut specs = Vec::new();

    for entry in WalkDir::new(dir).follow_links(true) {
        let entry = entry?;
        let path = entry.path();

        // Only process files with .unit.spec extension
        if !path.is_file() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        // Check if filename ends with .unit.spec
        if name.ends_with(".unit.spec") {
            specs.push(load_file(path)?);
        }
    }

    // Sort by file path for deterministic ordering
    specs.sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));

    Ok(specs)
}

/// Load all .unit.spec files from a directory recursively and collect all errors.
///
/// Unlike `load_directory`, this helper continues after failures so callers can
/// present grouped diagnostics for the full directory.
#[cfg(test)]
pub(crate) fn load_directory_collect_all<P: AsRef<Path>>(dir: P) -> DirectoryLoadReport {
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

                match load_file(path) {
                    Ok(spec) => match validate_semantic(&spec) {
                        Ok(()) => report.specs.push(spec),
                        Err(err) => report.errors.push(err),
                    },
                    Err(err) => report.errors.push(err),
                }
            }
            Err(err) => report.errors.push(err.into()),
        }
    }

    report
        .specs
        .sort_by(|a, b| a.source.file_path.cmp(&b.source.file_path));
    report
}

/// Check if a path is a .unit.spec file
pub fn is_unit_spec(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.ends_with(".unit.spec"))
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
        assert!(err.contains("Additional properties are not allowed"));
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
  rust: pub fn apply() {}
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
  rust: pub fn bad() {}
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
