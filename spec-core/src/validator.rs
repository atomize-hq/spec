//! Validator module: Validate specs against JSON Schema and perform semantic checks
//!
//! Two-stage validation:
//! 1. JSON Schema validation (using embedded unit.spec.json)
//! 2. Semantic validation (Rust keywords, deps, etc.)

use crate::graph::top_level_deps;
use crate::portability_contract::{SharedSeamAuthoredShapeRule, shared_surface_violation_message};
use crate::semantic_review::{
    SemanticReviewContext, SemanticSupportStatus, evaluate_semantic_review_with_context,
};
use crate::syntax::{token_stream_contains_unsafe_keyword, validate_expect_expr};
use crate::types::{
    AuthoredField, Contract, DepRef, DepRefParseError, LoadedMoleculeTest, LoadedSpec,
    MethodReceiver, QualifiedUnitRef, UnitKind, callable_name, has_callable_collision,
    ordered_unique_deps, type_name_for_identifier, type_name_for_unit_id,
};
use crate::{AUTHORED_SPEC_VERSION, Result, SpecError, SpecWarning};
use indexmap::IndexMap;
use serde_json::Value;
use serde_yaml_bw::Value as YamlValue;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

/// JSON Schema for unit.spec validation (embedded at compile time)
const SCHEMA_JSON: &str = include_str!("schema/unit.spec.json");

/// JSON Schema for test.spec validation (embedded at compile time)
const TEST_SCHEMA_JSON: &str = include_str!("schema/test.spec.json");

static COMPILED_SCHEMA: OnceLock<jsonschema::Validator> = OnceLock::new();
static COMPILED_TEST_SCHEMA: OnceLock<jsonschema::Validator> = OnceLock::new();

pub const TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_up.v1";
pub const TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_down_nonnegative.v1";
pub const TYPESCRIPT_TARGET_COMPATIBILITY_KEY: &str =
    TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY;
pub const TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.v1";
pub const TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY: &str =
    "function.wrapper.pipeline.normalized_required_arg.v1";
pub const TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.chain3.v1";
pub const TYPESCRIPT_WRAPPER_FIRST_DEP_COMPATIBILITY_KEY: &str =
    TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY;
pub const TYPESCRIPT_WRAPPER_SECOND_DEP_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_up.v1";
pub const TYPESCRIPT_CHAIN3_FIRST_DEP_COMPATIBILITY_KEY: &str =
    TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY;
pub const TYPESCRIPT_CHAIN3_SECOND_DEP_COMPATIBILITY_KEY: &str =
    TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY;
pub const TYPESCRIPT_CHAIN3_THIRD_DEP_COMPATIBILITY_KEY: &str =
    TYPESCRIPT_WRAPPER_FIRST_DEP_COMPATIBILITY_KEY;
pub const TYPESCRIPT_HELPER_COMPATIBILITY_KEY: &str = "function.helper.identity_passthrough.v1";
pub const TYPESCRIPT_MOLECULE_UNSUPPORTED_MESSAGE: &str = ".test.spec is not supported for --target-language typescript in M52; molecule tests remain Rust-only";
pub const TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE: &str =
    "TypeScript target currently supports only kind:function units in M52";
pub const TYPESCRIPT_DEP_ARITY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript monotone-up target requires deps: [] or exactly one direct helper dep in M55";
pub const TYPESCRIPT_MISSING_HELPER_UNSUPPORTED_MESSAGE: &str =
    "TypeScript target requires the direct helper dep to resolve from the loaded unit set in M55";
pub const TYPESCRIPT_HELPER_FAMILY_UNSUPPORTED_MESSAGE: &str = "TypeScript target requires the direct helper dep to classify as function.helper.identity_passthrough.v1 in M55";
pub const TYPESCRIPT_HELPER_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript target requires the direct helper dep to author body.typescript in M55";
pub const TYPESCRIPT_WRAPPER_DEP_ARITY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript wrapper target requires exactly two direct deps in M56";
pub const TYPESCRIPT_WRAPPER_MISSING_DEP_UNSUPPORTED_MESSAGE: &str = "TypeScript wrapper target requires every direct dep to resolve from the loaded unit set in M56";
pub const TYPESCRIPT_WRAPPER_DEP_FAMILY_UNSUPPORTED_MESSAGE: &str = "TypeScript wrapper target requires direct deps to classify as function.arithmetic_leaf.monotone_down_nonnegative.v1 then function.arithmetic_leaf.monotone_up.v1 in M56";
pub const TYPESCRIPT_WRAPPER_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript wrapper target requires direct deps to author body.typescript in M56";
pub const TYPESCRIPT_CHAIN3_DEP_ARITY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript chain3 target requires exactly three direct deps in M56";
pub const TYPESCRIPT_CHAIN3_MISSING_DEP_UNSUPPORTED_MESSAGE: &str =
    "TypeScript chain3 target requires every direct dep to resolve from the loaded unit set in M56";
pub const TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE: &str = "TypeScript chain3 target requires direct deps to classify as function.wrapper.pipeline.v1, function.wrapper.pipeline.normalized_required_arg.v1, or function.wrapper.pipeline.chain3.v1 then function.arithmetic_leaf.monotone_up.v1 then function.arithmetic_leaf.monotone_down_nonnegative.v1 in M61";
pub const TYPESCRIPT_CHAIN3_SAME_TREE_UNSUPPORTED_MESSAGE: &str =
    "TypeScript chain3 target requires recursive slot-1 chain3 deps to stay same-tree local in M58";
pub const TYPESCRIPT_CHAIN3_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE: &str =
    "TypeScript chain3 target requires direct deps to author body.typescript in M56";
pub const TYPESCRIPT_LOCAL_GRAPH_SHARED_DEP_UNSUPPORTED_MESSAGE: &str =
    "TypeScript same-tree local target requires every reachable dep to stay same-tree local in M59";
pub const TYPESCRIPT_LOCAL_GRAPH_MISSING_DEP_UNSUPPORTED_MESSAGE: &str = "TypeScript same-tree local target requires every reachable dep to resolve from the loaded unit set in M59";
pub const TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE: &str = "TypeScript same-tree local target requires every reachable unit to classify to a supported semantic review in M59";
pub const TYPESCRIPT_LOCAL_GRAPH_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE: &str = "TypeScript same-tree local target requires every reachable unit to author body.typescript in M59";
pub const TYPESCRIPT_EXPECT_UNSUPPORTED_MESSAGE: &str = "TypeScript target requires local_tests.expect to match `<current_unit>(Decimal::new(int, scale), ...) == Decimal::new(int, scale)` in M52";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypescriptTargetRootFamily {
    MonotoneDown,
    MonotoneUp,
    Helper,
    WrapperPipeline,
    NormalizedRequiredArgWrapperPipeline,
    Chain3WrapperPipeline,
}

#[derive(Debug, Clone)]
struct TypescriptQualifiedSpecIndex {
    specs_by_qualified_id: HashMap<QualifiedUnitRef, LoadedSpec>,
    qualified_ids_by_source: HashMap<(String, String), Vec<QualifiedUnitRef>>,
    authored_keys_by_source: HashMap<(String, String), Vec<String>>,
}

impl TypescriptQualifiedSpecIndex {
    fn build(specs_by_id: &HashMap<String, LoadedSpec>) -> Result<Self> {
        let mut authored_keys_by_source: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut qualified_ids_by_source: HashMap<(String, String), Vec<QualifiedUnitRef>> =
            HashMap::new();
        let mut specs_by_qualified_id = HashMap::new();

        for (key, spec) in specs_by_id {
            let parsed = DepRef::parse(key).map_err(|err| semantic_error(spec, err.to_string()))?;
            if parsed.unit_id() != spec.spec.id {
                return Err(semantic_error(
                    spec,
                    format!(
                        "TypeScript target lookup key '{}' does not match unit id '{}'",
                        key, spec.spec.id
                    ),
                ));
            }

            let source_key = (spec.source.file_path.clone(), spec.spec.id.clone());
            authored_keys_by_source
                .entry(source_key.clone())
                .or_default()
                .push(key.clone());
            let qualified_id = parsed.to_qualified(None);
            qualified_ids_by_source
                .entry(source_key)
                .or_default()
                .push(qualified_id.clone());

            if let Some(existing) = specs_by_qualified_id.insert(qualified_id.clone(), spec.clone())
                && existing.source.file_path != spec.source.file_path
            {
                return Err(semantic_error(
                    spec,
                    format!(
                        "TypeScript target lookup for '{}' collided across '{}' and '{}'",
                        qualified_id, existing.source.file_path, spec.source.file_path
                    ),
                ));
            }
        }

        Ok(Self {
            specs_by_qualified_id,
            qualified_ids_by_source,
            authored_keys_by_source,
        })
    }

    fn qualified_id_for_spec(&self, spec: &LoadedSpec) -> Result<QualifiedUnitRef> {
        let source_key = (spec.source.file_path.clone(), spec.spec.id.clone());
        let Some(candidates) = self.qualified_ids_by_source.get(&source_key) else {
            return Err(semantic_error(
                spec,
                format!(
                    "TypeScript target '{}' was not present in the loaded unit set",
                    spec.spec.id
                ),
            ));
        };

        if candidates.len() == 1 {
            return Ok(candidates[0].clone());
        }

        if let Some(local) = candidates
            .iter()
            .find(|candidate| candidate.library().is_none())
        {
            return Ok(local.clone());
        }

        if let Some(authored_key) = self
            .authored_keys_by_source
            .get(&source_key)
            .and_then(|keys| keys.first())
        {
            let parsed =
                DepRef::parse(authored_key).map_err(|err| semantic_error(spec, err.to_string()))?;
            return Ok(parsed.to_qualified(None));
        }

        Err(semantic_error(
            spec,
            format!(
                "TypeScript target '{}' had ambiguous qualified identities",
                spec.spec.id
            ),
        ))
    }

    fn spec_for_qualified_id(&self, qualified_id: &QualifiedUnitRef) -> Result<&LoadedSpec> {
        self.specs_by_qualified_id
            .get(qualified_id)
            .ok_or_else(|| SpecError::SemanticValidation {
                message: format!(
                    "TypeScript target '{}' was not present in the loaded unit set",
                    qualified_id
                ),
                path: "<typescript-qualified-lookup>".to_string(),
            })
    }

    fn resolve_dep(
        &self,
        spec: &LoadedSpec,
        owner_library: Option<&str>,
        dep: &str,
        missing_message: &str,
    ) -> Result<(QualifiedUnitRef, &LoadedSpec)> {
        let parsed = DepRef::parse(dep).map_err(|err| semantic_error(spec, err.to_string()))?;
        let qualified_dep = parsed.to_qualified(owner_library);
        let Some(dep_spec) = self.specs_by_qualified_id.get(&qualified_dep) else {
            return Err(semantic_error(
                spec,
                format!("{missing_message}: '{}'", parsed.authored()),
            ));
        };

        Ok((qualified_dep, dep_spec))
    }

    fn semantic_context_for(&self, owner_library: Option<&str>) -> HashMap<String, LoadedSpec> {
        let mut context = HashMap::new();
        for (qualified_id, spec) in &self.specs_by_qualified_id {
            if qualified_id.library() == owner_library {
                context.insert(spec.spec.id.clone(), spec.clone());
            }
            if let Some(library) = qualified_id.library() {
                context.insert(format!("{library}::{}", spec.spec.id), spec.clone());
            }
        }
        context
    }
}

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

/// A library-aware validation snapshot used by later M9 slices.
#[derive(Debug, Clone)]
pub struct QualifiedLoadedSpec<'a> {
    pub loaded: &'a LoadedSpec,
    pub qualified_id: QualifiedUnitRef,
    pub qualified_deps: Vec<QualifiedUnitRef>,
}

impl<'a> QualifiedLoadedSpec<'a> {
    pub fn local_identity(loaded: &'a LoadedSpec) -> Self {
        Self {
            loaded,
            qualified_id: QualifiedUnitRef::local(loaded.spec.id.clone()),
            qualified_deps: Vec::new(),
        }
    }

    pub fn local(loaded: &'a LoadedSpec) -> std::result::Result<Self, DepRefParseError> {
        let dep_refs = parse_dep_refs(&top_level_deps(loaded))?;
        Ok(Self {
            loaded,
            qualified_id: QualifiedUnitRef::local(loaded.spec.id.clone()),
            qualified_deps: dep_refs
                .into_iter()
                .map(|dep| dep.to_qualified(None))
                .collect(),
        })
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

fn humanize_validation_error(error: &jsonschema::ValidationError<'_>) -> String {
    use jsonschema::error::ValidationErrorKind;

    let field_path = error.instance_path.to_string();
    let field_label = if field_path.is_empty() || field_path == "/" {
        String::new()
    } else {
        format!(" at {field_path}")
    };

    match &error.kind {
        ValidationErrorKind::Required { property } => {
            format!("missing required field: {}{}", property, field_label)
        }
        ValidationErrorKind::AdditionalProperties { unexpected } => {
            let fields = unexpected.to_vec().join(", ");
            format!("unknown field{}: {}", field_label, fields)
        }
        ValidationErrorKind::Enum { .. } => {
            format!(
                "invalid value{}: {} — check allowed values",
                field_label, error
            )
        }
        ValidationErrorKind::Pattern { .. } => {
            if field_path == "/id" {
                format!(
                    "invalid id format{}: use \"module/name\" (e.g., \"pricing/apply_tax\")",
                    field_label
                )
            } else if field_path.ends_with("/id") {
                format!(
                    "invalid id format{}: use a snake_case identifier (e.g., \"total\")",
                    field_label
                )
            } else {
                format!("invalid format{}: {}", field_label, error)
            }
        }
        _ => {
            if field_path.is_empty() {
                error.to_string()
            } else {
                format!("{} (at {})", error, field_path)
            }
        }
    }
}

fn validate_json_value(spec_json: &Value, file_path: &str) -> Result<()> {
    let schema = compiled_schema()?;

    // Validate against schema
    let validation_result = schema.validate(spec_json);

    match validation_result {
        Ok(()) => Ok(()),
        Err(error) => Err(SpecError::SchemaValidation {
            message: humanize_validation_error(&error),
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

    // Reject IDs containing reserved namespace segments. "molecule_tests" is emitted as a
    // generated module/file name by build_namespaces + molecule test generation, so allowing it
    // anywhere in a unit ID can create molecule_tests.rs vs molecule_tests/mod.rs collisions.
    validate_reserved_spec_id_segments(&spec.spec.id, &spec.source.file_path)?;

    match spec
        .spec
        .unit_kind()
        .map_err(|message| semantic_error(spec, message))?
    {
        UnitKind::Function => validate_function_semantic(spec, options),
        UnitKind::Data => validate_data_semantic(spec, options),
        UnitKind::Sum => validate_sum_semantic(spec, options),
    }
}

fn validate_function_semantic(spec: &LoadedSpec, options: &ValidationOptions) -> Result<()> {
    let dep_refs = parse_dep_refs(&spec.spec.deps).map_err(|err| invalid_dep_error(err, spec))?;

    // Check dep IDs for Rust reserved keywords (would generate invalid use paths)
    for dep in &dep_refs {
        validate_dep_ref_keywords(dep, &spec.source.file_path)?;
    }

    let owner_callable_name = callable_name(&spec.spec.id);
    if let Some(authored_dep) = spec
        .spec
        .deps
        .iter()
        .zip(dep_refs.iter())
        .find(|(_, dep)| dep.callable_name() == owner_callable_name)
        .map(|(authored_dep, _)| authored_dep)
    {
        return Err(SpecError::DepCollision {
            dep1: authored_dep.clone(),
            dep2: spec.spec.id.clone(),
            fn_name: owner_callable_name.to_string(),
            path: spec.source.file_path.clone(),
        });
    }

    // Check for dep fn_name collisions
    if let Some((dep1, dep2)) = has_dep_ref_collision(&dep_refs) {
        return Err(SpecError::DepCollision {
            dep1: dep1.to_string(),
            dep2: dep2.to_string(),
            fn_name: dep1.callable_name().to_string(),
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

    validate_body_rust_block(spec)?;
    validate_local_test_expects(spec, options)?;
    if let Some(contract) = &spec.spec.contract {
        validate_contract_types(contract, "contract", &spec.source.file_path)?;
    }

    Ok(())
}

pub fn validate_typescript_execution_target_spec(spec: &LoadedSpec) -> Result<()> {
    let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
    validate_typescript_execution_target_spec_with_specs(spec, &specs_by_id)
}

pub fn validate_typescript_execution_target_spec_with_specs(
    spec: &LoadedSpec,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<()> {
    if spec
        .spec
        .unit_kind()
        .map_err(|message| semantic_error(spec, message))?
        != UnitKind::Function
    {
        return Err(semantic_error(spec, TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE));
    }

    let qualified_specs = TypescriptQualifiedSpecIndex::build(specs_by_id)?;
    let root_qualified_id = qualified_specs.qualified_id_for_spec(spec)?;
    let mut visited = HashSet::new();
    validate_typescript_recursive_closure_member_with_specs(
        &root_qualified_id,
        &qualified_specs,
        &mut visited,
        true,
    )?;
    validate_typescript_local_test_expect_shape(spec)
}

pub fn typescript_target_uses_local_graph_lane(spec: &LoadedSpec) -> Result<bool> {
    for dep in &spec.spec.deps {
        let parsed = DepRef::parse(dep).map_err(|err| semantic_error(spec, err.to_string()))?;
        if parsed.library_alias().is_some() {
            return Ok(false);
        }
    }

    Ok(true)
}

fn validate_typescript_recursive_closure_member_with_specs(
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
    visited: &mut HashSet<QualifiedUnitRef>,
    is_root: bool,
) -> Result<()> {
    let spec = qualified_specs.spec_for_qualified_id(qualified_id)?;
    if spec
        .spec
        .unit_kind()
        .map_err(|message| semantic_error(spec, message))?
        != UnitKind::Function
    {
        return Err(semantic_error(spec, TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE));
    }

    if !visited.insert(qualified_id.clone()) {
        return Ok(());
    }

    if spec
        .spec
        .body
        .typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}'",
                TYPESCRIPT_LOCAL_GRAPH_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE, qualified_id
            ),
        ));
    }

    let semantic_review_specs = qualified_specs.semantic_context_for(qualified_id.library());
    let semantic_review_context = SemanticReviewContext::new(&semantic_review_specs);
    let Some(review) = evaluate_semantic_review_with_context(spec, &semantic_review_context) else {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' is missing supported semantic review",
                TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE, qualified_id
            ),
        ));
    };

    if review.effective_support_status() != SemanticSupportStatus::Supported {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' resolved to {}",
                TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE,
                qualified_id,
                review.compatibility_key
            ),
        ));
    }

    match classify_typescript_target_root_family(spec, &semantic_review_specs)? {
        TypescriptTargetRootFamily::Helper if is_root => {
            return Err(semantic_error(
                spec,
                format!(
                    "TypeScript target requires compatibility key {}, {}, {}, {}, or {} in M61; found {}",
                    TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY,
                    TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY,
                    TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY,
                    TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY,
                    TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY,
                    review.compatibility_key
                ),
            ));
        }
        TypescriptTargetRootFamily::MonotoneDown | TypescriptTargetRootFamily::MonotoneUp => {
            validate_typescript_helper_dep_contract(spec, qualified_id, qualified_specs)?;
        }
        TypescriptTargetRootFamily::Helper => {
            if !spec.spec.deps.is_empty() {
                return Err(semantic_error(
                    spec,
                    "TypeScript helper closure members must have deps: [] in M52",
                ));
            }
        }
        TypescriptTargetRootFamily::WrapperPipeline
        | TypescriptTargetRootFamily::NormalizedRequiredArgWrapperPipeline => {
            validate_typescript_wrapper_dep_contract(spec, qualified_id, qualified_specs)?;
        }
        TypescriptTargetRootFamily::Chain3WrapperPipeline => {
            validate_typescript_chain3_dep_contract(spec, qualified_id, qualified_specs)?;
        }
    }

    for dep in &spec.spec.deps {
        let (dep_qualified_id, _) = qualified_specs.resolve_dep(
            spec,
            qualified_id.library(),
            dep,
            TYPESCRIPT_LOCAL_GRAPH_MISSING_DEP_UNSUPPORTED_MESSAGE,
        )?;
        validate_typescript_recursive_closure_member_with_specs(
            &dep_qualified_id,
            qualified_specs,
            visited,
            false,
        )?;
    }

    Ok(())
}

pub fn validate_typescript_closure_member_spec_with_specs(
    spec: &LoadedSpec,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<()> {
    let qualified_specs = TypescriptQualifiedSpecIndex::build(specs_by_id)?;
    let qualified_id = qualified_specs.qualified_id_for_spec(spec)?;
    let mut visited = HashSet::new();
    validate_typescript_recursive_closure_member_with_specs(
        &qualified_id,
        &qualified_specs,
        &mut visited,
        false,
    )
}

fn classify_typescript_target_root_family(
    spec: &LoadedSpec,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<TypescriptTargetRootFamily> {
    let semantic_review_context = SemanticReviewContext::new(specs_by_id);
    let Some(review) = evaluate_semantic_review_with_context(spec, &semantic_review_context) else {
        return Err(semantic_error(
            spec,
            format!(
                "TypeScript target requires compatibility key {}, {}, {}, {}, {}, or {} in M61",
                TYPESCRIPT_HELPER_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY
            ),
        ));
    };

    if review.effective_support_status() != SemanticSupportStatus::Supported {
        return Err(semantic_error(
            spec,
            format!(
                "TypeScript target requires compatibility key {}, {}, {}, {}, {}, or {} in M61; found {}",
                TYPESCRIPT_HELPER_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY,
                review.compatibility_key
            ),
        ));
    }

    match review.compatibility_key.as_str() {
        TYPESCRIPT_HELPER_COMPATIBILITY_KEY => Ok(TypescriptTargetRootFamily::Helper),
        TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY => {
            Ok(TypescriptTargetRootFamily::MonotoneDown)
        }
        TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY => {
            Ok(TypescriptTargetRootFamily::MonotoneUp)
        }
        TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY => {
            Ok(TypescriptTargetRootFamily::WrapperPipeline)
        }
        TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY => {
            Ok(TypescriptTargetRootFamily::NormalizedRequiredArgWrapperPipeline)
        }
        TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY => {
            Ok(TypescriptTargetRootFamily::Chain3WrapperPipeline)
        }
        _ => Err(semantic_error(
            spec,
            format!(
                "TypeScript target requires compatibility key {}, {}, {}, {}, {}, or {} in M61; found {}",
                TYPESCRIPT_HELPER_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_DOWN_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY,
                TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY,
                review.compatibility_key
            ),
        )),
    }
}

fn validate_typescript_helper_dep_contract(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
) -> Result<()> {
    match spec.spec.deps.as_slice() {
        [] => Ok(()),
        [dep] => {
            let (helper_qualified_id, helper_spec) = qualified_specs.resolve_dep(
                spec,
                qualified_id.library(),
                dep,
                TYPESCRIPT_MISSING_HELPER_UNSUPPORTED_MESSAGE,
            )?;
            let semantic_review_specs =
                qualified_specs.semantic_context_for(helper_qualified_id.library());
            let semantic_review_context = SemanticReviewContext::new(&semantic_review_specs);
            let Some(helper_review) =
                evaluate_semantic_review_with_context(helper_spec, &semantic_review_context)
            else {
                return Err(semantic_error(
                    spec,
                    format!(
                        "{}: '{}'",
                        TYPESCRIPT_HELPER_FAMILY_UNSUPPORTED_MESSAGE, helper_qualified_id
                    ),
                ));
            };

            if helper_review.effective_support_status() != SemanticSupportStatus::Supported
                || helper_review.compatibility_key != TYPESCRIPT_HELPER_COMPATIBILITY_KEY
            {
                return Err(semantic_error(
                    spec,
                    format!(
                        "{}: '{}' resolved to {}",
                        TYPESCRIPT_HELPER_FAMILY_UNSUPPORTED_MESSAGE,
                        helper_qualified_id,
                        helper_review.compatibility_key
                    ),
                ));
            }

            if helper_spec
                .spec
                .body
                .typescript
                .as_deref()
                .map(str::trim)
                .filter(|body| !body.is_empty())
                .is_none()
            {
                return Err(semantic_error(
                    spec,
                    format!(
                        "{}: '{}'",
                        TYPESCRIPT_HELPER_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE, helper_qualified_id
                    ),
                ));
            }

            Ok(())
        }
        deps => Err(semantic_error(
            spec,
            format!(
                "{}; found {} direct deps",
                TYPESCRIPT_DEP_ARITY_UNSUPPORTED_MESSAGE,
                deps.len()
            ),
        )),
    }
}

fn validate_typescript_wrapper_dep_contract(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
) -> Result<()> {
    let [first_dep, second_dep] = spec.spec.deps.as_slice() else {
        return Err(semantic_error(
            spec,
            format!(
                "{}; found {} direct deps",
                TYPESCRIPT_WRAPPER_DEP_ARITY_UNSUPPORTED_MESSAGE,
                spec.spec.deps.len()
            ),
        ));
    };

    validate_typescript_wrapper_dep_family(
        spec,
        qualified_id,
        qualified_specs,
        first_dep,
        TYPESCRIPT_WRAPPER_FIRST_DEP_COMPATIBILITY_KEY,
    )?;
    validate_typescript_wrapper_dep_family(
        spec,
        qualified_id,
        qualified_specs,
        second_dep,
        TYPESCRIPT_WRAPPER_SECOND_DEP_COMPATIBILITY_KEY,
    )
}

fn validate_typescript_wrapper_dep_family(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
    dep: &str,
    expected_family: &str,
) -> Result<()> {
    let (dep_qualified_id, dep_spec) = qualified_specs.resolve_dep(
        spec,
        qualified_id.library(),
        dep,
        TYPESCRIPT_WRAPPER_MISSING_DEP_UNSUPPORTED_MESSAGE,
    )?;
    let semantic_review_specs = qualified_specs.semantic_context_for(dep_qualified_id.library());
    let semantic_review_context = SemanticReviewContext::new(&semantic_review_specs);
    let Some(dep_review) =
        evaluate_semantic_review_with_context(dep_spec, &semantic_review_context)
    else {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' is missing supported semantic review",
                TYPESCRIPT_WRAPPER_DEP_FAMILY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    };

    if dep_review.effective_support_status() != SemanticSupportStatus::Supported
        || dep_review.compatibility_key != expected_family
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' resolved to {}",
                TYPESCRIPT_WRAPPER_DEP_FAMILY_UNSUPPORTED_MESSAGE,
                dep_qualified_id,
                dep_review.compatibility_key
            ),
        ));
    }

    if dep_spec
        .spec
        .body
        .typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}'",
                TYPESCRIPT_WRAPPER_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    }

    Ok(())
}

fn validate_typescript_chain3_dep_contract(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
) -> Result<()> {
    let [first_dep, second_dep, third_dep] = spec.spec.deps.as_slice() else {
        return Err(semantic_error(
            spec,
            format!(
                "{}; found {} direct deps",
                TYPESCRIPT_CHAIN3_DEP_ARITY_UNSUPPORTED_MESSAGE,
                spec.spec.deps.len()
            ),
        ));
    };

    validate_typescript_chain3_first_dep_family(spec, qualified_id, qualified_specs, first_dep)?;
    validate_typescript_chain3_dep_family(
        spec,
        qualified_id,
        qualified_specs,
        second_dep,
        TYPESCRIPT_CHAIN3_SECOND_DEP_COMPATIBILITY_KEY,
    )?;
    validate_typescript_chain3_dep_family(
        spec,
        qualified_id,
        qualified_specs,
        third_dep,
        TYPESCRIPT_CHAIN3_THIRD_DEP_COMPATIBILITY_KEY,
    )
}

fn validate_typescript_chain3_first_dep_family(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
    dep: &str,
) -> Result<()> {
    let (dep_qualified_id, dep_spec) = qualified_specs.resolve_dep(
        spec,
        qualified_id.library(),
        dep,
        TYPESCRIPT_CHAIN3_MISSING_DEP_UNSUPPORTED_MESSAGE,
    )?;
    let semantic_review_specs = qualified_specs.semantic_context_for(dep_qualified_id.library());
    let semantic_review_context = SemanticReviewContext::new(&semantic_review_specs);
    let Some(dep_review) =
        evaluate_semantic_review_with_context(dep_spec, &semantic_review_context)
    else {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' is missing supported semantic review",
                TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    };

    if dep_review.effective_support_status() != SemanticSupportStatus::Supported {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' resolved to {}",
                TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE,
                dep_qualified_id,
                dep_review.compatibility_key
            ),
        ));
    }

    match dep_review.compatibility_key.as_str() {
        TYPESCRIPT_CHAIN3_FIRST_DEP_COMPATIBILITY_KEY => {}
        TYPESCRIPT_NORMALIZED_REQUIRED_ARG_WRAPPER_TARGET_COMPATIBILITY_KEY => {}
        TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY => {}
        _ => {
            return Err(semantic_error(
                spec,
                format!(
                    "{}: '{}' resolved to {}",
                    TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE,
                    dep_qualified_id,
                    dep_review.compatibility_key
                ),
            ));
        }
    }

    if dep_spec
        .spec
        .body
        .typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}'",
                TYPESCRIPT_CHAIN3_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    }

    Ok(())
}

fn validate_typescript_chain3_dep_family(
    spec: &LoadedSpec,
    qualified_id: &QualifiedUnitRef,
    qualified_specs: &TypescriptQualifiedSpecIndex,
    dep: &str,
    expected_family: &str,
) -> Result<()> {
    let (dep_qualified_id, dep_spec) = qualified_specs.resolve_dep(
        spec,
        qualified_id.library(),
        dep,
        TYPESCRIPT_CHAIN3_MISSING_DEP_UNSUPPORTED_MESSAGE,
    )?;
    let semantic_review_specs = qualified_specs.semantic_context_for(dep_qualified_id.library());
    let semantic_review_context = SemanticReviewContext::new(&semantic_review_specs);
    let Some(dep_review) =
        evaluate_semantic_review_with_context(dep_spec, &semantic_review_context)
    else {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' is missing supported semantic review",
                TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    };

    if dep_review.effective_support_status() != SemanticSupportStatus::Supported
        || dep_review.compatibility_key != expected_family
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}' resolved to {}",
                TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE,
                dep_qualified_id,
                dep_review.compatibility_key
            ),
        ));
    }

    if dep_spec
        .spec
        .body
        .typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(semantic_error(
            spec,
            format!(
                "{}: '{}'",
                TYPESCRIPT_CHAIN3_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE, dep_qualified_id
            ),
        ));
    }

    Ok(())
}

pub fn validate_typescript_local_test_expect_shape(spec: &LoadedSpec) -> Result<()> {
    let unit_fn = callable_name(&spec.spec.id).to_string();
    for test in &spec.spec.local_tests {
        let expr = validate_expect_expr(test.expect.trim(), false).map_err(|err| {
            SpecError::LocalTestExpectNotExpr {
                id: test.id.clone(),
                message: err.message(),
                path: spec.source.file_path.clone(),
            }
        })?;

        if !is_supported_typescript_expect_expr(&expr, &unit_fn) {
            return Err(semantic_error(spec, TYPESCRIPT_EXPECT_UNSUPPORTED_MESSAGE));
        }
    }
    Ok(())
}

pub fn validate_typescript_molecule_target(test: &LoadedMoleculeTest) -> Result<()> {
    Err(SpecError::SemanticValidation {
        message: TYPESCRIPT_MOLECULE_UNSUPPORTED_MESSAGE.to_string(),
        path: test.source.file_path.clone(),
    })
}

fn is_supported_typescript_expect_expr(expr: &syn::Expr, unit_fn: &str) -> bool {
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    if !matches!(binary.op, syn::BinOp::Eq(_)) {
        return false;
    }

    let syn::Expr::Call(call) = binary.left.as_ref() else {
        return false;
    };
    if !matches!(call.func.as_ref(), syn::Expr::Path(path) if path.path.is_ident(unit_fn)) {
        return false;
    }

    if !call.args.iter().all(|arg| is_decimal_new_expr(arg, true)) {
        return false;
    }

    is_decimal_new_expr(binary.right.as_ref(), true)
}

fn is_decimal_new_expr(expr: &syn::Expr, allow_negative_value: bool) -> bool {
    let syn::Expr::Call(call) = expr else {
        return false;
    };

    let syn::Expr::Path(path) = call.func.as_ref() else {
        return false;
    };
    let segments: Vec<_> = path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    if segments.as_slice() != ["Decimal", "new"] {
        return false;
    }

    if call.args.len() != 2 {
        return false;
    }

    let mut args = call.args.iter();
    let value = args.next().expect("decimal value arg present");
    let scale = args.next().expect("decimal scale arg present");
    is_integer_literal_expr(value, allow_negative_value) && is_integer_literal_expr(scale, false)
}

fn is_integer_literal_expr(expr: &syn::Expr, allow_negative: bool) -> bool {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(_),
            ..
        }) => true,
        syn::Expr::Unary(unary) if allow_negative && matches!(unary.op, syn::UnOp::Neg(_)) => {
            matches!(
                unary.expr.as_ref(),
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(_),
                    ..
                })
            )
        }
        _ => false,
    }
}

fn validate_data_semantic(spec: &LoadedSpec, options: &ValidationOptions) -> Result<()> {
    validate_data_escape_hatches(spec)?;
    validate_data_fields(spec)?;
    validate_data_behavior_presence(spec)?;
    validate_seam_rust_backend(spec)?;
    let constructors = validate_data_constructors(spec)?;
    let methods = validate_seam_methods(spec, "data")?;
    validate_data_seam_collisions(spec, &constructors, &methods)?;
    validate_local_test_expects(spec, options)?;
    Ok(())
}

fn validate_sum_semantic(spec: &LoadedSpec, options: &ValidationOptions) -> Result<()> {
    validate_sum_escape_hatches(spec)?;
    let variants = validate_sum_variants(spec)?;
    validate_seam_rust_backend(spec)?;
    let methods = validate_seam_methods(spec, "sum")?;
    validate_sum_seam_collisions(spec, &variants, &methods)?;
    validate_local_test_expects(spec, options)?;
    Ok(())
}

fn validate_reserved_spec_id_segments(id: &str, file_path: &str) -> Result<()> {
    for segment in id.split('/') {
        if segment == "molecule_tests" {
            return Err(SpecError::ReservedUnitName {
                segment: segment.to_string(),
                path: file_path.to_string(),
            });
        }
    }

    Ok(())
}

fn parse_dep_refs(dep_ids: &[String]) -> std::result::Result<Vec<DepRef>, DepRefParseError> {
    dep_ids.iter().map(|dep| DepRef::parse(dep)).collect()
}

fn invalid_dep_error(err: DepRefParseError, spec: &LoadedSpec) -> SpecError {
    SpecError::SemanticValidation {
        message: err.to_string(),
        path: spec.source.file_path.clone(),
    }
}

fn semantic_error(spec: &LoadedSpec, message: impl Into<String>) -> SpecError {
    SpecError::SemanticValidation {
        message: message.into(),
        path: spec.source.file_path.clone(),
    }
}

fn validate_dep_ref_keywords(dep: &DepRef, file_path: &str) -> Result<()> {
    if let Some(alias) = dep.library_alias() {
        validate_keyword_segment(alias, &dep.to_string(), file_path)?;
    }

    for segment in dep.unit_id().split('/') {
        validate_keyword_segment(segment, &dep.to_string(), file_path)?;
    }

    Ok(())
}

fn validate_keyword_segment(segment: &str, authored: &str, file_path: &str) -> Result<()> {
    if crate::types::is_rust_keyword(segment) {
        return Err(SpecError::RustKeyword {
            segment: segment.to_string(),
            id: authored.to_string(),
            path: file_path.to_string(),
        });
    }

    Ok(())
}

fn validate_projected_rust_identifier(
    spec: &LoadedSpec,
    field: &str,
    authored: &str,
    projected: &str,
) -> Result<()> {
    syn::parse_str::<syn::Ident>(projected).map_err(|_| {
        semantic_error(
            spec,
            format!("{field} '{authored}' projects to invalid Rust identifier '{projected}'"),
        )
    })?;

    Ok(())
}

fn has_dep_ref_collision(deps: &[DepRef]) -> Option<(&DepRef, &DepRef)> {
    for (i, first) in deps.iter().enumerate() {
        for second in &deps[i + 1..] {
            if first.callable_name() == second.callable_name() {
                return Some((first, second));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct ValidatedDataConstructor {
    index: usize,
    id: String,
}

#[derive(Debug, Clone)]
struct ValidatedMethodDep {
    authored: String,
}

#[derive(Debug, Clone)]
struct ValidatedDataMethod {
    index: usize,
    id: String,
    deps: Vec<ValidatedMethodDep>,
}

#[derive(Debug, Clone)]
struct ValidatedSumVariant {
    index: usize,
    id: String,
    variant_name: String,
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

        validate_expect_expr(test.expect.trim(), options.allow_unsafe_local_test_expect).map_err(
            |err| SpecError::LocalTestExpectNotExpr {
                id: test.id.clone(),
                message: err.message(),
                path: path.clone(),
            },
        )?;
    }
    Ok(())
}

fn validate_body_rust_block(spec: &LoadedSpec) -> Result<()> {
    let path = spec.source.file_path.clone();
    syn::parse_str::<syn::Block>(&spec.spec.body.rust).map_err(|_| {
        if syn::parse_str::<syn::ItemFn>(&spec.spec.body.rust).is_ok() {
            SpecError::BodyRustLooksLikeFnDeclaration { path }
        } else {
            SpecError::BodyRustMustBeBlock {
                message: "body.rust must be a Rust block expression starting with `{`".to_string(),
                path,
            }
        }
    })?;
    Ok(())
}

fn validate_contract_types(contract: &Contract, field_root: &str, path: &str) -> Result<()> {
    if let Some(inputs) = &contract.inputs {
        for (name, type_str) in inputs {
            syn::parse_str::<syn::Ident>(name).map_err(|_| {
                SpecError::ContractInputNameInvalid {
                    name: name.clone(),
                    message: "use a snake_case identifier (e.g. my_param)".to_string(),
                    path: path.to_string(),
                }
            })?;
            syn::parse_str::<syn::Type>(type_str).map_err(|err| {
                SpecError::ContractTypeInvalid {
                    field: format!("{field_root}.inputs.{name}"),
                    type_str: type_str.clone(),
                    message: err.to_string(),
                    path: path.to_string(),
                }
            })?;
        }
    }
    if let Some(returns) = &contract.returns {
        syn::parse_str::<syn::Type>(returns).map_err(|err| SpecError::ContractTypeInvalid {
            field: format!("{field_root}.returns"),
            type_str: returns.clone(),
            message: err.to_string(),
            path: path.to_string(),
        })?;
    }
    Ok(())
}

fn validate_data_escape_hatches(spec: &LoadedSpec) -> Result<()> {
    validate_shared_seam_authored_shape(spec, UnitKind::Data)
}

fn validate_sum_escape_hatches(spec: &LoadedSpec) -> Result<()> {
    validate_shared_seam_authored_shape(spec, UnitKind::Sum)?;
    if spec.spec.extensions.data.is_some() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(
                UnitKind::Sum,
                SharedSeamAuthoredShapeRule::SumDataFields,
            ),
        ));
    }
    if !spec.spec.extensions.constructors.is_empty() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(
                UnitKind::Sum,
                SharedSeamAuthoredShapeRule::SumConstructors,
            ),
        ));
    }

    Ok(())
}

fn validate_shared_seam_authored_shape(spec: &LoadedSpec, kind: UnitKind) -> Result<()> {
    debug_assert!(matches!(kind, UnitKind::Data | UnitKind::Sum));

    if spec.spec.contract.is_some() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(kind, SharedSeamAuthoredShapeRule::TopLevelContract),
        ));
    }
    if !spec.spec.deps.is_empty() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(kind, SharedSeamAuthoredShapeRule::TopLevelDeps),
        ));
    }
    if !spec.spec.imports.is_empty() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(kind, SharedSeamAuthoredShapeRule::TopLevelImports),
        ));
    }
    if spec.spec.body.typescript.is_some() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(
                kind,
                SharedSeamAuthoredShapeRule::TopLevelTypescriptBody,
            ),
        ));
    }
    if !spec.spec.body.rust.trim().is_empty() {
        return Err(semantic_error(
            spec,
            shared_surface_violation_message(kind, SharedSeamAuthoredShapeRule::TopLevelRustBody),
        ));
    }

    Ok(())
}

fn validate_named_fields(
    spec: &LoadedSpec,
    fields: &IndexMap<String, AuthoredField>,
    field_root: &str,
) -> Result<()> {
    for (name, field) in fields {
        validate_keyword_segment(name, name, &spec.source.file_path)?;
        syn::parse_str::<syn::Ident>(name).map_err(|_| {
            semantic_error(
                spec,
                format!("{field_root}.{name} must use a snake_case identifier (e.g. subtotal)"),
            )
        })?;
        syn::parse_str::<syn::Type>(&field.type_).map_err(|err| {
            SpecError::ContractTypeInvalid {
                field: format!("{field_root}.{name}.type"),
                type_str: field.type_.clone(),
                message: err.to_string(),
                path: spec.source.file_path.clone(),
            }
        })?;
    }

    Ok(())
}

fn validate_data_fields(spec: &LoadedSpec) -> Result<()> {
    let Some(data) = spec.spec.extensions.data.as_ref() else {
        return Err(semantic_error(spec, "kind:data requires data.fields"));
    };

    validate_named_fields(spec, &data.fields, "data.fields")?;

    Ok(())
}

fn validate_data_behavior_presence(spec: &LoadedSpec) -> Result<()> {
    if spec.spec.extensions.constructors.is_empty() {
        return Err(semantic_error(
            spec,
            "kind:data requires at least one constructor",
        ));
    }
    if spec.spec.extensions.methods.is_empty() {
        return Err(semantic_error(
            spec,
            "kind:data requires at least one method",
        ));
    }

    Ok(())
}

fn validate_seam_rust_backend(spec: &LoadedSpec) -> Result<()> {
    let derives = spec
        .spec
        .extensions
        .backends
        .as_ref()
        .and_then(|backends| backends.rust.as_ref())
        .map(|rust| rust.derives.as_slice())
        .unwrap_or(&[]);

    for (index, derive) in derives.iter().enumerate() {
        syn::parse_str::<syn::Path>(derive).map_err(|err| {
            semantic_error(
                spec,
                format!(
                    "backends.rust.derives[{index}] must be a valid Rust path; got '{derive}': {err}"
                ),
            )
        })?;
    }

    Ok(())
}

fn validate_sum_variants(spec: &LoadedSpec) -> Result<Vec<ValidatedSumVariant>> {
    let Some(sum) = spec.spec.extensions.sum.as_ref() else {
        return Err(semantic_error(spec, "kind:sum requires sum.variants"));
    };
    if sum.variants.is_empty() {
        return Err(semantic_error(
            spec,
            "kind:sum requires at least one sum.variants entry",
        ));
    }

    let enum_name = type_name_for_unit_id(&spec.spec.id);
    let mut seen_ids = HashSet::new();
    let mut seen_variant_names: HashMap<String, (usize, String)> = HashMap::new();
    let mut variants = Vec::new();

    for (index, (variant_id, variant)) in sum.variants.iter().enumerate() {
        if !seen_ids.insert(variant_id.as_str()) {
            return Err(semantic_error(
                spec,
                format!(
                    "duplicate sum variant id '{}' at sum.variants[{index}]",
                    variant_id
                ),
            ));
        }

        validate_keyword_segment(variant_id, variant_id, &spec.source.file_path)?;
        let variant_name = type_name_for_identifier(variant_id);
        validate_projected_rust_identifier(
            spec,
            &format!("sum.variants[{index}].id"),
            variant_id,
            &variant_name,
        )?;
        if variant_name == enum_name {
            return Err(semantic_error(
                spec,
                format!(
                    "sum.variants[{index}].id '{}' projects to Rust variant name '{variant_name}', which conflicts with the emitted enum name '{enum_name}'",
                    variant_id
                ),
            ));
        }
        if let Some((first_index, first_id)) =
            seen_variant_names.insert(variant_name.clone(), (index, variant_id.clone()))
        {
            return Err(semantic_error(
                spec,
                format!(
                    "sum.variants[{index}].id '{}' projects to Rust variant name '{variant_name}', which conflicts with sum.variants[{first_index}].id '{}'",
                    variant_id, first_id
                ),
            ));
        }

        validate_named_fields(
            spec,
            &variant.fields,
            &format!("sum.variants[{index}].fields"),
        )?;
        variants.push(ValidatedSumVariant {
            index,
            id: variant_id.clone(),
            variant_name,
        });
    }

    Ok(variants)
}

fn validate_data_constructors(spec: &LoadedSpec) -> Result<Vec<ValidatedDataConstructor>> {
    let field_names: HashSet<_> = spec
        .spec
        .extensions
        .data
        .as_ref()
        .map(|data| data.fields.keys().cloned().collect())
        .unwrap_or_default();
    let mut seen_ids = HashSet::new();
    let mut constructors = Vec::new();

    for (index, constructor) in spec.spec.extensions.constructors.iter().enumerate() {
        if !seen_ids.insert(constructor.id.as_str()) {
            return Err(semantic_error(
                spec,
                format!(
                    "duplicate constructor id '{}' at constructors[{index}]",
                    constructor.id
                ),
            ));
        }

        validate_keyword_segment(&constructor.id, &constructor.id, &spec.source.file_path)?;
        if let Some(contract) = &constructor.contract {
            validate_contract_types(
                contract,
                &format!("constructors[{index}].contract"),
                &spec.source.file_path,
            )?;
            if contract.returns.is_some() {
                return Err(semantic_error(
                    spec,
                    format!(
                        "constructors[{index}].contract.returns is not allowed; constructor return shape is shared semantics"
                    ),
                ));
            }
        }

        let initialized_fields: HashSet<_> = constructor.initializes.keys().cloned().collect();
        let missing_fields: Vec<_> = field_names
            .iter()
            .filter(|field| !initialized_fields.contains(*field))
            .cloned()
            .collect();
        if !missing_fields.is_empty() {
            return Err(semantic_error(
                spec,
                format!(
                    "constructors[{index}] omits required field initialization for: {}",
                    missing_fields.join(", ")
                ),
            ));
        }

        for (field_name, input_name) in &constructor.initializes {
            if !field_names.contains(field_name) {
                return Err(semantic_error(
                    spec,
                    format!(
                        "constructors[{index}].initializes.{field_name} targets an unknown data field"
                    ),
                ));
            }

            let Some(contract) = &constructor.contract else {
                return Err(semantic_error(
                    spec,
                    format!(
                        "constructors[{index}] must declare contract.inputs for initializes.{field_name}"
                    ),
                ));
            };
            let Some(inputs) = contract.inputs.as_ref() else {
                return Err(semantic_error(
                    spec,
                    format!(
                        "constructors[{index}] must declare contract.inputs for initializes.{field_name}"
                    ),
                ));
            };
            if !inputs.contains_key(input_name) {
                return Err(semantic_error(
                    spec,
                    format!(
                        "constructors[{index}].initializes.{field_name} references unknown input '{input_name}'"
                    ),
                ));
            }
        }

        constructors.push(ValidatedDataConstructor {
            index,
            id: constructor.id.clone(),
        });
    }

    Ok(constructors)
}

fn validate_seam_methods(
    spec: &LoadedSpec,
    seam_kind: &'static str,
) -> Result<Vec<ValidatedDataMethod>> {
    let mut seen_ids = HashSet::new();
    let owner_callable_name = callable_name(&spec.spec.id);
    let mut methods = Vec::new();

    for (index, method) in spec.spec.extensions.methods.iter().enumerate() {
        if !seen_ids.insert(method.id.as_str()) {
            return Err(semantic_error(
                spec,
                format!("duplicate method id '{}' at methods[{index}]", method.id),
            ));
        }

        validate_keyword_segment(&method.id, &method.id, &spec.source.file_path)?;
        MethodReceiver::try_from(method.receiver.as_str()).map_err(|_| {
            semantic_error(
                spec,
                format!(
                    "methods[{index}].receiver uses unsupported mode '{}'; only 'shared_ref' is supported for kind:{seam_kind}",
                    method.receiver,
                ),
            )
        })?;

        let contract = method.contract.as_ref().ok_or_else(|| {
            semantic_error(
                spec,
                format!("methods[{index}].contract is required for kind:{seam_kind}"),
            )
        })?;
        validate_contract_types(
            contract,
            &format!("methods[{index}].contract"),
            &spec.source.file_path,
        )?;

        let dep_refs = parse_dep_refs(&method.deps).map_err(|err| {
            semantic_error(
                spec,
                format!("methods[{index}].deps contains invalid dep: {err}"),
            )
        })?;
        for dep in &dep_refs {
            validate_dep_ref_keywords(dep, &spec.source.file_path).map_err(|err| {
                semantic_error(
                    spec,
                    format!(
                        "methods[{index}].deps contains invalid dep '{}': {err}",
                        dep
                    ),
                )
            })?;
        }
        if let Some(authored_dep) = method
            .deps
            .iter()
            .zip(dep_refs.iter())
            .find(|(_, dep)| dep.callable_name() == owner_callable_name)
            .map(|(authored_dep, _)| authored_dep)
        {
            return Err(semantic_error(
                spec,
                format!(
                    "methods[{index}].deps contains self-dep collision between '{}' and '{}'",
                    authored_dep, spec.spec.id
                ),
            ));
        }
        if let Some((dep1, dep2)) = has_dep_ref_collision(&dep_refs) {
            return Err(semantic_error(
                spec,
                format!(
                    "methods[{index}].deps has callable collision: '{}' and '{}' both resolve to '{}'",
                    dep1,
                    dep2,
                    dep1.callable_name()
                ),
            ));
        }

        let rust_body = method
            .lowering
            .as_ref()
            .and_then(|lowering| lowering.rust.as_ref())
            .map(|rust| rust.body.trim())
            .filter(|body| !body.is_empty())
            .ok_or_else(|| {
                semantic_error(
                    spec,
                    format!(
                        "methods[{index}].lowering.rust.body must be provided for kind:{seam_kind}"
                    ),
                )
            })?;
        syn::parse_str::<syn::Block>(rust_body).map_err(|err| {
            semantic_error(
                spec,
                format!(
                    "methods[{index}].lowering.rust.body must be a Rust block expression starting with `{{`: {err}"
                ),
            )
        })?;

        methods.push(ValidatedDataMethod {
            index,
            id: method.id.clone(),
            deps: method
                .deps
                .iter()
                .cloned()
                .map(|authored| ValidatedMethodDep { authored })
                .collect(),
        });
    }

    Ok(methods)
}

fn validate_data_seam_collisions(
    spec: &LoadedSpec,
    constructors: &[ValidatedDataConstructor],
    methods: &[ValidatedDataMethod],
) -> Result<()> {
    for constructor in constructors {
        if let Some(method) = methods.iter().find(|method| method.id == constructor.id) {
            return Err(semantic_error(
                spec,
                format!(
                    "constructors[{}].id '{}' conflicts with methods[{}].id '{}'",
                    constructor.index, constructor.id, method.index, method.id
                ),
            ));
        }
    }

    let seam_deps = ordered_unique_deps(
        methods
            .iter()
            .flat_map(|method| method.deps.iter().map(|dep| dep.authored.as_str())),
    );
    let seam_dep_refs = parse_dep_refs(&seam_deps).map_err(|err| invalid_dep_error(err, spec))?;
    if let Some((dep1, dep2)) = has_dep_ref_collision(&seam_dep_refs) {
        return Err(SpecError::DepCollision {
            dep1: dep1.authored().to_string(),
            dep2: dep2.authored().to_string(),
            fn_name: dep1.callable_name().to_string(),
            path: spec.source.file_path.clone(),
        });
    }

    Ok(())
}

fn validate_sum_seam_collisions(
    spec: &LoadedSpec,
    variants: &[ValidatedSumVariant],
    methods: &[ValidatedDataMethod],
) -> Result<()> {
    let mut seen_variant_names: HashMap<&str, (&str, usize)> = HashMap::new();
    for variant in variants {
        if let Some((first_id, first_index)) =
            seen_variant_names.insert(variant.variant_name.as_str(), (&variant.id, variant.index))
        {
            return Err(semantic_error(
                spec,
                format!(
                    "sum.variants[{}].id '{}' projects to Rust variant name '{}', which conflicts with sum.variants[{}].id '{}'",
                    variant.index, variant.id, variant.variant_name, first_index, first_id,
                ),
            ));
        }
    }

    let seam_deps = ordered_unique_deps(
        methods
            .iter()
            .flat_map(|method| method.deps.iter().map(|dep| dep.authored.as_str())),
    );
    let seam_dep_refs = parse_dep_refs(&seam_deps).map_err(|err| invalid_dep_error(err, spec))?;
    if let Some((dep1, dep2)) = has_dep_ref_collision(&seam_dep_refs) {
        return Err(SpecError::DepCollision {
            dep1: dep1.authored().to_string(),
            dep2: dep2.authored().to_string(),
            fn_name: dep1.callable_name().to_string(),
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

/// Check for duplicate IDs across all loaded specs.
///
/// Returns all duplicate pairs, not just the first. Each additional file that
/// shares an ID produces a separate error citing the original file as file1.
pub fn validate_no_duplicate_ids(specs: &[LoadedSpec]) -> Vec<SpecError> {
    let scoped_specs: Vec<_> = specs
        .iter()
        .map(QualifiedLoadedSpec::local_identity)
        .collect();
    validate_no_duplicate_qualified_ids(&scoped_specs)
}

pub fn validate_no_duplicate_qualified_ids(specs: &[QualifiedLoadedSpec<'_>]) -> Vec<SpecError> {
    let mut seen: HashMap<QualifiedUnitRef, String> = HashMap::new();
    let mut errors = Vec::new();

    for spec in specs {
        if let Some(existing_file) = seen.get(&spec.qualified_id) {
            errors.push(SpecError::DuplicateId {
                id: spec.qualified_id.to_string(),
                file1: existing_file.clone(),
                file2: spec.loaded.source.file_path.clone(),
            });
        } else {
            seen.insert(
                spec.qualified_id.clone(),
                spec.loaded.source.file_path.clone(),
            );
        }
    }

    errors
}

/// Emit a warning for each spec that lacks a spec_version field.
///
/// Called after per-spec semantic validation; warnings are non-fatal.
pub fn check_spec_versions(specs: &[LoadedSpec]) -> Vec<SpecWarning> {
    specs
        .iter()
        .filter(|s| s.spec.spec_version.is_none())
        .map(|s| SpecWarning::MissingSpecVersion {
            path: s.source.file_path.clone(),
            version: AUTHORED_SPEC_VERSION,
        })
        .collect()
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
    let mut errors = Vec::<SpecError>::new();
    let mut scoped_specs = Vec::new();

    for spec in specs {
        match QualifiedLoadedSpec::local(spec) {
            Ok(scoped_spec) => scoped_specs.push(scoped_spec),
            Err(err) => errors.push(invalid_dep_error(err, spec)),
        }
    }

    let (mut dep_errors, warnings) =
        validate_qualified_deps_exist_with_options(&scoped_specs, options);
    errors.append(&mut dep_errors);

    (errors, warnings)
}

pub fn validate_qualified_deps_exist_with_options(
    specs: &[QualifiedLoadedSpec<'_>],
    options: &ValidationOptions,
) -> (Vec<SpecError>, Vec<SpecWarning>) {
    let ids: HashSet<_> = specs.iter().map(|spec| spec.qualified_id.clone()).collect();

    let mut errors = Vec::<SpecError>::new();
    let mut warnings = Vec::<SpecWarning>::new();
    for spec in specs {
        for dep in &spec.qualified_deps {
            if !ids.contains(dep) {
                let err = if dep.library().is_some() {
                    SpecError::CrossLibraryDepNotFound {
                        dep: dep.to_string(),
                        path: spec.loaded.source.file_path.clone(),
                    }
                } else {
                    SpecError::MissingDep {
                        dep: dep.to_string(),
                        path: spec.loaded.source.file_path.clone(),
                    }
                };
                if options.strict_deps {
                    errors.push(err);
                } else {
                    warnings.push(SpecWarning::MissingDep {
                        dep: dep.to_string(),
                        path: spec.loaded.source.file_path.clone(),
                    });
                }
            }
        }
    }

    let cycle_errors = detect_qualified_cycles(specs);
    errors.extend(cycle_errors);

    (errors, warnings)
}

/// DFS helper for cycle detection. Mutates `visited`, `in_stack`, `stack`, and `errors` in place.
fn dfs_qualified_cycle_check<'a>(
    node_id: &QualifiedUnitRef,
    id_map: &HashMap<QualifiedUnitRef, &'a QualifiedLoadedSpec<'a>>,
    visited: &mut HashSet<QualifiedUnitRef>,
    in_stack: &mut HashSet<QualifiedUnitRef>,
    stack: &mut Vec<QualifiedUnitRef>,
    errors: &mut Vec<SpecError>,
) {
    in_stack.insert(node_id.clone());
    stack.push(node_id.clone());

    if let Some(spec) = id_map.get(node_id) {
        for dep in &spec.qualified_deps {
            if !id_map.contains_key(dep) {
                // Missing dep — already reported by validate_deps_exist; skip during DFS
                continue;
            }
            if in_stack.contains(dep) {
                // Cycle found — reconstruct path from the point where dep appears on the stack
                let cycle_start = stack
                    .iter()
                    .position(|n| n == dep)
                    .expect("dep in in_stack must be in stack");
                let mut qualified_cycle_path = stack[cycle_start..].to_vec();
                qualified_cycle_path.push(dep.clone());
                let cycle_path: Vec<String> = qualified_cycle_path
                    .iter()
                    .map(QualifiedUnitRef::authored)
                    .collect();
                let distinct_libraries: HashSet<Option<&str>> = qualified_cycle_path
                    .iter()
                    .map(|unit| unit.library())
                    .collect();
                let path = spec.loaded.source.file_path.clone();

                if distinct_libraries.len() > 1 {
                    errors.push(SpecError::CrossLibraryCycle { cycle_path, path });
                } else {
                    errors.push(SpecError::CyclicDep { cycle_path, path });
                }
            } else if !visited.contains(dep) {
                dfs_qualified_cycle_check(dep, id_map, visited, in_stack, stack, errors);
            }
        }
    }

    stack.pop();
    in_stack.remove(node_id);
    visited.insert(node_id.clone());
}

/// Detect cycles in the dependency graph using depth-first search.
///
/// Cycles are always errors regardless of ValidationOptions — a cycle causes
/// infinite recursion during graph resolution.
///
/// NOTE: Cycle detection is in-tree only. Deps that reference units outside this
/// spec set (cross-library) are skipped during DFS — they are not in id_map.
/// The cross-library dep schema is locked in DECISIONS.md as namespace-prefixed
/// ids (for example `shared::money/round`), but cross-library cycle detection is
/// still deferred until M5 implements cross-library loading and validation.
pub fn detect_cycles(specs: &[LoadedSpec]) -> Vec<SpecError> {
    let mut errors = Vec::new();
    let mut scoped_specs = Vec::new();

    for spec in specs {
        match QualifiedLoadedSpec::local(spec) {
            Ok(scoped_spec) => scoped_specs.push(scoped_spec),
            Err(err) => errors.push(invalid_dep_error(err, spec)),
        }
    }

    errors.extend(detect_qualified_cycles(&scoped_specs));
    errors
}

pub fn detect_qualified_cycles(specs: &[QualifiedLoadedSpec<'_>]) -> Vec<SpecError> {
    let id_map: HashMap<QualifiedUnitRef, &QualifiedLoadedSpec<'_>> = specs
        .iter()
        .map(|spec| (spec.qualified_id.clone(), spec))
        .collect();

    let mut visited = HashSet::<QualifiedUnitRef>::new();
    let mut errors = Vec::<SpecError>::new();

    for spec in specs {
        if !visited.contains(&spec.qualified_id) {
            let mut in_stack = HashSet::<QualifiedUnitRef>::new();
            let mut stack = Vec::<QualifiedUnitRef>::new();
            dfs_qualified_cycle_check(
                &spec.qualified_id,
                &id_map,
                &mut visited,
                &mut in_stack,
                &mut stack,
                &mut errors,
            );
        }
    }

    errors
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

fn compiled_test_schema() -> Result<&'static jsonschema::Validator> {
    if let Some(schema) = COMPILED_TEST_SCHEMA.get() {
        return Ok(schema);
    }

    let schema_json: Value = serde_json::from_str(TEST_SCHEMA_JSON).map_err(SpecError::Json)?;
    let schema =
        jsonschema::draft7::new(&schema_json).map_err(|e| SpecError::SchemaValidation {
            message: format!("Test schema compilation failed: {e}"),
            path: "<test.spec.json schema>".to_string(),
        })?;

    let _ = COMPILED_TEST_SCHEMA.set(schema);

    Ok(COMPILED_TEST_SCHEMA
        .get()
        .expect("COMPILED_TEST_SCHEMA must be set after successful compilation"))
}

/// Validate a raw YAML-authored molecule test value against the test.spec JSON Schema.
///
/// Used by the loader before deserialization so unknown fields and authoring mistakes
/// are rejected before serde can apply defaults or drop data.
pub fn validate_raw_molecule_test_yaml(yaml_value: &YamlValue, file_path: &str) -> Result<()> {
    let spec_json = serde_json::to_value(yaml_value).map_err(SpecError::Json)?;
    let schema = compiled_test_schema()?;

    let validation_result = schema.validate(&spec_json);
    match validation_result {
        Ok(()) => Ok(()),
        Err(error) => Err(SpecError::SchemaValidation {
            message: humanize_validation_error(&error),
            path: file_path.to_string(),
        }),
    }
}

/// Perform per-test semantic validation on a loaded molecule test.
///
/// Checks:
/// 1. body.rust must parse as syn::Block
/// 2. body.rust must not contain `unsafe` blocks
/// 3. id segments must not be Rust reserved keywords
/// 4. id segments must not use reserved generated namespace names
pub fn validate_molecule_test_semantic(test: &LoadedMoleculeTest) -> Result<()> {
    if test.test.body.typescript.is_some() {
        return Err(SpecError::SemanticValidation {
            message:
                "body.typescript is not supported in .test.spec; molecule tests remain Rust-only"
                    .to_string(),
            path: test.source.file_path.clone(),
        });
    }

    syn::parse_str::<syn::Block>(&test.test.body.rust).map_err(|e| {
        SpecError::MoleculeBodyRustMustBeBlock {
            message: e.to_string(),
            test_path: test.source.file_path.clone(),
        }
    })?;

    let tokens = test
        .test
        .body
        .rust
        .parse::<proc_macro2::TokenStream>()
        .map_err(|e| SpecError::MoleculeBodyRustMustBeBlock {
            message: e.to_string(),
            test_path: test.source.file_path.clone(),
        })?;

    if token_stream_contains_unsafe_keyword(&tokens) {
        return Err(SpecError::MoleculeBodyContainsUnsafe {
            test_path: test.source.file_path.clone(),
        });
    }

    validate_rust_keywords(&test.test.id, &test.source.file_path)?;
    validate_reserved_spec_id_segments(&test.test.id, &test.source.file_path)?;

    Ok(())
}

/// Validate that all cover_ids reference real unit IDs in the loaded spec set.
///
/// Returns errors for each missing cover and a warning if covers is empty.
pub fn validate_molecule_test_covers(
    test: &LoadedMoleculeTest,
    unit_ids: &HashSet<&str>,
) -> (Vec<SpecError>, Vec<SpecWarning>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut existing_covers = Vec::new();

    if test.test.covers.is_empty() {
        warnings.push(SpecWarning::MoleculeTestNoCoveredUnits {
            test_id: test.test.id.clone(),
            test_path: test.source.file_path.clone(),
        });
    }

    if test.test.imports.is_none() {
        warnings.push(SpecWarning::MoleculeImplicitImportsDeprecated {
            test_id: test.test.id.clone(),
            test_path: test.source.file_path.clone(),
        });
    }

    for cover_id in &test.test.covers {
        let dep_ref = match DepRef::parse(cover_id) {
            Ok(dep_ref) => dep_ref,
            Err(err) => {
                errors.push(SpecError::SchemaValidation {
                    message: err.to_string(),
                    path: test.source.file_path.clone(),
                });
                continue;
            }
        };

        if dep_ref.library_alias().is_some() {
            errors.push(SpecError::CrossLibraryMoleculeCoverUnsupported {
                cover_id: cover_id.clone(),
                test_id: test.test.id.clone(),
                test_path: test.source.file_path.clone(),
            });
        } else if !unit_ids.contains(dep_ref.unit_id()) {
            errors.push(SpecError::MoleculeCoversNotFound {
                cover_id: cover_id.clone(),
                test_id: test.test.id.clone(),
                test_path: test.source.file_path.clone(),
            });
        } else {
            existing_covers.push(cover_id.clone());
        }
    }

    if test.test.imports.is_none()
        && let Some((cover1, cover2, fn_name)) = has_callable_collision(&existing_covers)
    {
        errors.push(SpecError::MoleculeCoversCollision {
            cover1: cover1.clone(),
            cover2: cover2.clone(),
            fn_name: fn_name.to_string(),
            test_id: test.test.id.clone(),
            test_path: test.source.file_path.clone(),
        });
    }

    (errors, warnings)
}

/// Check for duplicate IDs across all loaded molecule tests.
///
/// Returns all duplicate pairs. Each additional file that shares an ID produces
/// a separate error citing the first file as file1.
pub fn validate_no_duplicate_molecule_test_ids(tests: &[LoadedMoleculeTest]) -> Vec<SpecError> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut errors = Vec::new();

    for test in tests {
        if let Some(existing_file) = seen.get(&test.test.id) {
            errors.push(SpecError::DuplicateMoleculeTestId {
                id: test.test.id.clone(),
                file1: existing_file.clone(),
                file2: test.source.file_path.clone(),
            });
        } else {
            seen.insert(test.test.id.clone(), test.source.file_path.clone());
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering, AuthoredSumShape,
        AuthoredSumVariant, Body, Contract, Intent, LocalTest, MoleculeTestSource,
        MoleculeTestStruct, QualifiedUnitRef, SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;

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
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_lane_function_spec() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/apply_tax.unit.spec".to_string(),
                id: "pricing/apply_tax".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_tax".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Return the subtotal after applying the tax rate.".to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec!["output >= subtotal".to_string()],
                }),
                deps: vec![],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: "{ subtotal + subtotal * rate }".to_string(),
                    typescript: Some("return subtotal.add(subtotal.mul(rate));".to_string()),
                },
                local_tests: vec![LocalTest {
                    id: "taxes_subtotal".to_string(),
                    expect: "apply_tax(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(1070, 2)".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_helper_spec() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/money/round.unit.spec".to_string(),
                id: "money/round".to_string(),
            },
            spec: SpecStruct {
                id: "money/round".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Round a decimal value to two fractional digits for pricing flows."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([(
                        "value".to_string(),
                        "rust_decimal::Decimal".to_string(),
                    )])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![
                    "rust_decimal::Decimal".to_string(),
                    "rust_decimal::RoundingStrategy".to_string(),
                ],
                body: Body {
                    rust: "{ value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero) }"
                        .to_string(),
                    typescript: Some("return value;".to_string()),
                },
                local_tests: vec![LocalTest {
                    id: "rounds_decimal".to_string(),
                    expect: "round(Decimal::new(1005, 2)) == Decimal::new(1005, 2)".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_discount_spec() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/apply_discount.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Return the subtotal after applying the discount rate and clamping at zero."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec!["output <= subtotal".to_string(), "output >= 0".to_string()],
                }),
                deps: vec!["money/round".to_string()],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: "{ let discounted = subtotal - subtotal * rate; round(discounted.max(Decimal::ZERO)) }".to_string(),
                    typescript: Some(
                        "const discounted = subtotal.sub(subtotal.mul(rate)); return round(discounted);"
                            .to_string(),
                    ),
                },
                local_tests: vec![LocalTest {
                    id: "discounts_subtotal".to_string(),
                    expect: "apply_discount(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(930, 2)".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_wrapper_spec() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/calculate_total.unit.spec".to_string(),
                id: "pricing/calculate_total".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/calculate_total".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Combine discount and tax so a checkout flow can produce the final price."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("discount_rate".to_string(), "rust_decimal::Decimal".to_string()),
                        ("tax_rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec!["output >= 0".to_string()],
                }),
                deps: vec![
                    "pricing/apply_discount".to_string(),
                    "pricing/apply_tax".to_string(),
                ],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: "{ let discounted = apply_discount(subtotal, discount_rate); apply_tax(discounted, tax_rate) }".to_string(),
                    typescript: Some(
                        "const discounted = apply_discount(subtotal, discount_rate); return apply_tax(discounted, tax_rate);"
                            .to_string(),
                    ),
                },
                local_tests: vec![LocalTest {
                    id: "combined_flow".to_string(),
                    expect: "calculate_total(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2)) == Decimal::new(9951, 3)".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_normalized_required_arg_wrapper_spec() -> LoadedSpec {
        let mut spec = create_typescript_wrapper_spec();
        spec.spec.body.rust = "{ let discounted = apply_discount(subtotal, discount_rate); apply_tax(discounted, tax_rate.max(Decimal::ZERO)) }".to_string();
        spec.spec.body.typescript = Some(
            "const discounted = apply_discount(subtotal, discount_rate); return apply_tax(discounted, tax_rate.max(Decimal.zero()));"
                .to_string(),
        );
        spec
    }

    fn create_typescript_chain3_spec() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/checkout_chain3.unit.spec".to_string(),
                id: "pricing/checkout_chain3".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/checkout_chain3".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("discount_rate".to_string(), "rust_decimal::Decimal".to_string()),
                        ("tax_rate".to_string(), "rust_decimal::Decimal".to_string()),
                        ("surcharge_rate".to_string(), "rust_decimal::Decimal".to_string()),
                        ("loyalty_rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec![],
                }),
                deps: vec![
                    "pricing/calculate_total".to_string(),
                    "pricing/apply_tax".to_string(),
                    "pricing/apply_discount".to_string(),
                ],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: "{ let base_total = calculate_total(subtotal, discount_rate, tax_rate); let surcharged_total = apply_tax(base_total, surcharge_rate); apply_discount(surcharged_total, loyalty_rate) }".to_string(),
                    typescript: Some(
                        "const base_total = calculate_total(subtotal, discount_rate, tax_rate); const surcharged_total = apply_tax(base_total, surcharge_rate); return apply_discount(surcharged_total, loyalty_rate);"
                            .to_string(),
                    ),
                },
                local_tests: vec![LocalTest {
                    id: "chain3_flow".to_string(),
                    expect: "checkout_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(5, 2), Decimal::new(5, 2)) == Decimal::new(992638, 5)".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn create_typescript_base_nested_chain3_spec() -> LoadedSpec {
        let mut spec = create_typescript_chain3_spec();
        spec.source.file_path = "units/pricing/base_nested_chain3.unit.spec".to_string();
        spec.source.id = "pricing/base_nested_chain3".to_string();
        spec.spec.id = "pricing/base_nested_chain3".to_string();
        spec.spec.local_tests[0].id = "base_nested_chain3_flow".to_string();
        spec.spec.local_tests[0].expect =
            "base_nested_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(0, 2), Decimal::new(0, 2)) == Decimal::new(9951, 3)"
                .to_string();
        spec
    }

    fn create_typescript_checkout_nested_chain3_spec() -> LoadedSpec {
        let mut spec = create_typescript_chain3_spec();
        spec.source.file_path = "units/pricing/checkout_nested_chain3.unit.spec".to_string();
        spec.source.id = "pricing/checkout_nested_chain3".to_string();
        spec.spec.id = "pricing/checkout_nested_chain3".to_string();
        spec.spec.deps[0] = "pricing/base_nested_chain3".to_string();
        spec.spec.body.rust = "{ let base_total = base_nested_chain3(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate); let surcharged_total = apply_tax(base_total, Decimal::ZERO); apply_discount(surcharged_total, Decimal::ZERO) }".to_string();
        spec.spec.body.typescript = Some(
            "const base_total = base_nested_chain3(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate); const surcharged_total = apply_tax(base_total, Decimal.zero()); return apply_discount(surcharged_total, Decimal.zero());"
                .to_string(),
        );
        spec.spec.local_tests[0].id = "checkout_nested_chain3_flow".to_string();
        spec.spec.local_tests[0].expect =
            "checkout_nested_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(0, 2), Decimal::new(0, 2)) == Decimal::new(9951, 3)"
                .to_string();
        spec
    }

    fn create_molecule_test_spec(id: &str, covers: Vec<&str>) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: format!("test/{}.test.spec", id),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: format!("Test molecule spec for {}", id),
                },
                covers: covers.into_iter().map(str::to_string).collect(),
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: None,
            },
        }
    }

    fn create_data_spec(id: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("test/{}.unit.spec", id),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: format!("Test data seam for {}", id),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: String::new(),
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![AuthoredConstructor {
                        id: "new".to_string(),
                        intent: Intent {
                            why: "Create a data seam".to_string(),
                        },
                        contract: Some(Contract {
                            inputs: Some(IndexMap::from([
                                ("subtotal".to_string(), "Decimal".to_string()),
                                ("tax_rate".to_string(), "Decimal".to_string()),
                            ])),
                            returns: None,
                            invariants: vec![],
                        }),
                        initializes: IndexMap::from([
                            ("subtotal".to_string(), "subtotal".to_string()),
                            ("tax_rate".to_string(), "tax_rate".to_string()),
                        ]),
                    }],
                    methods: vec![AuthoredMethod {
                        id: "total".to_string(),
                        intent: Intent {
                            why: "Return the total".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("Decimal".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec!["pricing/apply_tax".to_string()],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ apply_tax(self.subtotal, self.tax_rate) }".to_string(),
                            }),
                        }),
                    }],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec!["Clone".to_string(), "Debug".to_string()],
                        }),
                    }),
                    sum: None,
                },
            },
        }
    }

    fn create_sum_spec(id: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("test/{}.unit.spec", id),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "sum".to_string(),
                intent: Intent {
                    why: format!("Test sum seam for {}", id),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: UnitExtensions {
                    sum: Some(AuthoredSumShape {
                        variants: IndexMap::from([
                            (
                                "pending".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::new(),
                                },
                            ),
                            (
                                "quoted_total".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::from([(
                                        "subtotal".to_string(),
                                        AuthoredField {
                                            type_: "Decimal".to_string(),
                                        },
                                    )]),
                                },
                            ),
                        ]),
                    }),
                    methods: vec![AuthoredMethod {
                        id: "label".to_string(),
                        intent: Intent {
                            why: "Return a stable label".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("&'static str".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec![],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ match self { Self::Pending => \"pending\", Self::QuotedTotal { .. } => \"quoted_total\" } }".to_string(),
                            }),
                        }),
                    }],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec!["Clone".to_string(), "Debug".to_string()],
                        }),
                    }),
                    ..UnitExtensions::default()
                },
            },
        }
    }

    #[test]
    fn test_validate_schema_valid() {
        let spec = create_test_spec("pricing/apply_discount", "{ }");
        let result = validate_schema(&spec);
        assert!(
            result.is_ok(),
            "Schema validation should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_schema_valid_spec_passes() {
        let spec = create_test_spec("pricing/apply_discount", "{ }");
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
        assert!(err.contains("unknown field"));
    }

    #[test]
    fn test_validate_raw_yaml_accepts_kind_data_shape_without_placeholder_body() {
        let yaml = r#"
id: pricing/checkout_quote
kind: data
intent:
  why: Quote a checkout total.
data:
  fields:
    subtotal:
      type: Decimal
    tax_rate:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
        tax_rate: Decimal
    initializes:
      subtotal: subtotal
      tax_rate: tax_rate
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.subtotal, self.tax_rate)
          }
backends:
  rust:
    derives:
      - Clone
      - Debug
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(
            result.is_ok(),
            "Expected valid kind:data shape to pass schema validation: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_raw_yaml_accepts_kind_data_shape_with_empty_placeholder_body() {
        let yaml = r#"
id: pricing/checkout_quote
kind: data
intent:
  why: Quote a checkout total.
body: {}
data:
  fields:
    subtotal:
      type: Decimal
    tax_rate:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
        tax_rate: Decimal
    initializes:
      subtotal: subtotal
      tax_rate: tax_rate
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.subtotal, self.tax_rate)
          }
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(
            result.is_ok(),
            "Expected kind:data body placeholder to pass schema validation: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_raw_yaml_accepts_kind_sum_shape_without_placeholder_body() {
        let yaml = r#"
id: pricing/checkout_status
kind: sum
intent:
  why: Track checkout state.
sum:
  variants:
    pending: {}
    quoted_total:
      fields:
        subtotal:
          type: Decimal
methods:
  - id: label
    intent:
      why: Return a stable label.
    receiver: shared_ref
    contract:
      returns: "&'static str"
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::Pending => "pending",
                  Self::QuotedTotal { .. } => "quoted_total",
              }
          }
backends:
  rust:
    derives:
      - Clone
      - Debug
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let result = validate_raw_yaml(&value, "test.unit.spec");
        assert!(
            result.is_ok(),
            "Expected valid kind:sum shape to pass schema validation: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_raw_yaml_rejects_kind_data_shape_with_unknown_body_key() {
        let yaml = r#"
id: pricing/checkout_quote
kind: data
intent:
  why: Quote a checkout total.
body:
  unexpected: true
data:
  fields:
    subtotal:
      type: Decimal
    tax_rate:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
        tax_rate: Decimal
    initializes:
      subtotal: subtotal
      tax_rate: tax_rate
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.subtotal, self.tax_rate)
          }
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Schema validation failed"), "{err}");
        assert!(err.contains("unexpected"), "{err}");
    }

    #[test]
    fn test_validate_raw_yaml_rejects_kind_function_shape_with_empty_placeholder_body() {
        let yaml = r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
contract:
  returns: Decimal
body: {}
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Schema validation failed"), "{err}");
        assert!(err.contains("rust"), "{err}");
    }

    #[test]
    fn test_validate_raw_yaml_rejects_kind_data_without_constructors() {
        let yaml = r#"
id: pricing/checkout_quote
kind: data
intent:
  why: Quote a checkout total.
data:
  fields:
    subtotal:
      type: Decimal
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    lowering:
      rust:
        body: |
          {
              self.subtotal
          }
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Schema validation failed"), "{err}");
        assert!(
            err.contains("missing required field: \"constructors\""),
            "{err}"
        );
    }

    #[test]
    fn test_validate_raw_yaml_rejects_kind_data_with_empty_methods() {
        let yaml = r#"
id: pricing/checkout_quote
kind: data
intent:
  why: Quote a checkout total.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods: []
"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();

        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Schema validation failed"), "{err}");
        assert!(
            err.contains("[] has less than 1 item (at /methods)"),
            "{err}"
        );
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
            create_test_spec("pricing/apply_discount", "{ }"),
            create_test_spec("utils/round", "{ }"),
        ];

        let errors = validate_no_duplicate_ids(&specs);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_duplicate_ids() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "{ }"),
            create_test_spec("pricing/apply_discount", "{ }"),
        ];

        let errors = validate_no_duplicate_ids(&specs);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].to_string().contains("Duplicate ID"));
        assert!(errors[0].to_string().contains("pricing/apply_discount"));
    }

    #[test]
    fn test_validate_duplicate_ids_all_reported() {
        let specs = vec![
            create_test_spec("pricing/apply_discount", "{ }"),
            create_test_spec("pricing/apply_discount", "{ }"),
            create_test_spec("pricing/apply_discount", "{ }"),
        ];

        let errors = validate_no_duplicate_ids(&specs);
        assert_eq!(errors.len(), 2, "all duplicate pairs should be reported");
    }

    #[test]
    fn test_validate_dep_collision() {
        let mut spec = create_test_spec("pricing/calculate_total", "{ round(1.5) }");
        spec.spec.deps = vec!["money/round".to_string(), "utils/round".to_string()];

        let result = validate_semantic(&spec);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("collision"));
        assert!(err.to_string().contains("round"));
    }

    #[test]
    fn test_validate_dep_collision_with_unit_callable_name() {
        let mut spec = create_test_spec("money/round", "{ round(value) }");
        spec.spec.deps = vec!["shared::money/round".to_string()];

        let err = validate_semantic(&spec).unwrap_err();
        match err {
            SpecError::DepCollision {
                dep1,
                dep2,
                fn_name,
                path,
            } => {
                assert_eq!(dep1, "shared::money/round");
                assert_eq!(dep2, "money/round");
                assert_eq!(fn_name, "round");
                assert_eq!(path, "test/money/round.unit.spec");
            }
            other => panic!("expected DepCollision, got {other:?}"),
        }
    }

    #[test]
    fn test_validate_semantic_accepts_external_dep_syntax() {
        let mut spec = create_test_spec("pricing/calculate_total", "{ round(1.5) }");
        spec.spec.deps = vec!["shared::money/round".to_string()];

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_semantic_rejects_invalid_external_dep_syntax() {
        let mut spec = create_test_spec("pricing/calculate_total", "{ round(1.5) }");
        spec.spec.deps = vec!["shared::Money/round".to_string()];

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("invalid segment 'Money'"), "{err}");
    }

    #[test]
    fn test_validate_molecule_test_covers_collision() {
        let molecule_test =
            create_molecule_test_spec("pricing/rounding_flow", vec!["money/round", "utils/round"]);
        let unit_ids: HashSet<&str> = ["money/round", "utils/round"].into_iter().collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert_eq!(warnings.len(), 1);
        assert!(matches!(
            warnings[0],
            SpecWarning::MoleculeImplicitImportsDeprecated { .. }
        ));
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SpecError::MoleculeCoversCollision {
                cover1,
                cover2,
                fn_name,
                test_id,
                test_path,
            } => {
                assert_eq!(cover1, "money/round");
                assert_eq!(cover2, "utils/round");
                assert_eq!(fn_name, "round");
                assert_eq!(test_id, "pricing/rounding_flow");
                assert_eq!(test_path, "test/pricing/rounding_flow.test.spec");
            }
            other => panic!("expected MoleculeCoversCollision, got {other:?}"),
        }
    }

    #[test]
    fn test_validate_molecule_test_covers_allows_collision_with_explicit_imports() {
        let mut molecule_test =
            create_molecule_test_spec("pricing/rounding_flow", vec!["money/round", "utils/round"]);
        molecule_test.test.imports = Some(vec![]);
        let unit_ids: HashSet<&str> = ["money/round", "utils/round"].into_iter().collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert!(
            errors.is_empty(),
            "explicit imports should disable cover collision errors"
        );
        assert!(
            warnings.iter().all(|warning| !matches!(
                warning,
                SpecWarning::MoleculeImplicitImportsDeprecated { .. }
            )),
            "explicit imports should suppress implicit-import deprecation warnings"
        );
    }

    #[test]
    fn test_validate_molecule_test_covers_rejects_cross_library_cover_without_collision() {
        let molecule_test = create_molecule_test_spec(
            "pricing/rounding_flow",
            vec!["shared::money/round", "money/round"],
        );
        let unit_ids: HashSet<&str> = ["money/round"].into_iter().collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert_eq!(warnings.len(), 1);
        assert!(matches!(
            warnings[0],
            SpecWarning::MoleculeImplicitImportsDeprecated { .. }
        ));
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SpecError::CrossLibraryMoleculeCoverUnsupported {
                cover_id,
                test_id,
                test_path,
            } => {
                assert_eq!(cover_id, "shared::money/round");
                assert_eq!(test_id, "pricing/rounding_flow");
                assert_eq!(test_path, "test/pricing/rounding_flow.test.spec");
            }
            other => panic!("expected CrossLibraryMoleculeCoverUnsupported, got {other:?}"),
        }
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
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
    fn test_validate_qualified_duplicate_ids_allow_same_unit_in_different_libraries() {
        let local = create_test_spec("money/round", "{ }");
        let shared = create_test_spec("money/round", "{ }");

        let scoped_specs = vec![
            QualifiedLoadedSpec {
                loaded: &local,
                qualified_id: QualifiedUnitRef::new(Some("root".to_string()), "money/round"),
                qualified_deps: Vec::new(),
            },
            QualifiedLoadedSpec {
                loaded: &shared,
                qualified_id: QualifiedUnitRef::new(Some("shared".to_string()), "money/round"),
                qualified_deps: Vec::new(),
            },
        ];

        let errors = validate_no_duplicate_qualified_ids(&scoped_specs);
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn test_validate_deps_exist_local_wrapper_still_passes_local_graph() {
        let mut calculate_total = create_test_spec("pricing/calculate_total", "{ round(1.5) }");
        calculate_total.spec.deps = vec!["money/round".to_string()];
        let round = create_test_spec("money/round", "{ amount }");

        let (errors, warnings) = validate_deps_exist(&[calculate_total, round]);
        assert!(errors.is_empty(), "{errors:?}");
        assert!(warnings.is_empty(), "{warnings:?}");
    }

    #[test]
    fn test_validate_deps_exist_local_wrapper_reports_external_missing_dep() {
        let mut calculate_total = create_test_spec("pricing/calculate_total", "{ round(1.5) }");
        calculate_total.spec.deps = vec!["shared::money/round".to_string()];

        let (errors, warnings) = validate_deps_exist(&[calculate_total]);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(warnings.is_empty(), "{warnings:?}");
        match &errors[0] {
            SpecError::CrossLibraryDepNotFound { dep, .. } => {
                assert_eq!(dep, "shared::money/round")
            }
            other => panic!("expected CrossLibraryDepNotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_validate_semantic_valid_spec() {
        let spec = create_test_spec("pricing/apply_discount", "{ subtotal - subtotal * rate }");
        let result = validate_semantic(&spec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_semantic_allows_additive_function_typescript_body() {
        let mut spec = create_test_spec("pricing/apply_tax", "{ subtotal + subtotal * rate }");
        spec.spec.body.typescript = Some("return subtotal + subtotal * rate;".to_string());

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_data_semantic_valid_spec() {
        let spec = create_data_spec("pricing/checkout_quote");
        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_data_semantic_valid_spec_with_empty_placeholder_body() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.body = Body::default();

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_sum_semantic_valid_spec() {
        let spec = create_sum_spec("pricing/checkout_status");
        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_sum_semantic_rejects_projected_variant_name_collision() {
        let mut spec = create_sum_spec("pricing/checkout_status");
        spec.spec.extensions.sum.as_mut().unwrap().variants.insert(
            "quoted__total".to_string(),
            AuthoredSumVariant {
                fields: IndexMap::new(),
            },
        );

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("projects to Rust variant name 'QuotedTotal'"),
            "{err}"
        );
        assert!(err.contains("quoted_total"), "{err}");
        assert!(err.contains("quoted__total"), "{err}");
    }

    #[test]
    fn test_validate_sum_semantic_rejects_projected_invalid_rust_identifier() {
        let mut spec = create_sum_spec("pricing/checkout_status");
        let sum = spec.spec.extensions.sum.as_mut().unwrap();
        let variant = sum.variants.shift_remove("pending").unwrap();
        sum.variants.insert("self_".to_string(), variant);

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("sum.variants[1].id"), "{err}");
        assert!(err.contains("'self_'"), "{err}");
        assert!(err.contains("'Self'"), "{err}");
    }

    #[test]
    fn test_validate_sum_semantic_allows_projected_pascal_case_keyword_like_name() {
        let mut spec = create_sum_spec("pricing/checkout_status");
        let sum = spec.spec.extensions.sum.as_mut().unwrap();
        let variant = sum.variants.shift_remove("pending").unwrap();
        sum.variants.insert("super_".to_string(), variant);

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_sum_semantic_rejects_variant_name_collision_with_enum_name() {
        let mut spec = create_sum_spec("pricing/checkout_status");
        let sum = spec.spec.extensions.sum.as_mut().unwrap();
        let variant = sum.variants.shift_remove("pending").unwrap();
        sum.variants.insert("checkout_status".to_string(), variant);

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("conflicts with the emitted enum name 'CheckoutStatus'"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_sum_semantic_rejects_top_level_typescript_body() {
        let mut spec = create_sum_spec("pricing/checkout_status");
        spec.spec.body.typescript = Some("return \"pending\";".to_string());

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("kind:sum must not declare top-level body.typescript"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_missing_constructor_behavior() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.constructors.clear();

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("kind:data requires at least one constructor"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_missing_method_behavior() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods.clear();

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("kind:data requires at least one method"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_allows_identical_cross_method_dep_reuse() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec
            .extensions
            .methods
            .push(spec.spec.extensions.methods[0].clone());
        spec.spec.extensions.methods[1].id = "tax_preview".to_string();

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_invalid_rust_backend_derive() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec
            .extensions
            .backends
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .derives = vec!["not valid rust".to_string()];

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("backends.rust.derives[0] must be a valid Rust path"),
            "{err}"
        );
        assert!(err.contains("not valid rust"), "{err}");
    }

    #[test]
    fn test_validate_data_semantic_accepts_multi_segment_rust_backend_derive() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec
            .extensions
            .backends
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .derives
            .push("serde::Serialize".to_string());

        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_duplicate_constructor_ids() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec
            .extensions
            .constructors
            .push(spec.spec.extensions.constructors[0].clone());

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("duplicate constructor id 'new'"), "{err}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_duplicate_method_ids() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec
            .extensions
            .methods
            .push(spec.spec.extensions.methods[0].clone());

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("duplicate method id 'total'"), "{err}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_constructor_method_id_collision() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods[0].id = "new".to_string();

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("constructors[0].id 'new' conflicts with methods[0].id 'new'"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_unsupported_receiver_mode() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods[0].receiver = "shared_mut".to_string();

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(err.contains("unsupported mode 'shared_mut'"), "{err}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_missing_required_constructor_fields() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.constructors[0]
            .initializes
            .shift_remove("tax_rate");

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("omits required field initialization for"),
            "{err}"
        );
        assert!(err.contains("tax_rate"), "{err}");
    }

    #[test]
    fn test_validate_data_semantic_rejects_shared_semantic_escape_hatch() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.body.rust = "{ unreachable!(\"escape hatch\") }".to_string();

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("kind:data must leave top-level body.rust empty"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_top_level_typescript_body() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.body.typescript = Some("return unreachable();".to_string());

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("kind:data must not declare top-level body.typescript"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_constructor_return_override() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.constructors[0]
            .contract
            .as_mut()
            .unwrap()
            .returns = Some("CheckoutQuote".to_string());

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("constructors[0].contract.returns is not allowed"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_missing_method_contract() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods[0].contract = None;

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("methods[0].contract is required for kind:data"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_invalid_method_dep() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods[0].deps = vec!["not a dep".to_string()];

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("methods[0].deps contains invalid dep"),
            "{err}"
        );
    }

    #[test]
    fn test_validate_data_semantic_rejects_cross_method_dep_callable_collision() {
        let mut spec = create_data_spec("pricing/checkout_quote");
        spec.spec.extensions.methods[0].deps = vec!["demo/foo".to_string()];
        spec.spec
            .extensions
            .methods
            .push(spec.spec.extensions.methods[0].clone());
        spec.spec.extensions.methods[1].id = "discount_preview".to_string();
        spec.spec.extensions.methods[1].deps = vec!["util/foo".to_string()];

        let err = validate_semantic(&spec).unwrap_err();
        match err {
            SpecError::DepCollision {
                dep1,
                dep2,
                fn_name,
                path,
            } => {
                assert_eq!(dep1, "demo/foo");
                assert_eq!(dep2, "util/foo");
                assert_eq!(fn_name, "foo");
                assert_eq!(path, "test/pricing/checkout_quote.unit.spec");
            }
            other => panic!("expected DepCollision, got {other:?}"),
        }
    }

    #[test]
    fn test_validate_deps_exist_reports_missing_data_method_dep() {
        let spec = create_data_spec("pricing/checkout_quote");

        let (errors, warnings) = validate_deps_exist(&[spec]);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(warnings.is_empty(), "{warnings:?}");
        match &errors[0] {
            SpecError::MissingDep { dep, .. } => assert_eq!(dep, "pricing/apply_tax"),
            other => panic!("expected MissingDep, got {other:?}"),
        }
    }

    #[test]
    fn test_detect_cycles_reports_data_method_cycle() {
        let mut alpha = create_data_spec("pricing/alpha");
        alpha.spec.extensions.methods[0].deps = vec!["pricing/beta".to_string()];

        let mut beta = create_data_spec("pricing/beta");
        beta.spec.extensions.methods[0].deps = vec!["pricing/alpha".to_string()];

        let errors = detect_cycles(&[alpha, beta]);
        assert_eq!(errors.len(), 1, "{errors:?}");
        match &errors[0] {
            SpecError::CyclicDep { cycle_path, .. } => {
                assert_eq!(
                    cycle_path,
                    &["pricing/alpha", "pricing/beta", "pricing/alpha"]
                );
            }
            other => panic!("expected CyclicDep, got {other:?}"),
        }
    }

    #[test]
    fn test_validate_full() {
        let spec = create_test_spec("utils/round", "{ x.floor() }");
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
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
    fn validate_body_fn_declaration_emits_migration_error() {
        let spec = create_test_spec("pricing/apply_discount", "pub fn apply_discount() {}");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("looks like a full function declaration"),
            "{err}"
        );
    }

    #[test]
    fn validate_body_fn_declaration_with_args_emits_migration_error() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            "pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal { subtotal }",
        );
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("looks like a full function declaration"),
            "{err}"
        );
    }

    #[test]
    fn validate_body_rust_block_valid() {
        let spec = create_test_spec("pricing/apply_discount", "{ subtotal - subtotal * rate }");
        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn validate_body_rust_invalid_block() {
        let spec = create_test_spec("pricing/apply_discount", "not valid rust at all !!!");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("body.rust must be a Rust block expression"),
            "{err}"
        );
    }

    #[test]
    fn validate_body_with_macros_in_block_passes() {
        let spec = create_test_spec(
            "pricing/apply_discount",
            r#"{
    let _v = vec![1, 2, 3];
    assert!(true);
    todo!()
}"#,
        );
        let result = validate_semantic(&spec);
        assert!(result.is_ok(), "{result:?}");
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
                    rust: "{ }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "injection_attempt".to_string(),
                    expect: "true); } } mod evil { fn steal() {}".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "apply_discount() == true".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "block_allowed".to_string(),
                    expect: "{ let ok = apply_discount(); ok }".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
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
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "block_attempt".to_string(),
                    expect: "{ std::process::exit(1); true }".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_call_arg".to_string(),
                    expect: "f(unsafe { true })".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "block_in_binary_operand".to_string(),
                    expect: "true && { false }".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_method_arg".to_string(),
                    expect: "foo.bar(unsafe { true })".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_field_base".to_string(),
                    expect: "(unsafe { foo }).field".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_index".to_string(),
                    expect: "arr[unsafe { 0 }]".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_unary".to_string(),
                    expect: "!(unsafe { true })".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "unsafe_in_cast".to_string(),
                    expect: "(unsafe { 0 }) as u64".to_string(),
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
            },
        };

        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("block, unsafe, closure"),
            "expected unsafe block in cast to be rejected: {err}"
        );
    }

    // --- contract.inputs type validation ---

    fn make_spec_with_contract(
        inputs: Option<indexmap::IndexMap<String, String>>,
        returns: Option<&str>,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "test/pricing/apply_tax.unit.spec".to_string(),
                id: "pricing/apply_tax".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_tax".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply tax.".to_string(),
                },
                contract: Some(Contract {
                    inputs,
                    returns: returns.map(String::from),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "{ () }".to_string(),
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    #[test]
    fn contract_type_validation_passes_for_valid_types() {
        let mut inputs = indexmap::IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        inputs.insert("rate".to_string(), "Decimal".to_string());
        let spec = make_spec_with_contract(Some(inputs), Some("Decimal"));
        assert!(validate_semantic(&spec).is_ok());
    }

    #[test]
    fn contract_type_validation_passes_with_no_contract() {
        let spec = create_test_spec("money/round", "{ () }");
        assert!(validate_semantic(&spec).is_ok());
    }

    #[test]
    fn contract_type_validation_rejects_invalid_input_type() {
        let mut inputs = indexmap::IndexMap::new();
        inputs.insert("amount".to_string(), "Strinng".to_string());
        // "Strinng" is a valid identifier so syn parses it fine as a Type::Path.
        // Use something syntactically invalid to verify the error path:
        inputs.insert("rate".to_string(), "Vec<".to_string());
        let spec = make_spec_with_contract(Some(inputs), Some("Decimal"));
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("contract.inputs.rate") && err.contains("invalid Rust type"),
            "expected ContractTypeInvalid for inputs.rate: {err}"
        );
    }

    #[test]
    fn contract_type_validation_rejects_invalid_return_type() {
        let mut inputs = indexmap::IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        let spec = make_spec_with_contract(Some(inputs), Some("Vec<"));
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("contract.returns") && err.contains("invalid Rust type"),
            "expected ContractTypeInvalid for returns: {err}"
        );
    }

    #[test]
    fn contract_type_validation_rejects_keyword_input_name() {
        let mut inputs = indexmap::IndexMap::new();
        inputs.insert("type".to_string(), "Decimal".to_string());
        let spec = make_spec_with_contract(Some(inputs), None);
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("'type'") && err.contains("not a valid Rust identifier"),
            "expected ContractInputNameInvalid for keyword key: {err}"
        );
    }

    #[test]
    fn contract_type_validation_rejects_hyphenated_input_name() {
        let mut inputs = indexmap::IndexMap::new();
        inputs.insert("bad-name".to_string(), "Decimal".to_string());
        let spec = make_spec_with_contract(Some(inputs), None);
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("'bad-name'") && err.contains("snake_case"),
            "expected ContractInputNameInvalid for hyphenated key: {err}"
        );
    }

    // --- cycle detection ---

    #[test]
    fn test_detect_cycles_no_cycle() {
        // A → B → C (no cycle)
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["b/bar".to_string()];
        let mut b = create_test_spec("b/bar", "{ }");
        b.spec.deps = vec!["c/baz".to_string()];
        let c = create_test_spec("c/baz", "{ }");
        let errors = detect_cycles(&[a, b, c]);
        assert!(errors.is_empty(), "expected no cycle errors: {:?}", errors);
    }

    #[test]
    fn test_detect_cycles_simple_cycle() {
        // A → B → A
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["b/bar".to_string()];
        let mut b = create_test_spec("b/bar", "{ }");
        b.spec.deps = vec!["a/foo".to_string()];
        let errors = detect_cycles(&[a, b]);
        assert_eq!(
            errors.len(),
            1,
            "expected exactly one cycle error: {:?}",
            errors
        );
        match &errors[0] {
            SpecError::CyclicDep { cycle_path, .. } => {
                assert_eq!(cycle_path, &["a/foo", "b/bar", "a/foo"]);
            }
            other => panic!("expected CyclicDep, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_cycles_self_loop() {
        // A → A
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["a/foo".to_string()];
        let errors = detect_cycles(&[a]);
        assert_eq!(
            errors.len(),
            1,
            "expected exactly one cycle error: {:?}",
            errors
        );
        match &errors[0] {
            SpecError::CyclicDep { cycle_path, .. } => {
                assert_eq!(cycle_path, &["a/foo", "a/foo"]);
            }
            other => panic!("expected CyclicDep, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_cycles_longer_cycle() {
        // A → B → C → A
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["b/bar".to_string()];
        let mut b = create_test_spec("b/bar", "{ }");
        b.spec.deps = vec!["c/baz".to_string()];
        let mut c = create_test_spec("c/baz", "{ }");
        c.spec.deps = vec!["a/foo".to_string()];
        let errors = detect_cycles(&[a, b, c]);
        assert_eq!(
            errors.len(),
            1,
            "expected exactly one cycle error: {:?}",
            errors
        );
        match &errors[0] {
            SpecError::CyclicDep { cycle_path, .. } => {
                assert_eq!(cycle_path, &["a/foo", "b/bar", "c/baz", "a/foo"]);
            }
            other => panic!("expected CyclicDep, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_cycles_missing_dep_skipped() {
        // A deps B but B is not in the set — no cycle, just a missing dep
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["b/bar".to_string()];
        let errors = detect_cycles(&[a]);
        assert!(
            errors.is_empty(),
            "expected no cycle errors for missing dep: {:?}",
            errors
        );
    }

    #[test]
    fn test_detect_cycles_multiple_cycles() {
        // (A → B → A) and (C → D → C)
        let mut a = create_test_spec("a/foo", "{ }");
        a.spec.deps = vec!["b/bar".to_string()];
        let mut b = create_test_spec("b/bar", "{ }");
        b.spec.deps = vec!["a/foo".to_string()];
        let mut c = create_test_spec("c/baz", "{ }");
        c.spec.deps = vec!["d/qux".to_string()];
        let mut d = create_test_spec("d/qux", "{ }");
        d.spec.deps = vec!["c/baz".to_string()];
        let errors = detect_cycles(&[a, b, c, d]);
        assert_eq!(errors.len(), 2, "expected two cycle errors: {:?}", errors);
        for err in &errors {
            assert!(
                matches!(err, SpecError::CyclicDep { .. }),
                "expected CyclicDep, got {:?}",
                err
            );
        }
    }

    #[test]
    fn test_detect_cycles_error_message_format() {
        let mut a = create_test_spec("money/round", "{ }");
        a.spec.deps = vec!["currency/convert".to_string()];
        let mut b = create_test_spec("currency/convert", "{ }");
        b.spec.deps = vec!["money/round".to_string()];
        let errors = detect_cycles(&[a, b]);
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].to_string().contains("cycle detected"),
            "{}",
            errors[0]
        );
        assert!(
            errors[0].to_string().contains("money/round"),
            "{}",
            errors[0]
        );
        assert!(
            errors[0].to_string().contains("currency/convert"),
            "{}",
            errors[0]
        );
    }

    #[test]
    fn test_detect_qualified_cycles_supports_cross_library_shapes() {
        let local = create_test_spec("pricing/calculate_total", "{ }");
        let shared = create_test_spec("money/round", "{ }");
        let scoped_specs = vec![
            QualifiedLoadedSpec {
                loaded: &local,
                qualified_id: QualifiedUnitRef::new(
                    Some("root".to_string()),
                    "pricing/calculate_total",
                ),
                qualified_deps: vec![QualifiedUnitRef::new(
                    Some("shared".to_string()),
                    "money/round",
                )],
            },
            QualifiedLoadedSpec {
                loaded: &shared,
                qualified_id: QualifiedUnitRef::new(Some("shared".to_string()), "money/round"),
                qualified_deps: vec![QualifiedUnitRef::new(
                    Some("root".to_string()),
                    "pricing/calculate_total",
                )],
            },
        ];

        let errors = detect_qualified_cycles(&scoped_specs);
        assert_eq!(errors.len(), 1, "{errors:?}");
        match &errors[0] {
            SpecError::CrossLibraryCycle { cycle_path, .. } => {
                assert_eq!(
                    cycle_path,
                    &[
                        "root::pricing/calculate_total".to_string(),
                        "shared::money/round".to_string(),
                        "root::pricing/calculate_total".to_string(),
                    ]
                );
            }
            other => panic!("expected CrossLibraryCycle, got {other:?}"),
        }
    }

    #[test]
    fn test_detect_qualified_cycles_classifies_root_and_imported_cycle_as_cross_library() {
        let local = create_test_spec("pricing/apply_discount", "{ }");
        let shared = create_test_spec("money/round", "{ }");
        let scoped_specs = vec![
            QualifiedLoadedSpec {
                loaded: &local,
                qualified_id: QualifiedUnitRef::local("pricing/apply_discount"),
                qualified_deps: vec![QualifiedUnitRef::new(
                    Some("shared".to_string()),
                    "money/round",
                )],
            },
            QualifiedLoadedSpec {
                loaded: &shared,
                qualified_id: QualifiedUnitRef::new(Some("shared".to_string()), "money/round"),
                qualified_deps: vec![QualifiedUnitRef::local("pricing/apply_discount")],
            },
        ];

        let errors = detect_qualified_cycles(&scoped_specs);
        assert_eq!(errors.len(), 1, "{errors:?}");
        match &errors[0] {
            SpecError::CrossLibraryCycle { cycle_path, .. } => {
                assert_eq!(
                    cycle_path,
                    &[
                        "pricing/apply_discount".to_string(),
                        "shared::money/round".to_string(),
                        "pricing/apply_discount".to_string(),
                    ]
                );
            }
            other => panic!("expected CrossLibraryCycle, got {other:?}"),
        }
    }

    // --- check_spec_versions ---

    #[test]
    fn check_spec_versions_warns_on_missing() {
        let spec = create_test_spec("pricing/apply_discount", "{ }");
        assert!(spec.spec.spec_version.is_none());
        let warnings = check_spec_versions(&[spec]);
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].to_string().contains("spec_version not set"),
            "{}",
            warnings[0]
        );
        assert!(
            warnings[0]
                .to_string()
                .contains(&format!("spec_version: \"{AUTHORED_SPEC_VERSION}\"")),
            "{}",
            warnings[0]
        );
    }

    #[test]
    fn check_spec_versions_no_warning_when_set() {
        let mut spec = create_test_spec("pricing/apply_discount", "{ }");
        spec.spec.spec_version = Some("0.3.0".to_string());
        let warnings = check_spec_versions(&[spec]);
        assert!(warnings.is_empty(), "expected no warnings: {:?}", warnings);
    }

    #[test]
    fn check_spec_versions_partial_warning() {
        let spec_with = {
            let mut s = create_test_spec("pricing/apply_discount", "{ }");
            s.spec.spec_version = Some("0.3.0".to_string());
            s
        };
        let spec_without = create_test_spec("money/round", "{ }");
        let warnings = check_spec_versions(&[spec_with, spec_without]);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].to_string().contains("money/round"));
    }

    #[test]
    fn spec_version_round_trips_through_serde() {
        let yaml = r#"
id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#;
        let spec: crate::types::SpecStruct = serde_yaml_bw::from_str(yaml).unwrap();
        assert_eq!(spec.spec_version, Some("0.3.0".to_string()));
    }

    #[test]
    fn spec_version_absent_round_trips_as_none() {
        let yaml = r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#;
        let spec: crate::types::SpecStruct = serde_yaml_bw::from_str(yaml).unwrap();
        assert!(spec.spec_version.is_none());
    }

    #[test]
    fn expect_deeply_nested_parens_are_rejected_at_depth_cap() {
        let nested = format!("{}true{}", "(".repeat(200), ")".repeat(200));
        let err = validate_semantic(&LoadedSpec {
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
                    rust: "{ true }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "deep".to_string(),
                    expect: nested,
                }],
                links: None,
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
            },
        })
        .unwrap_err();

        assert!(err.to_string().contains("maximum depth of 128"));
    }

    #[test]
    fn humanize_validation_error_required_field() {
        let yaml = r#"id: pricing/apply_tax
kind: function
body:
  rust: "{ 42 }""#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();
        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("missing required field"), "got: {err}");
        assert!(err.contains("intent"), "got: {err}");
    }

    #[test]
    fn humanize_validation_error_unknown_field() {
        let yaml = r#"id: pricing/apply_tax
kind: function
intent:
  why: test
body:
  rust: "{ 42 }"
extra_field: bad"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();
        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("unknown field"), "got: {err}");
        assert!(err.contains("extra_field"), "got: {err}");
    }

    #[test]
    fn humanize_validation_error_id_pattern() {
        let yaml = r#"id: BAD_FORMAT
kind: function
intent:
  why: test
body:
  rust: "{ 42 }""#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();
        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("invalid id format"), "got: {err}");
        assert!(err.contains("module/name"), "got: {err}");
    }

    #[test]
    fn humanize_validation_error_nested_id_pattern() {
        let yaml = r#"id: pricing/checkout_quote
kind: data
intent:
  why: test
data:
  fields:
    subtotal:
      type: i32
constructors:
  - id: invalid-id
    intent:
      why: build
    contract:
      inputs:
        subtotal: i32
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: total
    receiver: shared_ref
    contract:
      returns: i32
    lowering:
      rust:
        body: |
          {
              self.subtotal
          }"#;
        let value: YamlValue = serde_yaml_bw::from_str(yaml).unwrap();
        let err = validate_raw_yaml(&value, "test.unit.spec")
            .unwrap_err()
            .to_string();
        assert!(err.contains("invalid id format"), "got: {err}");
        assert!(err.contains("snake_case identifier"), "got: {err}");
        assert!(!err.contains("module/name"), "got: {err}");
    }

    // ── reserved unit name ───────────────────────────────────────────────────

    #[test]
    fn reserved_unit_name_molecule_tests_is_rejected() {
        let spec = create_test_spec("pricing/molecule_tests", "{ }");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
        assert!(
            err.contains("molecule_tests"),
            "expected segment name in error, got: {err}"
        );
    }

    #[test]
    fn reserved_unit_name_nested_molecule_tests_is_rejected() {
        let spec = create_test_spec("pricing/sub/molecule_tests", "{ }");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
    }

    #[test]
    fn reserved_unit_name_namespace_molecule_tests_is_rejected() {
        let spec = create_test_spec("qa/molecule_tests/foo", "{ }");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
        assert!(
            err.contains("molecule_tests"),
            "expected segment name in error, got: {err}"
        );
    }

    #[test]
    fn reserved_unit_name_deep_namespace_molecule_tests_is_rejected() {
        let spec = create_test_spec("qa/sub/molecule_tests/foo", "{ }");
        let err = validate_semantic(&spec).unwrap_err().to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
    }

    #[test]
    fn non_reserved_similar_unit_name_passes() {
        let spec = create_test_spec("qa/molecule_test_helpers/foo", "{ }");
        assert!(validate_semantic(&spec).is_ok());
    }

    #[test]
    fn non_reserved_unit_name_passes() {
        let spec = create_test_spec("pricing/apply_discount", "{ }");
        assert!(validate_semantic(&spec).is_ok());
    }

    #[test]
    fn reserved_molecule_test_name_molecule_tests_is_rejected() {
        let test = create_molecule_test_spec("pricing/molecule_tests", vec!["money/round"]);
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
        assert!(
            err.contains("molecule_tests"),
            "expected segment name in error, got: {err}"
        );
    }

    #[test]
    fn reserved_molecule_test_name_namespace_molecule_tests_is_rejected() {
        let test = create_molecule_test_spec("qa/molecule_tests/foo", vec!["money/round"]);
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
        assert!(
            err.contains("molecule_tests"),
            "expected segment name in error, got: {err}"
        );
    }

    #[test]
    fn reserved_molecule_test_name_deep_namespace_molecule_tests_is_rejected() {
        let test = create_molecule_test_spec("qa/sub/molecule_tests/foo", vec!["money/round"]);
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("reserved"),
            "expected 'reserved' in error, got: {err}"
        );
    }

    #[test]
    fn non_reserved_similar_molecule_test_name_passes() {
        let test = create_molecule_test_spec("qa/molecule_test_helpers/foo", vec!["money/round"]);
        assert!(validate_molecule_test_semantic(&test).is_ok());
    }

    // ── molecule body unsafe detection ───────────────────────────────────────

    fn make_molecule_test(id: &str, body: &str) -> crate::types::LoadedMoleculeTest {
        use crate::types::{
            Body, Intent, LoadedMoleculeTest, MoleculeTestSource, MoleculeTestStruct,
        };
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: format!("test/{}.test.spec", id),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: "test".to_string(),
                },
                covers: vec![],
                imports: None,
                body: Body {
                    rust: body.to_string(),
                    typescript: None,
                },
                spec_version: None,
            },
        }
    }

    #[test]
    fn molecule_body_with_unsafe_block_is_rejected() {
        let test = make_molecule_test("pricing/checkout_flow", "{ unsafe { let x = 1; } }");
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("unsafe"),
            "expected 'unsafe' in error, got: {err}"
        );
    }

    #[test]
    fn molecule_body_with_nested_unsafe_is_rejected() {
        let test = make_molecule_test(
            "pricing/checkout_flow",
            "{ let x = if true { unsafe { 1 } else { 2 } }; }",
        );
        // The body may or may not parse depending on syn's handling of incomplete if/else;
        // either a parse error or an unsafe error is acceptable rejection.
        let result = validate_molecule_test_semantic(&test);
        assert!(result.is_err(), "expected rejection of nested unsafe");
    }

    #[test]
    fn molecule_body_with_unsafe_in_array_index_expr_is_rejected() {
        let test = make_molecule_test(
            "pricing/checkout_flow",
            "{ let _x = [unsafe { std::mem::zeroed::<u8>() }][0]; }",
        );
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("unsafe"),
            "expected 'unsafe' in error, got: {err}"
        );
    }

    #[test]
    fn molecule_body_with_typescript_is_rejected() {
        let mut test = make_molecule_test("pricing/checkout_flow", "{ true }");
        test.test.body.typescript = Some("return true;".to_string());

        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("body.typescript is not supported in .test.spec"),
            "{err}"
        );
    }

    #[test]
    fn typescript_target_accepts_monotone_up_leaf_with_bounded_expect() {
        let spec = create_typescript_lane_function_spec();
        validate_typescript_execution_target_spec(&spec)
            .expect("bounded monotone-up spec should be eligible");
    }

    #[test]
    fn typescript_target_rejects_helper_root() {
        let spec = create_typescript_helper_spec();
        let err = validate_typescript_execution_target_spec(&spec).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_HELPER_COMPATIBILITY_KEY),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_local_graph_target_accepts_monotone_down_root() {
        let spec = create_typescript_discount_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("same-tree monotone-down roots should be eligible in M59");
    }

    #[test]
    fn typescript_target_accepts_one_local_helper_dep_with_context() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["money/round".to_string()];
        spec.spec.body.rust =
            "{ let taxed = subtotal + subtotal * rate; round(taxed).max(Decimal::ZERO) }"
                .to_string();
        spec.spec.body.typescript =
            Some("return round(subtotal.add(subtotal.mul(rate))).max(Decimal.zero());".to_string());

        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("one local helper dep should be eligible in M46");
    }

    #[test]
    fn typescript_target_accepts_normalized_required_arg_wrapper_root() {
        let spec = create_typescript_normalized_required_arg_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("normalized required-arg wrapper roots should be eligible in M61");
    }

    #[test]
    fn typescript_target_rejects_unsupported_multi_dep_root() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps.push("pricing/apply_tax_bonus".to_string());
        let wrapper = create_typescript_wrapper_spec();
        let tax = create_typescript_lane_function_spec();
        let discount = create_typescript_discount_spec();
        let helper = create_typescript_helper_spec();
        let mut bonus = create_typescript_lane_function_spec();
        bonus.spec.id = "pricing/apply_tax_bonus".to_string();
        bonus.source.id = "pricing/apply_tax_bonus".to_string();
        bonus.source.file_path = "units/pricing/apply_tax_bonus.unit.spec".to_string();
        bonus.spec.local_tests[0].expect =
            "apply_tax_bonus(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(1070, 2)"
                .to_string();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (tax.spec.id.clone(), tax),
            (discount.spec.id.clone(), discount),
            (helper.spec.id.clone(), helper),
            (bonus.spec.id.clone(), bonus),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string().contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_accepts_cross_library_helper_dep_with_loaded_helper() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["shared::money/round".to_string()];
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::money/round".to_string(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("cross-library helper dep should be eligible in M55 when the helper is loaded");
    }

    #[test]
    fn typescript_target_rejects_missing_shared_helper_dep() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["shared::money/round".to_string()];

        let err = validate_typescript_execution_target_spec(&spec).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_MISSING_HELPER_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_missing_local_helper_dep() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["money/round".to_string()];

        let err = validate_typescript_execution_target_spec(&spec).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_MISSING_HELPER_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_wrong_helper_family() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["pricing/apply_discount".to_string()];
        spec.spec.body.rust =
            "{ let taxed = subtotal + subtotal * rate; apply_discount(taxed, rate) }".to_string();
        spec.spec.body.typescript =
            Some("return apply_discount(subtotal.add(subtotal.mul(rate)), rate);".to_string());

        let mut wrong_helper = create_typescript_lane_function_spec();
        wrong_helper.spec.id = "pricing/apply_discount".to_string();
        wrong_helper.source.id = "pricing/apply_discount".to_string();
        wrong_helper.source.file_path = "units/pricing/apply_discount.unit.spec".to_string();
        wrong_helper.spec.intent.why =
            "Return the subtotal after applying the discount rate and clamping at zero."
                .to_string();
        wrong_helper.spec.contract.as_mut().unwrap().invariants =
            vec!["output <= subtotal".to_string(), "output >= 0".to_string()];
        wrong_helper.spec.body.rust =
            "{ (subtotal - subtotal * rate).max(Decimal::ZERO) }".to_string();
        wrong_helper.spec.body.typescript =
            Some("return subtotal.sub(subtotal.mul(rate)).max(Decimal.zero());".to_string());
        wrong_helper.spec.local_tests[0].expect =
            "apply_discount(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(930, 2)"
                .to_string();

        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrong_helper.spec.id.clone(), wrong_helper),
        ]);

        let err = validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains(TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
        assert!(
            err.contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_shared_helper_missing_typescript_body() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.deps = vec!["shared::money/round".to_string()];
        let mut helper = create_typescript_helper_spec();
        helper.spec.body.typescript = None;

        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::money/round".to_string(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_HELPER_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_recursive_closure_accepts_reachable_shared_dep() {
        let spec = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let mut tax = create_typescript_lane_function_spec();
        tax.spec.deps = vec!["shared::money/round".to_string()];
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper.clone()),
            ("shared::money/round".to_string(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("M61 should allow recursive closure to cross into shared deps");
    }

    #[test]
    fn typescript_local_graph_target_rejects_reachable_unsupported_semantic_member() {
        let spec = create_typescript_wrapper_spec();
        let mut discount = create_typescript_discount_spec();
        discount.spec.body.rust = "{ let discounted = subtotal - subtotal * rate; if discounted < Decimal::ZERO { Decimal::ZERO } else { round(discounted) } }".to_string();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        let err = err.to_string();
        assert!(
            err.contains(TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
        assert!(
            err.contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_local_graph_target_rejects_reachable_missing_typescript_body() {
        let spec = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let mut tax = create_typescript_lane_function_spec();
        tax.spec.body.typescript = None;
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_WRAPPER_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_accepts_wrapper_root_with_exact_local_dep_tuple() {
        let spec = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("wrapper root should be eligible in M52");
    }

    #[test]
    fn typescript_wrapper_direct_cross_library_deps_validate() {
        let mut spec = create_typescript_wrapper_spec();
        spec.spec.deps = vec![
            "shared::pricing/apply_discount".to_string(),
            "shared::pricing/apply_tax".to_string(),
        ];
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::pricing/apply_discount".to_string(), discount),
            ("shared::pricing/apply_tax".to_string(), tax),
            ("shared::money/round".to_string(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("direct cross-library wrapper deps should be eligible in M56");
    }

    #[test]
    fn typescript_wrapper_mixed_local_and_shared_deps_validate() {
        let mut spec = create_typescript_wrapper_spec();
        spec.spec.deps[1] = "shared::pricing/apply_tax".to_string();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (helper.spec.id.clone(), helper.clone()),
            ("shared::pricing/apply_tax".to_string(), tax),
            ("shared::money/round".to_string(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("mixed local/shared wrapper deps should be eligible in M56");
    }

    #[test]
    fn typescript_wrapper_shared_dep_missing_typescript_body_rejects() {
        let mut spec = create_typescript_wrapper_spec();
        spec.spec.deps[0] = "shared::pricing/apply_discount".to_string();
        let mut discount = create_typescript_discount_spec();
        discount.spec.body.typescript = None;
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::pricing/apply_discount".to_string(), discount),
            (tax.spec.id.clone(), tax),
            ("shared::money/round".to_string(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_WRAPPER_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_wrapper_missing_local_dep() {
        let spec = create_typescript_wrapper_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string().contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_wrapper_with_reordered_direct_deps() {
        let mut spec = create_typescript_wrapper_spec();
        spec.spec.deps = vec![
            "pricing/apply_tax".to_string(),
            "pricing/apply_discount".to_string(),
        ];
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_WRAPPER_DEP_FAMILY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_accepts_chain3_root_with_exact_local_dep_tuple() {
        let spec = create_typescript_chain3_spec();
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("chain3 root should be eligible in M54");
    }

    #[test]
    fn typescript_chain3_direct_cross_library_deps_validate() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps = vec![
            "shared::pricing/calculate_total".to_string(),
            "shared::pricing/apply_tax".to_string(),
            "shared::pricing/apply_discount".to_string(),
        ];
        let mut wrapper = create_typescript_wrapper_spec();
        wrapper.spec.deps = vec![
            "shared::pricing/apply_discount".to_string(),
            "shared::pricing/apply_tax".to_string(),
        ];
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (
                "shared::pricing/calculate_total".to_string(),
                wrapper.clone(),
            ),
            (
                "shared::pricing/apply_discount".to_string(),
                discount.clone(),
            ),
            ("shared::pricing/apply_tax".to_string(), tax.clone()),
            ("shared::money/round".to_string(), helper.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("direct cross-library chain3 deps should be eligible in M56");
    }

    #[test]
    fn typescript_chain3_mixed_local_and_shared_deps_validate() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps = vec![
            "pricing/calculate_total".to_string(),
            "shared::pricing/apply_tax".to_string(),
            "shared::pricing/apply_discount".to_string(),
        ];
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (
                "shared::pricing/apply_discount".to_string(),
                discount.clone(),
            ),
            ("shared::pricing/apply_tax".to_string(), tax.clone()),
            ("shared::money/round".to_string(), helper.clone()),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("mixed local/shared chain3 deps should be eligible in M56");
    }

    #[test]
    fn typescript_target_rejects_chain3_with_reordered_declared_deps() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps = vec![
            "pricing/apply_tax".to_string(),
            "pricing/calculate_total".to_string(),
            "pricing/apply_discount".to_string(),
        ];
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_chain3_shared_dep_wrong_slot_family_rejects() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps = vec![
            "shared::pricing/apply_tax".to_string(),
            "shared::pricing/calculate_total".to_string(),
            "shared::pricing/apply_discount".to_string(),
        ];
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::pricing/calculate_total".to_string(), wrapper),
            ("shared::pricing/apply_discount".to_string(), discount),
            ("shared::pricing/apply_tax".to_string(), tax),
            ("shared::money/round".to_string(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string().contains("unsupported.function.v1")
                || err
                    .to_string()
                    .contains(TYPESCRIPT_CHAIN3_DEP_FAMILY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_chain3_missing_dep() {
        let spec = create_typescript_chain3_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let discount = create_typescript_discount_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
            (discount.spec.id.clone(), discount),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string().contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_local_graph_target_rejects_chain3_with_wrong_supported_local_family_mix() {
        let mut spec = create_typescript_chain3_spec();
        spec.spec.deps[1] = "pricing/apply_discount".to_string();
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        let err = err.to_string();
        assert!(
            err.contains(TYPESCRIPT_LOCAL_GRAPH_SEMANTIC_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
        assert!(
            err.contains("unsupported.function.v1"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_chain3_dep_missing_typescript_body() {
        let spec = create_typescript_chain3_spec();
        let mut wrapper = create_typescript_wrapper_spec();
        wrapper.spec.body.typescript = None;
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        let err =
            validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_CHAIN3_DEP_TYPESCRIPT_BODY_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_nested_chain3_closure_member_accepts_same_tree_chain3_member() {
        let spec = create_typescript_base_nested_chain3_spec();
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_closure_member_spec_with_specs(&spec, &specs_by_id)
            .expect("same-tree nested chain3 closure member should validate in M58");
    }

    #[test]
    fn typescript_nested_chain3_root_accepts_same_tree_recursive_first_dep() {
        let spec = create_typescript_checkout_nested_chain3_spec();
        let nested = create_typescript_base_nested_chain3_spec();
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            (nested.spec.id.clone(), nested),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("same-tree nested chain3 root should be eligible in M58");
    }

    #[test]
    fn typescript_nested_chain3_root_accepts_cross_library_recursive_first_dep() {
        let mut spec = create_typescript_checkout_nested_chain3_spec();
        spec.spec.deps[0] = "shared::pricing/base_nested_chain3".to_string();
        let nested = create_typescript_base_nested_chain3_spec();
        let wrapper = create_typescript_wrapper_spec();
        let discount = create_typescript_discount_spec();
        let tax = create_typescript_lane_function_spec();
        let helper = create_typescript_helper_spec();
        let specs_by_id = HashMap::from([
            (spec.spec.id.clone(), spec.clone()),
            ("shared::pricing/base_nested_chain3".to_string(), nested),
            (
                "shared::pricing/calculate_total".to_string(),
                wrapper.clone(),
            ),
            (
                "shared::pricing/apply_discount".to_string(),
                discount.clone(),
            ),
            ("shared::pricing/apply_tax".to_string(), tax.clone()),
            ("shared::money/round".to_string(), helper.clone()),
            (wrapper.spec.id.clone(), wrapper),
            (discount.spec.id.clone(), discount),
            (tax.spec.id.clone(), tax),
            (helper.spec.id.clone(), helper),
        ]);

        validate_typescript_execution_target_spec_with_specs(&spec, &specs_by_id)
            .expect("M61 should allow shared recursive chain3 slot-1 members");
    }

    #[test]
    fn typescript_closure_member_prefers_owner_library_for_same_id_units() {
        let mut local_tax = create_typescript_lane_function_spec();
        local_tax.spec.body.rust = "{ subtotal + subtotal * rate + Decimal::ONE }".to_string();
        local_tax.spec.body.typescript =
            Some("return subtotal.add(subtotal.mul(rate)).add(Decimal.one());".to_string());

        let mut shared_tax = create_typescript_lane_function_spec();
        shared_tax.source.file_path = "shared/units/pricing/apply_tax.unit.spec".to_string();
        let wrapper = create_typescript_wrapper_spec();
        let mut shared_wrapper = create_typescript_wrapper_spec();
        shared_wrapper.source.file_path =
            "shared/units/pricing/calculate_total.unit.spec".to_string();
        let discount = create_typescript_discount_spec();
        let mut shared_discount = create_typescript_discount_spec();
        shared_discount.source.file_path =
            "shared/units/pricing/apply_discount.unit.spec".to_string();
        let helper = create_typescript_helper_spec();
        let mut shared_helper = create_typescript_helper_spec();
        shared_helper.source.file_path = "shared/units/money/round.unit.spec".to_string();

        let specs_by_id = HashMap::from([
            (
                "shared::pricing/calculate_total".to_string(),
                shared_wrapper,
            ),
            ("shared::pricing/apply_tax".to_string(), shared_tax),
            (
                "shared::pricing/apply_discount".to_string(),
                shared_discount,
            ),
            ("shared::money/round".to_string(), shared_helper),
            (wrapper.spec.id.clone(), wrapper),
            (local_tax.spec.id.clone(), local_tax),
            (discount.spec.id.clone(), discount),
            (helper.spec.id.clone(), helper),
        ]);

        let shared_root = specs_by_id
            .get("shared::pricing/calculate_total")
            .expect("shared wrapper root must be present")
            .clone();

        validate_typescript_closure_member_spec_with_specs(&shared_root, &specs_by_id)
            .expect("shared closure members should resolve local deps inside their owner library");
    }

    #[test]
    fn typescript_target_rejects_expect_outside_bounded_ast() {
        let mut spec = create_typescript_lane_function_spec();
        spec.spec.local_tests[0].expect =
            "apply_tax(Decimal::new(1000, 2), Decimal::new(7, 2)) >= Decimal::new(1070, 2)"
                .to_string();

        let err = validate_typescript_execution_target_spec(&spec).unwrap_err();
        assert!(
            err.to_string()
                .contains(TYPESCRIPT_EXPECT_UNSUPPORTED_MESSAGE),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn typescript_target_rejects_molecule_specs_before_execution() {
        let test =
            create_molecule_test_spec("pricing/discount_plus_tax", vec!["pricing/apply_tax"]);
        let err = validate_typescript_molecule_target(&test).unwrap_err();
        assert!(
            err.to_string()
                .contains(".test.spec is not supported for --target-language typescript in M52"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn molecule_body_with_unsafe_fn_item_is_rejected() {
        let test = make_molecule_test(
            "pricing/checkout_flow",
            "{ unsafe fn helper() {} helper(); }",
        );
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("unsafe"),
            "expected 'unsafe' in error, got: {err}"
        );
    }

    #[test]
    fn molecule_body_with_unsafe_inside_macro_body_is_rejected() {
        let test = make_molecule_test("pricing/checkout_flow", "{ m!(unsafe { 1 }); }");
        let err = validate_molecule_test_semantic(&test)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("unsafe"),
            "expected 'unsafe' in error, got: {err}"
        );
    }

    #[test]
    fn molecule_body_with_unsafe_string_literal_passes() {
        let test = make_molecule_test(
            "pricing/checkout_flow",
            r#"{ let label = "unsafe"; assert_eq!(label, "unsafe"); }"#,
        );
        assert!(validate_molecule_test_semantic(&test).is_ok());
    }

    #[test]
    fn molecule_body_without_unsafe_passes() {
        let test = make_molecule_test(
            "pricing/checkout_flow",
            "{ let x = apply_discount(100, 10); assert_eq!(x, 90); }",
        );
        assert!(validate_molecule_test_semantic(&test).is_ok());
    }

    // ── validate_molecule_test_semantic: MoleculeBodyRustMustBeBlock ──────────

    #[test]
    fn molecule_body_bare_expression_is_rejected() {
        // A bare expression (no braces) is not a syn::Block and must be rejected.
        let test = make_molecule_test("pricing/checkout_flow", "true");
        let result = validate_molecule_test_semantic(&test);
        assert!(result.is_err(), "bare expression should be rejected");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("block") || err.contains("Block"),
            "expected block error, got: {err}"
        );
    }

    // ── validate_molecule_test_covers: MoleculeCoversNotFound ────────────────

    #[test]
    fn covers_unknown_unit_id_returns_covers_not_found_error() {
        let mut molecule_test = create_molecule_test_spec(
            "pricing/checkout_flow",
            vec!["pricing/apply_discount", "pricing/unknown_unit"],
        );
        molecule_test.test.imports = Some(vec![]);
        let unit_ids: HashSet<&str> = ["pricing/apply_discount"].into_iter().collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert!(warnings.is_empty());
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SpecError::MoleculeCoversNotFound {
                cover_id,
                test_id,
                test_path,
            } => {
                assert_eq!(cover_id, "pricing/unknown_unit");
                assert_eq!(test_id, "pricing/checkout_flow");
                assert!(test_path.ends_with("checkout_flow.test.spec"));
            }
            other => panic!("expected MoleculeCoversNotFound, got {other:?}"),
        }
    }

    #[test]
    fn covers_all_known_unit_ids_returns_no_errors() {
        let mut molecule_test = create_molecule_test_spec(
            "pricing/checkout_flow",
            vec!["pricing/apply_discount", "pricing/apply_tax"],
        );
        molecule_test.test.imports = Some(vec![]);
        let unit_ids: HashSet<&str> = ["pricing/apply_discount", "pricing/apply_tax"]
            .into_iter()
            .collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert!(errors.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn covers_empty_list_emits_warning_not_error() {
        let molecule_test = create_molecule_test_spec("pricing/checkout_flow", vec![]);
        let unit_ids: HashSet<&str> = ["pricing/apply_discount"].into_iter().collect();

        let (errors, warnings) = validate_molecule_test_covers(&molecule_test, &unit_ids);

        assert!(errors.is_empty(), "empty covers should not be an error");
        assert_eq!(
            warnings.len(),
            2,
            "expected empty-covers and implicit-import warnings"
        );
        assert!(
            warnings
                .iter()
                .any(|warning| matches!(warning, SpecWarning::MoleculeTestNoCoveredUnits { .. }))
        );
        assert!(warnings.iter().any(|warning| matches!(
            warning,
            SpecWarning::MoleculeImplicitImportsDeprecated { .. }
        )));
    }

    // ── validate_no_duplicate_molecule_test_ids ───────────────────────────────

    #[test]
    fn duplicate_molecule_test_ids_returns_error() {
        let test_a = create_molecule_test_spec("pricing/checkout_flow", vec!["money/round"]);
        let test_b = create_molecule_test_spec("pricing/checkout_flow", vec!["pricing/apply_tax"]);

        let errors = validate_no_duplicate_molecule_test_ids(&[test_a, test_b]);

        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SpecError::DuplicateMoleculeTestId { id, .. } => {
                assert_eq!(id, "pricing/checkout_flow");
            }
            other => panic!("expected DuplicateMoleculeTestId, got {other:?}"),
        }
    }

    #[test]
    fn unique_molecule_test_ids_returns_no_errors() {
        let test_a = create_molecule_test_spec("pricing/checkout_flow", vec!["money/round"]);
        let test_b = create_molecule_test_spec("pricing/discount_flow", vec!["money/round"]);

        let errors = validate_no_duplicate_molecule_test_ids(&[test_a, test_b]);

        assert!(errors.is_empty());
    }

    #[test]
    fn empty_molecule_test_slice_returns_no_errors() {
        let errors = validate_no_duplicate_molecule_test_ids(&[]);
        assert!(errors.is_empty());
    }

    // ── validate_raw_molecule_test_yaml ───────────────────────────────────────

    #[test]
    fn raw_molecule_test_yaml_valid_input_passes() {
        let yaml = r#"
id: pricing/checkout_flow
intent:
  why: "Verify discount + tax chain."
covers:
  - pricing/apply_discount
body:
  rust: |
    { assert!(true); }
"#;
        let value: serde_yaml_bw::Value = serde_yaml_bw::from_str(yaml).unwrap();
        let result = validate_raw_molecule_test_yaml(&value, "pricing/checkout_flow.test.spec");
        assert!(
            result.is_ok(),
            "valid molecule test YAML should pass: {result:?}"
        );
    }

    #[test]
    fn raw_molecule_test_yaml_valid_imports_pass() {
        let yaml = r#"
id: pricing/checkout_flow
intent:
  why: "Verify discount + tax chain."
imports:
  - rust_decimal::Decimal
  - crate::pricing::apply_discount::apply_discount
body:
  rust: |
    { assert!(true); }
"#;
        let value: serde_yaml_bw::Value = serde_yaml_bw::from_str(yaml).unwrap();
        let result = validate_raw_molecule_test_yaml(&value, "pricing/checkout_flow.test.spec");
        assert!(
            result.is_ok(),
            "valid molecule test imports should pass: {result:?}"
        );
    }

    #[test]
    fn raw_molecule_test_yaml_invalid_import_is_rejected() {
        let yaml = r#"
id: pricing/checkout_flow
intent:
  why: "Verify discount + tax chain."
imports:
  - Decimal
body:
  rust: |
    { assert!(true); }
"#;
        let value: serde_yaml_bw::Value = serde_yaml_bw::from_str(yaml).unwrap();
        let result = validate_raw_molecule_test_yaml(&value, "pricing/checkout_flow.test.spec");
        assert!(
            result.is_err(),
            "invalid molecule test import should fail schema validation"
        );
    }

    #[test]
    fn raw_molecule_test_yaml_missing_body_is_rejected() {
        let yaml = r#"
id: pricing/checkout_flow
intent:
  why: "Missing body."
"#;
        let value: serde_yaml_bw::Value = serde_yaml_bw::from_str(yaml).unwrap();
        let result = validate_raw_molecule_test_yaml(&value, "pricing/checkout_flow.test.spec");
        assert!(result.is_err(), "missing body field should be rejected");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Schema validation failed"),
            "expected schema error, got: {err}"
        );
    }

    #[test]
    fn raw_molecule_test_yaml_unknown_field_is_rejected() {
        let yaml = r#"
id: pricing/checkout_flow
intent:
  why: "Unknown field."
body:
  rust: "{ assert!(true); }"
unknown_field: should_fail
"#;
        let value: serde_yaml_bw::Value = serde_yaml_bw::from_str(yaml).unwrap();
        let result = validate_raw_molecule_test_yaml(&value, "pricing/checkout_flow.test.spec");
        assert!(result.is_err(), "unknown field should be rejected");
    }
}
