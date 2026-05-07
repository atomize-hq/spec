use crate::XtaskError;
use crate::family::coverage::render_json_bytes;
use crate::family::decision_kernel::{
    corpus_program_basis_snapshot, derive_corpus_program_decision_contract,
};
use crate::family::paths::{
    FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH, FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
};
use crate::family::promotion_artifacts::{
    CorpusProgramDecisionAction, CorpusProgramDecisionArtifact, CorpusProgramDecisionBasisCode,
    DecisionReason, DecisionStatus, EvidenceState, FamilyRecommendationAnalysisArtifact,
    RecommendationStatus, RequiredNextAction,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const COMMAND_NAME: &str = "family verify-decision-contract";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum CheckStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum OverallVerdict {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
enum FailureReason {
    MissingRecommendationAnalysisArtifact,
    MissingCorpusProgramDecisionArtifact,
    InvalidArtifactJson,
    InvalidArtifactContract,
    BasisSnapshotMismatch,
    DerivedDecisionMismatch,
    FrozenHelperSurfaceEvidenceNotCurrent,
    FrozenHelperSurfaceFloorMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FieldMismatch {
    field: String,
    expected: Value,
    observed: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CheckResult {
    status: CheckStatus,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    failure_reasons: Vec<FailureReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    mismatches: Vec<FieldMismatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct VerificationChecks {
    recommendation_analysis_validation: CheckResult,
    corpus_program_decision_validation: CheckResult,
    basis_snapshot_parity: CheckResult,
    derived_decision_parity: CheckResult,
    frozen_helper_surface_floor: CheckResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct VerificationReport {
    command: String,
    format: String,
    recommendation_analysis_path: String,
    corpus_program_decision_path: String,
    checks: VerificationChecks,
    overall_verdict: OverallVerdict,
    failure_reasons: Vec<FailureReason>,
}

#[derive(Debug)]
enum ArtifactLoad<T> {
    Ready(T),
    Missing(PathBuf),
    InvalidJson { path: PathBuf, error: String },
    InvalidContract { path: PathBuf, error: String },
}

impl CheckResult {
    fn pass() -> Self {
        Self {
            status: CheckStatus::Pass,
            failure_reasons: Vec::new(),
            detail: None,
            mismatches: Vec::new(),
        }
    }

    fn fail(reason: FailureReason, detail: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Fail,
            failure_reasons: vec![reason],
            detail: Some(detail.into()),
            mismatches: Vec::new(),
        }
    }

    fn fail_with_mismatches(
        reason: FailureReason,
        detail: impl Into<String>,
        mismatches: Vec<FieldMismatch>,
    ) -> Self {
        Self {
            status: CheckStatus::Fail,
            failure_reasons: vec![reason],
            detail: Some(detail.into()),
            mismatches,
        }
    }

    fn fail_from_reasons(reasons: Vec<FailureReason>, detail: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Fail,
            failure_reasons: reasons,
            detail: Some(detail.into()),
            mismatches: Vec::new(),
        }
    }
}

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
            "family verify-decision-contract only supports `--format json`, found `{format}`"
        )));
    }

    let report = build_report(workspace_root);
    let bytes = render_json_bytes(&report)?;
    writer.write_all(&bytes).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to write verify-decision-contract output: {error}"
        ))
    })?;
    writer.flush().map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to flush verify-decision-contract output: {error}"
        ))
    })?;

    if report.overall_verdict == OverallVerdict::Pass {
        Ok(())
    } else {
        Err(XtaskError::InvalidInput(
            "family verify-decision-contract detected contract drift".to_string(),
        ))
    }
}

fn build_report(workspace_root: &Path) -> VerificationReport {
    let analysis_load = load_recommendation_analysis(workspace_root);
    let decision_load = load_corpus_program_decision(workspace_root);

    let recommendation_analysis_validation = artifact_check_result(
        &analysis_load,
        FailureReason::MissingRecommendationAnalysisArtifact,
        "recommendation analysis artifact",
    );

    let corpus_program_decision_validation =
        decision_validation_result(workspace_root, &analysis_load, &decision_load);

    let basis_snapshot_parity = basis_snapshot_parity_result(
        &analysis_load,
        &decision_load,
        &corpus_program_decision_validation,
    );
    let derived_decision_parity = derived_decision_parity_result(
        &analysis_load,
        &decision_load,
        &corpus_program_decision_validation,
    );
    let frozen_helper_surface_floor = frozen_helper_surface_floor_result(
        &analysis_load,
        &decision_load,
        &corpus_program_decision_validation,
    );

    let checks = VerificationChecks {
        recommendation_analysis_validation,
        corpus_program_decision_validation,
        basis_snapshot_parity,
        derived_decision_parity,
        frozen_helper_surface_floor,
    };
    let failure_reasons = collect_failure_reasons(&checks);
    let overall_verdict = if failure_reasons.is_empty() {
        OverallVerdict::Pass
    } else {
        OverallVerdict::Fail
    };

    VerificationReport {
        command: COMMAND_NAME.to_string(),
        format: "json".to_string(),
        recommendation_analysis_path: FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH.to_string(),
        corpus_program_decision_path: FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH.to_string(),
        checks,
        overall_verdict,
        failure_reasons,
    }
}

fn load_recommendation_analysis(
    workspace_root: &Path,
) -> ArtifactLoad<FamilyRecommendationAnalysisArtifact> {
    let path = workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return ArtifactLoad::Missing(path);
        }
        Err(error) => {
            return ArtifactLoad::InvalidContract {
                path,
                error: format!("failed to read artifact: {error}"),
            };
        }
    };
    let artifact: FamilyRecommendationAnalysisArtifact = match serde_json::from_slice(&bytes) {
        Ok(artifact) => artifact,
        Err(error) => {
            return ArtifactLoad::InvalidJson {
                path,
                error: error.to_string(),
            };
        }
    };
    match artifact.validate(workspace_root) {
        Ok(()) => ArtifactLoad::Ready(artifact),
        Err(error) => ArtifactLoad::InvalidContract {
            path,
            error: error.to_string(),
        },
    }
}

fn load_corpus_program_decision(
    workspace_root: &Path,
) -> ArtifactLoad<CorpusProgramDecisionArtifact> {
    let path = workspace_root.join(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH);
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return ArtifactLoad::Missing(path);
        }
        Err(error) => {
            return ArtifactLoad::InvalidContract {
                path,
                error: format!("failed to read artifact: {error}"),
            };
        }
    };
    let artifact: CorpusProgramDecisionArtifact = match serde_json::from_slice(&bytes) {
        Ok(artifact) => artifact,
        Err(error) => {
            return ArtifactLoad::InvalidJson {
                path,
                error: error.to_string(),
            };
        }
    };
    ArtifactLoad::Ready(artifact)
}

fn artifact_check_result<T>(
    load: &ArtifactLoad<T>,
    missing_reason: FailureReason,
    artifact_label: &str,
) -> CheckResult {
    match load {
        ArtifactLoad::Ready(_) => CheckResult::pass(),
        ArtifactLoad::Missing(path) => CheckResult::fail(
            missing_reason,
            format!("missing {artifact_label} at `{}`", path.display()),
        ),
        ArtifactLoad::InvalidJson { path, error } => CheckResult::fail(
            FailureReason::InvalidArtifactJson,
            format!("failed to deserialize `{}`: {error}", path.display()),
        ),
        ArtifactLoad::InvalidContract { path, error } => CheckResult::fail(
            FailureReason::InvalidArtifactContract,
            format!("artifact contract failed for `{}`: {error}", path.display()),
        ),
    }
}

fn decision_validation_result(
    workspace_root: &Path,
    analysis_load: &ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
    decision_load: &ArtifactLoad<CorpusProgramDecisionArtifact>,
) -> CheckResult {
    match decision_load {
        ArtifactLoad::Missing(path) => CheckResult::fail(
            FailureReason::MissingCorpusProgramDecisionArtifact,
            format!(
                "missing corpus program decision artifact at `{}`",
                path.display()
            ),
        ),
        ArtifactLoad::InvalidJson { path, error } => CheckResult::fail(
            FailureReason::InvalidArtifactJson,
            format!("failed to deserialize `{}`: {error}", path.display()),
        ),
        ArtifactLoad::InvalidContract { path, error } => CheckResult::fail(
            FailureReason::InvalidArtifactContract,
            format!("artifact contract failed for `{}`: {error}", path.display()),
        ),
        ArtifactLoad::Ready(artifact) => match analysis_load {
            ArtifactLoad::Ready(_) => match artifact.validate_contract_surface(workspace_root) {
                Ok(_) => CheckResult::pass(),
                Err(error) => CheckResult::fail(
                    FailureReason::InvalidArtifactContract,
                    format!(
                        "artifact contract failed for `{}`: {error}",
                        FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH
                    ),
                ),
            },
            _ => CheckResult::fail_from_reasons(
                prerequisite_reasons_from_analysis(analysis_load),
                "recommendation analysis artifact must validate before the corpus program decision contract can be verified",
            ),
        },
    }
}

fn basis_snapshot_parity_result(
    analysis_load: &ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
    decision_load: &ArtifactLoad<CorpusProgramDecisionArtifact>,
    decision_validation: &CheckResult,
) -> CheckResult {
    let (analysis, decision) = match ready_artifacts(
        analysis_load,
        decision_load,
        decision_validation,
        "basis snapshot parity",
    ) {
        Ok(values) => values,
        Err(result) => return result,
    };

    let expected = corpus_program_basis_snapshot(analysis);
    let observed = &decision.basis_snapshot;
    let mut mismatches = Vec::new();
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.recommendation_status",
        &expected.recommendation_status,
        &observed.recommendation_status,
    );
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.decision_status",
        &expected.decision_status,
        &observed.decision_status,
    );
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.top_candidate_id",
        &expected.top_candidate_id,
        &observed.top_candidate_id,
    );
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.open_blockers",
        &expected.open_blockers,
        &observed.open_blockers,
    );
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.missing_evidence",
        &expected.missing_evidence,
        &observed.missing_evidence,
    );
    push_mismatch(
        &mut mismatches,
        "basis_snapshot.stale_evidence",
        &expected.stale_evidence,
        &observed.stale_evidence,
    );

    if mismatches.is_empty() {
        CheckResult::pass()
    } else {
        CheckResult::fail_with_mismatches(
            FailureReason::BasisSnapshotMismatch,
            "corpus program decision basis_snapshot drifted from the validated recommendation analysis",
            mismatches,
        )
    }
}

fn derived_decision_parity_result(
    analysis_load: &ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
    decision_load: &ArtifactLoad<CorpusProgramDecisionArtifact>,
    decision_validation: &CheckResult,
) -> CheckResult {
    let (analysis, decision) = match ready_artifacts(
        analysis_load,
        decision_load,
        decision_validation,
        "derived decision parity",
    ) {
        Ok(values) => values,
        Err(result) => return result,
    };

    let expected = match derive_corpus_program_decision_contract(analysis) {
        Ok(expected) => expected,
        Err(error) => {
            return CheckResult::fail(
                FailureReason::InvalidArtifactContract,
                format!(
                    "failed to derive decision contract from validated analysis artifact: {error}"
                ),
            );
        }
    };

    let mut mismatches = Vec::new();
    push_mismatch(
        &mut mismatches,
        "decision_action",
        &expected.decision_action,
        &decision.decision_action,
    );
    push_mismatch(
        &mut mismatches,
        "decision_basis_code",
        &expected.decision_basis_code,
        &decision.decision_basis_code,
    );
    push_mismatch(
        &mut mismatches,
        "pivot_target_class",
        &expected.pivot_target_class,
        &decision.pivot_target_class,
    );
    push_mismatch(
        &mut mismatches,
        "required_next_action",
        &expected.required_next_action,
        &decision.required_next_action,
    );
    push_mismatch(
        &mut mismatches,
        "summary",
        &expected.summary,
        &decision.summary,
    );

    if mismatches.is_empty() {
        CheckResult::pass()
    } else {
        CheckResult::fail_with_mismatches(
            FailureReason::DerivedDecisionMismatch,
            "corpus program decision semantic fields drifted from the kernel-derived contract",
            mismatches,
        )
    }
}

fn frozen_helper_surface_floor_result(
    analysis_load: &ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
    decision_load: &ArtifactLoad<CorpusProgramDecisionArtifact>,
    decision_validation: &CheckResult,
) -> CheckResult {
    let (analysis, decision) = match ready_artifacts(
        analysis_load,
        decision_load,
        decision_validation,
        "frozen helper-surface floor",
    ) {
        Ok(values) => values,
        Err(result) => return result,
    };

    let mut reasons = Vec::new();
    let mut mismatches = Vec::new();
    if !analysis.evidence_summary.missing_evidence.is_empty()
        || !analysis.evidence_summary.stale_evidence.is_empty()
    {
        reasons.push(FailureReason::FrozenHelperSurfaceEvidenceNotCurrent);
    }
    push_mismatch(
        &mut mismatches,
        "recommendation_status",
        &RecommendationStatus::NoStrongCandidate,
        &analysis.recommendation_status,
    );
    push_mismatch(
        &mut mismatches,
        "decision_summary.decision_status",
        &DecisionStatus::NotRecommended,
        &analysis.decision_summary.decision_status,
    );
    push_mismatch(
        &mut mismatches,
        "decision_summary.open_blockers",
        &vec![DecisionReason::HelperSurfaceNotPromotable],
        &analysis.decision_summary.open_blockers,
    );
    push_mismatch(
        &mut mismatches,
        "evidence_summary.missing_evidence",
        &Vec::<EvidenceState>::new(),
        &analysis.evidence_summary.missing_evidence,
    );
    push_mismatch(
        &mut mismatches,
        "evidence_summary.stale_evidence",
        &Vec::<EvidenceState>::new(),
        &analysis.evidence_summary.stale_evidence,
    );
    push_mismatch(
        &mut mismatches,
        "decision_action",
        &CorpusProgramDecisionAction::PivotToArchitectureSharedCoreFollowOn,
        &decision.decision_action,
    );
    push_mismatch(
        &mut mismatches,
        "decision_basis_code",
        &CorpusProgramDecisionBasisCode::DurableNonPromotableHelperSurface,
        &decision.decision_basis_code,
    );
    push_mismatch(
        &mut mismatches,
        "required_next_action",
        &RequiredNextAction::AuthorArchitectureFollowOnPlan,
        &decision.required_next_action,
    );

    if !mismatches.is_empty() {
        reasons.push(FailureReason::FrozenHelperSurfaceFloorMismatch);
    }

    if reasons.is_empty() {
        CheckResult::pass()
    } else {
        CheckResult {
            status: CheckStatus::Fail,
            failure_reasons: reasons,
            detail: Some(
                "the validated artifacts no longer satisfy the frozen helper-surface floor"
                    .to_string(),
            ),
            mismatches,
        }
    }
}

fn ready_artifacts<'a>(
    analysis_load: &'a ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
    decision_load: &'a ArtifactLoad<CorpusProgramDecisionArtifact>,
    decision_validation: &CheckResult,
    label: &str,
) -> Result<
    (
        &'a FamilyRecommendationAnalysisArtifact,
        &'a CorpusProgramDecisionArtifact,
    ),
    CheckResult,
> {
    let analysis = match analysis_load {
        ArtifactLoad::Ready(artifact) => artifact,
        _ => {
            return Err(CheckResult::fail_from_reasons(
                prerequisite_reasons_from_analysis(analysis_load),
                format!(
                    "cannot evaluate {label} because the recommendation analysis artifact is unavailable"
                ),
            ));
        }
    };
    if decision_validation.status == CheckStatus::Fail {
        return Err(CheckResult::fail_from_reasons(
            decision_validation.failure_reasons.clone(),
            format!(
                "cannot evaluate {label} because the corpus program decision contract did not validate"
            ),
        ));
    }
    let decision = match decision_load {
        ArtifactLoad::Ready(artifact) => artifact,
        ArtifactLoad::Missing(_) => {
            return Err(CheckResult::fail(
                FailureReason::MissingCorpusProgramDecisionArtifact,
                format!(
                    "cannot evaluate {label} because `{}` is missing",
                    FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH
                ),
            ));
        }
        ArtifactLoad::InvalidJson { path, error } => {
            return Err(CheckResult::fail(
                FailureReason::InvalidArtifactJson,
                format!(
                    "cannot evaluate {label} because `{}` is invalid JSON: {error}",
                    path.display()
                ),
            ));
        }
        ArtifactLoad::InvalidContract { path, error } => {
            return Err(CheckResult::fail(
                FailureReason::InvalidArtifactContract,
                format!(
                    "cannot evaluate {label} because `{}` failed contract validation: {error}",
                    path.display()
                ),
            ));
        }
    };
    Ok((analysis, decision))
}

fn prerequisite_reasons_from_analysis(
    analysis_load: &ArtifactLoad<FamilyRecommendationAnalysisArtifact>,
) -> Vec<FailureReason> {
    match analysis_load {
        ArtifactLoad::Ready(_) => Vec::new(),
        ArtifactLoad::Missing(_) => vec![FailureReason::MissingRecommendationAnalysisArtifact],
        ArtifactLoad::InvalidJson { .. } => vec![FailureReason::InvalidArtifactJson],
        ArtifactLoad::InvalidContract { .. } => vec![FailureReason::InvalidArtifactContract],
    }
}

fn collect_failure_reasons(checks: &VerificationChecks) -> Vec<FailureReason> {
    let mut reasons = Vec::new();
    for check in [
        &checks.recommendation_analysis_validation,
        &checks.corpus_program_decision_validation,
        &checks.basis_snapshot_parity,
        &checks.derived_decision_parity,
        &checks.frozen_helper_surface_floor,
    ] {
        for reason in &check.failure_reasons {
            if !reasons.contains(reason) {
                reasons.push(*reason);
            }
        }
    }
    reasons
}

fn push_mismatch<T: Serialize + PartialEq>(
    mismatches: &mut Vec<FieldMismatch>,
    field: &str,
    expected: &T,
    observed: &T,
) {
    if expected == observed {
        return;
    }
    mismatches.push(FieldMismatch {
        field: field.to_string(),
        expected: serde_json::to_value(expected).expect("expected value must serialize"),
        observed: serde_json::to_value(observed).expect("observed value must serialize"),
    });
}

#[cfg(test)]
mod tests {
    use super::{
        CheckStatus, FailureReason, OverallVerdict, VerificationReport, build_report,
        run_with_writer,
    };
    use crate::XtaskError;
    use crate::family::coverage::render_json_bytes;
    use crate::family::decision_kernel::{
        corpus_program_basis_snapshot, derive_corpus_program_decision_contract,
    };
    use crate::family::inventory::inventory_sha256_hex;
    use crate::family::paths::{
        FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH, FAMILY_COVERAGE_LATEST_PATH,
        FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
    };
    use crate::family::promotion_artifacts::{
        ConfidenceLevel, CorpusProgramDecisionArtifact, DecisionSummary, DifficultyTier,
        EvidenceState, EvidenceSummary, FamilyRecommendationAnalysisArtifact, HoldReason,
        NextStepDetail, NextStepStatus, PromotionArtifactKind, PromotionReadiness,
        RECOMMENDATION_ANALYSIS_SCHEMA_VERSION, RecommendationCandidateEntry,
        RecommendationConfidence, RecommendationDelta, RecommendationDifficulty,
        RecommendationLeverage, RecommendationStatus,
    };
    use spec_core::semantic_review::UnsupportedFunctionReasonCode;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn verifier_passes_on_frozen_helper_surface_floor() {
        let workspace = seeded_workspace();
        let report = build_report(workspace.path());

        assert_eq!(report.overall_verdict, OverallVerdict::Pass);
        assert!(report.failure_reasons.is_empty());
        assert_eq!(
            report.checks.recommendation_analysis_validation.status,
            CheckStatus::Pass
        );
        assert_eq!(
            report.checks.corpus_program_decision_validation.status,
            CheckStatus::Pass
        );
        assert_eq!(
            report.checks.basis_snapshot_parity.status,
            CheckStatus::Pass
        );
        assert_eq!(
            report.checks.derived_decision_parity.status,
            CheckStatus::Pass
        );
        assert_eq!(
            report.checks.frozen_helper_surface_floor.status,
            CheckStatus::Pass
        );
    }

    #[test]
    fn verifier_reports_missing_recommendation_artifact() {
        let workspace = seeded_workspace();
        fs::remove_file(
            workspace
                .path()
                .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH),
        )
        .unwrap();

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::MissingRecommendationAnalysisArtifact)
        );
    }

    #[test]
    fn verifier_reports_missing_decision_artifact() {
        let workspace = seeded_workspace();
        fs::remove_file(
            workspace
                .path()
                .join(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH),
        )
        .unwrap();

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::MissingCorpusProgramDecisionArtifact)
        );
    }

    #[test]
    fn verifier_reports_invalid_json_for_analysis_artifact() {
        let workspace = seeded_workspace();
        fs::write(
            workspace
                .path()
                .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH),
            b"{",
        )
        .unwrap();

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::InvalidArtifactJson)
        );
    }

    #[test]
    fn verifier_reports_invalid_json_for_decision_artifact() {
        let workspace = seeded_workspace();
        fs::write(
            workspace
                .path()
                .join(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH),
            b"{",
        )
        .unwrap();

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::InvalidArtifactJson)
        );
    }

    #[test]
    fn verifier_reports_invalid_artifact_contract() {
        let workspace = seeded_workspace();
        let mut analysis = read_analysis(workspace.path());
        analysis.schema_version += 1;
        write_analysis_only(workspace.path(), &analysis);

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::InvalidArtifactContract)
        );
    }

    #[test]
    fn verifier_reports_basis_snapshot_mismatch() {
        let workspace = seeded_workspace();
        let mut decision = read_decision(workspace.path());
        decision.basis_snapshot.top_candidate_id = Some("drifted".to_string());
        write_decision(workspace.path(), &decision);

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::BasisSnapshotMismatch)
        );
    }

    #[test]
    fn verifier_reports_derived_decision_mismatch() {
        let workspace = seeded_workspace();
        let mut decision = read_decision(workspace.path());
        decision.summary.push_str(" drift");
        write_decision(workspace.path(), &decision);

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::DerivedDecisionMismatch)
        );
    }

    #[test]
    fn verifier_reports_non_current_helper_surface_evidence() {
        let workspace = seeded_workspace();
        let mut analysis = read_analysis(workspace.path());
        analysis.evidence_summary.stale_evidence = vec![EvidenceState::StaleEvidence];
        let mut decision = read_decision(workspace.path());
        sync_decision_to_analysis(&analysis, &mut decision);
        write_analysis_and_decision(workspace.path(), &analysis, &decision);

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::FrozenHelperSurfaceEvidenceNotCurrent)
        );
    }

    #[test]
    fn verifier_reports_frozen_helper_surface_floor_mismatch() {
        let workspace = seeded_workspace();
        let mut decision = read_decision(workspace.path());
        decision.decision_basis_code =
            crate::family::promotion_artifacts::CorpusProgramDecisionBasisCode::NoActionableCandidate;
        write_decision(workspace.path(), &decision);

        let report = failing_report(workspace.path());
        assert!(
            report
                .failure_reasons
                .contains(&FailureReason::FrozenHelperSurfaceFloorMismatch)
        );
    }

    #[test]
    fn verifier_rejects_non_json_format() {
        let workspace = seeded_workspace();
        let mut stdout = Vec::new();
        let error = run_with_writer(workspace.path(), "yaml", &mut stdout).unwrap_err();

        assert!(stdout.is_empty());
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("only supports `--format json`")));
    }

    fn failing_report(workspace_root: &Path) -> VerificationReport {
        let mut stdout = Vec::new();
        let error = run_with_writer(workspace_root, "json", &mut stdout).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
        serde_json::from_slice(&stdout).unwrap()
    }

    fn seeded_workspace() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let coverage_path = temp_dir.path().join(FAMILY_COVERAGE_LATEST_PATH);
        fs::create_dir_all(coverage_path.parent().unwrap()).unwrap();
        let coverage_bytes = b"{\"fixture\":\"coverage\"}\n";
        fs::write(&coverage_path, coverage_bytes).unwrap();

        let analysis = fixture_analysis_artifact();
        let analysis_bytes = render_json_bytes(&analysis).unwrap();
        fs::write(
            temp_dir
                .path()
                .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH),
            &analysis_bytes,
        )
        .unwrap();

        let derived = derive_corpus_program_decision_contract(&analysis).unwrap();
        let decision = CorpusProgramDecisionArtifact {
            schema_version: 1,
            artifact_kind: PromotionArtifactKind::CorpusProgramDecision,
            generated_at: "2026-05-06T00:00:00Z".to_string(),
            analysis_basis_path: FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH.to_string(),
            analysis_basis_sha256: inventory_sha256_hex(&analysis_bytes),
            basis_snapshot: corpus_program_basis_snapshot(&analysis),
            decision_action: derived.decision_action,
            decision_basis_code: derived.decision_basis_code,
            pivot_target_class: derived.pivot_target_class,
            required_next_action: derived.required_next_action,
            summary: derived.summary,
        };
        write_decision(temp_dir.path(), &decision);
        temp_dir
    }

    fn fixture_analysis_artifact() -> FamilyRecommendationAnalysisArtifact {
        FamilyRecommendationAnalysisArtifact {
            schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
            artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
            generated_at: "2026-05-06T00:00:00Z".to_string(),
            coverage_path: FAMILY_COVERAGE_LATEST_PATH.to_string(),
            coverage_sha256: inventory_sha256_hex(b"{\"fixture\":\"coverage\"}\n"),
            recommendation_status: RecommendationStatus::NoStrongCandidate,
            ranked_candidates: vec![RecommendationCandidateEntry {
                candidate_id: "fixture/helper_surface".to_string(),
                cluster_ids: vec!["cluster".to_string()],
                primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
                overlap_family: "unknown".to_string(),
                promotion_readiness: PromotionReadiness::Hold,
                hold_reasons: vec![HoldReason::HelperSurfaceNotPromotable],
                next_step_status: NextStepStatus::DurableHold,
                next_step_detail: NextStepDetail::HelperSurfaceNotPromotable,
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
            }],
            decision_summary: DecisionSummary {
                decision_status: crate::family::promotion_artifacts::DecisionStatus::NotRecommended,
                top_candidate_id: Some("fixture/helper_surface".to_string()),
                open_blockers: vec![
                    crate::family::promotion_artifacts::DecisionReason::HelperSurfaceNotPromotable,
                ],
                warnings: vec![
                    crate::family::promotion_artifacts::DecisionReason::RegressionWarning,
                ],
                summary: "fixture".to_string(),
            },
            evidence_summary: EvidenceSummary {
                missing_evidence: Vec::new(),
                stale_evidence: Vec::new(),
                warnings: vec![
                    crate::family::promotion_artifacts::DecisionReason::RegressionWarning,
                ],
                summary: "fixture".to_string(),
            },
            delta_from_previous: RecommendationDelta::no_previous_artifact(),
        }
    }

    fn read_analysis(workspace_root: &Path) -> FamilyRecommendationAnalysisArtifact {
        let bytes =
            fs::read(workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH)).unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn read_decision(workspace_root: &Path) -> CorpusProgramDecisionArtifact {
        let bytes =
            fs::read(workspace_root.join(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH)).unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn write_analysis_only(workspace_root: &Path, analysis: &FamilyRecommendationAnalysisArtifact) {
        let bytes = render_json_bytes(analysis).unwrap();
        fs::write(
            workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH),
            bytes,
        )
        .unwrap();
    }

    fn write_decision(workspace_root: &Path, decision: &CorpusProgramDecisionArtifact) {
        let bytes = render_json_bytes(decision).unwrap();
        fs::write(
            workspace_root.join(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH),
            bytes,
        )
        .unwrap();
    }

    fn write_analysis_and_decision(
        workspace_root: &Path,
        analysis: &FamilyRecommendationAnalysisArtifact,
        decision: &CorpusProgramDecisionArtifact,
    ) {
        write_analysis_only(workspace_root, analysis);
        write_decision(workspace_root, decision);
    }

    fn sync_decision_to_analysis(
        analysis: &FamilyRecommendationAnalysisArtifact,
        decision: &mut CorpusProgramDecisionArtifact,
    ) {
        let analysis_bytes = render_json_bytes(analysis).unwrap();
        decision.analysis_basis_sha256 =
            crate::family::inventory::inventory_sha256_hex(&analysis_bytes);
        decision.basis_snapshot = corpus_program_basis_snapshot(analysis);
        let derived = derive_corpus_program_decision_contract(analysis).unwrap();
        decision.decision_action = derived.decision_action;
        decision.decision_basis_code = derived.decision_basis_code;
        decision.pivot_target_class = derived.pivot_target_class;
        decision.required_next_action = derived.required_next_action;
        decision.summary = derived.summary;
    }
}
