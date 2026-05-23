# I8 Orchestration Plan

Status: **authoritative execution runbook for the I8 final proof run**  
Milestone: **I8 Rust V1 final proof run**  
Plan authority: **[`PLAN.md`](./PLAN.md)**  
Contract-stack authority: **[`docs/rust_v1_contract_stack.md`](./docs/rust_v1_contract_stack.md)**  
Frozen upstream authority: **`.runs/i7/decision-freeze.json` and `.runs/i7/i8-handoff.json`**  
Primary workspace: **`/home/azureuser/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Working branch: **`feat/i8-final-proof-run`**  
Plan validated at commit: **`5d849d4`**  
Current execution head at draft time: **`8d627b1af1fc4acd6e0e8a065805045d3f9b195a`**  
Last rewritten: **2026-05-23**

## Summary

- Execute from the current checked-out branch `feat/i8-final-proof-run`, because
  that is the live I8 branch in this workspace and `5d849d4` is an ancestor of
  the current head `8d627b1af1fc4acd6e0e8a065805045d3f9b195a`.
- Treat I8 as a verification-and-ratification run only. It is not a feature
  milestone, a benchmark-mechanics milestone, or a post-V1 planning wedge.
- Keep the critical path local to the parent agent for:
  - preflight freeze
  - interpretation of both positive proof walls
  - workspace inventory confirmation
  - authority drift ratification
  - blocker activation or rejection
  - final closeout
- Parallelize only the two disjoint positive proof reruns:
  - Lane A: `BENCH-ECOM`
  - Lane B: `BENCH-SERVICE`
- All worker lanes use fresh `gpt-5.4` subagents with
  `reasoning_effort=high`.
- Cap concurrency at `2` worker subagents total.
- The parent agent remains the only integrator and the only writer of
  canonical `.runs/i8/**`.
- Keep repo-root inventory confirmation parent-owned and strictly after both
  positive walls are understood.
- Use dedicated worktrees under
  `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/{ecom,service,blocker}`
  with workstream branches:
  - `ws/i8-ecom-proof`
  - `ws/i8-service-proof`
  - `ws/i8-blocker` only if a real blocker repair is required
- Keep orchestration state in one canonical parent-owned source of truth:
  - run root: `.runs/i8/`
  - queue: `.runs/i8/tasks.json`
  - session log: `.runs/i8/session-log.md`
  - preflight freeze: `.runs/i8/preflight.json`
  - authority drift record: `.runs/i8/authority-drift.md`
  - final closeout: `.runs/i8/closeout.json`
  - parent-owned per-task sentinels: `.runs/i8/sentinels/<task-id>.json`
- Treat `.runs/i8/**` as run artifacts and closeout records, not as new product
  surfaces. Treat `benchmarks/labels.json`, committed benchmark snapshots, and
  readability reviews as existing truth inputs, not authored scope for I8
  unless a live contradiction forces a bounded repair.

## Frozen Outcome Target

I8 is complete only when the repo can still say this sentence honestly with
fresh evidence from the live branch:

> Rust V1 is the current narrow-core `spec` surface: synchronous supported
> function families plus plain data and sum seams, proven by `BENCH-ECOM` and
> `BENCH-SERVICE`, with `BENCH-CROSSLIB` preserved as companion negative proof.

Nothing in I8 may widen, soften, or reinterpret that sentence.

## Hard Guards

- Do not change the frozen five-command wall:
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - `cargo run -p spec-cli -- status . --format json`
- Do not add slice-specific proof commands.
- Do not make repo-root `export .` supported for this workspace shape.
- Do not reinterpret repo-root `status . --format json` as a green ship gate.
  It must remain `scope_authority: inventory_only`.
- Do not change the benchmark roster:
  - `BENCH-ECOM` stays an active positive wall rooted at
    `examples/ecommerce/units`
  - `BENCH-SERVICE` stays an active positive wall rooted at
    `examples/service/units`
  - `BENCH-CROSSLIB` stays an active companion negative wall rooted at
    `examples/crosslib-app/units`
- Do not widen Rust V1 support beyond the I7 freeze:
  - bounded generics remain deferred to `V1.1`
  - async flows, runtime adapters, and IO-owned boundaries remain deferred to
    `V1.1`
- Do not imply an `I9` or any new checked-in post-I8 milestone.
- Do not start the run if `5d849d4` is not an ancestor of the current execution
  head. If that ancestry check ever fails, halt and rewrite `PLAN.md` before
  attempting I8.
- If any proposed blocker repair would require:
  - changing benchmark schema or benchmark roles
  - changing the five-command wall
  - adding new proof writers or artifact classes
  - promoting a deferred `V1.1` surface into Rust V1
  stop immediately. That is no longer I8.

## Parent And Worker Responsibilities

### Parent owns

- all canonical files under `.runs/i8/`
- all canonical task and sentinel state under `.runs/i8/tasks.json`,
  `.runs/i8/session-log.md`, and `.runs/i8/sentinels/`
- the branch and authority ancestry check
- acceptance or rejection of positive proof lane outputs
- workspace inventory interpretation
- authority drift ratification
- any decision to activate blocker repair
- final closeout and milestone verdict

### Workers own

- narrow branch-local proof reruns or bounded blocker repair only
- short return summaries plus any worker-local mirror sentinel the prompt
  requires

### Workers must not own

- canonical `.runs/i8/**` in the parent checkout
- final interpretation of `inventory_only`
- repo-facing authority conclusions
- final closeout status
- any creative scope decision about Rust V1

## Subagent Execution Policy

- Every worker lane is executed by a fresh `gpt-5.4` subagent with
  `reasoning_effort=high`.
- Maximum concurrency is locked at `2` worker subagents.
- The parent remains the only integrator.
- The parent remains the only writer of canonical `.runs/i8/**`, including:
  - `.runs/i8/tasks.json`
  - `.runs/i8/session-log.md`
  - `.runs/i8/sentinels/**`
  - `.runs/i8/evidence/**`
  - `.runs/i8/preflight.json`
  - `.runs/i8/authority-drift.md`
  - `.runs/i8/closeout.json`
- Worker completion state is recognized only through:
  - the worker's required narrow summary
  - parent-owned sentinel updates
  - parent-owned task acceptance updates
- The parent must not treat raw subagent transcript text as canonical run
  state.

## Locked Command-Capture Pattern

Every command whose raw stdout is archived must preserve the real command exit
code. The runbook must not rely on plain `cargo ... | tee ...` without
`pipefail`.

Locked capture pattern for commands expected to exit `0`:

```bash
bash -lc '
set -o pipefail
<command> | tee <output-path>
cmd_status=$?
printf "%s\n" "$cmd_status" > <exitcode-path>
exit "$cmd_status"
'
```

Locked capture pattern for commands whose non-zero exit is expected and
meaningful:

```bash
bash -lc '
set -o pipefail
<command> | tee <output-path>
cmd_status=$?
printf "%s\n" "$cmd_status" > <exitcode-path>
test "$cmd_status" -eq <expected-exit>
'
```

Rules:

- Archived commands must write both the raw stdout file and a sibling
  `.exitcode` file.
- Worker-local captures live under `.runs/i8-worker/**`.
- Canonical accepted captures live under `.runs/i8/evidence/**` and
  `.runs/i8/sentinels/**` only after parent acceptance.
- A missing or mismatched `.exitcode` file is a failed task even if stdout was
  archived.

## Worktree And Branch Layout

The parent checkout remains the canonical run root.

- Parent workspace: `/home/azureuser/__Active_Code/atomize-hq/spec`
- Parent branch: `feat/i8-final-proof-run`
- Parent execution head at draft time:
  `8d627b1af1fc4acd6e0e8a065805045d3f9b195a`
- Worktree root: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8`

Worker worktrees after Gate 0 preflight freeze:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom -b ws/i8-ecom-proof feat/i8-final-proof-run
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service -b ws/i8-service-proof feat/i8-final-proof-run
```

Conditional blocker worktree only if Gate 3 activates repair:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/blocker -b ws/i8-blocker feat/i8-final-proof-run
```

Concurrency policy:

- maximum concurrent workers before any blocker repair: `2`
- only `Lane A` and `Lane B` run in parallel
- `Lane C`, `Lane D`, and final closeout stay parent-owned and sequential
- `Lane E` exists only if a real contradiction is found after interpretation

## Canonical Run State

The parent-owned source of truth for I8 is:

- `I8_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i8`
- `tasks.json` at `$I8_RUN_ROOT/tasks.json`
- session log at `$I8_RUN_ROOT/session-log.md`
- preflight freeze at `$I8_RUN_ROOT/preflight.json`
- authority drift record at `$I8_RUN_ROOT/authority-drift.md`
- final closeout at `$I8_RUN_ROOT/closeout.json`
- per-task sentinels under `$I8_RUN_ROOT/sentinels/<task-id>.json`

Canonical evidence targets:

- `$I8_RUN_ROOT/evidence/ecommerce.status.json`
- `$I8_RUN_ROOT/evidence/ecommerce.status.exitcode`
- `$I8_RUN_ROOT/evidence/ecommerce.export.json`
- `$I8_RUN_ROOT/evidence/ecommerce.export.exitcode`
- `$I8_RUN_ROOT/evidence/service.status.json`
- `$I8_RUN_ROOT/evidence/service.status.exitcode`
- `$I8_RUN_ROOT/evidence/service.export.json`
- `$I8_RUN_ROOT/evidence/service.export.exitcode`
- `$I8_RUN_ROOT/evidence/workspace.status.json`
- `$I8_RUN_ROOT/evidence/workspace.status.exitcode`

Recommended worker-local mirrors:

- `.../spec-i8/ecom/.runs/i8-worker/ecommerce.status.json`
- `.../spec-i8/ecom/.runs/i8-worker/ecommerce.status.exitcode`
- `.../spec-i8/ecom/.runs/i8-worker/ecommerce.export.json`
- `.../spec-i8/ecom/.runs/i8-worker/ecommerce.export.exitcode`
- `.../spec-i8/service/.runs/i8-worker/service.status.json`
- `.../spec-i8/service/.runs/i8-worker/service.status.exitcode`
- `.../spec-i8/service/.runs/i8-worker/service.export.json`
- `.../spec-i8/service/.runs/i8-worker/service.export.exitcode`
- `.../spec-i8/blocker/.runs/i8-worker/blocker-notes.md` only if needed
- `.../spec-i8/blocker/.runs/i8-worker/<task-id>.sentinel.json` only if needed

Sentinel convention:

- Parent-owned canonical sentinels live at
  `.runs/i8/sentinels/<task-id>.json`.
- Each sentinel records at least:
  - `task_id`
  - `owner`
  - `status`
  - `updated_at`
  - `evidence_paths`
  - `notes`
- Allowed parent-owned statuses are:
  - `pending`
  - `running`
  - `worker_complete`
  - `accepted`
  - `rejected`
  - `blocked`
- Workers may write branch-local mirror sentinels under `.runs/i8-worker/`,
  but those are advisory only until the parent updates the canonical sentinel.

Workers may write branch-local mirrors or notes, but only the parent writes the
canonical `.runs/i8/**` records in the primary checkout.

## Workstream Matrix

| Workstream | Task id | Owner | Start gate | Owned write set | Deliverable |
| --- | --- | --- | --- | --- | --- |
| WS-PARENT-0 | `task/i8-p0-preflight-freeze` | parent | run start | `.runs/i8/**` | frozen input packet plus evidence map |
| WS-A | `task/i8-a1-ecom-proof` | worker lane A | Gate 0 green | worktree-local `.runs/i8-worker/**` only | BENCH-ECOM raw status/export outputs plus narrow summary |
| WS-B | `task/i8-b1-service-proof` | worker lane B | Gate 0 green | worktree-local `.runs/i8-worker/**` only | BENCH-SERVICE raw status/export outputs plus narrow summary |
| WS-PARENT-1 | `task/i8-p1-positive-wall-acceptance` | parent | WS-A and WS-B returned | `.runs/i8/evidence/ecommerce.*`, `.runs/i8/evidence/service.*`, `.runs/i8/tasks.json` | accepted canonical positive-wall evidence |
| WS-PARENT-2 | `task/i8-p2-workspace-inventory-confirmation` | parent | WS-PARENT-1 green | `.runs/i8/evidence/workspace.status.json` | confirmed broad inventory semantics |
| WS-PARENT-3 | `task/i8-p3-authority-drift-ratification` | parent | WS-PARENT-2 green | `.runs/i8/authority-drift.md`, checked-in authority docs only if needed | repo-facing I8 story aligned to live truth |
| WS-E | `task/i8-e1-conditional-blocker-repair` | worker lane E or parent | only if WS-PARENT-1, WS-PARENT-2, or WS-PARENT-3 finds a real contradiction | only the exact failing surface plus rerun evidence | narrow direct repair or honest blocked stop |
| WS-PARENT-4 | `task/i8-p4-final-closeout` | parent | WS-PARENT-3 done and WS-E done if activated | `.runs/i8/closeout.json` and release-note surfaces if drift repair touched them | final I8 verdict and closeout packet |

## Gate Model

I8 does not use a human approval gate by default. It uses strict parent-owned
execution gates.

- Gate 0: preflight freeze
  - confirm branch, head, and `5d849d4 -> HEAD` ancestry
  - freeze the exact five-command wall and artifact map
- Gate 1: positive wall acceptance
  - accept or reject the `BENCH-ECOM` and `BENCH-SERVICE` reruns
- Gate 2: workspace inventory confirmation
  - confirm repo-root `status . --format json` still means `inventory_only`
- Gate 3: authority ratification
  - either all repo-facing surfaces already agree, or drift is limited to doc
    repair only
- Gate 4: blocker decision
  - activate narrow repair only if live truth contradicts the frozen claim
- Gate 5: final closeout
  - write the final run verdict as `done` or `blocked`

## Workstream Plan

### WS-PARENT-0 (`task/i8-p0-preflight-freeze`) — parent only, sequential

Owned write set:

- `.runs/i8/**`

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i8/evidence
mkdir -p /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i8/sentinels
git -C /home/azureuser/__Active_Code/atomize-hq/spec branch --show-current
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec merge-base --is-ancestor 5d849d4 HEAD
```

`preflight.json` must record:

- execution branch
- execution head
- plan-validated commit `5d849d4`
- proof that `5d849d4` is an ancestor of the execution head
- authority inputs:
  - `PLAN.md`
  - `docs/rust_v1_contract_stack.md`
  - `.runs/i7/decision-freeze.json`
  - `.runs/i7/i8-handoff.json`
  - `benchmarks/labels.json`
  - `README.md`
  - `DECISIONS.md`
  - `CHANGELOG.md`
  - `TODOS.md`
- the exact five-command wall
- the canonical evidence output paths
- the frozen plain-English Rust V1 claim
- the deferred `V1.1` surfaces

Acceptance:

- `.runs/i8/preflight.json` names exactly the same five commands frozen by
  `PLAN.md`
- the run records both `5d849d4` and the live execution head instead of
  silently collapsing them
- no proof interpretation happens before preflight is written
- the parent writes `task/i8-p0-preflight-freeze` sentinel state under
  `.runs/i8/sentinels/`

### WS-A (`task/i8-a1-ecom-proof`) — worker lane A

Branch and worktree:

- branch: `ws/i8-ecom-proof`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom`

Owned write set:

- worker-local mirrors only:
  - `.runs/i8-worker/ecommerce.status.json`
  - `.runs/i8-worker/ecommerce.status.exitcode`
  - `.runs/i8-worker/ecommerce.export.json`
  - `.runs/i8-worker/ecommerce.export.exitcode`
  - `.runs/i8-worker/task-i8-a1-ecom-proof.sentinel.json`

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom/.runs/i8-worker
bash -lc '
set -o pipefail
cargo run -p spec-cli -- status examples/ecommerce/units --format json | tee /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom/.runs/i8-worker/ecommerce.status.json
cmd_status=$?
printf "%s\n" "$cmd_status" > /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom/.runs/i8-worker/ecommerce.status.exitcode
exit "$cmd_status"
'
bash -lc '
set -o pipefail
cargo run -p spec-cli -- export examples/ecommerce/units | tee /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom/.runs/i8-worker/ecommerce.export.json
cmd_status=$?
printf "%s\n" "$cmd_status" > /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/ecom/.runs/i8-worker/ecommerce.export.exitcode
exit "$cmd_status"
'
```

Acceptance:

- `status` exits `0`
- `BENCH-ECOM` remains `benchmark_status: passing`
- `gate_status` remains `satisfied`
- `readability_review_status` remains `current`
- `export` remains `schema_version: 4`
- the export still projects `BENCH-ECOM` as a required positive benchmark with
  the required molecule roster:
  - `pricing/checkout_flow`
  - `pricing/discount_strategy_checkout_flow`
  - `pricing/discount_plus_tax`
- the worker returns only:
  - changed files
  - commands run and exit codes
  - blockers
  - assumptions
- the worker prompt must contain only:
  - owned file set
  - relevant `PLAN.md` excerpt
  - required commands
  - forbidden surfaces
  - output-path contract
- completion is surfaced through the worker summary plus the worker-local mirror
  sentinel, then parent acceptance updates the canonical task sentinel

### WS-B (`task/i8-b1-service-proof`) — worker lane B

Branch and worktree:

- branch: `ws/i8-service-proof`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service`

Owned write set:

- worker-local mirrors only:
  - `.runs/i8-worker/service.status.json`
  - `.runs/i8-worker/service.status.exitcode`
  - `.runs/i8-worker/service.export.json`
  - `.runs/i8-worker/service.export.exitcode`
  - `.runs/i8-worker/task-i8-b1-service-proof.sentinel.json`

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service/.runs/i8-worker
bash -lc '
set -o pipefail
cargo run -p spec-cli -- status examples/service/units --format json | tee /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service/.runs/i8-worker/service.status.json
cmd_status=$?
printf "%s\n" "$cmd_status" > /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service/.runs/i8-worker/service.status.exitcode
exit "$cmd_status"
'
bash -lc '
set -o pipefail
cargo run -p spec-cli -- export examples/service/units | tee /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service/.runs/i8-worker/service.export.json
cmd_status=$?
printf "%s\n" "$cmd_status" > /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/service/.runs/i8-worker/service.export.exitcode
exit "$cmd_status"
'
```

Acceptance:

- `status` exits `0`
- `BENCH-SERVICE` remains `benchmark_status: passing`
- `gate_status` remains `satisfied`
- `readability_review_status` remains `current`
- `export` remains `schema_version: 4`
- the export still projects `BENCH-SERVICE` as a required positive benchmark
  with the required molecule roster:
  - `billing/checkout_success_flow`
  - `billing/checkout_declined_discount_flow`
  - `billing/discount_strategy_quote_flow`
- the worker returns only:
  - changed files
  - commands run and exit codes
  - blockers
  - assumptions
- the worker prompt must contain only:
  - owned file set
  - relevant `PLAN.md` excerpt
  - required commands
  - forbidden surfaces
  - output-path contract
- completion is surfaced through the worker summary plus the worker-local mirror
  sentinel, then parent acceptance updates the canonical task sentinel

### WS-PARENT-1 (`task/i8-p1-positive-wall-acceptance`) — parent only

Owned write set:

- `.runs/i8/evidence/ecommerce.status.json`
- `.runs/i8/evidence/ecommerce.status.exitcode`
- `.runs/i8/evidence/ecommerce.export.json`
- `.runs/i8/evidence/ecommerce.export.exitcode`
- `.runs/i8/evidence/service.status.json`
- `.runs/i8/evidence/service.status.exitcode`
- `.runs/i8/evidence/service.export.json`
- `.runs/i8/evidence/service.export.exitcode`
- `.runs/i8/tasks.json`
- `.runs/i8/session-log.md`
- `.runs/i8/sentinels/task-i8-a1-ecom-proof.json`
- `.runs/i8/sentinels/task-i8-b1-service-proof.json`
- `.runs/i8/sentinels/task-i8-p1-positive-wall-acceptance.json`

Required parent actions:

- review the two worker summaries and narrow diffs only
- copy or restage accepted worker-local outputs into the canonical `.runs/i8/`
  evidence paths
- copy or restage the worker `.exitcode` files into the canonical evidence paths
- reject any lane output that lacks the expected benchmark, gate, readability,
  or schema-version truth
- update canonical sentinels and `tasks.json` instead of relying on transcript
  interpretation alone

Acceptance:

- both positive walls are accepted into canonical evidence paths
- both positive walls still teach the same narrow-core story frozen in I7
- no authority docs are edited yet
- canonical sentinels record acceptance or rejection for both worker lanes

### WS-PARENT-2 (`task/i8-p2-workspace-inventory-confirmation`) — parent only

Owned write set:

- `.runs/i8/evidence/workspace.status.json`
- `.runs/i8/evidence/workspace.status.exitcode`
- `.runs/i8/sentinels/task-i8-p2-workspace-inventory-confirmation.json`

Required commands:

```bash
bash -lc '
set -o pipefail
cargo run -p spec-cli -- status . --format json | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i8/evidence/workspace.status.json
cmd_status=$?
printf "%s\n" "$cmd_status" > /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i8/evidence/workspace.status.exitcode
test "$cmd_status" -eq 1
'
```

Expected interpretation:

- exit code `1` is allowed and expected
- `scope_authority` must remain `inventory_only`
- `BENCH-CROSSLIB` must remain visible as `companion_negative_proof`
- `BENCH-CROSSLIB` must keep zero positive supported credit
- `BENCH-ECOM` and `BENCH-SERVICE` must still project as `passing` inside the
  broader inventory surface
- intentionally non-green roots outside the positive wall must remain visible
  rather than being trimmed to force green

Acceptance:

- repo-root inventory stays truthful without being misread as a ship gate
- the broad surface preserves negative visibility and positive-wall visibility
  at the same time
- the archived `.exitcode` file records `1` and is treated as a success case
  for this task

### WS-PARENT-3 (`task/i8-p3-authority-drift-ratification`) — parent only

Owned write set:

- `.runs/i8/authority-drift.md`
- `.runs/i8/session-log.md`
- `.runs/i8/sentinels/task-i8-p3-authority-drift-ratification.json`
- checked-in authority docs only if real drift exists:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `docs/rust_v1_contract_stack.md`
  - `README.md`
  - `DECISIONS.md`
  - `CHANGELOG.md`
  - `TODOS.md`

Required comparison set:

- `.runs/i8/evidence/ecommerce.status.json`
- `.runs/i8/evidence/ecommerce.export.json`
- `.runs/i8/evidence/service.status.json`
- `.runs/i8/evidence/service.export.json`
- `.runs/i8/evidence/workspace.status.json`
- `.runs/i7/decision-freeze.json`
- `.runs/i7/i8-handoff.json`

Allowed outcomes:

- no drift:
  - write that all checked-in authority surfaces already agree with live truth
- doc drift only:
  - patch only the prose that drifted

Disallowed outcomes:

- keeping stale prose by reinterpreting live output
- adding new commands to rescue stale prose
- implying a post-I8 discovery milestone

Acceptance:

- every repo-facing authority surface teaches the same I8 story
- no checked-in doc weakens the I7 deferral line
- no checked-in doc implies that repo-root `status .` should be globally green

### WS-E (`task/i8-e1-conditional-blocker-repair`) — conditional only

Activation rule:

- launch only if a live command, export surface, benchmark projection, or
  authority surface contradicts the frozen I8 claim

Possible owner:

- parent for doc-only drift repair
- worker lane E in `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i8/blocker`
  only if code or benchmark-read-side repair is narrowly required

Owned write set:

- only the direct failing surface
- plus any rerun evidence needed to prove the repair
- plus `.runs/i8/sentinels/task-i8-e1-conditional-blocker-repair.json` if the
  lane is activated

Repair rules:

- fix the direct blocker only
- rerun the affected command immediately
- rerun the full five-command wall before closeout
- stop and escalate if the repair would widen support, add commands, or alter
  benchmark roles

Acceptance:

- the contradiction is removed without changing the frozen I8 scope
- the rerun evidence proves the fix directly
- the full five-command wall is green under the same interpretation rules as
  the original plan

### WS-PARENT-4 (`task/i8-p4-final-closeout`) — parent only

Owned write set:

- `.runs/i8/closeout.json`
- `.runs/i8/session-log.md`
- `.runs/i8/sentinels/task-i8-p4-final-closeout.json`
- release-note surfaces only if WS-PARENT-3 or WS-E changed them

`closeout.json` must record:

- milestone: `I8`
- final plain-English Rust V1 claim
- deferred `V1.1` surfaces
- branch and final commit
- plan-validated commit and execution head used for the run
- exact five-command verdicts
- references to all raw evidence files
- any doc files changed for drift repair
- whether blocker repair was activated
- final status: `done` or `blocked`

Acceptance:

- a future maintainer can reconstruct the exact I8 decision from `.runs/i8/`
  without relying on conversation context
- the final verdict cites raw evidence, not memory or paraphrase

## Context-Control Rules

- Parent keeps only the authoritative working set live:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `.runs/i8/tasks.json`
  - `.runs/i8/session-log.md`
  - `.runs/i8/sentinels/`
  - the latest positive-wall acceptance summary
  - the latest authority drift summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
  - output-path contract
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers
  - assumptions
- The parent reviews summaries plus narrow diffs only. It does not ingest full
  worker transcripts into the main context.
- Completion state flows through worker summaries plus worker-local mirror
  sentinels, then parent-owned canonical sentinel and `tasks.json` updates.
- Close each worker immediately after its evidence is accepted or rejected.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Command wall
  - the exact five commands from `PLAN.md` are the only proof wall
  - all five raw outputs are archived under `.runs/i8/evidence/`
  - all archived commands preserve real exit codes via the locked capture
    pattern and sibling `.exitcode` files
- Positive proof walls
  - `BENCH-ECOM` stays `passing` with `gate_status: satisfied` and
    `readability_review_status: current`
  - `BENCH-SERVICE` stays `passing` with `gate_status: satisfied` and
    `readability_review_status: current`
  - both positive exports remain `schema_version: 4`
- Broad inventory
  - repo-root `status . --format json` remains `inventory_only`
  - `BENCH-CROSSLIB` remains active companion negative proof with zero positive
    supported credit
  - broad inventory remains non-green by design without invalidating I8
- Authority surfaces
  - `PLAN.md`, `ORCH_PLAN.md`, `docs/rust_v1_contract_stack.md`, `README.md`,
    `DECISIONS.md`, `CHANGELOG.md`, and `TODOS.md` all teach the same bounded
    Rust V1 story
  - no checked-in doc implies `I9`
- Blocker discipline
  - any repair remains narrow and direct
  - no repair adds proof commands, benchmark roles, support rows, or new scope
- Workspace boundary
  - the milestone remains a final proof run, not a mechanics rewrite or feature
    expansion

## Failure Modes

| Failure mode | Consequence | Guard in this plan |
| --- | --- | --- |
| `PLAN.md` validated at `5d849d4` but execution runs from a disconnected head | the run no longer reflects the frozen authority basis | require `merge-base --is-ancestor 5d849d4 HEAD` during preflight |
| repo-root `status .` exit `1` is misread as failure | a truthful inventory surface blocks closeout for the wrong reason | require `scope_authority: inventory_only` interpretation instead of exit-code-only reasoning |
| `BENCH-CROSSLIB` starts earning positive credit | Rust V1 widens by read-side drift | require active visibility plus zero positive supported credit |
| positive `status` passes but positive `export` drifts | machine consumers and human readers see different truth | require both `status` and `export` for each positive wall |
| readability reviews go stale while status stays green | the plan overstates current readability-backed proof | require `readability_review_status: current` for both positive walls |
| docs still imply I7 is current or imply I9 exists | milestone ownership becomes ambiguous again | compare all repo-facing authority docs after live reruns |
| blocker repair expands into a mechanics rewrite | I8 silently becomes a new milestone | hard-stop any repair that alters the wall, benchmark roles, or deferred boundaries |

Critical gap test:

- if either positive benchmark loses `passing` or `satisfied`, I8 is not done
- if any authority surface still needs caveats absent from the frozen claim, I8
  is not done
- if the only way to "pass" is to reinterpret `inventory_only` as proof, I8 is
  not done

## Immediate Next Move

Execute I8 in this order:

1. create `.runs/i8/` and freeze the exact input set plus `5d849d4 -> HEAD`
   ancestry
2. launch `Lane A` and `Lane B` in parallel worktrees
3. accept or reject the two positive proof reruns
4. run repo-root inventory confirmation in the parent checkout
5. ratify checked-in authority surfaces against live truth
6. repair only a real direct blocker if one exists
7. write `.runs/i8/closeout.json` with the final verdict

Do not start by changing code. Start by proving whether the already-ratified
Rust V1 claim still holds on the live branch.
