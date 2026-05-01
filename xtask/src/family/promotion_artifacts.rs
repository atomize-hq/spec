use crate::XtaskError;
use crate::family::inventory::inventory_sha256_hex;
use crate::family::paths::{
    FAMILY_COVERAGE_LATEST_PATH, FAMILY_PROMOTION_ARTIFACT_ROOT, FAMILY_PROMOTION_INVENTORY_DIR,
    FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH, FamilyId, M27_CORPUS_MANIFEST_PATH,
    validate_existing_repo_relative_path, validate_repo_relative_path,
};
use serde::{Deserialize, Serialize};
use spec_core::semantic_review::UnsupportedFunctionReasonCode;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub(crate) const RECOMMENDATION_SCHEMA_VERSION: u64 = 1;
pub(crate) const COVERAGE_SCHEMA_VERSION: u64 = 1;
pub(crate) const RECOMMENDATION_ANALYSIS_SCHEMA_VERSION: u64 = 2;
const PROMOTION_EXECUTION_SCHEMA_VERSION: u64 = 1;
const PROMOTION_BLOCKER_SCHEMA_VERSION: u64 = 1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromotionArtifactKind {
    FamilyRecommendation,
    FamilyCoverageSnapshot,
    FamilyRecommendationAnalysis,
    PromotionExecution,
    PromotionBlocker,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TargetLanguage {
    Rust,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockingStep {
    Inventory,
    Scaffold,
    Smoke,
    Prove,
    Certify,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    pub status: ExecutionStatus,
    pub recommendation_path: String,
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
        ArtifactPath::Recommendation => {
            let artifact: FamilyRecommendationArtifact =
                serde_json::from_slice(&bytes).map_err(deserialize_error(&absolute))?;
            artifact.validate(workspace_root)?;
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
    fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
        if self.schema_version != RECOMMENDATION_SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation schema_version must be {RECOMMENDATION_SCHEMA_VERSION}, found {}",
                self.schema_version
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
        if self.target_language != TargetLanguage::Rust {
            return Err(XtaskError::InvalidInput(
                "recommendation target_language must be `rust`".to_string(),
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

        Ok(())
    }
}

impl FamilyCoverageArtifact {
    fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
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
    fn validate(&self, workspace_root: &Path) -> Result<(), XtaskError> {
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
        if self.recommendation_status == RecommendationStatus::Ranked
            && self.ranked_candidates.is_empty()
        {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis ranked status must include ranked_candidates[]"
                    .to_string(),
            ));
        }
        if self.recommendation_status == RecommendationStatus::Ranked
            && self.ranked_candidates[0].promotion_readiness != PromotionReadiness::Ready
        {
            return Err(XtaskError::InvalidInput(
                "recommendation analysis ranked status requires the first candidate to be `ready`"
                    .to_string(),
            ));
        }
        for candidate in &self.ranked_candidates {
            candidate.validate()?;
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
    Recommendation,
    Coverage,
    RecommendationAnalysis,
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

fn validate_overlap_family(value: &str) -> Result<(), XtaskError> {
    match value {
        "function.arithmetic_leaf.monotone_*" | "function.wrapper.pipeline*" | "unknown" => Ok(()),
        _ => Err(XtaskError::InvalidInput(format!(
            "overlap_family `{value}` must be `function.arithmetic_leaf.monotone_*`, `function.wrapper.pipeline*`, or `unknown`"
        ))),
    }
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
