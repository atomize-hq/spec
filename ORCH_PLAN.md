# M37 Orchestration Plan

Status: **authoritative execution contract for the M37 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Baseline HEAD: **`d2e69249495049947d414b7126d663ae1452e076`** (`d2e6924`)  
Last rewritten: **`2026-05-05`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m37_decision_kernel_extraction`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Recommendation artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Decision artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`**  
Execution note: **M37 is the family-analysis decision-kernel extraction only. It is not a portability revisit, not a helper-surface redesign, and not a public surface expansion.**

## Summary

- This run is for **M37 - Family-Analysis Decision-Kernel Extraction After M36** only.
- `PLAN.md` is milestone authority. This file is the parent-owned operator contract for executing that authority without improvisation.
- The parent agent is the sole baseline capturer, sole freeze authority, sole worktree creator, sole merge authority, sole stale-lane invalidator, sole gatekeeper, sole final verifier, sole publisher, and sole closeout author.
- Worker model is allowed only where `PLAN.md` authorizes parallel worktree execution.
- Allowed worker profile for delegated lanes is:
  - `GPT-5.4`
  - `reasoning_effort=high`
- Concurrency is fixed:
  - `0` before `authority-freeze.json`
  - `1` during the parent foundation lane
  - `3` after `foundation-freeze.json`, for the three allowed post-foundation lanes
  - `1` after code-lane convergence, for the parent integration/test lane
- The critical path is fixed:
  1. capture baseline on live `feat/corpus-expansion`
  2. freeze authority and create integration + foundation worktrees
  3. execute the parent foundation lane first
  4. merge foundation into `ws/m37-int` and write `foundation-freeze.json`
  5. launch `recommend.rs` rewiring and `promotion_artifacts.rs` rewiring in parallel
  6. launch docs/TODO worker lane after foundation freeze
  7. merge B and C into `ws/m37-int`
  8. start parent integration/test work only after both B and C are merged
  9. merge docs lane if not already merged
  10. rerun the full verification wall
  11. publish back to `feat/corpus-expansion` only if fully green
- The frozen helper-surface wedge outcome must remain unchanged throughout the run:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "not_recommended"`
  - `open_blockers = ["helper_surface_not_promotable"]`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`

## Hard Guards

- `PLAN.md` wins over this file, worker summaries, stale worktrees, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion` is the baseline and publish target, not the merge surface.
- All integration, validation, and final proving happen on `ws/m37-int`, not on the live checkout.
- M37 introduces exactly one new internal module:
  - `xtask/src/family/decision_kernel.rs`
- M37 must not introduce:
  - any `spec-core` change
  - any new CLI command or flag
  - any schema version change
  - any new artifact kind
  - any second new module under `xtask/src/family/`
  - any generic engine, registry, trait layer, or policy DSL
  - any widening beyond the current helper-surface wedge
- `coverage.rs` remains coverage artifact construction and coverage proof fingerprinting only.
- Decision derivation must not perform a hidden coverage reread after validated analysis truth is already loaded.
- After B and C are integrated, there must be no duplicate semantic owners for:
  - basis snapshot derivation
  - helper-surface activation from validated basis truth
  - corpus-program decision derivation
  - normalized recommendation proof fingerprinting
  - normalized corpus-decision proof fingerprinting
- `decision_kernel.rs` is foundation-owned. No worker lane edits it.
- `xtask/src/lib.rs` is integration-owned unless the parent explicitly delegates it in a later freeze file. Default: not delegated.
- Docs lane must not invent new terminology, rename frozen wedge strings, or describe M37 as a broader architecture rewrite than `PLAN.md` allows.
- No worker edits anything under:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- If `PLAN.md` or `ORCH_PLAN.md` changes after freeze, the run stops and restarts from a fresh baseline.
- If overlapping local edits exist on owned surfaces before freeze, the parent either re-anchors around them or blocks the run. It does not overwrite them silently.

## Closed Implementation Surface

| Path | Post-M37 responsibility | Lane owner |
|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs` | helper-surface classifier input, fingerprint matching, frozen durable-hold tuple, frozen follow-on tuple, exact tuple-match predicates only | Parent foundation lane |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs` | single semantic owner for basis snapshot derivation, helper-surface activation from validated basis truth, corpus-program decision derivation, normalized recommendation fingerprint, normalized corpus-decision fingerprint | Parent foundation lane |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs` | candidate ranking, recommendation artifact assembly, latest-byte reuse, command entrypoints, artifact IO | Worker lane B |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs` | serde schema types, path validation, sha validation, schema validation, delegation to kernel for expected semantic truth | Worker lane C |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs` | coverage artifact construction and coverage proof fingerprinting only; no semantic ownership drift | Parent integration/test lane, compile-spillover only |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs` | export `decision_kernel` and preserve family module wiring | Parent foundation lane |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs` | existing regression floor plus all required M37 regression additions and final integration proving | Parent integration/test lane |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md` | boundary truth for the family-analysis decision kernel | Worker lane D |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md` | program tracker update for the new boundary | Worker lane D |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md` | capability guide update for the new boundary | Worker lane D |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md` | exact trigger-based deferred extraction entries | Worker lane D |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | closeout-only note or completion update after full green verification | Parent integration/test lane, closeout only |

Rules for the closed surface:

- Any edit outside this table is out of scope unless it is mechanically forced by merge conflict resolution or compile wiring and is recorded in `merge-log.md`.
- `recommend.rs` and `promotion_artifacts.rs` are worker-owned after `foundation-freeze.json`.
- If the parent must make an adapter-only touch in those files before worker launch to keep the foundation branch compiling, that touch must be recorded in `foundation-freeze.json` and is not permission for workers to expand scope.
- No lane may widen the implementation surface into `spec-core` or any other crate.

## Branch And Worktree Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Canonical branches and worktrees:

| Role | Branch | Worktree |
|---|---|---|
| Live baseline and publish target | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` |
| Parent integration spine | `ws/m37-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int` |
| Parent foundation lane | `ws/m37-foundation` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation` |
| Worker lane B, recommend rewiring | `ws/m37-lane-b-recommend` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-b-recommend` |
| Worker lane C, promotion-artifacts rewiring | `ws/m37-lane-c-promotion-artifacts` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-c-promotion-artifacts` |
| Worker lane D, docs and TODO updates | `ws/m37-lane-d-docs` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-d-docs` |

Creation rules:

1. The parent captures baseline on the live checkout before creating any M37 worktree.
2. `ws/m37-int` is created from the exact SHA recorded in `integration-base.txt`.
3. `ws/m37-foundation` is created from the same exact baseline SHA recorded in `integration-base.txt`.
4. `ws/m37-lane-b-recommend`, `ws/m37-lane-c-promotion-artifacts`, and `ws/m37-lane-d-docs` are created only after `foundation-freeze.json`, and only from the exact integrated SHA recorded there.
5. No worker lane forks from another worker lane.
6. No worker lane forks from the live checkout.
7. If any named worktree already exists with stale state, the parent recreates it and records that action in `session.log`.
8. If the live branch moves after baseline capture but before publish, the parent either re-baselines or explicitly merges the new live head into `ws/m37-int` and reruns the full verification wall.

Canonical worktree creation commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int \
  -b ws/m37-int <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation \
  -b ws/m37-foundation <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-b-recommend \
  -b ws/m37-lane-b-recommend <FOUNDATION_FREEZE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-c-promotion-artifacts \
  -b ws/m37-lane-c-promotion-artifacts <FOUNDATION_FREEZE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-d-docs \
  -b ws/m37-lane-d-docs <FOUNDATION_FREEZE_SHA>
```

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m37_decision_kernel_extraction`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction`

Canonical parent-owned files:

- `baseline.json`
- `integration-base.txt`
- `publish-head.txt`
- `authority-freeze.json`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `run-state.json`
- `tasks.json`
- `queue.json`
- `session.log`
- `foundation-freeze.json`
- `code-converge-freeze.json`
- `lane-b-launch.md`
- `lane-c-launch.md`
- `lane-d-launch.md`
- `merge-log.md`
- `proof-log.json`
- `acceptance.md`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `closeout.md`

Required contents:

- `baseline.json`
  - live branch
  - live HEAD SHA
  - dirty-state summary
  - overlapping local edit summary on the closed implementation surface
  - current recommendation artifact path
  - current decision artifact path
  - current raw byte hashes for recommendation and decision artifacts
  - current frozen wedge output values
- `integration-base.txt`
  - exact SHA used to create `ws/m37-int` and `ws/m37-foundation`
- `publish-head.txt`
  - exact live HEAD SHA captured during baseline
- `authority-freeze.json`
  - snapshot paths for `PLAN.md` and `ORCH_PLAN.md`
  - frozen branch and worktree layout
  - closed implementation surface table checksum or literal copy
  - explicit statement that no worker lanes are yet authorized
- `foundation-freeze.json`
  - exact `ws/m37-int` SHA after parent foundation merge
  - exact `ws/m37-foundation` SHA accepted by the parent
  - the frozen kernel API surface
  - the frozen ownership boundary for `helper_surface.rs`
  - explicit unblock for lanes B, C, and D
  - explicit statement that B and C must not change kernel function names or signatures
  - explicit statement that D is valid only against this freeze SHA
- `code-converge-freeze.json`
  - exact `ws/m37-int` SHA after both code lanes merge
  - accepted lane SHAs for B and C
  - explicit authorization for the parent integration/test lane
  - explicit statement that `xtask/src/lib.rs` remains parent-owned
- `lane-b-launch.md`
  - owned files
  - forbidden files
  - exact PLAN excerpts copied into the worker packet
  - exact commands to run
  - return contract
  - stale-lane invalidation triggers
- `lane-c-launch.md`
  - owned files
  - forbidden files
  - exact PLAN excerpts copied into the worker packet
  - exact commands to run
  - return contract
  - stale-lane invalidation triggers
- `lane-d-launch.md`
  - owned files
  - forbidden files
  - exact PLAN excerpts copied into the worker packet
  - exact commands to run
  - exact TODO wording requirements
  - stale-lane invalidation triggers
- `merge-log.md`
  - source branch
  - source SHA
  - target SHA before merge
  - target SHA after merge
  - conflicts encountered
  - resolutions applied
  - whether the merge preserved the frozen kernel API
- `proof-log.json`
  - command
  - cwd
  - exit code
  - artifact path if applicable
  - raw byte hash if captured
  - normalized fingerprint if captured
  - semantic interpretation
  - pass/fail
- `acceptance.md`
  - final checklist mapped to M37 acceptance criteria
  - existing regression anchor results
  - new test results
  - byte reuse proof
  - unchanged wedge proof
  - validator/emitter shared-kernel proof
  - docs/TODO completion proof
  - publish decision
- `blocked.json`
  - blocking task id
  - blocking lane
  - exact violated guard
  - restart requirement
- `closeout.md`
  - final integrated SHA
  - final live publish SHA
  - commands run
  - accepted deltas
  - deferred follow-ups recorded in `TODOS.md`
  - any allowed `PLAN.md` closeout note

Per-task sentinel directories:

- `task-m37-00-baseline-capture`
- `task-m37-05-authority-freeze`
- `task-m37-10-create-worktrees`
- `task-m37-20-parent-foundation`
- `task-m37-25-foundation-freeze`
- `task-m37-30-launch-lane-b`
- `task-m37-31-launch-lane-c`
- `task-m37-32-launch-lane-d`
- `task-m37-40-merge-lane-b`
- `task-m37-45-merge-lane-c`
- `task-m37-50-parent-integration-test`
- `task-m37-55-merge-lane-d`
- `task-m37-60-final-verification-wall`
- `task-m37-70-publish-back-to-live`
- `task-m37-80-closeout`

Each task directory contains parent-written markers only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Workstream Plan

Task graph:

```text
task-m37-00-baseline-capture
  -> task-m37-05-authority-freeze
      -> task-m37-10-create-worktrees
          -> task-m37-20-parent-foundation
              -> task-m37-25-foundation-freeze
                  -> task-m37-30-launch-lane-b
                  -> task-m37-31-launch-lane-c
                  -> task-m37-32-launch-lane-d
                      -> task-m37-40-merge-lane-b
                      -> task-m37-45-merge-lane-c
                          -> task-m37-50-parent-integration-test
                              -> task-m37-55-merge-lane-d
                                  -> task-m37-60-final-verification-wall
                                      -> task-m37-70-publish-back-to-live
                                          -> task-m37-80-closeout
```

### Parent Task 1 - Baseline Capture

Owner: `Parent`  
Branch: `feat/corpus-expansion`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Record current branch, HEAD SHA, dirty state, and overlapping local edits in `baseline.json`.
2. Write the exact same live HEAD SHA to both `integration-base.txt` and `publish-head.txt`.
3. Capture current recommendation and decision artifact raw byte hashes.
4. Run baseline command wall and verify the frozen helper-surface wedge outcome.
5. Update `session.log`, `tasks.json`, and `queue.json`.

Minimum command wall:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
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

This proves:

- live branch and head are what the run claims
- baseline artifacts validate before any work starts
- the frozen helper-surface wedge outcome is still the same
- raw byte hashes are captured for later reuse comparison

Stop conditions:

- any command fails
- any `jq` assertion fails
- overlapping local edits touch the closed implementation surface and are not explicitly accepted
- the live wedge drifts from the frozen M37 baseline truth

### Parent Task 2 - Authority Freeze

Owner: `Parent`  
Branch: `feat/corpus-expansion`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Snapshot `PLAN.md` and `ORCH_PLAN.md` into `authority-snapshot/`.
2. Write `authority-freeze.json`.
3. Freeze branch layout, worktree layout, implementation surface, and guard set for the run.
4. Mark all worker lanes blocked until foundation freeze.

Minimum command wall:

```bash
git diff -- PLAN.md ORCH_PLAN.md
git rev-parse HEAD
test -f PLAN.md
test -f ORCH_PLAN.md
```

This proves:

- authority files exist
- no unacknowledged in-flight edits are hidden at freeze time
- the freeze is anchored to a known live SHA

Stop conditions:

- either authority file changes during freeze
- scope no longer matches `PLAN.md`
- new overlapping edits appear on authority files during freeze

### Parent Task 3 - Create Worktrees

Owner: `Parent`  
Branch: `ws/m37-int`, `ws/m37-foundation`  
Paths:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation`

Actions:

1. Create `ws/m37-int` from the SHA in `integration-base.txt`.
2. Create `ws/m37-foundation` from the same SHA.
3. Verify both worktrees are exact forks from the frozen baseline.
4. Record creation details in `session.log`.

Minimum command wall:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int \
  -b ws/m37-int "$(cat .runs/m37_decision_kernel_extraction/integration-base.txt)"

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation \
  -b ws/m37-foundation "$(cat .runs/m37_decision_kernel_extraction/integration-base.txt)"

git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int status --short
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation status --short
```

This proves:

- integration and foundation worktrees exist
- both fork from the exact baseline SHA
- both start clean

Stop conditions:

- either worktree cannot be created from the exact baseline SHA
- either worktree points at the wrong SHA
- either worktree has unexpected pre-existing dirt

### Parent Task 4 - Parent Foundation Implementation And Local Verification

Owner: `Parent only`  
Branch: `ws/m37-foundation`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation`

Actions:

1. Create `xtask/src/family/decision_kernel.rs`.
2. Export it from `xtask/src/family/mod.rs`.
3. Move basis snapshot derivation into the kernel.
4. Move corpus-program decision derivation into the kernel.
5. Move recommendation and corpus-decision fingerprint helpers into the kernel.
6. Narrow `helper_surface.rs` to classifier input, fingerprint matching, frozen tuples, and exact tuple-match predicates only.
7. Keep any emergency touch to `recommend.rs` or `promotion_artifacts.rs` adapter-only and record it.

Minimum command wall:

```bash
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --exact
cargo test -p xtask corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_missing -- --exact
cargo test -p xtask corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_stale -- --exact
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

This proves:

- the kernel boundary exists and compiles enough to preserve the current wedge
- the M36 helper-surface decision floor remains intact
- no early change has drifted the live read-side semantics

Stop conditions:

- a second new module seems necessary
- any `spec-core` change becomes necessary
- any new CLI command, flag, schema, or artifact kind becomes necessary
- the current helper-surface outcome drifts

### Parent Task 5 - Foundation Merge And Freeze

Owner: `Parent only`  
Branch: `ws/m37-int`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int`

Actions:

1. Merge `ws/m37-foundation` into `ws/m37-int`.
2. Verify the merged integration branch still preserves the wedge floor.
3. Write `foundation-freeze.json`.
4. Launch authorization for lanes B, C, and D.

Minimum command wall:

```bash
git merge --no-ff ws/m37-foundation
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
git rev-parse HEAD
```

This proves:

- the integration spine now contains the canonical kernel boundary
- downstream worker lanes will fork from the exact accepted foundation state
- the worker API freeze is tied to a concrete integration SHA

Stop conditions:

- merge conflict touches `decision_kernel.rs`, `helper_surface.rs`, or `mod.rs` in a way that changes the frozen API
- merged integration no longer preserves the helper-surface wedge floor

### Parent Task 6 - Launch Worker Lanes B, C, And D

Owner: `Parent only`  
Branch base: `foundation-freeze.json`  
Paths: lane B, lane C, lane D worktrees

Actions:

1. Create worker worktrees from the exact SHA in `foundation-freeze.json`.
2. Write `lane-b-launch.md`, `lane-c-launch.md`, and `lane-d-launch.md`.
3. Deliver the launch packets with owned files, forbidden surfaces, command walls, stale rules, and return contract.
4. Mark each lane started in `tasks.json` and `queue.json`.

Minimum command wall:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-b-recommend \
  -b ws/m37-lane-b-recommend <FOUNDATION_FREEZE_SHA>
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-c-promotion-artifacts \
  -b ws/m37-lane-c-promotion-artifacts <FOUNDATION_FREEZE_SHA>
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-d-docs \
  -b ws/m37-lane-d-docs <FOUNDATION_FREEZE_SHA>
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-b-recommend rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-c-promotion-artifacts rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-d-docs rev-parse HEAD
```

This proves:

- all workers start from the same frozen kernel boundary
- there is no hidden branch skew between code lanes and docs lane
- every worker lane is pinned to an explicit freeze SHA

Stop conditions:

- a worker worktree is created from the wrong SHA
- a launch packet omits owned-file boundaries or stale-lane rules
- any worker packet asks for edits outside the closed implementation surface

### Parent Task 7 - Merge Lane B

Owner: `Parent only`  
Branch: `ws/m37-int`

Actions:

1. Inspect lane B diff against the launch contract.
2. Merge `ws/m37-lane-b-recommend` into `ws/m37-int`.
3. Run the lane-B post-merge command wall.
4. Record the merge in `merge-log.md`.

Minimum command wall:

```bash
git diff --stat ws/m37-int..ws/m37-lane-b-recommend
git merge --no-ff ws/m37-lane-b-recommend
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

This proves:

- only `recommend.rs` moved as expected
- latest-byte reuse and decision derivation still function after the merge
- integration remains on the frozen kernel API

Stop conditions:

- lane B edited forbidden files
- lane B requests kernel API drift
- the merge changes wedge semantics

### Parent Task 8 - Merge Lane C

Owner: `Parent only`  
Branch: `ws/m37-int`

Actions:

1. Inspect lane C diff against the launch contract.
2. Merge `ws/m37-lane-c-promotion-artifacts` into `ws/m37-int`.
3. Run the lane-C post-merge command wall.
4. Write `code-converge-freeze.json` only after both B and C are integrated.

Minimum command wall:

```bash
git diff --stat ws/m37-int..ws/m37-lane-c-promotion-artifacts
git merge --no-ff ws/m37-lane-c-promotion-artifacts
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
git rev-parse HEAD
```

This proves:

- only `promotion_artifacts.rs` moved as expected
- validator and emitter now share kernel truth on the integration branch
- parent integration/test work can now begin from a concrete converged SHA

Stop conditions:

- lane C edited forbidden files
- lane C requests kernel API drift
- validator and emitter still disagree after merge

### Parent Task 9 - Parent Integration/Test Implementation In `xtask/src/lib.rs`

Owner: `Parent only`  
Branch: `ws/m37-int`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int`

Actions:

1. Add the required M37 regression tests in `xtask/src/lib.rs`.
2. Keep `xtask/src/lib.rs` parent-owned through the entire run.
3. Make only compile-spillover touches outside `xtask/src/lib.rs` if strictly required.
4. Prove all required test coverage for the moved semantic seams.

Minimum command wall:

```bash
cargo test -p xtask corpus_program_basis_snapshot_matches_validated_analysis_basis -- --exact
cargo test -p xtask corpus_decision_ready_candidate_maps_to_family_promotion_run -- --exact
cargo test -p xtask corpus_decision_blocked_non_helper_candidate_maps_to_policy_run -- --exact
cargo test -p xtask corpus_decision_without_candidate_stops -- --exact
cargo test -p xtask corpus_decision_proof_fingerprint_is_stable_across_generated_at_churn -- --exact
cargo test -p xtask artifact_schema_rejects_corpus_decision_with_drifted_basis_snapshot -- --exact
cargo test -p xtask artifact_schema_rejects_ready_path_with_architecture_follow_on_tuple -- --exact
cargo test -p xtask corpus_decision_latest_bytes_are_reused_when_semantic_fingerprint_is_unchanged -- --exact
```

This proves:

- the new kernel seams have explicit regression coverage
- the parent, not a worker, owns the final integration proof surface
- M37 is verified as a complete extraction, not just a compile pass

Stop conditions:

- `xtask/src/lib.rs` changes are required before both B and C are merged
- any new test reveals duplicate semantic ownership is still present
- the required tests cannot be authored without widening scope

### Parent Task 10 - Merge Lane D

Owner: `Parent only`  
Branch: `ws/m37-int`

Actions:

1. Inspect lane D diff against the launch contract.
2. Merge `ws/m37-lane-d-docs` into `ws/m37-int`.
3. Confirm terminology matches the frozen foundation boundary.

Minimum command wall:

```bash
git diff --stat ws/m37-int..ws/m37-lane-d-docs
git merge --no-ff ws/m37-lane-d-docs
rg -n "helper_surface|decision_kernel|fingerprint|trigger" \
  semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md TODOS.md
```

This proves:

- docs and TODO updates are aligned to the accepted code boundary
- docs did not invent new architecture or stale M36 framing

Stop conditions:

- lane D was launched against a superseded foundation freeze
- docs introduce terminology not grounded in `PLAN.md`
- TODO trigger entries are duplicated or incomplete

### Parent Task 11 - Final Verification Wall

Owner: `Parent only`  
Branch: `ws/m37-int`

Ordered command wall:

```bash
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --exact
cargo test -p xtask artifact_schema_rejects_corpus_decision_with_contradictory_action_for_helper_surface_basis -- --exact
cargo test -p xtask recommendation_proof_fingerprint_is_stable_across_generated_at_churn -- --exact
cargo test -p xtask corpus_decision_proof_fingerprint_changes_on_semantic_action_change -- --exact
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
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

What each phase proves:

- targeted anchor tests: the M36 floor still holds
- full `cargo test -p xtask`: integration is globally green
- first `recommend` and `corpus-decision` run: artifacts still emit from the integrated code
- `validate-artifact`: validator truth matches emitter truth
- second artifact run plus hash capture: latest-byte reuse remains stable when semantic fingerprints are unchanged
- final `jq` checks: the helper-surface wedge outcome stayed exactly frozen

Stop conditions:

- any anchor regresses
- any new test regresses
- validator and emitter disagree
- byte reuse drifts without semantic change
- the helper-surface outcome drifts from the frozen values

### Parent Task 12 - Publish Back To Live

Owner: `Parent only`  
Branches: `ws/m37-int` -> `feat/corpus-expansion`

Actions:

1. Verify the live branch still matches `publish-head.txt`, or reconcile and rerun the full wall.
2. Publish only from `ws/m37-int`.
3. Record the final integrated SHA and final live SHA.

Minimum command wall:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
```

This proves:

- live branch and integration branch SHAs are known before publish
- publish happens from the integration result, not from a worker branch
- the live branch ends on the accepted integrated SHA

Stop conditions:

- live branch moved incompatibly and was not reconciled
- integration worktree is not fully green
- publish cannot happen cleanly from the accepted integration branch

### Parent Task 13 - Closeout

Owner: `Parent only`  
Branch: `feat/corpus-expansion`

Actions:

1. Write `acceptance.md` and `closeout.md`.
2. Mark the queue complete.
3. If needed, update `PLAN.md` with a closeout-only note after green publish.
4. Record final deferred follow-ups captured in `TODOS.md`.

Minimum command wall:

```bash
git rev-parse HEAD
test -f .runs/m37_decision_kernel_extraction/acceptance.md
test -f .runs/m37_decision_kernel_extraction/closeout.md
```

This proves:

- the final live SHA is captured
- closeout artifacts exist
- the run is documented as complete

Stop conditions:

- publish was not completed
- acceptance evidence is incomplete
- attempted `PLAN.md` change exceeds closeout-only scope

## Lane Definitions

### Lane 1 - Parent Foundation Lane

Owner: `Parent only`  
Branch: `ws/m37-foundation`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/foundation`  
Starts after: `task-m37-10-create-worktrees`  
Concurrency during this lane: `1`

Mission:

- establish the kernel boundary first
- create the only new module, `decision_kernel.rs`
- freeze the exported kernel API that downstream worker lanes must consume
- narrow `helper_surface.rs` to its post-M37 role
- preserve compileability and the current helper-surface wedge outcome

Owned files:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs`

Escalation-only surfaces:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`

Lane command wall:

```bash
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --exact
cargo test -p xtask corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_missing -- --exact
cargo test -p xtask corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_stale -- --exact
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Acceptance criteria for Lane 1:

- `decision_kernel.rs` exists
- `mod.rs` exports `decision_kernel`
- `helper_surface.rs` no longer owns basis snapshot derivation or basis activation truth
- the kernel API is frozen and documented in `foundation-freeze.json`
- the branch compiles and the M36 floor still holds
- the parent merges the accepted foundation branch into `ws/m37-int` before launching any workers

### Lane 2 - Worker Lane B, `recommend.rs` Rewiring

Owner: `Worker, GPT-5.4 high`  
Branch: `ws/m37-lane-b-recommend`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-b-recommend`  
Starts after: `foundation-freeze.json`  
Concurrency during this lane: one of the three allowed post-foundation parallel lanes

Mission:

- rewire `recommend.rs` to consume kernel-owned basis snapshot derivation
- rewire `recommend.rs` to consume kernel-owned decision derivation
- rewire `recommend.rs` to consume kernel-owned normalized proof fingerprints
- remove the hidden coverage reread path
- preserve latest-byte reuse and command wiring

Owned file:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`

Readable but not writable:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`

Lane command wall:

```bash
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Acceptance criteria for Lane 2:

- `recommend.rs` no longer owns corpus-program decision semantics
- `recommend.rs` no longer owns recommendation or corpus-decision proof-fingerprint helpers
- the coverage reread fallback is gone from decision derivation
- recommendation artifact assembly consumes kernel-derived truth
- unchanged semantic inputs still reuse latest bytes
- no kernel signature change is required

### Lane 3 - Worker Lane C, `promotion_artifacts.rs` Rewiring

Owner: `Worker, GPT-5.4 high`  
Branch: `ws/m37-lane-c-promotion-artifacts`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-c-promotion-artifacts`  
Starts after: `foundation-freeze.json`  
Concurrency during this lane: one of the three allowed post-foundation parallel lanes

Mission:

- replace local basis snapshot derivation with a kernel call
- replace local helper-surface alignment reasoning with kernel-derived expected decision truth
- preserve serde, path, sha, and schema validation in `promotion_artifacts.rs`
- make validator and emitter consume the same semantic owner

Owned file:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`

Readable but not writable:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`

Lane command wall:

```bash
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Acceptance criteria for Lane 3:

- validators derive expected basis snapshot truth through the kernel
- validators derive expected decision truth through the kernel
- contradictory decision artifacts are rejected for the same reason as emitter truth
- frozen helper-surface tuple exactness is preserved as a kernel-produced expectation
- no kernel signature change is required

### Lane 4 - Worker Lane D, Docs And TODO Updates

Owner: `Worker, GPT-5.4 high`  
Branch: `ws/m37-lane-d-docs`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/lane-d-docs`  
Starts after: `foundation-freeze.json`  
Concurrency during this lane: one of the three allowed post-foundation parallel lanes

Mission:

- update docs to describe the new boundary truthfully
- update `TODOS.md` with exact trigger-based deferred extraction entries
- avoid inventing terminology or reopening closed scope
- leave `PLAN.md` untouched

Owned files:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md`

Lane command wall:

```bash
rg -n "helper_surface|decision_kernel|fingerprint|trigger" \
  semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md TODOS.md
```

Acceptance criteria for Lane 4:

- docs say helper-surface classification still lives in `helper_surface.rs`
- docs say family-analysis decision truth now lives in `decision_kernel.rs`
- docs say normalized semantic fingerprints remain the proof surface
- `TODOS.md` contains the three exact deferred-extraction entries once, not more than once
- no new terminology, new scope, or new milestones are invented
- `PLAN.md` is not edited by this lane

### Lane 5 - Parent Integration/Test Lane

Owner: `Parent only`  
Branch: `ws/m37-int`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction/int`  
Starts after: `code-converge-freeze.json`  
Concurrency during this lane: `1`

Mission:

- integrate B and C in order
- resolve conflicts without changing the frozen kernel API
- own all `xtask/src/lib.rs` additions and final proving
- merge docs if not already merged
- run the full verification wall
- publish only after full green verification

Owned files:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs`, compile-spillover only
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`, closeout-only and only after all gates pass

Lane command wall:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Acceptance criteria for Lane 5:

- all required new tests are added in `xtask/src/lib.rs`
- existing regression anchors remain green
- B and C both merge into `ws/m37-int` before parent test work starts
- validator and emitter share kernel truth
- byte reuse remains stable when semantic fingerprints are unchanged
- the helper-surface wedge outcome is unchanged
- docs and TODO changes are merged before publish
- `PLAN.md` closeout, if any, happens only after final green

## Worker Launch Packets

Each worker launch packet is a parent-authored, single-source execution note. It must be written to the corresponding `lane-*.md` file and delivered verbatim.

Required prompt ingredients for every worker packet:

1. Milestone title: `M37 - Family-Analysis Decision-Kernel Extraction After M36`.
2. Authority path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`.
3. Launch freeze SHA from `foundation-freeze.json`.
4. Worker branch name and worktree path.
5. Worker model requirement:
   - `GPT-5.4`
   - `reasoning_effort=high`
6. Exact owned files.
7. Exact forbidden files.
8. Exact readable reference files.
9. Exact `PLAN.md` excerpt list for that lane.
10. Exact minimum command wall for that lane.
11. Hard guards copied verbatim:
   - one new module only
   - no `spec-core` changes
   - no new CLI commands
   - no schema changes
   - no generic engine
   - no widening beyond helper-surface wedge
   - no hidden coverage reread in decision derivation
   - no duplicate semantic owners after refactor
12. Merge policy: worker does not merge or publish.
13. Stale-lane rules.
14. Required return format.

Required return format for every worker:

```text
RESULT
- status: ready-to-merge | blocked
- branch: <worker-branch>
- base-freeze-sha: <foundation-freeze-sha>
- head-sha: <worker-head-sha>

FILES
- <absolute path>
- <absolute path>

COMMANDS
- <command> -> <exit/result summary>
- <command> -> <exit/result summary>

CHECKS
- owned-surface-only: yes|no
- kernel-api-change-requested: yes|no
- hidden-coverage-reread-removed-or-not-applicable: yes|no
- duplicate-semantic-owner-left-behind: yes|no

NOTES
- <brief operator note>

BLOCKERS
- <exact blocker or "none">
```

Worker stale-lane rules:

- If `foundation-freeze.json` changes after packet launch, B, C, and D are all stale.
- If B or C requests kernel API drift after `foundation-freeze.json`, both B and C are stale immediately. D is stale too if its packet was issued against the superseded freeze.
- If D was launched against a superseded foundation freeze, D is stale even if its doc content is otherwise correct.
- If a worker edits any forbidden file, that lane is stale.
- Stale workers are not repaired in place. The parent recreates their worktrees from the new freeze SHA and relaunches.

## Context-Control Rules

Every worker launch note must contain:

- the exact branch and worktree path
- the exact frozen base SHA from `foundation-freeze.json`
- the exact owned file set
- the exact forbidden file set
- the exact commands the worker is expected to run
- the exact return contract
- the exact stale-lane invalidation rules
- verbatim copies of the relevant `PLAN.md` excerpts listed below

Required `PLAN.md` excerpts by worker:

- Lane B packet must include:
  - `Locked Implementation Details`
  - `Phase 2 - Rewire recommendation emission`
  - `Code Quality Guardrails`
  - `Verification commands`
  - `Acceptance Criteria`
- Lane C packet must include:
  - `Locked Implementation Details`
  - `Phase 3 - Rewire artifact validation`
  - `Code Quality Guardrails`
  - `Verification commands`
  - `Acceptance Criteria`
- Lane D packet must include:
  - `Phase 4 - Tests, docs, and deferred triggers`
  - `TODOS.md updates required in the same PR`
  - `NOT in scope`
  - `Acceptance Criteria`

Shared worker prohibitions:

- no worker edits `decision_kernel.rs`
- no worker edits `xtask/src/lib.rs`
- no worker edits `.runs/**`
- no worker edits `.semantic-family-artifacts/**`
- no worker edits `PLAN.md`
- no worker edits `ORCH_PLAN.md`
- no worker merges branches
- no worker changes kernel API names or signatures
- no worker widens the milestone into new files, new commands, or new schema surface

## Diff Inspection Before Merge

The parent must inspect every worker diff before merge.

For lane B, confirm:

- only `recommend.rs` changed
- moved ownership is consumption-only, not new local semantic derivation
- no hidden coverage reread remains
- no kernel API edits are present

For lane C, confirm:

- only `promotion_artifacts.rs` changed
- validator now delegates expected basis snapshot and decision truth to the kernel
- no replacement local derivation was introduced
- no kernel API edits are present

For lane D, confirm:

- only the four doc/TODO files changed
- terminology matches the frozen foundation boundary
- TODO triggers are the exact three M37 deferred extractions
- no speculative architecture expansion or M36 carryover wording was introduced

## Conflict Rules

- `decision_kernel.rs` is foundation-owned only. Any requested change to its API after `foundation-freeze.json` invalidates all launched worker lanes and requires restart from a new freeze.
- `xtask/src/lib.rs` is integration-owned unless the parent explicitly delegates it in writing. Default: no delegation.
- `helper_surface.rs` remains foundation-owned for M37. Workers read it but do not edit it.
- `recommend.rs` is single-writer by lane B after freeze.
- `promotion_artifacts.rs` is single-writer by lane C after freeze.
- `coverage.rs` is not a worker lane surface.
- docs lane must not invent terminology or move trigger semantics beyond `PLAN.md`.
- B and C both depend on the exact frozen kernel API. They do not negotiate it between themselves.
- B and C must both merge into `ws/m37-int` before parent test work starts.
- If B or C requires kernel API drift after `foundation-freeze.json`, both lanes are stale immediately.
- If D was launched against a superseded `foundation-freeze.json`, D is stale immediately.
- If any merge conflict touches `decision_kernel.rs`, `helper_surface.rs`, or `mod.rs` after foundation freeze, the run stops and restarts from a new foundation freeze.
- If any merge conflict touches `xtask/src/lib.rs`, the parent resolves it in the integration lane and records the resolution in `merge-log.md`.

## Tests And Acceptance

### Existing regression anchors that must stay green

- `corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on`
- `corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_missing`
- `corpus_decision_does_not_activate_helper_surface_follow_on_when_evidence_is_stale`
- `recommendation_proof_fingerprint_is_stable_across_generated_at_churn`
- `corpus_decision_proof_fingerprint_changes_on_semantic_action_change`
- `artifact_schema_rejects_corpus_decision_with_contradictory_action_for_helper_surface_basis`

### Required new tests in `xtask/src/lib.rs`

- `corpus_program_basis_snapshot_matches_validated_analysis_basis`
- `corpus_decision_ready_candidate_maps_to_family_promotion_run`
- `corpus_decision_blocked_non_helper_candidate_maps_to_policy_run`
- `corpus_decision_without_candidate_stops`
- `corpus_decision_proof_fingerprint_is_stable_across_generated_at_churn`
- `artifact_schema_rejects_corpus_decision_with_drifted_basis_snapshot`
- `artifact_schema_rejects_ready_path_with_architecture_follow_on_tuple`
- `corpus_decision_latest_bytes_are_reused_when_semantic_fingerprint_is_unchanged`

### Verification commands

Minimum command wall:

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

Required unchanged-wedge assertions after final verification:

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

Required byte-reuse proof:

- the parent runs `cargo xtask family recommend --format json` twice with unchanged inputs and records both raw byte hashes
- the parent runs `cargo xtask family corpus-decision --format json` twice with unchanged inputs and records both raw byte hashes
- if normalized semantic fingerprints are unchanged, the second run must reuse identical latest bytes
- raw byte hashes are debug evidence only; semantic fingerprints remain the authoritative proof identity

Required semantic-owner proof:

- `recommend.rs` no longer defines decision derivation or recommendation/corpus-decision fingerprint helpers
- `promotion_artifacts.rs` no longer derives expected basis snapshot or decision truth locally
- validator and emitter both call into the kernel for semantic truth
- `coverage.rs` remains outside the decision-kernel ownership move

Final acceptance checklist:

1. `decision_kernel.rs` exists and is the only new module added for M37.
2. `helper_surface.rs` contains classifier and frozen tuple logic only.
3. `recommend.rs` is rewired and no longer owns semantic decision derivation.
4. `promotion_artifacts.rs` validators delegate semantic truth to the kernel.
5. No hidden coverage reread remains in decision derivation.
6. Existing regression anchors are green.
7. All eight required M37 tests are present and green.
8. Recommendation and decision latest bytes are reused when semantic fingerprints are unchanged.
9. The helper-surface wedge outcome is unchanged.
10. Docs and TODO updates truthfully describe the new boundary.
11. No `spec-core` changes, no CLI changes, no schema changes, and no widened wedge scope were introduced.
12. `PLAN.md` closeout, if any, was written only after all prior items were green.

## Stop And Restart Rules

- Stop immediately if `PLAN.md` changes after `authority-freeze.json`.
- Stop immediately if `ORCH_PLAN.md` changes after `authority-freeze.json`.
- Stop immediately if overlapping local edits exist on any owned surface and have not been explicitly accepted into the baseline.
- Stop immediately if foundation work requires a second new module, any `spec-core` edit, or any new CLI/schema surface.
- Stop immediately if B or C requires a kernel API change after `foundation-freeze.json`. This invalidates B and C together. If D was launched from that freeze, D is stale too.
- Stop immediately if a worker edits a forbidden surface.
- Stop immediately if `decision_kernel.rs` and any downstream file still co-own semantic truth after B and C are merged.
- Stop immediately if the helper-surface wedge output drifts from the frozen tuple.
- Stop immediately if byte reuse breaks while semantic fingerprints remain unchanged.
- Restart from a fresh baseline if the live branch moves and the parent cannot safely reconcile it into `ws/m37-int` with a rerun of the full verification wall.

## Publish And Landing Rules

- Only the parent publishes.
- Only `ws/m37-int` may be merged or fast-forwarded back to `feat/corpus-expansion`.
- Publish is allowed only after:
  - foundation freeze completed
  - B and C merged
  - parent integration/test lane completed
  - docs lane merged
  - full verification wall rerun on `ws/m37-int`
  - acceptance checklist completed in `acceptance.md`
- If the live branch at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` no longer matches `publish-head.txt`, the parent must reconcile that movement and rerun the full verification wall before publish.
- `PLAN.md` closeout note, if any, is written only after publish readiness is proven.
- No worker branch is published directly.
- No publish occurs from the live checkout until the parent has a green `ws/m37-int`.

## Assumptions

- `PLAN.md` at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` remains the M37 authority for the duration of the run.
- The baseline branch is `feat/corpus-expansion` at `d2e69249495049947d414b7126d663ae1452e076`.
- The repo can create the listed worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m37-decision-kernel-extraction`.
- `cargo test -p xtask` and the listed `cargo xtask family ...` commands are the required proving surface.
- Generated artifacts under `.semantic-family-artifacts` are derived outputs, not authored source.
- Any need to widen beyond the helper-surface wedge or move shared logic into another crate is a deferred follow-up, not M37 scope.
