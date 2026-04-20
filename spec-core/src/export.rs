//! JSON export support for loaded spec sets.
//!
//! The export bundle is a read-only artifact intended for downstream tooling.
//! It includes authored unit metadata, any readable co-located passports,
//! the dependency edge list, molecule tests, and structured warnings for skipped passports.
//!
//! # Breaking changes
//! - In M7, `ExportEdge` changed from a plain struct `{from, to}` to a tagged enum.
//! - In M9, dep refs changed from ambiguous strings to structured `{library, id}` objects.
//!
//! Consumers must handle the `kind` field: `"dep"` edges have structured `from`/`to` refs;
//! `"covers"` edges have `test`/`unit` string fields.

use crate::AUTHORED_SPEC_VERSION;
use crate::graph::{SpecEdge, SpecGraph};
use crate::passport::{ArtifactProvenance, Passport, passport_path_for};
use crate::plan::{LoadedPlan, PlanComputedImpact, PlanReport, PlanStruct};
use crate::types::{Contract, DepRef, LoadedMoleculeTest, LoadedSpec, LocalTest};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Export schema version. Bumped in M9 for structured dep refs.
const EXPORT_SCHEMA_VERSION: u8 = 3;
const PLAN_EXPORT_SCHEMA_VERSION: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportBundle {
    pub schema_version: u8,
    pub spec_version: String,
    pub exported_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ArtifactProvenance>,
    pub units: Vec<ExportUnit>,
    pub molecule_tests: Vec<ExportMoleculeTest>,
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
    pub deps: Vec<ExportDepRef>,
    pub local_tests: Vec<LocalTest>,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExportDepRef {
    pub library: Option<String>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportMoleculeTest {
    pub id: String,
    pub intent: String,
    pub covers: Vec<String>,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportGraph {
    pub edges: Vec<ExportEdge>,
}

/// An edge in the export graph.
///
/// Tagged with `kind` to distinguish dep edges from covers edges.
/// - `"dep"` edges have structured `from` and `to` refs (unit → dependency).
/// - `"covers"` edges have `test` and `unit` fields (molecule test → covered unit).
///
/// Breaking changes:
/// - M7: previously a plain struct `{from, to}`.
/// - M9: `from` and `to` changed from strings to `{library, id}` refs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ExportEdge {
    Dep {
        from: ExportDepRef,
        to: ExportDepRef,
    },
    Covers {
        test: String,
        unit: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportWarning {
    pub code: String,
    pub spec_id: String,
    pub passport_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanExportBundle {
    pub schema_version: u8,
    pub spec_version: String,
    pub exported_at: String,
    pub plan: PlanStruct,
    pub computed_impact: PlanComputedImpact,
    pub warnings: Vec<String>,
}

pub fn build_export_bundle(
    specs: &[LoadedSpec],
    molecule_tests: &[LoadedMoleculeTest],
    exported_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> ExportBundle {
    let (passports, warnings) = load_passports_for_specs(specs);

    // Project graph edges through the public SpecGraph surface.
    let graph = SpecGraph::build(specs, molecule_tests);
    let edges: Vec<ExportEdge> = graph
        .edges()
        .iter()
        .map(|edge| match edge {
            SpecEdge::Dep { from, to } => ExportEdge::Dep {
                from: ExportDepRef::local(from),
                to: ExportDepRef::from(to),
            },
            SpecEdge::Covers { test, unit } => ExportEdge::Covers {
                test: test.clone(),
                unit: unit.clone(),
            },
        })
        .collect();

    let export_molecule_tests: Vec<ExportMoleculeTest> = molecule_tests
        .iter()
        .map(ExportMoleculeTest::from)
        .collect();

    ExportBundle {
        schema_version: EXPORT_SCHEMA_VERSION,
        spec_version: AUTHORED_SPEC_VERSION.to_string(),
        exported_at: exported_at.to_string(),
        provenance: provenance.cloned(),
        units: specs.iter().map(ExportUnit::from).collect(),
        molecule_tests: export_molecule_tests,
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

pub fn build_plan_export_bundle(
    plan: &LoadedPlan,
    report: &PlanReport,
    exported_at: &str,
) -> PlanExportBundle {
    PlanExportBundle {
        schema_version: PLAN_EXPORT_SCHEMA_VERSION,
        spec_version: AUTHORED_SPEC_VERSION.to_string(),
        exported_at: exported_at.to_string(),
        plan: plan.plan.clone(),
        computed_impact: report.computed_impact.clone(),
        warnings: vec![],
    }
}

impl ExportDepRef {
    fn local(id: impl Into<String>) -> Self {
        Self {
            library: None,
            id: id.into(),
        }
    }
}

impl From<&DepRef> for ExportDepRef {
    fn from(dep: &DepRef) -> Self {
        Self {
            library: dep.library_alias().map(str::to_string),
            id: dep.unit_id().to_string(),
        }
    }
}

impl From<&LoadedSpec> for ExportUnit {
    fn from(spec: &LoadedSpec) -> Self {
        Self {
            id: spec.spec.id.clone(),
            intent: spec.spec.intent.why.clone(),
            contract: spec.spec.contract.clone(),
            deps: spec
                .spec
                .deps
                .iter()
                .map(|dep| {
                    let dep_ref = DepRef::parse(dep)
                        .expect("export assumes validated dep refs before projection");
                    ExportDepRef::from(&dep_ref)
                })
                .collect(),
            local_tests: spec.spec.local_tests.clone(),
            source_file: spec.source.file_path.clone(),
        }
    }
}

impl From<&LoadedMoleculeTest> for ExportMoleculeTest {
    fn from(test: &LoadedMoleculeTest) -> Self {
        Self {
            id: test.test.id.clone(),
            intent: test.test.intent.why.clone(),
            covers: test.test.covers.clone(),
            source_file: test.source.file_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::passport::{
        PassportEvidence, PassportTestResult, build_passport_with_evidence, write_passport,
    };
    use crate::plan::{
        LoadedPlan, PlanAcceptance, PlanChange, PlanChangeAction, PlanComputedImpact,
        PlanComputedImpactStatus, PlanReport, PlanSource, PlanStruct,
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
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    fn loaded_plan() -> LoadedPlan {
        LoadedPlan {
            source: PlanSource {
                file_path: "checkout-tax-refactor.plan.spec".to_string(),
                id: "checkout-tax-refactor".to_string(),
            },
            plan: PlanStruct {
                id: "checkout-tax-refactor".to_string(),
                intent: Intent {
                    why: "Refactor tax calculation.".to_string(),
                },
                changes: vec![PlanChange {
                    unit: "pricing/apply_tax".to_string(),
                    action: PlanChangeAction::Modify,
                    acceptance: PlanAcceptance {
                        validate: vec!["pricing/apply_tax".to_string()],
                        molecule_tests: vec!["pricing/checkout_flow".to_string()],
                        notes: vec![],
                    },
                }],
                notes: vec!["M10 plans are local-library only.".to_string()],
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
        let molecule_test = LoadedMoleculeTest {
            source: crate::types::MoleculeTestSource {
                file_path: dir
                    .path()
                    .join("tests/pricing/apply_tax.test.spec")
                    .display()
                    .to_string(),
                id: "pricing/apply_tax_behavior".to_string(),
            },
            test: crate::types::MoleculeTestStruct {
                id: "pricing/apply_tax_behavior".to_string(),
                intent: Intent {
                    why: "Why pricing/apply_tax_behavior".to_string(),
                },
                covers: vec!["money/round".to_string(), "pricing/apply_tax".to_string()],
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                },
                spec_version: None,
            },
        };

        let bundle = build_export_bundle(
            &[spec_a, spec_b],
            &[molecule_test],
            "2026-04-05T00:00:00Z",
            None,
        );

        assert_eq!(
            bundle.graph.edges,
            vec![
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/apply_tax"),
                    to: ExportDepRef::local("money/format"),
                },
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/apply_tax"),
                    to: ExportDepRef::local("money/round"),
                },
                ExportEdge::Covers {
                    test: "pricing/apply_tax_behavior".to_string(),
                    unit: "money/round".to_string(),
                },
                ExportEdge::Covers {
                    test: "pricing/apply_tax_behavior".to_string(),
                    unit: "pricing/apply_tax".to_string(),
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

        let bundle = build_export_bundle(&[spec], &[], "2026-04-05T00:00:00Z", None);

        assert_eq!(bundle.schema_version, 3);
        assert_eq!(bundle.spec_version, crate::AUTHORED_SPEC_VERSION);
        assert_ne!(bundle.schema_version.to_string(), bundle.spec_version);
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
                provenance: None,
            }),
            None,
        );
        write_passport(&passport_b, Path::new(&spec_b.source.file_path)).unwrap();

        let bundle = build_export_bundle(
            &[spec_a.clone(), spec_b.clone()],
            &[],
            "2026-04-05T01:00:00Z",
            None,
        );

        assert!(
            matches!(&bundle.graph.edges[0], ExportEdge::Dep { from, to } if from == &ExportDepRef::local("pricing/apply_discount") && to == &ExportDepRef::local("money/round"))
        );
        assert!(
            matches!(&bundle.graph.edges[1], ExportEdge::Dep { from, to } if from == &ExportDepRef::local("pricing/apply_tax") && to == &ExportDepRef::local("money/format"))
        );
        assert!(
            matches!(&bundle.graph.edges[2], ExportEdge::Dep { from, to } if from == &ExportDepRef::local("pricing/apply_tax") && to == &ExportDepRef::local("money/round"))
        );
        assert_eq!(bundle.warnings.len(), 1);
        assert_eq!(bundle.warnings[0].spec_id, spec_a.spec.id);
        assert_eq!(bundle.warnings[0].code, "passport_missing");
        assert_eq!(bundle.passports.len(), 1);
        assert_eq!(bundle.passports[0].id, spec_b.spec.id);
    }

    #[test]
    fn export_unit_and_graph_dep_refs_are_structured_in_schema_v3() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec!["shared::money/round", "money/format"],
        );

        let bundle = build_export_bundle(&[spec], &[], "2026-04-05T00:00:00Z", None);

        assert_eq!(
            bundle.units[0].deps,
            vec![
                ExportDepRef {
                    library: Some("shared".to_string()),
                    id: "money/round".to_string(),
                },
                ExportDepRef {
                    library: None,
                    id: "money/format".to_string(),
                },
            ]
        );
        assert_eq!(
            bundle.graph.edges,
            vec![
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/apply_tax"),
                    to: ExportDepRef {
                        library: None,
                        id: "money/format".to_string(),
                    },
                },
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/apply_tax"),
                    to: ExportDepRef {
                        library: Some("shared".to_string()),
                        id: "money/round".to_string(),
                    },
                },
            ]
        );
    }

    #[test]
    fn build_export_bundle_includes_top_level_provenance_when_present() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec![],
        );
        let provenance = ArtifactProvenance {
            git_commit_sha: "abc123".to_string(),
        };

        let bundle = build_export_bundle(&[spec], &[], "2026-04-05T00:00:00Z", Some(&provenance));

        assert_eq!(bundle.provenance, Some(provenance));
    }

    #[test]
    fn build_plan_export_bundle_uses_dedicated_schema_v1() {
        let plan = loaded_plan();
        let report = PlanReport {
            plan_id: plan.plan.id.clone(),
            computed_impact: PlanComputedImpact {
                status: PlanComputedImpactStatus::Complete,
                units: vec!["pricing/apply_tax".to_string()],
                molecule_tests: vec!["pricing/checkout_flow".to_string()],
                unresolved: vec![],
            },
            change_reports: vec![],
        };

        let bundle = build_plan_export_bundle(&plan, &report, "2026-04-17T00:00:00Z");

        assert_eq!(bundle.schema_version, 1);
        assert_eq!(bundle.spec_version, AUTHORED_SPEC_VERSION);
        assert_eq!(bundle.plan.id, "checkout-tax-refactor");
        assert_eq!(
            bundle.computed_impact.status,
            PlanComputedImpactStatus::Complete
        );
        assert!(bundle.warnings.is_empty());
    }
}
