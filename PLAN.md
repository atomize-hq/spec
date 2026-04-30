<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m26-autoplan-restore-20260430-130212.md -->
# M27 - Coverage Accounting + Next-Family Recommendation Engine

Status: **implementation contract**
Base branch: **main**
Working branch: **feat/m26**
Last rewritten: **2026-04-30**

## Plan Authority

This file is the authoritative M27 plan.

Primary milestone source:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Repo truth checked while rewriting this plan:

- `cargo xtask family inventory --format json` on 2026-04-30
- current `xtask/src/family/**` command and artifact layout
- current `spec-core/src/semantic_review.rs` semantic-review surface
- `semantic-families/README.md`
- `CLAUDE.md`
- `TODOS.md`
- current checked-in corpora under:
  - `examples/ecommerce/units`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`

If any older draft, review artifact, or milestone note disagrees with this file,
`PLAN.md` wins for M27 execution.

## Problem Statement

M26 solved the operator loop.

The repo now has a real approval-gated family-promotion workflow plus four
promoted Rust function families:

- `function.wrapper.pipeline.chain3.v1`
- `function.wrapper.pipeline.v1`
- `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `function.arithmetic_leaf.monotone_up.v1`

That means the old question is gone.

The repo is no longer blocked on "which runtime-supported family is still
unpromoted?" The live inventory answer is currently "none":

- `promoted_families == runtime_supported_routes`
- `supported_unpromoted_families == []`

So the M27 problem is narrower and more honest:

- account for how the checked-in corpus actually routes today
- show how much authored function demand is already covered by promoted families
- show which unsupported function clusters create real promotion pressure
- separate those clusters from boundary-only or adversarial fixtures that should
  not drive the next family choice

M27 is the milestone that turns "what should we promote next?" from taste into
deterministic repo output.

## Milestone Outcome

When M27 lands, the repo can truthfully claim:

- it can account for checked-in corpus coverage at the unit-shape level
- it can distinguish:
  - function units routed to promoted families
  - function units routed to supported-but-unpromoted families
  - function units routed to `unsupported.function.v1`
  - supported non-function semantic surfaces that still matter to read-side truth
- it can cluster unsupported function demand into deterministic candidate groups
- it can rank next-family candidates from measured pressure rather than freehand
  judgment
- it can emit machine-readable coverage and recommendation artifacts with the
  exact basis they used

M27 does **not** claim:

- the ranking engine is globally optimal
- every unsupported cluster should become a family
- the current corpus is large enough to remove human judgment
- multi-language factoring is solved
- non-function family promotion is in scope

## Scope

### In Scope

- extend the family workflow with deterministic coverage accounting
- keep the existing M26 `inventory` surface and layer M27 on top of it
- add a checked-in corpus manifest for the Rust function lane
- emit durable machine-readable M27 analysis artifacts
- cluster unsupported function units into stable candidate groups
- rank candidates from repo-owned evidence
- show supported non-function semantic coverage separately so it does not vanish
- validate M27 artifacts through the existing `validate-artifact` entrypoint
- document the maintainer workflow for M27 commands and corpus labels

### NOT In Scope

- promoting the next family packet itself
- changing runtime semantic-review routing order
- adding a second language
- ranking sum or data seams as next-family candidates
- adding a new human dashboard or UI
- moving orchestration out of `xtask`
- counting packet-local proof fixtures as real corpus demand
- broad corpus expansion beyond the three locked M27 sources

## Step 0 - Scope Challenge

### Current repo truth

| Area | Current truth at 2026-04-30 | M27 implication |
|---|---|---|
| Family inventory | `cargo xtask family inventory --format json` reports four promoted runtime-supported function families and `supported_unpromoted_families: []` | Inventory alone cannot answer "what next?" anymore. |
| Semantic review output | `SemanticReview` already exposes `compatibility_key`, `support_status`, `unsupported_reason_codes`, and `rewrite_hints` | M27 must reuse this truth, not invent a second classifier. |
| Unsupported vocabulary | Current unsupported function reasons are `unsupported_control_flow`, `unsupported_dep_topology`, `unsupported_required_argument_expression`, `unsupported_wrapper_body_shape`, `unsupported_arithmetic_shape`, and `unsupported_function_surface` | M27 can cluster from structured reason codes instead of vague prose. |
| Supported non-function surfaces | `sum.discount_policy.v1` and `data.checkout_quote.v1` already exist in repo truth | M27 must report them, but must not rank them as next-family candidates. |
| Artifact layout | M26 already reserves `.semantic-family-artifacts/family-promotion/recommendation.latest.json` for approval-gated promotion packets | M27 analysis artifacts must live under a different subdirectory to avoid contract collision. |
| Checked-in corpora | The maintained non-packet corpus is small but real: 6 ecommerce units, 12 M19 falsification fixtures, 9 M20 unsupported-truth fixtures | M27 must be honest about low-confidence output when real-example pressure is weak. |

### What already exists

| Sub-problem | Existing code or artifact | M27 reuse decision |
|---|---|---|
| Runtime supported-family truth | `xtask/src/family/inventory.rs` | Reuse directly. M27 joins against live inventory output, it does not restate routing logic. |
| Semantic classification | `spec-core/src/semantic_review.rs` | Reuse directly. M27 reads repo truth from `evaluate_semantic_review`, not from hand-built xtask heuristics. |
| Spec loading and validation | `spec-core/src/loader.rs`, `spec-core/src/validator.rs` | Reuse directly. M27 loads authored corpus units through `spec-core`, not shelling out to `spec`. |
| Artifact validation and path hardening | `xtask/src/family/promotion_artifacts.rs`, `xtask/src/family/paths.rs` | Reuse and extend. M27 artifacts should validate through the same style of repo-relative path and schema checks. |
| Maintainer workflow docs | `semantic-families/README.md` | Extend with M27 commands and corpus-label rules. |

### Current corpus basis

The initial locked M27 corpus is exactly:

- `examples/ecommerce/units`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`

Current source counts:

- 6 unit specs in `examples/ecommerce/units`
- 12 unit specs in the M19 falsification pack
- 9 unit specs in the M20 unsupported-truth pack
- 27 total unit specs in the initial M27 corpus

The current family basis was re-verified during this rewrite:

```json
{
  "promoted_families": [
    "function.wrapper.pipeline.chain3.v1",
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1"
  ],
  "runtime_supported_routes": [
    "function.wrapper.pipeline.chain3.v1",
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1"
  ],
  "supported_unpromoted_families": []
}
```

That basis is the reason M27 exists.

### Minimum honest change

The minimum honest M27 diff is:

1. add an authored corpus manifest
2. compute deterministic coverage accounting across that manifest
3. cluster unsupported function demand into stable groups
4. rank those groups with machine-readable evidence
5. persist the coverage and recommendation basis as durable artifacts

Anything less is still operator taste with nicer wording.

### Complexity hold

M27 is a real feature, so the diff will cross the 8-file smell line. That is
acceptable here because the scope is still intentionally boxed:

- keep the command surface inside existing `xtask family ...`
- keep semantic truth inside `spec-core`
- add only two new feature modules in `xtask/src/family/`
- add only one new authored source file outside Rust code, the corpus manifest
- do **not** create a new crate, service, or binary

This is the smallest change that still lands the milestone honestly.

## Locked Decisions

These choices close the ambiguity that remained in the prior draft.

| Decision | Lock |
|---|---|
| Coverage and recommendation remain separate explicit commands | **Locked.** M27 adds `family coverage` and `family recommend`. No combined `family analyze` command in M27. |
| `recommend` may depend on a previously written `coverage.latest.json` | **Rejected.** `family recommend` must recompute coverage in-process first, then rank from that fresh basis. |
| M27 may reuse the M26 root `recommendation.latest.json` path | **Rejected.** That path stays reserved for approval-gated promotion artifacts. M27 writes under a new `analysis/` subdirectory. |
| M27 should shell out to `spec export` to classify units | **Rejected.** M27 should call `spec-core` directly for loading, validation, and semantic review. |
| Packet fixtures may count toward leverage if they are "close enough" | **Rejected.** Packet fixtures are `proof_only` and never contribute to promotion pressure. |
| M27 should rank non-function units too | **Rejected.** M27 reports non-function coverage only. Ranking stays on the Rust function-family lane. |
| Open questions about stdout vs durable files | **Closed.** Both M27 commands print JSON to stdout and atomically write the same bytes to durable artifact paths. |

## Locked M27 Contract

The sections below lock the authored inputs, command surfaces, artifact schemas,
ranking rules, and clustering rules for M27.

### Command Surfaces

M27 adds exactly these subcommands:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`

M27 also extends this existing subcommand:

- `cargo xtask family validate-artifact <path>`

No other CLI surface is in scope.

#### `cargo xtask family coverage --format json`

Behavior:

- loads the authored corpus manifest from
  `semantic-families/corpus/rust-function.toml`
- collects the live family inventory in-process
- writes a retained inventory snapshot under
  `.semantic-family-artifacts/family-promotion/inventory/`
- loads every manifest source through `spec-core`
- validates each loaded unit with `validate_full`
- classifies each unit through `evaluate_semantic_review`
- aggregates function coverage, non-function coverage, and unsupported clusters
- prints the coverage JSON to stdout
- atomically writes the identical bytes to:
  `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`

#### `cargo xtask family recommend --format json`

Behavior:

- runs the full coverage collection path first in-process
- never trusts an existing `coverage.latest.json` as primary input
- writes fresh `coverage.latest.json`
- ranks only `candidate_status == "rankable"` function clusters
- prints recommendation JSON to stdout
- atomically writes the identical bytes to:
  `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

#### `cargo xtask family validate-artifact <path>`

M27 extends validation so the command accepts:

- legacy M26 promotion recommendation artifacts
- legacy M26 promotion execution artifacts
- legacy M26 blocker artifacts
- new M27 coverage artifacts
- new M27 recommendation-analysis artifacts

That keeps one repo-owned validator surface.

### Authored Corpus Manifest

Path:

- `semantic-families/corpus/rust-function.toml`

Schema:

```toml
schema_version = 1
target_language = "rust"
target_lane = "function"

[[sources]]
id = "examples_ecommerce"
path = "examples/ecommerce/units"
kind = "real_example"
counts_toward_recommendation = true
note = "Canonical maintained example corpus."

[[sources]]
id = "m19_semantic_falsification_pack"
path = "spec-cli/tests/fixtures/m19/semantic_falsification_pack/units"
kind = "regression_unsupported"
counts_toward_recommendation = true
note = "Regression pack with aligned, drift, under-specified, and boundary fixtures."

[[sources]]
id = "m20_unsupported_truth_pack"
path = "spec-cli/tests/fixtures/m20/unsupported_truth_pack/units"
kind = "regression_unsupported"
counts_toward_recommendation = true
note = "Explicit unsupported-function truth pack."
```

Manifest rules:

- all source paths are repo-relative and must stay inside the workspace root
- symlink escapes are rejected
- each source id is unique
- each source path must exist and contain at least one `.unit.spec`
- molecule tests are ignored by M27; unit specs are the only corpus input
- packet fixtures under `semantic-families/**/fixtures/**` are not allowed in the
  manifest for M27

### Per-Unit Corpus Role Rules

M27 uses both source kind and filename bucket to decide whether a unit creates
promotion pressure.

Bucket detection by file name:

- `*_unsupported_near_miss.unit.spec` -> `unsupported_near_miss`
- `*_under_specified.unit.spec` -> `under_specified`
- `*_drift.unit.spec` -> `drift`
- everything else -> `aligned_or_real`

Pressure rules:

- `real_example` source:
  - all function units count toward leverage
- `regression_unsupported` source:
  - `unsupported_near_miss` units do **not** add leverage
  - `drift`, `under_specified`, and `aligned_or_real` units may add leverage
- `proof_only` source:
  - never counts toward leverage

This is the M27 mechanism that separates "boundary pressure" from "real
promotion pressure" without hand-waving.

### Coverage Artifact Contract

Path:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`

Artifact kind:

- `family_coverage_snapshot`

Required top-level fields:

- `schema_version`
- `artifact_kind`
- `generated_at`
- `inventory_path`
- `inventory_sha256`
- `corpus_manifest_path`
- `corpus_manifest_sha256`
- `sources[]`
- `function_coverage`
- `non_function_coverage`
- `family_coverage[]`
- `unsupported_clusters[]`

Required `sources[]` fields:

- `id`
- `path`
- `kind`
- `counts_toward_recommendation`
- `note`
- `unit_count`

Required `function_coverage` fields:

- `total_units`
- `promoted_family_units`
- `supported_unpromoted_family_units`
- `unsupported_function_units`

Required `non_function_coverage` fields:

- `total_units`
- `supported_sum_units`
- `supported_data_units`
- `other_units`

Required `family_coverage[]` fields:

- `family`
- `unit_count`
- `unit_ids`
- `source_ids`

Required `unsupported_clusters[]` fields:

- `cluster_id`
- `reason_code`
- `shape_fingerprint`
- `representative_unit_ids`
- `source_ids`
- `real_example_hits`
- `promotion_relevant_regression_hits`
- `boundary_only_hits`
- `overlap_family`
- `candidate_status`

Allowed `candidate_status` values:

- `rankable`
- `boundary_only`
- `low_value`
- `insufficient_evidence`

Classification rules:

- `boundary_only` when every contributing unsupported unit is either:
  - from a `proof_only` source, or
  - an `unsupported_near_miss` bucket unit
- `insufficient_evidence` when:
  - `real_example_hits == 0`, and
  - `promotion_relevant_regression_hits <= 1`
- `low_value` when:
  - `real_example_hits == 0`, and
  - all promotion-relevant hits come from exactly one source id
- `rankable` otherwise

### Recommendation Artifact Contract

Path:

- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

Artifact kind:

- `family_recommendation_analysis`

Required top-level fields:

- `schema_version`
- `artifact_kind`
- `generated_at`
- `coverage_path`
- `coverage_sha256`
- `recommendation_status`
- `ranked_candidates[]`

Allowed `recommendation_status` values:

- `ranked`
- `no_strong_candidate`
- `insufficient_real_corpus`

Status rules:

Evaluate in this order:

- `insufficient_real_corpus` when:
  - no rankable candidate has any real-example hits, and
  - every rankable candidate has `confidence.level == "low"`
- `no_strong_candidate` when:
  - at least one candidate is `rankable`, and
  - every rankable candidate has `confidence.level == "low"`
- `ranked` when the top rankable candidate has `confidence.level` of `medium`
  or `high`

Each `ranked_candidates[]` entry must include:

- `candidate_id`
- `cluster_ids`
- `primary_reason_code`
- `overlap_family`
- `leverage`
- `difficulty`
- `confidence`
- `rationale`

Required `leverage` fields:

- `real_example_hits`
- `promotion_relevant_regression_hits`
- `boundary_only_hits`
- `total_units_in_cluster`

Required `difficulty` fields:

- `tier`
- `why`

Allowed `difficulty.tier` values:

- `adjacent`
- `moderate`
- `hard`

Required `confidence` fields:

- `level`
- `why`

Allowed `confidence.level` values:

- `high`
- `medium`
- `low`

Confidence rules:

- `high` when `real_example_hits >= 2`
- `medium` when:
  - `real_example_hits == 1`, or
  - `promotion_relevant_regression_hits >= 3`
- `low` otherwise

### Ranking Contract

M27 does **not** use an opaque single score.

Rankable candidates are sorted by this tuple, in order:

1. `real_example_hits` descending
2. `promotion_relevant_regression_hits` descending
3. `difficulty.tier` ascending (`adjacent` before `moderate` before `hard`)
4. `boundary_only_hits` ascending
5. `candidate_id` ascending

Difficulty mapping:

- `unsupported_arithmetic_shape` -> `adjacent`
- `unsupported_wrapper_body_shape` -> `adjacent`
- `unsupported_required_argument_expression` -> `moderate`
- `unsupported_dep_topology` -> `hard`
- `unsupported_control_flow` -> `hard`
- `unsupported_function_surface` -> `hard`

Overlap-family mapping:

- arithmetic-shaped clusters -> `function.arithmetic_leaf.monotone_*`
- wrapper-shaped clusters -> `function.wrapper.pipeline*`
- everything else -> `unknown`

If `overlap_family == "unknown"`, the candidate may still be surfaced, but it
must never sort above a cluster with the same leverage and a concrete promoted
overlap.

### Shape Fingerprint Contract

M27 needs a deterministic fingerprint for unsupported function clustering.

Implementation decision:

- add one narrow public helper in `spec-core` that derives an unsupported
  function shape fingerprint from the same authored function packet and semantic
  routing inputs the runtime review already uses

That helper must return a stable string based on:

- function dep arity
- callable dep topology class
- contract input count
- return presence
- whether the authored body is wrapper-like, arithmetic-like, or neither

M27 must not duplicate semantic-review shape logic in `xtask`.

## Architecture / Dependency View

```text
semantic-families/corpus/rust-function.toml
        │
        ▼
xtask/src/family/coverage.rs
        │
        ├── inventory::collect_inventory()
        ├── retained inventory snapshot write
        ├── spec-core loader + validator + semantic review
        ├── unsupported fingerprint helper
        ├── per-unit coverage accounting
        └── unsupported cluster aggregation
                │
                ▼
.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
                │
                ▼
xtask/src/family/recommend.rs
        │
        ├── filter rankable clusters
        ├── deterministic tuple sort
        ├── confidence + difficulty projection
        └── recommendation_status derivation
                │
                ▼
.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

### Module boundaries

| Module | M27 responsibility | Must not happen |
|---|---|---|
| `spec-core/src/semantic_review.rs` | expose one narrow fingerprint helper over existing semantic-review truth | do not absorb M27 artifact IO or ranking policy |
| `xtask/src/family/coverage.rs` | manifest loading, corpus evaluation, coverage aggregation, coverage artifact write | do not reimplement semantic-review routing |
| `xtask/src/family/recommend.rs` | cluster ranking and recommendation artifact write | do not read stale artifacts as authoritative input |
| `xtask/src/family/promotion_artifacts.rs` | validate new analysis artifacts alongside existing M26 artifacts | do not become the implementation home for coverage logic |
| `semantic-families/corpus/rust-function.toml` | authored corpus source of truth | do not include proof-only packet fixtures |
| `semantic-families/README.md` | maintainer-facing M27 workflow docs | do not restate the full milestone theory |

### Expected file touch set

The implementation should stay within this file set unless a discovered blocker
proves otherwise:

- `xtask/Cargo.toml`
- `xtask/src/lib.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/semantic_review.rs`
- `semantic-families/corpus/rust-function.toml`
- `semantic-families/README.md`

That is the bounded blast radius. Do not introduce a new crate.

## Implementation Slices

### Slice 1 - Corpus contract and CLI wiring

Goal:

- make the new M27 commands real without implementing ranking yet

Files:

- `xtask/Cargo.toml`
- `xtask/src/lib.rs`
- `xtask/src/family/mod.rs`
- `semantic-families/corpus/rust-function.toml`

Deliverables:

- `xtask` depends on `spec-core` at runtime, not only in dev tests
- new `FamilyCommand::{Coverage, Recommend}` variants exist
- the authored corpus manifest exists at the locked path
- manifest validation rejects path escape, duplicates, missing dirs, and packet fixtures

Tests:

- CLI dispatch tests for `family coverage --format json`
- CLI dispatch tests for `family recommend --format json`
- manifest parsing and validation tests

### Slice 2 - Coverage collection and artifact contract

Goal:

- produce a truthful coverage snapshot from the locked corpus

Files:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`

Deliverables:

- retained inventory snapshot write under
  `.semantic-family-artifacts/family-promotion/inventory/`
- coverage aggregation across the three locked corpus sources
- separate function vs non-function coverage reporting
- atomic write of `analysis/coverage.latest.json`
- `validate-artifact` accepts coverage artifacts

Tests:

- coverage artifact round-trip validation
- basis-case integration test proving:
  - promoted function coverage is non-zero
  - supported-unpromoted function coverage is zero
  - unsupported function coverage is non-zero
  - supported non-function coverage is visible
- proof-only exclusion test

### Slice 3 - Unsupported fingerprinting and cluster projection

Goal:

- make unsupported clusters deterministic instead of reason-code blobs

Files:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs`
- `xtask/src/family/coverage.rs`

Deliverables:

- public unsupported function fingerprint helper in `spec-core`
- cluster key = `(reason_code, shape_fingerprint)`
- overlap-family hint projected into each cluster
- `candidate_status` classification implemented exactly as locked above

Tests:

- fingerprint stability tests in `spec-core`
- cluster split tests proving two units with the same reason but different shape
  do not collapse into one cluster
- boundary-only classification tests for `_unsupported_near_miss`

### Slice 4 - Recommendation ranking

Goal:

- turn clusters into deterministic next-family output

Files:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`

Deliverables:

- `family recommend --format json` recomputes fresh coverage first
- deterministic tuple ranking implemented exactly as locked above
- confidence and difficulty projection
- `recommendation_status` projection
- atomic write of `analysis/recommendation.latest.json`
- `validate-artifact` accepts recommendation-analysis artifacts

Tests:

- deterministic ordering test with tied candidates broken by the locked tuple
- `insufficient_real_corpus` test when rankable pressure exists only in low-value
  regression signals with zero real-example hits
- `no_strong_candidate` test when rankable candidates exist but all remain
  `confidence.level == "low"`
- golden JSON output test for recommendation artifact shape

### Slice 5 - Maintainer docs and workflow lock

Goal:

- make the workflow usable without hidden context

Files:

- `semantic-families/README.md`

Deliverables:

- document the two new M27 commands
- document the new `analysis/` artifact directory
- document that packet fixtures are `proof_only`
- document corpus source kinds and per-bucket leverage rules
- document that M27 does not overwrite the root M26
  `recommendation.latest.json`

Tests:

- none beyond review of doc accuracy against command names and paths

## Test Diagram

```text
COMMAND PATH COVERAGE
===========================
[+] cargo xtask family coverage --format json
    │
    ├── manifest load + schema validation
    │   └── unit test: valid manifest / duplicate id / path escape / packet-fixture rejection
    │
    ├── inventory collection + retained inventory snapshot write
    │   └── unit test: retained snapshot path + sha wired into artifact
    │
    ├── source scan + spec-core validate/evaluate loop
    │   └── integration test: locked M27 corpus basis
    │
    ├── function coverage aggregation
    │   └── integration test: promoted > 0, supported-unpromoted = 0, unsupported > 0
    │
    ├── non-function coverage aggregation
    │   └── integration test: supported sum/data are visible separately
    │
    ├── unsupported cluster projection
    │   ├── unit test: same reason + different fingerprint stay split
    │   └── unit test: unsupported_near_miss becomes boundary_only
    │
    └── stdout bytes == analysis/coverage.latest.json bytes
        └── golden JSON test

[+] cargo xtask family recommend --format json
    │
    ├── recompute fresh coverage first
    │   └── integration test: recommend does not depend on prior latest file
    │
    ├── filter rankable clusters
    │   └── unit test: boundary_only / low_value / insufficient_evidence excluded from ranking
    │
    ├── deterministic tuple sort
    │   └── unit test: leverage, difficulty, boundary count, candidate id tie-break order
    │
    ├── recommendation_status projection
    │   ├── unit test: insufficient_real_corpus
    │   └── unit test: no_strong_candidate
    │
    └── stdout bytes == analysis/recommendation.latest.json bytes
        └── golden JSON test

ARTIFACT VALIDATION
===========================
[+] cargo xtask family validate-artifact <coverage path>
    └── schema + path + sha checks

[+] cargo xtask family validate-artifact <recommendation-analysis path>
    └── schema + path + coverage-basis checks
```

## Failure Modes Registry

| Codepath | Realistic failure | Test required | Error handling required | User-visible result |
|---|---|---|---|---|
| Manifest loading | repo-relative path escapes workspace or points at packet fixtures | yes | hard fail with invalid-input message | explicit command failure |
| Coverage collection | one corpus source contains an invalid `.unit.spec` | yes | hard fail naming the source id and path | explicit command failure |
| Semantic classification | a unit evaluates as unsupported but produces no reason code | yes | map to `unsupported_function_surface`, never panic | explicit artifact entry |
| Cluster projection | two structurally different unsupported units collapse into one cluster | yes | stable fingerprint helper and cluster-split tests | silent corruption if untested, so treat as critical |
| Recommendation ranking | root M26 `recommendation.latest.json` gets overwritten | yes | M27 writes only under `analysis/` | explicit path isolation |
| Artifact retention | stdout and written file diverge | yes | write exactly the serialized bytes that were printed | explicit test catch |

Critical-gap rule for M27:

- any path that could silently change candidate ordering without a failing test is
  a critical gap
- any path that could overwrite the M26 promotion artifact namespace is a
  critical gap

## Risks

### Risk 1: regression packs overwhelm real-example pressure

If M27 counts adversarial fixtures too aggressively, the top recommendation will
be flattering but wrong.

Mitigation:

- source kinds are explicit
- `unsupported_near_miss` is boundary-only pressure
- `insufficient_real_corpus` is an allowed honest outcome

### Risk 2: fingerprint logic drifts away from semantic-review truth

If xtask invents its own structural model, clusters will stop matching what the
runtime reviewer actually means.

Mitigation:

- the fingerprint helper lives in `spec-core`
- xtask consumes that helper, it does not fork it

### Risk 3: M27 collides with the M26 artifact namespace

The repo already has a root `recommendation.latest.json` contract for promotion.

Mitigation:

- M27 writes only under `analysis/`
- `validate-artifact` learns both contracts explicitly

## Worktree Parallelization Strategy

This plan has parallelization value. It is not fully sequential.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Fingerprint helper | `spec-core/semantic_review`, `spec-core/lib` | — |
| B. Corpus manifest + docs | `semantic-families/` | — |
| C. Coverage command + artifact validation | `xtask/`, `.semantic-family-artifacts` path rules | A |
| D. Recommendation command | `xtask/` | A, C |

### Parallel lanes

- Lane A: Step A
  - `spec-core` helper only
- Lane B: Step B
  - `semantic-families/` manifest and docs only
- Lane C: Step C -> Step D
  - sequential inside `xtask/`, shared module ownership

### Execution order

1. Launch Lane A and Lane B in parallel worktrees.
2. Merge Lane A first, because coverage and recommendation both depend on the
   fingerprint contract.
3. Merge Lane B whenever ready. It is independent of code, but must land before
   the feature is considered complete.
4. Launch Lane C after Lane A merges.
5. Execute Step C, then Step D sequentially in the same worktree because both
   touch `xtask/src/lib.rs`, `xtask/src/family/mod.rs`, and
   `xtask/src/family/promotion_artifacts.rs`.

### Conflict flags

- Lane A and Lane C both influence the fingerprint contract, but only Lane A
  touches `spec-core/`; keep that ownership hard.
- Step C and Step D both touch `xtask/src/lib.rs` and
  `xtask/src/family/promotion_artifacts.rs`. Do not split them across parallel
  worktrees.
- Lane B is safe to run independently because it only touches authored manifest
  and maintainer docs.

## Acceptance Gates

M27 is complete only when all of the following are true:

1. `cargo xtask family coverage --format json` exists and is deterministic.
2. `cargo xtask family recommend --format json` exists and is deterministic.
3. Both commands print JSON to stdout and atomically write identical bytes to
   their locked artifact paths.
4. Coverage writes a retained inventory snapshot and cites its path and sha.
5. Coverage reports:
   - promoted function units
   - supported-but-unpromoted function units
   - unsupported function units
   - supported non-function units
6. Recommendation ranks only `rankable` function clusters.
7. Recommendation can emit `insufficient_real_corpus` honestly on the current
   small-corpus reality.
8. M27 never overwrites the root M26
   `.semantic-family-artifacts/family-promotion/recommendation.latest.json`.
9. `cargo xtask family validate-artifact <path>` accepts both new M27 artifact
   kinds.
10. The locked M27 corpus basis is covered by regression tests.

## Post-M27 Branch Rule

M27 should end with one of exactly two honest next steps:

- if `recommendation_status == "ranked"`, the next milestone can be the top
  family promotion
- if `recommendation_status == "insufficient_real_corpus"` or
  `"no_strong_candidate"`, the next milestone should be one small maintained
  corpus-expansion pack, not fake ranking theatrics

That rule is part of the milestone. M27 is supposed to make the confidence gap
visible.
