# M43 Orchestration Plan

Status: **authoritative kickoff and execution contract for M43 `function.helper.identity_passthrough.v1` promotion**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Execute from current branch: **`feat/m40-plus`**  
Last rewritten: **`2026-05-09`**

## Summary

- Execute from the current branch `feat/m40-plus`. That branch is the live baseline for this run.
- Keep the critical path local to the parent agent for:
  - contract freeze
  - parent integration in `ws/m43-int`
  - the final proof wall
- Use subagents only for the three disjoint post-freeze lanes:
  - scaffold
  - packet
  - runtime regression
- Worker concurrency cap is **3**.
- Worker model assumption is fixed for all three worker lanes:
  - `model = GPT-5.4`
  - `reasoning_effort = high`
- Use dedicated worktrees and branches:
  - `ws/m43-contract` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/contract`
  - `ws/m43-scaffold` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/scaffold`
  - `ws/m43-packet` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/packet`
  - `ws/m43-core` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/core`
  - `ws/m43-int` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/int`
- Keep orchestration state in one canonical parent-owned location:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `M43_RUN_ROOT=$PRIMARY_ROOT/.runs/m43_helper_identity_passthrough_promotion`
  - `queue=$M43_RUN_ROOT/tasks.json`
  - `session_log=$M43_RUN_ROOT/session-log.md`
  - `contract_freeze=$M43_RUN_ROOT/contract-freeze.json`
  - per-task sentinels under `$PRIMARY_ROOT/.runs/task-m43-*/`
- Treat authored source, run-state artifacts, and derived proof artifacts as separate classes:
  - authored source is the milestone deliverable
  - `.runs/**` is parent-owned execution state only
  - `.semantic-family-artifacts/**` is derived proof output only

## Hard Guards

- Lock the supported helper-family contract exactly to:
  - `fn_name == round`
  - exactly one Decimal input
  - Decimal return
  - no deps
  - no invariants
  - no control flow
- Supported cases cannot introduce deps, invariants, or control flow.
- Aligned truth must prove both supported lanes:
  - round-like intent plus round-like body
  - passthrough intent plus direct-passthrough body
- Drift stays:
  - passthrough intent plus round-like body
- Under-specified stays:
  - vague intent plus otherwise-supported body
- Unsupported-near-miss stays:
  - control-flow branch around an otherwise helper-shaped body
- Starter scaffolds must remain valid-but-non-proving.
- No recommendation-policy work.
- No shared-core portability work.
- No second-language prove or certify work.
- No widening helper semantic support beyond the current honest subset.
- No worker lane may edit `ORCH_PLAN.md`.
- No worker lane may touch `.runs/**`.
- No worker lane may treat `.semantic-family-artifacts/**` as authored output.
- No worker lane may return derived proof artifacts as deliverables.
- Parent is the sole author of:
  - `M43_RUN_ROOT/**`
  - per-task sentinel status files
  - `acceptance.md`
  - `closeout.md`
  - `blocked.json`
  - merge log and scope log artifacts
- Workers may inspect local derived proof output created by their validation commands, but they may not preserve or publish those surfaces as authoritative run artifacts.
- `PLAN.md` wins over this file, worker summaries, and any lane-local assumptions if they disagree.

## Execution Topology

| Role | Branch | Worktree | Owner | Scope |
|---|---|---|---|---|
| primary baseline | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | authority, run-state, final landing |
| contract freeze | `ws/m43-contract` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/contract` | parent | `harness.rs` only |
| scaffold | `ws/m43-scaffold` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/scaffold` | worker A | scaffold and family-new tests |
| packet | `ws/m43-packet` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/packet` | worker B | packet source only |
| core regression | `ws/m43-core` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/core` | worker C | `spec-core` regression only |
| integration | `ws/m43-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/int` | parent | merge, projections, proof wall |

Rules:

- Worker branches must fork from the exact `contract_freeze_commit` recorded in `contract-freeze.json`.
- No worker may fork from stale `feat/m40-plus` HEAD.
- Parent is the sole integrator.
- Parent merges worker lanes into `ws/m43-int`.
- Parent resolves only textual conflicts locally.
- Any semantic disagreement on helper subset, packet semantics, starter semantics, or aligned-lane truth is:
  - bounced back to the owning lane, or
  - resolved by applying the frozen contract literally

## Canonical Run-State And Artifact Surfaces

### Authored source

Only these surfaces are in-bounds authored-source deliverables:

- `xtask/src/family/harness.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/lib.rs`
- `spec-core/src/semantic_review.rs`
- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `semantic-families/function.helper.identity_passthrough.v1/**`

### Parent-owned run-state artifacts

Canonical parent-owned run root:

- `M43_RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion`

Required parent-owned artifacts:

- `baseline.json`
- `authority-freeze.json`
- `contract-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`
- `merge-log.md`
- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination
- `validation/**`

### Per-task sentinels

Each task or gate gets a sentinel directory under `.runs/`:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-05-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-a1-contract-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-b-scaffold/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-c-packet/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-d-core-regression/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-e-integration/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-50-proof-wall/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m43-60-closeout/`

Each sentinel directory may contain:

- `started.json`
- `status.json`
- `done.json`
- `blocked.json`

### Derived proof artifacts

Derived proof artifacts are not authored source:

- `.semantic-family-artifacts/semantic-families/function.helper.identity_passthrough.v1/prove.latest.json`
- `.semantic-family-artifacts/semantic-families/function.helper.identity_passthrough.v1/attempt-*.json`
- `.semantic-family-artifacts/semantic-families/function.helper.identity_passthrough.v1/certification.report.json`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`

## Queue And Gates

| Order | ID | Kind | Owner | Worktree |
|---|---|---|---|---|
| 1 | `task-m43-00-baseline` | gate | parent | primary |
| 2 | `task-m43-05-authority-freeze` | gate | parent | primary |
| 3 | `task/m43-a1-contract-freeze` | task | parent | `ws/m43-contract` |
| 4 | `gate-m43-15-worker-launch` | gate | parent | primary |
| 5 | `task/m43-b-scaffold` | task | worker A | `ws/m43-scaffold` |
| 6 | `task/m43-c-packet` | task | worker B | `ws/m43-packet` |
| 7 | `task/m43-d-core-regression` | task | worker C | `ws/m43-core` |
| 8 | `task/m43-e-integration` | task | parent | `ws/m43-int` |
| 9 | `task-m43-50-proof-wall` | gate | parent | `ws/m43-int` |
| 10 | `task-m43-60-closeout` | gate | parent | primary |

## Workstream Plan

### `task/m43-a1-contract-freeze` — parent only

Purpose:

- freeze the helper-family contract before any worker branches exist

Owned files and directories:

- `xtask/src/family/harness.rs`

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/contract -b ws/m43-contract feat/m40-plus
cargo test -p xtask -- --color never
git status --short
```

Acceptance:

- `function.helper.identity_passthrough.v1` is registered in `harness.rs`
- routing and ownership are frozen
- the frozen contract explicitly encodes:
  - `fn_name == round`
  - `precedence = 5`
  - `must_not_shadow = ["unsupported.function.v1"]`
  - both aligned lanes are required truth
  - starters remain valid-but-non-proving
- no scaffold, packet, `spec-core`, inventory, or coverage work is mixed into this lane

### `task/m43-b-scaffold` — worker A

Purpose:

- add truthful helper-family scaffold support and keep starter generation honest

Owned files and directories:

- `xtask/src/family/scaffold.rs`
- scaffold and family-new tests in `xtask/src/lib.rs`

Required commands:

```bash
cargo test -p xtask -- --color never
cargo xtask family new function.helper.identity_passthrough.v1
git status --short
```

Lane rule:

- `cargo xtask family new function.helper.identity_passthrough.v1` is local validation only in the disposable scaffold worktree
- generated packet files and any derived proof output must not be part of the lane handoff

Acceptance:

- starter generation works for the helper family
- starter files remain valid-but-non-proving
- starter path shape stays locked under `fixtures/<bucket>/units/money/round.unit.spec`
- `xtask/src/lib.rs` edits are limited to scaffold and family-new tests only
- no packet files are authored in this lane

### `task/m43-c-packet` — worker B

Purpose:

- author the committed packet that the parent proof wall will defend

Owned files and directories:

- `semantic-families/function.helper.identity_passthrough.v1/**`

Required commands:

```bash
cargo xtask family smoke function.helper.identity_passthrough.v1
git status --short
```

Acceptance:

- packet is self-contained
- packet contains all four buckets
- aligned bucket proves both supported lanes
- committed proving fixtures respect `fn_name == round`
- no cross-packet source or xtask source is modified

### `task/m43-d-core-regression` — worker C

Purpose:

- add the missing direct-passthrough aligned regression without widening runtime semantics

Owned files and directories:

- `spec-core/src/semantic_review.rs`

Required commands:

```bash
cargo test -p spec-core helper_identity_passthrough -- --color never
git status --short
```

Acceptance:

- direct-passthrough aligned support is explicitly proven
- round-like aligned support remains proven
- drift, under-specified, and unsupported-near-miss coverage remain proven
- no packet or xtask files are changed

### `task/m43-e-integration` — parent only

Purpose:

- merge all worker lanes into `ws/m43-int`
- refresh read-side promotion truth
- run the full proof wall
- capture scope and acceptance evidence

Owned files and directories:

- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs` only if projection follow-up is forced
- merge mechanics in already-owned files

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/int -b ws/m43-int ws/m43-contract
git merge --no-ff ws/m43-packet
git merge --no-ff ws/m43-core
git merge --no-ff ws/m43-scaffold
git merge-base --is-ancestor "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/contract-freeze.json)" HEAD
cargo test -p xtask inventory -- --color never
cargo test -p xtask coverage -- --color never
cargo test -p xtask -- --color never
cargo test -p spec-core helper_identity_passthrough -- --color never
cargo xtask family smoke function.helper.identity_passthrough.v1
cargo xtask family prove function.helper.identity_passthrough.v1
cargo xtask family certify function.helper.identity_passthrough.v1
cargo xtask family inventory --format json
cargo xtask family coverage --format json
git diff --name-only --no-renames "$(jq -r '.baseline_head' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/baseline.json)"..HEAD
git diff --stat --no-renames "$(jq -r '.baseline_head' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/baseline.json)"..HEAD
```

Acceptance:

- worker lanes merge into `ws/m43-int`
- parent resolves only textual conflicts locally
- any semantic disagreement is bounced back or resolved by the frozen contract literally
- inventory and coverage flip from supported-unpromoted to promoted truth after green proof
- final diff remains in-bounds

## Gate And Task Procedures

### `task-m43-00-baseline`

Purpose:

- capture the real starting branch, head, workspace state, and pre-promotion projection truth

Owned files:

- no source files
- parent-owned run artifacts only

Exact commands:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
cargo xtask family inventory --format json
cargo xtask family coverage --format json
```

Artifacts written:

- `M43_RUN_ROOT/baseline.json`
- `M43_RUN_ROOT/validation/baseline-family-inventory.json`
- `M43_RUN_ROOT/validation/baseline-family-coverage.json`
- `.runs/task-m43-00-baseline/{started.json,status.json,done.json}`

Blocked conditions:

- current branch is not `feat/m40-plus`
- repo is in a state that prevents authoritative baseline capture
- baseline commands fail

Restart point if blocked:

- restart from `task-m43-00-baseline` after baseline conditions are repaired

### `task-m43-05-authority-freeze`

Purpose:

- freeze the exact M43 authority basis and scope before code edits

Owned files:

- no source files
- parent-owned run artifacts only

Exact commands:

```bash
rg -n "M43|Accepted Scope|Not In Scope|Packet Contract|Implementation Plan|Worktree Parallelization Strategy" PLAN.md
git status --short
```

Artifacts written:

- `M43_RUN_ROOT/authority-freeze.json`
- `M43_RUN_ROOT/in-scope-files.txt`
- `M43_RUN_ROOT/tasks.json`
- `M43_RUN_ROOT/queue.json`
- `M43_RUN_ROOT/run-state.json`
- `M43_RUN_ROOT/session-log.md`
- `.runs/task-m43-05-authority-freeze/{started.json,status.json,done.json}`

Blocked conditions:

- M43 authority cannot be extracted cleanly from `PLAN.md`
- in-scope surface is unclear

Restart point if blocked:

- restart from `task-m43-05-authority-freeze`

### `task/m43-a1-contract-freeze`

Purpose:

- land the one global gate before any parallel work starts

Owned files:

- `xtask/src/family/harness.rs`

Exact commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/contract -b ws/m43-contract feat/m40-plus
cargo test -p xtask -- --color never
git status --short
git rev-parse HEAD
```

Artifacts written:

- `M43_RUN_ROOT/contract-freeze.json`
- `M43_RUN_ROOT/validation/contract-freeze-xtask.stdout.txt`
- `.runs/task-m43-a1-contract-freeze/{started.json,status.json,done.json}`

Blocked conditions:

- harness registration cannot be made truthful without widening scope
- xtask contract tests fail after the harness change
- the lane needs scaffold, packet, runtime, or projection edits to finish honestly

Restart point if blocked:

- restart from `task/m43-a1-contract-freeze`
- no worker branches may be launched until this task is green

### `gate-m43-15-worker-launch`

Purpose:

- create the worker lanes from the frozen contract commit only

Owned files:

- no source files
- run-state artifacts only

Exact commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/scaffold -b ws/m43-scaffold "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/contract-freeze.json)"
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/packet -b ws/m43-packet "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/contract-freeze.json)"
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/core -b ws/m43-core "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/contract-freeze.json)"
```

Artifacts written:

- `M43_RUN_ROOT/validation/worker-launch.txt`
- `.runs/task-m43-a1-contract-freeze/status.json` update
- lane prompt handoff records if the operator uses them

Blocked conditions:

- any worker branch would fork from something other than `contract_freeze_commit`
- worktree creation fails

Restart point if blocked:

- restart from `gate-m43-15-worker-launch`

### `task/m43-b-scaffold`

Purpose:

- prove scaffold support and starter honesty

Owned files:

- `xtask/src/family/scaffold.rs`
- limited scaffold and family-new tests in `xtask/src/lib.rs`

Exact commands:

```bash
cargo test -p xtask -- --color never
cargo xtask family new function.helper.identity_passthrough.v1
git status --short
```

Artifacts written:

- worker summary only
- `.runs/task-m43-b-scaffold/{started.json,status.json,done.json}` written by parent after handoff review

Blocked conditions:

- scaffold support requires widening starter semantics
- scaffold support requires changing packet source or runtime source
- `xtask/src/lib.rs` edits escape scaffold and family-new tests

Restart point if blocked:

- restart from `task/m43-b-scaffold` after parent clarifies the frozen contract

### `task/m43-c-packet`

Purpose:

- commit the packet truth that smoke, prove, and certify will defend

Owned files:

- `semantic-families/function.helper.identity_passthrough.v1/**`

Exact commands:

```bash
cargo xtask family smoke function.helper.identity_passthrough.v1
git status --short
```

Artifacts written:

- worker summary only
- `.runs/task-m43-c-packet/{started.json,status.json,done.json}` written by parent after handoff review

Blocked conditions:

- packet truth requires changing the frozen helper subset
- packet truth requires scaffold widening first
- smoke cannot pass because packet semantics disagree with the frozen contract

Restart point if blocked:

- restart from `task/m43-c-packet` after parent resolves the contract issue
- if the issue is semantic, bounce back to contract freeze rather than improvising packet truth

### `task/m43-d-core-regression`

Purpose:

- prove direct-passthrough aligned support in runtime tests

Owned files:

- `spec-core/src/semantic_review.rs`

Exact commands:

```bash
cargo test -p spec-core helper_identity_passthrough -- --color never
git status --short
```

Artifacts written:

- worker summary only
- `.runs/task-m43-d-core-regression/{started.json,status.json,done.json}` written by parent after handoff review

Blocked conditions:

- runtime proof requires changing packet semantics or scaffold semantics
- runtime proof implies a wider helper classifier than the frozen contract permits

Restart point if blocked:

- restart from `task/m43-d-core-regression`
- if blocked by semantic disagreement, return to the owning lane or the frozen contract

### `task/m43-e-integration`

Purpose:

- merge worker lanes into `ws/m43-int`
- refresh projection truth
- prepare the final proof wall

Owned files:

- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs` only if projection follow-up is forced
- merge mechanics only in other in-scope files

Exact commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m43/int -b ws/m43-int ws/m43-contract
git merge --no-ff ws/m43-packet
git merge --no-ff ws/m43-core
git merge --no-ff ws/m43-scaffold
git merge-base --is-ancestor "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/contract-freeze.json)" HEAD
git status --short
cargo test -p xtask inventory -- --color never
cargo test -p xtask coverage -- --color never
cargo test -p xtask -- --color never
cargo test -p spec-core helper_identity_passthrough -- --color never
```

Artifacts written:

- `M43_RUN_ROOT/merge-log.md`
- `M43_RUN_ROOT/validation/post-merge-status.txt`
- `M43_RUN_ROOT/validation/post-merge-xtask-inventory.stdout.txt`
- `M43_RUN_ROOT/validation/post-merge-xtask-coverage.stdout.txt`
- `M43_RUN_ROOT/validation/post-merge-xtask-full.stdout.txt`
- `M43_RUN_ROOT/validation/post-merge-spec-core.stdout.txt`
- `.runs/task-m43-e-integration/{started.json,status.json,done.json}`

Blocked conditions:

- non-textual merge conflict
- semantic disagreement on helper subset, packet semantics, starter semantics, or aligned-lane truth
- inventory or coverage updates require forbidden surfaces

Restart point if blocked:

- restart from `task/m43-e-integration` after bouncing the conflict back to the owning lane
- if the conflict is really a contract disagreement, restart from `task/m43-a1-contract-freeze`

### `task-m43-50-proof-wall`

Purpose:

- run the exact final proof wall and capture end-state truth

Owned files:

- no new authored-source ownership beyond integration-owned follow-up
- parent-owned validation artifacts only

Exact commands:

```bash
cargo test -p spec-core helper_identity_passthrough -- --color never
cargo test -p xtask inventory -- --color never
cargo test -p xtask coverage -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.helper.identity_passthrough.v1
cargo xtask family prove function.helper.identity_passthrough.v1
cargo xtask family certify function.helper.identity_passthrough.v1
cargo xtask family inventory --format json
cargo xtask family coverage --format json
git diff --name-only --no-renames "$(jq -r '.baseline_head' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/baseline.json)"..HEAD
git diff --stat --no-renames "$(jq -r '.baseline_head' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m43_helper_identity_passthrough_promotion/baseline.json)"..HEAD
```

Artifacts written:

- `M43_RUN_ROOT/validation/spec-core-helper_identity_passthrough.stdout.txt`
- `M43_RUN_ROOT/validation/xtask-inventory-test.stdout.txt`
- `M43_RUN_ROOT/validation/xtask-coverage-test.stdout.txt`
- `M43_RUN_ROOT/validation/xtask-full.stdout.txt`
- `M43_RUN_ROOT/validation/family-smoke.stdout.txt`
- `M43_RUN_ROOT/validation/family-prove.stdout.txt`
- `M43_RUN_ROOT/validation/family-certify.stdout.txt`
- `M43_RUN_ROOT/validation/family-inventory.json`
- `M43_RUN_ROOT/validation/family-coverage.json`
- `M43_RUN_ROOT/validation/final-diff-name-only.txt`
- `M43_RUN_ROOT/validation/final-diff-stat.txt`
- `.runs/task-m43-50-proof-wall/{started.json,status.json,done.json}`

Blocked conditions:

- any proof-wall command fails
- inventory still reports `function.helper.identity_passthrough.v1` as supported-unpromoted
- coverage still counts helper units inside `supported_unpromoted_family_units`
- final diff escapes the in-scope surface

Restart point if blocked:

- restart from `task/m43-e-integration` if the failure is integration or projection related
- restart from the owning worker task if the failure localizes to one lane
- if the failure implies widened scope, stop and write `blocked.json`

### `task-m43-60-closeout`

Purpose:

- finalize acceptance, scope proof, and run closeout

Owned files:

- parent-owned run artifacts only

Exact commands:

```bash
git status --short
git rev-parse HEAD
```

Artifacts written:

- `M43_RUN_ROOT/acceptance.md`
- `M43_RUN_ROOT/closeout.md`
- `.runs/task-m43-60-closeout/{started.json,status.json,done.json}`

Blocked conditions:

- proof wall is not green
- acceptance criteria are not all met
- final diff is out of bounds

Restart point if blocked:

- restart from `task-m43-50-proof-wall`

## Context-Control Rules

- Parent keeps only these live artifacts in working context:
  - `PLAN.md`
  - `M43_RUN_ROOT/tasks.json`
  - `M43_RUN_ROOT/contract-freeze.json`
  - lane summaries
  - latest integration diff summary
- Each worker prompt contains only:
  - owned files
  - exact relevant `PLAN.md` excerpt
  - frozen contract excerpt
  - required commands
  - forbidden surfaces
  - `contract_freeze_commit`
- Workers return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not write:
  - `M43_RUN_ROOT/**`
  - `.runs/task-m43-*/**`
  - `acceptance.md`
  - `closeout.md`
  - `blocked.json`
- Parent reviews narrow diffs and summaries only.
- Parent closes each worker after merge or bounce-back.

## Tests And Acceptance

### Harness and scaffold

- `xtask/src/family/harness.rs` registers `function.helper.identity_passthrough.v1` with the frozen routing and ownership contract.
- `xtask/src/family/scaffold.rs` generates truthful helper-family starter content.
- `xtask/src/lib.rs` edits from worker A are limited to scaffold and family-new tests unless the parent later needs narrow projection follow-up in integration.
- starter generation remains valid-but-non-proving
- starter paths remain locked under `fixtures/<bucket>/units/money/round.unit.spec`

### Packet

- `semantic-families/function.helper.identity_passthrough.v1/**` exists and is self-contained
- all four buckets are present
- aligned proves both supported lanes
- drift, under-specified, and unsupported-near-miss remain honest
- proving fixtures respect `fn_name == round`

### Runtime

- `cargo test -p spec-core helper_identity_passthrough -- --color never` passes
- direct-passthrough aligned support is explicitly proven
- round-like aligned support remains proven
- runtime tests do not imply a wider helper subset than the frozen contract

### Projections

- `cargo test -p xtask inventory -- --color never` passes
- `cargo test -p xtask coverage -- --color never` passes
- `cargo xtask family inventory --format json` no longer lists `function.helper.identity_passthrough.v1` in `supported_unpromoted_families[]`
- `cargo xtask family coverage --format json` no longer counts helper units in `supported_unpromoted_family_units`
- projection truth flips only after the green proof wall, not before

### Operator flow and workspace boundary

- parent owns contract freeze, integration, and final proof wall
- worker concurrency never exceeds 3
- workers stay inside owned files
- no worker edits `ORCH_PLAN.md`
- no worker touches `.runs/**`
- no worker returns `.semantic-family-artifacts/**` as authored output
- final diff proves the run stayed inside:
  - `xtask/src/family/harness.rs`
  - `xtask/src/family/scaffold.rs`
  - `xtask/src/lib.rs`
  - `spec-core/src/semantic_review.rs`
  - `xtask/src/family/inventory.rs`
  - `xtask/src/family/coverage.rs`
  - `semantic-families/function.helper.identity_passthrough.v1/**`

## Scope-Boundary Checks

Forbidden surfaces for the full run:

- `xtask/src/family/recommend.rs`
- recommendation-policy files
- shared-core portability surfaces
- second-language prove or certify surfaces
- unrelated docs or planning files
- `ORCH_PLAN.md` during worker lanes

Final scope proof must include:

- `M43_RUN_ROOT/validation/final-diff-name-only.txt`
- `M43_RUN_ROOT/validation/final-diff-stat.txt`

Any out-of-bounds diff is a blocked run, not a creative follow-up.

## Assumptions

- The live branch remains `feat/m40-plus`.
- `PLAN.md` remains the only milestone authority.
- The current repo already routes truthful helper-shaped unary Decimal functions to `function.helper.identity_passthrough.v1` at runtime.
- The missing work is promotion, scaffold, packet, regression, and read-side proof, not new runtime semantic support.
- `cargo xtask family smoke`, `prove`, `certify`, `inventory`, and `coverage` remain the maintained operator commands for this family-promotion run.
