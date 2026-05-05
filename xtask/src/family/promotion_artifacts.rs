use crate::FamilyTargetLanguage;
use crate::XtaskError;
use crate::family::coverage::current_timestamp_rfc3339;
use crate::family::helper_surface::{
    basis_snapshot_requires_helper_surface_follow_on,
    decision_matches_helper_surface_follow_on_tuple, decision_uses_helper_surface_follow_on_tuple,
    recommendation_matches_helper_surface_durable_hold_tuple,
    recommendation_uses_helper_surface_durable_hold_tuple,
};
use crate::family::inventory::{inventory_sha256_hex, render_snapshot_bytes};
use crate::family::paths::{
    FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH, FAMILY_COVERAGE_LATEST_PATH,
    FAMILY_PROMOTION_ARTIFACT_ROOT, FAMILY_PROMOTION_INVENTORY_DIR,
    FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH, FamilyId, M27_CORPUS_MANIFEST_PATH,
    family_promotion_blocker_path, family_promotion_execution_path,
    family_recommendation_latest_path, validate_existing_repo_relative_path,
    validate_repo_relative_path, write_bytes_atomically,
};
use crate::family::prove::validate_target_language;
use crate::family::report::{CertificationReport, validate_report_artifact};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use spec_core::semantic_review::UnsupportedFunctionReasonCode;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub(crate) const RECOMMENDATION_SCHEMA_VERSION: u64 = 1;
pub(crate) const FAMILY_RECOMMENDATION_SCHEMA_VERSION: u64 = 2;
pub(crate) const COVERAGE_SCHEMA_VERSION: u64 = 1;
pub(crate) const RECOMMENDATION_ANALYSIS_SCHEMA_VERSION: u64 = 4;
pub(crate) const CORPUS_PROGRAM_DECISION_SCHEMA_VERSION: u64 = 1;
const PROMOTION_EXECUTION_SCHEMA_VERSION: u64 = 2;
const PROMOTION_BLOCKER_SCHEMA_VERSION: u64 = 2;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromotionArtifactKind {
    FamilyRecommendation,
    FamilyCoverageSnapshot,
    FamilyRecommendationAnalysis,
    CorpusProgramDecision,
    PromotionExecution,
    PromotionBlocker,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TargetLanguage {
    Rust,
    Typescript,
}

impl TargetLanguage {
    pub(crate) fn from_family_target_language(target_language: FamilyTargetLanguage) -> Self {
        match target_language {
            FamilyTargetLanguage::Rust => Self::Rust,
            FamilyTargetLanguage::Typescript => Self::Typescript,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExecutionStatus {
    Green,
    Blocked,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GateStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockingStep {
    Inventory,
    Scaffold,
    Smoke,
    Prove,
    Certify,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockerKind {
    InventoryProjectionMismatch,
    InventoryNoSupportedCandidate,
    ScaffoldContractMismatch,
    SmokeContractFailure,
    ProveSuiteFailure,
    CertifySuiteFailure,
    CertifyRoutingConflict,
    ProofArtifactMissing,
    HumanDecisionRequired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MachineEvidenceKind {
    Command,
    Artifact,
    Diff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SourceKind {
    RealExample,
    RegressionUnsupported,
    ProofOnly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CandidateStatus {
    Rankable,
    BoundaryOnly,
    LowValue,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RecommendationStatus {
    Ranked,
    NoStrongCandidate,
    InsufficientRealCorpus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DecisionStatus {
    Recommended,
    BlockedForNow,
    NotRecommended,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DecisionReason {
    UnknownOverlapFamily,
    HardDifficulty,
    ThinRealExampleSupport,
    ThinRegressionSupport,
    HelperSurfaceNotPromotable,
    RegressionWarning,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceState {
    ThinRealExampleSupport,
    ThinRegressionSupport,
    StaleEvidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DifficultyTier {
    Adjacent,
    Moderate,
    Hard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CorpusProgramDecisionAction {
    Stop,
    SpendCorpusRun1,
    PivotToFamilyPromotionRun,
    PivotToRecommendationPolicyRun,
    PivotToArchitectureSharedCoreFollowOn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CorpusProgramDecisionBasisCode {
    PromotionReadyCandidate,
    PlausibleCandidateMissingEvidence,
    DurableNonPromotableHelperSurface,
    NoActionableCandidate,
    PolicyInterpretationBlocker,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RequiredNextAction {
    RecordStopWithoutNewMilestone,
    AuthorCorpusExpansionPlan,
    AuthorFamilyPromotionPlan,
    AuthorRecommendationPolicyPlan,
    AuthorArchitectureFollowOnPlan,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PivotTargetClass {
    FamilyPromotionRun,
    RecommendationPolicyRun,
    ArchitectureSharedCoreFollowOn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromotionReadiness {
    Ready,
    Hold,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HoldReason {
    UnknownOverlapFamily,
    HardDifficulty,
    ThinRealExampleSupport,
    ThinRegressionSupport,
    HelperSurfaceNotPromotable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NextStepStatus {
    Promote,
    TargetedEvidenceGap,
    DurableHold,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NextStepDetail {
    ReadyForPromotion,
    TargetedEvidenceGap,
    HelperSurfaceNotPromotable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct FamilyRecommendationArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub generated_at: String,
    pub inventory_path: String,
    pub inventory_sha256: String,
    pub target_language: TargetLanguage,
    pub ranked_candidates: Vec<RankedCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_basis_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_basis_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_status: Option<DecisionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_blockers: Option<Vec<DecisionReason>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_or_stale_evidence: Option<Vec<EvidenceState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RankedCandidate {
    pub family: String,
    pub evidence: Vec<String>,
    pub expected_leverage: String,
    pub expected_risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionExecutionArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub run_id: String,
    pub family: String,
    pub target_language: TargetLanguage,
    pub status: ExecutionStatus,
    pub recommendation_path: String,
    pub analysis_basis_path: String,
    pub analysis_basis_sha256: String,
    pub decision_status_at_start: DecisionStatus,
    pub open_blockers_at_start: Vec<DecisionReason>,
    pub missing_or_stale_evidence_at_start: Vec<EvidenceState>,
    pub approvals: PromotionApprovals,
    pub files_changed: Vec<String>,
    pub commands: Vec<CommandRecord>,
    pub referenced_proof_artifacts: Vec<String>,
    pub iterations: u64,
    pub gate_summary: GateSummary,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionApprovals {
    pub target_family: ApprovalRecord,
    pub final_output: ApprovalRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct ApprovalRecord {
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct CommandRecord {
    pub step: String,
    pub command: String,
    pub exit_code: i32,
    pub started_at: String,
    pub finished_at: String,
    pub artifact_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct GateSummary {
    pub smoke: GateStatus,
    pub prove: GateStatus,
    pub certify: GateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionBlockerArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub run_id: String,
    pub family: String,
    pub target_language: TargetLanguage,
    pub analysis_basis_path: String,
    pub analysis_basis_sha256: String,
    pub decision_status_at_start: DecisionStatus,
    pub open_blockers_at_start: Vec<DecisionReason>,
    pub missing_or_stale_evidence_at_start: Vec<EvidenceState>,
    pub blocking_step: BlockingStep,
    pub blocker_kind: BlockerKind,
    pub summary: String,
    pub machine_evidence: Vec<MachineEvidence>,
    pub required_human_action: String,
    pub safe_next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct MachineEvidence {
    pub kind: MachineEvidenceKind,
    pub path: Option<String>,
    pub command: Option<String>,
    pub exit_code: Option<i32>,
    pub observed_at: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct FamilyCoverageArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub generated_at: String,
    pub inventory_path: String,
    pub inventory_sha256: String,
    pub corpus_manifest_path: String,
    pub corpus_manifest_sha256: String,
    pub sources: Vec<CorpusSourceEntry>,
    pub function_coverage: FunctionCoverageTotals,
    pub non_function_coverage: NonFunctionCoverageTotals,
    pub family_coverage: Vec<FamilyCoverageEntry>,
    pub unsupported_clusters: Vec<UnsupportedClusterEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct CorpusSourceEntry {
    pub id: String,
    pub path: String,
    pub kind: SourceKind,
    pub counts_toward_recommendation: bool,
    pub note: String,
    pub unit_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct FunctionCoverageTotals {
    pub total_units: usize,
    pub promoted_family_units: usize,
    pub supported_unpromoted_family_units: usize,
    pub unsupported_function_units: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct NonFunctionCoverageTotals {
    pub total_units: usize,
    pub supported_sum_units: usize,
    pub supported_data_units: usize,
    pub other_units: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct FamilyCoverageEntry {
    pub family: String,
    pub unit_count: usize,
    pub unit_ids: Vec<String>,
    pub source_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct UnsupportedClusterEntry {
    pub cluster_id: String,
    pub reason_code: UnsupportedFunctionReasonCode,
    pub shape_fingerprint: String,
    pub representative_unit_ids: Vec<String>,
    pub source_ids: Vec<String>,
    pub real_example_hits: usize,
    pub promotion_relevant_regression_hits: usize,
    pub boundary_only_hits: usize,
    pub overlap_family: String,
    pub candidate_status: CandidateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct FamilyRecommendationAnalysisArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub generated_at: String,
    pub coverage_path: String,
    pub coverage_sha256: String,
    pub recommendation_status: RecommendationStatus,
    pub ranked_candidates: Vec<RecommendationCandidateEntry>,
    pub decision_summary: DecisionSummary,
    pub evidence_summary: EvidenceSummary,
    pub delta_from_previous: RecommendationDelta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct CorpusProgramDecisionArtifact {
    pub schema_version: u64,
    pub artifact_kind: PromotionArtifactKind,
    pub generated_at: String,
    pub analysis_basis_path: String,
    pub analysis_basis_sha256: String,
    pub basis_snapshot: CorpusProgramBasisSnapshot,
    pub decision_action: CorpusProgramDecisionAction,
    pub decision_basis_code: CorpusProgramDecisionBasisCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_target_class: Option<PivotTargetClass>,
    pub required_next_action: RequiredNextAction,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct CorpusProgramBasisSnapshot {
    pub recommendation_status: RecommendationStatus,
    pub decision_status: DecisionStatus,
    pub top_candidate_id: Option<String>,
    pub open_blockers: Vec<DecisionReason>,
    pub missing_evidence: Vec<EvidenceState>,
    pub stale_evidence: Vec<EvidenceState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct DecisionSummary {
    pub decision_status: DecisionStatus,
    pub top_candidate_id: Option<String>,
    pub open_blockers: Vec<DecisionReason>,
    pub warnings: Vec<DecisionReason>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceSummary {
    pub missing_evidence: Vec<EvidenceState>,
    pub stale_evidence: Vec<EvidenceState>,
    pub warnings: Vec<DecisionReason>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecommendationDelta {
    pub previous_generated_at: Option<String>,
    pub previous_decision_status: Option<DecisionStatus>,
    pub previous_recommendation_status: Option<RecommendationStatus>,
    pub decision_changed: bool,
    pub top_candidate_changed: bool,
    pub reasons_added: Vec<DecisionReason>,
    pub reasons_cleared: Vec<DecisionReason>,
    pub evidence_changes: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecommendationCandidateEntry {
    pub candidate_id: String,
    pub cluster_ids: Vec<String>,
    pub primary_reason_code: UnsupportedFunctionReasonCode,
    pub overlap_family: String,
    pub promotion_readiness: PromotionReadiness,
    pub hold_reasons: Vec<HoldReason>,
    pub next_step_status: NextStepStatus,
    pub next_step_detail: NextStepDetail,
    pub leverage: RecommendationLeverage,
    pub difficulty: RecommendationDifficulty,
    pub confidence: RecommendationConfidence,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecommendationLeverage {
    pub real_example_hits: usize,
    pub promotion_relevant_regression_hits: usize,
    pub boundary_only_hits: usize,
    pub total_units_in_cluster: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecommendationDifficulty {
    pub tier: DifficultyTier,
    pub why: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecommendationConfidence {
    pub level: ConfidenceLevel,
    pub why: String,
}

pub(crate) fn candidate_qualifies_for_ranked_status(
    candidate: &RecommendationCandidateEntry,
) -> bool {
    candidate.promotion_readiness == PromotionReadiness::Ready
        && matches!(
            candidate.confidence.level,
            ConfidenceLevel::Medium | ConfidenceLevel::High
        )
}

pub(crate) fn run_refresh_recommendation(
    workspace_root: &Path,
    raw_family: &str,
    target_language: FamilyTargetLanguage,
) -> Result<(), XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    validate_target_language(
        &family,
        target_language,
        "family refresh-promotion-recommendation",
    )?;
    let (analysis_basis, analysis_basis_sha256) = load_analysis_basis_artifact(workspace_root)?;

    let generated_at = current_timestamp_rfc3339()?;
    let run_id = format_run_id(&generated_at, family.as_str())?;
    let inventory_path = Path::new(FAMILY_PROMOTION_INVENTORY_DIR).join(format!("{run_id}.json"));
    let inventory_bytes = render_snapshot_bytes(workspace_root)?;
    write_bytes_atomically(&workspace_root.join(&inventory_path), &inventory_bytes)?;

    let artifact = FamilyRecommendationArtifact {
        schema_version: FAMILY_RECOMMENDATION_SCHEMA_VERSION,
        artifact_kind: PromotionArtifactKind::FamilyRecommendation,
        generated_at,
        inventory_path: normalize_path(&inventory_path),
        inventory_sha256: inventory_sha256_hex(&inventory_bytes),
        target_language: TargetLanguage::from_family_target_language(target_language),
        ranked_candidates: vec![ranked_candidate_for_family(&family)?],
        analysis_basis_path: Some(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH.to_string()),
        analysis_basis_sha256: Some(analysis_basis_sha256),
        decision_status: Some(analysis_basis.decision_summary.decision_status),
        open_blockers: Some(analysis_basis.decision_summary.open_blockers.clone()),
        missing_or_stale_evidence: Some(missing_or_stale_evidence(&analysis_basis)),
    };
    let path = family_recommendation_latest_path(&family);
    let bytes = serde_json::to_vec_pretty(&artifact).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to serialize recommendation artifact: {error}"
        ))
    })?;
    write_bytes_atomically(&workspace_root.join(&path), &append_newline(bytes))?;
    Ok(())
}

pub(crate) fn run_emit_promotion_execution(
    workspace_root: &Path,
    raw_family: &str,
    run_id: &str,
    recommendation_path: &str,
    target_language: FamilyTargetLanguage,
    diff_base: &str,
) -> Result<(), XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    validate_target_language(&family, target_language, "family emit-promotion-execution")?;
    if !looks_like_run_id(run_id, family.as_str()) {
        return Err(XtaskError::InvalidInput(format!(
            "run_id `{run_id}` must match `{{UTC-basic-timestamp}}-{}`",
            family.as_str()
        )));
    }

    let prove_dir = Path::new(".semantic-family-artifacts")
        .join("semantic-families")
        .join(family.packet_dir_name());
    let prove_latest = prove_dir.join("prove.latest.json");
    let certification = prove_dir.join("certification.report.json");
    let latest_attempt = latest_attempt_relative_path(workspace_root, &prove_dir)?;
    let files_changed = git_diff_name_only(workspace_root, diff_base)?;
    let timestamp = current_timestamp_rfc3339()?;
    let target = TargetLanguage::from_family_target_language(target_language);
    let (analysis_basis, analysis_basis_sha256) = load_analysis_basis_artifact(workspace_root)?;
    let artifact = PromotionExecutionArtifact {
        schema_version: PROMOTION_EXECUTION_SCHEMA_VERSION,
        artifact_kind: PromotionArtifactKind::PromotionExecution,
        run_id: run_id.to_string(),
        family: family.as_str().to_string(),
        target_language: target,
        status: ExecutionStatus::Green,
        recommendation_path: recommendation_path.to_string(),
        analysis_basis_path: FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH.to_string(),
        analysis_basis_sha256,
        decision_status_at_start: analysis_basis.decision_summary.decision_status,
        open_blockers_at_start: analysis_basis.decision_summary.open_blockers.clone(),
        missing_or_stale_evidence_at_start: missing_or_stale_evidence(&analysis_basis),
        approvals: PromotionApprovals {
            target_family: ApprovalRecord {
                status: ApprovalStatus::Approved,
            },
            final_output: ApprovalRecord {
                status: ApprovalStatus::Pending,
            },
        },
        files_changed,
        commands: command_records_for_execution(&family, target_language, &timestamp),
        referenced_proof_artifacts: vec![
            normalize_path(&prove_latest),
            normalize_path(&latest_attempt),
            normalize_path(&certification),
        ],
        iterations: 1,
        gate_summary: GateSummary {
            smoke: GateStatus::Pass,
            prove: GateStatus::Pass,
            certify: GateStatus::Pass,
        },
        notes: vec![
            "Rust-default and requested target-language proof loops were rerun before artifact emission."
                .to_string(),
            "Referenced proof artifacts point at the latest merged-state prove, attempt, and certification outputs."
                .to_string(),
        ],
    };
    let path = family_promotion_execution_path(&family, run_id);
    let bytes = serde_json::to_vec_pretty(&artifact).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to serialize promotion execution artifact: {error}"
        ))
    })?;
    write_bytes_atomically(&workspace_root.join(&path), &append_newline(bytes))?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_emit_promotion_blocker(
    workspace_root: &Path,
    raw_family: &str,
    run_id: &str,
    target_language: FamilyTargetLanguage,
    blocking_step: BlockingStep,
    blocker_kind: BlockerKind,
    summary: &str,
    required_human_action: &str,
    safe_next_actions: &[String],
    evidence_command: Option<&str>,
    evidence_exit_code: Option<i32>,
    evidence_path: Option<&str>,
    evidence_note: &str,
) -> Result<(), XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    validate_target_language(&family, target_language, "family emit-promotion-blocker")?;
    if !looks_like_run_id(run_id, family.as_str()) {
        return Err(XtaskError::InvalidInput(format!(
            "run_id `{run_id}` must match `{{UTC-basic-timestamp}}-{}`",
            family.as_str()
        )));
    }
    let (analysis_basis, analysis_basis_sha256) = load_analysis_basis_artifact(workspace_root)?;

    let artifact = PromotionBlockerArtifact {
        schema_version: PROMOTION_BLOCKER_SCHEMA_VERSION,
        artifact_kind: PromotionArtifactKind::PromotionBlocker,
        run_id: run_id.to_string(),
        family: family.as_str().to_string(),
        target_language: TargetLanguage::from_family_target_language(target_language),
        analysis_basis_path: FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH.to_string(),
        analysis_basis_sha256,
        decision_status_at_start: analysis_basis.decision_summary.decision_status,
        open_blockers_at_start: analysis_basis.decision_summary.open_blockers.clone(),
        missing_or_stale_evidence_at_start: missing_or_stale_evidence(&analysis_basis),
        blocking_step,
        blocker_kind,
        summary: summary.to_string(),
        machine_evidence: vec![MachineEvidence {
            kind: if evidence_path.is_some() && evidence_command.is_none() {
                MachineEvidenceKind::Artifact
            } else if evidence_path.is_some() {
                MachineEvidenceKind::Diff
            } else {
                MachineEvidenceKind::Command
            },
            path: evidence_path.map(ToString::to_string),
            command: evidence_command.map(ToString::to_string),
            exit_code: evidence_exit_code,
            observed_at: current_timestamp_rfc3339()?,
            note: evidence_note.to_string(),
        }],
        required_human_action: required_human_action.to_string(),
        safe_next_actions: safe_next_actions.to_vec(),
    };
    let path = family_promotion_blocker_path(&family, run_id);
    let bytes = serde_json::to_vec_pretty(&artifact).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to serialize blocker artifact: {error}"))
    })?;
    write_bytes_atomically(&workspace_root.join(&path), &append_newline(bytes))?;
    Ok(())
}

pub(crate) fn run_validate_artifact(
    workspace_root: &Path,
    raw_path: &str,
) -> Result<(), XtaskError> {
    let absolute = resolve_requested_path(workspace_root, raw_path);
    let relative = absolute.strip_prefix(workspace_root).map_err(|_| {
        XtaskError::InvalidInput(format!(
            "artifact path `{}` must stay within the workspace root",
            absolute.display()
        ))
    })?;
    let artifact_path = classify_artifact_path(relative)?;
    let bytes = fs::read(&absolute).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to read artifact `{}`: {error}",
            absolute.display()
        ))
    })?;

    match artifact_path {
        ArtifactPath::SemanticFamilyReport => {
            let artifact: CertificationReport =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            validate_report_artifact(relative, &artifact)?;
        }
        ArtifactPath::Recommendation => {
            let artifact: FamilyRecommendationArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root, None)?;
        }
        ArtifactPath::RecommendationByFamily { family } => {
            let artifact: FamilyRecommendationArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root, Some(&family))?;
        }
        ArtifactPath::Coverage => {
            let artifact: FamilyCoverageArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root)?;
        }
        ArtifactPath::RecommendationAnalysis => {
            let artifact: FamilyRecommendationAnalysisArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root)?;
        }
        ArtifactPath::CorpusProgramDecision => {
            let artifact: CorpusProgramDecisionArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root)?;
        }
        ArtifactPath::PromotionExecution { family, run_id } => {
            let artifact: PromotionExecutionArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root, &family, &run_id)?;
        }
        ArtifactPath::PromotionBlocker { family, run_id } => {
            let artifact: PromotionBlockerArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root, &family, &run_id)?;
        }
    }

    Ok(())
}

impl FamilyRecommendationArtifact {
    fn validate(&self, workspace_root: &Path, path_family: Option<&str>) -> Result<(), XtaskError> {
        let expected_schema = if path_family.is_some() {
            FAMILY_RECOMMENDATION_SCHEMA_VERSION
        } else {
            RECOMMENDATION_SCHEMA_VERSION
        };
        if self.schema_version != expected_schema {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation schema_version must be {expected_schema}, found {}",
                self.schema_version,
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::FamilyRecommendation {
            return Err(XtaskError::InvalidInput(
                "recommendation artifact_kind must be `family_recommendation`".to_string(),
            ));
        }
        if !looks_like_utc_timestamp(&self.generated_at) {
            return Err(XtaskError::InvalidInput(
                "recommendation generated_at must be a UTC RFC3339 timestamp".to_string(),
            ));
        }
        if path_family.is_none() && self.target_language != TargetLanguage::Rust {
            return Err(XtaskError::InvalidInput(
                "global recommendation target_language must be `rust`".to_string(),
            ));
        }
        if self.ranked_candidates.is_empty() {
            return Err(XtaskError::InvalidInput(
                "recommendation must include at least one ranked candidate".to_string(),
            ));
        }

        let inventory_path = validate_existing_repo_relative_path(
            workspace_root,
            &self.inventory_path,
            "recommendation inventory_path",
        )?;
        if !self
            .inventory_path
            .starts_with(FAMILY_PROMOTION_INVENTORY_DIR)
        {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation inventory_path `{}` must stay under `{FAMILY_PROMOTION_INVENTORY_DIR}`",
                self.inventory_path
            )));
        }
        let inventory_bytes = fs::read(&inventory_path).map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to read inventory snapshot `{}`: {error}",
                inventory_path.display()
            ))
        })?;
        let observed_sha = inventory_sha256_hex(&inventory_bytes);
        if self.inventory_sha256 != observed_sha {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation inventory_sha256 `{}` does not match the exact bytes at `{}` (expected `{observed_sha}`)",
                self.inventory_sha256, self.inventory_path
            )));
        }

        for candidate in &self.ranked_candidates {
            FamilyId::parse(&candidate.family)?;
            if candidate.expected_leverage.trim().is_empty()
                || candidate.expected_leverage.contains('\n')
            {
                return Err(XtaskError::InvalidInput(format!(
                    "recommendation expected_leverage for `{}` must be a single non-empty line",
                    candidate.family
                )));
            }
            if candidate.evidence.is_empty() {
                return Err(XtaskError::InvalidInput(format!(
                    "recommendation candidate `{}` must cite at least one repo path in evidence[]",
                    candidate.family
                )));
            }
            for evidence_path in &candidate.evidence {
                validate_existing_repo_relative_path(
                    workspace_root,
                    evidence_path,
                    "recommendation evidence path",
                )?;
            }
        }

        if let Some(path_family) = path_family {
            FamilyId::parse(path_family)?;
            if self.ranked_candidates.len() != 1 {
                return Err(XtaskError::InvalidInput(format!(
                    "family-scoped recommendation for `{path_family}` must contain exactly one ranked candidate"
                )));
            }
            if self.ranked_candidates[0].family != path_family {
                return Err(XtaskError::InvalidInput(format!(
                    "family-scoped recommendation path family `{path_family}` must match ranked candidate `{}`",
                    self.ranked_candidates[0].family
                )));
            }
            let analysis_basis_path = self.analysis_basis_path.as_deref().ok_or_else(|| {
                XtaskError::InvalidInput(
                    "family-scoped recommendation must include analysis_basis_path".to_string(),
                )
            })?;
            let analysis_basis_sha256 = self.analysis_basis_sha256.as_deref().ok_or_else(|| {
                XtaskError::InvalidInput(
                    "family-scoped recommendation must include analysis_basis_sha256".to_string(),
                )
            })?;
            let decision_status = self.decision_status.ok_or_else(|| {
                XtaskError::InvalidInput(
                    "family-scoped recommendation must include decision_status".to_string(),
                )
            })?;
            let open_blockers = self.open_blockers.as_ref().ok_or_else(|| {
                XtaskError::InvalidInput(
                    "family-scoped recommendation must include open_blockers".to_string(),
                )
            })?;
            let basis_missing_or_stale_evidence =
                self.missing_or_stale_evidence.as_ref().ok_or_else(|| {
                    XtaskError::InvalidInput(
                        "family-scoped recommendation must include missing_or_stale_evidence"
                            .to_string(),
                    )
                })?;
            let analysis_basis = validate_analysis_basis_reference(
                workspace_root,
                analysis_basis_path,
                analysis_basis_sha256,
                "family-scoped recommendation",
            )?;
            if decision_status != analysis_basis.decision_summary.decision_status {
                return Err(XtaskError::InvalidInput(
                    "family-scoped recommendation decision_status must match the analysis basis"
                        .to_string(),
                ));
            }
            if *open_blockers != analysis_basis.decision_summary.open_blockers {
                return Err(XtaskError::InvalidInput(
                    "family-scoped recommendation open_blockers must match the analysis basis"
                        .to_string(),
                ));
            }
            if *basis_missing_or_stale_evidence != missing_or_stale_evidence(&analysis_basis) {
                return Err(XtaskError::InvalidInput(
                    "family-scoped recommendation missing_or_stale_evidence must match the analysis basis".to_string(),
                ));
            }
        } else if self.analysis_basis_path.is_some()
            || self.analysis_basis_sha256.is_some()
            || self.decision_status.is_some()
            || self.open_blockers.is_some()
            || self.missing_or_stale_evidence.is_some()
        {
            return Err(XtaskError::InvalidInput(
                "global recommendation artifacts must not include M33 analysis-basis fields"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl FamilyCoverageArtifact {
    pub(crate) fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if self.schema_version != COVERAGE_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "coverage schema_version must be {COVERAGE_SCHEMA_VERSION}, found {}",
                self.schema_version
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::FamilyCoverageSnapshot {
            return Err(XtaskError::InvalidInput(
                "coverage artifact_kind must be `family_coverage_snapshot`".to_string(),
            ));
        }
        if !looks_like_utc_timestamp(&self.generated_at) {
            return Err(XtaskError::InvalidInput(
                "coverage generated_at must be a UTC RFC3339 timestamp".to_string(),
            ));
        }
        validate_sha_bound_path(
            workspace_root,
            &self.inventory_path,
            FAMILY_PROMOTION_INVENTORY_DIR,
            &self.inventory_sha256,
            "coverage inventory_path",
        )?;
        validate_sha_bound_path(
            workspace_root,
            &self.corpus_manifest_path,
            M27_CORPUS_MANIFEST_PATH,
            &self.corpus_manifest_sha256,
            "coverage corpus_manifest_path",
        )?;
        if self.sources.is_empty() {
            return Err(XtaskError::InvalidInput(
                "coverage sources[] must not be empty".to_string(),
            ));
        }
        for source in &self.sources {
            source.validate(workspace_root)?;
        }
        for entry in &self.family_coverage {
            entry.validate()?;
        }
        for cluster in &self.unsupported_clusters {
            cluster.validate()?;
        }
        Ok(())
    }
}

impl FamilyRecommendationAnalysisArtifact {
    pub(crate) fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if self.schema_version != RECOMMENDATION_ANALYSIS_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation analysis schema_version must be {RECOMMENDATION_ANALYSIS_SCHEMA_VERSION}, found {}",
                self.schema_version
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::FamilyRecommendationAnalysis {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis artifact_kind must be `family_recommendation_analysis`"
                    .to_string(),
            ));
        }
        if !looks_like_utc_timestamp(&self.generated_at) {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis generated_at must be a UTC RFC3339 timestamp".to_string(),
            ));
        }
        validate_sha_bound_path(
            workspace_root,
            &self.coverage_path,
            FAMILY_COVERAGE_LATEST_PATH,
            &self.coverage_sha256,
            "recommendation analysis coverage_path",
        )?;
        for candidate in &self.ranked_candidates {
            candidate.validate()?;
        }
        self.decision_summary.validate()?;
        self.evidence_summary.validate()?;
        self.delta_from_previous.validate()?;
        let any_ready = self
            .ranked_candidates
            .iter()
            .any(|candidate| candidate.promotion_readiness == PromotionReadiness::Ready);
        let all_hold = self
            .ranked_candidates
            .iter()
            .all(|candidate| candidate.promotion_readiness == PromotionReadiness::Hold);
        let all_zero_real = self
            .ranked_candidates
            .iter()
            .all(|candidate| candidate.leverage.real_example_hits == 0);
        let any_positive_real = self
            .ranked_candidates
            .iter()
            .any(|candidate| candidate.leverage.real_example_hits > 0);
        match self.recommendation_status {
            RecommendationStatus::Ranked => {
                if self.ranked_candidates.is_empty() {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis ranked status must include ranked_candidates[]"
                            .to_string(),
                    ));
                }
                if !candidate_qualifies_for_ranked_status(&self.ranked_candidates[0]) {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis ranked status requires the first candidate to be `ready` with confidence `medium` or `high`".to_string(),
                    ));
                }
                if self
                    .ranked_candidates
                    .iter()
                    .any(|candidate| candidate.next_step_status == NextStepStatus::DurableHold)
                {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis ranked status must not include `durable_hold` candidates".to_string(),
                    ));
                }
            }
            RecommendationStatus::InsufficientRealCorpus => {
                if any_ready {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis non-ranked statuses must not include `ready` candidates"
                            .to_string(),
                    ));
                }
                if !(self.ranked_candidates.is_empty() || (all_hold && all_zero_real)) {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis insufficient_real_corpus status requires either no candidates or only held candidates with zero real_example_hits"
                            .to_string(),
                    ));
                }
            }
            RecommendationStatus::NoStrongCandidate => {
                if any_ready {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis non-ranked statuses must not include `ready` candidates"
                            .to_string(),
                    ));
                }
                if self.ranked_candidates.is_empty() || !all_hold || !any_positive_real {
                    return Err(XtaskError::InvalidInput(
                        "recommendation analysis no_strong_candidate status requires non-empty held candidates with at least one real_example_hits > 0"
                            .to_string(),
                    ));
                }
            }
        }
        let expected_open_blockers = expected_open_blockers(self.ranked_candidates.first());
        if self.decision_summary.open_blockers != expected_open_blockers {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis decision_summary.open_blockers must match the first candidate hold reasons".to_string(),
            ));
        }
        let expected_missing_evidence = expected_missing_evidence(self.ranked_candidates.first());
        if self.evidence_summary.missing_evidence != expected_missing_evidence {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis evidence_summary.missing_evidence must match the first candidate evidence gaps".to_string(),
            ));
        }
        if !self.evidence_summary.stale_evidence.is_empty()
            && !self
                .evidence_summary
                .stale_evidence
                .iter()
                .all(|state| *state == EvidenceState::StaleEvidence)
        {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis evidence_summary.stale_evidence may only contain `stale_evidence` entries".to_string(),
            ));
        }
        let expected_warnings = expected_warnings(self.ranked_candidates.first());
        if self.decision_summary.warnings != expected_warnings
            || self.evidence_summary.warnings != expected_warnings
        {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis warnings must match the derived regression warning set"
                    .to_string(),
            ));
        }
        let expected_decision_status = expected_decision_status(
            self.recommendation_status,
            self.ranked_candidates.first(),
            &self.evidence_summary,
        );
        if self.decision_summary.decision_status != expected_decision_status {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis decision_summary.decision_status does not match the ranked candidate and evidence state".to_string(),
            ));
        }
        if self.decision_summary.top_candidate_id
            != self
                .ranked_candidates
                .first()
                .map(|candidate| candidate.candidate_id.clone())
        {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis decision_summary.top_candidate_id must match the first ranked candidate".to_string(),
            ));
        }
        Ok(())
    }
}

impl CorpusProgramDecisionArtifact {
    pub(crate) fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if self.schema_version != CORPUS_PROGRAM_DECISION_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "corpus-program decision schema_version must be {CORPUS_PROGRAM_DECISION_SCHEMA_VERSION}, found {}",
                self.schema_version
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::CorpusProgramDecision {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision artifact_kind must be `corpus_program_decision`"
                    .to_string(),
            ));
        }
        if !looks_like_utc_timestamp(&self.generated_at) {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision generated_at must be a UTC RFC3339 timestamp".to_string(),
            ));
        }
        if self.summary.trim().is_empty() || self.summary.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision summary must be a single non-empty line".to_string(),
            ));
        }

        let analysis_basis = validate_analysis_basis_reference(
            workspace_root,
            &self.analysis_basis_path,
            &self.analysis_basis_sha256,
            "corpus-program decision",
        )?;
        let expected_snapshot = corpus_program_basis_snapshot(&analysis_basis);
        if self.basis_snapshot != expected_snapshot {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision basis_snapshot must exactly match the validated analysis basis".to_string(),
            ));
        }

        let basis_requires_helper_surface_follow_on =
            basis_snapshot_requires_helper_surface_follow_on(&self.basis_snapshot);
        let artifact_uses_helper_surface_follow_on =
            decision_uses_helper_surface_follow_on_tuple(self);
        if basis_requires_helper_surface_follow_on != artifact_uses_helper_surface_follow_on {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision helper-surface follow-on tuple must stay aligned with the validated analysis basis".to_string(),
            ));
        }
        if artifact_uses_helper_surface_follow_on
            && !decision_matches_helper_surface_follow_on_tuple(self)
        {
            return Err(XtaskError::InvalidInput(
                "corpus-program decision helper-surface follow-on tuple must use the frozen action, basis code, pivot target, and next action".to_string(),
            ));
        }

        Ok(())
    }
}

impl PromotionExecutionArtifact {
    fn validate(
        &self,
        workspace_root: &Path,
        path_family: &str,
        path_run_id: &str,
    ) -> Result<(), XtaskError> {
        if self.schema_version != PROMOTION_EXECUTION_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "promotion execution schema_version must be {PROMOTION_EXECUTION_SCHEMA_VERSION}, found {}",
                self.schema_version
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::PromotionExecution {
            return Err(XtaskError::InvalidInput(
                "promotion execution artifact_kind must be `promotion_execution`".to_string(),
            ));
        }
        validate_family_and_run_id(&self.family, &self.run_id, path_family, path_run_id)?;
        validate_target_language_matches_family(
            &self.family,
            self.target_language,
            "promotion execution target_language",
        )?;
        let analysis_basis = validate_analysis_basis_reference(
            workspace_root,
            &self.analysis_basis_path,
            &self.analysis_basis_sha256,
            "promotion execution",
        )?;
        if self.decision_status_at_start != analysis_basis.decision_summary.decision_status {
            return Err(XtaskError::InvalidInput(
                "promotion execution decision_status_at_start must match the analysis basis"
                    .to_string(),
            ));
        }
        if self.open_blockers_at_start != analysis_basis.decision_summary.open_blockers {
            return Err(XtaskError::InvalidInput(
                "promotion execution open_blockers_at_start must match the analysis basis"
                    .to_string(),
            ));
        }
        if self.missing_or_stale_evidence_at_start != missing_or_stale_evidence(&analysis_basis) {
            return Err(XtaskError::InvalidInput(
                "promotion execution missing_or_stale_evidence_at_start must match the analysis basis".to_string(),
            ));
        }

        let recommendation_path = validate_existing_repo_relative_path(
            workspace_root,
            &self.recommendation_path,
            "promotion execution recommendation_path",
        )?;
        if !recommendation_path.ends_with("recommendation.latest.json") {
            return Err(XtaskError::InvalidInput(format!(
                "promotion execution recommendation_path `{}` must reference `recommendation.latest.json`",
                self.recommendation_path
            )));
        }

        if self.approvals.target_family.status != ApprovalStatus::Approved {
            return Err(XtaskError::InvalidInput(
                "promotion execution approvals.target_family.status must be `approved`".to_string(),
            ));
        }
        if matches!(self.approvals.final_output.status, ApprovalStatus::Pending) {
        } else if !matches!(
            self.approvals.final_output.status,
            ApprovalStatus::Approved | ApprovalStatus::Rejected
        ) {
            return Err(XtaskError::InvalidInput(
                "promotion execution approvals.final_output.status must be `pending`, `approved`, or `rejected`".to_string(),
            ));
        }

        ensure_repo_relative_paths_sorted(
            &self.files_changed,
            "promotion execution files_changed",
        )?;
        for file_path in &self.files_changed {
            validate_repo_relative_path(file_path, "promotion execution files_changed entry")?;
        }

        if self.commands.is_empty() {
            return Err(XtaskError::InvalidInput(
                "promotion execution must record at least one command".to_string(),
            ));
        }
        for command in &self.commands {
            if command.step.trim().is_empty()
                || command.command.trim().is_empty()
                || !looks_like_utc_timestamp(&command.started_at)
                || !looks_like_utc_timestamp(&command.finished_at)
            {
                return Err(XtaskError::InvalidInput(
                    "promotion execution commands[] must include non-empty step/command and UTC timestamps".to_string(),
                ));
            }
            if let Some(artifact_path) = &command.artifact_path {
                validate_existing_repo_relative_path(
                    workspace_root,
                    artifact_path,
                    "promotion execution command artifact_path",
                )?;
            }
        }

        if self.referenced_proof_artifacts.is_empty() {
            return Err(XtaskError::InvalidInput(
                "promotion execution must reference proof artifacts".to_string(),
            ));
        }
        let mut has_prove_latest = false;
        let mut has_attempt_artifact = false;
        let mut has_certification = false;
        for proof_path in &self.referenced_proof_artifacts {
            validate_existing_repo_relative_path(
                workspace_root,
                proof_path,
                "promotion execution referenced_proof_artifacts entry",
            )?;
            has_prove_latest |= proof_path.ends_with("prove.latest.json");
            has_attempt_artifact |=
                proof_path.contains("/attempt-") && proof_path.ends_with(".json");
            has_certification |= proof_path.ends_with("certification.report.json");
        }
        if !(has_prove_latest && has_attempt_artifact && has_certification) {
            return Err(XtaskError::InvalidInput(
                "promotion execution referenced_proof_artifacts must include prove.latest.json, at least one attempt-*.json, and certification.report.json".to_string(),
            ));
        }

        for note in &self.notes {
            if note.trim().is_empty() || note.contains('\n') {
                return Err(XtaskError::InvalidInput(
                    "promotion execution notes[] must contain single-line factual notes"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl PromotionBlockerArtifact {
    fn validate(
        &self,
        workspace_root: &Path,
        path_family: &str,
        path_run_id: &str,
    ) -> Result<(), XtaskError> {
        if self.schema_version != PROMOTION_BLOCKER_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "blocker report schema_version must be {PROMOTION_BLOCKER_SCHEMA_VERSION}, found {}",
                self.schema_version
            )));
        }
        if self.artifact_kind != PromotionArtifactKind::PromotionBlocker {
            return Err(XtaskError::InvalidInput(
                "blocker report artifact_kind must be `promotion_blocker`".to_string(),
            ));
        }
        validate_family_and_run_id(&self.family, &self.run_id, path_family, path_run_id)?;
        validate_target_language_matches_family(
            &self.family,
            self.target_language,
            "blocker report target_language",
        )?;
        let analysis_basis = validate_analysis_basis_reference(
            workspace_root,
            &self.analysis_basis_path,
            &self.analysis_basis_sha256,
            "blocker report",
        )?;
        if self.decision_status_at_start != analysis_basis.decision_summary.decision_status {
            return Err(XtaskError::InvalidInput(
                "blocker report decision_status_at_start must match the analysis basis".to_string(),
            ));
        }
        if self.open_blockers_at_start != analysis_basis.decision_summary.open_blockers {
            return Err(XtaskError::InvalidInput(
                "blocker report open_blockers_at_start must match the analysis basis".to_string(),
            ));
        }
        if self.missing_or_stale_evidence_at_start != missing_or_stale_evidence(&analysis_basis) {
            return Err(XtaskError::InvalidInput(
                "blocker report missing_or_stale_evidence_at_start must match the analysis basis"
                    .to_string(),
            ));
        }
        if self.summary.trim().is_empty() || self.summary.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "blocker report summary must be a single non-empty line".to_string(),
            ));
        }
        if self.machine_evidence.is_empty() {
            return Err(XtaskError::InvalidInput(
                "blocker report machine_evidence[] must not be empty".to_string(),
            ));
        }
        for evidence in &self.machine_evidence {
            evidence.validate(workspace_root)?;
        }
        if self.required_human_action.trim().is_empty() || self.required_human_action.contains('\n')
        {
            return Err(XtaskError::InvalidInput(
                "blocker report required_human_action must be a single non-empty line".to_string(),
            ));
        }
        if self.safe_next_actions.is_empty() {
            return Err(XtaskError::InvalidInput(
                "blocker report safe_next_actions[] must not be empty".to_string(),
            ));
        }
        for action in &self.safe_next_actions {
            if action.trim().is_empty() || action.contains('\n') {
                return Err(XtaskError::InvalidInput(
                    "blocker report safe_next_actions[] must contain single-line entries"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl DecisionSummary {
    fn validate(&self) -> Result<(), XtaskError> {
        for warning in &self.warnings {
            if *warning != DecisionReason::RegressionWarning {
                return Err(XtaskError::InvalidInput(
                    "decision_summary.warnings may only contain `regression_warning`".to_string(),
                ));
            }
        }
        if self.summary.trim().is_empty() || self.summary.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "decision_summary.summary must be a single non-empty line".to_string(),
            ));
        }
        Ok(())
    }
}

impl EvidenceSummary {
    fn validate(&self) -> Result<(), XtaskError> {
        for warning in &self.warnings {
            if *warning != DecisionReason::RegressionWarning {
                return Err(XtaskError::InvalidInput(
                    "evidence_summary.warnings may only contain `regression_warning`".to_string(),
                ));
            }
        }
        if self.summary.trim().is_empty() || self.summary.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "evidence_summary.summary must be a single non-empty line".to_string(),
            ));
        }
        Ok(())
    }
}

impl RecommendationDelta {
    pub(crate) fn no_previous_artifact() -> Self {
        Self {
            previous_generated_at: None,
            previous_decision_status: None,
            previous_recommendation_status: None,
            decision_changed: false,
            top_candidate_changed: false,
            reasons_added: Vec::new(),
            reasons_cleared: Vec::new(),
            evidence_changes: Vec::new(),
            summary: "No previous validated analysis artifact existed at this path.".to_string(),
        }
    }

    pub(crate) fn normalized_placeholder() -> Self {
        Self {
            previous_generated_at: None,
            previous_decision_status: None,
            previous_recommendation_status: None,
            decision_changed: false,
            top_candidate_changed: false,
            reasons_added: Vec::new(),
            reasons_cleared: Vec::new(),
            evidence_changes: Vec::new(),
            summary: String::new(),
        }
    }

    fn validate(&self) -> Result<(), XtaskError> {
        if let Some(previous_generated_at) = &self.previous_generated_at
            && !looks_like_utc_timestamp(previous_generated_at)
        {
            return Err(XtaskError::InvalidInput(
                "delta_from_previous.previous_generated_at must be a UTC RFC3339 timestamp when present".to_string(),
            ));
        }
        if self.summary.trim().is_empty() || self.summary.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "delta_from_previous.summary must be a single non-empty line".to_string(),
            ));
        }
        for change in &self.evidence_changes {
            if change.trim().is_empty() || change.contains('\n') {
                return Err(XtaskError::InvalidInput(
                    "delta_from_previous.evidence_changes[] must contain single-line entries"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl MachineEvidence {
    fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if !looks_like_utc_timestamp(&self.observed_at) {
            return Err(XtaskError::InvalidInput(
                "blocker report machine_evidence[].observed_at must be a UTC RFC3339 timestamp"
                    .to_string(),
            ));
        }
        if self.note.trim().is_empty() || self.note.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "blocker report machine_evidence[].note must be a single non-empty line"
                    .to_string(),
            ));
        }

        match self.kind {
            MachineEvidenceKind::Command => {
                if self.command.as_deref().unwrap_or("").trim().is_empty()
                    || self.exit_code.is_none()
                {
                    return Err(XtaskError::InvalidInput(
                        "command machine_evidence entries must include command and exit_code"
                            .to_string(),
                    ));
                }
            }
            MachineEvidenceKind::Artifact | MachineEvidenceKind::Diff => {
                let path = self.path.as_deref().ok_or_else(|| {
                    XtaskError::InvalidInput(
                        "artifact and diff machine_evidence entries must include path".to_string(),
                    )
                })?;
                validate_existing_repo_relative_path(
                    workspace_root,
                    path,
                    "blocker report machine_evidence path",
                )?;
            }
        }

        Ok(())
    }
}

impl CorpusSourceEntry {
    fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if self.id.trim().is_empty() || self.note.trim().is_empty() || self.note.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "coverage sources[] entries require non-empty single-line id and note".to_string(),
            ));
        }
        validate_existing_repo_relative_path(workspace_root, &self.path, "coverage source path")?;
        if self.unit_count == 0 {
            return Err(XtaskError::InvalidInput(format!(
                "coverage source `{}` must report at least one unit",
                self.id
            )));
        }
        Ok(())
    }
}

impl FamilyCoverageEntry {
    fn validate(&self) -> Result<(), XtaskError> {
        FamilyId::parse(&self.family)?;
        if self.unit_count == 0 || self.unit_ids.is_empty() || self.source_ids.is_empty() {
            return Err(XtaskError::InvalidInput(format!(
                "family coverage entry `{}` must include unit_count, unit_ids, and source_ids",
                self.family
            )));
        }
        Ok(())
    }
}

impl UnsupportedClusterEntry {
    fn validate(&self) -> Result<(), XtaskError> {
        if self.cluster_id.trim().is_empty()
            || self.shape_fingerprint.trim().is_empty()
            || self.representative_unit_ids.is_empty()
            || self.source_ids.is_empty()
        {
            return Err(XtaskError::InvalidInput(
                "unsupported cluster entries must include cluster_id, shape_fingerprint, representative_unit_ids, and source_ids".to_string(),
            ));
        }
        validate_overlap_family(&self.overlap_family)
    }
}

impl RecommendationCandidateEntry {
    fn validate(&self) -> Result<(), XtaskError> {
        if self.candidate_id.trim().is_empty()
            || self.cluster_ids.is_empty()
            || self.rationale.trim().is_empty()
            || self.rationale.contains('\n')
        {
            return Err(XtaskError::InvalidInput(
                "recommendation candidates require non-empty candidate_id, cluster_ids, and single-line rationale".to_string(),
            ));
        }
        match self.promotion_readiness {
            PromotionReadiness::Ready if !self.hold_reasons.is_empty() => {
                return Err(XtaskError::InvalidInput(
                    "recommendation candidates marked `ready` must have empty hold_reasons"
                        .to_string(),
                ));
            }
            PromotionReadiness::Hold if self.hold_reasons.is_empty() => {
                return Err(XtaskError::InvalidInput(
                    "recommendation candidates marked `hold` must include at least one hold_reasons entry"
                        .to_string(),
                ));
            }
            _ => {}
        }
        match self.promotion_readiness {
            PromotionReadiness::Ready => {
                if self.next_step_status != NextStepStatus::Promote
                    || self.next_step_detail != NextStepDetail::ReadyForPromotion
                {
                    return Err(XtaskError::InvalidInput(
                        "recommendation candidates marked `ready` must use next_step_status `promote` and next_step_detail `ready_for_promotion`".to_string(),
                    ));
                }
            }
            PromotionReadiness::Hold => {
                if self.next_step_status == NextStepStatus::Promote
                    || self.next_step_detail == NextStepDetail::ReadyForPromotion
                {
                    return Err(XtaskError::InvalidInput(
                        "recommendation candidates marked `hold` must not use ready-for-promotion next-step fields".to_string(),
                    ));
                }
            }
        }
        if recommendation_uses_helper_surface_durable_hold_tuple(self)
            && !recommendation_matches_helper_surface_durable_hold_tuple(self)
        {
            return Err(XtaskError::InvalidInput(
                "recommendation candidates using the helper-surface durable-hold tuple must keep `promotion_readiness`, `hold_reasons`, `next_step_status`, and `next_step_detail` aligned to the frozen helper-surface contract".to_string(),
            ));
        }
        validate_overlap_family(&self.overlap_family)?;
        self.difficulty.validate()?;
        self.confidence.validate()?;
        Ok(())
    }
}

impl RecommendationDifficulty {
    fn validate(&self) -> Result<(), XtaskError> {
        if self.why.trim().is_empty() || self.why.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "recommendation difficulty.why must be a single non-empty line".to_string(),
            ));
        }
        Ok(())
    }
}

impl RecommendationConfidence {
    fn validate(&self) -> Result<(), XtaskError> {
        if self.why.trim().is_empty() || self.why.contains('\n') {
            return Err(XtaskError::InvalidInput(
                "recommendation confidence.why must be a single non-empty line".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArtifactPath {
    SemanticFamilyReport,
    Recommendation,
    RecommendationByFamily { family: String },
    Coverage,
    RecommendationAnalysis,
    CorpusProgramDecision,
    PromotionExecution { family: String, run_id: String },
    PromotionBlocker { family: String, run_id: String },
}

fn classify_artifact_path(path: &Path) -> Result<ArtifactPath, XtaskError> {
    let recommendation_path =
        Path::new(FAMILY_PROMOTION_ARTIFACT_ROOT).join("recommendation.latest.json");
    if path == recommendation_path {
        return Ok(ArtifactPath::Recommendation);
    }
    if path == Path::new(FAMILY_COVERAGE_LATEST_PATH) {
        return Ok(ArtifactPath::Coverage);
    }
    if path == Path::new(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH) {
        return Ok(ArtifactPath::RecommendationAnalysis);
    }
    if path == Path::new(FAMILY_CORPUS_PROGRAM_DECISION_LATEST_PATH) {
        return Ok(ArtifactPath::CorpusProgramDecision);
    }

    let components = path
        .components()
        .map(component_as_str)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            XtaskError::InvalidInput(format!(
                "artifact path `{}` must contain only normal UTF-8 path components",
                path.display()
            ))
        })?;

    if components.len() == 4
        && components[0] == ".semantic-family-artifacts"
        && components[1] == "semantic-families"
    {
        FamilyId::parse(components[2])?;
        match components[3] {
            "prove.latest.json" | "certification.report.json" => {
                return Ok(ArtifactPath::SemanticFamilyReport);
            }
            file_name if file_name.starts_with("attempt-") && file_name.ends_with(".json") => {
                return Ok(ArtifactPath::SemanticFamilyReport);
            }
            _ => {}
        }
    }

    if components.len() == 4
        && components[0] == ".semantic-family-artifacts"
        && components[1] == "family-promotion"
        && components[3] == "recommendation.latest.json"
    {
        return Ok(ArtifactPath::RecommendationByFamily {
            family: components[2].to_string(),
        });
    }

    if components.len() != 5
        || components[0] != ".semantic-family-artifacts"
        || components[1] != "family-promotion"
    {
        return Err(XtaskError::InvalidInput(format!(
            "artifact path `{}` is not a supported promotion artifact path",
            path.display()
        )));
    }

    let family = components[2].to_string();
    let run_id = components[3].to_string();
    match components[4] {
        "promotion.execution.json" => Ok(ArtifactPath::PromotionExecution { family, run_id }),
        "blocker.report.json" => Ok(ArtifactPath::PromotionBlocker { family, run_id }),
        _ => Err(XtaskError::InvalidInput(format!(
            "artifact path `{}` is not a supported promotion artifact path",
            path.display()
        ))),
    }
}

fn component_as_str(component: Component<'_>) -> Option<&str> {
    match component {
        Component::Normal(value) => value.to_str(),
        _ => None,
    }
}

fn resolve_requested_path(workspace_root: &Path, raw_path: &str) -> PathBuf {
    let requested = PathBuf::from(raw_path);
    if requested.is_absolute() {
        requested
    } else {
        workspace_root.join(requested)
    }
}

fn validate_family_and_run_id(
    family: &str,
    run_id: &str,
    path_family: &str,
    path_run_id: &str,
) -> Result<(), XtaskError> {
    FamilyId::parse(family)?;
    if family != path_family {
        return Err(XtaskError::InvalidInput(format!(
            "artifact family `{family}` does not match artifact path family `{path_family}`"
        )));
    }
    if run_id != path_run_id {
        return Err(XtaskError::InvalidInput(format!(
            "artifact run_id `{run_id}` does not match artifact path run id `{path_run_id}`"
        )));
    }
    if !looks_like_run_id(run_id, family) {
        return Err(XtaskError::InvalidInput(format!(
            "artifact run_id `{run_id}` must match `{{UTC-basic-timestamp}}-{family}`"
        )));
    }
    Ok(())
}

fn validate_sha_bound_path(
    workspace_root: &Path,
    raw_path: &str,
    expected_prefix: &str,
    expected_sha: &str,
    field: &str,
) -> Result<(), XtaskError> {
    let path = validate_existing_repo_relative_path(workspace_root, raw_path, field)?;
    if raw_path != expected_prefix && !raw_path.starts_with(expected_prefix) {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{raw_path}` must stay under or equal `{expected_prefix}`"
        )));
    }
    let bytes = fs::read(&path).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to read `{}`: {error}", path.display()))
    })?;
    let observed_sha = inventory_sha256_hex(&bytes);
    if observed_sha != expected_sha {
        return Err(XtaskError::InvalidInput(format!(
            "{field} sha `{expected_sha}` does not match the exact bytes at `{raw_path}` (expected `{observed_sha}`)"
        )));
    }
    Ok(())
}

fn load_analysis_basis_artifact(
    workspace_root: &Path,
) -> Result<(FamilyRecommendationAnalysisArtifact, String), XtaskError> {
    let path = workspace_root.join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);
    let bytes = fs::read(&path).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to read analysis basis `{}`: {error}",
            path.display()
        ))
    })?;
    let artifact: FamilyRecommendationAnalysisArtifact =
        serde_json::from_slice(&bytes).map_err(deserialize_error(&path))?;
    artifact.validate(workspace_root)?;
    Ok((artifact, inventory_sha256_hex(&bytes)))
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

fn validate_analysis_basis_reference(
    workspace_root: &Path,
    analysis_basis_path: &str,
    analysis_basis_sha256: &str,
    field_prefix: &str,
) -> Result<FamilyRecommendationAnalysisArtifact, XtaskError> {
    validate_sha_bound_path(
        workspace_root,
        analysis_basis_path,
        FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
        analysis_basis_sha256,
        &format!("{field_prefix} analysis_basis_path"),
    )?;
    let path = workspace_root.join(analysis_basis_path);
    let bytes = fs::read(&path).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to read analysis basis `{}`: {error}",
            path.display()
        ))
    })?;
    let artifact: FamilyRecommendationAnalysisArtifact =
        serde_json::from_slice(&bytes).map_err(deserialize_error(&path))?;
    artifact.validate(workspace_root)?;
    Ok(artifact)
}

fn expected_open_blockers(candidate: Option<&RecommendationCandidateEntry>) -> Vec<DecisionReason> {
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

fn expected_missing_evidence(
    candidate: Option<&RecommendationCandidateEntry>,
) -> Vec<EvidenceState> {
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

fn expected_warnings(candidate: Option<&RecommendationCandidateEntry>) -> Vec<DecisionReason> {
    if candidate.is_some_and(|candidate| candidate.leverage.promotion_relevant_regression_hits > 0)
    {
        vec![DecisionReason::RegressionWarning]
    } else {
        Vec::new()
    }
}

fn expected_decision_status(
    recommendation_status: RecommendationStatus,
    candidate: Option<&RecommendationCandidateEntry>,
    evidence_summary: &EvidenceSummary,
) -> DecisionStatus {
    let has_evidence_gaps = !evidence_summary.missing_evidence.is_empty()
        || !evidence_summary.stale_evidence.is_empty();
    match candidate {
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

fn missing_or_stale_evidence(
    artifact: &FamilyRecommendationAnalysisArtifact,
) -> Vec<EvidenceState> {
    let mut combined = artifact.evidence_summary.missing_evidence.clone();
    for stale in &artifact.evidence_summary.stale_evidence {
        if !combined.contains(stale) {
            combined.push(*stale);
        }
    }
    combined
}

fn validate_overlap_family(value: &str) -> Result<(), XtaskError> {
    match value {
        "function.arithmetic_leaf.monotone_*" | "function.wrapper.pipeline*" | "unknown" => Ok(()),
        _ => Err(XtaskError::InvalidInput(format!(
            "overlap_family `{value}` must be `function.arithmetic_leaf.monotone_*`, `function.wrapper.pipeline*`, or `unknown`"
        ))),
    }
}

fn append_newline(mut bytes: Vec<u8>) -> Vec<u8> {
    bytes.push(b'\n');
    bytes
}

fn format_run_id(generated_at: &str, family: &str) -> Result<String, XtaskError> {
    if !looks_like_utc_timestamp(generated_at) {
        return Err(XtaskError::InvalidInput(format!(
            "generated_at `{generated_at}` must be a UTC RFC3339 timestamp"
        )));
    }
    let basic = generated_at
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>();
    Ok(format!("{basic}-{family}"))
}

fn ranked_candidate_for_family(family: &FamilyId) -> Result<RankedCandidate, XtaskError> {
    match family.as_str() {
        "function.arithmetic_leaf.monotone_up.v1" => Ok(RankedCandidate {
            family: family.as_str().to_string(),
            evidence: vec![
                "examples/ecommerce/units/pricing/apply_tax.unit.spec".to_string(),
                "semantic-families/function.arithmetic_leaf.monotone_up.v1/candidate.md"
                    .to_string(),
                "spec-cli/tests/m14_regressions.rs".to_string(),
            ],
            expected_leverage:
                "Proves one bounded second-language promotion path on an already-supported monotone-up leaf family."
                    .to_string(),
            expected_risks: vec![
                "Read-side truth and promotion artifacts must stay explicit about TypeScript being bounded to this pilot."
                    .to_string(),
            ],
        }),
        "function.wrapper.pipeline.v1" => Ok(RankedCandidate {
            family: family.as_str().to_string(),
            evidence: vec![
                "examples/ecommerce/units/pricing/calculate_total.unit.spec".to_string(),
                "semantic-families/function.wrapper.pipeline.v1/candidate.md".to_string(),
                "spec-cli/tests/m14_regressions.rs".to_string(),
            ],
            expected_leverage:
                "Extends proof pressure from leaf math to a wrapper family without changing the public command surface."
                    .to_string(),
            expected_risks: vec![
                "Routing order and comparator-family expectations must remain stable while packet truth shifts."
                    .to_string(),
            ],
        }),
        _ => Err(XtaskError::InvalidInput(format!(
            "family refresh-promotion-recommendation does not support `{}`",
            family.as_str()
        ))),
    }
}

fn latest_attempt_relative_path(
    workspace_root: &Path,
    prove_dir: &Path,
) -> Result<PathBuf, XtaskError> {
    let absolute_dir = workspace_root.join(prove_dir);
    let mut attempts = fs::read_dir(&absolute_dir)
        .map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to read proof artifact directory `{}`: {error}",
                absolute_dir.display()
            ))
        })?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("attempt-") && name.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    attempts.sort();
    let latest = attempts.pop().ok_or_else(|| {
        XtaskError::InvalidInput(format!(
            "no attempt-*.json artifact exists under `{}`",
            prove_dir.display()
        ))
    })?;
    latest
        .strip_prefix(workspace_root)
        .map(Path::to_path_buf)
        .map_err(|_| {
            XtaskError::InvalidInput(format!(
                "attempt artifact `{}` must stay within the workspace root",
                latest.display()
            ))
        })
}

fn git_diff_name_only(workspace_root: &Path, diff_base: &str) -> Result<Vec<String>, XtaskError> {
    let output = Command::new("git")
        .current_dir(workspace_root)
        .args(["diff", "--name-only", &format!("{diff_base}...HEAD")])
        .output()
        .map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to run `git diff --name-only {diff_base}...HEAD`: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(XtaskError::WriteFailure(format!(
            "`git diff --name-only {diff_base}...HEAD` failed with exit code {}",
            output.status.code().unwrap_or(1)
        )));
    }
    let mut paths = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn command_records_for_execution(
    family: &FamilyId,
    target_language: FamilyTargetLanguage,
    timestamp: &str,
) -> Vec<CommandRecord> {
    let prove_path = format!(
        ".semantic-family-artifacts/semantic-families/{}/prove.latest.json",
        family.packet_dir_name()
    );
    let certification_path = format!(
        ".semantic-family-artifacts/semantic-families/{}/certification.report.json",
        family.packet_dir_name()
    );
    let mut commands = vec![
        CommandRecord {
            step: "rust_prove".to_string(),
            command: format!("cargo xtask family prove {}", family.as_str()),
            exit_code: 0,
            started_at: timestamp.to_string(),
            finished_at: timestamp.to_string(),
            artifact_path: Some(prove_path.clone()),
        },
        CommandRecord {
            step: "rust_certify".to_string(),
            command: format!("cargo xtask family certify {}", family.as_str()),
            exit_code: 0,
            started_at: timestamp.to_string(),
            finished_at: timestamp.to_string(),
            artifact_path: Some(certification_path.clone()),
        },
    ];
    if matches!(target_language, FamilyTargetLanguage::Typescript) {
        commands.push(CommandRecord {
            step: "typescript_prove".to_string(),
            command: format!(
                "cargo xtask family prove {} --target-language typescript",
                family.as_str()
            ),
            exit_code: 0,
            started_at: timestamp.to_string(),
            finished_at: timestamp.to_string(),
            artifact_path: Some(prove_path.clone()),
        });
        commands.push(CommandRecord {
            step: "typescript_certify".to_string(),
            command: format!(
                "cargo xtask family certify {} --target-language typescript",
                family.as_str()
            ),
            exit_code: 0,
            started_at: timestamp.to_string(),
            finished_at: timestamp.to_string(),
            artifact_path: Some(certification_path),
        });
    }
    commands
}

fn validate_target_language_matches_family(
    family: &str,
    target_language: TargetLanguage,
    field: &str,
) -> Result<(), XtaskError> {
    let family_id = FamilyId::parse(family)?;
    validate_target_language(
        &family_id,
        match target_language {
            TargetLanguage::Rust => FamilyTargetLanguage::Rust,
            TargetLanguage::Typescript => FamilyTargetLanguage::Typescript,
        },
        field,
    )
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn ensure_repo_relative_paths_sorted(paths: &[String], field: &str) -> Result<(), XtaskError> {
    let mut sorted = paths.to_vec();
    sorted.sort();
    if paths != sorted {
        return Err(XtaskError::InvalidInput(format!(
            "{field} must be sorted lexicographically"
        )));
    }
    Ok(())
}

fn looks_like_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
        })
}

fn looks_like_run_id(run_id: &str, family: &str) -> bool {
    let Some((timestamp, suffix_family)) = run_id.split_once('-') else {
        return false;
    };
    suffix_family == family
        && timestamp.len() == 16
        && timestamp.as_bytes()[8] == b'T'
        && timestamp.as_bytes()[15] == b'Z'
        && timestamp
            .as_bytes()
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 8 | 15) || byte.is_ascii_digit())
}

fn deserialize_error(path: &Path) -> impl FnOnce(serde_json::Error) -> XtaskError + '_ {
    move |error| {
        XtaskError::InvalidInput(format!(
            "failed to deserialize artifact `{}`: {error}",
            path.display()
        ))
    }
}
