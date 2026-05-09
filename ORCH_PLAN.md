# M41 Orchestration Plan

Status: **authoritative execution contract for M41 "Helper-Surface Semantic Review Substrate"**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Milestone: **Helper-Surface Semantic Review Substrate**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Working branch: **`feat/m40-plus`**  
Last rewritten: **`2026-05-09`**  
Canonical run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m41_helper_surface_semantic_review_substrate`**

## Summary

- `PLAN.md` is the sole authority. This file is the execution contract for finishing that authority.
- M41 is one bounded runtime semantic-review expansion plus read-side truth refresh plus `xtask` operator-truth refresh plus docs.
- This is not packet promotion.
- The parent agent remains the sole integrator, gate owner, acceptance owner, and closeout author.
- The only honest split is:
  1. parent lands and freezes the runtime helper route in `spec-core`
  2. read-side proof and `xtask` operator truth may proceed in parallel
  3. docs and changelog run last under the parent
- If the runtime route contract is not frozen, or if read-side work requires production `spec-cli/src/*.rs` edits, parallelism collapses and the parent continues sequentially on `feat/m40-plus`.

## Hard Guards

- No new packet under `semantic-families/function.*`.
- No new CLI flags.
- No new schema version.
- No generic helper taxonomy.
- No second helper route.
- No corpus-manifest changes.
- No decision-kernel policy widening.
- Do not edit production `spec-cli/src/*.rs` unless read-side tests prove a real projection bug.
- Do not edit `xtask/src/family/recommend.rs` unless `xtask` proof shows a real operator-truth bug that cannot be retired by the intended inventory and coverage refresh. If that happens, stop and rewrite this plan before continuing.
- Do not touch docs before read-side and `xtask` truth are integrated and proven.
- Workers never own gates, never redefine scope, and never merge directly into `feat/m40-plus`.
- `PLAN.md` wins over this file, stale notes, and `.runs/*` artifacts if they disagree.
- `.runs/*` is execution evidence only. It is never authority.

## Locked Outcome Contract

Post-M41 repo truth must be:

- `spec-core` exposes exactly one new supported helper route:
  - route marker: `HelperIdentityPassthrough`
  - compatibility key: `function.helper.identity_passthrough.v1`
- `money/round`-style helper units return supported helper semantic-review truth.
- Fresh read-side surfaces preserve that truth:
  - canonical passport
  - `spec status`
  - `spec export`
- `xtask` publishes the helper route as runtime-supported and supported-unpromoted.
- `cargo xtask family recommend --format json` no longer treats the current helper wedge as live unsupported pressure.
- Promoted family counts remain unchanged.
- No new promoted packet is introduced.

## Locked File Surface

Expected edit surface:

| Area | Expected files |
|---|---|
| runtime contract | `spec-core/src/semantic_review.rs` |
| read-side proof | `spec-cli/tests/cli.rs`, `examples/shared-spec/units/money/round.spec.passport.json` |
| operator truth | `xtask/src/family/inventory.rs`, `xtask/src/family/coverage.rs`, `xtask/src/lib.rs` |
| docs and release note | `semantic-families/README.md`, `CHANGELOG.md` |

Reference anchors, not default edit targets:

- `examples/shared-spec/units/money/round.unit.spec`
- `examples/ecommerce/units/money/round.unit.spec`

Escalation-only surface:

- `spec-cli/src/*.rs`
- `xtask/src/family/recommend.rs`

If escalation-only files become necessary, stop, record proof, and reopen orchestration rather than silently widening scope.

## Locked Implementation Order

```text
authored helper unit (.unit.spec)
            |
            v
spec-core/src/semantic_review.rs
  SupportedFunctionRoute::HelperIdentityPassthrough
            |
            +--> spec-cli passport / status / export truth
            |
            +--> xtask inventory runtime_supported_routes
            |
            +--> xtask coverage supported_unpromoted counts
            |
            `--> xtask recommend stops surfacing helper unsupported pressure
```

Execution order is fixed:

1. Runtime route in `spec-core` sets the contract.
2. Read-side proof surfaces in `spec-cli` tests plus canonical passport.
3. `xtask` operator truth refresh.
4. Docs and changelog last.

Steps 2 and 3 may run in parallel only after Step 1 is frozen.

## Execution Topology

Canonical paths:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m41_helper_surface_semantic_review_substrate`

Worktree layout:

| Role | Branch | Worktree | Owner | Status |
|---|---|---|---|---|
| primary execution lane | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | always authoritative |
| optional read-side lane | `codex/m41-read-side-proof` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/read-side` | worker `W1` (`GPT-5.4`, `reasoning_effort=high`) or parent | starts only after Gate 20 |
| optional xtask lane | `codex/m41-xtask-operator-truth` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/xtask` | worker `W2` (`GPT-5.4`, `reasoning_effort=high`) or parent | starts only after Gate 20 |
| optional staging lane | `codex/m41-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/int` | parent only | optional rehearsal only |

Topology rules:

- The parent stays on `feat/m40-plus`.
- Optional worker branches fork from the exact `runtime_contract_freeze_commit` recorded in `runtime-contract-freeze.json`.
- Worker branches never merge directly into `feat/m40-plus`.
- The parent cherry-picks or manually integrates worker diffs through the parent-controlled lane.
- The optional `int` worktree is disposable staging only. Final accepted integration still lands through the parent on `feat/m40-plus`.
- If the split proves dishonest, close worker lanes and continue sequentially on `feat/m40-plus`.

## Canonical Run-State

`RUN_ROOT` is execution evidence only. The parent owns it.

Required kickoff artifacts:

- `baseline.json`
- `authority-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`

Required freeze artifact:

- `runtime-contract-freeze.json`

Required validation artifacts during execution:

- `validation/spec-core-semantic-review.stdout.txt`
- `validation/spec-cli-cli.stdout.txt`
- `validation/xtask-tests.stdout.txt`
- `validation/shared-spec-round-unit-test.stdout.txt`
- `validation/shared-spec-status.json`
- `validation/shared-spec-export.json`
- `validation/family-inventory.json`
- `validation/family-coverage.json`
- `validation/family-recommend.json`
- `validation/diff-scope.txt`

Conditional escalation artifact:

- `validation/spec-cli-prod-bug-evidence.md` if `spec-cli/src/*.rs` changes become necessary

Required closeout or blocked artifacts:

- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination

Minimum required contents:

- `baseline.json`
  - branch
  - `HEAD` SHA
  - dirty-state summary
  - in-scope file list checksum or captured path
  - whether `PLAN.md` differs from `HEAD`
- `authority-freeze.json`
  - authority path
  - hard guards
  - locked outcome contract
  - in-scope file set
- `runtime-contract-freeze.json`
  - `runtime_contract_freeze_commit`
  - frozen route marker
  - frozen compatibility key
  - frozen matcher constraints
  - parent-reserved files
  - whether parallel split is valid
- `blocked.json`
  - failed workstream
  - failed gate
  - branch
  - `HEAD` SHA
  - blocking evidence
  - restart point
  - whether worker lanes were invalidated

## Queue And Gates

| Order | ID | Kind | Owner | Success outputs |
|---|---|---|---|---|
| 1 | `gate-m41-00-baseline-freeze` | gate | parent | `baseline.json`, baseline test outputs, `run-state.json` |
| 2 | `gate-m41-05-authority-freeze` | gate | parent | `authority-freeze.json`, `queue.json`, `tasks.json` |
| 3 | `task-m41-10-runtime-helper-route` | task | parent | source changes, `session-log.md` |
| 4 | `gate-m41-20-runtime-contract-lock` | gate | parent | `runtime-contract-freeze.json`, `validation/spec-core-semantic-review.stdout.txt` |
| 5 | `task-m41-30-read-side-proof-refresh` | task | worker `W1` or parent | lane diff or integrated read-side changes |
| 6 | `task-m41-35-xtask-operator-truth-refresh` | task | worker `W2` or parent | lane diff or integrated `xtask` changes |
| 7 | `gate-m41-40-parent-integration` | gate | parent | integrated tree, `validation/diff-scope.txt` |
| 8 | `gate-m41-50-full-proof-loop` | gate | parent | all validation artifacts captured and green |
| 9 | `task-m41-55-docs-changelog` | task | parent | docs and changelog changes only |
| 10 | `gate-m41-60-closeout` | gate | parent | `acceptance.md`, `closeout.md`, final `run-state.json` |

Queue rules:

- Gates never overlap.
- `task-m41-30-read-side-proof-refresh` and `task-m41-35-xtask-operator-truth-refresh` may overlap only after Gate 20 passes.
- `task-m41-55-docs-changelog` never starts before Gate 50 passes.
- If Gate 20 does not produce a stable runtime route contract, there is no split. Continue sequentially under the parent.

## Workstream Plan

### WS-ROUTE (`feat/m40-plus`) — parent only

Workstream purpose:

- land the one new supported helper route
- freeze the contract honestly before any split
- keep scope inside `spec-core/src/semantic_review.rs`

Task ownership:

| Task ID | Parent-owned files | Required outcome |
|---|---|---|
| `task-m41-10-runtime-helper-route` | `spec-core/src/semantic_review.rs` | `HelperIdentityPassthrough` exists, matches the locked helper shape, and proves supported verdict coverage |
| `gate-m41-20-runtime-contract-lock` | same file plus run artifacts | route marker, compatibility key, matcher constraints, and routing order are frozen |

Kickoff commands for Gate 00:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli
cargo test -p xtask
```

Authority freeze commands for Gate 05:

```bash
git rev-parse HEAD
rg -n "HelperIdentityPassthrough|function.helper.identity_passthrough.v1|supported_unpromoted|helper_surface_not_promotable|spec-cli/src|recommend.rs" /Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md
```

Runtime-route validation commands for Gate 20:

```bash
cargo test -p spec-core semantic_review
git rev-parse HEAD
```

WS-ROUTE acceptance:

- `HelperIdentityPassthrough` is added to the supported route enum.
- `function.helper.identity_passthrough.v1` is the frozen compatibility key.
- The matcher stays narrow:
  - zero deps
  - exactly one input
  - input type `Decimal`
  - return type `Decimal`
  - identity/passthrough helper intent only
- The route is appended at the end of supported routing order, immediately before unsupported fallback.
- Tests cover:
  - aligned
  - under specified
  - semantic drift
  - unsupported near miss
- No other files are edited in this workstream.

### Split Preconditions

The optional split is valid only if all of the following are true at Gate 20:

- `runtime-contract-freeze.json` records a stable route marker, key, and matcher contract.
- The parent explicitly reserves:
  - `spec-cli/src/*.rs`
  - `xtask/src/family/recommend.rs`
  - `semantic-families/README.md`
  - `CHANGELOG.md`
- Worker lanes can keep their validation green without editing reserved files.
- The parent confirms that read-side and `xtask` lanes do not overlap on owned files.

If any one of these is false, do not branch workers. Continue sequentially on `feat/m40-plus`.

### WS-READ (`codex/m41-read-side-proof`) — optional worker lane after Gate 20

Workstream purpose:

- refresh read-side proof surfaces only
- prove canonical helper truth in passport, status, and export surfaces
- avoid production `spec-cli` code unless tests prove a real bug

Exact file ownership:

- `spec-cli/tests/cli.rs`
- `examples/shared-spec/units/money/round.spec.passport.json`

Forbidden files:

- `spec-core/src/semantic_review.rs`
- `spec-cli/src/*.rs`
- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `CHANGELOG.md`

Lane start commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate
git worktree add -b codex/m41-read-side-proof /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/read-side <runtime_contract_freeze_commit>
```

Lane validation commands:

```bash
cargo test -p spec-cli --test cli
cargo run -p spec-cli -- test examples/shared-spec/units/money/round.unit.spec
cargo run -p spec-cli -- status examples/shared-spec --format json
cargo run -p spec-cli -- export examples/shared-spec --format json
```

WS-READ acceptance:

- `spec-cli/tests/cli.rs` proves fresh helper proof survives preserve/status/export projection.
- `examples/shared-spec/units/money/round.spec.passport.json` refreshes to supported helper truth with the frozen compatibility key.
- Fresh supported helper truth is visible in:
  - passport
  - status JSON
  - export JSON
- Existing stale behavior remains intact for the helper route exactly the way current supported routes behave.
- If lane work discovers production `spec-cli/src/*.rs` must change, the lane stops immediately, writes `validation/spec-cli-prod-bug-evidence.md`, and returns control to the parent.

### WS-XTASK (`codex/m41-xtask-operator-truth`) — optional worker lane after Gate 20

Workstream purpose:

- publish runtime-supported helper truth in operator surfaces
- retire helper unsupported pressure by reclassification
- keep promoted packet counts unchanged

Exact file ownership:

- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs`

Forbidden files:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/src/*.rs`
- `examples/shared-spec/units/money/round.spec.passport.json`
- `xtask/src/family/recommend.rs`
- `semantic-families/README.md`
- `CHANGELOG.md`

Lane start commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate
git worktree add -b codex/m41-xtask-operator-truth /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/xtask <runtime_contract_freeze_commit>
```

Lane validation commands:

```bash
cargo test -p xtask
cargo xtask family inventory --format json
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

WS-XTASK acceptance:

- Inventory publishes the helper route as runtime-supported and supported-unpromoted.
- Coverage moves current helper units out of unsupported pressure and into supported-unpromoted truth.
- Recommendation output no longer surfaces `helper_surface_not_promotable` for the current helper wedge.
- Promoted family counts remain unchanged.
- No new coverage class or fake packet metadata is introduced.
- If `xtask` proof shows `recommend.rs` must change, the lane stops and returns a blocker to the parent rather than widening scope on its own.

### WS-INT-DOCS (`feat/m40-plus`, optional `codex/m41-int` staging) — parent only

Workstream purpose:

- integrate worker output or sequentially finish remaining code
- run the full proof loop
- update docs and changelog last
- close the milestone

Parent-owned files in this workstream:

- any accepted in-scope files from prior workstreams
- `semantic-families/README.md`
- `CHANGELOG.md`
- all `RUN_ROOT` artifacts

Optional staging worktree creation:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate
git worktree add -b codex/m41-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m41-helper-substrate/int <runtime_contract_freeze_commit>
```

Integration commands for Gate 40:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli
cargo test -p xtask
git status --short
git rev-parse HEAD
```

Full proof-loop commands for Gate 50:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli
cargo test -p xtask
cargo run -p spec-cli -- test examples/shared-spec/units/money/round.unit.spec
cargo run -p spec-cli -- status examples/shared-spec --format json
cargo run -p spec-cli -- export examples/shared-spec --format json
cargo xtask family inventory --format json
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

Docs-last acceptance for `task-m41-55-docs-changelog`:

- `semantic-families/README.md` explains that the helper wedge is now supported substrate truth and still unpromoted.
- `CHANGELOG.md` records the semantic-review, read-side, and operator-surface truth change.
- Docs do not imply generic helper understanding or packet promotion.

WS-INT-DOCS closeout acceptance:

- Final accepted diff stays inside the locked file surface unless an escalation artifact justifies one narrow exception.
- `validation/diff-scope.txt` shows no unrelated touched files.
- Worker lanes are integrated or discarded by the parent only.
- Docs were authored after the code and proof were settled.

## Context-Control Rules

- The parent keeps only the following live authority context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `queue.json`
  - `runtime-contract-freeze.json`
  - latest diff summary
  - latest validation artifact index
- Each worker receives only:
  - exact owned file set
  - relevant `PLAN.md` excerpt
  - frozen `runtime_contract_freeze_commit`
  - frozen route marker and key
  - required commands
  - forbidden files
  - acceptance conditions
- Workers return only:
  - changed files
  - commands run and exit codes
  - blockers
  - unresolved assumptions
- Workers do not write `RUN_ROOT/*`.
- The parent reviews narrow diffs and validation summaries only, not full worker transcripts.
- Close worker lanes immediately after integration or cancellation.
- If the parent changes the frozen route contract after Gate 20, all worker lanes become stale.

## Blocked And Restart Semantics

- Failure before Gate 20 restarts from the last passing parent gate.
- If Gate 20 cannot freeze a stable helper route contract, there is no split. Continue sequentially on `feat/m40-plus`.
- If WS-READ discovers `spec-cli/src/*.rs` changes are required, invalidate WS-READ as a worker lane, record `validation/spec-cli-prod-bug-evidence.md`, and resume read-side work sequentially under the parent.
- If WS-XTASK discovers `xtask/src/family/recommend.rs` changes are required, stop and rewrite the orchestration plan before continuing. Do not silently widen the file surface.
- If a worker touches a forbidden file, invalidate that lane and restart the affected work under the parent from Gate 20.
- If `PLAN.md` changes after Gate 05, restart from Gate 00.
- If `feat/m40-plus` `HEAD` changes after Gate 20 but before worker integration, invalidate worker lanes and restart from Gate 20.
- If the frozen route marker, compatibility key, or matcher constraints change after workers start, all worker lanes become stale.
- If Gate 50 proves the helper route overmatches, reopen WS-ROUTE only.
- If Gate 50 proves read-side truth diverges, reopen WS-READ only.
- If Gate 50 proves operator truth diverges, reopen WS-XTASK only.
- If docs disagree with landed truth, reopen WS-INT-DOCS only.
- If any proof suggests packet-promotion logic, helper taxonomy expansion, or schema work, stop and rewrite this plan.

## Tests And Acceptance

Mandatory regression guards:

| ID | Location | Required assertion |
|---|---|---|
| TR-1 | `spec-core/src/semantic_review.rs` tests | helper route proves aligned, under specified, semantic drift, and unsupported near-miss outcomes |
| TR-2 | `spec-cli/tests/cli.rs` | fresh helper passport truth survives preserve/status/export projection |
| TR-3 | `examples/shared-spec/units/money/round.spec.passport.json` | canonical checked-in helper proof uses `function.helper.identity_passthrough.v1` and supported helper truth |
| TR-4 | `xtask/src/family/inventory.rs` and `xtask/src/lib.rs` tests | inventory lists the helper route as runtime-supported and supported-unpromoted |
| TR-5 | `xtask/src/family/coverage.rs` and `xtask/src/lib.rs` tests | coverage and recommendation outputs retire helper unsupported pressure without changing promoted-family counts |
| TR-6 | scope audit | `spec-cli/src/*.rs` remains unchanged unless escalation proof exists |

Mandatory final commands:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli
cargo test -p xtask
cargo run -p spec-cli -- test examples/shared-spec/units/money/round.unit.spec
cargo run -p spec-cli -- status examples/shared-spec --format json
cargo run -p spec-cli -- export examples/shared-spec --format json
cargo xtask family inventory --format json
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

Mandatory final acceptance:

- `spec-core` owns the helper route contract.
- Read-side surfaces show supported helper truth for fresh canonical proof.
- `xtask` surfaces show runtime-supported, supported-unpromoted helper truth.
- No new packet exists.
- No new schema or CLI surface exists.
- Production `spec-cli` code is unchanged unless a captured escalation artifact proves a real read-side bug.
- `helper_surface_not_promotable` is no longer live unsupported pressure for the current helper wedge.
- Promoted family counts are unchanged.

Coverage expectation:

- Current snapshot is `28 / 17 / 0 / 11`.
- Expected post-M41 snapshot is `28 / 17 / 3 / 8`.

If corpus shape shifts before landing, preserve the semantic delta instead of stale absolute numbers:

- `promoted_family_units` unchanged
- `supported_unpromoted_family_units` increases by the retired helper-unit count
- `unsupported_function_units` decreases by the same count

## Closeout Behavior

- The parent writes `acceptance.md` with:
  - gates passed
  - commands run
  - final semantic delta
  - any escalation artifacts
- The parent writes `closeout.md` with:
  - final changed file list
  - final proof artifact list
  - whether work ran sequentially or with honest split lanes
  - any deferred follow-on concerns that remain explicitly out of scope
- The parent updates `run-state.json` to terminal success or blocked state.
- Worker worktrees are removed or archived only after parent integration is complete.
- No closeout note may describe M41 as packet promotion.

## Assumptions

- `PLAN.md` remains the authority for M41 during execution.
- The primary worktree at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` remains the sole authoritative lane.
- The helper route contract can be frozen inside `spec-core/src/semantic_review.rs` without touching additional runtime files.
- Read-side proof refresh can stay confined to `spec-cli/tests/cli.rs` plus the canonical passport unless a real bug is proven.
- Existing `xtask` supported-unpromoted plumbing is sufficient to publish the helper route without inventing new categories.
- The `WT_ROOT` paths may be created if absent.
- Unrelated dirty worktree changes may exist and must not be reverted.

## Critical Assumptions

- The only honest parallel window is after the runtime route contract is frozen.
- Docs and changelog are last, not concurrent.
- Parent-only ownership of integration and gates is non-negotiable.
- Any need to widen beyond the locked file surface is a replanning event, not a silent implementation detail.
