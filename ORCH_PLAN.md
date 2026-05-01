# M27.75 Orchestration Plan

Status: **execution contract**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**
Working branch baseline: **`feat/m27`**
Last rewritten: **2026-05-01**

## Summary

- Execute from the current branch `feat/m27`, because that is the live checked-out
  baseline in this workspace.
- Keep the critical path local to the parent agent for baseline capture, manifest
  contract freeze, integration, derived-artifact regeneration, and final acceptance.
- Use subagents only for the two safe post-freeze lanes:
  - one `xtask` proof worker
  - one docs worker
- Use dedicated worktrees under
  `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_75/{contract,xtask,docs,int}`
  with workstream branches:
  - `ws/m27_75-contract`
  - `ws/m27_75-xtask`
  - `ws/m27_75-docs`
  - `ws/m27_75-int`
- Use GPT-5.4 with `reasoning_effort=high` for all workers. Cap concurrency at `2`.
  The parent agent remains the only integrator.
- Keep orchestration state in one canonical location owned by the parent agent:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_75`
  - `RUN_ROOT=$PRIMARY_ROOT/.runs/m27_75`
  - queue: `$RUN_ROOT/tasks.json`
  - session log: `$RUN_ROOT/session-log.md`
  - manifest freeze record: `$RUN_ROOT/contract-freeze.json`
  - acceptance record: `$RUN_ROOT/acceptance.md`
  - per-task sentinels: `$PRIMARY_ROOT/.runs/<TASK_ID>/`
- Treat `$RUN_ROOT/*` and
  `.semantic-family-artifacts/family-promotion/analysis/*` as run-state and derived
  proof surfaces, not the authored source of truth.

## Hard Guards

- `PLAN.md` is the sole authority for M27.75. If any older orchestration note,
  stale `ORCH_PLAN.md`, chat instruction, or branch-local artifact disagrees, `PLAN.md`
  wins.
- The milestone is M27.75 corpus expansion only.
  - No recommendation-policy edits.
  - No coverage-schema edits.
  - No recommendation-schema edits.
  - No M28 portability or shared-core work.
- Locked source touch set for this run:
  - `semantic-families/corpus/rust-function.toml`
  - `xtask/src/lib.rs`
  - `semantic-families/README.md`
- Explicit non-touch list for this run:
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `README.md`
  - `examples/crosslib-app/README.md`
- Derived proof surfaces expected to change only after integrated reruns:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- The manifest contract is frozen before any worker lane starts.
  - source ids
  - source order
  - source notes
  - expected five-source output contract
- Shared `xtask/src/lib.rs` proof work is a single serialized lane.
  Do not split `seed_locked_recommendation_workspace(...)` and
  `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
  across workers.
- There are no M26-style human approval gates in M27.75.
  The only intentional pauses are stop-and-replan events.
- If the first integrated five-source rerun does not match the locked values in `PLAN.md`,
  stop immediately.
  - Do not silently rewrite `PLAN.md`.
  - Do not “normalize” expected values to whatever the code currently emits.
  - Re-plan from the mismatch.
- If implementation starts requiring edits outside the locked touch set, stop and re-plan.
- If any lane discovers it must change `coverage.rs`, `recommend.rs`, or
  `promotion_artifacts.rs`, stop and re-plan. That means the milestone was mis-scoped.

## Discarded Stale Assumptions

The previous `ORCH_PLAN.md` described M27.5. Do not inherit any of this:

- no ownership of `xtask/src/family/recommend.rs`
- no ownership of `xtask/src/family/promotion_artifacts.rs`
- no ownership of `xtask/src/family/coverage.rs`
- no recommendation-analysis schema work
- no readiness-policy hardening work
- no approval-gated sequencing
- no branch or worktree names with `m27_5`

If execution starts depending on any of the above, stop and re-plan because the
session has fallen back into the wrong milestone.

## Integrator Model

- Parent agent is the only integrator.
- Parent agent is the only authority for:
  - baseline capture
  - manifest contract freeze
  - worker launch
  - merge decisions
  - conflict resolution
  - derived-artifact regeneration
  - acceptance recording
  - final go and no-go
- Maximum active workers: `2`
- Safe worker layout:
  - Lane A: one `xtask` proof worker
  - Lane B: one docs worker
- Lane B may start only after the parent freezes the manifest contract.
- Parallelism is intentionally bounded.
  `xtask/src/lib.rs` is the only Rust source touched in this milestone, so there is
  exactly one safe implementation worker.

## Worktree And Branch Strategy

All execution branches fork from the exact current `feat/m27` baseline SHA.
Do not fork from `main`. Do not develop directly on `feat/m27` once the run starts.

Canonical orchestration roots:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_75`
- `RUN_ROOT=$PRIMARY_ROOT/.runs/m27_75`

Canonical branches:

- contract: `ws/m27_75-contract`
- Lane A: `ws/m27_75-xtask`
- Lane B: `ws/m27_75-docs`
- integration: `ws/m27_75-int`

Canonical worktrees:

- contract: `$WORKTREE_ROOT/contract`
- Lane A: `$WORKTREE_ROOT/xtask`
- Lane B: `$WORKTREE_ROOT/docs`
- integration: `$WORKTREE_ROOT/int`

Creation commands from `PRIMARY_ROOT`:

```bash
mkdir -p "$WORKTREE_ROOT" "$RUN_ROOT"
BASE_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_SHA=$(git rev-parse HEAD)
git worktree add -b ws/m27_75-contract "$WORKTREE_ROOT/contract" "$BASE_SHA"
```

After the manifest contract is frozen and recorded:

```bash
FREEZE_SHA=$(jq -r '.contract_freeze_commit' "$RUN_ROOT/contract-freeze.json")
git worktree add -b ws/m27_75-xtask "$WORKTREE_ROOT/xtask" "$FREEZE_SHA"
git worktree add -b ws/m27_75-docs "$WORKTREE_ROOT/docs" "$FREEZE_SHA"
git worktree add -b ws/m27_75-int "$WORKTREE_ROOT/int" "$FREEZE_SHA"
```

Worktree rules:

- do not reuse dirty worktrees
- do not merge from worker worktrees directly into `feat/m27`
- do not let workers self-merge
- do not create extra side branches beyond the four locked branches
- if the baseline worktree has uncommitted edits in the locked source touch set or
  derived proof surfaces, stop and capture a clean baseline before branching
- dirty planning artifacts such as `PLAN.md` and `ORCH_PLAN.md` may be recorded, but
  they are not a reason to fork ambiguous product branches

## Parent-Owned Run State

Parent-managed orchestration state lives under `$RUN_ROOT`:

- `baseline.json`
- `tasks.json`
- `session-log.md`
- `merge-log.md`
- `contract-freeze.json`
- `acceptance.md`

Minimum `tasks.json` shape:

- `task_id`
- `branch`
- `worktree`
- `owner`
- `status`
- `depends_on`
- `sentinel_dir`

Per-task sentinel directories live under `PRIMARY_ROOT/.runs/<task-id>/` and contain:

- `started.json`
- `status.json`
- `done.json` or `blocked.json`

Worker chat is not the source of truth. Parent-owned run artifacts are.

## Task Graph

```text
task/m27_75-00-baseline
  -> task/m27_75-a1-freeze-manifest-contract
      -> task/m27_75-b1-xtask-proof
      -> task/m27_75-b2-docs
task/m27_75-b1-xtask-proof
task/m27_75-b2-docs
  -> task/m27_75-c1-integrate-and-rerun
      -> task/m27_75-c2-land
```

Execution intent:

1. parent validates the baseline and records dirty state
2. parent freezes the exact manifest contract first
3. workers branch from that freeze point
4. Lane A updates the locked `xtask` proof
5. Lane B updates maintainer docs in parallel
6. parent merges both lanes into integration
7. parent reruns coverage and recommendation from merged state, validates artifacts,
   runs tests, and checks exact locked values
8. parent lands the integrated result back onto `feat/m27`

Serialized tasks:

- `task/m27_75-00-baseline`
- `task/m27_75-a1-freeze-manifest-contract`
- `task/m27_75-c1-integrate-and-rerun`
- `task/m27_75-c2-land`

Parallel-safe window:

- `task/m27_75-b1-xtask-proof`
- `task/m27_75-b2-docs`

That is the only approved parallel phase.

## Lane And Ownership Boundaries

### WS-CONTRACT — parent only

Purpose:

- lock the exact two new manifest entries
- confirm the contract matches `PLAN.md`
- record the freeze point other lanes must branch from

Owned files:

- `semantic-families/corpus/rust-function.toml`
- `$RUN_ROOT/contract-freeze.json`
- parent-owned run-state files only

Forbidden files:

- `xtask/src/lib.rs`
- `semantic-families/README.md`
- all files in the explicit non-touch list

### Lane A — `xtask` proof worker

Purpose:

- extend the existing locked command-path proof to the five-source corpus

Owned files:

- `xtask/src/lib.rs`

Forbidden files:

- `semantic-families/corpus/rust-function.toml`
- `semantic-families/README.md`
- `.semantic-family-artifacts/family-promotion/analysis/*`
- all files in the explicit non-touch list

### Lane B — docs worker

Purpose:

- update maintainer docs after the manifest contract is frozen

Owned files:

- `semantic-families/README.md`

Forbidden files:

- all Rust source files
- `semantic-families/corpus/rust-function.toml`
- `.semantic-family-artifacts/family-promotion/analysis/*`
- all files in the explicit non-touch list

### Parent Integration

Purpose:

- merge completed lane branches
- regenerate derived proof surfaces from merged state
- run final validation
- write acceptance artifacts
- land the integrated result

Parent integration must not invent new product semantics.
Any conflict that requires reinterpretation of `PLAN.md` is a stop-and-bounce event
back to the relevant lane.

## Task Contracts

### `task/m27_75-00-baseline` — parent only

Owned files:

- `$RUN_ROOT/baseline.json`
- `$RUN_ROOT/tasks.json`
- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_75-00-baseline/*`

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
test -f PLAN.md
test -f ORCH_PLAN.md
```

Acceptance:

- current branch is `feat/m27`
- baseline SHA is captured
- dirty state is recorded
- any dirty files inside the locked source touch set or derived proof surfaces halt the run
- `PLAN.md` is present and treated as sole authority
- task graph and branch names are recorded in `tasks.json`

### `task/m27_75-a1-freeze-manifest-contract` — parent only

Owned files:

- `semantic-families/corpus/rust-function.toml`
- `$RUN_ROOT/contract-freeze.json`
- `PRIMARY_ROOT/.runs/task-m27_75-a1-freeze-manifest-contract/*`

Must do:

- add exactly two new manifest entries:
  - `examples_shared_spec`
  - `examples_crosslib_app`
- preserve existing source order and append the new entries as positions `4` and `5`
- preserve the exact notes locked in `PLAN.md`
- record the frozen source ids and notes in `contract-freeze.json`
- record the commit SHA that worker lanes must branch from

Must not do:

- edit `xtask/src/lib.rs`
- edit docs
- reroute or normalize manifest ordering
- touch files outside WS-CONTRACT ownership

Required commands:

```bash
cargo xtask family coverage --format json > /tmp/m27_75-contract-coverage.json
cargo xtask family recommend --format json > /tmp/m27_75-contract-recommend.json
```

Acceptance:

- manifest entries exactly match `PLAN.md`
- coverage preview shows five sources in the locked order
- recommendation preview still reports `no_strong_candidate`
- preview output is used only to confirm contract truth, not as final derived artifact ownership
- `contract-freeze.json` records:
  - `contract_freeze_commit`
  - `source_ids`
  - `source_notes`
  - `expected_recommendation_status`
  - `expected_candidate_count`

Stop conditions:

- preview output disagrees with the locked expected output deltas in `PLAN.md`
- any edit outside the manifest is needed to make preview output look right

### `task/m27_75-b1-xtask-proof` — Lane A

Owned files:

- `xtask/src/lib.rs`
- `PRIMARY_ROOT/.runs/task-m27_75-b1-xtask-proof/*`

Must do:

- update `seed_locked_recommendation_workspace(...)` so it copies:
  - `examples/shared-spec/units`
  - `examples/crosslib-app/units`
- extend
  `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
  to assert the full five-source truth from `PLAN.md`
- keep the test as the single command-path regression lock for this milestone unless
  a second test is absolutely necessary for readability

Must assert:

- source ids in exact order:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- source unit counts in exact order:
  - `6`
  - `12`
  - `9`
  - `1`
  - `1`
- `recommendation_status == no_strong_candidate`
- ranked candidate count is `2`
- first candidate is the `unsupported_function_surface-e40675da6fa0` cluster
- first candidate hold reasons equal exactly:
  - `unknown_overlap_family`
- first candidate leverage equals:
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 3`
- second candidate is the `unsupported_arithmetic_shape-2694b2baf65b` cluster
- second candidate hold reasons equal exactly:
  - `thin_real_example_support`
  - `thin_regression_support`
- second candidate leverage equals:
  - `real_example_hits = 1`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 2`

Must not do:

- edit the manifest
- edit docs
- edit `coverage.rs`, `recommend.rs`, or `promotion_artifacts.rs`
- write final derived artifacts

Required commands:

```bash
cargo test -p xtask recommendation_command_path -- --color never
cargo test -p xtask -- --color never
```

Acceptance:

- the updated test proves the exact five-source contract
- no other `xtask` source file is touched
- no policy logic changes are introduced

Stop conditions:

- the test can only be made green by changing policy or artifact code
- worker needs files outside `xtask/src/lib.rs`

### `task/m27_75-b2-docs` — Lane B

Owned files:

- `semantic-families/README.md`
- `PRIMARY_ROOT/.runs/task-m27_75-b2-docs/*`

Must do:

- update the `Corpus Source Kinds` section to stay truthful under M27.75
- replace the stale “exactly three sources” text
- list the new five-source manifest explicitly
- describe `examples/shared-spec/units` and `examples/crosslib-app/units` as
  maintained `real_example` sources

Must not do:

- edit root `README.md`
- edit the manifest
- edit Rust source files
- edit derived proof artifacts

Acceptance:

- maintainer docs no longer claim a three-source manifest
- docs wording matches the frozen manifest contract exactly
- no non-owned file changes are present

Stop conditions:

- docs lane discovers another repo file must change to keep M27.75 truthful
- worker needs to touch root `README.md` to make the milestone coherent

### `task/m27_75-c1-integrate-and-rerun` — parent only

Owned files:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `$RUN_ROOT/merge-log.md`
- `$RUN_ROOT/acceptance.md`
- `PRIMARY_ROOT/.runs/task-m27_75-c1-integrate-and-rerun/*`

Must do:

- merge `ws/m27_75-xtask` and `ws/m27_75-docs` into `ws/m27_75-int`
- resolve only mechanical merge conflicts
- rerun the integrated proof loop from merged state
- regenerate both analysis artifacts from merged state only
- validate both artifacts through the existing path-aware command
- confirm exact output deltas against `PLAN.md`

Required commands:

```bash
cargo fmt --all

tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"

cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"

cargo test -p xtask -- --color never
```

Acceptance:

- merged branch still touches only the locked source touch set plus derived proof surfaces
- `coverage.latest.json` and `recommendation.latest.json` both validate
- stdout bytes match the written artifact bytes for both commands
- coverage sources ids equal:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- coverage source unit counts equal:
  - `6`
  - `12`
  - `9`
  - `1`
  - `1`
- `function_coverage` equals:
  - `total_units = 27`
  - `promoted_family_units = 15`
  - `supported_unpromoted_family_units = 0`
  - `unsupported_function_units = 12`
- `non_function_coverage` equals:
  - `total_units = 2`
  - `supported_sum_units = 1`
  - `supported_data_units = 1`
  - `other_units = 0`
- family coverage remains exactly three promoted families at `unit_count = 5` each
- recommendation status remains `no_strong_candidate`
- ranked candidate count equals `2`
- first candidate remains the unsupported-function-surface cluster with:
  - `hold_reasons = ["unknown_overlap_family"]`
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `total_units_in_cluster = 3`
- second candidate remains the arithmetic-shape cluster with:
  - `hold_reasons = ["thin_real_example_support", "thin_regression_support"]`
  - `real_example_hits = 1`
  - `promotion_relevant_regression_hits = 1`
  - `total_units_in_cluster = 2`

Stop conditions:

- merged output disagrees with the locked values above
- `validate-artifact` fails on either derived proof surface
- any change to non-touch files becomes necessary to go green

### `task/m27_75-c2-land` — parent only

Owned files:

- primary branch merge state only
- `PRIMARY_ROOT/.runs/task-m27_75-c2-land/*`

Must do:

- land the integrated result back onto `feat/m27` from the parent worktree only
- rerun the core acceptance loop once on the landed branch
- record the final landed SHA in `acceptance.md`

Required commands:

```bash
git checkout feat/m27
git merge --ff-only ws/m27_75-int || git merge --no-ff ws/m27_75-int
cargo test -p xtask -- --color never
```

Acceptance:

- final landed branch contains the locked source touch set only
- final landed branch still passes the `xtask` suite
- acceptance record contains:
  - landed SHA
  - commands run
  - final status

## Context-Control Rules

- Parent agent keeps only five live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `$RUN_ROOT/tasks.json`
  - the acceptance checklist
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - the frozen manifest contract
  - required commands
  - forbidden touch surfaces
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not write `$RUN_ROOT/*`.
- Workers do not own `.semantic-family-artifacts/family-promotion/analysis/*`.
- The parent agent reviews summaries plus narrow diffs only. It does not ingest
  full worker transcripts into the main context.
- Close each worker immediately after merge.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Manifest contract
  - `semantic-families/corpus/rust-function.toml` ends with the two locked
    `real_example` entries and preserves the original first three entries and order.
- `xtask` proof
  - `seed_locked_recommendation_workspace(...)` copies the two new example trees.
  - the existing command-path regression test locks the full five-source truth.
  - no policy logic changes appear in `xtask` runtime code.
- Docs
  - `semantic-families/README.md` no longer claims a three-source manifest.
  - docs list the five-source manifest explicitly and truthfully.
- Derived proof surfaces
  - `coverage.latest.json` validates and matches stdout byte-for-byte.
  - `recommendation.latest.json` validates and matches stdout byte-for-byte.
  - both derived artifacts reflect the locked M27.75 expectations from `PLAN.md`.
- Operator flow
  - parent freezes manifest contract first
  - workers branch from the recorded freeze commit only
  - parent integrates, regenerates proof surfaces, validates, and lands
- Scope discipline
  - no edits to `coverage.rs`, `recommend.rs`, `promotion_artifacts.rs`, root `README.md`,
    or `examples/crosslib-app/README.md`

## Assumptions

- Worktree naming follows the repo’s existing `spec-m26/*` and `spec-m27/*` pattern.
- `cargo xtask family validate-artifact <path>` already exists and is the correct runtime
  validation surface for the two derived analysis artifacts.
- The current `PLAN.md` locked expected output deltas are authoritative.
- `.semantic-family-artifacts/family-promotion/analysis/*` remains a derived proof surface
  throughout M27.75, not the authored contract surface.
- No human approval gate is required for this milestone because `PLAN.md` does not call for one.
