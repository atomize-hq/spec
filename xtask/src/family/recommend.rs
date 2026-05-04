use crate::XtaskError;
use crate::family::coverage::{
    CoverageRunOutput, collect_latest, current_timestamp_rfc3339,
    normalized_for_recommend_determinism, render_json_bytes, write_latest,
};
use crate::family::inventory::inventory_sha256_hex;
use crate::family::paths::{
    FAMILY_COVERAGE_LATEST_PATH, FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH, write_bytes_atomically,
};
use crate::family::promotion_artifacts::{
    CandidateStatus, ConfidenceLevel, DifficultyTier, FamilyCoverageArtifact,
    FamilyRecommendationAnalysisArtifact, HoldReason, NextStepDetail, NextStepStatus,
    PromotionArtifactKind, PromotionReadiness, RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
    RecommendationCandidateEntry, RecommendationConfidence, RecommendationDifficulty,
    RecommendationLeverage, RecommendationStatus, UnsupportedClusterEntry,
    candidate_qualifies_for_ranked_status,
};
use serde::Deserialize;
use spec_core::semantic_review::UnsupportedFunctionReasonCode;
use std::cmp::Ordering;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub(crate) fn run(workspace_root: &Path, format: &str) -> Result<(), XtaskError> {
    let mut stdout = io::stdout().lock();
    run_with_writer(workspace_root, format, &mut stdout)
}

pub(crate) fn run_with_writer<W: Write>(
    workspace_root: &Path,
    format: &str,
    writer: &mut W,
) -> Result<(), XtaskError> {
    if format != "json" {
        return Err(XtaskError::InvalidInput(format!(
            "family recommend only supports `--format json`, found `{format}`"
        )));
    }

    let coverage = effective_coverage_for_recommend(workspace_root)?;
    let artifact = build_recommendation_analysis_artifact(
        current_timestamp_rfc3339()?,
        coverage.latest_path,
        inventory_sha256_hex(&coverage.latest_bytes),
        &coverage.artifact.unsupported_clusters,
    );
    let latest_bytes = effective_recommendation_bytes(workspace_root, artifact)?;
    writer.write_all(&latest_bytes).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write recommendation output: {error}"))
    })?;
    writer.flush().map_err(|error| {
        XtaskError::WriteFailure(format!("failed to flush recommendation output: {error}"))
    })
}

pub(crate) fn build_recommendation_analysis_artifact(
    generated_at: String,
    coverage_path: String,
    coverage_sha256: String,
    unsupported_clusters: &[UnsupportedClusterEntry],
) -> FamilyRecommendationAnalysisArtifact {
    let discovery_candidates = project_discovery_candidates(unsupported_clusters);
    let mut ranked_candidates = discovery_candidates
        .into_iter()
        .map(adjudicate_candidate)
        .collect::<Vec<_>>();
    ranked_candidates.sort_by(compare_candidates);
    let recommendation_status = recommendation_status_for(&ranked_candidates);

    FamilyRecommendationAnalysisArtifact {
        schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
        artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
        generated_at,
        coverage_path,
        coverage_sha256,
        recommendation_status,
        ranked_candidates,
    }
}

fn effective_coverage_for_recommend(
    workspace_root: &Path,
) -> Result<CoverageRunOutput, XtaskError> {
    let pending = collect_latest(workspace_root)?;
    let latest_path = FAMILY_COVERAGE_LATEST_PATH.to_string();
    if let Some((existing, existing_bytes)) = load_existing_coverage_artifact(workspace_root)
        && normalized_for_recommend_determinism(&existing)
            == normalized_for_recommend_determinism(&pending.artifact)
    {
        return Ok(CoverageRunOutput {
            artifact: existing,
            latest_bytes: existing_bytes,
            latest_path,
        });
    }
    write_latest(workspace_root, &pending)
}

fn effective_recommendation_bytes(
    workspace_root: &Path,
    artifact: FamilyRecommendationAnalysisArtifact,
) -> Result<Vec<u8>, XtaskError> {
    let latest_bytes = render_json_bytes(&artifact)?;
    let latest_path = FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH;
    if let Some((existing, existing_bytes)) = load_existing_recommendation_artifact(workspace_root)
        && normalized_recommendation_for_determinism(&existing)
            == normalized_recommendation_for_determinism(&artifact)
    {
        return Ok(existing_bytes);
    }
    write_bytes_atomically(&workspace_root.join(latest_path), &latest_bytes)?;
    Ok(latest_bytes)
}

fn load_existing_coverage_artifact(
    workspace_root: &Path,
) -> Option<(FamilyCoverageArtifact, Vec<u8>)> {
    let path = workspace_root.join(FAMILY_COVERAGE_LATEST_PATH);
    let bytes = fs::read(path).ok()?;
    let artifact: FamilyCoverageArtifact = serde_json::from_slice(&bytes).ok()?;
    artifact.validate(workspace_root).ok()?;
    Some((artifact, bytes))
}

fn load_existing_recommendation_artifact(
    workspace_root: &Path,
) -> Option<(FamilyRecommendationAnalysisArtifact, Vec<u8>)> {
    let path = workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);
    let bytes = fs::read(path).ok()?;
    let artifact: FamilyRecommendationAnalysisArtifact = serde_json::from_slice(&bytes).ok()?;
    artifact.validate(workspace_root).ok()?;
    Some((artifact, bytes))
}

fn normalized_recommendation_for_determinism(
    artifact: &FamilyRecommendationAnalysisArtifact,
) -> FamilyRecommendationAnalysisArtifact {
    let mut normalized = artifact.clone();
    normalized.generated_at.clear();
    normalized
}

#[derive(Debug, Clone)]
struct DiscoveryProjectionCandidate {
    candidate_id: String,
    cluster_ids: Vec<String>,
    primary_reason_code: UnsupportedFunctionReasonCode,
    shape_fingerprint: String,
    overlap_family: String,
    leverage: RecommendationLeverage,
    difficulty: RecommendationDifficulty,
    rationale: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UnsupportedShapeFingerprint {
    schema_version: u64,
    function_dep_arity: usize,
    callable_dep_topology_class: String,
    contract_input_count: usize,
    has_return: bool,
    authored_body_kind: String,
}

fn project_discovery_candidates(
    unsupported_clusters: &[UnsupportedClusterEntry],
) -> Vec<DiscoveryProjectionCandidate> {
    unsupported_clusters
        .iter()
        .filter(|cluster| cluster.candidate_status == CandidateStatus::Rankable)
        .map(|cluster| DiscoveryProjectionCandidate {
            candidate_id: candidate_id(cluster.overlap_family.as_str(), cluster),
            cluster_ids: vec![cluster.cluster_id.clone()],
            primary_reason_code: cluster.reason_code,
            shape_fingerprint: cluster.shape_fingerprint.clone(),
            overlap_family: cluster.overlap_family.clone(),
            leverage: RecommendationLeverage {
                real_example_hits: cluster.real_example_hits,
                promotion_relevant_regression_hits: cluster.promotion_relevant_regression_hits,
                boundary_only_hits: cluster.boundary_only_hits,
                total_units_in_cluster: cluster.representative_unit_ids.len(),
            },
            difficulty: difficulty_for(cluster.reason_code),
            rationale: format!(
                "Rankable cluster with {} real-example hit(s), {} promotion-relevant regression hit(s), and {} boundary-only hit(s).",
                cluster.real_example_hits,
                cluster.promotion_relevant_regression_hits,
                cluster.boundary_only_hits
            ),
        })
        .collect()
}

fn adjudicate_candidate(discovery: DiscoveryProjectionCandidate) -> RecommendationCandidateEntry {
    let resolution = next_step_resolution_for(&discovery);
    let confidence = confidence_for(
        discovery.overlap_family.as_str(),
        discovery.difficulty.tier,
        discovery.leverage.real_example_hits,
        discovery.leverage.promotion_relevant_regression_hits,
    );

    RecommendationCandidateEntry {
        candidate_id: discovery.candidate_id,
        cluster_ids: discovery.cluster_ids,
        primary_reason_code: discovery.primary_reason_code,
        overlap_family: discovery.overlap_family,
        promotion_readiness: resolution.promotion_readiness,
        hold_reasons: resolution.hold_reasons,
        next_step_status: resolution.next_step_status,
        next_step_detail: resolution.next_step_detail,
        leverage: discovery.leverage,
        difficulty: discovery.difficulty,
        confidence,
        rationale: discovery.rationale,
    }
}

#[derive(Debug)]
struct CandidateResolution {
    promotion_readiness: PromotionReadiness,
    hold_reasons: Vec<HoldReason>,
    next_step_status: NextStepStatus,
    next_step_detail: NextStepDetail,
}

fn compare_candidates(
    left: &RecommendationCandidateEntry,
    right: &RecommendationCandidateEntry,
) -> Ordering {
    left.promotion_readiness
        .cmp(&right.promotion_readiness)
        .then_with(|| compare_candidates_within_bucket(left, right))
}

fn compare_candidates_within_bucket(
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
    overlap_family: &str,
    difficulty_tier: DifficultyTier,
    real_example_hits: usize,
    promotion_relevant_regression_hits: usize,
) -> RecommendationConfidence {
    let overlap_is_known = overlap_family != "unknown";
    let has_medium_signal = real_example_hits >= 2
        || (real_example_hits == 1
            && promotion_relevant_regression_hits >= 3
            && difficulty_tier != DifficultyTier::Hard);
    let level = if overlap_is_known && real_example_hits >= 3 {
        ConfidenceLevel::High
    } else if overlap_is_known && has_medium_signal {
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

fn next_step_resolution_for(discovery: &DiscoveryProjectionCandidate) -> CandidateResolution {
    if is_durable_helper_surface_hold(discovery) {
        return CandidateResolution {
            promotion_readiness: PromotionReadiness::Hold,
            hold_reasons: vec![HoldReason::HelperSurfaceNotPromotable],
            next_step_status: NextStepStatus::DurableHold,
            next_step_detail: NextStepDetail::HelperSurfaceNotPromotable,
        };
    }

    let hold_reasons = hold_reasons_for(
        discovery.overlap_family.as_str(),
        discovery.difficulty.tier,
        discovery.leverage.real_example_hits,
        discovery.leverage.promotion_relevant_regression_hits,
    );
    if hold_reasons.is_empty() {
        CandidateResolution {
            promotion_readiness: PromotionReadiness::Ready,
            hold_reasons,
            next_step_status: NextStepStatus::Promote,
            next_step_detail: NextStepDetail::ReadyForPromotion,
        }
    } else {
        CandidateResolution {
            promotion_readiness: PromotionReadiness::Hold,
            hold_reasons,
            next_step_status: NextStepStatus::TargetedEvidenceGap,
            next_step_detail: NextStepDetail::TargetedEvidenceGap,
        }
    }
}

fn hold_reasons_for(
    overlap_family: &str,
    difficulty_tier: DifficultyTier,
    real_example_hits: usize,
    promotion_relevant_regression_hits: usize,
) -> Vec<HoldReason> {
    let mut hold_reasons = Vec::new();

    if overlap_family == "unknown" {
        push_hold_reason(&mut hold_reasons, HoldReason::UnknownOverlapFamily);
    }
    if difficulty_tier == DifficultyTier::Hard && real_example_hits < 2 {
        push_hold_reason(&mut hold_reasons, HoldReason::HardDifficulty);
    }
    if real_example_hits == 0 || (real_example_hits == 1 && promotion_relevant_regression_hits < 3)
    {
        push_hold_reason(&mut hold_reasons, HoldReason::ThinRealExampleSupport);
    }
    if promotion_relevant_regression_hits <= 1 && real_example_hits <= 1 {
        push_hold_reason(&mut hold_reasons, HoldReason::ThinRegressionSupport);
    }

    hold_reasons
}

fn push_hold_reason(hold_reasons: &mut Vec<HoldReason>, hold_reason: HoldReason) {
    if !hold_reasons.contains(&hold_reason) {
        hold_reasons.push(hold_reason);
    }
}

fn is_durable_helper_surface_hold(discovery: &DiscoveryProjectionCandidate) -> bool {
    discovery.primary_reason_code == UnsupportedFunctionReasonCode::UnsupportedFunctionSurface
        && discovery.overlap_family == "unknown"
        && discovery.leverage.real_example_hits > 0
        && helper_surface_shape(discovery)
}

fn helper_surface_shape(discovery: &DiscoveryProjectionCandidate) -> bool {
    let Ok(fingerprint) =
        serde_json::from_str::<UnsupportedShapeFingerprint>(&discovery.shape_fingerprint)
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

pub(crate) fn recommendation_status_for(
    ranked_candidates: &[RecommendationCandidateEntry],
) -> RecommendationStatus {
    if ranked_candidates
        .first()
        .is_some_and(candidate_qualifies_for_ranked_status)
    {
        RecommendationStatus::Ranked
    } else if ranked_candidates.is_empty()
        || ranked_candidates.iter().all(|candidate| {
            candidate.promotion_readiness == PromotionReadiness::Hold
                && candidate.leverage.real_example_hits == 0
        })
    {
        RecommendationStatus::InsufficientRealCorpus
    } else {
        RecommendationStatus::NoStrongCandidate
    }
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
