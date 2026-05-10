# M42 - Decision-Contract Verifier Stop-State Parity

Status: **authority plan**  
Milestone family: **family-decision-contract-truth**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-09**  
Source design doc: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-195035.md`**  
Source test plan: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260509-195035.md`**  
Supersedes: **the branch-local M41 helper-route authority plan**

## Executive Verdict

M42 is a narrow verifier-truth repair.

The live decision kernel already tells the truthful post-M41 story: there is no actionable next family move, so the correct outcome is a stop-state. The only broken surface is `cargo xtask family verify-decision-contract --format json`, which still freezes a retired helper-surface follow-on tuple and therefore fails on truthful HEAD artifacts.

This milestone fixes that verifier drift, centralizes the frozen floor beside the decision-contract seam that already owns the live truth, and adds enough regression coverage that this exact mismatch cannot silently return.

## Problem Statement

The branch currently has a split-brain maintainer story:

```text
recommend                    -> truthful stop-state
corpus-decision              -> truthful stop-state
verify-decision-contract     -> retired helper-surface floor -> FAIL
```

The live truthful state on this branch is:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `open_blockers = []`
- `missing_evidence = []`
- `stale_evidence = []`
- `decision_action = stop`
- `decision_basis_code = no_actionable_candidate`
- `required_next_action = record_stop_without_new_milestone`

Today, `xtask/src/family/verify.rs` still hard-codes the old helper-surface floor in its frozen-floor check. That means the verifier rejects the same repo state that the live kernel says is correct.

This is a consumer-parity bug, not a recommendation-policy bug.

## Repo Truth Basis

### Code truth

- `xtask/src/family/analysis_core/decision_contract.rs` already owns `derive_corpus_program_decision_contract()`.
- `xtask/src/family/analysis_core/mod.rs` already exists as the shared export seam for analysis-core helpers.
- `xtask/src/family/verify.rs` still owns a verifier-local frozen floor through `frozen_helper_surface_floor_result()`.
- `xtask/src/family/verify.rs` test seeding still assumes the retired helper-surface durable-hold path.
- `xtask/src/lib.rs` already covers CLI dispatch for `family verify-decision-contract --help` and non-JSON format rejection.

### Command truth

The branch-local command story that this plan must preserve is:

```text
cargo xtask family recommend --format json
  -> recommendation_status = "insufficient_real_corpus"
  -> decision_status = "not_recommended"
  -> open_blockers = []

cargo xtask family corpus-decision --format json
  -> decision_action = "stop"
  -> decision_basis_code = "no_actionable_candidate"
  -> required_next_action = "record_stop_without_new_milestone"

cargo xtask family verify-decision-contract --format json
  -> currently fails because the verifier still expects the retired helper-surface tuple
```

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing owner | M42 decision |
|---|---|---|
| Live decision derivation | `xtask/src/family/analysis_core/decision_contract.rs` | reuse directly, do not replace |
| Shared analysis-core seam | `xtask/src/family/analysis_core/mod.rs` | extend exports only if needed |
| Maintainer verifier | `xtask/src/family/verify.rs` | refresh to truthful stop-state parity |
| Verifier fixture seeding | `xtask/src/family/verify.rs` tests | reseed to truthful stop-state |
| CLI dispatch contract | `xtask/src/lib.rs` | keep command name and `--format json` behavior unchanged |
| Live proof loop | `recommend`, `corpus-decision`, `verify-decision-contract` | reuse as acceptance harness |

### Minimum complete change

M42 is complete only if all of this lands together:

1. The frozen expected stop-state is authored once, beside the decision-contract seam that already derives the live truth.
2. `verify.rs` consumes that shared stop-state contract instead of a verifier-local helper-surface literal tuple.
3. `verify.rs` tests seed the truthful stop-state and still prove fail-paths for real contract drift.
4. CLI dispatch behavior stays unchanged, with `xtask/src/lib.rs` touched only if existing command-facing assertions require it.
5. The live three-command maintainer loop passes on HEAD.

### Complexity check

This is intentionally a small wedge. The expected implementation surface is:

- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/lib.rs` only if command-surface tests need refresh

That is 3 required files, 1 conditional file, and no new module tree. If implementation starts touching `recommend.rs`, `decision_kernel.rs`, `promotion_artifacts.rs`, or schema versioning code, the milestone has drifted out of scope.

### Search check

`[Layer 1]` The repo already has the seam we should use.

The built-in solution here is the existing `analysis_core` decision-contract layer, not a new constants module, not a schema bump, and not another verifier-local tuple. This is a reuse problem, not a new architecture problem.

### TODOS cross-reference

Existing deferred work in [TODOS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md) does not block M42:

- `Generalized multi-wedge decision layer`
- `Cross-crate family-analysis shared core`

M42 must not partially do either one. It should only remove the stale verifier-local branch so those later follow-ons, if ever needed, start from a truthful base.

### Completeness check

Reject the shortcut where we only swap literal values inside `verify.rs`.

That would make today's failure go green while leaving the root cause alive: the verifier would still own a second source of decision-contract truth. The complete version is still a small lake, and it costs almost nothing more.

### Distribution check

No new binary, package, container, or release pipeline is introduced. Distribution is unchanged and out of scope.

## Architecture Contract

### Current to target

```text
CURRENT
  recommendation.latest.json -> truthful stop-state
  corpus-program-decision.latest.json -> truthful stop-state
  verify.rs -> frozen helper-surface floor -> FAIL

TARGET
  recommendation.latest.json -> truthful stop-state
  corpus-program-decision.latest.json -> truthful stop-state
  verify.rs -> shared frozen stop-state floor -> PASS
```

### Dependency graph

```text
FamilyRecommendationAnalysisArtifact
            |
            v
xtask/src/family/analysis_core/decision_contract.rs
  ├── corpus_program_basis_snapshot()
  ├── derive_corpus_program_decision_contract()
  └── NEW: frozen_truthful_stop_state_contract()
            |
            v
xtask/src/family/analysis_core/mod.rs
            |
            v
xtask/src/family/verify.rs
  ├── artifact validation
  ├── basis snapshot parity
  ├── derived decision parity
  └── frozen floor parity against shared stop-state
            |
            v
JSON report + CLI exit status
```

### Ownership map

| File | Owns after M42 | Must not own |
|---|---|---|
| `xtask/src/family/analysis_core/decision_contract.rs` | live decision derivation and the shared frozen truthful stop-state contract | verifier I/O, JSON rendering, CLI behavior |
| `xtask/src/family/analysis_core/mod.rs` | export surface for decision-contract helpers | duplicated contract logic |
| `xtask/src/family/verify.rs` | artifact loading, parity checks, report rendering, fixture seeding | independent policy truth about what the stop-state should be |
| `xtask/src/lib.rs` | CLI dispatch tests only | duplicated decision-contract expectations |

### Locked truthful stop-state

The shared frozen floor introduced by M42 must encode exactly this tuple:

```text
recommendation_status = insufficient_real_corpus
decision_status       = not_recommended
open_blockers         = []
missing_evidence      = []
stale_evidence        = []
decision_action       = stop
decision_basis_code   = no_actionable_candidate
required_next_action  = record_stop_without_new_milestone
```

There is no ambiguity here. These values are the contract.

### Public contract decision

M42 does **not** change the outward machine-readable verifier surface.

That means:

- keep the command name unchanged
- keep `--format json` as the only supported format
- keep the JSON check key `checks.frozen_helper_surface_floor`
- keep the failure reason `frozen_helper_surface_floor_mismatch`

Why:

- this is the minimal diff
- the user-facing problem is false verifier failure, not naming polish
- the repo search shows no need to widen this wedge into a public contract cleanup

### Internal naming decision

Internal Rust helper and test names should become honest stop-state names where doing so does not widen the public surface.

Allowed:

- rename local Rust helper names and local test names from helper-surface wording to stop-state wording
- add comments that explain the legacy outward JSON naming

Not allowed:

- changing outward JSON field names
- changing outward failure-reason strings
- turning M42 into a schema cleanup milestone

## Implementation Contract

### Step 1 - Add the shared frozen truthful stop-state contract

Files:

- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/mod.rs`

Do:

- add one explicit helper that returns the frozen post-M41 truthful stop-state contract
- place it beside `derive_corpus_program_decision_contract()`
- make the helper hold the exact fields listed in the locked tuple above
- export the helper through `analysis_core/mod.rs` if `verify.rs` consumes it through the seam

Do not:

- change `derive_corpus_program_decision_contract()`
- change recommendation policy
- add a second verifier-only copy of the same tuple

Done when:

- `decision_contract.rs` can answer both:
  - what the live kernel derives
  - what the verifier should freeze as the truthful stop-state floor

### Step 2 - Rewire the verifier to consume the shared stop-state

Files:

- `xtask/src/family/verify.rs`

Do:

- replace verifier-local literal comparisons in the frozen-floor check with comparisons against the shared truthful stop-state helper
- preserve the existing check structure:
  - recommendation analysis validation
  - corpus-program-decision validation
  - basis snapshot parity
  - derived decision parity
  - frozen floor parity
- keep pass/fail behavior unchanged:
  - JSON only
  - pass returns exit 0
  - fail returns the existing invalid-input exit path with JSON emitted
- rename internal helper/test identifiers to stop-state wording if that does not affect the public JSON surface

Do not:

- remove the frozen-floor check
- collapse the verifier into "derived parity is enough"
- rewrite the JSON report schema

Done when:

- `build_report()` passes on truthful stop-state artifacts
- the frozen-floor check still fails when any frozen field drifts

### Step 3 - Reseed verifier fixtures and regression tests

Files:

- `xtask/src/family/verify.rs`

Do:

- reseed `fixture_analysis_artifact()` to produce the truthful stop-state basis
- ensure `seeded_workspace()` writes a truthful stop-state decision artifact by deriving from that basis
- refresh or add tests that prove:
  - truthful stop-state passes
  - stale evidence fails
  - basis snapshot drift fails
  - derived decision drift fails
  - frozen stop-state floor drift fails
  - non-JSON format still rejects

Preferred local test naming:

- `verifier_passes_on_truthful_stop_state_floor`
- `verifier_reports_truthful_stop_state_floor_mismatch`

Do not:

- keep helper-surface-specific pass fixtures
- write vague fail tests that only assert "some mismatch happened"

Done when:

- the verifier fixture is no longer helper-surface-specific
- fail-path assertions describe real stop-state drift, not obsolete follow-on drift

### Step 4 - Audit CLI-dispatch coverage

Files:

- `xtask/src/lib.rs` only if needed

Do:

- rerun CLI tests that cover `family verify-decision-contract --help`
- rerun CLI tests that cover non-JSON format rejection
- update assertions only if the verifier refactor changed intentionally locked surfaced text

Do not:

- widen M42 into a general CLI wording cleanup
- touch `xtask/src/lib.rs` unless the existing dispatch tests require it

Done when:

- `xtask/src/lib.rs` is either untouched or only minimally refreshed for verifier dispatch assertions

## File-Level Change Contract

| File | Change required | Notes |
|---|---|---|
| `xtask/src/family/analysis_core/decision_contract.rs` | yes | add shared frozen truthful stop-state helper |
| `xtask/src/family/analysis_core/mod.rs` | likely | export the helper if needed |
| `xtask/src/family/verify.rs` | yes | consume helper, reseed fixtures, refresh tests |
| `xtask/src/lib.rs` | conditional | only if existing command-facing tests need updates |
| `xtask/src/family/recommend.rs` | no | policy is already truthful |
| `xtask/src/family/helper_surface.rs` | no | not part of this consumer-parity fix |
| `PLAN.md` | yes | this authority plan |
| `ORCH_PLAN.md` | no | no orchestration rewrite in M42 |

## Test Contract

### Test framework

This repo is Rust. The authoritative harness for M42 is:

- focused verifier coverage through `cargo test -p xtask verify`
- broader confirmation through `cargo test -p xtask`
- live maintainer proof through the three command loop

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/analysis_core/decision_contract.rs
    │
    ├── derive_corpus_program_decision_contract()
    │   ├── [EXISTING] recommended -> pivot_to_family_promotion_run
    │   ├── [EXISTING] blocked + missing/stale evidence -> spend_corpus_run_1
    │   ├── [EXISTING] blocked non-helper -> pivot_to_recommendation_policy_run
    │   └── [EXISTING] no actionable candidate -> stop
    │
    └── NEW frozen truthful stop-state helper
        ├── [MUST ADD] emits the locked truthful stop-state tuple
        └── [MUST ADD] agrees with derive() on the stop branch

[+] xtask/src/family/verify.rs
    │
    ├── recommendation_analysis_validation
    │   └── [EXISTING] missing/invalid artifact fail paths
    │
    ├── corpus_program_decision_validation
    │   └── [EXISTING] missing/invalid decision artifact fail paths
    │
    ├── basis_snapshot_parity_result()
    │   ├── [EXISTING] matching snapshot passes
    │   └── [EXISTING] drifted snapshot fails
    │
    ├── derived_decision_parity_result()
    │   ├── [EXISTING] matching derived decision passes
    │   └── [EXISTING] drifted derived decision fails
    │
    └── frozen floor parity
        ├── [MUST ADD] truthful stop-state passes
        ├── [MUST ADD] stale evidence fails
        ├── [MUST ADD] stop-state field drift fails
        └── [MUST ADD] retired helper-surface pass fixture is gone

[+] CLI DISPATCH
    │
    ├── [EXISTING] --help exits 0
    └── [EXISTING] --format yaml exits 2

[+] MAINTAINER FLOW
    │
    ├── [MUST PROVE] recommend -> corpus-decision -> verify loop agrees on stop-state
    └── [MUST PROVE] verify returns exit 0 on current branch truth

─────────────────────────────────
COVERAGE TARGET: 100% of changed verifier paths
REQUIRED GAPS TO FILL: 5
  1. shared frozen truthful stop-state helper test
  2. truthful stop-state verifier pass test
  3. frozen stop-state drift fail test
  4. stale-evidence fail test against stop-state fixture
  5. live three-command acceptance proof
─────────────────────────────────
```

### Required test additions

Add or refresh tests so every changed branch is proven:

1. `decision_contract.rs` unit test that the shared frozen helper returns the exact locked stop-state tuple.
2. `decision_contract.rs` unit test that the frozen helper and `derive_corpus_program_decision_contract()` agree for a truthful no-actionable-candidate basis.
3. `verify.rs` pass test for truthful stop-state artifacts.
4. `verify.rs` fail test that mutates each critical frozen field:
   - `recommendation_status`
   - `open_blockers`
   - `decision_action`
   - `decision_basis_code`
   - `required_next_action`
5. `verify.rs` fail test for stale evidence on otherwise truthful stop-state artifacts.
6. `xtask` command proof that `family verify-decision-contract --format yaml` still exits 2.

### Regression rule

This is a regression fix. A regression test is mandatory.

The broken behavior already exists on HEAD: truthful artifacts produced by the live kernel fail verifier parity because of an obsolete frozen tuple. M42 is not complete unless at least one test would have failed before the change and now passes after it.

### Manual proof loop

Run exactly:

```bash
cargo test -p xtask verify
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Expected final outcome:

- `recommend` emits `insufficient_real_corpus`
- `corpus-decision` emits `stop` and `record_stop_without_new_milestone`
- `verify-decision-contract` exits 0 and reports overall pass

## Failure Modes Registry

| Codepath | Real failure | Test covers it | Error handling exists | User sees | Critical gap before M42 lands |
|---|---|---|---|---|---|
| shared frozen truthful stop-state helper | helper returns stale or partial tuple | must add | compile-time + unit-test boundary only | verifier silently freezes wrong floor later | yes |
| verifier frozen-floor parity | truthful stop-state still compared against retired helper tuple | must add | yes, verifier fails loudly | maintainer sees false failure | yes |
| stale-evidence gate | stale evidence accidentally passes after refactor | must add | yes, explicit failure reason exists | maintainer gets false green | yes |
| basis snapshot parity | analysis artifact drifts without decision snapshot refresh | existing + keep | yes | verifier fails clearly | no |
| derived decision parity | decision artifact semantic fields drift from kernel | existing + keep | yes | verifier fails clearly | no |
| CLI format guard | non-JSON format starts passing or emitting ambiguous output | existing + keep | yes | maintainer gets confusing CLI behavior | no |

Critical-gap rule for M42:

Any failure mode that has no regression test and would create a false green or false red verifier result is release-blocking for this milestone.

## Performance Review

There is no material runtime performance work in M42.

The change is bounded to comparing a different frozen tuple during verifier execution. The real risk is contract drift, not latency, memory, or scale. The right investment is shared truth and sharper regression tests.

## Error And Rescue Registry

| Risk | Why it matters | Rescue |
|---|---|---|
| Verifier accepts the wrong stop-state | turns the guard into theater | add field-by-field negative tests for recommendation status, blockers, decision action, basis code, and required next action |
| Hidden consumer depends on old JSON naming | renaming could break downstream parsing | do not change outward JSON names in M42 |
| Fixture drift hides live command drift | green unit tests but broken real command | run the live three-command proof loop before closing the milestone |
| Implementation widens into policy change | turns a lake into an ocean | hard stop if `recommend.rs` or decision-policy semantics become necessary |

## What Already Exists

The repo already has most of what this milestone needs:

- a real decision kernel in `xtask/src/family/analysis_core/decision_contract.rs`
- a shared export seam in `xtask/src/family/analysis_core/mod.rs`
- a verifier command with validation and parity checks in `xtask/src/family/verify.rs`
- CLI dispatch tests in `xtask/src/lib.rs`

M42 reuses all of that. It does not invent a parallel path.

## NOT in scope

- changing recommendation policy
- changing `derive_corpus_program_decision_contract()` semantics
- changing helper-surface durable-hold behavior outside verifier parity
- deleting the frozen-floor check entirely
- changing outward JSON field names or failure-reason strings
- introducing a new verifier schema version
- broader cleanup of `xtask/src/lib.rs`
- new milestone selection, packet promotion, or corpus-expansion work
- `ORCH_PLAN.md` rewrites

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Step 1. Shared frozen truthful stop-state helper | `xtask/src/family/analysis_core/` | — |
| Step 2. Verifier consumer refactor | `xtask/src/family/verify.rs` | Step 1 |
| Step 3. Verifier fixture and regression refresh | `xtask/src/family/verify.rs` | Step 2 |
| Step 4. CLI dispatch audit | `xtask/src/lib.rs` | Step 2 |

### Parallel lanes

Sequential implementation, no safe parallelization opportunity.

Why:

- the wedge is intentionally tiny
- Steps 2 and 3 both center on `xtask/src/family/verify.rs`
- Step 4 is conditional and only becomes meaningful after verifier behavior is stable

### Execution order

1. Land Step 1.
2. Land Steps 2 and 3 sequentially in the same worktree.
3. Audit Step 4 last, only if command-facing tests actually require a touch in `xtask/src/lib.rs`.

### Conflict flags

No parallel lanes are recommended. Splitting Steps 2 and 3 across worktrees would create avoidable merge conflicts in `xtask/src/family/verify.rs` for no real speed gain.

## Acceptance Criteria

M42 is complete only if all of the following are true:

1. `cargo xtask family verify-decision-contract --format json` passes on the current truthful stop-state.
2. The verifier still fails on basis snapshot drift, derived decision drift, stale evidence, and frozen-floor drift.
3. The frozen floor lives in `analysis_core`, not as duplicated literals inside `verify.rs`.
4. No family-selection policy files change.
5. The outward command surface stays bounded:
   - same command name
   - same `--format json` restriction
   - same outward JSON check key and failure-reason names
6. `xtask/src/lib.rs` is either untouched or only minimally refreshed for verifier dispatch tests.

## Proof Loop

```bash
cargo test -p xtask verify
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

## Completion Summary

- Step 0: Scope Challenge — scope accepted as a narrow verifier-truth repair
- Architecture — shared decision-contract seam remains the single truth owner
- Code Quality — duplicated frozen tuples are forbidden after M42
- Tests — 5 required gaps must be closed
- Performance — 0 material runtime concerns
- What already exists — written
- NOT in scope — written
- Failure modes — critical verifier false-red and false-green paths explicitly covered
- Parallelization — sequential only, no safe worktree split
- Lake score — complete version chosen over the verifier-local literal swap
