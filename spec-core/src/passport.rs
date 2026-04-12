//! Passport generation for spec units.
//!
//! A passport is a static knowledge artifact derived from a LoadedSpec. One
//! `.spec.passport.json` file is emitted per unit, co-located with its
//! `.unit.spec` source file. Passports are derived artifacts (gitignored) and
//! are written atomically only after all generation succeeds.

use crate::generator::write_generated_file;
use crate::types::LoadedSpec;
use crate::{AUTHORED_SPEC_VERSION, Result, SpecError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// A single contract input parameter in the passport JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// Contract section of the passport.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportContract {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<PassportInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invariants: Vec<String>,
}

/// A local test entry in the passport.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportLocalTest {
    pub id: String,
    pub expect: String,
}

/// Observed runtime result for one declared local test.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportTestResult {
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Minimal artifact provenance for machine-readable outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactProvenance {
    pub git_commit_sha: String,
}

/// Observed runtime evidence captured from the last `spec test` run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportEvidence {
    pub build_status: String,
    pub test_results: Vec<PassportTestResult>,
    pub observed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ArtifactProvenance>,
}

/// The full passport document for one unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Passport {
    pub spec_version: String,
    pub id: String,
    pub intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<PassportContract>,
    pub deps: Vec<String>,
    pub local_tests: Vec<PassportLocalTest>,
    pub generated_at: String,
    pub source_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PassportEvidence>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_contract_hash"
    )]
    pub contract_hash: Option<String>,
}

/// Build a Passport from a LoadedSpec.
///
/// `generated_at` is injected so all passports in one run share an identical
/// timestamp (batch consistency).
pub fn build_passport(spec: &LoadedSpec, generated_at: &str) -> Passport {
    build_passport_with_evidence(spec, generated_at, None, None)
}

/// Build a Passport from a LoadedSpec and optional observed evidence.
pub fn build_passport_with_evidence(
    spec: &LoadedSpec,
    generated_at: &str,
    evidence: Option<PassportEvidence>,
    contract_hash: Option<String>,
) -> Passport {
    let contract = spec.spec.contract.as_ref().map(|c| PassportContract {
        inputs: c
            .inputs
            .as_ref()
            .map(|m| {
                m.iter()
                    .map(|(name, type_str)| PassportInput {
                        name: name.clone(),
                        type_: type_str.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        returns: c.returns.clone(),
        invariants: c.invariants.clone(),
    });

    Passport {
        spec_version: spec
            .spec
            .spec_version
            .clone()
            .unwrap_or_else(|| AUTHORED_SPEC_VERSION.to_string()),
        id: spec.spec.id.clone(),
        intent: spec.spec.intent.why.clone(),
        contract,
        deps: spec.spec.deps.clone(),
        local_tests: spec
            .spec
            .local_tests
            .iter()
            .map(|t| PassportLocalTest {
                id: t.id.clone(),
                expect: t.expect.clone(),
            })
            .collect(),
        generated_at: generated_at.to_string(),
        source_file: spec.source.file_path.clone(),
        evidence,
        contract_hash,
    }
}

/// Compute SHA-256 of the serialized contract field.
///
/// Returns `None` when the spec has no contract block.
pub fn compute_contract_hash(spec: &LoadedSpec) -> Option<String> {
    let contract = spec.spec.contract.as_ref()?;
    let json = serde_json::to_string(contract)
        .expect("contract serialization cannot fail for well-formed spec");
    let hash = Sha256::digest(json.as_bytes());
    Some(format!("sha256:{}", hex::encode(hash)))
}

/// Return the passport file path for a given source `.unit.spec` path.
///
/// Example: `units/pricing/apply_tax.unit.spec` →
///          `units/pricing/apply_tax.spec.passport.json`
pub fn passport_path_for(source_path: &Path) -> Result<PathBuf> {
    let parent = source_path.parent().ok_or_else(|| SpecError::Generator {
        message: format!(
            "passport_path_for: cannot determine parent of {}",
            source_path.display()
        ),
    })?;

    let filename = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "passport_path_for: no filename in {}",
                source_path.display()
            ),
        })?;

    let stem = filename
        .strip_suffix(".unit.spec")
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "passport_path_for: path does not end with .unit.spec: {}",
                source_path.display()
            ),
        })?;

    Ok(parent.join(format!("{stem}.spec.passport.json")))
}

/// Read a passport for a given source `.unit.spec` path.
///
/// Returns `Ok(None)` when the passport file does not exist.
/// Returns `Err` when the file exists but cannot be parsed.
pub fn read_passport(source_path: &Path) -> Result<Option<Passport>> {
    let passport_path = passport_path_for(source_path)?;

    let content = match fs::read_to_string(&passport_path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };

    Ok(Some(serde_json::from_str(&content)?))
}

/// Serialize a Passport to pretty-printed JSON and write it atomically
/// co-located with the source `.unit.spec` file.
pub fn write_passport(passport: &Passport, source_file_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(passport).map_err(|e| SpecError::Generator {
        message: format!("Failed to serialize passport for '{}': {e}", passport.id),
    })?;
    let passport_path = passport_path_for(source_file_path)?;
    write_generated_file(&passport_path.display().to_string(), &json)
}

fn deserialize_contract_hash<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let contract_hash = Option::<String>::deserialize(deserializer)?;
    Ok(contract_hash.filter(|hash| hash.starts_with("sha256:")))
}

/// Emit `**/*.spec.passport.json` to `<spec_root>/.gitignore` if not already
/// present. Creates the file if it does not exist; appends if the entry is
/// missing. Safe to call on every generate run (idempotent).
pub fn ensure_gitignore_entry(spec_root: &Path) -> Result<()> {
    const ENTRY: &str = "**/*.spec.passport.json";
    let gitignore_path = spec_root.join(".gitignore");

    let existing = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    // Check for the entry on any line (trim trailing whitespace per line).
    if existing.lines().any(|l| l.trim_end() == ENTRY) {
        return Ok(());
    }

    // Append the entry, ensuring a leading newline if the file is non-empty and
    // doesn't already end with a newline.
    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(ENTRY);
    content.push('\n');

    fs::write(&gitignore_path, content)?;
    Ok(())
}

/// Return an RFC 3339 UTC timestamp for the current moment (second precision).
///
/// Uses only `std::time`; no external crate dependency required.
/// Output format: `YYYY-MM-DDTHH:MM:SSZ`
pub fn rfc3339_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (year, month, day, h, m, s) = secs_to_gregorian(secs);
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Convert a Unix timestamp (seconds since epoch) to (year, month, day, hour,
/// minute, second) using the proleptic Gregorian calendar.
///
/// Algorithm: Richards (2013), "Calendrical Calculations" variant — integer
/// arithmetic only, handles leap years including 100/400-year rules.
fn secs_to_gregorian(secs: u64) -> (u32, u32, u32, u32, u32, u32) {
    let sec = (secs % 60) as u32;
    let min = ((secs / 60) % 60) as u32;
    let hour = ((secs / 3600) % 24) as u32;
    let days = secs / 86400; // days since 1970-01-01

    // Shift epoch to 1 March 0000 (simplifies leap-year arithmetic).
    // 719468 = days from 0000-03-01 to 1970-01-01
    let z = days + 719_468;
    let era = z / 146_097; // 400-year era
    let doe = z - era * 146_097; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // year of era [0, 399]
    let y = yoe + era * 400; // year (March-based)
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month of year (March = 0)
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let y = if m <= 2 { y + 1 } else { y }; // adjust year for Jan/Feb

    (y as u32, m as u32, d as u32, hour, min, sec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Contract, Intent, LocalTest, SpecSource, SpecStruct};
    use indexmap::IndexMap;
    use tempfile::TempDir;

    fn make_loaded_spec(
        id: &str,
        file_path: &str,
        spec_version: Option<&str>,
        contract: Option<Contract>,
        deps: Vec<&str>,
        local_tests: Vec<(&str, &str)>,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: file_path.to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract,
                deps: deps.into_iter().map(String::from).collect(),
                imports: vec![],
                body: Body {
                    rust: "{ 42 }".to_string(),
                },
                local_tests: local_tests
                    .into_iter()
                    .map(|(tid, exp)| LocalTest {
                        id: tid.to_string(),
                        expect: exp.to_string(),
                    })
                    .collect(),
                links: None,
                spec_version: spec_version.map(String::from),
            },
        }
    }

    #[test]
    fn build_passport_full_contract() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        inputs.insert("rate".to_string(), "Decimal".to_string());
        let contract = Contract {
            inputs: Some(inputs),
            returns: Some("Decimal".to_string()),
            invariants: vec!["output >= subtotal".to_string()],
        };

        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(contract),
            vec!["money/round"],
            vec![("basic", "apply_tax(1,2) == 3")],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");

        assert_eq!(passport.spec_version, "0.3.0");
        assert_eq!(passport.id, "pricing/apply_tax");
        assert_eq!(passport.intent, "Why pricing/apply_tax");
        assert_eq!(passport.deps, vec!["money/round"]);
        assert_eq!(passport.generated_at, "2026-04-04T00:00:00Z");
        assert_eq!(passport.source_file, "units/pricing/apply_tax.unit.spec");
        assert!(passport.contract_hash.is_none());

        let c = passport.contract.unwrap();
        assert_eq!(c.inputs.len(), 2);
        assert_eq!(c.inputs[0].name, "subtotal");
        assert_eq!(c.inputs[0].type_, "Decimal");
        assert_eq!(c.inputs[1].name, "rate");
        assert_eq!(c.inputs[1].type_, "Decimal");
        assert_eq!(c.returns, Some("Decimal".to_string()));
        assert_eq!(c.invariants, vec!["output >= subtotal"]);

        assert_eq!(passport.local_tests.len(), 1);
        assert_eq!(passport.local_tests[0].id, "basic");
        assert!(passport.evidence.is_none());
    }

    #[test]
    fn build_passport_no_contract() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            None,
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        assert!(passport.contract.is_none());
        assert_eq!(passport.spec_version, "0.3.0"); // default
        assert!(passport.deps.is_empty());
        assert!(passport.local_tests.is_empty());
        assert!(passport.evidence.is_none());
        assert!(passport.contract_hash.is_none());
    }

    #[test]
    fn build_passport_uses_spec_version_from_unit() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "t");
        assert_eq!(passport.spec_version, "0.3.0");
    }

    #[test]
    fn build_passport_defaults_spec_version_when_absent() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            None,
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "t");
        assert_eq!(passport.spec_version, "0.3.0");
    }

    #[test]
    fn passport_path_for_standard_unit() {
        let p = passport_path_for(Path::new("units/pricing/apply_tax.unit.spec")).unwrap();
        assert_eq!(
            p,
            PathBuf::from("units/pricing/apply_tax.spec.passport.json")
        );
    }

    #[test]
    fn passport_path_for_root_level_unit() {
        let p = passport_path_for(Path::new("money/round.unit.spec")).unwrap();
        assert_eq!(p, PathBuf::from("money/round.spec.passport.json"));
    }

    #[test]
    fn passport_path_for_rejects_non_unit_spec() {
        let result = passport_path_for(Path::new("units/pricing/apply_tax.rs"));
        assert!(result.is_err());
    }

    #[test]
    fn test_contract_hash_absent_for_no_contract() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );

        assert_eq!(compute_contract_hash(&spec), None);
    }

    #[test]
    fn test_contract_hash_present_for_contract() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        inputs.insert("rate".to_string(), "Decimal".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("Decimal".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            vec![],
            vec![],
        );

        let expected = {
            let contract = spec.spec.contract.as_ref().unwrap();
            let json = serde_json::to_string(contract).unwrap();
            let hash = Sha256::digest(json.as_bytes());
            format!("sha256:{}", hex::encode(hash))
        };

        assert_eq!(compute_contract_hash(&spec), Some(expected));
    }

    #[test]
    fn test_contract_hash_changes_on_input_reorder() {
        let mut inputs_ab = IndexMap::new();
        inputs_ab.insert("a".to_string(), "String".to_string());
        inputs_ab.insert("b".to_string(), "String".to_string());

        let mut inputs_ba = IndexMap::new();
        inputs_ba.insert("b".to_string(), "String".to_string());
        inputs_ba.insert("a".to_string(), "String".to_string());

        let spec_ab = make_loaded_spec(
            "example/alpha",
            "units/example/alpha.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs_ab),
                returns: Some("String".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let spec_ba = make_loaded_spec(
            "example/alpha",
            "units/example/alpha.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs_ba),
                returns: Some("String".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );

        assert_ne!(
            compute_contract_hash(&spec_ab),
            compute_contract_hash(&spec_ba)
        );
    }

    #[test]
    fn test_read_passport_returns_none_for_missing() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");

        let passport = read_passport(&source_path).unwrap();
        assert!(passport.is_none());
    }

    #[test]
    fn test_read_passport_returns_err_for_malformed() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        let passport_path = passport_path_for(&source_path).unwrap();
        fs::write(&passport_path, "{not valid json").unwrap();

        let result = read_passport(&source_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_passport_discards_non_sha256_contract_hash() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        let passport_path = passport_path_for(&source_path).unwrap();
        fs::write(
            &passport_path,
            r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_tax",
  "intent": "Why pricing/apply_tax",
  "deps": [],
  "local_tests": [],
  "generated_at": "2026-04-04T00:00:00Z",
  "source_file": "units/pricing/apply_tax.unit.spec",
  "contract_hash": "deadbeef"
}"#,
        )
        .unwrap();

        let passport = read_passport(&source_path).unwrap().unwrap();
        assert!(passport.contract_hash.is_none());
    }

    #[test]
    fn test_read_passport_roundtrip() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap();

        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i32".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            vec![],
            vec![],
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            }),
            compute_contract_hash(&spec),
        );
        write_passport(&passport, &source_path).unwrap();

        let parsed = read_passport(&source_path).unwrap().unwrap();
        assert_eq!(parsed, passport);
    }

    #[test]
    fn write_passport_creates_valid_json() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap(); // create source file so parent exists

        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        write_passport(&passport, &source_path).unwrap();

        let passport_path = dir.path().join("apply_tax.spec.passport.json");
        assert!(passport_path.exists());

        let content = fs::read_to_string(&passport_path).unwrap();
        let parsed: Passport = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.id, "pricing/apply_tax");
        assert_eq!(parsed.generated_at, "2026-04-04T00:00:00Z");
    }

    #[test]
    fn write_passport_round_trips_contract_with_omitted_empty_fields() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap();

        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        write_passport(&passport, &source_path).unwrap();

        let content = fs::read_to_string(dir.path().join("apply_tax.spec.passport.json")).unwrap();
        let parsed: Passport = serde_json::from_str(&content).unwrap();

        assert_eq!(
            parsed.contract.unwrap(),
            PassportContract {
                inputs: vec![PassportInput {
                    name: "subtotal".to_string(),
                    type_: "i32".to_string(),
                }],
                returns: Some("i32".to_string()),
                invariants: vec![],
            }
        );
    }

    #[test]
    fn build_passport_with_evidence_serializes_observed_results() {
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![("basic", "apply_tax(1,2) == 3")],
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: Some(ArtifactProvenance {
                    git_commit_sha: "abc123".to_string(),
                }),
            }),
            Some("sha256:abc123".to_string()),
        );

        assert_eq!(
            passport.evidence,
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: Some(ArtifactProvenance {
                    git_commit_sha: "abc123".to_string(),
                }),
            })
        );
        assert_eq!(passport.contract_hash, Some("sha256:abc123".to_string()));
    }

    #[test]
    fn legacy_passport_evidence_without_provenance_still_deserializes() {
        let passport: Passport = serde_json::from_str(
            r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_tax",
  "intent": "Why pricing/apply_tax",
  "deps": [],
  "local_tests": [],
  "generated_at": "2026-04-04T00:00:00Z",
  "source_file": "units/pricing/apply_tax.unit.spec",
  "evidence": {
    "build_status": "pass",
    "test_results": [],
    "observed_at": "2026-04-04T00:01:00Z"
  }
}"#,
        )
        .unwrap();

        assert_eq!(
            passport.evidence,
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            })
        );
    }

    #[test]
    fn spec_generate_passport_has_no_evidence() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        let json = serde_json::to_string(&passport).unwrap();

        assert!(passport.evidence.is_none());
        assert!(passport.contract_hash.is_none());
        assert!(
            !json.contains("\"evidence\""),
            "static passport should not serialize evidence: {json}"
        );
    }

    #[test]
    fn rfc3339_now_format() {
        let ts = rfc3339_now();
        // Must match YYYY-MM-DDTHH:MM:SSZ
        assert_eq!(ts.len(), 20, "timestamp length should be 20: {ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert_eq!(&ts[19..20], "Z");
    }

    #[test]
    fn rfc3339_known_epoch() {
        // Unix epoch = 1970-01-01T00:00:00Z
        let (y, mo, d, h, m, s) = secs_to_gregorian(0);
        assert_eq!((y, mo, d, h, m, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn rfc3339_known_date() {
        // 2026-04-04T12:34:56Z
        // Days from epoch to 2026-04-04: calculate manually
        // 2026-04-04 = epoch + 20547 days + 45296 seconds
        let ts = 20547 * 86400 + 12 * 3600 + 34 * 60 + 56;
        let (y, mo, d, h, m, s) = secs_to_gregorian(ts);
        assert_eq!((y, mo, d, h, m, s), (2026, 4, 4, 12, 34, 56));
    }

    #[test]
    fn ensure_gitignore_creates_file_when_absent() {
        let dir = TempDir::new().unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("**/*.spec.passport.json"));
    }

    #[test]
    fn ensure_gitignore_appends_when_entry_missing() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(".gitignore"), "*.rs\n").unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("*.rs"));
        assert!(content.contains("**/*.spec.passport.json"));
    }

    #[test]
    fn ensure_gitignore_is_idempotent() {
        let dir = TempDir::new().unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        let count = content.matches("**/*.spec.passport.json").count();
        assert_eq!(count, 1, "entry should appear exactly once");
    }

    #[test]
    fn ensure_gitignore_no_trailing_newline_handled() {
        let dir = TempDir::new().unwrap();
        // File without trailing newline
        fs::write(dir.path().join(".gitignore"), "*.rs").unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("*.rs\n**/*.spec.passport.json"));
    }
}
