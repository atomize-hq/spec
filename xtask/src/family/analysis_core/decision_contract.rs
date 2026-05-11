use crate::XtaskError;
use crate::family::analysis_core::helper_surface::{
    helper_surface_follow_on_decision_tuple,
    recommendation_matches_helper_surface_durable_hold_tuple,
};
use crate::family::promotion_artifacts::{
    CorpusProgramBasisSnapshot, CorpusProgramDecisionAction, CorpusProgramDecisionBasisCode,
    DecisionReason, DecisionStatus, EvidenceState, FamilyRecommendationAnalysisArtifact,
    PivotTargetClass, RecommendationCandidateEntry, RecommendationStatus, RequiredNextAction,
};
use spec_core::semantic_review::UnsupportedFunctionReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DerivedCorpusProgramDecision {
    pub decision_action: CorpusProgramDecisionAction,
    pub decision_basis_code: CorpusProgramDecisionBasisCode,
    pub pivot_target_class: Option<PivotTargetClass>,
    pub required_next_action: RequiredNextAction,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecisionContractStopStateTuple {
    pub recommendation_status: RecommendationStatus,
    pub decision_status: DecisionStatus,
    pub open_blockers: Vec<DecisionReason>,
    pub missing_evidence: Vec<EvidenceState>,
    pub stale_evidence: Vec<EvidenceState>,
    pub decision_action: CorpusProgramDecisionAction,
    pub decision_basis_code: CorpusProgramDecisionBasisCode,
    pub required_next_action: RequiredNextAction,
}

pub(crate) fn decision_contract_stop_state_tuple() -> DecisionContractStopStateTuple {
    DecisionContractStopStateTuple {
        recommendation_status: RecommendationStatus::InsufficientRealCorpus,
        decision_status: DecisionStatus::NotRecommended,
        open_blockers: Vec::new(),
        missing_evidence: Vec::new(),
        stale_evidence: Vec::new(),
        decision_action: CorpusProgramDecisionAction::Stop,
        decision_basis_code: CorpusProgramDecisionBasisCode::NoActionableCandidate,
        required_next_action: RequiredNextAction::RecordStopWithoutNewMilestone,
    }
}

pub(crate) fn corpus_program_basis_snapshot(
    artifact: &FamilyRecommendationAnalysisArtifact,
) -> CorpusProgramBasisSnapshot {
    CorpusProgramBasisSnapshot {
        recommendation_status: artifact.recommendation_status,
        decision_status: artifact.decision_summary.decision_status,
        top_candidate_id: artifact.decision_summary.top_candidate_id.clone(),
        open_blockers: artifact.decision_summary.open_blockers.clone(),
        missing_evidence: artifact.evidence_summary.missing_evidence.clone(),
        stale_evidence: artifact.evidence_summary.stale_evidence.clone(),
    }
}

pub(crate) fn basis_snapshot_requires_helper_surface_follow_on(
    snapshot: &CorpusProgramBasisSnapshot,
) -> bool {
    snapshot.decision_status == DecisionStatus::NotRecommended
        && snapshot.open_blockers == vec![DecisionReason::HelperSurfaceNotPromotable]
        && snapshot.missing_evidence.is_empty()
        && snapshot.stale_evidence.is_empty()
}

pub(crate) fn basis_activates_helper_surface_follow_on(
    basis: &FamilyRecommendationAnalysisArtifact,
) -> bool {
    let Some(candidate) = basis.ranked_candidates.first() else {
        return false;
    };
    basis_candidate_is_helper_surface_follow_on(candidate)
        && basis_snapshot_requires_helper_surface_follow_on(&corpus_program_basis_snapshot(basis))
}

pub(crate) fn derive_corpus_program_decision_contract(
    basis: &FamilyRecommendationAnalysisArtifact,
) -> Result<DerivedCorpusProgramDecision, XtaskError> {
    let snapshot = corpus_program_basis_snapshot(basis);

    if snapshot.decision_status == DecisionStatus::Recommended {
        return Ok(DerivedCorpusProgramDecision {
            decision_action: CorpusProgramDecisionAction::PivotToFamilyPromotionRun,
            decision_basis_code: CorpusProgramDecisionBasisCode::PromotionReadyCandidate,
            pivot_target_class: Some(PivotTargetClass::FamilyPromotionRun),
            required_next_action: RequiredNextAction::AuthorFamilyPromotionPlan,
            summary: format!(
                "Recommendation basis is `recommended`, so corpus run 1 stays unspent and the repo should pivot to a bounded family-promotion run for `{}`.",
                snapshot
                    .top_candidate_id
                    .as_deref()
                    .unwrap_or("the current top candidate")
            ),
        });
    }

    if snapshot.decision_status == DecisionStatus::BlockedForNow
        && (!snapshot.missing_evidence.is_empty() || !snapshot.stale_evidence.is_empty())
    {
        return Ok(DerivedCorpusProgramDecision {
            decision_action: CorpusProgramDecisionAction::SpendCorpusRun1,
            decision_basis_code: CorpusProgramDecisionBasisCode::PlausibleCandidateMissingEvidence,
            pivot_target_class: None,
            required_next_action: RequiredNextAction::AuthorCorpusExpansionPlan,
            summary: format!(
                "Recommendation basis is blocked by missing or stale evidence for `{}`, so corpus run 1 should be spent on bounded corpus expansion rather than a promotion run.",
                snapshot
                    .top_candidate_id
                    .as_deref()
                    .unwrap_or("the current top candidate")
            ),
        });
    }

    if basis_activates_helper_surface_follow_on(basis) {
        let follow_on = helper_surface_follow_on_decision_tuple();
        return Ok(DerivedCorpusProgramDecision {
            decision_action: follow_on.decision_action,
            decision_basis_code: follow_on.decision_basis_code,
            pivot_target_class: Some(follow_on.pivot_target_class),
            required_next_action: follow_on.required_next_action,
            summary: format!(
                "Recommendation basis holds `{}` as a durable non-promotable helper surface, so corpus run 1 stays unspent and the repo should pivot to an architecture shared-core follow-on plan.",
                snapshot
                    .top_candidate_id
                    .as_deref()
                    .unwrap_or("the current top candidate")
            ),
        });
    }

    if snapshot.decision_status == DecisionStatus::BlockedForNow {
        return Ok(DerivedCorpusProgramDecision {
            decision_action: CorpusProgramDecisionAction::PivotToRecommendationPolicyRun,
            decision_basis_code: CorpusProgramDecisionBasisCode::PolicyInterpretationBlocker,
            pivot_target_class: Some(PivotTargetClass::RecommendationPolicyRun),
            required_next_action: RequiredNextAction::AuthorRecommendationPolicyPlan,
            summary: "Recommendation basis is blocked without a bounded evidence-spend path, so the repo should pivot to a recommendation-policy follow-on before spending corpus run 1.".to_string(),
        });
    }

    Ok(DerivedCorpusProgramDecision {
        decision_action: CorpusProgramDecisionAction::Stop,
        decision_basis_code: CorpusProgramDecisionBasisCode::NoActionableCandidate,
        pivot_target_class: None,
        required_next_action: RequiredNextAction::RecordStopWithoutNewMilestone,
        summary: "Recommendation basis exposes no actionable next family move, so corpus run 1 remains unspent and no new milestone is authorized from this basis.".to_string(),
    })
}

fn basis_candidate_is_helper_surface_follow_on(candidate: &RecommendationCandidateEntry) -> bool {
    candidate.primary_reason_code == UnsupportedFunctionReasonCode::UnsupportedFunctionSurface
        && candidate.overlap_family == "unknown"
        && candidate.leverage.real_example_hits > 0
        && recommendation_matches_helper_surface_durable_hold_tuple(candidate)
}

#[cfg(test)]
mod tests {
    use super::{
        basis_activates_helper_surface_follow_on, basis_snapshot_requires_helper_surface_follow_on,
        corpus_program_basis_snapshot, decision_contract_stop_state_tuple,
        derive_corpus_program_decision_contract,
    };
    use crate::family::analysis_core::durable_non_promotable_helper_surface_candidate_tuple;
    use crate::family::promotion_artifacts::{
        ConfidenceLevel, CorpusProgramBasisSnapshot, DecisionReason, DecisionStatus,
        DifficultyTier, EvidenceState, FamilyRecommendationAnalysisArtifact, PromotionArtifactKind,
        RECOMMENDATION_ANALYSIS_SCHEMA_VERSION, RecommendationCandidateEntry,
        RecommendationConfidence, RecommendationDelta, RecommendationDifficulty,
        RecommendationLeverage, RecommendationStatus,
    };
    use spec_core::semantic_review::UnsupportedFunctionReasonCode;

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
    fn helper_surface_follow_on_activation_uses_validated_basis_truth() {
        let basis = helper_surface_follow_on_basis_fixture();

        let snapshot = corpus_program_basis_snapshot(&basis);
        assert!(basis_snapshot_requires_helper_surface_follow_on(&snapshot));
        assert!(basis_activates_helper_surface_follow_on(&basis));
    }

    #[test]
    fn stop_state_tuple_matches_locked_truth() {
        let tuple = decision_contract_stop_state_tuple();

        assert_eq!(
            tuple.recommendation_status,
            RecommendationStatus::InsufficientRealCorpus
        );
        assert_eq!(tuple.decision_status, DecisionStatus::NotRecommended);
        assert!(tuple.open_blockers.is_empty());
        assert!(tuple.missing_evidence.is_empty());
        assert!(tuple.stale_evidence.is_empty());
        assert_eq!(
            tuple.decision_action,
            crate::family::promotion_artifacts::CorpusProgramDecisionAction::Stop
        );
        assert_eq!(
            tuple.decision_basis_code,
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::NoActionableCandidate
        );
        assert_eq!(
            tuple.required_next_action,
            crate::family::promotion_artifacts::RequiredNextAction::RecordStopWithoutNewMilestone
        );
    }

    #[test]
    fn stop_state_tuple_matches_kernel_stop_decision() {
        let basis = stop_basis_fixture();

        let tuple = decision_contract_stop_state_tuple();
        let derived = derive_corpus_program_decision_contract(&basis).unwrap();
        let snapshot = corpus_program_basis_snapshot(&basis);

        assert_eq!(snapshot.recommendation_status, tuple.recommendation_status);
        assert_eq!(snapshot.decision_status, tuple.decision_status);
        assert_eq!(snapshot.open_blockers, tuple.open_blockers);
        assert_eq!(snapshot.missing_evidence, tuple.missing_evidence);
        assert_eq!(snapshot.stale_evidence, tuple.stale_evidence);
        assert_eq!(derived.decision_action, tuple.decision_action);
        assert_eq!(derived.decision_basis_code, tuple.decision_basis_code);
        assert_eq!(derived.required_next_action, tuple.required_next_action);
    }

    #[test]
    fn decision_contract_exposes_promotion_ready_branch() {
        let basis = FamilyRecommendationAnalysisArtifact {
            recommendation_status: RecommendationStatus::Ranked,
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::Recommended,
                top_candidate_id: Some("fixture".to_string()),
                open_blockers: Vec::new(),
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            ranked_candidates: vec![helper_surface_candidate_fixture()],
            ..analysis_basis_fixture()
        };

        let derived = derive_corpus_program_decision_contract(&basis).unwrap();
        assert_eq!(
            derived.decision_action,
            crate::family::promotion_artifacts::CorpusProgramDecisionAction::PivotToFamilyPromotionRun
        );
        assert_eq!(
            derived.decision_basis_code,
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::PromotionReadyCandidate
        );
        assert_eq!(
            derived.required_next_action,
            crate::family::promotion_artifacts::RequiredNextAction::AuthorFamilyPromotionPlan
        );
    }

    #[test]
    fn decision_contract_exposes_blocked_on_evidence_branch() {
        let basis = FamilyRecommendationAnalysisArtifact {
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::BlockedForNow,
                top_candidate_id: Some("fixture".to_string()),
                open_blockers: vec![DecisionReason::ThinRealExampleSupport],
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            evidence_summary: crate::family::promotion_artifacts::EvidenceSummary {
                missing_evidence: vec![EvidenceState::ThinRealExampleSupport],
                stale_evidence: Vec::new(),
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            ..analysis_basis_fixture()
        };

        let derived = derive_corpus_program_decision_contract(&basis).unwrap();
        assert_eq!(
            derived.decision_action,
            crate::family::promotion_artifacts::CorpusProgramDecisionAction::SpendCorpusRun1
        );
        assert_eq!(
            derived.decision_basis_code,
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::PlausibleCandidateMissingEvidence
        );
        assert_eq!(
            derived.required_next_action,
            crate::family::promotion_artifacts::RequiredNextAction::AuthorCorpusExpansionPlan
        );
    }

    #[test]
    fn decision_contract_exposes_helper_surface_follow_on_branch() {
        let basis = helper_surface_follow_on_basis_fixture();

        let derived = derive_corpus_program_decision_contract(&basis).unwrap();
        assert_eq!(
            derived.decision_action,
            crate::family::promotion_artifacts::CorpusProgramDecisionAction::PivotToArchitectureSharedCoreFollowOn
        );
        assert_eq!(
            derived.decision_basis_code,
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::DurableNonPromotableHelperSurface
        );
        assert_eq!(
            derived.required_next_action,
            crate::family::promotion_artifacts::RequiredNextAction::AuthorArchitectureFollowOnPlan
        );
    }

    #[test]
    fn decision_contract_exposes_policy_interpretation_blocker_branch() {
        let basis = FamilyRecommendationAnalysisArtifact {
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::BlockedForNow,
                top_candidate_id: Some("fixture".to_string()),
                open_blockers: vec![DecisionReason::RegressionWarning],
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            evidence_summary: crate::family::promotion_artifacts::EvidenceSummary {
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            ..analysis_basis_fixture()
        };

        let derived = derive_corpus_program_decision_contract(&basis).unwrap();
        assert_eq!(
            derived.decision_action,
            crate::family::promotion_artifacts::CorpusProgramDecisionAction::PivotToRecommendationPolicyRun
        );
        assert_eq!(
            derived.decision_basis_code,
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::PolicyInterpretationBlocker
        );
        assert_eq!(
            derived.required_next_action,
            crate::family::promotion_artifacts::RequiredNextAction::AuthorRecommendationPolicyPlan
        );
    }

    fn analysis_basis_fixture() -> FamilyRecommendationAnalysisArtifact {
        FamilyRecommendationAnalysisArtifact {
            schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
            artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
            generated_at: "2026-05-05T00:00:00Z".to_string(),
            coverage_path: "coverage.json".to_string(),
            coverage_sha256: "sha".to_string(),
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            ranked_candidates: Vec::new(),
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::NotRecommended,
                top_candidate_id: None,
                open_blockers: Vec::new(),
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            evidence_summary: crate::family::promotion_artifacts::EvidenceSummary {
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
                warnings: Vec::new(),
                summary: "fixture".to_string(),
            },
            delta_from_previous: RecommendationDelta::no_previous_artifact(),
        }
    }

    fn helper_surface_follow_on_basis_fixture() -> FamilyRecommendationAnalysisArtifact {
        FamilyRecommendationAnalysisArtifact {
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            ranked_candidates: vec![helper_surface_candidate_fixture()],
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::NotRecommended,
                top_candidate_id: Some("fixture".to_string()),
                open_blockers: vec![DecisionReason::HelperSurfaceNotPromotable],
                warnings: vec![DecisionReason::RegressionWarning],
                summary: "fixture".to_string(),
            },
            evidence_summary: crate::family::promotion_artifacts::EvidenceSummary {
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
                warnings: vec![DecisionReason::RegressionWarning],
                summary: "fixture".to_string(),
            },
            ..analysis_basis_fixture()
        }
    }

    fn helper_surface_candidate_fixture() -> RecommendationCandidateEntry {
        let tuple = durable_non_promotable_helper_surface_candidate_tuple();
        RecommendationCandidateEntry {
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
                tier: DifficultyTier::Hard,
                why: "fixture".to_string(),
            },
            confidence: RecommendationConfidence {
                level: ConfidenceLevel::Low,
                why: "fixture".to_string(),
            },
            rationale: "fixture".to_string(),
        }
    }

    fn stop_basis_fixture() -> FamilyRecommendationAnalysisArtifact {
        FamilyRecommendationAnalysisArtifact {
            recommendation_status: RecommendationStatus::InsufficientRealCorpus,
            ..analysis_basis_fixture()
        }
    }
}
