# M58 Orchestration Plan

Status: **authoritative execution runbook**  
Supersedes: **the stale M57 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M58: Bounded Nested Chain3 Closure TypeScript Execution Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Primary execution head at rewrite: **`6c7caf3`**  
Authority validated commit in `PLAN.md`: **`6c7caf3`**  
Base branch: **`main`**  
Authority date: **`2026-05-14`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**  
Rewrite intent: **replace the stale M57 closeout runbook with an execution-ready M58 runbook aligned to the current `PLAN.md` truth**  
Last rewritten: **`2026-05-14`**

## Summary

This runbook drives one bounded milestone to completion.

M58 is not generic TypeScript graph execution. It is one exact widen in the existing Bun-backed lane:

- a chain3 root may use a same-tree `function.wrapper.pipeline.chain3.v1` in direct dep slot 1
- nested chain3 recursion stays bounded to the same loaded tree
- slot 2 stays `function.arithmetic_leaf.monotone_up.v1`
- slot 3 stays `function.arithmetic_leaf.monotone_down_nonnegative.v1`

The parent agent remains the sole integrator on `feat/m40-plus`. The parent owns scope, run-state, merge order, and final proof. Worker lanes are allowed only where file ownership is disjoint and dependency shape is controlled.

The safe execution shape is fixed:

- Lane A: `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs`
- Lane B: `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*` and `spec-cli/tests/cli.rs`
- Lane C: `README.md` and `TODOS.md`

Lane A and Lane B may run in parallel only after the parent freezes exact fixture ids, lane scope, and rejection-wall expectations. Lane C runs last, after A and B are integrated, because docs must describe shipped truth rather than intended truth.

`PLAN.md` is the authority. `ORCH_PLAN.md` is derived orchestration only. If `PLAN.md` changes mid-run, the parent pauses, re-freezes scope, updates run-state, and relaunches only against the refreshed contract.

## Hard Guards

- `PLAN.md` is the only scope authority.
- M58 is exactly one bounded TypeScript-lane widen.
- Slot 1 widens from wrapper-only to wrapper-or-same-tree-chain3.
- Slot 2 remains monotone-up.
- Slot 3 remains monotone-down-nonnegative.
- Same-tree nested chain3 only. No cross-library recursive chain3.
- No generic DAG or multi-dependency TypeScript policy.
- No molecule TypeScript execution.
- No target-language `validate` or `export`.
- No schema churn.
- No new crate.
- No new runtime.
- No new command surface.
- No new fixture universe outside the existing `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*` pack.
- No generic “allowed TypeScript family graph” abstraction.
- All new recursive rejection paths must fail before Bun runs.
- The preserved rejection wall is part of done, not cleanup.
- The maintained green proof must include a nested same-tree chain3 path.
- The maintained red proof must preserve wrong-family, wrong-order, missing-`body.typescript`, and cross-library recursive rejection coverage.
- The parent is the only integrator onto `feat/m40-plus`.

Stop and re-scope immediately if any of these become true:

1. The core lane needs slot-2 or slot-3 widening to make the nested green path work.
2. The core lane needs cross-library recursive chain3 to make the nested green path work.
3. The implementation starts drifting toward generic graph execution or a new registry abstraction.
4. Any recursive red path starts failing at Bun/runtime instead of failing in pre-Bun validation.
5. The CLI proof requires new commands, new fixtures outside the maintained aligned pack, or target-language `validate/export`.
6. Docs would need to claim broader TypeScript graph support than the bounded same-tree slot-1 widen.
7. `PLAN.md` changes materially during execution and the parent has not re-frozen the contract.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m58
RUN_ROOT=$PRIMARY_ROOT/.runs/m58_bounded_nested_chain3_closure_typescript_execution
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, freeze, integration, final proof wall |
| `WS-A-CORE` | `$WT_ROOT/ws-a-core` | `codex/m58-core-nested-chain3` | Worker | validator slot-1 widen and backend nested recursion |
| `WS-B-FIXTURE-CLI` | `$WT_ROOT/ws-b-fixture-cli` | `codex/m58-fixture-cli-nested-chain3` | Worker | maintained nested fixture and CLI proof wall |
| `WS-C-DOCS` | `$WT_ROOT/ws-c-docs` | `codex/m58-doc-contract-sync` | Worker or parent | final README/TODOS wording sync only after A and B land |

### Worktree creation rules

- Do not create any worker worktree before `M58-01` contract freeze completes.
- Create `WS-A-CORE` and `WS-B-FIXTURE-CLI` together after `M58-01`.
- Create `WS-C-DOCS` only after `M58-21` completes or if the parent decides to do docs directly on `feat/m40-plus`.
- Do not split validator and backend into different worktrees. Both touch `spec-core/src/` and must stay one sequential lane.
- Do not move docs earlier. Docs describe shipped truth and therefore come after core and CLI proof are integrated.
- Record the current dirty tree at kickoff. At rewrite time, `PLAN.md` is already modified in the primary tree and must be preserved, not cleaned.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-a-core" -b codex/m58-core-nested-chain3 feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-b-fixture-cli" -b codex/m58-fixture-cli-nested-chain3 feat/m40-plus

# only after lanes A and B are integrated, unless the parent keeps docs local
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-c-docs" -b codex/m58-doc-contract-sync feat/m40-plus
```

## Durable Orchestration State

All durable session state lives under:

```bash
$RUN_ROOT
```

This directory is orchestration state only. It is not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, head, dirty-tree snapshot, authority commit | Parent |
| `contract-freeze.json` | frozen M58 contract, lane scopes, stop rules, gate commands | Parent |
| `worktrees.json` | exact worktree paths, branches, and lane states | Parent |
| `file-ownership.json` | exact owned file map per lane | Parent |
| `tasks.json` | canonical task ledger and dependencies | Parent |
| `session-log.md` | chronological launch, handoff, merge, rerun, block, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `final-proof-manifest.json` | exact final commands, exit codes, and artifact paths | Parent |
| `validation/kickoff/` | branch, head, status, authority snapshots | Parent |
| `validation/baseline/` | baseline proof captures and initial lane observations | Parent |
| `validation/lane-a/` | targeted core-lane proof captures | Parent |
| `validation/lane-b/` | targeted fixture/CLI proof captures | Parent |
| `validation/lane-c/` | docs sync captures if needed | Parent |
| `validation/final/` | final serial proof wall captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |
| `tasks/<TASK_ID>/` | per-task sentinels | Parent creates, lane updates |

### Required `baseline.json` contents

- `milestone`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_validated_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `observed_primary_surfaces`
- `baseline_commands`
- `known_scope_boundaries`
- `run_started_at`

### Required `contract-freeze.json` contents

- `milestone`
- `authority_plan_path`
- `authority_plan_head_commit`
- `primary_branch`
- `frozen_scope_claim`
- `locked_decisions`
- `not_in_scope`
- `same_tree_rule`
- `exact_lane_ownership`
- `fixture_ids`
- `worker_command_walls`
- `acceptance_commands`
- `integration_order`
- `merge_conflict_policy`
- `worker_return_contract`
- `exact_stop_rules`

### Frozen fixture ids

These ids must be written into `contract-freeze.json` before worker launch so Lane A and Lane B never drift:

- `pricing/base_nested_chain3_aligned`
- `pricing/checkout_nested_chain3_aligned`

## Task State And Sentinels

`tasks.json` is the single source of truth for task state.

Allowed states:

- `queued`
- `ready`
- `running`
- `blocked`
- `submitted`
- `integrated`
- `closed`
- `skipped`

Only the parent may set `integrated`, `closed`, or `skipped`. Workers may move only between `running`, `blocked`, and `submitted`.

Each task gets a dedicated sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Required files:

- `status.json`
- `owner.txt`
- `branch.txt`
- `write_scope.txt`
- `commands.txt`
- `changed_files.txt`
- `acceptance.md`
- `blocker.md`
- `handoff.md`

### Required `status.json` fields

- `task_id`
- `lane`
- `state`
- `owner`
- `branch`
- `write_scope`
- `started_at`
- `updated_at`
- `depends_on`
- `commands_run`
- `changed_files`
- `acceptance_status`
- `blocker_code`
- `blocker_summary`
- `next_action`

A task is not done when a worker says it is done. A task is done only after the parent integrates the lane and reruns the relevant gates.

## Blocked-State Protocol

Blocked state is explicit and durable.

### What a worker writes when blocked

If a worker cannot complete within scope, it must write all of the following before stopping:

- `tasks/<TASK_ID>/blocker.md`
- `tasks/<TASK_ID>/status.json`
- `tasks/<TASK_ID>/commands.txt`
- `tasks/<TASK_ID>/changed_files.txt`
- `tasks/<TASK_ID>/handoff.md`

### Required `blocker.md` contents

- `Blocker code`
- `Observed command`
- `Observed failure`
- `Why this is blocked within current scope`
- `Whether existing partial edits are safe to keep`
- `Requested parent action`
- `Whether the lane recommends bounce-back, parent fix-forward, or re-scope`

### Required blocked `status.json` updates

When blocked, the worker sets at minimum:

- `state: "blocked"`
- `acceptance_status: "not_met"`
- `blocker_code`
- `blocker_summary`
- `next_action`
- `commands_run`
- `changed_files`

### Parent behavior on blocked lanes

The parent records the worker’s block in:

- `tasks.json`
- `tasks/<TASK_ID>/status.json`
- `tasks/<TASK_ID>/blocker.md`
- `session-log.md`

Then the parent chooses one of three paths only:

1. `fix_forward_local`
   - allowed only if the issue is small, stays inside the parent integration tree, and does not expand milestone scope
   - typical case: trivial assertion drift, path typo, or handoff formatting issue
2. `bounce_back_to_lane`
   - required if the fix stays inside the original lane’s write scope and needs substantive code changes
   - typical case: validator wording mismatch, fixture/test mismatch, or incomplete recursive proof
3. `halt_and_rescope`
   - required if the blocker implies scope expansion, command-surface expansion, or a violated hard guard

### Fix-forward vs bounce-back rule

The parent may fix-forward locally only when all are true:

- the fix is within already-integrated files or the parent branch during integration
- the fix does not broaden the task’s original write scope
- the fix does not alter frozen milestone behavior
- the fix can be proven immediately with targeted reruns

Otherwise the parent must bounce back to the owning lane or halt.

## File Ownership And Exact Lane Scope

### Parent-owned throughout

- `PLAN.md`
- `ORCH_PLAN.md`
- all files under `$RUN_ROOT/`
- the primary integration branch `feat/m40-plus`
- all merge commits or squash steps
- all final proof execution
- all re-scope decisions

### `WS-A-CORE` owned files

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`

### `WS-B-FIXTURE-CLI` owned files

- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*`
- `spec-cli/tests/cli.rs`

### `WS-C-DOCS` owned files

- `README.md`
- `TODOS.md`

### Ownership guards

- No lane may edit another lane’s files.
- No worker lane may edit `PLAN.md`, `ORCH_PLAN.md`, or `$RUN_ROOT/**`.
- No worker may add new files outside its exact owned surface.
- If a lane discovers a required change outside its scope, it must stop and return a blocker rather than expanding scope implicitly.
- Parent-only integration remains mandatory even when lanes are conflict-free.

## Context-Control Rules For Subagents

- Give each worker only the authority summary, its write scope, the exact stop rules, its acceptance criteria, and the exact commands it is responsible for.
- Do not forward full raw transcripts between workers.
- Do not let Lane B invent fixture ids or error wording independently of the parent freeze.
- Keep validator and backend together in Lane A because both touch `spec-core/src/` and the backend behavior depends on validator admission.
- Allow Lane B to run in parallel only because its files are disjoint from Lane A after ids and scope are frozen.
- Keep docs out of parallel execution because `README.md` and `TODOS.md` must reflect the final landed truth, not speculative wording.
- Require workers to return only changed files, commands run with exit codes, blockers, and unresolved assumptions.
- Store proof output under `validation/*`; do not rely on chat summaries for gate evidence.
- If `PLAN.md` changes while workers are active, pause all workers, update `contract-freeze.json`, and relaunch only with refreshed scope.

## Worker Handoff Contract

Each worker return packet must include exactly:

- changed files
- commands run with exit codes
- blockers, if any
- unresolved assumptions, if any
- whether the task believes acceptance is met

Workers do not merge. Workers do not integrate. Workers do not commit unless the parent explicitly asks for a commit. Worker output is a handoff packet plus scoped file changes only.

The return must be written into:

- `handoffs/<TASK_ID>.md`
- `tasks/<TASK_ID>/handoff.md`
- `tasks/<TASK_ID>/commands.txt`
- `tasks/<TASK_ID>/changed_files.txt`
- `tasks/<TASK_ID>/acceptance.md`

## Parent-Only Integration Rules

- The parent integrates one lane at a time.
- Merge order is fixed: Lane A, then Lane B, then Lane C.
- Lane B may execute in parallel with Lane A, but it may not be integrated before Lane A.
- After integrating Lane A, the parent reruns targeted core proofs before touching Lane B.
- After integrating Lane B, the parent reruns targeted CLI and maintained-fixture proofs before touching Lane C.
- Docs never merge on top of unproven code.
- If Lane B’s assertions or fixture naming drift from landed Lane A behavior, the parent bounces Lane B for refresh instead of patching it ad hoc.
- The parent never asks workers to merge or reconcile each other’s worktrees.
- If an integration requires edits outside the submitted lane scope, the parent either fixes it directly on `feat/m40-plus` or reassigns scope explicitly and records that in `file-ownership.json`.

## Integration Mechanics

For every submitted lane, the parent follows the same mechanics in the same order:

1. review the submitted changed files against the lane’s write scope
2. review `commands.txt`, `acceptance.md`, `blocker.md`, and unresolved assumptions
3. merge or patch the lane’s changes onto `feat/m40-plus`
4. rerun the lane’s targeted gates immediately
5. if the targeted gates pass, mark the task `integrated`
6. if the targeted gates fail because of narrow drift inside lane scope, bounce the lane back
7. if the targeted gates fail because of a trivial parent-side integration issue, fix-forward locally and rerun
8. if the targeted gates fail because the milestone contract is breached, halt and re-scope

Integration failure is not silent. The parent must write the failure and chosen next action into:

- `tasks/<TASK_ID>/status.json`
- `tasks/<TASK_ID>/blocker.md`
- `session-log.md`

## Workstream Plan

| ID | Task | Owner | Write scope | Depends on | Parallel? | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- |
| `M58-00` | Kickoff + baseline capture | Parent | `$RUN_ROOT/**` | none | no | baseline snapshots and dirty-tree record stored |
| `M58-01` | Contract freeze + lane charter | Parent | `$RUN_ROOT/**` | `M58-00` | no | frozen scope, ids, ownership, stop rules, and command walls written |
| `M58-10` | Lane A core validator/backend implementation | `WS-A-CORE` | `spec-core/src/validator.rs`, `spec-core/src/typescript_backend.rs` | `M58-01` | yes | worker submits bounded slot-1 widen plus nested recursion proof |
| `M58-20` | Lane B maintained fixture + CLI proof | `WS-B-FIXTURE-CLI` | aligned fixture pack and `spec-cli/tests/cli.rs` | `M58-01` | yes | worker submits nested aligned fixture and CLI proof wall |
| `M58-11` | Parent integrate Lane A + core gate rerun | Parent | integration on `feat/m40-plus` only | `M58-10` | no | targeted spec-core proofs pass or lane is bounced/blocked |
| `M58-21` | Parent integrate Lane B + CLI gate rerun | Parent | integration on `feat/m40-plus` only | `M58-20`, `M58-11` | no | targeted CLI and maintained-fixture proofs pass or lane is bounced/blocked |
| `M58-30` | Lane C docs/backlog sync | `WS-C-DOCS` or Parent | `README.md`, `TODOS.md` | `M58-21` | no | public wording exactly matches shipped bounded behavior |
| `M58-31` | Final serial proof wall + closeout | Parent | `$RUN_ROOT/**` and minimal fix-forward only if needed | `M58-30` | no | full acceptance wall green and closeout artifacts written |

## Task Execution Details

### `M58-00` Kickoff + baseline capture

Required captures:

```bash
mkdir -p "$RUN_ROOT"/{validation/{kickoff,baseline,lane-a,lane-b,lane-c,final},handoffs,tasks}

git -C "$PRIMARY_ROOT" branch --show-current > "$RUN_ROOT/validation/kickoff/branch.txt"
git -C "$PRIMARY_ROOT" rev-parse HEAD > "$RUN_ROOT/validation/kickoff/head.txt"
git -C "$PRIMARY_ROOT" status --porcelain=v1 -uall > "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Required parent notes:

- record that `PLAN.md` is already dirty in the primary tree
- treat that dirty `PLAN.md` as authoritative session input
- do not clean, stash, or overwrite it by default

### `M58-01` Contract freeze + lane charter

The parent writes:

- `baseline.json`
- `contract-freeze.json`
- `worktrees.json`
- `file-ownership.json`
- `tasks.json`
- initial task sentinel files

The freeze must state all of this explicitly:

- same-tree recursive chain3 only
- slot 1 may be wrapper or same-tree chain3
- slot 2 and slot 3 remain frozen
- no molecule TypeScript execution
- no target-language `validate/export`
- frozen fixture ids
- Lane A owns only `spec-core/src/*` surfaces above
- Lane B owns only aligned fixture pack plus `spec-cli/tests/cli.rs`
- Lane C owns only `README.md` and `TODOS.md`
- merge order A then B then C
- exact command walls below

### `M58-10` Lane A core validator/backend implementation

Lane A is sequential internally.

Lane A responsibilities:

- widen `validate_typescript_chain3_dep_contract(...)` in `spec-core/src/validator.rs`
- keep slot-2 and slot-3 validation unchanged
- admit slot-1 same-tree chain3 only under the bounded M58 contract
- preserve explicit rejection for cross-library recursive chain3
- widen `validate_typescript_closure_member_spec_with_specs(...)` only through the same bounded contract
- extend `collect_typescript_closure_member(...)` in `spec-core/src/typescript_backend.rs` to recurse through validated nested chain3 members
- preserve reachability and dedupe behavior via the existing `included` set
- add or update unit tests in the touched files for nested admission, rejection, recursion, and exclusion boundaries

#### Lane A worker command wall

The expected worker command wall is:

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
```

If the final landed test selector names differ, the worker must record the exact replacement selectors in `handoff.md`, and the parent must update `contract-freeze.json` before integration.

Lane A may run additional narrow `cargo test -p spec-core ...` selectors during development, but the two commands above are the required handoff wall.

#### Lane A acceptance

Before the parent integrates Lane A, all of these must be true:

- `spec-core/src/validator.rs` admits slot-1 same-tree nested chain3 and still rejects cross-library recursive chain3
- `spec-core/src/validator.rs` keeps slot 2 and slot 3 frozen
- `spec-core/src/typescript_backend.rs` recurses through validated nested chain3 members without leaking unrelated loaded units
- the required Lane A command wall passes
- `acceptance.md` explicitly states whether any selector names were adjusted
- no lane-owned file outside `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs` changed

#### Lane A stop conditions

- needing a new abstraction or registry
- needing slot-2 or slot-3 widening
- needing cross-library recursion
- moving any recursive rejection from validation time to runtime

### `M58-20` Lane B maintained fixture + CLI proof

Lane B may run in parallel with Lane A after `M58-01`, because the files are disjoint and the ids are frozen.

Lane B responsibilities:

- add the maintained nested aligned proof shape in `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*`
- use exactly:
  - `pricing/base_nested_chain3_aligned`
  - `pricing/checkout_nested_chain3_aligned`
- reuse existing aligned wrapper and leaf units
- extend `spec-cli/tests/cli.rs` with:
  - nested green-path proof
  - wrong first-slot family rejection
  - wrong dep order rejection
  - missing nested `body.typescript` rejection
  - cross-library nested chain3 rejection
- keep all new recursive red paths failing before Bun runs

#### Lane B worker command wall

The expected worker command wall is:

```bash
cargo test -p spec-cli --test cli typescript_nested_chain3
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

Lane B may run additional narrow CLI selectors during development, but the two commands above are the required handoff wall.

#### Lane B acceptance

Before the parent integrates Lane B, all of these must be true:

- the aligned fixture pack contains the two frozen nested ids and reuses the existing aligned wrapper and leaf units
- `spec-cli/tests/cli.rs` covers the nested green path plus all required recursive red paths
- the required Lane B command wall passes
- recursive wrong-family, wrong-order, missing-`body.typescript`, and cross-library nested failures still fail before Bun
- no lane-owned file outside the aligned fixture pack and `spec-cli/tests/cli.rs` changed

#### Lane B guardrails

- do not edit `README.md` or `TODOS.md`
- do not expand to a new fixture pack
- do not add generic graph proof
- if final validator wording differs after Lane A lands, return for refresh rather than broadening assertions

### `M58-11` Parent integrate Lane A + core gate rerun

After Lane A submits, the parent integrates only Lane A and runs targeted core proofs.

Required gate commands:

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
```

Stop conditions here:

- any targeted failure that implies broader graph work
- any red path that no longer fails pre-Bun
- any change that leaks unrelated units into the tree or breaks dedupe expectations

### `M58-21` Parent integrate Lane B + CLI gate rerun

After Lane B submits and Lane A is already integrated, the parent integrates only Lane B and runs targeted CLI proofs.

Required gate commands:

```bash
cargo test -p spec-cli --test cli typescript_nested_chain3
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

Stop conditions here:

- maintained nested green path fails
- any recursive red path reaches Bun instead of failing earlier
- fixture ids or error expectations do not match landed Lane A behavior

### `M58-30` Lane C docs/backlog sync

Lane C starts only after A and B are integrated and proven.

Lane C responsibilities:

- update `README.md` so it no longer says nested chain3 closure is unsupported
- describe only the bounded M58 rule:
  - chain3 slot 1 may be wrapper or same-tree chain3
  - slot 2 and slot 3 remain fixed
  - recursive chain3 closure is same-tree only
  - generic multi-dependency execution remains unsupported
- update `TODOS.md` so generic multi-dependency TypeScript execution remains deferred and M58 is not overclaimed

#### Lane C worker verification wall

The expected worker verification wall is:

```bash
rg -n "chain3 root|same-tree|nested chain3|generic multi-dependency|molecule|target-language" README.md TODOS.md
cargo test -p spec-cli --test cli typescript_nested_chain3
```

The `rg` command is the wording verification wall. The CLI command is a contract-sanity check so docs do not race ahead of actual landed behavior.

#### Lane C acceptance

Before the parent integrates Lane C, all of these must be true:

- `README.md` states the bounded recursive rule and no longer says nested chain3 closure is unsupported
- `README.md` does not imply generic multi-dependency execution shipped
- `TODOS.md` keeps generic multi-dependency TypeScript execution deferred
- the verification wall passes or, for `rg`, clearly shows the exact intended wording matches
- no lane-owned file outside `README.md` and `TODOS.md` changed

#### Lane C stop conditions

- wording would imply generic graph execution
- docs would need to mention target-language `validate/export`
- docs would need to mention molecule TypeScript execution

### `M58-31` Final serial proof wall + closeout

The parent runs the full acceptance wall only after all required code and docs are integrated.

#### Final closeout command wall

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
cargo test -p spec-cli --test cli typescript_nested_chain3
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

Optional observability commands, if useful for the ledger:

```bash
cargo run -p spec-cli -- build semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- status semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

#### Final closeout acceptance

Before `M58-31` can be marked complete, all of these must be true:

- Lane A, Lane B, and Lane C are integrated or explicitly closed by the parent with equivalent landed changes
- the final closeout command wall passes
- validator recursive admission and rejection proofs are green
- generated TypeScript tree includes the nested chain3 closure and excludes unrelated units
- Bun-backed nested-chain3 execution passes
- recursive red paths still fail before Bun
- `README.md` and `TODOS.md` reflect only the bounded M58 rule
- `acceptance-ledger.md` is written
- `final-proof-manifest.json` is written

#### What gets written at closeout

`acceptance-ledger.md` must include:

- final branch and head
- accepted scope summary
- landed lane list
- required acceptance commands
- pass/fail result per command
- locations of stored validation outputs
- any narrow fix-forward notes the parent applied during integration

`final-proof-manifest.json` must include:

- milestone
- final head commit
- exact final commands
- exit code per command
- output artifact paths under `validation/final/`
- final status: `complete` or `blocked`

#### Successful vs blocked end state

Successful end state:

- all required final commands exit zero
- all acceptance bullets above are true
- `tasks.json` marks `M58-31` as `closed`
- `final-proof-manifest.json` marks `status: "complete"`

Blocked end state:

- any required final command fails
- any hard guard is breached
- any doc contract still overclaims or underclaims shipped truth
- `tasks/M58-31/blocker.md` explains the stop
- `tasks.json` marks `M58-31` as `blocked`
- `final-proof-manifest.json` marks `status: "blocked"`

## Validation And Proof Wall Sequencing

### Phase 1: baseline capture

Goal: record current branch/head, authority inputs, and dirty-tree state.

Required outcome:

- run-state created
- authority snapshotted
- `PLAN.md` dirty state recorded

### Phase 2: targeted core proof after Lane A

Goal: prove validator admission/rejection and backend recursion behavior before CLI integration.

Required commands:

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
```

Required stop condition:

- if these fail in a way that suggests broader graph work, pause and re-scope

### Phase 3: targeted CLI and maintained fixture proof after Lane B

Goal: prove the maintained nested green path and the preserved pre-Bun recursive rejection wall.

Required commands:

```bash
cargo test -p spec-cli --test cli typescript_nested_chain3
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

Required stop condition:

- if any recursive red path reaches Bun, pause and fix the rejection wall before proceeding

### Phase 4: final regression wall

Goal: prove the full lane-facing surfaces are green after docs sync.

Required commands:

```bash
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

Optional evidence commands if useful for the ledger:

```bash
cargo run -p spec-cli -- build semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- status semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

These optional commands are observability only. They do not replace the acceptance wall above.

## Acceptance And Tests

Acceptance is not complete until all three milestone gates are green:

1. validator recursive admission and rejection proofs are green
2. generated TypeScript tree includes the nested chain3 closure and excludes unrelated units
3. Bun-backed nested-chain3 execution passes while all recursive red paths still fail before Bun

The minimum command wall is:

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
cargo test -p spec-cli --test cli typescript_nested_chain3
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

No milestone closeout is allowed if any one of these remains true:

- same-tree nested chain3 is still rejected
- cross-library recursive chain3 is admitted
- slot-2 or slot-3 recursion slipped in
- missing nested `body.typescript` reaches Bun
- wrong nested dep order reaches Bun
- unrelated loaded units appear in the generated TypeScript tree
- docs still claim nested chain3 closure is unsupported
- docs imply generic multi-dependency TypeScript execution shipped

## Parallelization Strategy

### Why Lane A is sequential

Lane A owns both `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs`. The backend recursion is only valid after validator admission rules are correct. Splitting these files across workers would create unnecessary merge churn and false-positive proofs.

### Why Lane B may run in parallel

Lane B owns only the aligned fixture pack and `spec-cli/tests/cli.rs`. Those files are disjoint from Lane A. Parallel execution is safe only because the parent freezes exact ids and scope first.

### Why Lane C runs last

Docs are contract surfaces. They must describe the landed bounded rule, not a predicted one. Running docs in parallel risks overclaiming or documenting stale error boundaries.

### Effective concurrency policy

- one parent integrator always active
- up to two worker lanes active at once
- launch wave 1: Lane A and Lane B
- launch wave 2: Lane C only after A and B are integrated and proven

## Assumptions

- `PLAN.md` remains the authority unless the parent explicitly refreshes the contract.
- The repo continues to use `feat/m40-plus` as the execution branch and `main` as base.
- Bun is available in the execution environment for the maintained TypeScript proof.
- Existing flat chain3 TypeScript coverage remains intact and serves as the baseline shape being widened.
- The nested maintained fixture can be expressed by adding exactly two aligned units and reusing existing aligned wrapper and leaf units.
- Targeted test selectors named in `PLAN.md` will exist by the time the proof wall is run. If a lane chooses slightly different test names while implementing, the parent must update `contract-freeze.json` and `final-proof-manifest.json` before final closeout.
- `.runs/*` is acceptable as ephemeral orchestration state and not treated as product output.

## Completion Criteria

M58 is done only when all of the following are true:

- Lane A is integrated and its targeted core gates pass.
- Lane B is integrated and its targeted CLI plus maintained-fixture gates pass.
- Lane C is integrated or intentionally completed by the parent with equivalent doc updates.
- `README.md` and `TODOS.md` match the bounded shipped rule exactly.
- Final `cargo test -p spec-core` and `cargo test -p spec-cli --test cli` are green.
- `acceptance-ledger.md` and `final-proof-manifest.json` are written.
- No stop-rule breach remains unresolved.
