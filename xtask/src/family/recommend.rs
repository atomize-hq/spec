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
    CandidateStatus, ConfidenceLevel, DecisionReason, DecisionStatus, DecisionSummary,
    DifficultyTier, EvidenceState, EvidenceSummary, FamilyCoverageArtifact,
    FamilyRecommendationAnalysisArtifact, HoldReason, NextStepDetail, NextStepStatus,
    PromotionArtifactKind, PromotionReadiness, RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
    RecommendationCandidateEntry, RecommendationConfidence, RecommendationDelta,
    RecommendationDifficulty, RecommendationLeverage, RecommendationStatus,
    UnsupportedClusterEntry, candidate_qualifies_for_ranked_status,
};
use serde::Deserialize;
use spec_core::semantic_review::UnsupportedFunctionReasonCode;
use std::collections::BTreeSet;
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
    let evidence_summary = evidence_summary_for(&ranked_candidates);
    let decision_summary = decision_summary_for(recommendation_status, &ranked_candidates);

    FamilyRecommendationAnalysisArtifact {
        schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
        artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
        generated_at,
        coverage_path,
        coverage_sha256,
        recommendation_status,
        ranked_candidates,
        decision_summary,
        evidence_summary,
        delta_from_previous: RecommendationDelta::no_previous_artifact(),
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
    mut artifact: FamilyRecommendationAnalysisArtifact,
) -> Result<Vec<u8>, XtaskError> {
    let latest_path = FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH;
    let existing = load_existing_recommendation_artifact(workspace_root);
    if let Some((existing_artifact, existing_bytes)) = &existing
        && normalized_recommendation_for_determinism(existing_artifact)
            == normalized_recommendation_for_determinism(&artifact)
    {
        return Ok(existing_bytes.clone());
    }
    artifact.delta_from_previous =
        delta_from_previous(existing.as_ref().map(|(artifact, _)| artifact), &artifact);
    let latest_bytes = render_json_bytes(&artifact)?;
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
    normalized.delta_from_previous = RecommendationDelta::normalized_placeholder();
    normalized
}

fn decision_summary_for(
    recommendation_status: RecommendationStatus,
    ranked_candidates: &[RecommendationCandidateEntry],
) -> DecisionSummary {
    let top_candidate = ranked_candidates.first();
    let open_blockers = blockers_for(top_candidate);
    let warnings = warnings_for(top_candidate);
    let evidence_summary = evidence_summary_for(ranked_candidates);
    let decision_status =
        decision_status_for(recommendation_status, top_candidate, &evidence_summary);
    let summary = match (decision_status, top_candidate) {
        (DecisionStatus::Recommended, Some(candidate)) => format!(
            "Recommend promoting `{}` now; no missing or stale evidence is recorded.",
            candidate.candidate_id
        ),
        (DecisionStatus::BlockedForNow, Some(candidate)) => format!(
            "Do not promote `{}` yet; blockers `{}` and the current evidence state keep it held.",
            candidate.candidate_id,
            join_blockers(&open_blockers)
        ),
        (DecisionStatus::NotRecommended, Some(candidate))
            if candidate.next_step_status == NextStepStatus::DurableHold =>
        {
            format!(
                "Do not treat `{}` as the next family move; `{}` remains visible but helper surfaces are not promotable.",
                candidate.candidate_id,
                join_blockers(&open_blockers)
            )
        }
        (DecisionStatus::NotRecommended, Some(candidate)) => format!(
            "The current surface does not justify promoting `{}` as the next family move.",
            candidate.candidate_id
        ),
        (DecisionStatus::NotRecommended, None) => {
            "No plausible next-family action is recommended from the current analysis surface."
                .to_string()
        }
        (DecisionStatus::BlockedForNow, None) | (DecisionStatus::Recommended, None) => {
            "Decision summary could not identify a top candidate.".to_string()
        }
    };

    DecisionSummary {
        decision_status,
        top_candidate_id: top_candidate.map(|candidate| candidate.candidate_id.clone()),
        open_blockers,
        warnings,
        summary,
    }
}

fn evidence_summary_for(ranked_candidates: &[RecommendationCandidateEntry]) -> EvidenceSummary {
    let top_candidate = ranked_candidates.first();
    let missing_evidence = missing_evidence_for(top_candidate);
    let stale_evidence = Vec::new();
    let warnings = warnings_for(top_candidate);
    let summary = if top_candidate.is_none() {
        "No candidate-specific evidence obligations are recorded.".to_string()
    } else if missing_evidence.is_empty() && stale_evidence.is_empty() {
        "No missing or stale evidence is recorded.".to_string()
    } else {
        format!(
            "Missing evidence `{}`; stale evidence `{}`.",
            join_evidence(&missing_evidence),
            join_evidence(&stale_evidence)
        )
    };

    EvidenceSummary {
        missing_evidence,
        stale_evidence,
        warnings,
        summary,
    }
}

fn delta_from_previous(
    previous: Option<&FamilyRecommendationAnalysisArtifact>,
    current: &FamilyRecommendationAnalysisArtifact,
) -> RecommendationDelta {
    let Some(previous) = previous else {
        return RecommendationDelta::no_previous_artifact();
    };

    let current_reasons = current
        .decision_summary
        .open_blockers
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let previous_reasons = previous
        .decision_summary
        .open_blockers
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let reasons_added = current_reasons
        .difference(&previous_reasons)
        .copied()
        .collect::<Vec<_>>();
    let reasons_cleared = previous_reasons
        .difference(&current_reasons)
        .copied()
        .collect::<Vec<_>>();
    let evidence_changes = evidence_changes(previous, current);
    let decision_changed =
        current.decision_summary.decision_status != previous.decision_summary.decision_status;
    let top_candidate_changed =
        current.decision_summary.top_candidate_id != previous.decision_summary.top_candidate_id;
    let summary = if !decision_changed
        && !top_candidate_changed
        && reasons_added.is_empty()
        && reasons_cleared.is_empty()
        && evidence_changes.is_empty()
    {
        "Decision surface unchanged from the previous validated analysis artifact.".to_string()
    } else {
        format!(
            "Decision delta: decision_changed={}, top_candidate_changed={}, reasons_added={}, reasons_cleared={}, evidence_changes={}.",
            decision_changed,
            top_candidate_changed,
            join_blockers(&reasons_added),
            join_blockers(&reasons_cleared),
            if evidence_changes.is_empty() {
                "none".to_string()
            } else {
                evidence_changes.join(", ")
            }
        )
    };

    RecommendationDelta {
        previous_generated_at: Some(previous.generated_at.clone()),
        previous_decision_status: Some(previous.decision_summary.decision_status),
        previous_recommendation_status: Some(previous.recommendation_status),
        decision_changed,
        top_candidate_changed,
        reasons_added,
        reasons_cleared,
        evidence_changes,
        summary,
    }
}

fn evidence_changes(
    previous: &FamilyRecommendationAnalysisArtifact,
    current: &FamilyRecommendationAnalysisArtifact,
) -> Vec<String> {
    let mut changes = Vec::new();
    for change in diff_evidence(
        "missing_evidence",
        &previous.evidence_summary.missing_evidence,
        &current.evidence_summary.missing_evidence,
    ) {
        changes.push(change);
    }
    for change in diff_evidence(
        "stale_evidence",
        &previous.evidence_summary.stale_evidence,
        &current.evidence_summary.stale_evidence,
    ) {
        changes.push(change);
    }
    changes
}

fn diff_evidence(kind: &str, previous: &[EvidenceState], current: &[EvidenceState]) -> Vec<String> {
    let previous = previous.iter().copied().collect::<BTreeSet<_>>();
    let current = current.iter().copied().collect::<BTreeSet<_>>();
    let mut changes = current
        .difference(&previous)
        .map(|value| format!("{kind}:+{}", evidence_state_name(*value)))
        .collect::<Vec<_>>();
    changes.extend(
        previous
            .difference(&current)
            .map(|value| format!("{kind}:-{}", evidence_state_name(*value))),
    );
    changes
}

fn decision_status_for(
    recommendation_status: RecommendationStatus,
    top_candidate: Option<&RecommendationCandidateEntry>,
    evidence_summary: &EvidenceSummary,
) -> DecisionStatus {
    let has_evidence_gaps =
        !evidence_summary.missing_evidence.is_empty() || !evidence_summary.stale_evidence.is_empty();
    match top_candidate {
        Some(candidate)
            if candidate.promotion_readiness == PromotionReadiness::Ready
                && matches!(
                    candidate.confidence.level,
                    ConfidenceLevel::Medium | ConfidenceLevel::High
                )
                && !has_evidence_gaps =>
        {
            DecisionStatus::Recommended
        }
        Some(candidate)
            if candidate.next_step_status == NextStepStatus::DurableHold
                && candidate.next_step_detail == NextStepDetail::HelperSurfaceNotPromotable =>
        {
            DecisionStatus::NotRecommended
        }
        Some(_) if recommendation_status == RecommendationStatus::InsufficientRealCorpus => {
            DecisionStatus::NotRecommended
        }
        Some(_) => DecisionStatus::BlockedForNow,
        None => DecisionStatus::NotRecommended,
    }
}

fn blockers_for(candidate: Option<&RecommendationCandidateEntry>) -> Vec<DecisionReason> {
    let mut blockers = Vec::new();
    for hold_reason in candidate
        .map(|candidate| candidate.hold_reasons.as_slice())
        .unwrap_or(&[])
    {
        let reason = match hold_reason {
            HoldReason::UnknownOverlapFamily => DecisionReason::UnknownOverlapFamily,
            HoldReason::HardDifficulty => DecisionReason::HardDifficulty,
            HoldReason::ThinRealExampleSupport => DecisionReason::ThinRealExampleSupport,
            HoldReason::ThinRegressionSupport => DecisionReason::ThinRegressionSupport,
            HoldReason::HelperSurfaceNotPromotable => DecisionReason::HelperSurfaceNotPromotable,
        };
        if !blockers.contains(&reason) {
            blockers.push(reason);
        }
    }
    blockers
}

fn missing_evidence_for(candidate: Option<&RecommendationCandidateEntry>) -> Vec<EvidenceState> {
    let mut missing = Vec::new();
    for hold_reason in candidate
        .map(|candidate| candidate.hold_reasons.as_slice())
        .unwrap_or(&[])
    {
        let Some(state) = (match hold_reason {
            HoldReason::ThinRealExampleSupport => Some(EvidenceState::ThinRealExampleSupport),
            HoldReason::ThinRegressionSupport => Some(EvidenceState::ThinRegressionSupport),
            HoldReason::UnknownOverlapFamily
            | HoldReason::HardDifficulty
            | HoldReason::HelperSurfaceNotPromotable => None,
        }) else {
            continue;
        };
        if !missing.contains(&state) {
            missing.push(state);
        }
    }
    missing
}

fn warnings_for(candidate: Option<&RecommendationCandidateEntry>) -> Vec<DecisionReason> {
    if candidate.is_some_and(|candidate| candidate.leverage.promotion_relevant_regression_hits > 0)
    {
        vec![DecisionReason::RegressionWarning]
    } else {
        Vec::new()
    }
}

fn join_blockers(blockers: &[DecisionReason]) -> String {
    if blockers.is_empty() {
        "none".to_string()
    } else {
        blockers
            .iter()
            .map(|reason| match reason {
                DecisionReason::UnknownOverlapFamily => "unknown_overlap_family",
                DecisionReason::HardDifficulty => "hard_difficulty",
                DecisionReason::ThinRealExampleSupport => "thin_real_example_support",
                DecisionReason::ThinRegressionSupport => "thin_regression_support",
                DecisionReason::HelperSurfaceNotPromotable => "helper_surface_not_promotable",
                DecisionReason::RegressionWarning => "regression_warning",
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn join_evidence(evidence: &[EvidenceState]) -> String {
    if evidence.is_empty() {
        "none".to_string()
    } else {
        evidence
            .iter()
            .map(|state| evidence_state_name(*state))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn evidence_state_name(state: EvidenceState) -> &'static str {
    match state {
        EvidenceState::ThinRealExampleSupport => "thin_real_example_support",
        EvidenceState::ThinRegressionSupport => "thin_regression_support",
        EvidenceState::StaleEvidence => "stale_evidence",
    }
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
