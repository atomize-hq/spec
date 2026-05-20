# I4 Orchestration Plan

Status: **authoritative execution runbook**
Milestone: **I4 Rust V1 command-wall fixture and contract-test hardening**
Working branch: **`codex/i4-prep`**
Last rewritten: **2026-05-20**

## Summary

- Execute from `/home/azureuser/__Active_Code/atomize-hq/spec` on the current
  parent branch `codex/i4-prep`.
- Treat `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` as the only
  milestone authority. Treat the prior `ORCH_PLAN.md` only as historical
  context.
- Keep the parent agent on the critical path for:
  - freezing the six-command roster and normalization policy
  - launching and bounding worker lanes
  - deciding whether the optional stabilization lane is justified
  - integrating worker output in order
  - running the final proof wall
- Use workers only for the three bounded implementation lanes:
  - fixture promotion
  - CLI contract-test hardening
  - optional runtime stabilization if truthful nondeterminism blocks the wall
- Use dedicated worktrees under
  `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/`:
  - fixtures: `codex/i4-fixtures`
  - tests: `codex/i4-tests`
  - stabilize: `codex/i4-stabilize`
  - integration: `codex/i4-int`
- Cap concurrency at 2 active workers. The stabilization lane is conditional
  and never runs in parallel with ongoing fixture-name or normalization-policy
  edits.
- The parent agent remains the only integrator and the only authority for
  merge order, acceptance, and closeout.

## Orchestration State

- Canonical parent-owned run state lives under:
  - `I4_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i4`
  - queue: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i4/tasks.json`
  - session log: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i4/session-log.md`
  - per-task sentinels:
    `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i4/task/<task-id>/`
- Treat `.runs/i4/*` as run artifacts only.
- `.runs/i4/*` is not authored source, not milestone authority, and not a
  worker-owned output surface.
- Workers return code changes and narrow summaries only. The parent records any
  task state or orchestration notes back under `.runs/i4/`.

## Hard Guards

- Do not widen Rust V1 support.
- Do not implement `BENCH-SERVICE`.
- Do not redesign benchmark mechanics, benchmark roles, benchmark projection,
  or benchmark artifact semantics.
- Do not add repo-root aggregate export support.
- Do not change the frozen interpretation of repo-root `status`; it remains
  supported only as `inventory_only`.
- Do not change the frozen interpretation of repo-root `export`; it remains an
  unsupported scope with `SPEC_UNSUPPORTED_SCOPE`.
- Do not bump schema versions.
- Do not create a new CLI fixture subsystem, snapshot framework, crate, or test
  harness abstraction.
- Do not widen the write set beyond:
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/benchmarks/*.json`
  - `spec-cli/src/commands.rs` only if the parent agent declares a real
    nondeterminism blocker
  - `spec-core/src/export.rs` only if the same blocker cannot be contained in
    CLI glue
- Do not rewrite broad docs. I4 is not a docs milestone.
- Do not let workers invent new fixture names, new command surfaces, or new
  normalization fields beyond what `PLAN.md` already freezes.

## Workstream Plan

### WS-PARENT (`codex/i4-prep`) — parent agent only, sequential critical path

Worktree:
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/parent`

1. `task/i4-a-freeze-contract`
- Freeze the authoritative command roster exactly as:
  - benchmark-root `status`
  - benchmark-root `export`
  - namespace `status`
  - single-file `status`
  - repo-root `status`
  - repo-root `export`
- Freeze the maintained fixture roster exactly as:
  - `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
  - `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
  - `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json`
  - `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json`
  - `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
  - `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`
- Freeze the only allowed normalization targets:
  - absolute filesystem paths
  - unit and molecule `evidence_at` timestamps
  - `exported_at`
  - `provenance.git_commit_sha`
  - `freshness.authored_truth_digest`
  - benchmark `label_digest`
  - benchmark `projection_digest`
- Freeze the read-only authority inputs for workers:
  - `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i3_5_authority_alignment/validation/final-main/*.stdout`
  - existing benchmark fixtures only for local naming and format reference

Acceptance for `task/i4-a-freeze-contract`:
- the six command surfaces are final and need no reinterpretation
- the six fixture paths are final
- the normalization policy is final before workers start writing files
- the parent agent has prepared lane-specific prompts with owned files and
  forbidden surfaces
- `tasks.json` records the exact task sequence and lane ownership
- the session log records the frozen fixture roster and normalization policy

2. Worktree and branch setup
- Create the dedicated worktrees:
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/parent`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/fixtures`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/tests`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/stabilize`
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/int`
- Create the workstream branches from the frozen parent basis:
  - `codex/i4-fixtures`
  - `codex/i4-tests`
  - `codex/i4-stabilize`
  - `codex/i4-int`

3. Launch order
- Launch `WS-FIXTURES` and `WS-TESTS` in parallel after the contract freeze.
- `WS-TESTS` may start helper scaffolding immediately, but it must not finalize
  fixture assertions until the parent confirms the frozen fixture names remain
  unchanged.
- Launch `WS-STABILIZE` only if the parent agent can point to one concrete,
  truthful nondeterminism blocker that cannot be solved in test normalization.

### WS-FIXTURES (`codex/i4-fixtures`) — worker 1

Task id:
- `task/i4-b-fixture-promotion`

Worktree:
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/fixtures`

Own only:
- `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`

Read-only inputs:
- `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i3_5_authority_alignment/validation/final-main/*.stdout`
- `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` excerpt for fixture
  promotion and normalization rules

Required work:
- Promote the frozen I3.5 proof outputs into the six maintained fixture files.
- Normalize only the frozen unstable fields.
- Keep fixture content as command truth, not hand-authored summaries.
- Preserve repo-root export as an unsupported-scope fixture, not a success
  bundle.

Forbidden work:
- no edits to `spec-cli/tests/cli.rs`
- no edits to `spec-cli/src/commands.rs`
- no edits to `spec-core/src/export.rs`
- no extra fixture files
- no fixture renames

Acceptance:
- all six required fixture files exist
- each fixture is derived from final-main proof output rather than memory
- only allowed unstable fields are normalized
- partial status fixtures remain partial and do not mint full-scope positive
  credit
- repo-root export fixture still encodes `SPEC_UNSUPPORTED_SCOPE`
- benchmark-root `status` and `export` fixtures remain full-scope surfaces
- no fixture smuggles in new schema or support semantics

Worker verification:
- `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
- `cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json`
- `cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json`
- `cargo run -p spec-cli -- status . --format json`
- `cargo run -p spec-cli -- export examples/ecommerce/units`
- `cargo run -p spec-cli -- export .`

### WS-TESTS (`codex/i4-tests`) — worker 2

Task id:
- `task/i4-c-cli-contract-tests`

Worktree:
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/tests`

Own only:
- `spec-cli/tests/cli.rs`

Read-only inputs:
- `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md` excerpts for:
  - frozen command roster
  - required fixture set
  - allowed normalization fields
  - required explicit invariants
- the six frozen fixture paths only

Required work:
- add the narrow helper layer inside `spec-cli/tests/cli.rs`
- keep separate normalization entrypoints for status and export JSON
- add one dedicated command-wall regression path per frozen surface
- compare normalized full JSON to the exact fixture
- retain explicit invariants where the fixture diff alone is too opaque:
  - repo-root `scope_authority == "inventory_only"`
  - repo-root export `errors[0].code == "SPEC_UNSUPPORTED_SCOPE"`
  - namespace status stays partial and non-crediting
  - single-file status omits full-scope-only projection surfaces
  - benchmark-root status and export remain full benchmark-root surfaces

Required test-local helper surface:
- `read_contract_fixture(...)`
- `normalize_status_contract_json(...)`
- `normalize_export_contract_json(...)`
- `assert_contract_matches_fixture(...)`

Forbidden work:
- no fixture edits
- no product-code edits
- no new test binary
- no shared test-support crate
- no generic snapshot framework

Two-step lane execution:
- B1 scaffold can begin immediately:
  - helper block
  - test naming
  - dedicated assertion flow
- B2 closes only after the parent confirms fixture names and the worker has
  rebased or merged the fixture lane basis needed to run the full suite

Acceptance:
- one dedicated regression test exists per frozen command surface
- no surface relies on fragment-only assertions anymore
- helper logic is test-local and narrow
- no helper strips real contract fields
- explicit invariants remain readable and not hidden entirely inside fixture
  comparisons
- the test file still stays centered on the six frozen surfaces rather than
  expanding into adjacent CLI behavior

Worker verification:
- `cargo test -p spec-cli`
- `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
- `cargo run -p spec-cli -- status . --format json`
- `cargo run -p spec-cli -- export .`

### WS-STABILIZE (`codex/i4-stabilize`) — conditional worker lane

Task id:
- `task/i4-d-runtime-stabilize`

Worktree:
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/stabilize`

Launch only if both of these are true:
- `WS-FIXTURES` and `WS-TESTS` have already proven the intended command wall
  and fixture corpus
- the parent agent has concrete evidence that a frozen contract cannot be
  asserted truthfully because of remaining nondeterministic runtime noise

Own only one of:
- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`

Required work:
- make the smallest truthful stabilization needed for the existing frozen wall
- preserve semantics exactly; fix transport noise only
- add no new behavior surface

Forbidden work:
- no redesign of repo-root or benchmark-root scope rules
- no benchmark classification changes
- no new schema fields
- no test-only convenience semantics leaking into product behavior
- no broad refactor

Acceptance:
- the blocker is resolved with the smallest possible diff
- the stabilized output still matches the frozen command wall
- the parent can explain the fix as preserving an existing contract, not adding
  one
- the owning source file is exactly one file unless the parent explicitly
  rejects the lane as out of scope
- if the blocker would require touching both allowed source files, the lane does
  not proceed as a normal stabilization edit

Worker verification:
- `cargo test -p spec-cli`
- the exact blocked command from the final proof wall

### WS-INT (`codex/i4-int`) — parent agent only

Task id:
- `task/i4-e-integrate`

Worktree:
- `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i4/int`

Merge order is fixed:

1. merge `WS-FIXTURES`
2. merge `WS-TESTS`
3. merge `WS-STABILIZE` only if activated

Integration rules:
- If fixture names, normalization policy, or explicit invariant expectations
  disagree, the parent does not improvise a hybrid contract.
  - apply the frozen `PLAN.md` contract literally, or
  - bounce the lane back for correction
- If `WS-FIXTURES` and `WS-TESTS` disagree on fixture names, fixture paths, or
  which fields may be normalized, the parent freezes the merge, records the
  mismatch under `.runs/i4/task/i4-e-integrate/` and in
  `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i4/session-log.md`,
  then sends one lane back to the frozen `task/i4-a-freeze-contract` packet.
  No partial hybrid lands.
- Do not merge the stabilization lane until fixture and test work are already
  frozen.
- If the stabilization lane would require touching both
  `spec-cli/src/commands.rs` and `spec-core/src/export.rs`, treat that as scope
  expansion, not a normal stabilization pass. Halt the lane, do not merge it,
  and leave I4 blocked pending a new plan.
- If any lane proves the frozen wall cannot be asserted truthfully without
  widening support scope, redesigning benchmark mechanics, changing schema, or
  adding repo-root export behavior, the parent stops the milestone as blocked.
  Do not resolve that by creative edits inside I4.
- If `WS-STABILIZE` is not needed, close the milestone without touching product
  code.
- After `codex/i4-int` is green, fast-forward or merge the integrated result
  back into `codex/i4-prep`.

Parent-only closeout sequence:

1. confirm the merged diff still stays inside the allowed I4 write set
2. run `cargo test -p spec-cli`
3. run `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
4. run `cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json`
5. run `cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json`
6. run `cargo run -p spec-cli -- status . --format json`
7. run `cargo run -p spec-cli -- export examples/ecommerce/units`
8. run `cargo run -p spec-cli -- export .`
9. confirm each fixture-backed surface still matches its frozen role
10. confirm repo-root `status` is still `inventory_only`
11. confirm repo-root `export` still fails with `SPEC_UNSUPPORTED_SCOPE`
12. record closeout outcome in `.runs/i4/session-log.md`

## Context-Control Rules

- The parent agent keeps only these live authorities in working context:
  - `/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md`
  - this runbook
  - the frozen six-command roster
  - the frozen normalization policy
  - the latest integration diff summary
  - the current task row from `.runs/i4/tasks.json`
- Worker prompt packet requirements:
  - owned files only
  - exact `PLAN.md` excerpt only
  - required commands only
  - forbidden surfaces only
  - acceptance checks only
- Each worker receives only:
  - its owned file list
  - the exact relevant `PLAN.md` excerpt
  - the exact required fixture names or command roster rows it needs
  - required commands
  - forbidden touch surfaces
  - branch name and worktree path
- `WS-FIXTURES` gets final-main stdout mapping and normalization policy, but no
  unrelated test or product-code context.
- `WS-TESTS` gets fixture names and invariant obligations, but not broad repo
  planning context.
- `WS-STABILIZE` gets only:
  - the exact failing command
  - the exact nondeterministic field
  - the one allowed owned source file
  - proof that test normalization cannot solve the blocker honestly
- No worker may expand its owned file set on its own authority.
- No worker may edit:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - docs
  - unrelated tests
  - new crates or workspace config
- Each worker returns only:
  - changed files
  - commands run
  - exit codes
  - blockers or unresolved assumptions
- Workers do not hand off raw transcripts to other workers.
- The parent agent integrates from summaries plus narrow diffs only, not full
  worker transcripts, then closes each worker once its lane is merged or
  rejected.

## Tests And Acceptance

Required final verification commands:

```bash
cargo test -p spec-cli
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export .
```

Final acceptance requires all of the following:

- benchmark-root `status` matches
  `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
- benchmark-root `export` matches
  `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
- namespace `status` matches
  `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json`
- single-file `status` matches
  `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json`
- repo-root `status` matches
  `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
- repo-root `export` matches
  `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`
- repo-root `status` still emits `scope_authority: "inventory_only"`
- repo-root `export` still fails with `SPEC_UNSUPPORTED_SCOPE`
- namespace and single-file `status` remain partial and non-crediting
- partial status surfaces still omit full-scope-only benchmark projection
  surfaces
- benchmark-root surfaces remain the only full positive proof wall
- the final diff touched no out-of-scope subsystem
- if no truthful nondeterminism blocker exists, `spec-cli/src/commands.rs` and
  `spec-core/src/export.rs` remain untouched

## Assumptions

- The frozen I3.5 proof wall under
  `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i3_5_authority_alignment/validation/final-main/`
  is still the correct seed truth for the six I4 fixtures.
- Existing CLI tests and benchmark fixture patterns are close enough that
  `spec-cli/tests/cli.rs` can absorb the I4 helper layer without creating a new
  subsystem.
- lane-level verification may use a narrowed subset of the frozen command wall,
  while the full required command roster remains the parent-only closeout gate.
- The historical `ORCH_PLAN.md` informed lane shape only; it is not authority
  for I4 scope, acceptance, or merge order.
