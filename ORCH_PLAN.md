# M27.5 Orchestration Plan

Status: **execution contract**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**
Working branch baseline: **`feat/m27`**
Last rewritten: **2026-04-30**

## Session Goal

This document orchestrates the full M27.5 session from the current `feat/m27`
baseline through final acceptance.

M27.5 is a narrow recommendation-quality hardening milestone. It is not the
original M27 discovery buildout. The job is to harden recommendation-analysis
truth so the repo can honestly distinguish visible pressure from promotion-ready
pressure.

The run ends only when:

1. recommendation-analysis is upgraded independently to schema version `2`
2. recommendation candidates expose `promotion_readiness` and `hold_reasons`
3. readiness, confidence, and top-level status rules match `PLAN.md`
4. locked regression tests prove the current corpus no longer overclaims `ranked`
5. maintainer docs explain the sharpened meaning of `ranked`
6. final merged validation is green on the M27.5 touch set

## Authority And Scope Locks

- `PLAN.md` is the sole authority for M27.5. If any older orchestration note,
  branch-local note, chat instruction, or stale milestone artifact disagrees,
  `PLAN.md` wins.
- The baseline for this run is the current `feat/m27` branch head, not `main`
  and not any older M27 orchestration branch.
- Scope is recommendation-quality hardening only.
- This run does not absorb broader M27 discovery work, corpus expansion,
  portability, or M28 kickoff.
- Coverage semantics stay unchanged.
- Recommendation-analysis schema bumps independently to version `2`.
- There is no M26-style human approval gate in this milestone. The parent agent
  still stops on authority drift, scope drift, or ownership drift.

Locked product touch set:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- optional tiny helper edits in `xtask/src/family/coverage.rs` only if strictly
  necessary

Everything else is out of scope for this run unless the parent stops and
re-plans.

## Discarded Stale Assumptions

The previous `ORCH_PLAN.md` described the wrong milestone. Do not inherit these
assumptions:

- no `spec-core/*` work
- no corpus-manifest authoring work
- no new discovery buildout tasks
- no new artifact family rollout
- no branch baseline on any pre-M27 branch
- no touch-set expansion to `paths.rs`, `mod.rs`, manifest files, or other
  unrelated surfaces
- no approval-gated milestone sequencing

If execution starts depending on any of the above, stop and re-plan because the
session has left M27.5.

## Integrator Model

- Parent agent is the only integrator.
- Parent agent is the only authority for:
  - merge decisions
  - conflict resolution
  - terminology freeze
  - acceptance recording
  - final go/no-go
- Maximum active workers: `2`
- Safe worker layout:
  - Lane A: one implementation worker on serialized `xtask` work
  - Lane B: one docs worker on `semantic-families/README.md`
- Lane B may start only after the parent freezes public terminology and status
  wording from Lane A.

Parallelism is intentionally bounded. `recommend.rs`,
`promotion_artifacts.rs`, and `xtask/src/lib.rs` are a shared lane and must not
be split across concurrent implementation workers.

## Worktree And Branch Strategy

All execution branches fork from the exact current `feat/m27` baseline SHA.
Do not fork from `main`. Do not develop directly on `feat/m27`.

Canonical orchestration roots:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_5`
- `RUN_ROOT=$PRIMARY_ROOT/.runs/m27_5`

Canonical branches:

- integration: `codex/m27_5-int`
- Lane A: `codex/m27_5-lane-a`
- Lane B: `codex/m27_5-docs`

Canonical worktrees:

- integration: `$WORKTREE_ROOT/int`
- Lane A: `$WORKTREE_ROOT/lane-a`
- Lane B: `$WORKTREE_ROOT/docs`

Creation commands from `PRIMARY_ROOT`:

```bash
mkdir -p "$WORKTREE_ROOT" "$RUN_ROOT"
BASE_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_SHA=$(git rev-parse HEAD)
git worktree add -b codex/m27_5-int "$WORKTREE_ROOT/int" "$BASE_SHA"
git worktree add -b codex/m27_5-lane-a "$WORKTREE_ROOT/lane-a" "$BASE_SHA"
git worktree add -b codex/m27_5-docs "$WORKTREE_ROOT/docs" "$BASE_SHA"
```

Worktree rules:

- do not reuse dirty worktrees
- do not merge from worker worktrees directly into `feat/m27`
- do not let workers self-merge
- do not create side branches beyond the three locked branches
- if the baseline worktree has uncommitted edits in the M27.5 touch set, stop
  and capture a clean baseline SHA before starting worker branches

## Parent-Owned Run State

Parent-managed orchestration state lives under `$RUN_ROOT`:

- `baseline.json`
- `tasks.json`
- `session-log.md`
- `merge-log.md`
- `acceptance.md`

Minimum `tasks.json` shape:

- `task_id`
- `branch`
- `worktree`
- `owner`
- `status`
- `depends_on`
- `sentinel_dir`

Per-task sentinel directories live under `PRIMARY_ROOT/.runs/<task-id>/` and
contain:

- `started.json`
- `status.json`
- `done.json` or `blocked.json`

Worker chat is not the source of truth. Parent-owned run artifacts are.

## Task Graph

```text
task/m27_5-00-baseline
  -> task/m27_5-a1-schema-validator
      -> task/m27_5-a-freeze-terms
          -> task/m27_5-a2-policy-tests
          -> task/m27_5-b1-docs
task/m27_5-a2-policy-tests
task/m27_5-b1-docs
  -> task/m27_5-c-integrate
```

Execution intent:

1. parent validates the baseline and freezes authority
2. Lane A implements schema and validator changes first
3. parent freezes public terminology and status wording
4. Lane A continues with recommendation policy and locked tests
5. Lane B updates docs in parallel after the freeze point
6. parent merges both lanes into integration and runs final acceptance

Serialized tasks:

- `task/m27_5-00-baseline`
- `task/m27_5-a1-schema-validator`
- `task/m27_5-a-freeze-terms`
- `task/m27_5-a2-policy-tests`
- `task/m27_5-c-integrate`

Parallel-safe window:

- `task/m27_5-a2-policy-tests`
- `task/m27_5-b1-docs`

That is the only approved parallel phase.

## Lane And Ownership Boundaries

### Lane A

Purpose:

- implement the schema split
- enforce recommendation-analysis validation rules
- implement readiness, confidence, ordering, and status policy
- land locked regression tests

Lane A owned files:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/lib.rs`
- optional tiny helper edits in `xtask/src/family/coverage.rs` only if strictly
  necessary

Lane A forbidden files:

- `semantic-families/README.md`
- every file outside the locked product touch set

### Lane B

Purpose:

- update maintainer docs after terminology is frozen

Lane B owned files:

- `semantic-families/README.md`

Lane B forbidden files:

- all Rust source files
- every file outside the locked product touch set

### Parent Integration

Purpose:

- merge completed lane branches
- resolve mechanical conflicts only
- run final validation from merged state
- write final orchestration artifacts

Parent integration must not introduce new product semantics. Any conflict that
requires reinterpretation of `PLAN.md` is a stop-and-bounce event back to the
relevant lane.

## Task Contracts

### `task/m27_5-00-baseline` - parent only

Owned files:

- `$RUN_ROOT/baseline.json`
- `$RUN_ROOT/tasks.json`
- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_5-00-baseline/*`

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
test -f PLAN.md
```

Acceptance:

- current branch is `feat/m27`
- baseline SHA is captured
- dirty state is recorded
- no uncommitted edits exist in the locked M27.5 touch set, or the run is
  halted pending a clean baseline commit
- `PLAN.md` is present and treated as sole authority
- task graph and branch names are recorded in `tasks.json`

### `task/m27_5-a1-schema-validator` - Lane A

Owned files:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/lib.rs`

Must do:

- replace shared schema versioning with artifact-specific constants
- keep coverage semantics and coverage schema unchanged
- bump only recommendation-analysis to schema version `2`
- add `promotion_readiness`
- add `hold_reasons`
- add enums:
  - `PromotionReadiness`
  - `HoldReason`
- encode validator rules:
  - `ready` requires empty `hold_reasons`
  - `hold` requires non-empty `hold_reasons`
  - `ranked` requires the first candidate to be `ready`

Must not do:

- implement the final recommendation policy yet
- widen coverage artifact schema
- edit docs
- touch files outside Lane A ownership

Required commands:

```bash
cargo test -p xtask -- --color never
```

Acceptance:

- recommendation-analysis schema is independently versioned to `2`
- coverage semantics remain untouched
- new field names are final:
  - `promotion_readiness`
  - `hold_reasons`
- new enum spellings are final
- validator rejects inconsistent ready/hold combinations
- validator accepts a valid schema `2` recommendation-analysis artifact
- branch is green enough for terminology freeze

### `task/m27_5-a-freeze-terms` - parent only

Owned files:

- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_5-a-freeze-terms/*`

Freeze record:

- field names:
  - `promotion_readiness`
  - `hold_reasons`
- readiness values:
  - `ready`
  - `hold`
- hold reasons:
  - `unknown_overlap_family`
  - `hard_difficulty`
  - `thin_real_example_support`
  - `thin_regression_support`
- top-level statuses:
  - `ranked`
  - `insufficient_real_corpus`
  - `no_strong_candidate`

Acceptance:

- Lane A field names and public statuses are stable
- parent records the frozen vocabulary in `session-log.md`
- Lane B may now start

### `task/m27_5-a2-policy-tests` - Lane A

Owned files:

- `xtask/src/family/recommend.rs`
- `xtask/src/lib.rs`
- `xtask/src/family/promotion_artifacts.rs`
- optional tiny helper edits in `xtask/src/family/coverage.rs` only if strictly
  necessary

Must do:

- keep recommendation logic explicitly two-layer:
  - discovery projection
  - readiness adjudication
- implement readiness rules exactly:
  - hold when `overlap_family == "unknown"`
  - hold when `difficulty.tier == "hard"` and `real_example_hits < 2`
  - hold when `real_example_hits == 0`
  - hold when `real_example_hits == 1` and
    `promotion_relevant_regression_hits < 3`
  - hold when `promotion_relevant_regression_hits <= 1` and
    `real_example_hits <= 1`
- map hold reasons exactly:
  - `unknown_overlap_family`
  - `hard_difficulty`
  - `thin_real_example_support`
  - `thin_regression_support`
- implement confidence rules exactly:
  - `high` when `real_example_hits >= 3` and overlap is known
  - `medium` when `real_example_hits >= 2` and overlap is known
  - `medium` when `real_example_hits == 1`,
    `promotion_relevant_regression_hits >= 3`,
    `difficulty.tier != "hard"`, and overlap is known
  - otherwise `low`
- implement ready-first ordering while preserving current leverage ordering
  inside each readiness bucket
- implement status evaluation order exactly:
  1. `ranked`
  2. `insufficient_real_corpus`
  3. `no_strong_candidate`
- keep weak candidates visible in output even when held
- add locked tests in `xtask/src/lib.rs`

Locked tests required by this task:

- unknown-overlap hard candidate with one real example -> `hold`
- no discoverable candidates -> `insufficient_real_corpus`
- discoverable-but-held candidates -> `no_strong_candidate`
- known-overlap adjacent candidate with strong evidence -> `ranked`
- validator accepts schema `2` recommendation-analysis artifact
- current locked corpus no longer returns `ranked`
- command-path test proves stdout bytes equal written recommendation artifact
- command-path test proves held candidates remain visible and the locked corpus
  yields `no_strong_candidate`

Must not do:

- change coverage semantics
- introduce new command surface
- broaden touch set beyond the allowed files

Required commands:

```bash
cargo test -p xtask -- --color never
tmpdir=$(mktemp -d)
cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
```

Acceptance:

- recommendation policy matches `PLAN.md`
- top-level status is derived from readiness, not raw discoverability
- ready candidates sort ahead of held candidates
- locked corpus returns `no_strong_candidate`
- `money/round` remains visible
- `money/round` is `hold`
- `money/round` hold reasons include:
  - `unknown_overlap_family`
  - `hard_difficulty`
  - `thin_real_example_support`
- command-path output matches written artifact bytes
- schema `2` artifact validates

### `task/m27_5-b1-docs` - Lane B

Owned files:

- `semantic-families/README.md`

Must do:

- update maintainer wording only after terminology freeze
- state plainly:
  - `ranked` means promotion-worthy next-family pressure
  - visible held candidates are not errors
  - `no_strong_candidate` is an honest outcome
- keep docs aligned with the frozen field names and status terms

Must not do:

- invent new policy language beyond `PLAN.md`
- edit any Rust file

Required commands:

```bash
rg -n "promotion_readiness|hold_reasons|ranked|no_strong_candidate|hold" semantic-families/README.md
```

Acceptance:

- README matches the frozen M27.5 terminology
- README does not describe stale M27 discovery assumptions
- README does not imply broader scope than recommendation-quality hardening

### `task/m27_5-c-integrate` - parent only

Scope:

- merge Lane A and Lane B into `codex/m27_5-int`
- resolve mechanical conflicts only
- run final merged validation
- write final acceptance artifacts

Merge order:

1. merge `codex/m27_5-lane-a` into `codex/m27_5-int`
2. merge `codex/m27_5-docs` into `codex/m27_5-int`

Merge commands:

```bash
git -C "$WORKTREE_ROOT/int" merge --no-ff codex/m27_5-lane-a
git -C "$WORKTREE_ROOT/int" merge --no-ff codex/m27_5-docs
```

Mechanical conflict resolution allowed:

- import ordering
- adjacent test edits
- markdown line-wrap conflicts

Bounce back to a lane owner if integration reveals:

- policy disagreements
- schema disagreements
- status-rule disagreements
- any need to touch new files

Required commands:

```bash
cargo fmt --all
cargo test -p xtask -- --color never
tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
```

Acceptance:

- merged state is green
- only the locked touch set changed
- coverage stdout bytes equal the written artifact bytes
- unchanged coverage artifact validates through `cargo xtask family validate-artifact`
- coverage semantics remain unchanged
- recommendation artifact is regenerated from merged state
- recommendation stdout bytes equal written artifact bytes
- final acceptance checklist is written to `$RUN_ROOT/acceptance.md`
- merge SHAs and any mechanical resolutions are written to `$RUN_ROOT/merge-log.md`

## Context-Control Rules

- Worker prompts contain only:
  - relevant `PLAN.md` excerpt
  - owned files
  - forbidden files
  - commands
  - acceptance criteria
- Do not paste the full stale `ORCH_PLAN.md` into worker prompts.
- Do not paste unrelated milestone history into worker prompts.
- Parent reviews only:
  - narrow diff
  - command results
  - short status summary
- Parent does not use worker chat as acceptance evidence.
- If Lane A changes frozen terminology after Lane B starts, parent cancels Lane B,
  refreshes the freeze record, and relaunches docs work from a clean branch.
- Close idle workers after merge. Do not keep dormant workers attached to the run.

## Hard Validation Rules To Preserve

These are non-negotiable and must appear in task prompts and review:

- recommendation-analysis schema version is `2`
- `ready` requires `hold_reasons == []`
- `hold` requires `hold_reasons` to be non-empty
- `ranked` requires a ready first candidate
- ready-first ordering is mandatory
- coverage semantics are unchanged
- coverage stdout and persisted artifact bytes must still match after the schema split
- no `single_source_pressure` addition in M27.5

## Final Acceptance Checklist

M27.5 is complete only when all of the following are true:

1. only the locked touch set changed
2. recommendation-analysis alone moved to schema version `2`
3. candidate entries expose `promotion_readiness` and `hold_reasons`
4. validator rules for `ready`, `hold`, and `ranked` are enforced
5. readiness rules exactly match `PLAN.md`
6. confidence rules exactly match `PLAN.md`
7. status rules are evaluated in the locked order from `PLAN.md`
8. ready candidates sort ahead of held candidates
9. weak held candidates remain visible in output
10. current locked corpus yields `no_strong_candidate`
11. current locked corpus does not falsely yield `ranked`
12. `money/round` remains visible with the expected hold reasons
13. a stronger known-overlap candidate can still yield `ranked`
14. `cargo xtask family recommend --format json` remains deterministic
15. recommendation stdout bytes equal the written artifact bytes
16. `cargo xtask family validate-artifact <path>` accepts the schema `2`
    recommendation-analysis artifact
17. `cargo xtask family coverage --format json` remains deterministic
18. coverage stdout bytes equal the written artifact bytes
19. unchanged coverage artifact validates through `cargo xtask family validate-artifact`
20. coverage semantics remain unchanged after the schema split
21. `semantic-families/README.md` documents the sharpened meaning of `ranked`

## Stop Conditions

Stop immediately and re-plan if any of the following happens:

- baseline branch is not `feat/m27`
- baseline worktree has uncommitted edits in the M27.5 touch set
- any task requires files outside:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/lib.rs`
  - `semantic-families/README.md`
  - optional tiny helper edits in `xtask/src/family/coverage.rs`
- coverage semantics need to change to make M27.5 pass
- recommendation-analysis schema bump tries to drag other artifact schemas with it
- execution starts depending on `spec-core/*`, corpus-manifest work, or other
  stale M27 assumptions
- Lane B starts before terminology freeze
- locked corpus still returns `ranked` after Lane A claims completion
- integration requires semantic conflict resolution instead of mechanical merge
- collaborators have already modified a locked M27.5 file on the baseline branch
  and there is no agreed clean SHA to branch from

## Cleanup

After final acceptance is green:

- update `$RUN_ROOT/acceptance.md`
- update `$RUN_ROOT/merge-log.md`
- leave generated artifacts as run outputs from the merged state
- remove worktrees only after the parent confirms integration is complete

Cleanup commands:

```bash
git worktree remove "$WORKTREE_ROOT/lane-a"
git worktree remove "$WORKTREE_ROOT/docs"
git worktree remove "$WORKTREE_ROOT/int"
```
