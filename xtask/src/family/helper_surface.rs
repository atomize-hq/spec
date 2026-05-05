use crate::family::promotion_artifacts::{
    CorpusProgramBasisSnapshot, CorpusProgramDecisionAction, CorpusProgramDecisionArtifact,
    CorpusProgramDecisionBasisCode, DecisionReason, HoldReason, NextStepDetail, NextStepStatus,
    PivotTargetClass, PromotionReadiness, RecommendationCandidateEntry, RequiredNextAction,
};
use spec_core::semantic_review::UnsupportedFunctionReasonCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HelperSurfaceSignal<'a> {
    pub(crate) primary_reason_code: UnsupportedFunctionReasonCode,
    pub(crate) overlap_family: &'a str,
    pub(crate) real_example_hits: usize,
    pub(crate) shape_fingerprint: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HelperSurfaceDisposition {
    DurableNonPromotableHelperSurface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HelperSurfaceCandidateTuple {
    pub(crate) promotion_readiness: PromotionReadiness,
    pub(crate) hold_reason: HoldReason,
    pub(crate) next_step_status: NextStepStatus,
    pub(crate) next_step_detail: NextStepDetail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HelperSurfaceFollowOnDecisionTuple {
    pub(crate) decision_action: CorpusProgramDecisionAction,
    pub(crate) decision_basis_code: CorpusProgramDecisionBasisCode,
    pub(crate) pivot_target_class: PivotTargetClass,
    pub(crate) required_next_action: RequiredNextAction,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UnsupportedShapeFingerprint {
    schema_version: u64,
    function_dep_arity: usize,
    callable_dep_topology_class: String,
    contract_input_count: usize,
    has_return: bool,
    authored_body_kind: String,
}

pub(crate) const HELPER_SURFACE_FINGERPRINT: &str = "{\"schema_version\":1,\"function_dep_arity\":0,\"callable_dep_topology_class\":\"no_deps_or_helper\",\"contract_input_count\":1,\"has_return\":true,\"authored_body_kind\":\"neither\"}";

pub(crate) fn classify_helper_surface(
    signal: &HelperSurfaceSignal<'_>,
) -> Option<HelperSurfaceDisposition> {
    if signal.primary_reason_code != UnsupportedFunctionReasonCode::UnsupportedFunctionSurface {
        return None;
    }
    if signal.overlap_family != "unknown" || signal.real_example_hits == 0 {
        return None;
    }
    if !matches_helper_surface_fingerprint(signal.shape_fingerprint) {
        return None;
    }
    Some(HelperSurfaceDisposition::DurableNonPromotableHelperSurface)
}

pub(crate) fn durable_non_promotable_helper_surface_candidate_tuple() -> HelperSurfaceCandidateTuple
{
    HelperSurfaceCandidateTuple {
        promotion_readiness: PromotionReadiness::Hold,
        hold_reason: HoldReason::HelperSurfaceNotPromotable,
        next_step_status: NextStepStatus::DurableHold,
        next_step_detail: NextStepDetail::HelperSurfaceNotPromotable,
    }
}

pub(crate) fn recommendation_uses_helper_surface_durable_hold_tuple(
    candidate: &RecommendationCandidateEntry,
) -> bool {
    candidate
        .hold_reasons
        .contains(&HoldReason::HelperSurfaceNotPromotable)
        || candidate.next_step_status == NextStepStatus::DurableHold
        || candidate.next_step_detail == NextStepDetail::HelperSurfaceNotPromotable
}

pub(crate) fn recommendation_matches_helper_surface_durable_hold_tuple(
    candidate: &RecommendationCandidateEntry,
) -> bool {
    let durable_hold = durable_non_promotable_helper_surface_candidate_tuple();
    candidate.promotion_readiness == durable_hold.promotion_readiness
        && candidate.hold_reasons == [durable_hold.hold_reason]
        && candidate.next_step_status == durable_hold.next_step_status
        && candidate.next_step_detail == durable_hold.next_step_detail
}

pub(crate) fn helper_surface_follow_on_decision_tuple() -> HelperSurfaceFollowOnDecisionTuple {
    HelperSurfaceFollowOnDecisionTuple {
        decision_action: CorpusProgramDecisionAction::PivotToArchitectureSharedCoreFollowOn,
        decision_basis_code: CorpusProgramDecisionBasisCode::DurableNonPromotableHelperSurface,
        pivot_target_class: PivotTargetClass::ArchitectureSharedCoreFollowOn,
        required_next_action: RequiredNextAction::AuthorArchitectureFollowOnPlan,
    }
}

pub(crate) fn basis_snapshot_requires_helper_surface_follow_on(
    snapshot: &CorpusProgramBasisSnapshot,
) -> bool {
    snapshot.decision_status == crate::family::promotion_artifacts::DecisionStatus::NotRecommended
        && snapshot.open_blockers == vec![DecisionReason::HelperSurfaceNotPromotable]
        && snapshot.missing_evidence.is_empty()
        && snapshot.stale_evidence.is_empty()
}

pub(crate) fn decision_uses_helper_surface_follow_on_tuple(
    artifact: &CorpusProgramDecisionArtifact,
) -> bool {
    let follow_on = helper_surface_follow_on_decision_tuple();
    artifact.decision_action == follow_on.decision_action
        || artifact.decision_basis_code == follow_on.decision_basis_code
        || artifact.pivot_target_class == Some(follow_on.pivot_target_class)
        || artifact.required_next_action == follow_on.required_next_action
}

pub(crate) fn decision_matches_helper_surface_follow_on_tuple(
    artifact: &CorpusProgramDecisionArtifact,
) -> bool {
    let follow_on = helper_surface_follow_on_decision_tuple();
    artifact.decision_action == follow_on.decision_action
        && artifact.decision_basis_code == follow_on.decision_basis_code
        && artifact.pivot_target_class == Some(follow_on.pivot_target_class)
        && artifact.required_next_action == follow_on.required_next_action
}

fn matches_helper_surface_fingerprint(shape_fingerprint: &str) -> bool {
    if shape_fingerprint == HELPER_SURFACE_FINGERPRINT {
        return true;
    }
    let Ok(fingerprint) = serde_json::from_str::<UnsupportedShapeFingerprint>(shape_fingerprint)
    else {
        return false;
    };
    fingerprint.schema_version == 1
        && fingerprint.function_dep_arity == 0
        && fingerprint.callable_dep_topology_class == "no_deps_or_helper"
        && fingerprint.contract_input_count == 1
        && fingerprint.has_return
        && fingerprint.authored_body_kind == "neither"
}

#[cfg(test)]
mod tests {
    use super::{
        HELPER_SURFACE_FINGERPRINT, HelperSurfaceDisposition, HelperSurfaceSignal,
        basis_snapshot_requires_helper_surface_follow_on, classify_helper_surface,
        decision_matches_helper_surface_follow_on_tuple,
        durable_non_promotable_helper_surface_candidate_tuple,
        recommendation_matches_helper_surface_durable_hold_tuple,
    };
    use crate::family::promotion_artifacts::{
        CorpusProgramBasisSnapshot, CorpusProgramDecisionAction, CorpusProgramDecisionArtifact,
        CorpusProgramDecisionBasisCode, DecisionReason, DecisionStatus, EvidenceState,
        PivotTargetClass, PromotionArtifactKind, RecommendationCandidateEntry,
        RecommendationConfidence, RecommendationDifficulty, RecommendationLeverage,
        RecommendationStatus, RequiredNextAction,
    };
    use spec_core::semantic_review::UnsupportedFunctionReasonCode;

    #[test]
    fn helper_surface_classifies_durable_non_promotable_helper_surface() {
        let signal = HelperSurfaceSignal {
            primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            overlap_family: "unknown",
            real_example_hits: 2,
            shape_fingerprint: HELPER_SURFACE_FINGERPRINT,
        };

        assert_eq!(
            classify_helper_surface(&signal),
            Some(HelperSurfaceDisposition::DurableNonPromotableHelperSurface)
        );
    }

    #[test]
    fn helper_surface_rejects_non_matching_signal() {
        let signal = HelperSurfaceSignal {
            primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            overlap_family: "function.wrapper.pipeline*",
            real_example_hits: 2,
            shape_fingerprint: HELPER_SURFACE_FINGERPRINT,
        };

        assert_eq!(classify_helper_surface(&signal), None);
    }

    #[test]
    fn helper_surface_candidate_tuple_matches_exact_frozen_contract() {
        let tuple = durable_non_promotable_helper_surface_candidate_tuple();
        let candidate = RecommendationCandidateEntry {
            candidate_id: "fixture".to_string(),
            cluster_ids: vec!["cluster".to_string()],
            primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            overlap_family: "unknown".to_string(),
            promotion_readiness: tuple.promotion_readiness,
            hold_reasons: vec![tuple.hold_reason],
            next_step_status: tuple.next_step_status,
            next_step_detail: tuple.next_step_detail,
            leverage: RecommendationLeverage {
                real_example_hits: 2,
                promotion_relevant_regression_hits: 1,
                boundary_only_hits: 0,
                total_units_in_cluster: 3,
            },
            difficulty: RecommendationDifficulty {
                tier: crate::family::promotion_artifacts::DifficultyTier::Hard,
                why: "fixture".to_string(),
            },
            confidence: RecommendationConfidence {
                level: crate::family::promotion_artifacts::ConfidenceLevel::Low,
                why: "fixture".to_string(),
            },
            rationale: "fixture".to_string(),
        };

        assert!(recommendation_matches_helper_surface_durable_hold_tuple(
            &candidate
        ));
    }

    #[test]
    fn helper_surface_follow_on_requires_exact_basis_snapshot() {
        let snapshot = CorpusProgramBasisSnapshot {
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            decision_status: DecisionStatus::NotRecommended,
            top_candidate_id: Some("fixture".to_string()),
            open_blockers: vec![DecisionReason::HelperSurfaceNotPromotable],
            missing_evidence: Vec::new(),
            stale_evidence: Vec::new(),
        };

        assert!(basis_snapshot_requires_helper_surface_follow_on(&snapshot));

        let contradictory_snapshot = CorpusProgramBasisSnapshot {
            stale_evidence: vec![EvidenceState::StaleEvidence],
            ..snapshot
        };
        assert!(!basis_snapshot_requires_helper_surface_follow_on(
            &contradictory_snapshot
        ));
    }

    #[test]
    fn helper_surface_follow_on_decision_tuple_matches_exact_frozen_contract() {
        let artifact = CorpusProgramDecisionArtifact {
            schema_version: 1,
            artifact_kind: PromotionArtifactKind::CorpusProgramDecision,
            generated_at: "2026-05-05T02:00:00Z".to_string(),
            analysis_basis_path: "analysis.json".to_string(),
            analysis_basis_sha256: "sha".to_string(),
            basis_snapshot: CorpusProgramBasisSnapshot {
                recommendation_status: RecommendationStatus::NoStrongCandidate,
                decision_status: DecisionStatus::NotRecommended,
                top_candidate_id: Some("fixture".to_string()),
                open_blockers: vec![DecisionReason::HelperSurfaceNotPromotable],
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
            },
            decision_action: CorpusProgramDecisionAction::PivotToArchitectureSharedCoreFollowOn,
            decision_basis_code: CorpusProgramDecisionBasisCode::DurableNonPromotableHelperSurface,
            pivot_target_class: Some(PivotTargetClass::ArchitectureSharedCoreFollowOn),
            required_next_action: RequiredNextAction::AuthorArchitectureFollowOnPlan,
            summary: "fixture".to_string(),
        };

        assert!(decision_matches_helper_surface_follow_on_tuple(&artifact));

        let contradictory_artifact = CorpusProgramDecisionArtifact {
            required_next_action: RequiredNextAction::AuthorCorpusExpansionPlan,
            ..artifact
        };
        assert!(!decision_matches_helper_surface_follow_on_tuple(
            &contradictory_artifact
        ));
    }
}
