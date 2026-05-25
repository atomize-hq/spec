# Category Truth Orchestration Plan

Status: **authoritative execution runbook for the current `PLAN.md` wedge**  
Plan authority: **`/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md`**  
Historical context only: **prior `ORCH_PLAN.md` content is superseded in full**  
Primary workspace: **`/home/azureuser/__Active_Code/atomize-hq/spec`**  
Parent baseline branch: **`feat/i8-final-proof-run`**  
Parent baseline head: **`4c41fb36845b30d5527554b6a365f15f6fa58bc5`**  
Last rewritten: **2026-05-25**

## Summary

- This runbook executes the current category-truth wedge exactly as defined by
  `PLAN.md`. It does not re-scope the wedge and it does not widen supported
  surface area.
- The parent agent keeps the producer contract and the final CLI integration
  local. Those are the two drift-prone points.
- The only safe parallel window is after the contract spine lands:
  benchmark-core adoption and export projection adoption may run in parallel.
- `spec-cli/src/commands.rs` is a known merge-conflict hotspot because
  `PLAN.md` requires one shared helper there. No worker lane may touch it.
- Final snapshot/readability refresh happens only after the benchmark, export,
  and CLI projections agree from the same parent baseline.

## Outcome Target

This wedge is complete only when the branch can say all of the following
honestly with fresh local verification:

- there is one producer-owned category registry in `spec-core`
- category qualification is computed by one shared function
- benchmark, status, export, and snapshot surfaces all consume that same
  qualification result
- `BENCH-ECOM` remains passing
- `BENCH-SERVICE` full projection flips to `accounting_status = invalid`,
  `benchmark_status = invalid`, and `gate_status = open`
- `spec status --format json` and `spec export` both expose additive
  `category_qualification`
- export keeps qualification read-side only and does not persist it into
  `.spec.passport.json`

## Hard Guards

- Preserve the parent baseline exactly as:
  - branch: `feat/i8-final-proof-run`
  - head: `4c41fb36845b30d5527554b6a365f15f6fa58bc5`
- Treat `PLAN.md` as the authority. Do not change wedge scope in the
  orchestration flow.
- Do not touch files outside the lane-owned write set.
- Do not revert unrelated working-tree edits.
- Do not change `benchmarks/labels.json`.
- Do not add a checked-in external registry file.
- Do not persist `category_qualification` into `.spec.passport.json`.
- Do not let any worker lane edit `spec-cli/src/commands.rs`.
- Do not let any lane infer support from benchmark labels, health, or semantic
  review presence alone.
- Do not merge a lane that passes its local tests but fails an out-of-scope
  verification command.
- Stop and escalate if the parent baseline head is no longer the branch head
  when execution starts. The runbook assumes that exact commit as the frozen
  parent base.

## Parent Ownership

The parent agent owns all of the following for the full run:

- the canonical checkout at
  `/home/azureuser/__Active_Code/atomize-hq/spec`
- all state files under `.runs/category-truth/`
- the contract spine:
  - `spec-core/src/category_truth.rs`
  - `spec-core/src/lib.rs`
  - `spec-core/src/semantic_review.rs`
- the `commands.rs` shared-helper consolidation and all final CLI integration:
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
- final snapshot/readability refresh:
  - `benchmarks/snapshots/*.snapshot.json`
  - `benchmarks/reviews/*.readability.review.json`
- merge acceptance, conflict resolution, final verification, and closeout

Workers own only their explicit lane write sets and worker-local artifacts.
Workers do not own parent state, merge decisions, or final interpretation.

## Subagent Execution Policy

- All worker lanes use fresh `GPT-5.4` subagents with
  `reasoning_effort=high`.
- Maximum worker concurrency is `2`.
- The benchmark lane and export lane are the only lanes allowed to run in
  parallel.
- The parent agent is the only integrator.
- The parent agent is the only writer of canonical run-state files under
  `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/`.
- Worker-produced notes, sentinels, and verification captures are advisory
  until the parent reviews them and updates canonical state.
- Worker completion is not acceptance. Acceptance happens only when the parent:
  - reviews the lane output
  - updates `tasks.json`
  - updates the canonical per-task sentinel
  - records accepted verification artifacts in canonical run-state paths

## Worktree Layout

Parent checkout:

- `/home/azureuser/__Active_Code/atomize-hq/spec`

Worker worktree root:

- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth`

Worker worktrees and branches:

- benchmark lane:
  - branch: `ws/ct-benchmark`
  - worktree:
    `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/benchmark`
- export lane:
  - branch: `ws/ct-export`
  - worktree:
    `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/export`
- conditional blocker lane:
  - branch: `ws/ct-blocker`
  - worktree:
    `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/blocker`

No dedicated worker worktree is created for `commands.rs`. That lane stays in
the parent checkout on the parent baseline branch.

Exact worktree creation commands after Gate 1:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/benchmark -b ws/ct-benchmark feat/i8-final-proof-run
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/export -b ws/ct-export feat/i8-final-proof-run
```

Conditional blocker worktree command:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/blocker -b ws/ct-blocker feat/i8-final-proof-run
```

## Canonical State And Queue Files

Run root:

- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth`

Canonical parent-owned files:

- `tasks.json`
- `session-log.md`
- `preflight.json`
- `merge-order.md`
- `blockers.md`
- `sentinels/<task-id>.json`
- `verification/contract-spine/*`
- `verification/benchmark/*`
- `verification/export/*`
- `verification/cli/*`
- `verification/final/*`

Worker-local advisory files:

- `.runs/category-truth-worker/summary.md`
- `.runs/category-truth-worker/verification/*`
- `.runs/category-truth-worker/sentinel.json`

Task state vocabulary:

- `pending`
- `running`
- `worker_complete`
- `accepted`
- `rejected`
- `blocked`
- `done`

The parent is the only writer of canonical state. `.runs/category-truth/` is
the single source of truth for run progress, lane status, acceptance, block
state, merge order, and accepted verification evidence.

### Per-task Sentinel Contract

Canonical sentinel path:

- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/sentinels/<task-id>.json`

Required fields for every canonical sentinel:

- `task_id`
- `owner`
- `branch`
- `worktree`
- `status`
- `started_at`
- `updated_at`
- `owned_write_set`
- `required_commands`
- `required_artifacts`
- `worker_summary_path`
- `worker_mirror_paths`
- `canonical_artifact_paths`
- `acceptance_notes`
- `rejection_notes`
- `blocker_notes`
- `accepted_commit`

Required status rules:

- `pending`: task exists but work has not started
- `running`: owner is actively executing the task
- `worker_complete`: worker reports completion; parent review still pending
- `accepted`: parent accepted the lane and canonicalized its state
- `rejected`: parent rejected the lane; rerun required
- `blocked`: a hard guard or direct blocker stopped the lane
- `done`: parent-only terminal state for completed parent tasks

Worker-local mirror sentinel path convention:

- `<worktree>/.runs/category-truth-worker/sentinel.json`

Worker mirrors may include the same fields, but they remain non-canonical until
the parent copies the accepted facts into the canonical sentinel.

### Canonical vs Worker-local Verification Outputs

Canonical verification outputs live only under:

- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/verification/contract-spine/`
- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/verification/benchmark/`
- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/verification/export/`
- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/verification/cli/`
- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/verification/final/`

Worker-local mirrors live only under:

- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/benchmark/.runs/category-truth-worker/verification/`
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/export/.runs/category-truth-worker/verification/`
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth/blocker/.runs/category-truth-worker/verification/`

Rules:

- worker captures remain mirrors until parent acceptance
- parent acceptance copies or re-runs the accepted verification outputs into
  canonical verification paths
- rejected or blocked worker outputs stay worker-local and are referenced from
  the canonical sentinel only as diagnostics
- `tasks.json` and canonical sentinels always outrank worker-local mirrors

## Command Capture Rule

Every archived verification command writes both raw output and a sibling
`.exitcode` file. Use a `bash -lc` wrapper with `set -o pipefail` so captured
stdout does not mask failure.

Locked pattern:

```bash
bash -lc '
set -o pipefail
<command> | tee <output-path>
cmd_status=$?
printf "%s\n" "$cmd_status" > <exitcode-path>
exit "$cmd_status"
'
```

If a command is expected to fail meaningfully, assert the expected exit code
explicitly in the wrapper and record that fact in the lane summary.

## Workstream Matrix

| Workstream | Task id | Owner | Branch / worktree | Owned modules and artifacts | Start gate |
| --- | --- | --- | --- | --- | --- |
| Preflight freeze | `task/ct-p0-preflight` | parent | parent checkout | `.runs/category-truth/**` | run start |
| Contract spine | `task/ct-p1-contract-spine` | parent | parent checkout | `spec-core/src/category_truth.rs`, `spec-core/src/lib.rs`, `spec-core/src/semantic_review.rs`, substrate tests | Gate 0 |
| Benchmark adoption | `task/ct-b1-benchmark` | worker | `ws/ct-benchmark` | `spec-core/src/benchmark.rs`, `spec-cli/tests/rust_v1_service.rs`, `spec-cli/tests/rust_v1_closure.rs`, `spec-cli/tests/fixtures/benchmarks/*.json` | Gate 1 |
| Export adoption | `task/ct-c1-export` | worker | `ws/ct-export` | `spec-core/src/export.rs`, `spec-core/src/passport.rs`, export-focused tests | Gate 1 |
| CLI integration | `task/ct-p2-cli-integration` | parent | parent checkout | `spec-cli/src/commands.rs`, `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Gate 2 |
| Snapshot/readability refresh | `task/ct-p3-snapshot-refresh` | parent | parent checkout | `benchmarks/snapshots/*.snapshot.json`, `benchmarks/reviews/*.readability.review.json`, final expectation fixes caused by CLI output shape | Gate 3 |
| Conditional blocker lane | `task/ct-x1-blocker` | parent or worker | `ws/ct-blocker` only if needed | only the direct failing surface plus proof reruns | conditional |
| Final closeout | `task/ct-p4-closeout` | parent | parent checkout | `.runs/category-truth/**` | Gate 4 |

## Lane Boundaries

### Contract spine stays local

The parent keeps category substrate and semantic-review descriptor work in one
local stream because together they define the producer-owned truth contract.
This stream includes:

- `spec-core/src/category_truth.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/semantic_review.rs`
- unit tests proving:
  - supported ecommerce descriptors qualify
  - service seam siblings do not qualify as supported
  - unsupported rows qualify only as unsupported
  - missing `descriptor_id` is explicit failure

Do not split category substrate and semantic-review descriptor work into
separate workers. The type boundary is too shared and the merge win is too
small.

### Benchmark lane

The benchmark lane owns benchmark truth adoption only:

- `spec-core/src/benchmark.rs`
- benchmark-facing service and closure tests
- benchmark JSON fixtures under `spec-cli/tests/fixtures/benchmarks/`

This lane must not touch:

- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`

### Export lane

The export lane owns export projection adoption only:

- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- export/passport regression coverage

This lane must not touch:

- `spec-cli/src/commands.rs`
- benchmark fixtures
- snapshots or readability artifacts

### CLI integration lane

The parent-owned CLI lane owns the shared helper consolidation required by
`PLAN.md`. This lane must:

- add one shared `commands.rs` helper for projected unit truth
- wire `status`, benchmark read-side, and snapshot paths to that helper
- add `category_qualification` to status JSON
- keep benchmark snapshot output aligned with live benchmark projection

No other lane may add benchmark-truth extraction logic in `spec-cli`.

### Snapshot/readability lane

The parent-owned refresh lane runs only after CLI integration is stable. It
owns:

- snapshot refresh for `BENCH-ECOM` and `BENCH-SERVICE`
- readability review refresh only as additive artifact maintenance
- no semantic reinterpretation of readability freshness

## Gate Model

- Gate 0: preflight freeze
  - confirm branch and exact head
  - record current dirty files without reverting them
  - create run state and worktree plan
  - on pass, parent updates:
    - `preflight.json`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p0-preflight.json`
  - on fail, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p0-preflight.json`
  - stop immediately on fail
- Gate 1: contract spine complete
  - category substrate and semantic-review descriptor work merged locally
  - substrate verification is green
  - worker worktrees are created from this updated parent head only after this
  - on pass, parent updates:
    - `tasks.json`
    - `session-log.md`
    - `merge-order.md`
    - `verification/contract-spine/*`
    - `sentinels/task-ct-p1-contract-spine.json`
  - on fail, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p1-contract-spine.json`
  - do not spawn workers if Gate 1 fails
- Gate 2: benchmark and export lanes accepted
  - both worker lanes return in-scope diffs only
  - both lane verification sets are green
  - worker completion alone is insufficient; each lane must be parent-reviewed
    and explicitly marked `accepted`
  - on pass, parent updates:
    - `tasks.json`
    - `session-log.md`
    - `merge-order.md`
    - `verification/benchmark/*`
    - `verification/export/*`
    - `sentinels/task-ct-b1-benchmark.json`
    - `sentinels/task-ct-c1-export.json`
  - on fail, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - rejected lane sentinel(s)
  - if one lane is rejected, stop the run, document the rejection, and rerun
    from a clean worker lane before proceeding
- Gate 3: CLI integration complete
  - `commands.rs` helper is in place
  - status, benchmark, export, and snapshot read-side surfaces agree
  - on pass, parent updates:
    - `tasks.json`
    - `session-log.md`
    - `verification/cli/*`
    - `sentinels/task-ct-p2-cli-integration.json`
  - on fail, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p2-cli-integration.json`
  - conditional blocker lane may be activated only after this failure is
    written canonically
- Gate 4: snapshot/readability refresh and full sweep complete
  - frozen artifacts match live output
  - no remaining consumer drift
  - on pass, parent updates:
    - `tasks.json`
    - `session-log.md`
    - `verification/final/*`
    - `sentinels/task-ct-p3-snapshot-refresh.json`
  - on fail, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p3-snapshot-refresh.json`
  - blocker lane may run only after those files are updated
- Gate 5: closeout
  - canonical state updated to `done` or `blocked`
  - on pass, parent updates:
    - `tasks.json`
    - `session-log.md`
    - `merge-order.md`
    - `sentinels/task-ct-p4-closeout.json`
  - on blocked stop, parent updates:
    - `blockers.md`
    - `tasks.json`
    - `session-log.md`
    - `sentinels/task-ct-p4-closeout.json`

The flow is human-free by default. Pause only if a gate fails or a hard guard
is tripped.

### Blocked Path

When a task or gate blocks:

- parent writes the blocker first to:
  - `blockers.md`
  - `tasks.json`
  - the task sentinel
  - `session-log.md`
- parent records:
  - the failing command
  - exit code
  - affected files
  - whether the blocker is rerunnable or terminal
- if the blocker is rerunnable within scope:
  - rerun only the affected task commands
  - then rerun all downstream parent verification gates
- if the blocker would require scope expansion or forbidden-file ownership:
  - mark the run `blocked`
  - do not proceed to downstream gates
  - stop after closeout state is written

## Execution Order

1. Initialize `.runs/category-truth/` in the parent checkout.
2. Record branch, head, and dirty files in `preflight.json`,
   `session-log.md`, `tasks.json`, and the preflight sentinel.
3. Execute the contract spine locally on `feat/i8-final-proof-run`.
4. Run contract-spine verification and archive accepted outputs under
   `verification/contract-spine/`.
5. Update parent task state for `task/ct-p1-contract-spine` to `done`.
6. Create worker worktrees with the exact `git worktree add ... -b ws/...`
   commands in this document.
7. Spawn exactly two fresh `GPT-5.4` worker lanes:
   - benchmark
   - export
8. Mark both worker tasks `running` in `tasks.json` and canonical sentinels
   before the workers start editing.
9. Wait for worker completion via worker summaries and worker-local mirrors.
10. When a worker reports complete, mark only `worker_complete` in canonical
    state. Do not mark `accepted` yet.
11. Review the export lane first:
    - inspect owned-file diff
    - inspect worker command exits
    - inspect worker-local verification mirrors
    - reject if any forbidden surface changed
12. If export lane is accepted:
    - cherry-pick its commit into the parent branch
    - copy or rerun accepted verification outputs into
      `verification/export/`
    - update `tasks.json`, `merge-order.md`, `session-log.md`, and the export
      sentinel to `accepted`
    - close the export worker
13. If export lane is rejected:
    - write rejection details canonically
    - close the worker
    - stop and rerun that lane before benchmark acceptance proceeds
14. Review the benchmark lane with the same acceptance process.
15. If benchmark lane is accepted:
    - cherry-pick its commit into the parent branch
    - copy or rerun accepted verification outputs into
      `verification/benchmark/`
    - update `tasks.json`, `merge-order.md`, `session-log.md`, and the
      benchmark sentinel to `accepted`
    - close the benchmark worker
16. If benchmark lane is rejected:
    - write rejection details canonically
    - close the worker
    - stop and rerun that lane before continuing
17. Run the parent-owned CLI integration lane in the parent checkout.
18. Archive accepted CLI verification outputs under `verification/cli/` and
    update canonical state.
19. Refresh snapshots and readability artifacts locally from the post-CLI
    truth.
20. Run the full verification sweep and archive accepted final outputs under
    `verification/final/`.
21. If a direct blocker remains, activate `task/ct-x1-blocker`, run only the
    narrow blocker repair, rerun the affected task verification, then rerun
    the full verification sweep.
22. Update closeout state, mark the run `done` or `blocked`, and stop.

This flow walks the current `PLAN.md` session to completion. The run is not
finished at code integration; it is finished only after canonical verification,
state closeout, and final wedge-truth confirmation are complete.

## Integration And Merge Rules

- The contract spine is not optional staging. It is the branch-local truth base.
- Each worker lane should produce one cohesive commit if possible.
- Parent integration uses non-interactive git only.
- Preferred integration method is `git cherry-pick -x <worker-commit>` into the
  parent branch after parent review.
- Merge order is fixed:
  - export first because it is the smaller, more isolated core surface
  - benchmark second because its invalid `BENCH-SERVICE` truth becomes the
    parent baseline for CLI and snapshot adoption
- Do not merge a worker branch directly if it touched a forbidden file. Reject
  and rerun the lane instead.
- If a worker lane conflicts with unrelated dirty parent files, stop and
  rebase the lane around the current parent tree without reverting those files.

## Conflict Flags

| Flag | Surface | Risk | Required handling |
| --- | --- | --- | --- |
| `CF-1` | `spec-cli/src/commands.rs` | highest merge-conflict hotspot; shared helper required | parent-only lane; no worker edits permitted |
| `CF-2` | `spec-core/src/semantic_review.rs` | producer contract drift if lane-split | keep inside parent contract spine |
| `CF-3` | `spec-cli/tests/fixtures/benchmarks/*.json` vs snapshots | fixture truth can drift from final CLI truth | benchmark lane owns benchmark fixtures; snapshots wait for parent refresh lane |
| `CF-4` | readability review artifacts | qualification failure can be misread as readability change | refresh artifact timestamps/content only as generated output; do not rewrite freshness semantics |
| `CF-5` | dirty non-owned files in workspace | accidental overwrite of teammate work | record in preflight; never reset or force-checkout them |

## Workstream Detail

### `task/ct-p0-preflight`

Required parent actions:

- record:
  - branch
  - head
  - `git status --short`
  - worker worktree plan
- initialize:
  - `.runs/category-truth/tasks.json`
  - `.runs/category-truth/session-log.md`
  - `.runs/category-truth/preflight.json`
  - `.runs/category-truth/sentinels/`
  - `.runs/category-truth/verification/`
  - `task/ct-p0-preflight` canonical sentinel

Required commands:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec branch --show-current
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec status --short
mkdir -p /home/azureuser/__Active_Code/atomize-hq/spec/.runs/category-truth/{sentinels,verification}
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-category-truth
```

Acceptance:

- branch is `feat/i8-final-proof-run`
- head is `4c41fb36845b30d5527554b6a365f15f6fa58bc5`
- unrelated dirty files are recorded, not reverted
- `tasks.json`, `session-log.md`, and the preflight sentinel all agree

### `task/ct-p1-contract-spine`

Owned files:

- `spec-core/src/category_truth.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/semantic_review.rs`

Required verification commands:

```bash
cargo test -p spec-core category_truth
cargo test -p spec-core semantic_review
```

Acceptance:

- registry rows and qualification types exist
- `SemanticReview` exposes producer-owned `descriptor_id`
- supported ecommerce descriptors qualify
- service seam descriptors remain visible but not supported-qualified
- parent writes accepted verification artifacts to
  `verification/contract-spine/`
- parent marks the task `done` in canonical state before workers are spawned

### `task/ct-b1-benchmark`

Owned files:

- `spec-core/src/benchmark.rs`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/rust_v1_closure.rs`
- `spec-cli/tests/fixtures/benchmarks/*.json`

Required worker verification commands:

```bash
cargo test -p spec-core benchmark
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
```

Acceptance:

- `BenchmarkCaseProjection` carries `category_qualification`
- supported positive credit depends on qualified supported truth
- `BENCH-SERVICE` full benchmark becomes invalid/open
- partial mismatch projects `partial_invalid`
- worker summary and worker-local mirrors are reviewed before acceptance
- acceptance is recorded only after cherry-pick plus canonical state update

### `task/ct-c1-export`

Owned files:

- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`

Required worker verification commands:

```bash
cargo test -p spec-core export
```

Acceptance:

- `ExportBundle` adds `projected_units`
- projected units carry `semantic_review` plus `category_qualification`
- `.spec.passport.json` persistence remains unchanged
- export schema version is `5`
- worker summary and worker-local mirrors are reviewed before acceptance
- acceptance is recorded only after cherry-pick plus canonical state update

### `task/ct-p2-cli-integration`

Owned files:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`

Required parent verification commands:

```bash
cargo test -p spec-cli cli
cargo test -p spec-cli m14_regressions
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/service/units
```

Acceptance:

- `STATUS_JSON_SCHEMA_VERSION` is `5`
- status rows include additive `category_qualification`
- benchmark/status/snapshot paths share the same derived projected-unit truth
- no extra consumer-local semantic-review extraction logic remains in
  `commands.rs`
- parent archives accepted verification outputs under `verification/cli/`

### `task/ct-p3-snapshot-refresh`

Owned files:

- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`

Required parent verification commands:

```bash
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Acceptance:

- snapshot output includes case `category_qualification`
- `BENCH-ECOM` remains passing
- `BENCH-SERVICE` full snapshot is invalid/open
- readability freshness remains additive and unchanged in meaning
- parent archives accepted verification outputs under `verification/final/`

### `task/ct-x1-blocker`

Activate only if one of the following remains after the normal flow:

- a worker lane can only pass by touching a forbidden surface
- `commands.rs` integration reveals inconsistent qualification semantics that
  cannot be fixed locally in the parent lane without crossing owned boundaries
- the full verification sweep fails on a direct wedge surface

Rules:

- create `ws/ct-blocker` only after documenting the blocker in
  `.runs/category-truth/blockers.md`
- keep the owned write set as narrow as the failing surface allows
- rerun the affected targeted commands and then rerun the full sweep
- stop if fixing the blocker would widen support, alter schema scope beyond
  `PLAN.md`, or require new orchestration lanes

## Context-Control Rules

- Parent context stays centered on:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - the current lane diff
  - the current lane verification outputs
  - `.runs/category-truth/tasks.json`
  - `.runs/category-truth/session-log.md`
- Worker prompts contain only:
  - owned write set
  - exact required commands
  - forbidden surfaces
  - acceptance criteria
  - artifact output paths
- Workers return only:
  - changed files
  - commands run with exit codes
  - blocker notes
  - assumptions
- Parent records worker completion as `worker_complete`, not `accepted`.
- Close worker lanes immediately after parent acceptance or rejection.

## Full Verification Sweep

Run this exact sweep from the parent checkout after snapshot refresh and again
after any blocker repair:

```bash
cargo test -p spec-core
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo test -p spec-cli m14_regressions
cargo test -p spec-cli cli

cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Expected final truth:

- ecommerce full benchmark remains passing
- service full benchmark is invalid/open
- status and export expose the same qualification result for seam-backed rows
- snapshot output matches live benchmark projection
- `.spec.passport.json` stays free of persisted `category_qualification`

## Tests And Acceptance

### Workstream Acceptance

- Preflight is accepted only when:
  - branch and head match the frozen baseline
  - dirty files are recorded
  - canonical run-state files are initialized
- Contract spine is accepted only when:
  - category substrate and semantic-review descriptor work are complete
  - contract-spine verification artifacts are canonicalized
  - worker lanes have not started yet
- Benchmark lane is accepted only when:
  - owned-file diff stays within lane boundaries
  - worker verification passes
  - `BENCH-SERVICE` benchmark truth flips to invalid/open
  - parent cherry-picks the lane and updates canonical state
- Export lane is accepted only when:
  - owned-file diff stays within lane boundaries
  - worker verification passes
  - export schema is `5`
  - parent cherry-picks the lane and updates canonical state
- CLI integration is accepted only when:
  - `commands.rs` remains parent-owned
  - shared-helper consolidation is in place
  - status and export commands reflect the same qualification contract
- Snapshot/readability refresh is accepted only when:
  - snapshot artifacts match live projection output
  - readability artifacts remain additive only
- Blocker lane is accepted only when:
  - it fixes a direct, in-scope blocker
  - it does not widen scope
  - downstream verification is rerun and passes

### Final Wedge Acceptance

- `BENCH-ECOM` remains `passing`
- `BENCH-SERVICE` full projection is:
  - `accounting_status = invalid`
  - `benchmark_status = invalid`
  - `gate_status = open`
- `spec status --format json` and `spec export` expose additive
  `category_qualification`
- seam-backed rows show status/export parity for qualification truth
- benchmark snapshots match live benchmark projection
- `.spec.passport.json` does not persist `category_qualification`
- no worker lane edited `spec-cli/src/commands.rs`
- the parent closeout records enough canonical evidence to replay the run logic

## Closeout Criteria

Mark the run `done` only when all of the following are true:

- every workstream is `accepted` or `done`
- no forbidden-file edits were needed
- no worker lane changed `spec-cli/src/commands.rs`
- the final verification sweep is green under the expected semantics
- `tasks.json`, `merge-order.md`, and `session-log.md` let a future maintainer
  reconstruct what landed and in what order

Otherwise mark the run `blocked` and record the narrowest honest blocker in
`.runs/category-truth/blockers.md`.

## Assumptions

- The current `PLAN.md` frozen decisions remain valid at execution time.
- The exact parent baseline head remains the correct launch point for this
  wedge.
- Benchmark and export adoption remain separable after the contract spine lands.
- `commands.rs` remains the only shared-helper hotspot significant enough to
  justify parent-only ownership.
