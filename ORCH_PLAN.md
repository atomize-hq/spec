# M46 Orchestration Plan

Status: **authoritative kickoff and execution contract for M46 helper-aware monotone-up TypeScript execution**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Execute from current branch: **`feat/m40-plus`**  
Scope diff anchor commit: **`ce0e16d`**  
Last rewritten: **`2026-05-10`**

## Summary

- Execute from the current repo root on branch `feat/m40-plus`.
- Use `ce0e16d` as the fixed M46 scope diff anchor because it is the M45 landing tip and the last known-green boundary before helper-aware widening begins.
- Keep the true critical path in the parent lane for:
  - baseline capture
  - authority freeze
  - validator contract freeze
  - backend intake
  - CLI ownership cleanup
  - integration intake
  - final proof wall
  - closeout
- Launch exactly two early worker lanes after the validator contract is frozen:
  - backend helper generation and harness
  - proof-source refresh and product coverage
- Launch one late worker lane only after the integrated code path is stable:
  - docs closeout
- Worker concurrency cap is **2** before integration intake and **1** after docs launch.
- Worker model assumption is fixed for all worker lanes:
  - `model = GPT-5.4`
  - `reasoning_effort = high`
- Use dedicated `spec-m46` worktrees and branches:
  - primary baseline: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `ws/spec-m46-contract-freeze` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/contract-freeze`
  - `ws/spec-m46-backend-helper` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/backend-helper`
  - `ws/spec-m46-proof-assets` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/proof-assets`
  - `ws/spec-m46-docs-closeout` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/docs-closeout`
  - `ws/spec-m46-integration` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`
- Keep orchestration state in one canonical parent-owned run root:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `M46_RUN_ROOT=$PRIMARY_ROOT/.runs/m46_helper_aware_monotone_up_typescript`
  - queue: `$M46_RUN_ROOT/queue.json`
  - task state: `$M46_RUN_ROOT/tasks.json`
  - session log: `$M46_RUN_ROOT/session-log.md`
  - baseline record: `$M46_RUN_ROOT/baseline.json`
  - authority freeze: `$M46_RUN_ROOT/authority-freeze.json`
  - contract freeze: `$M46_RUN_ROOT/contract-freeze.json`
  - acceptance ledger: `$M46_RUN_ROOT/acceptance.md`
  - merge ledger: `$M46_RUN_ROOT/merge-log.md`
  - closeout: `$M46_RUN_ROOT/closeout.md`
- Treat authored source, run-state artifacts, and derived proof artifacts as different classes:
  - authored source is the milestone deliverable
  - `.runs/**` is parent-owned orchestration state only
  - refreshed passports, evidence, generated TypeScript trees, and validation captures are derived outputs only
- Worker worktrees do not become independent sources of truth for orchestration, approvals, or acceptance. Workers return code changes plus narrow summaries only. The parent writes all run artifacts back to `PRIMARY_ROOT`.

## Hard Guards

- `PLAN.md` is the sole scope authority. `ORCH_PLAN.md` is the execution contract, not a second product spec.
- M46 scope is exactly:
  - `kind:function`
  - Bun only
  - atom tests only
  - `function.arithmetic_leaf.monotone_up.v1` only
  - `deps: []` or exactly one direct helper dep
  - that one direct dep must classify as `function.helper.identity_passthrough.v1`
  - the helper unit must exist in the same loaded unit set and generated output tree
- M46 does not widen into:
  - wrapper execution
  - molecule execution
  - multi-dep execution
  - seam kinds
  - cross-library TypeScript resolution
  - any function family beyond `function.arithmetic_leaf.monotone_up.v1`
  - `spec validate --target-language`
  - `spec export --target-language`
- Rust remains the default target everywhere.
- TypeScript proof remains additive only:
  - `target_proofs.rust`
  - `target_proofs.typescript`
- TypeScript proof must never overwrite Rust proof.
- `.test.spec --target-language typescript` remains unsupported and must fail before Bun runs.
- The parent owns the validator contract freeze. No worker may redefine helper eligibility, helper presence rules, or frozen unsupported-lane wording.
- The parent owns `spec-cli/src/commands.rs`. Workers do not patch CLI generator ownership or CLI routing.
- The parent owns any fallback edits to:
  - `spec-core/src/pipeline.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
- `examples/ecommerce/units/pricing/apply_tax.unit.spec` is explicitly reopened for M46 because the canonical ecommerce proof source must exercise the helper topology for real.
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**` is explicitly reopened for M46 because the packet proof source must also exercise the helper topology for real.
- A fake green is any run where helper-aware TypeScript code lands but the checked proof sources still remain zero-dep, or where TypeScript read-side success is satisfied only by Rust proof.
- The parent does not resolve semantic disagreements creatively during integration. It either:
  - applies the already-locked `PLAN.md` contract literally, or
  - bounces the lane back to the owner
- No worker may edit:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `.runs/**`
  - another lane’s files
  - parent-reserved fallback surfaces
- Do not revert or overwrite unrelated user changes. Integrate around the current repo state.

## Execution Topology

| Role | Branch | Worktree | Owner | Scope |
|---|---|---|---|---|
| primary baseline | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | authority, run-state, final landing |
| contract freeze | `ws/spec-m46-contract-freeze` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/contract-freeze` | parent | `spec-core/src/validator.rs`, frozen TS eligibility, frozen helper file names, frozen unsupported wording |
| backend helper lane | `ws/spec-m46-backend-helper` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/backend-helper` | worker A | `spec-core/src/typescript_backend.rs` and only directly adjacent backend tests in that file or module |
| proof assets lane | `ws/spec-m46-proof-assets` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/proof-assets` | worker B | `examples/ecommerce/units/pricing/apply_tax.unit.spec`, `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`, `spec-cli/tests/cli.rs` |
| docs closeout lane | `ws/spec-m46-docs-closeout` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/docs-closeout` | worker C | `README.md`, `CHANGELOG.md` only |
| integration | `ws/spec-m46-integration` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration` | parent | merge intake, CLI ownership cleanup, fallback surfaces if proven necessary, proof wall, final acceptance |

Rules:

- Worker A and worker B must fork from the exact `contract_freeze_commit` recorded in `contract-freeze.json`.
- Worker C must fork from the exact `docs_base_commit` recorded by the parent after code and proof assets are integrated.
- `feat/m40-plus` is the canonical landing branch. After the proof wall passes on `ws/spec-m46-integration`, the parent fast-forwards `feat/m40-plus` to that integrated commit before closeout.
- The parent is the sole integrator.
- Merge order is fixed:
  1. `ws/spec-m46-backend-helper`
  2. parent-only CLI ownership cleanup on `ws/spec-m46-integration`
  3. `ws/spec-m46-proof-assets`
  4. `ws/spec-m46-docs-closeout`
- Worker B stays one lane on purpose:
  - the helper-aware proof-source rewrite and the CLI coverage both depend on the same frozen helper topology contract
  - they must agree on the exact success and failure surfaces
  - keeping them together avoids a second rebase point where proof-source topology and CLI assertions could drift independently

## Canonical Run-State And Artifact Surfaces

### Authored source deliverables

Only these authored surfaces are in-bounds by default for M46:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`
- `README.md`
- `CHANGELOG.md`

Parent-reserved fallback authored surfaces:

- `spec-core/src/pipeline.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`

Inline tests follow the ownership of the file they live in.

### Parent-owned run-state artifacts

Required parent-owned artifacts under `M46_RUN_ROOT`:

- `baseline.json`
- `authority-freeze.json`
- `contract-freeze.json`
- `in-scope-files.txt`
- `out-of-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`
- `merge-log.md`
- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination
- `validation/**`

Required contents:

| Artifact | Must record |
|---|---|
| `baseline.json` | `run_id`, current branch, starting `HEAD`, scope diff anchor `ce0e16d`, initial `git status --short`, whether `PLAN.md` or `ORCH_PLAN.md` were already dirty, and the exact baseline timestamp |
| `authority-freeze.json` | authority plan path, frozen in-scope authored files, frozen out-of-scope files, parent-reserved fallback surfaces, reopened proof-source surfaces, hard-guard summary, and the explicit statement that canonical ecommerce proof must become helper-aware |
| `contract-freeze.json` | `contract_freeze_commit`, frozen helper filenames, exact TypeScript helper eligibility contract, exact unsupported boundaries, exact pre-Bun failure classes, and any frozen error strings or stable message fragments that downstream tests may assert |
| `merge-log.md` | every merge intake attempt, merge order, merge base used, parent repair files, exact reason for any bounce, exact reason for any reopened fallback surface, and the commit ids for preview, post-cleanup, and final integrated heads |
| `acceptance.md` | proof-wall command outcomes, expected versus actual result for each command family, zero-dep preservation verdict, helper-aware proof separation verdict, fake-green check, final bounded-lane checklist, and final accept or reject decision |
| `closeout.md` | landed scope summary, final diff boundary verdict, proof-wall summary, remaining risks, deferred follow-ups outside M46, and whether any parent-reserved fallback surfaces were actually touched |

### Queue and task state semantics

`tasks.json` is the authoritative machine-readable state file for the run.

Each task entry must record:

- `id`
- `order`
- `title`
- `owner`
- `branch`
- `worktree`
- `status`
- `depends_on`
- `owned_paths`
- `required_commands`
- `acceptance_summary`
- `sentinel_dir`
- `handoff_commit`
- `started_at`
- `completed_at`
- `blocked_reason`
- `restart_from`
- `notes`

Allowed `status` values are:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `merged`
- `blocked`
- `bounced`
- `done`
- `cancelled`

`queue.json` is the parent-generated runnable projection. It must record:

- current runnable task ids in order
- the single `active_task`
- open gates
- blocked gates
- whether worker concurrency slots are available
- the current integration head if integration exists
- the next required human or parent-only action

`run-state.json` must record:

- `run_id`
- `current_phase`
- `scope_anchor_commit`
- `contract_freeze_commit`
- `docs_base_commit`
- `integration_head`
- `proof_wall_state`
- `final_status`

### Expected validation records

Minimum expected validation records:

- `validation/baseline/git-status.short.txt`
- `validation/baseline/git-diff.scope-anchor-name-only.txt`
- `validation/baseline/git-diff.scope-anchor-stat.txt`
- `validation/authority/in-scope-files.txt`
- `validation/authority/out-of-scope-files.txt`
- `validation/authority/plan-summary.txt`
- `validation/contract-freeze/spec-core-tests.txt`
- `validation/contract-freeze/validator-contract-notes.txt`
- `validation/backend-intake/spec-core-tests.txt`
- `validation/backend-intake/spec-generate-monotone-up-typescript.txt`
- `validation/backend-intake/spec-build-monotone-up-typescript.txt`
- `validation/cli-cleanup/spec-cli-tests.txt`
- `validation/cli-cleanup/commands-routing-notes.txt`
- `validation/proof-assets/spec-test-packet-aligned-typescript.txt`
- `validation/proof-assets/spec-test-example-apply-tax-typescript.txt`
- `validation/proof-assets/spec-test-molecule-negative-typescript.txt`
- `validation/proof-assets/spec-status-example-typescript.json`
- `validation/merge/final-name-only.diff`
- `validation/merge/final-stat.diff`
- `validation/proof-wall/cargo-test.txt`
- `validation/proof-wall/spec-generate-packet-aligned-typescript.txt`
- `validation/proof-wall/spec-build-packet-aligned-typescript.txt`
- `validation/proof-wall/spec-test-packet-aligned-typescript.txt`
- `validation/proof-wall/spec-test-packet-drift-typescript.txt`
- `validation/proof-wall/spec-test-packet-unsupported-near-miss-typescript.txt`
- `validation/proof-wall/spec-test-example-apply-tax-typescript.txt`
- `validation/proof-wall/spec-test-molecule-negative-typescript.txt`
- `validation/proof-wall/spec-status-example-typescript.json`
- `validation/proof-wall/spec-export-example.json`
- `validation/proof-wall/family-prove-typescript.txt`
- `validation/closeout/final-git-status.short.txt`

### Per-task sentinels

Required sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-05-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-10-contract-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-15-worker-launch/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-a-backend-helper/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-25-backend-intake/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-30-cli-ownership/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-b-proof-assets/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-40-docs-launch/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-c-docs-closeout/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-f-integration/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-50-proof-wall/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m46-60-closeout/`

Each sentinel directory may contain:

- `started.json`
- `status.json`
- `done.json`
- `blocked.json`

## Queue And Gates

| Order | ID | Kind | Owner | Worktree | Opens when |
|---|---|---|---|---|---|
| 1 | `task-m46-00-baseline` | gate | parent | primary | repo baseline and dirty-state capture complete |
| 2 | `task-m46-05-authority-freeze` | gate | parent | primary | `PLAN.md` scope, in-scope files, and hard guards are recorded |
| 3 | `task-m46-10-contract-freeze` | task | parent | `ws/spec-m46-contract-freeze` | baseline and authority freeze are complete |
| 4 | `task-m46-15-worker-launch` | gate | parent | primary | `contract-freeze.json` exists with one frozen commit and frozen wording |
| 5 | `task-m46-a-backend-helper` | task | worker A | `ws/spec-m46-backend-helper` | worker launch gate is open |
| 6 | `task-m46-b-proof-assets` | task | worker B | `ws/spec-m46-proof-assets` | worker launch gate is open |
| 7 | `task-m46-25-backend-intake` | gate | parent | `ws/spec-m46-integration` | backend lane is submitted or explicitly blocked |
| 8 | `task-m46-30-cli-ownership` | task | parent | `ws/spec-m46-integration` | backend intake is green |
| 9 | `task-m46-40-docs-launch` | gate | parent | primary | proof-assets lane is merged and code truth is stable |
| 10 | `task-m46-c-docs-closeout` | task | worker C | `ws/spec-m46-docs-closeout` | docs launch gate is open |
| 11 | `task-m46-f-integration` | task | parent | `ws/spec-m46-integration` | docs lane is submitted or explicitly blocked |
| 12 | `task-m46-50-proof-wall` | gate | parent | `ws/spec-m46-integration` | integrated branch is merged and locally consistent |
| 13 | `task-m46-60-closeout` | gate | parent | primary | full proof wall is green, `feat/m40-plus` is ready to fast-forward |

## Workstream Plan

### `task-m46-00-baseline` - parent only

Purpose:

- capture the exact M46 baseline before any M46 worktree opens

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/validation/baseline
git rev-parse --abbrev-ref HEAD
git rev-parse --short HEAD
git merge-base --is-ancestor ce0e16d HEAD
git status --short
git diff --name-only ce0e16d..HEAD
git diff --stat ce0e16d..HEAD
```

Blocked conditions:

- current branch is not `feat/m40-plus`
- current `HEAD` is not a descendant of `ce0e16d`
- repo state is ambiguous enough that the parent cannot distinguish pre-existing unrelated edits from M46 execution state

Restart point if blocked:

- stop before worktree creation
- restart from `task-m46-00-baseline` after the parent re-establishes the correct branch and baseline

### `task-m46-05-authority-freeze` - parent only

Purpose:

- freeze the authoritative M46 scope, reopened proof surfaces, parent-reserved fallback surfaces, and hard guards

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/validation/authority
sed -n '1,500p' PLAN.md
git ls-files spec-core/src spec-cli/src README.md CHANGELOG.md semantic-families examples
```

Acceptance:

- `apply_tax.unit.spec` is explicitly reopened as the canonical helper-aware ecommerce proof source
- monotone-up packet fixtures are explicitly reopened as the helper-aware packet proof source
- `pipeline.rs`, `passport.rs`, and `export.rs` remain parent-reserved only
- the frozen out-of-scope list explicitly excludes wrapper, seam, molecule, multi-dep, and cross-library TypeScript work

Blocked conditions:

- `PLAN.md` scope is unclear, contradictory, or changes mid-run
- the parent cannot freeze reopened proof-source surfaces cleanly
- the parent identifies mandatory authored files outside the authorized M46 surface

Restart point if blocked:

- stop before editing the contract-freeze lane
- restart from `task-m46-05-authority-freeze` after the parent resolves scope authority

### `task-m46-10-contract-freeze` - parent only

Purpose:

- lock the helper-aware validator contract and frozen unsupported boundaries before parallel implementation begins

Owned files:

- `spec-core/src/validator.rs`

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/contract-freeze -b ws/spec-m46-contract-freeze feat/m40-plus
cargo test -p spec-core -- --color never
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/contract-freeze status --short
```

Acceptance:

- one shared frozen TypeScript eligibility contract exists and is recorded
- the contract is exactly:
  - zero deps allowed
  - one direct dep allowed only if semantically classified as `function.helper.identity_passthrough.v1`
  - helper must exist in the same loaded unit set and generated tree
  - dep count `> 1` fails before Bun
  - wrong helper family fails before Bun
  - cross-library helper dep fails for this lane
  - `.test.spec` stays unsupported
- frozen generated helper filenames are recorded:
  - `__spec_ts/runtime.ts`
  - `__spec_ts/build_entry.ts`
  - `__spec_ts/local_tests.ts`
- frozen unsupported-lane wording is specific enough that worker B can assert against it without guessing

Blocked conditions:

- the parent cannot stabilize eligibility, helper filenames, or failure wording cleanly enough for parallel work
- the bounded TS gate still depends on backend generation or pipeline behavior to decide basic eligibility
- freezing the validator contract would require reopening worker-owned or out-of-scope files

Restart point if blocked:

- stop before worker launch
- restart from `task-m46-10-contract-freeze` after the parent resolves the contract locally

### `task-m46-15-worker-launch` - parent only

Purpose:

- launch the first two worker lanes from one frozen commit and record exact ownership plus banned drift

Required commands:

```bash
git rev-parse --short ws/spec-m46-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/backend-helper -b ws/spec-m46-backend-helper ws/spec-m46-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/proof-assets -b ws/spec-m46-proof-assets ws/spec-m46-contract-freeze
git worktree list
```

Acceptance:

- worker A and worker B both fork from the exact `contract_freeze_commit`
- worker A prompt contains only backend-helper scope
- worker B prompt contains only proof-source and CLI coverage scope
- both workers receive exact commands, frozen names, hard guards, and bounce rules

Blocked conditions:

- `contract-freeze.json` is missing or does not identify a single frozen commit
- worker worktrees or branches cannot be created cleanly from the frozen commit
- lane ownership is ambiguous enough that a worker would need to guess file scope

Restart point if blocked:

- stop before issuing worker prompts
- restart from `task-m46-15-worker-launch` after worktrees and lane contracts are clean

### `task-m46-a-backend-helper` - worker A

Purpose:

- make helper-aware monotone-up generation and harness execution truthful in the backend without owning routing, proof storage, or docs

Owned files:

- `spec-core/src/typescript_backend.rs`
- directly adjacent unit tests in `spec-core/src/typescript_backend.rs`
- directly adjacent helper-emission assertions only if they live in that module

Must not touch:

- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-core/src/pipeline.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- packet fixtures
- ecommerce example
- CLI tests
- docs

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
git status --short
```

Acceptance:

- helper-aware monotone-up generation emits the helper module into the same output tree
- the generated leaf module imports the helper through the truthful relative path
- the generated local test harness still works for helper-aware units
- the implementation stays topology-specific and does not introduce generic dep-graph scheduling
- zero-dep M45 behavior is preserved in backend tests or existing command behavior
- the lane is merge-safe only if the required commands pass and changed files stay inside owned paths

Blocked conditions:

- a required fix crosses into validator, CLI, pipeline, proof-storage, export, or docs ownership
- a failing command indicates routing or proof-surface drift rather than backend generation drift
- the frozen helper filenames or validator contract no longer suffice for backend generation

Restart point if blocked:

- stop in `ws/spec-m46-backend-helper`
- report the blocked file, exact failing command, and violated frozen assumption
- restart from `task-m46-a-backend-helper` after the parent republishes a valid freeze or reassigns the cross-lane fix

### `task-m46-b-proof-assets` - worker B

Purpose:

- refresh one real helper-aware packet proof source and one real helper-aware ecommerce proof source, then lock product coverage against that same topology

Owned files:

- `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`
- `spec-cli/tests/cli.rs`

Must not touch:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/src/commands.rs`
- `spec-core/src/pipeline.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- docs

Required commands:

```bash
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
git status --short
```

Acceptance:

- the packet aligned proof source actually becomes helper-aware, not merely renamed
- the ecommerce `apply_tax` proof source actually becomes helper-aware, not merely restated
- the negative path signal proves the worker exercised the helper topology boundary:
  - unsupported near miss fails before Bun
  - molecule target still fails before Bun
- `spec-cli/tests/cli.rs` covers:
  - helper-aware monotone-up success
  - wrong helper family rejection
  - missing helper rejection
  - dep count `> 1` rejection
  - molecule rejection still intact
  - TypeScript proof separation from Rust proof
  - zero-dep preservation at the product surface
- `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json` is an expected non-green truth surface for M46:
  - command exits `1`
  - `pricing/apply_tax` is `valid`
  - `money/round`, `pricing/apply_discount`, `pricing/calculate_total`, `pricing/checkout_quote`, and `pricing/discount_policy` are `untested`
  - no Rust proof inheritance appears in the TypeScript status view
- the lane is merge-safe only if the required commands produce their expected outcomes and changed files stay inside owned paths

Blocked conditions:

- CLI assertions require validator wording that was not frozen
- proof-source refresh requires backend, CLI routing, pipeline, proof-storage, export, or docs changes
- the helper-aware packet and helper-aware ecommerce example cannot be made truthful under the bounded one-helper contract

Restart point if blocked:

- stop in `ws/spec-m46-proof-assets`
- report the blocked file, exact failing command, and violated frozen assumption
- restart from `task-m46-b-proof-assets` after the parent resolves the upstream issue

### `task-m46-25-backend-intake` - parent only

Purpose:

- merge the backend lane into the integration worktree and prove the backend surface is stable enough for parent-owned CLI cleanup

Owned files and artifacts:

- `M46_RUN_ROOT/merge-log.md`
- `M46_RUN_ROOT/validation/backend-intake/**`
- `ws/spec-m46-integration`

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration -b ws/spec-m46-integration ws/spec-m46-contract-freeze
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration merge --no-ff ws/spec-m46-backend-helper
cargo test -p spec-core -- --color never
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration status --short
```

Acceptance:

- backend lane merges cleanly onto the frozen contract base
- parent repair is limited to merge mechanics only
- if backend semantics disagree with the frozen contract, the parent bounces worker A instead of reinterpreting the plan
- the post-merge backend preview head is recorded in `merge-log.md`

Blocked conditions:

- merge conflicts cannot be resolved without semantic reinterpretation
- the merged backend preview fails required generate or build commands
- backend lane drift reaches non-owned files

Restart point if blocked:

- stop in `ws/spec-m46-integration`
- record the blocker in `merge-log.md`
- restart from `task-m46-a-backend-helper` if worker A must reland, otherwise restart from `task-m46-25-backend-intake`

### `task-m46-30-cli-ownership` - parent only

Purpose:

- collapse duplicate CLI-side TypeScript generator ownership after backend truth exists, and reopen fallback surfaces only if evidence proves it is necessary

Owned files:

- `spec-cli/src/commands.rs`
- parent-reserved fallback surfaces only if proven necessary:
  - `spec-core/src/pipeline.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`

Required commands:

```bash
cargo test -p spec-cli --test cli -- --color never
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration status --short
```

Acceptance:

- `commands.rs` routes and reports only
- `typescript_backend.rs` remains the single generator owner
- any fallback change to `pipeline.rs`, `passport.rs`, or `export.rs` must be justified in `merge-log.md` by a specific failing command
- the parent does not widen M46 while fixing CLI ownership
- the post-cleanup head is recorded as the base that proof-assets must merge onto

Blocked conditions:

- parent cleanup would require semantic changes outside `PLAN.md`
- routing cleanup requires unfrozen validator drift
- fallback surfaces would need speculative edits rather than evidence-backed edits

Restart point if blocked:

- stop in `ws/spec-m46-integration`
- record the blocker in `merge-log.md`
- restart from `task-m46-10-contract-freeze` if the validator contract was insufficient, otherwise restart from `task-m46-30-cli-ownership`

### `task-m46-40-docs-launch` - parent only

Purpose:

- merge proof assets onto the stable code path, then fork the late docs lane from that stable integrated head

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration merge --no-ff ws/spec-m46-proof-assets
cargo test -p spec-cli --test cli -- --color never
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration rev-parse --short HEAD
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/docs-closeout -b ws/spec-m46-docs-closeout "$(git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration rev-parse HEAD)"
```

Acceptance:

- proof-assets lane merges onto the post-cleanup integration head
- parent repair is limited to merge mechanics only
- if proof assets semantically disagree with the frozen contract or the integrated backend path, the parent bounces worker B
- `docs_base_commit` is recorded after proof-assets merge, not before

Blocked conditions:

- proof-assets merge requires creative semantic resolution
- integrated proof-assets tree fails required CLI tests
- proof-assets drift reaches non-owned files

Restart point if blocked:

- stop before docs launch
- record the blocker in `merge-log.md`
- restart from `task-m46-b-proof-assets` if worker B must reland, otherwise restart from `task-m46-40-docs-launch`

### `task-m46-c-docs-closeout` - worker C

Purpose:

- move user-facing docs only after code and proof behavior are stable

Owned files:

- `README.md`
- `CHANGELOG.md`

Must not touch:

- any code files
- any test files
- any fixtures
- any examples
- any run artifacts

Required commands:

```bash
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
git status --short
```

Acceptance:

- README states exactly the bounded M46 lane and no broader claim
- CHANGELOG records only the helper-aware monotone-up widening
- `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json` is an expected non-green truth surface for M46 docs:
  - command exits `1`
  - the bounded TypeScript lane is documented as per-eligible-unit, not whole-root green
  - no wording implies that the entire ecommerce root is proven in TypeScript
- docs lane remains low-risk and late by design
- the lane is merge-safe only if changed files stay inside owned paths and required commands produce their expected outcomes

Blocked conditions:

- truthful docs would require broader product claims than the landed code allows
- docs lane appears to need code, test, fixture, or example edits
- CLI outputs are still unstable enough that docs would guess

Restart point if blocked:

- stop in `ws/spec-m46-docs-closeout`
- report the blocked file or wording gap
- restart from `task-m46-c-docs-closeout` after the parent resolves the upstream instability

### `task-m46-f-integration` - parent only

Purpose:

- merge docs, re-check scope boundaries, and prepare the exact integrated branch that faces the proof wall

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration merge --no-ff ws/spec-m46-docs-closeout
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration diff --name-only ce0e16d..HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration diff --stat ce0e16d..HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration status --short
```

Acceptance:

- final integrated diff stays inside frozen authored surfaces plus derived proof output and parent-owned run state
- no silent drift is introduced in:
  - helper filenames
  - validator contract
  - unsupported boundaries
  - proof separation
  - command flag boundaries
- the parent does not normalize a widened milestone during integration

Blocked conditions:

- merge conflicts cannot be resolved without reopening ownership or widening scope
- final integrated tree requires out-of-scope authored files
- proof-wall preparation depends on an unfrozen new surface

Restart point if blocked:

- stop in `ws/spec-m46-integration`
- record the blocker in `merge-log.md`
- restart from the owning lane or from `task-m46-30-cli-ownership`, depending on where drift began

### `task-m46-50-proof-wall` - parent only

Purpose:

- run and record the exact M46 proof wall on the integrated branch and treat any unexpected result as a stop condition

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/validation/proof-wall
cargo test
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/drift/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Expected results by command family:

- `cargo test`
  - full repo test suite passes
  - zero-dep monotone-up regressions stay green
  - helper-aware proof separation regressions stay green
- `spec generate` on packet aligned units
  - generated tree includes helper-aware modules and frozen helper files
  - output tree is same-generated-tree honest, not cross-library resolved
- `spec build` on packet aligned units
  - Bun build succeeds for the helper-aware packet
- `spec test` on packet aligned units
  - helper-aware packet succeeds in the TS lane
  - this is one of the two required checked helper-aware proof sources
- `spec test` on packet drift units
  - drift behavior remains truthful and does not silently greenlight the wrong semantics
- `spec test` on packet unsupported near miss
  - failure occurs before Bun executes unsupported semantics
  - near miss remains unsupported, not silently promoted
- `spec test` on ecommerce `apply_tax.unit.spec`
  - helper-aware ecommerce example succeeds in the TS lane
  - this is the second required checked helper-aware proof source
- `spec test` on `discount_plus_tax.test.spec`
  - molecule target is rejected for TypeScript before Bun runs
  - this remains an explicit boundary check in M46
- `spec status` on ecommerce root
  - command exits `1`
  - reads only `target_proofs.typescript`
  - reports TypeScript truth without inheriting Rust proof
  - reflects the helper-aware example as freshly proven in the TS lane
  - `pricing/apply_tax` is `valid`
  - `money/round`, `pricing/apply_discount`, `pricing/calculate_total`, `pricing/checkout_quote`, and `pricing/discount_policy` are `untested`
- `spec export` on ecommerce units
  - additive proof truth is preserved
  - Rust proof remains untouched by TypeScript execution
- `cargo xtask family prove ... --target-language typescript`
  - family-level proof remains green after helper-aware widening

Blocked conditions:

- any proof-wall command fails
- either checked proof source remains zero-dep after the supposed helper-aware code landed
- TypeScript proof overwrites Rust proof or status mirrors Rust truth
- molecule target reaches Bun instead of failing at the boundary
- fixing the failure would require out-of-scope files or widened milestone semantics

Restart point if blocked:

- stop with the integrated branch intact
- record the blocker in `acceptance.md` and `blocked.json`
- restart from the narrowest upstream owner that can fix the evidence-backed failure

### `task-m46-60-closeout` - parent only

Purpose:

- fast-forward the proven integration branch onto `feat/m40-plus` and close the run only after scope, proof, and docs all agree

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/validation/closeout
test "$(git rev-parse --abbrev-ref HEAD)" = "feat/m40-plus"
git merge --ff-only ws/spec-m46-integration
git status --short
git diff --name-only ce0e16d..HEAD
git diff --stat ce0e16d..HEAD
```

Acceptance:

- `feat/m40-plus` fast-forwards cleanly to the proven integration head
- final diff remains in-bounds
- closeout records whether parent-reserved fallback surfaces were untouched or touched with evidence-backed justification
- final status is green only if the fake-green check is explicitly passed

Blocked conditions:

- closeout is not running from the primary `feat/m40-plus` worktree
- final diff contains out-of-bounds authored files
- `feat/m40-plus` cannot fast-forward cleanly to `ws/spec-m46-integration`
- acceptance still relies on unresolved blockers or undocumented scope deviations

Restart point if blocked:

- stop before declaring M46 complete
- record the blocker in `closeout.md`
- restart from the narrowest upstream task that caused the boundary failure

## Scope-Boundary Checks

Required checks:

- capture baseline diff surfaces during `task-m46-00-baseline`
- capture per-lane merge intake behavior in `merge-log.md`
- capture final integrated name-only and stat diffs during `task-m46-f-integration`
- re-check final name-only and stat diffs during `task-m46-60-closeout`

Required commands:

```bash
git diff --name-only ce0e16d..HEAD
git diff --stat ce0e16d..HEAD
```

Boundary rule:

- every changed file in the final diff must be one of:
  - a frozen M46 authored surface
  - a derived proof artifact changed only by proof commands
  - a parent-owned `.runs/**` execution artifact

Blocked rule:

- any out-of-bounds authored diff is a blocker, not a follow-up idea
- do not silently absorb opportunistic refactors, broader TypeScript ambition, or unrelated cleanup into M46
- if a required fix truly needs a new authored surface, stop and reopen authority explicitly instead of normalizing it during integration

## Context-Control Rules

### Parent prompt rules

- Every worker prompt must include only:
  - the lane’s owned file set
  - the exact relevant `PLAN.md` excerpts
  - the recorded `contract_freeze_commit`
  - frozen helper filenames and frozen message fragments that matter for that lane
  - required commands
  - explicit forbidden touch surfaces
  - lane-local acceptance criteria
  - bounce rules
- Do not paste the full repo state or full orchestration history into worker prompts.
- The parent reviews narrow diffs plus command outcomes. It does not absorb giant worker transcripts into main context.
- Every scope exception, freeze reopening, merge-order deviation, and fallback-surface reopening must be written to `session-log.md` or `merge-log.md`.

### Worker return rules

- Each worker must return only:
  - changed files
  - commands run
  - exit codes
  - handoff commit
  - blockers or unresolved assumptions
  - whether the lane is safe to merge
- Workers do not write `M46_RUN_ROOT/*`.
- Workers do not write acceptance narratives, merge records, or orchestration state.
- Close each worker immediately after merge or bounce. Do not keep idle workers live after their lane has either landed or been rejected.

### Integration discipline rules

- The parent is the only integrator.
- The parent may resolve only:
  - merge mechanics
  - line-level non-semantic conflicts
  - evidence-backed parent-owned cleanup in `commands.rs` and reserved fallback surfaces
- The parent may not:
  - reinterpret helper eligibility
  - redefine packet topology
  - redefine proof-source intent
  - widen the TypeScript lane
- If two lanes disagree semantically, the parent either applies `PLAN.md` literally or bounces the offending lane. It does not invent a compromise in integration.

## Tests And Acceptance

### Required proof wall

The parent integration lane must run the exact M46 wall listed in `task-m46-50-proof-wall`.

### Mandatory non-goal checks

These are release-critical and must be covered by tests or command-path assertions even when they are not separate proof-wall commands:

- `spec validate` still has no TypeScript target support
- `spec export` still has no TypeScript target support
- `.test.spec --target-language typescript` fails before Bun runs
- units outside `function.arithmetic_leaf.monotone_up.v1` fail before Bun runs
- wrong helper family fails before Bun runs
- missing helper fails before Bun runs
- multi-dep units fail before Bun runs
- cross-library helper refs fail for this lane
- zero-dep monotone-up behavior is preserved
- Rust remains the default target everywhere

### Milestone acceptance checklist

M46 is complete only if all of the following are true:

1. The M45 zero-dep monotone-up lane still passes unchanged.
2. A monotone-up unit with exactly one helper passthrough dep executes in the TypeScript lane.
3. The helper dep is accepted only when it classifies as `function.helper.identity_passthrough.v1`.
4. Missing helper-in-tree fails before Bun runs.
5. More than one direct dep fails before Bun runs.
6. `.test.spec --target-language typescript` remains unsupported.
7. `spec-cli/src/commands.rs` no longer owns a second TypeScript generator path.
8. The checked packet proof source is helper-aware for real.
9. The checked ecommerce proof source is helper-aware for real.
10. The final acceptance ledger explicitly passes the fake-green check.
11. Rust proof remains untouched by TypeScript execution.
12. `status --target-language typescript` reads only TS proof.
13. `export` remains additive and honest.
14. README and CHANGELOG describe only the bounded M46 lane.
15. The full proof wall passes.

## Assumptions

- The parent launches from `feat/m40-plus` with `HEAD` at or ahead of `ce0e16d`.
- Bun is available by the time the proof wall runs, or missing-Bun messaging is actionable enough to fail honestly.
- Existing helper-family semantic truth remains authoritative and does not require new family-design work in M46.
- The monotone-up packet can be refreshed with packet-local helper units in the same general style already used by the monotone-down packet.
- `apply_tax.unit.spec` can be reopened without widening the example library beyond one direct helper dep.
- Any required changes to `pipeline.rs`, `passport.rs`, or `export.rs` are narrow enough for parent-only cleanup and do not imply a schema redesign.
