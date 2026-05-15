# M61 Orchestration Plan

Status: **authoritative execution runbook**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Plan title: **`M61: Bounded Recursive Cross-Library TypeScript Function-Graph Execution Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Authority validated commit in `PLAN.md`: **`96d2ee9`**  
Base branch: **`main`**  
Authority date: **`2026-05-15`**  
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**  
Worker model assumption: **`GPT-5.4` with `reasoning_effort=high`**  
Rewrite intent: **replace the stale M60 repo-root orchestration doc with an execution-ready M61 runbook grounded only in `PLAN.md`**  
Last rewritten: **`2026-05-15`**

## Summary

M61 ships one bounded capability and nothing broader:

`M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.`

The parent agent owns the critical path and is the only integrator. The safe execution shape is fixed:

1. parent freezes the M61 contract from `PLAN.md`
2. parent runs **Lane A** locally and sequentially inside `spec-core/src/`
3. parent integrates Lane A and freezes the qualified-identity contract, supported family list, exact docs sentence, and exact maintained example filenames
4. **Lane B** and **Lane C** run in parallel from the post-A integration state
5. parent integrates Lane B as soon as it is green, then launches **Lane D**
6. parent integrates Lane C and Lane D
7. parent runs **Lane E** serially for the final proof wall and artifact refresh
8. parent fast-forwards `feat/m40-plus`

Historical files are shape references only:

- `docs/m26_orchestration_kickoff_prompt.md`
- the current repo-root `ORCH_PLAN.md`

They are not authority for milestone facts, scope, branches, worktree roots, commands, or acceptance.

## Hard Guards

- `PLAN.md` is the only authority source for milestone facts.
- `docs/m26_orchestration_kickoff_prompt.md` and the stale repo-root `ORCH_PLAN.md` may be used only for structure and rigor examples.
- Do not copy any stale milestone facts, branches, commit ids, worktree roots, acceptance gates, packet paths, or promotion mechanics from M26 or M60.
- M61 is not a semantic-family promotion milestone.
- Prohibited stale mechanics:
  - no `.semantic-family-artifacts/*`
  - no `cargo xtask family *`
  - no approval-gate artifacts
  - no family packet creation
  - no corpus recommendation or promotion commands
- The public docs sentence is frozen and must be used verbatim on every docs surface:
  - `M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.`
- The supported family set is frozen to the six families already listed in `PLAN.md`:
  - `function.helper.identity_passthrough.v1`
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.wrapper.pipeline.v1`
  - `function.wrapper.pipeline.normalized_required_arg.v1`
  - `function.wrapper.pipeline.chain3.v1`
- No lane may introduce new semantic-family meaning.
- No lane may widen TypeScript execution to:
  - arbitrary authored 4+ direct-dep topology parity
  - molecule TypeScript execution
  - seam-kind TypeScript execution
  - `spec validate --target-language`
  - `spec export --target-language`
  - non-Bun TypeScript toolchains
- `spec-core/src/semantic_review.rs` is a no-touch truth source for this milestone.
- `spec-cli/src/commands.rs` is a no-touch surface unless the parent explicitly re-scopes after a blocker.
- Lane A owns the `spec-core/src/` contract wall and must run Step 1 then Step 2 sequentially.
- Lane B owns the maintained recursive cross-library example across both:
  - `examples/shared-spec/`
  - `examples/crosslib-app/`
- Lane C owns docs and release-note sync only after the exact contract sentence and maintained example filenames are frozen.
- Lane D owns `spec-cli/tests/cli.rs` only, and starts only after Lane A and Lane B converge.
- Lane E is parent-only final proof and artifact refresh after A + B + C + D converge.
- No lane may revert, reset, clean, stash, or overwrite unowned changes.

Stop and re-scope immediately if any of these become true:

1. `PLAN.md` changes materially during execution and the parent has not refreshed the freeze artifacts.
2. Lane A requires changes to `spec-core/src/semantic_review.rs` to make M61 work.
3. Lane A needs a new CLI surface, export schema, passport schema, or new target-language flag behavior.
4. Lane B needs example file names different from the exact maintained example seed frozen in `PLAN.md`.
5. Lane C cannot use the exact frozen sentence because code widened beyond the admitted M61 boundary.
6. Lane D requires edits outside `spec-cli/tests/cli.rs` to keep the CLI regression wall truthful.
7. Any lane needs to reopen semantic-family promotion or packet work.
8. Any lane needs to make `.test.spec` TypeScript execution work.
9. The final proof wall implies ambiguous raw-id selection still exists for same-id local/shared units.
10. A worker touches files outside its frozen write scope.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m61
RUN_ROOT=$PRIMARY_ROOT/.runs/m61_recursive_cross_library_typescript
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + state | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | contract freeze, durable run-state, final fast-forward |
| `WS-INT` | `$WT_ROOT/int` | `ws/m61-int` | Parent | integration branch and final proof lane |
| `WS-A` | `$WT_ROOT/core` | `ws/m61-core` | Parent | Lane A Step 1 then Step 2 inside `spec-core/src/` |
| `WS-B` | `$WT_ROOT/examples` | `ws/m61-examples` | Worker | maintained recursive shared example and derived artifacts |
| `WS-C` | `$WT_ROOT/docs` | `ws/m61-docs` | Worker | README, TODOS, CHANGELOG, and example README sync |
| `WS-D` | `$WT_ROOT/cli` | `ws/m61-cli` | Worker | CLI regression refresh in `spec-cli/tests/cli.rs` |

### Worktree creation rules

- Do not create worker worktrees before `M61-01` contract freeze completes.
- Create `WS-INT` and `WS-A` first.
- `WS-B` and `WS-C` must branch from the integrated post-A state in `ws/m61-int`, not from the stale pre-A branch tip.
- `WS-D` must branch only after Lane B is integrated into `ws/m61-int`.
- There is no separate `WS-E`. Lane E runs only in `WS-INT`.
- Do not split `examples/shared-spec/**` and `examples/crosslib-app/**` into different worktrees.
- Do not let docs run before the exact sentence and exact maintained example filenames are frozen in `contract-freeze.json`.
- Record the dirty tree at kickoff and preserve it.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b ws/m61-int feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/core" -b ws/m61-core feat/m40-plus

# after WS-A is integrated into ws/m61-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/examples" -b ws/m61-examples ws/m61-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/docs" -b ws/m61-docs ws/m61-int

# after WS-B is integrated into ws/m61-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/cli" -b ws/m61-cli ws/m61-int
```

## Durable Orchestration State

All durable orchestration state lives under:

```bash
$RUN_ROOT
```

This directory is run-state only. It is not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, HEAD, dirty-tree snapshot, authority commit | Parent |
| `contract-freeze.json` | frozen M61 contract, exact sentence, exact filenames, supported families, lane scopes | Parent |
| `worktrees.json` | exact worktree paths, branches, heads, and lane states | Parent |
| `file-ownership.json` | lane write scopes and global no-touch surfaces | Parent |
| `tasks.json` | canonical task ledger, dependencies, and current states | Parent |
| `session-log.md` | chronological launch, integration, rerun, block, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `final-proof-manifest.json` | exact final commands, exit codes, and captured output paths | Parent |
| `validation/kickoff/` | kickoff snapshots | Parent |
| `validation/ws-a/` | validator and collector proof captures | Parent |
| `validation/ws-b/` | example loop and target-language proof captures | Parent |
| `validation/ws-c/` | docs sentence and backlog wording captures | Parent |
| `validation/ws-d/` | CLI regression proof captures | Parent |
| `validation/final/` | final serial proof wall captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |
| `tasks/<TASK_ID>/` | per-task sentinels and state files | Parent creates, lane updates |

### Required `baseline.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `historical_shape_refs`
- `observed_primary_surfaces`
- `baseline_commands`
- `run_started_at`

### Required `contract-freeze.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_commit`
- `primary_branch`
- `frozen_scope_claim`
- `frozen_docs_sentence`
- `frozen_supported_families`
- `maintained_example_paths`
- `preserved_example_paths`
- `lane_ownership`
- `global_no_touch_surfaces`
- `serialization_points`
- `integration_order`
- `worker_model`
- `worker_return_contract`
- `forbidden_mechanics`
- `verification_commands`
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

A task is done only after the parent integrates the lane and reruns the relevant proof wall.

## Blocked-State Protocol

Blocked state is explicit and durable.

### Standard blocker codes

- `PLAN_DRIFT`
- `QUALIFIED_IDENTITY_DRIFT`
- `EXAMPLE_FILENAME_DRIFT`
- `DOC_SENTENCE_DRIFT`
- `TS_BOUNDARY_EXPANSION`
- `WRITE_SCOPE_VIOLATION`
- `OWNERSHIP_CONFLICT`
- `PROOF_WALL_FAIL`
- `MERGE_RISK`
- `ENVIRONMENT_MISSING`
- `UNEXPECTED_WRITE_SCOPE`

### What a lane writes when blocked

If a lane cannot complete within scope, it must write all of the following before stopping:

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

When blocked, the lane sets at minimum:

- `state: "blocked"`
- `acceptance_status: "not_met"`
- `blocker_code`
- `blocker_summary`
- `next_action`

## Kickoff Sequence

### `M61-00` — baseline snapshot

Parent only, `PRIMARY_ROOT`.

Run:

```bash
git -C "$PRIMARY_ROOT" branch --show-current
git -C "$PRIMARY_ROOT" rev-parse --short HEAD
git -C "$PRIMARY_ROOT" status --short
```

Write:

- `baseline.json`
- `validation/kickoff/branch.txt`
- `validation/kickoff/head.txt`
- `validation/kickoff/status.txt`

Acceptance:

- kickoff records the real dirty tree
- kickoff records `feat/m40-plus`
- kickoff records the authority commit `96d2ee9`
- kickoff records the two historical shape refs as non-authoritative

### `M61-01` — contract freeze

Parent only, `PRIMARY_ROOT`.

Freeze the following exact values into `contract-freeze.json`:

- frozen sentence:
  - `M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.`
- maintained example seed:
  - `examples/shared-spec/units/pricing/calculate_total.unit.spec`
  - `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec`
  - `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
- preserved direct-root proof owners:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `examples/crosslib-app/units/pricing/calculate_total.unit.spec`
- frozen supported family set from `PLAN.md`
- frozen lane ownership
- frozen verification commands
- frozen parent critical path
- prohibited stale M26/M60 mechanics

Write:

- `contract-freeze.json`
- `file-ownership.json`
- initial `tasks.json`

Acceptance:

- every later lane operates from one frozen M61 contract file instead of re-reading `PLAN.md` ad hoc
- the exact docs sentence and exact maintained example paths are durable before worker launch

### `M61-02` — create worktrees

Parent only, `PRIMARY_ROOT`.

Create:

- `WS-INT`
- `WS-A`

Delay:

- `WS-B`
- `WS-C`
- `WS-D`

until their dependencies are integrated.

Write:

- `worktrees.json`
- `session-log.md` kickoff entry

Acceptance:

- no worker starts from stale pre-A state

## Parent Critical Path

This path is fixed and parent-owned.

1. `M61-00` baseline snapshot in `PRIMARY_ROOT`
2. `M61-01` contract freeze in `PRIMARY_ROOT`
3. `task/m61-a1-validator-contract` in `WS-A`
4. `task/m61-a2-qualified-collector` in `WS-A`
5. integrate `WS-A` into `WS-INT`
6. create and launch `WS-B` and `WS-C` from the post-A `WS-INT` state
7. integrate `WS-B` into `WS-INT` as soon as it is green
8. create and launch `WS-D` from the post-B `WS-INT` state
9. integrate `WS-C` and `WS-D`
10. run `task/m61-e-final-proof-wall` in `WS-INT`
11. run `task/m61-f-final-branch-handoff` in `PRIMARY_ROOT`

Nothing skips step 4. Step 1 and Step 2 from `PLAN.md` stay serialized inside Lane A.

## Workstream Plan

### WS-A (`ws/m61-core`) — parent agent only, sequential

This is the contract wall. Keep it local.

#### `task/m61-a1-validator-contract`

- Own only:
  - `spec-core/src/validator.rs`
- Do:
  - replace the local-vs-portability root split with one recursive qualified closure validation flow
  - resolve deps in owner-library context
  - admit the existing eligible TypeScript root families from `PLAN.md`
  - explicitly include `function.wrapper.pipeline.normalized_required_arg.v1` in TypeScript root/member handling
  - keep helper, wrapper, normalized-wrapper, and chain3 dep contracts explicit and separate
  - preserve failure-before-Bun behavior for:
    - unsupported semantic review
    - wrong family
    - wrong dep order
    - missing `body.typescript`
    - unresolved dep
    - molecule TypeScript rejection
- Do not own:
  - `spec-core/src/typescript_backend.rs`
  - `spec-core/src/semantic_review.rs`
  - examples
  - docs
  - CLI tests

Run:

```bash
cargo test -p spec-core validator
```

Acceptance for `task/m61-a1-validator-contract`:

- supported local-only roots still validate
- supported direct cross-library roots still validate
- supported recursive shared roots now validate
- normalized-required-arg wrappers are legal in the TypeScript lane when their existing M60 family contract is satisfied
- preserved red paths still fail before Bun
- `spec-core/src/validator.rs` is the only edited file in this task

#### `task/m61-a2-qualified-collector`

- Own only:
  - `spec-core/src/typescript_backend.rs`
- Do:
  - replace raw-id collector lookup structures with qualified lookup structures
  - replace dep resolution with owner-library-qualified resolution
  - collapse local recursion and direct portability collection into one qualified recursive collector story
  - dedupe reachable members by qualified identity, not raw id
  - keep emitted file paths and import rendering stable unless the qualified collector proves a necessary change
  - keep normalized-required-arg wrapper support truthful in the collector path
- Do not own:
  - `spec-core/src/validator.rs`
  - `spec-core/src/semantic_review.rs`
  - examples
  - docs
  - CLI tests

Run:

```bash
cargo test -p spec-core validator
cargo test -p spec-core typescript_backend
```

Acceptance for `task/m61-a2-qualified-collector`:

- local/shared same-id units do not collide in closure membership
- recursive shared closure includes only reachable qualified members
- unrelated loaded units remain excluded from the emitted TS tree
- validator and collector no longer disagree about dep-edge ownership
- `spec-core/src/typescript_backend.rs` is the only edited file in this task

#### `task/m61-a-integrate-core`

Parent only, `WS-INT`.

Integrate `ws/m61-core` into `ws/m61-int`.

After integration:

- refresh `worktrees.json`
- refresh `tasks.json`
- create `WS-B` and `WS-C` from the integrated `ws/m61-int`
- write a session-log entry that the qualified-identity model, exact supported family set, exact docs sentence, and exact maintained example filenames are frozen

Acceptance:

- every downstream lane inherits the frozen post-A contract directly

### WS-B (`ws/m61-examples`) — worker 1

This lane owns the maintained recursive shared example and its derived artifacts.

#### `task/m61-b-maintained-example`

- Own only:
  - `examples/shared-spec/units/pricing/calculate_total.unit.spec`
  - `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec`
  - `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
  - derived outputs refreshed by the normal spec loop under:
    - `examples/shared-spec/src/generated/**`
    - `examples/crosslib-app/src/generated/**`
    - `examples/shared-spec/units/**/*.spec.passport.json`
    - `examples/crosslib-app/units/**/*.spec.passport.json`
- Do:
  - promote the recursive shared CLI prototype into the maintained example tree
  - keep the direct-root proof owners intact:
    - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
    - `examples/crosslib-app/units/pricing/calculate_total.unit.spec`
  - keep the maintained example filenames exactly as frozen
  - follow the spec source-truth loop:
    - validate source specs
    - build the affected example trees
    - test the exact edited source specs
  - verify the maintained example also works through the TypeScript target lane after branching from the post-A state
- Do not own:
  - `examples/crosslib-app/README.md`
  - docs
  - `spec-core/src/**`
  - `spec-cli/tests/cli.rs`

Run:

```bash
cargo run -p spec-cli -- validate examples/shared-spec/units/pricing/calculate_total.unit.spec --format json
cargo run -p spec-cli -- validate examples/shared-spec/units/pricing/base_nested_chain3.unit.spec --format json
cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --format json

cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-spec/src/generated
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated

cargo run -p spec-cli -- test examples/shared-spec/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- test examples/shared-spec/units/pricing/base_nested_chain3.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
```

Acceptance for `task/m61-b-maintained-example`:

- the three maintained example source files exist at the exact frozen paths
- the direct-root proof owners remain intact
- the normal spec loop succeeds for the edited source specs
- the maintained recursive shared root passes under `--target-language typescript`
- no example filename drift occurs
- only Lane B surfaces changed

### WS-C (`ws/m61-docs`) — worker 2

This lane is narrow and can run in parallel with Lane B after the post-A freeze.

#### `task/m61-c-doc-sync`

- Own only:
  - `README.md`
  - `TODOS.md`
  - `CHANGELOG.md`
  - `examples/crosslib-app/README.md`
- Do:
  - use the frozen M61 sentence verbatim
  - add the new recursive example command to `examples/crosslib-app/README.md`
  - narrow the TODO backlog exactly as `PLAN.md` requires
  - keep broader TypeScript oceans explicitly deferred:
    - arbitrary authored 4+ direct-dep topology parity
    - new semantic-family promotion
    - molecule TypeScript execution
    - seam-kind TypeScript execution
- Do not own:
  - example `.unit.spec` files
  - generated example artifacts
  - `spec-core/src/**`
  - `spec-cli/tests/**`

Run:

```bash
SENTENCE='M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.'

rg -nF "$SENTENCE" README.md TODOS.md CHANGELOG.md examples/crosslib-app/README.md
rg -n "arbitrary authored 4\\+ direct-dep topology parity|new semantic-family promotion|molecule TypeScript execution|seam-kind TypeScript execution" TODOS.md
rg -n "checkout_nested_chain3\\.unit\\.spec" examples/crosslib-app/README.md
```

Acceptance for `task/m61-c-doc-sync`:

- all four docs surfaces use the same frozen sentence
- `examples/crosslib-app/README.md` references the exact maintained recursive example filename
- `TODOS.md` narrows the backlog exactly as `PLAN.md` requires
- no docs imply arbitrary graph parity

### WS-D (`ws/m61-cli`) — worker 3, starts after WS-B is integrated

Do not create this lane until Lane B is integrated into `ws/m61-int`.

#### `task/m61-d-cli-regressions`

- Own only:
  - `spec-cli/tests/cli.rs`
- Do:
  - convert the current recursive shared nested chain3 rejection helper into the new green path
  - keep preserved red-path coverage for:
    - unsupported shared recursive member
    - wrong dep order inside a shared recursive member
    - missing `body.typescript` on a shared recursive member
    - unresolved shared dep
    - molecule TypeScript rejection
  - add at least one regression that proves owner-library-qualified resolution when local and shared same-id units coexist
  - keep the Bun-precheck rejection wall consistent
- Do not own:
  - `spec-core/src/**`
  - example source specs
  - generated example artifacts
  - docs

Run:

```bash
cargo test -p spec-cli --test cli
```

Acceptance for `task/m61-d-cli-regressions`:

- the new recursive shared green path passes
- preserved red paths still fail before Bun
- same-id owner-library-qualified lookup is covered
- only `spec-cli/tests/cli.rs` changed in this lane

## WS-INT (`ws/m61-int`) — parent agent only

### `task/m61-int-integrate-b-and-launch-d`

Integrate:

- `ws/m61-examples`

Policy:

- if Lane B changed any frozen example filename, reject the lane and bounce it back
- do not launch Lane D until Lane B is integrated into `ws/m61-int`

After integration:

- refresh `worktrees.json`
- refresh `tasks.json`
- create `WS-D` from the updated `ws/m61-int`
- launch Lane D with the integrated example state

### `task/m61-int-integrate-c-and-d`

Integrate:

- `ws/m61-docs`
- `ws/m61-cli`

Policy:

- if docs wording disagrees with the frozen sentence, docs lose and are bounced back
- if CLI expectations require code or example changes not already present in `ws/m61-int`, stop and bounce back rather than hot-fixing in integration
- do not resolve semantic disagreements creatively in `WS-INT`

### `task/m61-e-final-proof-wall`

Parent only, `WS-INT`.

Run the full proof wall in this order:

```bash
cargo fmt --all

cargo test -p spec-core validator
cargo test -p spec-core typescript_backend
cargo test -p spec-cli --test cli

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/crosslib-app/units --target-language typescript --format json
```

Write:

- `validation/final/*`
- `final-proof-manifest.json`
- `acceptance-ledger.md`

Acceptance for `task/m61-e-final-proof-wall`:

- validator tests are green
- TypeScript backend tests are green
- CLI regressions are green
- the maintained recursive shared root passes under Bun
- status remains target-specific and additive
- preserved red-path cases still reject before Bun
- formatting changes, if any, are limited to the integrated tree and are parent-owned

### `task/m61-f-final-branch-handoff`

Parent only, `PRIMARY_ROOT`.

After `ws/m61-int` is green:

- fast-forward `feat/m40-plus` to `ws/m61-int` if possible
- if fast-forward is not possible, stop and inspect manually rather than performing a creative merge in the authority root

Suggested command:

```bash
git -C "$PRIMARY_ROOT" merge --ff-only ws/m61-int
```

Acceptance:

- the primary execution branch now contains the integrated, proven M61 result

## Integration Order

This order is fixed.

1. kickoff and freeze on `PRIMARY_ROOT`
2. `WS-A`
3. integrate `WS-A` into `WS-INT`
4. create and launch `WS-B` + `WS-C` in parallel
5. integrate `WS-B`
6. create and launch `WS-D`
7. integrate `WS-C`
8. integrate `WS-D`
9. run final proof wall in `WS-INT`
10. fast-forward `feat/m40-plus`

If `WS-C` finishes before `WS-B`, keep it submitted and do not let it redefine the frozen sentence or filenames while waiting.

## Conflict Policies

- Lane A is not parallelizable internally. `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs` define one contract and stay sequential.
- Lane B and Lane D both touch the cross-library example story. Lane D must branch from the post-B integrated state.
- Lane C must not invent filenames, commands, or sentence variants. It consumes the freeze literally.
- `spec-core/src/semantic_review.rs` is a global no-touch surface.
- `spec-cli/src/commands.rs` is a global no-touch surface.
- `PLAN.md` is a global no-touch surface during execution.
- If a worker finds it needs another lane’s file, it stops and files a blocker instead of expanding its scope.
- The parent is the only integrator onto `ws/m61-int` and `feat/m40-plus`.

## Context-Control Rules

- Parent keeps only four live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - frozen values from `contract-freeze.json`
  - required commands
  - forbidden touch surfaces
  - the worker model assumption `GPT-5.4 / high`
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not write `RUN_ROOT` except through their assigned task sentinels.
- The parent reviews summaries plus narrow diffs only.
- Close each worker immediately after merge.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Core contract
  - one recursive qualified validator path exists
  - one qualified recursive collector path exists
  - normalized-required-arg wrapper support is truthful in the TypeScript lane
  - `spec-core/src/semantic_review.rs` remains unchanged
- Maintained example truth
  - the exact three maintained example source files exist
  - the normal spec loop is green for the edited source specs
  - the direct-root proof owners remain intact
  - the recursive shared root passes under `--target-language typescript`
- Docs
  - README, TODOS, CHANGELOG, and `examples/crosslib-app/README.md` all use the same frozen sentence
  - docs do not imply arbitrary graph parity
  - `TODOS.md` preserves the remaining TypeScript oceans exactly as bounded by `PLAN.md`
- CLI truth
  - `spec-cli/tests/cli.rs` contains the green recursive shared path
  - preserved red-path regressions still fail before Bun
  - same-id owner-library resolution is covered
- Final proof wall
  - all final commands pass in sequence
  - the maintained recursive shared root is green
  - status remains additive and target-specific
  - no stale M26/M60 commands were needed anywhere in the run

## Assumptions

- No human approval gate is required because `PLAN.md` does not define one for M61.
- `cargo run -p spec-cli -- test <unit.unit.spec> --target-language typescript` remains the correct public proof surface for this milestone.
- Generated outputs under `src/generated/**` and `*.spec.passport.json` are derived surfaces refreshed by the normal spec loop when Lane B edits source specs.
- The current dirty tree, if any, is preserved throughout execution.
- `spec-cli/tests/cli.rs` is the only CLI regression file Lane D needs. If the repo’s actual regression truth is split elsewhere, that is a blocker and not an implicit scope expansion.
