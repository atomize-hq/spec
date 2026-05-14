# M59 Orchestration Plan

Status: **authoritative execution runbook**  
Supersedes: **the stale M58 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M59: Semantic-Review-Driven Local TypeScript Function Graph Execution Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Authority validated commit in `PLAN.md`: **`bd55d0f`**  
Base branch: **`main`**  
Authority date: **`2026-05-14`**  
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**  
Rewrite intent: **replace the stale M58 runbook with an execution-ready M59 runbook aligned to the current `PLAN.md` contract**  
Last rewritten: **`2026-05-14`**

## Summary

M59 adds one new TypeScript execution lane and keeps the existing portability lane intact.

The new lane is exactly:

- same-tree local only
- `kind:function` only
- semantic-review-driven
- graph-generic only over the shipped supported function families already admitted by the lane

The new lane is not:

- arbitrary per-node dep arity support
- new semantic-family promotion
- molecule TypeScript execution
- seam-kind TypeScript execution
- target-language `validate` or `export`
- generic recursive cross-library graph execution

The parent remains the sole integrator on `feat/m40-plus`. Parallelism is allowed only where merge risk is low and ownership is exact:

- Lane A: validator lane split plus backend local graph collector in `spec-core/src/`
- Lane B: local graph fixture authoring plus CLI proof in `spec-cli/tests/` and `spec-cli/tests/fixtures/`
- Lane C: `README.md` and `TODOS.md` sync only after A and B are integrated and proven

`PLAN.md` is the authority. `ORCH_PLAN.md` is derived orchestration only. If `PLAN.md` changes during execution, the parent pauses worker lanes, updates the freeze artifacts under `.runs/`, and relaunches only against the refreshed contract.

## Hard Guards

- `PLAN.md` is the only scope authority.
- M59 adds exactly one same-tree local TypeScript function graph lane.
- The local graph lane is semantic-review-driven and same-tree only.
- Graph-generic means closure traversal is dep-driven over already supported families, not arbitrary authored function topology.
- Existing direct cross-library helper, wrapper, and chain3 lanes remain preserved portability contracts and must not regress.
- No lane may widen to arbitrary 4+ dep authored function units.
- No lane may add or promote new semantic families.
- No lane may add molecule TypeScript execution.
- No lane may add seam-kind TypeScript execution.
- No lane may add target-language `validate` or `export`.
- No lane may change `spec-core/src/semantic_review.rs` for M59. If validator/backend work requires semantic-review logic changes, stop and re-scope.
- All new rejection paths must fail before Bun runs.
- The parent is the only integrator onto `feat/m40-plus`.
- No lane may revert, clean, stash, or overwrite unowned worktree changes. Kickoff must record the dirty tree and preserve it.

Stop and re-scope immediately if any of these become true:

1. The local lane requires semantic-family promotion to ship the intended green path.
2. The local lane requires arbitrary authored dep-topology support beyond the currently shipped supported families.
3. The implementation needs edits to `spec-core/src/semantic_review.rs`, CLI command surface changes, schema changes, or new runtime contracts.
4. Any local-graph rejection path reaches Bun or runtime instead of failing in pre-Bun validation.
5. The implementation starts widening to cross-library recursive graphs instead of preserving the existing explicit helper/wrapper/chain3 portability lanes.
6. Lane B cannot complete without touching `spec-core/src/`, or Lane A cannot complete without touching `spec-cli/tests/` or fixture files.
7. `PLAN.md` changes materially during execution and the parent has not refreshed the contract freeze.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m59
RUN_ROOT=$PRIMARY_ROOT/.runs/m59_semantic_review_driven_local_typescript_function_graph
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, freeze, integration, final proof wall |
| `WS-A-CORE` | `$WT_ROOT/ws-a-core` | `codex/m59-core-local-graph` | Worker | validator lane split plus backend local graph collector |
| `WS-B-FIXTURE-CLI` | `$WT_ROOT/ws-b-fixture-cli` | `codex/m59-fixture-cli-local-graph` | Worker | maintained local graph fixture tree plus CLI proof wall |
| `WS-C-DOCS` | `$WT_ROOT/ws-c-docs` | `codex/m59-doc-contract-sync` | Worker or parent | final README/TODOS wording sync only after A+B merge |

### Worktree creation rules

- Do not create worker worktrees before `M59-01` contract freeze completes.
- Create `WS-A-CORE` and `WS-B-FIXTURE-CLI` together after the freeze, because they can execute in parallel once ids and wording expectations are locked.
- Create `WS-C-DOCS` only after `M59-21` completes, unless the parent decides to handle docs directly on `feat/m40-plus`.
- Do not split validator and backend across worktrees. Both touch `spec-core/src/` and belong to one sequential lane.
- Do not let docs run in parallel with core or CLI proof work. Public wording must describe landed truth, not predicted truth.
- Record the dirty tree at kickoff and preserve it. At rewrite time, `PLAN.md` is already modified in the primary tree; the parent must capture the actual dirty state rather than assume a clean baseline.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-a-core" -b codex/m59-core-local-graph feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-b-fixture-cli" -b codex/m59-fixture-cli-local-graph feat/m40-plus

# only after A and B are integrated, unless the parent keeps docs local
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-c-docs" -b codex/m59-doc-contract-sync feat/m40-plus
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
| `contract-freeze.json` | frozen M59 contract, fixture ids, lane scopes, stop rules, command walls | Parent |
| `worktrees.json` | exact worktree paths, branches, current heads, and lane states | Parent |
| `file-ownership.json` | exact file ownership and no-touch surfaces per lane | Parent |
| `tasks.json` | canonical task ledger, dependencies, and current states | Parent |
| `session-log.md` | chronological launch, handoff, integration, rerun, block, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `final-proof-manifest.json` | exact final commands, exit codes, and output paths | Parent |
| `validation/kickoff/` | branch, head, dirty-tree, and authority snapshots | Parent |
| `validation/lane-a-core/` | targeted core-lane proof captures | Parent |
| `validation/lane-b-cli/` | targeted fixture and CLI proof captures | Parent |
| `validation/lane-c-docs/` | wording verification captures | Parent |
| `validation/final/` | final serial proof wall captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |
| `tasks/<TASK_ID>/` | per-task sentinels and status files | Parent creates, lane updates |

### Required `baseline.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `known_dirty_files_at_kickoff`
- `observed_primary_surfaces`
- `baseline_commands`
- `run_started_at`

### Required `contract-freeze.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_commit`
- `primary_branch`
- `frozen_scope_claim`
- `shipped_supported_function_families_scope`
- `preserved_portability_contracts`
- `fixture_root`
- `frozen_fixture_ids`
- `lane_ownership`
- `command_walls`
- `acceptance_commands`
- `integration_order`
- `merge_conflict_policy`
- `worker_return_contract`
- `stop_rules`

### Required `worktrees.json` fields

- `milestone`
- `updated_at`
- `primary_root`
- `worktree_root`
- `lanes[]`
  - `lane_id`
  - `path`
  - `branch`
  - `owner`
  - `state`
  - `head_commit`
  - `write_scope`
  - `task_ids`

### Required `tasks.json` fields

- `milestone`
- `updated_at`
- `tasks[]`
  - `task_id`
  - `lane`
  - `state`
  - `owner`
  - `depends_on`
  - `write_scope`
  - `command_wall`
  - `acceptance_summary`
  - `stop_rules`
  - `sentinel_dir`

### Frozen fixture root and ids

The dedicated M59 proof surface is frozen to:

```text
spec-cli/tests/fixtures/typescript_local_supported_graph/
```

The parent must freeze these exact unit ids in `contract-freeze.json` before worker launch:

- `money/round`
- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`
- `pricing/checkout_total`
- `pricing/display_total`

`pricing/display_total` exists to support unrelated-unit exclusion proof. If Lane B believes different ids are required, it must block and return the proposed replacement instead of changing the contract ad hoc.

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
- `depends_on`
- `started_at`
- `updated_at`
- `commands_run`
- `changed_files`
- `acceptance_status`
- `blocker_code`
- `blocker_summary`
- `next_action`

A task is not done when a worker says it is done. A task is done only after the parent integrates the lane and reruns the relevant proof wall.

## Blocked-State Protocol

Blocked state is explicit and durable.

### Standard blocker codes

- `PLAN_DRIFT`
- `SCOPE_EXPANSION_REQUIRED`
- `OWNERSHIP_CONFLICT`
- `PROOF_WALL_FAIL`
- `FIXTURE_CONTRACT_DRIFT`
- `MERGE_RISK`
- `ENVIRONMENT_MISSING`

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
- `Whether the lane recommends fix-forward, bounce-back, or re-scope`

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

The parent records the block in:

- `tasks.json`
- `tasks/<TASK_ID>/status.json`
- `tasks/<TASK_ID>/blocker.md`
- `session-log.md`

Then the parent chooses one of three paths only:

1. `fix_forward_local`
   - allowed only if the issue is small, stays inside already integrated or parent-owned surfaces, and does not expand milestone scope
2. `bounce_back_to_lane`
   - required if the fix stays inside the original lane's write scope and needs substantive code or test changes
3. `halt_and_rescope`
   - required if the blocker implies milestone drift, new surfaces, or a hard-guard breach

### Fix-forward vs bounce-back rule

The parent may fix-forward locally only when all are true:

- the fix is within already integrated files or parent-owned integration work
- the fix does not broaden the task's write scope
- the fix does not alter frozen M59 behavior
- the fix can be proven immediately with targeted reruns

Otherwise the parent must bounce the task back or halt.

## Exact Lane Ownership

### Parent-owned throughout

- `PLAN.md`
- `ORCH_PLAN.md`
- all files under `$RUN_ROOT/`
- the primary integration branch `feat/m40-plus`
- all merge, cherry-pick, or patch integration actions
- all final proof execution
- all re-scope decisions

### `WS-A-CORE` owned files

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`

### `WS-B-FIXTURE-CLI` owned files

- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/typescript_local_supported_graph/**`

### `WS-C-DOCS` owned files

- `README.md`
- `TODOS.md`

### Explicit no-touch surfaces

- `spec-core/src/semantic_review.rs`
- `spec-cli/src/**`
- `spec-core/src/types.rs`
- `AGENTS.md`
- any file under `.runs/`

### Ownership guards

- No lane may edit another lane's files.
- No worker may edit `PLAN.md`, `ORCH_PLAN.md`, or `$RUN_ROOT/**`.
- No worker may add files outside its exact owned surface.
- Lane B may not create a second fixture tree. M59 owns only `spec-cli/tests/fixtures/typescript_local_supported_graph/**`.
- Lane C may not edit code or tests.
- If a lane discovers a required change outside its scope, it must stop and return a blocker rather than expanding scope implicitly.
- Parent-only integration remains mandatory even when branches are conflict-free.

## Context-Control Rules For Subagents

- Give each worker only the authority summary, its write scope, frozen fixture ids, required command wall, stop rules, and acceptance criteria.
- Do not forward full transcripts between workers.
- Do not let Lane B invent fixture ids, local-vs-portability wording, or broadened rejection semantics independently of Lane A and the parent freeze.
- Keep validator and backend together in Lane A because they share `spec-core/src/` and are sequential by design.
- Allow Lane B to run in parallel only because its files are disjoint from Lane A after fixture ids and error-surface expectations are frozen.
- Keep docs out of parallel execution because `README.md` and `TODOS.md` must reflect landed truth only.
- Require workers to return only changed files, commands with exit codes, blockers, unresolved assumptions, and an acceptance claim.
- Store proof output under `validation/*`; do not rely on chat summaries as gate evidence.
- If `PLAN.md` changes while workers are active, pause them, refresh `contract-freeze.json`, and relaunch only with the updated contract.

## Worker Handoff Contract

Each worker return packet must include exactly:

- changed files
- commands run with exit codes
- blockers, if any
- unresolved assumptions, if any
- whether the task believes acceptance is met

Workers do not merge. Workers do not integrate. Workers do not commit unless the parent explicitly requests a commit. Worker output is scoped file changes plus a handoff packet only.

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
- After integrating Lane A, the parent reruns targeted core proofs before touching Lane B integration.
- After integrating Lane B, the parent reruns targeted CLI and direct local-graph proofs before touching Lane C.
- Docs never merge on top of unproven code.
- If Lane B assertions or fixture naming drift from landed Lane A behavior, the parent bounces Lane B for refresh instead of papering over the mismatch in the integration tree.
- The parent never asks workers to merge or reconcile each other's worktrees.
- If an integration requires edits outside the submitted lane scope, the parent either fixes it directly on `feat/m40-plus` or explicitly revises ownership and records that change in `file-ownership.json`.

## Workstream Plan

| ID | Task | Owner | Write scope | Depends on | Parallel? | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- |
| `M59-00` | Kickoff + baseline capture | Parent | `$RUN_ROOT/**` | none | no | baseline snapshots and dirty-tree record stored |
| `M59-01` | Contract freeze + lane charter | Parent | `$RUN_ROOT/**` | `M59-00` | no | frozen scope, fixture ids, ownership, stop rules, and command walls written |
| `M59-10` | Lane A validator split + local graph validation + backend collector | `WS-A-CORE` | `spec-core/src/validator.rs`, `spec-core/src/typescript_backend.rs` | `M59-01` | yes | local-graph lane is implemented without portability regression |
| `M59-20` | Lane B local fixture authoring + CLI proof wall | `WS-B-FIXTURE-CLI` | `spec-cli/tests/cli.rs`, `spec-cli/tests/fixtures/typescript_local_supported_graph/**` | `M59-01` | yes | maintained fixture tree and CLI proof wall are submitted |
| `M59-11` | Parent integrate Lane A + core proof rerun | Parent | integration on `feat/m40-plus` only | `M59-10` | no | targeted spec-core proofs pass or lane is bounced/blocked |
| `M59-21` | Parent integrate Lane B + CLI proof rerun | Parent | integration on `feat/m40-plus` only | `M59-20`, `M59-11` | no | targeted CLI and direct local-graph proofs pass or lane is bounced/blocked |
| `M59-30` | Lane C docs/backlog sync | `WS-C-DOCS` or Parent | `README.md`, `TODOS.md` | `M59-21` | no | public wording matches shipped M59 truth exactly |
| `M59-31` | Final serial proof wall + closeout | Parent | `$RUN_ROOT/**` and minimal fix-forward only if needed | `M59-30` | no | full M59 acceptance wall is green and closeout artifacts are written |

## Task Execution Details

### `M59-00` Kickoff + baseline capture

Required captures:

```bash
mkdir -p "$RUN_ROOT"/{validation/{kickoff,lane-a-core,lane-b-cli,lane-c-docs,final},handoffs,tasks}

git -C "$PRIMARY_ROOT" branch --show-current > "$RUN_ROOT/validation/kickoff/branch.txt"
git -C "$PRIMARY_ROOT" rev-parse HEAD > "$RUN_ROOT/validation/kickoff/head.txt"
git -C "$PRIMARY_ROOT" status --porcelain=v1 -uall > "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Required parent notes:

- record the actual dirty tree instead of assuming a clean checkout
- preserve all pre-existing edits; do not clean, stash, or revert them
- capture that `PLAN.md` is already dirty at rewrite time unless the live status says otherwise

### `M59-01` Contract freeze + lane charter

The parent writes:

- `baseline.json`
- `contract-freeze.json`
- `worktrees.json`
- `file-ownership.json`
- `tasks.json`
- initial task sentinel files

The freeze must state all of this explicitly:

- one new same-tree local graph lane only
- local lane is semantic-review-driven and same-tree only
- graph-generic over shipped supported families only
- direct cross-library helper, wrapper, and chain3 lanes remain preserved portability contracts
- no `semantic_review.rs` edits in M59
- frozen fixture root and exact ids listed above
- Lane A owns only `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs`
- Lane B owns only `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/typescript_local_supported_graph/**`
- Lane C owns only `README.md` and `TODOS.md`
- merge order A then B then C
- exact command walls below

### `M59-10` Lane A validator split + local graph validation + backend collector

Lane A is sequential internally and must remain one lane.

Lane A responsibilities:

- split local graph lane selection from the preserved cross-library portability lane in `spec-core/src/validator.rs`
- keep direct cross-library helper, wrapper, and chain3 root validation unchanged
- add one explicit local-root path in `validate_typescript_execution_target_spec_with_specs(...)`
- validate the reachable local closure graph-wide
- reject reachable `shared::...` deps in the local lane
- reject reachable unsupported semantic-review members in the local lane
- reject reachable missing `body.typescript` members in the local lane
- reuse existing cycle detection
- add a dep-driven local graph collector in `spec-core/src/typescript_backend.rs`
- preserve `included` as the dedupe mechanism
- preserve unrelated-unit exclusion and keep the old portability-lane collector behavior intact
- add or update unit tests in the touched files for lane selection, rejection wall, dedupe, and exclusion boundaries

#### Lane A worker command wall

The default worker handoff wall is:

```bash
cargo test -p spec-core typescript
```

If implementation lands with narrower stable selectors that cover all new M59 core cases, the parent may replace this in `contract-freeze.json` before worker launch. Otherwise the broad selector above stays authoritative.

#### Lane A acceptance

Before the parent integrates Lane A, all of these must be true:

- `spec-core/src/validator.rs` implements an explicit local graph lane and keeps the preserved portability lanes intact
- local graph validation remains same-tree only and semantic-review-driven
- reachable `shared::...`, unsupported semantic-review, missing `body.typescript`, and cycle failures remain pre-Bun
- `spec-core/src/typescript_backend.rs` walks the reachable local closure dep-first, dedupes shared subgraphs, and excludes unrelated loaded units
- the required Lane A command wall passes
- no lane-owned file outside `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs` changed

#### Lane A stop conditions

- needing `semantic_review.rs` edits
- needing arbitrary dep-topology widen beyond current supported families
- needing new CLI surface or schema changes
- regressing preserved cross-library helper, wrapper, or chain3 behavior
- moving any rejection from validation time to Bun/runtime

### `M59-20` Lane B local fixture authoring + CLI proof wall

Lane B may run in parallel with Lane A after `M59-01`, because the files are disjoint and the ids are frozen.

Lane B responsibilities:

- author the dedicated fixture tree under `spec-cli/tests/fixtures/typescript_local_supported_graph/`
- use exactly the frozen ids:
  - `money/round`
  - `pricing/apply_discount`
  - `pricing/apply_tax`
  - `pricing/calculate_total`
  - `pricing/checkout_total`
  - `pricing/display_total`
- ensure the fixture tree proves helper root, monotone-down root, monotone-up-with-helper root, wrapper root, chain3 root, shared-subgraph reuse, and unrelated-unit exclusion
- extend `spec-cli/tests/cli.rs` with Bun-backed green-path proof for the local graph lane
- extend `spec-cli/tests/cli.rs` with pre-Bun red-path proof for:
  - reachable `shared::...` in the local lane
  - reachable unsupported semantic-review member
  - reachable missing `body.typescript`
  - unsupported authored topology still rejected
  - local cycle rejection
- preserve green proof for direct cross-library helper, wrapper, and chain3 lanes

#### Lane B worker command wall

The worker handoff wall is:

```bash
cargo test -p spec-cli --test cli typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript
```

#### Lane B acceptance

Before the parent integrates Lane B, all of these must be true:

- the dedicated local graph fixture tree exists at the frozen path with the frozen ids
- `spec-cli/tests/cli.rs` proves local helper, monotone-down, monotone-up, wrapper, and chain3 green paths
- `spec-cli/tests/cli.rs` proves the local-graph rejection wall stays pre-Bun
- cross-library helper, wrapper, and chain3 regressions still pass unchanged
- the required Lane B command wall passes
- no lane-owned file outside `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/typescript_local_supported_graph/**` changed

#### Lane B guardrails

- do not edit `README.md` or `TODOS.md`
- do not edit `spec-core/src/**`
- do not create a second fixture pack
- if Lane A lands different final error wording, return for refresh rather than broadening assertions ad hoc

### `M59-11` Parent integrate Lane A + core proof rerun

After Lane A submits, the parent integrates only Lane A and runs targeted core proof immediately.

Required gate command:

```bash
cargo test -p spec-core typescript
```

Stop conditions here:

- any failure implying semantic-family promotion or broader dep-topology work
- any rejection wall failure that now reaches Bun/runtime
- any regression in preserved portability-lane behavior

### `M59-21` Parent integrate Lane B + CLI proof rerun

After Lane B submits and Lane A is already integrated, the parent integrates only Lane B and reruns targeted CLI proof.

Required gate commands:

```bash
cargo test -p spec-cli --test cli typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript
```

Stop conditions here:

- any local-graph red path reaches Bun
- frozen fixture ids drift
- cross-library helper, wrapper, or chain3 regressions fail
- CLI expectations no longer match landed Lane A semantics

### `M59-30` Lane C docs/backlog sync

Lane C starts only after A and B are integrated and proven.

Lane C responsibilities:

- update `README.md` so it distinguishes the new same-tree local graph lane from the preserved direct cross-library portability lanes
- state clearly that local roots may now be any shipped supported local function family with `body.typescript`
- state clearly that local traversal is semantic-review-driven and same-tree only
- state clearly that arbitrary node-shape parity, molecule execution, seam kinds, and target-language `validate/export` remain out
- update `TODOS.md` so the remaining TypeScript backlog names the real oceans left after M59:
  - arbitrary authored 4+ dep topology
  - new supported semantic families
  - generic recursive cross-library function graphs

#### Lane C verification wall

The verification wall is:

```bash
rg -n "same-tree|semantic-review|cross-library|validate --target-language|export --target-language|molecule TypeScript|generic" README.md TODOS.md
cargo test -p spec-cli --test cli typescript_cross_library
```

#### Lane C acceptance

Before the parent integrates Lane C, all of these must be true:

- `README.md` states the local graph lane and preserved portability lane distinctly
- `README.md` does not imply arbitrary per-node dep-topology parity shipped
- `TODOS.md` no longer uses a fuzzy generic-multi-dependency bucket for the now-shipped local graph widen
- the verification wall passes or, for `rg`, clearly shows the exact intended wording
- no lane-owned file outside `README.md` and `TODOS.md` changed

#### Lane C stop conditions

- wording would imply molecule TypeScript execution shipped
- wording would imply seam-kind or target-language `validate/export` shipped
- wording would erase the remaining backlog oceans or understate preserved portability constraints

### `M59-31` Final serial proof wall + closeout

The parent runs the full acceptance wall only after all required code and docs are integrated.

#### Final closeout command wall

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli --test cli typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript
cargo test -p spec-cli --test cli typescript_cross_library
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

Optional observability commands, if useful for the ledger:

```bash
cargo run -p spec-cli -- build spec-cli/tests/fixtures/typescript_local_supported_graph/units --target-language typescript
cargo run -p spec-cli -- status spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
```

#### Final closeout acceptance

Before `M59-31` can be marked complete, all of these must be true:

- Lane A, Lane B, and Lane C are integrated or explicitly closed by the parent with equivalent landed changes
- the final closeout command wall passes
- local supported-family roots pass
- local graph-wide rejection wall stays pre-Bun
- shared-subgraph dedupe and unrelated-unit exclusion are proven
- existing cross-library helper, wrapper, and chain3 lanes still pass unchanged
- `README.md` and `TODOS.md` reflect only the shipped M59 truth
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

- `milestone`
- `final_head_commit`
- `commands`
- `exit_codes`
- `output_artifact_paths`
- `status`

#### Successful vs blocked end state

Successful end state:

- all required final commands exit zero
- all acceptance bullets above are true
- `tasks.json` marks `M59-31` as `closed`
- `final-proof-manifest.json` marks `status: "complete"`

Blocked end state:

- any required final command fails
- any hard guard is breached
- any doc contract still overclaims or underclaims shipped truth
- `tasks/M59-31/blocker.md` explains the stop
- `tasks.json` marks `M59-31` as `blocked`
- `final-proof-manifest.json` marks `status: "blocked"`

## Validation And Proof Sequencing

### Phase 1: baseline capture

Goal: record current branch/head, authority inputs, and dirty-tree state.

Required outcome:

- run-state created
- authority snapshotted
- dirty tree recorded and preserved

### Phase 2: targeted core proof after Lane A

Goal: prove lane selection, local closure validation, dedupe, and unrelated-unit exclusion before CLI integration.

Required command:

```bash
cargo test -p spec-core typescript
```

Required stop condition:

- if this implies semantic-family promotion or arbitrary dep-topology widen, pause and re-scope

### Phase 3: targeted CLI and direct root proof after Lane B

Goal: prove the maintained local graph fixture green paths and the preserved pre-Bun rejection wall.

Required commands:

```bash
cargo test -p spec-cli --test cli typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript
```

Required stop condition:

- if any local-graph red path reaches Bun, pause and repair the rejection wall before proceeding

### Phase 4: docs sync verification

Goal: prove public wording matches the shipped lane boundary and preserved portability boundary.

Required commands:

```bash
rg -n "same-tree|semantic-review|cross-library|validate --target-language|export --target-language|molecule TypeScript|generic" README.md TODOS.md
cargo test -p spec-cli --test cli typescript_cross_library
```

Required stop condition:

- if docs imply broader support than the landed code or hide remaining backlog oceans, bounce Lane C

### Phase 5: final regression wall

Goal: prove the full lane-facing surfaces are green after docs sync.

Required commands:

```bash
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

The phase-5 commands do not replace the earlier phases. They close the full surface after the targeted gates are already green.

## Acceptance And Tests

Acceptance is not complete until all four M59 proof-wall guarantees are green:

1. `local supported-family roots pass`
   - prove with `cargo test -p spec-cli --test cli typescript`
   - prove directly with:
     - `cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript`
     - `cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript`
     - `cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript`
2. `local graph-wide rejection wall stays pre-Bun`
   - prove with `cargo test -p spec-cli --test cli typescript`
   - required rejection coverage includes reachable `shared::...`, unsupported semantic-review member, missing deep `body.typescript`, unsupported authored topology, and local cycle
3. `shared-subgraph dedupe and unrelated-unit exclusion are proven`
   - prove with `cargo test -p spec-core typescript`
4. `existing cross-library helper, wrapper, and chain3 lanes still pass unchanged`
   - prove with `cargo test -p spec-cli --test cli typescript_cross_library`

No milestone closeout is allowed if any one of these remains true:

- the local lane still rejects supported same-tree roots that should now pass
- any local-graph rejection reaches Bun
- unrelated loaded units leak into the generated TypeScript tree
- shared subgraphs are emitted more than once
- cross-library helper, wrapper, or chain3 lanes regress
- docs still imply M59 shipped arbitrary dep-topology parity or target-language `validate/export`

## Parallelization Strategy

### Why Lane A is sequential

Lane A owns both `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs`. The backend collector is only valid after the validator lane split and local closure checks are correct. Splitting these files across workers would create merge churn and false confidence.

### Why Lane B may run in parallel

Lane B owns only `spec-cli/tests/cli.rs` and the dedicated fixture tree. Those files are disjoint from Lane A. Parallel execution is safe only because the parent freezes exact fixture ids, acceptance commands, and wording expectations first.

### Why Lane C runs last

Docs are contract surfaces. They must describe the landed M59 behavior, not an estimated one. Running docs earlier would create overclaim risk and extra merge churn.

### Effective concurrency policy

- one parent integrator always active
- up to two worker lanes active at once
- launch wave 1: Lane A and Lane B
- launch wave 2: Lane C only after A and B are integrated and proven

## Assumptions

- `PLAN.md` remains the authority unless the parent explicitly refreshes the contract.
- The repo continues to use `feat/m40-plus` as the primary execution branch and `main` as base.
- Bun is available in the execution environment for the maintained TypeScript proof.
- The shipped supported function-family vocabulary remains the one already admitted by the lane; M59 does not promote new families.
- The M59 local fixture tree can be expressed entirely under `spec-cli/tests/fixtures/typescript_local_supported_graph/`.
- Broad selectors `cargo test -p spec-core typescript` and `cargo test -p spec-cli --test cli typescript` are acceptable as the default targeted proof walls unless the parent freezes narrower stable selectors before worker launch.
- `.runs/*` is acceptable as ephemeral orchestration state and is not treated as product output.

## Completion Criteria

M59 is done only when all of the following are true:

- `M59-00` through `M59-31` are integrated or intentionally closed by the parent with equivalent landed changes.
- Lane A is integrated and its targeted core proof passes.
- Lane B is integrated and its targeted CLI plus direct local-graph proofs pass.
- Lane C is integrated or intentionally completed by the parent with equivalent doc updates.
- `README.md` and `TODOS.md` match the shipped M59 behavior exactly.
- Final `cargo test -p spec-core` and `cargo test -p spec-cli --test cli` are green.
- `acceptance-ledger.md` and `final-proof-manifest.json` are written under `$RUN_ROOT/`.
- No hard-guard breach remains unresolved.
