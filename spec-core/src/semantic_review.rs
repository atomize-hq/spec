use crate::escape_hatch::summarize_escape_hatch_semantic_markers;
use crate::generator::lower_sum_seam;
use crate::normalizer::normalize_unit;
use crate::types::{
    AuthoredMethod, AuthoredSumShape, LoadedSpec, NormalizedUnit, RustInherentMethodLowering,
    UnitKind,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticVerdict {
    Aligned,
    UnderSpecified,
    SemanticDrift,
    BackendOnlyMeaningPreserved,
    BackendOnlySemanticsLeaked,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SemanticReasonCode {
    MissingSemanticMethods,
    MissingMethodContract,
    VagueUnitIntent,
    VagueMethodIntent,
    VariantShapeMismatch,
    MethodSignatureMismatch,
    MethodBodyMissingCapBehavior,
    BackendOnlyExecutionMarker,
    ProofHelperOnlyMarker,
    DomainLoweringPresent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvaluatorScope {
    SupportedSumSurface,
    UnsupportedSurface,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticCitation {
    pub path: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticReview {
    pub verdict: SemanticVerdict,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<SemanticReasonCode>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authored_surfaces: Vec<SemanticCitation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub executable_surfaces: Vec<SemanticCitation>,
    pub evaluator_scope: EvaluatorScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticProjectionMode {
    Preserve,
    Refresh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticHealthEffect {
    KeepBase,
    DemoteIncomplete,
    DemoteFailing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticAuthoredPacket {
    pub id: String,
    pub intent: String,
    pub variants: Vec<SemanticVariantPacket>,
    pub methods: Vec<SemanticMethodPacket>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticExecutablePacket {
    pub id: String,
    pub enum_name: String,
    pub variants: Vec<SemanticVariantPacket>,
    pub methods: Vec<SemanticExecutableMethodPacket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVariantPacket {
    pub id: String,
    pub fields: Vec<SemanticFieldPacket>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticFieldPacket {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticMethodPacket {
    pub id: String,
    pub intent: String,
    pub receiver: String,
    pub inputs: Vec<SemanticFieldPacket>,
    pub returns: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticExecutableMethodPacket {
    pub id: String,
    pub receiver: String,
    pub inputs: Vec<SemanticFieldPacket>,
    pub returns: Option<String>,
    pub body_rust: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticMarkerSummary {
    pub has_domain_lowering: bool,
    pub has_helper_lowering: bool,
    pub has_backend_derives: bool,
}

pub fn project_semantic_review(
    spec: &LoadedSpec,
    existing: Option<&SemanticReview>,
    mode: SemanticProjectionMode,
) -> Option<SemanticReview> {
    match mode {
        SemanticProjectionMode::Preserve => existing.cloned(),
        SemanticProjectionMode::Refresh => evaluate_semantic_review(spec),
    }
}

pub fn semantic_health_effect(review: Option<&SemanticReview>) -> SemanticHealthEffect {
    let Some(review) = review else {
        return SemanticHealthEffect::KeepBase;
    };
    if review.evaluator_scope != EvaluatorScope::SupportedSumSurface {
        return SemanticHealthEffect::KeepBase;
    }

    match review.verdict {
        SemanticVerdict::Aligned | SemanticVerdict::BackendOnlyMeaningPreserved => {
            SemanticHealthEffect::KeepBase
        }
        SemanticVerdict::UnderSpecified => SemanticHealthEffect::DemoteIncomplete,
        SemanticVerdict::SemanticDrift | SemanticVerdict::BackendOnlySemanticsLeaked => {
            SemanticHealthEffect::DemoteFailing
        }
    }
}

pub fn semantic_review_summary(review: &SemanticReview) -> String {
    let prefix = match review.verdict {
        SemanticVerdict::Aligned => "semantic aligned",
        SemanticVerdict::UnderSpecified => "semantic under-specified",
        SemanticVerdict::SemanticDrift => "semantic drift",
        SemanticVerdict::BackendOnlyMeaningPreserved => "backend-only meaning preserved",
        SemanticVerdict::BackendOnlySemanticsLeaked => "backend-only semantics leaked",
    };

    if review.summary.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}: {}", review.summary)
    }
}

pub fn evaluate_semantic_review(spec: &LoadedSpec) -> Option<SemanticReview> {
    if !matches!(spec.spec.unit_kind(), Ok(UnitKind::Sum)) {
        return None;
    }

    let authored = build_authored_packet(spec)?;
    let executable = build_executable_packet(spec)?;
    let markers = summarize_markers(spec);
    let mut reasons = Vec::new();
    let mut authored_surfaces = authored_citations(spec, &authored);
    let mut executable_surfaces = executable_citations(spec, &executable, markers);

    if authored.methods.is_empty() {
        reasons.push(SemanticReasonCode::MissingSemanticMethods);
    }
    if semantic_text_is_vague(&authored.intent) {
        reasons.push(SemanticReasonCode::VagueUnitIntent);
    }
    for method in &authored.methods {
        if method.returns.is_none() {
            reasons.push(SemanticReasonCode::MissingMethodContract);
        }
        if semantic_text_is_vague(&method.intent) {
            reasons.push(SemanticReasonCode::VagueMethodIntent);
        }
    }
    reasons.sort();
    reasons.dedup();

    if !reasons.is_empty() {
        return Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            reason_codes: reasons,
            summary: "authored semantic surfaces are too weak for honest evaluation".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
    }

    let mut drift_reasons = Vec::new();
    if authored.variants != executable.variants {
        drift_reasons.push(SemanticReasonCode::VariantShapeMismatch);
    }

    for authored_method in &authored.methods {
        match executable
            .methods
            .iter()
            .find(|method| method.id == authored_method.id)
        {
            Some(executable_method) => {
                if authored_method.receiver != executable_method.receiver
                    || authored_method.inputs != executable_method.inputs
                    || authored_method.returns != executable_method.returns
                {
                    drift_reasons.push(SemanticReasonCode::MethodSignatureMismatch);
                }

                if authored_claims_capped_behavior(
                    &authored.intent,
                    authored_method.intent.as_str(),
                ) && !body_reflects_capped_behavior(&executable_method.body_rust)
                {
                    drift_reasons.push(SemanticReasonCode::MethodBodyMissingCapBehavior);
                }
            }
            None => drift_reasons.push(SemanticReasonCode::MethodSignatureMismatch),
        }
    }
    drift_reasons.sort();
    drift_reasons.dedup();

    if !drift_reasons.is_empty() {
        let verdict = if markers.has_domain_lowering {
            SemanticVerdict::BackendOnlySemanticsLeaked
        } else {
            SemanticVerdict::SemanticDrift
        };
        return Some(SemanticReview {
            verdict,
            reason_codes: drift_reasons,
            summary: "executable lowering contradicts authored semantic claims".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
    }

    if markers.has_backend_derives || markers.has_helper_lowering {
        let mut reason_codes = vec![SemanticReasonCode::BackendOnlyExecutionMarker];
        if markers.has_helper_lowering {
            reason_codes.push(SemanticReasonCode::ProofHelperOnlyMarker);
        }
        return Some(SemanticReview {
            verdict: SemanticVerdict::BackendOnlyMeaningPreserved,
            reason_codes,
            summary: "backend-only execution markers are present without changing authored meaning"
                .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
    }

    if markers.has_domain_lowering {
        authored_surfaces.push(SemanticCitation {
            path: "methods".to_string(),
            summary: "domain methods fully described by authored intent and contracts".to_string(),
        });
        executable_surfaces.push(SemanticCitation {
            path: "methods.*.lowering.rust.body".to_string(),
            summary: "domain lowering matches authored semantic claims".to_string(),
        });
    }

    Some(SemanticReview {
        verdict: SemanticVerdict::Aligned,
        reason_codes: Vec::new(),
        summary: "authored semantics and executable lowering agree on the supported sum surface"
            .to_string(),
        authored_surfaces,
        executable_surfaces,
        evaluator_scope: EvaluatorScope::SupportedSumSurface,
    })
}

fn build_authored_packet(spec: &LoadedSpec) -> Option<SemanticAuthoredPacket> {
    let sum = spec.spec.extensions.sum.as_ref()?;
    let mut variants = build_authored_variants(sum);
    let mut methods = spec
        .spec
        .extensions
        .methods
        .iter()
        .filter(|method| !is_proof_helper_method(method))
        .map(|method| SemanticMethodPacket {
            id: method.id.clone(),
            intent: method.intent.why.clone(),
            receiver: method.receiver.clone(),
            inputs: method
                .contract
                .as_ref()
                .and_then(|contract| contract.inputs.as_ref())
                .map(|inputs| {
                    inputs
                        .iter()
                        .map(|(name, type_)| SemanticFieldPacket {
                            name: name.clone(),
                            type_: type_.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            returns: method
                .contract
                .as_ref()
                .and_then(|contract| contract.returns.clone()),
        })
        .collect::<Vec<_>>();
    variants.sort();
    methods.sort_by(|left, right| left.id.cmp(&right.id));

    Some(SemanticAuthoredPacket {
        id: spec.spec.id.clone(),
        intent: spec.spec.intent.why.clone(),
        variants,
        methods,
    })
}

fn build_executable_packet(spec: &LoadedSpec) -> Option<SemanticExecutablePacket> {
    let normalized = normalize_unit(spec.spec.clone()).ok()?;
    let NormalizedUnit::Sum(unit) = normalized else {
        return None;
    };
    let lowering = lower_sum_seam(&unit).ok()?;
    let mut variants = lowering
        .variants
        .iter()
        .map(|variant| SemanticVariantPacket {
            id: variant.id.clone(),
            fields: variant
                .fields
                .iter()
                .map(|field| SemanticFieldPacket {
                    name: field.name.clone(),
                    type_: field.type_.clone(),
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    let mut methods = lowering
        .methods
        .iter()
        .filter(|method| !is_proof_helper_lowering(method))
        .map(|method| SemanticExecutableMethodPacket {
            id: method.id.clone(),
            receiver: method
                .receiver
                .map(|receiver| receiver.as_str().to_string())
                .unwrap_or_else(|| "value".to_string()),
            inputs: method
                .inputs
                .iter()
                .map(|(name, type_)| SemanticFieldPacket {
                    name: name.clone(),
                    type_: type_.clone(),
                })
                .collect(),
            returns: method.returns.clone(),
            body_rust: method.body_rust.trim().to_string(),
        })
        .collect::<Vec<_>>();
    variants.sort();
    methods.sort_by(|left, right| left.id.cmp(&right.id));

    Some(SemanticExecutablePacket {
        id: lowering.id,
        enum_name: lowering.enum_name,
        variants,
        methods,
        derives: lowering.derives,
    })
}

fn authored_citations(
    spec: &LoadedSpec,
    authored: &SemanticAuthoredPacket,
) -> Vec<SemanticCitation> {
    let mut citations = vec![SemanticCitation {
        path: "intent.why".to_string(),
        summary: authored.intent.clone(),
    }];

    if !authored.variants.is_empty() {
        citations.push(SemanticCitation {
            path: "sum.variants".to_string(),
            summary: format!("{} authored variant(s)", authored.variants.len()),
        });
    }
    if !authored.methods.is_empty() {
        citations.push(SemanticCitation {
            path: "methods".to_string(),
            summary: format!(
                "{} semantic method(s) on {}",
                authored.methods.len(),
                spec.spec.id
            ),
        });
    }
    citations
}

fn executable_citations(
    _spec: &LoadedSpec,
    executable: &SemanticExecutablePacket,
    markers: SemanticMarkerSummary,
) -> Vec<SemanticCitation> {
    let mut citations = vec![SemanticCitation {
        path: "sum".to_string(),
        summary: format!("projects to Rust enum {}", executable.enum_name),
    }];

    if !executable.methods.is_empty() {
        citations.push(SemanticCitation {
            path: "methods.*.lowering.rust.body".to_string(),
            summary: format!("{} executable semantic method(s)", executable.methods.len()),
        });
    }
    if markers.has_backend_derives {
        citations.push(SemanticCitation {
            path: "backends.rust.derives".to_string(),
            summary: "Rust derives contribute backend-only execution metadata".to_string(),
        });
    }
    citations
}

fn summarize_markers(spec: &LoadedSpec) -> SemanticMarkerSummary {
    let summary = summarize_escape_hatch_semantic_markers(spec);
    SemanticMarkerSummary {
        has_domain_lowering: summary.has_domain_lowering,
        has_helper_lowering: summary.has_proof_helper_lowering,
        has_backend_derives: summary.has_backend_rust_derives,
    }
}

fn build_authored_variants(sum: &AuthoredSumShape) -> Vec<SemanticVariantPacket> {
    let mut variants = sum
        .variants
        .iter()
        .map(|(id, variant)| {
            let mut fields = variant
                .fields
                .iter()
                .map(|(name, field)| SemanticFieldPacket {
                    name: name.clone(),
                    type_: field.type_.clone(),
                })
                .collect::<Vec<_>>();
            fields.sort();
            SemanticVariantPacket {
                id: id.clone(),
                fields,
            }
        })
        .collect::<Vec<_>>();
    variants.sort();
    variants
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

fn is_proof_helper_lowering(method: &RustInherentMethodLowering) -> bool {
    method.id.ends_with("_holds")
        && method.returns.as_deref() == Some("bool")
        && method.inputs.is_empty()
}

fn semantic_text_is_vague(text: &str) -> bool {
    let normalized = text.trim().to_ascii_lowercase();
    let word_count = normalized
        .split_whitespace()
        .filter(|segment| !segment.is_empty())
        .count();
    word_count < 4
        || matches!(
            normalized.as_str(),
            "todo" | "tbd" | "do it" | "handle it" | "discount policy" | "support behavior"
        )
}

fn authored_claims_capped_behavior(unit_intent: &str, method_intent: &str) -> bool {
    let combined = format!(
        "{} {}",
        unit_intent.to_ascii_lowercase(),
        method_intent.to_ascii_lowercase()
    );
    [
        "cap",
        "capped",
        "never below zero",
        "not below zero",
        "at most subtotal",
    ]
    .iter()
    .any(|needle| combined.contains(needle))
}

fn body_reflects_capped_behavior(body: &str) -> bool {
    let normalized = body.replace(char::is_whitespace, "");
    normalized.contains(".min(") || normalized.contains("Decimal::ZERO")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering,
        AuthoredSumVariant, Body, Contract, Intent, SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;

    fn discount_policy_sum_spec() -> LoadedSpec {
        let mut variants = IndexMap::new();
        variants.insert("none".to_string(), AuthoredSumVariant::default());
        variants.insert(
            "fixed_amount".to_string(),
            AuthoredSumVariant {
                fields: IndexMap::from([(
                    "amount".to_string(),
                    crate::types::AuthoredField {
                        type_: "Decimal".to_string(),
                    },
                )]),
            },
        );

        LoadedSpec {
            source: SpecSource {
                file_path: "units/pricing/discount_policy.unit.spec".to_string(),
                id: "pricing/discount_policy".to_string(),
            },
            spec: SpecStruct {
                id: "pricing/discount_policy".to_string(),
                kind: "sum".to_string(),
                intent: Intent {
                    why: "Represent discount strategies that cap fixed discounts at the subtotal."
                        .to_string(),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    sum: Some(AuthoredSumShape { variants }),
                    methods: vec![AuthoredMethod {
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
                                body: "{ (*amount).min(subtotal) }".to_string(),
                            }),
                        }),
                    }],
                    backends: Some(crate::types::AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec!["Clone".to_string()],
                        }),
                    }),
                    ..UnitExtensions::default()
                },
            },
        }
    }

    #[test]
    fn semantic_review_marks_vague_authored_sum_as_under_specified() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.intent.why = "discount policy".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert!(
            review
                .reason_codes
                .contains(&SemanticReasonCode::VagueUnitIntent)
        );
    }

    #[test]
    fn semantic_review_marks_missing_cap_behavior_as_backend_leak() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods[0]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = "{ amount.clone() }".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert!(
            review
                .reason_codes
                .contains(&SemanticReasonCode::MethodBodyMissingCapBehavior)
        );
    }

    #[test]
    fn semantic_health_effect_only_demotes_supported_verdicts() {
        let review = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            reason_codes: vec![],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        };
        assert_eq!(
            semantic_health_effect(Some(&review)),
            SemanticHealthEffect::DemoteIncomplete
        );
    }
}
