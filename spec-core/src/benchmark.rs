//! Benchmark registry loading and shared projection logic for the Rust V1 contract stack.
//!
//! This module is intentionally read-only with respect to proof truth: it consumes
//! registry/accounting inputs plus already-projected proof status and emits one
//! shared benchmark surface for status, export, and snapshotting.

use crate::category_truth::{
    CategoryQualification, ClaimStatus, ConsumerKind, PositiveCreditEligibility,
    is_first_scope_seam_unit_id, is_seam_category_claim_candidate, qualify_category_claim,
};
use crate::semantic_review::{SemanticReview, SemanticSupportStatus};
use crate::{BenchmarkRegistryInvalidDetails, Result, SpecError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const BENCHMARK_LABELS_SCHEMA_VERSION: u8 = 1;

#[derive(Default)]
struct BenchmarkRegistryMeta {
    benchmark_id: Option<String>,
    case_id: Option<String>,
    carrier_id: Option<String>,
    molecule_id: Option<String>,
    value: Option<String>,
}

fn benchmark_registry_invalid(
    code: impl Into<Box<str>>,
    path: impl Into<Box<str>>,
    message: impl Into<Box<str>>,
    meta: BenchmarkRegistryMeta,
) -> SpecError {
    SpecError::BenchmarkRegistryInvalid(Box::new(BenchmarkRegistryInvalidDetails {
        code: code.into(),
        path: path.into(),
        message: message.into(),
        benchmark_id: meta.benchmark_id.map(Into::into),
        case_id: meta.case_id.map(Into::into),
        carrier_id: meta.carrier_id.map(Into::into),
        molecule_id: meta.molecule_id.map(Into::into),
        value: meta.value.map(Into::into),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkLabelRegistry {
    pub schema_version: u8,
    pub benchmarks: Vec<BenchmarkLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkLabel {
    pub id: String,
    pub kind: BenchmarkKind,
    pub lifecycle: BenchmarkLifecycle,
    pub required_for_v1: bool,
    pub root: String,
    pub generated_root: String,
    pub readability_scope: BenchmarkReadabilityScope,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_molecule_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cases: Vec<BenchmarkCaseLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkCaseLabel {
    pub case_id: String,
    pub carrier_kind: BenchmarkCarrierKind,
    pub carrier_id: String,
    pub classification: BenchmarkClassification,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkKind {
    Positive,
    CompanionNegativeProof,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkLifecycle {
    Active,
    Reserved,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCarrierKind {
    Unit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkClassification {
    Supported,
    Deferred,
    FallbackBacked,
    ExplicitlyOut,
    CompanionNegativeProof,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkReadabilityScope {
    SupportedClosure,
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkPathScope {
    Full,
    Partial,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkAccountingStatus {
    Valid,
    Invalid,
    ReservedMissingCases,
    PartialValid,
    PartialInvalid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkStatus {
    Passing,
    Failing,
    Incomplete,
    Invalid,
    Reserved,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkGateStatus {
    Satisfied,
    Open,
    Reserved,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkReadabilityReviewStatus {
    Current,
    Stale,
    Missing,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkTruthStatus {
    Invalid,
    Failing,
    Stale,
    Incomplete,
    Untested,
    Valid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkCaseTruth {
    pub status: BenchmarkTruthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_review: Option<SemanticReview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_support_status: Option<SemanticSupportStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkMoleculeTruth {
    pub covers: Vec<String>,
    pub status: BenchmarkTruthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkReadabilityReviewInput {
    pub status: BenchmarkReadabilityReviewStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkProjectionRequest {
    pub benchmark_root_exists: bool,
    pub path_scope: BenchmarkPathScope,
    pub root_case_truths: BTreeMap<String, BenchmarkCaseTruth>,
    pub selected_carrier_ids: BTreeSet<String>,
    pub required_molecule_truths: BTreeMap<String, BenchmarkMoleculeTruth>,
    pub readability_review: Option<BenchmarkReadabilityReviewInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkProjection {
    pub benchmark_id: String,
    pub kind: BenchmarkKind,
    pub lifecycle: BenchmarkLifecycle,
    pub required_for_v1: bool,
    pub path_scope: BenchmarkPathScope,
    pub accounting_status: BenchmarkAccountingStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_status: Option<BenchmarkStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_status: Option<BenchmarkGateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readability_review_status: Option<BenchmarkReadabilityReviewStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readability_verdict: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<BenchmarkSummary>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_molecule_proofs: Vec<BenchmarkMoleculeProofProjection>,
    pub cases: Vec<BenchmarkCaseProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readability_generated_files: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkCaseProjection {
    pub case_id: String,
    pub carrier_kind: BenchmarkCarrierKind,
    pub carrier_id: String,
    pub classification: BenchmarkClassification,
    pub status: BenchmarkTruthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_support_status: Option<SemanticSupportStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_qualification: Option<CategoryQualification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_refs: Option<BenchmarkProofRefs>,
    pub counts_as_supported_positive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkProofRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub covering_molecule_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkMoleculeProofProjection {
    pub molecule_id: String,
    pub status: BenchmarkTruthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkSummary {
    pub total_cases: usize,
    pub supported_cases: usize,
    pub supported_valid_cases: usize,
    pub positive_credit_cases: usize,
    pub case_status_counts: BenchmarkTruthStatusCounts,
    pub required_molecule_total: usize,
    pub required_molecule_status_counts: BenchmarkTruthStatusCounts,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unlabeled_loaded_carrier_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkTruthStatusCounts {
    pub invalid: usize,
    pub failing: usize,
    pub stale: usize,
    pub incomplete: usize,
    pub untested: usize,
    pub valid: usize,
}

impl BenchmarkTruthStatusCounts {
    fn bump(&mut self, status: BenchmarkTruthStatus) {
        match status {
            BenchmarkTruthStatus::Invalid => self.invalid += 1,
            BenchmarkTruthStatus::Failing => self.failing += 1,
            BenchmarkTruthStatus::Stale => self.stale += 1,
            BenchmarkTruthStatus::Incomplete => self.incomplete += 1,
            BenchmarkTruthStatus::Untested => self.untested += 1,
            BenchmarkTruthStatus::Valid => self.valid += 1,
        }
    }
}

pub fn load_labels(path: &Path) -> Result<BenchmarkLabelRegistry> {
    let path_string = path.display().to_string();
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(benchmark_registry_invalid(
                "SPEC_BENCHMARK_REGISTRY_MISSING",
                path_string,
                "benchmark registry file is missing",
                BenchmarkRegistryMeta::default(),
            ));
        }
        Err(err) => return Err(err.into()),
    };

    let mut registry: BenchmarkLabelRegistry = serde_json::from_str(&content).map_err(|err| {
        benchmark_registry_invalid(
            "SPEC_BENCHMARK_REGISTRY_MALFORMED",
            path_string.clone(),
            err.to_string(),
            BenchmarkRegistryMeta::default(),
        )
    })?;

    if registry.schema_version != BENCHMARK_LABELS_SCHEMA_VERSION {
        return Err(benchmark_registry_invalid(
            "SPEC_BENCHMARK_REGISTRY_SCHEMA_VERSION",
            path_string,
            format!(
                "unsupported benchmark registry schema_version {}; expected {}",
                registry.schema_version, BENCHMARK_LABELS_SCHEMA_VERSION
            ),
            BenchmarkRegistryMeta {
                value: Some(registry.schema_version.to_string()),
                ..Default::default()
            },
        ));
    }

    validate_registry(path, &registry)?;
    for benchmark in &mut registry.benchmarks {
        benchmark.required_molecule_ids.sort();
        benchmark.required_molecule_ids.dedup();
        benchmark
            .cases
            .sort_by(|left, right| left.case_id.cmp(&right.case_id));
    }
    registry
        .benchmarks
        .sort_by(|left, right| left.id.cmp(&right.id));
    Ok(registry)
}

pub fn project_benchmark(
    benchmark: &BenchmarkLabel,
    request: &BenchmarkProjectionRequest,
) -> BenchmarkProjection {
    let case_labels =
        selected_case_labels(benchmark, request.path_scope, &request.selected_carrier_ids);
    let unlabeled_loaded_carrier_ids = unlabeled_loaded_carrier_ids(benchmark, request);

    let mut required_molecule_proofs = if matches!(request.path_scope, BenchmarkPathScope::Full) {
        benchmark
            .required_molecule_ids
            .iter()
            .map(|molecule_id| {
                let proof = request.required_molecule_truths.get(molecule_id);
                BenchmarkMoleculeProofProjection {
                    molecule_id: molecule_id.clone(),
                    status: proof
                        .map(|truth| truth.status)
                        .unwrap_or(BenchmarkTruthStatus::Untested),
                    reason: proof.and_then(|truth| truth.reason.clone()),
                    proof_ref: proof.map(|_| molecule_evidence_ref(benchmark, molecule_id)),
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    required_molecule_proofs.sort_by(|left, right| left.molecule_id.cmp(&right.molecule_id));

    let mut cases = case_labels
        .iter()
        .map(|label| project_case(benchmark, label, request))
        .collect::<Vec<_>>();
    let accounting_status = determine_accounting_status(
        benchmark,
        request.path_scope,
        &unlabeled_loaded_carrier_ids,
        &cases,
    );
    for case_projection in &mut cases {
        case_projection.counts_as_supported_positive = counts_as_supported_positive(
            benchmark,
            request.path_scope,
            accounting_status,
            case_projection,
        );
    }
    cases.sort_by(|left, right| left.case_id.cmp(&right.case_id));

    let readability_generated_files = matches!(request.path_scope, BenchmarkPathScope::Full)
        .then(|| readability_generated_files(benchmark, &cases));

    let readability_review_status =
        matches!(request.path_scope, BenchmarkPathScope::Full).then(|| {
            if !readability_applies(benchmark) {
                BenchmarkReadabilityReviewStatus::NotApplicable
            } else {
                request
                    .readability_review
                    .as_ref()
                    .map(|review| review.status)
                    .unwrap_or(BenchmarkReadabilityReviewStatus::Missing)
            }
        });

    let readability_verdict = matches!(request.path_scope, BenchmarkPathScope::Full)
        .then(|| {
            request
                .readability_review
                .as_ref()
                .and_then(|review| review.verdict.clone())
        })
        .flatten();

    let summary =
        matches!(request.path_scope, BenchmarkPathScope::Full).then(|| BenchmarkSummary {
            total_cases: cases.len(),
            supported_cases: cases
                .iter()
                .filter(|case_projection| {
                    matches!(
                        case_projection.classification,
                        BenchmarkClassification::Supported
                    )
                })
                .count(),
            supported_valid_cases: cases
                .iter()
                .filter(|case_projection| {
                    matches!(
                        case_projection.classification,
                        BenchmarkClassification::Supported
                    ) && matches!(case_projection.status, BenchmarkTruthStatus::Valid)
                })
                .count(),
            positive_credit_cases: cases
                .iter()
                .filter(|case_projection| case_projection.counts_as_supported_positive)
                .count(),
            case_status_counts: count_case_statuses(&cases),
            required_molecule_total: required_molecule_proofs.len(),
            required_molecule_status_counts: count_required_molecule_statuses(
                &required_molecule_proofs,
            ),
            unlabeled_loaded_carrier_ids: unlabeled_loaded_carrier_ids.clone(),
        });

    let benchmark_status = matches!(request.path_scope, BenchmarkPathScope::Full).then(|| {
        determine_benchmark_status(
            benchmark,
            accounting_status,
            &cases,
            &required_molecule_proofs,
        )
    });

    let gate_status = matches!(request.path_scope, BenchmarkPathScope::Full).then(|| {
        determine_gate_status(
            benchmark,
            benchmark_status.expect("full path scope computes benchmark_status"),
            summary.as_ref().expect("full path scope computes summary"),
        )
    });

    let label_digest = matches!(request.path_scope, BenchmarkPathScope::Full)
        .then(|| compute_label_digest(benchmark));

    let mut projection = BenchmarkProjection {
        benchmark_id: benchmark.id.clone(),
        kind: benchmark.kind,
        lifecycle: benchmark.lifecycle,
        required_for_v1: benchmark.required_for_v1,
        path_scope: request.path_scope,
        accounting_status,
        benchmark_status,
        gate_status,
        readability_review_status,
        readability_verdict,
        label_digest,
        projection_digest: None,
        summary,
        required_molecule_proofs,
        cases,
        readability_generated_files,
    };
    if matches!(request.path_scope, BenchmarkPathScope::Full) {
        projection.projection_digest = Some(compute_projection_digest(&projection));
    }
    projection
}

pub fn benchmark_path_scope(
    command_path: &Path,
    benchmark_root: &Path,
    loads_entire_benchmark_root: bool,
) -> Option<BenchmarkPathScope> {
    if command_path == benchmark_root
        || benchmark_root.starts_with(command_path)
        || command_path.starts_with(benchmark_root)
    {
        return Some(if loads_entire_benchmark_root {
            BenchmarkPathScope::Full
        } else {
            BenchmarkPathScope::Partial
        });
    }
    None
}

pub fn benchmark_root_path(repo_root: &Path, benchmark: &BenchmarkLabel) -> Result<PathBuf> {
    let relative = normalized_relative_path(&benchmark.root).map_err(|message| {
        benchmark_registry_invalid(
            "SPEC_BENCHMARK_ROOT_INVALID",
            benchmark.root.clone(),
            message,
            BenchmarkRegistryMeta {
                benchmark_id: Some(benchmark.id.clone()),
                value: Some(benchmark.root.clone()),
                ..Default::default()
            },
        )
    })?;
    Ok(repo_root.join(relative))
}

pub fn benchmark_labels_path(repo_root: &Path) -> PathBuf {
    repo_root.join("benchmarks").join("labels.json")
}

pub fn readability_review_path(repo_root: &Path, benchmark_id: &str) -> PathBuf {
    repo_root
        .join("benchmarks")
        .join("reviews")
        .join(format!("{benchmark_id}.readability.review.json"))
}

pub fn benchmark_snapshot_path(repo_root: &Path, benchmark_id: &str) -> PathBuf {
    repo_root
        .join("benchmarks")
        .join("snapshots")
        .join(format!("{benchmark_id}.snapshot.json"))
}

pub fn compute_label_digest(benchmark: &BenchmarkLabel) -> String {
    digest_json_value(&BenchmarkLabelDigestPayload::from_label(benchmark))
}

pub fn compute_projection_digest(projection: &BenchmarkProjection) -> String {
    digest_json_value(&BenchmarkProjectionDigestPayload::from_projection(
        projection,
    ))
}

pub fn readability_generated_files(
    benchmark: &BenchmarkLabel,
    cases: &[BenchmarkCaseProjection],
) -> Vec<String> {
    if !readability_applies(benchmark) {
        return Vec::new();
    }

    let mut files = BTreeSet::new();
    for case_projection in cases {
        if !matches!(
            case_projection.classification,
            BenchmarkClassification::Supported
        ) {
            continue;
        }
        files.insert(generated_unit_path(
            &benchmark.generated_root,
            &case_projection.carrier_id,
        ));
        for mod_path in generated_mod_paths(&benchmark.generated_root, &case_projection.carrier_id)
        {
            files.insert(mod_path);
        }
    }
    for molecule_id in &benchmark.required_molecule_ids {
        files.insert(generated_molecule_tests_path(
            &benchmark.generated_root,
            molecule_id,
        ));
        for mod_path in generated_mod_paths(&benchmark.generated_root, molecule_id) {
            files.insert(mod_path);
        }
    }
    files.into_iter().collect()
}

fn validate_registry(path: &Path, registry: &BenchmarkLabelRegistry) -> Result<()> {
    let path_string = path.display().to_string();
    let mut benchmark_ids = BTreeSet::new();

    for benchmark in &registry.benchmarks {
        if !benchmark_ids.insert(benchmark.id.clone()) {
            return Err(benchmark_registry_invalid(
                "SPEC_BENCHMARK_DUPLICATE_ID",
                path_string.clone(),
                format!("duplicate benchmark id '{}'", benchmark.id),
                BenchmarkRegistryMeta {
                    benchmark_id: Some(benchmark.id.clone()),
                    value: Some(benchmark.id.clone()),
                    ..Default::default()
                },
            ));
        }

        match (&benchmark.id[..], benchmark.kind) {
            ("BENCH-ECOM", BenchmarkKind::Positive)
            | ("BENCH-SERVICE", BenchmarkKind::Positive)
            | ("BENCH-CROSSLIB", BenchmarkKind::CompanionNegativeProof) => {}
            ("BENCH-ECOM", other) | ("BENCH-SERVICE", other) => {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_KIND_MISMATCH",
                    path_string.clone(),
                    format!(
                        "benchmark '{}' must use kind 'positive', got '{}'",
                        benchmark.id,
                        serde_json::to_string(&other).unwrap_or_else(|_| "\"unknown\"".to_string())
                    ),
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        ..Default::default()
                    },
                ));
            }
            ("BENCH-CROSSLIB", _) => {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_KIND_MISMATCH",
                    path_string.clone(),
                    "benchmark 'BENCH-CROSSLIB' must use kind 'companion_negative_proof'",
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        ..Default::default()
                    },
                ));
            }
            _ => {}
        }

        if matches!(benchmark.lifecycle, BenchmarkLifecycle::Reserved)
            && !benchmark.cases.is_empty()
        {
            return Err(benchmark_registry_invalid(
                "SPEC_BENCHMARK_RESERVED_HAS_CASES",
                path_string.clone(),
                format!(
                    "reserved benchmark '{}' must not declare cases",
                    benchmark.id
                ),
                BenchmarkRegistryMeta {
                    benchmark_id: Some(benchmark.id.clone()),
                    ..Default::default()
                },
            ));
        }

        let normalized_root = normalized_relative_path(&benchmark.root).map_err(|message| {
            benchmark_registry_invalid(
                "SPEC_BENCHMARK_ROOT_INVALID",
                path_string.clone(),
                message,
                BenchmarkRegistryMeta {
                    benchmark_id: Some(benchmark.id.clone()),
                    value: Some(benchmark.root.clone()),
                    ..Default::default()
                },
            )
        })?;
        let root_string = normalized_root.to_string_lossy().replace('\\', "/");

        let mut case_ids = BTreeSet::new();
        let mut case_carriers = BTreeSet::new();
        for case in &benchmark.cases {
            if !case_ids.insert(case.case_id.clone()) {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_DUPLICATE_CASE_ID",
                    path_string.clone(),
                    format!(
                        "duplicate case id '{}' in benchmark '{}'",
                        case.case_id, benchmark.id
                    ),
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        case_id: Some(case.case_id.clone()),
                        carrier_id: Some(case.carrier_id.clone()),
                        value: Some(case.case_id.clone()),
                        ..Default::default()
                    },
                ));
            }
            if !case_carriers.insert(case.carrier_id.clone()) {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_DUPLICATE_CARRIER",
                    path_string.clone(),
                    format!(
                        "carrier '{}' is duplicated within benchmark '{}'",
                        case.carrier_id, benchmark.id
                    ),
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        case_id: Some(case.case_id.clone()),
                        carrier_id: Some(case.carrier_id.clone()),
                        value: Some(case.carrier_id.clone()),
                        ..Default::default()
                    },
                ));
            }
            if !matches!(case.carrier_kind, BenchmarkCarrierKind::Unit) {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_CARRIER_KIND_UNSUPPORTED",
                    path_string.clone(),
                    "I2 benchmark cases may contain only unit carriers",
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        case_id: Some(case.case_id.clone()),
                        carrier_id: Some(case.carrier_id.clone()),
                        ..Default::default()
                    },
                ));
            }
            if !carrier_is_under_root(&root_string, &case.carrier_id) {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_CASE_OUTSIDE_ROOT",
                    path_string.clone(),
                    format!(
                        "carrier '{}' is outside benchmark root '{}'",
                        case.carrier_id, benchmark.root
                    ),
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        case_id: Some(case.case_id.clone()),
                        carrier_id: Some(case.carrier_id.clone()),
                        value: Some(case.carrier_id.clone()),
                        ..Default::default()
                    },
                ));
            }
        }

        for molecule_id in &benchmark.required_molecule_ids {
            if !carrier_is_under_root(&root_string, molecule_id) {
                return Err(benchmark_registry_invalid(
                    "SPEC_BENCHMARK_REQUIRED_MOLECULE_OUTSIDE_ROOT",
                    path_string.clone(),
                    format!(
                        "required molecule '{}' is outside benchmark root '{}'",
                        molecule_id, benchmark.root
                    ),
                    BenchmarkRegistryMeta {
                        benchmark_id: Some(benchmark.id.clone()),
                        molecule_id: Some(molecule_id.clone()),
                        value: Some(molecule_id.clone()),
                        ..Default::default()
                    },
                ));
            }
        }
    }

    Ok(())
}

fn selected_case_labels<'a>(
    benchmark: &'a BenchmarkLabel,
    path_scope: BenchmarkPathScope,
    selected_carrier_ids: &BTreeSet<String>,
) -> Vec<&'a BenchmarkCaseLabel> {
    benchmark
        .cases
        .iter()
        .filter(|label| match path_scope {
            BenchmarkPathScope::Full => true,
            BenchmarkPathScope::Partial => selected_carrier_ids.contains(&label.carrier_id),
        })
        .collect()
}

fn unlabeled_loaded_carrier_ids(
    benchmark: &BenchmarkLabel,
    request: &BenchmarkProjectionRequest,
) -> Vec<String> {
    if matches!(benchmark.lifecycle, BenchmarkLifecycle::Reserved) {
        return Vec::new();
    }

    let labeled_carriers = benchmark
        .cases
        .iter()
        .map(|case| case.carrier_id.as_str())
        .collect::<BTreeSet<_>>();

    let mut unlabeled = request
        .root_case_truths
        .keys()
        .filter(|carrier_id| !labeled_carriers.contains(carrier_id.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    if matches!(request.path_scope, BenchmarkPathScope::Partial) {
        unlabeled.retain(|carrier_id| request.selected_carrier_ids.contains(carrier_id));
    }

    unlabeled.sort();
    unlabeled
}

fn determine_accounting_status(
    benchmark: &BenchmarkLabel,
    path_scope: BenchmarkPathScope,
    unlabeled_loaded_carrier_ids: &[String],
    cases: &[BenchmarkCaseProjection],
) -> BenchmarkAccountingStatus {
    if matches!(benchmark.lifecycle, BenchmarkLifecycle::Reserved) {
        return BenchmarkAccountingStatus::ReservedMissingCases;
    }
    let has_unqualified_supported_seam_case = cases
        .iter()
        .any(supported_seam_case_requires_qualified_truth);
    if matches!(path_scope, BenchmarkPathScope::Partial) {
        if unlabeled_loaded_carrier_ids.is_empty() && !has_unqualified_supported_seam_case {
            BenchmarkAccountingStatus::PartialValid
        } else {
            BenchmarkAccountingStatus::PartialInvalid
        }
    } else if unlabeled_loaded_carrier_ids.is_empty() && !has_unqualified_supported_seam_case {
        BenchmarkAccountingStatus::Valid
    } else {
        BenchmarkAccountingStatus::Invalid
    }
}

fn project_case(
    benchmark: &BenchmarkLabel,
    label: &BenchmarkCaseLabel,
    request: &BenchmarkProjectionRequest,
) -> BenchmarkCaseProjection {
    let truth = request.root_case_truths.get(&label.carrier_id);
    let status = truth
        .map(|truth| truth.status)
        .unwrap_or(BenchmarkTruthStatus::Invalid);
    let reason = truth.and_then(|truth| truth.reason.clone()).or_else(|| {
        truth.is_none().then(|| {
            format!(
                "carrier '{}' is missing from loaded benchmark root '{}'",
                label.carrier_id, benchmark.root
            )
        })
    });
    let semantic_support_status = truth
        .and_then(|truth| {
            truth
                .semantic_review
                .as_ref()
                .map(SemanticReview::effective_support_status)
        })
        .or_else(|| truth.and_then(|truth| truth.semantic_support_status));
    let category_qualification = project_case_category_qualification(&label.carrier_id, truth);
    let proof_refs = build_case_proof_refs(benchmark, label, request);

    BenchmarkCaseProjection {
        case_id: label.case_id.clone(),
        carrier_kind: label.carrier_kind,
        carrier_id: label.carrier_id.clone(),
        classification: label.classification,
        status,
        reason,
        semantic_support_status,
        category_qualification,
        proof_refs,
        counts_as_supported_positive: false,
    }
}

fn counts_as_supported_positive(
    benchmark: &BenchmarkLabel,
    path_scope: BenchmarkPathScope,
    accounting_status: BenchmarkAccountingStatus,
    case_projection: &BenchmarkCaseProjection,
) -> bool {
    matches!(benchmark.kind, BenchmarkKind::Positive)
        && matches!(benchmark.lifecycle, BenchmarkLifecycle::Active)
        && matches!(path_scope, BenchmarkPathScope::Full)
        && matches!(accounting_status, BenchmarkAccountingStatus::Valid)
        && matches!(
            case_projection.classification,
            BenchmarkClassification::Supported
        )
        && matches!(case_projection.status, BenchmarkTruthStatus::Valid)
        && match &case_projection.category_qualification {
            Some(qualification) => {
                matches!(qualification.claim_status, ClaimStatus::SupportedQualified)
                    && matches!(
                        qualification.positive_credit_eligibility,
                        PositiveCreditEligibility::Eligible
                    )
            }
            None => {
                !is_first_scope_seam_unit_id(&case_projection.carrier_id)
                    && matches!(
                        case_projection.semantic_support_status,
                        Some(SemanticSupportStatus::Supported)
                    )
            }
        }
}

fn supported_seam_case_requires_qualified_truth(case_projection: &BenchmarkCaseProjection) -> bool {
    matches!(
        case_projection.classification,
        BenchmarkClassification::Supported
    ) && if let Some(qualification) = case_projection.category_qualification.as_ref() {
        !matches!(qualification.claim_status, ClaimStatus::SupportedQualified)
            || !matches!(
                qualification.positive_credit_eligibility,
                PositiveCreditEligibility::Eligible
            )
    } else {
        is_first_scope_seam_unit_id(&case_projection.carrier_id)
    }
}

fn project_case_category_qualification(
    carrier_id: &str,
    truth: Option<&BenchmarkCaseTruth>,
) -> Option<CategoryQualification> {
    let semantic_review = truth.and_then(|truth| truth.semantic_review.as_ref());
    if let Some(review) = semantic_review
        && is_seam_category_claim_candidate(review)
    {
        return Some(qualify_category_claim(
            ConsumerKind::Benchmark,
            Some(review),
            Some(carrier_id),
        ));
    }
    None
}

fn build_case_proof_refs(
    benchmark: &BenchmarkLabel,
    label: &BenchmarkCaseLabel,
    request: &BenchmarkProjectionRequest,
) -> Option<BenchmarkProofRefs> {
    if !request.root_case_truths.contains_key(&label.carrier_id) {
        return None;
    }

    let mut covering_molecule_evidence = request
        .required_molecule_truths
        .iter()
        .filter(|(_, proof)| {
            proof
                .covers
                .iter()
                .any(|cover_id| cover_id == &label.carrier_id)
        })
        .filter(|(_, proof)| !matches!(proof.status, BenchmarkTruthStatus::Untested))
        .map(|(molecule_id, _)| molecule_evidence_ref(benchmark, molecule_id))
        .collect::<Vec<_>>();
    covering_molecule_evidence.sort();

    Some(BenchmarkProofRefs {
        passport: Some(passport_ref(benchmark, &label.carrier_id)),
        covering_molecule_evidence,
    })
}

fn determine_benchmark_status(
    benchmark: &BenchmarkLabel,
    accounting_status: BenchmarkAccountingStatus,
    cases: &[BenchmarkCaseProjection],
    required_molecule_proofs: &[BenchmarkMoleculeProofProjection],
) -> BenchmarkStatus {
    if matches!(benchmark.lifecycle, BenchmarkLifecycle::Reserved) {
        return BenchmarkStatus::Reserved;
    }
    if matches!(accounting_status, BenchmarkAccountingStatus::Invalid) {
        return BenchmarkStatus::Invalid;
    }

    if cases.iter().any(|case| {
        matches!(
            case.status,
            BenchmarkTruthStatus::Invalid | BenchmarkTruthStatus::Failing
        )
    }) || required_molecule_proofs.iter().any(|proof| {
        matches!(
            proof.status,
            BenchmarkTruthStatus::Invalid | BenchmarkTruthStatus::Failing
        )
    }) {
        return BenchmarkStatus::Failing;
    }

    if cases.iter().any(|case| {
        matches!(
            case.status,
            BenchmarkTruthStatus::Stale
                | BenchmarkTruthStatus::Incomplete
                | BenchmarkTruthStatus::Untested
        )
    }) || required_molecule_proofs.iter().any(|proof| {
        matches!(
            proof.status,
            BenchmarkTruthStatus::Stale
                | BenchmarkTruthStatus::Incomplete
                | BenchmarkTruthStatus::Untested
        )
    }) {
        return BenchmarkStatus::Incomplete;
    }

    BenchmarkStatus::Passing
}

fn determine_gate_status(
    benchmark: &BenchmarkLabel,
    benchmark_status: BenchmarkStatus,
    summary: &BenchmarkSummary,
) -> BenchmarkGateStatus {
    if matches!(benchmark.lifecycle, BenchmarkLifecycle::Reserved) {
        return BenchmarkGateStatus::Reserved;
    }
    if !benchmark.required_for_v1 || !matches!(benchmark.kind, BenchmarkKind::Positive) {
        return BenchmarkGateStatus::NotApplicable;
    }
    let supported_gate_total = summary.supported_cases;
    if matches!(benchmark_status, BenchmarkStatus::Passing)
        && supported_gate_total > 0
        && summary.positive_credit_cases == supported_gate_total
    {
        BenchmarkGateStatus::Satisfied
    } else {
        BenchmarkGateStatus::Open
    }
}

fn count_case_statuses(cases: &[BenchmarkCaseProjection]) -> BenchmarkTruthStatusCounts {
    let mut counts = BenchmarkTruthStatusCounts::default();
    for case_projection in cases {
        counts.bump(case_projection.status);
    }
    counts
}

fn count_required_molecule_statuses(
    proofs: &[BenchmarkMoleculeProofProjection],
) -> BenchmarkTruthStatusCounts {
    let mut counts = BenchmarkTruthStatusCounts::default();
    for proof in proofs {
        counts.bump(proof.status);
    }
    counts
}

fn passport_ref(benchmark: &BenchmarkLabel, carrier_id: &str) -> String {
    format!("{}/{}.spec.passport.json", benchmark.root, carrier_id)
}

fn molecule_evidence_ref(benchmark: &BenchmarkLabel, molecule_id: &str) -> String {
    format!("{}/{}.test.evidence.json", benchmark.root, molecule_id)
}

fn carrier_is_under_root(_root: &str, carrier_id: &str) -> bool {
    normalized_relative_path(carrier_id).is_ok()
}

fn readability_applies(benchmark: &BenchmarkLabel) -> bool {
    matches!(benchmark.kind, BenchmarkKind::Positive)
        && matches!(
            benchmark.readability_scope,
            BenchmarkReadabilityScope::SupportedClosure
        )
}

fn generated_unit_path(generated_root: &str, carrier_id: &str) -> String {
    format!("{generated_root}/{carrier_id}.rs")
}

fn generated_molecule_tests_path(generated_root: &str, molecule_id: &str) -> String {
    let module_path = molecule_id
        .rsplit_once('/')
        .map(|(module_path, _)| module_path)
        .unwrap_or("");
    if module_path.is_empty() {
        format!("{generated_root}/molecule_tests.rs")
    } else {
        format!("{generated_root}/{module_path}/molecule_tests.rs")
    }
}

fn generated_mod_paths(generated_root: &str, carrier_id: &str) -> Vec<String> {
    let module_path = carrier_id
        .rsplit_once('/')
        .map(|(module_path, _)| module_path)
        .unwrap_or("");
    let mut paths = vec![format!("{generated_root}/mod.rs")];
    if module_path.is_empty() {
        return paths;
    }

    let mut prefix = String::new();
    for segment in module_path.split('/') {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(segment);
        paths.push(format!("{generated_root}/{prefix}/mod.rs"));
    }
    paths
}

fn normalized_relative_path(path: &str) -> std::result::Result<PathBuf, String> {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        return Err("benchmark paths must be repo-relative".to_string());
    }

    let mut normalized = PathBuf::new();
    for component in candidate.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => {
                return Err("benchmark paths must not escape the repo root".to_string());
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err("benchmark paths must be repo-relative".to_string());
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err("benchmark paths must not be empty".to_string());
    }
    Ok(normalized)
}

fn digest_json_value(value: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(value).expect("benchmark digest payload must serialize");
    let hash = Sha256::digest(&bytes);
    format!("sha256:{}", hex::encode(hash))
}

#[derive(Serialize)]
struct BenchmarkLabelDigestPayload<'a> {
    id: &'a str,
    kind: BenchmarkKind,
    lifecycle: BenchmarkLifecycle,
    required_for_v1: bool,
    root: &'a str,
    generated_root: &'a str,
    readability_scope: BenchmarkReadabilityScope,
    required_molecule_ids: Vec<&'a str>,
    cases: Vec<BenchmarkCaseDigestPayload<'a>>,
}

impl<'a> BenchmarkLabelDigestPayload<'a> {
    fn from_label(label: &'a BenchmarkLabel) -> Self {
        let mut required_molecule_ids = label
            .required_molecule_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        required_molecule_ids.sort();

        let mut cases = label
            .cases
            .iter()
            .map(|case| BenchmarkCaseDigestPayload {
                case_id: case.case_id.as_str(),
                carrier_kind: case.carrier_kind,
                carrier_id: case.carrier_id.as_str(),
                classification: case.classification,
            })
            .collect::<Vec<_>>();
        cases.sort_by(|left, right| left.case_id.cmp(right.case_id));

        Self {
            id: &label.id,
            kind: label.kind,
            lifecycle: label.lifecycle,
            required_for_v1: label.required_for_v1,
            root: &label.root,
            generated_root: &label.generated_root,
            readability_scope: label.readability_scope,
            required_molecule_ids,
            cases,
        }
    }
}

#[derive(Serialize)]
struct BenchmarkCaseDigestPayload<'a> {
    case_id: &'a str,
    carrier_kind: BenchmarkCarrierKind,
    carrier_id: &'a str,
    classification: BenchmarkClassification,
}

#[derive(Serialize)]
struct BenchmarkProjectionDigestPayload<'a> {
    benchmark_id: &'a str,
    kind: BenchmarkKind,
    lifecycle: BenchmarkLifecycle,
    required_for_v1: bool,
    path_scope: BenchmarkPathScope,
    accounting_status: BenchmarkAccountingStatus,
    benchmark_status: Option<BenchmarkStatus>,
    gate_status: Option<BenchmarkGateStatus>,
    readability_review_status: Option<BenchmarkReadabilityReviewStatus>,
    label_digest: Option<&'a str>,
    summary: Option<&'a BenchmarkSummary>,
    required_molecule_proofs: &'a [BenchmarkMoleculeProofProjection],
    cases: &'a [BenchmarkCaseProjection],
    readability_generated_files: Option<&'a [String]>,
}

impl<'a> BenchmarkProjectionDigestPayload<'a> {
    fn from_projection(projection: &'a BenchmarkProjection) -> Self {
        Self {
            benchmark_id: &projection.benchmark_id,
            kind: projection.kind,
            lifecycle: projection.lifecycle,
            required_for_v1: projection.required_for_v1,
            path_scope: projection.path_scope,
            accounting_status: projection.accounting_status,
            benchmark_status: projection.benchmark_status,
            gate_status: projection.gate_status,
            readability_review_status: projection.readability_review_status,
            label_digest: projection.label_digest.as_deref(),
            summary: projection.summary.as_ref(),
            required_molecule_proofs: &projection.required_molecule_proofs,
            cases: &projection.cases,
            readability_generated_files: projection.readability_generated_files.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn valid_registry() -> BenchmarkLabelRegistry {
        BenchmarkLabelRegistry {
            schema_version: BENCHMARK_LABELS_SCHEMA_VERSION,
            benchmarks: vec![
                BenchmarkLabel {
                    id: "BENCH-CROSSLIB".to_string(),
                    kind: BenchmarkKind::CompanionNegativeProof,
                    lifecycle: BenchmarkLifecycle::Active,
                    required_for_v1: false,
                    root: "examples/crosslib-app/units".to_string(),
                    generated_root: "examples/crosslib-app/src/generated".to_string(),
                    readability_scope: BenchmarkReadabilityScope::None,
                    required_molecule_ids: vec![],
                    cases: vec![BenchmarkCaseLabel {
                        case_id: "pricing/apply_discount".to_string(),
                        carrier_kind: BenchmarkCarrierKind::Unit,
                        carrier_id: "pricing/apply_discount".to_string(),
                        classification: BenchmarkClassification::CompanionNegativeProof,
                    }],
                },
                BenchmarkLabel {
                    id: "BENCH-ECOM".to_string(),
                    kind: BenchmarkKind::Positive,
                    lifecycle: BenchmarkLifecycle::Active,
                    required_for_v1: true,
                    root: "examples/ecommerce/units".to_string(),
                    generated_root: "examples/ecommerce/src/generated".to_string(),
                    readability_scope: BenchmarkReadabilityScope::SupportedClosure,
                    required_molecule_ids: vec![
                        "pricing/discount_plus_tax".to_string(),
                        "pricing/checkout_flow".to_string(),
                    ],
                    cases: vec![
                        BenchmarkCaseLabel {
                            case_id: "pricing/apply_tax".to_string(),
                            carrier_kind: BenchmarkCarrierKind::Unit,
                            carrier_id: "pricing/apply_tax".to_string(),
                            classification: BenchmarkClassification::Supported,
                        },
                        BenchmarkCaseLabel {
                            case_id: "pricing/apply_discount".to_string(),
                            carrier_kind: BenchmarkCarrierKind::Unit,
                            carrier_id: "pricing/apply_discount".to_string(),
                            classification: BenchmarkClassification::Supported,
                        },
                    ],
                },
                BenchmarkLabel {
                    id: "BENCH-SERVICE".to_string(),
                    kind: BenchmarkKind::Positive,
                    lifecycle: BenchmarkLifecycle::Reserved,
                    required_for_v1: true,
                    root: "examples/service/units".to_string(),
                    generated_root: "examples/service/src/generated".to_string(),
                    readability_scope: BenchmarkReadabilityScope::SupportedClosure,
                    required_molecule_ids: vec![],
                    cases: vec![],
                },
            ],
        }
    }

    fn write_registry(registry: &BenchmarkLabelRegistry) -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("labels.json");
        fs::write(&path, serde_json::to_vec_pretty(registry).unwrap()).unwrap();
        (dir, path)
    }

    fn case_truth(
        status: BenchmarkTruthStatus,
        semantic_support_status: Option<SemanticSupportStatus>,
    ) -> BenchmarkCaseTruth {
        BenchmarkCaseTruth {
            status,
            reason: None,
            semantic_review: None,
            semantic_support_status,
        }
    }

    fn molecule_truth(covers: &[&str], status: BenchmarkTruthStatus) -> BenchmarkMoleculeTruth {
        BenchmarkMoleculeTruth {
            covers: covers.iter().map(|cover| (*cover).to_string()).collect(),
            status,
            reason: None,
        }
    }

    #[test]
    fn load_labels_parses_valid_registry() {
        let (dir, path) = write_registry(&valid_registry());
        let registry = load_labels(&path).unwrap();
        assert_eq!(registry.schema_version, BENCHMARK_LABELS_SCHEMA_VERSION);
        assert_eq!(registry.benchmarks.len(), 3);
        assert_eq!(registry.benchmarks[0].id, "BENCH-CROSSLIB");
        assert_eq!(
            registry.benchmarks[1].required_molecule_ids,
            vec![
                "pricing/checkout_flow".to_string(),
                "pricing/discount_plus_tax".to_string()
            ]
        );
        drop(dir);
    }

    #[test]
    fn load_labels_rejects_duplicate_benchmark_id() {
        let mut registry = valid_registry();
        registry.benchmarks.push(registry.benchmarks[0].clone());
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(details.code.as_ref(), "SPEC_BENCHMARK_DUPLICATE_ID");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn load_labels_rejects_duplicate_case_id_within_benchmark() {
        let mut registry = valid_registry();
        registry.benchmarks[1].cases.push(BenchmarkCaseLabel {
            case_id: "pricing/apply_discount".to_string(),
            carrier_kind: BenchmarkCarrierKind::Unit,
            carrier_id: "pricing/calculate_total".to_string(),
            classification: BenchmarkClassification::Supported,
        });
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(details.code.as_ref(), "SPEC_BENCHMARK_DUPLICATE_CASE_ID");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn load_labels_rejects_duplicate_carrier_mapping() {
        let mut registry = valid_registry();
        registry.benchmarks[1].cases.push(BenchmarkCaseLabel {
            case_id: "pricing/apply_discount-copy".to_string(),
            carrier_kind: BenchmarkCarrierKind::Unit,
            carrier_id: "pricing/apply_discount".to_string(),
            classification: BenchmarkClassification::Supported,
        });
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(details.code.as_ref(), "SPEC_BENCHMARK_DUPLICATE_CARRIER");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn load_labels_rejects_reserved_benchmark_with_cases() {
        let mut registry = valid_registry();
        registry.benchmarks[2].cases.push(BenchmarkCaseLabel {
            case_id: "pricing/apply_discount".to_string(),
            carrier_kind: BenchmarkCarrierKind::Unit,
            carrier_id: "pricing/apply_discount".to_string(),
            classification: BenchmarkClassification::Supported,
        });
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(details.code.as_ref(), "SPEC_BENCHMARK_RESERVED_HAS_CASES");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn load_labels_rejects_case_outside_root() {
        let mut registry = valid_registry();
        registry.benchmarks[1].cases[0].carrier_id = "../money/round".to_string();
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(details.code.as_ref(), "SPEC_BENCHMARK_CASE_OUTSIDE_ROOT");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn load_labels_rejects_required_molecule_outside_root() {
        let mut registry = valid_registry();
        registry.benchmarks[1]
            .required_molecule_ids
            .push("../checkout_flow".to_string());
        let (_dir, path) = write_registry(&registry);
        let err = load_labels(&path).unwrap_err();
        match err {
            SpecError::BenchmarkRegistryInvalid(details) => {
                assert_eq!(
                    details.code.as_ref(),
                    "SPEC_BENCHMARK_REQUIRED_MOLECULE_OUTSIDE_ROOT"
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn project_benchmark_full_active_positive_passes() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::from([
                (
                    "pricing/apply_discount".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/apply_tax".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
            ]),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::from([
                (
                    "pricing/checkout_flow".to_string(),
                    molecule_truth(
                        &["pricing/apply_discount", "pricing/apply_tax"],
                        BenchmarkTruthStatus::Valid,
                    ),
                ),
                (
                    "pricing/discount_plus_tax".to_string(),
                    molecule_truth(
                        &["pricing/apply_discount", "pricing/apply_tax"],
                        BenchmarkTruthStatus::Valid,
                    ),
                ),
            ]),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::Valid
        );
        assert_eq!(projection.benchmark_status, Some(BenchmarkStatus::Passing));
        assert_eq!(projection.gate_status, Some(BenchmarkGateStatus::Satisfied));
        assert_eq!(
            projection.readability_review_status,
            Some(BenchmarkReadabilityReviewStatus::Missing)
        );
        assert_eq!(projection.required_molecule_proofs.len(), 2);
        assert!(projection.label_digest.is_some());
        assert!(projection.projection_digest.is_some());
        assert_eq!(projection.cases.len(), 2);
        assert!(
            projection
                .cases
                .iter()
                .all(|case| case.counts_as_supported_positive)
        );
        assert_eq!(
            projection
                .summary
                .as_ref()
                .expect("full scope summary")
                .positive_credit_cases,
            2
        );
    }

    #[test]
    fn project_benchmark_full_active_positive_incomplete() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::from([
                (
                    "pricing/apply_discount".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/apply_tax".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
            ]),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::from([(
                "pricing/checkout_flow".to_string(),
                molecule_truth(&["pricing/apply_discount"], BenchmarkTruthStatus::Untested),
            )]),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::Valid
        );
        assert_eq!(
            projection.benchmark_status,
            Some(BenchmarkStatus::Incomplete)
        );
        assert_eq!(projection.gate_status, Some(BenchmarkGateStatus::Open));
    }

    #[test]
    fn project_benchmark_full_active_positive_invalid_accounting() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::from([
                (
                    "pricing/apply_discount".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/apply_tax".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/unlabeled".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
            ]),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::new(),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::Invalid
        );
        assert_eq!(projection.benchmark_status, Some(BenchmarkStatus::Invalid));
        assert_eq!(projection.gate_status, Some(BenchmarkGateStatus::Open));
        assert_eq!(
            projection.summary.unwrap().unlabeled_loaded_carrier_ids,
            vec!["pricing/unlabeled".to_string()]
        );
    }

    #[test]
    fn project_benchmark_full_companion_negative_never_counts_positive_credit() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-CROSSLIB")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::from([(
                "pricing/apply_discount".to_string(),
                case_truth(
                    BenchmarkTruthStatus::Valid,
                    Some(SemanticSupportStatus::Supported),
                ),
            )]),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::new(),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::Valid
        );
        assert_eq!(projection.benchmark_status, Some(BenchmarkStatus::Passing));
        assert_eq!(
            projection.gate_status,
            Some(BenchmarkGateStatus::NotApplicable)
        );
        assert!(
            projection
                .cases
                .iter()
                .all(|case| !case.counts_as_supported_positive)
        );
    }

    #[test]
    fn project_benchmark_full_reserved_emits_reserved_gate_state() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-SERVICE")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: false,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::new(),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::new(),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::ReservedMissingCases
        );
        assert_eq!(projection.benchmark_status, Some(BenchmarkStatus::Reserved));
        assert_eq!(projection.gate_status, Some(BenchmarkGateStatus::Reserved));
        assert_eq!(
            projection.readability_review_status,
            Some(BenchmarkReadabilityReviewStatus::Missing)
        );
    }

    #[test]
    fn project_benchmark_partial_valid_when_selected_carriers_are_fully_labeled() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Partial,
            root_case_truths: BTreeMap::from([
                (
                    "pricing/apply_discount".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/apply_tax".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
            ]),
            selected_carrier_ids: BTreeSet::from(["pricing/apply_discount".to_string()]),
            required_molecule_truths: BTreeMap::new(),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::PartialValid
        );
        assert_eq!(projection.benchmark_status, None);
        assert_eq!(projection.gate_status, None);
        assert_eq!(projection.label_digest, None);
        assert_eq!(projection.projection_digest, None);
        assert_eq!(projection.cases.len(), 1);
        assert!(!projection.cases[0].counts_as_supported_positive);
    }

    #[test]
    fn project_benchmark_partial_invalid_when_selected_carrier_is_unlabeled() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Partial,
            root_case_truths: BTreeMap::from([(
                "pricing/unlabeled".to_string(),
                case_truth(
                    BenchmarkTruthStatus::Valid,
                    Some(SemanticSupportStatus::Supported),
                ),
            )]),
            selected_carrier_ids: BTreeSet::from(["pricing/unlabeled".to_string()]),
            required_molecule_truths: BTreeMap::new(),
            readability_review: None,
        };

        let projection = project_benchmark(&benchmark, &request);
        assert_eq!(
            projection.accounting_status,
            BenchmarkAccountingStatus::PartialInvalid
        );
        assert!(projection.cases.is_empty());
        assert_eq!(projection.benchmark_status, None);
    }

    #[test]
    fn compute_label_digest_is_stable_across_outer_registry_and_case_ordering() {
        let registry_a = valid_registry();
        let mut registry_b = valid_registry();
        registry_b.benchmarks.reverse();
        registry_b.benchmarks[1].cases.reverse();

        let bench_a = registry_a
            .benchmarks
            .iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let bench_b = registry_b
            .benchmarks
            .iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();

        assert_eq!(compute_label_digest(bench_a), compute_label_digest(bench_b));
    }

    #[test]
    fn compute_projection_digest_excludes_readability_verdict_and_changes_for_proof_refs() {
        let benchmark = valid_registry()
            .benchmarks
            .into_iter()
            .find(|benchmark| benchmark.id == "BENCH-ECOM")
            .unwrap();
        let request = BenchmarkProjectionRequest {
            benchmark_root_exists: true,
            path_scope: BenchmarkPathScope::Full,
            root_case_truths: BTreeMap::from([
                (
                    "pricing/apply_discount".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
                (
                    "pricing/apply_tax".to_string(),
                    case_truth(
                        BenchmarkTruthStatus::Valid,
                        Some(SemanticSupportStatus::Supported),
                    ),
                ),
            ]),
            selected_carrier_ids: BTreeSet::new(),
            required_molecule_truths: BTreeMap::from([(
                "pricing/checkout_flow".to_string(),
                molecule_truth(&["pricing/apply_discount"], BenchmarkTruthStatus::Valid),
            )]),
            readability_review: Some(BenchmarkReadabilityReviewInput {
                status: BenchmarkReadabilityReviewStatus::Current,
                verdict: Some(json!({"score": "high"})),
            }),
        };

        let projection = project_benchmark(&benchmark, &request);
        let mut changed_verdict = projection.clone();
        changed_verdict.readability_verdict = Some(json!({"score": "medium"}));
        assert_eq!(
            compute_projection_digest(&projection),
            compute_projection_digest(&changed_verdict)
        );

        let mut changed_proofs = projection.clone();
        changed_proofs.cases[0]
            .proof_refs
            .as_mut()
            .unwrap()
            .covering_molecule_evidence
            .push(
                "examples/ecommerce/units/pricing/discount_plus_tax.test.evidence.json".to_string(),
            );
        changed_proofs.cases[0]
            .proof_refs
            .as_mut()
            .unwrap()
            .covering_molecule_evidence
            .sort();
        assert_ne!(
            compute_projection_digest(&projection),
            compute_projection_digest(&changed_proofs)
        );
    }

    #[test]
    fn readability_generated_files_include_supported_case_files_mods_and_required_molecule_tests() {
        let benchmark = BenchmarkLabel {
            id: "BENCH-ECOM".to_string(),
            kind: BenchmarkKind::Positive,
            lifecycle: BenchmarkLifecycle::Active,
            required_for_v1: true,
            root: "examples/ecommerce/units".to_string(),
            generated_root: "examples/ecommerce/src/generated".to_string(),
            readability_scope: BenchmarkReadabilityScope::SupportedClosure,
            required_molecule_ids: vec![
                "pricing/checkout_flow".to_string(),
                "pricing/sub/checkout_branch".to_string(),
            ],
            cases: vec![],
        };
        let cases = vec![
            BenchmarkCaseProjection {
                case_id: "pricing/apply_discount".to_string(),
                carrier_kind: BenchmarkCarrierKind::Unit,
                carrier_id: "pricing/apply_discount".to_string(),
                classification: BenchmarkClassification::Supported,
                status: BenchmarkTruthStatus::Valid,
                reason: None,
                semantic_support_status: Some(SemanticSupportStatus::Supported),
                category_qualification: None,
                proof_refs: None,
                counts_as_supported_positive: true,
            },
            BenchmarkCaseProjection {
                case_id: "pricing/sub/deferred_case".to_string(),
                carrier_kind: BenchmarkCarrierKind::Unit,
                carrier_id: "pricing/sub/deferred_case".to_string(),
                classification: BenchmarkClassification::Deferred,
                status: BenchmarkTruthStatus::Valid,
                reason: None,
                semantic_support_status: Some(SemanticSupportStatus::Supported),
                category_qualification: None,
                proof_refs: None,
                counts_as_supported_positive: false,
            },
        ];

        let files = readability_generated_files(&benchmark, &cases);
        assert!(files.contains(&"examples/ecommerce/src/generated/mod.rs".to_string()));
        assert!(files.contains(&"examples/ecommerce/src/generated/pricing/mod.rs".to_string()));
        assert!(
            files.contains(
                &"examples/ecommerce/src/generated/pricing/apply_discount.rs".to_string()
            )
        );
        assert!(
            files.contains(
                &"examples/ecommerce/src/generated/pricing/molecule_tests.rs".to_string()
            )
        );
        assert!(files.contains(
            &"examples/ecommerce/src/generated/pricing/sub/molecule_tests.rs".to_string()
        ));
        assert!(!files.contains(
            &"examples/ecommerce/src/generated/pricing/sub/deferred_case.rs".to_string()
        ));
    }

    #[test]
    fn benchmark_path_scope_detects_full_and_partial_relations() {
        let benchmark_root = Path::new("/repo/examples/ecommerce/units");
        assert_eq!(
            benchmark_path_scope(Path::new("/repo"), benchmark_root, true),
            Some(BenchmarkPathScope::Full)
        );
        assert_eq!(
            benchmark_path_scope(
                Path::new("/repo/examples/ecommerce/units/pricing"),
                benchmark_root,
                false
            ),
            Some(BenchmarkPathScope::Partial)
        );
        assert_eq!(
            benchmark_path_scope(Path::new("/repo/other"), benchmark_root, false),
            None
        );
    }
}
