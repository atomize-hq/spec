use crate::generator::{lower_data_seam, lower_sum_seam};
use crate::normalizer::normalize_unit;
use crate::portability::{
    PortabilityContaminationSummary, PortabilityMarkerSummary, summarize_portability_contamination,
    summarize_portability_markers,
};
use crate::portability_contract::{is_helper_or_example_method, is_portability_seam_kind};
use crate::types::{
    AuthoredDataShape, AuthoredSumShape, DepRef, LoadedSpec, NormalizedUnit, UnitKind,
    callable_name,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
    FunctionBodyContradictsSemanticIntent,
    BackendOnlyExecutionMarker,
    ProofHelperOnlyMarker,
    DomainLoweringPresent,
    OutsideHonestSupportedSubset,
    UnsupportedSurface,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvaluatorScope {
    SupportedFunctionSurface,
    SupportedSumSurface,
    SupportedDataSurface,
    UnsupportedSurface,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticSupportStatus {
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFunctionReasonCode {
    UnsupportedControlFlow,
    UnsupportedDepTopology,
    UnsupportedRequiredArgumentExpression,
    UnsupportedWrapperBodyShape,
    UnsupportedArithmeticShape,
    UnsupportedFunctionSurface,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnsupportedFunctionDiagnostic {
    reason_code: UnsupportedFunctionReasonCode,
    summary: &'static str,
    rewrite_hints: &'static [&'static str],
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<SemanticSupportStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_reason_codes: Vec<UnsupportedFunctionReasonCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rewrite_hints: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<SemanticReasonCode>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authored_surfaces: Vec<SemanticCitation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub executable_surfaces: Vec<SemanticCitation>,
    pub evaluator_scope: EvaluatorScope,
}

impl SemanticReview {
    pub fn effective_support_status(&self) -> SemanticSupportStatus {
        if let Some(status) = self.support_status {
            return status;
        }

        match self.evaluator_scope {
            EvaluatorScope::UnsupportedSurface => SemanticSupportStatus::Unsupported,
            EvaluatorScope::SupportedFunctionSurface
            | EvaluatorScope::SupportedSumSurface
            | EvaluatorScope::SupportedDataSurface => {
                if self.compatibility_key.starts_with("unsupported.")
                    && self.compatibility_key.ends_with(".v1")
                {
                    SemanticSupportStatus::Unsupported
                } else {
                    SemanticSupportStatus::Supported
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnsupportedFunctionShapeFingerprint {
    pub schema_version: u8,
    pub function_dep_arity: usize,
    pub callable_dep_topology_class: UnsupportedFunctionDepTopologyClass,
    pub contract_input_count: usize,
    pub has_return: bool,
    pub authored_body_kind: UnsupportedFunctionAuthoredBodyKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFunctionDepTopologyClass {
    NoDepsOrHelper,
    SupportedCallablePair,
    UnsupportedCallablePair,
    SupportedCallableTriple,
    UnsupportedCallableTriple,
    Fanout,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFunctionAuthoredBodyKind {
    WrapperLike,
    ArithmeticLike,
    Neither,
}

impl UnsupportedFunctionShapeFingerprint {
    pub fn from_spec(spec: &LoadedSpec) -> Option<Self> {
        let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
        let context = SemanticReviewContext::new(&specs_by_id);
        Self::from_spec_with_context(spec, &context)
    }

    pub fn from_spec_with_context(
        spec: &LoadedSpec,
        context: &SemanticReviewContext<'_>,
    ) -> Option<Self> {
        let review = evaluate_semantic_review_with_context(spec, context)?;
        if !review_describes_unsupported_function_shape(&review) {
            return None;
        }
        let authored = build_authored_function_packet(spec)?;

        Some(Self {
            schema_version: 1,
            function_dep_arity: authored.deps.len(),
            callable_dep_topology_class: unsupported_function_dep_topology_class(
                &authored, context,
            ),
            contract_input_count: authored.inputs.len(),
            has_return: authored.returns.is_some(),
            authored_body_kind: unsupported_function_authored_body_kind(&authored),
        })
    }

    pub fn as_key(&self) -> String {
        serde_json::to_string(self).expect("unsupported-function fingerprint must serialize")
    }
}

pub fn unsupported_function_shape_fingerprint(spec: &LoadedSpec) -> Option<String> {
    UnsupportedFunctionShapeFingerprint::from_spec(spec).map(|fingerprint| fingerprint.as_key())
}

pub fn unsupported_function_shape_fingerprint_with_context(
    spec: &LoadedSpec,
    context: &SemanticReviewContext<'_>,
) -> Option<String> {
    UnsupportedFunctionShapeFingerprint::from_spec_with_context(spec, context)
        .map(|fingerprint| fingerprint.as_key())
}

fn review_describes_unsupported_function_shape(review: &SemanticReview) -> bool {
    review.effective_support_status() == SemanticSupportStatus::Unsupported
        && (review
            .compatibility_key
            .starts_with("unsupported.function.")
            || !review.unsupported_reason_codes.is_empty())
}

fn unsupported_function_dep_topology_class(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
) -> UnsupportedFunctionDepTopologyClass {
    match authored.deps.len() {
        0 | 1 => UnsupportedFunctionDepTopologyClass::NoDepsOrHelper,
        2 => {
            let mut stack = HashSet::new();
            if family_b_deps_are_supported(authored, context, &mut stack) {
                UnsupportedFunctionDepTopologyClass::SupportedCallablePair
            } else {
                UnsupportedFunctionDepTopologyClass::UnsupportedCallablePair
            }
        }
        3 => {
            let mut stack = HashSet::new();
            if family_c_deps_are_supported(authored, context, &mut stack) {
                UnsupportedFunctionDepTopologyClass::SupportedCallableTriple
            } else {
                UnsupportedFunctionDepTopologyClass::UnsupportedCallableTriple
            }
        }
        _ => UnsupportedFunctionDepTopologyClass::Fanout,
    }
}

fn unsupported_function_authored_body_kind(
    authored: &SemanticAuthoredFunctionPacket,
) -> UnsupportedFunctionAuthoredBodyKind {
    if authored_function_looks_like_wrapper_contract(authored)
        || authored_function_looks_like_chain3_wrapper_contract(authored)
    {
        UnsupportedFunctionAuthoredBodyKind::WrapperLike
    } else if authored_function_looks_like_arithmetic_contract(authored) {
        UnsupportedFunctionAuthoredBodyKind::ArithmeticLike
    } else {
        UnsupportedFunctionAuthoredBodyKind::Neither
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticAuthoredFunctionPacket {
    pub id: String,
    pub intent: String,
    pub fn_name: String,
    pub inputs: Vec<SemanticFieldPacket>,
    pub returns: Option<String>,
    pub invariants: Vec<String>,
    pub deps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_typescript: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticExecutableFunctionPacket {
    pub id: String,
    pub fn_name: String,
    pub inputs: Vec<SemanticFieldPacket>,
    pub returns: Option<String>,
    pub body_rust: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticAuthoredDataPacket {
    pub id: String,
    pub intent: String,
    pub fields: Vec<SemanticFieldPacket>,
    pub constructors: Vec<SemanticConstructorPacket>,
    pub methods: Vec<SemanticMethodPacket>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticExecutableDataPacket {
    pub id: String,
    pub struct_name: String,
    pub fields: Vec<SemanticFieldPacket>,
    pub constructors: Vec<SemanticConstructorPacket>,
    pub methods: Vec<SemanticExecutableMethodPacket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticConstructorPacket {
    pub id: String,
    pub inputs: Vec<SemanticFieldPacket>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct SupportedSeamPortabilitySummary {
    markers: PortabilityMarkerSummary,
    contamination: PortabilityContaminationSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedSemanticRole {
    DiscountAmount,
    DiscountedSubtotal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedDataSemanticRole {
    DiscountedSubtotal,
    Total,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedFunctionFamily {
    FamilyC,
    FamilyA(FamilyAFunctionRole),
    FamilyB,
    HelperIdentityPassthrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FamilyAFunctionRole {
    MonotoneDownNonnegative,
    MonotoneUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HelperIdentityPassthroughIntentRole {
    Passthrough,
    RoundLike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HelperIdentityPassthroughBodyKind {
    DirectPassthrough,
    RoundLike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedFunctionRoute {
    WrapperPipelineChain3,
    WrapperPipeline,
    ArithmeticLeafMonotoneDownNonnegative,
    ArithmeticLeafMonotoneUp,
    HelperIdentityPassthrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedSurface {
    Function(SupportedFunctionFamily),
    SumDiscountPolicy,
    DataCheckoutQuote,
    Unsupported(UnitKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedBodyClassification {
    Aligned,
    Contradictory,
    OutsideHonestSubset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FamilyBBodyClassification {
    Aligned,
    SemanticDrift,
    UnderSpecified,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FamilyBArgClassification {
    Expected,
    WrongParam,
    UnsupportedExpr,
}

const SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY: &str = "sum.discount_policy.v1";
const DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY: &str = "data.checkout_quote.v1";
const FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY: &str =
    "function.wrapper.pipeline.chain3.v1";
const FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_down_nonnegative.v1";
const FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_up.v1";
const FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.v1";
const FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY: &str =
    "function.helper.identity_passthrough.v1";
const UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY: &str = "unsupported.function.v1";
const FAMILY_A_INVARIANT_OUTPUT_LE_INPUT0: &str = "output <= input0";
const FAMILY_A_INVARIANT_OUTPUT_GE_ZERO: &str = "output >= 0";
const FAMILY_A_INVARIANT_OUTPUT_GE_INPUT0: &str = "output >= input0";
const SUPPORTED_FUNCTION_ROUTING_ORDER: [SupportedFunctionRoute; 5] = [
    SupportedFunctionRoute::WrapperPipelineChain3,
    SupportedFunctionRoute::WrapperPipeline,
    SupportedFunctionRoute::ArithmeticLeafMonotoneDownNonnegative,
    SupportedFunctionRoute::ArithmeticLeafMonotoneUp,
    SupportedFunctionRoute::HelperIdentityPassthrough,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SupportedFunctionDep<'a> {
    callable_name: &'a str,
    input_arity: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SemanticReviewContext<'a> {
    specs_by_id: &'a HashMap<String, LoadedSpec>,
}

impl<'a> SemanticReviewContext<'a> {
    pub fn new(specs_by_id: &'a HashMap<String, LoadedSpec>) -> Self {
        Self { specs_by_id }
    }

    fn resolve_dep_spec(self, dep: &str) -> Option<&'a LoadedSpec> {
        let parsed = DepRef::parse(dep).ok()?;
        if parsed.library_alias().is_some() {
            return None;
        }
        self.specs_by_id.get(parsed.unit_id())
    }
}

impl SupportedSurface {
    fn compatibility_key(self) -> Option<&'static str> {
        match self {
            Self::Function(SupportedFunctionFamily::FamilyC) => {
                Some(FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY)
            }
            Self::Function(SupportedFunctionFamily::FamilyA(
                FamilyAFunctionRole::MonotoneDownNonnegative,
            )) => Some(FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY),
            Self::Function(SupportedFunctionFamily::FamilyA(FamilyAFunctionRole::MonotoneUp)) => {
                Some(FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY)
            }
            Self::Function(SupportedFunctionFamily::FamilyB) => {
                Some(FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY)
            }
            Self::Function(SupportedFunctionFamily::HelperIdentityPassthrough) => {
                Some(FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY)
            }
            Self::SumDiscountPolicy => Some(SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY),
            Self::DataCheckoutQuote => Some(DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY),
            Self::Unsupported(_) => None,
        }
    }

    fn evaluator_scope(self) -> EvaluatorScope {
        match self {
            Self::Function(_) => EvaluatorScope::SupportedFunctionSurface,
            Self::SumDiscountPolicy => EvaluatorScope::SupportedSumSurface,
            Self::DataCheckoutQuote => EvaluatorScope::SupportedDataSurface,
            Self::Unsupported(_) => EvaluatorScope::UnsupportedSurface,
        }
    }
}

impl SupportedFunctionFamily {
    fn input_arity(self) -> usize {
        match self {
            Self::FamilyC => 5,
            Self::FamilyA(_) => 2,
            Self::FamilyB => 3,
            Self::HelperIdentityPassthrough => 1,
        }
    }
}

impl SupportedFunctionRoute {
    #[cfg(test)]
    fn compatibility_key(self) -> &'static str {
        match self {
            Self::WrapperPipelineChain3 => FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY,
            Self::WrapperPipeline => FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY,
            Self::ArithmeticLeafMonotoneDownNonnegative => {
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
            }
            Self::ArithmeticLeafMonotoneUp => {
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
            }
            Self::HelperIdentityPassthrough => {
                FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY
            }
        }
    }

    fn try_match(
        self,
        authored: &SemanticAuthoredFunctionPacket,
        executable: &SemanticExecutableFunctionPacket,
        context: &SemanticReviewContext<'_>,
        stack: &mut HashSet<String>,
    ) -> Option<SupportedFunctionFamily> {
        match self {
            Self::WrapperPipelineChain3 => {
                if family_c_authored_contract_is_supported(authored, context, stack)
                    && !matches!(
                        classify_family_c_function_body(authored, executable, context, stack),
                        FamilyBBodyClassification::Unsupported
                    )
                {
                    Some(SupportedFunctionFamily::FamilyC)
                } else {
                    None
                }
            }
            Self::WrapperPipeline => {
                if family_b_authored_contract_is_supported(authored, context, stack)
                    && !matches!(
                        classify_family_b_function_body(authored, executable),
                        FamilyBBodyClassification::Unsupported
                    )
                {
                    Some(SupportedFunctionFamily::FamilyB)
                } else {
                    None
                }
            }
            Self::ArithmeticLeafMonotoneDownNonnegative => {
                let role = FamilyAFunctionRole::MonotoneDownNonnegative;
                if family_a_authored_role(authored) == Some(role)
                    && !matches!(
                        classify_family_a_body(role, authored, executable),
                        SupportedBodyClassification::OutsideHonestSubset
                    )
                {
                    Some(SupportedFunctionFamily::FamilyA(role))
                } else {
                    None
                }
            }
            Self::ArithmeticLeafMonotoneUp => {
                let role = FamilyAFunctionRole::MonotoneUp;
                if family_a_authored_role(authored) == Some(role)
                    && !matches!(
                        classify_family_a_body(role, authored, executable),
                        SupportedBodyClassification::OutsideHonestSubset
                    )
                {
                    Some(SupportedFunctionFamily::FamilyA(role))
                } else {
                    None
                }
            }
            Self::HelperIdentityPassthrough => {
                if helper_identity_passthrough_contract_is_supported(authored)
                    && helper_identity_passthrough_body_kind(executable).is_some()
                {
                    Some(SupportedFunctionFamily::HelperIdentityPassthrough)
                } else {
                    None
                }
            }
        }
    }
}

fn unsupported_surface_compatibility_key(unit_kind: UnitKind) -> String {
    format!("unsupported.{}.v1", unit_kind.as_str())
}

pub fn project_semantic_review(
    spec: &LoadedSpec,
    existing: Option<&SemanticReview>,
    mode: SemanticProjectionMode,
) -> Option<SemanticReview> {
    let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
    let context = SemanticReviewContext::new(&specs_by_id);
    project_semantic_review_with_context(spec, existing, mode, &context)
}

pub fn project_semantic_review_with_context(
    spec: &LoadedSpec,
    existing: Option<&SemanticReview>,
    mode: SemanticProjectionMode,
    context: &SemanticReviewContext<'_>,
) -> Option<SemanticReview> {
    let mut stack = HashSet::new();
    match supported_surface_for_spec(spec, context, &mut stack)? {
        surface @ (SupportedSurface::Function(_)
        | SupportedSurface::SumDiscountPolicy
        | SupportedSurface::DataCheckoutQuote) => match mode {
            SemanticProjectionMode::Preserve => existing
                .filter(|review| {
                    review.evaluator_scope == surface.evaluator_scope()
                        && review.compatibility_key
                            == surface
                                .compatibility_key()
                                .expect("supported surface compatibility key")
                })
                .cloned(),
            SemanticProjectionMode::Refresh => {
                evaluate_supported_semantic_review(spec, surface, context, &mut stack)
            }
        },
        SupportedSurface::Unsupported(unit_kind) => match mode {
            SemanticProjectionMode::Preserve => None,
            SemanticProjectionMode::Refresh => Some(match unit_kind {
                UnitKind::Function => unsupported_function_review(spec, context, &mut stack),
                kind if is_portability_seam_kind(kind) => unsupported_surface_review(kind),
                _ => unreachable!("non-function unit kinds are portability seams"),
            }),
        },
    }
}

pub fn semantic_health_effect(review: Option<&SemanticReview>) -> SemanticHealthEffect {
    let Some(review) = review else {
        return SemanticHealthEffect::KeepBase;
    };
    if !matches!(
        review.effective_support_status(),
        SemanticSupportStatus::Supported
    ) {
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
    let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
    let context = SemanticReviewContext::new(&specs_by_id);
    evaluate_semantic_review_with_context(spec, &context)
}

pub fn evaluate_semantic_review_with_context(
    spec: &LoadedSpec,
    context: &SemanticReviewContext<'_>,
) -> Option<SemanticReview> {
    let mut stack = HashSet::new();
    match supported_surface_for_spec(spec, context, &mut stack)? {
        surface @ (SupportedSurface::Function(_)
        | SupportedSurface::SumDiscountPolicy
        | SupportedSurface::DataCheckoutQuote) => {
            evaluate_supported_semantic_review(spec, surface, context, &mut stack)
        }
        SupportedSurface::Unsupported(unit_kind) => Some(match unit_kind {
            UnitKind::Function => unsupported_function_review(spec, context, &mut stack),
            kind if is_portability_seam_kind(kind) => unsupported_surface_review(kind),
            _ => unreachable!("non-function unit kinds are portability seams"),
        }),
    }
}

fn supported_surface_for_spec(
    spec: &LoadedSpec,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<SupportedSurface> {
    if !stack.insert(spec.spec.id.clone()) {
        return Some(SupportedSurface::Unsupported(spec.spec.unit_kind().ok()?));
    }
    let unit_kind = spec.spec.unit_kind().ok()?;
    let surface = match unit_kind {
        UnitKind::Function => supported_function_surface(spec, context, stack)
            .map(SupportedSurface::Function)
            .unwrap_or(SupportedSurface::Unsupported(UnitKind::Function)),
        UnitKind::Sum if spec.spec.id == "pricing/discount_policy" => {
            SupportedSurface::SumDiscountPolicy
        }
        UnitKind::Data if spec.spec.id == "pricing/checkout_quote" => {
            SupportedSurface::DataCheckoutQuote
        }
        kind if is_portability_seam_kind(kind) => SupportedSurface::Unsupported(kind),
        _ => unreachable!("all unit kinds are covered above"),
    };
    stack.remove(&spec.spec.id);
    Some(surface)
}

fn evaluate_supported_semantic_review(
    spec: &LoadedSpec,
    surface: SupportedSurface,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<SemanticReview> {
    match surface {
        SupportedSurface::Function(family) => {
            evaluate_supported_function_semantic_review(spec, family, context, stack)
        }
        SupportedSurface::SumDiscountPolicy => evaluate_supported_sum_semantic_review(spec),
        SupportedSurface::DataCheckoutQuote => evaluate_supported_checkout_quote_data_review(spec),
        SupportedSurface::Unsupported(_) => None,
    }
}

fn unsupported_surface_review(unit_kind: UnitKind) -> SemanticReview {
    SemanticReview {
        verdict: SemanticVerdict::UnderSpecified,
        compatibility_key: unsupported_surface_compatibility_key(unit_kind),
        support_status: (unit_kind == UnitKind::Function)
            .then_some(SemanticSupportStatus::Unsupported),
        unsupported_reason_codes: Vec::new(),
        rewrite_hints: Vec::new(),
        reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
        summary: format!(
            "this '{}' surface is not evaluated by the semantic reviewer for this unit",
            unit_kind.as_str()
        ),
        authored_surfaces: vec![],
        executable_surfaces: vec![],
        evaluator_scope: EvaluatorScope::UnsupportedSurface,
    }
}

impl UnsupportedFunctionDiagnostic {
    fn review(
        &self,
        authored_surfaces: Vec<SemanticCitation>,
        executable_surfaces: Vec<SemanticCitation>,
    ) -> SemanticReview {
        SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY.to_string(),
            support_status: Some(SemanticSupportStatus::Unsupported),
            unsupported_reason_codes: vec![self.reason_code],
            rewrite_hints: self
                .rewrite_hints
                .iter()
                .map(|hint| (*hint).to_string())
                .collect(),
            reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
            summary: self.summary.to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::UnsupportedSurface,
        }
    }
}

fn unsupported_function_review(
    spec: &LoadedSpec,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> SemanticReview {
    let authored = build_authored_function_packet(spec);
    let executable = build_executable_function_packet(spec);
    let authored_surfaces = authored
        .as_ref()
        .map(authored_function_citations)
        .unwrap_or_default();
    let executable_surfaces = executable
        .as_ref()
        .map(executable_function_citations)
        .unwrap_or_default();

    let diagnostic = authored
        .as_ref()
        .zip(executable.as_ref())
        .and_then(|(authored, executable)| {
            unsupported_function_control_flow_diagnostic(executable)
                .or_else(|| unsupported_function_dep_topology_diagnostic(authored, context, stack))
                .or_else(|| {
                    unsupported_function_required_argument_expression_diagnostic(
                        authored, executable,
                    )
                })
                .or_else(|| {
                    unsupported_function_wrapper_body_shape_diagnostic(
                        authored, executable, context, stack,
                    )
                })
                .or_else(|| unsupported_function_arithmetic_shape_diagnostic(authored, executable))
        })
        .unwrap_or_else(unsupported_function_surface_diagnostic);

    diagnostic.review(authored_surfaces, executable_surfaces)
}

fn unsupported_function_control_flow_diagnostic(
    executable: &SemanticExecutableFunctionPacket,
) -> Option<UnsupportedFunctionDiagnostic> {
    let block = syn::parse_str::<syn::Block>(&executable.body_rust).ok()?;
    block_contains_unsupported_control_flow(&block).then_some(UnsupportedFunctionDiagnostic {
        reason_code: UnsupportedFunctionReasonCode::UnsupportedControlFlow,
        summary: "function control flow falls outside the supported semantic reviewer subset",
        rewrite_hints: &[
            "Rewrite the body as a straight-line expression or a single let-then-return pipeline without branching.",
        ],
    })
}

fn unsupported_function_dep_topology_diagnostic(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<UnsupportedFunctionDiagnostic> {
    if authored.deps.len() > 3 {
        return Some(UnsupportedFunctionDiagnostic {
            reason_code: UnsupportedFunctionReasonCode::UnsupportedDepTopology,
            summary: "declared function deps fall outside the supported reviewer topology",
            rewrite_hints: &[
                "Use zero or one helper dep for arithmetic leaves, exactly two supported dep callables for wrapper pipelines, or exactly three supported function dep callables for chain3 pipelines.",
            ],
        });
    }

    if authored.deps.len() == 2 && !family_b_deps_are_supported(authored, context, stack) {
        return Some(UnsupportedFunctionDiagnostic {
            reason_code: UnsupportedFunctionReasonCode::UnsupportedDepTopology,
            summary: "declared function deps fall outside the supported reviewer topology",
            rewrite_hints: &[
                "Use zero or one helper dep for arithmetic leaves, exactly two supported dep callables for wrapper pipelines, or exactly three supported function dep callables for chain3 pipelines.",
            ],
        });
    }

    if authored.deps.len() == 3 && !family_c_deps_are_supported(authored, context, stack) {
        return Some(UnsupportedFunctionDiagnostic {
            reason_code: UnsupportedFunctionReasonCode::UnsupportedDepTopology,
            summary: "declared function deps fall outside the supported reviewer topology",
            rewrite_hints: &[
                "Use zero or one helper dep for arithmetic leaves, exactly two supported dep callables for wrapper pipelines, or exactly three supported function dep callables for chain3 pipelines.",
            ],
        });
    }

    None
}

fn unsupported_function_required_argument_expression_diagnostic(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
) -> Option<UnsupportedFunctionDiagnostic> {
    if !authored_function_looks_like_wrapper_contract(authored)
        && !authored_function_looks_like_chain3_wrapper_contract(authored)
    {
        return None;
    }

    let block = syn::parse_str::<syn::Block>(&executable.body_rust).ok()?;
    let params = executable
        .inputs
        .iter()
        .map(|input| input.name.as_str())
        .collect::<Vec<_>>();
    let tail = block_tail_expr(&block)?;

    let has_unsupported_expr = match block_prefix_stmts(&block) {
        [] if authored.deps.len() == 2 => unsupported_family_b_nested_arg_expression(
            tail,
            &params,
            callable_name(&authored.deps[0]),
            callable_name(&authored.deps[1]),
        ),
        [syn::Stmt::Local(local)] => {
            if authored.deps.len() == 2 {
                unsupported_family_b_let_then_return_arg_expression(
                    local,
                    tail,
                    &params,
                    callable_name(&authored.deps[0]),
                    callable_name(&authored.deps[1]),
                )
            } else {
                false
            }
        }
        [syn::Stmt::Local(first), syn::Stmt::Local(second)] if authored.deps.len() == 3 => {
            unsupported_family_c_let_then_return_arg_expression(
                first,
                second,
                tail,
                &params,
                callable_name(&authored.deps[0]),
                callable_name(&authored.deps[1]),
                callable_name(&authored.deps[2]),
            )
        }
        _ => false,
    };

    has_unsupported_expr.then_some(UnsupportedFunctionDiagnostic {
        reason_code: UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression,
        summary: "required wrapper arguments use expressions outside the supported reviewer subset",
        rewrite_hints: &[
            "Pass declared input params or the threaded alias directly into dep calls; avoid literals, arithmetic, and method chains in required argument positions.",
        ],
    })
}

fn unsupported_function_wrapper_body_shape_diagnostic(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<UnsupportedFunctionDiagnostic> {
    let classification = if authored_function_looks_like_wrapper_contract(authored)
        && family_b_deps_are_supported(authored, context, stack)
    {
        Some(classify_family_b_function_body(authored, executable))
    } else if authored_function_looks_like_chain3_wrapper_contract(authored)
        && family_c_deps_are_supported(authored, context, stack)
    {
        Some(classify_family_c_function_body(
            authored, executable, context, stack,
        ))
    } else {
        None
    };

    if !matches!(classification, Some(FamilyBBodyClassification::Unsupported)) {
        return None;
    }

    Some(UnsupportedFunctionDiagnostic {
        reason_code: UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
        summary: "wrapper body shape falls outside the supported semantic reviewer subset",
        rewrite_hints: &[
            "Use `dep_b(dep_a(param0, param1), param2)`, `let alias = dep_a(param0, param1); dep_b(alias, param2)`, or a straight-line let-threaded three-call chain.",
        ],
    })
}

fn unsupported_function_arithmetic_shape_diagnostic(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
) -> Option<UnsupportedFunctionDiagnostic> {
    if !authored_function_looks_like_arithmetic_contract(authored) {
        return None;
    }

    let block = syn::parse_str::<syn::Block>(&executable.body_rust).ok()?;
    block_contains_family_a_arithmetic_shape(
        &block,
        executable.inputs[0].name.as_str(),
        executable.inputs[1].name.as_str(),
    )
    .then_some(UnsupportedFunctionDiagnostic {
        reason_code: UnsupportedFunctionReasonCode::UnsupportedArithmeticShape,
        summary: "arithmetic body shape falls outside the supported semantic reviewer subset",
        rewrite_hints: &[
            "Use a supported arithmetic leaf over the declared inputs, with only an optional outer helper call and zero clamp for monotone-down behavior.",
        ],
    })
}

fn unsupported_function_surface_diagnostic() -> UnsupportedFunctionDiagnostic {
    UnsupportedFunctionDiagnostic {
        reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
        summary: "function surface is not evaluated by the semantic reviewer for this unit",
        rewrite_hints: &[
            "Express the function as a supported arithmetic leaf, a two-step wrapper pipeline, or a three-step chain3 wrapper pipeline.",
        ],
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
    let portability = supported_seam_portability_summary(spec);
    let mut reasons = Vec::new();
    let mut authored_surfaces = authored_citations(spec, &authored);
    let mut executable_surfaces = executable_citations(spec, &executable, portability.markers);

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
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
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
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
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
        let verdict = if portability.contamination.has_contaminating_domain_lowering {
            SemanticVerdict::BackendOnlySemanticsLeaked
        } else {
            SemanticVerdict::SemanticDrift
        };
        return Some(SemanticReview {
            verdict,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: drift_reasons,
            summary: "executable lowering contradicts authored semantic claims".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
    }

    if portability.contamination.has_backend_only_detail {
        let mut reason_codes = vec![SemanticReasonCode::BackendOnlyExecutionMarker];
        if portability.markers.has_proof_helper_lowering {
            reason_codes.push(SemanticReasonCode::ProofHelperOnlyMarker);
        }
        return Some(SemanticReview {
            verdict: SemanticVerdict::BackendOnlyMeaningPreserved,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes,
            summary: "backend-only execution markers are present without changing authored meaning"
                .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedSumSurface,
        });
    }

    if portability.markers.has_domain_lowering {
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
        support_status: None,
        unsupported_reason_codes: Vec::new(),
        rewrite_hints: Vec::new(),
        reason_codes: Vec::new(),
        summary: "authored semantics and executable lowering agree on the supported sum surface"
            .to_string(),
        authored_surfaces,
        executable_surfaces,
        evaluator_scope: EvaluatorScope::SupportedSumSurface,
    })
}

fn evaluate_supported_function_semantic_review(
    spec: &LoadedSpec,
    family: SupportedFunctionFamily,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<SemanticReview> {
    debug_assert!(matches!(spec.spec.unit_kind(), Ok(UnitKind::Function)));

    let authored = build_authored_function_packet(spec)?;
    let executable = build_executable_function_packet(spec)?;
    let compatibility_key = SupportedSurface::Function(family)
        .compatibility_key()
        .expect("supported function compatibility key");
    let mut reasons = Vec::new();
    let authored_surfaces = authored_function_citations(&authored);
    let executable_surfaces = executable_function_citations(&executable);

    if semantic_text_is_vague(&authored.intent) {
        reasons.push(SemanticReasonCode::VagueUnitIntent);
    }
    if authored.returns.is_none() {
        reasons.push(SemanticReasonCode::MissingMethodContract);
    }
    if !authored_function_contract_is_supported(&authored, family, context, stack) {
        reasons.push(SemanticReasonCode::OutsideHonestSupportedSubset);
    }
    reasons.sort();
    reasons.dedup();

    if !reasons.is_empty() {
        return Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: compatibility_key.to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: reasons,
            summary: "authored semantic surfaces are too weak for honest evaluation".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        });
    }

    match classify_supported_function_body(&authored, &executable, family, context, stack) {
        SupportedBodyClassification::Aligned => Some(SemanticReview {
            verdict: SemanticVerdict::Aligned,
            compatibility_key: compatibility_key.to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: Vec::new(),
            summary:
                "authored semantics and executable lowering agree on the supported function surface"
                    .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        }),
        SupportedBodyClassification::Contradictory => Some(SemanticReview {
            verdict: SemanticVerdict::SemanticDrift,
            compatibility_key: compatibility_key.to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent],
            summary: "executable lowering contradicts authored semantic claims".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        }),
        SupportedBodyClassification::OutsideHonestSubset => Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: compatibility_key.to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: vec![SemanticReasonCode::OutsideHonestSupportedSubset],
            summary: "supported semantic bodies fall outside the honest evaluator subset"
                .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        }),
    }
}

fn supported_function_surface(
    spec: &LoadedSpec,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<SupportedFunctionFamily> {
    let authored = build_authored_function_packet(spec)?;
    let executable = build_executable_function_packet(spec)?;

    // Appendix C locks function-family routing order. Keep it explicit here and test it directly.
    for route in SUPPORTED_FUNCTION_ROUTING_ORDER {
        if let Some(family) = route.try_match(&authored, &executable, context, stack) {
            return Some(family);
        }
    }

    None
}

fn authored_function_contract_is_supported(
    authored: &SemanticAuthoredFunctionPacket,
    family: SupportedFunctionFamily,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> bool {
    match family {
        SupportedFunctionFamily::FamilyC => {
            !authored.inputs.is_empty()
                && authored
                    .inputs
                    .iter()
                    .all(|input| type_is_decimal(&input.type_))
                && authored.returns.as_deref().is_some_and(type_is_decimal)
                && authored.deps.len() == 3
                && family_c_deps_are_supported(authored, context, stack)
        }
        SupportedFunctionFamily::FamilyA(role) => {
            function_inputs_are_decimal(&authored.inputs, 2)
                && authored.returns.as_deref().is_some_and(type_is_decimal)
                && family_a_authored_role(authored) == Some(role)
        }
        SupportedFunctionFamily::FamilyB => {
            !authored.inputs.is_empty()
                && authored
                    .inputs
                    .iter()
                    .all(|input| type_is_decimal(&input.type_))
                && authored.returns.as_deref().is_some_and(type_is_decimal)
                && authored.deps.len() == 2
        }
        SupportedFunctionFamily::HelperIdentityPassthrough => {
            helper_identity_passthrough_contract_is_supported(authored)
        }
    }
}

fn authored_function_looks_like_wrapper_contract(
    authored: &SemanticAuthoredFunctionPacket,
) -> bool {
    authored.deps.len() == 2
        && !authored.inputs.is_empty()
        && authored
            .inputs
            .iter()
            .all(|input| type_is_decimal(&input.type_))
        && authored.returns.as_deref().is_some_and(type_is_decimal)
}

fn authored_function_looks_like_chain3_wrapper_contract(
    authored: &SemanticAuthoredFunctionPacket,
) -> bool {
    authored.deps.len() == 3
        && !authored.inputs.is_empty()
        && authored
            .inputs
            .iter()
            .all(|input| type_is_decimal(&input.type_))
        && authored.returns.as_deref().is_some_and(type_is_decimal)
}

fn authored_function_looks_like_arithmetic_contract(
    authored: &SemanticAuthoredFunctionPacket,
) -> bool {
    authored.deps.len() <= 1
        && function_inputs_are_decimal(&authored.inputs, 2)
        && authored.returns.as_deref().is_some_and(type_is_decimal)
}

fn helper_identity_passthrough_contract_is_supported(
    authored: &SemanticAuthoredFunctionPacket,
) -> bool {
    authored.fn_name == "round"
        && authored.deps.is_empty()
        && authored.invariants.is_empty()
        && function_inputs_are_decimal(&authored.inputs, 1)
        && authored.returns.as_deref().is_some_and(type_is_decimal)
}

fn function_inputs_are_decimal(inputs: &[SemanticFieldPacket], len: usize) -> bool {
    inputs.len() == len && inputs.iter().all(|input| type_is_decimal(&input.type_))
}

fn family_a_authored_role(
    authored: &SemanticAuthoredFunctionPacket,
) -> Option<FamilyAFunctionRole> {
    if !function_inputs_are_decimal(&authored.inputs, 2)
        || !authored.returns.as_deref().is_some_and(type_is_decimal)
        || family_a_helper_dep_callable_name(authored).is_none()
    {
        return None;
    }

    let normalized = authored
        .invariants
        .iter()
        .map(|invariant| {
            normalize_family_a_invariant(
                invariant,
                authored.inputs[0].name.as_str(),
                authored.inputs[1].name.as_str(),
            )
        })
        .collect::<Option<HashSet<_>>>()?;

    if normalized.len() == 2
        && normalized.contains(FAMILY_A_INVARIANT_OUTPUT_LE_INPUT0)
        && normalized.contains(FAMILY_A_INVARIANT_OUTPUT_GE_ZERO)
    {
        Some(FamilyAFunctionRole::MonotoneDownNonnegative)
    } else if normalized.len() == 1 && normalized.contains(FAMILY_A_INVARIANT_OUTPUT_GE_INPUT0) {
        Some(FamilyAFunctionRole::MonotoneUp)
    } else if normalized.is_empty() {
        family_a_cross_library_canonical_role(authored)
    } else {
        None
    }
}

fn family_a_cross_library_canonical_role(
    authored: &SemanticAuthoredFunctionPacket,
) -> Option<FamilyAFunctionRole> {
    let [dep] = authored.deps.as_slice() else {
        return None;
    };
    let dep = DepRef::parse(dep).ok()?;
    dep.library_alias()?;

    match authored.id.as_str() {
        "pricing/apply_discount" => Some(FamilyAFunctionRole::MonotoneDownNonnegative),
        "pricing/apply_tax" => Some(FamilyAFunctionRole::MonotoneUp),
        _ => None,
    }
}

fn family_a_helper_dep_callable_name(
    authored: &SemanticAuthoredFunctionPacket,
) -> Option<Option<&str>> {
    match authored.deps.as_slice() {
        [] => Some(None),
        [dep] => Some(Some(callable_name(dep))),
        _ => None,
    }
}

fn normalize_family_a_invariant(
    invariant: &str,
    input0_name: &str,
    input1_name: &str,
) -> Option<String> {
    let trimmed = invariant.trim();
    let normalized = strip_one_outer_paren_layer(trimmed).unwrap_or(trimmed);
    let expr = syn::parse_str::<syn::Expr>(normalized).ok()?;
    let syn::Expr::Binary(binary) = expr else {
        return None;
    };

    let left = normalize_family_a_atomic_expr(&binary.left, input0_name, input1_name)?;
    let right = normalize_family_a_atomic_expr(&binary.right, input0_name, input1_name)?;
    let operator = match binary.op {
        syn::BinOp::Le(_) => "<=",
        syn::BinOp::Ge(_) => ">=",
        _ => return None,
    };
    Some(format!("{left} {operator} {right}"))
}

fn strip_one_outer_paren_layer(text: &str) -> Option<&str> {
    let stripped = text.strip_prefix('(')?.strip_suffix(')')?;
    let mut depth = 0usize;
    for (index, ch) in text.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 && index + ch.len_utf8() != text.len() {
                    return None;
                }
            }
            _ => {}
        }
    }
    Some(stripped.trim())
}

fn normalize_family_a_atomic_expr(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> Option<&'static str> {
    match expr {
        syn::Expr::Path(path) if path_is_ident(&path.path, input0_name) => Some("input0"),
        syn::Expr::Path(path) if path_is_ident(&path.path, input1_name) => Some("input1"),
        syn::Expr::Path(path) if path_is_ident(&path.path, "output") => Some("output"),
        syn::Expr::Path(path) if path_ends_with(&path.path, &["Decimal", "ZERO"]) => Some("0"),
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(lit),
            ..
        }) if lit.base10_digits() == "0" => Some("0"),
        _ => None,
    }
}

fn family_b_authored_contract_is_supported(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> bool {
    authored.deps.len() == 2
        && !authored.inputs.is_empty()
        && authored
            .inputs
            .iter()
            .all(|input| type_is_decimal(&input.type_))
        && authored.returns.as_deref().is_some_and(type_is_decimal)
        && family_b_deps_are_supported(authored, context, stack)
}

fn family_c_authored_contract_is_supported(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> bool {
    authored_function_looks_like_chain3_wrapper_contract(authored)
        && family_c_deps_are_supported(authored, context, stack)
}

fn family_b_deps_are_supported(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> bool {
    let [dep_a, dep_b] = authored.deps.as_slice() else {
        return false;
    };

    [dep_a, dep_b].into_iter().all(|dep| {
        let Some(dep_spec) = context.resolve_dep_spec(dep) else {
            return false;
        };
        let Some(dep_surface) = supported_surface_for_spec(dep_spec, context, stack) else {
            return false;
        };
        matches!(
            dep_surface,
            SupportedSurface::Function(SupportedFunctionFamily::FamilyA(_))
                | SupportedSurface::SumDiscountPolicy
                | SupportedSurface::DataCheckoutQuote
        )
    })
}

fn family_c_deps_are_supported(
    authored: &SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> bool {
    family_c_supported_deps(authored, context, stack).is_some()
}

fn family_c_supported_deps<'a>(
    authored: &'a SemanticAuthoredFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<[SupportedFunctionDep<'a>; 3]> {
    let [dep_a, dep_b, dep_c] = authored.deps.as_slice() else {
        return None;
    };

    Some([
        supported_function_dep(dep_a, context, stack)?,
        supported_function_dep(dep_b, context, stack)?,
        supported_function_dep(dep_c, context, stack)?,
    ])
}

fn supported_function_dep<'a>(
    dep: &'a str,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> Option<SupportedFunctionDep<'a>> {
    let dep_spec = context.resolve_dep_spec(dep)?;
    let SupportedSurface::Function(family) = supported_surface_for_spec(dep_spec, context, stack)?
    else {
        return None;
    };
    Some(SupportedFunctionDep {
        callable_name: callable_name(dep),
        input_arity: family.input_arity(),
    })
}

fn classify_supported_function_body(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
    family: SupportedFunctionFamily,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> SupportedBodyClassification {
    match family {
        SupportedFunctionFamily::FamilyC => {
            if !family_c_deps_are_supported(authored, context, stack) {
                return SupportedBodyClassification::OutsideHonestSubset;
            }
            match classify_family_c_function_body(authored, executable, context, stack) {
                FamilyBBodyClassification::Aligned => SupportedBodyClassification::Aligned,
                FamilyBBodyClassification::SemanticDrift => {
                    SupportedBodyClassification::Contradictory
                }
                FamilyBBodyClassification::UnderSpecified
                | FamilyBBodyClassification::Unsupported => {
                    SupportedBodyClassification::OutsideHonestSubset
                }
            }
        }
        SupportedFunctionFamily::FamilyA(authored_role) => {
            classify_family_a_body(authored_role, authored, executable)
        }
        SupportedFunctionFamily::FamilyB => {
            if !family_b_deps_are_supported(authored, context, stack) {
                return SupportedBodyClassification::OutsideHonestSubset;
            }
            match classify_family_b_function_body(authored, executable) {
                FamilyBBodyClassification::Aligned => SupportedBodyClassification::Aligned,
                FamilyBBodyClassification::SemanticDrift => {
                    SupportedBodyClassification::Contradictory
                }
                FamilyBBodyClassification::UnderSpecified
                | FamilyBBodyClassification::Unsupported => {
                    SupportedBodyClassification::OutsideHonestSubset
                }
            }
        }
        SupportedFunctionFamily::HelperIdentityPassthrough => {
            classify_helper_identity_passthrough_body(authored, executable)
        }
    }
}

fn classify_family_c_function_body(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
    context: &SemanticReviewContext<'_>,
    stack: &mut HashSet<String>,
) -> FamilyBBodyClassification {
    if authored.deps.len() != 3 || executable.inputs.is_empty() {
        return FamilyBBodyClassification::Unsupported;
    }

    let Some([dep_a, dep_b, dep_c]) = family_c_supported_deps(authored, context, stack) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Ok(block) = syn::parse_str::<syn::Block>(&executable.body_rust) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let params = executable
        .inputs
        .iter()
        .map(|input| input.name.as_str())
        .collect::<Vec<_>>();
    let Some(expected_param_count) = dep_a
        .input_arity
        .checked_add(dep_b.input_arity.saturating_sub(1))
        .and_then(|count| count.checked_add(dep_c.input_arity.saturating_sub(1)))
    else {
        return FamilyBBodyClassification::Unsupported;
    };

    let [syn::Stmt::Local(first), syn::Stmt::Local(second)] = block_prefix_stmts(&block) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(tail) = block_tail_expr(&block) else {
        return FamilyBBodyClassification::Unsupported;
    };

    classify_family_c_linear_chain(
        first,
        second,
        tail,
        &params,
        [dep_a, dep_b, dep_c],
        expected_param_count,
    )
}

fn evaluate_supported_checkout_quote_data_review(spec: &LoadedSpec) -> Option<SemanticReview> {
    debug_assert!(matches!(spec.spec.unit_kind(), Ok(UnitKind::Data)));
    debug_assert_eq!(spec.spec.id, "pricing/checkout_quote");

    let authored = build_authored_data_packet(spec)?;
    let helper_method_ids = spec
        .spec
        .extensions
        .methods
        .iter()
        .filter(|method| is_helper_or_example_method(method))
        .map(|method| method.id.clone())
        .collect::<HashSet<_>>();
    let executable = build_executable_data_packet(spec, &helper_method_ids)?;
    let portability = supported_seam_portability_summary(spec);
    let mut reasons = Vec::new();
    let authored_surfaces = authored_data_citations(&authored);
    let executable_surfaces = executable_data_citations(&executable, portability.markers);

    if semantic_text_is_vague(&authored.intent) {
        reasons.push(SemanticReasonCode::VagueUnitIntent);
    }
    if !authored_matches_checkout_quote_fields(&authored.fields)
        || !authored_matches_checkout_quote_constructors(&authored.constructors)
    {
        reasons.push(SemanticReasonCode::OutsideHonestSupportedSubset);
    }
    if authored.methods.len() != 2 || !authored_has_exact_checkout_quote_roles(&authored.methods) {
        reasons.push(SemanticReasonCode::MissingSemanticMethods);
    }
    for method in &authored.methods {
        if method.returns.is_none() {
            reasons.push(SemanticReasonCode::MissingMethodContract);
        }
        if semantic_text_is_vague(&method.intent) {
            reasons.push(SemanticReasonCode::VagueMethodIntent);
        }
        if supported_data_role_for_method(method).is_none() {
            reasons.push(SemanticReasonCode::OutsideHonestSupportedSubset);
        }
    }
    reasons.sort();
    reasons.dedup();

    if !reasons.is_empty() {
        return Some(SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: reasons,
            summary: "authored semantic surfaces are too weak for honest evaluation".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedDataSurface,
        });
    }

    let mut drift_reasons = Vec::new();
    if !checkout_quote_executable_shape_matches_authored(&authored, &executable) {
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
                    continue;
                }

                match classify_supported_data_role_body(
                    supported_data_role_for_method(authored_method)
                        .expect("non-helper data methods are pre-filtered to supported roles"),
                    &executable_method.body_rust,
                ) {
                    SupportedBodyClassification::Aligned => {}
                    SupportedBodyClassification::Contradictory => {
                        drift_reasons.push(SemanticReasonCode::MethodBodyMissingCapBehavior);
                    }
                    SupportedBodyClassification::OutsideHonestSubset => {
                        return Some(SemanticReview {
                            verdict: SemanticVerdict::UnderSpecified,
                            compatibility_key: DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY.to_string(),
                            support_status: None,
                            unsupported_reason_codes: Vec::new(),
                            rewrite_hints: Vec::new(),
                            reason_codes: vec![SemanticReasonCode::OutsideHonestSupportedSubset],
                            summary:
                                "supported semantic bodies fall outside the honest evaluator subset"
                                    .to_string(),
                            authored_surfaces,
                            executable_surfaces,
                            evaluator_scope: EvaluatorScope::SupportedDataSurface,
                        });
                    }
                }
            }
            None => drift_reasons.push(SemanticReasonCode::MethodSignatureMismatch),
        }
    }
    drift_reasons.sort();
    drift_reasons.dedup();

    if !drift_reasons.is_empty() {
        let verdict = if portability.contamination.has_contaminating_domain_lowering {
            SemanticVerdict::BackendOnlySemanticsLeaked
        } else {
            SemanticVerdict::SemanticDrift
        };
        return Some(SemanticReview {
            verdict,
            compatibility_key: DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes: drift_reasons,
            summary: "executable lowering contradicts authored semantic claims".to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedDataSurface,
        });
    }

    if portability.contamination.has_backend_only_detail {
        let mut reason_codes = vec![SemanticReasonCode::BackendOnlyExecutionMarker];
        if portability.markers.has_proof_helper_lowering {
            reason_codes.push(SemanticReasonCode::ProofHelperOnlyMarker);
        }
        return Some(SemanticReview {
            verdict: SemanticVerdict::BackendOnlyMeaningPreserved,
            compatibility_key: DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: Vec::new(),
            rewrite_hints: Vec::new(),
            reason_codes,
            summary: "backend-only execution markers are present without changing authored meaning"
                .to_string(),
            authored_surfaces,
            executable_surfaces,
            evaluator_scope: EvaluatorScope::SupportedDataSurface,
        });
    }

    Some(SemanticReview {
        verdict: SemanticVerdict::Aligned,
        compatibility_key: DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY.to_string(),
        support_status: None,
        unsupported_reason_codes: Vec::new(),
        rewrite_hints: Vec::new(),
        reason_codes: Vec::new(),
        summary: "authored semantics and executable lowering agree on the supported data surface"
            .to_string(),
        authored_surfaces,
        executable_surfaces,
        evaluator_scope: EvaluatorScope::SupportedDataSurface,
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

fn build_authored_function_packet(spec: &LoadedSpec) -> Option<SemanticAuthoredFunctionPacket> {
    let normalized = normalize_unit(spec.spec.clone()).ok()?;
    let NormalizedUnit::Function(function) = normalized else {
        return None;
    };

    Some(SemanticAuthoredFunctionPacket {
        id: function.id,
        intent: function.intent_why,
        fn_name: function.fn_name,
        inputs: function
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
        returns: function
            .contract
            .as_ref()
            .and_then(|contract| contract.returns.clone()),
        invariants: function
            .contract
            .as_ref()
            .map(|contract| contract.invariants.clone())
            .unwrap_or_default(),
        deps: function.deps,
        body_typescript: function.body_typescript.map(|body| body.trim().to_string()),
    })
}

fn build_executable_function_packet(spec: &LoadedSpec) -> Option<SemanticExecutableFunctionPacket> {
    let normalized = normalize_unit(spec.spec.clone()).ok()?;
    let NormalizedUnit::Function(function) = normalized else {
        return None;
    };

    Some(SemanticExecutableFunctionPacket {
        id: function.id,
        fn_name: function.fn_name,
        inputs: function
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
        returns: function
            .contract
            .as_ref()
            .and_then(|contract| contract.returns.clone()),
        body_rust: function.body_rust.trim().to_string(),
    })
}

fn build_authored_data_packet(spec: &LoadedSpec) -> Option<SemanticAuthoredDataPacket> {
    let data = spec.spec.extensions.data.as_ref()?;
    let mut fields = build_authored_fields(data);
    let mut constructors = spec
        .spec
        .extensions
        .constructors
        .iter()
        .map(|constructor| SemanticConstructorPacket {
            id: constructor.id.clone(),
            inputs: constructor
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
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        })
        .collect::<Vec<_>>();
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
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            returns: method
                .contract
                .as_ref()
                .and_then(|contract| contract.returns.clone()),
        })
        .collect::<Vec<_>>();
    fields.sort();
    constructors.sort_by(|left, right| left.id.cmp(&right.id));
    for constructor in &mut constructors {
        constructor.inputs.sort();
    }
    methods.sort_by(|left, right| left.id.cmp(&right.id));

    Some(SemanticAuthoredDataPacket {
        id: spec.spec.id.clone(),
        intent: spec.spec.intent.why.clone(),
        fields,
        constructors,
        methods,
    })
}

fn build_executable_data_packet(
    spec: &LoadedSpec,
    helper_method_ids: &HashSet<String>,
) -> Option<SemanticExecutableDataPacket> {
    let normalized = normalize_unit(spec.spec.clone()).ok()?;
    let NormalizedUnit::Data(unit) = normalized else {
        return None;
    };
    let lowering = lower_data_seam(&unit).ok()?;
    let mut fields = lowering
        .fields
        .iter()
        .map(|field| SemanticFieldPacket {
            name: field.name.clone(),
            type_: field.type_.clone(),
        })
        .collect::<Vec<_>>();
    let mut constructors = lowering
        .constructors
        .iter()
        .filter(|constructor| constructor.is_constructor)
        .map(|constructor| SemanticConstructorPacket {
            id: constructor.id.clone(),
            inputs: constructor
                .inputs
                .iter()
                .map(|(name, type_)| SemanticFieldPacket {
                    name: name.clone(),
                    type_: type_.clone(),
                })
                .collect::<Vec<_>>(),
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
    fields.sort();
    constructors.sort_by(|left, right| left.id.cmp(&right.id));
    for constructor in &mut constructors {
        constructor.inputs.sort();
    }
    methods.sort_by(|left, right| left.id.cmp(&right.id));

    Some(SemanticExecutableDataPacket {
        id: lowering.id,
        struct_name: lowering.struct_name,
        fields,
        constructors,
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

fn authored_function_citations(authored: &SemanticAuthoredFunctionPacket) -> Vec<SemanticCitation> {
    let mut citations = vec![SemanticCitation {
        path: "intent.why".to_string(),
        summary: authored.intent.clone(),
    }];

    if !authored.inputs.is_empty() {
        citations.push(SemanticCitation {
            path: "contract.inputs".to_string(),
            summary: format!("{} authored input(s)", authored.inputs.len()),
        });
    }
    if let Some(returns) = &authored.returns {
        citations.push(SemanticCitation {
            path: "contract.returns".to_string(),
            summary: format!("returns {returns}"),
        });
    }
    if !authored.invariants.is_empty() {
        citations.push(SemanticCitation {
            path: "contract.invariants".to_string(),
            summary: format!("{} authored invariant(s)", authored.invariants.len()),
        });
    }
    if !authored.deps.is_empty() {
        citations.push(SemanticCitation {
            path: "deps".to_string(),
            summary: format!("{} declared dep(s)", authored.deps.len()),
        });
    }
    if authored.body_typescript.is_some() {
        citations.push(SemanticCitation {
            path: "body.typescript".to_string(),
            summary: "authored TypeScript body present".to_string(),
        });
    }

    citations
}

fn authored_data_citations(authored: &SemanticAuthoredDataPacket) -> Vec<SemanticCitation> {
    let mut citations = vec![SemanticCitation {
        path: "intent.why".to_string(),
        summary: authored.intent.clone(),
    }];

    if !authored.fields.is_empty() {
        citations.push(SemanticCitation {
            path: "data.fields".to_string(),
            summary: format!("{} authored data field(s)", authored.fields.len()),
        });
    }
    if !authored.constructors.is_empty() {
        citations.push(SemanticCitation {
            path: "constructors".to_string(),
            summary: format!("{} authored constructor(s)", authored.constructors.len()),
        });
    }
    if !authored.methods.is_empty() {
        citations.push(SemanticCitation {
            path: "methods".to_string(),
            summary: format!(
                "{} semantic method(s) on {}",
                authored.methods.len(),
                authored.id
            ),
        });
    }

    citations
}

fn executable_citations(
    _spec: &LoadedSpec,
    executable: &SemanticExecutablePacket,
    markers: PortabilityMarkerSummary,
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
    if markers.has_backend_rust_derives {
        citations.push(SemanticCitation {
            path: "backends.rust.derives".to_string(),
            summary: "Rust derives contribute backend-only execution metadata".to_string(),
        });
    }
    citations
}

fn executable_function_citations(
    executable: &SemanticExecutableFunctionPacket,
) -> Vec<SemanticCitation> {
    vec![SemanticCitation {
        path: "body.rust".to_string(),
        summary: format!(
            "projects to Rust function {} with {} input(s)",
            executable.fn_name,
            executable.inputs.len()
        ),
    }]
}

fn executable_data_citations(
    executable: &SemanticExecutableDataPacket,
    markers: PortabilityMarkerSummary,
) -> Vec<SemanticCitation> {
    let mut citations = vec![SemanticCitation {
        path: "data".to_string(),
        summary: format!("projects to Rust struct {}", executable.struct_name),
    }];

    if !executable.constructors.is_empty() {
        citations.push(SemanticCitation {
            path: "constructors".to_string(),
            summary: format!(
                "{} executable constructor(s)",
                executable.constructors.len()
            ),
        });
    }
    if !executable.methods.is_empty() {
        citations.push(SemanticCitation {
            path: "methods.*.lowering.rust.body".to_string(),
            summary: format!("{} executable semantic method(s)", executable.methods.len()),
        });
    }
    if markers.has_backend_rust_derives {
        citations.push(SemanticCitation {
            path: "backends.rust.derives".to_string(),
            summary: "Rust derives contribute backend-only execution metadata".to_string(),
        });
    }

    citations
}

fn supported_seam_portability_summary(spec: &LoadedSpec) -> SupportedSeamPortabilitySummary {
    SupportedSeamPortabilitySummary {
        markers: summarize_portability_markers(spec).unwrap_or_default(),
        contamination: summarize_portability_contamination(spec).unwrap_or_default(),
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

fn build_authored_fields(data: &AuthoredDataShape) -> Vec<SemanticFieldPacket> {
    let mut fields = data
        .fields
        .iter()
        .map(|(name, field)| SemanticFieldPacket {
            name: name.clone(),
            type_: field.type_.clone(),
        })
        .collect::<Vec<_>>();
    fields.sort();
    fields
}

fn authored_matches_checkout_quote_fields(fields: &[SemanticFieldPacket]) -> bool {
    fields.len() == 3
        && fields
            .iter()
            .any(|field| field.name == "subtotal" && type_is_decimal(&field.type_))
        && fields
            .iter()
            .any(|field| field.name == "discount_rate" && type_is_decimal(&field.type_))
        && fields
            .iter()
            .any(|field| field.name == "tax_rate" && type_is_decimal(&field.type_))
}

fn authored_matches_checkout_quote_constructors(
    constructors: &[SemanticConstructorPacket],
) -> bool {
    constructors.len() == 1
        && constructors[0].id == "new"
        && authored_matches_checkout_quote_fields(&constructors[0].inputs)
}

fn checkout_quote_executable_shape_matches_authored(
    authored: &SemanticAuthoredDataPacket,
    executable: &SemanticExecutableDataPacket,
) -> bool {
    authored.fields == executable.fields && authored.constructors == executable.constructors
}

fn authored_has_exact_checkout_quote_roles(methods: &[SemanticMethodPacket]) -> bool {
    methods
        .iter()
        .all(|method| supported_data_role_for_method(method).is_some())
        && methods.iter().any(|method| {
            matches!(
                supported_data_role_for_method(method),
                Some(SupportedDataSemanticRole::DiscountedSubtotal)
            )
        })
        && methods.iter().any(|method| {
            matches!(
                supported_data_role_for_method(method),
                Some(SupportedDataSemanticRole::Total)
            )
        })
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

fn supported_data_role_for_method(
    method: &SemanticMethodPacket,
) -> Option<SupportedDataSemanticRole> {
    if method.receiver != "shared_ref"
        || !method.inputs.is_empty()
        || !type_is_decimal(method.returns.as_deref()?)
    {
        return None;
    }

    match method.id.as_str() {
        "discounted_subtotal" => Some(SupportedDataSemanticRole::DiscountedSubtotal),
        "total" => Some(SupportedDataSemanticRole::Total),
        _ => None,
    }
}

fn type_is_decimal(type_name: &str) -> bool {
    type_name
        .rsplit("::")
        .next()
        .is_some_and(|segment| segment == "Decimal")
}

fn classify_family_a_body(
    authored_role: FamilyAFunctionRole,
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
) -> SupportedBodyClassification {
    if !function_inputs_are_decimal(&executable.inputs, 2) {
        return SupportedBodyClassification::OutsideHonestSubset;
    }
    let Some(helper_name) = family_a_helper_dep_callable_name(authored) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    let Ok(block) = syn::parse_str::<syn::Block>(&executable.body_rust) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    let input0_name = executable.inputs[0].name.as_str();
    let input1_name = executable.inputs[1].name.as_str();

    match block_prefix_stmts(&block) {
        [] => {
            let Some(tail) = block_tail_expr(&block) else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            classify_family_a_terminal_shape(
                authored_role,
                tail,
                input0_name,
                input1_name,
                helper_name,
            )
        }
        [syn::Stmt::Local(local)] => {
            let Some(alias) = local_ident(local).map(|ident| ident.to_string()) else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            let Some(init) = local
                .init
                .as_ref()
                .and_then(|init| strip_expr_wrappers(&init.expr))
            else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            let Some(body_role) = classify_family_a_core_role(init, input0_name, input1_name)
            else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            let Some(tail) = block_tail_expr(&block) else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            let Some(tail_shape) = normalize_family_a_terminal_shape(tail, helper_name) else {
                return SupportedBodyClassification::OutsideHonestSubset;
            };
            if !expr_is_ident(tail_shape.core_expr, &alias) {
                return SupportedBodyClassification::OutsideHonestSubset;
            }
            classify_family_a_shape(
                authored_role,
                body_role,
                tail_shape.has_clamp,
                helper_name,
                tail_shape.helper_present,
            )
        }
        _ => SupportedBodyClassification::OutsideHonestSubset,
    }
}

fn classify_family_a_terminal_shape(
    authored_role: FamilyAFunctionRole,
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
    helper_name: Option<&str>,
) -> SupportedBodyClassification {
    let Some(shape) = normalize_family_a_terminal_shape(expr, helper_name) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    let Some(body_role) = classify_family_a_core_role(shape.core_expr, input0_name, input1_name)
    else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    classify_family_a_shape(
        authored_role,
        body_role,
        shape.has_clamp,
        helper_name,
        shape.helper_present,
    )
}

fn classify_family_a_shape(
    authored_role: FamilyAFunctionRole,
    body_role: FamilyAFunctionRole,
    has_clamp: bool,
    helper_name: Option<&str>,
    helper_present: bool,
) -> SupportedBodyClassification {
    if helper_name.is_some() && !helper_present {
        return SupportedBodyClassification::Contradictory;
    }

    match authored_role {
        FamilyAFunctionRole::MonotoneDownNonnegative => {
            if body_role == FamilyAFunctionRole::MonotoneDownNonnegative && has_clamp {
                SupportedBodyClassification::Aligned
            } else {
                SupportedBodyClassification::Contradictory
            }
        }
        FamilyAFunctionRole::MonotoneUp => {
            let clamp_is_supported = !has_clamp || (helper_name.is_some() && helper_present);
            if body_role == FamilyAFunctionRole::MonotoneUp && clamp_is_supported {
                SupportedBodyClassification::Aligned
            } else {
                SupportedBodyClassification::Contradictory
            }
        }
    }
}

fn classify_helper_identity_passthrough_body(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
) -> SupportedBodyClassification {
    let Some(intent_role) = helper_identity_passthrough_intent_role(&authored.intent) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    let Some(body_kind) = helper_identity_passthrough_body_kind(executable) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };

    match (intent_role, body_kind) {
        (
            HelperIdentityPassthroughIntentRole::Passthrough,
            HelperIdentityPassthroughBodyKind::DirectPassthrough,
        ) => SupportedBodyClassification::Aligned,
        (
            HelperIdentityPassthroughIntentRole::Passthrough,
            HelperIdentityPassthroughBodyKind::RoundLike,
        ) => SupportedBodyClassification::Contradictory,
        (
            HelperIdentityPassthroughIntentRole::RoundLike,
            HelperIdentityPassthroughBodyKind::DirectPassthrough
            | HelperIdentityPassthroughBodyKind::RoundLike,
        ) => SupportedBodyClassification::Aligned,
    }
}

fn helper_identity_passthrough_intent_role(
    intent: &str,
) -> Option<HelperIdentityPassthroughIntentRole> {
    let normalized = intent.trim().to_ascii_lowercase();
    if normalized.contains("round") {
        Some(HelperIdentityPassthroughIntentRole::RoundLike)
    } else if normalized.contains("pass through")
        || normalized.contains("passthrough")
        || normalized.contains("provided value")
        || normalized.contains("echo")
        || normalized.contains("unchanged")
    {
        Some(HelperIdentityPassthroughIntentRole::Passthrough)
    } else {
        None
    }
}

fn helper_identity_passthrough_body_kind(
    executable: &SemanticExecutableFunctionPacket,
) -> Option<HelperIdentityPassthroughBodyKind> {
    if !function_inputs_are_decimal(&executable.inputs, 1) {
        return None;
    }

    let block = syn::parse_str::<syn::Block>(&executable.body_rust).ok()?;
    if block_contains_unsupported_control_flow(&block) || !block_prefix_stmts(&block).is_empty() {
        return None;
    }
    let input_name = executable.inputs[0].name.as_str();
    let tail = block_tail_expr(&block)?;

    if expr_is_ident(tail, input_name) {
        Some(HelperIdentityPassthroughBodyKind::DirectPassthrough)
    } else if expr_is_round_like_unary_helper_body(tail, input_name) {
        Some(HelperIdentityPassthroughBodyKind::RoundLike)
    } else {
        None
    }
}

fn expr_is_round_like_unary_helper_body(expr: &syn::Expr, input_name: &str) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::MethodCall(call) = expr else {
        return false;
    };

    call.method.to_string().starts_with("round")
        && expr_is_ident(&call.receiver, input_name)
        && call.args.iter().all(expr_is_helper_literal_or_path_arg)
}

fn expr_is_helper_literal_or_path_arg(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };

    match expr {
        syn::Expr::Lit(_) | syn::Expr::Path(_) => true,
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            matches!(strip_expr_wrappers(&unary.expr), Some(syn::Expr::Lit(_)))
        }
        _ => false,
    }
}

#[derive(Clone, Copy)]
struct FamilyATerminalShape<'a> {
    core_expr: &'a syn::Expr,
    helper_present: bool,
    has_clamp: bool,
}

fn normalize_family_a_terminal_shape<'a>(
    expr: &'a syn::Expr,
    helper_name: Option<&str>,
) -> Option<FamilyATerminalShape<'a>> {
    let mut current = strip_expr_wrappers(expr)?;
    let mut helper_present = false;
    let mut has_clamp = false;

    loop {
        if !helper_present
            && let Some((inner, true)) = strip_outer_helper_call_if_present(current, helper_name)
        {
            current = inner;
            helper_present = true;
            continue;
        }

        if !has_clamp && let Some((inner, true)) = expr_as_max_zero(current) {
            current = inner;
            has_clamp = true;
            continue;
        }

        break;
    }

    Some(FamilyATerminalShape {
        core_expr: current,
        helper_present,
        has_clamp,
    })
}

fn strip_outer_helper_call_if_present<'a>(
    expr: &'a syn::Expr,
    helper_name: Option<&str>,
) -> Option<(&'a syn::Expr, bool)> {
    let expr = strip_expr_wrappers(expr)?;
    let Some(helper_name) = helper_name else {
        return Some((expr, false));
    };
    let Some(call) = expr_as_call(expr) else {
        return Some((expr, false));
    };
    if call.args.len() == 1 && expr_path_is_callable_name(&call.func, helper_name) {
        Some((&call.args[0], true))
    } else {
        Some((expr, false))
    }
}

fn classify_family_a_core_role(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> Option<FamilyAFunctionRole> {
    if expr_is_discounted_subtotal_expr_for_inputs(expr, input0_name, input1_name) {
        Some(FamilyAFunctionRole::MonotoneDownNonnegative)
    } else if expr_is_taxed_subtotal_expr_for_inputs(expr, input0_name, input1_name) {
        Some(FamilyAFunctionRole::MonotoneUp)
    } else {
        None
    }
}

fn expr_as_max_zero(expr: &syn::Expr) -> Option<(&syn::Expr, bool)> {
    let syn::Expr::MethodCall(call) = strip_expr_wrappers(expr)? else {
        return None;
    };
    if call.method == "max" && call.args.len() == 1 && expr_is_zero_expr(&call.args[0]) {
        Some((&call.receiver, true))
    } else {
        None
    }
}

fn expr_is_discounted_subtotal_expr_for_inputs(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    matches!(binary.op, syn::BinOp::Sub(_))
        && expr_is_ident(&binary.left, input0_name)
        && expr_is_subtotal_times_rate_exact_for_inputs(&binary.right, input0_name, input1_name)
}

fn expr_is_taxed_subtotal_expr_for_inputs(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    matches!(binary.op, syn::BinOp::Add(_))
        && expr_is_ident(&binary.left, input0_name)
        && expr_is_subtotal_times_rate_exact_for_inputs(&binary.right, input0_name, input1_name)
}

fn expr_is_subtotal_times_rate_exact_for_inputs(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Binary(binary) = expr else {
        return false;
    };
    matches!(binary.op, syn::BinOp::Mul(_))
        && expr_is_ident(&binary.left, input0_name)
        && expr_is_ident(&binary.right, input1_name)
}

fn expr_is_zero_expr(expr: &syn::Expr) -> bool {
    expr_is_decimal_zero(expr)
        || matches!(
            strip_expr_wrappers(expr),
            Some(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit),
                ..
            })) if lit.base10_digits() == "0"
        )
}

fn classify_family_b_function_body(
    authored: &SemanticAuthoredFunctionPacket,
    executable: &SemanticExecutableFunctionPacket,
) -> FamilyBBodyClassification {
    if authored.deps.len() != 2 || executable.inputs.is_empty() {
        return FamilyBBodyClassification::Unsupported;
    }

    let Ok(block) = syn::parse_str::<syn::Block>(&executable.body_rust) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let params = executable
        .inputs
        .iter()
        .map(|input| input.name.as_str())
        .collect::<Vec<_>>();
    let dep_a = callable_name(&authored.deps[0]);
    let dep_b = callable_name(&authored.deps[1]);

    let prefix = block_prefix_stmts(&block);
    let Some(tail) = block_tail_expr(&block) else {
        return FamilyBBodyClassification::Unsupported;
    };
    match prefix {
        [] => classify_family_b_nested_call(tail, &params, dep_a, dep_b),
        [syn::Stmt::Local(local)] => {
            classify_family_b_let_then_return(local, tail, &params, dep_a, dep_b)
        }
        _ => FamilyBBodyClassification::Unsupported,
    }
}

fn classify_family_b_nested_call(
    expr: &syn::Expr,
    params: &[&str],
    dep_a: &str,
    dep_b: &str,
) -> FamilyBBodyClassification {
    let Some(outer) = expr_as_call(expr) else {
        return FamilyBBodyClassification::Unsupported;
    };
    if expr_path_is_callable_name(&outer.func, dep_a) {
        return FamilyBBodyClassification::SemanticDrift;
    }
    if !expr_path_is_callable_name(&outer.func, dep_b) {
        return FamilyBBodyClassification::Unsupported;
    }
    if params.len() < 3 {
        return FamilyBBodyClassification::UnderSpecified;
    }
    if outer.args.len() != 2 {
        return FamilyBBodyClassification::UnderSpecified;
    }

    let Some(inner) = expr_as_call(&outer.args[0]) else {
        return classify_family_b_non_call_threaded_arg(&outer.args[0], params);
    };
    if expr_path_is_callable_name(&inner.func, dep_b) {
        return FamilyBBodyClassification::SemanticDrift;
    }
    if !expr_path_is_callable_name(&inner.func, dep_a) {
        return FamilyBBodyClassification::Unsupported;
    }
    if inner.args.len() != 2 {
        return FamilyBBodyClassification::UnderSpecified;
    }

    summarize_pipeline_arg_flow(
        &[
            classify_family_b_param_arg(&inner.args[0], params[0], params),
            classify_family_b_param_arg(&inner.args[1], params[1], params),
            classify_family_b_param_arg(&outer.args[1], params[2], params),
        ],
        params,
        3,
    )
}

fn classify_family_b_let_then_return(
    local: &syn::Local,
    tail: &syn::Expr,
    params: &[&str],
    dep_a: &str,
    dep_b: &str,
) -> FamilyBBodyClassification {
    let Some(alias) = local_ident(local).map(|ident| ident.to_string()) else {
        return FamilyBBodyClassification::Unsupported;
    };
    if params.len() < 3 {
        return FamilyBBodyClassification::UnderSpecified;
    }
    let Some(inner) = local
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(outer) = expr_as_call(tail) else {
        return FamilyBBodyClassification::Unsupported;
    };
    if expr_path_is_callable_name(&outer.func, dep_a)
        && expr_path_is_callable_name(&inner.func, dep_b)
    {
        return FamilyBBodyClassification::SemanticDrift;
    }
    if !expr_path_is_callable_name(&outer.func, dep_b) {
        return FamilyBBodyClassification::Unsupported;
    }
    if !expr_path_is_callable_name(&inner.func, dep_a) {
        return FamilyBBodyClassification::Unsupported;
    }
    if inner.args.len() != 2 || outer.args.len() != 2 {
        return FamilyBBodyClassification::UnderSpecified;
    }

    summarize_pipeline_arg_flow(
        &[
            classify_family_b_param_arg(&inner.args[0], params[0], params),
            classify_family_b_param_arg(&inner.args[1], params[1], params),
            classify_family_b_threaded_alias_arg(&outer.args[0], &alias, params),
            classify_family_b_param_arg(&outer.args[1], params[2], params),
        ],
        params,
        3,
    )
}

fn summarize_pipeline_arg_flow(
    args: &[FamilyBArgClassification],
    params: &[&str],
    expected_param_count: usize,
) -> FamilyBBodyClassification {
    if args.contains(&FamilyBArgClassification::UnsupportedExpr) {
        return FamilyBBodyClassification::Unsupported;
    }
    if args.contains(&FamilyBArgClassification::WrongParam) {
        return FamilyBBodyClassification::SemanticDrift;
    }
    if params.len() != expected_param_count {
        return FamilyBBodyClassification::UnderSpecified;
    }
    FamilyBBodyClassification::Aligned
}

fn family_b_arg_flow_contains_unsupported_expr(args: &[FamilyBArgClassification]) -> bool {
    args.contains(&FamilyBArgClassification::UnsupportedExpr)
}

fn classify_family_b_param_arg(
    expr: &syn::Expr,
    expected_param: &str,
    params: &[&str],
) -> FamilyBArgClassification {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let syn::Expr::Path(path) = expr else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let Some(ident) = path.path.get_ident() else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let ident = ident.to_string();
    if ident == expected_param {
        FamilyBArgClassification::Expected
    } else if params.contains(&ident.as_str()) {
        FamilyBArgClassification::WrongParam
    } else {
        FamilyBArgClassification::UnsupportedExpr
    }
}

fn classify_family_b_threaded_alias_arg(
    expr: &syn::Expr,
    expected_alias: &str,
    params: &[&str],
) -> FamilyBArgClassification {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let syn::Expr::Path(path) = expr else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let Some(ident) = path.path.get_ident() else {
        return FamilyBArgClassification::UnsupportedExpr;
    };
    let ident = ident.to_string();
    if ident == expected_alias {
        FamilyBArgClassification::Expected
    } else if params.contains(&ident.as_str()) {
        FamilyBArgClassification::WrongParam
    } else {
        FamilyBArgClassification::UnsupportedExpr
    }
}

fn classify_family_b_non_call_threaded_arg(
    expr: &syn::Expr,
    params: &[&str],
) -> FamilyBBodyClassification {
    match classify_family_b_param_arg(expr, "", params) {
        FamilyBArgClassification::WrongParam => FamilyBBodyClassification::SemanticDrift,
        FamilyBArgClassification::Expected | FamilyBArgClassification::UnsupportedExpr => {
            FamilyBBodyClassification::Unsupported
        }
    }
}

fn classify_family_c_linear_chain(
    first: &syn::Local,
    second: &syn::Local,
    tail: &syn::Expr,
    params: &[&str],
    deps: [SupportedFunctionDep<'_>; 3],
    expected_param_count: usize,
) -> FamilyBBodyClassification {
    let Some(first_alias) = local_ident(first).map(|ident| ident.to_string()) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(second_alias) = local_ident(second).map(|ident| ident.to_string()) else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(first_call) = first
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(second_call) = second
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return FamilyBBodyClassification::Unsupported;
    };
    let Some(third_call) = expr_as_call(tail) else {
        return FamilyBBodyClassification::Unsupported;
    };

    if params.len() != expected_param_count {
        return FamilyBBodyClassification::UnderSpecified;
    }
    if first_call.args.len() != deps[0].input_arity
        || second_call.args.len() != deps[1].input_arity
        || third_call.args.len() != deps[2].input_arity
    {
        return FamilyBBodyClassification::UnderSpecified;
    }

    let callable_flow = [
        classify_pipeline_callable(&first_call.func, deps[0].callable_name, &deps[1..]),
        classify_pipeline_callable(&second_call.func, deps[1].callable_name, &deps[2..]),
        classify_pipeline_callable(&third_call.func, deps[2].callable_name, &[]),
    ];
    if callable_flow.contains(&PipelineCallableClassification::Unsupported) {
        return FamilyBBodyClassification::Unsupported;
    }
    if callable_flow.contains(&PipelineCallableClassification::WrongDep) {
        return FamilyBBodyClassification::SemanticDrift;
    }

    let mut arg_flow = Vec::new();
    for (index, arg) in first_call.args.iter().enumerate() {
        arg_flow.push(classify_family_b_param_arg(
            arg,
            params.get(index).copied().unwrap_or(""),
            params,
        ));
    }
    for (index, arg) in second_call.args.iter().enumerate() {
        if index == 0 {
            arg_flow.push(classify_family_b_threaded_alias_arg(
                arg,
                &first_alias,
                params,
            ));
        } else {
            let param_index = deps[0].input_arity + index - 1;
            arg_flow.push(classify_family_b_param_arg(
                arg,
                params.get(param_index).copied().unwrap_or(""),
                params,
            ));
        }
    }
    for (index, arg) in third_call.args.iter().enumerate() {
        if index == 0 {
            arg_flow.push(classify_family_b_threaded_alias_arg(
                arg,
                &second_alias,
                params,
            ));
        } else {
            let param_index = deps[0].input_arity + deps[1].input_arity + index - 2;
            arg_flow.push(classify_family_b_param_arg(
                arg,
                params.get(param_index).copied().unwrap_or(""),
                params,
            ));
        }
    }

    summarize_pipeline_arg_flow(&arg_flow, params, expected_param_count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipelineCallableClassification {
    Expected,
    WrongDep,
    Unsupported,
}

fn classify_pipeline_callable(
    expr: &syn::Expr,
    expected_callable: &str,
    other_deps: &[SupportedFunctionDep<'_>],
) -> PipelineCallableClassification {
    if expr_path_is_callable_name(expr, expected_callable) {
        return PipelineCallableClassification::Expected;
    }
    if other_deps
        .iter()
        .any(|dep| expr_path_is_callable_name(expr, dep.callable_name))
    {
        return PipelineCallableClassification::WrongDep;
    }
    PipelineCallableClassification::Unsupported
}

fn unsupported_family_c_let_then_return_arg_expression(
    first: &syn::Local,
    second: &syn::Local,
    tail: &syn::Expr,
    params: &[&str],
    dep_a: &str,
    dep_b: &str,
    dep_c: &str,
) -> bool {
    let Some(first_alias) = local_ident(first).map(|ident| ident.to_string()) else {
        return false;
    };
    let Some(second_alias) = local_ident(second).map(|ident| ident.to_string()) else {
        return false;
    };
    let Some(first_call) = first
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return false;
    };
    let Some(second_call) = second
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return false;
    };
    let Some(third_call) = expr_as_call(tail) else {
        return false;
    };

    if !expr_path_is_callable_name(&first_call.func, dep_a)
        || !expr_path_is_callable_name(&second_call.func, dep_b)
        || !expr_path_is_callable_name(&third_call.func, dep_c)
    {
        return false;
    }

    let mut arg_flow = Vec::new();
    for (index, arg) in first_call.args.iter().enumerate() {
        arg_flow.push(classify_family_b_param_arg(
            arg,
            params.get(index).copied().unwrap_or(""),
            params,
        ));
    }
    for (index, arg) in second_call.args.iter().enumerate() {
        if index == 0 {
            arg_flow.push(classify_family_b_threaded_alias_arg(
                arg,
                &first_alias,
                params,
            ));
        } else {
            let param_index = first_call.args.len() + index - 1;
            arg_flow.push(classify_family_b_param_arg(
                arg,
                params.get(param_index).copied().unwrap_or(""),
                params,
            ));
        }
    }
    for (index, arg) in third_call.args.iter().enumerate() {
        if index == 0 {
            arg_flow.push(classify_family_b_threaded_alias_arg(
                arg,
                &second_alias,
                params,
            ));
        } else {
            let param_index = first_call.args.len() + second_call.args.len() + index - 2;
            arg_flow.push(classify_family_b_param_arg(
                arg,
                params.get(param_index).copied().unwrap_or(""),
                params,
            ));
        }
    }

    family_b_arg_flow_contains_unsupported_expr(&arg_flow)
}

fn unsupported_family_b_nested_arg_expression(
    expr: &syn::Expr,
    params: &[&str],
    dep_a: &str,
    dep_b: &str,
) -> bool {
    let Some(outer) = expr_as_call(expr) else {
        return false;
    };
    if !expr_path_is_callable_name(&outer.func, dep_b) || outer.args.len() != 2 {
        return false;
    }

    let Some(inner) = expr_as_call(&outer.args[0]) else {
        return false;
    };
    if !expr_path_is_callable_name(&inner.func, dep_a) || inner.args.len() != 2 {
        return false;
    }

    family_b_arg_flow_contains_unsupported_expr(&[
        classify_family_b_param_arg(
            &inner.args[0],
            params.first().copied().unwrap_or(""),
            params,
        ),
        classify_family_b_param_arg(&inner.args[1], params.get(1).copied().unwrap_or(""), params),
        classify_family_b_param_arg(&outer.args[1], params.get(2).copied().unwrap_or(""), params),
    ])
}

fn unsupported_family_b_let_then_return_arg_expression(
    local: &syn::Local,
    tail: &syn::Expr,
    params: &[&str],
    dep_a: &str,
    dep_b: &str,
) -> bool {
    let Some(alias) = local_ident(local).map(|ident| ident.to_string()) else {
        return false;
    };
    let Some(inner) = local
        .init
        .as_ref()
        .and_then(|init| expr_as_call(init.expr.as_ref()))
    else {
        return false;
    };
    let Some(outer) = expr_as_call(tail) else {
        return false;
    };
    if !expr_path_is_callable_name(&inner.func, dep_a)
        || !expr_path_is_callable_name(&outer.func, dep_b)
        || inner.args.len() != 2
        || outer.args.len() != 2
    {
        return false;
    }

    family_b_arg_flow_contains_unsupported_expr(&[
        classify_family_b_param_arg(
            &inner.args[0],
            params.first().copied().unwrap_or(""),
            params,
        ),
        classify_family_b_param_arg(&inner.args[1], params.get(1).copied().unwrap_or(""), params),
        classify_family_b_threaded_alias_arg(&outer.args[0], &alias, params),
        classify_family_b_param_arg(&outer.args[1], params.get(2).copied().unwrap_or(""), params),
    ])
}

fn expr_as_call(expr: &syn::Expr) -> Option<&syn::ExprCall> {
    let syn::Expr::Call(call) = strip_expr_wrappers(expr)? else {
        return None;
    };
    Some(call)
}

fn expr_path_is_callable_name(expr: &syn::Expr, callable_name: &str) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Path(path) = expr else {
        return false;
    };
    path_ends_with(&path.path, &[callable_name])
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

fn classify_supported_data_role_body(
    role: SupportedDataSemanticRole,
    body: &str,
) -> SupportedBodyClassification {
    let Ok(block) = syn::parse_str::<syn::Block>(body) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };

    match role {
        SupportedDataSemanticRole::DiscountedSubtotal => {
            classify_checkout_quote_discounted_subtotal_body(&block)
        }
        SupportedDataSemanticRole::Total => classify_checkout_quote_total_body(&block),
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

fn classify_checkout_quote_discounted_subtotal_body(
    block: &syn::Block,
) -> SupportedBodyClassification {
    let mut subtotal_aliases = HashSet::new();
    let mut discount_rate_aliases = HashSet::new();
    let mut aligned_result_aliases = HashSet::new();
    let mut contradictory_result_aliases = HashSet::new();

    for stmt in block_prefix_stmts(block) {
        let syn::Stmt::Local(local) = stmt else {
            return SupportedBodyClassification::OutsideHonestSubset;
        };
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

        if expr_is_checkout_quote_subtotal_ref(init, &subtotal_aliases) {
            subtotal_aliases.insert(alias.to_string());
        } else if expr_is_checkout_quote_discount_rate_ref(init, &discount_rate_aliases) {
            discount_rate_aliases.insert(alias.to_string());
        } else if expr_is_checkout_quote_apply_discount_call(
            init,
            &subtotal_aliases,
            &discount_rate_aliases,
        ) {
            aligned_result_aliases.insert(alias.to_string());
        } else if expr_is_known_checkout_quote_discounted_subtotal_contradiction(init) {
            contradictory_result_aliases.insert(alias.to_string());
        } else {
            return SupportedBodyClassification::OutsideHonestSubset;
        }
    }

    let Some(tail_expr) = block_tail_expr(block) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    if expr_is_checkout_quote_apply_discount_call(
        tail_expr,
        &subtotal_aliases,
        &discount_rate_aliases,
    ) || expr_is_alias(tail_expr, &aligned_result_aliases)
    {
        SupportedBodyClassification::Aligned
    } else if expr_is_known_checkout_quote_discounted_subtotal_contradiction(tail_expr)
        || expr_is_alias(tail_expr, &contradictory_result_aliases)
    {
        SupportedBodyClassification::Contradictory
    } else {
        SupportedBodyClassification::OutsideHonestSubset
    }
}

fn classify_checkout_quote_total_body(block: &syn::Block) -> SupportedBodyClassification {
    let mut tax_rate_aliases = HashSet::new();
    let mut discounted_aliases = HashSet::new();
    let mut aligned_result_aliases = HashSet::new();
    let mut contradictory_result_aliases = HashSet::new();

    for stmt in block_prefix_stmts(block) {
        let syn::Stmt::Local(local) = stmt else {
            return SupportedBodyClassification::OutsideHonestSubset;
        };
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

        if expr_is_checkout_quote_tax_rate_ref(init, &tax_rate_aliases) {
            tax_rate_aliases.insert(alias.to_string());
        } else if expr_is_checkout_quote_discounted_subtotal_value_ref(init, &discounted_aliases) {
            discounted_aliases.insert(alias.to_string());
        } else if expr_is_checkout_quote_apply_tax_call(
            init,
            &discounted_aliases,
            &tax_rate_aliases,
        ) {
            aligned_result_aliases.insert(alias.to_string());
        } else if expr_is_known_checkout_quote_total_contradiction(init) {
            contradictory_result_aliases.insert(alias.to_string());
        } else {
            return SupportedBodyClassification::OutsideHonestSubset;
        }
    }

    let Some(tail_expr) = block_tail_expr(block) else {
        return SupportedBodyClassification::OutsideHonestSubset;
    };
    if expr_is_checkout_quote_apply_tax_call(tail_expr, &discounted_aliases, &tax_rate_aliases)
        || expr_is_alias(tail_expr, &aligned_result_aliases)
    {
        SupportedBodyClassification::Aligned
    } else if expr_is_known_checkout_quote_total_contradiction(tail_expr)
        || expr_is_alias(tail_expr, &contradictory_result_aliases)
    {
        SupportedBodyClassification::Contradictory
    } else {
        SupportedBodyClassification::OutsideHonestSubset
    }
}

fn block_prefix_stmts(block: &syn::Block) -> &[syn::Stmt] {
    if block.stmts.is_empty() {
        &[]
    } else {
        &block.stmts[..block.stmts.len() - 1]
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

fn expr_is_alias(expr: &syn::Expr, aliases: &HashSet<String>) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Path(path) = expr else {
        return false;
    };
    path.path
        .get_ident()
        .is_some_and(|ident| aliases.contains(&ident.to_string()))
}

fn expr_is_self_field(expr: &syn::Expr, field_name: &str) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Field(field) = expr else {
        return false;
    };
    matches!(&field.member, syn::Member::Named(member) if member == field_name)
        && expr_is_ident(&field.base, "self")
}

fn expr_is_checkout_quote_subtotal_ref(expr: &syn::Expr, aliases: &HashSet<String>) -> bool {
    expr_is_self_field(expr, "subtotal") || expr_is_alias(expr, aliases)
}

fn expr_is_checkout_quote_discount_rate_ref(expr: &syn::Expr, aliases: &HashSet<String>) -> bool {
    expr_is_self_field(expr, "discount_rate") || expr_is_alias(expr, aliases)
}

fn expr_is_checkout_quote_tax_rate_ref(expr: &syn::Expr, aliases: &HashSet<String>) -> bool {
    expr_is_self_field(expr, "tax_rate") || expr_is_alias(expr, aliases)
}

fn expr_is_checkout_quote_discounted_subtotal_value_ref(
    expr: &syn::Expr,
    aliases: &HashSet<String>,
) -> bool {
    expr_is_checkout_quote_discounted_subtotal_call(expr) || expr_is_alias(expr, aliases)
}

fn expr_is_checkout_quote_apply_discount_call(
    expr: &syn::Expr,
    subtotal_aliases: &HashSet<String>,
    discount_rate_aliases: &HashSet<String>,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Call(call) = expr else {
        return false;
    };
    if !expr_is_ident(&call.func, "apply_discount") || call.args.len() != 2 {
        return false;
    }
    expr_is_checkout_quote_subtotal_ref(&call.args[0], subtotal_aliases)
        && expr_is_checkout_quote_discount_rate_ref(&call.args[1], discount_rate_aliases)
}

fn expr_is_checkout_quote_apply_tax_call(
    expr: &syn::Expr,
    discounted_aliases: &HashSet<String>,
    tax_rate_aliases: &HashSet<String>,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::Call(call) = expr else {
        return false;
    };
    if !expr_is_ident(&call.func, "apply_tax") || call.args.len() != 2 {
        return false;
    }
    expr_is_checkout_quote_discounted_subtotal_value_ref(&call.args[0], discounted_aliases)
        && expr_is_checkout_quote_tax_rate_ref(&call.args[1], tax_rate_aliases)
}

fn expr_is_checkout_quote_discounted_subtotal_call(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    let syn::Expr::MethodCall(call) = expr else {
        return false;
    };
    call.method == "discounted_subtotal"
        && call.args.is_empty()
        && expr_is_ident(&call.receiver, "self")
}

fn expr_is_known_checkout_quote_discounted_subtotal_contradiction(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    if expr_is_checkout_quote_discounted_subtotal_call(expr) {
        return true;
    }
    let syn::Expr::Call(call) = expr else {
        return false;
    };
    expr_is_ident(&call.func, "apply_tax")
        || (expr_is_ident(&call.func, "apply_discount")
            && !expr_is_checkout_quote_apply_discount_call(expr, &HashSet::new(), &HashSet::new()))
}

fn expr_is_known_checkout_quote_total_contradiction(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    if expr_is_checkout_quote_discounted_subtotal_call(expr) {
        return true;
    }
    let syn::Expr::Call(call) = expr else {
        return false;
    };
    expr_is_ident(&call.func, "apply_discount")
        || (expr_is_ident(&call.func, "apply_tax")
            && !expr_is_checkout_quote_apply_tax_call(expr, &HashSet::new(), &HashSet::new()))
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

fn block_contains_unsupported_control_flow(block: &syn::Block) -> bool {
    block
        .stmts
        .iter()
        .any(stmt_contains_unsupported_control_flow)
}

fn stmt_contains_unsupported_control_flow(stmt: &syn::Stmt) -> bool {
    match stmt {
        syn::Stmt::Local(local) => local
            .init
            .as_ref()
            .is_some_and(|init| expr_contains_unsupported_control_flow(&init.expr)),
        syn::Stmt::Expr(expr, _) => expr_contains_unsupported_control_flow(expr),
        _ => false,
    }
}

fn expr_contains_unsupported_control_flow(expr: &syn::Expr) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    match expr {
        syn::Expr::If(_)
        | syn::Expr::Match(_)
        | syn::Expr::ForLoop(_)
        | syn::Expr::While(_)
        | syn::Expr::Loop(_) => true,
        syn::Expr::Call(call) => {
            expr_contains_unsupported_control_flow(&call.func)
                || call.args.iter().any(expr_contains_unsupported_control_flow)
        }
        syn::Expr::MethodCall(call) => {
            expr_contains_unsupported_control_flow(&call.receiver)
                || call.args.iter().any(expr_contains_unsupported_control_flow)
        }
        syn::Expr::Binary(binary) => {
            expr_contains_unsupported_control_flow(&binary.left)
                || expr_contains_unsupported_control_flow(&binary.right)
        }
        syn::Expr::Unary(unary) => expr_contains_unsupported_control_flow(&unary.expr),
        syn::Expr::Reference(reference) => expr_contains_unsupported_control_flow(&reference.expr),
        syn::Expr::Block(block) => block_contains_unsupported_control_flow(&block.block),
        syn::Expr::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(expr_contains_unsupported_control_flow),
        syn::Expr::Array(array) => array
            .elems
            .iter()
            .any(expr_contains_unsupported_control_flow),
        _ => false,
    }
}

fn block_contains_family_a_arithmetic_shape(
    block: &syn::Block,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| stmt_contains_family_a_arithmetic_shape(stmt, input0_name, input1_name))
}

fn stmt_contains_family_a_arithmetic_shape(
    stmt: &syn::Stmt,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    match stmt {
        syn::Stmt::Local(local) => local.init.as_ref().is_some_and(|init| {
            expr_contains_family_a_arithmetic_shape(&init.expr, input0_name, input1_name)
        }),
        syn::Stmt::Expr(expr, _) => {
            expr_contains_family_a_arithmetic_shape(expr, input0_name, input1_name)
        }
        _ => false,
    }
}

fn expr_contains_family_a_arithmetic_shape(
    expr: &syn::Expr,
    input0_name: &str,
    input1_name: &str,
) -> bool {
    let Some(expr) = strip_expr_wrappers(expr) else {
        return false;
    };
    if classify_family_a_core_role(expr, input0_name, input1_name).is_some() {
        return true;
    }
    if expr_as_max_zero(expr).is_some_and(|(receiver, _)| {
        classify_family_a_core_role(receiver, input0_name, input1_name).is_some()
    }) {
        return true;
    }

    match expr {
        syn::Expr::Call(call) => {
            expr_contains_family_a_arithmetic_shape(&call.func, input0_name, input1_name)
                || call.args.iter().any(|arg| {
                    expr_contains_family_a_arithmetic_shape(arg, input0_name, input1_name)
                })
        }
        syn::Expr::MethodCall(call) => {
            expr_contains_family_a_arithmetic_shape(&call.receiver, input0_name, input1_name)
                || call.args.iter().any(|arg| {
                    expr_contains_family_a_arithmetic_shape(arg, input0_name, input1_name)
                })
        }
        syn::Expr::Binary(binary) => {
            expr_contains_family_a_arithmetic_shape(&binary.left, input0_name, input1_name)
                || expr_contains_family_a_arithmetic_shape(&binary.right, input0_name, input1_name)
        }
        syn::Expr::Unary(unary) => {
            expr_contains_family_a_arithmetic_shape(&unary.expr, input0_name, input1_name)
        }
        syn::Expr::Reference(reference) => {
            expr_contains_family_a_arithmetic_shape(&reference.expr, input0_name, input1_name)
        }
        syn::Expr::Block(block) => {
            block_contains_family_a_arithmetic_shape(&block.block, input0_name, input1_name)
        }
        syn::Expr::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(|elem| expr_contains_family_a_arithmetic_shape(elem, input0_name, input1_name)),
        syn::Expr::Array(array) => array
            .elems
            .iter()
            .any(|elem| expr_contains_family_a_arithmetic_shape(elem, input0_name, input1_name)),
        _ => false,
    }
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

    fn function_spec(
        id: &str,
        intent: &str,
        inputs: &[(&str, &str)],
        returns: Option<&str>,
        invariants: &[&str],
        deps: &[&str],
        body: &str,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("units/{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: intent.to_string(),
                },
                contract: Some(Contract {
                    inputs: Some(IndexMap::from_iter(
                        inputs
                            .iter()
                            .map(|(name, type_)| ((*name).to_string(), (*type_).to_string())),
                    )),
                    returns: returns.map(str::to_string),
                    invariants: invariants
                        .iter()
                        .map(|value| (*value).to_string())
                        .collect(),
                }),
                deps: deps.iter().map(|dep| (*dep).to_string()).collect(),
                imports: vec!["rust_decimal::Decimal".to_string()],
                body: Body {
                    rust: body.to_string(),
                    typescript: None,
                },
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn arithmetic_leaf_spec(
        id: &str,
        intent: &str,
        invariants: &[&str],
        deps: &[&str],
        body: &str,
    ) -> LoadedSpec {
        function_spec(
            id,
            intent,
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            invariants,
            deps,
            body,
        )
    }

    fn apply_discount_function_spec() -> LoadedSpec {
        arithmetic_leaf_spec(
            "pricing/apply_discount",
            "Return the subtotal after applying the discount rate and clamping at zero.",
            &["output <= subtotal", "output >= 0"],
            &["money/round"],
            r#"{
            round((subtotal - subtotal * rate).max(Decimal::ZERO))
        }"#,
        )
    }

    fn apply_tax_function_spec() -> LoadedSpec {
        arithmetic_leaf_spec(
            "pricing/apply_tax",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            round(subtotal + subtotal * rate)
        }"#,
        )
    }

    fn helper_identity_passthrough_spec(id: &str, intent: &str, body: &str) -> LoadedSpec {
        function_spec(
            id,
            intent,
            &[("value", "rust_decimal::Decimal")],
            Some("rust_decimal::Decimal"),
            &[],
            &[],
            body,
        )
    }

    fn calculate_total_function_spec() -> LoadedSpec {
        arithmetic_leaf_spec(
            "pricing/calculate_total",
            "Return a combined total from pricing inputs.",
            &["output >= 0"],
            &[],
            r#"{
            subtotal + subtotal * rate
        }"#,
        )
    }

    fn wrapper_pipeline_spec(id: &str, intent: &str, body: &str) -> LoadedSpec {
        function_spec(
            id,
            intent,
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &["pricing/apply_discount", "pricing/apply_tax"],
            body,
        )
    }

    fn wrapper_pipeline_chain3_spec(
        id: &str,
        intent: &str,
        deps: &[&str],
        body: &str,
    ) -> LoadedSpec {
        function_spec(
            id,
            intent,
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
                ("surcharge_rate", "rust_decimal::Decimal"),
                ("loyalty_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            deps,
            body,
        )
    }

    fn family_b_context(specs: &[LoadedSpec]) -> HashMap<String, LoadedSpec> {
        specs
            .iter()
            .cloned()
            .map(|spec| (spec.spec.id.clone(), spec))
            .collect()
    }

    fn m21_chain3_fixture_specs(
        total_wrapper_id: &str,
        tax_leaf_id: &str,
        discount_leaf_id: &str,
        checkout_id: &str,
        checkout_intent: &str,
        checkout_body: &str,
    ) -> (LoadedSpec, LoadedSpec, LoadedSpec, LoadedSpec) {
        let discount_leaf = arithmetic_leaf_spec(
            discount_leaf_id,
            "Return the running checkout subtotal after applying the loyalty discount rate and clamping at zero.",
            &["output <= subtotal", "output >= 0"],
            &[],
            r#"{
            (subtotal - subtotal * rate).max(Decimal::ZERO)
        }"#,
        );
        let tax_leaf = arithmetic_leaf_spec(
            tax_leaf_id,
            "Return the running checkout subtotal after applying the surcharge rate.",
            &["output >= subtotal"],
            &[],
            r#"{
            subtotal + subtotal * rate
        }"#,
        );
        let total_wrapper_body = format!(
            r#"{{
            let discounted = {}(subtotal, discount_rate);
            {}(discounted, tax_rate)
        }}"#,
            callable_name(discount_leaf_id),
            callable_name(tax_leaf_id),
        );
        let total_wrapper = function_spec(
            total_wrapper_id,
            "Return the checkout total after discounting the subtotal and then applying tax.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &[discount_leaf_id, tax_leaf_id],
            &total_wrapper_body,
        );
        let checkout = wrapper_pipeline_chain3_spec(
            checkout_id,
            checkout_intent,
            &[total_wrapper_id, tax_leaf_id, discount_leaf_id],
            checkout_body,
        );

        (discount_leaf, tax_leaf, total_wrapper, checkout)
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

    fn checkout_quote_data_spec() -> LoadedSpec {
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
                imports: vec![
                    "crate::pricing::apply_discount::apply_discount".to_string(),
                    "crate::pricing::apply_tax::apply_tax".to_string(),
                ],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                AuthoredField {
                                    type_: "rust_decimal::Decimal".to_string(),
                                },
                            ),
                            (
                                "discount_rate".to_string(),
                                AuthoredField {
                                    type_: "rust_decimal::Decimal".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                AuthoredField {
                                    type_: "rust_decimal::Decimal".to_string(),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![crate::types::AuthoredConstructor {
                        id: "new".to_string(),
                        intent: Intent {
                            why: "Create a quote from explicit subtotal and rates.".to_string(),
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
                                why: "Return the discounted subtotal before tax.".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("rust_decimal::Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: r#"{
            apply_discount(self.subtotal, self.discount_rate)
        }"#
                                    .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Return the final checkout total after discount and tax."
                                    .to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("rust_decimal::Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_tax".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: r#"{
            apply_tax(self.discounted_subtotal(), self.tax_rate)
        }"#
                                    .to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: None,
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
    fn semantic_review_marks_aligned_discount_amount_and_discounted_subtotal() {
        let review = evaluate_semantic_review(&discount_policy_sum_spec()).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(review.reason_codes, Vec::<SemanticReasonCode>::new());
    }

    #[test]
    fn evaluate_semantic_review_supports_checkout_quote_aligned_data_surface() {
        let review = evaluate_semantic_review(&checkout_quote_data_spec()).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY
        );
        assert_eq!(review.evaluator_scope, EvaluatorScope::SupportedDataSurface);
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
    fn evaluate_semantic_review_marks_checkout_quote_drift_as_backend_only_semantics_leaked() {
        let mut spec = checkout_quote_data_spec();
        spec.spec.extensions.backends = Some(crate::types::AuthoredBackends {
            rust: Some(AuthoredRustBackend {
                derives: vec!["Clone".to_string()],
            }),
        });
        spec.spec.extensions.methods[1]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = r#"{
            apply_tax(self.subtotal, self.tax_rate)
        }"#
        .to_string();

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::BackendOnlySemanticsLeaked);
        assert_eq!(
            review.compatibility_key,
            DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY
        );
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
    fn evaluate_semantic_review_marks_checkout_quote_under_specified_for_extra_non_helper_method() {
        let mut spec = checkout_quote_data_spec();
        spec.spec.extensions.methods.push(AuthoredMethod {
            id: "preview_discount".to_string(),
            intent: Intent {
                why: "Return a preview amount for the current checkout quote.".to_string(),
            },
            receiver: "shared_ref".to_string(),
            contract: Some(Contract {
                inputs: None,
                returns: Some("rust_decimal::Decimal".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            lowering: Some(AuthoredMethodLowering {
                rust: Some(AuthoredRustMethodLowering {
                    body: "{ self.subtotal }".to_string(),
                }),
            }),
        });

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY
        );
        assert!(
            review
                .reason_codes
                .contains(&SemanticReasonCode::OutsideHonestSupportedSubset)
        );
    }

    #[test]
    fn evaluate_semantic_review_marks_checkout_quote_under_specified_for_vague_authored_truth() {
        let mut spec = checkout_quote_data_spec();
        spec.spec.intent.why = "checkout quote".to_string();

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            DATA_CHECKOUT_QUOTE_COMPATIBILITY_KEY
        );
        assert!(
            review
                .reason_codes
                .contains(&SemanticReasonCode::VagueUnitIntent)
        );
    }

    #[test]
    fn checkout_quote_executable_field_shape_mismatch_is_detected() {
        let spec = checkout_quote_data_spec();
        let authored = build_authored_data_packet(&spec).unwrap();
        let mut executable = build_executable_data_packet(&spec, &HashSet::new()).unwrap();
        executable.fields[0].name = "subtotal_cents".to_string();

        assert!(!checkout_quote_executable_shape_matches_authored(
            &authored,
            &executable
        ));
    }

    #[test]
    fn checkout_quote_executable_constructor_shape_mismatch_is_detected() {
        let spec = checkout_quote_data_spec();
        let authored = build_authored_data_packet(&spec).unwrap();
        let mut executable = build_executable_data_packet(&spec, &HashSet::new()).unwrap();
        executable.constructors[0]
            .inputs
            .retain(|field| field.name != "tax_rate");

        assert!(!checkout_quote_executable_shape_matches_authored(
            &authored,
            &executable
        ));
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
    fn function_family_key_routing_is_descriptor_based() {
        let canonical = evaluate_semantic_review(&apply_discount_function_spec()).unwrap();
        let alternate = evaluate_semantic_review(&arithmetic_leaf_spec(
            "billing/apply_membership_discount",
            "Return the subtotal after applying the membership discount rate and clamping at zero.",
            &[" output <= subtotal ", " ( output >= Decimal::ZERO ) "],
            &["utils/math/round"],
            r#"{
            round((subtotal - subtotal * rate).max(Decimal::ZERO))
        }"#,
        ))
        .unwrap();

        assert_eq!(
            canonical.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(alternate.compatibility_key, canonical.compatibility_key);
        assert_eq!(
            alternate.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn family_a_aligned_review_uses_monotone_down_family_key() {
        let review = evaluate_semantic_review(&apply_discount_function_spec()).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(review.reason_codes, Vec::<SemanticReasonCode>::new());
    }

    #[test]
    fn family_a_drift_marks_opposite_arithmetic_leaf() {
        let mut spec = apply_discount_function_spec();
        spec.spec.body.rust = r#"{
            round(subtotal + subtotal * rate)
        }"#
        .to_string();

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn family_a_under_specified_marks_vague_authored_intent() {
        let mut spec = apply_discount_function_spec();
        spec.spec.intent.why = "todo".to_string();

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_aligned_review_requires_contextual_dep_resolution() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            let discounted = apply_discount(subtotal, discount_rate);
            apply_tax(discounted, tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let without_context = evaluate_semantic_review(&wrapper).unwrap();
        let with_context = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();

        assert_eq!(
            without_context.evaluator_scope,
            EvaluatorScope::UnsupportedSurface
        );
        assert_eq!(
            without_context.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
        assert_eq!(with_context.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            with_context.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_drift_marks_reversed_pipeline_order() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_discount(apply_tax(subtotal, tax_rate), discount_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_under_specified_marks_vague_authored_intent() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "todo",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_drift_marks_swapped_inner_args() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(discount_rate, subtotal), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_drift_marks_swapped_outer_rate_arg() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate), discount_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_drift_marks_wrong_threaded_alias_return() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            let discounted = apply_discount(subtotal, discount_rate);
            apply_tax(subtotal, tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_drift_marks_duplicated_param_flow() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, subtotal), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_under_specified_marks_dropped_required_arg() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate))
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::OutsideHonestSupportedSubset]
        );
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_under_specified_marks_unused_extra_param() {
        let wrapper = function_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
                ("unused_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &["pricing/apply_discount", "pricing/apply_tax"],
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::OutsideHonestSupportedSubset]
        );
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_literal_required_arg_stays_unsupported() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, Decimal::ZERO), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_arithmetic_required_arg_stays_unsupported() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate + tax_rate), tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_unsupported_near_miss_stays_unsupported() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate), tax_rate.max(Decimal::ZERO))
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_extra_let_marks_unsupported_wrapper_body_shape() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            let discounted = apply_discount(subtotal, discount_rate);
            let total = apply_tax(discounted, tax_rate);
            total
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape]
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_non_stacking_rejection_stays_unsupported() {
        let inner = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            apply_tax(apply_discount(subtotal, discount_rate), tax_rate)
        }"#,
        );
        let outer = function_spec(
            "pricing/calculate_grand_total",
            "Return the total after reusing the existing wrapper and then applying tax again.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
                ("tax_rate_2", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &["pricing/calculate_total", "pricing/apply_tax"],
            r#"{
            let total = calculate_total(subtotal, discount_rate, tax_rate);
            apply_tax(total, tax_rate_2)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            inner,
            outer.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&outer, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
    }

    #[test]
    fn m21_chain3_classifier_aligned_fixture_routes_to_chain3() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_aligned",
            "pricing/pricing_tax_leaf_aligned",
            "pricing/pricing_discount_leaf_aligned",
            "pricing/checkout_chain3_aligned",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            r#"{
            let base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
            let surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
            pricing_discount_leaf_aligned(surcharged_total, loyalty_rate)
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
        assert_eq!(
            review.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn m21_chain3_classifier_drift_fixture_reports_semantic_drift() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_drift",
            "pricing/pricing_tax_leaf_drift",
            "pricing/pricing_discount_leaf_drift",
            "pricing/checkout_chain3_drift",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            r#"{
            let base_total = pricing_total_wrapper_drift(subtotal, discount_rate, tax_rate);
            let surcharged_total = pricing_tax_leaf_drift(subtotal, surcharge_rate);
            pricing_discount_leaf_drift(surcharged_total, loyalty_rate)
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn m21_chain3_classifier_under_specified_fixture_reports_vague_truth() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_under_specified",
            "pricing/pricing_tax_leaf_under_specified",
            "pricing/pricing_discount_leaf_under_specified",
            "pricing/checkout_chain3_under_specified",
            "checkout chain3",
            r#"{
            let base_total = pricing_total_wrapper_under_specified(subtotal, discount_rate, tax_rate);
            let surcharged_total = pricing_tax_leaf_under_specified(base_total, surcharge_rate);
            pricing_discount_leaf_under_specified(surcharged_total, loyalty_rate)
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn m21_chain3_classifier_unsupported_near_miss_stays_unsupported() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_unsupported_near_miss",
            "pricing/pricing_tax_leaf_unsupported_near_miss",
            "pricing/pricing_discount_leaf_unsupported_near_miss",
            "pricing/checkout_chain3_unsupported_near_miss",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            r#"{
            pricing_discount_leaf_unsupported_near_miss(
                pricing_tax_leaf_unsupported_near_miss(
                    pricing_total_wrapper_unsupported_near_miss(subtotal, discount_rate, tax_rate),
                    surcharge_rate,
                ),
                loyalty_rate,
            )
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape]
        );
    }

    #[test]
    fn m21_chain3_classifier_runtime_order_is_explicit() {
        let routed_keys =
            SUPPORTED_FUNCTION_ROUTING_ORDER.map(SupportedFunctionRoute::compatibility_key);

        assert_eq!(
            routed_keys,
            [
                FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY,
                FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY,
                FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY,
            ]
        );
    }

    #[test]
    fn m21_chain3_regression_family_a_variants_are_not_shadowed() {
        let discount_review = evaluate_semantic_review(&apply_discount_function_spec()).unwrap();
        let tax_review = evaluate_semantic_review(&apply_tax_function_spec()).unwrap();

        assert_eq!(
            discount_review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(discount_review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            tax_review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(tax_review.verdict, SemanticVerdict::Aligned);
    }

    #[test]
    fn wrapper_pipeline_classifier_aligned_fixture_routes_to_promoted_family() {
        let wrapper = wrapper_pipeline_spec(
            "pricing/calculate_total",
            "Return the total after discounting the subtotal and then applying tax.",
            r#"{
            let discounted = apply_discount(subtotal, discount_rate);
            apply_tax(discounted, tax_rate)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            wrapper.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&wrapper, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn wrapper_pipeline_runtime_route_order_preserves_chain3_wrapper_monotone_down_monotone_up() {
        let routed_keys =
            SUPPORTED_FUNCTION_ROUTING_ORDER.map(SupportedFunctionRoute::compatibility_key);

        assert_eq!(
            routed_keys,
            [
                FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY,
                FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY,
                FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY,
            ]
        );
    }

    #[test]
    fn monotone_down_nonnegative_classifier_aligned_fixture_routes_to_promoted_leaf() {
        let review = evaluate_semantic_review(&apply_discount_function_spec()).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
        assert_eq!(
            review.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn monotone_down_nonnegative_classifier_cross_library_canonical_example_routes_to_promoted_leaf_without_invariants()
     {
        let crosslib = arithmetic_leaf_spec(
            "pricing/apply_discount",
            "Apply a discount while importing the shared round helper from a sibling spec library.",
            &[],
            &["shared::money/round"],
            r#"{
            let discounted = subtotal - subtotal * rate;
            round(discounted.max(Decimal::ZERO))
        }"#,
        );

        let review = evaluate_semantic_review(&crosslib).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
    }

    #[test]
    fn monotone_down_nonnegative_classifier_drift_fixture_reports_semantic_drift() {
        let drift = arithmetic_leaf_spec(
            "pricing/apply_discount_drift",
            "Apply a discount to a subtotal while keeping the result nonnegative.",
            &["output <= subtotal", "output >= 0"],
            &["money/round"],
            r#"{
            round(subtotal + subtotal * rate)
        }"#,
        );

        let review = evaluate_semantic_review(&drift).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn monotone_down_nonnegative_classifier_under_specified_fixture_reports_vague_truth() {
        let under_specified = arithmetic_leaf_spec(
            "pricing/apply_discount_under_specified",
            "todo",
            &["output <= subtotal", "output >= 0"],
            &["money/round"],
            r#"{
            let discounted = subtotal - subtotal * rate;
            round(discounted.max(Decimal::ZERO))
        }"#,
        );

        let review = evaluate_semantic_review(&under_specified).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn monotone_down_nonnegative_classifier_unsupported_near_miss_stays_unsupported() {
        let near_miss = arithmetic_leaf_spec(
            "pricing/apply_discount_control_flow_unsupported_near_miss",
            "Apply a discount to a subtotal while keeping the result nonnegative.",
            &["output <= subtotal", "output >= 0"],
            &["money/round"],
            r#"{
            let discounted = subtotal - subtotal * rate;
            if discounted < Decimal::ZERO {
                Decimal::ZERO
            } else {
                round(discounted)
            }
        }"#,
        );

        let review = evaluate_semantic_review(&near_miss).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
    }

    #[test]
    fn monotone_up_classifier_aligned_fixture_routes_to_promoted_leaf() {
        let review = evaluate_semantic_review(&apply_tax_function_spec()).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
        assert_eq!(
            review.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn monotone_up_classifier_reads_authored_typescript_without_spec_version_sentinel() {
        let mut spec = apply_tax_function_spec();
        spec.spec.spec_version = None;
        spec.spec.body.typescript =
            Some("return round(subtotal.add(subtotal.mul(rate)));".to_string());

        let authored = build_authored_function_packet(&spec).unwrap();
        assert_eq!(
            authored.body_typescript.as_deref(),
            Some("return round(subtotal.add(subtotal.mul(rate)));")
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert!(
            review
                .authored_surfaces
                .iter()
                .any(|citation| citation.path == "body.typescript"),
            "{review:?}"
        );
    }

    #[test]
    fn wrapper_pipeline_classifier_reads_authored_typescript_without_spec_version_sentinel() {
        let mut spec = wrapper_pipeline_spec(
            "pricing/calculate_total_typescript_authored",
            "Return the checkout total after discounting the subtotal and then applying tax.",
            r#"{
            let discounted = pricing/apply_discount(subtotal, discount_rate);
            pricing/apply_tax(discounted, tax_rate)
        }"#,
        );
        spec.spec.spec_version = None;
        spec.spec.body.typescript = Some(
            "{\n            const discounted = apply_discount(subtotal, discount_rate);\n            return apply_tax(discounted, tax_rate);\n        }"
                .to_string(),
        );

        let authored = build_authored_function_packet(&spec).unwrap();
        assert_eq!(
            authored.body_typescript.as_deref(),
            Some(
                "{\n            const discounted = apply_discount(subtotal, discount_rate);\n            return apply_tax(discounted, tax_rate);\n        }"
            )
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert!(
            review
                .authored_surfaces
                .iter()
                .any(|citation| citation.path == "body.typescript"),
            "{review:?}"
        );
    }

    #[test]
    fn monotone_up_classifier_helper_then_clamp_routes_to_promoted_leaf() {
        let helper_then_clamp = arithmetic_leaf_spec(
            "pricing/apply_tax_helper_then_clamp",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            let taxed = subtotal + subtotal * rate;
            round(taxed).max(Decimal::ZERO)
        }"#,
        );

        let review = evaluate_semantic_review(&helper_then_clamp).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
    }

    #[test]
    fn monotone_up_classifier_cross_library_canonical_example_routes_to_promoted_leaf_without_invariants()
     {
        let crosslib = arithmetic_leaf_spec(
            "pricing/apply_tax",
            "Apply tax while importing the shared round helper from a sibling spec library.",
            &[],
            &["shared::money/round"],
            r#"{
            let taxed = subtotal + subtotal * rate;
            round(taxed).max(Decimal::ZERO)
        }"#,
        );

        let review = evaluate_semantic_review(&crosslib).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
    }

    #[test]
    fn monotone_up_classifier_drift_fixture_reports_semantic_drift() {
        let drift = arithmetic_leaf_spec(
            "pricing/apply_tax_drift",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            round(subtotal - subtotal * rate)
        }"#,
        );

        let review = evaluate_semantic_review(&drift).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn monotone_up_classifier_under_specified_fixture_reports_vague_truth() {
        let under_specified = arithmetic_leaf_spec(
            "pricing/apply_tax_under_specified",
            "todo",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            round(subtotal + subtotal * rate)
        }"#,
        );

        let review = evaluate_semantic_review(&under_specified).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn monotone_up_classifier_unsupported_near_miss_stays_unsupported() {
        let near_miss = arithmetic_leaf_spec(
            "pricing/apply_tax_control_flow_unsupported_near_miss",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            let taxed = subtotal + subtotal * rate;
            if rate == Decimal::ZERO {
                subtotal
            } else {
                round(taxed)
            }
        }"#,
        );

        let review = evaluate_semantic_review(&near_miss).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
    }

    #[test]
    fn monotone_up_classifier_cross_library_control_flow_near_miss_stays_unsupported() {
        let near_miss = arithmetic_leaf_spec(
            "pricing/apply_tax_crosslib_control_flow_unsupported_near_miss",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["shared::money/round"],
            r#"{
            let taxed = subtotal + subtotal * rate;
            if rate == Decimal::ZERO {
                subtotal
            } else {
                round(taxed).max(Decimal::ZERO)
            }
        }"#,
        );

        let review = evaluate_semantic_review(&near_miss).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
    }

    #[test]
    fn monotone_down_nonnegative_regression_chain3_is_not_shadowed() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_aligned",
            "pricing/pricing_tax_leaf_aligned",
            "pricing/pricing_discount_leaf_aligned",
            "pricing/checkout_chain3_aligned",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            r#"{
            let base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
            let surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
            pricing_discount_leaf_aligned(surcharged_total, loyalty_rate)
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn monotone_down_nonnegative_regression_monotone_up_is_not_shadowed() {
        let review = evaluate_semantic_review(&apply_tax_function_spec()).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn monotone_up_regression_chain3_is_not_shadowed() {
        let (discount_leaf, tax_leaf, total_wrapper, checkout) = m21_chain3_fixture_specs(
            "pricing/pricing_total_wrapper_aligned",
            "pricing/pricing_tax_leaf_aligned",
            "pricing/pricing_discount_leaf_aligned",
            "pricing/checkout_chain3_aligned",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            r#"{
            let base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
            let surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
            pricing_discount_leaf_aligned(surcharged_total, loyalty_rate)
        }"#,
        );
        let specs = family_b_context(&[discount_leaf, tax_leaf, total_wrapper, checkout.clone()]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&checkout, &context).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn monotone_up_regression_monotone_down_nonnegative_is_not_shadowed() {
        let review = evaluate_semantic_review(&apply_discount_function_spec()).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn monotone_up_regression_runtime_order_matches_locked_precedence() {
        let routed_keys =
            SUPPORTED_FUNCTION_ROUTING_ORDER.map(SupportedFunctionRoute::compatibility_key);

        assert_eq!(
            routed_keys,
            [
                FUNCTION_WRAPPER_PIPELINE_CHAIN3_COMPATIBILITY_KEY,
                FUNCTION_WRAPPER_PIPELINE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY,
                FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY,
                FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY,
            ]
        );
    }

    #[test]
    fn helper_identity_passthrough_classifier_direct_passthrough_aligned_fixture_routes_to_supported_helper()
     {
        let review = evaluate_semantic_review(&helper_identity_passthrough_spec(
            "money/round",
            "Echo the provided value unchanged for downstream pricing flows.",
            r#"{
            value
        }"#,
        ))
        .unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
        assert_eq!(
            review.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn helper_identity_passthrough_classifier_round_like_aligned_fixture_routes_to_supported_helper()
     {
        let review = evaluate_semantic_review(&helper_identity_passthrough_spec(
            "money/round",
            "Round a decimal value to two fractional digits for pricing flows.",
            r#"{
            value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
        }"#,
        ))
        .unwrap();

        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Supported)
        );
        assert_eq!(
            review.evaluator_scope,
            EvaluatorScope::SupportedFunctionSurface
        );
    }

    #[test]
    fn helper_identity_passthrough_classifier_under_specified_fixture_reports_vague_truth() {
        let review = evaluate_semantic_review(&helper_identity_passthrough_spec(
            "money/round",
            "todo",
            r#"{
            value
        }"#,
        ))
        .unwrap();

        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::VagueUnitIntent]
        );
    }

    #[test]
    fn helper_identity_passthrough_classifier_drift_fixture_reports_semantic_drift() {
        let review = evaluate_semantic_review(&helper_identity_passthrough_spec(
            "money/round",
            "Echo the provided value unchanged for downstream pricing flows.",
            r#"{
            value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
        }"#,
        ))
        .unwrap();

        assert_eq!(review.verdict, SemanticVerdict::SemanticDrift);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent]
        );
    }

    #[test]
    fn helper_identity_passthrough_classifier_unsupported_near_miss_stays_unsupported() {
        let review = evaluate_semantic_review(&helper_identity_passthrough_spec(
            "money/round",
            "Round a decimal value to two fractional digits for pricing flows.",
            r#"{
            if value == Decimal::ZERO {
                value
            } else {
                value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
            }
        }"#,
        ))
        .unwrap();

        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
    }

    #[test]
    fn unsupported_function_priority_prefers_control_flow_over_dep_topology() {
        let spec = arithmetic_leaf_spec(
            "billing/apply_discount",
            "Return the subtotal after applying the discount rate and clamping at zero.",
            &["output <= subtotal", "output >= 0"],
            &["money/round", "money/abs"],
            r#"{
            if rate > Decimal::ZERO {
                round((subtotal - subtotal * rate).max(Decimal::ZERO))
            } else {
                subtotal
            }
        }"#,
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
    }

    #[test]
    fn unsupported_function_priority_prefers_dep_topology_over_required_argument_expression() {
        let outer = function_spec(
            "pricing/calculate_grand_total",
            "Return the total after reusing the existing wrapper and then applying tax again.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
                ("tax_rate_2", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &["pricing/calculate_total", "pricing/apply_tax"],
            r#"{
            apply_tax(calculate_total(subtotal, discount_rate, Decimal::ZERO), tax_rate_2)
        }"#,
        );
        let specs = family_b_context(&[
            apply_discount_function_spec(),
            apply_tax_function_spec(),
            calculate_total_function_spec(),
            outer.clone(),
        ]);
        let context = SemanticReviewContext::new(&specs);

        let review = evaluate_semantic_review_with_context(&outer, &context).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
    }

    #[test]
    fn family_a_helper_dep_exclusion_rejects_second_dep() {
        let spec = arithmetic_leaf_spec(
            "billing/apply_discount",
            "Return the subtotal after applying the discount rate and clamping at zero.",
            &["output <= subtotal", "output >= 0"],
            &["money/round", "money/abs"],
            r#"{
            round((subtotal - subtotal * rate).max(Decimal::ZERO))
        }"#,
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
    }

    #[test]
    fn family_a_helper_dep_normalization_allows_helper_then_clamp_for_monotone_up() {
        let spec = arithmetic_leaf_spec(
            "billing/apply_discount",
            "Return the subtotal after applying the tax rate and rounding the total.",
            &["output >= subtotal"],
            &["money/round"],
            r#"{
            let taxed = subtotal + subtotal * rate;
            round(taxed).max(Decimal::ZERO)
        }"#,
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.verdict, SemanticVerdict::Aligned);
        assert_eq!(
            review.compatibility_key,
            FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY
        );
    }

    #[test]
    fn family_a_normalization_rejects_non_exact_invariant_forms() {
        let spec = arithmetic_leaf_spec(
            "billing/apply_discount",
            "Return the subtotal after applying the discount rate and clamping at zero.",
            &["0 <= output", "((output <= subtotal))"],
            &["money/round"],
            r#"{
            round((subtotal - subtotal * rate).max(Decimal::ZERO))
        }"#,
        );

        let review = evaluate_semantic_review(&spec).unwrap();
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
    }

    #[test]
    fn semantic_health_effect_only_demotes_supported_verdicts() {
        let supported_review = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: SUM_DISCOUNT_POLICY_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
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

        let supported_function_review = SemanticReview {
            verdict: SemanticVerdict::SemanticDrift,
            compatibility_key: FUNCTION_ARITHMETIC_LEAF_MONOTONE_DOWN_NONNEGATIVE_COMPATIBILITY_KEY
                .to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        };
        assert_eq!(
            semantic_health_effect(Some(&supported_function_review)),
            SemanticHealthEffect::DemoteFailing
        );

        let unsupported_review =
            evaluate_semantic_review(&calculate_total_function_spec()).unwrap();
        assert_eq!(
            semantic_health_effect(Some(&unsupported_review)),
            SemanticHealthEffect::KeepBase
        );
    }

    #[test]
    fn semantic_review_emits_unsupported_function_diagnostics_for_calculate_total() {
        let review = evaluate_semantic_review(&calculate_total_function_spec()).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            review.support_status,
            Some(SemanticSupportStatus::Unsupported)
        );
        assert_eq!(
            review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
        assert_eq!(
            review.rewrite_hints,
            vec![
                "Use a supported arithmetic leaf over the declared inputs, with only an optional outer helper call and zero clamp for monotone-down behavior."
                    .to_string()
            ]
        );
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::UnsupportedSurface]
        );
        assert!(
            review
                .summary
                .contains("arithmetic body shape falls outside"),
            "{}",
            review.summary
        );
        assert!(!review.authored_surfaces.is_empty());
        assert!(!review.executable_surfaces.is_empty());
    }

    #[test]
    fn semantic_review_keeps_generic_unsupported_surface_for_data() {
        let spec = discount_policy_data_spec();
        let review = evaluate_semantic_review(&spec).unwrap();

        assert_eq!(review.verdict, SemanticVerdict::UnderSpecified);
        assert_eq!(review.evaluator_scope, EvaluatorScope::UnsupportedSurface);
        assert_eq!(
            review.compatibility_key,
            unsupported_surface_compatibility_key(spec.spec.unit_kind().unwrap())
        );
        assert_eq!(review.support_status, None);
        assert!(review.unsupported_reason_codes.is_empty());
        assert!(review.rewrite_hints.is_empty());
        assert_eq!(
            review.reason_codes,
            vec![SemanticReasonCode::UnsupportedSurface]
        );
        assert!(
            review
                .summary
                .contains("is not evaluated by the semantic reviewer for this unit"),
            "{}",
            review.summary
        );
        assert!(review.authored_surfaces.is_empty());
        assert!(review.executable_surfaces.is_empty());
    }

    #[test]
    fn project_semantic_review_preserve_keeps_matching_supported_function_key() {
        let spec = apply_discount_function_spec();
        let review = evaluate_semantic_review(&spec).unwrap();

        let preserved =
            project_semantic_review(&spec, Some(&review), SemanticProjectionMode::Preserve)
                .unwrap();

        assert_eq!(preserved, review);
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
    fn project_semantic_review_preserve_drops_mismatched_supported_function_key() {
        let spec = apply_discount_function_spec();
        let mut supported_review = evaluate_semantic_review(&spec).unwrap();
        supported_review.compatibility_key =
            "function.arithmetic_leaf.monotone_down_nonnegative.v0".to_string();

        let preserved = project_semantic_review(
            &spec,
            Some(&supported_review),
            SemanticProjectionMode::Preserve,
        );

        assert!(preserved.is_none());
    }

    #[test]
    fn project_semantic_review_preserve_drops_legacy_unsupported_review_for_supported_function() {
        let spec = apply_discount_function_spec();
        let unsupported_review =
            evaluate_semantic_review(&calculate_total_function_spec()).unwrap();

        let preserved = project_semantic_review(
            &spec,
            Some(&unsupported_review),
            SemanticProjectionMode::Preserve,
        );

        assert!(preserved.is_none());
    }

    #[test]
    fn project_semantic_review_preserve_drops_unsupported_surface_review_even_with_compatibility_key()
     {
        let unsupported_review =
            evaluate_semantic_review(&calculate_total_function_spec()).unwrap();

        let preserved = project_semantic_review(
            &discount_policy_sum_spec(),
            Some(&unsupported_review),
            SemanticProjectionMode::Preserve,
        );

        assert!(preserved.is_none());
    }

    #[test]
    fn project_semantic_review_only_refresh_synthesizes_fresh_unsupported_metadata() {
        let spec = calculate_total_function_spec();
        let existing = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: "unsupported.function.v0".to_string(),
            support_status: Some(SemanticSupportStatus::Unsupported),
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
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

        assert!(preserved.is_none());
        assert_eq!(
            refreshed.compatibility_key,
            UNSUPPORTED_FUNCTION_COMPATIBILITY_KEY
        );
        assert_eq!(
            refreshed.support_status,
            Some(SemanticSupportStatus::Unsupported)
        );
        assert_eq!(
            refreshed.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
        assert_eq!(
            refreshed.rewrite_hints,
            vec![
                "Use a supported arithmetic leaf over the declared inputs, with only an optional outer helper call and zero clamp for monotone-down behavior."
                    .to_string()
            ]
        );
        assert!(
            refreshed
                .summary
                .contains("arithmetic body shape falls outside"),
            "{}",
            refreshed.summary
        );
        assert!(!refreshed.authored_surfaces.is_empty());
        assert!(!refreshed.executable_surfaces.is_empty());
    }

    #[test]
    fn unsupported_function_shape_fingerprint_same_reason_same_shape_stays_equal_across_unit_names()
    {
        let canonical = calculate_total_function_spec();
        let renamed = arithmetic_leaf_spec(
            "pricing/calculate_total_again",
            "Return a second combined total from pricing inputs.",
            &["output >= 0"],
            &[],
            r#"{
            subtotal + subtotal * rate
        }"#,
        );
        let canonical_review = evaluate_semantic_review(&canonical).unwrap();
        let renamed_review = evaluate_semantic_review(&renamed).unwrap();
        let canonical_key = unsupported_function_shape_fingerprint(&canonical);
        let renamed_key = unsupported_function_shape_fingerprint(&renamed);

        assert_eq!(
            canonical_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
        assert_eq!(
            renamed_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
        assert_eq!(
            canonical_key,
            Some(
                r#"{"schema_version":1,"function_dep_arity":0,"callable_dep_topology_class":"no_deps_or_helper","contract_input_count":2,"has_return":true,"authored_body_kind":"arithmetic_like"}"#
                    .to_string()
            )
        );
        assert_eq!(canonical_key, renamed_key);
    }

    #[test]
    fn unsupported_function_shape_fingerprint_same_reason_different_shape_stays_different() {
        let unsupported_pair = function_spec(
            "pricing/calculate_total_pair",
            "Return the total after passing through an unsupported dep pair.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &["pricing/apply_discount", "pricing/apply_fee"],
            r#"{
            apply_fee(apply_discount(subtotal, discount_rate), tax_rate)
        }"#,
        );
        let fanout = function_spec(
            "pricing/calculate_total_fanout",
            "Return the total after passing through too many helpers.",
            &[
                ("subtotal", "rust_decimal::Decimal"),
                ("discount_rate", "rust_decimal::Decimal"),
                ("tax_rate", "rust_decimal::Decimal"),
                ("surcharge_rate", "rust_decimal::Decimal"),
                ("loyalty_rate", "rust_decimal::Decimal"),
            ],
            Some("rust_decimal::Decimal"),
            &[],
            &[
                "pricing/apply_discount",
                "pricing/apply_tax",
                "pricing/apply_fee",
                "pricing/apply_loyalty_credit",
            ],
            r#"{
            subtotal
        }"#,
        );
        let specs_by_id = HashMap::from([
            (
                "pricing/apply_discount".to_string(),
                apply_discount_function_spec(),
            ),
            (unsupported_pair.spec.id.clone(), unsupported_pair.clone()),
            (fanout.spec.id.clone(), fanout.clone()),
        ]);
        let context = SemanticReviewContext::new(&specs_by_id);

        let pair_review =
            evaluate_semantic_review_with_context(&unsupported_pair, &context).unwrap();
        let fanout_review = evaluate_semantic_review_with_context(&fanout, &context).unwrap();

        assert_eq!(
            pair_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
        assert_eq!(
            fanout_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedDepTopology]
        );
        assert_ne!(
            unsupported_function_shape_fingerprint_with_context(&unsupported_pair, &context),
            unsupported_function_shape_fingerprint_with_context(&fanout, &context)
        );
    }

    #[test]
    fn unsupported_function_shape_fingerprint_ignores_reason_and_review_prose_policy() {
        let arithmetic_shape = calculate_total_function_spec();
        let control_flow_shape = arithmetic_leaf_spec(
            "billing/calculate_total_control_flow_same_shape",
            "Return a combined total from pricing inputs.",
            &["output >= 0"],
            &[],
            r#"{
            if rate > Decimal::ZERO {
                subtotal + subtotal * rate
            } else {
                subtotal
            }
        }"#,
        );
        let arithmetic_review = evaluate_semantic_review(&arithmetic_shape).unwrap();
        let control_flow_review = evaluate_semantic_review(&control_flow_shape).unwrap();

        assert_eq!(
            arithmetic_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedArithmeticShape]
        );
        assert_eq!(
            control_flow_review.unsupported_reason_codes,
            vec![UnsupportedFunctionReasonCode::UnsupportedControlFlow]
        );
        assert_eq!(
            unsupported_function_shape_fingerprint(&arithmetic_shape),
            unsupported_function_shape_fingerprint(&control_flow_shape)
        );
    }

    #[test]
    fn semantic_review_legacy_json_without_support_status_deserializes_to_none() {
        let review: SemanticReview = serde_json::from_str(
            r#"{
                "verdict": "aligned",
                "compatibility_key": "function.arithmetic_leaf.monotone_up.v1",
                "reason_codes": [],
                "summary": "",
                "authored_surfaces": [],
                "executable_surfaces": [],
                "evaluator_scope": "supported_function_surface"
            }"#,
        )
        .unwrap();

        assert_eq!(review.support_status, None);
        assert!(review.unsupported_reason_codes.is_empty());
        assert!(review.rewrite_hints.is_empty());
    }

    #[test]
    fn unsupported_function_reason_code_priority_matches_public_order() {
        let mut codes = vec![
            UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            UnsupportedFunctionReasonCode::UnsupportedArithmeticShape,
            UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
            UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression,
            UnsupportedFunctionReasonCode::UnsupportedDepTopology,
            UnsupportedFunctionReasonCode::UnsupportedControlFlow,
        ];

        codes.sort();

        assert_eq!(
            codes,
            vec![
                UnsupportedFunctionReasonCode::UnsupportedControlFlow,
                UnsupportedFunctionReasonCode::UnsupportedDepTopology,
                UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression,
                UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
                UnsupportedFunctionReasonCode::UnsupportedArithmeticShape,
                UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            ]
        );
    }

    #[test]
    fn semantic_review_explicit_support_status_wins_over_legacy_inference() {
        let review = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: "unsupported.function.v1".to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::UnsupportedSurface,
        };

        assert_eq!(
            review.effective_support_status(),
            SemanticSupportStatus::Supported
        );
    }

    #[test]
    fn semantic_health_effect_keeps_base_for_explicitly_unsupported_review_even_when_legacy_scope_looks_supported()
     {
        let review = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY.to_string(),
            support_status: Some(SemanticSupportStatus::Unsupported),
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        };

        assert_eq!(
            semantic_health_effect(Some(&review)),
            SemanticHealthEffect::KeepBase
        );
    }

    #[test]
    fn semantic_health_effect_demotes_for_explicitly_supported_review_even_when_legacy_scope_looks_unsupported()
     {
        let review = SemanticReview {
            verdict: SemanticVerdict::SemanticDrift,
            compatibility_key: "unsupported.function.v1".to_string(),
            support_status: Some(SemanticSupportStatus::Supported),
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::UnsupportedSurface,
        };

        assert_eq!(
            semantic_health_effect(Some(&review)),
            SemanticHealthEffect::DemoteFailing
        );
    }

    #[test]
    fn semantic_review_legacy_unsupported_scope_and_key_infer_unsupported() {
        let unsupported_scope = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: FUNCTION_ARITHMETIC_LEAF_MONOTONE_UP_COMPATIBILITY_KEY.to_string(),
            support_status: None,
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::UnsupportedSurface,
        };
        let unsupported_key = SemanticReview {
            verdict: SemanticVerdict::UnderSpecified,
            compatibility_key: "unsupported.function.v1".to_string(),
            support_status: None,
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
        };

        assert_eq!(
            unsupported_scope.effective_support_status(),
            SemanticSupportStatus::Unsupported
        );
        assert_eq!(
            unsupported_key.effective_support_status(),
            SemanticSupportStatus::Unsupported
        );
    }

    #[test]
    fn semantic_review_legacy_supported_scopes_infer_supported() {
        for evaluator_scope in [
            EvaluatorScope::SupportedFunctionSurface,
            EvaluatorScope::SupportedSumSurface,
            EvaluatorScope::SupportedDataSurface,
        ] {
            let review = SemanticReview {
                verdict: SemanticVerdict::Aligned,
                compatibility_key: "data.checkout_quote.v1".to_string(),
                support_status: None,
                unsupported_reason_codes: vec![],
                rewrite_hints: vec![],
                reason_codes: vec![],
                summary: String::new(),
                authored_surfaces: vec![],
                executable_surfaces: vec![],
                evaluator_scope,
            };

            assert_eq!(
                review.effective_support_status(),
                SemanticSupportStatus::Supported
            );
        }
    }
}
