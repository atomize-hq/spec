use crate::escape_hatch::{is_helper_or_example_method, summarize_escape_hatch_semantic_markers};
use crate::generator::lower_sum_seam;
use crate::normalizer::normalize_unit;
use crate::types::{AuthoredSumShape, LoadedSpec, NormalizedUnit, UnitKind};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    OutsideHonestSupportedSubset,
    UnsupportedSurface,
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
    pub compatibility_key: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedSemanticRole {
    DiscountAmount,
    DiscountedSubtotal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedBodyClassification {
    Aligned,
    Contradictory,
    OutsideHonestSubset,
}

const SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY: &str = "sum.discount_policy.v1";

fn unsupported_surface_compatibility_key(unit_kind: UnitKind) -> String {
    format!("unsupported.{}.v1", unit_kind.as_str())
}

pub fn project_semantic_review(
    spec: &LoadedSpec,
    existing: Option<&SemanticReview>,
    mode: SemanticProjectionMode,
) -> Option<SemanticReview> {
    let unit_kind = spec.spec.unit_kind().ok()?;
    match evaluator_scope_for_kind(unit_kind) {
        EvaluatorScope::SupportedSumSurface => match mode {
            SemanticProjectionMode::Preserve => existing
                .filter(|review| {
                    review.evaluator_scope == EvaluatorScope::SupportedSumSurface
                        && review.compatibility_key == SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY
                })
                .cloned(),
            SemanticProjectionMode::Refresh => evaluate_supported_sum_semantic_review(spec),
        },
        EvaluatorScope::UnsupportedSurface => match mode {
            SemanticProjectionMode::Preserve => None,
            SemanticProjectionMode::Refresh => Some(unsupported_surface_review(unit_kind)),
        },
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
    let unit_kind = spec.spec.unit_kind().ok()?;
    match evaluator_scope_for_kind(unit_kind) {
        EvaluatorScope::SupportedSumSurface => evaluate_supported_sum_semantic_review(spec),
        EvaluatorScope::UnsupportedSurface => Some(unsupported_surface_review(unit_kind)),
    }
}

fn evaluator_scope_for_kind(unit_kind: UnitKind) -> EvaluatorScope {
    match unit_kind {
        UnitKind::Sum => EvaluatorScope::SupportedSumSurface,
        UnitKind::Function | UnitKind::Data => EvaluatorScope::UnsupportedSurface,
    }
}

fn unsupported_surface_review(unit_kind: UnitKind) -> SemanticReview {
    SemanticReview {
        verdict: SemanticVerdict::UnderSpecified,
        compatibility_key: unsupported_surface_compatibility_key(unit_kind),
        reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
        summary: format!(
            "unit kind '{}' is not evaluated by the M15 semantic reviewer",
            unit_kind.as_str()
        ),
        authored_surfaces: vec![],
        executable_surfaces: vec![],
        evaluator_scope: EvaluatorScope::UnsupportedSurface,
    }
}

fn evaluate_supported_sum_semantic_review(spec: &LoadedSpec) -> Option<SemanticReview> {
    debug_assert!(matches!(spec.spec.unit_kind(), Ok(UnitKind::Sum)));

    let authored = build_authored_packet(spec)?;
    let helper_method_ids = spec
        .spec
        .extensions
        .methods
        .iter()
        .filter(|method| is_helper_or_example_method(method))
        .map(|method| method.id.clone())
        .collect::<HashSet<_>>();
    let executable = build_executable_packet(spec, &helper_method_ids)?;
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
        if supported_role_for_method(method).is_none() {
            reasons.push(SemanticReasonCode::OutsideHonestSupportedSubset);
        }
    }
    reasons.sort();
    reasons.dedup();

    if !reasons.is_empty() {
        return Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
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
    let mut under_specified_reasons = Vec::new();

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

                match classify_supported_role_body(
                    supported_role_for_method(authored_method)
                        .expect("non-helper methods are pre-filtered to supported roles"),
                    &executable_method.body_rust,
                ) {
                    SupportedBodyClassification::Aligned => {}
                    SupportedBodyClassification::Contradictory => {
                        drift_reasons.push(SemanticReasonCode::MethodBodyMissingCapBehavior);
                    }
                    SupportedBodyClassification::OutsideHonestSubset => {
                        under_specified_reasons
                            .push(SemanticReasonCode::OutsideHonestSupportedSubset);
                    }
                }
            }
            None => drift_reasons.push(SemanticReasonCode::MethodSignatureMismatch),
        }
    }
    under_specified_reasons.sort();
    under_specified_reasons.dedup();

    if !under_specified_reasons.is_empty() {
        return Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
            reason_codes: under_specified_reasons,
            summary: "supported semantic bodies fall outside the honest evaluator subset"
                .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
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
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
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
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
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
        compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
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
        .filter(|method| !is_helper_or_example_method(method))
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

fn build_executable_packet(
    spec: &LoadedSpec,
    helper_method_ids: &HashSet<String>,
) -> Option<SemanticExecutablePacket> {
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
        .filter(|method| !helper_method_ids.contains(&method.id))
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

fn supported_role_for_method(method: &SemanticMethodPacket) -> Option<SupportedSemanticRole> {
    if method.receiver != "shared_ref" || !type_is_decimal(method.returns.as_deref()?) {
        return None;
    }

    let [subtotal] = method.inputs.as_slice() else {
        return None;
    };
    if subtotal.name != "subtotal" || !type_is_decimal(&subtotal.type_) {
        return None;
    }

    match method.id.as_str() {
        "discount_amount" => Some(SupportedSemanticRole::DiscountAmount),
        "discounted_subtotal" => Some(SupportedSemanticRole::DiscountedSubtotal),
        _ => None,
    }
}

fn type_is_decimal(type_name: &str) -> bool {
    type_name
        .rsplit("::")
        .next()
        .is_some_and(|segment| segment == "Decimal")
}

fn classify_supported_role_body(
    role: SupportedSemanticRole,
    body: &str,
) -> SupportedBodyClassification {
    let Ok(block) = syn::parse_str::<syn::Block>(body) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };

    match role {
        SupportedSemanticRole::DiscountAmount => classify_discount_amount_body(&block),
        SupportedSemanticRole::DiscountedSubtotal => classify_discounted_subtotal_body(&block),
    }
}

fn classify_discount_amount_body(block: &syn::Block) -> SupportedBodyClassification {
    let Some(tail_expr) = block_tail_expr(block) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    let Some(match_expr) = strip_expr_wrappers(tail_expr).and_then(expr_as_match) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    if !expr_is_ident(&match_expr.expr, "self") {
        return SupportedBodyClassification::OutsideHonestSubset;
    }

    let mut none_arm = None;
    let mut percentage_arm = None;
    let mut fixed_amount_arm = None;
    for arm in &match_expr.arms {
        if arm.guard.is_some() {
            return SupportedBodyClassification::OutsideHonestSubset;
        }
        match variant_name_from_pat(&arm.pat) {
            Some("none") if none_arm.is_none() => none_arm = Some(&arm.body),
            Some("percentage") if percentage_arm.is_none() => percentage_arm = Some(&arm.body),
            Some("fixed_amount") if fixed_amount_arm.is_none() => {
                fixed_amount_arm = Some(&arm.body)
            }
            _ => return SupportedBodyClassification::OutsideHonestSubset,
        }
    }

    if !none_arm.is_some_and(|expr| expr_is_decimal_zero(expr))
        || !percentage_arm.is_some_and(|expr| expr_is_subtotal_times_rate(expr))
    {
        return SupportedBodyClassification::OutsideHonestSubset;
    }

    let Some(fixed_amount_expr) = fixed_amount_arm else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    if expr_is_capped_amount(fixed_amount_expr) {
        SupportedBodyClassification::Aligned
    } else if expr_is_uncapped_amount(fixed_amount_expr) {
        SupportedBodyClassification::Contradictory
    } else {
        SupportedBodyClassification::OutsideHonestSubset
    }
}

fn classify_discounted_subtotal_body(block: &syn::Block) -> SupportedBodyClassification {
    let mut discount_aliases = HashSet::new();
    if block.stmts.len() > 1 {
        for stmt in &block.stmts[..block.stmts.len() - 1] {
            match stmt {
                syn::Stmt::Local(local) => {
                    let Some(alias) = local_ident(local) else {
                        return SupportedBodyClassification::OutsideHonestSubset;
                    };
                    let Some(init) = local
                        .init
                        .as_ref()
                        .map(|init| strip_expr_wrappers(&init.expr).unwrap_or(&init.expr))
                    else {
                        return SupportedBodyClassification::OutsideHonestSubset;
                    };
                    if !expr_is_discount_amount_call(init) {
                        return SupportedBodyClassification::OutsideHonestSubset;
                    }
                    discount_aliases.insert(alias.to_string());
                }
                _ => return SupportedBodyClassification::OutsideHonestSubset,
            }
        }
    }

    let Some(tail_expr) = block_tail_expr(block) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    if expr_is_discounted_subtotal_subtraction(tail_expr, &discount_aliases) {
        SupportedBodyClassification::Aligned
    } else {
        SupportedBodyClassification::OutsideHonestSubset
    }
}

fn block_tail_expr(block: &syn::Block) -> Option<&syn::Expr> {
    match block.stmts.last()? {
        syn::Stmt::Expr(expr, None) => Some(expr),
        _ => None,
    }
}

fn strip_expr_wrappers(expr: &syn::Expr) -> Option<&syn::Expr> {
    match expr {
        syn::Expr::Paren(inner) => strip_expr_wrappers(&inner.expr),
        syn::Expr::Group(inner) => strip_expr_wrappers(&inner.expr),
        syn::Expr::Block(inner) => block_tail_expr(&inner.block).and_then(strip_expr_wrappers),
        _ => Some(expr),
    }
}

fn expr_as_match(expr: &syn::Expr) -> Option<&syn::ExprMatch> {
    match expr {
        syn::Expr::Match(expr_match) => Some(expr_match),
        _ => None,
    }
}

fn variant_name_from_pat(pat: &syn::Pat) -> Option<&'static str> {
    let ident = match pat {
        syn::Pat::Path(path) => path.path.segments.last()?.ident.to_string(),
        syn::Pat::Struct(path) => path.path.segments.last()?.ident.to_string(),
        syn::Pat::TupleStruct(path) => path.path.segments.last()?.ident.to_string(),
        _ => return None,
    };
    match ident.as_str() {
        "None" => Some("none"),
        "Percentage" => Some("percentage"),
        "FixedAmount" => Some("fixed_amount"),
        _ => None,
    }
}

fn expr_is_decimal_zero(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Path(path) = expr else {
        return false;
    };
    path_ends_with(&path.path, &["Decimal", "ZERO"])
}

fn expr_is_subtotal_times_rate(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    matches!(binary.op, syn::BinOp::Mul(_))
        && ((expr_is_ident(&binary.left, "subtotal") && expr_is_rate_expr(&binary.right))
            || (expr_is_ident(&binary.right, "subtotal") && expr_is_rate_expr(&binary.left)))
}

fn expr_is_rate_expr(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    match expr {
        syn::Expr::Path(path) => path_is_ident(&path.path, "rate"),
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Deref(_)) => {
            expr_is_rate_expr(&unary.expr)
        }
        syn::Expr::MethodCall(call) if call.args.is_empty() => {
            matches!(call.method.to_string().as_str(), "clone" | "to_owned")
                && expr_is_rate_expr(&call.receiver)
        }
        _ => false,
    }
}

fn expr_is_capped_amount(expr: &syn::Expr) -> bool {
    expr_is_min_of_amount_and_subtotal(expr) || expr_is_explicit_capped_branch(expr)
}

fn expr_is_min_of_amount_and_subtotal(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::MethodCall(call) = expr else {
        return false;
    };
    call.method == "min"
        && call.args.len() == 1
        && ((expr_is_amount_expr(&call.receiver) && expr_is_ident(&call.args[0], "subtotal"))
            || (expr_is_ident(&call.receiver, "subtotal") && expr_is_amount_expr(&call.args[0])))
}

fn expr_is_explicit_capped_branch(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::If(expr_if) = expr else {
        return false;
    };
    let Some((_, else_branch)) = &expr_if.else_branch else {
        return false;
    };
    let Some(then_expr) = block_tail_expr(&expr_if.then_branch) else {
        return false;
    };
    let Some(else_expr) = strip_expr_wrappers(else_branch) else {
        return false;
    };

    match comparison_kind(&expr_if.cond) {
        Some(ComparisonKind::AmountAboveSubtotal) => {
            expr_is_ident(then_expr, "subtotal") && expr_is_amount_expr(else_expr)
        }
        Some(ComparisonKind::AmountAtMostSubtotal) => {
            expr_is_amount_expr(then_expr) && expr_is_ident(else_expr, "subtotal")
        }
        None => false,
    }
}

fn expr_is_uncapped_amount(expr: &syn::Expr) -> bool {
    expr_is_amount_expr(expr)
}

fn expr_is_amount_expr(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    match expr {
        syn::Expr::Path(path) => path_is_ident(&path.path, "amount"),
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Deref(_)) => {
            expr_is_amount_expr(&unary.expr)
        }
        syn::Expr::MethodCall(call) if call.args.is_empty() => {
            matches!(call.method.to_string().as_str(), "clone" | "to_owned")
                && expr_is_amount_expr(&call.receiver)
        }
        _ => false,
    }
}

fn expr_is_discounted_subtotal_subtraction(
    expr: &syn::Expr,
    discount_aliases: &HashSet<String>,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    matches!(binary.op, syn::BinOp::Sub(_))
        && expr_is_ident(&binary.left, "subtotal")
        && (expr_is_discount_amount_call(&binary.right)
            || expr_is_discount_amount_alias(&binary.right, discount_aliases))
}

fn expr_is_discount_amount_call(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::MethodCall(call) = expr else {
        return false;
    };
    call.method == "discount_amount"
        && expr_is_ident(&call.receiver, "self")
        && call.args.len() == 1
        && expr_is_ident(&call.args[0], "subtotal")
}

fn expr_is_discount_amount_alias(expr: &syn::Expr, discount_aliases: &HashSet<String>) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Path(path) = expr else {
        return false;
    };
    path.path
        .get_ident()
        .is_some_and(|ident| discount_aliases.contains(&ident.to_string()))
}

fn expr_is_ident(expr: &syn::Expr, ident: &str) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Path(path) = expr else {
        return false;
    };
    path_is_ident(&path.path, ident)
}

fn path_is_ident(path: &syn::Path, ident: &str) -> bool {
    path.get_ident().is_some_and(|segment| segment == ident)
}

fn path_ends_with(path: &syn::Path, suffix: &[&str]) -> bool {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    segments.ends_with(
        &suffix
            .iter()
            .map(|segment| (*segment).to_string())
            .collect::<Vec<_>>(),
    )
}

fn local_ident(local: &syn::Local) -> Option<&syn::Ident> {
    let syn::Pat::Ident(pat_ident) = &local.pat else {
        return None;
    };
    Some(&pat_ident.ident)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonKind {
    AmountAboveSubtotal,
    AmountAtMostSubtotal,
}

fn comparison_kind(expr: &syn::Expr) -> Option<ComparisonKind> {
    let expr = strip_expr_wrappers(expr)?;
    let syn::Expr::Binary(binary) = expr else {
        return None;
    };

    match (&binary.left, &binary.op, &binary.right) {
        (left, syn::BinOp::Gt(_), right)
            if expr_is_amount_expr(left) && expr_is_ident(right, "subtotal") =>
        {
            Some(ComparisonKind::AmountAboveSubtotal)
        }
        (left, syn::BinOp::Ge(_), right)
            if expr_is_amount_expr(left) && expr_is_ident(right, "subtotal") =>
        {
            Some(ComparisonKind::AmountAboveSubtotal)
        }
        (left, syn::BinOp::Lt(_), right)
            if expr_is_ident(left, "subtotal") && expr_is_amount_expr(right) =>
        {
            Some(ComparisonKind::AmountAboveSubtotal)
        }
        (left, syn::BinOp::Le(_), right)
            if expr_is_ident(left, "subtotal") && expr_is_amount_expr(right) =>
        {
            Some(ComparisonKind::AmountAboveSubtotal)
        }
        (left, syn::BinOp::Lt(_), right)
            if expr_is_amount_expr(left) && expr_is_ident(right, "subtotal") =>
        {
            Some(ComparisonKind::AmountAtMostSubtotal)
        }
        (left, syn::BinOp::Le(_), right)
            if expr_is_amount_expr(left) && expr_is_ident(right, "subtotal") =>
        {
            Some(ComparisonKind::AmountAtMostSubtotal)
        }
        (left, syn::BinOp::Gt(_), right)
            if expr_is_ident(left, "subtotal") && expr_is_amount_expr(right) =>
        {
            Some(ComparisonKind::AmountAtMostSubtotal)
        }
        (left, syn::BinOp::Ge(_), right)
            if expr_is_ident(left, "subtotal") && expr_is_amount_expr(right) =>
        {
            Some(ComparisonKind::AmountAtMostSubtotal)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredDataShape, AuthoredField, AuthoredMethod, AuthoredMethodLowering,
        AuthoredRustBackend, AuthoredRustMethodLowering, AuthoredSumVariant, Body, Contract,
        Intent, SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;

    fn decimal_contract_method(id: &str, intent: &str, body: &str) -> AuthoredMethod {
        AuthoredMethod {
            id: id.to_string(),
            intent: Intent {
                why: intent.to_string(),
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
                    body: body.to_string(),
                }),
            }),
        }
    }

    fn helper_method(id: &str, body: &str) -> AuthoredMethod {
        AuthoredMethod {
            id: id.to_string(),
            intent: Intent {
                why: "Support a direct proof/example helper.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: body.to_string(),
                }),
            }),
        }
    }

    fn bool_domain_predicate_method(id: &str, body: &str) -> AuthoredMethod {
        AuthoredMethod {
            id: id.to_string(),
            intent: Intent {
                why: "Report whether the current discount policy has a real domain property."
                    .to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: body.to_string(),
                }),
            }),
        }
    }

    fn aligned_discount_amount_body() -> &'static str {
        r#"{
            match self {
                Self::None => Decimal::ZERO,
                Self::Percentage { rate } => subtotal * *rate,
                Self::FixedAmount { amount } => (*amount).min(subtotal),
            }
        }"#
    }

    fn aligned_discounted_subtotal_body() -> &'static str {
        r#"{
            subtotal - self.discount_amount(subtotal)
        }"#
    }

    fn discount_policy_sum_spec() -> LoadedSpec {
        let mut variants = IndexMap::new();
        variants.insert("none".to_string(), AuthoredSumVariant::default());
        variants.insert(
            "percentage".to_string(),
            AuthoredSumVariant {
                fields: IndexMap::from([(
                    "rate".to_string(),
                    crate::types::AuthoredField {
                        type_: "Decimal".to_string(),
                    },
                )]),
            },
        );
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
                    methods: vec![
                        decimal_contract_method(
                            "discount_amount",
                            "Return the capped discount amount to subtract from the subtotal.",
                            aligned_discount_amount_body(),
                        ),
                        decimal_contract_method(
                            "discounted_subtotal",
                            "Return the subtotal after applying the selected discount strategy.",
                            aligned_discounted_subtotal_body(),
                        ),
                    ],
                    backends: None,
                    ..UnitExtensions::default()
                },
            },
        }
    }

    fn discount_policy_function_spec() -> LoadedSpec {
        let mut spec = discount_policy_sum_spec();
        spec.spec.kind = "function".to_string();
        spec.spec.contract = Some(Contract {
            inputs: Some(IndexMap::from([(
                "subtotal".to_string(),
                "Decimal".to_string(),
            )])),
            returns: Some("Decimal".to_string()),
            invariants: vec![],
        });
        spec.spec.body = Body {
            rust: "{ subtotal }".to_string(),
        };
        spec.spec.extensions = UnitExtensions::default();
        spec
    }

    fn discount_policy_sum_spec_with_backend_markers() -> LoadedSpec {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.backends = Some(crate::types::AuthoredBackends {
            rust: Some(AuthoredRustBackend {
                derives: vec!["Clone".to_string()],
            }),
        });
        spec
    }

    fn discount_policy_data_spec() -> LoadedSpec {
        let mut spec = discount_policy_sum_spec();
        spec.spec.kind = "data".to_string();
        spec.spec.contract = None;
        spec.spec.body = Body::default();
        spec.spec.extensions = UnitExtensions {
            data: Some(AuthoredDataShape {
                fields: IndexMap::from([(
                    "subtotal".to_string(),
                    AuthoredField {
                        type_: "Decimal".to_string(),
                    },
                )]),
            }),
            ..UnitExtensions::default()
        };
        spec
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
    fn semantic_review_marks_aligned_discount_amount_and_discounted_subtotal() {
        let review = evaluate_semantic_review(&discount_policy_sum_spec()).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(review.reason_codes, Vec::<SemanticReasonCode>::new());
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
            .body = "{ match self { Self::None => Decimal::ZERO, Self::Percentage { rate } => subtotal * *rate, Self::FixedAmount { amount } => amount.clone() } }".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert!(
            review
                .reason_codes
                .contains(&SemanticReasonCode::MethodBodyMissingCapBehavior)
        );
    }

    #[test]
    fn semantic_review_marks_backend_only_semantics_leaked_when_markers_present() {
        let mut spec = discount_policy_sum_spec_with_backend_markers();
        spec.spec.extensions.methods[0]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = "{ match self { Self::None => Decimal::ZERO, Self::Percentage { rate } => subtotal * *rate, Self::FixedAmount { amount } => amount.clone() } }".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::MethodBodyMissingCapBehavior]
        );
    }

    #[test]
    fn semantic_review_helper_example_decimal_zero_does_not_mask_drift() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods.push(helper_method(
            "fixed_amount_capped_example",
            r#"{
                Decimal::ZERO == Decimal::ZERO
            }"#,
        ));
        spec.spec.extensions.methods[0]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = "{ match self { Self::None => Decimal::ZERO, Self::Percentage { rate } => subtotal * *rate, Self::FixedAmount { amount } => amount.clone() } }".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::MethodBodyMissingCapBehavior]
        );
    }

    #[test]
    fn semantic_review_holds_helper_does_not_mask_drift() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods.push(helper_method(
            "fixed_amount_capped_behavior_holds",
            r#"{
                Decimal::ZERO == Decimal::ZERO
            }"#,
        ));
        spec.spec.extensions.methods[0]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = "{ match self { Self::None => Decimal::ZERO, Self::Percentage { rate } => subtotal * *rate, Self::FixedAmount { amount } => amount.clone() } }".to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::MethodBodyMissingCapBehavior]
        );
    }

    #[test]
    fn semantic_review_marks_extra_non_helper_method_as_under_specified() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods.push(decimal_contract_method(
            "preview_discount_label",
            "Return a preview amount for the current discount policy.",
            "{ subtotal }",
        ));
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::OutsideHonestSupportedSubset]
        );
    }

    #[test]
    fn semantic_review_marks_bool_domain_predicate_as_under_specified() {
        let mut spec = discount_policy_sum_spec();
        spec.spec
            .extensions
            .methods
            .push(bool_domain_predicate_method(
                "has_cap",
                r#"{
                matches!(self, Self::FixedAmount { .. })
            }"#,
            ));
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::OutsideHonestSupportedSubset]
        );
    }

    #[test]
    fn semantic_review_marks_unrecognized_supported_role_body_as_under_specified() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods[1]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = r#"{
                if subtotal == Decimal::ZERO {
                    subtotal
                } else {
                    subtotal - self.discount_amount(subtotal)
                }
            }"#
        .to_string();
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::OutsideHonestSupportedSubset]
        );
    }

    #[test]
    fn semantic_review_reports_backend_only_meaning_preserved_for_helper_markers() {
        let mut spec = discount_policy_sum_spec();
        spec.spec.extensions.methods.push(helper_method(
            "percentage_example",
            r#"{
                let policy = Self::Percentage { rate: Decimal::new(10, 2) };
                policy.discounted_subtotal(Decimal::new(10000, 2)) == Decimal::new(9000, 2)
            }"#,
        ));
        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlyMeaningPreserved);
        assert_eq!(
            review.reason_codes,
            vec![
                SemanticReasonCode::BackendOnlyExecutionMarker,
                SemanticReasonCode::ProofHelperOnlyMarker,
            ]
        );
    }

    #[test]
    fn semantic_health_effect_only_demotes_supported_verdicts() {
        let supported_review = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
            reason_codes: vec![],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        };
        assert_eq!(
            semantic_health_effect(Some(&supported_review)),
            SemanticHealthEffect::DemoteIncomplete
        );

        let unsupported_review =
            evaluate_semantic_review(&discount_policy_function_spec()).unwrap();
        assert_eq!(
            semantic_health_effect(Some(&unsupported_review)),
            SemanticHealthEffect::KeepBase
        );
    }

    #[test]
    fn semantic_review_emits_explicit_unsupported_surface_for_function_and_data() {
        for spec in [discount_policy_function_spec(), discount_policy_data_spec()] {
            let review = evaluate_semantic_review(&spec).unwrap();
            assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
            assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
            assert_eq!(
                review.compatibility_key,
                unsupported_surface_compatibility_key(spec.spec.unit_kind().unwrap())
            );
            assert_eq!(
                review.reason_codes,
                vec![SemanticReasonCode::UnsupportedSurface]
            );
            assert!(
                review
                    .summary
                    .contains("is not evaluated by the M15 semantic reviewer"),
                "{}",
                review.summary
            );
            assert!(review.authored_surfaces.is_empty());
            assert!(review.executable_surfaces.is_empty());
        }
    }

    #[test]
    fn project_semantic_review_preserve_keeps_matching_sum_compatibility_key() {
        let spec = discount_policy_sum_spec();
        let review = evaluate_semantic_review(&spec).unwrap();

        let preserved =
            project_semantic_review(&spec, Some(&review), SemanticProjectionMode::Preserve)
                .unwrap();

        assert_eq!(preserved, review);
    }

    #[test]
    fn project_semantic_review_preserve_drops_mismatched_supported_compatibility_key() {
        let spec = discount_policy_sum_spec();
        let mut supported_review = evaluate_semantic_review(&spec).unwrap();
        supported_review.compatibility_key = "sum.discount_policy.v0".to_string();

        let preserved = project_semantic_review(
            &spec,
            Some(&supported_review),
            SemanticProjectionMode::Preserve,
        );

        assert!(preserved.is_none());
    }

    #[test]
    fn project_semantic_review_preserve_drops_unsupported_surface_review_even_with_compatibility_key(
    ) {
        let unsupported_review =
            evaluate_semantic_review(&discount_policy_function_spec()).unwrap();

        let preserved = project_semantic_review(
            &discount_policy_sum_spec(),
            Some(&unsupported_review),
            SemanticProjectionMode::Preserve,
        );

        assert!(preserved.is_none());
    }

    #[test]
    fn project_semantic_review_only_refresh_synthesizes_fresh_unsupported_metadata() {
        let spec = discount_policy_function_spec();
        let existing = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: "unsupported.function.v0".to_string(),
            reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
            summary: "stale unsupported summary".to_string(),
            authored_surfaces: vec![SemanticCitation {
                path: "intent.why".to_string(),
                summary: "stale".to_string(),
            }],
            executable_surfaces: vec![SemanticCitation {
                path: "body.rust".to_string(),
                summary: "stale".to_string(),
            }],
            evaluator_scope: EvaluatorScope::UnsupportedSurface,
        };

        let preserved =
            project_semantic_review(&spec, Some(&existing), SemanticProjectionMode::Preserve);
        let refreshed =
            project_semantic_review(&spec, Some(&existing), SemanticProjectionMode::Refresh)
                .unwrap();

        let expected_summary = "unit kind 'function' is not evaluated by the M15 semantic reviewer";
        assert!(preserved.is_none());
        assert_eq!(refreshed.summary, expected_summary);
        assert!(refreshed.authored_surfaces.is_empty());
        assert!(refreshed.executable_surfaces.is_empty());
    }
}
