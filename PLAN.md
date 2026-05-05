# M37 - Family-Analysis Decision-Kernel Extraction After M36

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-05**
Supersedes: **M36 - Helper-Surface Follow-On Contract Consolidation**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260505-160449.md`**
Frozen baseline commit: **`d2e69249495049947d414b7126d663ae1452e076`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**
Execution note: **M37 is a bounded internal extraction inside `xtask/src/family/`. It does not reopen M31 portability, does not reopen the M36 helper-surface outcome, and does not widen any public schema or CLI surface.**

## Objective

Extract the remaining family-analysis decision kernel that is still split across
`xtask/src/family/helper_surface.rs`, `xtask/src/family/recommend.rs`, and
`xtask/src/family/promotion_artifacts.rs`, while preserving the exact current
helper-surface read-side outcome.

After M37, the repo must still emit:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- `open_blockers = ["helper_surface_not_promotable"]`
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

The change is ownership cleanup, not behavior churn.

## Frozen Premises

1. M31 already extracted seam portability into
   `spec-core/src/portability.rs`. M37 does not revisit that boundary.
2. M36 already extracted the helper-surface follow-on contract into
   `xtask/src/family/helper_surface.rs`. M37 does not relitigate the frozen
   helper-surface vocabulary or outcome.
3. The remaining duplication is family-analysis decision machinery, not
   portability machinery.
4. The right shared core for this milestone still lives inside
   `xtask/src/family/`, not `spec-core`.
5. Any larger extraction beyond this lane must be deferred with explicit
   trigger-based TODOs, or the repo will keep re-arguing the same architecture
   question.

## Problem Statement

The repo already knows the right answer, but too many files still co-own the
logic that arrives at that answer.

Today:

- `helper_surface.rs` owns wedge classification, frozen durable-hold tuples,
  frozen follow-on tuples, and basis-level follow-on activation logic
- `recommend.rs` owns corpus-program decision derivation, normalized semantic
  fingerprinting for recommendation and decision artifacts, and a hidden
  helper-surface basis replay path
- `promotion_artifacts.rs` owns artifact validation and separately recomputes
  expected basis snapshot truth

That means one semantic lane still has multiple owners:

1. basis snapshot derivation
2. decision-path activation
3. normalized proof-fingerprint rules

There is also one avoidable footgun:

- `recommend.rs::helper_surface_disposition_for_basis_candidate(...)` tries to
  re-read coverage from disk after the analysis basis has already been loaded
  and validated

That reread is accidental complexity. Once
`FamilyRecommendationAnalysisArtifact` is validated, decision derivation should
operate on validated analysis truth only.

## Scope Challenge

### What already exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Helper-surface classification | `xtask/src/family/helper_surface.rs::classify_helper_surface(...)` | Reuse as-is. Keep wedge-specific classification here. |
| Frozen helper-surface tuples | `helper_surface.rs` durable-hold and follow-on tuple helpers | Reuse as-is. Do not rename or widen this vocabulary. |
| Recommendation artifact assembly | `xtask/src/family/recommend.rs::build_recommendation_analysis_artifact(...)` | Reuse. Keep candidate ranking, artifact assembly, and command wiring here. |
| Corpus-program decision derivation | `xtask/src/family/recommend.rs::derive_corpus_program_decision_contract(...)` | Move into the new kernel module. |
| Basis snapshot derivation | `xtask/src/family/promotion_artifacts.rs::corpus_program_basis_snapshot(...)` | Move into the new kernel module. |
| Recommendation proof fingerprint | `xtask/src/family/recommend.rs::normalized_recommendation_proof_fingerprint(...)` | Move into the new kernel module. |
| Corpus-decision proof fingerprint | `xtask/src/family/recommend.rs::normalized_corpus_program_decision_proof_fingerprint(...)` | Move into the new kernel module. |
| Coverage proof fingerprint | `xtask/src/family/coverage.rs::normalized_coverage_proof_fingerprint(...)` | Reuse untouched. Coverage is not the M37 extraction target. |
| Artifact validators | `promotion_artifacts.rs` `validate(...)` implementations | Reuse validators, but make them delegate expected decision truth to the kernel. |
| Regression harness | `xtask/src/lib.rs` targeted artifact and fingerprint tests | Reuse and extend. Do not invent a second test harness. |

### Minimum complete change set

The smallest honest M37 is:

1. add `xtask/src/family/decision_kernel.rs`
2. export it from `xtask/src/family/mod.rs`
3. move basis snapshot derivation into the kernel
4. move corpus-program decision derivation into the kernel
5. move recommendation and corpus-decision normalized proof-fingerprint helpers
   into the kernel
6. rewire `recommend.rs` to call the kernel instead of owning semantic decision
   logic
7. rewire `promotion_artifacts.rs` validators to call the same kernel truth
8. extend `xtask/src/lib.rs` regressions for the moved seams
9. update docs and `TODOS.md` to describe the new boundary and deferred
   extraction triggers

Anything beyond that is scope leak.

### Complexity check

This plan touches more than 8 files, which is normally a smell. The reduction is
that the milestone still introduces exactly one new module and zero new crates.

The bound is explicit:

- no `spec-core` changes
- no new CLI commands
- no schema version bumps
- no new artifact kinds
- no generic policy engine
- no coverage-layer redesign
- no second helper-surface abstraction file

### Search check

**[Layer 1]** Reuse the repo's existing normalization pattern. Coverage already
proves that proof fingerprints should ignore churn-only fields.

**[Layer 1]** Reuse the existing crate boundary. Family-analysis policy remains
inside `xtask/src/family/`.

**[Layer 3]** The right move is not "generalize the system." The right move is
"make the current family-analysis decision lane have one owner."

**[EUREKA]** Do not keep corpus-program decision derivation dependent on a
filesystem replay after analysis validation. That IO path is not additional
truth. It is just re-derivation risk.

### TODOS cross-reference

No current `TODOS.md` item blocks M37.

M37 must add three new deferred-extraction entries with explicit triggers:

1. generalized multi-wedge decision layer
2. cross-crate family-analysis shared core
3. public semantic fingerprint fields

### Completeness check

The complete version is:

- one semantic owner for basis snapshot derivation
- one semantic owner for decision-path activation
- one semantic owner for recommendation and corpus-decision proof fingerprints
- emitters and validators consuming the same owner
- old M36 regression anchors still green
- new M37 regressions added in the same PR
- docs and TODO triggers updated in the same PR

That is the lake. Ship the whole thing.

### Distribution check

No new artifact type is introduced. Existing consumers remain:

- `cargo xtask family recommend --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family validate-artifact ...`
- `.semantic-family-artifacts/family-promotion/analysis/*.latest.json`

No CI, packaging, or release-pipeline change is required for M37.

## Decision

M37 ships exactly one new internal module:

- `xtask/src/family/decision_kernel.rs`

That module becomes the single semantic owner for:

1. `CorpusProgramBasisSnapshot` derivation from a validated
   `FamilyRecommendationAnalysisArtifact`
2. helper-surface follow-on activation from validated analysis-basis truth
3. derived corpus-program decision contract assembly
4. normalized proof fingerprints for recommendation-analysis and
   corpus-program-decision artifacts

Everything else stays where it already belongs:

- `helper_surface.rs` keeps wedge-specific classification and frozen tuples
- `recommend.rs` keeps command wiring, candidate ranking, latest-artifact IO,
  and artifact emission
- `promotion_artifacts.rs` keeps serde types, schema validators, and artifact
  path checks
- `coverage.rs` keeps coverage artifact construction and coverage proof
  fingerprinting

This is the smallest complete M37. One new module, no new crate, no schema
bump, no generic registry.

## Target Architecture

### Post-M37 ownership

| File | Responsibility after M37 |
|---|---|
| `xtask/src/family/helper_surface.rs` | helper-surface classifier, fingerprint matcher, frozen durable-hold tuple, frozen follow-on tuple, exact tuple-match predicates |
| `xtask/src/family/decision_kernel.rs` | basis snapshot derivation, helper-surface activation, derived corpus-program decision contract, normalized recommendation fingerprint, normalized corpus-decision fingerprint |
| `xtask/src/family/recommend.rs` | candidate ranking, recommendation artifact assembly, latest-byte reuse, command entrypoints, artifact IO |
| `xtask/src/family/promotion_artifacts.rs` | serde schema types, path validation, sha validation, schema validation, delegation to kernel for expected semantic truth |
| `xtask/src/family/coverage.rs` | coverage artifact construction and coverage proof fingerprinting only |
| `xtask/src/lib.rs` | regression tests across emitter, validator, and fingerprint seams |

### Ownership rule

`decision_kernel.rs` is the only place allowed to answer:

- what the basis snapshot is
- whether the validated basis activates helper-surface follow-on
- what corpus-program decision contract follows from a validated basis
- what constitutes semantic identity for recommendation and corpus-decision
  artifacts

If `recommend.rs` or `promotion_artifacts.rs` needs one of those answers, it
must call the kernel. No duplicate derivation.

### Dependency graph

```text
unsupported clusters / coverage
            |
            v
    recommend.rs
      |    |
      |    +--> candidate ranking / analysis artifact write
      |
      +--> decision_kernel.rs
              |    |
              |    +--> basis snapshot derivation
              |    +--> helper-surface activation
              |    +--> derived corpus-program decision contract
              |    +--> normalized recommendation fingerprint
              |    +--> normalized corpus-decision fingerprint
              |
              +--> helper_surface.rs
                       |
                       +--> frozen helper tuples + classifier only
      |
      +--> promotion_artifacts.rs validators
               |
               +--> assert emitted artifacts match kernel-derived truth
```

### Command flow after M37

```text
cargo xtask family recommend --format json
    -> build_recommendation_analysis_artifact(...)
    -> decision_kernel::normalized_recommendation_proof_fingerprint(...)
    -> reuse or write recommendation.latest.json

cargo xtask family corpus-decision --format json
    -> load validated analysis basis
    -> decision_kernel::derive_corpus_program_decision_contract(...)
    -> decision_kernel::corpus_program_basis_snapshot(...)
    -> decision_kernel::normalized_corpus_program_decision_proof_fingerprint(...)
    -> reuse or write corpus-program-decision.latest.json

cargo xtask family validate-artifact <decision artifact>
    -> promotion_artifacts.rs schema/path checks
    -> decision_kernel::corpus_program_basis_snapshot(...)
    -> decision_kernel::derive_corpus_program_decision_contract(...)
    -> reject any artifact that contradicts kernel truth
```

## Locked Implementation Details

1. Add exactly one new module: `decision_kernel.rs`.
2. Keep `CorpusProgramBasisSnapshot` as the serde type in
   `promotion_artifacts.rs`, but move the derivation function into the kernel.
3. Move these symbols out of `recommend.rs`:
   - `DerivedCorpusProgramDecision`
   - `derive_corpus_program_decision_contract(...)`
   - `normalized_recommendation_proof_fingerprint(...)`
   - `normalized_corpus_program_decision_proof_fingerprint(...)`
4. Move `corpus_program_basis_snapshot(...)` out of
   `promotion_artifacts.rs`.
5. Remove the current coverage reread fallback from decision derivation.
   Kernel logic must operate only on the validated analysis artifact.
6. Keep helper-surface classifier and exact tuple helpers in
   `helper_surface.rs`. Do not turn that file into a generic engine.
7. Keep coverage proof fingerprinting in `coverage.rs`.
8. Do not introduce traits, builders, generics, or a policy registry. Plain
   module functions plus one small contract struct are enough.

## Implementation Plan

### Phase 1 - Establish the kernel boundary

Files:

- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`

Actions:

1. Create `decision_kernel.rs`.
2. Move `DerivedCorpusProgramDecision` and
   `derive_corpus_program_decision_contract(...)` into the kernel.
3. Move `corpus_program_basis_snapshot(...)` into the kernel.
4. Move recommendation and corpus-decision normalized fingerprint helpers into
   the kernel.
5. Remove basis-level activation logic from `helper_surface.rs`.
6. Keep `helper_surface.rs` limited to:
   - classifier input
   - helper-surface fingerprint matching
   - frozen durable-hold tuple
   - frozen follow-on tuple
   - exact tuple-match predicates

Done when:

- `decision_kernel.rs` compiles and is exported from `mod.rs`
- `helper_surface.rs` no longer owns basis snapshot or basis activation truth
- the kernel API is stable enough for downstream rewiring

### Phase 2 - Rewire recommendation emission

Files:

- `xtask/src/family/recommend.rs`

Actions:

1. Import basis snapshot derivation, decision derivation, and normalized
   fingerprint helpers from the kernel.
2. Keep `build_recommendation_analysis_artifact(...)` in `recommend.rs`.
3. Keep latest-artifact reuse in `effective_recommendation_bytes(...)` and
   `effective_corpus_program_decision_bytes(...)`.
4. Delete the hidden coverage reread path
   `helper_surface_disposition_from_coverage_basis(...)`.
5. Make corpus-program decision artifact assembly consume kernel-derived basis
   snapshot and kernel-derived decision contract.

Done when:

- `recommend.rs` owns IO and assembly, not semantic decision truth
- unchanged semantic inputs still reuse existing latest bytes
- decision derivation no longer depends on reading coverage from disk

### Phase 3 - Rewire artifact validation

Files:

- `xtask/src/family/promotion_artifacts.rs`

Actions:

1. Replace local basis-snapshot derivation with a kernel call.
2. Replace local helper-surface alignment reasoning with kernel-derived
   expected decision truth.
3. Keep path, sha, schema-version, and serde validation in
   `promotion_artifacts.rs`.
4. Preserve frozen helper-surface tuple exactness, but only as a
   kernel-produced expectation.

Done when:

- validators and emitters consume the same semantic owner
- contradictory decision artifacts are rejected for the same reason in both
  codepaths

### Phase 4 - Tests, docs, and deferred triggers

Files:

- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `TODOS.md`

Actions:

1. Preserve the current M36 regression floor.
2. Add M37 regressions for the moved kernel seams.
3. Update docs to say:
   - helper-surface classification still lives in `helper_surface.rs`
   - family-analysis decision truth now lives in `decision_kernel.rs`
   - normalized semantic fingerprints remain the proof surface
4. Add exact deferred-extraction TODO entries with trigger conditions.

Done when:

- docs no longer describe the old M36 boundary as the final state
- future extraction debates have explicit trigger-based backlog entries

## Code Quality Guardrails

1. One new module only.
2. No hidden filesystem rereads inside semantic derivation.
3. No widening from one wedge to a generalized registry.
4. No public schema changes.
5. No cross-crate extraction.
6. No duplicate truth between emitter and validator after the refactor.
7. Keep the diff explicit. Free functions beat a new object model here.

## Test Review

### Test framework

This repo already uses Rust `cargo test` coverage for `xtask`, with targeted
regressions in `xtask/src/lib.rs`.

M37 must extend that existing harness. Do not add a second test runner.

### Existing regression anchors that must stay green

- `xtask/src/lib.rs:3472`
  `corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on`
- `xtask/src/lib.rs:3505`
  `corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_missing`
- `xtask/src/lib.rs:3526`
  `corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_stale`
- `xtask/src/lib.rs:3892`
  `recommendation_proof_fingerprint_is_stable_across_generated_at_churn`
- `xtask/src/lib.rs:3916`
  `corpus_decision_proof_fingerprint_changes_on_semantic_action_change`
- `xtask/src/lib.rs:4031`
  `artifact_schema_rejects_corpus_decision_with_contradictory_action_for_helper_surface_basis`

These are the M36 floor. M37 is not allowed to weaken them.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] decision_kernel.rs
    |
    ├── [REGRESSION] derive basis snapshot from validated analysis basis
    │   └── [GAP] add exact snapshot derivation test
    │
    ├── [TESTED] helper-surface durable hold -> architecture follow-on
    │   └── xtask/src/lib.rs existing regression
    │
    ├── [TESTED] missing evidence -> spend corpus run 1
    │   └── xtask/src/lib.rs existing regression
    │
    ├── [TESTED] stale evidence -> spend corpus run 1
    │   └── xtask/src/lib.rs existing regression
    │
    ├── [GAP] ready candidate -> family promotion run
    ├── [GAP] blocked non-helper candidate -> recommendation policy run
    ├── [GAP] no candidate -> stop
    ├── [TESTED] recommendation fingerprint ignores churn-only fields
    │   └── xtask/src/lib.rs existing regression
    ├── [GAP] corpus-decision fingerprint ignores generated_at churn
    └── [TESTED] corpus-decision fingerprint changes on semantic action drift
        └── xtask/src/lib.rs existing regression

[+] promotion_artifacts.rs validator
    |
    ├── [TESTED] contradictory helper-surface action rejected
    ├── [GAP] drifted basis_snapshot rejected even when tuple is internally consistent
    └── [GAP] ready-path decision rejected if pivot/action mismatch kernel expectation

[+] recommend.rs latest artifact reuse
    |
    ├── [TESTED] recommendation byte reuse is gated by semantic fingerprint
    └── [GAP] corpus-decision byte reuse remains stable after kernel move
```

### Maintainer flow coverage

```text
MAINTAINER FLOW COVERAGE
===========================
[+] cargo xtask family recommend --format json
    ├── [GAP] unchanged semantic inputs reuse existing latest bytes
    └── [GAP] kernel move does not change ranked output or blocker vocabulary

[+] cargo xtask family corpus-decision --format json
    ├── [REGRESSION] helper-surface wedge still emits architecture follow-on
    ├── [GAP] no filesystem replay after validated analysis load
    └── [GAP] unchanged semantic inputs reuse existing latest bytes

[+] cargo xtask family validate-artifact <decision artifact>
    ├── [TESTED] contradictory helper-surface action rejected
    ├── [GAP] drifted basis snapshot rejected
    └── [GAP] ready-path contradiction rejected
```

### Required new tests

Add these tests in `xtask/src/lib.rs` using the existing helper fixture
patterns:

1. `corpus_program_basis_snapshot_matches_validated_analysis_basis`
2. `corpus_decision_ready_candidate_maps_to_family_promotion_run`
3. `corpus_decision_blocked_non_helper_candidate_maps_to_policy_run`
4. `corpus_decision_without_candidate_stops`
5. `corpus_decision_proof_fingerprint_is_stable_across_generated_at_churn`
6. `artifact_schema_rejects_corpus_decision_with_drifted_basis_snapshot`
7. `artifact_schema_rejects_ready_path_with_architecture_follow_on_tuple`
8. `corpus_decision_latest_bytes_are_reused_when_semantic_fingerprint_is_unchanged`

### Verification commands

Run at minimum:

```bash
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --exact
cargo test -p xtask artifact_schema_rejects_corpus_decision_with_contradictory_action_for_helper_surface_basis -- --exact
cargo test -p xtask recommendation_proof_fingerprint_is_stable_across_generated_at_churn -- --exact
cargo test -p xtask corpus_decision_proof_fingerprint_changes_on_semantic_action_change -- --exact
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

## Failure Modes Registry

| Codepath | Real production failure | Test required | Error handling required | User-visible impact if missed |
|---|---|---|---|---|
| Basis snapshot derivation | snapshot fields drift from validated analysis basis | yes | validator rejects exact mismatch | maintainer sees a contradictory artifact that looks valid |
| Helper-surface activation | missing or stale evidence still pivots to architecture follow-on | yes | derived decision must fall back to `spend_corpus_run_1` | repo recommends the wrong next milestone |
| Recommendation fingerprint | churn-only field changes cause byte churn | yes | latest-byte reuse must still short-circuit | proof surface becomes noisy and non-durable |
| Corpus-decision fingerprint | semantic field accidentally normalized away | yes | fingerprint must change on action or basis drift | proof surface misses a real decision change |
| Validator delegation | emitter and validator derive different actions | yes | both paths must call the same kernel | false green validation on contradictory artifacts |
| Docs and TODO boundary | future work reopens M37 as if it were unfinished | no runtime test | exact trigger-based docs and TODO entries | repeated architecture churn and review debt |

Current critical gaps before M37 lands:

1. semantic decision truth is still duplicated
2. corpus-program decision derivation still hides avoidable filesystem replay

M37 is not done until both are removed.

## Performance Review

There is no database or request-path work here. The performance concerns are
maintainability and avoidable IO:

1. Remove the coverage reread from decision derivation. Validated analysis
   truth is enough.
2. Keep normalized fingerprinting as serialize-and-hash once per artifact.
3. Keep latest-byte reuse so unchanged semantic artifacts do not churn disk
   output.
4. Do not add a second normalization pass or second artifact parse in the hot
   path.

## TODOS.md updates required in the same PR

Add these entries exactly once M37 lands:

1. **Generalized multi-wedge decision layer**
   Trigger: add a second durable non-promotable wedge whose decision path
   cannot be expressed in `decision_kernel.rs` without branching beyond the
   current helper-surface contract.
2. **Cross-crate family-analysis shared core**
   Trigger: at least two non-`recommend.rs` / non-`promotion_artifacts.rs`
   consumers inside `xtask/src/family/` need the same kernel logic, or a
   non-`xtask` crate needs the same decision semantics.
3. **Public semantic fingerprint fields**
   Trigger: an external consumer needs first-class semantic fingerprint fields
   in emitted JSON, not just internal normalized proof gating.

## NOT in scope

- moving family-analysis policy into `spec-core`
- changing any artifact schema version
- changing any public CLI flag or command surface
- redesigning coverage artifact shape
- introducing a generic decision engine or registry
- widening beyond the current helper-surface wedge
- changing the frozen helper-surface strings
- removing operational fields like `generated_at` or `inventory_path` from
  artifacts

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Kernel foundation | `xtask/src/family/decision_kernel.rs`, `xtask/src/family/mod.rs`, `xtask/src/family/helper_surface.rs` | - |
| B. Recommendation rewiring | `xtask/src/family/recommend.rs` | A |
| C. Validator rewiring | `xtask/src/family/promotion_artifacts.rs` | A |
| D. Regression tests | `xtask/src/lib.rs` | B, C |
| E. Docs and deferred triggers | `semantic-families/README.md`, `docs/`, `TODOS.md`, `PLAN.md` | A |

### Parallel lanes

- Lane A: `A`
  Foundation lane. Must land first because it freezes the kernel API.
- Lane B: `B`
  Recommendation command rewiring. Can run in parallel with Lane C after A.
- Lane C: `C`
  Artifact-validator rewiring. Can run in parallel with Lane B after A.
- Lane D: `D`
  Regression lane. Starts after B and C merge because it validates the final
  boundary.
- Lane E: `E`
  Docs and TODO lane. Can start after A once the module name and ownership are
  frozen.

### Execution order

1. Launch Lane A first.
2. After A is merged or otherwise frozen, launch Lanes B and C in parallel
   worktrees.
3. After A is frozen, Lane E may also start in parallel.
4. Merge B and C.
5. Launch Lane D against the merged code surface.
6. Run the full verification floor after D lands.

### Conflict flags

- Lanes B and C both depend on the exact exported API from
  `decision_kernel.rs`. Freeze function names and signatures in Lane A before
  parallelizing.
- Lane D depends on both B and C because `xtask/src/lib.rs` tests cover both
  emitter and validator behavior.
- Lane E must not invent new terminology. Docs must follow the boundary frozen
  in Lane A.

## Acceptance Criteria

M37 is done only when all of these are true:

1. `decision_kernel.rs` exists and is the single semantic owner for basis
   snapshot derivation, decision derivation, and recommendation and
   corpus-decision proof fingerprints.
2. `helper_surface.rs` contains wedge-specific classifier and frozen tuples
   only.
3. `recommend.rs` no longer owns corpus-program decision semantics or
   recommendation and corpus-decision proof-fingerprint helpers.
4. `promotion_artifacts.rs` validators derive expected decision truth through
   the kernel.
5. The helper-surface wedge still emits the exact frozen M36 outcome.
6. Recommendation-analysis and corpus-program-decision latest artifacts still
   reuse bytes when semantic fingerprints are unchanged.
7. Existing M36 regression anchors remain green.
8. New M37 regression tests land in `xtask/src/lib.rs`.
9. `semantic-families/README.md`, the program guide, the capability guide, and
   `TODOS.md` all describe the new boundary truthfully.

## Completion Summary

- Step 0: Scope challenge, accepted with bounded reduction
- Architecture: one new module, zero new crates, zero schema changes
- Code quality: duplicated semantic ownership removed
- Test review: diagram produced, 8 required regression additions
- Performance review: hidden coverage reread removed, latest-byte reuse preserved
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 3 exact trigger-based entries required
- Failure modes: 2 current critical gaps, both closed by the extraction
- Parallelization: 5 steps, 3 post-foundation lanes can run in parallel, 2 hard sequential gates
- Lake score: 6/6 complete-path decisions chosen
