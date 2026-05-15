# M60 Orchestration Plan

Status: **authoritative execution runbook**  
Supersedes: **the stale M59 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M60: Normalized-Required-Arg Wrapper Family Execution Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Authority validated commit in `PLAN.md`: **`f401d49`**  
Base branch: **`main`**  
Authority date: **`2026-05-15`**  
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**  
Rewrite intent: **replace the stale M59 runbook with an execution-ready M60 runbook aligned to the current `PLAN.md` contract**  
Last rewritten: **`2026-05-15`**

## Summary

M60 is one bounded semantic-review widen and nothing more.

It ships exactly one new supported family:

- `function.wrapper.pipeline.normalized_required_arg.v1`

It preserves exactly one existing sibling boundary:

- `function.wrapper.pipeline.v1` stays raw-arg only

It admits exactly one new required-arg normalization surface:

- `param.max(Decimal::ZERO)`

It does not ship:

- generic wrapper expression support
- broader required-arg normalization
- new dep topology support
- any new TypeScript execution behavior
- any new seam family
- any corpus-program reopen

The parent agent remains the sole integrator.

Parallelism is intentionally narrow:

- `WS-CORE` stays parent-owned and serialized because `spec-core/src/semantic_review.rs` is the contract wall
- `WS-PACKET` and `WS-DOCS` can run in parallel only after the core family contract is frozen
- `WS-CLI` starts only after packet/example/fixture work is merged into the integration branch
- `WS-INT` remains parent-only and is the only lane allowed to run the final proof wall

This runbook optimizes for one thing: land the exact M60 family boundary without packet drift, fixture drift, or read-side lies.

## Hard Guards

- `PLAN.md` is the only scope authority.
- M60 ships exactly one new family key:
  - `function.wrapper.pipeline.normalized_required_arg.v1`
- `function.wrapper.pipeline.v1` must remain raw-arg only.
- The only newly supported required-arg normalization surface is:
  - `param.max(Decimal::ZERO)`
- No lane may widen support to:
  - literals
  - arithmetic expressions
  - chained method pipelines
  - multi-input expressions
  - multi-arg normalization
- No lane may change `examples/ecommerce/units/pricing/calculate_total.unit.spec`.
- The maintained example added by M60 is frozen to:
  - `examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec`
- The promoted unsupported fixtures must be repaired in place, not moved and not renamed:
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/checkout_net_total_unsupported_near_miss.unit.spec`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/calculate_total.unit.spec`
- The exact replacement unsupported shapes are frozen:
  - M19 replacement: `regional_rate.max(Decimal::ZERO).round_dp(4)`
  - M20 replacement: `tax_rate + Decimal::ZERO`
- The new packet must mirror the existing wrapper packet layout under:
  - `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/`
- The public docs sentence is frozen and must be used verbatim:
  - `M60 adds one supported wrapper family for apply_tax(discounted, tax_rate.max(Decimal::ZERO)); broader required-argument expressions remain unsupported.`
- The route order freeze from `PLAN.md` must hold:
  - `WrapperPipelineChain3`
  - `WrapperPipelineNormalizedRequiredArg`
  - `WrapperPipeline`
  - `ArithmeticLeafMonotoneDownNonnegative`
  - `ArithmeticLeafMonotoneUp`
  - `HelperIdentityPassthrough`
- The parent is the only integrator onto `feat/m40-plus`.
- No lane may revert, clean, stash, or overwrite unowned changes.

Stop and re-scope immediately if any of these become true:

1. `spec-core/src/semantic_review.rs` needs a broader expression classifier than the frozen `param.max(Decimal::ZERO)` surface.
2. The packet/example/fixture lane discovers it must rename or move the frozen M19 or M20 file paths to make tests pass.
3. The docs lane cannot use the frozen sentence because the code widened beyond the admitted surface.
4. The CLI lane needs write access outside `spec-cli/tests/cli.rs` unless the parent explicitly expands ownership.
5. The implementation requires changes to:
   - CLI command surface
   - export schema
   - passport schema
   - family-analysis policy semantics
6. The final proof wall implies the corpus program should reopen.
7. `PLAN.md` changes materially during execution and the parent has not refreshed the contract freeze before continuing.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m60
RUN_ROOT=$PRIMARY_ROOT/.runs/m60_normalized_required_arg_wrapper_family
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + state | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, freeze, run-state, final fast-forward |
| `WS-CORE` | `$WT_ROOT/core` | `ws/m60-core` | Parent | Step 1 + Step 2 classifier contract in `spec-core/src/semantic_review.rs` |
| `WS-PACKET` | `$WT_ROOT/packet` | `ws/m60-packet` | Worker | maintained example + new family packet + M19/M20 in-place fixture repair |
| `WS-DOCS` | `$WT_ROOT/docs` | `ws/m60-docs` | Worker | README/TODOS/CHANGELOG sync only |
| `WS-CLI` | `$WT_ROOT/cli` | `ws/m60-cli` | Worker | CLI truth assertions after packet lane lands |
| `WS-INT` | `$WT_ROOT/int` | `ws/m60-int` | Parent | integration, final proof wall, final branch handoff |

### Worktree creation rules

- Do not create worker worktrees before `M60-01` contract freeze completes.
- Create `WS-CORE` and `WS-INT` first.
- `WS-PACKET` and `WS-DOCS` must branch from the integrated post-core state, not from stale pre-core HEAD.
- `WS-CLI` must branch only after the packet lane is integrated into `WS-INT`.
- Do not split packet, example, and fixture work across multiple worktrees. That is one product boundary and one lane.
- Do not let docs author speculative wording before the replacement shapes and family key are frozen in the run-state artifacts.
- Record the dirty tree at kickoff and preserve it. `PLAN.md` and this runbook may already be dirty at session start.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b ws/m60-int feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/core" -b ws/m60-core feat/m40-plus

# after WS-CORE is integrated into ws/m60-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/packet" -b ws/m60-packet ws/m60-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/docs" -b ws/m60-docs ws/m60-int

# after WS-PACKET is integrated into ws/m60-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/cli" -b ws/m60-cli ws/m60-int
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
| `baseline.json` | kickoff branch, HEAD, dirty-tree snapshot, authority commit | Parent |
| `contract-freeze.json` | frozen M60 contract, route order, replacement shapes, docs sentence, lane scopes | Parent |
| `worktrees.json` | exact worktree paths, branches, current heads, and lane states | Parent |
| `file-ownership.json` | exact file ownership and no-touch surfaces per lane | Parent |
| `tasks.json` | canonical task ledger, dependencies, and current states | Parent |
| `session-log.md` | chronological launch, handoff, integration, rerun, block, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `final-proof-manifest.json` | exact final commands, exit codes, and output paths | Parent |
| `validation/kickoff/` | branch, HEAD, dirty-tree, and authority snapshots | Parent |
| `validation/ws-core/` | targeted core-lane proof captures | Parent |
| `validation/ws-packet/` | example, packet, and fixture proof captures | Parent |
| `validation/ws-docs/` | wording verification captures | Parent |
| `validation/ws-cli/` | CLI truth proof captures | Parent |
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
- `new_family_key`
- `preserved_raw_family_key`
- `admitted_required_arg_surface`
- `frozen_route_order`
- `frozen_docs_sentence`
- `maintained_example_path`
- `rewritten_fixture_paths`
- `rewritten_fixture_shapes`
- `new_packet_root`
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
- `ROUTE_ORDER_DRIFT`
- `FIXTURE_ID_DRIFT`
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

### `M60-00` — baseline snapshot

Parent only, `PRIMARY_ROOT`.

Run:

```bash
git -C "$PRIMARY_ROOT" branch --show-current
git -C "$PRIMARY_ROOT" rev-parse HEAD
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
- kickoff records the authority commit `f401d49` from `PLAN.md`

### `M60-01` — contract freeze

Parent only, `PRIMARY_ROOT`.

Freeze the following exact values into `contract-freeze.json`:

- family key:
  - `function.wrapper.pipeline.normalized_required_arg.v1`
- preserved sibling:
  - `function.wrapper.pipeline.v1`
- admitted surface:
  - `param.max(Decimal::ZERO)`
- maintained example:
  - `examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec`
- protected unchanged example:
  - `examples/ecommerce/units/pricing/calculate_total.unit.spec`
- rewritten M19 file path and exact replacement expression
- rewritten M20 file path and exact replacement expression
- frozen docs sentence
- frozen route order

Write:

- `contract-freeze.json`
- `file-ownership.json`
- initial `tasks.json`

Acceptance:

- every later lane can operate from one frozen contract file instead of interpreting `PLAN.md` ad hoc

### `M60-02` — create worktrees

Parent only, `PRIMARY_ROOT`.

Create:

- `WS-INT`
- `WS-CORE`

Delay:

- `WS-PACKET`
- `WS-DOCS`
- `WS-CLI`

until their dependencies are integrated.

Write:

- `worktrees.json`
- `session-log.md` kickoff entry

Acceptance:

- no worker lane starts from stale pre-core state

## Workstream Plan

### WS-CORE (`ws/m60-core`) — parent agent only, sequential

This is the critical path. Keep it local.

#### `task/m60-a-core-classifier`

- Own only:
  - `spec-core/src/semantic_review.rs`
- Do:
  - Step 1 from `PLAN.md`
  - Step 2 from `PLAN.md`
  - add the new family key
  - add the new route before the raw wrapper route
  - implement the exact bounded required-arg classifier for `param.max(Decimal::ZERO)`
  - preserve raw wrapper strictness
  - add or update semantic-review unit tests in the same file for:
    - aligned
    - drift
    - under-specified
    - unsupported arithmetic expression
    - unsupported chained-method expression
    - unsupported literal replacement
    - route ordering where the test naturally belongs in `semantic_review.rs`
- Do not own:
  - packet files
  - example files
  - fixtures
  - docs
  - CLI tests

Run:

```bash
cargo test -p spec-core semantic_review
```

Acceptance for `task/m60-a-core-classifier`:

- the exact family key exists
- the exact route order is preserved
- `function.wrapper.pipeline.v1` still rejects normalized required args
- broader required-arg expressions still remain unsupported
- `spec-core/src/semantic_review.rs` is the only edited file in the lane

#### `task/m60-a-integrate-core`

Parent only, `WS-INT`.

Integrate `ws/m60-core` into `ws/m60-int`.

After integration:

- refresh `worktrees.json`
- refresh `tasks.json`
- create `WS-PACKET` and `WS-DOCS` from the integrated `ws/m60-int`

Acceptance:

- packet and docs lanes inherit the frozen core family contract directly

### Parallel workers after WS-CORE is green and integrated

### WS-PACKET (`ws/m60-packet`) — worker 1

This lane owns the entire product-boundary surface outside core classifier logic.

#### `task/m60-b-packet-example-fixtures`

- Own only:
  - `examples/ecommerce/units/pricing/**`
  - `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/**`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/**`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/**`
- Do:
  - add `calculate_total_guarded_tax.unit.spec`
  - keep `calculate_total.unit.spec` unchanged
  - create the new family packet by mirroring the wrapper sibling packet layout
  - keep packet-local naming aligned with the sibling wrapper packet
  - rewrite the M19 and M20 unsupported fixtures in place using the frozen replacement expressions
  - preserve `unsupported_required_argument_expression` ownership
- Do not own:
  - `spec-core/src/semantic_review.rs`
  - `spec-cli/tests/cli.rs`
  - `README.md`
  - `TODOS.md`
  - `CHANGELOG.md`

Suggested verification inside lane:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec --format json
```

Acceptance for `task/m60-b-packet-example-fixtures`:

- the new packet exists and mirrors the sibling wrapper packet shape
- the maintained example exists at the frozen path
- the raw-arg canonical example remains unchanged
- M19 and M20 fixture file paths and ids stay stable
- the replacement unsupported expressions match the frozen contract exactly

### WS-DOCS (`ws/m60-docs`) — worker 2

This lane is intentionally narrow and can run in parallel with the packet lane after the core freeze.

#### `task/m60-c-doc-sync`

- Own only:
  - `README.md`
  - `TODOS.md`
  - `CHANGELOG.md`
- Do:
  - update supported-family inventory language
  - mark M60 as shipped in `TODOS.md`
  - keep the broader normalization backlog explicitly deferred
  - use the frozen docs sentence verbatim
- Do not own:
  - tests
  - fixtures
  - example files
  - `spec-core/src/semantic_review.rs`

Acceptance for `task/m60-c-doc-sync`:

- all three docs surfaces use the same frozen sentence
- no docs imply generic computed required-arg support
- `TODOS.md` still preserves the post-M59 TypeScript oceans as deferred

### WS-CLI (`ws/m60-cli`) — worker 3, starts after WS-PACKET is integrated

Do not create this lane until the packet/example/fixture lane is integrated into `ws/m60-int`.

#### `task/m60-d-cli-truth`

- Own only:
  - `spec-cli/tests/cli.rs`
- Do:
  - update whole-pack truth assertions for the new family
  - update M19 unsupported truth expectations
  - update M20 unsupported truth expectations
  - add guarded-tax example validate/test/status coverage if the coverage belongs in `cli.rs`
  - preserve read-side truth for unsupported surfaces
- Do not own:
  - `spec-core/src/semantic_review.rs`
  - packet files
  - docs
  - `spec-cli/tests/m14_regressions.rs` unless the parent explicitly expands scope after finding existing wrapper truth there

Run:

```bash
cargo test -p spec-cli --test cli
```

Acceptance for `task/m60-d-cli-truth`:

- CLI truth assertions match the promoted family boundary
- M19 replacement unsupported case stays unsupported
- M20 replacement unsupported case stays unsupported
- no unsupported pack silently turns green

## WS-INT (`ws/m60-int`) — parent agent only

### `task/m60-e-integrate-packet-and-docs`

Integrate:

- `ws/m60-packet`
- `ws/m60-docs`

Policy:

- if packet layout and docs wording disagree, the packet contract wins and docs are rewritten to the frozen sentence
- do not resolve creative semantic disagreements at integration time
- if packet lane changed protected file paths or ids, reject the lane and bounce it back

After integration:

- refresh `worktrees.json`
- refresh `tasks.json`
- create `WS-CLI` from the updated `ws/m60-int`

### `task/m60-f-integrate-cli`

Integrate:

- `ws/m60-cli`

Policy:

- if CLI expectations require code or packet changes not already present in `ws/m60-int`, stop and bounce back rather than hot-fixing in integration

### `task/m60-g-final-proof-wall`

Parent only, `WS-INT`.

Run the full proof wall in this order:

```bash
cargo fmt --all

cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli

cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json

cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Write:

- `validation/final/*`
- `final-proof-manifest.json`
- `acceptance-ledger.md`

Acceptance for `task/m60-g-final-proof-wall`:

- `spec-core` semantic-review tests are green
- `spec-cli` truth tests are green
- the guarded-tax example validates and tests cleanly
- status for the maintained example tree stays honest
- coverage reflects the new family honestly
- recommendation remains stop-state
- corpus decision remains stop-state
- verify-decision-contract passes

### `task/m60-h-final-branch-handoff`

Parent only, `PRIMARY_ROOT`.

After `ws/m60-int` is green:

- fast-forward `feat/m40-plus` to `ws/m60-int` if possible
- if fast-forward is not possible, stop and inspect manually rather than performing a creative merge in the authority root

Suggested command:

```bash
git -C "$PRIMARY_ROOT" merge --ff-only ws/m60-int
```

Acceptance:

- the primary execution branch now contains the integrated, proven M60 result

## Integration Order

This order is fixed.

1. Kickoff and freeze on `PRIMARY_ROOT`
2. `WS-CORE`
3. integrate core into `WS-INT`
4. create and launch `WS-PACKET` + `WS-DOCS` in parallel
5. integrate packet and docs into `WS-INT`
6. create and launch `WS-CLI`
7. integrate CLI into `WS-INT`
8. run final proof wall in `WS-INT`
9. fast-forward `feat/m40-plus`

## Context-Control Rules

- Parent keeps only four live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- The parent reviews summaries plus narrow diffs only. It does not ingest full worker transcripts into the main context.
- Close each worker immediately after merge.
- Use task sentinels or long waits, not tight polling.

## Tests And Acceptance

- Core contract
  - `spec-core/src/semantic_review.rs` alone owns the new route, key, and exact bounded classifier.
  - route order stays `chain3 -> normalized -> raw -> monotone_down -> monotone_up -> helper`.
  - raw wrapper remains strict.
- Packet/example/fixture truth
  - `calculate_total_guarded_tax.unit.spec` exists.
  - `calculate_total.unit.spec` remains unchanged.
  - the new family packet exists under the frozen root.
  - M19 and M20 unsupported file paths and ids remain unchanged.
  - the frozen replacement unsupported expressions are present exactly.
- Docs
  - README, TODOS, and CHANGELOG all use the same frozen sentence.
  - broader required-arg support remains explicitly deferred.
- CLI truth
  - `spec-cli/tests/cli.rs` reflects the new family and the repaired unsupported truth surfaces.
  - unsupported packs remain unsupported after the promotion.
- Final proof wall
  - all commands from `PLAN.md` pass in sequence.
  - family-analysis remains in stop-state.
  - no accidental corpus reopen signal appears.

## Assumptions

- Worktree naming follows the repo's existing `.worktrees/spec-*` pattern.
- The parent agent is allowed to keep orchestration state under `.runs/`.
- Generated proof artifacts from `spec test` and related commands are derived surfaces. The parent decides whether any tracked proof artifacts need refresh based on existing repo conventions after the final proof wall.
- No human approval gate is required for M60 because `PLAN.md` does not define one; the only intentional pauses are blocked-state stops and the final parent fast-forward step.
- `spec-cli/tests/cli.rs` remains the primary CLI truth surface for this milestone. If the repo’s actual assertions are split elsewhere, that is a scope-expansion signal and must be handled explicitly, not implicitly.
