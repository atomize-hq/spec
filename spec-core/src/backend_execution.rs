use crate::types::{AuthoredMethod, LoadedSpec, UnitKind};
use serde::Serialize;
use sha2::{Digest, Sha256};

const ACCEPTED_EXAMPLE_HELPER_IDS: &[&str] = &[
    "percentage_example",
    "fixed_amount_example",
    "fixed_amount_capped_example",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendExecutionMarkerKind {
    DomainLowering,
    ProofHelperLowering,
    BackendRustDerives,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendExecutionMarker {
    pub kind: BackendExecutionMarkerKind,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BackendExecutionMarkerSummary {
    pub has_domain_lowering: bool,
    pub has_proof_helper_lowering: bool,
    pub has_backend_rust_derives: bool,
}

#[derive(Serialize)]
struct SeamBackendExecutionSurface<'a> {
    method_lowering_rust_bodies: Vec<&'a str>,
    rust_derives: Vec<&'a str>,
}

pub fn collect_backend_execution_markers(spec: &LoadedSpec) -> Vec<BackendExecutionMarker> {
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
        markers.push(BackendExecutionMarker {
            kind: BackendExecutionMarkerKind::BackendRustDerives,
            path: "backends.rust.derives".to_string(),
        });
    }

    for method in &spec.spec.extensions.methods {
        if let Some(kind) = classify_method_lowering_marker(method) {
            markers.push(BackendExecutionMarker {
                kind,
                path: format!("methods.{}.lowering.rust.body", method.id),
            });
        }
    }

    markers
}

pub fn summarize_backend_execution_markers(spec: &LoadedSpec) -> BackendExecutionMarkerSummary {
    let mut summary = BackendExecutionMarkerSummary::default();
    for marker in collect_backend_execution_markers(spec) {
        match marker.kind {
            BackendExecutionMarkerKind::DomainLowering => {
                summary.has_domain_lowering = true;
            }
            BackendExecutionMarkerKind::ProofHelperLowering => {
                summary.has_proof_helper_lowering = true;
            }
            BackendExecutionMarkerKind::BackendRustDerives => {
                summary.has_backend_rust_derives = true;
            }
        }
    }
    summary
}

pub fn compute_backend_execution_digest(spec: &LoadedSpec) -> Option<String> {
    if !matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum)) {
        return None;
    }

    let method_lowering_rust_bodies: Vec<&str> = spec
        .spec
        .extensions
        .methods
        .iter()
        .filter_map(|method| {
            method
                .lowering
                .as_ref()
                .and_then(|lowering| lowering.rust.as_ref())
                .map(|rust| rust.body.as_str())
        })
        .collect();
    let rust_derives: Vec<&str> = spec
        .spec
        .extensions
        .backends
        .as_ref()
        .and_then(|backends| backends.rust.as_ref())
        .map(|rust| rust.derives.iter().map(String::as_str).collect())
        .unwrap_or_default();

    if method_lowering_rust_bodies.is_empty() && rust_derives.is_empty() {
        return None;
    }

    let json = serde_json::to_string(&SeamBackendExecutionSurface {
        method_lowering_rust_bodies,
        rust_derives,
    })
    .expect("seam backend-execution serialization cannot fail for well-formed spec");
    Some(sha256_digest(&json))
}

pub fn classify_method_lowering_marker(
    method: &AuthoredMethod,
) -> Option<BackendExecutionMarkerKind> {
    method
        .lowering
        .as_ref()
        .and_then(|lowering| lowering.rust.as_ref())?;

    Some(if is_helper_or_example_method(method) {
        BackendExecutionMarkerKind::ProofHelperLowering
    } else {
        BackendExecutionMarkerKind::DomainLowering
    })
}

pub fn is_helper_or_example_method(method: &AuthoredMethod) -> bool {
    has_helper_or_example_shape(method) && has_accepted_helper_or_example_name(method.id.as_str())
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

fn sha256_digest(json: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredBackends, AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering,
        AuthoredSumShape, AuthoredSumVariant, Body, Intent, LoadedSpec, SpecSource, SpecStruct,
        UnitExtensions,
    };
    use indexmap::IndexMap;

    #[test]
    fn authored_only_seam_edits_do_not_change_backend_execution_digest() {
        let mut base = seam_with_markers(vec![domain_method()], vec!["Clone".to_string()]);
        let baseline = compute_backend_execution_digest(&base);

        base.spec.intent.why =
            "Updated authored intent without touching backend execution.".to_string();
        base.spec.extensions.sum = Some(AuthoredSumShape {
            variants: IndexMap::from([(
                "percentage".to_string(),
                AuthoredSumVariant {
                    fields: IndexMap::from([(
                        "percent".to_string(),
                        crate::types::AuthoredField {
                            type_: "Decimal".to_string(),
                        },
                    )]),
                },
            )]),
        });

        assert_eq!(compute_backend_execution_digest(&base), baseline);
    }

    #[test]
    fn backend_only_edits_do_not_change_helper_classification_or_authored_shape() {
        let mut spec = seam_with_markers(vec![proof_helper_method()], vec![]);
        let baseline_summary = summarize_backend_execution_markers(&spec);
        let baseline_markers = collect_backend_execution_markers(&spec);

        spec.spec
            .extensions
            .backends
            .get_or_insert(AuthoredBackends {
                rust: Some(AuthoredRustBackend {
                    derives: Vec::new(),
                }),
            })
            .rust
            .get_or_insert(AuthoredRustBackend {
                derives: Vec::new(),
            })
            .derives
            .push("Clone".to_string());

        let summary = summarize_backend_execution_markers(&spec);
        let markers = collect_backend_execution_markers(&spec);

        assert_eq!(
            baseline_summary.has_domain_lowering,
            summary.has_domain_lowering
        );
        assert_eq!(
            baseline_summary.has_proof_helper_lowering,
            summary.has_proof_helper_lowering
        );
        assert_eq!(summary.has_backend_rust_derives, true);
        assert_eq!(
            markers
                .iter()
                .filter(|marker| marker.kind == BackendExecutionMarkerKind::ProofHelperLowering)
                .count(),
            baseline_markers
                .iter()
                .filter(|marker| marker.kind == BackendExecutionMarkerKind::ProofHelperLowering)
                .count()
        );
    }

    #[test]
    fn helper_and_domain_markers_remain_distinguishable() {
        let helper = seam_with_markers(vec![proof_helper_method()], vec![]);
        let domain = seam_with_markers(vec![domain_method()], vec!["Clone".to_string()]);

        assert_eq!(
            collect_backend_execution_markers(&helper),
            vec![BackendExecutionMarker {
                kind: BackendExecutionMarkerKind::ProofHelperLowering,
                path: "methods.discount_policy_holds.lowering.rust.body".to_string(),
            }]
        );
        assert_eq!(
            collect_backend_execution_markers(&domain),
            vec![
                BackendExecutionMarker {
                    kind: BackendExecutionMarkerKind::BackendRustDerives,
                    path: "backends.rust.derives".to_string(),
                },
                BackendExecutionMarker {
                    kind: BackendExecutionMarkerKind::DomainLowering,
                    path: "methods.discount_amount.lowering.rust.body".to_string(),
                },
            ]
        );
    }

    fn seam_with_markers(
        methods: Vec<crate::types::AuthoredMethod>,
        derives: Vec<String>,
    ) -> LoadedSpec {
        LoadedSpec {
            spec: SpecStruct {
                id: "pricing/discount_policy".to_string(),
                kind: "sum".to_string(),
                intent: Intent {
                    why: "Model the discount policy seam.".to_string(),
                },
                contract: None,
                deps: Vec::new(),
                body: Body::default(),
                local_tests: Vec::new(),
                imports: Vec::new(),
                links: None,
                spec_version: None,
                extensions: UnitExtensions {
                    sum: Some(AuthoredSumShape {
                        variants: IndexMap::from([(
                            "percentage".to_string(),
                            AuthoredSumVariant {
                                fields: IndexMap::from([(
                                    "percent".to_string(),
                                    crate::types::AuthoredField {
                                        type_: "Decimal".to_string(),
                                    },
                                )]),
                            },
                        )]),
                    }),
                    data: None,
                    constructors: Vec::new(),
                    methods,
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend { derives }),
                    }),
                },
            },
            source: SpecSource {
                file_path: "examples/ecommerce/units/pricing/discount_policy.unit.spec".to_string(),
                id: "pricing/discount_policy".to_string(),
            },
        }
    }

    fn domain_method() -> crate::types::AuthoredMethod {
        crate::types::AuthoredMethod {
            id: "discount_amount".to_string(),
            intent: Intent {
                why: "Calculate the discount amount.".to_string(),
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
            deps: Vec::new(),
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ subtotal * percent }".to_string(),
                }),
            }),
        }
    }

    fn proof_helper_method() -> crate::types::AuthoredMethod {
        crate::types::AuthoredMethod {
            id: "discount_policy_holds".to_string(),
            intent: Intent {
                why: "Check the proof helper.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(crate::types::Contract {
                inputs: None,
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
}
