//! Type definitions for spec-core
//!
//! This module defines the core data structures used throughout the spec pipeline:
//! - SpecStruct: Raw parsed form from YAML (mirrors schema)
//! - ResolvedSpec: Normalized function-unit IR used by the generator
//! - NormalizedUnit / NormalizedDataSeam: kind-aware M12 unit normalization

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
    #[serde(default)]
    pub body: Body,
    #[serde(default)]
    pub local_tests: Vec<LocalTest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
    #[serde(flatten)]
    pub extensions: UnitExtensions,
}

/// Required intent block explaining why this unit exists
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Intent {
    pub why: String,
}

/// Body containing the native Rust implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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

/// Authoring-time extension surface for non-function units.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UnitExtensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AuthoredDataShape>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constructors: Vec<AuthoredConstructor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<AuthoredMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<AuthoredBackends>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AuthoredDataShape {
    #[serde(default)]
    pub fields: IndexMap<String, AuthoredField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthoredField {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthoredConstructor {
    pub id: String,
    pub intent: Intent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    #[serde(default)]
    pub initializes: IndexMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthoredMethod {
    pub id: String,
    pub intent: Intent,
    pub receiver: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowering: Option<AuthoredMethodLowering>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AuthoredMethodLowering {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust: Option<AuthoredRustMethodLowering>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthoredRustMethodLowering {
    pub body: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AuthoredBackends {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust: Option<AuthoredRustBackend>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AuthoredRustBackend {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitKind {
    Function,
    Data,
}

impl UnitKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Data => "data",
        }
    }
}

impl TryFrom<&str> for UnitKind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "function" => Ok(Self::Function),
            "data" => Ok(Self::Data),
            other => Err(format!("unsupported unit kind '{other}'")),
        }
    }
}

impl SpecStruct {
    pub fn unit_kind(&self) -> std::result::Result<UnitKind, String> {
        UnitKind::try_from(self.kind.as_str())
    }
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

/// Kind-aware normalized unit surface introduced in M12.
#[derive(Debug, Clone, PartialEq)]
pub enum NormalizedUnit {
    Function(ResolvedSpec),
    Data(NormalizedDataSeam),
}

impl NormalizedUnit {
    pub fn id(&self) -> &str {
        match self {
            Self::Function(unit) => &unit.id,
            Self::Data(unit) => &unit.id,
        }
    }

    pub fn module_path(&self) -> &str {
        match self {
            Self::Function(unit) => &unit.module_path,
            Self::Data(unit) => &unit.module_path,
        }
    }

    pub fn kind(&self) -> UnitKind {
        match self {
            Self::Function(_) => UnitKind::Function,
            Self::Data(_) => UnitKind::Data,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedDataSeam {
    pub id: String,
    pub intent_why: String,
    pub type_name: String,
    pub module_path: String,
    pub fields: Vec<NormalizedDataField>,
    pub constructors: Vec<NormalizedConstructor>,
    pub methods: Vec<NormalizedMethod>,
    pub deps: Vec<String>,
    pub local_tests: Vec<LocalTest>,
    pub links: Option<Links>,
    pub spec_version: Option<String>,
    pub rust_backend: RustDataSeamBackend,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedDataField {
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedConstructor {
    pub id: String,
    pub intent_why: String,
    pub inputs: IndexMap<String, String>,
    pub initializes: IndexMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedMethod {
    pub id: String,
    pub intent_why: String,
    pub receiver: MethodReceiver,
    pub contract: Contract,
    pub deps: Vec<String>,
    pub rust_body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodReceiver {
    SharedRef,
}

impl MethodReceiver {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SharedRef => "shared_ref",
        }
    }
}

impl TryFrom<&str> for MethodReceiver {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "shared_ref" => Ok(Self::SharedRef),
            other => Err(format!("unsupported receiver '{other}'")),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RustDataSeamBackend {
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustDataSeamLowering {
    pub id: String,
    pub module_path: String,
    pub struct_name: String,
    pub fields: Vec<RustDataFieldLowering>,
    pub constructors: Vec<RustInherentMethodLowering>,
    pub methods: Vec<RustInherentMethodLowering>,
    pub local_tests: Vec<LocalTest>,
    pub deps: Vec<String>,
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustDataFieldLowering {
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustInherentMethodLowering {
    pub id: String,
    pub is_constructor: bool,
    pub receiver: Option<MethodReceiver>,
    pub inputs: IndexMap<String, String>,
    pub returns: Option<String>,
    pub body_rust: String,
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

impl NormalizedDataSeam {
    pub fn from_spec(spec: SpecStruct) -> std::result::Result<Self, String> {
        let parts: Vec<&str> = spec.id.split('/').collect();
        let module_path = if parts.len() > 1 {
            parts[..parts.len() - 1].join("/")
        } else {
            String::new()
        };

        let fields = spec
            .extensions
            .data
            .as_ref()
            .map(|data| {
                data.fields
                    .iter()
                    .map(|(name, field)| NormalizedDataField {
                        name: name.clone(),
                        type_: field.type_.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let constructors = spec
            .extensions
            .constructors
            .iter()
            .map(|constructor| NormalizedConstructor {
                id: constructor.id.clone(),
                intent_why: constructor.intent.why.clone(),
                inputs: constructor
                    .contract
                    .as_ref()
                    .and_then(|contract| contract.inputs.clone())
                    .unwrap_or_default(),
                initializes: constructor.initializes.clone(),
            })
            .collect::<Vec<_>>();

        let mut deps = Vec::new();
        let methods = spec
            .extensions
            .methods
            .iter()
            .map(|method| {
                for dep in &method.deps {
                    if !deps.contains(dep) {
                        deps.push(dep.clone());
                    }
                }

                Ok(NormalizedMethod {
                    id: method.id.clone(),
                    intent_why: method.intent.why.clone(),
                    receiver: MethodReceiver::try_from(method.receiver.as_str())?,
                    contract: method.contract.clone().ok_or_else(|| {
                        format!("kind:data method '{}' is missing contract", method.id)
                    })?,
                    deps: method.deps.clone(),
                    rust_body: method
                        .lowering
                        .as_ref()
                        .and_then(|lowering| lowering.rust.as_ref())
                        .map(|rust| rust.body.clone())
                        .unwrap_or_default(),
                })
            })
            .collect::<std::result::Result<Vec<_>, String>>()?;

        Ok(Self {
            type_name: type_name_for_unit_id(&spec.id),
            id: spec.id,
            intent_why: spec.intent.why,
            module_path,
            fields,
            constructors,
            methods,
            deps,
            local_tests: spec.local_tests,
            links: spec.links,
            spec_version: spec.spec_version,
            rust_backend: RustDataSeamBackend {
                derives: spec
                    .extensions
                    .backends
                    .as_ref()
                    .and_then(|backends| backends.rust.as_ref())
                    .map(|rust| rust.derives.clone())
                    .unwrap_or_default(),
            },
        })
    }
}

/// Get the callable name (last segment) from a hierarchical spec ID.
pub fn callable_name(spec_id: &str) -> &str {
    spec_id
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(spec_id)
}

pub fn module_path_for_unit_id(unit_id: &str) -> String {
    unit_id
        .rsplit_once('/')
        .map(|(module_path, _)| module_path.to_string())
        .unwrap_or_default()
}

pub fn type_name_for_unit_id(unit_id: &str) -> String {
    callable_name(unit_id)
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            let mut out = String::new();
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
            out
        })
        .collect::<String>()
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
            extensions: UnitExtensions::default(),
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
            extensions: UnitExtensions::default(),
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

    #[test]
    fn test_type_name_for_unit_id() {
        assert_eq!(
            type_name_for_unit_id("pricing/checkout_quote"),
            "CheckoutQuote"
        );
    }

    #[test]
    fn test_normalized_data_seam_collects_unique_deps() {
        let spec = SpecStruct {
            id: "pricing/checkout_quote".to_string(),
            kind: "data".to_string(),
            intent: Intent {
                why: "Quote checkout totals".to_string(),
            },
            contract: None,
            deps: vec![],
            imports: vec![],
            body: Body::default(),
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
                        why: "Create a quote".to_string(),
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
                methods: vec![
                    AuthoredMethod {
                        id: "discounted_subtotal".to_string(),
                        intent: Intent {
                            why: "Compute discounted subtotal".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("Decimal".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec!["pricing/apply_discount".to_string()],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ apply_discount(self.subtotal, Decimal::ZERO) }"
                                    .to_string(),
                            }),
                        }),
                    },
                    AuthoredMethod {
                        id: "total".to_string(),
                        intent: Intent {
                            why: "Compute final total".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("Decimal".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec![
                            "pricing/apply_discount".to_string(),
                            "pricing/apply_tax".to_string(),
                        ],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ apply_tax(self.subtotal, self.tax_rate) }".to_string(),
                            }),
                        }),
                    },
                ],
                backends: Some(AuthoredBackends {
                    rust: Some(AuthoredRustBackend {
                        derives: vec!["Clone".to_string(), "Debug".to_string()],
                    }),
                }),
            },
        };

        let normalized = NormalizedDataSeam::from_spec(spec).unwrap();
        assert_eq!(normalized.type_name, "CheckoutQuote");
        assert_eq!(normalized.fields.len(), 2);
        assert_eq!(
            normalized.deps,
            vec![
                "pricing/apply_discount".to_string(),
                "pricing/apply_tax".to_string(),
            ]
        );
        assert_eq!(normalized.rust_backend.derives, vec!["Clone", "Debug"]);
    }
}
