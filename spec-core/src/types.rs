//! Type definitions for spec-core
//!
//! This module defines the core data structures used throughout the spec pipeline:
//! - SpecStruct: Raw parsed form from YAML (mirrors schema)
//! - ResolvedSpec: Normalized internal representation (IR) used by the generator

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Raw parsed form from YAML (mirrors schema structure)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecStruct {
    pub id: String,
    pub kind: String,
    pub intent: Intent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default)]
    pub imports: Vec<String>,
    pub body: Body,
    #[serde(default)]
    pub local_tests: Vec<LocalTest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Required intent block explaining why this unit exists
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Intent {
    pub why: String,
}

/// Body containing the native Rust implementation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Body {
    pub rust: String,
}

/// Contract metadata - human-readable specifications (not used for codegen in M1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<IndexMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
    #[serde(default)]
    pub invariants: Vec<String>,
}

/// Local atom-level tests owned by this unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalTest {
    pub id: String,
    pub expect: String,
}

/// Links to molecule/organism level tests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Links {
    #[serde(default)]
    pub molecule_tests: Vec<String>,
}

/// Normalized internal representation (IR) consumed by the generator
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedSpec {
    /// Canonical ID: "pricing/apply_discount"
    pub id: String,
    /// Last segment: "apply_discount"
    pub fn_name: String,
    /// Everything before last segment: "pricing"
    pub module_path: String,
    /// Fully resolved dep IDs (empty vec if none)
    pub deps: Vec<String>,
    /// External/native Rust imports (e.g. "rust_decimal::Decimal")
    pub imports: Vec<String>,
    /// Raw Rust code from body.rust block
    pub body_rust: String,
    /// Contract metadata (stored, not used for codegen in M1)
    pub contract: Option<Contract>,
    /// Local tests (stored, not executed in M1)
    pub local_tests: Vec<LocalTest>,
    /// Links to molecule tests (stored, not used in M1)
    pub links: Option<Links>,
}

/// Source information for loaded specs (file path tracking)
#[derive(Debug, Clone, PartialEq)]
pub struct SpecSource {
    /// File path where this spec was loaded from
    pub file_path: String,
    /// ID of the spec within the file
    pub id: String,
}

/// Represents a loaded spec with its source file info
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedSpec {
    /// The source file information
    pub source: SpecSource,
    /// The parsed spec structure
    pub spec: SpecStruct,
}

impl ResolvedSpec {
    /// Derive fn_name and module_path from hierarchical ID
    pub fn from_spec(spec: SpecStruct) -> Self {
        let parts: Vec<&str> = spec.id.split('/').collect();
        let fn_name = parts.last().unwrap_or(&"").to_string();
        let module_path = if parts.len() > 1 {
            parts[..parts.len() - 1].join("/")
        } else {
            "".to_string()
        };

        Self {
            id: spec.id,
            fn_name,
            module_path,
            deps: spec.deps,
            imports: spec.imports,
            body_rust: spec.body.rust,
            contract: spec.contract,
            local_tests: spec.local_tests,
            links: spec.links,
        }
    }

    /// Convert a dep ID to its Rust use statement path
    /// e.g., "money/round" -> "crate::money::round::round"
    pub fn dep_to_use_path(dep_id: &str) -> String {
        format!(
            "crate::{}::{};",
            dep_id.replace('/', "::"),
            dep_id
                .rsplit_once('/')
                .map(|(_, name)| name)
                .unwrap_or(dep_id)
        )
    }

    /// Get the fn_name (last segment) from a dep ID
    /// e.g., "money/round" -> "round"
    pub fn dep_fn_name(dep_id: &str) -> &str {
        dep_id
            .rsplit_once('/')
            .map(|(_, name)| name)
            .unwrap_or(dep_id)
    }

    /// Check for dep fn_name collisions
    /// Returns true if two deps in the list have the same fn_name
    pub fn has_dep_collision(deps: &[String]) -> Option<(&String, &String)> {
        for (i, dep1) in deps.iter().enumerate() {
            let fn1 = Self::dep_fn_name(dep1);
            for dep2 in &deps[i + 1..] {
                if Self::dep_fn_name(dep2) == fn1 {
                    return Some((dep1, dep2));
                }
            }
        }
        None
    }
}

/// Rust reserved keywords that cannot be used as identifiers.
/// Covers both active keywords (Rust 2018+) and reserved-for-future-use keywords that
/// are also invalid as identifiers in current editions.
pub const RUST_KEYWORDS: &[&str] = &[
    // Active keywords
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while",
    // Reserved for future use (also invalid as plain identifiers)
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
    "unsized", "virtual", "yield",
];

/// Check if a string is a Rust reserved keyword
pub fn is_rust_keyword(s: &str) -> bool {
    RUST_KEYWORDS.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_spec_basic() {
        let spec = SpecStruct {
            id: "pricing/apply_discount".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Apply discount".to_string(),
            },
            contract: None,
            deps: vec![],
            imports: vec![],
            body: Body {
                rust: "{ }".to_string(),
            },
            local_tests: vec![],
            links: None,
        };

        let resolved = ResolvedSpec::from_spec(spec);
        assert_eq!(resolved.id, "pricing/apply_discount");
        assert_eq!(resolved.fn_name, "apply_discount");
        assert_eq!(resolved.module_path, "pricing");
    }

    #[test]
    fn test_from_spec_nested() {
        let spec = SpecStruct {
            id: "utils/math/round".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Round numbers".to_string(),
            },
            contract: None,
            deps: vec![],
            imports: vec![],
            body: Body {
                rust: "{ }".to_string(),
            },
            local_tests: vec![],
            links: None,
        };

        let resolved = ResolvedSpec::from_spec(spec);
        assert_eq!(resolved.id, "utils/math/round");
        assert_eq!(resolved.fn_name, "round");
        assert_eq!(resolved.module_path, "utils/math");
    }

    #[test]
    fn test_dep_to_use_path() {
        assert_eq!(
            ResolvedSpec::dep_to_use_path("money/round"),
            "crate::money::round::round;"
        );
        assert_eq!(
            ResolvedSpec::dep_to_use_path("utils/math/round"),
            "crate::utils::math::round::round;"
        );
    }

    #[test]
    fn test_dep_fn_name() {
        assert_eq!(ResolvedSpec::dep_fn_name("money/round"), "round");
        assert_eq!(ResolvedSpec::dep_fn_name("utils/math/round"), "round");
    }

    #[test]
    fn test_has_dep_collision() {
        let deps = vec!["money/round".to_string(), "utils/round".to_string()];
        let collision = ResolvedSpec::has_dep_collision(&deps);
        assert!(collision.is_some());

        let deps_no_collision = vec!["money/round".to_string(), "money/add".to_string()];
        let no_collision = ResolvedSpec::has_dep_collision(&deps_no_collision);
        assert!(no_collision.is_none());
    }

    #[test]
    fn test_rust_keywords() {
        // Active keywords
        assert!(is_rust_keyword("type"));
        assert!(is_rust_keyword("mod"));
        assert!(is_rust_keyword("pub"));
        // Reserved-for-future-use keywords (newly added)
        assert!(is_rust_keyword("try"));
        assert!(is_rust_keyword("abstract"));
        assert!(is_rust_keyword("yield"));
        assert!(is_rust_keyword("final"));
        // Not keywords
        assert!(!is_rust_keyword("my_function"));
        assert!(!is_rust_keyword("pricing"));
    }
}
