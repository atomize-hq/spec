<!-- plan backup: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-plan-backup-20260502-112700.md -->
# M27.9A - Stop-Path Closeout And Analysis Contract Recalibration

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-02**  
Supersedes: **M27.9 - Cross-Library Arithmetic Helper Alignment**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-105221.md`**

## Summary

M27.9A is a closeout-and-recalibration milestone.

It does not reopen semantic-family scope. It does not ask for one more corpus
run. It does not try to rescue stale scoreboard math.

It does four concrete things:

1. lands the already-authored M27.9 source truth from the integration worktree
2. reproduces the blocked stop-path evidence on `feat/corpus-expansion`
3. refreshes the locked `xtask` analysis contract to the truthful post-fix
   baseline
4. rewrites the planning and program ledger so the repo records M27.9 as
   implementation success plus accounting failure

The important repo truth is already known:

- cross-library `pricing/apply_discount` now routes to
  `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- cross-library `pricing/apply_tax` now routes to
  `function.arithmetic_leaf.monotone_up.v1`
- the obsolete M20 arithmetic-shape fixture was removed and replaced with a
  truthful control-flow near miss
- the arithmetic-ready recommendation candidate disappeared
- the truthful post-fix analysis is `28 / 17 / 0 / 11` with
  `recommendation_status = "no_strong_candidate"`

What failed was the old milestone accounting assumption, not the semantic work.
The obsolete `28 / 18 / 0 / 10` target survives only as the wrong historical
expectation that M27.9A closes out.

## Done Means

M27.9A is complete only when all of the following are true:

1. the authored semantic, fixture, CLI, and maintainer-doc changes from the
   integration worktree are preserved on `feat/corpus-expansion`
2. the merged branch reproduces `function_coverage = 28 / 17 / 0 / 11`
3. the merged branch reproduces
   `recommendation_status = "no_strong_candidate"`
4. `xtask/src/lib.rs` locks the truthful post-fix baseline instead of the stale
   arithmetic-ready baseline
5. `PLAN.md` and `docs/recommendation_corpus_expansion_program_v0.1.md`
   explicitly record M27.9 as semantic success plus accounting failure
6. future next-step selection stops treating the retired arithmetic cluster as
   live roadmap pressure
7. `money/round` remains the next visible held candidate after the recalibration

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

### Observed reproduced stop-state on `feat/corpus-expansion`

Commit anchor: `8577dfb5a64daaa02be54501f97131fb25459f72`

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- both cross-library arithmetic units now route into promoted arithmetic
  families
- `pricing/apply_tax_arithmetic_shape` is gone from the M20 pack
- `pricing/apply_tax_control_flow` replaces it as truthful unsupported demand
- the arithmetic-ready cluster disappears from ranked candidates
- `unsupported_function_surface-e40675da6fa0` remains the only visible held
  candidate, represented by `money/round`

### Why the counts changed this way

The correct interpretation is:

- `+2 promoted-family units`
  because the two cross-library arithmetic examples reclassified into promoted
  families
- `-2 unsupported-function units`
  because those same cross-library units left unsupported demand
- `0 new promoted hits from M20`
  because the obsolete M20 arithmetic fixture was replaced with a truthful
  unsupported near miss rather than converted into a promoted-family hit

The old `+3 / -3` target was therefore wrong even though the semantic fix
worked. The old `28 / 18 / 0 / 10` gate is historical context only, not a live
target state.

## Authority And Evidence

Primary decision inputs:

- `PLAN.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-105221.md`
- reproduced parent stop-state on `feat/corpus-expansion` at
  `8577dfb5a64daaa02be54501f97131fb25459f72`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `semantic-families/README.md`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `xtask/src/lib.rs`

Implementation note:

- `ORCH_PLAN.md` is stale for this pass because it still encodes the invalid
  `28 / 18 / 0 / 10` gate.
- Do not execute from `ORCH_PLAN.md` until it is regenerated from this M27.9A
  contract.

## Problem Statement

M27.9 surfaced a repo-governance bug.

The semantic reviewer, CLI truth surfaces, and recommendation analysis all
moved in the intended direction. The milestone still stopped because the old
plan assumed that removing one fake unsupported corpus unit would also create
one more promoted-family unit.

That assumption is false.

M27.9A fixes the planning and locked-analysis contract without reopening
semantic-family scope, recommendation-policy scope, or corpus-expansion scope.

## Scope Challenge

### What already exists

| Sub-problem | Existing code / truth | Decision |
|---|---|---|
| Truthful semantic fix | The integration worktree at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int` already contains the authored semantic-review, fixture, CLI, and README updates | Reuse. Do not reimplement the fix from scratch. |
| Blocked evidence bundle | `.runs/m27_9/session-log.md` plus `.runs/m27_9/diagnostics/*.json` already record the exact stop-path facts | Reuse. Do not reconstruct the story from memory. |
| Locked recommendation harness | `xtask/src/lib.rs` already owns the single locked-corpus assertion surface | Update the existing lock. Do not build a second analysis harness. |
| Program tracker | `docs/recommendation_corpus_expansion_program_v0.1.md` already distinguishes per-run planning from multi-run program tracking | Reuse and recalibrate. Do not invent a new governance document. |
| Next live held pressure | `unsupported_function_surface-e40675da6fa0` already survives as the only held candidate | Preserve. Do not solve `money/round` in this milestone. |

### Minimum honest change

The minimum complete M27.9A change set is:

1. import the authored source truth from the integration worktree
2. reproduce the blocked stop-state on the main working branch
3. refresh `xtask/src/lib.rs` to expect `28 / 17 / 0 / 11` and
   `no_strong_candidate`
4. rewrite planning and program-language surfaces so M27.9 is recorded as
   implementation success plus accounting failure

Anything smaller leaves the repo split across contradictory stories.

### Complexity and blast-radius rule

This milestone stays inside one narrow blast radius:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/`
- `spec-cli/tests/cli.rs`
- `semantic-families/README.md`
- `xtask/src/lib.rs`
- `PLAN.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

If implementation expands beyond that authored set plus the required proof
artifacts, stop and split the work.

### Search and boring-tech rule

- **[Layer 1]** Reuse the frozen blocked diagnostics instead of re-deriving the
  output story manually.
- **[Layer 1]** Reuse the authored worktree diff instead of writing a second
  semantic fix locally.
- **[Layer 1]** Reuse the existing `xtask` lock surface. Update expectations,
  not framework shape.
- **[EUREKA]** The bug is not semantic routing and not insufficient evidence.
  The bug is the assumption that truthful corpus repair should count like a new
  promoted-family addition.

### Locked decisions

1. M27.9 semantic work stands. This milestone does not reopen the classifier
   decision.
2. The M20 pack remains an unsupported truth pack.
3. `xtask` policy does not get loosened to force the old arithmetic-ready
   outcome.
4. `28 / 17 / 0 / 11` becomes the locked expected truth if the merged branch
   reproduces the current blocked diagnostics.
5. `recommendation_status = "no_strong_candidate"` is the correct post-fix
   result.
6. The arithmetic-ready cluster is retired as roadmap pressure.
7. `money/round` remains the next live held candidate after M27.9A.
8. `ORCH_PLAN.md` is non-authoritative until regenerated from this plan.

### NOT in scope

- new family packet authoring
  reason: M27.9A closes accounting around a landed semantic fix
- another corpus-expansion run
  reason: corpus growth is not the active blocker
- `money/round` overlap-family resolution
  reason: it remains the next held pressure surface after this closeout
- recommendation-policy changes
  reason: this milestone updates locked outputs, not ranking heuristics
- artifact schema changes
  reason: no evidence says artifact shape is the blocker
- shared-core or M28 work
  reason: still downstream of truthful next-family governance
- second-language or non-function expansion
  reason: unrelated to the blocked stop-path
- general documentation cleanup
  reason: only the surfaces needed to fix roadmap truth should move

## Exact File Contract

### Authoritative source-of-truth nuance

The authoritative authored M27.9 source truth is the current content of the
integration worktree at:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int`

That matters because the critical CLI, README, and M20 fixture edits currently
exist as worktree changes, not as a clean branch delta. Do not assume the
committed tip of `ws/m27_9-int` alone is sufficient. The worktree content must
be inspected and landed intentionally.

### Authored source surfaces that must land

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
| `spec-core/src/semantic_review.rs` | Preserve the optional-helper cross-library arithmetic fix exactly as proven in the integration worktree | Do not widen wrapper, chain3, or unrelated family routing. |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/` | Keep the M20 pack truthfully unsupported by replacing the obsolete arithmetic-shape unit with a control-flow near miss | Do not convert the M20 pack into a mixed or supported pack. |
| `spec-cli/tests/cli.rs` | Lock the public `status`, `export`, and passport truth for the new cross-library and M20 behavior | Do not weaken unsupported-reason coverage. |
| `semantic-families/README.md` | State plainly that promoted arithmetic leaf families already cover zero-or-one helper deps, including cross-library helper-aware examples | Do not turn README into milestone meta-commentary. |
| `xtask/src/lib.rs` | Replace the stale locked-corpus arithmetic-ready expectation with the truthful `no_strong_candidate` and `28 / 17 / 0 / 11` contract | Do not rewrite ranking policy or artifact schema here. |
| `PLAN.md` | Close M27.9 on the stop path and define the M27.9A success contract | Do not preserve the invalid `18 / 10` success gate anywhere in the file. |
| `docs/recommendation_corpus_expansion_program_v0.1.md` | Update the baseline, run-log language, and immediate-next-step guidance so arithmetic pressure is retired and the program ledger matches the new truth | Do not reopen corpus-run planning by habit. |

## Architecture Review

### Core rule

Land semantic truth first, then re-lock analysis truth around what the semantic
fix actually produced.

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
integration worktree
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
OLD LOCKED STATE
================
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

### Architecture-specific failure scenario

If `xtask` is refreshed before the merged branch reproduces the stop-state, the
repo will lock a governance story that may not actually match branch truth.
That is the primary architectural landmine here.

## Code Quality And Complexity Guardrails

- Reuse the authored integration worktree diff. Do not restate the same logic in
  a fresh local patch.
- Keep all recalibration logic on the existing lock surfaces. No new helpers, no
  new harness layer, no secondary analysis fixtures.
- Preserve the current semantic-family boundary. This milestone is not allowed to
  widen classifier scope to make the counts look prettier.
- Keep docs explicit. The plan and program tracker must say plainly that corpus
  repair and promoted-family reclassification are different accounting events.

## Implementation Plan

### Step 0 - Capture and import the actual authored source truth

Before editing the parent branch, inspect the integration worktree diff and
import the authored source changes from the worktree content, not just the
branch name:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int`

Required import set:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `semantic-families/README.md`
- delete `apply_tax_arithmetic_shape.unit.spec`
- add `apply_tax_control_flow.unit.spec`

Do not import derived passport files by hand. They are proof surfaces and must
be regenerated during the proof loop.

### Step 1 - Land the authored M27.9 semantic and CLI truth

Bring the already-proven authored source changes onto
`feat/corpus-expansion` without reworking their intent:

- semantic-review optional-helper parity
- truthful M20 control-flow replacement
- CLI truth-lock expansion
- maintainer doc update

Done means the branch source diff matches the intended authored state, not just
the earlier partial parent-workspace semantic edit.

### Step 2 - Reproduce the blocked stop-state on the merged branch

Before changing `xtask` locks, rerun the proof surfaces and confirm the branch
still lands on:

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- no ranked arithmetic-ready candidate remains
- `unsupported_function_surface-e40675da6fa0` remains the live held candidate

Done means the observed stop-path is branch truth, not just worktree-local
truth.

### Step 3 - Refresh `xtask` locked corpus expectations

Update the locked-corpus assertions in `xtask/src/lib.rs` to reflect the real
baseline:

- replace the stale arithmetic-ready expectation block
- assert `RecommendationStatus::NoStrongCandidate`
- assert `promoted_family_units = 17`
- assert `unsupported_function_units = 11`
- assert ranked candidates no longer include
  `unsupported_arithmetic_shape-2694b2baf65b`
- assert the remaining visible held candidate is
  `unsupported_function_surface-e40675da6fa0`
  with `unknown_overlap_family`

Done means the analysis harness encodes the truthful post-fix corpus.

### Step 4 - Rewrite the planning and program ledger

Update the planning surfaces so future sessions read the correct story:

- M27.9 semantic work succeeded
- the old count target was wrong
- corpus repair and promoted-family reclassification are different accounting
  categories
- arithmetic-ready pressure is retired
- `money/round` remains the next live held candidate after this closeout

Done means the repo stops steering future roadmap choices from stale arithmetic
pressure.

### Step 5 - Run final proof and close M27.9 cleanly

Record the closeout explicitly:

- M27.9 closed on the stop path
- M27.9A owns the lock refresh and accounting recalibration
- next milestone selection starts from the truthful
  `28 / 17 / 0 / 11` + `no_strong_candidate` baseline

Done means nobody has to reverse-engineer why the milestone stopped or what the
next honest move is.

## Test Review

### Framework and suites

Runtime: Rust workspace with `cargo test`

Suites that must move together:

- `spec-core` unit tests
- `spec-cli` integration tests
- `xtask` locked-analysis tests

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/semantic_review.rs
    │
    ├── [MUST LAND] cross-library monotone-down canonical route
    │   -> promoted monotone-down family
    │
    ├── [MUST LAND] cross-library monotone-up canonical route
    │   -> promoted monotone-up family
    │
    ├── [MUST LAND] helper-then-clamp monotone-up normalization
    │   -> still supported
    │
    └── [MUST LAND] cross-library control-flow near miss
        -> stays unsupported

[+] spec-cli/tests/cli.rs
    │
    ├── [MUST LAND] passport / status / export truth for cross-library tax
    ├── [MUST LAND] whole-pack M20 unsupported reason matrix
    └── [MUST LAND] repo-root crosslib workspace status now shows 2 units

[+] xtask/src/lib.rs
    │
    ├── [ADD TEST LOCK] coverage at 28 / 17 / 0 / 11
    ├── [ADD TEST LOCK] recommendation_status = no_strong_candidate
    ├── [ADD TEST LOCK] arithmetic-ready cluster absent from ranking
    └── [ADD TEST LOCK] money/round surface remains held

[+] governance docs
    │
    ├── [MUST LAND] PLAN.md records stop-path success plus accounting failure
    └── [MUST LAND] program ledger retires arithmetic pressure explicitly
```

### Required tests and proof assertions

Tests already authored in the integration worktree and required to survive
intact:

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
   - replace the stale arithmetic-ready lock expectations with the truthful
     post-fix lock

### Regression rule

This is partly a regression-preservation milestone.

Any path that previously claimed the arithmetic cluster was `ready` but now
truthfully resolves to `no_strong_candidate` must be locked with an explicit
regression test. No shortcut.

### Test plan artifact

During implementation verification, write the QA-facing artifact to:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-{timestamp}.md`

That artifact should tell QA to verify:

- both cross-library arithmetic units now classify as supported promoted leaves
- the M20 pack stays unsupported for `unsupported_control_flow`
- the locked corpus no longer ranks an arithmetic-ready next-family candidate
- the remaining visible candidate is still `money/round` held for
  `unknown_overlap_family`

## Failure Modes

| Codepath | Realistic failure | Test required? | Error handling exists? | Silent if missed? | Critical gap? |
|---|---|---|---|---|---|
| worktree truth is only partially imported | parent branch keeps the semantic file but loses CLI, README, or M20 fixture truth | yes | no | yes | **yes** |
| merged branch cannot reproduce stop-state counts | branch drift or partial source landing changes `28 / 17 / 0 / 11` | yes | no, only proof-loop detection | yes | **yes** |
| `xtask` lock stays on the arithmetic-ready baseline | roadmap steering still treats retired arithmetic pressure as live | yes | test-only | yes | **yes** |
| plan/program docs remain stale | future sessions reopen arithmetic work or misclassify M27.9 as execution failure | yes | no | yes | **yes** |
| M20 pack drifts back toward supported arithmetic shape | the unsupported regression pack stops being honest | yes | test-only | yes | **yes** |

Critical here means the repo would silently make the wrong next-milestone
decision.

## Performance And Operational Review

There is no meaningful runtime-performance risk in the shipped product surface.
The risk here is proof-loop cost and governance drift, not user-facing latency.

Operational rules:

- do not rerun `xtask` lock edits speculatively before the branch reproduces the
  stop-state
- refresh derived analysis artifacts only after source truth is integrated
- regenerate passports and analysis artifacts once on the integrated branch,
  not independently in multiple lanes

## Proof Loop

Run in this exact order:

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
governance language further.

## Worktree Parallelization Strategy

This milestone has limited but real parallelization.

The rule is simple: source truth first, docs in parallel once the evidence
bundle is accepted, `xtask` last.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Import and land authored source truth | `spec-core/src/`, `spec-cli/tests/`, `semantic-families/`, `spec-cli/tests/fixtures/` | — |
| Reproduce blocked stop-state | `spec-core/`, `spec-cli/`, `examples/`, `.semantic-family-artifacts/` | Import and land authored source truth |
| Rewrite plan and program accounting language | repo root docs, `docs/` | blocked evidence bundle accepted as source truth |
| Refresh `xtask` locked corpus expectations | `xtask/src/` | Reproduce blocked stop-state |
| Final proof and closeout | `spec-core/`, `spec-cli/`, `xtask/`, docs, proof artifacts | prior three steps complete |

### Parallel lanes

- Lane A: source landing -> reproduced stop-state
  `spec-core/src/`, `spec-cli/tests/`, `spec-cli/tests/fixtures/`,
  `semantic-families/`, proof surfaces
- Lane B: plan/program closeout rewrite
  `PLAN.md`, `docs/recommendation_corpus_expansion_program_v0.1.md`
- Lane C: `xtask` lock refresh
  `xtask/src/`

### Execution order

1. Launch Lane A first. It owns branch truth.
2. Lane B may run in parallel once the blocked evidence bundle is accepted as
   authoritative input, but its final wording must match the reproduced stop-state.
3. Launch Lane C only after Lane A proves the merged branch still lands on
   `28 / 17 / 0 / 11` and `no_strong_candidate`.
4. Merge A + B, then finish C, then run the proof loop once on the integrated
   branch.

### Conflict flags

- Lane A and Lane C must stay separate. `xtask/src/` should not move until the
  merged source truth is proven.
- Lane B must not quietly rewrite counts or candidate interpretation without
  reading `.runs/m27_9/diagnostics/blocked-summary.md` and the actual artifacts.
- Do not use the current `ORCH_PLAN.md` as a worker prompt source. It still
  encodes the invalid `18 / 10` gate.

### Practical recommendation

Use two real concurrent workstreams:

- Workstream 1: authored source landing plus proof reproduction
- Workstream 2: plan/program closeout rewrite

Then do the `xtask` lock refresh as a final sequential lane after both are
aligned on the reproduced stop-state.

## Acceptance Criteria

M27.9A is complete only when all of the following are true:

1. the integration worktree authored source truth is merged onto
   `feat/corpus-expansion`
2. the branch reproduces `28 / 17 / 0 / 11`
3. the branch reproduces `recommendation_status = "no_strong_candidate"`
4. the arithmetic-ready cluster is absent from ranked candidates
5. `unsupported_function_surface-e40675da6fa0` remains held for
   `unknown_overlap_family`
6. the M20 pack remains truthfully unsupported through
   `pricing/apply_tax_control_flow`
7. `xtask/src/lib.rs` locks the new truthful baseline
8. `PLAN.md` explicitly says M27.9 failed on accounting, not implementation
9. `docs/recommendation_corpus_expansion_program_v0.1.md` no longer points the
   next move at arithmetic promotion pressure

## Next Step After Closeout

After M27.9A lands, the next live decision surface is:

- `unsupported_function_surface-e40675da6fa0`
- held for `unknown_overlap_family`
- concretely represented today by `money/round`

That next milestone is downstream of this recalibration. Do not start it inside
M27.9A.
