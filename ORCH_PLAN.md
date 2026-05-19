# I3 Orchestration Plan

Status: **authoritative execution runbook**
Authority source: **the current working-tree bytes of `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` only**
Plan title: **`I3: Rust V1 Contract Stack Mechanics Landing Plan`**
Repo root: **`/home/azureuser/__Active_Code/atomize-hq/spec`**
Base branch: **`main`**
Current checked-out branch: **`main`**
Authority validated commit in `PLAN.md`: **`3af2526`**
Observed `HEAD` at kickoff: **`3af2526e100443b0f709554f60890d4b52fd6d67`**
Observed dirty tree at kickoff: **`PLAN.md` modified**
Authority date: **`2026-05-18`**
Maximum safe worker concurrency: **2 worker lanes plus the parent integrator**
Worker model assumption: **`GPT-5.4` with `reasoning_effort=high`**
Rewrite intent: **fully replace the stale I2 `ORCH_PLAN.md` with a fresh I3 benchmark-mechanics runbook grounded only in the current I3 `PLAN.md` and live repo state**
Last rewritten: **`2026-05-18`**

## Summary

I3 lands the locked benchmark-mechanics surface and nothing broader:

`one benchmark registry, one shared benchmark projection core, schema-v4 benchmark projection in status/export, benchmark snapshots, readability-review anchoring, reserved BENCH-SERVICE visibility, and the exact anti-laundering/path-scope rules locked in PLAN.md.`

The parent agent owns the critical path and is the only integrator. The execution shape follows the current `PLAN.md` lane structure exactly:

1. freeze the exact working-tree authority version of `PLAN.md`
2. run Lane A Phase 1 in a parent-owned `spec-core` worktree
3. freeze the Phase 1 benchmark projection contract
4. launch Lane B Phase 3 and Lane C Phase 4 from that exact Phase 1 freeze
5. continue Lane A Phase 2 in parallel as the parent-owned critical path
6. integrate Lane A, then integrate Lane B and Lane C
7. launch Lane D Phase 5 only after Phases 2, 3, and 4 are all integrated
8. run the full proof wall on the integration branch
9. end with one verified landing-candidate commit on the integration branch

The chosen launch point is: **Lane B and Lane C start immediately after the Phase 1 freeze while Lane A continues Phase 2.**
This is the closest faithful reading of `PLAN.md` because:

- `PLAN.md` defines the only real architectural freeze after Phase 1
- the dependency table makes Phases 2, 3, and 4 all depend on Phase 1
- Lane A remains sequential and parent-owned through Phase 2
- the safe parallelism seam is therefore `Phase 1 frozen -> Phase 2 / Phase 3 / Phase 4 proceed with limited overlap`

Extra concurrency beyond that is intentionally rejected. Lane B and Lane C are already the maximum safe parallel CLI split, and even that split carries an explicit fallback rule if `spec-cli/src/commands.rs` or `spec-cli/tests/cli.rs` becomes too noisy.

Because the primary checkout is already on `main` with a dirty `PLAN.md`, the parent must preserve that checkout as the authority reference and use worktrees for all implementation branches. The parent may not treat the primary root as a clean integration tree.

## Assumptions

- The parent is the only integrator.
- `GPT-5.4` with `reasoning_effort=high` is the worker model assumption.
- Worktree-based execution is required because the primary checkout is dirty and must preserve the authority `PLAN.md` bytes.
- Lane A is sequential and parent-owned:
  - Phase 1 benchmark core and registry
  - then Phase 2 export integration
- The only real architecture freeze is after Phase 1.
- Lane B and Lane C may launch only after the Phase 1 freeze is recorded.
- Lane D may launch only after Phases 2, 3, and 4 are integrated.
- `benchmarks/labels.json` is a real Phase 1 deliverable, not a Phase 5 afterthought.
- `BENCH-SERVICE` remains reserved and required for truthful broad-scope projection, but it is not implemented in I3.
- `BENCH-CROSSLIB` remains visible and never positive-credit.
- `examples/ecommerce/units/**`, `examples/crosslib-app/units/**`, and `examples/shared-spec/units/**` are existing benchmark inputs, not authoring targets.
- The existing repo-root `ORCH_PLAN.md` is stale I2 structure only and may not supply milestone facts.

## Hard Guards

- The current working-tree bytes of `PLAN.md` at kickoff are the only milestone authority.
- Snapshot those exact bytes before creating worker branches.
- Do not use stale `ORCH_PLAN.md`, older milestone notes, or remembered I2 sequencing as authority.
- Phase order is fixed:
  - kickoff and authority freeze
  - Lane A Phase 1
  - Phase 1 freeze
  - Lane A Phase 2 plus Lane B Phase 3 plus Lane C Phase 4
  - Lane D Phase 5
  - final proof and closeout
- The parent is the only integrator.
- Do not widen, narrow, or reinterpret M66 support rows.
- Do not add benchmark metadata to `.unit.spec`, `.test.spec`, passports, or molecule evidence.
- Do not make `spec status`, `spec export`, or `spec benchmark snapshot` write proof truth.
- Do not implement `BENCH-SERVICE`.
- Do not create `examples/service/**`.
- Do not rewrite authored units, passports, or molecule evidence to make benchmarks look green.
- Partial scope never emits positive supported credit.
- `BENCH-CROSSLIB` never emits positive supported credit.
- Broad-scope `BENCH-SERVICE` visibility is mandatory.
- If the working-tree bytes of `PLAN.md` change after kickoff, stop and refresh the authority snapshot, freeze record, and worker briefs before more code moves.

## Current Repo Reality Freeze

Observed at kickoff:

- branch: `main`
- `HEAD`: `3af2526e100443b0f709554f60890d4b52fd6d67`
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
  - `spec-core/src/graph.rs`
  - `spec-core/src/types.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/*.json`
  - `README.md`
  - `CHANGELOG.md`
  - `TODOS.md`

Operational consequence:

- all implementation edits happen in worktrees, not in the dirty primary checkout
- all worker prompts cite the frozen `authority-plan.snapshot.md`
- final landing proof runs in the integration worktree, not in the dirty primary root

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec
WT_ROOT=/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i3-benchmark-mechanics
RUN_ROOT=$PRIMARY_ROOT/.runs/i3_rust_v1_contract_stack_mechanics
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority root | `PRIMARY_ROOT` | `main` | Parent | preserves dirty `PLAN.md`, owns durable run-state only |
| `WS-INT` | `$WT_ROOT/int` | `codex/i3-int` | Parent | integration branch and final proof branch |
| `WS-A` | `$WT_ROOT/lane-a` | `codex/i3-lane-a` | Parent | Lane A sequential Phase 1 -> Phase 2 |
| `WS-B` | `$WT_ROOT/lane-b-status` | `codex/i3-lane-b-status` | Worker | Lane B Phase 3 status integration |
| `WS-C` | `$WT_ROOT/lane-c-snapshot` | `codex/i3-lane-c-snapshot` | Worker | Lane C Phase 4 snapshot and readability integration |
| `WS-D` | `$WT_ROOT/lane-d-finalize` | `codex/i3-lane-d-finalize` | Worker | Lane D Phase 5 artifacts, fixtures, and docs reconciliation |

### Worktree creation rules

- Do not create worker worktrees before the authority snapshot is frozen.
- Create `WS-INT` and `WS-A` first from `main`.
- `WS-B` and `WS-C` must branch from the exact Phase 1 freeze commit recorded in `phase1-freeze.json`.
- `WS-A` continues Phase 2 on top of that same Phase 1 freeze commit.
- `WS-D` must branch only after Lane A, Lane B, and Lane C are all integrated into `WS-INT`.
- There is no separate worker for final integration. Final proof is parent-only in `WS-INT`.
- Never branch worker lanes from the dirty primary checkout state.
- Never check out implementation branches in `PRIMARY_ROOT`.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT" "$RUN_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b codex/i3-int main
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/lane-a" -b codex/i3-lane-a main

# after Phase 1 is integrated and phase1-freeze.json records phase_1_freeze_commit
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/lane-b-status" -b codex/i3-lane-b-status <phase_1_freeze_commit>
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/lane-c-snapshot" -b codex/i3-lane-c-snapshot <phase_1_freeze_commit>

# after WS-A, WS-B, and WS-C are integrated into codex/i3-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/lane-d-finalize" -b codex/i3-lane-d-finalize codex/i3-int
```

## Durable Orchestration State

All parent-owned run-state lives under:

```bash
$RUN_ROOT
```

This is orchestration state only, not product truth.

### Required run-state artifacts

| Path | Purpose |
| --- | --- |
| `authority-plan.snapshot.md` | exact working-tree bytes of `PLAN.md` at kickoff |
| `authority-plan.sha256` | hash of the exact authority snapshot |
| `baseline.json` | kickoff branch, `HEAD`, dirty-tree summary, observed repo reality |
| `phase1-freeze.json` | frozen Phase 1 projection contract and worker lane boundaries |
| `worktrees.json` | exact worktree paths, branches, heads, and lane states |
| `file-ownership.json` | lane write scopes and global no-touch surfaces |
| `tasks.json` | canonical task ledger and states |
| `session-log.md` | chronological launch, merge, rerun, block, and close log |
| `acceptance-ledger.md` | final gate checklist and proof references |
| `final-proof-manifest.json` | exact final commands, exit codes, and captured output paths |
| `handoffs/` | worker briefs and worker return summaries |
| `validation/kickoff/` | kickoff proof captures |
| `validation/ws-a/` | Lane A proof captures |
| `validation/ws-b/` | Lane B proof captures |
| `validation/ws-c/` | Lane C proof captures |
| `validation/ws-d/` | Lane D proof captures |
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

### Required `phase1-freeze.json` fields

- `milestone`
- `authority_plan_snapshot_path`
- `authority_plan_sha256`
- `phase_1_freeze_commit`
- `frozen_registry_shape`
- `frozen_benchmark_enums`
- `frozen_full_projection_fields`
- `frozen_partial_projection_omissions`
- `frozen_digest_rules`
- `frozen_summary_rules`
- `frozen_reserved_state`
- `frozen_path_scope_rules`
- `frozen_anti_laundering_rule`
- `frozen_readability_selection_rules`
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

Run this in order. Do not skip ahead.

1. Freeze authority bytes.
- Read the current working-tree `PLAN.md`.
- Write those exact bytes to `authority-plan.snapshot.md`.
- Write its SHA-256 to `authority-plan.sha256`.
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
- Save raw command outputs under `validation/kickoff/`.

3. Bootstrap parent-owned worktrees.
- Create `WS-INT` from `main`.
- Create `WS-A` from `main`.
- Initialize:
  - `worktrees.json`
  - `file-ownership.json`
  - `tasks.json`
  - `acceptance-ledger.md`

4. Run Lane A Phase 1 in `WS-A`.
- Execute `task/i3-a1-phase1-registry-core`.
- Keep scope limited to the shared benchmark core and real registry file shape.

5. Integrate the Phase 1 freeze point.
- Merge the Phase 1 slice from `WS-A` into `WS-INT`.
- Write `phase1-freeze.json`.
- Record the exact `phase_1_freeze_commit`.
- Freeze the Phase 1 projection contract before any worker lane starts.

6. Launch Lane B and Lane C from the exact Phase 1 freeze.
- Create `WS-B` from `phase_1_freeze_commit`.
- Create `WS-C` from `phase_1_freeze_commit`.
- Emit worker briefs from `handoffs/`.

7. Continue Lane A Phase 2 in `WS-A`.
- Execute `task/i3-a2-phase2-export`.
- Keep Lane A parent-owned and sequential.

8. Integrate Lane A first.
- Merge the completed Lane A Phase 2 branch into `WS-INT`.
- Record the merge in `session-log.md` and `tasks.json`.

9. Integrate Lane B and Lane C.
- Merge each completed worker lane into `WS-INT` using the ownership split and fallback rules below.
- Do not launch Lane D until all three lanes are integrated.

10. Launch Lane D finalization.
- Create `WS-D` from the merged `WS-INT`.
- Execute deterministic artifact, fixture, and docs reconciliation only.

## Workstream Plan

### WS-BASELINE — parent only, no implementation edits

#### `task/i3-00-authority-snapshot`

Own:

- `RUN_ROOT/**` only

Do:

- snapshot the exact working-tree bytes of `PLAN.md`
- hash that snapshot
- record kickoff branch, `HEAD`, and dirty tree
- record the observed absence of `benchmarks/`
- record the observed absence of `examples/service/units`
- record stale `ORCH_PLAN.md` as excluded context only

Acceptance:

- `authority-plan.snapshot.md` exists
- `authority-plan.sha256` exists
- `baseline.json` captures dirty-tree reality accurately
- no implementation files changed

#### `task/i3-01-bootstrap-worktrees`

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

### WS-A (`codex/i3-lane-a`) — parent only, sequential

#### `task/i3-a1-phase1-registry-core`

Own only:

- `spec-core/src/benchmark.rs` new
- `spec-core/src/lib.rs`
- `spec-core/src/types.rs` only if required for benchmark type wiring
- `spec-core/src/graph.rs` only if required for benchmark traversal wiring
- `spec-core/src/passport.rs` only if required for read-only proof projection access
- `spec-core/src/molecule_evidence.rs` only if required for read-only proof projection access
- `benchmarks/labels.json`

Must not touch:

- `spec-core/src/export.rs`
- `spec-cli/**`
- `benchmarks/reviews/**`
- `benchmarks/snapshots/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Deliver:

- typed benchmark enums and structs
- registry loader and validator for `benchmarks/labels.json`
- shared case projection record
- benchmark-level projection record
- path-scope classifier
- summary formulas
- deterministic digest builders
- readability-generated file selection rules
- initial `benchmarks/labels.json` with:
  - `BENCH-ECOM`
  - `BENCH-SERVICE`
  - `BENCH-CROSSLIB`

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
```

Acceptance:

- core types compile
- registry validation tests pass
- the benchmark core can produce full and partial projections without CLI printing logic
- `benchmarks/labels.json` exists and matches the frozen roster
- no CLI, snapshot, or docs surfaces are touched

#### `task/i3-a1-freeze-and-launch`

Own:

- merge the Phase 1 slice into `WS-INT`
- `RUN_ROOT/**`

Do:

- integrate the Phase 1 freeze point into `codex/i3-int`
- record `phase_1_freeze_commit`
- write `phase1-freeze.json`
- freeze the exact registry shape
- freeze the exact full-projection field set
- freeze the exact partial-projection omission set
- freeze enum values, digest rules, summary rules, path-scope rules, reserved-state rules, anti-laundering rule, and readability-selection rules
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
- allow `WS-A` to continue Phase 2 on top of the same frozen Phase 1 commit

Acceptance:

- `phase1-freeze.json` exists and is complete
- `WS-B` and `WS-C` branch from `phase_1_freeze_commit`
- no worker starts from any earlier `main` head

#### `task/i3-a2-phase2-export`

Own only:

- `spec-core/src/export.rs`
- export tests colocated with `spec-core/src/export.rs`

Read-only dependencies:

- `spec-core/src/benchmark.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/molecule_evidence.rs`
- `authority-plan.snapshot.md`
- `phase1-freeze.json`

Must not touch:

- `spec-cli/**`
- `benchmarks/reviews/**`
- `benchmarks/snapshots/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Deliver:

- `spec export` schema bump to `schema_version: 4`
- additive top-level `benchmarks[]`
- full versus partial benchmark entries mirrored exactly from the frozen shared core
- no regression to existing unit, passport, graph, or plan export behavior

Command wall:

```bash
cargo fmt --all
cargo test -p spec-core
cargo run -p spec-cli -- export .
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
```

Acceptance:

- export bundle tests pass
- `benchmarks[]` is additive, not disruptive
- no benchmark logic is duplicated outside the shared core
- no CLI or docs surfaces are changed in this lane

### WS-B (`codex/i3-lane-b-status`) — worker 1, Phase 3

#### `task/i3-b-phase3-status`

Own only:

- `spec-cli/src/commands.rs` status-owned regions
- `spec-cli/tests/cli.rs` status-owned assertions
- `spec-cli/tests/fixtures/status-*.json`

Read-only dependencies:

- `spec-core/src/benchmark.rs`
- `spec-core/src/export.rs`
- `authority-plan.snapshot.md`
- `phase1-freeze.json`

Must not touch:

- snapshot subcommand parsing or dispatch in `spec-cli/src/commands.rs`
- snapshot writing helpers
- readability review loading helpers
- `benchmarks/reviews/**`
- `benchmarks/snapshots/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `spec-core/**`

Deliver:

- `spec status --format json` schema bump to `schema_version: 4`
- additive top-level `benchmarks[]` in status JSON
- repo-root full benchmark projection
- benchmark-root full benchmark projection
- namespace and single-file partial benchmark projection
- explicit broad-scope reserved `BENCH-SERVICE` visibility
- status fixture baselines for full, partial, reserved, invalid, and companion-negative status surfaces

Command wall:

```bash
cargo fmt --all
cargo test -p spec-cli
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
```

Acceptance:

- repo-root, benchmark-root, namespace, and single-file status fixtures all match
- partial scope omits whole-benchmark fields and forces `counts_as_supported_positive=false`
- no `spec-core`, snapshot, or docs surfaces are changed in this lane

### WS-C (`codex/i3-lane-c-snapshot`) — worker 2, Phase 4

#### `task/i3-c-phase4-snapshot-readability`

Own only:

- `spec-cli/src/commands.rs` snapshot-owned regions
- `spec-cli/tests/cli.rs` snapshot/readability-owned assertions
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`

Read-only dependencies:

- `spec-core/src/benchmark.rs`
- `spec-core/src/export.rs`
- `authority-plan.snapshot.md`
- `phase1-freeze.json`

Must not touch:

- status JSON schema constant
- status-only benchmark emission helpers
- status-owned assertions in `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/status-*.json`
- `benchmarks/labels.json`
- `benchmarks/snapshots/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `spec-core/**`

Deliver:

- `spec benchmark snapshot <benchmark-id>`
- snapshot artifact writing under `benchmarks/snapshots/`
- readability review file loading
- full-scope readability status and verdict projection for positive benchmarks
- reserved snapshot behavior for `BENCH-SERVICE`
- snapshot and readability assertions that do not require editing Lane B status fixtures

Command wall:

```bash
cargo fmt --all
cargo test -p spec-cli
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Acceptance:

- snapshot command writes only snapshot files
- active positive snapshots validate readability-generated file existence
- reserved `BENCH-SERVICE` snapshot path does not fake generated-tree freshness
- no `spec-core`, status-fixture, or docs surfaces are changed in this lane

## Lane B / Lane C Ownership Split And Merge Protocol

This split is mandatory and must be written into `file-ownership.json` before worker launch.

### `spec-cli/src/commands.rs` ownership

Lane B owns only:

- existing `status` command code path
- `status` JSON schema version constant
- status benchmark projection wiring
- status-only serializer or formatter helpers
- status-only error handling for benchmark projection

Lane C owns only:

- `benchmark snapshot` parser, dispatch, and command branch
- snapshot file writer helpers
- readability review loading helpers
- snapshot-specific error handling
- snapshot-specific success output

Neither lane may:

- refactor shared CLI scaffolding outside its owned command surface
- move the other lane’s entrypoint
- rewrite shared helper placement for aesthetics
- widen ownership by “cleanup” edits

### `spec-cli/tests/cli.rs` ownership

Lane B owns only:

- status benchmark tests
- status fixture assertion blocks
- status scope matrix assertions

Lane C owns only:

- snapshot command tests
- readability review assertions
- snapshot failure-mode assertions

Lane C must not edit Lane B’s status assertion blocks.
Lane B must not edit Lane C’s snapshot assertion blocks.

### Fixture ownership during worker execution

Lane B owns during worker execution:

- `spec-cli/tests/fixtures/status-*.json`

Lane C must not edit those fixtures. If Phase 4 behavior appears to require a change to a Lane B-owned status fixture, that is not a silent overlap. It is a declared orchestration conflict.

### Merge protocol

1. The parent records the ownership split in `file-ownership.json`.
2. Each worker brief quotes only its owned surfaces.
3. The parent reviews worker diffs against ownership before merge.
4. Preferred merge order is:
   - Lane A first
   - then Lane B
   - then Lane C
5. If Lane C requires edits inside Lane B-owned `commands.rs` regions, Lane B-owned `cli.rs` regions, or `status-*.json` fixtures:
   - the worker stops
   - the parent records the conflict in `tasks/<TASK_ID>/blocker.md` and `session-log.md`
   - the parent marks `tasks.json` with `blocked`
   - Lane C is collapsed behind Lane B sequentially only after that declaration
6. The same fallback applies if merge conflict noise in `spec-cli/src/commands.rs` or `spec-cli/tests/cli.rs` exceeds the recorded ownership split.

### Sequential fallback rule

If the ownership split cannot be maintained honestly, the parent must not improvise an unsafe merge. The fallback is:

1. record a conflict declaration in run-state
2. integrate Lane B first
3. rebase or recreate Lane C from the post-B integration head
4. complete Lane C sequentially
5. resume the normal integration sequence

This fallback is allowed only after explicit parent declaration in run-state. It is not a worker choice.

### WS-INT (`codex/i3-int`) — parent integration branch

#### `task/i3-int-merge-lanes`

Own:

- integration merges only
- `RUN_ROOT/**`

Do:

- integrate Lane A after Phase 2 is green
- integrate Lane B and Lane C using the ownership split above
- reject any worker change that mutates frozen Phase 1 contract surfaces without an explicit parent re-freeze
- keep `phase1-freeze.json` truthful after every merge

Acceptance:

- `codex/i3-int` contains Phase 1, Phase 2, Phase 3, and Phase 4
- the merged tree preserves the Phase 1 freeze unchanged
- any fallback from parallel to sequential CLI work is recorded explicitly in run-state

### WS-D (`codex/i3-lane-d-finalize`) — worker after Phases 2, 3, and 4 are integrated

#### `task/i3-d-phase5-finalize`

Own only:

- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/snapshots/*.snapshot.json`
- `spec-cli/tests/fixtures/*.json`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Read-only dependencies:

- merged `codex/i3-int`
- `benchmarks/labels.json`
- `spec-cli/tests/cli.rs`
- `phase1-freeze.json`

Must not touch:

- `spec-core/**`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `benchmarks/labels.json`
- `examples/ecommerce/units/**`
- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- passports
- molecule evidence

Deliver:

- canonical snapshots for:
  - `BENCH-ECOM`
  - `BENCH-CROSSLIB`
  - `BENCH-SERVICE`
- final readability review artifact bytes anchored to the merged branch’s actual `projection_digest`
- deterministic fixture reconciliation across `spec-cli/tests/fixtures/*.json`
- docs updated in:
  - `README.md`
  - `CHANGELOG.md`
  - `TODOS.md`

### Phase 5 fixture reconciliation rule

Lane D is allowed to modify fixture files only as deterministic artifact refresh on top of the already-merged code and test surfaces.

Lane D may:

- recapture fixture bytes from merged command outputs
- normalize fixture baselines to the merged truth
- add missing benchmark fixture files required by the merged surface

Lane D may not:

- change CLI code
- change test logic in `spec-cli/tests/cli.rs`
- change schema fields
- change benchmark projection semantics
- reopen Phase 1 freeze decisions
- fix fixture drift by editing code elsewhere

If fixture reconciliation requires code changes or test-logic changes, Lane D must stop and the parent must declare a blocker.

Hard guards:

- do not create `examples/service/**`
- do not widen benchmark roster beyond the frozen plan roster
- do not change required molecule ids beyond the frozen pair
- do not move benchmark truth into specs, passports, or evidence
- do not reopen code or contract surfaces under the banner of “fixture refresh”

Command wall:

```bash
cargo fmt --all
cargo test -p spec-cli
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- export .
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Acceptance:

- snapshot files exist and are reproducible from the merged branch
- fixture baselines match merged command truth
- docs explain the benchmark roster, reserved-gate truth, readability anchoring, and writer-versus-reader wall accurately
- no code surfaces outside the allowed write scope are touched

### WS-INT final closeout — parent only

#### `task/i3-e-final-proof-and-closeout`

Own:

- merge `WS-D` into `WS-INT`
- final verification
- `RUN_ROOT/**`

Do:

- merge Phase 5 into `codex/i3-int`
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
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- export .
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
- readability-review state projects truthfully for full positive benchmarks
- status and export agree on the same benchmark projection for the same scope

Closeout rule:

- `codex/i3-int` is the verified landing-candidate branch
- do not mutate the dirty primary `main` checkout while its `PLAN.md` working copy is still being preserved as authority
- advancing `main` is a separate clean-ref operation after the dirty primary checkout has been safely reconciled

## Parent-Vs-Worker Context Control

- The parent keeps only these live authority artifacts in active context:
  - `authority-plan.snapshot.md`
  - `phase1-freeze.json`
  - `tasks.json`
  - `acceptance-ledger.md`
  - latest integration diff summary
- Each worker brief must include only:
  - task id
  - owned file set
  - relevant `PLAN.md` excerpt from `authority-plan.snapshot.md`
  - relevant `phase1-freeze.json` excerpt
  - exact write scope
  - command wall
  - forbidden touch surfaces
  - worker return contract
- Workers do not write `RUN_ROOT/**`.
- Workers do not rewrite milestone scope.
- Workers do not independently reinterpret freeze rules.
- The parent reviews narrow diffs and command results only.
- Close each worker as soon as its branch is integrated.

### Worker return contract

Each worker must return only:

- changed files
- commands run and exit codes
- blockers or unresolved assumptions
- exact merge-hotspot notes

Workers do not return long transcripts, alternative milestone designs, or rewritten authority facts.

## Stop Rules And Blocked-State Protocol

Stop immediately and mark the task `blocked` if any of these happen:

1. the working-tree bytes of `PLAN.md` no longer match `authority-plan.snapshot.md`
2. any post-freeze lane needs to change frozen enums, registry shape, field sets, digest rules, summary rules, reserved-state rules, anti-laundering behavior, path-scope behavior, or readability-selection behavior
3. any lane tries to create `examples/service/**`
4. any lane tries to rewrite authored specs, passports, or molecule evidence to make benchmark accounting pass
5. any lane edits outside its explicit write scope
6. `BENCH-SERVICE` disappears from broad-scope status or export
7. partial scope leaks positive credit
8. companion-negative scope leaks positive credit
9. snapshotting mutates passports or molecule evidence
10. status and export diverge for the same scope
11. Lane C requires edits inside Lane B-owned status regions or fixtures without a recorded fallback declaration
12. Phase 5 fixture reconciliation requires code changes or test-logic changes
13. the final proof wall fails on any command

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
  - refreshes the authority snapshot and re-freezes before relaunching downstream lanes

## Tests And Acceptance

### Core projection contract

- Registry parse and validation pass in `spec-core`.
- Invalid registry states fail loudly and machine-readably.
- Full-scope projection covers:
  - active positive benchmark
  - companion-negative benchmark
  - reserved benchmark
- Partial-scope projection enforces:
  - omitted full-scope-only fields
  - `counts_as_supported_positive: false` always
- `label_digest` and `projection_digest` are deterministic and ordering-stable.
- Reserved `BENCH-SERVICE` full-scope state remains explicit, not omitted or greenwashed.

### Phase 2 export integration

- `spec export` emits `schema_version: 4`.
- Export includes additive top-level `benchmarks[]`.
- Repo-root scope shows:
  - `BENCH-ECOM`
  - `BENCH-CROSSLIB`
  - `BENCH-SERVICE`
- Benchmark-root scope for `examples/ecommerce/units` shows full `BENCH-ECOM`.
- Single-file export scope shows partial benchmark projection.
- Export agrees with the frozen shared core exactly.

### Phase 3 status integration

- `spec status --format json` emits `schema_version: 4`.
- Status includes additive top-level `benchmarks[]`.
- Repo-root scope shows:
  - `BENCH-ECOM`
  - `BENCH-CROSSLIB`
  - `BENCH-SERVICE`
- Benchmark-root scope shows full `BENCH-ECOM`.
- Namespace and single-file scopes show partial `BENCH-ECOM`.
- Status fixtures cover full, partial, invalid, reserved, and companion-negative states.

### Phase 4 snapshot/readability integration

- `spec benchmark snapshot BENCH-ECOM` writes only snapshot output.
- `spec benchmark snapshot BENCH-CROSSLIB` writes only snapshot output.
- `spec benchmark snapshot BENCH-SERVICE` writes reserved snapshot state without faking missing workload freshness.
- Readability-review loading distinguishes:
  - `current`
  - `stale`
  - `missing`
  - `not_applicable`

### Phase 5 finalization

- `benchmarks/reviews/BENCH-ECOM.readability.review.json` is anchored to the merged branch’s actual `projection_digest`.
- `benchmarks/snapshots/*.snapshot.json` are present for the three seeded benchmark ids.
- fixture baselines across `spec-cli/tests/fixtures/*.json` match the merged benchmark truth
- docs in `README.md`, `CHANGELOG.md`, and `TODOS.md` describe:
  - benchmark roster
  - reserved `BENCH-SERVICE` visibility
  - writer-versus-reader wall
  - readability anchoring
  - M68 mechanics landing
  - M69 still deferred

### Operator/orchestration flow

- The exact working-tree `PLAN.md` was snapshotted before worktrees or implementation edits.
- `baseline.json` records the dirty primary checkout truthfully.
- `phase1-freeze.json` is written only after Phase 1 integration.
- `WS-B` and `WS-C` branch from the exact `phase_1_freeze_commit`.
- Lane A continues Phase 2 after the Phase 1 freeze while B and C run in parallel.
- Lane D starts only after Phases 2, 3, and 4 are integrated.
- The parent remains the only integrator for all merges and the final proof wall.
- Workers follow the explicit return contract and file-ownership wall.

### Workspace boundary

- No implementation work happens in the dirty primary checkout.
- No lane creates `examples/service/**`.
- No lane rewrites authored specs, passports, or molecule evidence.
- No lane widens benchmark roster or support boundaries beyond `PLAN.md`.
- No lane uses stale I2 branch names, commits, or sequencing.

## Acceptance And Exit Criteria

I3 is done only when all of these are true:

1. `benchmarks/labels.json` exists, validates, and carries the locked `BENCH-ECOM` / `BENCH-SERVICE` / `BENCH-CROSSLIB` roster
2. shared benchmark projection core exists in `spec-core`
3. `spec status --format json` emits schema-v4 additive `benchmarks[]`
4. `spec export` emits schema-v4 additive `benchmarks[]`
5. full versus partial path-scope behavior matches the frozen contract
6. `BENCH-SERVICE` reserved projection is visible at broad scope
7. companion-negative cases stay visible but never count as positive
8. readability-review state projects correctly for full positive benchmarks
9. benchmark snapshots can be written without mutating proof truth
10. CLI fixtures cover full, partial, invalid, reserved, and companion-negative states
11. docs explain the benchmark roster and writer-versus-reader wall truthfully

The run is incomplete if code is green but any one of those remains false.
