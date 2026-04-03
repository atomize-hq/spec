//! Validator module: Validate specs against JSON Schema and perform semantic checks
//!
//! Two-stage validation:
//! 1. JSON Schema validation (using embedded unit.spec.json)
//! 2. Semantic validation (Rust keywords, deps, etc.)

use crate::types::LoadedSpec;
use crate::{Result, SpecError, SpecWarning};
use serde_json::Value;
use serde_yaml_bw::Value as YamlValue;
use std::collections::HashSet;
use std::sync::OnceLock;

/// JSON Schema for unit.spec validation (embedded at compile time)
const SCHEMA_JSON: &str = include_str!("schema/unit.spec.json");

static COMPILED_SCHEMA: OnceLock<jsonschema::Validator> = OnceLock::new();

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidationOptions {
    pub strict_deps: bool,
    pub allow_unsafe_local_test_expect: bool,
}

impl ValidationOptions {
    pub fn strict() -> Self {
        Self {
            strict_deps: true,
            allow_unsafe_local_test_expect: false,
        }
    }
}

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
    validate_semantic_with_options(spec, &ValidationOptions::strict())
}

/// Perform semantic validation with explicit options.
pub fn validate_semantic_with_options(
    spec: &LoadedSpec,
    options: &ValidationOptions,
) -> Result<()> {
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

    validate_body_rust_alignment(spec)?;
    validate_local_test_expects(spec, options)?;

    Ok(())
}

fn validate_local_test_expects(spec: &LoadedSpec, options: &ValidationOptions) -> Result<()> {
    let path = spec.source.file_path.clone();
    let mut seen_ids = HashSet::new();
    for test in &spec.spec.local_tests {
        if !seen_ids.insert(test.id.as_str()) {
            return Err(SpecError::DuplicateLocalTestId {
                id: test.id.clone(),
                path: path.clone(),
            });
        }

        let expr = syn::parse_str::<syn::Expr>(test.expect.trim()).map_err(|err| {
            SpecError::LocalTestExpectNotExpr {
                id: test.id.clone(),
                message: err.to_string(),
                path: path.clone(),
            }
        })?;
        // Reject syntax-level injection vectors: block expressions, unsafe blocks,
        // closures, and control-flow forms (If, Match, Loop, etc.) that introduce
        // statements or scope inside assert!(). Note: call expressions to
        // side-effectful functions (std::fs, std::process::Command, etc.) are not
        // blocked — .unit.spec files are treated as trusted input, the same as
        // body.rust. A config lever and defense-in-depth options are deferred to
        // M3. See TODOS.md.
        if !options.allow_unsafe_local_test_expect && !is_safe_expect_expr(&expr) {
            return Err(SpecError::LocalTestExpectNotExpr {
                id: test.id.clone(),
                message: "expect must use only operators, function calls, and value access; block, unsafe, closure, and control-flow forms are not allowed".to_string(),
                path: path.clone(),
            });
        }
    }
    Ok(())
}

fn is_safe_expect_expr(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Binary(b) => is_safe_expect_expr(&b.left) && is_safe_expect_expr(&b.right),
        syn::Expr::Call(c) => {
            is_safe_expect_expr(&c.func) && c.args.iter().all(is_safe_expect_expr)
        }
        syn::Expr::MethodCall(m) => {
            is_safe_expect_expr(&m.receiver) && m.args.iter().all(is_safe_expect_expr)
        }
        syn::Expr::Field(f) => is_safe_expect_expr(&f.base),
        syn::Expr::Index(i) => is_safe_expect_expr(&i.expr) && is_safe_expect_expr(&i.index),
        syn::Expr::Unary(u) => is_safe_expect_expr(&u.expr),
        syn::Expr::Path(_) | syn::Expr::Lit(_) => true,
        syn::Expr::Paren(inner) => is_safe_expect_expr(&inner.expr),
        syn::Expr::Cast(c) => is_safe_expect_expr(&c.expr),
        // Block, Unsafe, Closure, If, Match, Loop, ForLoop, While, Async, etc. → rejected
        _ => false,
    }
}

fn validate_body_rust_alignment(spec: &LoadedSpec) -> Result<()> {
    let path = spec.source.file_path.clone();
    let expected_fn_name = spec.spec.id.rsplit('/').next().unwrap_or_default();

    let file =
        syn::parse_file(&spec.spec.body.rust).map_err(|err| SpecError::BodyRustParseFailed {
            message: err.to_string(),
            path: path.clone(),
        })?;

    if file.items.len() != 1 {
        return Err(SpecError::BodyRustMustBeSingleFn {
            found: file.items.len(),
            path,
        });
    }

    let syn::Item::Fn(item_fn) = &file.items[0] else {
        return Err(SpecError::BodyRustMustBeSingleFn { found: 0, path });
    };

    let found_name = item_fn.sig.ident.to_string();
    if found_name != expected_fn_name {
        return Err(SpecError::BodyRustFnNameMismatch {
            expected: expected_fn_name.to_string(),
            found: found_name,
            path,
        });
    }

    for input in &item_fn.sig.inputs {
        match input {
            syn::FnArg::Receiver(_) => return Err(SpecError::BodyRustMethodRejected { path }),
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat
                    && pat_ident.ident == "self"
                {
                    return Err(SpecError::BodyRustMethodRejected { path });
                }
            }
        }
    }

    if let Some(contract) = &spec.spec.contract
        && let Some(inputs) = &contract.inputs
    {
        let mut params = HashSet::<String>::new();
        for input in &item_fn.sig.inputs {
            let syn::FnArg::Typed(pat_type) = input else {
                continue;
            };
            let syn::Pat::Ident(pat_ident) = &*pat_type.pat else {
                continue;
            };
            params.insert(pat_ident.ident.to_string());
        }

        let mut input_keys: Vec<&String> = inputs.keys().collect();
        input_keys.sort();
        for input in input_keys {
            if !params.contains(input.as_str()) {
                return Err(SpecError::ContractInputParamMismatch {
                    input: input.clone(),
                    path,
                });
            }
        }
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

/// Check for duplicate IDs across all loaded specs.
///
/// Returns all duplicate pairs, not just the first. Each additional file that
/// shares an ID produces a separate error citing the original file as file1.
pub fn validate_no_duplicate_ids(specs: &[LoadedSpec]) -> Vec<SpecError> {
    use std::collections::HashMap;
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut errors = Vec::new();

    for spec in specs {
        if let Some(existing_file) = seen.get(&spec.spec.id) {
            errors.push(SpecError::DuplicateId {
                id: spec.spec.id.clone(),
                file1: existing_file.clone(),
                file2: spec.source.file_path.clone(),
            });
        } else {
            seen.insert(spec.spec.id.clone(), spec.source.file_path.clone());
        }
    }

    errors
}

/// Validate that all internal deps referenced by loaded specs exist in the same spec set.
///
/// For M2, deps are always strict: any missing dep is an error.
pub fn validate_deps_exist(specs: &[LoadedSpec]) -> (Vec<SpecError>, Vec<SpecWarning>) {
    validate_deps_exist_with_options(specs, &ValidationOptions::strict())
}

pub fn validate_deps_exist_with_options(
    specs: &[LoadedSpec],
    options: &ValidationOptions,
) -> (Vec<SpecError>, Vec<SpecWarning>) {
    let mut ids = HashSet::<&str>::new();
    for spec in specs {
        ids.insert(spec.spec.id.as_str());
    }

    let mut errors = Vec::<SpecError>::new();
    let mut warnings = Vec::<SpecWarning>::new();
    for spec in specs {
        for dep in &spec.spec.deps {
            if !ids.contains(dep.as_str()) {
                if options.strict_deps {
                    errors.push(SpecError::MissingDep {
                        dep: dep.clone(),
                        path: spec.source.file_path.clone(),
                    });
                } else {
                    warnings.push(SpecWarning::MissingDep {
                        dep: dep.clone(),
                        path: spec.source.file_path.clone(),
                    });
                }
            }
        }
    }

    (errors, warnings)
}

/// Full validation (schema + semantic)
pub fn validate_full(spec: &LoadedSpec) -> Result<()> {
    validate_full_with_options(spec, &ValidationOptions::strict())
}

/// Full validation (schema + semantic) with explicit options.
pub fn validate_full_with_options(spec: &LoadedSpec, options: &ValidationOptions) -> Result<()> {
    validate_schema(spec)?;
    validate_semantic_with_options(spec, options)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Contract, Intent, SpecSource, SpecStruct};

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
        assert!(
            result.is_ok(),
            "Expected valid imports to pass: {:?}",
            result
        );

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
    fn validate_local_test_id_must_be_valid_identifier() {
        let invalid = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    pub fn apply_discount() {}
local_tests:
  - id: some case!
    expect: "true"
"#;
        let value: YamlValue = serde_yaml_bw::from_str(invalid).unwrap();
        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_err(), "Expected invalid local_tests id to fail");

        let valid = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    pub fn apply_discount() {}
local_tests:
  - id: happy_path
    expect: "true"
"#;
        let value: YamlValue = serde_yaml_bw::from_str(valid).unwrap();
        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(result.is_ok(), "Expected valid local_tests id to pass");
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

        let errors = validate_no_duplicate_ids(&specs);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_duplicate_ids() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "pub fn test1() {}"),
            create_test_spec("pricing/apply_discount", "pub fn test2() {}"),
        ];

        let errors = validate_no_duplicate_ids(&specs);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].to_string().contains("Duplicate ID"));
        assert!(errors[0].to_string().contains("pricing/apply_discount"));
    }

    #[test]
    fn test_validate_duplicate_ids_all_reported() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "pub fn test1() {}"),
            create_test_spec("pricing/apply_discount", "pub fn test2() {}"),
            create_test_spec("pricing/apply_discount", "pub fn test3() {}"),
        ];

        let errors = validate_no_duplicate_ids(&specs);
        assert_eq!(errors.len(), 2, "all duplicate pairs should be reported");
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

    #[test]
    fn validate_body_fn_name_mismatch() {
        let spec = create_test_spec("pricing/apply_discount", "pub fn wrong_name() {}");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("expected 'apply_discount'"), "{err}");
    }

    #[test]
    fn validate_body_multiple_fns_rejected() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            "pub fn apply_discount() {}\npub fn other() {}",
        );
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("exactly one top-level function"), "{err}");
    }

    #[test]
    fn validate_contract_arg_name_mismatch() {
        use std::collections::HashMap;

        let mut spec = create_test_spec(
            "pricing/apply_discount",
            "pub fn apply_discount(price: Decimal) {}",
        );
        let mut inputs = HashMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        spec.spec.contract = Some(Contract {
            inputs: Some(inputs),
            returns: None,
            invariants: vec![],
        });

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("contract.inputs contains 'subtotal'"), "{err}");
    }

    #[test]
    fn validate_body_with_macros_passes_fn_name_check() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            r#"
#[allow(dead_code)]
pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
    let _v = vec![1, 2, 3];
    assert!(true);
    todo!()
}
"#,
        );
        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn validate_body_method_rejected() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            "pub fn apply_discount(&self, subtotal: Decimal) {}",
        );
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("free function (no self parameter)"), "{err}");
    }

    #[test]
    fn validate_local_test_expect_rejects_non_expression() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() {}".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "injection_attempt".to_string(),
                    expect: "true); } } mod evil { fn steal() {}".to_string(),
                }],
                links: None,
            },
        };
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("not a valid Rust expression"),
            "expected injection to be rejected: {err}"
        );
    }

    #[test]
    fn validate_local_test_expect_accepts_valid_expression() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "apply_discount() == true".to_string(),
                }],
                links: None,
            },
        };
        assert!(validate_semantic(&spec).is_ok());
    }

    #[test]
    fn validate_local_test_expect_allows_block_expr_when_configured() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "block_allowed".to_string(),
                    expect: "{ let ok = apply_discount(); ok }".to_string(),
                }],
                links: None,
            },
        };

        let options = ValidationOptions {
            strict_deps: true,
            allow_unsafe_local_test_expect: true,
        };
        assert!(validate_semantic_with_options(&spec, &options).is_ok());
    }

    #[test]
    fn validate_local_test_duplicate_ids_are_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![
                    LocalTest {
                        id: "happy_path".to_string(),
                        expect: "apply_discount()".to_string(),
                    },
                    LocalTest {
                        id: "happy_path".to_string(),
                        expect: "apply_discount()".to_string(),
                    },
                ],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("duplicate local_tests id 'happy_path'"),
            "{err}"
        );
    }

    #[test]
    fn validate_local_test_expect_rejects_block_expression() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "block_attempt".to_string(),
                    expect: "{ std::process::exit(1); true }".to_string(),
                }],
                links: None,
            },
        };
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected block expression to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_call_arg_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_call_arg".to_string(),
                    expect: "f(unsafe { true })".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in call arg to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_block_in_binary_operand_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "block_in_binary_operand".to_string(),
                    expect: "true && { false }".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected block expression in binary operand to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_method_call_arg_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_method_arg".to_string(),
                    expect: "foo.bar(unsafe { true })".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in method call arg to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_field_base_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_field_base".to_string(),
                    expect: "(unsafe { foo }).field".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in field base to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_index_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_index".to_string(),
                    expect: "arr[unsafe { 0 }]".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in index to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_unary_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_unary".to_string(),
                    expect: "!(unsafe { true })".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in unary operand to be rejected: {err}"
        );
    }

    #[test]
    fn expect_with_unsafe_block_in_cast_is_rejected() {
        use crate::types::{Body, Intent, LocalTest, SpecSource, SpecStruct};
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "test.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount.".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "pub fn apply_discount() -> bool { true }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_cast".to_string(),
                    expect: "(unsafe { 0 }) as u64".to_string(),
                }],
                links: None,
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in cast to be rejected: {err}"
        );
    }
}
