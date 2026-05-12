# M52 Bounded Same-Tree Wrapper TypeScript Execution Orchestration Plan

Status: **authoritative orchestration plan for executing M52**
Supersedes: **the stale M51 `ORCH_PLAN.md`**
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Plan title: **`M52: Bounded Same-Tree Wrapper TypeScript Execution Implementation Plan`**
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**
Base branch: **`main`**
Primary execution branch: **`feat/m40-plus`**
Baseline authority date: **`2026-05-12`**
Worker model: **GPT-5.4 with `reasoning_effort=high`**
Maximum concurrency after contract freeze: **2 workers**
Last rewritten: **2026-05-12**

## Summary

- Execute from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`.
- `PLAN.md` is the sole scope authority. This file is the operator contract derived from it.
- M52 is a bounded second-language backend milestone. It widens the existing TypeScript executor to one new supported family-shaped slice:
  - `function.wrapper.pipeline.v1`
  - same-tree local dep closure only
- M52 is not:
  - cross-library TypeScript work
  - generic multi-dependency execution
  - chain3 execution
  - molecule TypeScript support
  - shared-core extraction
- Parent-owned pre-parallel work is the execution-contract freeze across:
  - `spec-core/src/validator.rs`
  - `spec-core/src/typescript_backend.rs`
  - `spec-cli/src/commands.rs`
- After that freeze, split at most two narrow lanes:
  - `WS-EXECUTOR`
  - `WS-AUTHORED-TRUTH`
- Parent remains the only integrator and the only writer of authoritative orchestration state.

## Hard Guards

- `PLAN.md` is the only scope authority for M52.
- The widened executor must remain family-shaped, not generic dep-count-shaped.
- The only new target family allowed is `function.wrapper.pipeline.v1`.
- `function.wrapper.pipeline.chain3.v1` remains out of scope.
- Cross-library TypeScript dep resolution remains out of scope.
- `.test.spec --target-language typescript` remains unsupported.
- Rust remains the default target and TypeScript proof remains additive.
- Bun remains the only TypeScript runtime/tooling contract.
- Docs must not claim generic multi-dep TypeScript support.

Abort and re-scope if any of these become necessary:

1. Cross-library resolution is required to make the canonical wrapper pass.
2. Chain3 is required for the first honest wrapper proof.
3. The validator must widen to arbitrary supported multi-dep graphs.
4. A new proof schema is required instead of reusing `target_proofs.typescript`.
5. A new package-manager or config-file contract is required beyond Bun.

## Current Code Truth And Rationale

- `spec test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript` already passes.
- `spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript` currently fails before Bun because the wrapper closure is validated under the old M46 root-only lane.
- `spec-core/src/typescript_backend.rs` still assumes every included unit in a TypeScript tree is either:
  - the monotone-up target root, or
  - a single helper dep
- `spec-core/src/semantic_review.rs` already supports `function.wrapper.pipeline.v1` with exact same-tree local dep semantics.
- `semantic-families/function.wrapper.pipeline.v1/candidate.md` already freezes the truthful wrapper family boundary.

M52 exists to make executor truth catch up to already-supported family truth, without widening the family boundary itself.

## Workstream Topology

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m52-parent-authority` | `WS-AUTHORITY` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | kickoff capture, contract freeze, worker fork, merge, final proof |
| `lane/m52-executor` | `WS-EXECUTOR` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/executor` | `ws/m52-executor` | Worker | validator, generator, CLI, and test harness widening |
| `lane/m52-authored-truth` | `WS-AUTHORED-TRUTH` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/authored-truth` | `ws/m52-authored-truth` | Worker | canonical example specs, packet fixtures, README/CHANGELOG/TODOS wording |
| `lane/m52-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/int` | `ws/m52-int` | Parent | merge worker lanes and run authoritative proof |

## Canonical Run Root

Use these exact paths:

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52
RUN_ROOT=$PRIMARY_ROOT/.runs/m52_same_tree_wrapper_typescript
```

All authoritative orchestration state lives under `RUN_ROOT`.

## Orchestration State

### File-Of-Record Inventory

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, and authority snapshot metadata | Parent |
| `contract-freeze.json` | exact M52 eligibility and non-goal contract | Parent |
| `worktrees.json` | worktree path and branch inventory | Parent |
| `file-ownership.json` | exact writable repo surfaces per lane | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | dependency queue and task state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance-ledger.md` | final acceptance evidence and signoff ledger | Parent |
| `validation/kickoff/*` | kickoff command captures | Parent |
| `validation/freeze/*` | contract-freeze captures | Parent |
| `validation/executor/*` | executor lane proof captures | Parent |
| `validation/authored-truth/*` | authored truth lane proof captures | Parent |
| `validation/int/*` | integrated proof captures | Parent |
| `validation/final/*` | final branch and acceptance captures | Parent |

### Task Status Vocabulary

Use only these statuses:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `cancelled`

## Kickoff Rule

Kickoff requires a clean execution tree.

Required kickoff commands:

```bash
mkdir -p "$RUN_ROOT"/validation/{kickoff,freeze,executor,authored-truth,int,final}
mkdir -p "$RUN_ROOT"/tasks

git branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Kickoff acceptance:

- branch is `feat/m40-plus`
- tree is clean or only contains the expected authority-doc edits before execution starts
- authority snapshots are captured before code edits

## Contract Freeze

Purpose: freeze the exact M52 execution contract before parallel work.

The freeze must record:

- root target families allowed for TypeScript execution
- exact wrapper dep contract:
  - exactly two direct deps
  - local only
  - same loaded unit set
  - same generated tree
- exact out-of-scope list:
  - cross-library
  - chain3
  - molecule tests
  - generic multi-dep execution
- writable file ownership per lane

Required freeze outputs:

- `contract-freeze.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`

## File Ownership

### `WS-EXECUTOR`

Owns only:

- `spec-core/src/typescript_backend.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Must not edit:

- canonical example specs
- packet fixture specs
- README / CHANGELOG / TODOS
- plan authority docs

### `WS-AUTHORED-TRUTH`

Owns only:

- `examples/ecommerce/units/pricing/apply_discount.unit.spec`
- `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- `examples/ecommerce/units/pricing/calculate_total.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Must not edit:

- executor core files
- passport or status core logic
- plan authority docs

## Execution Sequence

### Step 1: Freeze the validator and generator contract

Parent work only.

Decide and record:

1. exact wrapper TypeScript eligibility rule
2. exact closure-membership rule
3. exact rejection behavior for cross-library and chain3 paths
4. whether any CLI helper changes are required to distinguish target root vs closure member roles

This step must finish before any worker starts.

### Step 2: `WS-EXECUTOR`

Goal: make the TypeScript execution path admit the wrapper family honestly.

Required outputs:

- widened validator tests
- widened generator tests
- CLI end-to-end coverage for wrapper success and rejection paths
- no scope leakage beyond the owned files

### Step 3: `WS-AUTHORED-TRUTH`

Goal: make the canonical closure and maintained packet truth executable under the widened contract.

Required outputs:

- authored `body.typescript` on the canonical wrapper closure where needed
- wrapper packet fixture parity for the aligned slice, and any targeted negative fixtures required by the new tests
- README/CHANGELOG/TODOS wording aligned to the landed scope

### Step 4: Integration

Parent merges `WS-EXECUTOR` first, then rebases or merges `WS-AUTHORED-TRUTH` on top.

Parent owns all conflict resolution.

### Step 5: Authoritative Proof

Run these from the integrated tree:

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli wrapper -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

Optional final read-side check:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

## Acceptance Criteria

M52 is accepted only if:

- the canonical wrapper unit passes under `spec test --target-language typescript`
- the monotone-up unit still passes under the existing M46 lane
- out-of-scope paths still reject clearly
- docs describe the widened lane as wrapper-family same-tree execution only

## Blocker Handling

If a blocker appears:

- capture it under `validation/*`
- record the exact file, command, and scope leak in `blocked.json`
- stop instead of silently widening the milestone

This milestone is only useful if the boundaries stay sharp.
