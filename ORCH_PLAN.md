# I2 Orchestration Plan

Status: **authoritative execution runbook**  
Authority source: **the current working-tree bytes of `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Plan title: **`I2: Rust V1 Contract Stack Mechanics Landing Plan`**  
Repo root: **`/home/azureuser/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Current checked-out branch: **`main`**  
Authority validated commit in `PLAN.md`: **`aca0307`**  
Observed `HEAD` at kickoff: **`aca0307ccd07480388f2c195b17795a203abf607`**  
Observed dirty tree at kickoff: **`PLAN.md` modified**  
Authority date: **`2026-05-18`**  
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**  
Worker model assumption: **`GPT-5.4` with `reasoning_effort=high`**  
Rewrite intent: **fully replace the stale M61 TypeScript `ORCH_PLAN.md` with an I2 benchmark-mechanics runbook grounded only in the current I2 `PLAN.md` and live repo state**  
Last rewritten: **`2026-05-18`**

## Summary

I2 lands the Rust V1 benchmark and truth-surface mechanics layer and nothing broader:

`one benchmark registry, one shared benchmark projection core, schema-v4 benchmark projection in status/export, benchmark snapshots, readability review anchoring, reserved BENCH-SERVICE visibility, and the exact anti-laundering/path-scope rules locked in PLAN.md.`

The parent agent owns the critical path and is the only integrator. The execution shape is fixed:

1. freeze the exact working-tree authority version of `PLAN.md`
2. run Phase 1 and Phase 2 sequentially in one parent-owned core lane
3. merge that lane and record the Phase 2 contract freeze
4. fork two parallel workers from the exact freeze commit:
   - Phase 3 `status`/`export` schema-v4 integration
   - Phase 4 snapshot writer and readability loading
5. integrate both parallel lanes back into the integration branch
6. fork one final worker for Phase 5 repo seeding, fixtures, and docs sync
7. integrate Phase 5 and run the full acceptance wall
8. end with one verified landing candidate commit on the integration branch

No approval-gate artifacts are part of I2. Do not invent them.

Because the primary checkout is already on `main` with a dirty `PLAN.md`, the parent must preserve that checkout as the authority reference and use worktrees for all implementation branches. The parent may not treat the primary root as a clean integration tree.

## Assumptions

These assumptions are justified by the current `PLAN.md` and observed repo state and remain in force unless refreshed at kickoff:

- The parent is the only integrator.
  - This is explicitly required by the requested orchestration shape and matches the plan’s need for one owner of the Phase 2 freeze and final acceptance wall.
- `GPT-5.4` with `reasoning_effort=high` is the worker model assumption.
  - This was part of the requested execution constraints and is carried literally.
- Worktree-based execution is required.
  - The primary checkout is on `main` and the working tree is dirty because `PLAN.md` is modified, so implementation work must not reuse the primary checkout as a clean lane.
- Phase 1 and Phase 2 are sequential and parent-owned.
  - `PLAN.md` fixes the exact freeze point at the end of Phase 2.
- Phase 3 and Phase 4 are the only safe parallel lanes before Phase 5.
  - `PLAN.md` explicitly allows parallelization only after the Phase 2 freeze, with Phase 5 after both.
- Benchmark seed artifacts are intended to become tracked repo files in Phase 5.
  - `PLAN.md` lists `benchmarks/labels.json`, benchmark snapshots, readability review anchoring, fixtures, and docs sync as milestone deliverables.
- `examples/ecommerce/units/**`, `examples/crosslib-app/units/**`, and `examples/shared-spec/units/**` are existing benchmark inputs, not new authoring targets for I2.
  - This is consistent with the current repo state and the exact initial roster in `PLAN.md`.
- `BENCH-SERVICE` remains reserved and visible without a real workload tree.
  - `PLAN.md` explicitly calls out the missing `examples/service/units` tree as acceptable only because `BENCH-SERVICE` remains reserved and machine-visible.

## Hard Guards

- `PLAN.md` working-tree bytes at kickoff are the only authority for milestone facts.
- Snapshot those exact bytes before creating worker branches. Do not rely on `HEAD:PLAN.md` after kickoff.
- The existing repo-root `ORCH_PLAN.md` is stale M61 TypeScript material and is shape-reference only.
- Do not copy any stale M61 facts, branches, paths, commands, approvals, TypeScript scope, or acceptance mechanics.
- No human approval gates exist in I2 unless `PLAN.md` itself adds them. It does not.
- Phase order is fixed:
  - kickoff and authority freeze
  - Phase 1
  - Phase 2
  - Phase 2 freeze
  - Phase 3 and Phase 4 in parallel
  - Phase 5
  - final proof and closeout
- The parent is the only integrator.
- Do not widen, narrow, or reinterpret M66 support rows.
- Do not add benchmark fields to `.unit.spec`, `.test.spec`, passports, or molecule evidence.
- Do not make `spec status`, `spec export`, or `spec benchmark snapshot` write proof truth.
- Do not implement `BENCH-SERVICE`. It stays `reserved` and machine-visible.
- Do not create `examples/service/units` or `examples/service/src/generated` in I2.
- `examples/ecommerce/units/**`, `examples/crosslib-app/units/**`, and `examples/shared-spec/units/**` are read-side benchmark inputs, not authoring targets for this milestone.
- `.spec.passport.json` and `*.test.evidence.json` remain read-only truth inputs during I2.
- Before Phase 5, do not commit repo-root `benchmarks/labels.json`, `benchmarks/reviews/**`, or `benchmarks/snapshots/**`.
- The exact initial `BENCH-ECOM` required molecule set is frozen:
  - `pricing/checkout_flow`
  - `pricing/discount_plus_tax`
- Do not silently add `pricing/discount_strategy_checkout_flow` to benchmark-required molecules.
- `BENCH-CROSSLIB` remains visible but never positive-credit.
- Partial scope never emits positive supported credit.
- If the working-tree bytes of `PLAN.md` change after kickoff, halt and refresh the authority snapshot, freeze record, and all downstream worker briefs before more code moves.

## Current Repo Reality Freeze

Observed at kickoff:

- branch: `main`
- `HEAD`: `aca0307ccd07480388f2c195b17795a203abf607`
- dirty tree:
  - `PLAN.md`
- `benchmarks/` does not exist yet
- `examples/service/units` does not exist
- current benchmark inputs already exist under:
  - `examples/ecommerce/units/**`
  - `examples/crosslib-app/units/**`
  - `examples/shared-spec/units/**`
- current primary code surfaces already exist under:
  - `spec-core/src/lib.rs`
  - `spec-core/src/export.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/molecule_evidence.rs`
  - `spec-core/src/types.rs`
  - `spec-core/src/graph.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/*.json`
  - `README.md`
  - `CHANGELOG.md`
  - `TODOS.md`

This matters operationally:

- all implementation edits happen in worktrees, not in the dirty primary checkout
- all worker prompts cite the parent-owned `authority-plan.snapshot.md`, not a branch-local `PLAN.md`
- final landing proof runs in the integration worktree, not the dirty primary root

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec
WT_ROOT=/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i2-benchmark-mechanics
RUN_ROOT=$PRIMARY_ROOT/.runs/i2_rust_v1_contract_stack_mechanics
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority root | `PRIMARY_ROOT` | `main` | Parent | preserves dirty `PLAN.md`, owns durable run-state only |
| `WS-INT` | `$WT_ROOT/int` | `codex/i2-int` | Parent | integration branch and final proof branch |
| `WS-A` | `$WT_ROOT/core` | `codex/i2-core` | Parent | Phase 1 and Phase 2 sequential core contract lane |
| `WS-B` | `$WT_ROOT/status-export` | `codex/i2-status-export` | Worker | Phase 3 schema-v4 `status`/`export` integration |
| `WS-C` | `$WT_ROOT/snapshot` | `codex/i2-snapshot` | Worker | Phase 4 snapshot writer and readability loading |
| `WS-D` | `$WT_ROOT/seed-docs` | `codex/i2-seed-docs` | Worker | Phase 5 repo seeding, fixtures, and docs sync |

### Worktree creation rules

- Do not create worker worktrees before the authority snapshot is frozen.
- Create `WS-INT` and `WS-A` first from `main`.
- `WS-B` and `WS-C` must branch from the exact Phase 2 freeze commit recorded in `contract-freeze.json`.
- `WS-D` must branch only after both `WS-B` and `WS-C` are integrated into `WS-INT`.
- There is no separate worker for final integration. Final proof is parent-only in `WS-INT`.
- Never branch worker lanes from the dirty primary checkout state.
- Never check out implementation branches in `PRIMARY_ROOT`.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT" "$RUN_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b codex/i2-int main
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/core" -b codex/i2-core main

# after Phase 2 is integrated and contract-freeze.json records phase_2_freeze_commit
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/status-export" -b codex/i2-status-export codex/i2-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/snapshot" -b codex/i2-snapshot codex/i2-int

# after WS-B and WS-C are integrated into codex/i2-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/seed-docs" -b codex/i2-seed-docs codex/i2-int
```

## Durable Orchestration State

All parent-owned run-state lives under:

```bash
$RUN_ROOT
```

This is run-state only, not product truth.

### Required run-state artifacts

| Path | Purpose |
| --- | --- |
| `authority-plan.snapshot.md` | exact working-tree bytes of `PLAN.md` at kickoff |
| `authority-plan.sha256` | hash of the exact authority snapshot |
| `baseline.json` | kickoff branch, `HEAD`, dirty-tree summary, observed repo reality |
| `contract-freeze.json` | frozen Phase 2 benchmark contract and worker lane boundaries |
| `worktrees.json` | exact worktree paths, branches, heads, and lane states |
| `file-ownership.json` | lane write scopes and global no-touch surfaces |
| `tasks.json` | canonical task ledger and states |
| `session-log.md` | chronological launch, merge, rerun, block, and close log |
| `acceptance-ledger.md` | final gate checklist and proof references |
| `final-proof-manifest.json` | exact final commands, exit codes, and captured output paths |
| `handoffs/` | worker briefs and worker return summaries |
| `validation/kickoff/` | kickoff proof captures |
| `validation/ws-a/` | core-lane proof captures |
| `validation/ws-b/` | Phase 3 proof captures |
| `validation/ws-c/` | Phase 4 proof captures |
| `validation/ws-d/` | Phase 5 proof captures |
| `validation/final/` | final proof-wall captures |
| `tasks/<TASK_ID>/` | per-task parent-owned sentinels |

### Required `baseline.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_snapshot_path`
- `authority_plan_sha256`
- `authority_plan_validated_commit`
- `observed_head_commit`
- `primary_branch`
- `dirty_tree_summary`
- `dirty_tree_files`
- `observed_primary_surfaces`
- `missing_surfaces`
- `initial_benchmark_roster_from_plan`
- `kickoff_commands`
- `run_started_at`

### Required `contract-freeze.json` fields

- `milestone`
- `authority_plan_snapshot_path`
- `authority_plan_sha256`
- `phase_2_freeze_commit`
- `frozen_benchmark_enums`
- `frozen_full_projection_fields`
- `frozen_partial_projection_omissions`
- `frozen_digest_rules`
- `frozen_reserved_state`
- `frozen_path_scope_rules`
- `frozen_anti_laundering_rule`
- `frozen_initial_roster`
- `frozen_required_molecule_ids`
- `lane_ownership`
- `global_no_touch_surfaces`
- `integration_order`
- `worker_model`
- `worker_return_contract`
- `verification_commands`
- `stop_rules`

### Required `tasks.json` fields

- `milestone`
- `updated_at`
- `tasks[]`
  - `task_id`
  - `lane`
  - `owner`
  - `state`
  - `depends_on`
  - `write_scope`
  - `command_wall`
  - `acceptance_summary`
  - `stop_rules`

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

Only the parent may set `integrated`, `closed`, or `skipped`.

Workers do not write run-state directly. Workers return narrow summaries; the parent updates `tasks.json`, sentinel files, and the session log.

Each task gets a sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Minimum sentinel files:

- `status.json`
- `notes.md`
- `commands.txt`
- `submission.md` when returned
- `blocker.md` when blocked

## Kickoff Sequence

This section is deterministic startup procedure. Run it in order. Do not skip ahead.

1. Freeze authority bytes.
- Read the current working-tree `PLAN.md`.
- Write those exact bytes to:
  - `$RUN_ROOT/authority-plan.snapshot.md`
- Write its SHA-256 to:
  - `$RUN_ROOT/authority-plan.sha256`
- Record in `session-log.md` that all downstream work keys off this snapshot, not branch-local `PLAN.md`.

2. Capture baseline repo state.
- In `PRIMARY_ROOT`, record:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short`
- Record that `PLAN.md` is dirty in the primary checkout.
- Record that `benchmarks/` does not yet exist.
- Record that `examples/service/units` does not yet exist.
- Write `baseline.json`.
- Save command outputs under:
  - `$RUN_ROOT/validation/kickoff/`

3. Bootstrap parent-owned worktrees.
- Create `WS-INT` from `main`.
- Create `WS-A` from `main`.
- Initialize:
  - `worktrees.json`
  - `file-ownership.json`
  - `tasks.json`
  - `acceptance-ledger.md`
- Do not create `WS-B`, `WS-C`, or `WS-D` yet.

4. Run Phase 1 in `WS-A`.
- Execute `task/i2-a1-phase1-registry-types`.
- Keep scope limited to core benchmark types, registry parsing, and validation.
- Capture proof under:
  - `$RUN_ROOT/validation/ws-a/`

5. Run Phase 2 in `WS-A`.
- Execute `task/i2-a2-phase2-projection-core`.
- Keep scope limited to shared projection, digests, reserved-state projection, path-scope rules, and readability file selection.
- Capture proof under:
  - `$RUN_ROOT/validation/ws-a/`

6. Integrate Phase 1 and Phase 2 into `WS-INT`.
- Merge `WS-A` into `WS-INT`.
- Verify `WS-INT` is green for the Phase 1 and Phase 2 command wall.

7. Create the Phase 2 contract freeze.
- Write `contract-freeze.json` from the integrated `WS-INT` state.
- Record:
  - `phase_2_freeze_commit`
  - frozen enums
  - frozen full projection field set
  - frozen partial projection omission set
  - frozen digest rules
  - frozen reserved `BENCH-SERVICE` state
  - frozen anti-laundering rule
  - frozen path-scope rules
  - frozen initial roster and required molecules
- No worker may proceed without this freeze record.

8. Bootstrap the only allowed pre-Phase-5 parallel lanes.
- Create `WS-B` from the exact `phase_2_freeze_commit`.
- Create `WS-C` from the exact `phase_2_freeze_commit`.
- Emit worker briefs from:
  - `$RUN_ROOT/handoffs/`
- Confirm both workers receive:
  - `authority-plan.snapshot.md`
  - relevant `contract-freeze.json` excerpt
  - exact write scope
  - command wall
  - worker return contract

9. Hold `WS-D` until Phase 3 and Phase 4 are both integrated.
- Do not create the seeding/docs lane before `WS-B` and `WS-C` converge in `WS-INT`.

## Workstream Plan

### WS-BASELINE — parent only, no implementation edits

#### `task/i2-00-authority-snapshot`

Own:

- `RUN_ROOT/**` only

Do:

- snapshot the exact working-tree bytes of `PLAN.md`
- hash that snapshot
- record `git branch --show-current`
- record `git rev-parse HEAD`
- record `git status --short`
- record the observed absence of `benchmarks/`
- record the observed absence of `examples/service/units`
- record the stale M61 `ORCH_PLAN.md` only as excluded context

Acceptance:

- `authority-plan.snapshot.md` exists
- `authority-plan.sha256` exists
- `baseline.json` captures dirty-tree reality accurately
- no implementation files changed

#### `task/i2-01-bootstrap-worktrees`

Own:

- `RUN_ROOT/**`
- worktree creation only

Do:

- create `WS-INT`
- create `WS-A`
- initialize `worktrees.json`
- initialize `file-ownership.json`
- initialize `tasks.json`

Acceptance:

- `WS-INT` and `WS-A` exist from `main`
- no worker worktrees exist yet
- parent has not modified product files in `PRIMARY_ROOT`

### WS-A (`codex/i2-core`) — parent only, sequential

#### `task/i2-a1-phase1-registry-types`

Own:

- `spec-core/src/benchmark.rs` new
- `spec-core/src/lib.rs`
- `spec-core/src/types.rs` only if required for benchmark type wiring
- `spec-core/src/graph.rs` only if required for benchmark scope/carrier wiring
- `spec-core/src/passport.rs` only if required for read-only proof projection access
- `spec-core/src/molecule_evidence.rs` only if required for read-only proof projection access

Must not touch:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/**`
- `benchmarks/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Deliver:

- registry loader and validator
- typed benchmark enums and structs
- carrier-to-label matching
- benchmark scope intersection logic
- machine-readable invalid-registry failure path

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
```

Acceptance:

- full parse/validation tests pass
- invalid registry surfaces explicit machine-readable failure
- no repo-root benchmark files are committed yet

#### `task/i2-a2-phase2-projection-core`

Own:

- same write scope as `task/i2-a1-phase1-registry-types`

Deliver:

- full and partial benchmark projection builder
- case-level anti-laundering behavior
- summary computation
- deterministic `label_digest`
- deterministic `projection_digest`
- readability-generated file selection
- explicit reserved benchmark state contract

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
```

Acceptance:

- full positive, companion-negative, and reserved unit tests pass
- digest stability tests pass
- partial projections omit the exact locked field set
- no CLI or repo-root benchmark seed files are touched

#### `task/i2-a3-integrate-and-freeze`

Own:

- merge `WS-A` into `WS-INT`
- `RUN_ROOT/**`

Do:

- integrate Phase 1 and Phase 2 into `codex/i2-int`
- record `phase_2_freeze_commit`
- write `contract-freeze.json`
- freeze the exact full-projection field set
- freeze the exact partial-projection omission set
- freeze enum values, digest rules, path-scope rules, reserved-state rules, and anti-laundering rule
- freeze the exact initial benchmark roster:
  - `BENCH-ECOM`
  - `BENCH-SERVICE`
  - `BENCH-CROSSLIB`
- freeze exact initial `BENCH-ECOM` carriers:
  - `money/round`
  - `pricing/apply_discount`
  - `pricing/apply_tax`
  - `pricing/calculate_total`
  - `pricing/calculate_total_guarded_tax`
  - `pricing/discount_strategy`
  - `pricing/pricing_quote`
- freeze exact initial `BENCH-CROSSLIB` carriers:
  - `pricing/apply_discount`
  - `pricing/apply_tax`
  - `pricing/calculate_total`
  - `pricing/checkout_nested_chain3`
- freeze exact benchmark-required molecules:
  - `pricing/checkout_flow`
  - `pricing/discount_plus_tax`
- create `WS-B` and `WS-C` from that integrated state only

Acceptance:

- `contract-freeze.json` exists and is complete
- `WS-B` and `WS-C` branch from `phase_2_freeze_commit`
- no worker may start from any earlier `main` head

### Parallel workers after Phase 2 freeze

All worker prompts must quote `authority-plan.snapshot.md` and `contract-freeze.json`. No worker may reopen milestone facts from stale `ORCH_PLAN.md`.

### WS-B (`codex/i2-status-export`) — worker 1

#### `task/i2-b-phase3-status-export`

Own only:

- `spec-cli/src/commands.rs` regions for existing `status` and `export` paths
- `spec-core/src/export.rs`

Read-only dependencies:

- `spec-core/src/benchmark.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/molecule_evidence.rs`
- `authority-plan.snapshot.md`
- `contract-freeze.json`

Must not touch:

- snapshot subcommand parsing/writing regions in `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/**`
- `benchmarks/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Deliver:

- additive top-level `benchmarks[]` in `spec status --format json`
- additive top-level `benchmarks[]` in `spec export`
- `schema_version: 4` for both surfaces
- shared projection core wired into both surfaces
- repo-root, benchmark-root, subtree, and single-file scope handling

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
cargo test -p spec-cli
```

Acceptance:

- build and tests are green in the lane
- no benchmark seed files or fixtures are committed here
- no local benchmark logic is duplicated outside shared core
- status and export call the same projection core

### WS-C (`codex/i2-snapshot`) — worker 2

#### `task/i2-c-phase4-snapshot-readability`

Own only:

- `spec-cli/src/commands.rs` regions for new `benchmark snapshot` parsing and file writing
- `spec-core/src/benchmark.rs` only for snapshot assembly and readability review loading that stays inside the frozen Phase 2 contract

Read-only dependencies:

- `spec-core/src/export.rs`
- `authority-plan.snapshot.md`
- `contract-freeze.json`

Must not touch:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs` status/export wiring regions
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/**`
- committed `benchmarks/**` seed files
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Deliver:

- `spec benchmark snapshot <benchmark-id>`
- snapshot artifact writing under `benchmarks/snapshots/`
- readability review file loading
- full-scope readability status and verdict projection
- reserved snapshot behavior for `BENCH-SERVICE`

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
cargo test -p spec-cli
```

Acceptance:

- snapshot command writes only snapshot files
- active positive snapshots validate readability-generated file existence
- reserved `BENCH-SERVICE` snapshot path does not fake generated-tree freshness
- no repo-root seed files are committed here

### WS-INT (`codex/i2-int`) — parent integration between parallel lanes

#### `task/i2-int-bc-merge`

Own:

- integration merges only
- `RUN_ROOT/**`

Do:

- integrate whichever of `WS-B` or `WS-C` finishes first
- integrate the second lane after the first is green
- resolve `spec-cli/src/commands.rs` conflicts strictly by ownership wall:
  - `WS-B` wins `status` and `export` schema-v4 wiring
  - `WS-C` wins `benchmark snapshot` parsing and snapshot file writing
- reject any worker change that mutates frozen Phase 2 contract surfaces without an explicit parent re-freeze

Acceptance:

- `codex/i2-int` contains both Phase 3 and Phase 4
- `commands.rs` merge preserves the frozen split of responsibilities
- `contract-freeze.json` remains truthful after integration

### WS-D (`codex/i2-seed-docs`) — worker after WS-B and WS-C are integrated

#### Why WS-D owns CLI fixtures and repo-facing integration assertions

This ownership split is intentional.

- The benchmark machine contract is frozen after Phase 2.
- Phase 3 and Phase 4 can wire behavior in parallel against that frozen contract.
- The repo-facing JSON fixtures and high-level CLI integration assertions should be authored once, after the merged truth from Phase 3 and Phase 4 exists in one branch.
- This avoids duplicate fixture churn across two parallel workers.
- This reduces merge conflicts in `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/*.json`.
- This keeps one authoritative repo-facing test lane for schema-v4 baseline capture, seeded benchmark artifacts, and docs wording that depend on the merged benchmark surface rather than one half of it.

#### `task/i2-d-phase5-seeding-docs`

Own only:

- `benchmarks/labels.json`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/snapshots/*.snapshot.json`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/*.json`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Must not touch:

- `spec-core/**`
- `spec-cli/src/commands.rs`
- `examples/ecommerce/units/**`
- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- passports
- molecule evidence

Deliver:

- initial committed `benchmarks/labels.json` with the exact frozen roster
- initial `BENCH-ECOM` readability review anchored to the current `projection_digest`
- generated benchmark snapshots for:
  - `BENCH-ECOM`
  - `BENCH-CROSSLIB`
  - `BENCH-SERVICE`
- schema-v4 fixture baselines for:
  - full-scope positive benchmark projection
  - partial-scope positive benchmark projection
  - reserved benchmark projection
  - companion-negative benchmark projection
  - export bundle benchmark projection
  - invalid registry path
- docs sync in:
  - `README.md`
  - `CHANGELOG.md`
  - `TODOS.md`

Hard guards:

- do not create `examples/service/**`
- do not widen benchmark roster beyond the frozen plan roster
- do not change required molecule ids beyond the frozen pair
- do not move benchmark truth into specs, passports, or evidence
- do not touch example authored units to make the registry look cleaner

Command wall:

```bash
cargo fmt --all
cargo test -p spec-cli
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Acceptance:

- repo-root benchmark files now exist and match the frozen contract
- fixture baselines lock schema-v4 truth from merged Phase 3 and Phase 4 behavior
- docs describe M68 mechanics landing accurately and keep M69 deferred
- the verification commands above are sufficient to inform fixture capture for:
  - broad-scope benchmark projection
  - benchmark-root full projection
  - namespace and single-file partial projection
  - export benchmark projection
  - snapshot output presence

### WS-INT final closeout — parent only

#### `task/i2-e-final-proof-and-closeout`

Own:

- merge `WS-D` into `WS-INT`
- final verification
- `RUN_ROOT/**`

Do:

- merge Phase 5 into `codex/i2-int`
- run the full acceptance wall from `PLAN.md`
- capture outputs under `validation/final/`
- write `final-proof-manifest.json`
- update `acceptance-ledger.md`

Final command wall:

```bash
cargo fmt --all
cargo test -p spec-core
cargo test -p spec-cli
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Final expectations:

- repo-root `status` and `export` show `BENCH-ECOM`, `BENCH-CROSSLIB`, and reserved `BENCH-SERVICE`
- benchmark-root scope shows full `BENCH-ECOM`
- namespace and single-file scope show partial `BENCH-ECOM`
- partial scope never emits positive supported credit
- companion-negative scope never emits positive supported credit
- snapshots write only under `benchmarks/snapshots/`
- readability review state projects truthfully for full positive benchmarks
- status and export agree on the same benchmark projection for the same scope

Closeout rule:

- `codex/i2-int` is the verified landing candidate branch
- do not mutate the dirty primary `main` checkout while its `PLAN.md` working copy is still being preserved as authority
- advancing `main` is a separate clean-ref operation after the dirty primary checkout has been safely reconciled

## Context-Control Rules

- The parent keeps only these live authority artifacts in working context:
  - `authority-plan.snapshot.md`
  - `contract-freeze.json`
  - `tasks.json`
  - `acceptance-ledger.md`
  - latest integration diff summary
- Each worker brief must include only:
  - task id
  - owned file set
  - relevant `PLAN.md` excerpt from `authority-plan.snapshot.md`
  - relevant `contract-freeze.json` excerpt
  - exact write scope
  - command wall
  - forbidden touch surfaces
  - worker return contract
- Workers do not write `RUN_ROOT/**`.
- Workers do not rewrite milestone scope.
- The parent reviews narrow diffs and command results only.
- Close each worker as soon as its branch is integrated.

### Worker Return Contract

Each worker must return only:

- changed files
- commands run and exit codes
- blockers or unresolved assumptions
- exact merge hotspot notes

Nothing else is required. Workers do not return long transcripts, alternate milestone designs, or rewritten scope.

## Blocked-State Protocol

Stop immediately and mark the task `blocked` if any of these happen:

1. the working-tree bytes of `PLAN.md` no longer match `authority-plan.snapshot.md`
2. Phase 1 or Phase 2 requires CLI or docs edits before the Phase 2 freeze
3. Phase 3 or Phase 4 needs to change frozen enums, field sets, digest rules, reserved-state rules, or anti-laundering behavior
4. any lane tries to create `examples/service/**`
5. any lane tries to rewrite authored specs, passports, or molecule evidence to make benchmark accounting pass
6. `spec-cli/src/commands.rs` merge cannot be resolved by the frozen ownership split
7. status and export projections diverge for the same scope
8. partial scope leaks positive credit
9. companion-negative scope leaks positive credit
10. seeded readability review cannot be anchored to the merged branch’s actual `projection_digest`

Blocked-task handling:

- worker stops without widening scope
- worker returns:
  - last green commit
  - failing command
  - exact blocker
  - proposed smallest honest unblock
- parent records `tasks/<TASK_ID>/blocker.md`
- parent either:
  - resolves inside the frozen contract, or
  - re-freezes from a refreshed authority snapshot before relaunching downstream lanes

## Tests And Acceptance

This is the consolidated execution-review matrix. It complements the per-task acceptance bullets and final exit criteria.

### Core projection contract

- Registry parse and validation pass in `spec-core`.
- Invalid registry states fail loudly and machine-readably.
- Full-scope projection covers:
  - active positive benchmark
  - companion-negative benchmark
  - reserved benchmark
- Partial-scope projection enforces:
  - omitted full-scope-only fields
  - `partial_valid` or `partial_invalid` only
  - `counts_as_supported_positive: false` always
- `label_digest` and `projection_digest` are deterministic and ordering-stable.
- Reserved `BENCH-SERVICE` full-scope state remains explicit, not omitted or greenwashed.

### Phase 3 status/export integration

- `spec status --format json` emits `schema_version: 4`.
- `spec export` emits `schema_version: 4`.
- Both surfaces include additive top-level `benchmarks[]`.
- Repo-root scope shows:
  - `BENCH-ECOM`
  - `BENCH-CROSSLIB`
  - `BENCH-SERVICE`
- Benchmark-root scope for `examples/ecommerce/units` shows full `BENCH-ECOM`.
- Namespace and single-file scopes show partial `BENCH-ECOM`.
- Status and export agree on benchmark projection for the same scope.

### Phase 4 snapshot/readability

- `spec benchmark snapshot BENCH-ECOM` writes only snapshot output.
- `spec benchmark snapshot BENCH-CROSSLIB` writes only snapshot output.
- `spec benchmark snapshot BENCH-SERVICE` writes reserved snapshot state without faking missing workload freshness.
- Readability review loading distinguishes:
  - `current`
  - `stale`
  - `missing`
  - `not_applicable`
- Full positive benchmark snapshotting validates `readability_generated_files[]` existence.

### Phase 5 seeding/fixtures/docs

- `benchmarks/labels.json` exists and matches the frozen initial roster.
- `benchmarks/reviews/BENCH-ECOM.readability.review.json` is anchored to the merged branch’s actual `projection_digest`.
- `benchmarks/snapshots/*.snapshot.json` are present for the three seeded benchmark ids.
- `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/*.json` lock the merged schema-v4 benchmark truth.
- Repo docs in `README.md`, `CHANGELOG.md`, and `TODOS.md` describe:
  - benchmark roster
  - reserved `BENCH-SERVICE` visibility
  - writer-vs-reader wall
  - M68 mechanics landing
  - M69 still deferred

### Operator/orchestration flow

- The exact working-tree `PLAN.md` was snapshotted before worktrees or implementation edits.
- `baseline.json` records the dirty primary checkout truthfully.
- `contract-freeze.json` is written only after Phase 2 integration.
- `WS-B` and `WS-C` branch from the exact `phase_2_freeze_commit`.
- `WS-D` starts only after `WS-B` and `WS-C` are integrated.
- The parent remains the only integrator for all merges and the final proof wall.
- Workers follow the explicit worker return contract.

### Workspace boundary

- No implementation work happens in the dirty primary checkout.
- No lane creates `examples/service/**`.
- No lane rewrites authored specs, passports, or molecule evidence.
- No lane invents approval artifacts or external validators not present in I2.
- No lane widens benchmark roster or support boundaries beyond `PLAN.md`.

## Acceptance And Exit Criteria

I2 is done only when all of these are true:

1. `benchmarks/labels.json` exists and validates
2. shared benchmark projection core exists in `spec-core`
3. `status` and `export` emit schema-v4 additive `benchmarks[]`
4. full versus partial path-scope behavior matches the frozen contract
5. `BENCH-SERVICE` reserved projection is visible at broad scope
6. companion-negative cases stay visible but never count as positive
7. readability review state projects correctly for full positive benchmarks
8. benchmark snapshots can be written without mutating proof truth
9. fixtures cover full, partial, invalid, reserved, and companion-negative states
10. docs explain the benchmark roster and writer-vs-reader wall truthfully

The run is incomplete if code is green but any one of those remains false.
