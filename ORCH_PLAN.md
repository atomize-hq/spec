# M52 Bounded Same-Tree Wrapper TypeScript Execution Orchestration Plan

Status: **authoritative execution plan**
Supersedes: **the stale M51 `ORCH_PLAN.md`**
Authority source: **`PLAN.md`**
Plan title: **`M52: Bounded Same-Tree Wrapper TypeScript Execution Implementation Plan`**
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**
Base branch: **`main`**
Primary execution branch: **`feat/m40-plus`**
Authority date: **`2026-05-12`**
Worker model: **GPT-5.4 with `reasoning_effort=high`**
Maximum concurrency after contract freeze: **2 worker lanes**
Last rewritten: **2026-05-12**

## Summary

Execute from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`.

`PLAN.md` is the only scope authority. This file is the operator contract derived from it.

M52 is one bounded second-language backend milestone:

- widen the existing TypeScript executor to one new family-shaped slice
- support `function.wrapper.pipeline.v1`
- support only same-tree local direct-dep closure
- keep Bun as the only TypeScript runtime contract
- keep Rust proof and TypeScript proof additive and distinct

M52 is not:

- cross-library TypeScript work
- generic multi-dependency execution
- chain3 execution
- molecule TypeScript support
- seam-kind TypeScript support
- shared-core extraction

Parent-owned work must freeze the execution contract before any worker starts. After that freeze, execution may split into at most two worker lanes plus one parent integration lane.

## Hard Guards

- `PLAN.md` is the sole scope authority.
- The widened executor must remain family-shaped, not generic dep-count-shaped.
- The only new TypeScript target family allowed is `function.wrapper.pipeline.v1`.
- `function.wrapper.pipeline.chain3.v1` remains out of scope.
- Cross-library TypeScript dep resolution remains out of scope.
- `.test.spec --target-language typescript` remains unsupported.
- Rust remains the default target and TypeScript proof remains additive.
- Bun remains the only TypeScript runtime/tooling contract.
- Docs must not claim generic multi-dep TypeScript support.

Abort and re-scope if any of these become necessary:

1. cross-library resolution is required to make the canonical wrapper pass
2. chain3 is required for the first honest wrapper proof
3. the validator must widen to arbitrary supported multi-dep graphs
4. a new proof schema is required instead of reusing `target_proofs.typescript`
5. a new package-manager or config-file contract is required beyond Bun

## Current Code Truth And Rationale

- `spec test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript` already passes.
- `spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript` currently fails before Bun because the wrapper closure is validated under the old M46 monotone-up lane.
- `spec-core/src/validator.rs` still encodes:
  - root target must classify to `function.arithmetic_leaf.monotone_up.v1`
  - deps must be `[]` or exactly one direct local helper dep
- `spec-core/src/typescript_backend.rs` still assumes every emitted TypeScript unit is either:
  - the monotone-up target root, or
  - a helper unit with `deps: []`
- `spec-core/src/semantic_review.rs` already supports `function.wrapper.pipeline.v1` and `function.wrapper.pipeline.chain3.v1`.
- `semantic-families/function.wrapper.pipeline.v1/candidate.md` already freezes the truthful wrapper boundary.

M52 exists to make executor truth catch up to already-supported family truth without widening the family boundary itself.

## Canonical Run Roots

Use these exact paths:

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52
RUN_ROOT=$PRIMARY_ROOT/.runs/m52_same_tree_wrapper_typescript
```

All authoritative orchestration state lives under `RUN_ROOT`.

## Workstream Topology

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m52-parent-authority` | `WS-AUTHORITY` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | kickoff capture, contract freeze, worker fork, merge, final proof |
| `lane/m52-executor` | `WS-EXECUTOR` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/executor` | `ws/m52-executor` | Worker | validator, backend, CLI, and integration-harness widening |
| `lane/m52-authored-truth` | `WS-AUTHORED-TRUTH` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/authored-truth` | `ws/m52-authored-truth` | Worker | canonical specs, wrapper packet fixtures, README/CHANGELOG/TODOS alignment |
| `lane/m52-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m52/int` | `ws/m52-int` | Parent | merge worker lanes and run authoritative proof |

## Parallelization Contract

Parallel work is allowed only after the parent freezes:

1. exact validator eligibility rules
2. exact backend tree-membership rules
3. exact file ownership per lane
4. exact acceptance commands per lane

Before that point, execution is sequential.

After that point:

- `WS-EXECUTOR` and `WS-AUTHORED-TRUTH` may run in parallel
- `WS-INT` starts only after both workers submit

## Orchestration State

### File-Of-Record Inventory

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, authority snapshot metadata | Parent |
| `contract-freeze.json` | exact M52 execution contract | Parent |
| `worktrees.json` | worktree path and branch inventory | Parent |
| `file-ownership.json` | writable repo surfaces per lane | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | dependency queue and task state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance-ledger.md` | final acceptance evidence and signoff ledger | Parent |
| `blocked.json` | blocker capture with file, command, and scope leak | Parent |
| `validation/kickoff/*` | kickoff command captures | Parent |
| `validation/freeze/*` | contract-freeze captures | Parent |
| `validation/executor/*` | executor lane proof captures | Parent |
| `validation/authored-truth/*` | authored-truth lane proof captures | Parent |
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
- tree is clean or contains only the expected authority-doc edits before code execution starts
- authority snapshots are captured before code edits begin

## Contract Freeze

Purpose: freeze the exact M52 execution contract before parallel work.

The freeze must record:

- allowed TypeScript root families:
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.wrapper.pipeline.v1`
- wrapper dep contract:
  - exactly two direct deps
  - local only
  - same loaded unit set
  - same generated tree
  - exact dep-family tuple only:
    - dep 1 = `function.arithmetic_leaf.monotone_down_nonnegative.v1`
    - dep 2 = `function.arithmetic_leaf.monotone_up.v1`
- continued non-goals:
  - cross-library resolution
  - chain3
  - molecule tests
  - generic multi-dep execution
- file ownership per lane
- acceptance commands per lane

Required freeze outputs:

- `contract-freeze.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`

No worker starts before those files exist and are consistent with `PLAN.md`.

## File Ownership

### `WS-EXECUTOR`

Owns only:

- `spec-core/src/typescript_backend.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Must not edit:

- canonical example specs
- semantic-family packet fixtures
- `README.md`, `CHANGELOG.md`, `TODOS.md`
- authority plan docs

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

- `spec-core/`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- authority plan docs

### `WS-AUTHORITY`

Owns only:

- `PLAN.md`
- `ORCH_PLAN.md`
- `RUN_ROOT/**`
- integration conflict resolution in the parent or integration worktree

## Task Graph

| Task ID | Lane | Description | Depends on | Submit when |
| --- | --- | --- | --- | --- |
| `M52-00` | Parent | kickoff capture | — | kickoff artifacts written |
| `M52-01` | Parent | contract freeze | `M52-00` | `contract-freeze.json`, `file-ownership.json`, `tasks.json`, `queue.json` written |
| `M52-10` | `WS-EXECUTOR` | validator widening | `M52-01` | wrapper lane eligibility and rejection tests land |
| `M52-11` | `WS-EXECUTOR` | backend tree widening | `M52-10` | wrapper tree emission and closure-member role tests land |
| `M52-12` | `WS-EXECUTOR` | CLI/proof harness widening | `M52-11` | canonical and fixture TypeScript CLI proofs land |
| `M52-20` | `WS-AUTHORED-TRUTH` | canonical ecommerce TypeScript closure truth | `M52-01` | canonical wrapper closure authors missing TS truth |
| `M52-21` | `WS-AUTHORED-TRUTH` | wrapper packet TypeScript fixture truth | `M52-20` | aligned fixture parity and any required negative fixture land |
| `M52-22` | `WS-AUTHORED-TRUTH` | docs and TODO alignment | `M52-21` | README/CHANGELOG/TODOS wording matches landed scope |
| `M52-30` | Parent | merge executor lane | `M52-12` | executor branch merged cleanly |
| `M52-31` | Parent | merge authored-truth lane | `M52-30`, `M52-22` | authored-truth branch rebased or merged cleanly |
| `M52-40` | Parent | authoritative integrated proof | `M52-31` | proof commands pass and captures are written |
| `M52-41` | Parent | final acceptance and signoff | `M52-40` | acceptance ledger complete |

## Worker Submission Requirements

### `WS-EXECUTOR`

Submit only when all are true:

- validator accepts exact same-tree wrapper roots
- validator rejects cross-library, wrong-arity, wrong-family, missing-dep, and chain3 paths
- backend renders wrapper roots with the exact direct local closure
- unrelated loaded units are not emitted
- canonical wrapper CLI success test lands
- aligned wrapper fixture CLI success test lands
- one bounded rejection-before-Bun test lands

Required capture examples:

- `cargo test -p spec-core typescript`
- targeted `cargo test -p spec-cli ...wrapper...`

### `WS-AUTHORED-TRUTH`

Submit only when all are true:

- canonical ecommerce wrapper closure has the missing authored TypeScript truth
- aligned wrapper packet fixtures author the bounded TypeScript truth needed for M52
- docs describe the widened lane as wrapper-family same-tree execution only
- TODO wording removes only the spent wrapper-execution deferral

Required capture examples:

- diff of canonical specs
- diff of packet fixtures
- diff of README/CHANGELOG/TODOS wording

## Execution Sequence

### Step 1: Parent kickoff

Parent work only.

Create `RUN_ROOT`, capture baseline branch/commit/tree state, snapshot the two authority docs, and refuse to proceed if the working tree already contains unexpected code edits.

### Step 2: Freeze the validator and backend contract

Parent work only.

Decide and record:

1. exact wrapper TypeScript eligibility rule
2. exact closure-membership rule
3. exact rejection behavior for cross-library, chain3, wrong-family, and wrong-arity paths
4. whether any CLI helper changes are required to preserve target-proof routing

This step must finish before any worker starts.

### Step 3: Launch `WS-EXECUTOR`

Goal: make the TypeScript execution path admit the wrapper family honestly.

Required outputs:

- widened validator tests
- widened backend tests
- CLI end-to-end coverage for wrapper success and bounded rejection paths
- no scope leakage beyond lane ownership

### Step 4: Launch `WS-AUTHORED-TRUTH`

Goal: make the canonical closure and maintained wrapper packet truth executable under the widened contract.

Required outputs:

- authored `body.typescript` on the canonical wrapper closure where needed
- wrapper packet fixture TypeScript parity for the aligned slice
- any truly necessary negative fixtures
- README/CHANGELOG/TODOS wording aligned to landed scope

### Step 5: Integration

Parent merges `WS-EXECUTOR` first, then rebases or merges `WS-AUTHORED-TRUTH` on top.

Parent owns all conflict resolution.

If docs or fixture wording contradict the executable contract, code truth wins and docs must be corrected before proof.

### Step 6: Authoritative Proof

Run these from the integrated tree:

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli wrapper -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

Recommended additional read-side check:

```bash
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

### Step 7: Final signoff

Write:

- `validation/final/*` captures
- `acceptance-ledger.md`
- final `session-log.md` entry

Acceptance must reference command outputs, not vibes.

## Acceptance Criteria

M52 is accepted only if:

- the canonical wrapper unit passes under `spec test --target-language typescript`
- the monotone-up unit still passes under the existing M46 lane
- the aligned wrapper packet proves the same bounded TypeScript lane
- out-of-scope paths still reject clearly
- `target_proofs.typescript` remains additive and target-specific
- docs describe the widened lane as wrapper-family same-tree execution only

## Blocker Handling

If a blocker appears:

- capture it under `validation/*`
- record the exact file, command, and scope leak in `blocked.json`
- stop instead of silently widening the milestone

This milestone is only useful if the boundaries stay sharp.

## Done Definition

M52 is done only when:

1. authority docs still match landed behavior
2. both worker lanes stayed within ownership
3. integrated proof passed from the parent or integration worktree
4. no non-goal was silently spent
5. the repo can explain the new TypeScript lane in one paragraph without lying
