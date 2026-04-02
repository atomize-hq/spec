//! Validator module: Validate specs against JSON Schema and perform semantic checks
//!
//! Two-stage validation:
//! 1. JSON Schema validation (using embedded unit.spec.json)
//! 2. Semantic validation (Rust keywords, deps, etc.)

use crate::types::LoadedSpec;
use crate::{Result, SpecError};
use serde_json::Value;
use serde_yaml_bw::Value as YamlValue;
use std::sync::OnceLock;

/// JSON Schema for unit.spec validation (embedded at compile time)
const SCHEMA_JSON: &str = include_str!("schema/unit.spec.json");

static COMPILED_SCHEMA: OnceLock<jsonschema::Validator> = OnceLock::new();

/// Validate a single spec against the JSON Schema
pub fn validate_schema(spec: &LoadedSpec) -> Result<()> {
    validate_json_value(
        &serde_json::to_value(&spec.spec).map_err(SpecError::Json)?,
        &spec.source.file_path,
    )
}

/// Validate a raw YAML-authored value against the JSON Schema before deserialization.
///
/// This is the validation path used by the loader so that unknown fields and other
/// authoring-time mistakes are rejected before serde can apply defaults or drop data.
pub fn validate_raw_yaml(yaml_value: &YamlValue, file_path: &str) -> Result<()> {
    let spec_json = serde_json::to_value(yaml_value).map_err(SpecError::Json)?;
    validate_json_value(&spec_json, file_path)
}

fn compiled_schema() -> Result<&'static jsonschema::Validator> {
    if let Some(schema) = COMPILED_SCHEMA.get() {
        return Ok(schema);
    }

    let schema_json: Value = serde_json::from_str(SCHEMA_JSON).map_err(SpecError::Json)?;
    let schema =
        jsonschema::draft7::new(&schema_json).map_err(|e| SpecError::SchemaValidation {
            message: format!("Schema compilation failed: {e}"),
            path: "<schema>".to_string(),
        })?;

    let _ = COMPILED_SCHEMA.set(schema);

    Ok(COMPILED_SCHEMA
        .get()
        .expect("COMPILED_SCHEMA must be set after successful compilation"))
}

fn validate_json_value(spec_json: &Value, file_path: &str) -> Result<()> {
    let schema = compiled_schema()?;

    // Validate against schema
    let validation_result = schema.validate(spec_json);

    match validation_result {
        Ok(()) => Ok(()),
        Err(error) => Err(SpecError::SchemaValidation {
            message: error.to_string(),
            path: file_path.to_string(),
        }),
    }
}

/// Perform semantic validation (Rust keywords, deps, etc.)
pub fn validate_semantic(spec: &LoadedSpec) -> Result<()> {
    // Check if ID contains Rust reserved keywords
    validate_rust_keywords(&spec.spec.id, &spec.source.file_path)?;

    // Check dep IDs for Rust reserved keywords (would generate invalid use paths)
    for dep in &spec.spec.deps {
        validate_rust_keywords(dep, &spec.source.file_path)?;
    }

    // Check for dep fn_name collisions
    if let Some((dep1, dep2)) = crate::types::ResolvedSpec::has_dep_collision(&spec.spec.deps) {
        return Err(SpecError::DepCollision {
            dep1: dep1.clone(),
            dep2: dep2.clone(),
            fn_name: crate::types::ResolvedSpec::dep_fn_name(dep1).to_string(),
            path: spec.source.file_path.clone(),
        });
    }

    // Check for use statements in body.rust (line-start to avoid false positives in comments).
    // Also catches visibility-prefixed forms: `pub use`, `pub(crate) use`, etc.
    let has_use_stmt = spec.spec.body.rust.lines().any(|line| {
        let trimmed = line.trim_start();
        // Plain `use` or `use<TAB>`
        if trimmed.starts_with("use ") || trimmed.starts_with("use\t") {
            return true;
        }
        // `pub use ...` / `pub(crate) use ...` / `pub(super) use ...`
        if let Some(rest) = trimmed.strip_prefix("pub") {
            let rest = rest
                .trim_start_matches(|c: char| {
                    c == '(' || c == ')' || c.is_alphanumeric() || c == '_'
                })
                .trim_start();
            if rest.starts_with("use ") || rest.starts_with("use\t") {
                return true;
            }
        }
        false
    });
    if has_use_stmt {
        return Err(SpecError::UseStatementInBody {
            path: spec.source.file_path.clone(),
        });
    }

    Ok(())
}

/// Check if any segment of an ID is a Rust reserved keyword
pub fn validate_rust_keywords(id: &str, file_path: &str) -> Result<()> {
    for segment in id.split('/') {
        if crate::types::is_rust_keyword(segment) {
            return Err(SpecError::RustKeyword {
                segment: segment.to_string(),
                id: id.to_string(),
                path: file_path.to_string(),
            });
        }
    }
    Ok(())
}

/// Check for duplicate IDs across all loaded specs
pub fn validate_no_duplicate_ids(specs: &[LoadedSpec]) -> Result<()> {
    use std::collections::HashMap;
    let mut seen: HashMap<String, String> = HashMap::new();

    for spec in specs {
        if let Some(existing_file) = seen.get(&spec.spec.id) {
            return Err(SpecError::DuplicateId {
                id: spec.spec.id.clone(),
                file1: existing_file.clone(),
                file2: spec.source.file_path.clone(),
            });
        }
        seen.insert(spec.spec.id.clone(), spec.source.file_path.clone());
    }

    Ok(())
}

/// Full validation (schema + semantic)
pub fn validate_full(spec: &LoadedSpec) -> Result<()> {
    validate_schema(spec)?;
    validate_semantic(spec)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Intent, SpecSource, SpecStruct};

    fn create_test_spec(id: &str, rust_body: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("test/{}.unit.spec", id),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Test spec for {}", id),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: rust_body.to_string(),
                },
                local_tests: vec![],
                links: None,
            },
        }
    }

    #[test]
    fn test_validate_schema_valid() {
        let spec = create_test_spec("pricing/apply_discount", "pub fn test() {}");
        let result = validate_schema(&spec);
        assert!(
            result.is_ok(),
            "Schema validation should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_schema_valid_spec_passes() {
        let spec = create_test_spec("pricing/apply_discount", "pub fn test() {}");
        let result = validate_schema(&spec);
        assert!(
            result.is_ok(),
            "A complete valid spec should pass schema validation: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_raw_yaml_rejects_unknown_fields() {
        let yaml = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
body:
  rust: |
    pub fn apply_discount() {}
extra_field: should_fail
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Schema validation failed"));
        assert!(err.contains("Additional properties are not allowed"));
    }

    #[test]
    fn imports_field_validates_rust_path() {
        let valid = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
imports:
  - rust_decimal::Decimal
  - std::collections::HashMap
body:
  rust: |
    pub fn apply_discount() {}
"#;
        let value: YamlValue = serde_yaml_bw::from_str(valid).unwrap();
        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_ok(), "Expected valid imports to pass: {:?}", result);

        let invalid_bare = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
imports:
  - Decimal
body:
  rust: |
    pub fn apply_discount() {}
"#;
        let value: YamlValue = serde_yaml_bw::from_str(invalid_bare).unwrap();
        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_err(), "Expected bare import to fail");

        let invalid_leading = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount.
imports:
  - ::Decimal
body:
  rust: |
    pub fn apply_discount() {}
"#;
        let value: YamlValue = serde_yaml_bw::from_str(invalid_leading).unwrap();
        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_err(), "Expected leading :: import to fail");
    }

    #[test]
    fn test_validate_rust_keywords_in_id() {
        let result = validate_rust_keywords("pricing/type", "test.unit.spec");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Rust reserved keyword"));
        assert!(err.to_string().contains("type"));
    }

    #[test]
    fn test_validate_valid_id() {
        let result = validate_rust_keywords("pricing/apply_discount", "test.unit.spec");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_duplicate_ids() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "pub fn test1() {}"),
            create_test_spec("utils/round", "pub fn test2() {}"),
        ];

        let result = validate_no_duplicate_ids(&specs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_duplicate_ids() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "pub fn test1() {}"),
            create_test_spec("pricing/apply_discount", "pub fn test2() {}"),
        ];

        let result = validate_no_duplicate_ids(&specs);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Duplicate ID"));
        assert!(err.to_string().contains("pricing/apply_discount"));
    }

    #[test]
    fn test_validate_dep_collision() {
        let mut spec = create_test_spec("pricing/calculate_total", "pub fn test() { round(1.5) }");
        spec.spec.deps = vec!["money/round".to_string(), "utils/round".to_string()];

        let result = validate_semantic(&spec);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("collision"));
        assert!(err.to_string().contains("round"));
    }

    #[test]
    fn test_validate_use_statement_in_body() {
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "test/test".to_string(),
            },
            spec: SpecStruct {
                id: "test/test".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Test spec".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "use std::collections::HashMap; pub fn test() {}".to_string(),
                },
                local_tests: vec![],
                links: None,
            },
        };

        let result = validate_semantic(&spec);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("body.rust must not contain use statements")
        );
    }

    #[test]
    fn test_validate_semantic_valid_spec() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            "pub fn apply_discount(subtotal: f64, rate: f64) -> f64 { subtotal - subtotal * rate }",
        );
        let result = validate_semantic(&spec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_full() {
        let spec = create_test_spec("utils/round", "pub fn round(x: f64) -> f64 { x.floor() }");
        let result = validate_full(&spec);
        assert!(result.is_ok(), "Full validation should pass: {:?}", result);
    }

    #[test]
    fn test_validate_use_statement_pub_use_in_body() {
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "test/func".to_string(),
            },
            spec: SpecStruct {
                id: "test/func".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Test pub use".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub use std::collections::HashMap;\npub fn func() {}".to_string(),
                },
                local_tests: vec![],
                links: None,
            },
        };
        let result = validate_semantic(&spec);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("body.rust must not contain use statements")
        );
    }

    #[test]
    fn test_validate_rust_keyword_try_in_id() {
        // `try` is reserved since Rust 2018 and was previously missing from the list
        let result = validate_rust_keywords("pricing/try", "test.unit.spec");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Rust reserved keyword")
        );
    }
}
