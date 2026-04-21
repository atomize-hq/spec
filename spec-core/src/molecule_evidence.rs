//! Observed evidence artifacts for `.test.spec` molecule tests.
//!
//! Molecule evidence is intentionally separate from unit passports so unit health
//! and molecule health remain independent trust planes.

use crate::passport::{ArtifactProvenance, compute_contract_hash};
use crate::types::{LoadedMoleculeTest, LoadedSpec};
use crate::{Result, SpecError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Observed outcome for one molecule test execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MoleculeEvidenceStatus {
    Pass,
    Fail,
    Unknown,
    BuildFail,
    Timeout,
    Stale,
}

impl MoleculeEvidenceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Unknown => "unknown",
            Self::BuildFail => "build_fail",
            Self::Timeout => "timeout",
            Self::Stale => "stale",
        }
    }
}

/// Observed runtime evidence for one `.test.spec` file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoleculeEvidence {
    pub schema_version: u8,
    pub id: String,
    pub source_file: String,
    pub covers: Vec<String>,
    pub status: MoleculeEvidenceStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub observed_at: String,
    pub test_body_hash: String,
    pub covered_unit_contract_hashes: BTreeMap<String, Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ArtifactProvenance>,
}

pub const MOLECULE_EVIDENCE_SCHEMA_VERSION: u8 = 1;

pub fn molecule_evidence_path_for(source_path: &Path) -> Result<PathBuf> {
    let parent = source_path.parent().ok_or_else(|| SpecError::Generator {
        message: format!(
            "molecule_evidence_path_for: cannot determine parent of {}",
            source_path.display()
        ),
    })?;

    let filename = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "molecule_evidence_path_for: no filename in {}",
                source_path.display()
            ),
        })?;

    let stem = filename
        .strip_suffix(".test.spec")
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "molecule_evidence_path_for: path does not end with .test.spec: {}",
                source_path.display()
            ),
        })?;

    Ok(parent.join(format!("{stem}.test.evidence.json")))
}

pub fn read_molecule_evidence(source_path: &Path) -> Result<Option<MoleculeEvidence>> {
    let evidence_path = molecule_evidence_path_for(source_path)?;

    let content = match fs::read_to_string(&evidence_path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };

    serde_json::from_str(&content)
        .map(Some)
        .map_err(|err| SpecError::MoleculeEvidenceMalformed {
            path: evidence_path.display().to_string(),
            message: err.to_string(),
        })
}

pub fn write_molecule_evidence(evidence: &MoleculeEvidence, source_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(evidence).map_err(|err| SpecError::Generator {
        message: format!(
            "Failed to serialize molecule evidence for '{}': {err}",
            evidence.id
        ),
    })?;
    let evidence_path = molecule_evidence_path_for(source_path)?;
    crate::generator::write_generated_file(&evidence_path.display().to_string(), &json)
}

pub fn compute_molecule_test_body_hash(test: &LoadedMoleculeTest) -> String {
    let json = serde_json::to_string(&test.test)
        .expect("molecule test serialization cannot fail for a loaded spec");
    let hash = Sha256::digest(json.as_bytes());
    format!("sha256:{}", hex::encode(hash))
}

pub fn covered_unit_contract_hashes_for_test(
    test: &LoadedMoleculeTest,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> BTreeMap<String, Option<String>> {
    let mut hashes = BTreeMap::new();
    for cover_id in &test.test.covers {
        let hash = specs_by_id.get(cover_id).and_then(compute_contract_hash);
        hashes.insert(cover_id.clone(), hash);
    }
    hashes
}

pub fn build_molecule_evidence(
    test: &LoadedMoleculeTest,
    status: MoleculeEvidenceStatus,
    reason: Option<String>,
    observed_at: &str,
    specs_by_id: &HashMap<String, LoadedSpec>,
    provenance: Option<&ArtifactProvenance>,
) -> MoleculeEvidence {
    MoleculeEvidence {
        schema_version: MOLECULE_EVIDENCE_SCHEMA_VERSION,
        id: test.test.id.clone(),
        source_file: test.source.file_path.clone(),
        covers: test.test.covers.clone(),
        status,
        reason,
        observed_at: observed_at.to_string(),
        test_body_hash: compute_molecule_test_body_hash(test),
        covered_unit_contract_hashes: covered_unit_contract_hashes_for_test(test, specs_by_id),
        provenance: provenance.cloned(),
    }
}

pub fn molecule_evidence_is_stale(
    evidence: &MoleculeEvidence,
    test: &LoadedMoleculeTest,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> bool {
    if evidence.test_body_hash != compute_molecule_test_body_hash(test) {
        return true;
    }

    evidence.covered_unit_contract_hashes
        != covered_unit_contract_hashes_for_test(test, specs_by_id)
}

pub fn ensure_gitignore_entry(spec_root: &Path) -> Result<()> {
    const ENTRY: &str = "**/*.test.evidence.json";
    let gitignore_path = spec_root.join(".gitignore");

    let existing = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    if existing.lines().any(|line| line.trim_end() == ENTRY) {
        return Ok(());
    }

    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(ENTRY);
    content.push('\n');

    fs::write(&gitignore_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredDataShape, AuthoredField, AuthoredMethod, AuthoredMethodLowering,
        AuthoredRustMethodLowering, Body, Intent, MoleculeTestSource, MoleculeTestStruct,
        SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;
    use tempfile::TempDir;

    fn loaded_spec(id: &str, returns: Option<&str>) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "test".to_string(),
                },
                contract: returns.map(|returns| crate::types::Contract {
                    inputs: Some(IndexMap::new()),
                    returns: Some(returns.to_string()),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "{ true }".to_string(),
                },
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    fn loaded_test(body: &str) -> LoadedMoleculeTest {
        loaded_test_covering("pricing/apply_tax", body)
    }

    fn loaded_test_covering(cover_id: &str, body: &str) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: "pricing/checkout_flow.test.spec".to_string(),
                id: "pricing/checkout_flow".to_string(),
            },
            test: MoleculeTestStruct {
                id: "pricing/checkout_flow".to_string(),
                intent: Intent {
                    why: "test".to_string(),
                },
                covers: vec![cover_id.to_string()],
                imports: None,
                body: Body {
                    rust: body.to_string(),
                },
                spec_version: Some("0.3.0".to_string()),
            },
        }
    }

    fn loaded_data_spec(id: &str, intent_why: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: intent_why.to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([(
                            "subtotal".to_string(),
                            AuthoredField {
                                type_: "Decimal".to_string(),
                            },
                        )]),
                    }),
                    constructors: vec![],
                    methods: vec![AuthoredMethod {
                        id: "total".to_string(),
                        intent: Intent {
                            why: "Return total".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: None,
                        deps: vec!["pricing/apply_tax".to_string()],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ self.subtotal }".to_string(),
                            }),
                        }),
                    }],
                    backends: None,
                    sum: None,
                },
            },
        }
    }

    #[test]
    fn molecule_evidence_path_for_maps_test_specs() {
        let path =
            molecule_evidence_path_for(Path::new("units/pricing/checkout_flow.test.spec")).unwrap();
        assert_eq!(
            path,
            PathBuf::from("units/pricing/checkout_flow.test.evidence.json")
        );
    }

    #[test]
    fn write_and_read_molecule_evidence_round_trip() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir.path().join("pricing/checkout_flow.test.spec");
        fs::create_dir_all(source_path.parent().unwrap()).unwrap();
        fs::write(&source_path, "").unwrap();

        let evidence = MoleculeEvidence {
            schema_version: MOLECULE_EVIDENCE_SCHEMA_VERSION,
            id: "pricing/checkout_flow".to_string(),
            source_file: source_path.display().to_string(),
            covers: vec!["pricing/apply_tax".to_string()],
            status: MoleculeEvidenceStatus::Pass,
            reason: None,
            observed_at: "2026-04-17T00:00:00Z".to_string(),
            test_body_hash: "sha256:test".to_string(),
            covered_unit_contract_hashes: BTreeMap::from([(
                "pricing/apply_tax".to_string(),
                Some("sha256:contract".to_string()),
            )]),
            provenance: None,
        };

        write_molecule_evidence(&evidence, &source_path).unwrap();
        let loaded = read_molecule_evidence(&source_path).unwrap().unwrap();
        assert_eq!(loaded, evidence);
    }

    #[test]
    fn read_molecule_evidence_returns_structured_malformed_error() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir.path().join("pricing/checkout_flow.test.spec");
        let evidence_path = temp_dir
            .path()
            .join("pricing/checkout_flow.test.evidence.json");
        fs::create_dir_all(source_path.parent().unwrap()).unwrap();
        fs::write(&source_path, "").unwrap();
        fs::write(&evidence_path, "{\"id\":").unwrap();

        let err = read_molecule_evidence(&source_path).unwrap_err();
        match err {
            SpecError::MoleculeEvidenceMalformed { path, .. } => {
                assert!(path.ends_with("checkout_flow.test.evidence.json"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn molecule_evidence_is_stale_when_body_changes() {
        let test = loaded_test("{ assert!(true); }");
        let specs_by_id = HashMap::from([(
            "pricing/apply_tax".to_string(),
            loaded_spec("pricing/apply_tax", Some("i32")),
        )]);
        let mut evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-17T00:00:00Z",
            &specs_by_id,
            None,
        );
        evidence.test_body_hash = "sha256:old".to_string();

        assert!(molecule_evidence_is_stale(&evidence, &test, &specs_by_id));
    }

    #[test]
    fn molecule_evidence_is_stale_when_covered_contract_hash_changes() {
        let test = loaded_test("{ assert!(true); }");
        let specs_by_id = HashMap::from([(
            "pricing/apply_tax".to_string(),
            loaded_spec("pricing/apply_tax", Some("i32")),
        )]);
        let mut evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-17T00:00:00Z",
            &specs_by_id,
            None,
        );
        evidence.covered_unit_contract_hashes = BTreeMap::from([(
            "pricing/apply_tax".to_string(),
            Some("sha256:old".to_string()),
        )]);

        assert!(molecule_evidence_is_stale(&evidence, &test, &specs_by_id));
    }

    #[test]
    fn molecule_evidence_is_stale_when_covered_data_seam_hash_changes() {
        let test = loaded_test_covering("pricing/checkout_quote", "{ assert!(true); }");
        let specs_by_id = HashMap::from([(
            "pricing/checkout_quote".to_string(),
            loaded_data_spec("pricing/checkout_quote", "updated intent"),
        )]);
        let mut evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-17T00:00:00Z",
            &specs_by_id,
            None,
        );
        evidence.covered_unit_contract_hashes = BTreeMap::from([(
            "pricing/checkout_quote".to_string(),
            Some("sha256:old".to_string()),
        )]);

        assert!(molecule_evidence_is_stale(&evidence, &test, &specs_by_id));
    }

    #[test]
    fn ensure_gitignore_entry_appends_molecule_evidence_pattern_once() {
        let temp_dir = TempDir::new().unwrap();
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(&gitignore_path, "*.rs\n").unwrap();

        ensure_gitignore_entry(temp_dir.path()).unwrap();
        ensure_gitignore_entry(temp_dir.path()).unwrap();

        let content = fs::read_to_string(gitignore_path).unwrap();
        assert!(content.contains("**/*.test.evidence.json"));
        assert_eq!(content.matches("**/*.test.evidence.json").count(), 1);
    }
}
