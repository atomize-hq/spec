use crate::molecule_evidence::{MoleculeEvidence, molecule_evidence_is_current_pass};
use crate::passport::{
    FreshnessStatus, Passport, compute_passport_markers, resolve_passport_freshness,
};
use crate::types::{AuthoredMethod, LoadedMoleculeTest, LoadedSpec, UnitKind};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EscapeHatchSemanticMarkerKind {
    DomainLowering,
    ProofHelperLowering,
    BackendRustDerives,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EscapeHatchSemanticMarker {
    pub kind: EscapeHatchSemanticMarkerKind,
    pub path: String,
}

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

pub(crate) fn collect_escape_hatch_semantic_markers(
    spec: &LoadedSpec,
) -> Vec<EscapeHatchSemanticMarker> {
    if !matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum)) {
        return Vec::new();
    }

    let mut markers = Vec::new();
    if spec
        .spec
        .extensions
        .backends
        .as_ref()
        .and_then(|backends| backends.rust.as_ref())
        .map(|rust| !rust.derives.is_empty())
        .unwrap_or(false)
    {
        markers.push(EscapeHatchSemanticMarker {
            kind: EscapeHatchSemanticMarkerKind::BackendRustDerives,
            path: "backends.rust.derives".to_string(),
        });
    }

    for method in &spec.spec.extensions.methods {
        if method
            .lowering
            .as_ref()
            .and_then(|lowering| lowering.rust.as_ref())
            .is_some()
        {
            markers.push(EscapeHatchSemanticMarker {
                kind: if is_proof_helper_method(method) {
                    EscapeHatchSemanticMarkerKind::ProofHelperLowering
                } else {
                    EscapeHatchSemanticMarkerKind::DomainLowering
                },
                path: format!("methods.{}.lowering.rust.body", method.id),
            });
        }
    }

    markers
}

pub(crate) fn summarize_escape_hatch_semantic_markers(
    spec: &LoadedSpec,
) -> EscapeHatchSemanticMarkerSummary {
    let mut summary = EscapeHatchSemanticMarkerSummary::default();
    for marker in collect_escape_hatch_semantic_markers(spec) {
        match marker.kind {
            EscapeHatchSemanticMarkerKind::DomainLowering => {
                summary.has_domain_lowering = true;
            }
            EscapeHatchSemanticMarkerKind::ProofHelperLowering => {
                summary.has_proof_helper_lowering = true;
            }
            EscapeHatchSemanticMarkerKind::BackendRustDerives => {
                summary.has_backend_rust_derives = true;
            }
        }
    }
    summary
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

fn is_proof_helper_method(method: &AuthoredMethod) -> bool {
    method.id.ends_with("_holds")
        && method
            .contract
            .as_ref()
            .and_then(|contract| contract.returns.as_deref())
            == Some("bool")
        && method
            .contract
            .as_ref()
            .and_then(|contract| contract.inputs.as_ref())
            .is_none_or(|inputs| inputs.is_empty())
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
