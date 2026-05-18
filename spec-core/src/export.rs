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

use crate::benchmarks::{
    build_benchmark_molecule_truth_map, build_benchmark_unit_truth_map, project_benchmarks,
    BenchmarkProjection, BenchmarkProjectionInput, BenchmarkRegistry,
};
use crate::graph::{top_level_deps, SpecEdge, SpecGraph};
use crate::molecule_evidence::{read_molecule_evidence, MoleculeEvidence};
use crate::passport::{
    apply_projected_passport_truth, passport_path_for, project_passport_truth_with_context,
    refresh_passport_target_proofs, ArtifactProvenance, Passport, PassportProjectionContext,
};
use crate::plan::{LoadedPlan, PlanAcceptanceClosure, PlanComputedImpact, PlanReport, PlanStruct};
use crate::semantic_review::{SemanticProjectionMode, SemanticReviewContext};
use crate::types::{
    AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredMethod, AuthoredSumShape,
    Contract, DepRef, LoadedMoleculeTest, LoadedSpec, LocalTest, UnitKind,
};
use crate::Result;
use crate::AUTHORED_SPEC_VERSION;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benchmarks: Vec<BenchmarkProjection>,
    pub warnings: Vec<ExportWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportUnit {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AuthoredDataShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<AuthoredSumShape>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constructors: Vec<AuthoredConstructor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<AuthoredMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<AuthoredBackends>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<String>>,
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
    pub acceptance_closure: PlanAcceptanceClosure,
    pub warnings: Vec<String>,
}

pub struct ExportBenchmarkContext<'a> {
    pub registry: &'a BenchmarkRegistry,
    pub repo_root: &'a Path,
    pub scope_path: &'a Path,
}

pub fn build_export_bundle(
    specs: &[LoadedSpec],
    molecule_tests: &[LoadedMoleculeTest],
    exported_at: &str,
    provenance: Option<&ArtifactProvenance>,
) -> ExportBundle {
    build_export_bundle_with_benchmarks(specs, molecule_tests, exported_at, provenance, None)
        .expect("benchmark-less export bundle construction must not fail")
}

pub fn build_export_bundle_with_benchmarks(
    specs: &[LoadedSpec],
    molecule_tests: &[LoadedMoleculeTest],
    exported_at: &str,
    provenance: Option<&ArtifactProvenance>,
    benchmark_context: Option<&ExportBenchmarkContext<'_>>,
) -> Result<ExportBundle> {
    let (passports, warnings) = read_passports_for_specs(specs);
    let specs_by_id: HashMap<String, LoadedSpec> = specs
        .iter()
        .map(|spec| (spec.spec.id.clone(), spec.clone()))
        .collect();
    let molecule_evidence_by_id = load_molecule_evidence_for_tests(molecule_tests);
    let passports = enrich_passports_for_export(
        specs,
        molecule_tests,
        &molecule_evidence_by_id,
        &specs_by_id,
        passports,
    );
    let passports_by_id: HashMap<String, Passport> = passports
        .iter()
        .cloned()
        .map(|passport| (passport.id.clone(), passport))
        .collect();

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
    let benchmarks = if let Some(context) = benchmark_context {
        let unit_truth_by_id = build_benchmark_unit_truth_map(specs, &passports_by_id)?;
        let molecule_truth_by_id = build_benchmark_molecule_truth_map(
            molecule_tests,
            &molecule_evidence_by_id,
            &specs_by_id,
        )?;
        project_benchmarks(
            context.registry,
            BenchmarkProjectionInput {
                repo_root: context.repo_root,
                scope_path: context.scope_path,
                specs,
                molecule_tests,
                unit_truth_by_id: &unit_truth_by_id,
                molecule_truth_by_id: &molecule_truth_by_id,
            },
        )?
    } else {
        Vec::new()
    };

    Ok(ExportBundle {
        schema_version: EXPORT_SCHEMA_VERSION,
        spec_version: AUTHORED_SPEC_VERSION.to_string(),
        exported_at: exported_at.to_string(),
        provenance: provenance.cloned(),
        units: specs.iter().map(ExportUnit::from).collect(),
        molecule_tests: export_molecule_tests,
        passports,
        graph: ExportGraph { edges },
        benchmarks,
        warnings,
    })
}

fn enrich_passports_for_export(
    specs: &[LoadedSpec],
    molecule_tests: &[LoadedMoleculeTest],
    molecule_evidence_by_id: &HashMap<String, MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
    passports: Vec<Passport>,
) -> Vec<Passport> {
    let semantic_review_context = SemanticReviewContext::new(specs_by_id);
    let projection_context = PassportProjectionContext {
        molecule_tests,
        molecule_evidence_by_id,
        specs_by_id,
        semantic_projection_mode: SemanticProjectionMode::Preserve,
    };
    passports
        .into_iter()
        .map(|mut passport| {
            if let Some(spec) = specs.iter().find(|spec| spec.spec.id == passport.id) {
                refresh_passport_target_proofs(&mut passport, spec);
                let projected_truth = project_passport_truth_with_context(
                    spec,
                    Some(&passport),
                    &projection_context,
                    &semantic_review_context,
                );
                apply_projected_passport_truth(&mut passport, projected_truth);
            }
            passport
        })
        .collect()
}

fn load_molecule_evidence_for_tests(
    molecule_tests: &[LoadedMoleculeTest],
) -> HashMap<String, MoleculeEvidence> {
    molecule_tests
        .iter()
        .filter_map(|test| {
            read_molecule_evidence(Path::new(&test.source.file_path))
                .ok()
                .flatten()
                .map(|evidence| (test.test.id.clone(), evidence))
        })
        .collect()
}

fn read_passports_for_specs(specs: &[LoadedSpec]) -> (Vec<Passport>, Vec<ExportWarning>) {
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

pub fn load_passports_for_specs(specs: &[LoadedSpec]) -> (Vec<Passport>, Vec<ExportWarning>) {
    let (passports, warnings) = read_passports_for_specs(specs);
    let specs_by_id: HashMap<String, LoadedSpec> = specs
        .iter()
        .map(|spec| (spec.spec.id.clone(), spec.clone()))
        .collect();
    let semantic_review_context = SemanticReviewContext::new(&specs_by_id);
    let empty_molecule_tests: &[LoadedMoleculeTest] = &[];
    let empty_molecule_evidence: HashMap<String, MoleculeEvidence> = HashMap::new();
    let projection_context = PassportProjectionContext {
        molecule_tests: empty_molecule_tests,
        molecule_evidence_by_id: &empty_molecule_evidence,
        specs_by_id: &specs_by_id,
        semantic_projection_mode: SemanticProjectionMode::Preserve,
    };
    let passports = passports
        .into_iter()
        .map(|mut passport| {
            if let Some(spec) = specs.iter().find(|spec| spec.spec.id == passport.id) {
                let projected_truth = project_passport_truth_with_context(
                    spec,
                    Some(&passport),
                    &projection_context,
                    &semantic_review_context,
                );
                apply_projected_passport_truth(&mut passport, projected_truth);
            }
            passport
        })
        .collect();
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
        acceptance_closure: report.acceptance_closure.clone(),
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
        let is_seam = matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum));
        Self {
            id: spec.spec.id.clone(),
            kind: is_seam.then(|| spec.spec.kind.clone()),
            intent: spec.spec.intent.why.clone(),
            contract: spec.spec.contract.clone(),
            data: spec.spec.extensions.data.clone(),
            sum: spec.spec.extensions.sum.clone(),
            constructors: spec.spec.extensions.constructors.clone(),
            methods: spec.spec.extensions.methods.clone(),
            backends: spec.spec.extensions.backends.clone(),
            deps: top_level_deps(spec)
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
            imports: test.test.imports.clone(),
            source_file: test.source.file_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::{
        BenchmarkCaseLabel, BenchmarkClassification, BenchmarkKind, BenchmarkLabel,
        BenchmarkLifecycle, BenchmarkRegistry,
    };
    use crate::escape_hatch::{EscapeHatchGate, EscapeHatchGateStatus, EscapeHatchProofSurface};
    use crate::molecule_evidence::{
        build_molecule_evidence, write_molecule_evidence, MoleculeEvidenceStatus,
    };
    use crate::passport::{
        build_passport_with_evidence, write_passport, PassportEvidence, PassportTestResult,
        ProofSurface,
    };
    use crate::plan::{
        LoadedPlan, PlanAcceptance, PlanChange, PlanChangeAction, PlanComputedImpact,
        PlanComputedImpactStatus, PlanReport, PlanSource, PlanStruct,
    };
    use crate::semantic_review::{
        evaluate_semantic_review, evaluate_semantic_review_with_context, SemanticReviewContext,
    };
    use crate::types::{
        AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering, AuthoredSumShape,
        AuthoredSumVariant, Body, Intent, MoleculeTestSource, MoleculeTestStruct, SpecSource,
        SpecStruct, UnitExtensions,
    };
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
                    typescript: None,
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

    fn loaded_data_seam(dir: &TempDir, rel_path: &str, id: &str) -> LoadedSpec {
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
                kind: "data".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: vec!["legacy/ignored".to_string()],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![
                    LocalTest {
                        id: "discounted_subtotal_basic".to_string(),
                        expect: "CheckoutQuote::new(...).discounted_subtotal() == expected"
                            .to_string(),
                    },
                    LocalTest {
                        id: "total_basic".to_string(),
                        expect: "CheckoutQuote::new(...).total() == expected".to_string(),
                    },
                ],
                links: None,
                spec_version: Some("9.9.9".to_string()),
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
                                "discount_rate".to_string(),
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
                                ("discount_rate".to_string(), "Decimal".to_string()),
                                ("tax_rate".to_string(), "Decimal".to_string()),
                            ])),
                            returns: None,
                            invariants: vec![],
                        }),
                        initializes: IndexMap::from([
                            ("subtotal".to_string(), "subtotal".to_string()),
                            ("discount_rate".to_string(), "discount_rate".to_string()),
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
                                    body: "{ apply_discount(self.subtotal, self.discount_rate) }"
                                        .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Compute total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_tax".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body:
                                        "{ apply_tax(self.discounted_subtotal(), self.tax_rate) }"
                                            .to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec![
                                "Clone".to_string(),
                                "Debug".to_string(),
                                "PartialEq".to_string(),
                            ],
                        }),
                    }),
                    sum: None,
                },
            },
        }
    }

    fn loaded_supported_apply_discount_function(dir: &TempDir, rel_path: &str) -> LoadedSpec {
        let source_path = dir.path().join(rel_path);
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&source_path, "placeholder").unwrap();

        LoadedSpec {
            source: SpecSource {
                file_path: source_path.display().to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount to a subtotal while keeping the result nonnegative."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec!["output <= subtotal".to_string(), "output >= 0".to_string()],
                }),
                deps: vec!["money/round".to_string()],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: r#"{
    round((subtotal - subtotal * rate).max(Decimal::ZERO))
}"#
                    .to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)".to_string(),
                }],
                links: None,
                spec_version: Some("9.9.9".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn loaded_supported_apply_tax_function(dir: &TempDir, rel_path: &str) -> LoadedSpec {
        let source_path = dir.path().join(rel_path);
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&source_path, "placeholder").unwrap();

        LoadedSpec {
            source: SpecSource {
                file_path: source_path.display().to_string(),
                id: "pricing/apply_tax".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_tax".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply tax to a subtotal and round the resulting total.".to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec!["output >= subtotal".to_string()],
                }),
                deps: vec!["money/round".to_string()],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: r#"{
    round(subtotal + subtotal * rate)
}"#
                    .to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "apply_tax(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)".to_string(),
                }],
                links: None,
                spec_version: Some("9.9.9".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn loaded_supported_wrapper_pipeline_function(
        dir: &TempDir,
        rel_path: &str,
        id: &str,
    ) -> LoadedSpec {
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
                    why: "Return the total after discounting the subtotal and then applying tax."
                        .to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                        (
                            "discount_rate".to_string(),
                            "rust_decimal::Decimal".to_string(),
                        ),
                        ("tax_rate".to_string(), "rust_decimal::Decimal".to_string()),
                    ])),
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec![],
                }),
                deps: vec![
                    "pricing/apply_discount".to_string(),
                    "pricing/apply_tax".to_string(),
                ],
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: r#"{
    let discounted = apply_discount(subtotal, discount_rate);
    apply_tax(discounted, tax_rate)
}"#
                    .to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "true".to_string(),
                }],
                links: None,
                spec_version: Some("9.9.9".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn family_b_specs_by_id(specs: &[LoadedSpec]) -> HashMap<String, LoadedSpec> {
        specs
            .iter()
            .cloned()
            .map(|spec| (spec.spec.id.clone(), spec))
            .collect()
    }

    fn loaded_sum_seam(dir: &TempDir, rel_path: &str, id: &str) -> LoadedSpec {
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
                kind: "sum".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: vec!["legacy/ignored".to_string()],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![LocalTest {
                    id: "label_basic".to_string(),
                    expect: "CheckoutStatus::Pending.label() == \"pending\"".to_string(),
                }],
                links: None,
                spec_version: Some("9.9.9".to_string()),
                extensions: UnitExtensions {
                    data: None,
                    sum: Some(AuthoredSumShape {
                        variants: IndexMap::from([
                            (
                                "pending".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::new(),
                                },
                            ),
                            (
                                "quoted_total".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::from([
                                        (
                                            "subtotal".to_string(),
                                            AuthoredField {
                                                type_: "i32".to_string(),
                                            },
                                        ),
                                        (
                                            "tax_rate".to_string(),
                                            AuthoredField {
                                                type_: "i32".to_string(),
                                            },
                                        ),
                                    ]),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![],
                    methods: vec![
                        AuthoredMethod {
                            id: "label".to_string(),
                            intent: Intent {
                                why: "Return a variant label".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("&'static str".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ \"pending\" }".to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Return a computed total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("i32".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![
                                "pricing/apply_discount".to_string(),
                                "pricing/apply_tax".to_string(),
                            ],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ 0 }".to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec![
                                "Clone".to_string(),
                                "Debug".to_string(),
                                "PartialEq".to_string(),
                            ],
                        }),
                    }),
                },
            },
        }
    }

    fn loaded_supported_discount_strategy_sum_seam(
        dir: &TempDir,
        rel_path: &str,
        id: &str,
    ) -> LoadedSpec {
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
                kind: "sum".to_string(),
                intent: Intent {
                    why: "Represent discount strategies that cap fixed discounts at the subtotal."
                        .to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![
                    LocalTest {
                        id: "variant_none".to_string(),
                        expect: "true".to_string(),
                    },
                    LocalTest {
                        id: "variant_percentage".to_string(),
                        expect: "true".to_string(),
                    },
                    LocalTest {
                        id: "variant_fixed_amount".to_string(),
                        expect: "true".to_string(),
                    },
                    LocalTest {
                        id: "behavior_fixed_amount_capped".to_string(),
                        expect: "true".to_string(),
                    },
                ],
                links: None,
                spec_version: Some("9.9.9".to_string()),
                extensions: UnitExtensions {
                    data: None,
                    sum: Some(AuthoredSumShape {
                        variants: IndexMap::from([
                            ("none".to_string(), AuthoredSumVariant::default()),
                            (
                                "percentage".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::from([(
                                        "rate".to_string(),
                                        AuthoredField {
                                            type_: "Decimal".to_string(),
                                        },
                                    )]),
                                },
                            ),
                            (
                                "fixed_amount".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::from([(
                                        "amount".to_string(),
                                        AuthoredField {
                                            type_: "Decimal".to_string(),
                                        },
                                    )]),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![],
                    methods: vec![
                        AuthoredMethod {
                            id: "discount_amount".to_string(),
                            intent: Intent {
                                why: "Return the capped discount amount to subtract from the subtotal."
                                    .to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: Some(IndexMap::from([(
                                    "subtotal".to_string(),
                                    "Decimal".to_string(),
                                )])),
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: r#"{
    match self {
        Self::None => Decimal::ZERO,
        Self::Percentage { rate } => subtotal * *rate,
        Self::FixedAmount { amount } => (*amount).min(subtotal),
    }
}"#
                                    .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "discounted_subtotal".to_string(),
                            intent: Intent {
                                why: "Return the subtotal after applying the selected discount strategy."
                                    .to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: Some(IndexMap::from([(
                                    "subtotal".to_string(),
                                    "Decimal".to_string(),
                                )])),
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: r#"{
    subtotal - self.discount_amount(subtotal)
}"#
                                    .to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: None,
                },
            },
        }
    }

    fn loaded_discount_strategy_sum_seam(dir: &TempDir) -> LoadedSpec {
        let mut spec = loaded_sum_seam(
            dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );
        spec.spec.intent.why =
            "Represent mutually exclusive discount strategies for checkout pricing.".to_string();
        spec.spec.local_tests = [
            "variant_none",
            "variant_percentage",
            "variant_fixed_amount",
            "behavior_fixed_amount_capped",
        ]
        .into_iter()
        .map(|local_test_id| LocalTest {
            id: local_test_id.to_string(),
            expect: "true".to_string(),
        })
        .collect();
        spec
    }

    fn covering_molecule_test(dir: &TempDir, id: &str, cover_id: &str) -> LoadedMoleculeTest {
        let source_path = dir
            .path()
            .join("units/pricing/discount_strategy_checkout_flow.test.spec");
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&source_path, "placeholder").unwrap();

        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: source_path.display().to_string(),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: "cover the seam".to_string(),
                },
                covers: vec![cover_id.to_string()],
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: Some("0.3.0".to_string()),
            },
        }
    }

    fn proof_coverage_surfaces(passport: &Passport, coverage_id: &str) -> Vec<ProofSurface> {
        passport
            .proof_coverage
            .as_ref()
            .expect("expected proof coverage metadata")
            .iter()
            .find(|coverage| coverage.id == coverage_id)
            .expect("expected proof coverage entry")
            .surfaces
            .clone()
    }

    fn loaded_molecule_test(
        dir: &TempDir,
        rel_path: &str,
        id: &str,
        covers: Vec<&str>,
        imports: Option<Vec<&str>>,
    ) -> LoadedMoleculeTest {
        let source_path = dir.path().join(rel_path);
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&source_path, "placeholder").unwrap();

        LoadedMoleculeTest {
            source: crate::types::MoleculeTestSource {
                file_path: source_path.display().to_string(),
                id: id.to_string(),
            },
            test: crate::types::MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                covers: covers.into_iter().map(str::to_string).collect(),
                imports: imports.map(|values| values.into_iter().map(str::to_string).collect()),
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: None,
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
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
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
    fn build_export_bundle_with_benchmarks_adds_top_level_benchmark_projection() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_supported_apply_discount_function(
            &dir,
            "examples/ecommerce/units/pricing/apply_discount.unit.spec",
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-05-18T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "happy_path".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-05-18T00:00:00Z".to_string(),
                provenance: None,
            }),
            None,
        );
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let registry = BenchmarkRegistry {
            benchmarks: vec![BenchmarkLabel {
                id: "BENCH-ECOM".to_string(),
                kind: BenchmarkKind::Positive,
                lifecycle: BenchmarkLifecycle::Active,
                root: "examples/ecommerce/units/pricing".to_string(),
                generated_root: "examples/ecommerce/src/generated".to_string(),
                required_molecules: vec![],
                cases: vec![BenchmarkCaseLabel {
                    case_id: "discount".to_string(),
                    carrier_id: "pricing/apply_discount".to_string(),
                    classification: BenchmarkClassification::Supported,
                }],
            }],
        };

        let bundle = build_export_bundle_with_benchmarks(
            &[spec],
            &[],
            "2026-05-18T00:00:00Z",
            None,
            Some(&ExportBenchmarkContext {
                registry: &registry,
                repo_root: dir.path(),
                scope_path: &dir.path().join("examples/ecommerce"),
            }),
        )
        .unwrap();

        assert_eq!(bundle.schema_version, 3);
        assert_eq!(bundle.benchmarks.len(), 1);
        assert_eq!(bundle.benchmarks[0].id, "BENCH-ECOM");
        assert_eq!(bundle.benchmarks[0].cases.len(), 1);
    }

    #[test]
    fn export_molecule_test_preserves_non_empty_imports() {
        let dir = TempDir::new().unwrap();
        let molecule_test = loaded_molecule_test(
            &dir,
            "tests/pricing/checkout_flow.test.spec",
            "pricing/checkout_flow",
            vec!["pricing/apply_discount"],
            Some(vec![
                "rust_decimal::Decimal",
                "crate::pricing::apply_discount::apply_discount",
            ]),
        );

        let exported = ExportMoleculeTest::from(&molecule_test);

        assert_eq!(
            exported.imports,
            Some(vec![
                "rust_decimal::Decimal".to_string(),
                "crate::pricing::apply_discount::apply_discount".to_string(),
            ])
        );
    }

    #[test]
    fn export_molecule_test_preserves_explicit_empty_imports() {
        let dir = TempDir::new().unwrap();
        let molecule_test = loaded_molecule_test(
            &dir,
            "tests/pricing/checkout_flow.test.spec",
            "pricing/checkout_flow",
            vec!["pricing/apply_discount"],
            Some(vec![]),
        );

        let exported = ExportMoleculeTest::from(&molecule_test);
        let json = serde_json::to_value(&exported).unwrap();

        assert_eq!(exported.imports, Some(vec![]));
        assert_eq!(json["imports"], serde_json::json!([]));
    }

    #[test]
    fn export_molecule_test_omits_missing_imports() {
        let dir = TempDir::new().unwrap();
        let molecule_test = loaded_molecule_test(
            &dir,
            "tests/pricing/checkout_flow.test.spec",
            "pricing/checkout_flow",
            vec!["pricing/apply_discount"],
            None,
        );

        let exported = ExportMoleculeTest::from(&molecule_test);
        let json = serde_json::to_value(&exported).unwrap();

        assert_eq!(exported.imports, None);
        assert!(json.get("imports").is_none(), "{json}");
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
        assert!(warnings[0]
            .message
            .contains("Failed to parse passport JSON"));
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
    fn load_passports_for_specs_uses_legacy_contract_hash_freshness() {
        let dir = TempDir::new().unwrap();
        let original_spec = loaded_spec(
            &dir,
            "units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
            vec![],
        );
        let mut changed_spec = original_spec.clone();
        changed_spec.spec.contract.as_mut().unwrap().returns = Some("i64".to_string());

        let mut legacy_passport = build_passport_with_evidence(
            &original_spec,
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
            crate::passport::compute_contract_hash(&original_spec),
        );
        legacy_passport.freshness = None;
        write_passport(&legacy_passport, Path::new(&original_spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&[changed_spec.clone()]);

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        let freshness = passports[0]
            .freshness
            .as_ref()
            .expect("export should project freshness");
        assert_eq!(
            freshness.authored_truth_status,
            crate::passport::FreshnessStatus::Stale
        );
        assert_eq!(
            freshness.backend_execution_status,
            crate::passport::FreshnessStatus::Unknown
        );
        assert_eq!(
            freshness.snapshot.authored_truth_digest,
            crate::passport::compute_authored_truth_digest(&changed_spec)
        );
    }

    #[test]
    fn load_passports_for_specs_drops_stale_sum_review_on_unsupported_kind() {
        let dir = TempDir::new().unwrap();
        let original_spec = loaded_sum_seam(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );
        let mut changed_spec = loaded_spec(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
            vec![],
        );
        changed_spec.spec.intent.why = "Apply a function-style discount".to_string();

        let mut passport = build_passport_with_evidence(
            &original_spec,
            "2026-04-05T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "label_basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-05T00:00:00Z".to_string(),
                provenance: None,
            }),
            None,
        );
        passport.semantic_review = evaluate_semantic_review(&original_spec);
        write_passport(&passport, Path::new(&original_spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&[changed_spec]);

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert!(passports[0].semantic_review.is_none());
    }

    #[test]
    fn load_passports_for_specs_preserve_matching_data_compatibility_key() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );
        let supported_review =
            evaluate_semantic_review(&spec).expect("supported data review expected after Lane A");
        assert_eq!(supported_review.compatibility_key, "data.pricing_quote.v1");

        let mut passport = build_passport_with_evidence(
            &spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&spec),
        );
        passport.semantic_review = Some(supported_review.clone());
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(std::slice::from_ref(&spec));

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert_eq!(passports[0].semantic_review, Some(supported_review));
    }

    #[test]
    fn load_passports_for_specs_preserve_drops_removed_legacy_seam_compatibility_keys() {
        let dir = TempDir::new().unwrap();
        let sum_spec = loaded_supported_discount_strategy_sum_seam(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );
        let data_spec = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );

        let mut sum_review =
            evaluate_semantic_review(&sum_spec).expect("supported sum review expected");
        assert_eq!(sum_review.compatibility_key, "sum.discount_strategy.v1");
        sum_review.compatibility_key = "sum.discount_policy.v1".to_string();

        let mut data_review =
            evaluate_semantic_review(&data_spec).expect("supported data review expected");
        assert_eq!(data_review.compatibility_key, "data.pricing_quote.v1");
        data_review.compatibility_key = "data.checkout_quote.v1".to_string();

        let mut sum_passport = build_passport_with_evidence(
            &sum_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: sum_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&sum_spec),
        );
        sum_passport.semantic_review = Some(sum_review.clone());
        write_passport(&sum_passport, Path::new(&sum_spec.source.file_path)).unwrap();

        let mut data_passport = build_passport_with_evidence(
            &data_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: data_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&data_spec),
        );
        data_passport.semantic_review = Some(data_review.clone());
        write_passport(&data_passport, Path::new(&data_spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&[sum_spec, data_spec]);

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 2);
        assert!(passports
            .iter()
            .find(|passport| passport.id == "pricing/discount_strategy")
            .expect("sum passport")
            .semantic_review
            .is_none());
        assert!(passports
            .iter()
            .find(|passport| passport.id == "pricing/pricing_quote")
            .expect("data passport")
            .semantic_review
            .is_none());
    }

    #[test]
    fn load_passports_for_specs_preserve_drops_cross_family_legacy_seam_compatibility_keys() {
        let dir = TempDir::new().unwrap();
        let sum_spec = loaded_supported_discount_strategy_sum_seam(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );
        let data_spec = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );

        let mut sum_review =
            evaluate_semantic_review(&sum_spec).expect("supported sum review expected");
        sum_review.compatibility_key = "data.checkout_quote.v1".to_string();

        let mut data_review =
            evaluate_semantic_review(&data_spec).expect("supported data review expected");
        data_review.compatibility_key = "sum.discount_policy.v1".to_string();

        let mut sum_passport = build_passport_with_evidence(
            &sum_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: sum_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&sum_spec),
        );
        sum_passport.semantic_review = Some(sum_review);
        write_passport(&sum_passport, Path::new(&sum_spec.source.file_path)).unwrap();

        let mut data_passport = build_passport_with_evidence(
            &data_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: data_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&data_spec),
        );
        data_passport.semantic_review = Some(data_review);
        write_passport(&data_passport, Path::new(&data_spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&[sum_spec, data_spec]);

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 2);
        assert!(passports
            .iter()
            .find(|passport| passport.id == "pricing/discount_strategy")
            .expect("sum passport")
            .semantic_review
            .is_none());
        assert!(passports
            .iter()
            .find(|passport| passport.id == "pricing/pricing_quote")
            .expect("data passport")
            .semantic_review
            .is_none());
    }

    #[test]
    fn load_passports_for_specs_preserves_matching_supported_function_compatibility_key() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_supported_apply_discount_function(
            &dir,
            "units/pricing/apply_discount.unit.spec",
        );
        let Some(supported_review) = evaluate_semantic_review(&spec)
            .filter(|review| review.compatibility_key != "unsupported.function.v1")
        else {
            return;
        };

        let mut passport = build_passport_with_evidence(
            &spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&spec),
        );
        passport.semantic_review = Some(supported_review.clone());
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(std::slice::from_ref(&spec));

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert_eq!(passports[0].semantic_review, Some(supported_review));
    }

    #[test]
    fn load_passports_for_specs_preserve_keeps_matching_supported_family_key() {
        let dir = TempDir::new().unwrap();
        let apply_discount = loaded_supported_apply_discount_function(
            &dir,
            "units/pricing/apply_discount.unit.spec",
        );
        let apply_tax =
            loaded_supported_apply_tax_function(&dir, "units/pricing/apply_tax.unit.spec");
        let wrapper = loaded_supported_wrapper_pipeline_function(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
        );
        let specs = vec![apply_discount.clone(), apply_tax.clone(), wrapper.clone()];
        let specs_by_id = family_b_specs_by_id(&specs);
        let semantic_review_context = SemanticReviewContext::new(&specs_by_id);
        let supported_review =
            evaluate_semantic_review_with_context(&wrapper, &semantic_review_context)
                .expect("supported wrapper family review expected");
        assert_eq!(
            supported_review.compatibility_key,
            "function.wrapper.pipeline.v1"
        );

        for spec in [&apply_discount, &apply_tax] {
            let passport = build_passport_with_evidence(
                spec,
                "2026-04-23T00:00:00Z",
                Some(PassportEvidence {
                    build_status: "pass".to_string(),
                    test_results: spec
                        .spec
                        .local_tests
                        .iter()
                        .map(|local_test| PassportTestResult {
                            id: local_test.id.clone(),
                            status: "pass".to_string(),
                            reason: None,
                        })
                        .collect(),
                    observed_at: "2026-04-23T00:00:00Z".to_string(),
                    provenance: None,
                }),
                crate::passport::compute_contract_hash(spec),
            );
            write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();
        }

        let mut wrapper_passport = build_passport_with_evidence(
            &wrapper,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: wrapper
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&wrapper),
        );
        wrapper_passport.semantic_review = Some(supported_review.clone());
        write_passport(&wrapper_passport, Path::new(&wrapper.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&specs);
        let wrapper_passport = passports
            .iter()
            .find(|passport| passport.id == wrapper.spec.id)
            .expect("wrapper passport");

        assert!(warnings.is_empty());
        assert_eq!(wrapper_passport.semantic_review, Some(supported_review));
    }

    #[test]
    fn load_passports_for_specs_preserve_drops_mismatched_old_exact_id_review() {
        let dir = TempDir::new().unwrap();
        let apply_discount = loaded_supported_apply_discount_function(
            &dir,
            "units/pricing/apply_discount.unit.spec",
        );
        let apply_tax =
            loaded_supported_apply_tax_function(&dir, "units/pricing/apply_tax.unit.spec");
        let wrapper = loaded_supported_wrapper_pipeline_function(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
        );
        let specs = vec![apply_discount.clone(), apply_tax.clone(), wrapper.clone()];
        let specs_by_id = family_b_specs_by_id(&specs);
        let semantic_review_context = SemanticReviewContext::new(&specs_by_id);
        let mut supported_review =
            evaluate_semantic_review_with_context(&wrapper, &semantic_review_context)
                .expect("supported wrapper family review expected");
        supported_review.compatibility_key = wrapper.spec.id.clone();

        for spec in [&apply_discount, &apply_tax] {
            let passport = build_passport_with_evidence(
                spec,
                "2026-04-23T00:00:00Z",
                Some(PassportEvidence {
                    build_status: "pass".to_string(),
                    test_results: spec
                        .spec
                        .local_tests
                        .iter()
                        .map(|local_test| PassportTestResult {
                            id: local_test.id.clone(),
                            status: "pass".to_string(),
                            reason: None,
                        })
                        .collect(),
                    observed_at: "2026-04-23T00:00:00Z".to_string(),
                    provenance: None,
                }),
                crate::passport::compute_contract_hash(spec),
            );
            write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();
        }

        let mut wrapper_passport = build_passport_with_evidence(
            &wrapper,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: wrapper
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&wrapper),
        );
        wrapper_passport.semantic_review = Some(supported_review);
        write_passport(&wrapper_passport, Path::new(&wrapper.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&specs);
        let wrapper_passport = passports
            .iter()
            .find(|passport| passport.id == wrapper.spec.id)
            .expect("wrapper passport");

        assert!(warnings.is_empty());
        assert!(wrapper_passport.semantic_review.is_none());
    }

    #[test]
    fn load_passports_for_specs_preserve_does_not_promote_unsupported_additive_review_into_supported_family_truth(
    ) {
        let dir = TempDir::new().unwrap();
        let apply_discount = loaded_supported_apply_discount_function(
            &dir,
            "units/pricing/apply_discount.unit.spec",
        );
        let apply_tax =
            loaded_supported_apply_tax_function(&dir, "units/pricing/apply_tax.unit.spec");
        let wrapper = loaded_supported_wrapper_pipeline_function(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
        );
        let specs = vec![apply_discount.clone(), apply_tax.clone(), wrapper.clone()];

        for spec in [&apply_discount, &apply_tax] {
            let passport = build_passport_with_evidence(
                spec,
                "2026-04-23T00:00:00Z",
                Some(PassportEvidence {
                    build_status: "pass".to_string(),
                    test_results: spec
                        .spec
                        .local_tests
                        .iter()
                        .map(|local_test| PassportTestResult {
                            id: local_test.id.clone(),
                            status: "pass".to_string(),
                            reason: None,
                        })
                        .collect(),
                    observed_at: "2026-04-23T00:00:00Z".to_string(),
                    provenance: None,
                }),
                crate::passport::compute_contract_hash(spec),
            );
            write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();
        }

        let mut wrapper_passport = build_passport_with_evidence(
            &wrapper,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: wrapper
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&wrapper),
        );
        wrapper_passport.semantic_review = evaluate_semantic_review(&wrapper);
        assert_eq!(
            wrapper_passport
                .semantic_review
                .as_ref()
                .map(|review| review.compatibility_key.as_str()),
            Some("unsupported.function.v1")
        );
        write_passport(&wrapper_passport, Path::new(&wrapper.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(&specs);
        let wrapper_passport = passports
            .iter()
            .find(|passport| passport.id == wrapper.spec.id)
            .expect("wrapper passport");

        assert!(warnings.is_empty());
        assert!(wrapper_passport.semantic_review.is_none());
    }

    #[test]
    fn load_passports_for_specs_drops_supported_function_review_for_unsupported_function() {
        let dir = TempDir::new().unwrap();
        let supported_spec = loaded_supported_apply_discount_function(
            &dir,
            "units/pricing/apply_discount.unit.spec",
        );
        let Some(supported_review) = evaluate_semantic_review(&supported_spec)
            .filter(|review| review.compatibility_key != "unsupported.function.v1")
        else {
            return;
        };
        let spec = loaded_spec(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
            vec![],
        );

        let mut passport = build_passport_with_evidence(
            &supported_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: supported_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&supported_spec),
        );
        passport.id = spec.spec.id.clone();
        passport.source_file = spec.source.file_path.clone();
        passport.semantic_review = Some(supported_review);
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(std::slice::from_ref(&spec));

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert!(passports[0].semantic_review.is_none());
    }

    #[test]
    fn load_passports_for_specs_preserves_fresh_unsupported_function_review() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
            vec![],
        );

        let mut passport = build_passport_with_evidence(
            &spec,
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
        passport.semantic_review = evaluate_semantic_review(&spec);
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(std::slice::from_ref(&spec));

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert_eq!(passports[0].semantic_review, passport.semantic_review);
    }

    #[test]
    fn load_passports_for_specs_does_not_invent_unsupported_review_metadata() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
            vec![],
        );

        let passport = build_passport_with_evidence(
            &spec,
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
        assert!(passport.semantic_review.is_none());
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let (passports, warnings) = load_passports_for_specs(std::slice::from_ref(&spec));

        assert!(warnings.is_empty());
        assert_eq!(passports.len(), 1);
        assert!(passports[0].semantic_review.is_none());
    }

    #[test]
    fn build_export_bundle_does_not_invent_supported_data_semantic_review_on_preserve() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );

        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&spec),
        );
        assert!(passport.semantic_review.is_none());
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let bundle = build_export_bundle(&[spec], &[], "2026-04-23T01:00:00Z", None);

        assert!(bundle.warnings.is_empty());
        assert_eq!(bundle.passports.len(), 1);
        assert!(bundle.passports[0].semantic_review.is_none());
    }

    #[test]
    fn build_export_bundle_does_not_invent_supported_sum_semantic_review_on_preserve() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_supported_discount_strategy_sum_seam(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );

        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&spec),
        );
        assert!(passport.semantic_review.is_none());
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let bundle = build_export_bundle(&[spec], &[], "2026-04-23T01:00:00Z", None);

        assert!(bundle.warnings.is_empty());
        assert_eq!(bundle.passports.len(), 1);
        assert!(bundle.passports[0].semantic_review.is_none());
    }

    #[test]
    fn refresh_projection_reads_legacy_seam_reviews_back_canonically() {
        let dir = TempDir::new().unwrap();
        let sum_spec = loaded_supported_discount_strategy_sum_seam(
            &dir,
            "units/pricing/discount_strategy.unit.spec",
            "pricing/discount_strategy",
        );
        let data_spec = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );
        let specs_by_id = HashMap::from([
            (sum_spec.spec.id.clone(), sum_spec.clone()),
            (data_spec.spec.id.clone(), data_spec.clone()),
        ]);
        let semantic_review_context = SemanticReviewContext::new(&specs_by_id);
        let empty_molecule_tests: &[LoadedMoleculeTest] = &[];
        let empty_molecule_evidence: HashMap<String, MoleculeEvidence> = HashMap::new();
        let projection_context = PassportProjectionContext {
            molecule_tests: empty_molecule_tests,
            molecule_evidence_by_id: &empty_molecule_evidence,
            specs_by_id: &specs_by_id,
            semantic_projection_mode: SemanticProjectionMode::Refresh,
        };

        let mut sum_review =
            evaluate_semantic_review(&sum_spec).expect("supported sum review expected");
        sum_review.compatibility_key = "sum.discount_policy.v1".to_string();
        let mut sum_passport = build_passport_with_evidence(
            &sum_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: sum_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&sum_spec),
        );
        sum_passport.semantic_review = Some(sum_review);

        let mut data_review =
            evaluate_semantic_review(&data_spec).expect("supported data review expected");
        data_review.compatibility_key = "data.checkout_quote.v1".to_string();
        let mut data_passport = build_passport_with_evidence(
            &data_spec,
            "2026-04-23T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: data_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-23T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&data_spec),
        );
        data_passport.semantic_review = Some(data_review);

        let sum_projected = project_passport_truth_with_context(
            &sum_spec,
            Some(&sum_passport),
            &projection_context,
            &semantic_review_context,
        );
        apply_projected_passport_truth(&mut sum_passport, sum_projected);

        let data_projected = project_passport_truth_with_context(
            &data_spec,
            Some(&data_passport),
            &projection_context,
            &semantic_review_context,
        );
        apply_projected_passport_truth(&mut data_passport, data_projected);

        assert_eq!(
            sum_passport
                .semantic_review
                .as_ref()
                .map(|review| review.compatibility_key.as_str()),
            Some("sum.discount_strategy.v1")
        );
        assert_eq!(
            data_passport
                .semantic_review
                .as_ref()
                .map(|review| review.compatibility_key.as_str()),
            Some("data.pricing_quote.v1")
        );
    }

    #[test]
    fn build_export_bundle_preserves_fresh_unsupported_function_review() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_spec(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
            vec![],
        );

        let mut passport = build_passport_with_evidence(
            &spec,
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
        passport.semantic_review = evaluate_semantic_review(&spec);
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let bundle = build_export_bundle(&[spec], &[], "2026-04-05T01:00:00Z", None);

        assert!(bundle.warnings.is_empty());
        assert_eq!(bundle.passports.len(), 1);
        assert_eq!(
            bundle.passports[0].semantic_review,
            passport.semantic_review
        );
    }

    #[test]
    fn build_export_bundle_drops_stale_unsupported_function_review() {
        let dir = TempDir::new().unwrap();
        let original_spec = loaded_spec(
            &dir,
            "units/pricing/calculate_total.unit.spec",
            "pricing/calculate_total",
            vec![],
        );
        let mut changed_spec = original_spec.clone();
        changed_spec.spec.intent.why = "Apply a revised checkout pricing flow".to_string();

        let mut passport = build_passport_with_evidence(
            &original_spec,
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
        passport.semantic_review = evaluate_semantic_review(&original_spec);
        write_passport(&passport, Path::new(&original_spec.source.file_path)).unwrap();

        let bundle = build_export_bundle(&[changed_spec], &[], "2026-04-05T01:00:00Z", None);

        assert!(bundle.warnings.is_empty());
        assert_eq!(bundle.passports.len(), 1);
        assert!(bundle.passports[0].semantic_review.is_none());
        assert_eq!(
            bundle.passports[0]
                .freshness
                .as_ref()
                .map(|freshness| freshness.authored_truth_status),
            Some(crate::passport::FreshnessStatus::Stale)
        );
    }

    #[test]
    fn export_recomputes_escape_hatch_gate_from_current_evidence() {
        let dir = TempDir::new().unwrap();
        let spec = loaded_discount_strategy_sum_seam(&dir);
        let molecule_test = covering_molecule_test(
            &dir,
            "pricing/discount_strategy_checkout_flow",
            "pricing/discount_strategy",
        );
        let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);

        let mut passport = build_passport_with_evidence(
            &spec,
            "2026-04-21T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-21T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&spec),
        );
        passport.escape_hatch_gate = Some(EscapeHatchGate {
            status: EscapeHatchGateStatus::Open,
            required_surfaces: vec![
                EscapeHatchProofSurface::Atom,
                EscapeHatchProofSurface::Molecule,
            ],
            present_surfaces: vec![EscapeHatchProofSurface::Atom],
            missing_surfaces: vec![EscapeHatchProofSurface::Molecule],
            reason: Some("missing required escape-hatch proof: molecule".to_string()),
        });
        write_passport(&passport, Path::new(&spec.source.file_path)).unwrap();

        let molecule_evidence = build_molecule_evidence(
            &molecule_test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &specs_by_id,
            None,
        );
        write_molecule_evidence(
            &molecule_evidence,
            Path::new(&molecule_test.source.file_path),
        )
        .unwrap();

        let bundle = build_export_bundle(
            std::slice::from_ref(&spec),
            std::slice::from_ref(&molecule_test),
            "2026-04-21T00:00:00Z",
            None,
        );
        let gate = bundle.passports[0]
            .escape_hatch_gate
            .as_ref()
            .expect("marked seam should project a gate");

        assert_eq!(gate.status, EscapeHatchGateStatus::Closed);
        assert_eq!(
            gate.present_surfaces,
            vec![
                EscapeHatchProofSurface::Atom,
                EscapeHatchProofSurface::Molecule,
            ]
        );
        assert!(gate.missing_surfaces.is_empty());
        assert_eq!(gate.reason, None);
        assert_eq!(
            proof_coverage_surfaces(&bundle.passports[0], "variant.none"),
            vec![ProofSurface::Atom, ProofSurface::Molecule]
        );
    }

    #[test]
    fn export_reprojects_stale_branch_proof_coverage_from_current_surfaces() {
        let dir = TempDir::new().unwrap();
        let original_spec = loaded_discount_strategy_sum_seam(&dir);
        let mut changed_spec = original_spec.clone();
        changed_spec.spec.intent.why = "Represent revised discount policy".to_string();

        let passport = build_passport_with_evidence(
            &original_spec,
            "2026-04-21T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: original_spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|local_test| PassportTestResult {
                        id: local_test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-04-21T00:00:00Z".to_string(),
                provenance: None,
            }),
            crate::passport::compute_contract_hash(&original_spec),
        );
        write_passport(&passport, Path::new(&original_spec.source.file_path)).unwrap();

        let bundle = build_export_bundle(
            std::slice::from_ref(&changed_spec),
            &[],
            "2026-04-21T00:00:00Z",
            None,
        );
        let exported = &bundle.passports[0];
        let gate = exported
            .escape_hatch_gate
            .as_ref()
            .expect("marked seam should project a gate");

        assert_eq!(gate.status, EscapeHatchGateStatus::Open);
        assert!(gate.present_surfaces.is_empty());
        assert_eq!(
            proof_coverage_surfaces(exported, "variant.none"),
            vec![ProofSurface::ImplicitOnly]
        );
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
    fn build_export_bundle_additively_includes_data_seam_truth() {
        let dir = TempDir::new().unwrap();
        let seam = loaded_data_seam(
            &dir,
            "units/pricing/pricing_quote.unit.spec",
            "pricing/pricing_quote",
        );

        let bundle = build_export_bundle(&[seam], &[], "2026-04-19T00:00:00Z", None);

        assert_eq!(bundle.units.len(), 1);
        assert_eq!(bundle.units[0].kind, Some("data".to_string()));
        assert!(bundle.units[0].contract.is_none());
        assert_eq!(bundle.units[0].data.as_ref().unwrap().fields.len(), 3);
        assert_eq!(bundle.units[0].constructors.len(), 1);
        assert_eq!(bundle.units[0].methods.len(), 2);
        assert_eq!(
            bundle.units[0].deps,
            vec![
                ExportDepRef::local("pricing/apply_discount"),
                ExportDepRef::local("pricing/apply_tax"),
            ]
        );
        assert_eq!(
            bundle.graph.edges,
            vec![
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/pricing_quote"),
                    to: ExportDepRef::local("pricing/apply_discount"),
                },
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/pricing_quote"),
                    to: ExportDepRef::local("pricing/apply_tax"),
                },
            ]
        );
    }

    #[test]
    fn build_export_bundle_additively_includes_sum_seam_truth() {
        let dir = TempDir::new().unwrap();
        let seam = loaded_sum_seam(
            &dir,
            "units/pricing/checkout_status.unit.spec",
            "pricing/checkout_status",
        );

        let bundle = build_export_bundle(&[seam], &[], "2026-04-19T00:00:00Z", None);

        assert_eq!(bundle.units.len(), 1);
        assert_eq!(bundle.units[0].kind, Some("sum".to_string()));
        assert!(bundle.units[0].contract.is_none());
        let variants = &bundle.units[0].sum.as_ref().unwrap().variants;
        assert_eq!(
            variants.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["pending", "quoted_total"]
        );
        assert_eq!(variants["quoted_total"].fields["subtotal"].type_, "i32");
        assert_eq!(bundle.units[0].constructors.len(), 0);
        assert_eq!(bundle.units[0].methods.len(), 2);
        assert_eq!(
            bundle.units[0].deps,
            vec![
                ExportDepRef::local("pricing/apply_discount"),
                ExportDepRef::local("pricing/apply_tax"),
            ]
        );
        assert_eq!(
            bundle.units[0]
                .backends
                .as_ref()
                .unwrap()
                .rust
                .as_ref()
                .unwrap()
                .derives,
            vec![
                "Clone".to_string(),
                "Debug".to_string(),
                "PartialEq".to_string(),
            ]
        );
        assert_eq!(
            bundle.graph.edges,
            vec![
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/checkout_status"),
                    to: ExportDepRef::local("pricing/apply_discount"),
                },
                ExportEdge::Dep {
                    from: ExportDepRef::local("pricing/checkout_status"),
                    to: ExportDepRef::local("pricing/apply_tax"),
                },
            ]
        );
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
            acceptance_closure: crate::plan::PlanAcceptanceClosure {
                status: crate::plan::PlanAcceptanceClosureStatus::Closed,
                missing_validate: vec![],
                missing_molecule_tests: vec![],
                extra_validate: vec![],
                extra_molecule_tests: vec![],
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
        assert_eq!(
            bundle.acceptance_closure.status,
            crate::plan::PlanAcceptanceClosureStatus::Closed
        );
        assert!(bundle.warnings.is_empty());
    }
}
