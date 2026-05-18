//! Shared benchmark registry parsing and read-side projection.

use crate::escape_hatch::EscapeHatchGateStatus;
use crate::molecule_evidence::{
    MoleculeEvidence, MoleculeEvidenceStatus, molecule_evidence_is_current_pass,
    molecule_evidence_is_stale, molecule_evidence_path_for,
};
use crate::passport::{Passport, passport_path_for};
use crate::semantic_review::{SemanticHealthEffect, SemanticSupportStatus, semantic_health_effect};
use crate::types::{LoadedMoleculeTest, LoadedSpec};
use crate::{Result, SpecError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

const BENCHMARK_REGISTRY_PATH: &str = "benchmarks/labels.json";

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
pub enum BenchmarkClassification {
    Supported,
    Deferred,
    FallbackBacked,
    ExplicitlyOut,
    CompanionNegativeProof,
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
pub enum BenchmarkCarrierKind {
    Unit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCarrierStatus {
    Missing,
    Invalid,
    Failing,
    Stale,
    Incomplete,
    Untested,
    Valid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkSemanticSupportStatus {
    Supported,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkRegistry {
    pub benchmarks: Vec<BenchmarkLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkLabel {
    pub id: String,
    pub kind: BenchmarkKind,
    pub lifecycle: BenchmarkLifecycle,
    pub root: String,
    pub generated_root: String,
    #[serde(default)]
    pub required_molecules: Vec<String>,
    #[serde(default)]
    pub cases: Vec<BenchmarkCaseLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkCaseLabel {
    pub case_id: String,
    pub carrier_id: String,
    pub classification: BenchmarkClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkProjection {
    pub id: String,
    pub kind: BenchmarkKind,
    pub lifecycle: BenchmarkLifecycle,
    pub root: String,
    pub generated_root: String,
    pub path_scope: BenchmarkPathScope,
    pub accounting_status: BenchmarkAccountingStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_status: Option<BenchmarkStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_status: Option<BenchmarkGateStatus>,
    pub cases: Vec<BenchmarkCaseProjection>,
    pub required_molecule_proofs: Vec<BenchmarkRequiredMoleculeProjection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkCaseProjection {
    pub case_id: String,
    pub carrier_kind: BenchmarkCarrierKind,
    pub carrier_id: String,
    pub classification: BenchmarkClassification,
    pub carrier_status: BenchmarkCarrierStatus,
    pub semantic_support_status: BenchmarkSemanticSupportStatus,
    pub passport_path: Option<String>,
    pub counts_as_supported_positive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkRequiredMoleculeProjection {
    pub id: String,
    pub status: BenchmarkCarrierStatus,
    pub evidence_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkUnitTruth {
    pub carrier_status: BenchmarkCarrierStatus,
    pub semantic_support_status: BenchmarkSemanticSupportStatus,
    pub passport_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkMoleculeTruth {
    pub status: BenchmarkCarrierStatus,
    pub evidence_path: Option<String>,
}

pub struct BenchmarkProjectionInput<'a> {
    pub repo_root: &'a Path,
    pub scope_path: &'a Path,
    pub specs: &'a [LoadedSpec],
    pub molecule_tests: &'a [LoadedMoleculeTest],
    pub unit_truth_by_id: &'a HashMap<String, BenchmarkUnitTruth>,
    pub molecule_truth_by_id: &'a HashMap<String, BenchmarkMoleculeTruth>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkRegistryDocument {
    benchmarks: Vec<BenchmarkLabelDocument>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkLabelDocument {
    id: String,
    kind: BenchmarkKind,
    lifecycle: BenchmarkLifecycle,
    root: String,
    generated_root: String,
    #[serde(default)]
    required_molecules: Vec<String>,
    #[serde(default)]
    cases: Vec<BenchmarkCaseLabelDocument>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkCaseLabelDocument {
    case_id: String,
    carrier_id: String,
    classification: BenchmarkClassification,
}

pub fn load_benchmark_registry(repo_root: &Path) -> Result<BenchmarkRegistry> {
    let registry_path = repo_root.join(BENCHMARK_REGISTRY_PATH);
    let content = fs::read_to_string(&registry_path)?;
    parse_benchmark_registry_from_str(&content, repo_root, &registry_path)
}

pub fn parse_benchmark_registry_from_str(
    content: &str,
    repo_root: &Path,
    source_path: &Path,
) -> Result<BenchmarkRegistry> {
    let document: BenchmarkRegistryDocument =
        serde_json::from_str(content).map_err(|err| benchmark_registry_error(source_path, err))?;
    let mut benchmark_ids = HashSet::new();
    let mut benchmarks = Vec::with_capacity(document.benchmarks.len());

    for benchmark in document.benchmarks {
        if !benchmark_ids.insert(benchmark.id.clone()) {
            return Err(benchmark_registry_message(
                source_path,
                format!("duplicate benchmark id '{}'", benchmark.id),
            ));
        }

        let root = normalize_repo_relative_path(
            Path::new(&benchmark.root),
            repo_root,
            source_path,
            "root",
            false,
        )?;
        let generated_root = normalize_repo_relative_path(
            Path::new(&benchmark.generated_root),
            repo_root,
            source_path,
            "generated_root",
            false,
        )?;

        if benchmark.lifecycle == BenchmarkLifecycle::Active && path_looks_generated(&root) {
            return Err(benchmark_registry_message(
                source_path,
                format!(
                    "active benchmark '{}' root '{}' points at generated output",
                    benchmark.id, root
                ),
            ));
        }

        let mut case_ids = HashSet::new();
        let mut carrier_ids = HashSet::new();
        let mut cases = Vec::with_capacity(benchmark.cases.len());
        for case in benchmark.cases {
            if !case_ids.insert(case.case_id.clone()) {
                return Err(benchmark_registry_message(
                    source_path,
                    format!(
                        "duplicate case_id '{}' in benchmark '{}'",
                        case.case_id, benchmark.id
                    ),
                ));
            }
            if !carrier_ids.insert(case.carrier_id.clone()) {
                return Err(benchmark_registry_message(
                    source_path,
                    format!(
                        "duplicate carrier mapping '{}' in benchmark '{}'",
                        case.carrier_id, benchmark.id
                    ),
                ));
            }
            cases.push(BenchmarkCaseLabel {
                case_id: case.case_id,
                carrier_id: case.carrier_id,
                classification: case.classification,
            });
        }

        benchmarks.push(BenchmarkLabel {
            id: benchmark.id,
            kind: benchmark.kind,
            lifecycle: benchmark.lifecycle,
            root,
            generated_root,
            required_molecules: benchmark.required_molecules,
            cases,
        });
    }

    Ok(BenchmarkRegistry { benchmarks })
}

pub fn build_benchmark_unit_truth_map(
    specs: &[LoadedSpec],
    passports_by_id: &HashMap<String, Passport>,
) -> Result<HashMap<String, BenchmarkUnitTruth>> {
    specs
        .iter()
        .map(|spec| {
            Ok((
                spec.spec.id.clone(),
                benchmark_unit_truth_from_passport(spec, passports_by_id.get(&spec.spec.id))?,
            ))
        })
        .collect()
}

pub fn build_benchmark_molecule_truth_map(
    molecule_tests: &[LoadedMoleculeTest],
    molecule_truth_by_id: &HashMap<String, MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<HashMap<String, BenchmarkMoleculeTruth>> {
    molecule_tests
        .iter()
        .map(|test| {
            Ok((
                test.test.id.clone(),
                benchmark_molecule_truth_from_evidence(
                    test,
                    molecule_truth_by_id.get(&test.test.id),
                    specs_by_id,
                )?,
            ))
        })
        .collect()
}

pub fn project_benchmarks(
    registry: &BenchmarkRegistry,
    input: BenchmarkProjectionInput<'_>,
) -> Result<Vec<BenchmarkProjection>> {
    let normalized_scope = normalize_repo_relative_path(
        input.scope_path,
        input.repo_root,
        input.repo_root,
        "scope",
        true,
    )?;
    let specs_by_id: HashMap<String, &LoadedSpec> = input
        .specs
        .iter()
        .map(|spec| (spec.spec.id.clone(), spec))
        .collect();
    let spec_paths_by_id: HashMap<String, String> = input
        .specs
        .iter()
        .map(|spec| {
            normalize_repo_relative_path(
                Path::new(&spec.source.file_path),
                input.repo_root,
                input.repo_root,
                "spec.source.file_path",
                false,
            )
            .map(|path| (spec.spec.id.clone(), path))
        })
        .collect::<Result<_>>()?;
    let molecule_tests_by_id: HashMap<String, &LoadedMoleculeTest> = input
        .molecule_tests
        .iter()
        .map(|test| (test.test.id.clone(), test))
        .collect();
    let molecule_paths_by_id: HashMap<String, String> = input
        .molecule_tests
        .iter()
        .map(|test| {
            normalize_repo_relative_path(
                Path::new(&test.source.file_path),
                input.repo_root,
                input.repo_root,
                "molecule_test.source.file_path",
                false,
            )
            .map(|path| (test.test.id.clone(), path))
        })
        .collect::<Result<_>>()?;

    let mut projections = Vec::new();
    for benchmark in &registry.benchmarks {
        let Some(path_scope) = classify_benchmark_scope(&benchmark.root, &normalized_scope) else {
            continue;
        };
        if benchmark.lifecycle == BenchmarkLifecycle::Reserved
            && path_scope != BenchmarkPathScope::Full
        {
            continue;
        }

        validate_full_scope_registry_refs(
            benchmark,
            path_scope,
            &specs_by_id,
            &spec_paths_by_id,
            &molecule_tests_by_id,
        )?;

        let visible_spec_ids: Vec<&str> = input
            .specs
            .iter()
            .filter_map(|spec| {
                let path = spec_paths_by_id.get(&spec.spec.id)?;
                case_path_visible(path, &benchmark.root, &normalized_scope)
                    .then_some(spec.spec.id.as_str())
            })
            .collect();
        let labeled_carrier_ids: HashSet<&str> = benchmark
            .cases
            .iter()
            .map(|case| case.carrier_id.as_str())
            .collect();
        let has_unlabeled_visible_specs = visible_spec_ids
            .iter()
            .any(|spec_id| !labeled_carrier_ids.contains(spec_id));
        let accounting_status = match (benchmark.lifecycle, path_scope, has_unlabeled_visible_specs)
        {
            (BenchmarkLifecycle::Reserved, _, _) => BenchmarkAccountingStatus::ReservedMissingCases,
            (BenchmarkLifecycle::Active, BenchmarkPathScope::Full, false) => {
                BenchmarkAccountingStatus::Valid
            }
            (BenchmarkLifecycle::Active, BenchmarkPathScope::Full, true) => {
                BenchmarkAccountingStatus::Invalid
            }
            (BenchmarkLifecycle::Active, BenchmarkPathScope::Partial, false) => {
                BenchmarkAccountingStatus::PartialValid
            }
            (BenchmarkLifecycle::Active, BenchmarkPathScope::Partial, true) => {
                BenchmarkAccountingStatus::PartialInvalid
            }
        };

        let cases = benchmark
            .cases
            .iter()
            .filter_map(|case| {
                let spec = specs_by_id.get(&case.carrier_id)?;
                let path = spec_paths_by_id.get(&case.carrier_id)?;
                case_path_visible(path, &benchmark.root, &normalized_scope).then_some((case, *spec))
            })
            .map(|(case, spec)| {
                let truth = input
                    .unit_truth_by_id
                    .get(&case.carrier_id)
                    .cloned()
                    .unwrap_or_else(|| default_benchmark_unit_truth(spec));
                let counts_as_supported_positive = benchmark.kind
                    != BenchmarkKind::CompanionNegativeProof
                    && path_scope == BenchmarkPathScope::Full
                    && case.classification == BenchmarkClassification::Supported
                    && truth.carrier_status == BenchmarkCarrierStatus::Valid
                    && truth.semantic_support_status == BenchmarkSemanticSupportStatus::Supported;
                BenchmarkCaseProjection {
                    case_id: case.case_id.clone(),
                    carrier_kind: BenchmarkCarrierKind::Unit,
                    carrier_id: case.carrier_id.clone(),
                    classification: case.classification,
                    carrier_status: truth.carrier_status,
                    semantic_support_status: truth.semantic_support_status,
                    passport_path: truth.passport_path,
                    counts_as_supported_positive,
                }
            })
            .collect::<Vec<_>>();

        let required_molecule_proofs = benchmark
            .required_molecules
            .iter()
            .filter_map(|molecule_id| {
                let test = molecule_tests_by_id.get(molecule_id)?;
                let path = molecule_paths_by_id.get(molecule_id)?;
                case_path_visible(path, &benchmark.root, &normalized_scope)
                    .then_some((molecule_id, *test))
            })
            .map(|(molecule_id, test)| {
                let truth = input
                    .molecule_truth_by_id
                    .get(molecule_id)
                    .cloned()
                    .unwrap_or_else(|| default_benchmark_molecule_truth(test));
                BenchmarkRequiredMoleculeProjection {
                    id: molecule_id.clone(),
                    status: truth.status,
                    evidence_path: truth.evidence_path,
                }
            })
            .collect::<Vec<_>>();

        let label_digest =
            (path_scope == BenchmarkPathScope::Full).then(|| compute_label_digest(benchmark));
        let (benchmark_status, gate_status) = match path_scope {
            BenchmarkPathScope::Partial => (None, None),
            BenchmarkPathScope::Full => derive_rollup_status(
                benchmark,
                accounting_status,
                &cases,
                &required_molecule_proofs,
            ),
        };

        projections.push(BenchmarkProjection {
            id: benchmark.id.clone(),
            kind: benchmark.kind,
            lifecycle: benchmark.lifecycle,
            root: benchmark.root.clone(),
            generated_root: benchmark.generated_root.clone(),
            path_scope,
            accounting_status,
            label_digest,
            benchmark_status,
            gate_status,
            cases,
            required_molecule_proofs,
        });
    }

    Ok(projections)
}

fn benchmark_unit_truth_from_passport(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
) -> Result<BenchmarkUnitTruth> {
    Ok(BenchmarkUnitTruth {
        carrier_status: benchmark_carrier_status_from_passport(passport),
        semantic_support_status: passport
            .and_then(|passport| passport.semantic_review.as_ref())
            .map_or(
                BenchmarkSemanticSupportStatus::Unknown,
                |review| match review.effective_support_status() {
                    SemanticSupportStatus::Supported => BenchmarkSemanticSupportStatus::Supported,
                    SemanticSupportStatus::Unsupported => {
                        BenchmarkSemanticSupportStatus::Unsupported
                    }
                },
            ),
        passport_path: Some(
            passport_path_for(Path::new(&spec.source.file_path))?
                .display()
                .to_string(),
        ),
    })
}

fn benchmark_molecule_truth_from_evidence(
    test: &LoadedMoleculeTest,
    evidence: Option<&MoleculeEvidence>,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<BenchmarkMoleculeTruth> {
    let status = match evidence {
        None => BenchmarkCarrierStatus::Untested,
        Some(evidence)
            if matches!(evidence.status, MoleculeEvidenceStatus::Stale)
                || molecule_evidence_is_stale(evidence, test, specs_by_id) =>
        {
            BenchmarkCarrierStatus::Stale
        }
        Some(evidence) if molecule_evidence_is_current_pass(evidence, test, specs_by_id) => {
            BenchmarkCarrierStatus::Valid
        }
        Some(evidence) => match evidence.status {
            MoleculeEvidenceStatus::BuildFail
            | MoleculeEvidenceStatus::Timeout
            | MoleculeEvidenceStatus::Fail => BenchmarkCarrierStatus::Failing,
            MoleculeEvidenceStatus::Unknown => BenchmarkCarrierStatus::Incomplete,
            MoleculeEvidenceStatus::Pass | MoleculeEvidenceStatus::Stale => {
                BenchmarkCarrierStatus::Invalid
            }
        },
    };

    Ok(BenchmarkMoleculeTruth {
        status,
        evidence_path: Some(
            molecule_evidence_path_for(Path::new(&test.source.file_path))?
                .display()
                .to_string(),
        ),
    })
}

fn benchmark_carrier_status_from_passport(passport: Option<&Passport>) -> BenchmarkCarrierStatus {
    let Some(passport) = passport else {
        return BenchmarkCarrierStatus::Untested;
    };
    let evidence = passport.evidence.as_ref();
    if let Some(evidence) = evidence {
        if evidence.build_status != "pass"
            || evidence
                .test_results
                .iter()
                .any(|result| result.status == "fail")
        {
            return BenchmarkCarrierStatus::Failing;
        }
    }
    if passport
        .freshness
        .as_ref()
        .is_some_and(passport_freshness_is_stale)
    {
        return BenchmarkCarrierStatus::Stale;
    }
    if let Some(evidence) = evidence {
        if evidence
            .test_results
            .iter()
            .any(|result| result.status == "unknown")
        {
            return BenchmarkCarrierStatus::Incomplete;
        }
    }
    if evidence.is_none() {
        return BenchmarkCarrierStatus::Untested;
    }
    if passport
        .escape_hatch_gate
        .as_ref()
        .is_some_and(|gate| gate.status == EscapeHatchGateStatus::Open)
    {
        return BenchmarkCarrierStatus::Incomplete;
    }
    match semantic_health_effect(passport.semantic_review.as_ref()) {
        SemanticHealthEffect::KeepBase => BenchmarkCarrierStatus::Valid,
        SemanticHealthEffect::DemoteIncomplete => BenchmarkCarrierStatus::Incomplete,
        SemanticHealthEffect::DemoteFailing => BenchmarkCarrierStatus::Failing,
    }
}

fn passport_freshness_is_stale(freshness: &crate::passport::PassportFreshness) -> bool {
    freshness.authored_truth_status == crate::passport::FreshnessStatus::Stale
        || freshness.backend_execution_status == crate::passport::FreshnessStatus::Stale
}

fn default_benchmark_unit_truth(spec: &LoadedSpec) -> BenchmarkUnitTruth {
    BenchmarkUnitTruth {
        carrier_status: BenchmarkCarrierStatus::Untested,
        semantic_support_status: BenchmarkSemanticSupportStatus::Unknown,
        passport_path: passport_path_for(Path::new(&spec.source.file_path))
            .ok()
            .map(|path| path.display().to_string()),
    }
}

fn default_benchmark_molecule_truth(test: &LoadedMoleculeTest) -> BenchmarkMoleculeTruth {
    BenchmarkMoleculeTruth {
        status: BenchmarkCarrierStatus::Untested,
        evidence_path: molecule_evidence_path_for(Path::new(&test.source.file_path))
            .ok()
            .map(|path| path.display().to_string()),
    }
}

fn validate_full_scope_registry_refs(
    benchmark: &BenchmarkLabel,
    path_scope: BenchmarkPathScope,
    specs_by_id: &HashMap<String, &LoadedSpec>,
    spec_paths_by_id: &HashMap<String, String>,
    molecule_tests_by_id: &HashMap<String, &LoadedMoleculeTest>,
) -> Result<()> {
    if benchmark.lifecycle != BenchmarkLifecycle::Active || path_scope != BenchmarkPathScope::Full {
        return Ok(());
    }

    for case in &benchmark.cases {
        let Some(_) = specs_by_id.get(&case.carrier_id) else {
            return Err(benchmark_registry_message(
                Path::new(BENCHMARK_REGISTRY_PATH),
                format!(
                    "active benchmark '{}' carrier '{}' does not resolve to an authored unit id",
                    benchmark.id, case.carrier_id
                ),
            ));
        };
        let path = spec_paths_by_id
            .get(&case.carrier_id)
            .expect("spec path must exist");
        if !path_is_ancestor_or_same(&benchmark.root, path) {
            return Err(benchmark_registry_message(
                Path::new(BENCHMARK_REGISTRY_PATH),
                format!(
                    "active benchmark '{}' carrier '{}' is outside root '{}'",
                    benchmark.id, case.carrier_id, benchmark.root
                ),
            ));
        }
    }

    for required_molecule in &benchmark.required_molecules {
        if !molecule_tests_by_id.contains_key(required_molecule) {
            return Err(benchmark_registry_message(
                Path::new(BENCHMARK_REGISTRY_PATH),
                format!(
                    "active benchmark '{}' required molecule '{}' does not resolve to a loaded molecule test",
                    benchmark.id, required_molecule
                ),
            ));
        }
    }

    Ok(())
}

fn derive_rollup_status(
    benchmark: &BenchmarkLabel,
    accounting_status: BenchmarkAccountingStatus,
    cases: &[BenchmarkCaseProjection],
    required_molecule_proofs: &[BenchmarkRequiredMoleculeProjection],
) -> (Option<BenchmarkStatus>, Option<BenchmarkGateStatus>) {
    if benchmark.lifecycle == BenchmarkLifecycle::Reserved {
        return (
            Some(BenchmarkStatus::Reserved),
            Some(BenchmarkGateStatus::Reserved),
        );
    }

    match benchmark.kind {
        BenchmarkKind::Positive => {
            let status = derive_positive_benchmark_status(
                accounting_status,
                cases,
                required_molecule_proofs,
            );
            let gate_status = if status == BenchmarkStatus::Passing {
                BenchmarkGateStatus::Satisfied
            } else {
                BenchmarkGateStatus::Open
            };
            (Some(status), Some(gate_status))
        }
        BenchmarkKind::CompanionNegativeProof => {
            let status = if accounting_status == BenchmarkAccountingStatus::Invalid {
                BenchmarkStatus::Invalid
            } else if cases
                .iter()
                .any(|case| case.carrier_status == BenchmarkCarrierStatus::Missing)
                || cases.len() != benchmark.cases.len()
            {
                BenchmarkStatus::Failing
            } else if cases.iter().any(|case| case.counts_as_supported_positive) {
                BenchmarkStatus::Failing
            } else {
                BenchmarkStatus::Passing
            };
            (Some(status), Some(BenchmarkGateStatus::NotApplicable))
        }
    }
}

fn derive_positive_benchmark_status(
    accounting_status: BenchmarkAccountingStatus,
    cases: &[BenchmarkCaseProjection],
    required_molecule_proofs: &[BenchmarkRequiredMoleculeProjection],
) -> BenchmarkStatus {
    if accounting_status == BenchmarkAccountingStatus::Invalid {
        return BenchmarkStatus::Invalid;
    }

    if cases
        .iter()
        .filter(|case| case.classification == BenchmarkClassification::Supported)
        .any(|case| {
            matches!(
                case.carrier_status,
                BenchmarkCarrierStatus::Missing
                    | BenchmarkCarrierStatus::Invalid
                    | BenchmarkCarrierStatus::Failing
            ) || !case.counts_as_supported_positive
        })
        || required_molecule_proofs.iter().any(|molecule| {
            matches!(
                molecule.status,
                BenchmarkCarrierStatus::Missing
                    | BenchmarkCarrierStatus::Invalid
                    | BenchmarkCarrierStatus::Failing
            )
        })
    {
        return BenchmarkStatus::Failing;
    }

    if cases
        .iter()
        .filter(|case| case.classification == BenchmarkClassification::Supported)
        .any(|case| {
            matches!(
                case.carrier_status,
                BenchmarkCarrierStatus::Stale
                    | BenchmarkCarrierStatus::Incomplete
                    | BenchmarkCarrierStatus::Untested
            )
        })
        || required_molecule_proofs.iter().any(|molecule| {
            matches!(
                molecule.status,
                BenchmarkCarrierStatus::Stale
                    | BenchmarkCarrierStatus::Incomplete
                    | BenchmarkCarrierStatus::Untested
            )
        })
    {
        return BenchmarkStatus::Incomplete;
    }

    BenchmarkStatus::Passing
}

fn compute_label_digest(benchmark: &BenchmarkLabel) -> String {
    let mut digest_document = BTreeMap::new();
    digest_document.insert("id", serde_json::json!(benchmark.id));
    digest_document.insert("kind", serde_json::json!(benchmark.kind));
    digest_document.insert("lifecycle", serde_json::json!(benchmark.lifecycle));
    digest_document.insert("root", serde_json::json!(benchmark.root));
    digest_document.insert(
        "generated_root",
        serde_json::json!(benchmark.generated_root),
    );

    let mut required_molecules = benchmark.required_molecules.clone();
    required_molecules.sort();
    digest_document.insert("required_molecules", serde_json::json!(required_molecules));

    let mut cases = benchmark.cases.clone();
    cases.sort_by(|left, right| left.case_id.cmp(&right.case_id));
    digest_document.insert("cases", serde_json::json!(cases));

    let canonical = serde_json::to_vec(&digest_document).expect("benchmark digest serialization");
    let hash = Sha256::digest(&canonical);
    format!("sha256:{}", hex::encode(hash))
}

fn classify_benchmark_scope(
    benchmark_root: &str,
    normalized_scope: &str,
) -> Option<BenchmarkPathScope> {
    if path_is_ancestor_or_same(normalized_scope, benchmark_root) {
        Some(BenchmarkPathScope::Full)
    } else if path_is_ancestor_or_same(benchmark_root, normalized_scope) {
        Some(BenchmarkPathScope::Partial)
    } else {
        None
    }
}

fn case_path_visible(path: &str, benchmark_root: &str, normalized_scope: &str) -> bool {
    path_is_ancestor_or_same(benchmark_root, path)
        && (path_is_ancestor_or_same(normalized_scope, path)
            || path_is_ancestor_or_same(path, normalized_scope))
}

fn path_is_ancestor_or_same(ancestor: &str, descendant: &str) -> bool {
    let ancestor_segments = path_segments(ancestor);
    let descendant_segments = path_segments(descendant);
    ancestor_segments.len() <= descendant_segments.len()
        && ancestor_segments
            .iter()
            .zip(descendant_segments.iter())
            .all(|(left, right)| left == right)
}

fn path_segments(path: &str) -> Vec<&str> {
    if path.is_empty() {
        Vec::new()
    } else {
        path.split('/').collect()
    }
}

fn path_looks_generated(path: &str) -> bool {
    let segments = path_segments(path);
    segments
        .windows(2)
        .any(|window| window == ["src", "generated"])
}

fn normalize_repo_relative_path(
    path: &Path,
    repo_root: &Path,
    source_path: &Path,
    field: &str,
    allow_empty: bool,
) -> Result<String> {
    let relative = if path.is_absolute() {
        path.strip_prefix(repo_root).map_err(|_| {
            benchmark_registry_message(
                source_path,
                format!(
                    "{field} path '{}' is outside repo root '{}'",
                    path.display(),
                    repo_root.display()
                ),
            )
        })?
    } else {
        path
    };

    let mut normalized = PathBuf::new();
    for component in relative.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(benchmark_registry_message(
                        source_path,
                        format!("{field} path '{}' escapes the repo root", path.display()),
                    ));
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(benchmark_registry_message(
                    source_path,
                    format!("{field} path '{}' must be repo-relative", path.display()),
                ));
            }
        }
    }

    let rendered = normalized
        .iter()
        .map(|segment| segment.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    if rendered.is_empty() && !allow_empty {
        return Err(benchmark_registry_message(
            source_path,
            format!(
                "{field} path '{}' resolves to the repo root",
                path.display()
            ),
        ));
    }
    Ok(rendered)
}

fn benchmark_registry_error(source_path: &Path, err: serde_json::Error) -> SpecError {
    benchmark_registry_message(
        source_path,
        format!("failed to parse benchmark registry JSON: {err}"),
    )
}

fn benchmark_registry_message(source_path: &Path, message: String) -> SpecError {
    SpecError::BenchmarkRegistry {
        path: source_path.display().to_string(),
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Body, Intent, LoadedMoleculeTest, LoadedSpec, LocalTest, MoleculeTestSource,
        MoleculeTestStruct, SpecSource, SpecStruct, UnitExtensions,
    };
    use indexmap::IndexMap;
    use tempfile::TempDir;

    fn loaded_spec(repo_root: &Path, rel_path: &str, id: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: repo_root.join(rel_path).display().to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: Some(crate::types::Contract {
                    inputs: Some(IndexMap::from([
                        ("value".to_string(), "Decimal".to_string()),
                        ("rate".to_string(), "Decimal".to_string()),
                    ])),
                    returns: Some("Decimal".to_string()),
                    invariants: vec![],
                }),
                deps: vec![],
                imports: vec![],
                body: Body {
                    rust: "{ value }".to_string(),
                    typescript: None,
                },
                local_tests: vec![LocalTest {
                    id: "happy_path".to_string(),
                    expect: "true".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions::default(),
            },
        }
    }

    fn loaded_molecule_test(repo_root: &Path, rel_path: &str, id: &str) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: repo_root.join(rel_path).display().to_string(),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                covers: vec!["pricing/apply_discount".to_string()],
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                    typescript: None,
                },
                spec_version: Some("0.3.0".to_string()),
            },
        }
    }

    fn positive_registry() -> BenchmarkRegistry {
        BenchmarkRegistry {
            benchmarks: vec![BenchmarkLabel {
                id: "BENCH-ECOM".to_string(),
                kind: BenchmarkKind::Positive,
                lifecycle: BenchmarkLifecycle::Active,
                root: "examples/ecommerce/units".to_string(),
                generated_root: "examples/ecommerce/src/generated".to_string(),
                required_molecules: vec!["pricing/checkout_flow".to_string()],
                cases: vec![
                    BenchmarkCaseLabel {
                        case_id: "discount".to_string(),
                        carrier_id: "pricing/apply_discount".to_string(),
                        classification: BenchmarkClassification::Supported,
                    },
                    BenchmarkCaseLabel {
                        case_id: "tax".to_string(),
                        carrier_id: "pricing/apply_tax".to_string(),
                        classification: BenchmarkClassification::Supported,
                    },
                ],
            }],
        }
    }

    #[test]
    fn benchmark_registry_load_normalizes_repo_relative_paths() {
        let repo_root = TempDir::new().unwrap();
        let registry_dir = repo_root.path().join("benchmarks");
        fs::create_dir_all(&registry_dir).unwrap();
        fs::write(
            registry_dir.join("labels.json"),
            r#"{
  "benchmarks": [
    {
      "id": "BENCH-ECOM",
      "kind": "positive",
      "lifecycle": "active",
      "root": "./examples/ecommerce/units",
      "generated_root": "examples/ecommerce/src/generated",
      "required_molecules": ["pricing/checkout_flow"],
      "cases": [
        {
          "case_id": "discount",
          "carrier_id": "pricing/apply_discount",
          "classification": "supported"
        }
      ]
    }
  ]
}"#,
        )
        .unwrap();

        let registry = load_benchmark_registry(repo_root.path()).unwrap();
        assert_eq!(registry.benchmarks.len(), 1);
        assert_eq!(registry.benchmarks[0].root, "examples/ecommerce/units");
        assert_eq!(
            registry.benchmarks[0].generated_root,
            "examples/ecommerce/src/generated"
        );
    }

    #[test]
    fn benchmark_registry_rejects_duplicate_carrier_mappings() {
        let repo_root = TempDir::new().unwrap();
        let error = parse_benchmark_registry_from_str(
            r#"{
  "benchmarks": [
    {
      "id": "BENCH-ECOM",
      "kind": "positive",
      "lifecycle": "active",
      "root": "examples/ecommerce/units",
      "generated_root": "examples/ecommerce/src/generated",
      "cases": [
        {
          "case_id": "discount",
          "carrier_id": "pricing/apply_discount",
          "classification": "supported"
        },
        {
          "case_id": "discount-2",
          "carrier_id": "pricing/apply_discount",
          "classification": "supported"
        }
      ]
    }
  ]
}"#,
            repo_root.path(),
            &repo_root.path().join("benchmarks/labels.json"),
        )
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("duplicate carrier mapping 'pricing/apply_discount'")
        );
    }

    #[test]
    fn benchmark_projector_derives_full_positive_status_and_gate() {
        let repo_root = TempDir::new().unwrap();
        let apply_discount = loaded_spec(
            repo_root.path(),
            "examples/ecommerce/units/pricing/apply_discount.unit.spec",
            "pricing/apply_discount",
        );
        let apply_tax = loaded_spec(
            repo_root.path(),
            "examples/ecommerce/units/pricing/apply_tax.unit.spec",
            "pricing/apply_tax",
        );
        let molecule = loaded_molecule_test(
            repo_root.path(),
            "examples/ecommerce/units/pricing/checkout_flow.test.spec",
            "pricing/checkout_flow",
        );

        let unit_truth = HashMap::from([
            (
                "pricing/apply_discount".to_string(),
                BenchmarkUnitTruth {
                    carrier_status: BenchmarkCarrierStatus::Valid,
                    semantic_support_status: BenchmarkSemanticSupportStatus::Supported,
                    passport_path: Some("apply_discount.spec.passport.json".to_string()),
                },
            ),
            (
                "pricing/apply_tax".to_string(),
                BenchmarkUnitTruth {
                    carrier_status: BenchmarkCarrierStatus::Valid,
                    semantic_support_status: BenchmarkSemanticSupportStatus::Supported,
                    passport_path: Some("apply_tax.spec.passport.json".to_string()),
                },
            ),
        ]);
        let molecule_truth = HashMap::from([(
            "pricing/checkout_flow".to_string(),
            BenchmarkMoleculeTruth {
                status: BenchmarkCarrierStatus::Valid,
                evidence_path: Some("checkout_flow.test.evidence.json".to_string()),
            },
        )]);

        let projections = project_benchmarks(
            &positive_registry(),
            BenchmarkProjectionInput {
                repo_root: repo_root.path(),
                scope_path: &repo_root.path().join("examples/ecommerce"),
                specs: &[apply_discount, apply_tax],
                molecule_tests: &[molecule],
                unit_truth_by_id: &unit_truth,
                molecule_truth_by_id: &molecule_truth,
            },
        )
        .unwrap();

        assert_eq!(projections.len(), 1);
        let benchmark = &projections[0];
        assert_eq!(benchmark.path_scope, BenchmarkPathScope::Full);
        assert_eq!(
            benchmark.accounting_status,
            BenchmarkAccountingStatus::Valid
        );
        assert_eq!(benchmark.benchmark_status, Some(BenchmarkStatus::Passing));
        assert_eq!(benchmark.gate_status, Some(BenchmarkGateStatus::Satisfied));
        assert!(benchmark.label_digest.is_some());
        assert!(
            benchmark
                .cases
                .iter()
                .all(|case| case.counts_as_supported_positive)
        );
    }

    #[test]
    fn benchmark_projector_marks_partial_scope_invalid_and_zero_credit() {
        let repo_root = TempDir::new().unwrap();
        let apply_discount = loaded_spec(
            repo_root.path(),
            "examples/ecommerce/units/pricing/apply_discount.unit.spec",
            "pricing/apply_discount",
        );
        let unlabeled = loaded_spec(
            repo_root.path(),
            "examples/ecommerce/units/pricing/unlabeled.unit.spec",
            "pricing/unlabeled",
        );
        let unit_truth = HashMap::from([(
            "pricing/apply_discount".to_string(),
            BenchmarkUnitTruth {
                carrier_status: BenchmarkCarrierStatus::Valid,
                semantic_support_status: BenchmarkSemanticSupportStatus::Supported,
                passport_path: Some("apply_discount.spec.passport.json".to_string()),
            },
        )]);

        let projections = project_benchmarks(
            &positive_registry(),
            BenchmarkProjectionInput {
                repo_root: repo_root.path(),
                scope_path: &repo_root.path().join("examples/ecommerce/units/pricing"),
                specs: &[apply_discount, unlabeled],
                molecule_tests: &[],
                unit_truth_by_id: &unit_truth,
                molecule_truth_by_id: &HashMap::new(),
            },
        )
        .unwrap();

        let benchmark = &projections[0];
        assert_eq!(benchmark.path_scope, BenchmarkPathScope::Partial);
        assert_eq!(
            benchmark.accounting_status,
            BenchmarkAccountingStatus::PartialInvalid
        );
        assert!(benchmark.label_digest.is_none());
        assert!(benchmark.benchmark_status.is_none());
        assert!(benchmark.gate_status.is_none());
        assert_eq!(benchmark.cases.len(), 1);
        assert!(!benchmark.cases[0].counts_as_supported_positive);
    }

    #[test]
    fn benchmark_projector_keeps_companion_negative_and_reserved_benchmarks_non_green() {
        let repo_root = TempDir::new().unwrap();
        let apply_discount = loaded_spec(
            repo_root.path(),
            "examples/crosslib-app/units/pricing/apply_discount.unit.spec",
            "pricing/apply_discount",
        );
        let chain3 = loaded_spec(
            repo_root.path(),
            "examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec",
            "pricing/checkout_nested_chain3",
        );
        let registry = BenchmarkRegistry {
            benchmarks: vec![
                BenchmarkLabel {
                    id: "BENCH-CROSSLIB".to_string(),
                    kind: BenchmarkKind::CompanionNegativeProof,
                    lifecycle: BenchmarkLifecycle::Active,
                    root: "examples/crosslib-app/units".to_string(),
                    generated_root: "examples/crosslib-app/src/generated".to_string(),
                    required_molecules: vec![],
                    cases: vec![
                        BenchmarkCaseLabel {
                            case_id: "discount".to_string(),
                            carrier_id: "pricing/apply_discount".to_string(),
                            classification: BenchmarkClassification::Supported,
                        },
                        BenchmarkCaseLabel {
                            case_id: "chain3".to_string(),
                            carrier_id: "pricing/checkout_nested_chain3".to_string(),
                            classification: BenchmarkClassification::CompanionNegativeProof,
                        },
                    ],
                },
                BenchmarkLabel {
                    id: "BENCH-SERVICE".to_string(),
                    kind: BenchmarkKind::Positive,
                    lifecycle: BenchmarkLifecycle::Reserved,
                    root: "examples/service/units".to_string(),
                    generated_root: "examples/service/src/generated".to_string(),
                    required_molecules: vec![],
                    cases: vec![],
                },
            ],
        };
        let unit_truth = HashMap::from([
            (
                "pricing/apply_discount".to_string(),
                BenchmarkUnitTruth {
                    carrier_status: BenchmarkCarrierStatus::Valid,
                    semantic_support_status: BenchmarkSemanticSupportStatus::Supported,
                    passport_path: Some("apply_discount.spec.passport.json".to_string()),
                },
            ),
            (
                "pricing/checkout_nested_chain3".to_string(),
                BenchmarkUnitTruth {
                    carrier_status: BenchmarkCarrierStatus::Valid,
                    semantic_support_status: BenchmarkSemanticSupportStatus::Unsupported,
                    passport_path: Some("checkout_nested_chain3.spec.passport.json".to_string()),
                },
            ),
        ]);

        let projections = project_benchmarks(
            &registry,
            BenchmarkProjectionInput {
                repo_root: repo_root.path(),
                scope_path: repo_root.path(),
                specs: &[apply_discount, chain3],
                molecule_tests: &[],
                unit_truth_by_id: &unit_truth,
                molecule_truth_by_id: &HashMap::new(),
            },
        )
        .unwrap();

        assert_eq!(projections.len(), 2);
        let companion = projections
            .iter()
            .find(|projection| projection.id == "BENCH-CROSSLIB")
            .unwrap();
        assert_eq!(companion.benchmark_status, Some(BenchmarkStatus::Passing));
        assert_eq!(
            companion.gate_status,
            Some(BenchmarkGateStatus::NotApplicable)
        );
        assert!(
            companion
                .cases
                .iter()
                .all(|case| !case.counts_as_supported_positive)
        );

        let reserved = projections
            .iter()
            .find(|projection| projection.id == "BENCH-SERVICE")
            .unwrap();
        assert_eq!(
            reserved.accounting_status,
            BenchmarkAccountingStatus::ReservedMissingCases
        );
        assert_eq!(reserved.benchmark_status, Some(BenchmarkStatus::Reserved));
        assert_eq!(reserved.gate_status, Some(BenchmarkGateStatus::Reserved));
        assert!(reserved.cases.is_empty());
        assert!(reserved.required_molecule_proofs.is_empty());
    }
}
