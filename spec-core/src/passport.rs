//! Passport generation for spec units.
//!
//! A passport is a static knowledge artifact derived from a LoadedSpec. One
//! `.spec.passport.json` file is emitted per unit, co-located with its
//! `.unit.spec` source file. Passports are derived artifacts (gitignored) and
//! are written atomically only after all generation succeeds.

use crate::escape_hatch::{EscapeHatchGate, evaluate_escape_hatch_gate};
use crate::generator::write_generated_file;
use crate::graph::top_level_deps;
use crate::molecule_evidence::{MoleculeEvidence, molecule_evidence_is_current_pass};
use crate::types::{
    AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredMethod, AuthoredSumShape,
    Contract, Intent, LoadedMoleculeTest, LoadedSpec, UnitKind,
};
use crate::{AUTHORED_SPEC_VERSION, Result, SpecError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// A single contract input parameter in the passport JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// Contract section of the passport.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportContract {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<PassportInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invariants: Vec<String>,
}

/// A local test entry in the passport.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportLocalTest {
    pub id: String,
    pub expect: String,
}

/// Observed runtime result for one declared local test.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportTestResult {
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Minimal artifact provenance for machine-readable outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactProvenance {
    pub git_commit_sha: String,
}

/// Observed runtime evidence captured from the last `spec test` run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportEvidence {
    pub build_status: String,
    pub test_results: Vec<PassportTestResult>,
    pub observed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ArtifactProvenance>,
}

/// Split freshness metadata for authored semantics vs backend execution details.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportFreshnessSnapshot {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_sha256_digest"
    )]
    pub authored_truth_digest: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_sha256_digest"
    )]
    pub backend_execution_digest: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FreshnessStatus {
    Fresh,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportFreshness {
    #[serde(flatten)]
    pub snapshot: PassportFreshnessSnapshot,
    pub authored_truth_status: FreshnessStatus,
    pub backend_execution_status: FreshnessStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedPassportTruth {
    pub freshness: Option<PassportFreshness>,
    pub markers: Option<Vec<PassportMarker>>,
    pub proof_coverage: Option<Vec<PassportProofCoverage>>,
    pub escape_hatch_gate: Option<EscapeHatchGate>,
}

pub struct PassportProjectionContext<'a> {
    pub molecule_tests: &'a [LoadedMoleculeTest],
    pub molecule_evidence_by_id: &'a HashMap<String, MoleculeEvidence>,
    pub specs_by_id: &'a HashMap<String, LoadedSpec>,
}

/// Explicit marker for backend-only escape hatches on seam units.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportMarker {
    pub id: PassportMarkerId,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportMarkerId {
    MethodLoweringRustBody,
    BackendRustDerives,
}

/// Additive proof-coverage metadata for seam-localized review surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportProofCoverage {
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surfaces: Vec<ProofSurface>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProofSurface {
    Atom,
    Molecule,
    ImplicitOnly,
}

/// The full passport document for one unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Passport {
    pub spec_version: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<PassportContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AuthoredDataShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<AuthoredSumShape>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constructors: Vec<AuthoredConstructor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<AuthoredMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<AuthoredBackends>,
    pub deps: Vec<String>,
    pub local_tests: Vec<PassportLocalTest>,
    pub generated_at: String,
    pub source_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PassportEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_anchor: Option<PassportFreshnessSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<PassportFreshness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<PassportMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_coverage: Option<Vec<PassportProofCoverage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escape_hatch_gate: Option<EscapeHatchGate>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_sha256_digest"
    )]
    pub contract_hash: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassportBuildMetadata {
    pub evidence: Option<PassportEvidence>,
    pub freshness_anchor: Option<PassportFreshnessSnapshot>,
    pub contract_hash: Option<String>,
    pub freshness: Option<PassportFreshness>,
    pub markers: Option<Vec<PassportMarker>>,
    pub proof_coverage: Option<Vec<PassportProofCoverage>>,
}

/// Build a Passport from a LoadedSpec.
///
/// `generated_at` is injected so all passports in one run share an identical
/// timestamp (batch consistency).
pub fn build_passport(spec: &LoadedSpec, generated_at: &str) -> Passport {
    build_passport_with_evidence(spec, generated_at, None, None)
}

/// Build a Passport from a LoadedSpec and optional observed evidence.
pub fn build_passport_with_evidence(
    spec: &LoadedSpec,
    generated_at: &str,
    evidence: Option<PassportEvidence>,
    contract_hash: Option<String>,
) -> Passport {
    let has_evidence = evidence.is_some();
    let freshness_anchor = passport_freshness_anchor_for_write(spec, has_evidence);
    let freshness = resolve_passport_freshness_with_anchor(
        spec,
        freshness_anchor.as_ref(),
        contract_hash.as_deref(),
        None,
    );
    build_passport_with_metadata(
        spec,
        generated_at,
        PassportBuildMetadata {
            evidence,
            freshness_anchor,
            contract_hash,
            freshness,
            markers: compute_passport_markers(spec),
            proof_coverage: default_passport_proof_coverage(spec),
        },
    )
}

/// Build a Passport without minting any new proof state.
///
/// This is for non-test callers such as `spec build` / `spec generate`. They
/// may refresh authored metadata, markers, and proof-coverage projections, but
/// they must preserve the last observed proof anchor written by `spec test`.
pub fn build_passport_preserving_proof_state(
    spec: &LoadedSpec,
    generated_at: &str,
    existing: Option<&Passport>,
    contract_hash: Option<String>,
) -> Passport {
    let freshness_anchor = preserved_freshness_anchor(existing);
    let freshness = resolve_passport_freshness_with_anchor(
        spec,
        freshness_anchor.as_ref(),
        contract_hash.as_deref(),
        None,
    );
    build_passport_with_metadata(
        spec,
        generated_at,
        PassportBuildMetadata {
            evidence: existing.and_then(|passport| passport.evidence.clone()),
            freshness_anchor,
            contract_hash,
            freshness,
            markers: compute_passport_markers(spec),
            proof_coverage: default_passport_proof_coverage(spec),
        },
    )
}

pub fn build_passport_with_metadata(
    spec: &LoadedSpec,
    generated_at: &str,
    metadata: PassportBuildMetadata,
) -> Passport {
    let PassportBuildMetadata {
        evidence,
        freshness_anchor,
        contract_hash,
        freshness,
        markers,
        proof_coverage,
    } = metadata;
    let is_seam = matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum));
    let contract = spec.spec.contract.as_ref().map(|c| PassportContract {
        inputs: c
            .inputs
            .as_ref()
            .map(|m| {
                m.iter()
                    .map(|(name, type_str)| PassportInput {
                        name: name.clone(),
                        type_: type_str.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        returns: c.returns.clone(),
        invariants: c.invariants.clone(),
    });

    Passport {
        spec_version: spec
            .spec
            .spec_version
            .clone()
            .unwrap_or_else(|| AUTHORED_SPEC_VERSION.to_string()),
        id: spec.spec.id.clone(),
        kind: is_seam.then(|| spec.spec.kind.clone()),
        intent: spec.spec.intent.why.clone(),
        contract,
        data: spec.spec.extensions.data.clone(),
        sum: spec.spec.extensions.sum.clone(),
        constructors: spec.spec.extensions.constructors.clone(),
        methods: spec.spec.extensions.methods.clone(),
        backends: spec.spec.extensions.backends.clone(),
        deps: top_level_deps(spec),
        local_tests: spec
            .spec
            .local_tests
            .iter()
            .map(|t| PassportLocalTest {
                id: t.id.clone(),
                expect: t.expect.clone(),
            })
            .collect(),
        generated_at: generated_at.to_string(),
        source_file: spec.source.file_path.clone(),
        evidence,
        freshness_anchor,
        freshness,
        markers,
        proof_coverage,
        escape_hatch_gate: None,
        contract_hash,
    }
}

#[derive(Serialize)]
struct DataSeamAuthoredTruthSurface<'a> {
    intent: &'a str,
    data: Option<&'a AuthoredDataShape>,
    constructors: &'a [AuthoredConstructor],
    methods: Vec<AuthoredMethodTruthSurface<'a>>,
}

#[derive(Serialize)]
struct SumSeamAuthoredTruthSurface<'a> {
    intent: &'a str,
    sum: Option<&'a AuthoredSumShape>,
    constructors: &'a [AuthoredConstructor],
    methods: Vec<AuthoredMethodTruthSurface<'a>>,
}

#[derive(Serialize)]
struct AuthoredMethodTruthSurface<'a> {
    id: &'a str,
    intent: &'a Intent,
    receiver: &'a str,
    contract: Option<&'a Contract>,
    deps: &'a [String],
}

#[derive(Serialize)]
struct SeamBackendExecutionSurface<'a> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    method_lowering_rust_bodies: Vec<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    rust_derives: Vec<&'a str>,
}

/// Compute SHA-256 of the unit's top-level truth surface.
///
/// Function units hash only the legacy top-level `contract` surface.
/// Data/sum seams hash the shared authored seam surface without backend-only
/// lowering or derive details.
pub fn compute_contract_hash(spec: &LoadedSpec) -> Option<String> {
    compute_authored_truth_digest(spec)
}

/// Compute the M14 digest snapshot for one unit.
pub fn compute_passport_freshness_snapshot(spec: &LoadedSpec) -> Option<PassportFreshnessSnapshot> {
    Some(PassportFreshnessSnapshot {
        authored_truth_digest: compute_authored_truth_digest(spec),
        backend_execution_digest: compute_backend_execution_digest(spec),
    })
}

/// Compute the shared authored-truth digest for one unit.
pub fn compute_authored_truth_digest(spec: &LoadedSpec) -> Option<String> {
    let json = match spec.spec.unit_kind() {
        Ok(UnitKind::Data) => serde_json::to_string(&DataSeamAuthoredTruthSurface {
            intent: &spec.spec.intent.why,
            data: spec.spec.extensions.data.as_ref(),
            constructors: &spec.spec.extensions.constructors,
            methods: authored_method_truth_surfaces(&spec.spec.extensions.methods),
        })
        .expect("data seam authored-truth serialization cannot fail for well-formed spec"),
        Ok(UnitKind::Sum) => serde_json::to_string(&SumSeamAuthoredTruthSurface {
            intent: &spec.spec.intent.why,
            sum: spec.spec.extensions.sum.as_ref(),
            constructors: &spec.spec.extensions.constructors,
            methods: authored_method_truth_surfaces(&spec.spec.extensions.methods),
        })
        .expect("sum seam authored-truth serialization cannot fail for well-formed spec"),
        _ => {
            let contract = spec.spec.contract.as_ref()?;
            serde_json::to_string(contract)
                .expect("contract serialization cannot fail for well-formed spec")
        }
    };

    Some(sha256_digest(&json))
}

/// Compute the backend-only execution digest for seam escape hatches.
pub fn compute_backend_execution_digest(spec: &LoadedSpec) -> Option<String> {
    let seam_kind = matches!(spec.spec.unit_kind(), Ok(UnitKind::Data | UnitKind::Sum));
    if !seam_kind {
        return None;
    }

    let method_lowering_rust_bodies: Vec<&str> = spec
        .spec
        .extensions
        .methods
        .iter()
        .filter_map(|method| {
            method
                .lowering
                .as_ref()
                .and_then(|lowering| lowering.rust.as_ref())
                .map(|rust| rust.body.as_str())
        })
        .collect();
    let rust_derives: Vec<&str> = spec
        .spec
        .extensions
        .backends
        .as_ref()
        .and_then(|backends| backends.rust.as_ref())
        .map(|rust| rust.derives.iter().map(String::as_str).collect())
        .unwrap_or_default();

    if method_lowering_rust_bodies.is_empty() && rust_derives.is_empty() {
        return None;
    }

    let json = serde_json::to_string(&SeamBackendExecutionSurface {
        method_lowering_rust_bodies,
        rust_derives,
    })
    .expect("seam backend-execution serialization cannot fail for well-formed spec");
    Some(sha256_digest(&json))
}

/// Compute explicit backend-only markers for seam units.
pub fn compute_passport_markers(spec: &LoadedSpec) -> Option<Vec<PassportMarker>> {
    match spec.spec.unit_kind() {
        Ok(UnitKind::Data | UnitKind::Sum) => {
            let mut markers = Vec::new();
            if spec
                .spec
                .extensions
                .backends
                .as_ref()
                .and_then(|backends| backends.rust.as_ref())
                .map(|rust| !rust.derives.is_empty())
                .unwrap_or(false)
            {
                markers.push(PassportMarker {
                    id: PassportMarkerId::BackendRustDerives,
                    path: "backends.rust.derives".to_string(),
                });
            }

            for method in &spec.spec.extensions.methods {
                if method
                    .lowering
                    .as_ref()
                    .and_then(|lowering| lowering.rust.as_ref())
                    .is_some()
                {
                    markers.push(PassportMarker {
                        id: PassportMarkerId::MethodLoweringRustBody,
                        path: format!("methods.{}.lowering.rust.body", method.id),
                    });
                }
            }

            if markers.is_empty() {
                None
            } else {
                Some(markers)
            }
        }
        _ => None,
    }
}

/// Default proof-coverage hook for seam-localized review metadata.
pub fn default_passport_proof_coverage(spec: &LoadedSpec) -> Option<Vec<PassportProofCoverage>> {
    canonical_discount_policy_proof_coverage(spec)
}

pub fn normalize_proof_surfaces(mut surfaces: Vec<ProofSurface>) -> Vec<ProofSurface> {
    surfaces.sort_by_key(|surface| match surface {
        ProofSurface::Atom => 0,
        ProofSurface::Molecule => 1,
        ProofSurface::ImplicitOnly => 2,
    });
    surfaces.dedup();
    surfaces
}

pub fn passport_freshness_for_write(
    spec: &LoadedSpec,
    has_evidence: bool,
) -> Option<PassportFreshness> {
    resolve_passport_freshness_with_anchor(
        spec,
        passport_freshness_anchor_for_write(spec, has_evidence).as_ref(),
        None,
        None,
    )
}

pub fn resolve_passport_freshness(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
) -> Option<PassportFreshness> {
    let anchor = stored_freshness_anchor(passport);
    let legacy_authored_truth_present = passport.map(|passport| {
        passport.contract.is_some()
            || passport.data.is_some()
            || passport.sum.is_some()
            || !passport.constructors.is_empty()
            || !passport.methods.is_empty()
    });
    let legacy_contract_hash = if anchor.is_some() {
        None
    } else {
        passport.and_then(|passport| passport.contract_hash.as_deref())
    };
    resolve_passport_freshness_with_anchor(
        spec,
        anchor.as_ref(),
        legacy_contract_hash,
        legacy_authored_truth_present,
    )
}

pub fn project_passport_truth(
    spec: &LoadedSpec,
    passport: Option<&Passport>,
    context: &PassportProjectionContext<'_>,
) -> ProjectedPassportTruth {
    ProjectedPassportTruth {
        freshness: resolve_passport_freshness(spec, passport),
        markers: compute_passport_markers(spec),
        proof_coverage: project_passport_proof_coverage(spec, context),
        escape_hatch_gate: evaluate_escape_hatch_gate(
            spec,
            passport,
            context.molecule_tests,
            context.molecule_evidence_by_id,
            context.specs_by_id,
        ),
    }
}

pub fn apply_projected_passport_truth(
    passport: &mut Passport,
    projected_truth: ProjectedPassportTruth,
) {
    passport.freshness = projected_truth.freshness;
    passport.markers = projected_truth.markers;
    passport.proof_coverage = projected_truth.proof_coverage;
    passport.escape_hatch_gate = projected_truth.escape_hatch_gate;
}

fn resolve_passport_freshness_with_anchor(
    spec: &LoadedSpec,
    freshness_anchor: Option<&PassportFreshnessSnapshot>,
    legacy_contract_hash: Option<&str>,
    legacy_authored_truth_present: Option<bool>,
) -> Option<PassportFreshness> {
    let snapshot = compute_passport_freshness_snapshot(spec)?;
    let authored_truth_status = resolve_authored_truth_status(
        freshness_anchor,
        legacy_contract_hash,
        legacy_authored_truth_present,
        &snapshot,
    );
    let backend_execution_status = resolve_freshness_status(
        freshness_anchor.and_then(|anchor| anchor.backend_execution_digest.as_deref()),
        snapshot.backend_execution_digest.as_deref(),
    );
    Some(PassportFreshness {
        authored_truth_status,
        backend_execution_status,
        snapshot,
    })
}

fn resolve_authored_truth_status(
    freshness_anchor: Option<&PassportFreshnessSnapshot>,
    legacy_contract_hash: Option<&str>,
    legacy_authored_truth_present: Option<bool>,
    snapshot: &PassportFreshnessSnapshot,
) -> FreshnessStatus {
    if let Some(stored) = freshness_anchor {
        return resolve_freshness_status(
            stored.authored_truth_digest.as_deref(),
            snapshot.authored_truth_digest.as_deref(),
        );
    }

    let status = resolve_freshness_status(
        legacy_contract_hash,
        snapshot.authored_truth_digest.as_deref(),
    );
    if let (FreshnessStatus::Unknown, Some(stored_present)) =
        (status, legacy_authored_truth_present)
    {
        return resolve_freshness_presence_status(
            stored_present,
            snapshot.authored_truth_digest.is_some(),
        );
    }

    status
}

fn resolve_freshness_status(
    stored_digest: Option<&str>,
    live_digest: Option<&str>,
) -> FreshnessStatus {
    match (stored_digest, live_digest) {
        (Some(stored), Some(live)) if stored == live => FreshnessStatus::Fresh,
        (Some(_), Some(_)) | (Some(_), None) => FreshnessStatus::Stale,
        (None, _) => FreshnessStatus::Unknown,
    }
}

fn resolve_freshness_presence_status(stored_present: bool, live_present: bool) -> FreshnessStatus {
    match (stored_present, live_present) {
        (true, true) => FreshnessStatus::Unknown,
        (true, false) | (false, true) => FreshnessStatus::Stale,
        (false, false) => FreshnessStatus::Unknown,
    }
}

fn stored_freshness_anchor(passport: Option<&Passport>) -> Option<PassportFreshnessSnapshot> {
    passport
        .and_then(|passport| passport.freshness_anchor.clone())
        .or_else(|| {
            passport.and_then(|passport| {
                passport
                    .freshness
                    .as_ref()
                    .map(|freshness| freshness.snapshot.clone())
            })
        })
        .or_else(|| {
            passport.and_then(|passport| {
                passport
                    .contract_hash
                    .as_ref()
                    .map(|contract_hash| PassportFreshnessSnapshot {
                        authored_truth_digest: Some(contract_hash.clone()),
                        backend_execution_digest: None,
                    })
            })
        })
}

fn preserved_freshness_anchor(passport: Option<&Passport>) -> Option<PassportFreshnessSnapshot> {
    stored_freshness_anchor(passport)
}

fn passport_freshness_anchor_for_write(
    spec: &LoadedSpec,
    has_evidence: bool,
) -> Option<PassportFreshnessSnapshot> {
    has_evidence
        .then(|| compute_passport_freshness_snapshot(spec))
        .flatten()
}

fn project_passport_proof_coverage(
    spec: &LoadedSpec,
    context: &PassportProjectionContext<'_>,
) -> Option<Vec<PassportProofCoverage>> {
    let mut proof_coverage = default_passport_proof_coverage(spec)?;
    let molecule_surface_present = context
        .molecule_tests
        .iter()
        .filter(|test| {
            test.test
                .covers
                .iter()
                .any(|cover_id| cover_id == &spec.spec.id)
        })
        .any(|test| {
            context
                .molecule_evidence_by_id
                .get(&test.test.id)
                .is_some_and(|evidence| {
                    molecule_evidence_is_current_pass(evidence, test, context.specs_by_id)
                })
        });
    if molecule_surface_present {
        for coverage in &mut proof_coverage {
            coverage.surfaces = normalize_proof_surfaces(
                coverage
                    .surfaces
                    .iter()
                    .cloned()
                    .chain(std::iter::once(ProofSurface::Molecule))
                    .collect(),
            );
        }
    }
    Some(proof_coverage)
}

fn canonical_discount_policy_proof_coverage(
    spec: &LoadedSpec,
) -> Option<Vec<PassportProofCoverage>> {
    if spec.spec.id != "pricing/discount_policy" {
        return None;
    }

    let local_test_ids = spec
        .spec
        .local_tests
        .iter()
        .map(|test| test.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    Some(
        [
            ("variant.none", "variant_none"),
            ("variant.percentage", "variant_percentage"),
            ("variant.fixed_amount", "variant_fixed_amount"),
            (
                "behavior.fixed_amount_capped",
                "behavior_fixed_amount_capped",
            ),
        ]
        .into_iter()
        .map(|(coverage_id, local_test_id)| PassportProofCoverage {
            id: coverage_id.to_string(),
            surfaces: normalize_proof_surfaces(if local_test_ids.contains(local_test_id) {
                vec![ProofSurface::Atom]
            } else {
                vec![ProofSurface::ImplicitOnly]
            }),
        })
        .collect(),
    )
}

/// Return the passport file path for a given source `.unit.spec` path.
///
/// Example: `units/pricing/apply_tax.unit.spec` →
///          `units/pricing/apply_tax.spec.passport.json`
pub fn passport_path_for(source_path: &Path) -> Result<PathBuf> {
    let parent = source_path.parent().ok_or_else(|| SpecError::Generator {
        message: format!(
            "passport_path_for: cannot determine parent of {}",
            source_path.display()
        ),
    })?;

    let filename = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "passport_path_for: no filename in {}",
                source_path.display()
            ),
        })?;

    let stem = filename
        .strip_suffix(".unit.spec")
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "passport_path_for: path does not end with .unit.spec: {}",
                source_path.display()
            ),
        })?;

    Ok(parent.join(format!("{stem}.spec.passport.json")))
}

/// Read a passport for a given source `.unit.spec` path.
///
/// Returns `Ok(None)` when the passport file does not exist.
/// Returns `Err` when the file exists but cannot be parsed.
pub fn read_passport(source_path: &Path) -> Result<Option<Passport>> {
    let passport_path = passport_path_for(source_path)?;

    let content = match fs::read_to_string(&passport_path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };

    Ok(Some(serde_json::from_str(&content)?))
}

/// Serialize a Passport to pretty-printed JSON and write it atomically
/// co-located with the source `.unit.spec` file.
pub fn write_passport(passport: &Passport, source_file_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(passport).map_err(|e| SpecError::Generator {
        message: format!("Failed to serialize passport for '{}': {e}", passport.id),
    })?;
    let passport_path = passport_path_for(source_file_path)?;
    write_generated_file(&passport_path.display().to_string(), &json)
}

fn authored_method_truth_surfaces(
    methods: &[AuthoredMethod],
) -> Vec<AuthoredMethodTruthSurface<'_>> {
    methods
        .iter()
        .map(|method| AuthoredMethodTruthSurface {
            id: method.id.as_str(),
            intent: &method.intent,
            receiver: method.receiver.as_str(),
            contract: method.contract.as_ref(),
            deps: method.deps.as_slice(),
        })
        .collect()
}

fn sha256_digest(json: &str) -> String {
    let hash = Sha256::digest(json.as_bytes());
    format!("sha256:{}", hex::encode(hash))
}

fn deserialize_optional_sha256_digest<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let digest = Option::<String>::deserialize(deserializer)?;
    Ok(digest.filter(|hash| hash.starts_with("sha256:")))
}

/// Emit `**/*.spec.passport.json` to `<spec_root>/.gitignore` if not already
/// present. Creates the file if it does not exist; appends if the entry is
/// missing. Safe to call on every generate run (idempotent).
pub fn ensure_gitignore_entry(spec_root: &Path) -> Result<()> {
    const ENTRY: &str = "**/*.spec.passport.json";
    let gitignore_path = spec_root.join(".gitignore");

    let existing = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    // Check for the entry on any line (trim trailing whitespace per line).
    if existing.lines().any(|l| l.trim_end() == ENTRY) {
        return Ok(());
    }

    // Append the entry, ensuring a leading newline if the file is non-empty and
    // doesn't already end with a newline.
    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(ENTRY);
    content.push('\n');

    fs::write(&gitignore_path, content)?;
    Ok(())
}

/// Return an RFC 3339 UTC timestamp for the current moment (second precision).
///
/// Uses only `std::time`; no external crate dependency required.
/// Output format: `YYYY-MM-DDTHH:MM:SSZ`
pub fn rfc3339_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (year, month, day, h, m, s) = secs_to_gregorian(secs);
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Convert a Unix timestamp (seconds since epoch) to (year, month, day, hour,
/// minute, second) using the proleptic Gregorian calendar.
///
/// Algorithm: Richards (2013), "Calendrical Calculations" variant — integer
/// arithmetic only, handles leap years including 100/400-year rules.
fn secs_to_gregorian(secs: u64) -> (u32, u32, u32, u32, u32, u32) {
    let sec = (secs % 60) as u32;
    let min = ((secs / 60) % 60) as u32;
    let hour = ((secs / 3600) % 24) as u32;
    let days = secs / 86400; // days since 1970-01-01

    // Shift epoch to 1 March 0000 (simplifies leap-year arithmetic).
    // 719468 = days from 0000-03-01 to 1970-01-01
    let z = days + 719_468;
    let era = z / 146_097; // 400-year era
    let doe = z - era * 146_097; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // year of era [0, 399]
    let y = yoe + era * 400; // year (March-based)
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month of year (March = 0)
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let y = if m <= 2 { y + 1 } else { y }; // adjust year for Jan/Feb

    (y as u32, m as u32, d as u32, hour, min, sec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering, AuthoredSumShape,
        AuthoredSumVariant, Body, Contract, Intent, LocalTest, SpecSource, SpecStruct,
        UnitExtensions,
    };
    use indexmap::IndexMap;
    use tempfile::TempDir;

    fn make_loaded_spec(
        id: &str,
        file_path: &str,
        spec_version: Option<&str>,
        contract: Option<Contract>,
        deps: Vec<&str>,
        local_tests: Vec<(&str, &str)>,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: file_path.to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract,
                deps: deps.into_iter().map(String::from).collect(),
                imports: vec![],
                body: Body {
                    rust: "{ 42 }".to_string(),
                },
                local_tests: local_tests
                    .into_iter()
                    .map(|(tid, exp)| LocalTest {
                        id: tid.to_string(),
                        expect: exp.to_string(),
                    })
                    .collect(),
                links: None,
                spec_version: spec_version.map(String::from),
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    fn make_loaded_data_seam(id: &str, file_path: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: file_path.to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: vec!["legacy/ignored".to_string()],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![LocalTest {
                    id: "total_basic".to_string(),
                    expect: "CheckoutQuote::new(...).total() == expected".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![AuthoredConstructor {
                        id: "new".to_string(),
                        intent: Intent {
                            why: "Create a quote".to_string(),
                        },
                        contract: Some(Contract {
                            inputs: Some(IndexMap::from([
                                ("subtotal".to_string(), "Decimal".to_string()),
                                ("tax_rate".to_string(), "Decimal".to_string()),
                            ])),
                            returns: None,
                            invariants: vec![],
                        }),
                        initializes: IndexMap::from([
                            ("subtotal".to_string(), "subtotal".to_string()),
                            ("tax_rate".to_string(), "tax_rate".to_string()),
                        ]),
                    }],
                    methods: vec![
                        AuthoredMethod {
                            id: "discounted_subtotal".to_string(),
                            intent: Intent {
                                why: "Compute discounted subtotal".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ apply_discount(self.subtotal, Decimal::ZERO) }"
                                        .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Compute total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![
                                "pricing/apply_discount".to_string(),
                                "pricing/apply_tax".to_string(),
                            ],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ apply_tax(self.subtotal, self.tax_rate) }".to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec!["Clone".to_string(), "Debug".to_string()],
                        }),
                    }),
                    sum: None,
                },
            },
        }
    }

    fn make_loaded_sum_seam(id: &str, file_path: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: file_path.to_string(),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "sum".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: vec!["legacy/ignored".to_string()],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![LocalTest {
                    id: "label_basic".to_string(),
                    expect: "CheckoutStatus::Pending.label() == \"pending\"".to_string(),
                }],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: None,
                    sum: Some(AuthoredSumShape {
                        variants: IndexMap::from([
                            (
                                "pending".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::new(),
                                },
                            ),
                            (
                                "quoted_total".to_string(),
                                AuthoredSumVariant {
                                    fields: IndexMap::from([
                                        (
                                            "subtotal".to_string(),
                                            AuthoredField {
                                                type_: "i32".to_string(),
                                            },
                                        ),
                                        (
                                            "tax_rate".to_string(),
                                            AuthoredField {
                                                type_: "i32".to_string(),
                                            },
                                        ),
                                    ]),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![],
                    methods: vec![
                        AuthoredMethod {
                            id: "label".to_string(),
                            intent: Intent {
                                why: "Return a variant label".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("&'static str".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ \"pending\" }".to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Return a computed total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("i32".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![
                                "pricing/apply_discount".to_string(),
                                "pricing/apply_tax".to_string(),
                            ],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ 0 }".to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec![
                                "Clone".to_string(),
                                "Debug".to_string(),
                                "PartialEq".to_string(),
                            ],
                        }),
                    }),
                },
            },
        }
    }

    #[test]
    fn build_passport_full_contract() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        inputs.insert("rate".to_string(), "Decimal".to_string());
        let contract = Contract {
            inputs: Some(inputs),
            returns: Some("Decimal".to_string()),
            invariants: vec!["output >= subtotal".to_string()],
        };

        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(contract),
            vec!["money/round"],
            vec![("basic", "apply_tax(1,2) == 3")],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");

        assert_eq!(passport.spec_version, "0.3.0");
        assert_eq!(passport.id, "pricing/apply_tax");
        assert_eq!(passport.intent, "Why pricing/apply_tax");
        assert_eq!(passport.deps, vec!["money/round"]);
        assert_eq!(passport.generated_at, "2026-04-04T00:00:00Z");
        assert_eq!(passport.source_file, "units/pricing/apply_tax.unit.spec");
        assert!(passport.contract_hash.is_none());
        assert_eq!(
            passport.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&spec),
                    backend_execution_digest: None,
                },
                authored_truth_status: FreshnessStatus::Unknown,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
        assert!(passport.markers.is_none());
        assert!(passport.proof_coverage.is_none());

        let c = passport.contract.unwrap();
        assert_eq!(c.inputs.len(), 2);
        assert_eq!(c.inputs[0].name, "subtotal");
        assert_eq!(c.inputs[0].type_, "Decimal");
        assert_eq!(c.inputs[1].name, "rate");
        assert_eq!(c.inputs[1].type_, "Decimal");
        assert_eq!(c.returns, Some("Decimal".to_string()));
        assert_eq!(c.invariants, vec!["output >= subtotal"]);

        assert_eq!(passport.local_tests.len(), 1);
        assert_eq!(passport.local_tests[0].id, "basic");
        assert!(passport.evidence.is_none());
    }

    #[test]
    fn build_passport_no_contract() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            None,
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        assert!(passport.contract.is_none());
        assert_eq!(passport.spec_version, "0.3.0"); // default
        assert!(passport.deps.is_empty());
        assert!(passport.local_tests.is_empty());
        assert!(passport.evidence.is_none());
        assert!(passport.contract_hash.is_none());
        assert_eq!(
            passport.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&spec),
                    backend_execution_digest: None,
                },
                authored_truth_status: FreshnessStatus::Unknown,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
        assert!(passport.markers.is_none());
        assert!(passport.proof_coverage.is_none());
    }

    #[test]
    fn build_passport_data_seam_serializes_top_level_truth_only() {
        let spec = make_loaded_data_seam(
            "pricing/checkout_quote",
            "units/pricing/checkout_quote.unit.spec",
        );

        let passport = build_passport(&spec, "2026-04-19T00:00:00Z");

        assert_eq!(passport.kind, Some("data".to_string()));
        assert!(passport.contract.is_none());
        assert_eq!(
            passport.deps,
            vec![
                "pricing/apply_discount".to_string(),
                "pricing/apply_tax".to_string(),
            ]
        );
        assert_eq!(passport.data.unwrap().fields.len(), 2);
        assert_eq!(passport.constructors.len(), 1);
        assert_eq!(passport.methods.len(), 2);
        assert_eq!(
            passport.backends.unwrap().rust.unwrap().derives,
            vec!["Clone".to_string(), "Debug".to_string()]
        );
        assert_eq!(passport.local_tests.len(), 1);
        assert_eq!(
            passport.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&spec),
                    backend_execution_digest: compute_backend_execution_digest(&spec),
                },
                authored_truth_status: FreshnessStatus::Unknown,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
        assert_eq!(
            passport.markers,
            Some(vec![
                PassportMarker {
                    id: PassportMarkerId::BackendRustDerives,
                    path: "backends.rust.derives".to_string(),
                },
                PassportMarker {
                    id: PassportMarkerId::MethodLoweringRustBody,
                    path: "methods.discounted_subtotal.lowering.rust.body".to_string(),
                },
                PassportMarker {
                    id: PassportMarkerId::MethodLoweringRustBody,
                    path: "methods.total.lowering.rust.body".to_string(),
                },
            ])
        );
        assert!(passport.proof_coverage.is_none());
        assert!(passport.escape_hatch_gate.is_none());
    }

    #[test]
    fn build_passport_sum_seam_serializes_top_level_truth_only() {
        let spec = make_loaded_sum_seam(
            "pricing/checkout_status",
            "units/pricing/checkout_status.unit.spec",
        );

        let passport = build_passport(&spec, "2026-04-19T00:00:00Z");

        assert_eq!(passport.kind, Some("sum".to_string()));
        assert!(passport.contract.is_none());
        assert_eq!(
            passport.deps,
            vec![
                "pricing/apply_discount".to_string(),
                "pricing/apply_tax".to_string(),
            ]
        );
        let variants = passport.sum.unwrap().variants;
        assert_eq!(
            variants.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["pending", "quoted_total"]
        );
        assert_eq!(variants["quoted_total"].fields["subtotal"].type_, "i32");
        assert_eq!(passport.methods.len(), 2);
        assert_eq!(
            passport.backends.unwrap().rust.unwrap().derives,
            vec![
                "Clone".to_string(),
                "Debug".to_string(),
                "PartialEq".to_string(),
            ]
        );
        assert_eq!(passport.local_tests.len(), 1);
        assert_eq!(
            passport.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&spec),
                    backend_execution_digest: compute_backend_execution_digest(&spec),
                },
                authored_truth_status: FreshnessStatus::Unknown,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
        assert!(passport.proof_coverage.is_none());
        assert!(passport.escape_hatch_gate.is_none());
    }

    #[test]
    fn build_passport_uses_spec_version_from_unit() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "t");
        assert_eq!(passport.spec_version, "0.3.0");
    }

    #[test]
    fn build_passport_defaults_spec_version_when_absent() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            None,
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "t");
        assert_eq!(passport.spec_version, "0.3.0");
    }

    #[test]
    fn passport_path_for_standard_unit() {
        let p = passport_path_for(Path::new("units/pricing/apply_tax.unit.spec")).unwrap();
        assert_eq!(
            p,
            PathBuf::from("units/pricing/apply_tax.spec.passport.json")
        );
    }

    #[test]
    fn passport_path_for_root_level_unit() {
        let p = passport_path_for(Path::new("money/round.unit.spec")).unwrap();
        assert_eq!(p, PathBuf::from("money/round.spec.passport.json"));
    }

    #[test]
    fn passport_path_for_rejects_non_unit_spec() {
        let result = passport_path_for(Path::new("units/pricing/apply_tax.rs"));
        assert!(result.is_err());
    }

    #[test]
    fn test_contract_hash_absent_for_no_contract() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );

        assert_eq!(compute_contract_hash(&spec), None);
    }

    #[test]
    fn test_contract_hash_present_for_contract() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "Decimal".to_string());
        inputs.insert("rate".to_string(), "Decimal".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("Decimal".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            vec![],
            vec![],
        );

        let expected = {
            let contract = spec.spec.contract.as_ref().unwrap();
            let json = serde_json::to_string(contract).unwrap();
            let hash = Sha256::digest(json.as_bytes());
            format!("sha256:{}", hex::encode(hash))
        };

        assert_eq!(compute_contract_hash(&spec), Some(expected));
    }

    #[test]
    fn test_contract_hash_changes_on_input_reorder() {
        let mut inputs_ab = IndexMap::new();
        inputs_ab.insert("a".to_string(), "String".to_string());
        inputs_ab.insert("b".to_string(), "String".to_string());

        let mut inputs_ba = IndexMap::new();
        inputs_ba.insert("b".to_string(), "String".to_string());
        inputs_ba.insert("a".to_string(), "String".to_string());

        let spec_ab = make_loaded_spec(
            "example/alpha",
            "units/example/alpha.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs_ab),
                returns: Some("String".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let spec_ba = make_loaded_spec(
            "example/alpha",
            "units/example/alpha.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs_ba),
                returns: Some("String".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );

        assert_ne!(
            compute_contract_hash(&spec_ab),
            compute_contract_hash(&spec_ba)
        );
    }

    #[test]
    fn test_contract_hash_present_for_data_seam() {
        let spec = make_loaded_data_seam(
            "pricing/checkout_quote",
            "units/pricing/checkout_quote.unit.spec",
        );

        assert!(
            compute_contract_hash(&spec).is_some(),
            "data seams must write a top-level truth hash"
        );
    }

    #[test]
    fn test_contract_hash_changes_on_data_seam_intent_change() {
        let spec_original = make_loaded_data_seam(
            "pricing/checkout_quote",
            "units/pricing/checkout_quote.unit.spec",
        );
        let mut spec_changed = spec_original.clone();
        spec_changed.spec.intent.why = "Changed seam intent".to_string();

        assert_ne!(
            compute_contract_hash(&spec_original),
            compute_contract_hash(&spec_changed)
        );
    }

    #[test]
    fn test_contract_hash_present_for_sum_seam() {
        let spec = make_loaded_sum_seam(
            "pricing/checkout_status",
            "units/pricing/checkout_status.unit.spec",
        );

        assert!(
            compute_contract_hash(&spec).is_some(),
            "sum seams must write a top-level truth hash"
        );
    }

    #[test]
    fn test_contract_hash_changes_on_sum_seam_variant_reorder() {
        let spec_original = make_loaded_sum_seam(
            "pricing/checkout_status",
            "units/pricing/checkout_status.unit.spec",
        );
        let mut spec_reordered = spec_original.clone();
        spec_reordered.spec.extensions.sum = Some(AuthoredSumShape {
            variants: IndexMap::from([
                (
                    "quoted_total".to_string(),
                    AuthoredSumVariant {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                AuthoredField {
                                    type_: "i32".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                AuthoredField {
                                    type_: "i32".to_string(),
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "pending".to_string(),
                    AuthoredSumVariant {
                        fields: IndexMap::new(),
                    },
                ),
            ]),
        });

        assert_ne!(
            compute_contract_hash(&spec_original),
            compute_contract_hash(&spec_reordered)
        );
    }

    #[test]
    fn test_contract_hash_changes_on_sum_seam_method_truth_change() {
        let spec_original = make_loaded_sum_seam(
            "pricing/checkout_status",
            "units/pricing/checkout_status.unit.spec",
        );
        let mut spec_changed = spec_original.clone();
        spec_changed.spec.extensions.methods[1]
            .deps
            .push("money/round".to_string());

        assert_ne!(
            compute_contract_hash(&spec_original),
            compute_contract_hash(&spec_changed)
        );
    }

    #[test]
    fn test_contract_hash_changes_on_data_seam_method_truth_change() {
        let spec_original = make_loaded_data_seam(
            "pricing/checkout_quote",
            "units/pricing/checkout_quote.unit.spec",
        );
        let mut spec_changed = spec_original.clone();
        spec_changed.spec.extensions.methods[1]
            .deps
            .push("money/round".to_string());

        assert_ne!(
            compute_contract_hash(&spec_original),
            compute_contract_hash(&spec_changed)
        );
    }

    #[test]
    fn test_authored_truth_digest_ignores_backend_only_seam_changes() {
        let spec_original = make_loaded_data_seam(
            "pricing/checkout_quote",
            "units/pricing/checkout_quote.unit.spec",
        );
        let mut spec_changed = spec_original.clone();
        spec_changed.spec.extensions.methods[0]
            .lowering
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .body = "{ self.subtotal }".to_string();
        spec_changed
            .spec
            .extensions
            .backends
            .as_mut()
            .unwrap()
            .rust
            .as_mut()
            .unwrap()
            .derives
            .push("PartialEq".to_string());

        assert_eq!(
            compute_authored_truth_digest(&spec_original),
            compute_authored_truth_digest(&spec_changed)
        );
        assert_ne!(
            compute_backend_execution_digest(&spec_original),
            compute_backend_execution_digest(&spec_changed)
        );
    }

    #[test]
    fn test_backend_execution_digest_ignores_authored_only_seam_changes() {
        let spec_original = make_loaded_sum_seam(
            "pricing/checkout_status",
            "units/pricing/checkout_status.unit.spec",
        );
        let mut spec_changed = spec_original.clone();
        spec_changed.spec.intent.why = "Changed authored truth".to_string();
        spec_changed.spec.extensions.methods[0]
            .deps
            .push("money/round".to_string());

        assert_ne!(
            compute_authored_truth_digest(&spec_original),
            compute_authored_truth_digest(&spec_changed)
        );
        assert_eq!(
            compute_backend_execution_digest(&spec_original),
            compute_backend_execution_digest(&spec_changed)
        );
    }

    #[test]
    fn test_read_passport_returns_none_for_missing() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");

        let passport = read_passport(&source_path).unwrap();
        assert!(passport.is_none());
    }

    #[test]
    fn test_read_passport_returns_err_for_malformed() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        let passport_path = passport_path_for(&source_path).unwrap();
        fs::write(&passport_path, "{not valid json").unwrap();

        let result = read_passport(&source_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_passport_discards_non_sha256_contract_hash() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        let passport_path = passport_path_for(&source_path).unwrap();
        fs::write(
            &passport_path,
            r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_tax",
  "intent": "Why pricing/apply_tax",
  "deps": [],
  "local_tests": [],
  "generated_at": "2026-04-04T00:00:00Z",
  "source_file": "units/pricing/apply_tax.unit.spec",
  "contract_hash": "deadbeef"
}"#,
        )
        .unwrap();

        let passport = read_passport(&source_path).unwrap().unwrap();
        assert!(passport.contract_hash.is_none());
    }

    #[test]
    fn test_read_passport_roundtrip() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap();

        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i32".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            vec![],
            vec![],
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            }),
            compute_contract_hash(&spec),
        );
        write_passport(&passport, &source_path).unwrap();

        let parsed = read_passport(&source_path).unwrap().unwrap();
        assert_eq!(parsed, passport);
    }

    #[test]
    fn write_passport_creates_valid_json() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap(); // create source file so parent exists

        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        write_passport(&passport, &source_path).unwrap();

        let passport_path = dir.path().join("apply_tax.spec.passport.json");
        assert!(passport_path.exists());

        let content = fs::read_to_string(&passport_path).unwrap();
        let parsed: Passport = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.id, "pricing/apply_tax");
        assert_eq!(parsed.generated_at, "2026-04-04T00:00:00Z");
    }

    #[test]
    fn write_passport_round_trips_contract_with_omitted_empty_fields() {
        let dir = TempDir::new().unwrap();
        let source_path = dir.path().join("apply_tax.unit.spec");
        fs::write(&source_path, "").unwrap();

        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path.to_str().unwrap(),
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        write_passport(&passport, &source_path).unwrap();

        let content = fs::read_to_string(dir.path().join("apply_tax.spec.passport.json")).unwrap();
        let parsed: Passport = serde_json::from_str(&content).unwrap();

        assert_eq!(
            parsed.contract.unwrap(),
            PassportContract {
                inputs: vec![PassportInput {
                    name: "subtotal".to_string(),
                    type_: "i32".to_string(),
                }],
                returns: Some("i32".to_string()),
                invariants: vec![],
            }
        );
    }

    #[test]
    fn build_passport_with_evidence_serializes_observed_results() {
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![("basic", "apply_tax(1,2) == 3")],
        );
        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: Some(ArtifactProvenance {
                    git_commit_sha: "abc123".to_string(),
                }),
            }),
            Some("sha256:abc123".to_string()),
        );

        assert_eq!(
            passport.evidence,
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: Some(ArtifactProvenance {
                    git_commit_sha: "abc123".to_string(),
                }),
            })
        );
        assert_eq!(passport.contract_hash, Some("sha256:abc123".to_string()));
    }

    #[test]
    fn build_passport_with_evidence_marks_available_freshness_as_fresh() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );

        let passport = build_passport_with_evidence(
            &spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            }),
            compute_contract_hash(&spec),
        );

        assert_eq!(
            passport.freshness_anchor,
            Some(PassportFreshnessSnapshot {
                authored_truth_digest: compute_authored_truth_digest(&spec),
                backend_execution_digest: None,
            })
        );
        assert_eq!(
            passport.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&spec),
                    backend_execution_digest: None,
                },
                authored_truth_status: FreshnessStatus::Fresh,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
    }

    #[test]
    fn build_passport_preserving_proof_state_preserves_anchor_and_reprojects_freshness() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let source_path = "units/pricing/apply_tax.unit.spec";
        let original_spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path,
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs.clone()),
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![("basic", "true")],
        );
        let changed_spec = make_loaded_spec(
            "pricing/apply_tax",
            source_path,
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i64".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![("basic", "true")],
        );
        let existing = build_passport_with_evidence(
            &original_spec,
            "2026-04-04T00:00:00Z",
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![PassportTestResult {
                    id: "basic".to_string(),
                    status: "pass".to_string(),
                    reason: None,
                }],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            }),
            compute_contract_hash(&original_spec),
        );

        let rebuilt = build_passport_preserving_proof_state(
            &changed_spec,
            "2026-04-05T00:00:00Z",
            Some(&existing),
            existing.contract_hash.clone(),
        );

        assert_eq!(rebuilt.evidence, existing.evidence);
        assert_eq!(rebuilt.freshness_anchor, existing.freshness_anchor);
        assert_eq!(
            rebuilt.freshness,
            Some(PassportFreshness {
                snapshot: PassportFreshnessSnapshot {
                    authored_truth_digest: compute_authored_truth_digest(&changed_spec),
                    backend_execution_digest: None,
                },
                authored_truth_status: FreshnessStatus::Stale,
                backend_execution_status: FreshnessStatus::Unknown,
            })
        );
        assert_eq!(rebuilt.contract_hash, existing.contract_hash);
        assert_eq!(rebuilt.generated_at, "2026-04-05T00:00:00Z");
        assert_eq!(rebuilt.contract.unwrap().returns.as_deref(), Some("i64"));
    }

    #[test]
    fn resolve_passport_freshness_uses_legacy_contract_hash_when_freshness_missing() {
        let mut inputs = IndexMap::new();
        inputs.insert("subtotal".to_string(), "i32".to_string());
        let original_spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs.clone()),
                returns: Some("i32".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let changed_spec = make_loaded_spec(
            "pricing/apply_tax",
            "units/pricing/apply_tax.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: Some(inputs),
                returns: Some("i64".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let legacy_passport = Passport {
            freshness: None,
            contract_hash: compute_contract_hash(&original_spec),
            ..build_passport(&original_spec, "2026-04-04T00:00:00Z")
        };

        let freshness = resolve_passport_freshness(&changed_spec, Some(&legacy_passport))
            .expect("freshness should resolve");

        assert_eq!(freshness.authored_truth_status, FreshnessStatus::Stale);
        assert_eq!(freshness.backend_execution_status, FreshnessStatus::Unknown);
        assert_eq!(
            freshness.snapshot.authored_truth_digest,
            compute_authored_truth_digest(&changed_spec)
        );
        assert_eq!(freshness.snapshot.backend_execution_digest, None);
    }

    #[test]
    fn resolve_passport_freshness_marks_legacy_contract_addition_as_stale() {
        let original_spec = make_loaded_spec(
            "pricing/apply_discount",
            "units/pricing/apply_discount.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let changed_spec = make_loaded_spec(
            "pricing/apply_discount",
            "units/pricing/apply_discount.unit.spec",
            Some("0.3.0"),
            Some(Contract {
                inputs: None,
                returns: Some("bool".to_string()),
                invariants: vec![],
            }),
            vec![],
            vec![],
        );
        let legacy_passport = Passport {
            evidence: Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            }),
            freshness: None,
            freshness_anchor: None,
            contract_hash: None,
            ..build_passport(&original_spec, "2026-04-04T00:00:00Z")
        };

        let freshness = resolve_passport_freshness(&changed_spec, Some(&legacy_passport))
            .expect("freshness should resolve");

        assert_eq!(freshness.authored_truth_status, FreshnessStatus::Stale);
        assert_eq!(freshness.backend_execution_status, FreshnessStatus::Unknown);
        assert_eq!(
            freshness.snapshot.authored_truth_digest,
            compute_authored_truth_digest(&changed_spec)
        );
    }

    #[test]
    fn normalize_proof_surfaces_orders_and_deduplicates() {
        assert_eq!(
            normalize_proof_surfaces(vec![
                ProofSurface::ImplicitOnly,
                ProofSurface::Atom,
                ProofSurface::Molecule,
                ProofSurface::Atom,
            ]),
            vec![
                ProofSurface::Atom,
                ProofSurface::Molecule,
                ProofSurface::ImplicitOnly,
            ]
        );
    }

    #[test]
    fn legacy_passport_evidence_without_provenance_still_deserializes() {
        let passport: Passport = serde_json::from_str(
            r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_tax",
  "intent": "Why pricing/apply_tax",
  "deps": [],
  "local_tests": [],
  "generated_at": "2026-04-04T00:00:00Z",
  "source_file": "units/pricing/apply_tax.unit.spec",
  "evidence": {
    "build_status": "pass",
    "test_results": [],
    "observed_at": "2026-04-04T00:01:00Z"
  }
}"#,
        )
        .unwrap();

        assert_eq!(
            passport.evidence,
            Some(PassportEvidence {
                build_status: "pass".to_string(),
                test_results: vec![],
                observed_at: "2026-04-04T00:01:00Z".to_string(),
                provenance: None,
            })
        );
    }

    #[test]
    fn spec_generate_passport_has_no_evidence() {
        let spec = make_loaded_spec(
            "money/round",
            "units/money/round.unit.spec",
            Some("0.3.0"),
            None,
            vec![],
            vec![],
        );
        let passport = build_passport(&spec, "2026-04-04T00:00:00Z");
        let json = serde_json::to_string(&passport).unwrap();

        assert!(passport.evidence.is_none());
        assert!(passport.contract_hash.is_none());
        assert!(passport.markers.is_none());
        assert!(passport.proof_coverage.is_none());
        assert!(
            !json.contains("\"evidence\""),
            "static passport should not serialize evidence: {json}"
        );
        assert!(
            !json.contains("\"markers\""),
            "function passports should omit markers: {json}"
        );
        assert!(
            !json.contains("\"proof_coverage\""),
            "function passports should omit proof coverage: {json}"
        );
        assert!(
            !json.contains("\"escape_hatch_gate\""),
            "function passports should omit escape hatch gate metadata: {json}"
        );
    }

    #[test]
    fn rfc3339_now_format() {
        let ts = rfc3339_now();
        // Must match YYYY-MM-DDTHH:MM:SSZ
        assert_eq!(ts.len(), 20, "timestamp length should be 20: {ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert_eq!(&ts[19..20], "Z");
    }

    #[test]
    fn rfc3339_known_epoch() {
        // Unix epoch = 1970-01-01T00:00:00Z
        let (y, mo, d, h, m, s) = secs_to_gregorian(0);
        assert_eq!((y, mo, d, h, m, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn rfc3339_known_date() {
        // 2026-04-04T12:34:56Z
        // Days from epoch to 2026-04-04: calculate manually
        // 2026-04-04 = epoch + 20547 days + 45296 seconds
        let ts = 20547 * 86400 + 12 * 3600 + 34 * 60 + 56;
        let (y, mo, d, h, m, s) = secs_to_gregorian(ts);
        assert_eq!((y, mo, d, h, m, s), (2026, 4, 4, 12, 34, 56));
    }

    #[test]
    fn ensure_gitignore_creates_file_when_absent() {
        let dir = TempDir::new().unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("**/*.spec.passport.json"));
    }

    #[test]
    fn ensure_gitignore_appends_when_entry_missing() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(".gitignore"), "*.rs\n").unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("*.rs"));
        assert!(content.contains("**/*.spec.passport.json"));
    }

    #[test]
    fn ensure_gitignore_is_idempotent() {
        let dir = TempDir::new().unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        let count = content.matches("**/*.spec.passport.json").count();
        assert_eq!(count, 1, "entry should appear exactly once");
    }

    #[test]
    fn ensure_gitignore_no_trailing_newline_handled() {
        let dir = TempDir::new().unwrap();
        // File without trailing newline
        fs::write(dir.path().join(".gitignore"), "*.rs").unwrap();
        ensure_gitignore_entry(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("*.rs\n**/*.spec.passport.json"));
    }
}
