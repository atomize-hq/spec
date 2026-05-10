use crate::types::{AuthoredMethod, LoadedSpec, UnitKind};

const ACCEPTED_EXAMPLE_HELPER_IDS: &[&str] = &[
    "percentage_example",
    "fixed_amount_example",
    "fixed_amount_capped_example",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortabilityMarkerKind {
    DomainLowering,
    ProofHelperLowering,
    BackendRustDerives,
}

impl PortabilityMarkerKind {
    pub fn is_backend_only_detail(self) -> bool {
        matches!(
            self,
            Self::ProofHelperLowering | Self::BackendRustDerives
        )
    }

    pub fn contaminates_portability_claims(self) -> bool {
        matches!(self, Self::DomainLowering)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSeamAuthoredShapeRule {
    TopLevelContract,
    TopLevelDeps,
    TopLevelImports,
    TopLevelTypescriptBody,
    TopLevelRustBody,
    SumDataFields,
    SumConstructors,
}

pub fn is_portability_seam_kind(kind: UnitKind) -> bool {
    matches!(kind, UnitKind::Data | UnitKind::Sum)
}

pub fn is_portability_seam_spec(spec: &LoadedSpec) -> bool {
    spec.spec.unit_kind().is_ok_and(is_portability_seam_kind)
}

pub fn classify_method_portability_marker(
    method: &AuthoredMethod,
) -> Option<PortabilityMarkerKind> {
    method
        .lowering
        .as_ref()
        .and_then(|lowering| lowering.rust.as_ref())?;

    Some(if is_helper_or_example_method(method) {
        PortabilityMarkerKind::ProofHelperLowering
    } else {
        PortabilityMarkerKind::DomainLowering
    })
}

pub fn is_helper_or_example_method(method: &AuthoredMethod) -> bool {
    has_helper_or_example_shape(method) && has_accepted_helper_or_example_name(method.id.as_str())
}

pub fn shared_surface_violation_message(
    kind: UnitKind,
    rule: SharedSeamAuthoredShapeRule,
) -> &'static str {
    match (kind, rule) {
        (UnitKind::Data, SharedSeamAuthoredShapeRule::TopLevelContract) => {
            "kind:data must not use top-level contract; shared seam semantics belong in data.fields, constructors, and methods"
        }
        (UnitKind::Data, SharedSeamAuthoredShapeRule::TopLevelDeps) => {
            "kind:data must not use top-level deps; attach deps to individual methods instead"
        }
        (UnitKind::Data, SharedSeamAuthoredShapeRule::TopLevelImports) => {
            "kind:data must not use top-level imports; that is an invalid shared-surface authored shape. Rust-specific details are only authored in methods[].lowering.rust.body and backends.rust.derives, and any portability contamination is decided later"
        }
        (UnitKind::Data, SharedSeamAuthoredShapeRule::TopLevelTypescriptBody) => {
            "kind:data must not declare top-level body.typescript; that is an invalid shared-surface authored shape, not a portability verdict"
        }
        (UnitKind::Data, SharedSeamAuthoredShapeRule::TopLevelRustBody) => {
            "kind:data must leave top-level body.rust empty; that authored slot is outside the shared seam surface. Rust-specific lowering belongs in methods[].lowering.rust.body, and portability consequences are decided later"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::TopLevelContract) => {
            "kind:sum must not use top-level contract; shared seam semantics belong in sum.variants and methods"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::TopLevelDeps) => {
            "kind:sum must not use top-level deps; attach deps to individual methods instead"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::TopLevelImports) => {
            "kind:sum must not use top-level imports; that is an invalid shared-surface authored shape. Rust-specific details are only authored in methods[].lowering.rust.body and backends.rust.derives, and any portability contamination is decided later"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::TopLevelTypescriptBody) => {
            "kind:sum must not declare top-level body.typescript; that is an invalid shared-surface authored shape, not a portability verdict"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::TopLevelRustBody) => {
            "kind:sum must leave top-level body.rust empty; that authored slot is outside the shared seam surface. Rust-specific lowering belongs in methods[].lowering.rust.body, and portability consequences are decided later"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::SumDataFields) => {
            "kind:sum must not declare data.fields; sum seams own variants instead"
        }
        (UnitKind::Sum, SharedSeamAuthoredShapeRule::SumConstructors) => {
            "kind:sum must not declare constructors; enum cases are authored via sum.variants"
        }
        _ => panic!("unsupported shared-surface rule for unit kind"),
    }
}

fn has_helper_or_example_shape(method: &AuthoredMethod) -> bool {
    method.receiver == "shared_ref"
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

fn has_accepted_helper_or_example_name(method_id: &str) -> bool {
    method_id.ends_with("_holds") || ACCEPTED_EXAMPLE_HELPER_IDS.contains(&method_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredMethodLowering, AuthoredRustMethodLowering, Contract, Intent, LoadedSpec,
        SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;

    #[test]
    fn recognizes_data_and_sum_as_portability_seams() {
        assert!(is_portability_seam_kind(UnitKind::Data));
        assert!(is_portability_seam_kind(UnitKind::Sum));
        assert!(!is_portability_seam_kind(UnitKind::Function));
    }

    #[test]
    fn classifies_helper_and_domain_method_markers() {
        assert_eq!(
            classify_method_portability_marker(&proof_helper_method()),
            Some(PortabilityMarkerKind::ProofHelperLowering)
        );
        assert_eq!(
            classify_method_portability_marker(&domain_method()),
            Some(PortabilityMarkerKind::DomainLowering)
        );
    }

    #[test]
    fn backend_only_and_contaminating_marker_meaning_stays_explicit() {
        assert!(PortabilityMarkerKind::ProofHelperLowering.is_backend_only_detail());
        assert!(PortabilityMarkerKind::BackendRustDerives.is_backend_only_detail());
        assert!(!PortabilityMarkerKind::DomainLowering.is_backend_only_detail());
        assert!(PortabilityMarkerKind::DomainLowering.contaminates_portability_claims());
        assert!(!PortabilityMarkerKind::ProofHelperLowering.contaminates_portability_claims());
    }

    #[test]
    fn emits_frozen_data_and_sum_shared_surface_messages() {
        assert_eq!(
            shared_surface_violation_message(
                UnitKind::Data,
                SharedSeamAuthoredShapeRule::TopLevelRustBody
            ),
            "kind:data must leave top-level body.rust empty; that authored slot is outside the shared seam surface. Rust-specific lowering belongs in methods[].lowering.rust.body, and portability consequences are decided later"
        );
        assert_eq!(
            shared_surface_violation_message(
                UnitKind::Sum,
                SharedSeamAuthoredShapeRule::TopLevelImports
            ),
            "kind:sum must not use top-level imports; that is an invalid shared-surface authored shape. Rust-specific details are only authored in methods[].lowering.rust.body and backends.rust.derives, and any portability contamination is decided later"
        );
    }

    #[test]
    fn loaded_spec_helper_uses_same_seam_detection() {
        let data = seam_spec("pricing/quote", "data");
        let sum = seam_spec("pricing/discount_policy", "sum");
        let function = seam_spec("pricing/apply_discount", "function");

        assert!(is_portability_seam_spec(&data));
        assert!(is_portability_seam_spec(&sum));
        assert!(!is_portability_seam_spec(&function));
    }

    fn proof_helper_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discount_policy_holds".to_string(),
            intent: Intent {
                why: "Check the helper invariant.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(Contract {
                inputs: Some(IndexMap::new()),
                returns: Some("bool".to_string()),
                invariants: Vec::new(),
            }),
            deps: Vec::new(),
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ true }".to_string(),
                }),
            }),
        }
    }

    fn domain_method() -> AuthoredMethod {
        AuthoredMethod {
            id: "discount_amount".to_string(),
            intent: Intent {
                why: "Compute the domain-level discount amount.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(Contract {
                inputs: Some(IndexMap::from([(
                    "subtotal".to_string(),
                    "Decimal".to_string(),
                )])),
                returns: Some("Money".to_string()),
                invariants: Vec::new(),
            }),
            deps: Vec::new(),
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ subtotal }".to_string(),
                }),
            }),
        }
    }

    fn seam_spec(id: &str, kind: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("examples/ecommerce/units/{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: kind.to_string(),
                intent: Intent {
                    why: "Fixture".to_string(),
                },
                contract: None,
                deps: Vec::new(),
                imports: Vec::new(),
                body: Default::default(),
                local_tests: Vec::new(),
                links: None,
                spec_version: None,
                extensions: UnitExtensions::default(),
            },
        }
    }
}
