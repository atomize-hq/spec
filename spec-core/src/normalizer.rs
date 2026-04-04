//! Normalizer module: Convert SpecStruct to ResolvedSpec (IR)
//!
//! Phase 3: Normalize parsed specs into canonical IR for generation.

use crate::types::{ResolvedSpec, SpecStruct, is_rust_keyword};
use crate::{Result, SpecError};

pub fn normalize_spec(mut spec: SpecStruct) -> Result<ResolvedSpec> {
    spec.id = canonicalize_id(&spec.id)?;
    Ok(ResolvedSpec::from_spec(spec))
}

fn canonicalize_id(id: &str) -> Result<String> {
    let trimmed = id.trim();
    validate_canonical_id(trimmed)?;
    Ok(trimmed.to_string())
}

fn validate_canonical_id(id: &str) -> Result<()> {
    if !id.contains('/') {
        return Err(SpecError::SemanticValidation {
            message: format!("ID '{id}' must be hierarchical and contain '/'"),
            path: String::new(),
        });
    }

    for segment in id.split('/') {
        let mut chars = segment.chars();
        let Some(first) = chars.next() else {
            return Err(SpecError::SemanticValidation {
                message: format!("ID '{id}' contains an empty segment"),
                path: String::new(),
            });
        };

        if !first.is_ascii_lowercase() {
            return Err(SpecError::SemanticValidation {
                message: format!("ID segment '{segment}' must start with a lowercase ASCII letter"),
                path: String::new(),
            });
        }

        if !chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_') {
            return Err(SpecError::SemanticValidation {
                message: format!(
                    "ID segment '{segment}' contains invalid characters; expected lowercase ASCII, digits, or '_'"
                ),
                path: String::new(),
            });
        }

        if is_rust_keyword(segment) {
            return Err(SpecError::SemanticValidation {
                message: format!("ID segment '{segment}' is a Rust reserved keyword"),
                path: String::new(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Intent};

    fn make_spec(id: &str) -> SpecStruct {
        SpecStruct {
            id: id.to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Normalize this unit".to_string(),
            },
            contract: None,
            deps: vec![],
            imports: vec![],
            body: Body {
                rust: "pub fn apply_discount() {}".to_string(),
            },
            local_tests: vec![],
            links: None,
            spec_version: None,
        }
    }

    #[test]
    fn trims_id_before_building_ir() {
        let resolved = normalize_spec(make_spec(" pricing/apply_discount \n")).unwrap();
        assert_eq!(resolved.id, "pricing/apply_discount");
        assert_eq!(resolved.fn_name, "apply_discount");
        assert_eq!(resolved.module_path, "pricing");
    }

    #[test]
    fn rejects_non_hierarchical_ids() {
        let err = normalize_spec(make_spec("apply_discount")).unwrap_err();
        assert!(err.to_string().contains("must be hierarchical"));
    }

    #[test]
    fn rejects_invalid_segments() {
        let err = normalize_spec(make_spec("pricing/ApplyDiscount")).unwrap_err();
        assert!(
            err.to_string()
                .contains("must start with a lowercase ASCII letter")
        );
    }

    #[test]
    fn rejects_keywords_defensively() {
        let err = normalize_spec(make_spec("pricing/type")).unwrap_err();
        assert!(err.to_string().contains("Rust reserved keyword"));
    }
}
