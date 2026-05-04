use crate::backend_execution::{
    BackendExecutionMarkerKind, collect_backend_execution_markers,
    compute_backend_execution_digest as compute_backend_execution_digest_from_boundary,
    summarize_backend_execution_markers,
};
use crate::escape_hatch::{EscapeHatchGate, current_proof_surfaces, evaluate_escape_hatch_gate};
use crate::molecule_evidence::MoleculeEvidence;
use crate::passport::Passport;
use crate::types::{LoadedMoleculeTest, LoadedSpec, UnitKind};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortabilityMarkerKind {
    DomainLowering,
    ProofHelperLowering,
    BackendRustDerives,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityMarker {
    pub kind: PortabilityMarkerKind,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortabilityMarkerSummary {
    pub has_domain_lowering: bool,
    pub has_proof_helper_lowering: bool,
    pub has_backend_rust_derives: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortabilityProofSurfaces {
    pub atom: bool,
    pub molecule: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortabilityContaminationSummary {
    pub has_backend_only_detail: bool,
    pub has_contaminating_domain_lowering: bool,
}

pub struct PortabilityProjectionContext<'a> {
    pub molecule_tests: &'a [LoadedMoleculeTest],
    pub molecule_evidence_by_id: &'a HashMap<String, MoleculeEvidence>,
    pub specs_by_id: &'a HashMap<String, LoadedSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityProjection {
    pub markers: Vec<PortabilityMarker>,
    pub marker_summary: PortabilityMarkerSummary,
    pub backend_execution_digest: Option<String>,
    pub proof_surfaces: PortabilityProofSurfaces,
    pub escape_hatch_gate: Option<EscapeHatchGate>,
    pub contamination_summary: PortabilityContaminationSummary,
}

pub fn collect_portability_markers(spec: &LoadedSpec) -> Vec<PortabilityMarker> {
    if !is_seam(spec) {
        return Vec::new();
    }

    collect_backend_execution_markers(spec)
        .into_iter()
        .map(|marker| PortabilityMarker {
            kind: match marker.kind {
                BackendExecutionMarkerKind::DomainLowering => PortabilityMarkerKind::DomainLowering,
                BackendExecutionMarkerKind::ProofHelperLowering => {
                    PortabilityMarkerKind::ProofHelperLowering
                }
                BackendExecutionMarkerKind::BackendRustDerives => {
                    PortabilityMarkerKind::BackendRustDerives
                }
            },
            path: marker.path,
        })
        .collect()
}

pub fn summarize_portability_markers(spec: &LoadedSpec) -> Option<PortabilityMarkerSummary> {
    if !is_seam(spec) {
        return None;
    }

    let summary = summarize_backend_execution_markers(spec);
    Some(PortabilityMarkerSummary {
        has_domain_lowering: summary.has_domain_lowering,
        has_proof_helper_lowering: summary.has_proof_helper_lowering,
        has_backend_rust_derives: summary.has_backend_rust_derives,
    })
}

pub fn compute_portability_backend_digest(spec: &LoadedSpec) -> Option<String> {
    is_seam(spec)
        .then(|| compute_backend_execution_digest_from_boundary(spec))
        .flatten()
        .map(|digest| format!("sha256:{digest}"))
}

pub fn evaluate_portability_gate(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
    context: &PortabilityProjectionContext<'_>,
) -> Option<EscapeHatchGate> {
    if !is_seam(spec) {
        return None;
    }

    evaluate_escape_hatch_gate(
        spec,
        passport,
        context.molecule_tests,
        context.molecule_evidence_by_id,
        context.specs_by_id,
    )
}

pub fn project_portability_truth(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
    context: &PortabilityProjectionContext<'_>,
) -> Option<PortabilityProjection> {
    if !is_seam(spec) {
        return None;
    }

    let markers = collect_portability_markers(spec);
    let marker_summary = summarize_portability_markers(spec).unwrap_or_default();
    let current_surfaces = current_proof_surfaces(
        spec,
        passport,
        context.molecule_tests,
        context.molecule_evidence_by_id,
        context.specs_by_id,
    );

    Some(PortabilityProjection {
        backend_execution_digest: compute_portability_backend_digest(spec),
        escape_hatch_gate: evaluate_portability_gate(spec, passport, context),
        proof_surfaces: PortabilityProofSurfaces {
            atom: current_surfaces.atom,
            molecule: current_surfaces.molecule,
        },
        contamination_summary: summarize_portability_contamination(spec).unwrap_or_default(),
        markers,
        marker_summary,
    })
}

pub fn summarize_portability_contamination(
    spec: &LoadedSpec,
) -> Option<PortabilityContaminationSummary> {
    let summary = summarize_portability_markers(spec)?;
    Some(PortabilityContaminationSummary {
        has_backend_only_detail: summary.has_proof_helper_lowering
            || summary.has_backend_rust_derives,
        has_contaminating_domain_lowering: summary.has_domain_lowering,
    })
}

fn is_seam(spec: &LoadedSpec) -> bool {
    matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum))
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

    #[test]
    fn helper_only_projection_is_backend_only_but_not_contaminating() {
        let spec = seam_with_markers(vec![proof_helper_method()], vec![], true);
        let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
        let molecule_evidence_by_id = HashMap::new();
        let context = PortabilityProjectionContext {
            molecule_tests: &[],
            molecule_evidence_by_id: &molecule_evidence_by_id,
            specs_by_id: &specs_by_id,
        };

        let projection = project_portability_truth(&spec, None, &context).unwrap();

        assert_eq!(
            projection.markers,
            vec![PortabilityMarker {
                kind: PortabilityMarkerKind::ProofHelperLowering,
                path: "methods.discount_policy_holds.lowering.rust.body".to_string(),
            }]
        );
        assert_eq!(
            projection.contamination_summary,
            PortabilityContaminationSummary {
                has_backend_only_detail: true,
                has_contaminating_domain_lowering: false,
            }
        );
        assert!(
            projection
                .backend_execution_digest
                .as_deref()
                .is_some_and(|digest| digest.starts_with("sha256:"))
        );
    }

    #[test]
    fn projection_reports_current_proof_surfaces_and_gate() {
        let spec = seam_with_markers(vec![domain_method()], vec!["Clone".to_string()], true);
        let molecule_test = molecule_test_covering(&spec.spec.id);
        let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
        let molecule_evidence = build_molecule_evidence(
            &molecule_test,
            MoleculeEvidenceStatus::Pass,
            Some("pass".to_string()),
            "2026-05-04T17:10:46Z",
            &specs_by_id,
            None,
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-05-04T17:10:46Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: spec
                    .spec
                    .local_tests
                    .iter()
                    .map(|test| PassportTestResult {
                        id: test.id.clone(),
                        status: "pass".to_string(),
                        reason: None,
                    })
                    .collect(),
                observed_at: "2026-05-04T17:10:46Z".to_string(),
                provenance: None,
            }),
            None,
        );
        let molecule_evidence_by_id =
            HashMap::from([(molecule_test.test.id.clone(), molecule_evidence)]);
        let context = PortabilityProjectionContext {
            molecule_tests: std::slice::from_ref(&molecule_test),
            molecule_evidence_by_id: &molecule_evidence_by_id,
            specs_by_id: &specs_by_id,
        };

        let projection = project_portability_truth(&spec, Some(&passport), &context).unwrap();

        assert_eq!(
            projection.marker_summary,
            PortabilityMarkerSummary {
                has_domain_lowering: true,
                has_proof_helper_lowering: false,
                has_backend_rust_derives: true,
            }
        );
        assert_eq!(
            projection.proof_surfaces,
            PortabilityProofSurfaces {
                atom: true,
                molecule: true,
            }
        );
        assert_eq!(
            projection
                .escape_hatch_gate
                .expect("gate for marked seam")
                .status,
            crate::escape_hatch::EscapeHatchGateStatus::Closed
        );
    }

    #[test]
    fn non_seams_do_not_project_seam_portability_truth() {
        let spec = LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/apply_discount.unit.spec".to_string(),
                id: "pricing/apply_discount".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/apply_discount".to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: "Apply a discount to the subtotal.".to_string(),
                },
                contract: Some(crate::types::Contract {
                    inputs: Some(IndexMap::from([
                        ("subtotal".to_string(), "Decimal".to_string()),
                        ("discount_rate".to_string(), "Decimal".to_string()),
                    ])),
                    returns: Some("Decimal".to_string()),
                    invariants: Vec::new(),
                }),
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "{ subtotal }".to_string(),
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                extensions: UnitExtensions::default(),
                spec_version: None,
            },
        };
        let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
        let molecule_evidence_by_id = HashMap::new();
        let context = PortabilityProjectionContext {
            molecule_tests: &[],
            molecule_evidence_by_id: &molecule_evidence_by_id,
            specs_by_id: &specs_by_id,
        };

        assert_eq!(collect_portability_markers(&spec), Vec::new());
        assert_eq!(summarize_portability_markers(&spec), None);
        assert_eq!(compute_portability_backend_digest(&spec), None);
        assert_eq!(evaluate_portability_gate(&spec, None, &context), None);
        assert_eq!(project_portability_truth(&spec, None, &context), None);
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
                body: Body::default(),
                local_tests: with_local_tests
                    .then(|| {
                        vec![LocalTest {
                            id: "variant_none".to_string(),
                            expect: "true".to_string(),
                        }]
                    })
                    .unwrap_or_default(),
                links: None,
                extensions: UnitExtensions {
                    sum: Some(AuthoredSumShape { variants }),
                    methods,
                    backends: Some(crate::types::AuthoredBackends {
                        rust: Some(AuthoredRustBackend { derives }),
                    }),
                    ..UnitExtensions::default()
                },
                spec_version: None,
            },
        }
    }

    fn domain_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discounted_subtotal".to_string(),
            intent: Intent {
                why: "Return the subtotal after discount.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: Some(IndexMap::from([(
                    "subtotal".to_string(),
                    "Decimal".to_string(),
                )])),
                returns: Some("Decimal".to_string()),
                invariants: Vec::new(),
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ subtotal }".to_string(),
                }),
            }),
        }
    }

    fn proof_helper_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discount_policy_holds".to_string(),
            intent: Intent {
                why: "Prove helper invariants for discount policy.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: Some(IndexMap::new()),
                returns: Some("bool".to_string()),
                invariants: Vec::new(),
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ true }".to_string(),
                }),
            }),
        }
    }

    fn molecule_test_covering(cover_id: &str) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: "units/pricing/discount_policy_checkout_flow.test.spec".to_string(),
                id: "pricing/discount_policy_checkout_flow".to_string(),
            },
            test: MoleculeTestStruct {
                id: "pricing/discount_policy_checkout_flow".to_string(),
                intent: Intent {
                    why: "Prove discount policy coverage in a molecule test.".to_string(),
                },
                covers: vec![cover_id.to_string()],
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: None,
            },
        }
    }
}
