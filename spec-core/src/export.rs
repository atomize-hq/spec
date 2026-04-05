//! JSON export support for loaded spec sets.
//!
//! The export bundle is a read-only artifact intended for downstream tooling.
//! It includes authored unit metadata, any readable co-located passports,
//! the dependency edge list, and structured warnings for skipped passports.

use crate::passport::{Passport, passport_path_for};
use crate::types::{Contract, LoadedSpec, LocalTest};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const EXPORT_SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportBundle {
    pub schema_version: String,
    pub spec_version: String,
    pub exported_at: String,
    pub units: Vec<ExportUnit>,
    pub passports: Vec<Passport>,
    pub graph: ExportGraph,
    pub warnings: Vec<ExportWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportUnit {
    pub id: String,
    pub intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    pub deps: Vec<String>,
    pub local_tests: Vec<LocalTest>,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportGraph {
    pub edges: Vec<ExportEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExportEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportWarning {
    pub code: String,
    pub spec_id: String,
    pub passport_path: String,
    pub message: String,
}

pub fn build_export_bundle(specs: &[LoadedSpec], exported_at: &str) -> ExportBundle {
    let (passports, warnings) = load_passports_for_specs(specs);
    let mut edges = specs
        .iter()
        .flat_map(|spec| {
            spec.spec.deps.iter().map(|dep| ExportEdge {
                from: spec.spec.id.clone(),
                to: dep.clone(),
            })
        })
        .collect::<Vec<_>>();
    edges.sort();

    ExportBundle {
        schema_version: EXPORT_SCHEMA_VERSION.to_string(),
        spec_version: env!("CARGO_PKG_VERSION").to_string(),
        exported_at: exported_at.to_string(),
        units: specs.iter().map(ExportUnit::from).collect(),
        passports,
        graph: ExportGraph { edges },
        warnings,
    }
}

pub fn load_passports_for_specs(specs: &[LoadedSpec]) -> (Vec<Passport>, Vec<ExportWarning>) {
    let mut passports = Vec::new();
    let mut warnings = Vec::new();

    for spec in specs {
        let source_path = Path::new(&spec.source.file_path);
        let passport_path = match passport_path_for(source_path) {
            Ok(path) => path,
            Err(err) => {
                warnings.push(ExportWarning {
                    code: "passport_malformed".to_string(),
                    spec_id: spec.spec.id.clone(),
                    passport_path: source_path.display().to_string(),
                    message: err.to_string(),
                });
                continue;
            }
        };

        match fs::read_to_string(&passport_path) {
            Ok(content) => match serde_json::from_str::<Passport>(&content) {
                Ok(passport) => passports.push(passport),
                Err(err) => warnings.push(ExportWarning {
                    code: "passport_malformed".to_string(),
                    spec_id: spec.spec.id.clone(),
                    passport_path: passport_path.display().to_string(),
                    message: format!("Failed to parse passport JSON: {err}"),
                }),
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                warnings.push(ExportWarning {
                    code: "passport_missing".to_string(),
                    spec_id: spec.spec.id.clone(),
                    passport_path: passport_path.display().to_string(),
                    message: format!("Passport file not found: {}", passport_path.display()),
                });
            }
            Err(err) => warnings.push(ExportWarning {
                code: "passport_malformed".to_string(),
                spec_id: spec.spec.id.clone(),
                passport_path: passport_path.display().to_string(),
                message: format!("Failed to read passport file: {err}"),
            }),
        }
    }

    (passports, warnings)
}

impl From<&LoadedSpec> for ExportUnit {
    fn from(spec: &LoadedSpec) -> Self {
        Self {
            id: spec.spec.id.clone(),
            intent: spec.spec.intent.why.clone(),
            contract: spec.spec.contract.clone(),
            deps: spec.spec.deps.clone(),
            local_tests: spec.spec.local_tests.clone(),
            source_file: spec.source.file_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::passport::{
        PassportEvidence, PassportTestResult, build_passport_with_evidence, write_passport,
    };
    use crate::types::{Body, Intent, SpecSource, SpecStruct};
    use indexmap::IndexMap;
    use tempfile::TempDir;

    fn loaded_spec(dir: &TempDir, rel_path: &str, id: &str, deps: Vec<&str>) -> LoadedSpec {
        let source_path = dir.path().join(rel_path);
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&source_path, "placeholder").unwrap();

        LoadedSpec {
            source: SpecSource {
                file_path: source_path.display().to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([("value".to_string(), "i32".to_string())])),
                    returns: Some("i32".to_string()),
                    invariants: vec![],
                }),
                deps: deps.into_iter().map(str::to_string).collect(),
                imports: vec![],
                body: Body {
                    rust: "{ value }".to_string(),
                },
                local_tests: vec![LocalTest {
                    id: "basic".to_string(),
                    expect: "true".to_string(),
                }],
                links: None,
                spec_version: Some("9.9.9".to_string()),
            },
        }
    }

    #[test]
    fn build_export_bundle_graph_edges_correct() {
        let dir = TempDir::new().unwrap();
        let spec_a = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec!["money/round", "money/format"],
        );
        let spec_b = loaded_spec(&dir, "units/money/round.unit.spec", "money/round", vec![]);

        let bundle = build_export_bundle(&[spec_a, spec_b], "2026-04-05T00:00:00Z");

        assert_eq!(
            bundle.graph.edges,
            vec![
                ExportEdge {
                    from: "pricing/apply_tax".to_string(),
                    to: "money/format".to_string(),
                },
                ExportEdge {
                    from: "pricing/apply_tax".to_string(),
                    to: "money/round".to_string(),
                },
            ]
        );
    }

    #[test]
    fn spec_export_schema_version_separate_from_spec_version() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec![],
        );

        let bundle = build_export_bundle(&[spec], "2026-04-05T00:00:00Z");

        assert_eq!(bundle.schema_version, "1.0");
        assert_eq!(bundle.spec_version, env!("CARGO_PKG_VERSION"));
        assert_ne!(bundle.schema_version, bundle.spec_version);
    }

    #[test]
    fn spec_export_malformed_passport_json_produces_warning_not_crash() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec![],
        );
        let passport_path = passport_path_for(Path::new(&spec.source.file_path)).unwrap();
        fs::write(&passport_path, "{\"id\":").unwrap();

        let (passports, warnings) = load_passports_for_specs(&[spec]);

        assert!(passports.is_empty());
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "passport_malformed");
        assert!(
            warnings[0]
                .message
                .contains("Failed to parse passport JSON")
        );
    }

    #[test]
    fn build_export_bundle_is_deterministic_for_edges_and_warnings() {
        let dir = TempDir::new().unwrap();
        let spec_a = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec!["money/round", "money/format"],
        );
        let spec_b = loaded_spec(
            &dir,
            "units/pricing/apply_discount.unit.spec",
            "pricing/apply_discount",
            vec!["money/round"],
        );

        let passport_b = build_passport_with_evidence(
            &spec_b,
            "2026-04-05T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-05T00:00:00Z".to_string(),
            }),
        );
        write_passport(&passport_b, Path::new(&spec_b.source.file_path)).unwrap();

        let bundle = build_export_bundle(&[spec_a.clone(), spec_b.clone()], "2026-04-05T01:00:00Z");

        assert_eq!(bundle.graph.edges[0].from, "pricing/apply_discount");
        assert_eq!(bundle.graph.edges[0].to, "money/round");
        assert_eq!(bundle.graph.edges[1].from, "pricing/apply_tax");
        assert_eq!(bundle.graph.edges[1].to, "money/format");
        assert_eq!(bundle.graph.edges[2].from, "pricing/apply_tax");
        assert_eq!(bundle.graph.edges[2].to, "money/round");
        assert_eq!(bundle.warnings.len(), 1);
        assert_eq!(bundle.warnings[0].spec_id, spec_a.spec.id);
        assert_eq!(bundle.warnings[0].code, "passport_missing");
        assert_eq!(bundle.passports.len(), 1);
        assert_eq!(bundle.passports[0].id, spec_b.spec.id);
    }
}
