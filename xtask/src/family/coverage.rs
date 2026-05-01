use crate::XtaskError;
use crate::family::inventory::{collect_inventory, inventory_sha256_hex, render_snapshot_bytes};
use crate::family::paths::{
    FAMILY_COVERAGE_LATEST_PATH, FAMILY_PROMOTION_INVENTORY_DIR, M27_CORPUS_MANIFEST_PATH,
    path_is_semantic_family_fixture, validate_existing_relative_path, validate_repo_relative_path,
    write_bytes_atomically,
};
use crate::family::promotion_artifacts::{
    CandidateStatus, CorpusSourceEntry, FamilyCoverageArtifact, FamilyCoverageEntry,
    FunctionCoverageTotals, NonFunctionCoverageTotals, PromotionArtifactKind, SourceKind,
    UnsupportedClusterEntry,
};
use serde::Deserialize;
use serde::Serialize;
use sha2::{Digest, Sha256};
use spec_core::loader::load_directory_report_bounded;
use spec_core::semantic_review::{
    EvaluatorScope, SemanticReviewContext, SemanticSupportStatus, UnsupportedFunctionReasonCode,
    evaluate_semantic_review_with_context, unsupported_function_shape_fingerprint_with_context,
};
use spec_core::types::LoadedSpec;
use spec_core::types::UnitKind;
use spec_core::validator::validate_full;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub(crate) struct CoverageRunOutput {
    pub artifact: FamilyCoverageArtifact,
    pub latest_bytes: Vec<u8>,
    pub latest_path: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PendingCoverageOutput {
    pub artifact: FamilyCoverageArtifact,
    pub latest_bytes: Vec<u8>,
    inventory_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusManifest {
    schema_version: u64,
    target_language: String,
    target_lane: String,
    sources: Vec<ManifestSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestSource {
    id: String,
    path: String,
    kind: SourceKind,
    counts_toward_recommendation: bool,
    note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CorpusBucket {
    UnsupportedNearMiss,
    UnderSpecified,
    Drift,
    AlignedOrReal,
}

#[derive(Debug, Clone)]
struct LoadedSource {
    source: ManifestSource,
    unit_count: usize,
    specs: Vec<LoadedSpec>,
}

#[derive(Debug, Clone)]
struct LoadedUnitContext {
    source_id: String,
    source_kind: SourceKind,
    counts_toward_recommendation: bool,
    bucket: CorpusBucket,
}

#[derive(Debug, Default)]
struct FamilyCoverageAccum {
    unit_ids: BTreeSet<String>,
    source_ids: BTreeSet<String>,
}

#[derive(Debug)]
struct UnsupportedClusterAccum {
    representative_unit_ids: BTreeSet<String>,
    source_ids: BTreeSet<String>,
    promotion_relevant_source_ids: BTreeSet<String>,
    real_example_hits: usize,
    promotion_relevant_regression_hits: usize,
    boundary_only_hits: usize,
    overlap_family: String,
}

pub(crate) fn run(workspace_root: &Path, format: &str) -> Result<(), XtaskError> {
    if format != "json" {
        return Err(XtaskError::InvalidInput(format!(
            "family coverage only supports `--format json`, found `{format}`"
        )));
    }

    let output = collect_and_write_latest(workspace_root)?;
    let mut stdout = io::stdout().lock();
    stdout.write_all(&output.latest_bytes).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write coverage output: {error}"))
    })?;
    stdout.flush().map_err(|error| {
        XtaskError::WriteFailure(format!("failed to flush coverage output: {error}"))
    })
}

pub(crate) fn collect_and_write_latest(
    workspace_root: &Path,
) -> Result<CoverageRunOutput, XtaskError> {
    let output = collect_latest(workspace_root)?;
    write_latest(workspace_root, &output)
}

pub(crate) fn collect_latest(workspace_root: &Path) -> Result<PendingCoverageOutput, XtaskError> {
    let generated_at = current_timestamp_rfc3339()?;
    let inventory = collect_inventory(workspace_root)?;
    let inventory_bytes = render_snapshot_bytes(workspace_root)?;
    let inventory_path = format!(
        "{}/{}.json",
        FAMILY_PROMOTION_INVENTORY_DIR,
        fresh_artifact_token()
    );
    let inventory_sha = inventory_sha256_hex(&inventory_bytes);

    let (_manifest, manifest_bytes, loaded_sources) = load_manifest_and_specs(workspace_root)?;
    let manifest_sha = sha256_hex(&manifest_bytes);

    let mut function_coverage = FunctionCoverageTotals {
        total_units: 0,
        promoted_family_units: 0,
        supported_unpromoted_family_units: 0,
        unsupported_function_units: 0,
    };
    let mut non_function_coverage = NonFunctionCoverageTotals {
        total_units: 0,
        supported_sum_units: 0,
        supported_data_units: 0,
        other_units: 0,
    };
    let promoted_families = inventory
        .promoted_families
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut family_coverage = BTreeMap::<String, FamilyCoverageAccum>::new();
    let mut unsupported_clusters =
        BTreeMap::<(UnsupportedFunctionReasonCode, String), UnsupportedClusterAccum>::new();

    for loaded_source in &loaded_sources {
        let specs_by_id = loaded_source
            .specs
            .iter()
            .map(|spec| (spec.spec.id.clone(), spec.clone()))
            .collect::<HashMap<_, _>>();
        if specs_by_id.len() != loaded_source.specs.len() {
            return Err(XtaskError::InvalidInput(format!(
                "corpus source `{}` contains duplicate unit ids",
                loaded_source.source.id
            )));
        }
        let review_context = SemanticReviewContext::new(&specs_by_id);

        for spec in &loaded_source.specs {
            let unit_context = LoadedUnitContext {
                source_id: loaded_source.source.id.clone(),
                source_kind: loaded_source.source.kind,
                counts_toward_recommendation: loaded_source.source.counts_toward_recommendation,
                bucket: bucket_for_spec_path(&spec.source.file_path),
            };
            let review =
                evaluate_semantic_review_with_context(spec, &review_context).ok_or_else(|| {
                    XtaskError::InvalidInput(format!(
                        "semantic review was unavailable for corpus unit `{}` in source `{}`",
                        spec.spec.id, loaded_source.source.id
                    ))
                })?;
            let unit_kind = spec.spec.unit_kind().map_err(|message| {
                XtaskError::InvalidInput(format!(
                    "unit `{}` is not semantically valid for M27 coverage: {message}",
                    spec.spec.id
                ))
            })?;
            let qualified_unit_id = qualify_unit_id(&loaded_source.source.id, &spec.spec.id);

            match unit_kind {
                UnitKind::Function => {
                    function_coverage.total_units += 1;
                    if review.effective_support_status() == SemanticSupportStatus::Supported
                        && review.evaluator_scope == EvaluatorScope::SupportedFunctionSurface
                    {
                        if promoted_families.contains(&review.compatibility_key) {
                            function_coverage.promoted_family_units += 1;
                        } else {
                            function_coverage.supported_unpromoted_family_units += 1;
                        }
                        let entry = family_coverage
                            .entry(review.compatibility_key.clone())
                            .or_default();
                        entry.unit_ids.insert(qualified_unit_id);
                        entry.source_ids.insert(unit_context.source_id.clone());
                    } else {
                        function_coverage.unsupported_function_units += 1;
                        let reason_code =
                            review.unsupported_reason_codes.first().copied().unwrap_or(
                                UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
                            );
                        let shape_fingerprint = unsupported_function_shape_fingerprint_with_context(
                            spec,
                            &review_context,
                        )
                        .ok_or_else(|| {
                            XtaskError::InvalidInput(format!(
                                "unsupported function `{}` did not produce a stable shape fingerprint",
                                spec.spec.id
                            ))
                        })?;
                        let key = (reason_code, shape_fingerprint.clone());
                        let cluster = unsupported_clusters.entry(key).or_insert_with(|| {
                            UnsupportedClusterAccum {
                                representative_unit_ids: BTreeSet::new(),
                                source_ids: BTreeSet::new(),
                                promotion_relevant_source_ids: BTreeSet::new(),
                                real_example_hits: 0,
                                promotion_relevant_regression_hits: 0,
                                boundary_only_hits: 0,
                                overlap_family: overlap_family_for_cluster(
                                    reason_code,
                                    &shape_fingerprint,
                                ),
                            }
                        });
                        cluster.representative_unit_ids.insert(qualified_unit_id);
                        cluster.source_ids.insert(unit_context.source_id.clone());
                        match leverage_bucket(&unit_context) {
                            LeverageBucket::RealExample => cluster.real_example_hits += 1,
                            LeverageBucket::PromotionRelevantRegression => {
                                cluster.promotion_relevant_regression_hits += 1;
                                cluster
                                    .promotion_relevant_source_ids
                                    .insert(unit_context.source_id.clone());
                            }
                            LeverageBucket::BoundaryOnly => cluster.boundary_only_hits += 1,
                        }
                    }
                }
                UnitKind::Data | UnitKind::Sum => {
                    non_function_coverage.total_units += 1;
                    if review.effective_support_status() == SemanticSupportStatus::Supported {
                        match review.evaluator_scope {
                            EvaluatorScope::SupportedSumSurface => {
                                non_function_coverage.supported_sum_units += 1
                            }
                            EvaluatorScope::SupportedDataSurface => {
                                non_function_coverage.supported_data_units += 1
                            }
                            _ => non_function_coverage.other_units += 1,
                        }
                    } else {
                        non_function_coverage.other_units += 1;
                    }
                }
            }
        }
    }

    let sources = loaded_sources
        .into_iter()
        .map(|loaded| CorpusSourceEntry {
            id: loaded.source.id,
            path: loaded.source.path,
            kind: loaded.source.kind,
            counts_toward_recommendation: loaded.source.counts_toward_recommendation,
            note: loaded.source.note,
            unit_count: loaded.unit_count,
        })
        .collect::<Vec<_>>();

    let family_coverage = family_coverage
        .into_iter()
        .map(|(family, accum)| FamilyCoverageEntry {
            unit_count: accum.unit_ids.len(),
            family,
            unit_ids: accum.unit_ids.into_iter().collect(),
            source_ids: accum.source_ids.into_iter().collect(),
        })
        .collect::<Vec<_>>();

    let unsupported_clusters = unsupported_clusters
        .into_iter()
        .map(
            |((reason_code, shape_fingerprint), accum)| UnsupportedClusterEntry {
                cluster_id: cluster_id(reason_code, &shape_fingerprint),
                reason_code,
                shape_fingerprint,
                representative_unit_ids: accum.representative_unit_ids.into_iter().collect(),
                source_ids: accum.source_ids.into_iter().collect(),
                real_example_hits: accum.real_example_hits,
                promotion_relevant_regression_hits: accum.promotion_relevant_regression_hits,
                boundary_only_hits: accum.boundary_only_hits,
                overlap_family: accum.overlap_family,
                candidate_status: classify_candidate_status(
                    accum.real_example_hits,
                    accum.promotion_relevant_regression_hits,
                    accum.boundary_only_hits,
                    accum.promotion_relevant_source_ids.len(),
                ),
            },
        )
        .collect::<Vec<_>>();

    let artifact = FamilyCoverageArtifact {
        schema_version: 1,
        artifact_kind: PromotionArtifactKind::FamilyCoverageSnapshot,
        generated_at,
        inventory_path,
        inventory_sha256: inventory_sha,
        corpus_manifest_path: M27_CORPUS_MANIFEST_PATH.to_string(),
        corpus_manifest_sha256: manifest_sha,
        sources,
        function_coverage,
        non_function_coverage,
        family_coverage,
        unsupported_clusters,
    };
    let latest_bytes = render_json_bytes(&artifact)?;
    Ok(PendingCoverageOutput {
        artifact,
        latest_bytes,
        inventory_bytes,
    })
}

pub(crate) fn write_latest(
    workspace_root: &Path,
    output: &PendingCoverageOutput,
) -> Result<CoverageRunOutput, XtaskError> {
    write_bytes_atomically(
        &workspace_root.join(&output.artifact.inventory_path),
        &output.inventory_bytes,
    )?;
    let latest_path = FAMILY_COVERAGE_LATEST_PATH.to_string();
    write_bytes_atomically(&workspace_root.join(&latest_path), &output.latest_bytes)?;
    Ok(CoverageRunOutput {
        artifact: output.artifact.clone(),
        latest_bytes: output.latest_bytes.clone(),
        latest_path,
    })
}

pub(crate) fn normalized_for_recommend_determinism(
    artifact: &FamilyCoverageArtifact,
) -> FamilyCoverageArtifact {
    let mut normalized = artifact.clone();
    normalized.generated_at.clear();
    normalized.inventory_path.clear();
    normalized.inventory_sha256.clear();
    normalized
}

pub(crate) fn current_timestamp_rfc3339() -> Result<String, XtaskError> {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map_err(|error| XtaskError::WriteFailure(format!("failed to run `date`: {error}")))?;
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        return Err(XtaskError::WriteFailure(
            "failed to produce a UTC timestamp".to_string(),
        ));
    }
    Ok(value)
}

pub(crate) fn render_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, XtaskError> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| XtaskError::WriteFailure(format!("failed to serialize JSON: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn load_manifest_and_specs(
    workspace_root: &Path,
) -> Result<(CorpusManifest, Vec<u8>, Vec<LoadedSource>), XtaskError> {
    let manifest_relative =
        validate_repo_relative_path(M27_CORPUS_MANIFEST_PATH, "corpus manifest path")?;
    let manifest_path = validate_existing_relative_path(
        workspace_root,
        &manifest_relative,
        "corpus manifest path",
    )?;
    let manifest_bytes = fs::read(&manifest_path).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to read corpus manifest `{}`: {error}",
            manifest_path.display()
        ))
    })?;
    let manifest_text = std::str::from_utf8(&manifest_bytes).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "corpus manifest `{}` must be valid UTF-8: {error}",
            manifest_path.display()
        ))
    })?;
    let manifest = toml::from_str::<CorpusManifest>(manifest_text).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to parse `{}`: {error}",
            manifest_path.display()
        ))
    })?;
    validate_manifest_header(&manifest)?;

    let mut seen_source_ids = BTreeSet::new();
    let mut loaded_sources = Vec::new();
    for source in &manifest.sources {
        if source.id.trim().is_empty()
            || source.note.trim().is_empty()
            || source.note.contains('\n')
        {
            return Err(XtaskError::InvalidInput(
                "corpus manifest sources require non-empty single-line id and note".to_string(),
            ));
        }
        if !seen_source_ids.insert(source.id.clone()) {
            return Err(XtaskError::InvalidInput(format!(
                "corpus manifest source id `{}` must be unique",
                source.id
            )));
        }
        let relative_path = validate_repo_relative_path(&source.path, "corpus source path")?;
        if path_is_semantic_family_fixture(&relative_path) {
            return Err(XtaskError::InvalidInput(format!(
                "corpus source path `{}` must not reference semantic-family packet fixtures",
                source.path
            )));
        }
        let absolute_path =
            validate_existing_relative_path(workspace_root, &relative_path, "corpus source path")?;
        let report =
            load_directory_report_bounded(&absolute_path, workspace_root).map_err(|error| {
                XtaskError::InvalidInput(format!(
                    "failed to scan corpus source `{}`: {error}",
                    source.id
                ))
            })?;
        if let Some(error) = report.errors.first() {
            return Err(XtaskError::InvalidInput(format!(
                "corpus source `{}` contains an invalid `.unit.spec`: {error}",
                source.id
            )));
        }
        if report.specs.is_empty() {
            return Err(XtaskError::InvalidInput(format!(
                "corpus source `{}` must contain at least one `.unit.spec`",
                source.id
            )));
        }

        for spec in &report.specs {
            validate_full(spec).map_err(|error| {
                XtaskError::InvalidInput(format!(
                    "corpus source `{}` failed validation for `{}`: {error}",
                    source.id, spec.source.file_path
                ))
            })?;
        }

        loaded_sources.push(LoadedSource {
            source: source.clone(),
            unit_count: report.specs.len(),
            specs: report.specs,
        });
    }

    Ok((manifest, manifest_bytes, loaded_sources))
}

fn validate_manifest_header(manifest: &CorpusManifest) -> Result<(), XtaskError> {
    if manifest.schema_version != 1 {
        return Err(XtaskError::InvalidInput(format!(
            "corpus manifest schema_version must be 1, found {}",
            manifest.schema_version
        )));
    }
    if manifest.target_language != "rust" {
        return Err(XtaskError::InvalidInput(format!(
            "corpus manifest target_language must be `rust`, found `{}`",
            manifest.target_language
        )));
    }
    if manifest.target_lane != "function" {
        return Err(XtaskError::InvalidInput(format!(
            "corpus manifest target_lane must be `function`, found `{}`",
            manifest.target_lane
        )));
    }
    if manifest.sources.is_empty() {
        return Err(XtaskError::InvalidInput(
            "corpus manifest must include at least one source".to_string(),
        ));
    }
    Ok(())
}

fn leverage_bucket(context: &LoadedUnitContext) -> LeverageBucket {
    if !context.counts_toward_recommendation {
        return LeverageBucket::BoundaryOnly;
    }

    match context.source_kind {
        SourceKind::RealExample => LeverageBucket::RealExample,
        SourceKind::RegressionUnsupported => match context.bucket {
            CorpusBucket::UnsupportedNearMiss => LeverageBucket::BoundaryOnly,
            CorpusBucket::UnderSpecified | CorpusBucket::Drift | CorpusBucket::AlignedOrReal => {
                LeverageBucket::PromotionRelevantRegression
            }
        },
        SourceKind::ProofOnly => LeverageBucket::BoundaryOnly,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeverageBucket {
    RealExample,
    PromotionRelevantRegression,
    BoundaryOnly,
}

fn classify_candidate_status(
    real_example_hits: usize,
    promotion_relevant_regression_hits: usize,
    boundary_only_hits: usize,
    promotion_relevant_source_count: usize,
) -> CandidateStatus {
    if real_example_hits == 0 && promotion_relevant_regression_hits == 0 && boundary_only_hits > 0 {
        CandidateStatus::BoundaryOnly
    } else if real_example_hits == 0 && promotion_relevant_regression_hits <= 1 {
        CandidateStatus::InsufficientEvidence
    } else if real_example_hits == 0 && promotion_relevant_source_count == 1 {
        CandidateStatus::LowValue
    } else {
        CandidateStatus::Rankable
    }
}

fn overlap_family_for_cluster(
    reason_code: UnsupportedFunctionReasonCode,
    shape_fingerprint: &str,
) -> String {
    if matches!(
        reason_code,
        UnsupportedFunctionReasonCode::UnsupportedArithmeticShape
    ) || shape_fingerprint.contains("\"authored_body_kind\":\"arithmetic_like\"")
    {
        "function.arithmetic_leaf.monotone_*".to_string()
    } else if matches!(
        reason_code,
        UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape
            | UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression
            | UnsupportedFunctionReasonCode::UnsupportedDepTopology
    ) || shape_fingerprint.contains("\"authored_body_kind\":\"wrapper_like\"")
    {
        "function.wrapper.pipeline*".to_string()
    } else {
        "unknown".to_string()
    }
}

fn bucket_for_spec_path(path: &str) -> CorpusBucket {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if file_name.ends_with("_unsupported_near_miss.unit.spec") {
        CorpusBucket::UnsupportedNearMiss
    } else if file_name.ends_with("_under_specified.unit.spec") {
        CorpusBucket::UnderSpecified
    } else if file_name.ends_with("_drift.unit.spec") {
        CorpusBucket::Drift
    } else {
        CorpusBucket::AlignedOrReal
    }
}

fn cluster_id(reason_code: UnsupportedFunctionReasonCode, shape_fingerprint: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{reason_code:?}:{shape_fingerprint}").as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    format!(
        "{}-{}",
        serde_json::to_string(&reason_code)
            .expect("reason code JSON serialization should succeed")
            .trim_matches('"'),
        &digest[..12]
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn fresh_artifact_token() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after UNIX_EPOCH")
        .as_nanos()
        .to_string()
}

fn qualify_unit_id(source_id: &str, unit_id: &str) -> String {
    format!("{source_id}::{unit_id}")
}
