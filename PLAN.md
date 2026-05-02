<!-- plan backup: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-plan-backup-20260502-112700.md -->
# M27.9A - Stop-Path Closeout And Analysis Contract Recalibration

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-02**  
Supersedes: **M27.9 - Cross-Library Arithmetic Helper Alignment**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-105221.md`**

## Summary

M27.9A is not another semantic-routing milestone.

It is a closeout-and-recalibration milestone that does three things together:

1. lands the truthful authored M27.9 source changes already isolated in
   `ws/m27_9-int`
2. closes M27.9 as a stop-path success on implementation truth and a failure on
   milestone accounting
3. rewrites the locked analysis contract around the actual observed result:
   `28 / 17 / 0 / 11` with `recommendation_status = "no_strong_candidate"`

The repo already proved the important product truth:

- `examples_crosslib_app::pricing/apply_discount` now belongs in
  `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `examples_crosslib_app::pricing/apply_tax` now belongs in
  `function.arithmetic_leaf.monotone_up.v1`
- the obsolete M20 arithmetic-shape fixture was retired and replaced by a
  truthful control-flow near miss
- the fake arithmetic promotion pressure disappeared

What failed was the old milestone math, not the semantic work.

## Milestone Outcome

M27.9A is done only when all of the following are true:

1. the authored semantic, CLI, fixture, and maintainer-doc changes from
   `ws/m27_9-int` are preserved on `feat/corpus-expansion`
2. the repo can reproduce the blocked stop-path evidence on the merged branch:
   `28 / 17 / 0 / 11` and `no_strong_candidate`
3. `xtask/src/lib.rs` locks the truthful new baseline instead of the stale
   arithmetic-ready baseline
4. `PLAN.md` and the program ledger describe M27.9 as semantic success plus
   accounting failure, not as a failed implementation milestone
5. future milestone selection stops chasing the retired arithmetic candidate and
   instead treats `money/round` as the next live held surface after the
   accounting recalibration is complete

## Current Repo Truth

### Pre-M27.9 locked baseline

- `function_coverage = 28 / 15 / 0 / 13`
- `recommendation_status = "ranked"`
- first ranked candidate:
  `unsupported_arithmetic_shape-2694b2baf65b`
  with `promotion_readiness = "ready"`
- second ranked candidate:
  `unsupported_function_surface-e40675da6fa0`
  with `promotion_readiness = "hold"` for `unknown_overlap_family`

### Blocked stop-path observation from `.runs/m27_9`

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- both cross-library arithmetic units now route into promoted arithmetic
  families
- `pricing/apply_tax_arithmetic_shape` is gone from the M20 pack
- `pricing/apply_tax_control_flow` replaces it as truthful unsupported demand
- the arithmetic ready candidate disappears from ranking
- `unsupported_function_surface-e40675da6fa0` remains the only visible held
  candidate

### Why the counts changed this way

The correct interpretation is:

- `+2 promoted-family units`
  because the two cross-library arithmetic examples reclassified into promoted
  families
- `-2 unsupported-function units`
  because those same two cross-library units left unsupported demand
- `0 new promoted hits from M20`
  because the obsolete M20 arithmetic fixture was not converted into supported
  truth, it was replaced with a truthful unsupported control-flow near miss

So the old `+3 / -3` target was wrong even though the semantic fix worked.

## Plan Authority

Primary decision inputs:

- `PLAN.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-105221.md`
- `.runs/m27_9/session-log.md`
- `.runs/m27_9/diagnostics/blocked-summary.md`
- `.runs/m27_9/diagnostics/coverage.actual.json`
- `.runs/m27_9/diagnostics/recommendation.actual.json`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `semantic-families/README.md`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `xtask/src/lib.rs`

Implementation note:

- The current `ORCH_PLAN.md` still encodes the invalid `28 / 18 / 0 / 10` gate.
  It is stale the moment this plan lands.
- Do not execute from `ORCH_PLAN.md` until it is regenerated from the M27.9A
  contract.

## Problem Statement

M27.9 surfaced a repo-governance bug.

The semantic reviewer, CLI truth surfaces, and recommendation output all moved
in the intended direction. The milestone still stopped because the old plan
assumed that removing one fake unsupported corpus unit would also create one
more promoted-family unit.

That assumption is false.

M27.9A therefore fixes the planning and accounting contract without reopening
semantic-family scope, recommendation-policy scope, or corpus-expansion scope.

## Scope Challenge

### What already exists

| Sub-problem | Existing code / truth | Decision |
|---|---|---|
| Truthful semantic fix | `ws/m27_9-int` already contains the authored semantic-review, fixture, CLI, and README updates | Reuse. Do not reimplement the fix from scratch. |
| Blocked evidence bundle | `.runs/m27_9/session-log.md` plus `.runs/m27_9/diagnostics/*.json` already record the exact stop-path facts | Reuse. Do not reconstruct the story from memory. |
| Locked recommendation harness | `xtask/src/lib.rs` already has a single locked-corpus assertion point for coverage and recommendation truth | Update the existing lock. Do not build a second analysis harness. |
| Program stop-rule language | `docs/recommendation_corpus_expansion_program_v0.1.md` already distinguishes corpus work from promotion/policy work | Reuse and recalibrate. Do not invent a new governance document. |
| Next live held pressure | `unsupported_function_surface-e40675da6fa0` already survives as the only held candidate | Preserve. Do not solve `money/round` in this milestone. |

### Minimum honest change

The minimum complete M27.9A change set is:

1. land the authored source truth from `ws/m27_9-int`
2. reproduce the blocked stop-path evidence on the merged branch
3. refresh `xtask/src/lib.rs` so the locked corpus expects
   `28 / 17 / 0 / 11` and `no_strong_candidate`
4. rewrite planning and program-language surfaces so M27.9 is recorded as
   implementation success plus accounting failure

Anything smaller leaves the repo split across two contradictory stories.

### Complexity rule

This milestone should stay inside one narrow blast radius:

- the four authored source surfaces already proven in `ws/m27_9-int`
- one `xtask` lock surface
- one primary plan surface
- one program ledger surface

If implementation expands past that set, stop and split the work. M27.9A is a
truth-and-accounting closeout, not a fresh architecture project.

### Search and boring-tech rule

- **[Layer 1]** Reuse the frozen blocked diagnostics instead of re-deriving the
  output story manually.
- **[Layer 1]** Reuse the authored source diff in `ws/m27_9-int` instead of
  writing a second semantic fix.
- **[Layer 1]** Reuse the existing `xtask` artifact and lock contract. Update
  expectations, not framework shape.
- **[EUREKA]** The bug is not in semantic routing and not in evidence volume.
  The bug is the assumption that corpus repair should count like new promoted
  demand.

### Completeness rule

Do the complete version now:

- semantic truth lands
- CLI truth lands
- truthful M20 repair lands
- maintainer docs land
- `xtask` locks land
- plan and program accounting language land

Do not land only the semantic fix or only the doc rewrite. That would keep the
repo in a half-truth state.

### Distribution check

No new user-facing distribution artifact exists here.

The relevant distribution surface is repo governance truth:

- checked-in source and tests
- checked-in analysis lock expectations
- checked-in planning and program documents
- generated analysis artifacts validated from current repo truth

## Locked Decisions

1. M27.9 semantic work stands. This milestone does not reopen the classifier
   decision.
2. The M20 pack remains an unsupported truth pack.
3. `xtask` policy does not get loosened to force the old arithmetic-ready
   outcome.
4. `28 / 17 / 0 / 11` becomes the expected locked truth if the merged branch
   reproduces the current blocked diagnostics.
5. `recommendation_status = "no_strong_candidate"` is the correct outcome for
   the post-fix corpus.
6. The arithmetic ready candidate is retired as roadmap pressure.
7. `money/round` remains the next live held candidate after M27.9A.
8. `ORCH_PLAN.md` is non-authoritative until rewritten from this contract.

## Alternatives Rejected

### Reopen M27.9 until promoted count reaches 18

Rejected.

That optimizes for stale scoreboard math instead of repo truth.

### Edit `xtask` locks to force `28 / 18 / 0 / 10`

Rejected.

That would make the analysis layer lie about the corpus.

### Treat M27.9 as a failed implementation milestone and discard `ws/m27_9-int`

Rejected.

The authored semantic and CLI work is the successful part.

### Jump straight to `money/round` without recalibrating accounting

Rejected.

That would leave the repo steering future roadmap choices from an already
retired arithmetic-pressure story.

## NOT in Scope

- new family packet authoring
  reason: M27.9A closes accounting around a landed semantic fix
- another corpus-expansion run
  reason: the corpus is not the current blocker
- `money/round` overlap-family resolution
  reason: it remains the next held pressure surface after this closeout
- recommendation-policy changes
  reason: this milestone updates locked outputs, not ranking heuristics
- artifact schema changes
  reason: no evidence says the artifact shape is the blocker
- shared-core / M28 work
  reason: still downstream of truthful next-family governance
- second-language or non-function expansion
  reason: unrelated to the blocked stop-path
- general documentation cleanup
  reason: only the surfaces needed to fix roadmap truth should move

## Exact File Contract

### Authored source surfaces that must land from `ws/m27_9-int`

1. `spec-core/src/semantic_review.rs`
2. `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
   - delete
3. `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_control_flow.unit.spec`
   - add
4. `spec-cli/tests/cli.rs`
5. `semantic-families/README.md`

### Recalibration surfaces owned by M27.9A

6. `xtask/src/lib.rs`
7. `PLAN.md`
8. `docs/recommendation_corpus_expansion_program_v0.1.md`

### Derived surfaces expected to refresh during proof

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `examples/shared-crate/src/generated/**`
- `examples/crosslib-app/units/pricing/*.spec.passport.json`

### File-by-file responsibility

| File | Responsibility | Must not happen |
|---|---|---|
| `spec-core/src/semantic_review.rs` | Preserve the optional-helper cross-library arithmetic fix exactly as proven in `ws/m27_9-int` | Do not widen wrapper, chain3, or unrelated family routing. |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/` | Keep the M20 pack truthfully unsupported by replacing the obsolete arithmetic-shape unit with a control-flow near miss | Do not convert the M20 pack into a mixed or supported pack. |
| `spec-cli/tests/cli.rs` | Lock the public `status`, `export`, and passport truth for the new cross-library and M20 behavior | Do not weaken unsupported-reason coverage. |
| `semantic-families/README.md` | State plainly that promoted arithmetic leaf families already cover zero-or-one helper deps, including cross-library helper-aware examples | Do not turn README into milestone meta-commentary. |
| `xtask/src/lib.rs` | Replace the stale locked-corpus arithmetic-ready expectation with the truthful `no_strong_candidate` and `28 / 17 / 0 / 11` contract | Do not rewrite ranking policy or artifact schema here. |
| `PLAN.md` | Close M27.9 on the stop path and define the M27.9A success contract | Do not preserve the invalid `18 / 10` success gate anywhere in the file. |
| `docs/recommendation_corpus_expansion_program_v0.1.md` | Update the run log and immediate-next-step guidance so arithmetic pressure is retired and the program ledger matches the new truth | Do not reopen corpus-run planning by habit. |

## Architecture Contract

### Core rule

M27.9A lands semantic truth first, then re-locks analysis truth around what that
semantic fix actually produced.

Never reverse that order.

### Two-layer truth model

```text
LAYER 1: PRODUCT / SEMANTIC TRUTH
=================================
authored units
    -> semantic review routing
    -> CLI read-side surfaces

LAYER 2: GOVERNANCE / ROADMAP TRUTH
===================================
CLI read-side surfaces
    -> coverage analysis artifact
    -> recommendation analysis artifact
    -> locked xtask corpus expectations
    -> plan and program next-step decisions
```

### Data flow

```text
AUTHORED SOURCE CHANGES
=======================
ws/m27_9-int
  - spec-core semantic review
  - M20 fixture repair
  - spec-cli truth locks
  - semantic-families README
        │
        ▼
SEMANTIC + CLI TRUTH
====================
spec-core::semantic_review
spec status / export / passports
        │
        ▼
ANALYSIS PROJECTION
===================
cargo xtask family coverage --format json
cargo xtask family recommend --format json
        │
        ▼
GOVERNANCE CONTRACT
===================
xtask/src/lib.rs locked corpus assertions
PLAN.md milestone success language
recommendation_corpus_expansion_program_v0.1.md next-step guidance
```

### Dependency graph

```text
spec-core/src/semantic_review.rs
    │
    ├── defines runtime semantic truth
    ├── consumed by spec-cli truth surfaces
    └── consumed by xtask analysis

spec-cli/tests/cli.rs
    │
    └── locks what maintainers and agents actually see

xtask/src/lib.rs
    │
    └── locks the roadmap-steering interpretation of the current corpus

PLAN.md + recommendation_corpus_expansion_program_v0.1.md
    │
    └── define what the repo should do next with that analysis
```

### State transition

```text
OLD STATE
=========
15 promoted / 13 unsupported / ranked arithmetic-ready candidate
        │
        │ semantic truth fix + truthful M20 repair
        ▼
OBSERVED STOP STATE
===================
17 promoted / 11 unsupported / no_strong_candidate
        │
        │ M27.9A accounting recalibration
        ▼
NEW LOCKED STATE
================
same observed counts, same recommendation outcome,
new success interpretation, retired arithmetic pressure
```

## Implementation Plan

### Step 1 - Land the authored M27.9 truth from `ws/m27_9-int`

Bring the already-proven authored source changes onto `feat/corpus-expansion`
without reworking their intent:

- semantic-review optional-helper parity
- truthful M20 control-flow replacement
- CLI truth-lock expansion
- maintainer doc update

Done means the branch source diff matches the intentional authored state, not
just the partial parent-workspace semantic edit.

### Step 2 - Reproduce the blocked stop-path evidence on the merged branch

Before changing `xtask` locks, rerun the proof surfaces and confirm the branch
still lands on the same stop-state:

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- no ranked arithmetic-ready candidate remains
- `unsupported_function_surface-e40675da6fa0` remains the live held candidate

Done means the observed stop-path is branch truth, not just worktree-local
truth.

### Step 3 - Refresh `xtask` locked corpus expectations

Update the locked-corpus assertions in `xtask/src/lib.rs` to reflect the real
baseline:

- rename or rewrite the stale arithmetic-ready command-path test
- assert `RecommendationStatus::NoStrongCandidate`
- assert `promoted_family_units = 17`
- assert `unsupported_function_units = 11`
- assert the surviving ranked candidate set no longer contains
  `unsupported_arithmetic_shape-2694b2baf65b`
- assert `unsupported_function_surface-e40675da6fa0` remains held for
  `unknown_overlap_family`

Done means the analysis harness now encodes the truthful post-fix corpus, not
the old milestone fantasy.

### Step 4 - Rewrite plan and program accounting language

Update planning surfaces so future sessions and maintainers read the correct
story:

- M27.9 semantic work succeeded
- the old count target was wrong
- corpus repair and promoted-family reclassification are different accounting
  categories
- arithmetic ready pressure is retired
- `money/round` remains the next live held candidate after this closeout

Done means the repo stops steering roadmap choices from stale arithmetic
pressure.

### Step 5 - Close M27.9 and hand off cleanly

Record the closeout explicitly:

- M27.9 closed on the stop path
- M27.9A owns the lock refresh and accounting recalibration
- next milestone selection starts from the truthful `no_strong_candidate`
  baseline, not from the old arithmetic-ready narrative

Done means nobody has to reverse-engineer why the milestone stopped or what the
next honest move is.

## Expected Output Delta

### Coverage

The truthful post-fix lock for M27.9A is:

- `function_coverage.total_units = 28`
- `function_coverage.promoted_family_units = 17`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 11`

### Recommendation

- `recommendation_status = "no_strong_candidate"`
- `unsupported_arithmetic_shape-2694b2baf65b` is absent from ranked candidates
- `unsupported_function_surface-e40675da6fa0` remains visible and held for
  `unknown_overlap_family`

### Cluster interpretation

The accounting interpretation must explicitly separate:

- promoted-count gains from reclassification
- unsupported-count drops from those reclassifications
- unsupported-count preservation from truthful fixture repair

The repo must not blur those three things into one raw count target again.

### Stop gate

If the merged branch does not reproduce the observed `28 / 17 / 0 / 11` and
`no_strong_candidate` state, stop.

Capture:

- the per-unit semantic-review outputs for both cross-library arithmetic units
- the M20 control-flow fixture truth
- the full recommendation artifact

Then split the work again. Do not patch `xtask` locks or planning docs around a
non-reproduced state.

## Test Review

### Framework and suites

Runtime: Rust workspace with `cargo test`

Suites that must move together:

- `spec-core` unit tests
- `spec-cli` integration tests
- `xtask` locked-analysis tests

### Coverage diagram

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/semantic_review.rs
    │
    ├── [WORKTREE READY] cross-library monotone-down canonical route
    │   -> promoted monotone-down family
    │
    ├── [WORKTREE READY] cross-library monotone-up canonical route
    │   -> promoted monotone-up family
    │
    ├── [WORKTREE READY] helper-then-clamp monotone-up normalization
    │   -> still supported
    │
    └── [WORKTREE READY] cross-library control-flow near miss
        -> stays unsupported

[+] spec-cli/tests/cli.rs
    │
    ├── [WORKTREE READY] passport / status / export truth for cross-library tax
    ├── [WORKTREE READY] whole-pack M20 unsupported reason matrix
    └── [WORKTREE READY] repo-root crosslib workspace status now shows 2 units

[+] xtask/src/lib.rs
    │
    ├── [GAP] lock coverage at 28 / 17 / 0 / 11
    ├── [GAP] lock recommendation_status = no_strong_candidate
    ├── [GAP] remove arithmetic-ready candidate expectation
    └── [GAP] keep money/round held for unknown_overlap_family

[+] governance docs
    │
    ├── [GAP] PLAN.md records stop-path success plus accounting failure
    └── [GAP] program ledger retires arithmetic pressure explicitly
```

### Required tests

Tests already authored in `ws/m27_9-int` and required to survive intact:

1. `spec-core/src/semantic_review.rs`
   - `monotone_down_nonnegative_classifier_cross_library_canonical_example_routes_to_promoted_leaf_without_invariants`
   - `monotone_up_classifier_helper_then_clamp_routes_to_promoted_leaf`
   - `monotone_up_classifier_cross_library_canonical_example_routes_to_promoted_leaf_without_invariants`
   - `monotone_up_classifier_cross_library_control_flow_near_miss_stays_unsupported`
   - `family_a_helper_dep_normalization_allows_helper_then_clamp_for_monotone_up`

2. `spec-cli/tests/cli.rs`
   - `cross_library_monotone_up_truth_surfaces_preserve_supported_semantic_review`
   - the M20 whole-pack unsupported reason-code matrix
   - the repo-root crosslib status/export assertions

3. `xtask/src/lib.rs`
   - replace the stale
     `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held`
     expectation with the truthful post-fix lock

### Regression rule

This is partly a regression-preservation milestone.

Any path that previously claimed the arithmetic candidate was `ready` but now
truthfully resolves to `no_strong_candidate` must be locked with an explicit
regression test. No shortcut.

### Test plan artifact

During implementation verification, write the QA-facing artifact to:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-{timestamp}.md`

The artifact should tell QA to verify:

- both cross-library arithmetic units now classify as supported promoted leaves
- the M20 pack stays unsupported for `unsupported_control_flow`
- the locked corpus no longer ranks an arithmetic-ready next-family candidate
- the remaining visible candidate is still `money/round` held for
  `unknown_overlap_family`

## Failure Modes

| Codepath | Realistic failure | Test required? | Error handling exists? | Silent if missed? | Critical gap? |
|---|---|---|---|---|---|
| semantic fix partially lands | parent workspace keeps only the semantic file but loses CLI or fixture truth | yes | no | yes | **yes** |
| merged branch cannot reproduce stop-state counts | branch drift or partial source landing changes `28 / 17 / 0 / 11` | yes | no, only proof-loop detection | yes | **yes** |
| `xtask` lock stays on arithmetic-ready baseline | roadmap steering still treats retired arithmetic pressure as live | yes | test-only | yes | **yes** |
| plan/program docs remain stale | future sessions reopen arithmetic work or misclassify M27.9 as execution failure | yes | no | yes | **yes** |
| M20 pack truth drifts back toward supported arithmetic shape | unsupported regression pack stops being honest | yes | test-only | yes | **yes** |

Critical here means the repo would silently make the wrong next-milestone
decision.

## Proof Loop

Run in this order:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never

cargo xtask family coverage --format json
cargo xtask family recommend --format json

cargo test -p xtask -- --color never

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Acceptance rule:

- `spec-core` tests pass
- `spec-cli` CLI truth tests pass
- `xtask` locked-corpus tests pass
- both analysis commands succeed
- both artifacts validate
- the outputs match the truthful M27.9A lock:
  `28 / 17 / 0 / 11` and `no_strong_candidate`

If any one of those fails, stop and inspect branch truth before editing
governance language.

## Worktree Parallelization Strategy

This milestone does have real parallelization, but only after the source-of-
truth boundaries are respected.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Land `ws/m27_9-int` authored source truth | `spec-core/src/`, `spec-cli/tests/`, `semantic-families/` | — |
| Reproduce blocked stop-state | `spec-core/`, `spec-cli/`, `.semantic-family-artifacts/`, `examples/` | Land `ws/m27_9-int` authored source truth |
| Refresh `xtask` locked corpus expectations | `xtask/src/` | Reproduce blocked stop-state |
| Rewrite plan and program accounting language | repo root docs, `docs/` | blocked evidence already exists; final wording should confirm reproduced stop-state |

### Parallel lanes

- Lane A: land authored source truth -> reproduce blocked stop-state
  `spec-core/src/`, `spec-cli/tests/`, `semantic-families/`, proof surfaces
- Lane B: rewrite plan and program accounting language
  `PLAN.md`, `docs/recommendation_corpus_expansion_program_v0.1.md`
- Lane C: refresh `xtask` locked corpus expectations
  `xtask/src/`

### Execution order

1. Launch Lane A first. It owns branch truth.
2. Lane B can start in parallel once the blocked evidence bundle is accepted as
   source truth, but it should finalize only after Lane A reproduces the same
   stop-state.
3. Launch Lane C only after Lane A proves the merged branch still lands on
   `28 / 17 / 0 / 11` and `no_strong_candidate`.
4. Merge A + B, then finish C, then run the proof loop once on the integrated
   branch.

### Conflict flags

- Lane A and Lane C must stay separate. `xtask/src/` should not be edited until
  the merged source truth is proven.
- Lane B must not quietly rewrite counts or candidate interpretation without
  reading the blocked evidence bundle.
- Do not use the current `ORCH_PLAN.md` as a worker prompt source. It still
  encodes the invalid `18 / 10` gate.

### Practical recommendation

Use two real workstreams:

- Workstream 1: authored source landing plus proof reproduction
- Workstream 2: plan/program closeout rewrite

Then do the `xtask` lock refresh as a final sequential lane after both are
aligned on the reproduced stop-state.

## Acceptance Criteria

M27.9A is complete only when all of the following are true:

1. the `ws/m27_9-int` authored source truth is merged onto
   `feat/corpus-expansion`
2. the branch reproduces `28 / 17 / 0 / 11`
3. the branch reproduces `recommendation_status = "no_strong_candidate"`
4. the arithmetic ready candidate is absent from ranked candidates
5. `unsupported_function_surface-e40675da6fa0` remains held for
   `unknown_overlap_family`
6. the M20 pack remains truthfully unsupported through
   `pricing/apply_tax_control_flow`
7. `xtask/src/lib.rs` locks the new truthful baseline
8. `PLAN.md` explicitly says M27.9 failed on accounting, not implementation
9. `docs/recommendation_corpus_expansion_program_v0.1.md` no longer points the
   next move at arithmetic promotion pressure

## TODOS.md Impact

No new TODO belongs in `TODOS.md` if M27.9A lands cleanly.

If implementation proves the merged branch cannot reproduce the blocked
diagnostics, that is not TODO debt. That is a new explicit milestone with a new
problem statement.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Treat M27.9 as implementation success plus accounting failure | mechanical | explicit over clever | the semantic and CLI truth already moved correctly | call it generic failure |
| 2 | CEO | Keep M27.9A narrow and do not reopen corpus expansion | mechanical | pragmatic | corpus is no longer the active blocker | one more evidence run by reflex |
| 3 | Eng | Reuse `ws/m27_9-int` as the authored source of truth | mechanical | minimal diff | reimplementation adds risk with no new value | rewrite the semantic fix locally |
| 4 | Eng | Re-lock `xtask` to `28 / 17 / 0 / 11` and `no_strong_candidate` | mechanical | systems over heroes | roadmap steering must match repo truth deterministically | preserve stale arithmetic-ready lock |
| 5 | Eng | Keep the M20 pack unsupported through control-flow truth | mechanical | choose completeness | pack naming and semantic reality must agree | keep a knowingly supported arithmetic shape in the unsupported pack |
| 6 | CEO | Keep `money/round` as the next live held pressure after M27.9A | taste | bias toward action | arithmetic pressure is retired, but the next decision surface already exists | open another vague search milestone |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | — |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |

**VERDICT:** NEW M27.9A PLAN WRITTEN. This file replaces the stale M27.9
contract and is ready for implementation or a formal review pass.
