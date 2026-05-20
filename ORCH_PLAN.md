# I5 Orchestration Plan

Status: **authoritative execution runbook**  
Milestone: **I5 Rust V1 supported-core closure**  
Frozen basis: **`main` at `1dbff70`**  
Working parent branch: **`codex/i5-prep`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Last rewritten: **2026-05-20**

## Summary

- Execute from `/home/azureuser/__Active_Code/atomize-hq/spec`.
- Treat `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` as the only milestone authority.
- Treat the existing `/home/azureuser/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` as historical context only.
- The parent agent is the only integrator, the only merge authority, and the only owner of final acceptance.
- I5 closes exactly four things and nothing broader:
  1. `BENCH-ECOM` must deliberately require `pricing/discount_strategy_checkout_flow`.
  2. `BENCH-CROSSLIB` must stop carrying active untested cases.
  3. the shipped supported-boundary rejection wall must be frozen behind one deliberate closure suite.
  4. `BENCH-ECOM` readability must become current again and stay regression-protected.
- Parallelize only the disjoint implementation lanes:
  - Lane A: positive benchmark closure
  - Lane B: companion-negative closure
  - Lane C: supported-boundary rejection wall
- Launch Lane D only after A, B, and C are merged and the live benchmark wall is stable.
- Cap concurrency at **3 active worker lanes**. Do not exceed A, B, and C in parallel.
- Keep the critical path in the parent for:
  - basis freeze
  - shared-suite scaffolding
  - fixture partition freeze
  - worktree and branch setup
  - merge order
  - gate decisions
  - final artifact refresh launch
  - final proof wall

## Starting Truth

- Current repo root is `/home/azureuser/__Active_Code/atomize-hq/spec`.
- Current branch is `main`.
- Current commit is `1dbff70`.
- `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs` does **not** exist yet.
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/reviews/BENCH-ECOM.readability.review.json` exists but is stale against the live projection digest.
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/BENCH-ECOM.snapshot.json` exists.
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json` exists.
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/BENCH-SERVICE.snapshot.json` exists and remains reserved-only context.
- `/home/azureuser/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec` and its evidence file already exist.
- `/home/azureuser/__Active_Code/atomize-hq/spec/examples/crosslib-app/units/pricing/calculate_total.unit.spec` and `/home/azureuser/__Active_Code/atomize-hq/spec/examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec` exist and are the live untested active companion-negative cases called out by `PLAN.md`.
- `/home/azureuser/__Active_Code/atomize-hq/spec/TODOS.md` still describes `M69` with stale benchmark-expansion wording and must be updated during closeout.

## Hard Guards

- Do not implement `BENCH-SERVICE`.
- Do not author `examples/service/**`.
- Do not widen Rust V1 support rows.
- Do not promote new semantic families.
- Do not reopen the I3.5/I4 command-scope contract.
- Repo-root `status . --format json` must remain `scope_authority: "inventory_only"`.
- Repo-root `export .` must remain unsupported for this workspace shape.
- Reuse the M68 writer/reader boundary exactly.
- Treat readability as a closure gate, not a support classifier.
- Keep benchmark accounting label-driven.
- Do not invent a new rejection taxonomy for Phase 3.
- Do not redesign benchmark schemas, projection shapes, or artifact families.
- `benchmarks/labels.json` belongs to Lane A only during parallel work.
- `benchmarks/reviews/BENCH-ECOM.readability.review.json` belongs to Lane D only.
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json` and `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json` belong to Lane D only for committed refresh.
- `TODOS.md` belongs to Lane D only.
- Shared benchmark fixture refresh for final committed full-output files belongs to Lane D only.
- The parent is the only integrator and the only lane allowed to resolve cross-lane conflicts.

## Worktree And Branch Plan

Create the I5 worktree root once:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5
```

Freeze the basis explicitly before creating any I5 branch:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse --abbrev-ref HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse --short HEAD
```

The expected basis is `main` at `1dbff70`. If `main` has moved, stop and either re-freeze the plan against the new SHA or re-check out `1dbff70` as the run basis before branching.

Create the parent worktree first and only from `main`:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/parent -b codex/i5-prep main
```

Run the parent freeze task on `codex/i5-prep` before any other worktree exists. That freeze task must:

- create `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/`
- seed `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`
- freeze the lane section markers inside that suite
- freeze the lane-specific fixture partition policy
- commit that parent-owned scaffolding on `codex/i5-prep`

Only after that parent-owned scaffold commit exists may the parent create the remaining worktrees from `codex/i5-prep`:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/int -b codex/i5-int codex/i5-prep
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-a -b codex/i5-lane-a-ecom codex/i5-prep
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-b -b codex/i5-lane-b-crosslib codex/i5-prep
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-c -b codex/i5-lane-c-boundary codex/i5-prep
```

Do **not** create Lane D yet. Create it only after Gate 1 passes:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-d -b codex/i5-lane-d-closeout codex/i5-int
```

## Orchestration State

Canonical run state lives under:

- `I5_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5`
- queue: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/tasks.json`
- session log: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/session-log.md`
- per-task state: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/task/<task-id>/`

The parent owns all writes under `.runs/i5/`. Workers do not update orchestration state directly.

Per-task sentinel files are zero-byte files inside `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/task/<task-id>/`:

| Sentinel | Meaning |
| --- | --- |
| `QUEUED` | task frozen, not started |
| `RUNNING` | worker or parent actively executing |
| `BLOCKED` | waiting on parent decision or prior dependency |
| `READY` | lane believes acceptance is satisfied and requests merge |
| `MERGED` | parent merged the task result |
| `REJECTED` | parent rejected the lane result or bounced it back |

Each task directory also carries:

- `scope.md` with exact owned files and forbidden surfaces
- `acceptance.md` with exact commands and expected outcomes
- `handoff.md` with the lane summary the worker returns
- `decisions.md` for parent-only gate outcomes and bounce-back notes

`tasks.json` is the authoritative execution queue. Every task row should include:

- `task_id`
- `branch`
- `worktree`
- `owner`
- `depends_on`
- `owned_paths`
- `forbidden_paths`
- `required_commands`
- `status`

## Lane Map

| Lane | Branch | Worktree | Owned write set | Goal |
| --- | --- | --- | --- | --- |
| Parent | `codex/i5-prep` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/parent` | `.runs/i5/**`, parent-owned shared-suite prelude/helpers, lane section markers, fixture-partition freeze | freeze basis and orchestrate |
| Lane A | `codex/i5-lane-a-ecom` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-a` | `benchmarks/labels.json`, `examples/ecommerce/units/**`, Lane A-owned closure fixture files, Lane A section only in `spec-cli/tests/rust_v1_closure.rs` | Phase 1 positive benchmark closure |
| Lane B | `codex/i5-lane-b-crosslib` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-b` | `examples/crosslib-app/units/**`, Lane B-owned closure fixture files, Lane B section only in `spec-cli/tests/rust_v1_closure.rs` | Phase 2 companion-negative closure |
| Lane C | `codex/i5-lane-c-boundary` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-c` | `spec-core/src/validator.rs`, `spec-core/src/benchmark.rs`, `spec-cli/src/commands.rs`, Lane C-owned dedicated closure fixtures, Lane C section only in `spec-cli/tests/rust_v1_closure.rs` | Phase 3 supported-boundary rejection wall |
| Lane D | `codex/i5-lane-d-closeout` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-d` | `benchmarks/reviews/BENCH-ECOM.readability.review.json`, impacted snapshot files, final benchmark fixtures, `TODOS.md` | Phase 4 readability currentness and closeout |
| Integration | `codex/i5-int` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/int` | merge-only plus minimal parent reconciliation | merge and final acceptance |

## Fixture Partition Policy

This policy is frozen by the parent before fan-out and is not invented at runtime.

1. Lane A and Lane B do **not** edit the same full-output benchmark fixture files in parallel.
2. Shared committed benchmark fixture refresh is deferred to Lane D only.
3. During parallel work, Lane A and Lane B may add or edit only lane-local closure fixtures under lane-specific subtrees, for example:
   - `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_a/**`
   - `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_b/**`
4. Lane C may add or edit only its own dedicated closure fixtures under:
   - `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_c/**`
5. Existing shared full-output benchmark fixtures such as the repo’s general `status-*.json` and `export-*.json` files are read-only until Lane D.
6. Lane A and Lane B validate their work against live `status` and `export` output plus their own lane-local adversarial closure fixtures, not by refreshing shared committed full-output fixtures.
7. If a lane believes a shared full-output fixture must change earlier, that is a bounce-back request to the parent, not permission to edit the shared fixture file.
8. Lane D is the only lane allowed to refresh the final committed benchmark fixture outputs after A, B, and C are integrated.

This partition is the default even if it costs some duplication in lane-local fixture files. Avoiding merge fights is more important than minimizing temporary fixture count.

## Shared Suite Ownership

`/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs` is a seeded shared suite with strict section ownership.

- The parent owns the common prelude, imports, helpers, and suite skeleton.
- The parent seeds and commits that skeleton before any worker lane is created.
- Lane A owns only the Lane A section.
- Lane B owns only the Lane B section.
- Lane C owns only the Lane C section.
- No worker may edit the common prelude/helpers.
- If any lane believes shared helpers must change, it must stop and bounce that request back to the parent.
- The parent either:
  - applies the helper change on `codex/i5-prep` and rebases affected lanes, or
  - rejects the helper change as unnecessary or out of scope.
- Any lane that edits another lane’s section or the parent-owned common block is rejected and bounced back.

## Snapshot Policy

Committed snapshot refresh is reserved to Lane D and the final parent wall.

- Lane A does not run `cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM`.
- Lane B does not run `cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB`.
- Gate 1 does not rely on committed snapshot refresh.
- Lane D performs the deliberate committed refresh of:
  - `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
  - `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- The parent may run the benchmark snapshot commands again during final acceptance after Lane D lands. At that point they are verification of clean final state, not a new source of truth. If they produce a diff, final acceptance fails and the run returns to Lane D.

## Workstream Plan

### WS-PARENT (`codex/i5-prep`) — parent only

Task id: `i5-a-freeze-basis`

1. Basis freeze
- Confirm `main` is at `1dbff70`.
- Create `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/parent` on `codex/i5-prep` from `main`.
- Create `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/`.
- Record the basis SHA and the live I5 gap list in `session-log.md`.

2. Shared-suite and partition freeze
- Seed `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`.
- Seed parent-owned common prelude/helpers and explicit section markers for:
  - Lane A
  - Lane B
  - Lane C
- Freeze the fixture partition policy under `spec-cli/tests/fixtures/benchmarks/rust_v1_closure/`.
- Create any empty lane-local fixture subtree scaffolding that workers are expected to inherit.
- Freeze lane scope and acceptance documents under `.runs/i5/task/<task-id>/`.
- Commit the parent-owned scaffold on `codex/i5-prep`.

3. Post-freeze worktree creation
- Only after the scaffold commit exists, create:
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/int`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-a`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-b`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i5/lane-c`

Acceptance for `i5-a-freeze-basis`:

- `tasks.json` exists and reflects the full lane map.
- `session-log.md` records the frozen basis SHA and the I5 gap list.
- `spec-cli/tests/rust_v1_closure.rs` exists on `codex/i5-prep` with parent-owned prelude/helpers plus lane section markers.
- Any lane-local fixture subtree scaffolding needed for the partition policy exists on `codex/i5-prep`.
- The scaffold commit exists before `codex/i5-int` or any worker branch is created.
- Each lane has a bounded `scope.md` and `acceptance.md`.
- No worker needs to guess owned files, shared-suite rules, snapshot policy, or fixture policy.

Task id: `i5-b-launch-abc`

- Create the integration and A/B/C worktrees from `codex/i5-prep`.
- Launch Lane C first only in the sense that it receives the shared-suite contract packet first.
- Launch Lane A and Lane B immediately after the parent confirms the scaffold commit is the basis of their branches.
- Mark the relevant task sentinels `RUNNING`.
- Record each launch in `session-log.md`.

### WS-A (`codex/i5-lane-a-ecom`) — worker lane A

Task id: `i5-c-phase1-bench-ecom`

Own only:

- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/labels.json`
- `/home/azureuser/__Active_Code/atomize-hq/spec/examples/ecommerce/units/**`
- Lane A-owned closure fixtures under `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_a/**`
- Lane A section only inside `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`

Required work:

- Add `pricing/discount_strategy_checkout_flow` to `BENCH-ECOM.required_molecule_ids`.
- Preserve the current `BENCH-ECOM` supported-case roster unless there is a proven mislabel that the parent explicitly approves.
- Refresh the proof surfaces needed to keep the positive benchmark truthful:
  - `pricing_quote.unit.spec`
  - `discount_strategy.unit.spec`
  - `discount_strategy_checkout_flow.test.spec`
- Add phase-1 closure assertions that `BENCH-ECOM` becomes non-passing when `pricing/discount_strategy_checkout_flow` is required but missing, stale, or failing.
- Use only Lane A-owned closure fixtures if adversarial fixture inputs are needed.
- Validate against live `status` and `export` output. Do not refresh committed snapshots or shared full-output fixtures.

Forbidden work:

- no edits under `/home/azureuser/__Active_Code/atomize-hq/spec/examples/crosslib-app/units/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/benchmark.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/reviews/BENCH-ECOM.readability.review.json`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/*.json`
- no edits to shared benchmark fixture files outside the Lane A closure subtree
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/TODOS.md`
- no edits outside the Lane A section of `spec-cli/tests/rust_v1_closure.rs`

Worker verification:

```bash
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo test -p spec-cli rust_v1_closure
```

Acceptance:

- `BENCH-ECOM.required_molecule_ids` deliberately includes `pricing/discount_strategy_checkout_flow`.
- The three targeted proof-refresh commands above pass.
- `BENCH-ECOM` still presents as passing on live benchmark-root output.
- The phase-1 closure suite makes missing or stale seam proof observable as a regression.
- The lane diff stays inside the Lane A write set.

Bounce-back rule:

- If Lane A concludes a supported-case roster change is required beyond the planned required-molecule addition, it must stop, mark `BLOCKED`, and return a concrete parent decision request. Lane A must not silently widen or relabel support on its own authority.
- If Lane A believes the common suite helpers must change, it must stop and return that request to the parent. It may not edit the parent-owned common block directly.

### WS-B (`codex/i5-lane-b-crosslib`) — worker lane B

Task id: `i5-d-phase2-bench-crosslib`

Own only:

- `/home/azureuser/__Active_Code/atomize-hq/spec/examples/crosslib-app/units/**`
- Lane B-owned closure fixtures under `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_b/**`
- Lane B section only inside `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`

Required work:

- Refresh proof for the two currently untested active companion-negative cases:
  - `pricing/calculate_total`
  - `pricing/checkout_nested_chain3`
- Preserve the rule that companion-negative cases remain visible but contribute zero positive credit.
- Add phase-2 closure assertions that an active companion-negative case without current proof makes `BENCH-CROSSLIB` incomplete.
- Add phase-2 closure assertions that companion-negative cases never increment `positive_credit_cases`.
- Use only Lane B-owned closure fixtures if adversarial fixture inputs are needed.
- Validate against live `status` and `export` output. Do not edit `benchmarks/labels.json`, committed snapshots, or shared full-output fixtures.

Forbidden work:

- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/labels.json`
- no edits under `/home/azureuser/__Active_Code/atomize-hq/spec/examples/ecommerce/units/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/benchmark.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/reviews/BENCH-ECOM.readability.review.json`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/*.json`
- no edits to shared benchmark fixture files outside the Lane B closure subtree
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/TODOS.md`
- no edits outside the Lane B section of `spec-cli/tests/rust_v1_closure.rs`

Worker verification:

```bash
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- export examples/crosslib-app/units
cargo test -p spec-cli rust_v1_closure
```

Acceptance:

- The two targeted crosslib proof-refresh commands for `calculate_total` and `checkout_nested_chain3` pass.
- `BENCH-CROSSLIB` becomes complete instead of incomplete on live benchmark-root output.
- `BENCH-CROSSLIB` still reports zero positive credit.
- The phase-2 closure suite makes missing active companion proof observable as a regression.
- The lane diff stays inside the Lane B write set.

Bounce-back rule:

- If Lane B believes a crosslib case should stop being active companion proof, it must not change `benchmarks/labels.json` directly. It returns a concrete parent request with the exact case id and rationale. The parent either rejects that scope drift or applies the minimal label change during post-merge integration.
- If Lane B believes the common suite helpers must change, it must stop and return that request to the parent. It may not edit the parent-owned common block directly.

### WS-C (`codex/i5-lane-c-boundary`) — worker lane C

Task id: `i5-e-phase3-boundary-wall`

Own only:

- `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs`
- `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/benchmark.rs`
- `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs`
- Lane C-owned closure fixtures under `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/rust_v1_closure/lane_c/**`
- Lane C section only inside `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`

Required work:

- Inventory the exact already-detectable supported-boundary rejections that belong to the shipped supported core.
- Freeze one machine-visible regression assertion per in-scope rejection boundary.
- Prefer existing unsupported and near-miss fixture families where they already match the boundary being frozen.
- Add new Lane C-owned dedicated closure fixtures only when an existing fixture family cannot truthfully encode the frozen boundary.
- Change product code only if the current surface fails to emit a stable, truthful early rejection for a boundary already in scope.
- Keep repo-root inventory semantics unchanged.

Forbidden work:

- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/labels.json`
- no edits under `/home/azureuser/__Active_Code/atomize-hq/spec/examples/ecommerce/units/**`
- no edits under `/home/azureuser/__Active_Code/atomize-hq/spec/examples/crosslib-app/units/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/reviews/BENCH-ECOM.readability.review.json`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/*.json`
- no edits to shared benchmark fixture files outside the Lane C closure subtree
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/TODOS.md`
- no edits to the parent-owned common prelude/helpers in `spec-cli/tests/rust_v1_closure.rs`
- no edits to Lane A or Lane B sections in `spec-cli/tests/rust_v1_closure.rs`

Worker verification:

```bash
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status . --format json
```

Acceptance:

- The supported-boundary wall is frozen behind one deliberate suite instead of scattered folklore.
- Every in-scope rejection in the lane packet has one stable observable failure contract.
- Repo-root inventory-only status semantics remain unchanged.
- Any product-code diff is minimal and directly tied to making an already-in-scope rejection stable and truthful.
- The lane diff stays inside the Lane C write set.

Bounce-back rule:

- If Lane C cannot freeze a boundary without inventing a new taxonomy, widening support, redesigning benchmark/read-side semantics, or changing shared suite helpers, it must stop and return that boundary or helper request to the parent as out of lane scope.

### WS-INT-ABC (`codex/i5-int`) — parent-only integration of Phases 1-3

Task id: `i5-f-integrate-abc`

Merge order is fixed:

1. merge `codex/i5-lane-c-boundary`
2. merge `codex/i5-lane-a-ecom`
3. merge `codex/i5-lane-b-crosslib`

Why this order is fixed:

- Lane C owns the boundary implementation and its section of the shared closure suite.
- Lane A owns the only allowed parallel edit to `benchmarks/labels.json`.
- Lane B owns no label edits and must land after the positive benchmark contract is frozen.

Integration rules:

- If a lane edited outside its owned file list, reject the lane and bounce it back.
- If a lane edited outside its named section in `spec-cli/tests/rust_v1_closure.rs`, reject the lane and bounce it back.
- If a lane edited shared full-output benchmark fixture files reserved to Lane D, reject the lane and bounce it back.
- If B returns a legitimate label change request, the parent may apply the minimal post-merge `benchmarks/labels.json` adjustment on `codex/i5-int` only after recording the rationale in `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i5/task/i5-f-integrate-abc/decisions.md`.
- If C’s boundary work would force new benchmark schema, new support taxonomy, or changed repo-root semantics, reject the lane as out of scope.
- If A, B, or C still causes benchmark outputs to drift between repeated runs, stop. Lane D is not allowed to chase unstable outputs.

Gate 1 is the **post-Phase-3 launch gate** for Lane D. Run these commands on `codex/i5-int` after A, B, and C are merged:

```bash
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- export examples/crosslib-app/units
cargo run -p spec-cli -- status . --format json
```

Gate 1 passes only when all of the following are true:

- `BENCH-ECOM` is passing with the required seam molecule deliberately part of the gate.
- `BENCH-CROSSLIB` is complete and still zero-credit.
- the supported-boundary suite is green.
- repo-root status remains `inventory_only`.
- the only remaining intentional drift is readability currentness plus final committed snapshot and shared fixture refresh.

Only after Gate 1 passes may the parent create and launch Lane D.

### WS-D (`codex/i5-lane-d-closeout`) — worker lane D, strictly sequential

Task id: `i5-g-phase4-readability-closeout`

Own only:

- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- final shared benchmark fixtures under `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/fixtures/benchmarks/**`
- `/home/azureuser/__Active_Code/atomize-hq/spec/TODOS.md`

Required work:

- Refresh `BENCH-ECOM.readability.review.json` against the final live projection digest and final `readability_generated_files`.
- Refresh only the benchmark snapshots impacted by the now-frozen closure wall.
- Refresh the final shared committed benchmark fixtures after A, B, and C are already integrated.
- Update the `M69` entry in `TODOS.md` so it names supported-core closure rather than stale benchmark-expansion wording.
- Keep all code and proof surfaces frozen. Lane D is an artifact and closeout lane, not a reopen-the-milestone lane.

Forbidden work:

- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/benchmarks/labels.json`
- no edits under `/home/azureuser/__Active_Code/atomize-hq/spec/examples/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-core/src/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/src/**`
- no edits to `/home/azureuser/__Active_Code/atomize-hq/spec/spec-cli/tests/rust_v1_closure.rs`
- no edits to unrelated fixture files

Worker verification:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- export examples/crosslib-app/units
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- status . --format json
cargo test -p spec-cli rust_v1_closure
```

Acceptance:

- `BENCH-ECOM` readability review is current again on live read-side output.
- impacted snapshots match the frozen final benchmark state.
- impacted shared benchmark fixtures match the frozen final benchmark state.
- `TODOS.md` truthfully describes the closure milestone.
- The lane diff stays inside the Lane D write set.

Bounce-back rule:

- If Lane D sees the projection digest or benchmark outputs still moving after Gate 1, it must stop immediately and return the run to `i5-f-integrate-abc`. Lane D must not hand-edit readability, snapshots, or fixtures to fit unstable upstream outputs.

### WS-FINAL (`codex/i5-int` then `codex/i5-prep`) — parent-only closeout

Task id: `i5-h-final-acceptance`

- Merge `codex/i5-lane-d-closeout` into `codex/i5-int`.
- Verify the merged diff still stays inside the allowed I5 write set.
- Run the full acceptance wall.
- If green, fast-forward or merge `codex/i5-int` back into `codex/i5-prep`.
- Record the final outcome in `.runs/i5/session-log.md`.
- Mark all completed task sentinels `MERGED`.

Final closeout command wall:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- export examples/crosslib-app/units
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status . --format json
```

Final acceptance requires all of the following:

- `BENCH-ECOM` still passes.
- `BENCH-ECOM.required_molecule_ids` explicitly includes `pricing/discount_strategy_checkout_flow`.
- `BENCH-ECOM` reports a current readability review.
- `BENCH-CROSSLIB` is complete, not incomplete.
- `BENCH-CROSSLIB` still contributes zero positive credit.
- the supported-boundary rejection wall is green and machine-visible.
- repo-root `status . --format json` still reports `scope_authority: "inventory_only"`.
- `BENCH-SERVICE` remains reserved-only context.
- the final snapshot commands produce no new diff after Lane D.
- the final diff introduced no new scope outside I5.

## Gate Model

Gate 0: **basis and lane freeze**

- Owned by the parent.
- Must pass before any worker launch.
- Requires the frozen basis SHA, `.runs/i5/` state, the parent scaffold commit on `codex/i5-prep`, lane packets, the seeded `spec-cli/tests/rust_v1_closure.rs`, and the frozen fixture partition policy.

Gate 1: **post-Phase-3 integration gate**

- Owned by the parent.
- Must pass before Lane D exists.
- Requires A, B, and C merged into `codex/i5-int`, the live closure wall stable, and readability/snapshot/shared-fixture/doc refresh to be the only remaining work.

Gate 2: **final closeout gate**

- Owned by the parent.
- Must pass before I5 is declared done.
- Requires the full acceptance wall above plus session-log closeout.

These gates are practical, not ceremonial. They exist to stop two failure patterns that would otherwise waste time:

- launching Lane D against moving benchmark truth
- masking Phase 1-3 scope drift with late artifact churn

## Context-Control Rules

- Every worker lane runs on GPT-5.4 with `reasoning_effort=high`.
- Each worker receives only:
  - its branch name
  - its worktree path
  - its owned files
  - its forbidden files
  - the exact `PLAN.md` excerpt relevant to its phase
  - the exact commands it must run
  - its lane-specific acceptance rules
- No worker receives the entire repo planning context unless the parent determines the lane is blocked without it.
- No worker may expand its owned file set on its own authority.
- No worker may edit `PLAN.md` or `ORCH_PLAN.md`.
- No worker may create a new side lane.
- No worker may pass raw transcripts to another worker.
- Workers return only:
  - changed files
  - commands run
  - exit codes
  - blockers
  - any explicit decision request for the parent
- The parent integrates from narrow summaries plus diffs, not from whole worker transcripts.
- Keep concurrency capped at 3. If the parent cannot keep the shared-suite partition clean, reduce to 2 or pause a lane. Do not add more lanes.

## Conflict And Bounce-Back Rules

- `spec-cli/tests/rust_v1_closure.rs` is the highest-risk shared file. Parent-owned prelude/helpers and lane section markers are mandatory.
- Lane A, Lane B, and Lane C may not rewrite imports, helper signatures, or shared harness shape in `spec-cli/tests/rust_v1_closure.rs`.
- If a lane needs shared helper changes, it must bounce that request back to the parent.
- Shared full-output benchmark fixtures are reserved to Lane D. Any earlier lane edit to those files is an automatic bounce-back.
- If benchmark fixture ownership becomes ambiguous, stop the merge, update `tasks.json`, and bounce the conflicting lane back. Do not hand-merge a hybrid fixture contract.
- If a worker requests BENCH-SERVICE work, support-row widening, new semantic-family promotions, repo-root export support, or schema redesign, reject the request as out of I5 scope.
- If a worker finds scope drift that could still be solved by a minimal parent integration edit, record that request in `decisions.md` and keep the lane otherwise frozen. Do not let the worker self-expand.

## Assumptions

- `PLAN.md` at `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` remains the sole authority for I5 scope and acceptance.
- The current frozen basis is still correctly represented by `main@1dbff70`.
- `spec-cli/tests/rust_v1_closure.rs` is the preferred dedicated suite path and may be created during I5 because it is absent today.
- Running `cargo run -p spec-cli -- test <spec>` continues to be the correct way to refresh unit passports and molecule evidence in place.
- Benchmark snapshot artifacts remain explicit committed files, not side effects of read-only status or export commands.
- The historical `ORCH_PLAN.md` is useful only for execution shape, not for scope or decisions.

## Success Condition

I5 is complete only when one parent agent can execute the final command wall on the integrated branch and see, without interpretation drift:

- `BENCH-ECOM` passing
- `pricing/discount_strategy_checkout_flow` deliberately required by the benchmark gate
- `BENCH-ECOM` readability current again
- `BENCH-CROSSLIB` complete and zero-credit
- the supported-boundary rejection wall frozen behind one deliberate suite
- repo-root inventory semantics unchanged
- `BENCH-SERVICE` still reserved and still untouched
