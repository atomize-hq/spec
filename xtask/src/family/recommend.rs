use crate::XtaskError;
use crate::family::coverage::{
    collect_and_write_latest, current_timestamp_rfc3339, render_json_bytes,
};
use crate::family::inventory::inventory_sha256_hex;
use crate::family::paths::{FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH, write_bytes_atomically};
use crate::family::promotion_artifacts::{
    CandidateStatus, ConfidenceLevel, DifficultyTier, FamilyRecommendationAnalysisArtifact,
    PromotionArtifactKind, RecommendationCandidateEntry, RecommendationConfidence,
    RecommendationDifficulty, RecommendationLeverage, RecommendationStatus,
};
use spec_core::semantic_review::UnsupportedFunctionReasonCode;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::path::Path;

pub(crate) fn run(workspace_root: &Path, format: &str) -> Result<(), XtaskError> {
    if format != "json" {
        return Err(XtaskError::InvalidInput(format!(
            "family recommend only supports `--format json`, found `{format}`"
        )));
    }

    let coverage = collect_and_write_latest(workspace_root)?;
    let generated_at = current_timestamp_rfc3339()?;
    let coverage_sha256 = inventory_sha256_hex(&coverage.latest_bytes);
    let mut ranked_candidates = coverage
        .artifact
        .unsupported_clusters
        .iter()
        .filter(|cluster| cluster.candidate_status == CandidateStatus::Rankable)
        .map(|cluster| {
            let difficulty = difficulty_for(cluster.reason_code);
            let confidence = confidence_for(
                cluster.real_example_hits,
                cluster.promotion_relevant_regression_hits,
            );
            RecommendationCandidateEntry {
                candidate_id: candidate_id(cluster.overlap_family.as_str(), cluster),
                cluster_ids: vec![cluster.cluster_id.clone()],
                primary_reason_code: cluster.reason_code,
                overlap_family: cluster.overlap_family.clone(),
                leverage: RecommendationLeverage {
                    real_example_hits: cluster.real_example_hits,
                    promotion_relevant_regression_hits: cluster.promotion_relevant_regression_hits,
                    boundary_only_hits: cluster.boundary_only_hits,
                    total_units_in_cluster: cluster.representative_unit_ids.len(),
                },
                difficulty,
                confidence,
                rationale: format!(
                    "Rankable cluster with {} real-example hit(s), {} promotion-relevant regression hit(s), and {} boundary-only hit(s).",
                    cluster.real_example_hits,
                    cluster.promotion_relevant_regression_hits,
                    cluster.boundary_only_hits
                ),
            }
        })
        .collect::<Vec<_>>();

    ranked_candidates.sort_by(compare_candidates);

    let recommendation_status = if ranked_candidates.is_empty() {
        RecommendationStatus::InsufficientRealCorpus
    } else if ranked_candidates
        .iter()
        .all(|candidate| candidate.confidence.level == ConfidenceLevel::Low)
        && ranked_candidates
            .iter()
            .all(|candidate| candidate.leverage.real_example_hits == 0)
    {
        RecommendationStatus::InsufficientRealCorpus
    } else if ranked_candidates
        .iter()
        .all(|candidate| candidate.confidence.level == ConfidenceLevel::Low)
    {
        RecommendationStatus::NoStrongCandidate
    } else {
        RecommendationStatus::Ranked
    };

    let artifact = FamilyRecommendationAnalysisArtifact {
        schema_version: 1,
        artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
        generated_at,
        coverage_path: coverage.latest_path,
        coverage_sha256,
        recommendation_status,
        ranked_candidates,
    };

    let latest_bytes = render_json_bytes(&artifact)?;
    write_bytes_atomically(
        &workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH),
        &latest_bytes,
    )?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&latest_bytes).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write recommendation output: {error}"))
    })?;
    stdout.flush().map_err(|error| {
        XtaskError::WriteFailure(format!("failed to flush recommendation output: {error}"))
    })
}

fn compare_candidates(
    left: &RecommendationCandidateEntry,
    right: &RecommendationCandidateEntry,
) -> Ordering {
    right
        .leverage
        .real_example_hits
        .cmp(&left.leverage.real_example_hits)
        .then_with(|| {
            right
                .leverage
                .promotion_relevant_regression_hits
                .cmp(&left.leverage.promotion_relevant_regression_hits)
        })
        .then_with(|| left.difficulty.tier.cmp(&right.difficulty.tier))
        .then_with(|| {
            left.leverage
                .boundary_only_hits
                .cmp(&right.leverage.boundary_only_hits)
        })
        .then_with(|| left.candidate_id.cmp(&right.candidate_id))
}

fn difficulty_for(reason_code: UnsupportedFunctionReasonCode) -> RecommendationDifficulty {
    match reason_code {
        UnsupportedFunctionReasonCode::UnsupportedArithmeticShape => RecommendationDifficulty {
            tier: DifficultyTier::Adjacent,
            why: "Arithmetic-shaped unsupported demand is adjacent to the promoted arithmetic leaf families.".to_string(),
        },
        UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape => RecommendationDifficulty {
            tier: DifficultyTier::Adjacent,
            why: "Wrapper-body unsupported demand is adjacent to the promoted wrapper pipeline families.".to_string(),
        },
        UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression => {
            RecommendationDifficulty {
                tier: DifficultyTier::Moderate,
                why: "Required-argument expression support needs broader wrapper argument handling.".to_string(),
            }
        }
        UnsupportedFunctionReasonCode::UnsupportedDepTopology
        | UnsupportedFunctionReasonCode::UnsupportedControlFlow
        | UnsupportedFunctionReasonCode::UnsupportedFunctionSurface => {
            RecommendationDifficulty {
                tier: DifficultyTier::Hard,
                why: "This unsupported demand expands the semantic reviewer beyond the currently promoted family boundaries.".to_string(),
            }
        }
    }
}

fn confidence_for(
    real_example_hits: usize,
    promotion_relevant_regression_hits: usize,
) -> RecommendationConfidence {
    let level = if real_example_hits >= 2 {
        ConfidenceLevel::High
    } else if real_example_hits == 1 || promotion_relevant_regression_hits >= 3 {
        ConfidenceLevel::Medium
    } else {
        ConfidenceLevel::Low
    };
    let why = match level {
        ConfidenceLevel::High => {
            "Multiple real-example hits demonstrate repeated unsupported demand.".to_string()
        }
        ConfidenceLevel::Medium => {
            "The cluster is backed by at least one real example or a concentrated regression signal.".to_string()
        }
        ConfidenceLevel::Low => {
            "The cluster is visible, but the current real-example support is still thin.".to_string()
        }
    };
    RecommendationConfidence { level, why }
}

fn candidate_id(
    overlap_family: &str,
    cluster: &crate::family::promotion_artifacts::UnsupportedClusterEntry,
) -> String {
    let prefix = if overlap_family == "unknown" {
        "z"
    } else {
        "a"
    };
    format!("{prefix}-{:?}-{}", cluster.reason_code, cluster.cluster_id).to_ascii_lowercase()
}
