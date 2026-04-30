use crate::XtaskError;
use crate::family::inventory::inventory_sha256_hex;
use crate::family::paths::FamilyId;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

const PROMOTION_ARTIFACT_ROOT: &str = ".semantic-family-artifacts/family-promotion";
const INVENTORY_ROOT: &str = ".semantic-family-artifacts/family-promotion/inventory";
const SCHEMA_VERSION: u64 = 1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromotionArtifactKind {
    FamilyRecommendation,
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
        if self.schema_version != SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation schema_version must be {SCHEMA_VERSION}, found {}",
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
        if !self.inventory_path.starts_with(INVENTORY_ROOT) {
            return Err(XtaskError::InvalidInput(format!(
                "recommendation inventory_path `{}` must stay under `{INVENTORY_ROOT}`",
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

impl PromotionExecutionArtifact {
    fn validate(
        &self,
        workspace_root: &Path,
        path_family: &str,
        path_run_id: &str,
    ) -> Result<(), XtaskError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "promotion execution schema_version must be {SCHEMA_VERSION}, found {}",
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
        if self.schema_version != SCHEMA_VERSION {
            return Err(XtaskError::InvalidInput(format!(
                "blocker report schema_version must be {SCHEMA_VERSION}, found {}",
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArtifactPath {
    Recommendation,
    PromotionExecution { family: String, run_id: String },
    PromotionBlocker { family: String, run_id: String },
}

fn classify_artifact_path(path: &Path) -> Result<ArtifactPath, XtaskError> {
    let recommendation_path = Path::new(PROMOTION_ARTIFACT_ROOT).join("recommendation.latest.json");
    if path == recommendation_path {
        return Ok(ArtifactPath::Recommendation);
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

fn validate_existing_repo_relative_path(
    workspace_root: &Path,
    raw_path: &str,
    field: &str,
) -> Result<PathBuf, XtaskError> {
    let relative = validate_repo_relative_path(raw_path, field)?;
    let absolute = workspace_root.join(&relative);
    if !absolute.exists() {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{raw_path}` does not exist in the workspace"
        )));
    }
    Ok(absolute)
}

fn validate_repo_relative_path(raw_path: &str, field: &str) -> Result<PathBuf, XtaskError> {
    let path = Path::new(raw_path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(XtaskError::InvalidInput(format!(
            "{field} must be a non-empty repo-relative path, found `{raw_path}`"
        )));
    }
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{raw_path}` must contain only normal path components"
        )));
    }
    Ok(path.to_path_buf())
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
