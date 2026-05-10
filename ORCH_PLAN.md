# M44 Orchestration Plan

Status: **authoritative kickoff and execution contract for M44 shared-core portability-contract freeze**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Execute from current branch: **`feat/m40-plus`**  
Current baseline commit: **`7a986a6`**  
Last rewritten: **`2026-05-10`**

## Summary

- Execute from the current repo root on branch `feat/m40-plus` at baseline `7a986a6` until the parent opens worktrees.
- Keep the true critical path local to the parent for:
  - baseline capture
  - authority freeze
  - contract-freeze API ownership
  - merge ordering
  - parity cleanup
  - final proof wall
  - closeout
- Run four post-freeze worker lanes in parallel. That is the maximum safe split for M44 without letting multiple workers co-own the same `spec-core` policy surface.
- Worker concurrency cap is **4**.
- Worker model assumption is fixed for all worker lanes:
  - `model = GPT-5.4`
  - `reasoning_effort = high`
- Use dedicated `spec-m44` worktrees and branches:
  - primary baseline: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `ws/spec-m44-contract-freeze` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/contract-freeze`
  - `ws/spec-m44-validator-markers` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/validator-markers`
  - `ws/spec-m44-projection-gate` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/projection-gate`
  - `ws/spec-m44-semantic-readside` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/semantic-readside`
  - `ws/spec-m44-docs` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/docs`
  - `ws/spec-m44-integration` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/integration`
- Keep orchestration state in one canonical parent-owned run root:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `M44_RUN_ROOT=$PRIMARY_ROOT/.runs/m44_shared_core_portability_contract`
  - `queue=$M44_RUN_ROOT/queue.json`
  - `tasks=$M44_RUN_ROOT/tasks.json`
  - `session_log=$M44_RUN_ROOT/session-log.md`
  - `contract_freeze=$M44_RUN_ROOT/contract-freeze.json`
  - `acceptance=$M44_RUN_ROOT/acceptance.md`
  - `merge_log=$M44_RUN_ROOT/merge-log.md`
- Treat authored source, run-state artifacts, and derived proof artifacts as different classes:
  - authored source is the milestone deliverable
  - `.runs/**` is parent-owned orchestration state only
  - refreshed passports, molecule evidence, export captures, and `xtask` JSON captures are derived proof output only

## Hard Guards

- `PLAN.md` is the sole scope authority. `ORCH_PLAN.md` is the execution contract, not a second spec.
- M44 adds exactly one new module:
  - `spec-core/src/portability_contract.rs`
- `spec-core/src/lib.rs` may be updated only to export the new module.
- The code-rewire scope is frozen to:
  - `spec-core/src/validator.rs`
  - `spec-core/src/backend_execution.rs`
  - `spec-core/src/escape_hatch.rs`
  - `spec-core/src/portability.rs`
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
- The documentation scope is frozen to:
  - `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
  - `semantic-families/README.md`
  - `README.md` only if the parent explicitly decides it is necessary
  - `CHANGELOG.md` only if the parent explicitly decides it is necessary
- Do not widen into:
  - a new crate
  - second-language execution
  - fresh family-choice work
  - recommendation-policy work
  - schema churn by default
  - a new machine-readable gate
- The contract-freeze gate is mandatory and first. No worker lane starts before the parent records a frozen API surface.
- The parent is the sole owner of:
  - contract API freeze
  - `M44_RUN_ROOT/**`
  - merge sequencing
  - final parity cleanup
  - proof wall execution
  - closeout artifacts
- No worker lane may edit:
  - `ORCH_PLAN.md`
  - `.runs/**`
  - any file outside its assigned lane without explicit parent reassignment
- No worker may silently change the frozen contract API after launch. If the freeze is wrong, stop and bounce to the parent.
- No worker may resolve shared `spec-core` overlap by rebasing into another worker lane. All cross-lane resolution happens through the parent integration lane.
- The docs lane must describe only landed code truth. It must not imply:
  - new shared portability schema
  - broader function-family support
  - broader second-language claims
  - new crate extraction
- Read-side parity is release-critical:
  - `semantic_review`
  - passport projection
  - export projection
  - CLI status / export rendering
  must agree on the same underlying portability truth.
- If a schema bump, broader file scope, or additional module split becomes necessary, stop and escalate instead of mutating the milestone silently.
- Do not revert or overwrite other contributors' changes outside this plan's owned scope. Integrate around the current repo state.

## Execution Topology

| Role | Branch | Worktree | Owner | Scope |
|---|---|---|---|---|
| primary baseline | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | authority, run-state, final landing |
| contract freeze | `ws/spec-m44-contract-freeze` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/contract-freeze` | parent | `portability_contract.rs`, `lib.rs`, contract direct tests |
| validator and raw markers | `ws/spec-m44-validator-markers` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/validator-markers` | worker A | `validator.rs`, `backend_execution.rs`, lane-local regressions |
| projection and gate | `ws/spec-m44-projection-gate` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/projection-gate` | worker B | `escape_hatch.rs`, `portability.rs`, lane-local regressions |
| semantic and read-side | `ws/spec-m44-semantic-readside` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/semantic-readside` | worker C | `semantic_review.rs`, `passport.rs`, `export.rs`, `commands.rs`, `spec-cli/tests/cli.rs` |
| docs | `ws/spec-m44-docs` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/docs` | worker D | roadmap and maintainer-facing docs only |
| integration | `ws/spec-m44-integration` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/integration` | parent | merge, drift repair, proof wall, final acceptance |

Rules:

- Every worker branch must fork from the exact `contract_freeze_commit` captured in `contract-freeze.json`.
- No worker may fork from a later ad hoc `feat/m40-plus` HEAD after the freeze opens.
- The parent is the sole integrator.
- The parent merges in this default order:
  1. `ws/spec-m44-validator-markers`
  2. `ws/spec-m44-projection-gate`
  3. `ws/spec-m44-semantic-readside`
  4. `ws/spec-m44-docs`
- The merge order is intentional:
  - lane A stabilizes validator and marker call sites first
  - lane B owns the projection layer that lane C consumes
  - lane C owns downstream semantic and CLI parity
  - lane D is intentionally last because it must match landed code, not forecasted code
- If lane C discovers a frozen API gap in lane B or the contract surface itself, it must stop and hand the gap to the parent. It may not patch `portability.rs` or the contract module on its own.

## Canonical Run-State And Artifact Surfaces

### Authored source deliverables

Only these authored surfaces are in-bounds for M44:

- `spec-core/src/portability_contract.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/backend_execution.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/portability.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `semantic-families/README.md`
- `README.md` only if opened by the parent
- `CHANGELOG.md` only if opened by the parent

Lane-local tests may live inline in the owned source modules. Inline tests follow the same ownership as the source file that contains them.

### Parent-owned run-state artifacts

Canonical parent-owned run root:

- `M44_RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract`

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

Execution-record expectations:

- `baseline.json`
  - baseline branch
  - baseline commit
  - initial `git status --short`
  - initial authored-surface inventory
- `authority-freeze.json`
  - authority `PLAN.md` path
  - frozen in-scope authored surfaces
  - frozen out-of-scope surfaces
  - hard-guard summary
- `contract-freeze.json`
  - `contract_freeze_commit`
  - exported helper/type names
  - lane ownership map
  - banned post-freeze API drift
- `merge-log.md`
  - merge order
  - conflicts encountered
  - exact files manually repaired by the parent
  - explicit note if a merge was paused and relaunched
- `acceptance.md`
  - proof-wall command outcomes
  - preserved-truth checklist
  - final diff boundary verdict
- `closeout.md`
  - landed scope summary
  - remaining risks
  - blocked items, if any
  - follow-up decisions deferred outside M44

Expected `M44_RUN_ROOT/validation/` records:

- `validation/baseline/git-status.short.txt`
- `validation/baseline/git-diff.baseline-name-only.txt`
- `validation/baseline/git-diff.baseline-stat.txt`
- `validation/authority/in-scope-files.txt`
- `validation/authority/out-of-scope-files.txt`
- `validation/contract-freeze/spec-core-tests.txt`
- `validation/merge/git-merge-status.txt`
- `validation/merge/final-name-only.diff`
- `validation/merge/final-stat.diff`
- `validation/proof-wall/cargo-test.txt`
- `validation/proof-wall/spec-validate.json`
- `validation/proof-wall/spec-test.txt`
- `validation/proof-wall/spec-status.json`
- `validation/proof-wall/spec-export.json`
- `validation/proof-wall/family-recommend.json`
- `validation/proof-wall/family-corpus-decision.json`
- `validation/proof-wall/family-verify-decision-contract.json`
- `validation/closeout/final-git-status.short.txt`

The parent may add more evidence files, but these are the minimum expected execution record surfaces.

### Per-task sentinels

Each gate or task gets a sentinel directory under `.runs/`:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-05-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-a1-contract-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-15-worker-launch/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-b-validator-markers/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-c-projection-gate/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-d-semantic-readside/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-e-docs/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-40-merge-window/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-f-integration/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-50-proof-wall/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-60-closeout/`

Each sentinel directory may contain:

- `started.json`
- `status.json`
- `done.json`
- `blocked.json`

### Derived proof artifacts

These are derived outputs, not authored deliverables:

- refreshed `examples/ecommerce/**/*.spec.passport.json`
- refreshed `examples/ecommerce/**/*.test.evidence.json`
- JSON captures under `M44_RUN_ROOT/validation/` for:
  - `validate`
  - `status`
  - `export`
  - `xtask family recommend`
  - `xtask family corpus-decision`
  - `xtask family verify-decision-contract`

Rules:

- Derived proof artifacts may change only as a result of the proof commands.
- No one hand-edits passports, molecule evidence, or command captures.
- Parent acceptance is based on command results and repo truth, not on preserving generated output as authored scope.

## Queue And Gates

| Order | ID | Kind | Owner | Worktree | Opens when |
|---|---|---|---|---|---|
| 1 | `task-m44-00-baseline` | gate | parent | primary | repo baseline and dirty-state capture complete |
| 2 | `task-m44-05-authority-freeze` | gate | parent | primary | `PLAN.md` scope, in-scope files, and hard guards are recorded |
| 3 | `task/m44-a1-contract-freeze` | task | parent | `ws/spec-m44-contract-freeze` | baseline and authority freeze are complete |
| 4 | `task-m44-15-worker-launch` | gate | parent | primary | `contract-freeze.json` exists with commit, owned symbols, and banned drift |
| 5 | `task/m44-b-validator-markers` | task | worker A | `ws/spec-m44-validator-markers` | worker launch gate is open |
| 6 | `task/m44-c-projection-gate` | task | worker B | `ws/spec-m44-projection-gate` | worker launch gate is open |
| 7 | `task/m44-d-semantic-readside` | task | worker C | `ws/spec-m44-semantic-readside` | worker launch gate is open |
| 8 | `task/m44-e-docs` | task | worker D | `ws/spec-m44-docs` | worker launch gate is open |
| 9 | `task-m44-40-merge-window` | gate | parent | primary | all worker handoffs are submitted or explicitly blocked |
| 10 | `task/m44-f-integration` | task | parent | `ws/spec-m44-integration` | merge window is open |
| 11 | `task-m44-50-proof-wall` | gate | parent | `ws/spec-m44-integration` | integration branch is merged and locally consistent |
| 12 | `task-m44-60-closeout` | gate | parent | primary | full proof wall is green and acceptance is recorded |

## Workstream Plan

### `task-m44-00-baseline` - parent only

Purpose:

- capture the execution baseline before any M44 worktree opens

Owned files and artifacts:

- `M44_RUN_ROOT/baseline.json`
- `M44_RUN_ROOT/run-state.json`
- `M44_RUN_ROOT/validation/baseline/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-00-baseline/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract/validation/baseline
git rev-parse --abbrev-ref HEAD
git rev-parse --short HEAD
git status --short
git diff --name-only 7a986a6..HEAD
git diff --stat 7a986a6..HEAD
```

Artifacts written:

- `baseline.json`
- `validation/baseline/git-status.short.txt`
- `validation/baseline/git-diff.baseline-name-only.txt`
- `validation/baseline/git-diff.baseline-stat.txt`
- `task-m44-00-baseline/started.json`
- `task-m44-00-baseline/done.json`

Blocked conditions:

- current branch is not `feat/m40-plus`
- current baseline commit is not `7a986a6`
- repo state is ambiguous enough that the parent cannot distinguish pre-existing unrelated edits from M44 execution state

Restart point if blocked:

- stop before worktree creation
- record the reason in `task-m44-00-baseline/blocked.json`
- restart from `task-m44-00-baseline` after the parent re-establishes the correct branch and baseline

### `task-m44-05-authority-freeze` - parent only

Purpose:

- freeze the authoritative scope, authored surfaces, and non-negotiable guards before the contract lane begins

Owned files and artifacts:

- `M44_RUN_ROOT/authority-freeze.json`
- `M44_RUN_ROOT/in-scope-files.txt`
- `M44_RUN_ROOT/validation/authority/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-05-authority-freeze/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract/validation/authority
sed -n '1,260p' PLAN.md
git ls-files spec-core/src spec-cli/src docs semantic-families README.md CHANGELOG.md
```

Artifacts written:

- `authority-freeze.json`
- `in-scope-files.txt`
- `validation/authority/in-scope-files.txt`
- `validation/authority/out-of-scope-files.txt`
- `task-m44-05-authority-freeze/started.json`
- `task-m44-05-authority-freeze/done.json`

Blocked conditions:

- `PLAN.md` scope is unclear, contradictory, or changed mid-run
- in-scope authored surfaces cannot be frozen cleanly
- the parent identifies mandatory files outside the currently authorized M44 surface

Restart point if blocked:

- stop before editing the contract lane
- record the blocker in `task-m44-05-authority-freeze/blocked.json`
- restart from `task-m44-05-authority-freeze` after the parent resolves scope authority

### `task/m44-a1-contract-freeze` - parent only

Purpose:

- create and freeze the shared portability contract API before any consumer work begins

Owned files and directories:

- `spec-core/src/portability_contract.rs`
- `spec-core/src/lib.rs`

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/contract-freeze -b ws/spec-m44-contract-freeze feat/m40-plus
cargo test -p spec-core -- --color never
git status --short
```

Artifacts written:

- `contract-freeze.json`
- `validation/contract-freeze/spec-core-tests.txt`
- `task-m44-a1-contract-freeze/started.json`
- `task-m44-a1-contract-freeze/done.json`

Blocked conditions:

- the parent cannot stabilize helper/type names cleanly enough for downstream parallel work
- contract tests or `cargo test -p spec-core` fail in a way that indicates the API freeze is not ready
- the lane needs consumer rewiring to make the contract compile

Restart point if blocked:

- stop before worker launch
- record the blocker in `task-m44-a1-contract-freeze/blocked.json`
- restart from `task/m44-a1-contract-freeze` after the parent resolves the contract API shape locally

Acceptance:

- `spec-core/src/portability_contract.rs` exists.
- The module owns only:
  - seam-kind helpers
  - shared authored-surface rule helpers
  - backend-marker classification helpers
  - stable wording helpers only where consumers would otherwise duplicate text
- The module does not own:
  - file IO
  - CLI rendering
  - artifact loading
  - cargo execution
  - proof evaluation
- `spec-core/src/lib.rs` exports the module.
- Direct contract tests exist in the owned module or in its immediate inline test surface.
- `contract-freeze.json` records:
  - `contract_freeze_commit`
  - frozen helper/type names
  - frozen lane ownership
  - prohibited drift after launch
- No consumer rewiring, docs edits, or CLI changes are mixed into this lane.

### `task-m44-15-worker-launch` - parent only

Purpose:

- launch worker lanes against one frozen contract commit and record the exact worker ownership contract

Owned files and artifacts:

- `M44_RUN_ROOT/tasks.json`
- `M44_RUN_ROOT/queue.json`
- `M44_RUN_ROOT/session-log.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-15-worker-launch/**`

Required commands:

```bash
git rev-parse --short ws/spec-m44-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/validator-markers -b ws/spec-m44-validator-markers ws/spec-m44-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/projection-gate -b ws/spec-m44-projection-gate ws/spec-m44-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/semantic-readside -b ws/spec-m44-semantic-readside ws/spec-m44-contract-freeze
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/docs -b ws/spec-m44-docs ws/spec-m44-contract-freeze
git worktree list
```

Artifacts written:

- updated `tasks.json`
- updated `queue.json`
- worker-launch note in `session-log.md`
- `task-m44-15-worker-launch/started.json`
- `task-m44-15-worker-launch/done.json`

Blocked conditions:

- `contract-freeze.json` is missing or does not identify a single frozen commit
- a worker worktree or branch cannot be created cleanly from the frozen commit
- lane ownership is still ambiguous enough that a worker would need to guess file scope

Restart point if blocked:

- stop before issuing worker prompts
- record the blocker in `task-m44-15-worker-launch/blocked.json`
- restart from `task-m44-15-worker-launch` after worktrees and lane contracts are clean

### `task/m44-b-validator-markers` - worker A

Purpose:

- move validator-owned seam restrictions behind the contract and keep raw marker collection honest

Owned files and directories:

- `spec-core/src/validator.rs`
- `spec-core/src/backend_execution.rs`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo run -p spec-cli -- validate examples/ecommerce/units --format json
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m44-b-validator-markers/started.json`
- `task-m44-b-validator-markers/status.json`
- `task-m44-b-validator-markers/done.json` or `blocked.json`

Acceptance:

- `validator.rs` becomes a contract caller, not a seam-policy author.
- `kind:data` and `kind:sum` still hard-fail illegal top-level authored shapes.
- top-level seam `body.rust` still hard-fails through the centralized contract path.
- `backend_execution.rs` still owns:
  - raw marker detection
  - backend execution digest material
- helper-only versus domain-lowering raw markers remain distinguishable.
- No edits spill into `escape_hatch.rs`, `portability.rs`, semantic/read-side files, docs, or run-state artifacts.

Blocked conditions:

- the frozen contract lacks a required validator or marker-classification symbol
- the lane needs to change `escape_hatch.rs`, `portability.rs`, semantic/read-side files, or docs to make progress
- local validation failure indicates projection- or read-side-owned drift rather than lane A work

Restart point if blocked:

- stop in `ws/spec-m44-validator-markers`
- hand the blocker to the parent with the failing command and missing symbol
- restart from `task/m44-b-validator-markers` after the parent republishes a valid freeze or reassigns the cross-lane fix

### `task/m44-c-projection-gate` - worker B

Purpose:

- rewire portability projection and escape-hatch gate evaluation onto the frozen contract without turning either module into a second policy owner

Owned files and directories:

- `spec-core/src/escape_hatch.rs`
- `spec-core/src/portability.rs`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m44-c-projection-gate/started.json`
- `task-m44-c-projection-gate/status.json`
- `task-m44-c-projection-gate/done.json` or `blocked.json`

Acceptance:

- `portability.rs` composes truth from contract helpers instead of local seam-policy duplication.
- `escape_hatch.rs` still owns:
  - proof-surface derivation
  - gate open/closed evaluation
- `escape_hatch.rs` does not become a portability classifier.
- helper-only lowering still projects as backend-only but non-contaminating.
- domain lowering still contaminates portability claims.
- stale atom or missing molecule proof still opens the gate.
- No edits spill into validator/raw-marker files, semantic/read-side files, docs, or run-state artifacts.

Blocked conditions:

- the frozen contract lacks projection helpers needed to remove local seam-policy duplication
- the lane needs to edit semantic/read-side files or the contract module to preserve parity
- local status/export regressions indicate downstream rendering-only drift instead of projection/gate drift

Restart point if blocked:

- stop in `ws/spec-m44-projection-gate`
- hand the blocker to the parent with the failing command and affected projection surface
- restart from `task/m44-c-projection-gate` after the parent resolves the contract or downstream ownership issue

### `task/m44-d-semantic-readside` - worker C

Purpose:

- move semantic and read-side consumers onto the centralized contract meaning and add command-path parity coverage

Owned files and directories:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m44-d-semantic-readside/started.json`
- `task-m44-d-semantic-readside/status.json`
- `task-m44-d-semantic-readside/done.json` or `blocked.json`

Acceptance:

- `semantic_review.rs` consumes centralized marker and contamination meaning.
- `semantic_review.rs` does not keep its own shared-vs-backend-only classification table.
- passport, export, and CLI status/export surfaces agree on:
  - markers
  - contamination summary
  - proof coverage
  - gate state
- `spec-cli/tests/cli.rs` contains command-path regressions for read-side parity.
- If this lane needs a new projection helper or contract symbol, it stops and asks the parent. It does not edit `portability.rs` or `portability_contract.rs` on its own.

Blocked conditions:

- parity requires changes to `portability.rs` or `portability_contract.rs`
- command-path failures originate in validator/raw-marker or projection/gate ownership instead of read-side ownership
- the lane would need to touch docs or out-of-scope user-facing files to proceed

Restart point if blocked:

- stop in `ws/spec-m44-semantic-readside`
- hand the blocker to the parent with the exact parity mismatch and command output
- restart from `task/m44-d-semantic-readside` after the parent resolves upstream ownership or republishes the freeze

### `task/m44-e-docs` - worker D

Purpose:

- update written milestone truth after the contract API is frozen and without widening the milestone

Owned files and directories:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `semantic-families/README.md`
- `README.md` only if opened by the parent
- `CHANGELOG.md` only if opened by the parent

Required commands:

```bash
rg -n "M44|portability|shared-core|escape-hatch|second-language|schema" docs/ai_promotion_and_multilanguage_milestones_v0.1.md semantic-families/README.md README.md CHANGELOG.md
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m44-e-docs/started.json`
- `task-m44-e-docs/status.json`
- `task-m44-e-docs/done.json` or `blocked.json`

Acceptance:

- the roadmap doc says M44 centralized seam portability policy in `spec-core/src/portability_contract.rs`
- `semantic-families/README.md` stays aligned with the actual portability boundary
- docs do not imply:
  - new crate extraction
  - second-language execution
  - fresh family-choice work
  - schema churn by default
- `README.md` and `CHANGELOG.md` stay untouched unless the parent explicitly opens them
- no code files are changed in this lane

Blocked conditions:

- the landed code boundary is still moving enough that docs would be speculative
- the lane appears to require `README.md` or `CHANGELOG.md` without explicit parent approval
- a docs fix would need code edits or scope expansion to become truthful

Restart point if blocked:

- stop in `ws/spec-m44-docs`
- hand the blocker to the parent with the exact wording gap or blocked file
- restart from `task/m44-e-docs` after the parent confirms the code boundary or expands doc scope explicitly

### `task/m44-f-integration` - parent only

Purpose:

- merge worker outputs and repair narrow in-scope integration drift before the proof-wall gate runs

Owned files and directories:

- merged in-scope files from lanes A through D
- parent-owned run-state artifacts under `M44_RUN_ROOT/**`

Required commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/integration -b ws/spec-m44-integration ws/spec-m44-contract-freeze
git merge --no-ff ws/spec-m44-validator-markers
git merge --no-ff ws/spec-m44-projection-gate
git merge --no-ff ws/spec-m44-semantic-readside
git merge --no-ff ws/spec-m44-docs
git merge-base --is-ancestor "$(jq -r '.contract_freeze_commit' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract/contract-freeze.json)" HEAD
git diff --name-only 7a986a6..HEAD
git diff --stat 7a986a6..HEAD
git status --short
```

Artifacts written:

- `merge-log.md`
- `validation/merge/git-merge-status.txt`
- `validation/merge/final-name-only.diff`
- `validation/merge/final-stat.diff`
- `task-m44-f-integration/started.json`
- `task-m44-f-integration/status.json`
- `task-m44-f-integration/done.json` or `blocked.json`

Acceptance:

- merge order is preserved unless the parent records an explicit deviation in `merge-log.md`
- no silent contract API drift is introduced during integration
- parent repair is limited to:
  - straightforward merge mechanics
  - line-level conflict resolution
  - in-scope parity drift exposed by the merged lanes
  - proof-wall fixes inside the already frozen M44 authored surfaces
- parent repair is not allowed to introduce:
  - quiet contract API redesign
  - new authored surfaces
  - broadened milestone claims
  - opportunistic cleanup outside M44 scope
- the final code and docs still do not widen into:
  - new crate extraction
  - second-language execution
  - fresh family-choice work
  - schema churn by default
- `README.md` and `CHANGELOG.md` remain unchanged unless the parent recorded why they were needed

Blocked conditions:

- merge conflicts cannot be resolved without changing ownership or widening file scope
- the merged tree requires a contract API redesign rather than a narrow parity repair
- proof-wall failures require files outside the frozen M44 authored surfaces

Restart point if blocked:

- stop in `ws/spec-m44-integration`
- record the blocker in `task-m44-f-integration/blocked.json` and `merge-log.md`
- restart from `task-m44-40-merge-window` if worker relaunch is required, otherwise restart from `task/m44-f-integration` after the parent resolves the narrow in-scope drift

### `task-m44-40-merge-window` - parent only

Purpose:

- verify that worker handoffs are complete, merge-safe, and still within the frozen lane contract before integration begins

Owned files and artifacts:

- `M44_RUN_ROOT/merge-log.md`
- `M44_RUN_ROOT/tasks.json`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-40-merge-window/**`

Required commands:

```bash
git rev-parse --short ws/spec-m44-validator-markers
git rev-parse --short ws/spec-m44-projection-gate
git rev-parse --short ws/spec-m44-semantic-readside
git rev-parse --short ws/spec-m44-docs
git diff --name-only ws/spec-m44-contract-freeze..ws/spec-m44-validator-markers
git diff --name-only ws/spec-m44-contract-freeze..ws/spec-m44-projection-gate
git diff --name-only ws/spec-m44-contract-freeze..ws/spec-m44-semantic-readside
git diff --name-only ws/spec-m44-contract-freeze..ws/spec-m44-docs
```

Artifacts written:

- merge-window note in `merge-log.md`
- updated `tasks.json`
- `task-m44-40-merge-window/started.json`
- `task-m44-40-merge-window/done.json`

Blocked conditions:

- a worker lane changed files outside its frozen ownership
- a worker lane is incomplete and not explicitly blocked
- handoff summaries are missing enough detail that the parent cannot merge safely

Restart point if blocked:

- stop before `ws/spec-m44-integration` is created
- record the blocker in `task-m44-40-merge-window/blocked.json`
- restart from `task-m44-15-worker-launch` if a lane must be relaunched, otherwise restart from `task-m44-40-merge-window`

### `task-m44-50-proof-wall` - parent only

Purpose:

- run and record the full M44 proof wall on the integrated branch and treat any proof failure as a stop condition

Owned files and artifacts:

- `M44_RUN_ROOT/validation/proof-wall/**`
- `M44_RUN_ROOT/acceptance.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-50-proof-wall/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract/validation/proof-wall
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units --format json
cargo run -p spec-cli -- test examples/ecommerce/units
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Artifacts written:

- `validation/proof-wall/cargo-test.txt`
- `validation/proof-wall/spec-validate.json`
- `validation/proof-wall/spec-test.txt`
- `validation/proof-wall/spec-status.json`
- `validation/proof-wall/spec-export.json`
- `validation/proof-wall/family-recommend.json`
- `validation/proof-wall/family-corpus-decision.json`
- `validation/proof-wall/family-verify-decision-contract.json`
- proof-wall summary in `acceptance.md`
- `task-m44-50-proof-wall/started.json`
- `task-m44-50-proof-wall/done.json` or `blocked.json`

Blocked conditions:

- any proof-wall command fails
- read-side parity diverges across status, export, passport, or semantic review
- fixing the failure would require out-of-scope files or a broadened contract

Restart point if blocked:

- stop with the integrated branch intact
- record the blocker in `task-m44-50-proof-wall/blocked.json` and `acceptance.md`
- restart from `task/m44-f-integration` after the parent resolves the narrow in-scope proof drift

### `task-m44-60-closeout` - parent only

Purpose:

- close the run only after proof, scope, and acceptance all agree

Owned files and artifacts:

- `M44_RUN_ROOT/acceptance.md`
- `M44_RUN_ROOT/closeout.md`
- `M44_RUN_ROOT/validation/closeout/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m44-60-closeout/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m44_shared_core_portability_contract/validation/closeout
git status --short
git diff --name-only 7a986a6..HEAD
git diff --stat 7a986a6..HEAD
```

Artifacts written:

- `closeout.md`
- `validation/closeout/final-git-status.short.txt`
- final acceptance note in `acceptance.md`
- `task-m44-60-closeout/started.json`
- `task-m44-60-closeout/done.json` or `blocked.json`

Blocked conditions:

- final diff contains out-of-bounds files
- acceptance is still relying on unresolved blockers or undocumented scope deviations
- closeout would require retrospective scope justification instead of an already-clean run

Restart point if blocked:

- stop before declaring M44 complete
- record the blocker in `task-m44-60-closeout/blocked.json`
- restart from `task/m44-f-integration` if code drift caused the issue, otherwise restart from `task-m44-40-merge-window` if a lane must be relaunched

## Scope-Boundary Checks

The parent must prove that the final diff stayed inside the frozen M44 authored surfaces.

Required checks:

- capture baseline diff surfaces during `task-m44-00-baseline`
- capture per-lane name-only diffs during `task-m44-40-merge-window`
- capture final integrated name-only and stat diffs during `task/m44-f-integration`
- re-check final name-only and stat diffs during `task-m44-60-closeout`

Required commands:

```bash
git diff --name-only 7a986a6..HEAD
git diff --stat 7a986a6..HEAD
```

Boundary rule:

- every changed file in the final diff must be one of:
  - a frozen M44 authored surface
  - a derived proof artifact changed only by proof commands
  - a parent-owned `.runs/**` execution artifact

Blocked rule:

- any out-of-bounds authored diff is a blocked condition, not a creative follow-up
- do not silently absorb unrelated cleanup, opportunistic refactors, or broader roadmap edits into the integrated branch
- if a required fix truly needs a new authored surface, stop and escalate scope instead of normalizing it in closeout

## Conflict-Control Rules

M44 has real overlap risk because multiple lanes consume the same new contract but should not co-own it.

Primary conflict flags:

- lane A and lane B both depend on the frozen contract helper names
- lane B and lane C both depend on the same projection meaning
- lane C can expose parity gaps that look like contract gaps but are really projection gaps

Resolution rules:

- only the parent may change `portability_contract.rs` after the worker-launch gate opens
- only lane B may change `portability.rs`
- only lane C may change `spec-cli/src/commands.rs` or `spec-cli/tests/cli.rs`
- if lane C needs a `portability.rs` change, it files a blocker to the parent rather than editing lane B's surface
- if lane A or B believes the frozen contract API is wrong, the parent either:
  - reopens the contract-freeze gate and republishes the freeze, or
  - rejects the requested drift and keeps the original call shape
- once the parent starts `ws/spec-m44-integration`, workers stop rebasing and hand off only summaries plus commit pointers

## Context-Control Rules

### Parent prompt rules

- Every worker prompt must include only:
  - the relevant `PLAN.md` excerpts
  - the lane's owned files
  - the frozen contract API excerpt from `contract-freeze.json`
  - the lane's required commands
  - the lane's acceptance criteria
  - the hard guards that matter for that lane
- Do not paste the full repo state into every worker prompt.
- Do not give any worker permission to edit `ORCH_PLAN.md`, `.runs/**`, or another lane's files.
- Record every scope exception, frozen-API clarification, or merge-order deviation in `session-log.md` and `merge-log.md`.

### Worker prompt rules

- Work only in the assigned `spec-m44` worktree.
- Treat `PLAN.md` plus the frozen contract excerpt as authoritative.
- Keep edits inside the assigned files and their inline tests.
- Run only the lane-local commands unless the parent explicitly asks for broader proof.
- If a required fix crosses lane ownership, stop and report:
  - the blocked file
  - the missing symbol or contract gap
  - the exact command that exposed it
- Return concise handoff notes:
  - changed files
  - commands run
  - remaining risk
  - whether the lane is safe to merge

## Tests And Acceptance

### Required proof wall

The parent integration lane must run this exact wall before closeout:

```bash
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units --format json
cargo run -p spec-cli -- test examples/ecommerce/units
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

### Lane-local regression expectations

- contract-freeze lane proves direct `portability_contract.rs` behavior
- validator/raw-marker lane proves seam validator strictness and raw marker preservation
- projection/gate lane proves contamination split and escape-hatch gate behavior
- semantic/read-side lane proves passport, export, status, and semantic parity plus CLI command-path coverage
- docs lane proves wording alignment only; it does not substitute for runtime proof

### Milestone acceptance checklist

M44 is complete only if all of the following are true:

1. `spec-core/src/portability_contract.rs` exists and is the sole portability-policy owner.
2. `validator.rs` no longer owns duplicated seam-policy rules inline.
3. `backend_execution.rs` still owns raw marker detection and backend digest material.
4. `escape_hatch.rs` still owns proof-surface and gate evaluation.
5. `portability.rs` composes portability truth from the contract instead of local policy duplication.
6. `semantic_review.rs`, passport projection, export projection, and CLI status/export rendering agree on the same portability truth.
7. direct and command-path regressions exist for the M44 extraction risks.
8. helper-only lowering is still backend-only but non-contaminating.
9. domain lowering still contaminates portability claims.
10. stale atom or missing molecule proof still opens the escape-hatch gate.
11. `cargo xtask family recommend --format json` stays aligned with the existing family stop-state.
12. `cargo xtask family corpus-decision --format json` stays aligned with the existing family stop-state.
13. `cargo xtask family verify-decision-contract --format json` stays aligned with the existing family stop-state.
14. the roadmap and maintainer-facing docs match the landed code boundary exactly.
15. the proof wall passes without reopening milestone scope.

## Assumptions

- The parent launches from the current baseline branch `feat/m40-plus` at `7a986a6`.
- The current M44 `PLAN.md` remains the only scope authority during execution.
- The repo can support disposable worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m44/`.
- Inline or colocated tests are acceptable for `spec-core` module regressions where no separate test file is already established.
- `README.md` and `CHANGELOG.md` are probably unnecessary for M44 and should stay untouched unless integration proves otherwise.
- No schema bump is required to land the centralized portability contract.
- The `xtask family` commands are proof-wall parity checks only in this milestone, not a reason to reopen family semantics.

## Completion Summary

This orchestration plan keeps the only truly dangerous decisions in the parent lane:

- freeze the portability contract once
- let consumers parallelize against that freeze
- merge in dependency order
- rerun the entire proof wall locally

That gives M44 materially better parallelism than M43 without letting multiple workers co-own the same portability policy. The milestone stays bounded to one new module, one centralized contract, consumer rewiring, regression coverage, and doc parity.
