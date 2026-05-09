<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260508-105337.md -->
# Shared-Core Portability Follow-On for the Family-Analysis Decision Seam

Status: **authority plan**
Milestone family: **operator-consumer-tooling**
Implementation readiness: **ready-now**
Next artifact kind: **authority_plan**
Autoplan ready: **yes**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-09**
Source design doc: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-132105.md`**
Supersedes: **the prior `PLAN.md` for `feat/m40-plus`, which was the selector-contract hardening authority artifact**

## Executive Verdict

The selector-contract hardening work is done. The next honest move is a bounded internal architecture pass:
make the family-analysis shared seam explicit inside `xtask/src/family`, keep repo-local CLI and artifact IO
local, and stop letting `recommend.rs` act like the reusable facade.

This is not a crate-extraction milestone. It is not a corpus-expansion milestone. It is not second-language
work. It is a small internal seam extraction that makes the already-existing pure decision logic explicit,
directly imported, and regression-protected.

If this plan grows into new command paths, new JSON schemas, repo-root-independent packaging, or broader
family-module cleanup, it has already missed the point.

## Design Basis

This plan is derived from the approved design doc at:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-132105.md`

That design doc establishes the governing truth:

- the helper-surface classifier is already stable in `xtask/src/family/helper_surface.rs`
- the follow-on decision contract is already stable in `xtask/src/family/decision_kernel.rs`
- normalized proof fingerprints already exist, but are split across `decision_kernel.rs` and `coverage.rs`
- `recommend.rs` still mixes pure semantics with repo-local orchestration and write behavior
- the right first portability cut is an internal seam inside `xtask`, not a new crate

## Live Repo Basis

This plan is grounded in the live tree, not in theory.

Validated module surface:

- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/paths.rs`

Observed current facts:

- `helper_surface.rs` owns the durable helper-surface classification plus the frozen follow-on tuple.
- `decision_kernel.rs` owns basis snapshot derivation, follow-on activation, and normalized recommendation
  and corpus-decision proof fingerprints.
- `coverage.rs` still owns `normalized_coverage_proof_fingerprint`, which means the proof surface is split.
- `recommend.rs` imports proof helpers from both `coverage.rs` and `decision_kernel.rs`, and also re-exports
  semantic helpers from `decision_kernel.rs`, which makes a command adapter look like the shared facade.
- `verify.rs` and `promotion_artifacts.rs` already consume the same underlying semantic truth, so the seam is
  already multi-consumer inside `xtask`.

Recent history supports this boundary choice:

- `b9f3bb2` refreshed the prior M40+ selector-contract plan.
- `ed93e45` landed the corpus-expansion and decision-contract stack.

This plan is the follow-on that stabilizes that work without widening scope.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner | Plan decision |
|---|---|---|
| helper-surface durable-hold classification | `xtask/src/family/helper_surface.rs` | keep logic, move behind explicit seam |
| frozen architecture-follow-on tuple | `xtask/src/family/helper_surface.rs` | keep exact values, move behind explicit seam |
| basis snapshot projection | `xtask/src/family/decision_kernel.rs` | keep logic, move behind explicit seam |
| corpus-program decision derivation | `xtask/src/family/decision_kernel.rs` | keep logic, move behind explicit seam |
| normalized recommendation proof fingerprint | `xtask/src/family/decision_kernel.rs` | keep logic, move into shared proof-fingerprint home |
| normalized corpus-decision proof fingerprint | `xtask/src/family/decision_kernel.rs` | keep logic, move into shared proof-fingerprint home |
| normalized coverage proof fingerprint | `xtask/src/family/coverage.rs` | move into the same shared proof-fingerprint home |
| latest-artifact reuse and write behavior | `xtask/src/family/recommend.rs` plus `paths.rs` | keep local, do not pull into shared core |
| artifact validation and schema types | `xtask/src/family/promotion_artifacts.rs` | keep local, consume shared semantics directly |

### Minimum complete change

This milestone is complete only if all of the following land together:

1. A single explicit internal seam exists under `xtask/src/family/analysis_core/` for pure helper-surface,
   decision-contract, and proof-fingerprint logic.
2. `normalized_coverage_proof_fingerprint` moves into that seam so all normalized proof helpers live
   together.
3. `recommend.rs` stops re-exporting reusable semantic helpers.
4. `verify.rs` and `promotion_artifacts.rs` import shared helpers directly from `analysis_core`.
5. Repo-local concerns stay local:
   - command entrypoints
   - path constants
   - latest-artifact reuse
   - atomic writes
   - stdout formatting
6. The existing semantic outputs still hold on the current branch truth:
   - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
   - `decision_basis_code = "durable_non_promotable_helper_surface"`
   - `required_next_action = "author_architecture_follow_on_plan"`

Anything less leaves the seam half-declared. Anything more is scope creep.

### Complexity check

This stays below the overbuilt threshold if it is kept honest.

- Expected modules touched: `xtask/src/family/mod.rs`, `helper_surface.rs`, `decision_kernel.rs`,
  `coverage.rs`, `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`
- Expected new modules: exactly `xtask/src/family/analysis_core/mod.rs`,
  `analysis_core/helper_surface.rs`, `analysis_core/decision_contract.rs`,
  `analysis_core/proof_fingerprint.rs`
- New commands: `0`
- New artifact schemas: `0`
- New services or crates: `0`

Stop conditions:

- if implementation introduces a new crate, stop
- if implementation moves `Path`, `fs`, or repo-root logic into `analysis_core`, stop
- if implementation touches unrelated family modules like `inventory.rs`, `routing.rs`, `scaffold.rs`,
  `prove.rs`, or `smoke.rs` without a direct compile-time need, stop

### Search check

No framework or runtime built-in is being bypassed here. This is a repo-internal seam extraction, not a
new concurrency model or infrastructure choice. The correct bias is boring:

- reuse current Rust module boundaries
- preserve current artifact types
- preserve current command surfaces

### TODOS cross-reference

Existing TODOs do not block this milestone directly. The one relevant follow-up already exists:

- `Cross-crate family-analysis shared core`

This plan must not consume that TODO early. This milestone creates a clean internal seam so that later
cross-crate work, if it becomes honest, is smaller and safer.

### Completeness check

The complete version is still small.

The shortcut version would document the seam and leave proof helpers split, direct-consumer imports indirect,
and `recommend.rs` still acting as the accidental facade. That saves almost no effort and guarantees future
drift. With AI-assisted implementation, that is false economy. The complete move is to finish the seam now.

### Distribution check

No new artifact type is introduced. No new CI/CD or packaging work is required. Distribution is unchanged.

## NOT in scope

- creating a new crate for the shared seam
- changing CLI command names or flags
- changing JSON schema versions or artifact shapes
- moving repo-relative path constants out of `paths.rs`
- moving `write_bytes_atomically`, latest-artifact reuse, or stdout formatting into the shared seam
- changing candidate projection, ranking policy, or unsupported-cluster discovery rules unless required
  for compile correctness
- changing corpus manifests, routing contracts, or packet layout rules
- second-language execution or any TypeScript portability work
- broader cleanup of unrelated `xtask/src/family` modules

## Architecture Review

### Locked target boundary

The target structure is fixed. Do not improvise filenames during implementation.

```text
xtask/src/family/
  analysis_core/
    mod.rs
    helper_surface.rs
    decision_contract.rs
    proof_fingerprint.rs
  coverage.rs
  recommend.rs
  verify.rs
  promotion_artifacts.rs
  paths.rs
  mod.rs
```

### Exact ownership map

| Module | Owns after change | Must not own |
|---|---|---|
| `analysis_core/helper_surface.rs` | helper-surface types, helper-surface classification, frozen durable-hold tuple, frozen follow-on tuple, tuple-comparison helpers | repo paths, file IO, artifact loading, stdout |
| `analysis_core/decision_contract.rs` | basis snapshot projection, follow-on activation checks, derived corpus-program decision contract | repo paths, file IO, artifact writes, latest-artifact reuse |
| `analysis_core/proof_fingerprint.rs` | normalized coverage, recommendation, and corpus-decision proof fingerprint logic plus any private normalization-only helpers they need | workspace root, `Path`, `fs`, latest-artifact selection, stdout |
| `recommend.rs` | CLI entrypoints, coverage collection orchestration, latest-artifact reuse, file writes, ranking orchestration, JSON output writing | semantic re-exports, shared-facade ownership |
| `coverage.rs` | coverage collection, manifest/spec loading, latest coverage write path, timestamp generation, generic JSON rendering used by command adapters | ownership of normalized proof semantics |
| `verify.rs` | parity validation and verify-command reporting | backdoor semantic ownership through `recommend.rs` |
| `promotion_artifacts.rs` | schema types, validation helpers, artifact-specific error reporting | backdoor semantic ownership through `recommend.rs` |

### Exact move plan

| Current owner | Symbol(s) | Destination | Notes |
|---|---|---|---|
| `helper_surface.rs` | `HelperSurfaceSignal`, `HelperSurfaceDisposition`, `HelperSurfaceCandidateTuple`, `HelperSurfaceFollowOnDecisionTuple`, `classify_helper_surface`, `durable_non_promotable_helper_surface_candidate_tuple`, `recommendation_uses_helper_surface_durable_hold_tuple`, `recommendation_matches_helper_surface_durable_hold_tuple`, `helper_surface_follow_on_decision_tuple`, `decision_matches_helper_surface_follow_on_tuple` | `analysis_core/helper_surface.rs` | move without changing tuple values or behavior |
| `decision_kernel.rs` | `DerivedCorpusProgramDecision`, `corpus_program_basis_snapshot`, `basis_snapshot_requires_helper_surface_follow_on`, `basis_activates_helper_surface_follow_on`, `derive_corpus_program_decision_contract` | `analysis_core/decision_contract.rs` | keep API behavior identical |
| `decision_kernel.rs` | `normalized_recommendation_proof_fingerprint`, `normalized_corpus_program_decision_proof_fingerprint` | `analysis_core/proof_fingerprint.rs` | keep hashes identical |
| `coverage.rs` | `normalized_for_recommend_determinism`, `normalized_coverage_proof_fingerprint` | `analysis_core/proof_fingerprint.rs` | move normalization with the fingerprint so the proof surface lives in one place |
| `recommend.rs` | `pub(crate) use crate::family::decision_kernel::{...}` re-export block | delete | direct imports from `analysis_core` only |
| `family/mod.rs` | top-level module wiring | add `pub mod analysis_core;`, keep existing modules | no other module cleanup in this milestone |

Implementation constraint:

- `analysis_core` must not depend on `recommend.rs` or `coverage.rs`
- if a moved fingerprint helper currently relies on a private JSON serialization helper from `coverage.rs`,
  either move that private helper into `analysis_core/proof_fingerprint.rs` or duplicate the normalization-only
  helper there
- do not create a dependency edge where `analysis_core` calls back into a command adapter

### Dependency graph

```text
validated unsupported-cluster truth
            +
validated recommendation-analysis truth
            +
validated corpus-decision truth
                    |
                    v
      xtask/src/family/analysis_core/
      +------------------------------+
      | helper_surface               |
      | decision_contract            |
      | proof_fingerprint            |
      +------------------------------+
         |            |            |
         |            |            |
         v            v            v
   recommend.rs   verify.rs   promotion_artifacts.rs
   command/path    parity      artifact/schema
   orchestration   checks      enforcement

outside the seam:
  paths.rs
  fs writes
  stdout formatting
  repo-relative latest-artifact selection
```

### Production failure scenarios

| Surface | Realistic failure | Covered by plan |
|---|---|---|
| helper-surface classifier | a later refactor widens the classifier and starts tagging non-helper candidates as durable holds | yes, preserve exact tuple and classification tests |
| decision derivation | helper-surface follow-on activates even when evidence is stale or missing | yes, keep verifier parity tests and no-bad-activation tests |
| proof fingerprint seam | coverage fingerprint normalization drifts from recommendation and corpus-decision normalization rules | yes, move all three into one seam and test stability together |
| command adapter boundary | `recommend.rs` keeps serving as the shared facade and downstream consumers continue importing through it | yes, delete semantic re-exports and force direct imports |
| portability claim | a pure seam starts taking `Path` or `fs` and becomes fake portability | yes, explicit API boundary forbids repo-root knowledge |

### Architecture decision

Recommendation: keep the new seam internal to `xtask/src/family` and do not extract a crate yet.

Why:

- it matches the current number of consumers
- it minimizes diff size
- it keeps the change reversible
- it avoids spending an innovation token on packaging before the seam is settled

## Code Quality Review

### Problems this plan removes

1. **Split proof surface**
   - today the normalized proof helpers are split across `coverage.rs` and `decision_kernel.rs`
   - after this milestone, all three normalized proof helpers live in `analysis_core/proof_fingerprint.rs`

2. **Wrong facade**
   - today `recommend.rs` re-exports semantic helpers from `decision_kernel.rs`
   - after this milestone, every consumer imports shared semantics directly from `analysis_core`

3. **Implicit seam**
   - today the seam exists, but only maintainers who have read the whole stack know where it is
   - after this milestone, the module tree says exactly where pure family-analysis semantics live

### Implementation rules

- move functions before rewriting functions
- do not mix structural refactor with semantic behavior changes
- preserve frozen helper-surface tuple names and contract values exactly
- keep imports direct and boring
- do not create a second abstraction layer on top of `analysis_core`
- do not move generic helpers like timestamping or final-output JSON rendering unless the new seam
  would otherwise depend back on a command adapter

### Existing patterns to preserve

- validation-before-use in artifact loads
- normalized proof fingerprint reuse to avoid byte-churn writes
- explicit enum-based decision contracts in `promotion_artifacts.rs`

## Test Review

### Test framework and proof loop

This is a Rust repo. The required proof loop for this milestone is:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Expected preserved outputs:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- `helper_surface_not_promotable` remains the decisive blocker
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `required_next_action = "author_architecture_follow_on_plan"`

### Existing coverage baseline

The existing suite already covers much of the seam:

- `family::helper_surface::tests::*`
- `family::decision_kernel::tests::*`
- `family::verify::tests::*`
- top-level `tests::coverage_proof_fingerprint_*`
- top-level `tests::corpus_decision_*`
- top-level `tests::recommendation_*`

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[~] analysis_core/helper_surface.rs
    |
    |-- classify_helper_surface()
    |   |-- [PASS TESTED] durable helper-surface match
    |   `-- [PASS TESTED] non-matching rejection
    |
    |-- durable_non_promotable_helper_surface_candidate_tuple()
    |   `-- [PASS TESTED] exact frozen tuple match
    |
    `-- helper_surface_follow_on_decision_tuple()
        `-- [PASS TESTED] exact frozen decision tuple match

[~] analysis_core/decision_contract.rs
    |
    |-- corpus_program_basis_snapshot()
    |   `-- [PASS TESTED] basis snapshot parity
    |
    |-- basis_snapshot_requires_helper_surface_follow_on()
    |   |-- [PASS TESTED] exact positive case
    |   `-- [PASS TESTED] stale-evidence rejection
    |
    `-- derive_corpus_program_decision_contract()
        |-- [PASS TESTED] helper-surface architecture follow-on
        |-- [PASS TESTED] recommended candidate maps to promotion run
        |-- [PASS TESTED] missing/stale evidence maps to spend corpus run 1
        |-- [PASS TESTED] non-helper blocked case maps to policy run
        `-- [PASS TESTED] no candidate maps to stop

[!] analysis_core/proof_fingerprint.rs
    |
    |-- normalized_recommendation_proof_fingerprint()
    |   `-- [PASS TESTED] generated_at churn ignored
    |
    |-- normalized_corpus_program_decision_proof_fingerprint()
    |   |-- [PASS TESTED] generated_at churn ignored
    |   `-- [PASS TESTED] semantic action change changes fingerprint
    |
    `-- normalized_coverage_proof_fingerprint()
        |-- [GAP] relocation regression proving identical hash after move
        `-- [PASS TESTED] generated_at and inventory-path churn ignored

[!] recommend.rs
    |
    |-- effective_coverage_for_recommend()
    |   `-- [GOOD TESTED] semantic fingerprint reuse for unchanged coverage
    |
    |-- effective_recommendation_bytes()
    |   `-- [GOOD TESTED] semantic fingerprint reuse for unchanged recommendation
    |
    `-- semantic re-export removal
        `-- [GAP] compile-level regression coverage after direct imports

[!] verify.rs + promotion_artifacts.rs
    |
    `-- direct import migration to analysis_core
        |-- [GAP] direct-consumer compile coverage after seam extraction
        `-- [GAP] regression proving artifact validation still matches derived contract

---------------------------------
COVERAGE: strong semantic baseline exists
MAIN GAPS:
1. relocation regression for the moved coverage fingerprint helper
2. compile-level migration coverage after import rewiring
3. direct-consumer parity coverage after seam extraction
4. seam-purity guard against repo-path or file-IO leakage
---------------------------------
```

### Required test additions

| ID | Location | Type | Required assertion |
|---|---|---|---|
| TR-1 | `analysis_core/proof_fingerprint.rs` tests | unit regression | moved coverage fingerprint returns the same hash for the same semantic input as before the move |
| TR-2 | `verify.rs` tests | unit regression | verify-command parity still derives the same basis snapshot and follow-on decision after direct import rewiring |
| TR-3 | `promotion_artifacts.rs` tests | unit regression | artifact validation still matches the derived contract after direct import rewiring |
| TR-4 | `recommend.rs` tests | unit regression | unchanged semantic fingerprints still reuse prior bytes and avoid rewrite churn |
| TR-5 | `analysis_core` export surface review plus test where practical | compile/API guard | exported seam functions operate on typed artifacts and in-memory values, not workspace-root or file-system inputs |

Regression rule:

- TR-1 through TR-4 are mandatory
- TR-5 is mandatory as an acceptance condition, even if enforced partly by API review rather than a perfect runtime test

### Failure-mode coverage

| Code path | Failure mode | Test required | Error handling required | User-visible outcome |
|---|---|---|---|---|
| moved coverage fingerprint helper | fingerprint changes after a pure move | yes, critical regression | n/a | otherwise maintainers see false artifact churn |
| direct consumer imports | consumer compiles against stale import path | yes | compiler catches it | hard failure, not silent |
| helper-surface follow-on derivation | stale evidence still activates follow-on | already covered, keep | yes | avoids false architectural pivot |
| proof reuse in command paths | unchanged artifact rewrites anyway | yes | current reuse path should remain | otherwise noisy artifact diffs |
| seam purity boundary | `analysis_core` starts requiring repo-root knowledge | yes, by API guard | n/a | otherwise the portability claim becomes false |

Critical gap rule:

- no seam change may land without either preserving an existing regression test or adding one when ownership
  moves across modules

## Performance Review

This is not a runtime hot-path milestone, but there are still two performance rules:

1. Keep semantic fingerprint reuse intact so unchanged artifacts do not rewrite needlessly.
2. Do not add extra disk reads or duplicate full-artifact serialization passes just because functions moved.

Specific watchpoints:

- `effective_coverage_for_recommend()` must continue short-circuiting on identical semantic fingerprints
- `effective_recommendation_bytes()` and `effective_corpus_program_decision_bytes()` must continue returning
  prior bytes when the normalized fingerprint is unchanged
- the new seam should move code, not add new cloning or normalization passes beyond current behavior

No new caching layer is justified here.

## Failure Modes Registry

| ID | Failure mode | Severity | Guardrail |
|---|---|---|---|
| FM-1 | `analysis_core` grows repo-path or file-IO arguments | high | keep seam API pure, reject `Path` and `fs` dependencies |
| FM-2 | coverage fingerprint helper moves but semantic hash changes | high | mandatory TR-1 regression before merge |
| FM-3 | `recommend.rs` still acts as semantic facade through re-exports | medium | delete semantic re-exports and import directly |
| FM-4 | helper-surface follow-on starts activating when evidence is stale or missing | high | preserve verifier parity and stale-evidence tests |
| FM-5 | plan quietly widens into crate extraction or command redesign | high | enforce explicit out-of-scope boundary |

## Implementation Plan

### Step 1: Add the seam skeleton and module wiring

Touch:

- `xtask/src/family/mod.rs`
- `xtask/src/family/analysis_core/mod.rs`

Required work:

- add `pub mod analysis_core;` in `family/mod.rs`
- create `analysis_core/mod.rs`
- wire exactly three child modules:
  - `helper_surface`
  - `decision_contract`
  - `proof_fingerprint`
- export only the symbols needed by `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs`

Exit criteria:

- the new module tree compiles
- no consumer has been rewired yet
- no semantic logic has changed

### Step 2: Move helper-surface semantics into `analysis_core/helper_surface.rs`

Touch:

- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`

Required work:

- move the helper-surface structs, enums, tuple builders, and tuple-comparison helpers
- preserve all frozen tuple values exactly
- leave behind either:
  - no top-level `helper_surface.rs`, if imports are fully rewired immediately, or
  - a short-lived compatibility shim only inside the same PR while the tree still compiles

Exit criteria:

- all helper-surface logic now lives under `analysis_core/helper_surface.rs`
- the frozen durable-hold and follow-on tuples still match existing tests exactly

### Step 3: Move decision derivation into `analysis_core/decision_contract.rs`

Touch:

- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`

Required work:

- move `DerivedCorpusProgramDecision`
- move basis snapshot projection
- move helper-surface follow-on activation checks
- move `derive_corpus_program_decision_contract`

Exit criteria:

- decision derivation logic lives under `analysis_core/decision_contract.rs`
- the derived contract for the current branch truth remains unchanged

### Step 4: Move proof fingerprint ownership into `analysis_core/proof_fingerprint.rs`

Touch:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`

Required work:

- move `normalized_recommendation_proof_fingerprint`
- move `normalized_corpus_program_decision_proof_fingerprint`
- move `normalized_for_recommend_determinism`
- move `normalized_coverage_proof_fingerprint`
- keep any required normalization-only helper private to `proof_fingerprint.rs`
- do not make `analysis_core` depend back on `coverage.rs`

Exit criteria:

- all three normalized proof helpers live behind one seam
- their hashes are unchanged for identical semantic input

### Step 5: Rewire consumers to import `analysis_core` directly

Touch:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/mod.rs`

Required work:

- delete the semantic re-export block from `recommend.rs`
- import helper-surface, decision-contract, and proof-fingerprint helpers directly from `analysis_core`
- keep `recommend.rs` as the command adapter for:
  - latest-artifact reuse
  - coverage collection orchestration
  - path coordination
  - write behavior

Exit criteria:

- no shared semantic helper is imported through `recommend.rs`
- `verify.rs` and `promotion_artifacts.rs` compile against `analysis_core` directly

### Step 6: Land regression tests and run the proof loop

Touch:

- `analysis_core/*` tests
- `recommend.rs` tests
- `verify.rs` tests
- `promotion_artifacts.rs` tests

Required work:

- add TR-1 through TR-4
- enforce TR-5 as an API guard
- run:
  - `cargo test -p xtask`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo xtask family verify-decision-contract --format json`

Exit criteria:

- tests pass
- command outputs remain semantically unchanged
- no artifact-churn regressions appear

## Worktree Parallelization Strategy

Default recommendation: implement this in one worktree sequentially.

Why: the milestone is small, the seam boundary is shared, and the highest-risk merge conflicts are not in
business logic, they are in module wiring and import churn. Splitting that badly saves almost no time.

That said, there is one honest parallel window after the seam shape is locked.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. seam skeleton and exports | `xtask/src/family/mod.rs`, `xtask/src/family/analysis_core/` | - |
| B. helper-surface + decision-contract move | `xtask/src/family/analysis_core/`, `helper_surface.rs`, `decision_kernel.rs` | A |
| C. consumer rewires | `xtask/src/family/recommend.rs`, `verify.rs`, `promotion_artifacts.rs` | B |
| D. proof-fingerprint move | `xtask/src/family/analysis_core/`, `coverage.rs`, `decision_kernel.rs`, `recommend.rs` | B |
| E. regression tests + proof loop | `xtask/src/family/analysis_core/`, `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`, xtask tests | C + D |

### Parallel lanes

Safe default:

- Lane A: A -> B -> C -> D -> E

Optional worktree split after Step B is merged or otherwise interface-locked:

- Lane A: A -> B
- Lane B: C
- Lane C: D
- Lane D: E

Interpretation:

- Lane B and Lane C may run in parallel only after Step B locks the `analysis_core` API
- Lane D waits for both, because test updates need the final import graph and final proof-helper home

### Execution order

Recommended order:

1. Land Step A and Step B first. This establishes the seam and freezes its API.
2. If parallelizing, launch Lane B and Lane C in separate worktrees.
3. Merge Lane B and Lane C.
4. Run Lane D last for regression tests and the proof loop.

If there is any doubt, skip the split and do A -> B -> C -> D -> E in one worktree.

### Conflict flags

- Lane A conflicts with everything. It defines the seam and the export surface.
- Lane B and Lane C both depend on the same `analysis_core` API. If that API is still moving, parallel work is
  fake progress.
- Lane C and Lane D both touch `recommend.rs`. If a single implementer owns both, keep them sequential.
- Lane E must be last. It verifies the settled graph. It is not a productive early-parallel lane.

## Acceptance Criteria

This plan is done only when all of the following are true:

1. `xtask/src/family/analysis_core/` exists with exactly:
   - `mod.rs`
   - `helper_surface.rs`
   - `decision_contract.rs`
   - `proof_fingerprint.rs`
2. Helper-surface durable-hold classification still yields the exact frozen tuple.
3. Corpus-program decision derivation still yields the same architecture-follow-on contract.
4. Coverage, recommendation, and corpus-decision proof fingerprints all live behind one shared seam.
5. `recommend.rs` still owns CLI, repo-path, latest-artifact, and write behavior.
6. `verify.rs` and `promotion_artifacts.rs` consume shared semantics directly from `analysis_core`.
7. `recommend.rs` no longer re-exports shared semantic helpers.
8. `analysis_core` exports do not require workspace-root, `Path`, or file-system inputs.
9. `cargo test -p xtask` passes.
10. The three family-analysis command outputs above remain semantically unchanged on the current branch truth.

## Completion Summary

- Step 0: Scope Challenge, scope accepted as a bounded internal seam extraction
- Architecture Review: locked target boundary, exact ownership map, exact move plan
- Code Quality Review: 3 concrete issues removed, split proof surface, wrong facade, implicit seam
- Test Review: code path coverage diagram produced, 5 required regression guards identified
- Performance Review: 2 guardrails, preserve fingerprint-based write reuse and avoid extra serialization
- NOT in scope: written
- What already exists: written in Step 0
- Failure modes: 5 concrete failure modes captured
- Parallelization: included, default sequential with one optional parallel window after API lock
- Lake Score: complete option chosen, because the shortcut leaves the seam half-declared
