# M27.9 Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Primary branch baseline: **`feat/corpus-expansion`**  
Starting HEAD: **`1ef4489`**  
Frozen run artifacts: **`.runs/m27_9/tasks.json`, `.runs/m27_9/session-log.md`, `.runs/m27_9/baseline.json`**  
Last rewritten: **2026-05-01**

## Summary

- Execute from the live branch `feat/corpus-expansion`. `PLAN.md` is the only implementation authority. `ORCH_PLAN.md` exists to drive execution, not reinterpret scope.
- Keep the critical path local to the parent agent for:
  - baseline capture
  - semantic-boundary implementation and validation
  - final integration
  - xtask lock refresh
  - final proof loop
- Use subagents only for the two honest post-semantic lanes:
  - Lane B: repair the M20 unsupported pack and lock CLI truth
  - Lane C: refresh maintainer docs
- Do not invent fake parallelism. M27.9 is mostly serialized because `xtask` truth must stay downstream of semantic truth and CLI truth.
- Use dedicated worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/{m20-cli,docs,int}` with workstream branches:
  - `ws/m27_9-m20-cli`
  - `ws/m27_9-docs`
  - `ws/m27_9-int`
- Keep the parent agent as the only integrator. Workers never self-merge and never own derived artifacts.
- Use GPT-5.4 with `reasoning_effort=high` for all workers. Cap concurrency at `2`.
- Keep orchestration state in one parent-owned source of truth:
  - queue: `.runs/m27_9/tasks.json`
  - session log: `.runs/m27_9/session-log.md`
  - baseline snapshot: `.runs/m27_9/baseline.json`
  - dirty-tree snapshot: `.runs/m27_9/dirty-state.json`
  - per-task sentinels: `.runs/<TASK_ID>/`
- Treat `.runs/m27_9/**` and `.semantic-family-artifacts/**` as run artifacts and derived proof surfaces, not authored source.

## Hard Guards

- `PLAN.md` wins over memory, worker suggestions, stale run notes, and old `ORCH_PLAN.md`.
- No new family packet, no new packet directory, and no new compatibility key.
- No recommendation-policy rewrite.
- No artifact schema rewrite.
- No corpus manifest change.
- No edits to:
  - `semantic-families/corpus/rust-function.toml`
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/inventory.rs`
  unless the parent explicitly stops the run and opens a new plan because M27.9 was mis-scoped.
- The M20 pack must remain an unsupported truth pack.
  - It may be repaired.
  - It may not be quietly converted into a mixed or supported pack.
- `xtask/src/lib.rs` may only be updated after:
  - semantic truth is stable
  - M20 unsupported truth is repaired
  - CLI truth locks are updated and passing
- Derived analysis outputs are downstream proof only:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- If the refreshed analysis delta is not exactly:
  - promoted `15 -> 18`
  - unsupported `13 -> 10`
  - recommendation `ranked -> no_strong_candidate`
  then stop immediately, capture evidence, and do not hand-edit `xtask` locks to force the expected answer.
- Workers do not write:
  - `.runs/**`
  - `.semantic-family-artifacts/**`
  - generated output
  - passports

## Source Vs Derived Surfaces

Authored source surfaces for M27.9:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/*`
- `spec-cli/tests/cli.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`

Derived or run-state surfaces for M27.9:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `examples/**/src/generated/**`
- `examples/**/*.spec.passport.json`
- `.runs/m27_9/**`

Execution rule:

- workers edit authored source only, within their owned path set
- derived surfaces are refreshed only by the parent in integration / proof phases
- no worker refreshes analysis JSON, generated code, or passports

## Parent-Owned Run-State Protocol

Canonical run root: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9`

Parent-owned mutable run-state:

- `baseline.json`
  - branch, HEAD SHA, timestamp, and authoritative file list re-read at run start
- `dirty-state.json`
  - exact `git status --short` snapshot before any worktree is created
- `tasks.json`
  - authoritative task queue and dependency graph
- `session-log.md`
  - append-only run diary with worker launch/return, command results, and stop/land disposition
- `diagnostics/*`
  - blocked-closeout evidence bundle only

Minimum parent-owned task IDs:

- `task/m27_9-00-baseline`
- `task/m27_9-a1-freeze-run-contract`
- `task/m27_9-a2-semantic-boundary`
- `task/m27_9-b1-m20-cli-truth`
- `task/m27_9-b2-docs-refresh`
- `task/m27_9-c1-integrate-pre-xtask`
- `task/m27_9-c2-analysis-lock-refresh`
- `task/m27_9-c3-final-proof`
- `task/m27_9-c4-land-or-stop`

Run-state rules:

- parent writes all `.runs/m27_9/**` files
- workers may read `PLAN.md` and `ORCH_PLAN.md`
- workers may not write logs, queue state, or diagnostics
- if the run blocks, the parent records the exact stop reason before any retry

## Task Graph

```text
task/m27_9-00-baseline
  -> task/m27_9-a1-freeze-run-contract
      -> task/m27_9-a2-semantic-boundary
          -> task/m27_9-b1-m20-cli-truth
          -> task/m27_9-b2-docs-refresh
task/m27_9-b1-m20-cli-truth
task/m27_9-b2-docs-refresh
  -> task/m27_9-c1-integrate-pre-xtask
      -> task/m27_9-c2-analysis-lock-refresh
          -> task/m27_9-c3-final-proof
              -> task/m27_9-c4-land-or-stop
```

Execution intent:

1. parent captures the live repo baseline and freezes the run contract
2. parent lands the semantic-boundary change first
3. once semantic truth is proven, worker lanes B and C run in parallel
4. parent merges both lanes into integration
5. parent refreshes `xtask` locks only after semantic + CLI truth are both stable
6. parent runs the full proof loop and either lands or writes a blocked closeout

## Workstream Plan

### Critical Path

`WS-0 baseline/freeze -> WS-A parent semantic boundary -> WS-B M20+CLI + WS-C docs (parallel) -> WS-D parent integration -> WS-E parent xtask refresh -> WS-F final proof / land-or-stop`

This is intentionally not a many-lane plan. The milestone is dominated by one
semantic change and one downstream re-projection chain.

### Parallelism Boundary

Only two worker lanes are justified.

- Lane B is real because M20 repair and CLI truth locks are coupled and can be isolated after semantic behavior is fixed.
- Lane C is real because the README change is downstream of semantic truth but independent of CLI and `xtask` mechanics.
- No `xtask` worker lane is allowed, because `xtask` expectations depend on the final merged semantic + CLI truth.
- No separate fixture-only lane is allowed, because the M20 fixture and CLI assertions should move together.

### WS-0 Parent Baseline And Freeze

Parent only.

Task IDs:

- `task/m27_9-00-baseline`
- `task/m27_9-a1-freeze-run-contract`

Owned paths:

- `.runs/m27_9/baseline.json`
- `.runs/m27_9/dirty-state.json`
- `.runs/m27_9/tasks.json`
- `.runs/m27_9/session-log.md`
- `.runs/task-m27_9-00-baseline/**`
- `.runs/task-m27_9-a1-freeze-run-contract/**`

Required parent actions:

1. Re-read:
   - `PLAN.md`
   - `ORCH_PLAN.md`
2. Record:
   - `git rev-parse HEAD`
   - `git branch --show-current`
   - `git status --short`
3. Create worktrees from live `feat/corpus-expansion` HEAD:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9
git worktree add -b ws/m27_9-m20-cli /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/m20-cli HEAD
git worktree add -b ws/m27_9-docs /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/docs HEAD
git worktree add -b ws/m27_9-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int HEAD
```

4. Seed `tasks.json` with owned-path contracts and dependencies.
5. Write a freeze record in `session-log.md` that says:
   - semantic truth is parent-owned and must go first
   - worker lanes start only after semantic gate passes
   - `xtask` is downstream only

WS-0 acceptance:

- baseline and dirty-tree snapshots exist
- worktrees exist for `m20-cli`, `docs`, and `int`
- no authored source file changed during WS-0
- worker prompts are not sent before baseline capture completes

### WS-A Semantic Boundary

Parent only. Keep the semantic core on the critical path.

Task ID: `task/m27_9-a2-semantic-boundary`

Owned file:

- `spec-core/src/semantic_review.rs`

Required outcome:

- cross-library `shared::money/round` helper refs route the same way as local `money/round`
- control-flow arithmetic near-misses remain unsupported
- route precedence remains:
  `chain3 -> wrapper -> monotone_down -> monotone_up`

Required commands:

```bash
cargo test -p spec-core -- --color never
```

Semantic gate:

- do not launch workers until `spec-core` is green enough to prove the boundary
- if semantic behavior is still ambiguous, stop before parallel work begins

WS-A acceptance:

- only `spec-core/src/semantic_review.rs` changed in this phase
- regression coverage exists for:
  - cross-library monotone-down
  - cross-library monotone-up
  - unsupported control-flow near-miss
  - precedence non-shadowing
- parent records semantic-gate success or failure in `session-log.md`

### WS-B M20 Unsupported Pack And CLI Truth

Worker 1 on `ws/m27_9-m20-cli` in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/m20-cli`

Task ID: `task/m27_9-b1-m20-cli-truth`

Owned files:

- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/*`
- `spec-cli/tests/cli.rs`

Required commands / policy:

- read the semantic-boundary result before editing
- keep M20 as an unsupported truth pack
- repair the current supported arithmetic-shape slot honestly
- update CLI truth locks to reflect:
  - cross-library arithmetic is supported
  - repaired M20 near-miss is unsupported for the right reason
- worker may run:

```bash
cargo test -p spec-cli --test cli -- --color never
```

Forbidden:

- no edits to `spec-core/src/semantic_review.rs`
- no edits to `xtask/src/lib.rs`
- no edits to `semantic-families/README.md`
- no refresh of `.semantic-family-artifacts/**`

WS-B acceptance:

- the M20 pack remains an unsupported pack on its face and in behavior
- fixture naming and unit ids are truthful
- CLI assertions cover:
  - supported cross-library arithmetic semantic-review surfaces
  - repaired M20 unsupported truth in `status`, `export`, and passport reads
- owned paths and only owned paths changed

### WS-C Maintainer Docs Refresh

Worker 2 on `ws/m27_9-docs` in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/docs`

Task ID: `task/m27_9-b2-docs-refresh`

Owned file:

- `semantic-families/README.md`

Required outcome:

- docs state plainly that promoted arithmetic leaf families already cover zero-or-one helper deps
- docs explain that packet-local `money/round` models helper-aware shape truth
- docs now align cross-library helper-aware examples with the promoted boundary

Forbidden:

- no milestone-theory essay
- no edits outside `semantic-families/README.md`
- no claims about new family ids or policy changes

WS-C acceptance:

- README diff is narrow and contract-focused
- wording matches the semantic truth that WS-A established

### WS-D Parent Integration Before xtask Refresh

Parent only on `ws/m27_9-int`

Task ID: `task/m27_9-c1-integrate-pre-xtask`

Parent merges:

- WS-A semantic change from the control workspace
- WS-B worker branch
- WS-C worker branch

Integration rules:

- do not resolve semantic/CLI contract conflicts creatively
- if worker output conflicts with `PLAN.md`, apply `PLAN.md`
- if the M20 lane and docs lane disagree about the boundary, treat the semantic core as source of truth and fix docs, not runtime

Required commands:

```bash
cargo fmt --all
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
```

Pre-xtask gate:

- semantic and CLI truth must both be green before `xtask` lock refresh begins
- if this gate fails, stop and write a blocked diagnostic bundle

WS-D acceptance:

- merged repo contains semantic, M20/CLI, and README truth together
- no `xtask` expectation edits have happened yet
- parent records exact pre-xtask gate result

### WS-E Parent Analysis Lock Refresh

Parent only.

Task ID: `task/m27_9-c2-analysis-lock-refresh`

Owned file:

- `xtask/src/lib.rs`

Execution order is locked:

1. run analysis commands from merged repo truth
2. inspect generated output
3. update `xtask/src/lib.rs` to match that truth
4. do not modify any policy code to force the expected result

Required commands:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

Expected analysis truth:

- coverage moves from `28 / 15 / 0 / 13` to `28 / 18 / 0 / 10`
- recommendation moves from `ranked` to `no_strong_candidate`
- arithmetic ready candidate disappears
- `unsupported_function_surface-e40675da6fa0` remains held

Stop rule:

- if output truth differs from the expected delta, do not update locks blindly
- capture the real output and stop

WS-E acceptance:

- `xtask/src/lib.rs` reflects the real merged-truth output
- no policy files outside `xtask/src/lib.rs` changed
- parent records the actual coverage/recommendation delta

### WS-F Final Proof Loop

Parent only.

Task ID: `task/m27_9-c3-final-proof`

Run the full proof loop from `PLAN.md`:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo test -p xtask -- --color never

cargo xtask family coverage --format json
cargo xtask family recommend --format json

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Green-path rules:

- all three test suites pass
- both analysis commands succeed
- both artifacts validate
- locked delta remains exactly the expected delta after refresh

Red-path rules:

- stop at the first unexplained mismatch
- do not patch `xtask` again to chase the failure
- write blocked-closeout diagnostics

### WS-G Land Or Stop

Parent only.

Task ID: `task/m27_9-c4-land-or-stop`

Green path:

- record success in `session-log.md`
- record final acceptance checklist in `.runs/m27_9/`
- leave derived analysis artifacts refreshed from the merged repo state

Blocked path:

- write `.runs/m27_9/diagnostics/blocked-summary.md`
- write `.runs/m27_9/diagnostics/coverage.actual.json`
- write `.runs/m27_9/diagnostics/recommendation.actual.json`
- write `.runs/m27_9/diagnostics/semantic-review-notes.md`
- record which gate failed:
  - semantic gate
  - pre-xtask integration gate
  - analysis delta gate
  - final proof loop

Blocked-closeout rule:

- no silent retries after blocked closeout
- a new run must start from a fresh orchestration decision, not from ad hoc patching

## Worker Prompt Rules

- Each worker prompt contains only:
  - owned file set
  - relevant `PLAN.md` excerpt
  - required commands
  - forbidden surfaces
- Each worker returns only:
  - changed files
  - commands run with exit codes
  - blockers or unresolved assumptions
- The parent reviews narrow diffs and summaries only.
- Close each worker immediately after merge.

## Tests And Acceptance

- Semantic boundary
  - `spec-core/src/semantic_review.rs` proves cross-library helper parity with local helper semantics.
  - control-flow arithmetic near-misses stay unsupported.
  - route order stays `chain3 -> wrapper -> monotone_down -> monotone_up`.
- M20 unsupported pack
  - pack remains unsupported in both naming and behavior.
  - no supported arithmetic unit remains parked inside the unsupported pack.
- CLI truth
  - `status`, `export`, and passport-facing semantic-review surfaces show supported cross-library arithmetic truth.
  - repaired M20 truth remains unsupported on read-side surfaces.
- xtask truth
  - `xtask/src/lib.rs` locks the actual merged-truth analysis output only after runtime and CLI truth are stable.
  - no recommendation-policy logic changes are required or allowed here.
- Docs
  - `semantic-families/README.md` matches runtime truth literally.
- Operator flow
  - parent-owned run-state exists and reflects the live baseline.
  - worker lanes touch only their owned authored files.
  - blocked-closeout captures real evidence instead of opinion.

## Completion Criteria

The run is complete only when all of the following are true:

1. semantic truth is fixed first
2. M20 is repaired as an unsupported pack
3. CLI truth locks pass
4. `xtask` locks are refreshed from merged truth
5. coverage is `28 / 18 / 0 / 10`
6. recommendation is `no_strong_candidate`
7. arithmetic ready candidate is gone
8. `unsupported_function_surface-e40675da6fa0` remains held
9. README is aligned
10. the final proof loop passes cleanly

## Assumptions

- The existing branch `feat/corpus-expansion` remains the live execution baseline.
- `cargo xtask family validate-artifact <path>` already exists and remains the runtime validator for the generated analysis artifacts.
- `spec-core/src/semantic_review.rs` contains both runtime code and enough local tests to make WS-A self-contained.
- The only worthwhile worker parallelism is:
  - M20 fixture + CLI truth
  - README refresh
- Generated analysis JSON remains a derived proof surface, not a checked-in authored contract by itself.
