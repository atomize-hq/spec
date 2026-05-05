# M35 Orchestration Plan

Status: **authoritative execution contract for the M35 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Last rewritten: **`2026-05-05`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m35_architecture_shared_core_follow_on`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Recommendation basis path: **`.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Decision artifact path: **`.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`**  
Reality-alignment source: **validated M34 commit `df15e3e392be30a13b10f028eb19e4286c931523` from `ws/m34-int`**  
Execution note: **M35 is a two-phase run: first land the validated M34 contract unchanged in behavior, then extract one bounded helper-surface classifier inside `xtask/src/family/` and rewire both recommend and corpus-decision to use it.**

## Summary

- This run is for **M35 architecture shared-core follow-on** only.
- `PLAN.md` remains milestone authority. `ORCH_PLAN.md` is the parent-owned execution contract for the session that lands M35 safely.
- The parent agent is the sole baseline capturer, sole freeze authority, sole M34 lander, sole helper-surface API freezer, sole integrator, sole stale-lane invalidator, sole final verifier, sole publish authority, and sole closeout author.
- The opening path is strictly sequential:
  1. capture baseline on `feat/corpus-expansion`
  2. freeze authority and create `ws/m35-int`
  3. land validated M34 commit `df15e3e392be30a13b10f028eb19e4286c931523`
  4. verify the landed M34 wedge
  5. add and freeze `xtask/src/family/helper_surface.rs`
- Real parallelism starts only after `helper-surface-api-freeze.json` exists.
- Recommended worker profile for every post-freeze lane is:
  - `GPT-5.4`
  - `reasoning_effort=high`
- Worker concurrency cap is:
  - `0` before `m34-landing-freeze.json`
  - `0` before `helper-surface-api-freeze.json`
  - `3` after `helper-surface-api-freeze.json`
- The parent remains the only integrator. Workers never merge each other, never publish, and never write orchestration state.
- `.runs/**` and `.semantic-family-artifacts/**` are run artifacts and derived output surfaces:
  - not authored source
  - not assumed git-tracked deliverables
  - not worker-owned edit surfaces
- The live wedge must remain explicit end to end:
  - recommendation basis remains `no_strong_candidate`
  - recommendation decision status remains `not_recommended`
  - blocker remains `helper_surface_not_promotable`
  - corpus decision remains `pivot_to_architecture_shared_core_follow_on`
  - decision basis remains `durable_non_promotable_helper_surface`
  - required next action remains `author_architecture_follow_on_plan`
- M35 does not spend corpus run `1`, does not promote a family, does not create a new crate, does not create a new artifact family, does not move helper-surface truth into `spec-core`, and does not widen into a generic decision engine.

## Hard Guards

- `PLAN.md` wins over this file, worker summaries, stale worktree copies, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen on `ws/m35-int`.
- The live checkout on `feat/corpus-expansion` is the publish target and baseline reference, not the merge surface.
- The parent records live branch name, head SHA, dirty state, and overlapping local edits before creating any M35 worktree.
- If local or incoming edits overlap the M35-owned surface before `authority-freeze.json`, the parent must either re-anchor around them or block the run. It must not silently overwrite them.
- After `authority-freeze.json` is written, both authority files are frozen:
  - `PLAN.md`
  - `ORCH_PLAN.md`
- If either authority file changes after freeze, the run stops and restarts from a fresh baseline.
- Reality alignment must land the validated M34 commit first. No M35-only edits start before that gate is green.
- The M34 landing must preserve behavior. Preferred path is:
  - `git cherry-pick -x df15e3e392be30a13b10f028eb19e4286c931523`
- If cherry-pick conflicts, the parent may resolve only to produce a behavior-equivalent landed result and must record that rationale in `session-log.md` and `m34-landing-freeze.json`.
- The shared-core extraction is locked to one new module:
  - `xtask/src/family/helper_surface.rs`
- The shared classifier is locked to one bounded concept:
  - `durable_non_promotable_helper_surface`
- The classifier must not:
  - emit corpus actions
  - emit recommendation statuses
  - read files
  - live in `spec-core`
  - become a generic decision engine
- `family corpus-decision` must keep reading the existing recommendation artifact. It must not rescan coverage or recompute recommendation from raw corpus inputs.
- No one hand-edits JSON under `.semantic-family-artifacts/`. Derived artifacts are produced only by repo commands.
- `.runs/**` and `.semantic-family-artifacts/**` remain run artifacts and derived output, not authored source.
- M35 must not edit or widen into:
  - `spec-core/src/**`
  - `semantic-families/corpus/rust-function.toml`
  - any promoted family packet under `semantic-families/**`
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/inventory.rs`
  - `xtask/src/family/report.rs`
  - prove/certify runtime behavior
- `xtask/src/family/paths.rs` may change only as part of the M34 landing. No M35-only edits are allowed there after `m34-landing-freeze.json`.
- `xtask/src/lib.rs` is parent-owned for M35:
  - workers do not edit it
  - any M35 test additions in `xtask/src/lib.rs` are parent-only in integration
  - any corpus-decision dispatch or command wiring in `xtask/src/lib.rs` remains whatever M34 landed; workers do not touch it
- Stop immediately if any lane requires:
  - a second new module
  - a new crate
  - a new artifact family
  - moving helper-surface truth into `spec-core`
  - a generic decision-policy framework
  - a second semantic classifier in validators

## Closed Implementation Surface

Parent-owned or lane-owned work is limited to:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs` (new)
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs` only for M34 landing
- `xtask/src/lib.rs` parent only
- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `PLAN.md` completion notes only after final verification

Allowed mechanical spillover is compile- or module-wire-forced only:

- imports
- module exports
- test names or fixtures inside `xtask/src/lib.rs`

## Worktree Layout

Canonical worktrees:

- integration and parent-owned sequential lane
  - branch: `ws/m35-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/int`
- lane A, consumer rewiring
  - branch: `ws/m35-lane-a-consumers`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/lane-a-consumers`
- lane B, validator alignment
  - branch: `ws/m35-lane-b-validators`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/lane-b-validators`
- lane C, docs alignment
  - branch: `ws/m35-lane-c-docs`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/lane-c-docs`

Creation rules:

1. The parent captures baseline on the live branch before creating any M35 worktree.
2. `ws/m35-int` is created from the exact SHA recorded in `integration-base.txt`.
3. `ws/m35-int` lands M34 and produces both `m34-landing-freeze.json` and `helper-surface-api-freeze.json` before any worker lane exists.
4. Every worker lane forks from the exact SHA recorded in `helper-surface-api-freeze.json`.
5. No worker is forked from another worker branch.
6. If any named worktree already exists with stale state, the parent recreates it and records that in `session-log.md`.
7. A stale lane is discarded and recreated from the latest relevant freeze SHA. The parent does not hand-forward stale worker branches.
8. If the live branch moves after baseline capture, the parent either re-baselines and rebuilds orchestration state or blocks publish.

## Parent vs Worker Ownership

### Parent-owned always

- baseline capture
- authority freeze
- M34 landing
- M34 behavior verification
- helper-surface module creation
- helper-surface API freeze
- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/lib.rs`
- worker launch packets
- all merges
- stale-lane invalidation
- final regression tests
- green-path validation
- blocked-path capture
- publish and CI observation
- closeout

### Lane A: consumer rewiring

Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Owned path:

- `xtask/src/family/recommend.rs`

Mission:

- rewire recommendation durable-hold logic inside `recommend.rs` to use the frozen shared classifier
- rewire the landed M34 corpus-decision derivation logic inside `recommend.rs` to use the same frozen shared classifier
- preserve outward artifact vocabulary and live-wedge behavior

Lane A may change only inside `xtask/src/family/recommend.rs`:

- helper-surface signal construction
- helper-surface classification call sites
- recommendation durable-hold branching
- corpus-decision basis derivation branching
- local helper functions in `recommend.rs` that become unnecessary after extraction

Lane A may not change:

- `xtask/src/lib.rs`
- command dispatch
- clap enum wiring
- parent-owned tests in `xtask/src/lib.rs`
- `promotion_artifacts.rs`
- `helper_surface.rs`
- `mod.rs`
- `.runs/**`
- `.semantic-family-artifacts/**`

### Lane B: validator alignment

Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Owned path:

- `xtask/src/family/promotion_artifacts.rs`

Mission:

- align recommendation and corpus-decision tuple validators to the frozen helper-surface contract without introducing a second classifier

Lane B may change only inside `xtask/src/family/promotion_artifacts.rs`:

- tuple-validation rules
- schema-validation error text
- corpus-decision tuple acceptance/rejection paths
- recommendation durable-hold tuple acceptance/rejection paths

Lane B may not change:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/lib.rs`
- `.runs/**`
- `.semantic-family-artifacts/**`

### Lane C: docs alignment

Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Owned paths:

- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`

Mission:

- explain that helper-surface pressure is still real
- explain that one shared classifier now owns non-promotability classification
- explain that recommendation analysis remains input truth
- explain that corpus-decision remains operator-action output
- avoid implying M35 spent corpus run `1` or implemented a broader engine

Lane C may not change:

- any `xtask/src/**`
- `PLAN.md`
- `ORCH_PLAN.md`
- `.runs/**`
- `.semantic-family-artifacts/**`

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m35_architecture_shared_core_follow_on`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on`
- `ARTIFACT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`

Canonical parent-owned files:

- `baseline.json`
- `integration-base.txt`
- `publish-head.txt`
- `closed-surface-base.txt`
- `authority-freeze.json`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `tasks.json`
- `session-log.md`
- `basis-freeze.json`
- `m34-landing-freeze.json`
- `helper-surface-api-freeze.json`
- `lane-a-launch.md`
- `lane-b-launch.md`
- `lane-c-launch.md`
- `merge-log.md`
- `code-freeze.json`
- `green-path-record.json`
- `proof-log.json`
- `push-record.json`
- `ci-observation.json`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `blocked-analysis.sha-before`
- `blocked-analysis.sha-after`
- `blocked-decision.present-before`
- `blocked-decision.sha-before`
- `blocked-decision.present-after`
- `blocked-decision.sha-after`
- `blocked-decision.change-status`
- `blocked-stop-reason.txt`
- `closeout.md`

Required freeze-record contents:

- `basis-freeze.json`
  - live branch name
  - live head SHA
  - dirty-state summary
  - exact recommendation artifact SHA
  - exact helper-surface wedge assertions
- `m34-landing-freeze.json`
  - exact landed SHA
  - proof that `df15e3e392be30a13b10f028eb19e4286c931523` is ancestor or was cherry-picked
  - exact M34-owned paths now present
  - exact emitted corpus-decision wedge
- `helper-surface-api-freeze.json`
  - exact launch SHA for all worker lanes
  - frozen module path
  - frozen enum/type/function names
  - frozen classifier semantics
  - explicit statement that `paths.rs` is frozen after M34 landing
  - frozen docs vocabulary:
    - `helper_surface_not_promotable`
    - `durable_non_promotable_helper_surface`
    - `author_architecture_follow_on_plan`

Per-task sentinel directories:

- `task-m35-00-baseline`
- `task-m35-01-authority-freeze`
- `task-m35-02-reality-alignment`
- `task-m35-03-helper-api-freeze`
- `task-m35-a-consumers`
- `task-m35-b-validators`
- `task-m35-c-docs`
- `task-m35-04-parent-integration`
- `task-m35-05-code-freeze`
- `task-m35-06-green-path`
- `task-m35-07-final-verify`
- `task-m35-08-push-observe`
- `task-m35-09-closeout`

Each sentinel directory contains:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Launch Packets And Worker Return Contract

Parent-written launch packets:

- `RUN_ROOT/lane-a-launch.md`
- `RUN_ROOT/lane-b-launch.md`
- `RUN_ROOT/lane-c-launch.md`

Each launch packet must include:

- lane id
- branch name
- worktree path
- owned paths
- forbidden paths
- exact relevant `PLAN.md` excerpt
- exact relevant `ORCH_PLAN.md` excerpt
- exact `helper-surface-api-freeze.json` excerpt
- required commands
- acceptance criteria
- return contract
- stale-lane invalidation triggers

Worker return contract is fixed for every lane. A worker returns only:

- changed files
- commands run and exit codes
- blockers or unresolved assumptions

A worker does not return:

- transcript dumps
- long reasoning logs
- ad hoc plan rewrites
- edits to `.runs/**`
- edits to `.semantic-family-artifacts/**`

The parent records worker completion, merge, or relaunch outcomes in `merge-log.md` and `session-log.md`.

## Task Graph

```text
task/m35-00-baseline
  -> task/m35-01-authority-freeze
      -> task/m35-02-reality-alignment
          -> task/m35-03-helper-api-freeze
              -> task/m35-a-consumers
              -> task/m35-b-validators
              -> task/m35-c-docs
task/m35-a-consumers
  -> task/m35-04-parent-integration
task/m35-b-validators
  -> task/m35-04-parent-integration
task/m35-c-docs
  -> task/m35-04-parent-integration
task/m35-04-parent-integration
  -> task/m35-05-code-freeze
      -> task/m35-06-green-path
          -> task/m35-07-final-verify
              -> task/m35-08-push-observe
                  -> task/m35-09-closeout
```

Execution meaning:

1. Parent proves the live branch still carries the expected M33/M34 helper-surface basis.
2. Parent freezes authority and creates the integration worktree.
3. Parent lands validated M34 before any M35-only edits.
4. Parent freezes the new helper-surface API before any worker launches.
5. Workers operate only on disjoint owned paths from the exact freeze SHA.
6. Parent integrates all lanes and owns `xtask/src/lib.rs` for final regression additions and dispatch reconciliation.
7. Parent runs the authoritative green-path and final verification floors from merged integration state only.
8. Parent publishes only the exact verified `ws/m35-int` SHA.

## Workstream Plan

### WS-0 Baseline capture and wedge proof

#### `task/m35-00-baseline`

Parent mission:

- capture the live branch baseline and prove that the current recommendation artifact still matches the helper-surface wedge that M34 and M35 both depend on

Required commands:

```bash
git branch --show-current
git rev-parse --verify HEAD
git status --short
git diff --name-only
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact "$ANALYSIS_PATH"
jq -e '.recommendation_status == "no_strong_candidate"' "$ANALYSIS_PATH"
jq -e '.decision_summary.decision_status == "not_recommended"' "$ANALYSIS_PATH"
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' "$ANALYSIS_PATH"
jq -e '.evidence_summary.missing_evidence == [] and .evidence_summary.stale_evidence == []' "$ANALYSIS_PATH"
shasum -a 256 "$ANALYSIS_PATH"
```

Acceptance:

- live branch is `feat/corpus-expansion`
- recommendation artifact validates
- live basis still matches the helper-surface wedge
- overlapping local edits inside the M35-owned surface are either absent or explicitly recorded as a blocker
- exact working-tree bytes of `PLAN.md` are snapshotted under `authority-snapshot/PLAN.md`

### WS-1 Authority freeze and integration worktree creation

#### `task/m35-01-authority-freeze`

Parent mission:

- freeze the orchestration contract and create `ws/m35-int` from the recorded baseline SHA

Required commands:

```bash
BASE_SHA=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m35_architecture_shared_core_follow_on/integration-base.txt)
git worktree add -b ws/m35-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/int "$BASE_SHA"
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m35-architecture-shared-core-follow-on/int rev-parse --verify HEAD
```

Acceptance:

- `authority-freeze.json` exists
- `tasks.json` exists
- `ws/m35-int` exists and points at the recorded baseline SHA
- no worker launches before this checkpoint completes

### WS-2 Reality alignment: land validated M34 first

#### `task/m35-02-reality-alignment`

Parent mission:

- land the validated M34 contract onto `ws/m35-int` before any M35-only edits begin

Required parent actions:

1. Land `df15e3e392be30a13b10f028eb19e4286c931523` from `ws/m34-int`.
2. Verify these surfaces now exist on `ws/m35-int`:
   - `cargo xtask family corpus-decision --format json`
   - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
   - M34 action vocabulary and docs
3. Freeze M34 behavior before any new helper-surface extraction work starts.

Preferred command floor:

```bash
git cherry-pick -x df15e3e392be30a13b10f028eb19e4286c931523
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
```

Acceptance:

- M34 commit is landed on `ws/m35-int`
- M34 command surface exists and validates
- live wedge remains unchanged after landing
- `m34-landing-freeze.json` exists before any M35-only edits begin

### WS-3 Helper-surface API freeze

#### `task/m35-03-helper-api-freeze`

Parent mission:

- create the shared helper-surface module, freeze the API, and make that frozen commit the only worker launch base

Required parent actions:

1. Add `xtask/src/family/helper_surface.rs`.
2. Export it from `xtask/src/family/mod.rs`.
3. Freeze the API shape:
   - `HelperSurfaceSignal`
   - `HelperSurfaceDisposition`
   - `classify_helper_surface(...) -> Option<HelperSurfaceDisposition>`
4. Freeze the semantics:
   - `UnsupportedFunctionReasonCode::UnsupportedFunctionSurface`
   - `overlap_family == "unknown"`
   - `real_example_hits > 0`
   - current helper/no-deps fingerprint rule
5. Keep the classifier pure, explicit, and wedge-specific.
6. Keep `recommend.rs` and `promotion_artifacts.rs` consumer rewiring for later lanes.

Suggested command floor:

```bash
cargo fmt --all
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
```

Acceptance:

- `helper_surface.rs` exists
- `mod.rs` exports it
- `helper-surface-api-freeze.json` exists
- all worker lanes will fork from this exact commit
- no worker may alter the frozen helper-surface API without invalidation and relaunch

### WS-4 Parallel lanes after API freeze

All lanes fork from the exact SHA recorded in `helper-surface-api-freeze.json`.

#### `task/m35-a-consumers` on `ws/m35-lane-a-consumers`

Worker mission:

- rewire both recommendation and corpus-decision consumers inside `xtask/src/family/recommend.rs` to call the frozen shared classifier

Required outcomes:

- `recommend.rs` no longer owns an inline helper-surface classifier
- the durable-hold recommendation path still emits:
  - `hold_reasons = ["helper_surface_not_promotable"]`
  - `next_step_status = "durable_hold"`
  - `next_step_detail = "helper_surface_not_promotable"`
- the corpus-decision path still emits:
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`

Lane-local command floor:

```bash
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --color never
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Acceptance:

- only `xtask/src/family/recommend.rs` changed
- both consumers use the frozen shared classifier
- no `xtask/src/lib.rs` edits exist on the lane branch
- outward artifact bytes and wedge vocabulary remain unchanged on unchanged basis

#### `task/m35-b-validators` on `ws/m35-lane-b-validators`

Worker mission:

- align recommendation and corpus-decision tuple validators to the frozen helper-surface contract without introducing a second classifier

Required outcomes:

- validators continue to enforce tuple consistency
- validators do not independently reconstruct helper-surface semantics from raw fields
- `promotion_artifacts.rs` remains the artifact schema/validation layer only

Lane-local command floor:

```bash
cargo test -p xtask artifact_schema_ -- --color never
```

Acceptance:

- only `xtask/src/family/promotion_artifacts.rs` changed
- helper-surface validation remains tuple-based, not semantic reclassification
- no new artifact kind or schema family appears

#### `task/m35-c-docs` on `ws/m35-lane-c-docs`

Worker mission:

- align maintainer docs to the frozen M35 boundary and vocabulary

Lane-local command floor:

```bash
rg -n "helper_surface_not_promotable|durable_non_promotable_helper_surface|author_architecture_follow_on_plan|corpus-decision|corpus-program-decision.latest.json" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
! rg -n "new crate|generic decision engine|spec-core owns helper-surface truth|M35 spends corpus run 1|M35 promotes a new family" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
```

Acceptance:

- only the three owned docs paths changed
- docs explain the new shared code owner honestly
- docs do not imply widened scope

## Merge Order, Conflict Flags, And Relaunch Rules

### Exact merge order into `ws/m35-int`

1. merge `ws/m35-lane-a-consumers`
2. merge `ws/m35-lane-b-validators`
3. parent-only `xtask/src/lib.rs` integration and regression completion
4. run targeted post-code-merge test floor
5. merge `ws/m35-lane-c-docs`
6. run doc grep floor
7. run code-freeze, green-path, and final verification floors

Rationale:

- lane A and lane B are the code-bearing lanes and define the M35 functional contract
- `xtask/src/lib.rs` is parent-only and is the correct place to finish cross-consumer regressions after both code lanes are present
- docs merge after code/test freeze avoids wording drift if the parent needed to tighten test names or exact vocabulary during integration

### Merge command concept

Parent uses only non-interactive merges from `ws/m35-int`:

```bash
git merge --no-ff ws/m35-lane-a-consumers
git merge --no-ff ws/m35-lane-b-validators
# parent-only xtask/src/lib.rs integration edits and regression completion happen here
git merge --no-ff ws/m35-lane-c-docs
```

If the parent decides squash or cherry-pick is safer for a lane, that choice must be recorded in `merge-log.md` and the lane's acceptance must be re-proved on the integrated tree.

### Conflict magnets

The main conflict magnets are:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- helper-surface vocabulary strings that appear in docs and tests

### Conflicts that the parent may resolve in integration

The parent may resolve only straightforward merge mechanics:

- import ordering
- module import additions
- formatting-only collisions
- test name updates in parent-owned `xtask/src/lib.rs`
- doc wording alignment after code vocabulary is frozen

### Conflicts that bounce the lane instead of being resolved creatively

Bounce the lane and recreate it if any of these happen:

- lane A edits any file other than `xtask/src/family/recommend.rs`
- lane B edits any file other than `xtask/src/family/promotion_artifacts.rs`
- lane C edits any file outside its three owned docs
- a lane rewrites the frozen helper-surface API
- a lane requires `xtask/src/lib.rs` edits
- a lane introduces a second semantic classifier
- a lane changes the emitted live-wedge vocabulary
- a lane implies changes to `spec-core`
- a lane requires widening into a new artifact family, new crate, or generic engine
- a lane makes the parent choose between conflicting interpretations of helper-surface semantics rather than the frozen contract

### Lane discard and relaunch rules

Discard and recreate a lane from `helper-surface-api-freeze.json` if:

- the lane touched a forbidden path
- the lane diverged from the frozen API or vocabulary
- the lane cannot merge without semantic conflict on the frozen contract
- the parent changed the frozen API after launch
- the lane’s owned-file diff was polluted by unrelated edits
- the worker return contract is incomplete or ambiguous
- targeted lane acceptance commands fail on the lane branch

The parent does not hand-forward a stale lane. Stale or polluted lanes are deleted and relaunched cleanly.

## Parent Integration Command Floor

### `task/m35-04-parent-integration`

Parent mission:

- merge all worker lanes into `ws/m35-int`, resolve only straightforward conflicts, and finish the parent-owned `xtask/src/lib.rs` regression surface

Required parent merge and validation floor:

```bash
git checkout ws/m35-int
git merge --no-ff ws/m35-lane-a-consumers
git merge --no-ff ws/m35-lane-b-validators
cargo fmt --all
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
cargo test -p xtask corpus_decision_maps_helper_surface_wedge_to_architecture_follow_on -- --color never
cargo test -p xtask artifact_schema_rejects_corpus_decision_with_contradictory_action_for_helper_surface_basis -- --color never
cargo test -p xtask corpus_decision_command_path_writes_same_bytes_for_unchanged_basis -- --color never
# parent-only xtask/src/lib.rs regression completion happens here if needed
cargo fmt --all
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
git merge --no-ff ws/m35-lane-c-docs
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|author_architecture_follow_on_plan|corpus-decision|corpus-program-decision.latest.json' semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
```

Acceptance:

- lane A and lane B merge before any parent `xtask/src/lib.rs` completion
- parent-only `xtask/src/lib.rs` work is complete before docs merge
- lane C merges only after code/test vocabulary is frozen
- targeted post-merge test floor passes

## Blocked-Path Evidence Capture

### Parent-owned blocked-path files

If any step after `authority-freeze.json` fails and the parent decides the run is blocked, the parent must write these files under `RUN_ROOT`:

- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `blocked-analysis.sha-before`
- `blocked-analysis.sha-after`
- `blocked-decision.present-before`
- `blocked-decision.sha-before`
- `blocked-decision.present-after`
- `blocked-decision.sha-after`
- `blocked-decision.change-status`
- `blocked-stop-reason.txt`
- `blocked.json`

### Canonical blocked-path capture floor

The parent must run this exact evidence-capture floor with the real failing command and exit code:

```bash
RUN_ROOT="/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m35_architecture_shared_core_follow_on"
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
FAILING_COMMAND="${FAILING_COMMAND:?set FAILING_COMMAND to the exact command that failed}"
FAILING_EXIT_CODE="${FAILING_EXIT_CODE:?set FAILING_EXIT_CODE to the exact non-zero exit code}"

ANALYSIS_SHA_BEFORE=$(shasum -a 256 "$ANALYSIS_PATH" | awk '{print $1}')
DECISION_PRESENT_BEFORE=0
DECISION_SHA_BEFORE=""
if [ -f "$DECISION_PATH" ]; then
  DECISION_PRESENT_BEFORE=1
  DECISION_SHA_BEFORE=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
fi

printf '%s\n' "$FAILING_COMMAND" > "$RUN_ROOT/blocked-failing-command.txt"
printf '%s\n' "$FAILING_EXIT_CODE" > "$RUN_ROOT/blocked-failing-exit-code.txt"
printf '%s\n' "$ANALYSIS_SHA_BEFORE" > "$RUN_ROOT/blocked-analysis.sha-before"
printf '%s\n' "$DECISION_PRESENT_BEFORE" > "$RUN_ROOT/blocked-decision.present-before"
printf '%s\n' "$DECISION_SHA_BEFORE" > "$RUN_ROOT/blocked-decision.sha-before"

cargo xtask family validate-artifact "$ANALYSIS_PATH"

ANALYSIS_SHA_AFTER=$(shasum -a 256 "$ANALYSIS_PATH" | awk '{print $1}')
DECISION_PRESENT_AFTER=0
DECISION_SHA_AFTER=""
if [ -f "$DECISION_PATH" ]; then
  DECISION_PRESENT_AFTER=1
  cargo xtask family validate-artifact "$DECISION_PATH"
  DECISION_SHA_AFTER=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
fi

printf '%s\n' "$ANALYSIS_SHA_AFTER" > "$RUN_ROOT/blocked-analysis.sha-after"
printf '%s\n' "$DECISION_PRESENT_AFTER" > "$RUN_ROOT/blocked-decision.present-after"
printf '%s\n' "$DECISION_SHA_AFTER" > "$RUN_ROOT/blocked-decision.sha-after"

if [ "$DECISION_PRESENT_BEFORE" = "1" ] && [ "$DECISION_PRESENT_AFTER" = "1" ]; then
  if [ "$DECISION_SHA_BEFORE" = "$DECISION_SHA_AFTER" ]; then
    printf 'stable\n' > "$RUN_ROOT/blocked-decision.change-status"
  else
    printf 'changed_unexpectedly\n' > "$RUN_ROOT/blocked-decision.change-status"
  fi
elif [ "$DECISION_PRESENT_BEFORE" = "0" ] && [ "$DECISION_PRESENT_AFTER" = "1" ]; then
  printf 'appeared_during_failure_window\n' > "$RUN_ROOT/blocked-decision.change-status"
elif [ "$DECISION_PRESENT_BEFORE" = "1" ] && [ "$DECISION_PRESENT_AFTER" = "0" ]; then
  printf 'disappeared_during_failure_window\n' > "$RUN_ROOT/blocked-decision.change-status"
else
  printf 'absent_both_before_and_after\n' > "$RUN_ROOT/blocked-decision.change-status"
fi

printf '%s\n' 'stop_publish_and_closeout' > "$RUN_ROOT/blocked-stop-reason.txt"
test "$FAILING_EXIT_CODE" -ne 0
```

Blocked-path rules:

- the parent preserves the exact failing command and exit code
- the parent preserves recommendation artifact SHA before and after
- the parent preserves decision artifact presence and SHA before and after
- the parent classifies decision-artifact change status as one of:
  - `stable`
  - `changed_unexpectedly`
  - `appeared_during_failure_window`
  - `disappeared_during_failure_window`
  - `absent_both_before_and_after`
- the parent writes `blocked.json`
- the parent stops downstream publish and closeout
- the parent does not report partial green success

## Code Freeze

### `task/m35-05-code-freeze`

Parent mission:

- freeze the merged M35 code lane only after the targeted test and determinism floors pass

Required commands:

```bash
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
PRE_SHA=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo xtask family corpus-decision --format json
POST_SHA=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
test "$PRE_SHA" = "$POST_SHA"
```

Acceptance:

- targeted M35 xtask tests pass
- corpus-decision output is byte-stable on unchanged basis
- `code-freeze.json` exists before green-path validation

## Green-Path Validation

### `task/m35-06-green-path`

Parent mission:

- run the exact merged-state green-path floor and prove the live wedge is unchanged

Required commands:

```bash
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.recommendation_status == "no_strong_candidate"' "$ANALYSIS_PATH"
jq -e '.decision_summary.decision_status == "not_recommended"' "$ANALYSIS_PATH"
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' "$ANALYSIS_PATH"
jq -e '.evidence_summary.missing_evidence == [] and .evidence_summary.stale_evidence == []' "$ANALYSIS_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
```

Acceptance:

- both artifacts validate
- recommendation wedge is unchanged
- corpus-decision wedge is unchanged
- `green-path-record.json` records artifact SHAs and command results

## Final Verification

### `task/m35-07-final-verify`

Parent mission:

- run the exact final verification floor from `ws/m35-int` before calling M35 done

Required commands:

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
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|author_architecture_follow_on_plan|corpus run `1`' semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md PLAN.md
```

Acceptance:

- formatting, clippy, tests, artifact generation, validation, and doc grep all pass
- recommendation and corpus-decision remain aligned on the helper-surface wedge
- no forbidden scope widening is present

## Publish And Observe

### `task/m35-08-push-observe`

Parent mission:

- publish only the exact verified `ws/m35-int` SHA

Publish safety rules:

- parent publishes only if `publish-head.txt` still matches the intended live-branch publish base, or the parent explicitly re-baselines first
- parent never publishes from a worker branch
- if live `PLAN.md` authority bytes changed since `authority-freeze.json`, block publish and restart
- parent records pushed SHA and CI observation in `push-record.json` and `ci-observation.json`

## Closeout

### `task/m35-09-closeout`

Parent mission:

- record the final outcome, lane history, freeze history, blocked-path status if any, and scope-control summary

Closeout must state:

- M34 was landed first
- helper-surface truth now has one code owner in `xtask/src/family/helper_surface.rs`
- recommendation and corpus-decision both consume that shared classifier
- live wedge still says:
  - do not spend corpus run `1`
  - do not promote a new family
  - pivot to `author_architecture_follow_on_plan`
- no new crate, no new artifact family, no generic decision engine, no move into `spec-core`

## Stale-Lane Invalidation Rules

- Any change to `helper-surface-api-freeze.json` invalidates all worker lanes.
- Any parent edit to `xtask/src/family/recommend.rs` after lane A launch invalidates lane A.
- Any parent edit to `xtask/src/family/promotion_artifacts.rs` after lane B launch invalidates lane B.
- Any parent change to the frozen docs vocabulary after lane C launch invalidates lane C.
- Any worker touching a forbidden path is discarded and relaunched from the freeze SHA.
- Any lane that fails its own acceptance command floor is discarded and relaunched.
- Any lane merged after drifting from the frozen helper-surface contract is invalid and must be recreated.
- The parent never patches a stale worker branch forward manually. Stale or polluted lanes are deleted and relaunched cleanly.

## Context-Control Rules

- Parent keeps only these live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - latest freeze record
  - latest integration diff summary
- Each worker prompt contains only:
  - owned paths
  - forbidden paths
  - exact freeze-record excerpt
  - exact acceptance commands
  - required return contract
- Workers return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not return transcript dumps.
- Workers do not write `.runs/**` or `.semantic-family-artifacts/**`.
- The parent reviews narrow diffs and summaries only, not full worker transcripts.
- Close each worker immediately after merge or invalidation.

## Tests And Acceptance

### M34 landing acceptance

- `df15e3e392be30a13b10f028eb19e4286c931523` is landed on `ws/m35-int` before any M35-only edits.
- `cargo xtask family corpus-decision --format json` exists on the integration branch.
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json` validates after landing.
- The landed M34 wedge still emits:
  - `decision_action = pivot_to_architecture_shared_core_follow_on`
  - `decision_basis_code = durable_non_promotable_helper_surface`
  - `required_next_action = author_architecture_follow_on_plan`

### Helper-surface API freeze acceptance

- `xtask/src/family/helper_surface.rs` exists.
- `xtask/src/family/mod.rs` exports the new module.
- `helper-surface-api-freeze.json` records the exact API and semantics.
- No worker launches before the API freeze exists.
- Any post-freeze API change invalidates every worker lane.

### Lane acceptance

- Lane A acceptance:
  - only `xtask/src/family/recommend.rs` changed
  - recommendation and corpus-decision logic in that file both call the shared classifier
  - no `xtask/src/lib.rs` edits exist on the lane branch
- Lane B acceptance:
  - only `xtask/src/family/promotion_artifacts.rs` changed
  - validator behavior remains tuple-consistency validation only
  - no second semantic classifier exists
- Lane C acceptance:
  - only the three owned docs changed
  - docs describe the shared helper-surface code owner accurately
  - docs do not imply widened scope

### Operator flow acceptance

- Sequential opening path is honored:
  - baseline
  - authority freeze
  - M34 landing
  - helper-surface API freeze
- Worker lanes start only after the API freeze.
- Merge order into `ws/m35-int` is honored:
  - lane A
  - lane B
  - parent-only `xtask/src/lib.rs` integration
  - lane C
- Parent integration runs the targeted post-merge test floor.
- Blocked-path capture runs before any blocked termination is declared.
- Green-path and final verification floors both pass before publish.

### Workspace boundary acceptance

- No new crate is added.
- No new artifact family is added.
- No `spec-core` helper-surface migration occurs.
- No widening into a generic decision engine occurs.
- No worker writes `.runs/**` or `.semantic-family-artifacts/**`.
- `.runs/**` and `.semantic-family-artifacts/**` remain run artifacts and derived output, not authored source.

### Final publish acceptance

- `cargo fmt --all --check` passes.
- `cargo clippy -p xtask --all-targets --all-features -- -D warnings` passes.
- `cargo test -p xtask helper_surface -- --color never` passes.
- `cargo test -p xtask recommend -- --color never` passes.
- `cargo test -p xtask corpus_decision -- --color never` passes.
- `cargo test -p xtask artifact_schema_ -- --color never` passes.
- Coverage, recommendation, and corpus-decision artifacts validate.
- Final doc grep floor passes.
- Publish occurs only from the exact verified `ws/m35-int` SHA.
- If any publish precondition fails, the run stops with blocked-path evidence rather than partial publish.

## Done When

1. `feat/corpus-expansion` has the validated M34 command surface first.
2. `xtask/src/family/helper_surface.rs` exists and is the sole helper-surface classifier.
3. `recommend.rs` no longer owns an inline helper-surface classifier.
4. `corpus-decision` and recommendation both use the same shared classifier.
5. Validators remain tuple-consistency checks, not a second classifier.
6. Final verification proves the live wedge is unchanged.
7. No new crate, no new artifact family, no generic decision framework, and no `spec-core` widening were introduced.
