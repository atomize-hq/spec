# M36 - Helper-Surface Follow-On Contract Consolidation

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-05**  
Supersedes: **M35 - Architecture Shared-Core Follow-On**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260505-125701.md`**  
Frozen baseline commit: **`4622a30aee132329e87a5d3f2a556d9599a73fb5`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
Execution note: **keep the work bounded inside `xtask/src/family/`, preserve the frozen public wedge vocabulary, and harden proof semantics without inventing a generic decision engine or a new artifact family.**

## Objective

Turn the remaining M35 seam into one first-class follow-on contract for the
durable helper-surface wedge, then make closeout proof assert stable semantic
identity instead of unstable raw bytes.

After M36, the repo must still say the same thing:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- `open_blockers = ["helper_surface_not_promotable"]`
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

But it must reach that result through one named contract surface, and the proof
of that result must survive harmless `generated_at` churn.

## Decision

M36 ships as a **bounded contract extraction plus proof-surface hardening**
inside the existing `xtask` family layer.

The implementation has four parts:

1. freeze the shipped M35 baseline on `feat/corpus-expansion`
2. extend `xtask/src/family/helper_surface.rs` so it owns the helper-surface
   follow-on contract, not just the low-level classification
3. rewire `recommend.rs` and `promotion_artifacts.rs` to consume that shared
   contract instead of reconstructing it in parallel
4. add stable proof-fingerprint helpers based on normalized artifact meaning,
   while keeping raw artifact SHA available only as debug evidence

This is the smallest honest M36. It finishes the architecture move M35 pointed
at without widening into a framework.

## Problem Statement

M35 is done on this branch.

The current branch already passes the exact frozen M35 verification floor and
already contains the low-level classifier surface at
`xtask/src/family/helper_surface.rs`.

What is still split today is the higher-level contract that means:

> this recommendation basis is the durable non-promotable helper-surface wedge,
> so corpus run `1` stays unspent and the repo pivots to architecture follow-on
> work.

That idea still lives in multiple encodings:

- `xtask/src/family/helper_surface.rs` classifies the low-level helper surface
- `xtask/src/family/recommend.rs` reconstructs the follow-on outcome in
  `derive_corpus_program_decision_contract(...)`
- `xtask/src/family/recommend.rs` also reconstructs helper-surface disposition
  from coverage or durable-hold fields when loading an analysis basis
- `xtask/src/family/promotion_artifacts.rs` validates the same outcome through
  tuple-specific helper checks over basis snapshots and emitted decision fields

That is still "one idea, several encodings."

There is also one real proof gap:

- the ignored analysis artifacts under
  `.semantic-family-artifacts/family-promotion/analysis/` are not byte-stable
  across unchanged reruns because `generated_at` changes, and coverage also
  embeds a fresh inventory path and upstream SHA chain
- raw SHA of those latest artifacts is therefore not durable identity
- M36 must preserve semantic truth while hardening how closeouts prove sameness

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Low-level helper-surface classification | `xtask/src/family/helper_surface.rs` | Reuse and extend. Keep this as the single contract owner. |
| Corpus-program decision derivation | `xtask/src/family/recommend.rs` | Reuse the existing command path, but remove bespoke helper-surface follow-on reconstruction. |
| Artifact validation | `xtask/src/family/promotion_artifacts.rs` | Reuse the validator surface, but make it consume the shared contract instead of parallel tuple-only reasoning. |
| Coverage normalization | `xtask/src/family/coverage.rs::normalized_for_recommend_determinism(...)` | Reuse for proof hardening. Do not redesign coverage artifacts. |
| Recommendation normalization | `xtask/src/family/recommend.rs::normalized_recommendation_for_determinism(...)` and `normalized_corpus_program_decision_for_determinism(...)` | Reuse and expose through stable proof-fingerprint helpers. |
| Regression harness | `xtask/src/lib.rs` targeted tests for `recommend`, `corpus_decision`, and artifact schema validation | Reuse. Add M36 fixtures here instead of creating a new harness. |
| Maintainer documentation | `semantic-families/README.md`, `docs/recommendation_corpus_expansion_program_v0.1.md`, `docs/semantic_family_capability_corpus_guide_v0.1.md` | Update only where M36 changes ownership or proof semantics. |
| Current branch truth | commit `4622a30aee132329e87a5d3f2a556d9599a73fb5` | Treat as the opening gate. M36 does not relitigate M35. |

### Minimum change set

The minimum complete implementation is:

1. freeze and reverify the current M35 wedge on this branch
2. extend `helper_surface.rs` with one explicit follow-on contract type and one
   derivation function for the durable helper-surface wedge
3. rewire `derive_corpus_program_decision_contract(...)` in `recommend.rs` to
   consume that contract
4. rewire `CorpusProgramDecisionArtifact::validate(...)` in
   `promotion_artifacts.rs` to recompute or consume that same contract before
   enforcing frozen tuple exactness
5. add stable proof-fingerprint helpers over normalized coverage,
   recommendation, and corpus-decision artifacts
6. add regression tests and maintainer docs proving both the contract
   extraction and proof hardening are real

Anything beyond that is scope leak.

### Complexity check

This milestone should stay inside roughly these touched areas:

- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `PLAN.md`

That is a real diff, but still one bounded lane.

The smell threshold is acceptable only because M36 is fixing two tightly-coupled
surfaces:

- contract duplication
- proof identity duplication

No new crate, no schema-wide artifact redesign, no new CLI family, no generic
"decision engine."

### Search check

**[Layer 1]** Reuse the repo's existing normalization pattern instead of
inventing a new proof subsystem. The code already has normalized views for
coverage and recommendation determinism.

**[Layer 1]** Reuse the repo's current boundary: semantic family policy stays in
`xtask/src/family/`, not `spec-core`.

**[Layer 3]** The correct architecture move is not "make it more generic."
The correct move is "make the exact durable helper-surface follow-on contract
explicit once." The broader framework can stay imaginary until the repo proves
it needs one.

**[EUREKA]** Do not add a new artifact field just to carry an intermediate
follow-on contract.

That feels cleaner at first glance, but it spends scope to serialize an
internal conclusion that can already be recomputed from the validated analysis
basis. M36 should harden contract ownership, not widen public schema.

### TODOS cross-reference

No open TODO in `TODOS.md` blocks M36 directly.

The closest active backlog themes already point the same direction:

- keep read-side truth honest
- prefer one explicit owner over parallel recomposition
- fix proof semantics where raw artifact bytes are not durable identity

M36 should add new TODOs only if it deliberately defers:

- a future generalized non-helper follow-on contract, or
- a later artifact-schema redesign for first-class semantic proof fingerprints

### Completeness check

A partial M36 is not enough.

The complete version is:

- one code owner for the helper-surface follow-on contract
- both decision derivation and validation consume that owner
- stable proof identity exists for unchanged semantic inputs
- tests prove semantic sameness survives byte churn
- docs explain both boundaries honestly

That is the lake. Boil it.

### Distribution check

No new package, binary, container image, or publish pipeline is required.

The deliverable is internal maintainer truth layered onto the existing command
surfaces:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family validate-artifact <path>`

## Locked Decisions

### 1. The follow-on contract lives in `helper_surface.rs`

M36 does **not** create a sibling `follow_on_contract.rs`.

Reason:

- the scope is still one wedge
- the existing module already owns the prerequisite classifier
- keeping classifier and derived contract together is the minimal diff

If the file grows awkwardly later, a future milestone can split it. M36 does
not pre-spend that abstraction.

### 2. Validation recomputes the contract from the analysis basis

`promotion_artifacts.rs` should derive the helper-surface follow-on contract
from the already-validated analysis basis and compare the emitted tuple against
that expectation.

M36 does **not** add a new intermediate contract field to public artifacts.

### 3. Stable proof identity is additive and normalized

M36 hardens proof by computing stable semantic fingerprints from normalized
artifact meaning.

Specifically:

- coverage proof ignores `generated_at`, `inventory_path`, and
  `inventory_sha256`
- recommendation proof ignores `generated_at` and replaces
  `delta_from_previous` with its normalized placeholder
- corpus-decision proof ignores `generated_at`

Raw SHA of latest artifact bytes may still be logged for debugging, but it is
not authoritative proof identity anymore.

### 4. Artifact payload churn stays out of scope

M36 does **not** remove `generated_at` or `inventory_path` from the current
artifacts.

Those fields still serve their operational purpose. The fix is to stop treating
them as semantic identity.

### 5. The frozen wedge vocabulary remains unchanged

These exact strings stay frozen:

- `helper_surface_not_promotable`
- `durable_non_promotable_helper_surface`
- `pivot_to_architecture_shared_core_follow_on`
- `author_architecture_follow_on_plan`

M36 consolidates how the repo reaches them. It does not rename them.

### 6. No generic decision framework

Do not introduce:

- a registry of follow-on contract types
- a policy DSL
- generic tuple engines
- shared-core portability work

M36 is one bounded wedge repair.

## Proposed Shared Contract Shape

Keep the existing low-level classifier and add one derived contract beside it:

```rust
pub(crate) enum HelperSurfaceDisposition {
    DurableNonPromotableHelperSurface,
}

pub(crate) struct HelperSurfaceSignal<'a> {
    pub(crate) primary_reason_code: UnsupportedFunctionReasonCode,
    pub(crate) overlap_family: &'a str,
    pub(crate) real_example_hits: usize,
    pub(crate) shape_fingerprint: &'a str,
}

pub(crate) enum HelperSurfaceFollowOnContract {
    ArchitectureSharedCoreFollowOn,
}

pub(crate) struct HelperSurfaceFollowOnInputs<'a> {
    pub(crate) disposition: Option<HelperSurfaceDisposition>,
    pub(crate) decision_status: DecisionStatus,
    pub(crate) open_blockers: &'a [DecisionReason],
    pub(crate) missing_evidence: &'a [String],
    pub(crate) stale_evidence: &'a [String],
}

pub(crate) fn classify_helper_surface(
    signal: &HelperSurfaceSignal<'_>,
) -> Option<HelperSurfaceDisposition>;

pub(crate) fn derive_helper_surface_follow_on_contract(
    inputs: &HelperSurfaceFollowOnInputs<'_>,
) -> Option<HelperSurfaceFollowOnContract>;
```

Design rules:

1. `classify_helper_surface(...)` stays pure and low-level.
2. `derive_helper_surface_follow_on_contract(...)` is still wedge-specific.
3. The derived contract answers only whether the architecture follow-on wedge is
   active.
4. Action mapping stays outside the helper module.
5. The module remains readable in one screen or two, not a mini-framework.

## Target Architecture

```text
semantic-families/corpus/rust-function.toml
        |
        v
cargo xtask family coverage --format json
        |
        v
coverage.latest.json
        |
        v
recommend.rs
  ├── candidate discovery
  ├── helper_surface::classify_helper_surface(...)
  ├── helper_surface::derive_helper_surface_follow_on_contract(...)
  ├── recommendation.latest.json
  └── corpus-program-decision.latest.json
                |
                v
promotion_artifacts.rs
  ├── validate analysis basis exactness
  ├── recompute helper-surface follow-on contract
  └── enforce frozen emitted tuple only when that contract is active

proof closeout / run logs
  ├── raw latest artifact SHA (debug only)
  └── normalized semantic fingerprint (authoritative identity)

docs/
  └── explain one contract owner + stable proof semantics
```

### Why this boundary is right

- `coverage.rs` still owns observed corpus truth
- `recommend.rs` still owns recommendation assembly and operator action mapping
- `promotion_artifacts.rs` still owns artifact validation
- `helper_surface.rs` owns only the wedge-specific semantic contract
- proof hardening reuses existing normalized views instead of inventing new
  runtime truth

That is the whole game.

## Implementation Plan

### Phase 1 - Freeze and verify the M35 baseline

1. Re-run the frozen M35 verification floor on `feat/corpus-expansion`.
2. Confirm the live wedge still emits:
   - `no_strong_candidate`
   - `not_recommended`
   - `helper_surface_not_promotable`
   - `pivot_to_architecture_shared_core_follow_on`
3. Treat any drift here as an opening blocker. M36 does not start from moving
   ground.

Exit gate:

- the branch matches the shipped M35 baseline semantically
- the current analysis outputs still justify the M36 follow-on instead of a new
  corpus run

### Phase 2 - Extend `helper_surface.rs` from classifier to contract owner

1. Keep `HelperSurfaceSignal` and `classify_helper_surface(...)` intact in
   meaning.
2. Add the derived follow-on contract type and one derivation function.
3. Encode the exact contract preconditions once:
   - helper-surface disposition is durable non-promotable
   - decision status is `not_recommended`
   - open blockers are exactly `helper_surface_not_promotable`
   - missing evidence is empty
   - stale evidence is empty
4. Keep all non-helper cases as `None`.

Exit gate:

- one file owns both the low-level helper-surface classification and the
  high-level follow-on contract
- no other file contains a second semantic copy of those preconditions

### Phase 3 - Rewire `recommend.rs`

#### Recommendation/read-side logic

Rewire `recommend.rs` so the corpus-program decision path consumes the shared
follow-on contract instead of reconstructing the wedge through local tuple
checks.

Rules:

- action mapping remains in `recommend.rs`
- helper-surface contract detection moves out of `derive_corpus_program_decision_contract(...)`
- the outward decision artifact stays byte-compatible except for ordinary
  `generated_at` churn

#### Basis recovery logic

Rewire the basis-loading path so recovery of helper-surface disposition from a
validated analysis basis is explicit and bounded.

Keep the current recovery order:

1. prefer real coverage-basis reconstruction when the coverage SHA still matches
2. fall back to the durable-hold tuple only for the exact frozen helper-surface
   case

But the final follow-on decision must still pass through the shared contract
owner.

Exit gate:

- `recommend.rs` still owns operator action mapping
- `recommend.rs` no longer owns a second semantic copy of the helper-surface
  follow-on contract

### Phase 4 - Rewire `promotion_artifacts.rs`

1. Replace `basis_snapshot_requires_helper_surface_follow_on(...)` as the
   semantic owner with shared-contract derivation from the validated basis.
2. Keep tuple-validation helpers only as frozen field exactness guards.
3. Make contradictory states fail loudly:
   - if the shared contract says the wedge is active, the emitted tuple must
     match the frozen M35 vocabulary exactly
   - if the shared contract says the wedge is not active, helper-surface
     follow-on tuple fields must not appear

Exit gate:

- validation and decision derivation are anchored on the same contract owner
- tuple helpers remain read-side guards, not competing semantic engines

### Phase 5 - Harden proof semantics

1. Expose stable proof-fingerprint helpers for:
   - coverage artifacts
   - recommendation artifacts
   - corpus-program decision artifacts
2. Build those helpers from the repo's existing normalization logic.
3. Add targeted regression tests proving:
   - semantic fingerprints are stable across reruns with only ephemeral-field
     churn
   - fingerprints still change when semantic inputs change
4. Update closeout guidance so run logs treat normalized fingerprints as the
   durable assertion surface.

Exit gate:

- maintainers can prove unchanged meaning without pretending raw latest JSON
  bytes are stable

### Phase 6 - Docs and closeout

1. Update `semantic-families/README.md` to explain that:
   - helper-surface classification exists
   - the architecture follow-on wedge is now a first-class shared contract
   - raw latest artifact SHA is not semantic identity
2. Update `docs/recommendation_corpus_expansion_program_v0.1.md` to make the
   M35-to-M36 handoff explicit:
   - M35 froze the wedge
   - M36 consolidates the follow-on contract and proof semantics
3. Update `docs/semantic_family_capability_corpus_guide_v0.1.md` so future
   maintainers do not mistake byte churn for recommendation drift.
4. Update `PLAN.md` completion notes only after all verification commands pass.

## Code Quality Rules

### DRY rule

There must be one semantic owner for the helper-surface follow-on contract.

Reject any implementation that leaves:

- one contract in `helper_surface.rs`
- a second contract in `recommend.rs`
- a third contract in `promotion_artifacts.rs`

That is exactly the duplication M36 exists to remove.

### Explicit over clever

Do not introduce:

- registries of contract evaluators
- generic policy traits
- schema-driven rule tables
- magical normalization layers

Use named structs, explicit enums, and obvious `match` blocks.

### Minimal diff

Prefer:

- one existing module extended
- two existing consumers rewired
- proof helpers added beside current normalization logic
- tests added to the current xtask surface

Do not turn M36 into a general artifact refactor.

## Test Review

### Test framework

This repo is still Rust-first for this milestone.

M36 verification stays inside:

- `cargo test -p xtask ...`
- targeted `cargo xtask family ...` command runs
- `cargo xtask family validate-artifact ...`

No new test framework. No special harness.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/helper_surface.rs
    |
    ├── [REQ TEST] exact live helper wedge -> classify_helper_surface = Some(DurableNonPromotableHelperSurface)
    ├── [REQ TEST] exact live helper wedge -> derive_helper_surface_follow_on_contract = Some(ArchitectureSharedCoreFollowOn)
    ├── [REQ TEST] helper disposition present but missing_evidence non-empty -> None
    ├── [REQ TEST] helper disposition present but stale_evidence non-empty -> None
    ├── [REQ TEST] helper disposition present but blocker != helper_surface_not_promotable -> None
    └── [REQ TEST] non-helper fingerprint / known overlap / zero real-example hits -> None

[+] xtask/src/family/recommend.rs
    |
    ├── derive_corpus_program_decision_contract(...)
    │   ├── [REQ TEST] recommended candidate -> family promotion action unchanged
    │   ├── [REQ TEST] missing/stale evidence -> spend_corpus_run_1 unchanged
    │   ├── [REQ TEST] durable helper wedge -> architecture follow-on action via shared contract
    │   ├── [REQ TEST] blocked_for_now without helper contract -> policy run unchanged
    │   └── [REQ TEST] no actionable candidate -> stop unchanged
    |
    ├── basis recovery
    │   ├── [REQ TEST] matching coverage basis reconstructs helper disposition
    │   └── [REQ TEST] durable-hold fallback still recovers only the exact frozen helper tuple
    |
    └── proof helpers
        ├── [REQ TEST] normalized recommendation fingerprint stable across generated_at churn
        └── [REQ TEST] normalized corpus-decision fingerprint stable across generated_at churn

[+] xtask/src/family/promotion_artifacts.rs
    |
    ├── CorpusProgramDecisionArtifact::validate(...)
    │   ├── [REQ TEST] shared helper contract active + frozen tuple exact -> validates
    │   ├── [REQ TEST] shared helper contract active + wrong decision_action -> rejects
    │   ├── [REQ TEST] shared helper contract inactive + helper tuple fields present -> rejects
    │   └── [REQ TEST] basis snapshot drift -> rejects before tuple interpretation
    |
    └── proof helpers / artifact basis exactness
        └── [REQ TEST] semantic fingerprint changes when basis meaning changes

[+] xtask/src/family/coverage.rs
    |
    └── [REQ TEST] normalized coverage fingerprint stable across generated_at + inventory path churn
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Maintainer asks "should we spend corpus run 1?"
    |
    ├── [REQ TEST] coverage -> recommend -> corpus-decision still answers "no"
    └── [REQ TEST] answer is the frozen architecture follow-on tuple

[+] Maintainer validates the same wedge twice one second apart
    |
    ├── [REQ TEST] raw latest artifact SHA may differ
    └── [REQ TEST] semantic fingerprint remains identical

[+] Maintainer encounters contradictory decision state
    |
    ├── [REQ TEST] validate-artifact rejects helper tuple without shared contract support
    └── [REQ TEST] failure message points at tuple/contract mismatch, not a silent fallback

[+] Maintainer updates docs after rerunning commands
    |
    ├── [REQ TEST] docs still describe corpus run `1` as unspent
    └── [REQ TEST] docs still describe helper_surface_not_promotable as a durable hold
```

### Regression rule

M36 is partly a regression-protection milestone.

The highest-priority regressions are:

1. the live wedge stops emitting the frozen M35 decision tuple
2. validation and decision derivation disagree on whether the helper follow-on
   contract is active
3. unchanged semantic inputs produce different stable proof fingerprints

If any of those regress, the fix must include a regression test before merge.

### Required tests to add

1. `helper_surface.rs` unit tests for the new derived contract and its negative
   cases
2. `recommend.rs` regression tests proving the live wedge still maps to
   architecture follow-on through the shared contract
3. `promotion_artifacts.rs` validator tests for:
   - correct helper follow-on tuple
   - contradictory helper follow-on tuple
   - helper tuple when the contract is inactive
4. proof-fingerprint tests covering coverage, recommendation, and corpus
   decision normalization
5. one end-to-end command-path regression:
   - `coverage`
   - `recommend`
   - `corpus-decision`
   - `validate-artifact`
   all agree on the same wedge
6. doc-grep regression ensuring maintainer docs still match the frozen public
   wedge vocabulary

## Failure Modes Registry

| Codepath | Realistic failure | Test coverage required | Error handling required | User-visible outcome |
|---|---|---|---|---|
| Contract split persists | `recommend.rs` and `promotion_artifacts.rs` silently disagree about whether the helper follow-on wedge is active | yes | yes | failing test or hard validation error, not divergent green outputs |
| Contract over-generalizes | non-helper unsupported pressure is incorrectly treated as architecture follow-on | yes | yes | blocked by targeted negative tests |
| Tuple-only validation survives | validator still accepts the frozen tuple without proving the shared contract | yes | yes | hard validation failure on contradictory artifacts |
| Proof fingerprint is under-normalized | harmless `generated_at` or inventory-path churn still changes the "stable" proof identity | yes | yes | failing regression before closeout |
| Proof fingerprint is over-normalized | meaningful semantic change is masked because the helper removed too much data | yes | yes | failing regression when basis meaning changes |
| Closeout still treats raw SHA as truth | maintainers think the wedge changed when only ignored latest artifacts churned | yes | yes, doc/update guidance | explicit documentation and test-backed fingerprint surface |

Critical gap rule:

If any helper-surface contract or proof-identity failure mode has no test
**and** no hard error path, M36 is incomplete.

## Performance Review

M36 should stay read-side and cheap.

Rules:

1. `family corpus-decision` must keep reading the existing recommendation
   artifact. It must not rerun coverage internally.
2. The helper-surface contract helper stays pure and in-memory.
3. Stable proof fingerprints are computed from already-loaded artifacts or tiny
   normalized clones. No extra filesystem crawl. No second corpus load.
4. `promotion_artifacts.rs` may load the validated analysis basis once, exactly
   as it already does. No new O(n) scan hidden in validation.
5. There is no legitimate reason for M36 to change runtime complexity class.

## What Already Exists

The plan intentionally reuses the repo's current truth surfaces:

- `xtask/src/family/helper_surface.rs` already owns the low-level helper
  classifier
- `xtask/src/family/recommend.rs` already owns the corpus-program decision
  command path
- `xtask/src/family/promotion_artifacts.rs` already owns artifact validation
- `xtask/src/family/coverage.rs` and `recommend.rs` already contain the exact
  normalization hooks M36 needs for proof hardening
- `xtask/src/lib.rs` already contains targeted regression-test surfaces for the
  relevant commands and artifact schemas

M36 succeeds by consolidating and hardening these surfaces, not by replacing
them.

## NOT in scope

- spending corpus run `1`
- promoting a new family
- widening the family recommendation engine
- moving this policy into `spec-core`
- creating a new crate or a generic decision framework
- changing the public wedge vocabulary
- removing `generated_at` or inventory-path fields from artifacts
- redesigning the entire artifact schema around proof fingerprints
- broad roadmap or docs rewrites unrelated to helper-surface follow-on truth

## Verification Commands

Run these in merged M36 state:

```bash
cargo fmt --all --check
cargo clippy -p xtask --all-targets --all-features -- -D warnings
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Then assert the live wedge still says:

```bash
jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

And confirm docs stay aligned:

```bash
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan|corpus run `1` remains unspent' \
  semantic-families/README.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md
```

Stable proof identity verification should be covered by the new xtask tests.
Do not use raw latest-artifact SHA as the closeout gate.

## Worktree Parallelization Strategy

This plan has one small parallel window, not a wide fan-out.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Baseline freeze + API shape decision | `xtask/src/family/`, `PLAN.md` | — |
| Contract extraction + recommend rewiring | `xtask/src/family/`, `xtask/src/lib.rs` | Baseline freeze |
| Validator alignment | `xtask/src/family/`, `xtask/src/lib.rs` | Contract extraction API freeze |
| Proof-fingerprint hardening | `xtask/src/family/`, `xtask/src/lib.rs` | Contract extraction API freeze |
| Docs alignment | `semantic-families/`, `docs/`, `PLAN.md` | Contract vocabulary freeze |

### Parallel lanes

Lane A: contract extraction → recommend rewiring → validator alignment → proof-fingerprint hardening  
Lane B: docs alignment after the contract vocabulary is frozen

Sequential note:

- Lane A stays sequential because `helper_surface.rs`, `recommend.rs`,
  `promotion_artifacts.rs`, and `xtask/src/lib.rs` are one tightly-coupled code
  surface.
- Lane B is the only safe parallel lane. It can move once the names and final
  contract wording are frozen.

### Execution order

1. Freeze the M35 baseline and settle the contract API locally.
2. Run **Lane A** through contract extraction, validator alignment, and proof
   hardening.
3. Once Lane A freezes the final vocabulary, launch **Lane B** for docs in a
   separate worktree if useful.
4. Merge docs last, then run the full verification floor sequentially on the
   parent branch.

### Conflict flags

- `xtask/src/lib.rs` is a conflict magnet. Keep command-path and regression-test
  edits in one lane.
- `recommend.rs` is shared by both contract rewiring and proof hardening. Do not
  split those into separate parallel workstreams.
- Docs can move in parallel only after the exact frozen phrases are final.

## Completion Summary

- Step 0: Scope Challenge — scope accepted as bounded contract consolidation +
  proof hardening
- Architecture Review: one contract owner, no new crate, no schema-spread
  intermediate artifact
- Code Quality Review: remove semantic duplication, keep tuple validation as a
  guard only
- Test Review: coverage diagram produced, stable proof-fingerprint regressions
  required
- Performance Review: no new complexity class, no extra artifact crawl, no
  internal rerun of coverage
- NOT in scope: written
- What already exists: written
- Failure modes: critical gap rule defined for both contract drift and proof
  identity drift
- Parallelization: 2 lanes, but only 1 real code lane and 1 docs lane
- Lake Score: choose the complete version, consolidate the contract and harden
  proof semantics in one pass

## Done when

1. The repo has exactly one semantic owner for the durable helper-surface
   follow-on contract.
2. `recommend.rs` uses that owner to choose the architecture follow-on path.
3. `promotion_artifacts.rs` uses that owner to validate whether the helper
   follow-on tuple is allowed.
4. The live wedge still deterministically emits the frozen M35 decision
   vocabulary.
5. Stable proof fingerprints remain unchanged across harmless reruns, even when
   raw latest-artifact bytes change.
6. Docs teach maintainers to trust semantic fingerprints, not raw ignored
   artifact SHA, for this wedge.
