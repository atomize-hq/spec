//! Type definitions for spec-core
//!
//! This module defines the core data structures used throughout the spec pipeline:
//! - SpecStruct: Raw parsed form from YAML (mirrors schema)
//! - ResolvedSpec: Normalized internal representation (IR) used by the generator

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
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
    /// Human-readable intent string used for generated Rust doc comments.
    pub intent_why: String,
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
    /// spec_version from the source unit (e.g., "0.3.0")
    pub spec_version: Option<String>,
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

/// A library-qualified unit identity. `library == None` represents the local/root library.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedUnitRef {
    library: Option<String>,
    unit_id: String,
}

impl QualifiedUnitRef {
    pub fn local(unit_id: impl Into<String>) -> Self {
        Self {
            library: None,
            unit_id: unit_id.into(),
        }
    }

    pub fn new(library: Option<impl Into<String>>, unit_id: impl Into<String>) -> Self {
        Self {
            library: library.map(Into::into),
            unit_id: unit_id.into(),
        }
    }

    pub fn library(&self) -> Option<&str> {
        self.library.as_deref()
    }

    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn callable_name(&self) -> &str {
        callable_name(&self.unit_id)
    }

    pub fn authored(&self) -> String {
        match &self.library {
            Some(library) => format!("{library}::{}", self.unit_id),
            None => self.unit_id.clone(),
        }
    }
}

impl fmt::Display for QualifiedUnitRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.authored())
    }
}

/// Parsed identity for an authored dep string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepRef {
    library_alias: Option<String>,
    unit_id: String,
}

impl DepRef {
    pub fn local(unit_id: impl Into<String>) -> Self {
        Self {
            library_alias: None,
            unit_id: unit_id.into(),
        }
    }

    pub fn external(library_alias: impl Into<String>, unit_id: impl Into<String>) -> Self {
        Self {
            library_alias: Some(library_alias.into()),
            unit_id: unit_id.into(),
        }
    }

    pub fn parse(authored: &str) -> Result<Self, DepRefParseError> {
        let authored = authored.trim();
        if authored.is_empty() {
            return Err(DepRefParseError::InvalidFormat {
                authored: authored.to_string(),
            });
        }

        if let Some((library_alias, unit_id)) = authored.split_once("::") {
            if library_alias.is_empty() || unit_id.is_empty() || unit_id.contains("::") {
                return Err(DepRefParseError::InvalidFormat {
                    authored: authored.to_string(),
                });
            }

            validate_dep_segment(library_alias, authored)?;
            validate_unit_id(unit_id, authored)?;

            Ok(Self::external(library_alias, unit_id))
        } else {
            validate_unit_id(authored, authored)?;
            Ok(Self::local(authored))
        }
    }

    pub fn library_alias(&self) -> Option<&str> {
        self.library_alias.as_deref()
    }

    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn callable_name(&self) -> &str {
        callable_name(&self.unit_id)
    }

    pub fn authored(&self) -> String {
        match &self.library_alias {
            Some(library_alias) => format!("{library_alias}::{}", self.unit_id),
            None => self.unit_id.clone(),
        }
    }

    pub fn to_qualified(&self, current_library: Option<&str>) -> QualifiedUnitRef {
        QualifiedUnitRef::new(
            self.library_alias
                .as_deref()
                .or(current_library)
                .map(str::to_string),
            self.unit_id.clone(),
        )
    }
}

impl fmt::Display for DepRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.authored())
    }
}

impl TryFrom<&str> for DepRef {
    type Error = DepRefParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepRefParseError {
    InvalidFormat { authored: String },
    InvalidSegment { segment: String, authored: String },
}

impl fmt::Display for DepRefParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat { authored } => write!(
                f,
                "dep '{authored}' must use 'module/name' or 'library::module/name' syntax"
            ),
            Self::InvalidSegment { segment, authored } => {
                write!(f, "dep '{authored}' contains invalid segment '{segment}'")
            }
        }
    }
}

impl std::error::Error for DepRefParseError {}

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
            intent_why: spec.intent.why,
            fn_name,
            module_path,
            deps: spec.deps,
            imports: spec.imports,
            body_rust: spec.body.rust,
            contract: spec.contract,
            local_tests: spec.local_tests,
            links: spec.links,
            spec_version: spec.spec_version,
        }
    }

    /// Convert a dep ID to its Rust use statement path
    /// e.g., "money/round" -> "crate::money::round::round"
    pub fn dep_to_use_path(dep_id: &str) -> String {
        let unit_id = dep_unit_id(dep_id);
        format!(
            "crate::{}::{};",
            unit_id.replace('/', "::"),
            callable_name(unit_id)
        )
    }

    /// Get the fn_name (last segment) from a dep ID
    /// e.g., "money/round" -> "round"
    pub fn dep_fn_name(dep_id: &str) -> &str {
        callable_name(dep_unit_id(dep_id))
    }

    /// Returns `Some((dep1, dep2))` if two deps share the same callable name, `None` otherwise.
    pub fn has_dep_collision(deps: &[String]) -> Option<(&String, &String)> {
        for (i, first) in deps.iter().enumerate() {
            let Ok(first_dep) = DepRef::parse(first) else {
                continue;
            };
            for second in &deps[i + 1..] {
                let Ok(second_dep) = DepRef::parse(second) else {
                    continue;
                };
                if first_dep.callable_name() == second_dep.callable_name() {
                    return Some((first, second));
                }
            }
        }
        None
    }
}

/// Get the callable name (last segment) from a hierarchical spec ID.
pub fn callable_name(spec_id: &str) -> &str {
    spec_id
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(spec_id)
}

/// Strip an authored dep string down to its unit ID segment.
pub fn dep_unit_id(dep_id: &str) -> &str {
    dep_id
        .split_once("::")
        .map(|(_, unit_id)| unit_id)
        .unwrap_or(dep_id)
}

/// Check for callable-name collisions across arbitrary hierarchical IDs.
pub fn has_callable_collision(ids: &[String]) -> Option<(&String, &String, &str)> {
    for (i, first) in ids.iter().enumerate() {
        let first_name = callable_name(first);
        for second in &ids[i + 1..] {
            if callable_name(second) == first_name {
                return Some((first, second, first_name));
            }
        }
    }
    None
}

/// Raw parsed form from YAML for .test.spec files (molecule tests)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoleculeTestStruct {
    pub id: String,
    pub intent: Intent,
    #[serde(default)]
    pub covers: Vec<String>,
    pub body: Body,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
}

/// Source information for a loaded molecule test (file path tracking)
#[derive(Debug, Clone, PartialEq)]
pub struct MoleculeTestSource {
    pub file_path: String,
    pub id: String,
}

/// A loaded molecule test with its source file info
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedMoleculeTest {
    pub source: MoleculeTestSource,
    pub test: MoleculeTestStruct,
}

/// Normalized internal representation for molecule tests, consumed by the generator
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedMoleculeTest {
    /// Canonical ID: "pricing/discount_plus_tax"
    pub id: String,
    /// Last segment after final '/': "discount_plus_tax"
    pub fn_name: String,
    /// Everything before final '/': "pricing"
    pub module_path: String,
    pub intent_why: String,
    pub covers: Vec<String>,
    pub body_rust: String,
    pub spec_version: Option<String>,
}

impl ResolvedMoleculeTest {
    /// Derive fn_name and module_path from hierarchical ID.
    /// The schema enforces at least one '/', so the rsplit_once fallback is defensive only.
    pub fn from_loaded(loaded: &LoadedMoleculeTest) -> Self {
        let id = loaded.test.id.as_str();
        let (module_path, fn_name) = id
            .rsplit_once('/')
            .map(|(m, f)| (m.to_string(), f.to_string()))
            .unwrap_or_else(|| (String::new(), id.to_string()));
        Self {
            id: id.to_string(),
            fn_name,
            module_path,
            intent_why: loaded.test.intent.why.clone(),
            covers: loaded.test.covers.clone(),
            body_rust: loaded.test.body.rust.clone(),
            spec_version: loaded.test.spec_version.clone(),
        }
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

fn validate_unit_id(unit_id: &str, authored: &str) -> Result<(), DepRefParseError> {
    if !unit_id.contains('/') {
        return Err(DepRefParseError::InvalidFormat {
            authored: authored.to_string(),
        });
    }

    for segment in unit_id.split('/') {
        validate_dep_segment(segment, authored)?;
    }

    Ok(())
}

fn validate_dep_segment(segment: &str, authored: &str) -> Result<(), DepRefParseError> {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return Err(DepRefParseError::InvalidSegment {
            segment: segment.to_string(),
            authored: authored.to_string(),
        });
    };

    if !first.is_ascii_lowercase()
        || !chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(DepRefParseError::InvalidSegment {
            segment: segment.to_string(),
            authored: authored.to_string(),
        });
    }

    Ok(())
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
            spec_version: None,
        };

        let resolved = ResolvedSpec::from_spec(spec);
        assert_eq!(resolved.id, "pricing/apply_discount");
        assert_eq!(resolved.intent_why, "Apply discount");
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
            spec_version: None,
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
        assert_eq!(ResolvedSpec::dep_fn_name("shared::money/round"), "round");
    }

    #[test]
    fn test_callable_name() {
        assert_eq!(callable_name("money/round"), "round");
        assert_eq!(callable_name("utils/math/round"), "round");
    }

    #[test]
    fn test_has_dep_collision() {
        let deps = vec!["money/round".to_string(), "utils/round".to_string()];
        let collision = ResolvedSpec::has_dep_collision(&deps);
        assert!(collision.is_some());

        let external_collision = vec!["money/round".to_string(), "shared::utils/round".to_string()];
        let collision = ResolvedSpec::has_dep_collision(&external_collision);
        assert!(collision.is_some());

        let deps_no_collision = vec!["money/round".to_string(), "money/add".to_string()];
        let no_collision = ResolvedSpec::has_dep_collision(&deps_no_collision);
        assert!(no_collision.is_none());
    }

    #[test]
    fn test_parse_local_dep_ref() {
        let dep = DepRef::parse("money/round").expect("local dep should parse");
        assert_eq!(dep.library_alias(), None);
        assert_eq!(dep.unit_id(), "money/round");
        assert_eq!(dep.callable_name(), "round");
        assert_eq!(dep.to_string(), "money/round");
        assert_eq!(
            dep.to_qualified(None),
            QualifiedUnitRef::local("money/round")
        );
    }

    #[test]
    fn test_parse_external_dep_ref() {
        let dep = DepRef::parse("shared::money/round").expect("external dep should parse");
        assert_eq!(dep.library_alias(), Some("shared"));
        assert_eq!(dep.unit_id(), "money/round");
        assert_eq!(dep.callable_name(), "round");
        assert_eq!(dep.to_string(), "shared::money/round");
        assert_eq!(
            dep.to_qualified(None),
            QualifiedUnitRef::new(Some("shared".to_string()), "money/round")
        );
    }

    #[test]
    fn test_qualify_local_dep_ref_into_library() {
        let dep = DepRef::parse("money/round").expect("local dep should parse");
        assert_eq!(
            dep.to_qualified(Some("shared")),
            QualifiedUnitRef::new(Some("shared".to_string()), "money/round")
        );
    }

    #[test]
    fn test_parse_invalid_dep_ref() {
        let err = DepRef::parse("shared::").expect_err("invalid dep should fail");
        assert_eq!(
            err.to_string(),
            "dep 'shared::' must use 'module/name' or 'library::module/name' syntax"
        );

        let err = DepRef::parse("shared::Money/round").expect_err("invalid segment should fail");
        assert_eq!(
            err.to_string(),
            "dep 'shared::Money/round' contains invalid segment 'Money'"
        );
    }

    #[test]
    fn test_has_callable_collision() {
        let ids = vec!["money/round".to_string(), "utils/round".to_string()];
        let collision = has_callable_collision(&ids).expect("expected collision");
        assert_eq!(collision.0, "money/round");
        assert_eq!(collision.1, "utils/round");
        assert_eq!(collision.2, "round");

        let ids_no_collision = vec!["money/round".to_string(), "money/add".to_string()];
        assert!(has_callable_collision(&ids_no_collision).is_none());
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
