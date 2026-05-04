use crate::molecule_evidence::{MoleculeEvidence, molecule_evidence_is_current_pass};
use crate::passport::{
    FreshnessStatus, Passport, compute_passport_markers, resolve_passport_freshness,
};
use crate::types::{LoadedMoleculeTest, LoadedSpec, UnitKind};
#[cfg(test)]
use crate::{
    backend_execution::{
        BackendExecutionMarkerKind, collect_backend_execution_markers,
        is_helper_or_example_method as backend_is_helper_or_example_method,
        summarize_backend_execution_markers,
    },
    types::AuthoredMethod,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EscapeHatchGateStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum EscapeHatchProofSurface {
    Atom,
    Molecule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EscapeHatchGate {
    pub status: EscapeHatchGateStatus,
    pub required_surfaces: Vec<EscapeHatchProofSurface>,
    pub present_surfaces: Vec<EscapeHatchProofSurface>,
    pub missing_surfaces: Vec<EscapeHatchProofSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CurrentProofSurfaces {
    pub atom: bool,
    pub molecule: bool,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EscapeHatchSemanticMarkerKind {
    DomainLowering,
    ProofHelperLowering,
    BackendRustDerives,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EscapeHatchSemanticMarker {
    pub kind: EscapeHatchSemanticMarkerKind,
    pub path: String,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct EscapeHatchSemanticMarkerSummary {
    pub has_domain_lowering: bool,
    pub has_proof_helper_lowering: bool,
    pub has_backend_rust_derives: bool,
}

pub fn evaluate_escape_hatch_gate(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
    molecule_tests: &[LoadedMoleculeTest],
    molecule_evidence_by_id: &HashMap<String, MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Option<EscapeHatchGate> {
    if !matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum)) {
        return None;
    }
    compute_passport_markers(spec).as_ref()?;

    let required_surfaces = required_surfaces();
    let mut present_surfaces = Vec::new();
    let current_surfaces = current_proof_surfaces(
        spec,
        passport,
        molecule_tests,
        molecule_evidence_by_id,
        specs_by_id,
    );

    if current_surfaces.atom {
        present_surfaces.push(EscapeHatchProofSurface::Atom);
    }
    if current_surfaces.molecule {
        present_surfaces.push(EscapeHatchProofSurface::Molecule);
    }

    let missing_surfaces = required_surfaces
        .iter()
        .copied()
        .filter(|surface| !present_surfaces.contains(surface))
        .collect::<Vec<_>>();
    let status = if missing_surfaces.is_empty() {
        EscapeHatchGateStatus::Closed
    } else {
        EscapeHatchGateStatus::Open
    };

    Some(EscapeHatchGate {
        status,
        required_surfaces,
        present_surfaces,
        reason: format_open_reason(&missing_surfaces),
        missing_surfaces,
    })
}

#[cfg(test)]
pub(crate) fn collect_escape_hatch_semantic_markers(
    spec: &LoadedSpec,
) -> Vec<EscapeHatchSemanticMarker> {
    collect_backend_execution_markers(spec)
        .into_iter()
        .map(|marker| EscapeHatchSemanticMarker {
            kind: match marker.kind {
                BackendExecutionMarkerKind::DomainLowering => {
                    EscapeHatchSemanticMarkerKind::DomainLowering
                }
                BackendExecutionMarkerKind::ProofHelperLowering => {
                    EscapeHatchSemanticMarkerKind::ProofHelperLowering
                }
                BackendExecutionMarkerKind::BackendRustDerives => {
                    EscapeHatchSemanticMarkerKind::BackendRustDerives
                }
            },
            path: marker.path,
        })
        .collect()
}

#[cfg(test)]
pub(crate) fn summarize_escape_hatch_semantic_markers(
    spec: &LoadedSpec,
) -> EscapeHatchSemanticMarkerSummary {
    let summary = summarize_backend_execution_markers(spec);
    EscapeHatchSemanticMarkerSummary {
        has_domain_lowering: summary.has_domain_lowering,
        has_proof_helper_lowering: summary.has_proof_helper_lowering,
        has_backend_rust_derives: summary.has_backend_rust_derives,
    }
}

fn required_surfaces() -> Vec<EscapeHatchProofSurface> {
    vec![
        EscapeHatchProofSurface::Atom,
        EscapeHatchProofSurface::Molecule,
    ]
}

pub(crate) fn current_proof_surfaces(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
    molecule_tests: &[LoadedMoleculeTest],
    molecule_evidence_by_id: &HashMap<String, MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> CurrentProofSurfaces {
    CurrentProofSurfaces {
        atom: atom_surface_present(spec, passport),
        molecule: molecule_surface_present(
            spec,
            molecule_tests,
            molecule_evidence_by_id,
            specs_by_id,
        ),
    }
}

fn atom_surface_present(spec: &LoadedSpec, passport: Option<&Passport>) -> bool {
    if spec.spec.local_tests.is_empty() {
        return false;
    }

    let Some(passport) = passport else {
        return false;
    };
    let Some(evidence) = passport.evidence.as_ref() else {
        return false;
    };
    if evidence.build_status != "pass" {
        return false;
    }

    if !spec.spec.local_tests.iter().all(|local_test| {
        evidence
            .test_results
            .iter()
            .find(|result| result.id == local_test.id)
            .is_some_and(|result| result.status == "pass")
    }) {
        return false;
    }

    let Some(freshness) = resolve_passport_freshness(spec, Some(passport)) else {
        return false;
    };

    freshness.authored_truth_status != FreshnessStatus::Stale
        && freshness.backend_execution_status != FreshnessStatus::Stale
}

fn molecule_surface_present(
    spec: &LoadedSpec,
    molecule_tests: &[LoadedMoleculeTest],
    molecule_evidence_by_id: &HashMap<String, MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> bool {
    molecule_tests
        .iter()
        .filter(|test| {
            test.test
                .covers
                .iter()
                .any(|cover_id| cover_id == &spec.spec.id)
        })
        .any(|test| {
            molecule_evidence_by_id
                .get(&test.test.id)
                .is_some_and(|evidence| {
                    molecule_evidence_is_current_pass(evidence, test, specs_by_id)
                })
        })
}

fn format_open_reason(missing_surfaces: &[EscapeHatchProofSurface]) -> Option<String> {
    if missing_surfaces.is_empty() {
        return None;
    }

    let joined = missing_surfaces
        .iter()
        .map(|surface| surface.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!("missing required escape-hatch proof: {joined}"))
}

#[cfg(test)]
pub(crate) fn is_helper_or_example_method(method: &AuthoredMethod) -> bool {
    backend_is_helper_or_example_method(method)
}

impl EscapeHatchProofSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Atom => "atom",
            Self::Molecule => "molecule",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule_evidence::{MoleculeEvidenceStatus, build_molecule_evidence};
    use crate::passport::{PassportEvidence, PassportTestResult, build_passport_with_evidence};
    use crate::types::{
        AuthoredMethod, AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering,
        AuthoredSumShape, AuthoredSumVariant, Body, Intent, LocalTest, MoleculeTestSource,
        MoleculeTestStruct, SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;

    const ACCEPTED_EXAMPLE_HELPER_IDS: &[&str] = &[
        "percentage_example",
        "fixed_amount_example",
        "fixed_amount_capped_example",
    ];

    fn marked_sum_seam(with_local_tests: bool) -> LoadedSpec {
        seam_with_markers(
            vec![domain_method()],
            vec!["Clone".to_string()],
            with_local_tests,
        )
    }

    fn proof_helper_sum_seam(with_local_tests: bool) -> LoadedSpec {
        seam_with_markers(vec![proof_helper_method()], Vec::new(), with_local_tests)
    }

    fn proof_helper_data_seam(with_local_tests: bool) -> LoadedSpec {
        data_seam_with_markers(vec![proof_helper_method()], Vec::new(), with_local_tests)
    }

    fn seam_with_markers(
        methods: Vec<AuthoredMethod>,
        derives: Vec<String>,
        with_local_tests: bool,
    ) -> LoadedSpec {
        let mut variants = IndexMap::new();
        variants.insert("none".to_string(), AuthoredSumVariant::default());
        variants.insert("percentage".to_string(), AuthoredSumVariant::default());

        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/discount_policy.unit.spec".to_string(),
                id: "pricing/discount_policy".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/discount_policy".to_string(),
                kind: "sum".to_string(),
                intent: Intent {
                    why: "Represent discount policy".to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: String::new(),
                    typescript: None,
                },
                local_tests: if with_local_tests {
                    vec![LocalTest {
                        id: "variant_percentage".to_string(),
                        expect: "true".to_string(),
                    }]
                } else {
                    vec![]
                },
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    sum: Some(AuthoredSumShape { variants }),
                    methods,
                    backends: Some(crate::types::AuthoredBackends {
                        rust: Some(AuthoredRustBackend { derives }),
                    }),
                    ..UnitExtensions::default()
                },
            },
        }
    }

    fn data_seam_with_markers(
        methods: Vec<AuthoredMethod>,
        derives: Vec<String>,
        with_local_tests: bool,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/checkout_quote.unit.spec".to_string(),
                id: "pricing/checkout_quote".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/checkout_quote".to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: "Quote a checkout total from subtotal plus discount and tax rates."
                        .to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: if with_local_tests {
                    vec![LocalTest {
                        id: "total_basic".to_string(),
                        expect: "true".to_string(),
                    }]
                } else {
                    vec![]
                },
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: Some(crate::types::AuthoredDataShape {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                crate::types::AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "discount_rate".to_string(),
                                crate::types::AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                crate::types::AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                        ]),
                    }),
                    methods,
                    backends: Some(crate::types::AuthoredBackends {
                        rust: Some(AuthoredRustBackend { derives }),
                    }),
                    ..UnitExtensions::default()
                },
            },
        }
    }

    fn domain_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discount_amount".to_string(),
            intent: Intent {
                why: "Calculate discount amount".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: None,
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ 0 }".to_string(),
                }),
            }),
        }
    }

    fn proof_helper_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discount_policy_holds".to_string(),
            intent: Intent {
                why: "Check the proof helper".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ true }".to_string(),
                }),
            }),
        }
    }

    fn example_helper_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "percentage_example".to_string(),
            intent: Intent {
                why: "Check the example helper".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ true }".to_string(),
                }),
            }),
        }
    }

    fn accepted_example_helper_method(id: &str) -> AuthoredMethod {
        let mut method = example_helper_method();
        method.id = id.to_string();
        method
    }

    fn bool_domain_predicate_method(id: &str) -> AuthoredMethod {
        AuthoredMethod {
            id: id.to_string(),
            intent: Intent {
                why: "Report a real domain predicate about the current discount policy."
                    .to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ true }".to_string(),
                }),
            }),
        }
    }

    fn plain_function_unit() -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/apply_tax.unit.spec".to_string(),
                id: "pricing/apply_tax".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_tax".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "apply tax".to_string(),
                },
                contract: Some(crate::types::Contract {
                    inputs: None,
                    returns: Some("i32".to_string()),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "{ 1 }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "basic".to_string(),
                    expect: "true".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn covering_test() -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: "units/pricing/discount_policy_checkout_flow.test.spec".to_string(),
                id: "pricing/discount_policy_checkout_flow".to_string(),
            },
            test: MoleculeTestStruct {
                id: "pricing/discount_policy_checkout_flow".to_string(),
                intent: Intent {
                    why: "cover the seam".to_string(),
                },
                covers: vec!["pricing/discount_policy".to_string()],
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: Some("0.3.0".to_string()),
            },
        }
    }

    fn specs_by_id(specs: &[LoadedSpec]) -> HashMap<String, LoadedSpec> {
        specs
            .iter()
            .map(|spec| (spec.spec.id.clone(), spec.clone()))
            .collect()
    }

    fn seam_passport(spec: &LoadedSpec) -> Passport {
        build_passport_with_evidence(
            spec,
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
            crate::passport::compute_contract_hash(spec),
        )
    }

    #[test]
    fn atom_surface_is_present_with_current_passing_local_proof() {
        let spec = marked_sum_seam(true);
        let passport = seam_passport(&spec);

        assert!(atom_surface_present(&spec, Some(&passport)));
    }

    #[test]
    fn atom_surface_is_missing_when_authored_truth_is_stale() {
        let original = marked_sum_seam(true);
        let passport = seam_passport(&original);
        let mut changed = original.clone();
        changed.spec.intent.why = "Represent revised discount policy".to_string();

        let freshness =
            resolve_passport_freshness(&changed, Some(&passport)).expect("freshness should exist");
        assert_eq!(freshness.authored_truth_status, FreshnessStatus::Stale);
        assert_eq!(freshness.backend_execution_status, FreshnessStatus::Fresh);
        assert!(!atom_surface_present(&changed, Some(&passport)));
    }

    #[test]
    fn atom_surface_is_missing_when_backend_execution_is_stale() {
        let original = marked_sum_seam(true);
        let passport = seam_passport(&original);
        let mut changed = original.clone();
        changed
            .spec
            .extensions
            .backends
            .as_mut()
            .expect("sum seam should have rust backend")
            .rust
            .as_mut()
            .expect("sum seam should have rust backend config")
            .derives
            .push("PartialEq".to_string());

        let freshness =
            resolve_passport_freshness(&changed, Some(&passport)).expect("freshness should exist");
        assert_eq!(freshness.authored_truth_status, FreshnessStatus::Fresh);
        assert_eq!(freshness.backend_execution_status, FreshnessStatus::Stale);
        assert!(!atom_surface_present(&changed, Some(&passport)));
    }

    #[test]
    fn semantic_marker_classification_distinguishes_domain_and_proof_helper_lowering() {
        let domain_spec = marked_sum_seam(true);
        let helper_spec = proof_helper_sum_seam(true);

        assert_eq!(
            collect_escape_hatch_semantic_markers(&domain_spec),
            vec![
                EscapeHatchSemanticMarker {
                    kind: EscapeHatchSemanticMarkerKind::BackendRustDerives,
                    path: "backends.rust.derives".to_string(),
                },
                EscapeHatchSemanticMarker {
                    kind: EscapeHatchSemanticMarkerKind::DomainLowering,
                    path: "methods.discount_amount.lowering.rust.body".to_string(),
                },
            ]
        );
        assert_eq!(
            collect_escape_hatch_semantic_markers(&helper_spec),
            vec![EscapeHatchSemanticMarker {
                kind: EscapeHatchSemanticMarkerKind::ProofHelperLowering,
                path: "methods.discount_policy_holds.lowering.rust.body".to_string(),
            }]
        );
        assert_eq!(
            summarize_escape_hatch_semantic_markers(&domain_spec),
            EscapeHatchSemanticMarkerSummary {
                has_domain_lowering: true,
                has_proof_helper_lowering: false,
                has_backend_rust_derives: true,
            }
        );
        assert_eq!(
            summarize_escape_hatch_semantic_markers(&helper_spec),
            EscapeHatchSemanticMarkerSummary {
                has_domain_lowering: false,
                has_proof_helper_lowering: true,
                has_backend_rust_derives: false,
            }
        );
    }

    #[test]
    fn helper_or_example_method_matches_holds_and_example_shapes() {
        assert!(is_helper_or_example_method(&proof_helper_method()));
        for example_id in ACCEPTED_EXAMPLE_HELPER_IDS {
            assert!(is_helper_or_example_method(
                &accepted_example_helper_method(example_id)
            ));
        }
        assert!(!is_helper_or_example_method(&domain_method()));
        assert!(!is_helper_or_example_method(&bool_domain_predicate_method(
            "has_cap"
        )));
        assert!(!is_helper_or_example_method(&bool_domain_predicate_method(
            "is_discountable"
        )));
    }

    #[test]
    fn helper_or_example_method_uses_same_rule_for_data_and_sum_methods() {
        let sum_helper_markers =
            collect_escape_hatch_semantic_markers(&proof_helper_sum_seam(false));
        let data_helper_markers =
            collect_escape_hatch_semantic_markers(&proof_helper_data_seam(false));
        assert_eq!(sum_helper_markers, data_helper_markers);
        assert_eq!(
            sum_helper_markers,
            vec![EscapeHatchSemanticMarker {
                kind: EscapeHatchSemanticMarkerKind::ProofHelperLowering,
                path: "methods.discount_policy_holds.lowering.rust.body".to_string(),
            }]
        );

        let sum_example_markers = collect_escape_hatch_semantic_markers(&seam_with_markers(
            vec![accepted_example_helper_method("percentage_example")],
            Vec::new(),
            false,
        ));
        let data_example_markers = collect_escape_hatch_semantic_markers(&data_seam_with_markers(
            vec![accepted_example_helper_method("percentage_example")],
            Vec::new(),
            false,
        ));
        assert_eq!(sum_example_markers, data_example_markers);
        assert_eq!(
            sum_example_markers,
            vec![EscapeHatchSemanticMarker {
                kind: EscapeHatchSemanticMarkerKind::ProofHelperLowering,
                path: "methods.percentage_example.lowering.rust.body".to_string(),
            }]
        );

        let sum_domain_markers = collect_escape_hatch_semantic_markers(&seam_with_markers(
            vec![bool_domain_predicate_method("has_cap")],
            Vec::new(),
            false,
        ));
        let data_domain_markers = collect_escape_hatch_semantic_markers(&data_seam_with_markers(
            vec![bool_domain_predicate_method("has_cap")],
            Vec::new(),
            false,
        ));
        assert_eq!(sum_domain_markers, data_domain_markers);
        assert_eq!(
            sum_domain_markers,
            vec![EscapeHatchSemanticMarker {
                kind: EscapeHatchSemanticMarkerKind::DomainLowering,
                path: "methods.has_cap.lowering.rust.body".to_string(),
            }]
        );
    }

    #[test]
    fn bool_domain_predicate_lowering_is_collected_as_domain_marker() {
        let spec = seam_with_markers(vec![bool_domain_predicate_method("has_cap")], vec![], false);

        assert_eq!(
            collect_escape_hatch_semantic_markers(&spec),
            vec![EscapeHatchSemanticMarker {
                kind: EscapeHatchSemanticMarkerKind::DomainLowering,
                path: "methods.has_cap.lowering.rust.body".to_string(),
            }]
        );
        assert_eq!(
            summarize_escape_hatch_semantic_markers(&spec),
            EscapeHatchSemanticMarkerSummary {
                has_domain_lowering: true,
                has_proof_helper_lowering: false,
                has_backend_rust_derives: false,
            }
        );
    }

    #[test]
    fn bool_domain_predicate_does_not_flip_marker_summary_to_proof_helper() {
        let spec = seam_with_markers(
            vec![bool_domain_predicate_method("is_discountable")],
            Vec::new(),
            false,
        );

        let markers = collect_escape_hatch_semantic_markers(&spec);
        assert_eq!(markers.len(), 1);
        assert_eq!(
            markers[0].kind,
            EscapeHatchSemanticMarkerKind::DomainLowering
        );
        assert_eq!(
            summarize_escape_hatch_semantic_markers(&spec),
            EscapeHatchSemanticMarkerSummary {
                has_domain_lowering: true,
                has_proof_helper_lowering: false,
                has_backend_rust_derives: false,
            }
        );
    }

    #[test]
    fn gate_omits_unmarked_units() {
        let spec = plain_function_unit();
        let gate = evaluate_escape_hatch_gate(
            &spec,
            None,
            &[],
            &HashMap::new(),
            &specs_by_id(std::slice::from_ref(&spec)),
        );

        assert!(gate.is_none());
    }

    #[test]
    fn gate_closes_when_atom_and_molecule_proof_are_present() {
        let spec = marked_sum_seam(true);
        let passport = seam_passport(&spec);
        let test = covering_test();
        let specs_by_id = specs_by_id(std::slice::from_ref(&spec));
        let evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &specs_by_id,
            None,
        );
        let gate = evaluate_escape_hatch_gate(
            &spec,
            Some(&passport),
            std::slice::from_ref(&test),
            &HashMap::from([(test.test.id.clone(), evidence)]),
            &specs_by_id,
        )
        .unwrap();

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
    }

    #[test]
    fn gate_opens_when_molecule_proof_is_missing() {
        let spec = marked_sum_seam(true);
        let passport = seam_passport(&spec);
        let gate = evaluate_escape_hatch_gate(
            &spec,
            Some(&passport),
            &[],
            &HashMap::new(),
            &specs_by_id(std::slice::from_ref(&spec)),
        )
        .unwrap();

        assert_eq!(gate.status, EscapeHatchGateStatus::Open);
        assert_eq!(
            gate.missing_surfaces,
            vec![EscapeHatchProofSurface::Molecule]
        );
        assert_eq!(
            gate.reason.as_deref(),
            Some("missing required escape-hatch proof: molecule")
        );
    }

    #[test]
    fn gate_opens_when_atom_proof_is_missing() {
        let spec = marked_sum_seam(false);
        let test = covering_test();
        let specs_by_id = specs_by_id(std::slice::from_ref(&spec));
        let evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &specs_by_id,
            None,
        );
        let gate = evaluate_escape_hatch_gate(
            &spec,
            None,
            std::slice::from_ref(&test),
            &HashMap::from([(test.test.id.clone(), evidence)]),
            &specs_by_id,
        )
        .unwrap();

        assert_eq!(gate.status, EscapeHatchGateStatus::Open);
        assert_eq!(gate.missing_surfaces, vec![EscapeHatchProofSurface::Atom]);
        assert_eq!(
            gate.reason.as_deref(),
            Some("missing required escape-hatch proof: atom")
        );
    }

    #[test]
    fn gate_opens_when_both_surfaces_are_missing() {
        let spec = marked_sum_seam(false);
        let gate = evaluate_escape_hatch_gate(
            &spec,
            None,
            &[],
            &HashMap::new(),
            &specs_by_id(std::slice::from_ref(&spec)),
        )
        .unwrap();

        assert_eq!(gate.status, EscapeHatchGateStatus::Open);
        assert_eq!(
            gate.missing_surfaces,
            vec![
                EscapeHatchProofSurface::Atom,
                EscapeHatchProofSurface::Molecule,
            ]
        );
        assert_eq!(
            gate.reason.as_deref(),
            Some("missing required escape-hatch proof: atom, molecule")
        );
    }

    #[test]
    fn gate_behavior_stays_constant_across_semantic_marker_classes() {
        let domain_spec = marked_sum_seam(true);
        let helper_spec = proof_helper_sum_seam(true);
        let domain_passport = seam_passport(&domain_spec);
        let helper_passport = seam_passport(&helper_spec);
        let domain_test = covering_test();
        let helper_test = covering_test();
        let domain_specs_by_id = specs_by_id(std::slice::from_ref(&domain_spec));
        let helper_specs_by_id = specs_by_id(std::slice::from_ref(&helper_spec));
        let domain_evidence = build_molecule_evidence(
            &domain_test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &domain_specs_by_id,
            None,
        );
        let helper_evidence = build_molecule_evidence(
            &helper_test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &helper_specs_by_id,
            None,
        );

        let domain_gate = evaluate_escape_hatch_gate(
            &domain_spec,
            Some(&domain_passport),
            std::slice::from_ref(&domain_test),
            &HashMap::from([(domain_test.test.id.clone(), domain_evidence)]),
            &domain_specs_by_id,
        )
        .unwrap();
        let helper_gate = evaluate_escape_hatch_gate(
            &helper_spec,
            Some(&helper_passport),
            std::slice::from_ref(&helper_test),
            &HashMap::from([(helper_test.test.id.clone(), helper_evidence)]),
            &helper_specs_by_id,
        )
        .unwrap();

        assert_ne!(
            summarize_escape_hatch_semantic_markers(&domain_spec),
            summarize_escape_hatch_semantic_markers(&helper_spec)
        );
        assert_eq!(domain_gate, helper_gate);
        assert_eq!(domain_gate.status, EscapeHatchGateStatus::Closed);
    }

    #[test]
    fn semantic_marker_classification_is_independent_from_current_proof_surfaces() {
        let spec = marked_sum_seam(true);
        let passport = seam_passport(&spec);
        let test = covering_test();
        let specs_by_id = specs_by_id(std::slice::from_ref(&spec));
        let evidence = build_molecule_evidence(
            &test,
            MoleculeEvidenceStatus::Pass,
            None,
            "2026-04-21T00:00:00Z",
            &specs_by_id,
            None,
        );
        let closed_gate = evaluate_escape_hatch_gate(
            &spec,
            Some(&passport),
            std::slice::from_ref(&test),
            &HashMap::from([(test.test.id.clone(), evidence)]),
            &specs_by_id,
        )
        .unwrap();
        let open_gate =
            evaluate_escape_hatch_gate(&spec, None, &[], &HashMap::new(), &specs_by_id).unwrap();

        let summary_before = summarize_escape_hatch_semantic_markers(&spec);
        let summary_after = summarize_escape_hatch_semantic_markers(&spec);

        assert_eq!(summary_before, summary_after);
        assert_eq!(closed_gate.status, EscapeHatchGateStatus::Closed);
        assert_eq!(open_gate.status, EscapeHatchGateStatus::Open);
    }
}
