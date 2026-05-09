use crate::XtaskError;
use crate::family::inventory::inventory_sha256_hex;
use crate::family::promotion_artifacts::{
    CorpusProgramDecisionArtifact, FamilyCoverageArtifact, FamilyRecommendationAnalysisArtifact,
    RecommendationDelta,
};
use serde::Serialize;

pub(crate) fn normalized_for_recommend_determinism(
    artifact: &FamilyCoverageArtifact,
) -> FamilyCoverageArtifact {
    let mut normalized = artifact.clone();
    normalized.generated_at.clear();
    normalized.inventory_path.clear();
    normalized.inventory_sha256.clear();
    normalized
}

pub(crate) fn normalized_coverage_proof_fingerprint(
    artifact: &FamilyCoverageArtifact,
) -> Result<String, XtaskError> {
    let normalized = normalized_for_recommend_determinism(artifact);
    let bytes = render_json_bytes(&normalized)?;
    Ok(inventory_sha256_hex(&bytes))
}

pub(crate) fn normalized_recommendation_proof_fingerprint(
    artifact: &FamilyRecommendationAnalysisArtifact,
) -> Result<String, XtaskError> {
    let mut normalized = artifact.clone();
    normalized.generated_at.clear();
    normalized.delta_from_previous = RecommendationDelta::normalized_placeholder();
    let bytes = render_json_bytes(&normalized)?;
    Ok(inventory_sha256_hex(&bytes))
}

pub(crate) fn normalized_corpus_program_decision_proof_fingerprint(
    artifact: &CorpusProgramDecisionArtifact,
) -> Result<String, XtaskError> {
    let mut normalized = artifact.clone();
    normalized.generated_at.clear();
    let bytes = render_json_bytes(&normalized)?;
    Ok(inventory_sha256_hex(&bytes))
}

fn render_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, XtaskError> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| XtaskError::WriteFailure(format!("failed to serialize JSON: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{
        normalized_corpus_program_decision_proof_fingerprint,
        normalized_coverage_proof_fingerprint, normalized_recommendation_proof_fingerprint,
    };
    use crate::family::promotion_artifacts::{
        CandidateStatus, CorpusProgramDecisionAction, CorpusProgramDecisionArtifact,
        CorpusProgramDecisionBasisCode, DecisionReason, DecisionStatus, DifficultyTier,
        EvidenceSummary, FamilyCoverageArtifact, FamilyCoverageEntry,
        FamilyRecommendationAnalysisArtifact, FunctionCoverageTotals, NonFunctionCoverageTotals,
        PivotTargetClass, PromotionArtifactKind, RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
        RecommendationCandidateEntry, RecommendationConfidence, RecommendationDelta,
        RecommendationDifficulty, RecommendationLeverage, RecommendationStatus, RequiredNextAction,
        SourceKind, UnsupportedClusterEntry,
    };
    use spec_core::semantic_review::UnsupportedFunctionReasonCode;

    #[test]
    fn moved_coverage_fingerprint_matches_previous_hash_for_same_semantics() {
        let artifact = coverage_artifact_fixture();
        let baseline = normalized_coverage_proof_fingerprint(&artifact).unwrap();

        let mut churned = artifact.clone();
        churned.generated_at = "2026-05-06T03:00:00Z".to_string();
        churned.inventory_path =
            ".semantic-family-artifacts/family-promotion/inventory/churned.json".to_string();
        churned.inventory_sha256 = "different-inventory-sha".to_string();

        assert_eq!(
            normalized_coverage_proof_fingerprint(&churned).unwrap(),
            baseline
        );
    }

    #[test]
    fn moved_coverage_fingerprint_changes_when_cluster_semantics_change() {
        let artifact = coverage_artifact_fixture();
        let baseline = normalized_coverage_proof_fingerprint(&artifact).unwrap();

        let mut changed = artifact.clone();
        changed.unsupported_clusters[0].real_example_hits = 3;

        assert_ne!(
            normalized_coverage_proof_fingerprint(&changed).unwrap(),
            baseline
        );
    }

    #[test]
    fn recommendation_fingerprint_stays_stable_across_generated_at_and_delta_churn() {
        let artifact = recommendation_artifact_fixture();
        let baseline = normalized_recommendation_proof_fingerprint(&artifact).unwrap();

        let mut churned = artifact.clone();
        churned.generated_at = "2026-05-06T03:00:00Z".to_string();
        churned.delta_from_previous = RecommendationDelta {
            previous_generated_at: Some("2026-05-04T03:00:00Z".to_string()),
            previous_decision_status: Some(DecisionStatus::BlockedForNow),
            previous_recommendation_status: Some(RecommendationStatus::Ranked),
            decision_changed: true,
            top_candidate_changed: true,
            reasons_added: vec![DecisionReason::ThinRealExampleSupport],
            reasons_cleared: vec![DecisionReason::HelperSurfaceNotPromotable],
            evidence_changes: vec!["missing_evidence:+thin_real_example_support".to_string()],
            summary: "churned".to_string(),
        };

        assert_eq!(
            normalized_recommendation_proof_fingerprint(&churned).unwrap(),
            baseline
        );
    }

    #[test]
    fn corpus_decision_fingerprint_changes_only_on_semantic_change() {
        let artifact = corpus_decision_artifact_fixture();
        let baseline = normalized_corpus_program_decision_proof_fingerprint(&artifact).unwrap();

        let mut churned = artifact.clone();
        churned.generated_at = "2026-05-06T03:00:00Z".to_string();
        assert_eq!(
            normalized_corpus_program_decision_proof_fingerprint(&churned).unwrap(),
            baseline
        );

        let mut changed = artifact.clone();
        changed.decision_action = CorpusProgramDecisionAction::Stop;
        changed.decision_basis_code = CorpusProgramDecisionBasisCode::NoActionableCandidate;
        changed.pivot_target_class = None;
        changed.required_next_action = RequiredNextAction::RecordStopWithoutNewMilestone;
        assert_ne!(
            normalized_corpus_program_decision_proof_fingerprint(&changed).unwrap(),
            baseline
        );
    }

    fn coverage_artifact_fixture() -> FamilyCoverageArtifact {
        FamilyCoverageArtifact {
            schema_version: 1,
            artifact_kind: PromotionArtifactKind::FamilyCoverageSnapshot,
            generated_at: "2026-05-05T00:00:00Z".to_string(),
            inventory_path: ".semantic-family-artifacts/family-promotion/inventory/current.json"
                .to_string(),
            inventory_sha256: "inventory-sha".to_string(),
            corpus_manifest_path: "xtask/fixtures/family-corpus.toml".to_string(),
            corpus_manifest_sha256: "manifest-sha".to_string(),
            sources: vec![crate::family::promotion_artifacts::CorpusSourceEntry {
                id: "real".to_string(),
                path: "fixtures/real".to_string(),
                kind: SourceKind::RealExample,
                counts_toward_recommendation: true,
                note: "fixture".to_string(),
                unit_count: 2,
            }],
            function_coverage: FunctionCoverageTotals {
                total_units: 2,
                promoted_family_units: 0,
                supported_unpromoted_family_units: 0,
                unsupported_function_units: 2,
            },
            non_function_coverage: NonFunctionCoverageTotals {
                total_units: 0,
                supported_sum_units: 0,
                supported_data_units: 0,
                other_units: 0,
            },
            family_coverage: vec![FamilyCoverageEntry {
                family: "function.wrapper.pipeline.v1".to_string(),
                unit_count: 1,
                unit_ids: vec!["real::pricing/calculate_total".to_string()],
                source_ids: vec!["real".to_string()],
            }],
            unsupported_clusters: vec![UnsupportedClusterEntry {
                cluster_id: "unsupported_function_surface-e40675da6fa0".to_string(),
                reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
                shape_fingerprint: "{\"schema_version\":1}".to_string(),
                representative_unit_ids: vec!["real::pricing/helper".to_string()],
                source_ids: vec!["real".to_string()],
                real_example_hits: 2,
                promotion_relevant_regression_hits: 1,
                boundary_only_hits: 0,
                overlap_family: "unknown".to_string(),
                candidate_status: CandidateStatus::Rankable,
            }],
        }
    }

    fn recommendation_artifact_fixture() -> FamilyRecommendationAnalysisArtifact {
        FamilyRecommendationAnalysisArtifact {
            schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
            artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
            generated_at: "2026-05-05T02:00:00Z".to_string(),
            coverage_path:
                ".semantic-family-artifacts/family-promotion/latest/family.coverage.json"
                    .to_string(),
            coverage_sha256: "coverage-sha".to_string(),
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            ranked_candidates: vec![RecommendationCandidateEntry {
                candidate_id: "z-unsupported_function_surface-e40675da6fa0".to_string(),
                cluster_ids: vec!["unsupported_function_surface-e40675da6fa0".to_string()],
                primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
                overlap_family: "unknown".to_string(),
                promotion_readiness: crate::family::promotion_artifacts::PromotionReadiness::Hold,
                hold_reasons: vec![
                    crate::family::promotion_artifacts::HoldReason::HelperSurfaceNotPromotable,
                ],
                next_step_status: crate::family::promotion_artifacts::NextStepStatus::DurableHold,
                next_step_detail:
                    crate::family::promotion_artifacts::NextStepDetail::HelperSurfaceNotPromotable,
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
                    level: crate::family::promotion_artifacts::ConfidenceLevel::Low,
                    why: "fixture".to_string(),
                },
                rationale: "fixture".to_string(),
            }],
            decision_summary: crate::family::promotion_artifacts::DecisionSummary {
                decision_status: DecisionStatus::NotRecommended,
                top_candidate_id: Some("z-unsupported_function_surface-e40675da6fa0".to_string()),
                open_blockers: vec![DecisionReason::HelperSurfaceNotPromotable],
                warnings: vec![DecisionReason::RegressionWarning],
                summary: "fixture".to_string(),
            },
            evidence_summary: EvidenceSummary {
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
                warnings: vec![DecisionReason::RegressionWarning],
                summary: "fixture".to_string(),
            },
            delta_from_previous: RecommendationDelta::no_previous_artifact(),
        }
    }

    fn corpus_decision_artifact_fixture() -> CorpusProgramDecisionArtifact {
        CorpusProgramDecisionArtifact {
            schema_version: 1,
            artifact_kind: PromotionArtifactKind::CorpusProgramDecision,
            generated_at: "2026-05-05T03:00:00Z".to_string(),
            analysis_basis_path:
                ".semantic-family-artifacts/family-promotion/latest/family.recommendation.analysis.json"
                    .to_string(),
            analysis_basis_sha256: "analysis-sha".to_string(),
            basis_snapshot: crate::family::promotion_artifacts::CorpusProgramBasisSnapshot {
                recommendation_status: RecommendationStatus::NoStrongCandidate,
                decision_status: DecisionStatus::NotRecommended,
                top_candidate_id: Some(
                    "z-unsupported_function_surface-e40675da6fa0".to_string(),
                ),
                open_blockers: vec![DecisionReason::HelperSurfaceNotPromotable],
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
            },
            decision_action: CorpusProgramDecisionAction::PivotToArchitectureSharedCoreFollowOn,
            decision_basis_code: CorpusProgramDecisionBasisCode::DurableNonPromotableHelperSurface,
            pivot_target_class: Some(PivotTargetClass::ArchitectureSharedCoreFollowOn),
            required_next_action: RequiredNextAction::AuthorArchitectureFollowOnPlan,
            summary: "fixture".to_string(),
        }
    }
}
