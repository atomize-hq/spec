//! spec-core: Core library for parsing and generating Rust code from .unit.spec files
//!
//! This crate provides the core functionality for the spec toolchain:
//! - Loading and parsing .unit.spec YAML files
//! - Validating specs against the JSON Schema
//! - Normalizing to internal representation (IR)
//! - Generating readable Rust code

pub mod generator;
pub mod loader;
pub mod normalizer;
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

    #[error(
        "body.rust must not contain use statements; declare imports via deps instead at {path}"
    )]
    UseStatementInBody { path: String },

    #[error("Generator error: {message}")]
    Generator { message: String },

    #[error("Output directory error: {message}")]
    OutputDir { message: String },

    #[error("Missing .spec-generated marker in {path} - refusing to clean without marker")]
    MissingMarker { path: String },
}

impl From<walkdir::Error> for SpecError {
    fn from(err: walkdir::Error) -> Self {
        SpecError::Io(std::io::Error::other(err))
    }
}

/// Result type alias for spec-core operations
pub type Result<T> = std::result::Result<T, SpecError>;

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
