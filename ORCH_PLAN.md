# I6 Orchestration Plan

Status: **authoritative execution runbook**  
Milestone: **I6 Rust V1 service benchmark activation**  
Plan authority: **`/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md`**  
Frozen basis: **current `HEAD` of `codex/i6-service-benchmark-activation` when the run begins**  
Primary workspace: **`/home/azureuser/__Active_Code/atomize-hq/spec`**  
Last rewritten: **2026-05-21**

## Summary

- Execute from `/home/azureuser/__Active_Code/atomize-hq/spec`.
- Treat `PLAN.md` as the only milestone authority.
- Treat the existing `ORCH_PLAN.md` as stale I5 context only.
- Keep the live primary checkout on `codex/i6-service-benchmark-activation` as the parent lane and canonical run-state root.
- Use dedicated worker worktrees under `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/`.
- All worker lanes use **GPT-5.4** with **`reasoning_effort=high`**.
- Cap concurrency at **2 active worker lanes**:
  - Lane A runs first alone
  - Lane B and conditional Lane C may overlap only after the Lane A freeze commit exists
  - Lane D starts only after the post-B/C truth gate passes
- The parent agent remains the **only integrator**, **only merge authority**, and **only final acceptance authority**.
- Keep the critical path local to the parent for:
  - basis freeze
  - queue freeze
  - service-freeze record
  - worker launch
  - gate decisions
  - merge order
  - final proof wall
  - final fast-forward of the parent branch

## Starting Truth

Observed on `codex/i6-service-benchmark-activation` at the then-current `HEAD`:

- `cargo run -p spec-cli -- status . --format json` reports:
  - `schema_version: 4`
  - `scope_authority: "inventory_only"`
  - `BENCH-CROSSLIB` is `active`, `passing`, and `positive_credit_cases: 0`
  - `BENCH-ECOM` is `active`, `passing`, and `readability_review_status: "current"`
  - `BENCH-SERVICE` is `reserved`, `accounting_status: "reserved_missing_cases"`, and `summary.total_cases: 0`
- `benchmarks/labels.json` already declares:
  - `id: "BENCH-SERVICE"`
  - `root: "examples/service/units"`
  - `generated_root: "examples/service/src/generated"`
  - `readability_scope: "supported_closure"`
  - `cases: []`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json` exists and is still reserved-form.
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json` does not exist.
- `examples/service/` does not exist.
- `spec-cli/tests/rust_v1_service.rs` does not exist.
- `.runs/` already exists and can host `.runs/i6/`.
- Existing reusable patterns live in:
  - `examples/ecommerce/units/pricing/discount_strategy.unit.spec`
  - `examples/ecommerce/units/pricing/pricing_quote.unit.spec`
  - `examples/ecommerce/units/pricing/*.test.spec`
  - `spec-cli/tests/rust_v1_closure.rs`
  - `spec-cli/tests/fixtures/benchmarks/**`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/**`

## Hard Guards

- Do not widen M66 support.
- Do not reopen M68 benchmark design or command-scope mechanics.
- Do not introduce async, IO, traits, generics, lifetimes, framework-heavy authored surfaces, or cross-library service proof.
- Keep `BENCH-SERVICE` single-library under `examples/service/**`.
- Keep the positive service roster exactly:
  - `billing/apply_membership_discount`
  - `billing/apply_regional_fee`
  - `billing/checkout_net_total`
  - `billing/checkout_net_total_guarded_fee`
  - `billing/discount_strategy`
  - `billing/pricing_quote`
- Keep the required molecule roster exactly:
  - `billing/checkout_success_flow`
  - `billing/checkout_declined_discount_flow`
  - `billing/discount_strategy_quote_flow`
- Treat `.unit.spec` and `.test.spec` files as authored truth.
- Treat generated Rust, passports, molecule evidence, snapshots, and readability reviews as derived or observation surfaces.
- Allow projection/core edits only if the active service benchmark exposes a real read-side truth bug.
- Keep `BENCH-ECOM` and `BENCH-CROSSLIB` green and truthful throughout I6.
- Keep repo-root `status . --format json` diagnostic-only. It must remain `scope_authority: "inventory_only"` and is not a zero-exit acceptance gate.

## Worktree And Branch Plan

Create the I6 worktree root once:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6
```

Freeze the live basis before any worker branch is created:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse --abbrev-ref HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse --short HEAD
```

Expected basis:

- branch: `codex/i6-service-benchmark-activation`
- commit: record the current `HEAD` in `.runs/i6/basis.json` at queue freeze and use that as the run-local basis for all later drift checks

Use the live primary checkout as the parent lane:

- Parent branch: `codex/i6-service-benchmark-activation`
- Parent workspace: `/home/azureuser/__Active_Code/atomize-hq/spec`

Create Lane A first and only:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-a -b codex/i6-lane-a-service-root codex/i6-service-benchmark-activation
```

After Lane A merges and the parent records `service_freeze_commit`, create integration and Lane B from that exact commit:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/int -b codex/i6-int <service_freeze_commit>
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-b -b codex/i6-lane-b-service-regressions <service_freeze_commit>
```

Create Lane C only if the parent confirms a real read-side truth bug:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-c -b codex/i6-lane-c-projection-fix <service_freeze_commit>
```

Create Lane D only after Gate 2 passes:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-d -b codex/i6-lane-d-closeout codex/i6-int
```

Final landing is parent-owned only:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec merge --ff-only codex/i6-int
```

## Orchestration State

Canonical run state lives under:

- `I6_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6`
- queue: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/tasks.json`
- session log: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/session-log.md`
- basis record: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/basis.json`
- Gate 1 record: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/service-freeze.json`
- Gate 2 record: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/service-truth-gate.json`
- Gate 3 record: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/final-acceptance.json`
- per-task state: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i6/task/<task-id>/`

The parent owns all writes under `.runs/i6/`. Workers do not update orchestration state directly.

Per-task sentinels inside `.runs/i6/task/<task-id>/`:

| Sentinel | Meaning |
| --- | --- |
| `QUEUED` | task frozen, not started |
| `RUNNING` | lane or parent actively executing |
| `BLOCKED` | waiting on parent decision or dependency |
| `READY` | lane believes acceptance is satisfied |
| `MERGED` | parent merged result |
| `REJECTED` | parent bounced lane for scope or quality reasons |

Each task directory carries:

- `scope.md`
- `acceptance.md`
- `handoff.md`
- `decisions.md`

## Lane Map

| Lane | Branch | Worktree | Owned write set | Goal |
| --- | --- | --- | --- | --- |
| Parent | `codex/i6-service-benchmark-activation` | `/home/azureuser/__Active_Code/atomize-hq/spec` | `.runs/i6/**`, gate records, merge decisions, final fast-forward | orchestration and final authority |
| Lane A | `codex/i6-lane-a-service-root` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-a` | `benchmarks/labels.json`, `examples/service/**`, generated Rust under `examples/service/src/generated/**`, service passports, service molecule evidence | service scaffold, active labels, authored proof wall |
| Lane B | `codex/i6-lane-b-service-regressions` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-b` | `spec-cli/tests/rust_v1_service.rs`, `spec-cli/tests/fixtures/benchmarks/**` | service regression truth and benchmark fixtures |
| Lane C | `codex/i6-lane-c-projection-fix` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-c` | `spec-core/src/benchmark.rs`, `spec-cli/src/commands.rs` | minimal read-side truth fix only if needed |
| Lane D | `codex/i6-lane-d-closeout` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/lane-d` | `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`, `benchmarks/reviews/BENCH-SERVICE.readability.review.json`, `README.md`, `docs/rust_v1_contract_stack.md`, `TODOS.md`, `CHANGELOG.md`, and only-if-truthfully-required closeout refreshes for existing ECOM/CROSSLIB snapshot or readability or doc surfaces | artifact/docs closeout only |
| Integration | `codex/i6-int` | `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/int` | merge-only plus minimal parent reconciliation | integration and gate execution |

## Gate Model

### Gate 0: Basis Freeze / Queue Freeze

Owner: parent  
Task id: `i6-a0-freeze-basis`

Advance only when:

- live branch is `codex/i6-service-benchmark-activation`
- live commit is the branch `HEAD` recorded in `basis.json` for this run
- `.runs/i6/` exists
- `basis.json`, `tasks.json`, and task packets exist
- frozen positive service roster and required molecules are recorded
- Lane A is the only worker branch created

Reopens if:

- the live parent basis drifts before worker launch
- queue ownership or frozen rosters are unclear
- any worker needs to guess write-set or gate behavior

### Gate 1: Post-Lane-A Service-Freeze Gate

Owner: parent  
Task id: `i6-a1-service-freeze`

Advance only when:

- `examples/service/**` exists as a real single-library benchmark root
- `benchmarks/labels.json` marks `BENCH-SERVICE` active
- labels list exactly the frozen six service units and three required molecules
- all six service units have fresh passports
- all three service molecules have fresh evidence
- the parent writes `service-freeze.json` with:
  - `service_freeze_commit`
  - frozen rosters
  - current `label_digest`
  - whether a projection bug is open

Reopens Lane A if:

- authored service truth is incomplete, stale, invalid, or out of roster
- service root or labels drift from the frozen roster
- any claimed proof depends on widened support

### Gate 2: Post-B/C Service-Truth Gate

Owner: parent  
Task id: `i6-d0-service-truth-gate`

Advance only when:

- Lane B is merged into `codex/i6-int`
- Lane C is either merged or explicitly skipped
- `cargo run -p spec-cli -- status examples/service/units --format json` and `cargo run -p spec-cli -- export examples/service/units` project truthful service-root benchmark state
- service-root benchmark truth is passing with full proof
- service-root fixtures and assertions in Lane B are derived from direct service-root `status` and `export` truth, not only indirect unit-test expectations
- partial service scopes emit zero positive credit
- `BENCH-ECOM` remains green/current
- `BENCH-CROSSLIB` remains green with zero positive credit

Reopens prior lanes if:

- service-root truth is wrong because authored proof or labels are wrong: reopen Lane A
- service-root truth is wrong because regression tests or fixtures are missing or dishonest: reopen Lane B
- service-root truth is wrong because projection/core behavior is wrong: reopen Lane C
- any cross-lane disagreement conflicts with `PLAN.md`: bounce to the owning lane, do not resolve creatively in integration

### Gate 3: Final Acceptance Gate

Owner: parent  
Task id: `i6-e-final-acceptance`

Advance only when:

- `BENCH-SERVICE` is active, passing, and gate-satisfied at service-root scope
- `BENCH-SERVICE` readability review is `current`
- committed `BENCH-SERVICE.snapshot.json` is stable on rerun
- service-root `status` and `export` remain the proof wall
- `BENCH-ECOM` and `BENCH-CROSSLIB` stay truthful
- repo-root `status . --format json` remains diagnostic inventory with `scope_authority: "inventory_only"`

Reopens prior lanes if:

- snapshot or readability truth drifts: reopen Lane D
- service-root proof wall drifts: reopen Lane A, B, or C based on cause
- docs overstate shipped truth: reopen Lane D

## Conflict And Bounce-Back Rules

- If a lane edits outside its write set, the parent rejects it.
- If a lane discovers required scope drift to the frozen positive roster or required molecule roster, it must stop, mark `BLOCKED`, and escalate to the parent.
- Lane A may not change regression suites, fixtures, snapshots, reviews, or docs.
- Lane B may not patch core behavior. If it requires `spec-core/src/benchmark.rs` or `spec-cli/src/commands.rs` changes, it must bounce to the parent and conditional Lane C.
- Lane C may not patch labels, specs, proof artifacts, tests, fixtures, snapshots, reviews, or docs.
- Lane D is closeout-only. It may not reopen service semantics, labels, tests, fixtures, or core behavior.
- Integration does not resolve creative cross-lane disagreements. It either:
  - applies `PLAN.md` literally, or
  - bounces the issue back to the owning lane
- If a lane believes another lane’s owned surface must change, it does not edit that surface directly. It returns a narrow parent escalation.

## Shared Ownership And Artifact Policy

- `PLAN.md` is the only milestone authority.
- Source service specs are authored truth:
  - `examples/service/units/**/*.unit.spec`
  - `examples/service/units/**/*.test.spec`
- Lane A may refresh the derived service proof surfaces required by those specs:
  - `examples/service/src/generated/**`
  - `examples/service/units/**/*.spec.passport.json`
  - `examples/service/units/**/*.test.evidence.json`
- No lane hand-edits generated Rust, passports, molecule evidence, snapshot JSON, or readability review JSON.
- Lane D alone owns committed snapshot and readability refresh for `BENCH-SERVICE`.
- If merged I6 truth makes `BENCH-ECOM` or `BENCH-CROSSLIB` snapshot or readability or closeout docs require refresh, that refresh belongs to Lane D and nowhere else.
- Lane B owns benchmark test fixtures, including any repo-root benchmark fixture JSON that changes because `BENCH-SERVICE` becomes active.
- Repo-root `status . --format json` is always interpreted as structured inventory, never as a zero-exit proof command.

## Workstream Plan

### WS-PARENT-0 — basis freeze and queue freeze

Task id: `i6-a0-freeze-basis`

Parent actions:

- confirm live branch and commit
- create `.runs/i6/`
- write `basis.json`
- write `tasks.json`, `session-log.md`, and task packets
- create Lane A only
- freeze the exact positive roster and required molecule roster in run state

Acceptance:

- Gate 0 is green
- no worker needs to guess branch names, worktree paths, write-sets, or gate rules

### WS-A — service scaffold, labels, and proof wall

Task id: `i6-a1-service-freeze`

Lane A owns:

- `benchmarks/labels.json`
- `examples/service/**`
- generated Rust under `examples/service/src/generated/**`
- service passports and service molecule evidence

Required outcomes:

- scaffold the service example
- author the six frozen service units
- author the three frozen molecule tests
- activate `BENCH-SERVICE`
- build generated Rust
- refresh all six unit passports
- refresh all three molecule evidence files

Required commands:

```bash
cargo run -p spec-cli -- build examples/service/units --output examples/service/src/generated
cargo run -p spec-cli -- test examples/service/units/billing/apply_membership_discount.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/apply_regional_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total_guarded_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_success_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_declined_discount_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy_quote_flow.test.spec
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
```

Hard rules:

- no snapshot refresh
- no readability review authoring
- no test fixture edits
- no docs edits
- no core projection edits

Acceptance:

- Gate 1 is green
- if read-side truth is wrong, Lane A returns a narrow reproduced bug statement rather than patching core behavior itself

### WS-PARENT-1 — post-freeze fan-out

Task id: `i6-a2-launch-int-b-c`

Parent actions:

- create `codex/i6-int` and `codex/i6-lane-b-service-regressions` from `service_freeze_commit`
- create `codex/i6-lane-c-projection-fix` only if needed
- mark Lane B running
- mark Lane C running only if spawned

Acceptance:

- all post-freeze branches fork from the exact same `service_freeze_commit`
- active worker count never exceeds 2

### WS-B — service regression truth and benchmark fixtures

Task id: `i6-b1-service-regressions`

Lane B owns:

- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`

Required outcomes:

- add a dedicated `rust_v1_service` suite
- add or refresh benchmark fixtures for:
  - full service-root `status`
  - full service-root `export`
  - partial service-scope zero-credit behavior
  - active repo-root benchmark inventory with `BENCH-SERVICE`
- prove regressions for:
  - missing required molecule proof
  - stale required molecule proof
  - failing required molecule proof
  - readability drift becoming non-current
  - partial service scopes emitting zero positive credit
  - preserved `BENCH-ECOM`
  - preserved `BENCH-CROSSLIB`
  - preserved repo-root inventory semantics

Required commands:

```bash
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- status . --format json
```

Hard rules:

- no edits to `benchmarks/labels.json`
- no edits under `examples/service/**`
- no snapshot or readability-review edits
- no docs edits
- no core behavior edits

Acceptance:

- lane diff stays entirely within Lane B ownership
- service-root fixtures are derived from direct `status examples/service/units --format json` and `export examples/service/units` truth
- service-root fixtures are not justified only by indirect unit-test expectations
- repo-root inventory fixtures stay truthful to active `BENCH-SERVICE`

### WS-C — minimal projection/core fix only if needed

Task id: `i6-c1-projection-fix`

Lane C owns:

- `spec-core/src/benchmark.rs`
- `spec-cli/src/commands.rs`

Required outcomes:

- reproduce the exact read-side truth bug
- fix only the minimum code needed for truthful service-root projection
- preserve M68 mechanics and current command-scope behavior

Required commands:

```bash
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
```

Hard rules:

- no edits to specs, proof artifacts, labels, tests, fixtures, snapshots, reviews, or docs
- no benchmark-schema redesign
- no support expansion

Acceptance:

- diff stays entirely inside Lane C ownership
- bug is explained as read-side truth repair, not feature work

### WS-INT-2 — post-B/C service-truth gate

Task id: `i6-d0-service-truth-gate`

Parent actions in `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i6/int`:

- merge Lane C first if present
- merge Lane B second
- run Gate 2 before Lane D exists

Required commands:

```bash
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
```

Acceptance:

- Gate 2 is green
- service-root `status` and `export` are truthful proof walls
- `readability_review_status` may still be non-current here because Lane D has not run yet

### WS-D — artifact/docs closeout only

Task id: `i6-d1-closeout`

Lane D owns:

- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
- `README.md`
- `docs/rust_v1_contract_stack.md`
- `TODOS.md`
- `CHANGELOG.md`
- only-if-truthfully-required closeout refreshes for ECOM/CROSSLIB snapshot, readability, or docs surfaces

Required outcomes:

- refresh `BENCH-SERVICE.snapshot.json` from reserved-form to active-form
- author the first `BENCH-SERVICE.readability.review.json`
- align repo-facing docs to shipped truth only

Required commands:

```bash
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
```

Hard rules:

- no service semantic edits
- no label edits
- no spec edits
- no test edits
- no fixture edits
- no core behavior edits
- Lane D is artifact/docs closeout only

Acceptance:

- `BENCH-SERVICE` snapshot is active-form
- `BENCH-SERVICE` readability review is current against final projection digest and generated file set
- docs teach the same story the CLI now projects

### WS-INT-3 — final acceptance and landing

Task id: `i6-e-final-acceptance`

Parent actions:

- merge Lane D into `codex/i6-int`
- run Gate 3
- if green, fast-forward the parent branch
- if red, bounce only the owning lane

Required commands:

```bash
cargo run -p spec-cli -- build examples/service/units --output examples/service/src/generated
cargo run -p spec-cli -- test examples/service/units/billing/apply_membership_discount.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/apply_regional_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total_guarded_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_success_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_declined_discount_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy_quote_flow.test.spec
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- status . --format json
```

Acceptance:

- Gate 3 is green
- `BENCH-SERVICE` is active, passing, gate-satisfied, and `readability_review_status: "current"`
- rerunning `cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE` leaves the committed snapshot stable
- service-root `status` and `export` are the proof wall
- repo-root `status . --format json` remains diagnostic inventory with `scope_authority: "inventory_only"`

## Merge Order

1. Parent freezes basis and queue.
2. Lane A lands on the parent branch.
3. Parent records `service_freeze_commit`.
4. Parent creates `codex/i6-int` and Lane B from `service_freeze_commit`.
5. Parent creates Lane C only if a real projection bug exists.
6. Lane C, if present, merges into `codex/i6-int` first.
7. Lane B rebases if Lane C changed truth-surface behavior, then merges into `codex/i6-int`.
8. Parent runs Gate 2.
9. Lane D branches from `codex/i6-int` only after Gate 2 passes.
10. Lane D merges into `codex/i6-int`.
11. Parent runs Gate 3.
12. Parent fast-forwards `codex/i6-service-benchmark-activation` to `codex/i6-int`.

## Context-Control Rules

- Parent keeps only:
  - `PLAN.md`
  - `.runs/i6/tasks.json`
  - gate records
  - the latest narrow diff summary per lane
- Each worker prompt contains only:
  - owned files
  - forbidden files
  - relevant `PLAN.md` excerpt
  - required commands
  - the recorded `service_freeze_commit` when applicable
- Workers return only:
  - changed files
  - commands run and exit codes
  - blockers
  - unresolved assumptions
- Workers do not write `.runs/i6/**`.
- Close each worker immediately after merge or rejection.

## Tests And Acceptance

- Lane A acceptance is authored service truth plus refreshed proof.
- Lane B acceptance is direct service-root regression truth plus truthful benchmark fixtures.
- Lane C acceptance is a minimal reproduced-and-fixed read-side bug.
- Lane D acceptance is snapshot, readability, and docs closeout only.
- Final acceptance is the full service benchmark wall plus non-regression across `BENCH-ECOM`, `BENCH-CROSSLIB`, and repo-root inventory semantics.

The service benchmark wall is closed only when all are true at once:

- `BENCH-SERVICE` is active rather than reserved.
- the roster is exactly six positive units and three required molecules.
- all six positive units are valid with fresh proof.
- all three required molecules are valid with fresh evidence.
- service-root `status` and `export` both project passing active truth.
- service partial scopes do not launder positive credit.
- `BENCH-SERVICE` readability review is current.
- committed `BENCH-SERVICE.snapshot.json` is stable on rerun.
- `BENCH-ECOM` remains passing/current.
- `BENCH-CROSSLIB` remains passing with zero positive credit.
- repo-root `status . --format json` remains diagnostic inventory and not a zero-exit acceptance gate.

## Assumptions

- The primary checkout on `codex/i6-service-benchmark-activation` remains the parent workspace for the full run.
- `spec-cli/tests/fixtures/benchmarks/**` is broad enough that Lane B may own all service-root and repo-root benchmark fixture refresh required by active `BENCH-SERVICE`.
- `.runs/i6/**` is parent-owned run state and is not assumed to be a checked-in deliverable.
