# I1 Orchestration Plan

Status: **authoritative execution runbook**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Plan title: **`I1: Benchmark Registry + Shared Projection Core`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m60-plus`**  
Base branch: **`main`**  
Authority commit in `PLAN.md`: **`3561bd1`**  
Milestone: **`I1 Benchmark Registry + Shared Projection Core`**  
Last rewritten: **`2026-05-18`**  
Concurrency cap: **2 parallel workers maximum, plus exactly 1 parent integrator**  
Worker model: **`GPT-5.4` with `reasoning_effort=high`**  
Parent ownership: **the parent is the only integrator and the only owner of run-state, merges, proof wall, and closeout**  
Rewrite intent: **replace the stale M64 orchestration doc with a frozen I1 runbook grounded only in `PLAN.md`**

## Summary

- Execute from the current checked-out branch `feat/m60-plus`.
- Treat `PLAN.md` as the sole authority for I1 facts, scope, ordering, and
  acceptance. `ORCH_PLAN.md` owns execution mechanics only.
- Use this exact worktree root:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-i1`
- Use these exact execution branches:
  - parent integration: `ws/i1-int`
  - worker A registry: `ws/i1-registry`
  - worker B core: `ws/i1-core`
  - worker C cli: `ws/i1-cli`
  - worker D tests: `ws/i1-tests`
- Keep the critical path local to the parent:
  - kickoff and authority freeze
  - worktree creation
  - merge/integration order
  - projector API freeze
  - schema-v4 shape freeze
  - proof wall
  - green closeout or blocked closeout
- Allow exactly one safe parallel start:
  - `LANE-A`: benchmark registry authoring
  - `LANE-B`: shared `spec-core` benchmark projector and export integration
- Launch `LANE-C` only after the parent freezes the shared projector API.
- Launch `LANE-D` only after the parent freezes the final schema-v4 machine
  shape.
- Benchmark rules live once in `spec-core`. `status` and `export` must share
  one projector. There is no second benchmark-rule implementation anywhere in
  the run.

## Hard Guards

- `PLAN.md` is the only authority for I1 milestone facts.
- The working branch remains `feat/m60-plus`.
- The authority commit recorded in the run state remains `3561bd1`.
- The parent is the only integrator. Workers do not merge, rebase, or close
  each other.
- The parent is the only owner of:
  - `.runs/i1_benchmark_registry_run1/**`
  - worktree lifecycle
  - cherry-picks / merges
  - proof-wall execution
  - acceptance ledger
  - final green or blocked closeout
- Scope is frozen to the I1 wedge from `PLAN.md`:
  - benchmark registry
  - shared `spec-core` projection core
  - schema-v4 additive `benchmarks[]` on `status` and `export`
  - full-vs-partial scope honesty
  - explicit reserved `BENCH-SERVICE` projection
  - no new proof writers
- Architectural hard rule:
  - benchmark validation, scope classification, case projection,
    anti-laundering, benchmark status derivation, and reserved-gate semantics
    live once in `spec-core`
  - `status` and `export` must consume that one projector
- No worker may widen scope.
- No worker may touch files outside its frozen write scope.
- Preserve any pre-existing dirty tree exactly as found. Never revert unrelated
  changes.

Stop immediately and mark the run blocked if any of these occur:

1. `PLAN.md` changes materially after the authority freeze.
2. Any worker needs to touch a file outside its assigned ownership.
3. The `spec-core` API cannot serve both `status` and `export` without
   duplicating benchmark rules.
4. `spec-cli/src/commands.rs` would require concurrent authorship.
5. A green I1 outcome would require a new proof writer, snapshot surface,
   readability surface, or `projection_digest`.
6. The benchmark registry requires facts not already derivable from `PLAN.md`
   and the current example roots.
7. The final proof wall cannot demonstrate full-scope, partial-scope,
   reserved, and companion-negative behavior together.

## Concrete Worktree And Branch Topology

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-i1
RUN_ROOT=$PRIMARY_ROOT/.runs/i1_benchmark_registry_run1
```

### Branch inventory

| Lane | Path | Branch | Owner | Write scope |
| --- | --- | --- | --- | --- |
| Primary authority | `PRIMARY_ROOT` | `feat/m60-plus` | Parent | run-state only, no product edits |
| `LANE-INT` | `$WT_ROOT/int` | `ws/i1-int` | Parent | integration, proof wall, closeout |
| `LANE-A` | `$WT_ROOT/registry` | `ws/i1-registry` | Worker A | `benchmarks/labels.json` |
| `LANE-B` | `$WT_ROOT/core` | `ws/i1-core` | Worker B | `spec-core/src/benchmarks.rs`, `spec-core/src/lib.rs`, `spec-core/src/export.rs` |
| `LANE-C` | `$WT_ROOT/cli` | `ws/i1-cli` | Worker C | `spec-cli/src/commands.rs` |
| `LANE-D` | `$WT_ROOT/tests` | `ws/i1-tests` | Worker D | `spec-cli/tests/cli.rs`, `spec-cli/tests/fixtures/` |

### Worktree creation rules

- Do not create worker worktrees until `TASK-I1-00` is closed.
- Create every execution worktree from `feat/m60-plus`.
- Do not edit product files in the primary workspace.
- Workers operate only in their assigned worktree.
- The parent integrates only in `LANE-INT`.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT" "$RUN_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b ws/i1-int feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/registry" -b ws/i1-registry feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/core" -b ws/i1-core feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/cli" -b ws/i1-cli feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/tests" -b ws/i1-tests feat/m60-plus
```

## Durable Orchestration State

All durable orchestration state lives under:

```bash
$RUN_ROOT
```

This directory is run-state only. It is not product truth.

### Required top-level run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, HEAD, dirty-tree snapshot, authority commit | Parent |
| `authority-freeze.json` | frozen I1 facts, stop rules, ownership, gates | Parent |
| `worktrees.json` | exact worktree paths, branches, heads, lifecycle | Parent |
| `file-ownership.json` | authoritative write scopes and no-touch surfaces | Parent |
| `tasks.json` | canonical task ledger and state machine | Parent |
| `session-log.md` | chronological orchestration log | Parent |
| `acceptance-ledger.md` | final gate ledger and proof references | Parent |
| `validation/kickoff/` | kickoff command captures | Parent |
| `validation/lanes/` | lane-specific captures and worker return packets | Parent |
| `validation/final/` | merged proof-wall captures | Parent |
| `handoffs/` | worker briefs and returned summaries | Parent |
| `blocked-summary.md` | required only on blocked closeout | Parent |

### Required per-task sentinel directories

Every task has a sentinel directory here:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Each task directory must contain:

| Path | Required contents |
| --- | --- |
| `status.json` | task id, lane, owner, current state, dependency state, head commit, updated-at timestamp |
| `notes.md` | compact running notes, decisions, scope reminders |
| `commands.log` | exact commands run for that task with exit codes |
| `started-at.txt` | start timestamp |
| `finished-at.txt` | finish timestamp; empty until task closes |
| `acceptance.md` | task-local acceptance checklist and pass/fail result |
| `inputs.md` | exact inputs given to the lane |
| `outputs.md` | exact outputs returned by the lane |
| `handoff.md` | required for worker tasks only; brief + return packet + blocker summary |

### Required `authority-freeze.json` contents

- `milestone`
- `authority_plan_path`
- `authority_plan_commit`
- `primary_branch`
- `base_branch`
- `frozen_scope_claim`
- `architectural_rules`
- `core_authored_surfaces`
- `lane_ownership`
- `integration_order`
- `proof_wall_commands`
- `closeout_gates`
- `stop_rules`
- `worker_model`
- `concurrency_cap`
- `worker_return_contract`

### Required `tasks.json` states

Allowed states:

- `queued`
- `ready`
- `running`
- `blocked`
- `submitted`
- `integrated`
- `closed`
- `skipped`

Only the parent may set `integrated`, `closed`, or `skipped`.

## Lane Ownership And Task Map

### Parent-owned orchestration tasks

| Task ID | Lane | Owner | Purpose | Depends on |
| --- | --- | --- | --- | --- |
| `TASK-I1-00` | `LANE-P` | Parent | baseline freeze and authority capture | - |
| `TASK-I1-01` | `LANE-P` | Parent | create run-state and worktrees | `TASK-I1-00` |
| `TASK-I1-02` | `LANE-P` | Parent | launch worker briefs for `LANE-A` and `LANE-B` | `TASK-I1-01` |
| `TASK-I1-50` | `LANE-INT` | Parent | integrate `LANE-A` and `LANE-B` | `TASK-I1-10`, `TASK-I1-20` |
| `TASK-I1-60` | `LANE-INT` | Parent | freeze projector API and launch `LANE-C` | `TASK-I1-50` |
| `TASK-I1-70` | `LANE-INT` | Parent | integrate `LANE-C` and freeze schema-v4 shape | `TASK-I1-30` |
| `TASK-I1-80` | `LANE-INT` | Parent | launch and integrate `LANE-D` | `TASK-I1-70`, `TASK-I1-10` |
| `TASK-I1-90` | `LANE-INT` | Parent | merged proof wall and acceptance ledger | `TASK-I1-40` |
| `TASK-I1-99` | `LANE-INT` | Parent | green closeout or blocked closeout | `TASK-I1-90` |

### Worker-owned implementation tasks

| Task ID | Lane | Owner | Write scope | Deliverable | Depends on |
| --- | --- | --- | --- | --- | --- |
| `TASK-I1-10` | `LANE-A` | Worker A | `benchmarks/labels.json` | authored benchmark registry matching `PLAN.md` roster | `TASK-I1-02` |
| `TASK-I1-20` | `LANE-B` | Worker B | `spec-core/src/benchmarks.rs`, `spec-core/src/lib.rs`, `spec-core/src/export.rs` | shared benchmark types, registry validation, projector, export bundle field | `TASK-I1-02` |
| `TASK-I1-30` | `LANE-C` | Worker C | `spec-cli/src/commands.rs` | schema-v4 CLI wiring using the frozen shared projector | `TASK-I1-60` |
| `TASK-I1-40` | `LANE-D` | Worker D | `spec-cli/tests/cli.rs`, `spec-cli/tests/fixtures/` | benchmark-aware contract tests and fixtures | `TASK-I1-80` |

## Context-Control Rules

- The parent keeps only a minimal active context set:
  - current gate
  - task ledger state
  - blockers
  - integration order
  - proof-wall status
- The parent does not carry full file contents from worker threads unless
  needed to resolve a blocker.
- Workers may read broadly for context but write only inside frozen ownership.
- Workers return only:
  - changed files
  - commands run with exit codes
  - blockers or unresolved assumptions
  - compact diff summary
- Workers do not return large prose summaries, pasted file bodies, or repeated
  plan restatements.
- Workers are closed immediately after the parent integrates or rejects their
  task output. They are not kept alive for follow-up polling.
- Prefer long waits and sentinel-based check-ins over tight polling loops.
  The parent checks task sentinel state at explicit orchestration boundaries
  only.

## Worker Brief And Return Contract

### Required worker brief

Every worker brief must contain exactly:

- task id
- lane id
- worktree path
- branch name
- frozen write scope
- explicit no-touch surfaces
- local acceptance criteria
- required commands
- worker return contract
- blocked-return rule

### Required worker return packet

Every worker return packet must contain exactly:

- `task_id`
- `state`: `submitted` or `blocked`
- `head_commit`
- `changed_files`
- `commands_run`
- `exit_codes`
- `compact_diff_summary`
- `blockers_or_unresolved_assumptions`

If `state = blocked`, the packet must also contain:

- the first forbidden file or surface encountered
- why current ownership is insufficient
- the narrowest safe next action

## Formal Gate Ledger

| Gate | Owner | Opens when | Closes when |
| --- | --- | --- | --- |
| `G0 authority-freeze` | Parent | run starts | baseline is captured and authority commit `3561bd1` is frozen |
| `G1 topology-freeze` | Parent | `G0` green | worktrees, file ownership, and run-state scaffolding exist |
| `G2 core-freeze` | Parent | `TASK-I1-10` and `TASK-I1-20` submitted | registry + `spec-core` projector integrate cleanly and projector API is frozen |
| `G3 schema-freeze` | Parent | `TASK-I1-30` submitted | schema version `4` and additive `benchmarks[]` are live on both machine surfaces |
| `G4 contract-wall` | Parent | `TASK-I1-40` submitted | benchmark contract tests and fixtures merge cleanly and pass |
| `G5 proof-wall` | Parent | `G4` green | merged repo satisfies all final I1 acceptance criteria |
| `G6 closeout` | Parent | `G5` green or blocked | green closeout written or blocked summary written |

## Workstream Plan

### Phase 0: Kickoff And Freeze

#### `TASK-I1-00` - baseline freeze

Owner: Parent  
Worktree: primary authority workspace

Run:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
git merge-base --is-ancestor 3561bd1 HEAD
```

Record in `baseline.json`:

- current branch
- current `HEAD`
- dirty-tree snapshot
- authority commit `3561bd1`
- freeze timestamp

Acceptance:

1. branch is `feat/m60-plus`
2. `3561bd1` is recorded as the authority commit from `PLAN.md`
3. dirty-tree state is preserved exactly

#### `TASK-I1-01` - run-state and worktree setup

Owner: Parent  
Worktree: primary authority workspace

Create:

- `RUN_ROOT`
- top-level run-state files
- `tasks/<TASK_ID>/` sentinel directories
- all execution worktrees

Acceptance:

1. all worktree paths exist
2. all branches match this runbook exactly
3. `file-ownership.json` matches this runbook exactly
4. no product file was edited in the primary workspace

#### `TASK-I1-02` - launch worker briefs

Owner: Parent  
Worktree: primary authority workspace

Launch exactly:

- `TASK-I1-10` in `LANE-A`
- `TASK-I1-20` in `LANE-B`

Acceptance:

1. both briefs are written under `handoffs/`
2. both worker task sentinel directories are initialized
3. no other worker lane is launched

### Phase 1: Safe Parallel Start

#### `TASK-I1-10` - `LANE-A` benchmark registry authoring

Owner: Worker A  
Worktree: `LANE-A`  
Write scope: `benchmarks/labels.json`

Required implementation:

- `BENCH-ECOM` active positive benchmark
- `BENCH-CROSSLIB` active companion negative-proof benchmark
- `BENCH-SERVICE` reserved positive benchmark
- required molecules and labeled cases exactly as specified in `PLAN.md`

Required commands:

```bash
rg -n "BENCH-ECOM|BENCH-CROSSLIB|BENCH-SERVICE|examples/ecommerce|examples/crosslib-app" PLAN.md
```

Acceptance:

1. registry roster matches `PLAN.md` exactly
2. paths are repo-relative and resolve to the intended roots
3. active benchmark carriers correspond to authored unit ids already in repo
4. reserved benchmark starts with empty cases
5. no file outside `benchmarks/labels.json` is edited

#### `TASK-I1-20` - `LANE-B` shared `spec-core` projection core

Owner: Worker B  
Worktree: `LANE-B`  
Write scope:

- `spec-core/src/benchmarks.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/export.rs`

Required implementation:

- benchmark enums and projection structs
- registry parsing and validation
- full-vs-partial scope classification
- case projection and required molecule projection
- anti-laundering rules
- benchmark status and gate-status derivation
- additive `benchmarks[]` in export bundle

Required commands:

```bash
cargo test -p spec-core benchmark
```

Acceptance:

1. one shared projector can serve both `status` and `export`
2. reserved `BENCH-SERVICE` state is representable
3. partial entries omit whole-benchmark rollup fields as required by `PLAN.md`
4. export bundle exposes additive top-level `benchmarks[]`
5. no file outside the frozen `spec-core` surfaces is edited

### Phase 2: Parent Integration Barrier

#### `TASK-I1-50` - integrate `LANE-A` and `LANE-B`

Owner: Parent  
Worktree: `LANE-INT`

Integration order:

1. integrate `TASK-I1-10`
2. integrate `TASK-I1-20`
3. run narrow proof to confirm the registry and shared projector compile

Required commands:

```bash
git cherry-pick <registry-commit>
git cherry-pick <core-commit>
cargo test -p spec-core benchmark
```

Acceptance:

1. registry and `spec-core` surfaces merge cleanly
2. shared projector API is frozen and written into task notes
3. no CLI or fixture files are touched yet
4. `G2 core-freeze` closes green

#### `TASK-I1-60` - freeze projector API and launch `LANE-C`

Owner: Parent  
Worktree: `LANE-INT`

Launch `TASK-I1-30` only after `G2` closes.

Acceptance:

1. `LANE-C` brief explicitly states the frozen projector API
2. `spec-cli/src/commands.rs` is single-owned by `LANE-C`
3. any requested API change from `LANE-C` is treated as a blocker, not an ad
   hoc scope drift

### Phase 3: Serialized CLI Wiring

#### `TASK-I1-30` - `LANE-C` status/export integration

Owner: Worker C  
Worktree: `LANE-C`  
Write scope: `spec-cli/src/commands.rs`

Required implementation:

- load `benchmarks/labels.json` once per invocation
- bump `STATUS_JSON_SCHEMA_VERSION` to `4`
- wire shared projector into `spec status --format json`
- wire shared projector into `spec export --format json`
- keep text mode unchanged

Required commands:

```bash
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- export examples/crosslib-app --format json
```

Acceptance:

1. `status` and `export` call the same shared projector
2. full-scope repo-root and benchmark-root behavior is live
3. partial-scope behavior is live without positive-credit laundering
4. reserved `BENCH-SERVICE` appears only on broad enough scope
5. no file outside `spec-cli/src/commands.rs` is edited

#### `TASK-I1-70` - integrate CLI wiring and freeze schema shape

Owner: Parent  
Worktree: `LANE-INT`

Required commands:

```bash
git cherry-pick <cli-commit>
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- export examples/crosslib-app --format json
```

Acceptance:

1. schema version `4` is live on both machine surfaces
2. additive top-level `benchmarks[]` is present on both surfaces
3. shared projector path remains singular
4. fixture files remain untouched at this point
5. `G3 schema-freeze` closes green

### Phase 4: Contract Tests And Fixtures

#### `TASK-I1-80` - launch `LANE-D`

Owner: Parent  
Worktree: `LANE-INT`

Launch `TASK-I1-40` only after `G3` closes.

Acceptance:

1. `LANE-D` brief pins the schema-v4 shape frozen by the parent
2. `LANE-D` owns only `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/`

#### `TASK-I1-40` - `LANE-D` benchmark contract tests

Owner: Worker D  
Worktree: `LANE-D`  
Write scope:

- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/`

Required coverage:

- repo-root full-scope benchmark projection
- benchmark-root full-scope benchmark projection
- nested-directory partial-scope projection
- single-file partial-scope projection
- export benchmark projection
- reserved `BENCH-SERVICE` broad-scope visibility only
- unlabeled active-carrier invalidation
- companion-negative visibility and zero positive credit
- schema version `4` fixtures for status and export

Required commands:

```bash
cargo test -p spec-cli --test cli benchmark
```

Acceptance:

1. tests prove full, partial, reserved, and companion-negative semantics
2. fixtures pin additive `benchmarks[]` without weakening unrelated surfaces
3. `spec-cli/tests/cli.rs` asserts schema version `4`
4. no production files are edited

#### `TASK-I1-85` - integrate tests and fixtures

Owner: Parent  
Worktree: `LANE-INT`

Required commands:

```bash
git cherry-pick <tests-commit>
cargo test -p spec-cli --test cli benchmark
```

Acceptance:

1. fixture updates merge cleanly against the frozen schema shape
2. benchmark-focused CLI tests pass
3. no worker had to widen scope to close the contract wall
4. `G4 contract-wall` closes green

## Tests, Proof Wall, And Final Acceptance

The proof wall is parent-owned and runs only from merged `LANE-INT`.

### Required proof commands

Run these in `LANE-INT` and capture stdout/stderr plus exit codes under
`validation/final/`:

```bash
cargo test -p spec-core benchmark
cargo test -p spec-cli --test cli benchmark
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- export examples/crosslib-app --format json
```

### Final acceptance criteria

`G5 proof-wall` closes only when all are true:

1. `benchmarks/labels.json` exists and matches the I1 roster from `PLAN.md`.
2. benchmark rules live once and only once in `spec-core`.
3. `spec status --format json` reports `schema_version: 4`.
4. `spec export --format json` reports `schema_version: 4`.
5. both surfaces emit additive top-level `benchmarks[]`.
6. repo-root or benchmark-root full-scope queries produce benchmark status and
   gate status.
7. partial-scope queries produce only honest partial entries and zero positive
   credit.
8. `BENCH-SERVICE` appears as reserved only on broad enough queries.
9. `BENCH-CROSSLIB` remains visible as companion negative proof and never
   counts positive.
10. no new proof writers, snapshot surfaces, readability surfaces, or digest
    surfaces were introduced.
11. benchmark-focused `spec-core` and CLI tests pass from the merged state.

## Failure And Blocked Behavior

If any lane blocks, the worker returns immediately with the required blocked
packet and stops. The parent then chooses exactly one response:

1. reassign the blocked file or decision to the parent and continue serially
2. relaunch the blocked lane with corrected frozen scope
3. stop the run and write `blocked-summary.md`

`blocked-summary.md` must include:

- failed gate
- last green task
- smallest unresolved scope expansion
- whether the blocker is architectural, fixture-related, or proof-related
- exact `PLAN.md` clause forcing the stop
- next valid restart point

## Final Closeout

### Green path closeout

If `G5` is green:

1. parent records final proof references in `acceptance-ledger.md`
2. parent marks `TASK-I1-90` closed
3. parent marks `TASK-I1-99` closed
4. parent closes all worker lanes and records final integrated head commits
5. run ends green

### Blocked path closeout

If any stop rule trips before `G5`:

1. parent writes `blocked-summary.md`
2. parent marks the active task blocked
3. parent marks `TASK-I1-99` closed as blocked closeout
4. worker lanes are closed immediately
5. run ends blocked without silent scope widening

## Assumptions

- `PLAN.md` at commit `3561bd1` remains the authority throughout execution.
- The branch remains `feat/m60-plus`.
- The live repo already contains the example roots named in `PLAN.md`:
  - `examples/ecommerce/units`
  - `examples/crosslib-app/units`
- The authored I1 file set remains:
  - `benchmarks/labels.json`
  - `spec-core/src/benchmarks.rs`
  - `spec-core/src/lib.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/*` as needed
- The final I1 run stays inside the reader-side boundary:
  - benchmark projection only
  - no new proof-writing behavior
  - no snapshot or readability closure

## Completion Summary

- Fresh kickoff from current workspace state
- Frozen authority from `PLAN.md` only
- Exact worktree root and branch topology
- Parent-only integration, proof wall, and closeout
- One serialized architecture barrier around the shared `spec-core` projector
- Strong per-task sentinels and worker return packets
- Formal gate ledger from kickoff through final closeout
