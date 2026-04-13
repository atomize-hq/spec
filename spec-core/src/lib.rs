//! spec-core: Core library for parsing and generating Rust code from .unit.spec files
//!
//! This crate provides the core functionality for the spec toolchain:
//! - Loading and parsing .unit.spec YAML files
//! - Validating specs against the JSON Schema
//! - Normalizing to internal representation (IR)
//! - Generating readable Rust code

pub const AUTHORED_SPEC_VERSION: &str = "0.3.0";

pub mod export;
pub mod generator;
pub mod graph;
pub mod loader;
pub mod normalizer;
pub mod passport;
pub mod pipeline;
mod syntax;
pub mod types;
pub mod validator;

use thiserror::Error;

/// Error types for spec-core operations
#[derive(Error, Debug)]
pub enum SpecError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File is not valid UTF-8: {path}")]
    InvalidUtf8 { path: String },

    #[error("YAML parse error: {message}")]
    YamlParse { message: String, path: String },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Schema validation failed: {message}")]
    SchemaValidation { message: String, path: String },

    #[error("Semantic validation error: {message}")]
    SemanticValidation { message: String, path: String },

    #[error("ID segment '{segment}' is a Rust reserved keyword in '{id}' at {path}")]
    RustKeyword {
        segment: String,
        id: String,
        path: String,
    },

    #[error("Duplicate ID '{id}' in {file1} and {file2}")]
    DuplicateId {
        id: String,
        file1: String,
        file2: String,
    },

    #[error("Dep fn_name collision: '{dep1}' and '{dep2}' both resolve to '{fn_name}' at {path}")]
    DepCollision {
        dep1: String,
        dep2: String,
        fn_name: String,
        path: String,
    },

    #[error("❌ dep '{dep}' not found in this spec set")]
    MissingDep { dep: String, path: String },

    #[error("❌ cycle detected: {}", cycle_path.join(" → "))]
    CyclicDep {
        cycle_path: Vec<String>,
        path: String,
    },

    #[error(
        "body.rust must not contain use statements; declare external imports via imports (and internal unit deps via deps) at {path}"
    )]
    UseStatementInBody { path: String },

    #[error("body.rust failed to parse as a block: {message} at {path}")]
    BodyRustMustBeBlock { message: String, path: String },

    #[error(
        "body.rust looks like a full function declaration — spec 0.3.0 expects only the function body block. \
         Remove the `pub fn name(params) -> ReturnType` line and keep only the `{{ ... }}` block. \
         See migration guide. at {path}"
    )]
    BodyRustLooksLikeFnDeclaration { path: String },

    #[error("local_tests[{id}].expect is not a valid Rust expression: {message} at {path}")]
    LocalTestExpectNotExpr {
        id: String,
        message: String,
        path: String,
    },

    #[error("duplicate local_tests id '{id}' at {path}")]
    DuplicateLocalTestId { id: String, path: String },

    #[error("contract.{field} has invalid Rust type '{type_str}': {message} at {path}")]
    ContractTypeInvalid {
        field: String,
        type_str: String,
        message: String,
        path: String,
    },

    #[error("contract.inputs key '{name}' is not a valid Rust identifier: {message} at {path}")]
    ContractInputNameInvalid {
        name: String,
        message: String,
        path: String,
    },

    #[error("Traversal error: {message} at {path}")]
    Traversal { message: String, path: String },

    #[error("Generator error: {message}")]
    Generator { message: String },

    #[error("Output directory error: {message}")]
    OutputDir { message: String },

    #[error("Missing .spec-generated marker in {path} - refusing to clean without marker")]
    MissingMarker { path: String },

    #[error("Molecule test '{test_id}' covers '{cover_id}' which was not found in the spec set at {test_path}")]
    MoleculeCoversNotFound {
        cover_id: String,
        test_id: String,
        test_path: String,
    },

    #[error("Duplicate molecule test ID '{id}' in {file1} and {file2}")]
    DuplicateMoleculeTestId {
        id: String,
        file1: String,
        file2: String,
    },

    #[error("body.rust failed to parse as a block: {message} at {test_path}")]
    MoleculeBodyRustMustBeBlock {
        message: String,
        test_path: String,
    },
}

impl From<walkdir::Error> for SpecError {
    fn from(err: walkdir::Error) -> Self {
        SpecError::Io(std::io::Error::other(err))
    }
}

/// Result type alias for spec-core operations
pub type Result<T> = std::result::Result<T, SpecError>;

#[derive(Error, Debug)]
pub enum SpecWarning {
    #[error("⚠ dep '{dep}' not found in this spec set")]
    MissingDep { dep: String, path: String },

    #[error("⚠ skipped symlink cycle at '{path}'; subtree was skipped")]
    SymlinkCycleSkipped { path: String },

    #[error(
        "⚠ spec_version not set in {path} — add `spec_version: \"{version}\"` to suppress this warning"
    )]
    MissingSpecVersion { path: String, version: &'static str },

    #[error("⚠ molecule test '{test_id}' has no covered units at {test_path}")]
    MoleculeTestNoCoveredUnits {
        test_id: String,
        test_path: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SpecError::InvalidUtf8 {
            path: "foo.unit.spec".to_string(),
        };
        assert_eq!(err.to_string(), "File is not valid UTF-8: foo.unit.spec");

        let err = SpecError::RustKeyword {
            segment: "type".to_string(),
            id: "pricing/type".to_string(),
            path: "test.unit.spec".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "ID segment 'type' is a Rust reserved keyword in 'pricing/type' at test.unit.spec"
        );
    }
}
