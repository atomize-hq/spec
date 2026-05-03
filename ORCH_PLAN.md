# M29 Orchestration Plan

Status: **execution contract, recovery refreeze**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Live branch: **`feat/corpus-expansion`**
Recovery seed: **`741a83e`**
Blocked checkpoint history: **`d10679a`**
Review base: **`main`**
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot`**
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot`**
Locked packet root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/function.arithmetic_leaf.monotone_up.v1`**
Last rewritten: **2026-05-02**

## Summary

- M29 remains one milestone: `M29 - Scoped Second-Language TypeScript Pilot`.
- Execute from the live branch `feat/corpus-expansion`, but do not continue from the blocked integration state. The active recovery loop is fixed:
  1. re-freeze from `741a83e`
  2. preserve `d10679a` as blocked history only
  3. relaunch `Lane A` and `Lane B` from the same frozen foundation SHA
  4. merge both into a fresh integration base
  5. freeze the packet contract
  6. launch `Lane C`
  7. merge packet truth and freeze the CI contract
  8. launch `Lane D`
  9. run the final local proof loop from merged state
  10. push the integration candidate
  11. observe CI on the exact pushed SHA
  12. close with exactly one verdict: `EXPAND`, `NARROW`, or `STOP`
- Parent is the sole integrator, merger, freeze authority, relaunch authority, push authority, CI observer, and final verifier.
- Maximum worker concurrency is `2`, and only `Lane A` plus `Lane B` may run in parallel.
- `Lane C` waits for the post-foundation packet freeze. `Lane D` waits for the post-packet CI freeze.
- The implementation surface stays closed to the M29 surfaces already locked by `PLAN.md`. If honest completion needs wider files or repo-wide target-language plumbing, stop M29.

## Hard Guards

- `PLAN.md` is the only milestone authority. If any worker suggestion, stale run artifact, or branch-local state conflicts with `PLAN.md`, `PLAN.md` wins.
- `ORCH_PLAN.md` is parent-owned only.
- Parent remains the only actor allowed to:
  - create or recreate worktrees
  - freeze or refreeze any contract
  - merge branches
  - resolve cross-lane conflicts
  - invalidate stale lanes
  - push a branch
  - observe CI on the pushed SHA
  - issue the final verdict
- Workers may not merge, rebase, push, or reinterpret the milestone scope.
- Only these surfaces are in scope for M29 execution:
  - `spec-core/src/types.rs`
  - `spec-core/src/validator.rs`
  - `spec-core/src/generator.rs`
  - `spec-core/src/semantic_review.rs`
  - `xtask/src/lib.rs`
  - `xtask/src/family/harness.rs`
  - `xtask/src/family/layout.rs`
  - `xtask/src/family/scaffold.rs`
  - `xtask/src/family/smoke.rs`
  - `xtask/src/family/prove.rs`
  - `xtask/src/family/certify.rs`
  - `xtask/src/family/report.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
  - `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
  - `.github/workflows/ci.yml`
  - `PLAN.md`
- For this M29 run, `PLAN.md` is treated as read-only authority. Closeout goes into run-state unless a separate explicit authorization reopens plan editing.
- The packet root is locked to `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
- `semantic-families-typescript/` is forbidden.
- `kind:function` uses additive authored bodies only:

```yaml
body:
  rust: |
    { ... }
  typescript: |
    { ... }
```

- TypeScript pilot truth is packet-local only. M29 must not add:
  - repo-wide `spec build --target-language typescript`
  - repo-wide `spec test --target-language typescript`
  - TypeScript support for `kind:data`
  - TypeScript support for `kind:sum`
  - passport redesign
  - `spec status` redesign
  - `spec export` redesign
  - second family rollout
  - second target-language rollout
- Rust remains the default when `--target-language` is omitted.
- The public family command surface remains:
  - `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1`
  - `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
  - `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1`
  - `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
  - `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1`
  - `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
- Stop immediately if any of these become true:
  - M29 needs files outside the closed implementation surface
  - TypeScript packet truth cannot stay under the locked packet root
  - the pilot requires repo-wide target-language CLI support
  - Rust-default behavior regresses
  - the final branch push cannot trigger automatic CI for the pilot

## Locked Recovery Basis

- `feat/corpus-expansion` is the live user branch and remains the human-facing baseline for this run.
- `d10679a` is preserved as blocked checkpoint history only. It is evidence of the failed first foundation merge, not an active execution base.
- `741a83e` is the recovery refreeze seed. Every relaunched foundation branch must fork from the same `741a83e`-based integration seed.
- The parent must record both values in run-state before any worker starts:
  - `recovery_seed_sha = 741a83e`
  - `blocked_checkpoint_sha = d10679a`
- The first active integration branch for the recovery is a fresh `ws/m29-int` rooted from `741a83e`, not a continuation of the blocked checkpoint.
- No packet or CI work may start until the parent has:
  - relaunched `Lane A` and `Lane B`
  - merged them into the fresh integration base
  - written the post-foundation `packet-contract-freeze.json`

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot`

All canonical parent-owned run-state files and per-task sentinels live under `RUN_ROOT`.

Canonical parent-owned files:

- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty status summary
  - `741a83e` recovery seed confirmation
  - `d10679a` blocked checkpoint confirmation
  - overlap check between live dirtiness and lane-owned surfaces
- `tasks.json`
  - ordered task ledger
  - `task_id`
  - `owner`
  - `branch`
  - `worktree`
  - `depends_on`
  - `owned_paths`
  - `status`
  - `restart_count`
- `session-log.md`
  - append-only parent timeline
  - freeze creation
  - worker launch
  - merge results
  - relaunch decisions
  - push and CI observation notes
- `docs-contract.json`
  - authority path
  - live milestone
  - worker model
  - concurrency cap
  - lane ownership map
- `recovery-freeze.json`
  - fresh integration branch seed SHA
  - frozen record that `d10679a` is history only
  - fresh worktree and branch map
- `foundation-freeze.json`
  - exact `ws/m29-int` SHA used to fork `Lane A` and `Lane B`
  - owned paths
  - forbidden paths
  - exact lane acceptance commands
- `packet-contract-freeze.json`
  - exact post-foundation integration SHA
  - frozen `body.typescript` authoring contract
  - frozen generator and semantic-review expectations
  - frozen packet layout and artifact-path rules
  - exact `Lane C` acceptance commands
- `ci-freeze.json`
  - exact post-packet integration SHA
  - exact workflow commands
  - exact `Lane D` branch point
- `merge-log.md`
  - ordered merge history
  - merge SHAs
  - mechanical conflict notes
  - stale-lane invalidations
- `proof-log.json`
  - actual final local proof commands
  - exit code per command
  - artifact locations
  - packet dirtiness checks
- `push-record.json`
  - remote
  - pushed branch
  - pushed SHA
  - push timestamp
- `ci-observation.json`
  - workflow name
  - run id or URL
  - observed branch
  - observed SHA
  - Rust lane result
  - TypeScript lane result
- `blocked.json`
  - blocking task
  - blocking evidence
  - required next decision
- `closeout.md`
  - what stayed shared
  - what leaked
  - packet contract honesty
  - verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-01-lock-contract/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-02-refreeze-foundation/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-a-lane-spec-core/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-b-lane-xtask/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-03-freeze-packet/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-c-lane-packet/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-04-freeze-ci/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-d-lane-ci/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-05-final-proof/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-06-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29_typescript_pilot/task-m29-07-closeout/`

Each sentinel directory contains:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Worktree Plan

Branches and worktrees:

- integration
  - branch: `ws/m29-int`
  - worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/int`
- `Lane A`
  - branch: `ws/m29-spec-core`
  - worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/spec-core`
- `Lane B`
  - branch: `ws/m29-xtask`
  - worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/xtask`
- `Lane C`
  - branch: `ws/m29-packet`
  - worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/packet`
- `Lane D`
  - branch: `ws/m29-ci`
  - worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/ci`

Creation and restart rules:

- If a named M29 recovery worktree or branch already exists, the parent removes it first and records that teardown in `session-log.md` before recreating the frozen replacement.
- `ws/m29-int` is created from `741a83e`.
- `ws/m29-spec-core` and `ws/m29-xtask` are both created from the same recorded `foundation-freeze.json` SHA.
- `ws/m29-packet` is created only after `packet-contract-freeze.json` exists.
- `ws/m29-ci` is created only after `ci-freeze.json` exists.
- A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent never hand-forwards a stale worker branch.
- The parent does not integrate on the live checkout. All merges and proof happen in `ws/m29-int`.

## Command Contract

### Lane A command contract

Owned files:

- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`

Required acceptance commands:

```bash
cargo test -p spec-core --lib body_typescript_ -- --color never
cargo test -p spec-core --lib validator_typescript_ -- --color never
cargo test -p spec-core --lib generator_typescript_ -- --color never
cargo test -p spec-core --lib monotone_up_typescript_ -- --color never
cargo test -p spec-core --lib semantic_review_typescript_ -- --color never
cargo test -p spec-core --lib -- --color never
```

Lane A must deliver:

- additive `body.typescript` support for `kind:function`
- TypeScript pilot validation that reads `body.typescript`
- TypeScript lowering limited to the locked pilot family
- TypeScript semantic-review wedge limited to `function.arithmetic_leaf.monotone_up.v1`
- unchanged Rust-default behavior

### Lane B command contract

Owned files:

- `xtask/src/lib.rs`
- `xtask/src/family/harness.rs`
- `xtask/src/family/layout.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/family/smoke.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/report.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`

Required acceptance commands:

```bash
cargo test -p xtask target_language_ -- --color never
cargo test -p xtask typescript_layout_ -- --color never
cargo test -p xtask scaffold_typescript_ -- --color never
cargo test -p xtask smoke_typescript_ -- --color never
cargo test -p xtask prove_typescript_ -- --color never
cargo test -p xtask certify_typescript_ -- --color never
cargo test -p xtask artifact_path_ -- --color never
cargo test -p xtask report_target_language_ -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
```

Lane B must deliver:

- `--target-language rust|typescript` on `family smoke/prove/certify`
- Rust-default behavior when flag is omitted
- locked packet-root handling under `semantic-families/`
- target-partitioned artifact paths
- TypeScript scaffold and layout truth under the locked packet root

### Lane C command contract

Owned files:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Required acceptance commands:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
rg --files semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/fixtures
rg -n "typescript:" semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures
```

Lane C must deliver:

- additive `body.typescript` truth in the pilot packet only
- committed TypeScript runtime roots for all four buckets
- no checked-in generated output
- no checked-in `node_modules`
- no packet truth outside the locked packet root

### Lane D command contract

Owned files:

- `.github/workflows/ci.yml`

Workflow commands that must exist after merge:

```bash
cargo test -p spec-core --lib body_typescript_ -- --color never
cargo test -p xtask target_language_ -- --color never
cargo test -p xtask typescript_ -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Lane D must deliver:

- automatic CI execution on branch push
- existing Rust lane preserved
- Node setup before the TypeScript pilot lane
- exact pushed-SHA observability

## Task Graph

```text
task/m29-00-baseline
  -> task/m29-01-lock-contract
      -> task/m29-02-refreeze-foundation
          -> task/m29-a-lane-spec-core
          -> task/m29-b-lane-xtask
              -> task/m29-03-freeze-packet
                  -> task/m29-c-lane-packet
                      -> task/m29-04-freeze-ci
                          -> task/m29-d-lane-ci
                              -> task/m29-05-final-proof
                                  -> task/m29-06-push-observe
                                      -> task/m29-07-closeout
```

Execution meaning:

1. Parent captures baseline and recovery facts from the live branch.
2. Parent stabilizes the orchestration contract.
3. Parent refreezes the foundation from `741a83e`.
4. `Lane A` and `Lane B` run in parallel, and only those two lanes.
5. Parent merges both, freezes the packet contract, then and only then launches `Lane C`.
6. Parent merges packet truth, freezes the CI contract, then and only then launches `Lane D`.
7. Parent merges CI truth, runs the final proof loop, pushes, observes CI on the exact pushed SHA, and closes the milestone.

## Workstream Plan

### WS-0 Baseline And Recovery Refreeze - parent only

#### `task/m29-00-baseline`

Required parent actions:

1. Capture live branch and dirty state from `feat/corpus-expansion`.
2. Record whether any live dirty files overlap with the M29-owned surfaces.
3. Record `741a83e` as the recovery seed and `d10679a` as blocked history.
4. Stop if unresolved dirty overlap exists inside any lane-owned path.

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
git rev-parse 741a83e
git rev-parse d10679a
```

Acceptance:

- `baseline.json` exists
- `recovery_seed_sha` and `blocked_checkpoint_sha` are recorded
- unresolved dirty overlap is either empty or explicitly blocked

#### `task/m29-01-lock-contract`

Required parent actions:

1. Stabilize `ORCH_PLAN.md`.
2. Write `docs-contract.json`.
3. Write `tasks.json`.
4. Record worker model, concurrency cap, lane ownership, and blocker rules.

Acceptance:

- no worker launches before `docs-contract.json`
- all worker prompts reference the same frozen contract snapshot

#### `task/m29-02-refreeze-foundation`

Required parent actions:

1. Remove any pre-existing `ws/m29-int`, `ws/m29-spec-core`, and `ws/m29-xtask` worktrees or branches that still point at blocked or stale recovery state.
2. Create `ws/m29-int` from `741a83e`.
3. Create `ws/m29-spec-core` and `ws/m29-xtask` from the same `ws/m29-int` SHA.
4. Write `recovery-freeze.json`.
5. Write `foundation-freeze.json`.

Acceptance:

- both foundation lanes fork from the same recorded SHA
- `d10679a` is preserved in run-state as blocked history only
- both foundation worker prompts include exact owned paths and commands

### WS-1 Foundation Lanes - parallel, concurrency cap 2

#### `task/m29-a-lane-spec-core` - worker

Mission:

- repair the shared `spec-core` body contract so the TypeScript pilot reads `body.typescript`
- keep all support bounded to the locked pilot family
- preserve Rust-default behavior

Acceptance:

- all `Lane A` commands pass, or exact narrower replacements are documented and pass
- no files outside `Lane A` ownership change

#### `task/m29-b-lane-xtask` - worker

Mission:

- repair target-aware family plumbing under the locked packet root
- keep Rust-default behavior on omitted `--target-language`
- freeze layout, scaffold, smoke/prove/certify, artifact path, and report rules

Acceptance:

- all `Lane B` commands pass, or exact narrower replacements are documented and pass
- no files outside `Lane B` ownership change

### WS-2 Parent Merge And Packet Freeze - parent only

#### `task/m29-03-freeze-packet`

Strict merge order:

1. merge `ws/m29-spec-core` into `ws/m29-int`
2. verify `Lane A`
3. merge `ws/m29-xtask` into `ws/m29-int`
4. verify `Lane B`
5. write `packet-contract-freeze.json`

Parent may resolve only:

- straightforward import or module ordering
- adjacent test additions
- mechanical context drift inside already-approved owned files

Parent must bounce back to lane owners for:

- packet-root disagreements
- body-selection disagreements
- artifact-path disagreements
- target-language command-surface disagreements
- any conflict that changes meaning rather than syntax

Acceptance:

- both foundation lanes are merged into a fresh recovery integration branch
- `packet-contract-freeze.json` exists
- `Lane C` has exact frozen packet expectations and exact commands

### WS-3 Packet Lane - serialized after packet freeze

#### `task/m29-c-lane-packet` - worker

Mission:

- land the committed TypeScript pilot packet under the locked packet root
- add all four TypeScript bucket runtime roots
- consume the post-foundation frozen contract literally

Acceptance:

- all `Lane C` commands pass
- no files outside the locked packet root change
- no generated TypeScript output or `node_modules` are checked in

### WS-4 Parent Merge And CI Freeze - parent only

#### `task/m29-04-freeze-ci`

Required parent actions:

1. merge `ws/m29-packet` into `ws/m29-int`
2. rerun the packet acceptance commands from merged state
3. write `ci-freeze.json`
4. create `ws/m29-ci` from the recorded post-packet SHA

Acceptance:

- `ci-freeze.json` contains the exact workflow command list
- `Lane D` starts only from the recorded post-packet freeze SHA

### WS-5 CI Lane - serialized after CI freeze

#### `task/m29-d-lane-ci` - worker

Mission:

- add the automatic CI lane for the TypeScript pilot
- preserve existing Rust CI behavior
- encode the exact frozen command list in `.github/workflows/ci.yml`

Acceptance:

- `.github/workflows/ci.yml` is the only changed file
- workflow uses Node setup before the TypeScript pilot commands
- workflow runs the frozen Rust and TypeScript command list

### WS-6 Final Proof, Push, Observation, Closeout - parent only

#### `task/m29-05-final-proof`

Required final local proof loop in `ws/m29-int`:

```bash
cargo fmt --all --check
cargo test -p spec-core --lib body_typescript_ -- --color never
cargo test -p spec-core --lib validator_typescript_ -- --color never
cargo test -p spec-core --lib generator_typescript_ -- --color never
cargo test -p spec-core --lib monotone_up_typescript_ -- --color never
cargo test -p spec-core --lib semantic_review_typescript_ -- --color never
cargo test -p spec-core --lib -- --color never
cargo test -p xtask target_language_ -- --color never
cargo test -p xtask typescript_layout_ -- --color never
cargo test -p xtask scaffold_typescript_ -- --color never
cargo test -p xtask smoke_typescript_ -- --color never
cargo test -p xtask prove_typescript_ -- --color never
cargo test -p xtask certify_typescript_ -- --color never
cargo test -p xtask artifact_path_ -- --color never
cargo test -p xtask report_target_language_ -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Rules:

- if workers introduced exact narrower selectors instead of the prefix selectors above, the parent substitutes the documented exact selectors and records them in `proof-log.json`
- every actual command and exit code must be recorded
- parent must confirm the committed packet tree is clean after TypeScript prove and certify
- parent must confirm Rust artifact paths remain stable and TypeScript artifacts land under the target partition

#### `task/m29-06-push-observe`

Required parent actions:

1. merge `ws/m29-ci` into `ws/m29-int`
2. push `ws/m29-int` or a designated final candidate branch
3. record branch name, remote, and pushed SHA in `push-record.json`
4. observe the CI run triggered by that exact pushed SHA
5. record workflow run id or URL and per-lane results in `ci-observation.json`

Acceptance:

- push succeeded
- the pushed branch triggered the CI workflow
- the observed workflow run references the exact pushed SHA
- Rust lane is green
- TypeScript pilot lane is green

#### `task/m29-07-closeout`

Closeout must write `closeout.md` and answer exactly:

1. what stayed truly shared between Rust and TypeScript
2. what portability seams leaked
3. whether the packet contract stayed honest across both targets
4. whether the verdict is `EXPAND`, `NARROW`, or `STOP`

Verdict rules:

- `EXPAND` only if:
  - final local proof is green
  - pushed-SHA CI is green
  - Rust path stability held
  - TypeScript stayed packet-local and additive
  - no closed-surface breach was required
- `NARROW` only if:
  - final local proof is green
  - pushed-SHA CI is green
  - the pilot worked, but the closeout shows that expansion would first require tighter containment or follow-on repair
- `STOP` if:
  - any final proof command fails
  - pushed-SHA CI fails or does not run on the exact pushed SHA
  - the pilot leaked beyond the locked implementation surface
  - Rust-default behavior was not preserved

## Worker Return Contract

Every worker handoff must contain only:

- changed files
- commands run
- exit code for every command
- blockers
- unresolved assumptions
- skipped acceptance commands, if any

If a command was skipped, the worker must also report:

- the exact skipped command
- why it was skipped
- the exact narrower substitute, if any
- whether the skip blocks merge

Workers do not return:

- new milestone scope
- new authority text
- merge decisions
- push decisions
- narrative status reports beyond the contract above

## Conflict Policy And Stale-Lane Invalidation

- Parent does not invent a hybrid contract during merge.
- If a worker result conflicts with a frozen contract, parent must do exactly one of:
  - reject the lane and relaunch from the latest freeze
  - apply the already-frozen authority literally if the lane drifted
  - block the run if the conflict exposes real authority drift
- Stale-lane invalidation is automatic when:
  - `body.typescript` contract changes after a lane was forked
  - generator or semantic-review shape changes after `Lane C` was forked
  - packet layout or artifact-path rules change after `Lane C` was forked
  - workflow command list changes after `Lane D` was forked
- Stale lanes are discarded and recreated. The parent does not hand-patch them.

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen M29 scope
- they cannot preserve Rust-default behavior
- they need repo-wide target-language CLI support
- they cannot satisfy acceptance commands with concrete evidence

Worker blocker response:

- stop work
- write the sentinel `blocked` terminal state
- report the smallest blocking fact, not a speculative redesign

Parent blocker response:

- write `blocked.json`
- stop downstream launches
- stop push and closeout
- do not report partial green success

## Context-Control Rules

- Worker prompts must include only:
  - the relevant `PLAN.md` excerpt
  - the relevant `ORCH_PLAN.md` excerpt
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - handoff contract
  - freeze SHA
- Parent-owned run-state under `.runs/m29_typescript_pilot` is the orchestration source of truth. Worker chat is not.
- Parent reviews only:
  - changed files
  - command results
  - blockers
  - unresolved assumptions
- Parent should not keep idle workers attached after a merge or relaunch decision.

## Acceptance Gates

### Gate 0: baseline and recovery gate

Required:

- `baseline.json`
- recorded `741a83e`
- recorded `d10679a`
- no unresolved dirty overlap inside lane-owned paths

### Gate 1: foundation launch gate

Required:

- `docs-contract.json`
- `recovery-freeze.json`
- `foundation-freeze.json`
- `Lane A` and `Lane B` forked from the same recorded SHA
- both worker prompts include exact commands and owned paths

### Gate 2: packet launch gate

Required:

- `Lane A` merged and verified
- `Lane B` merged and verified
- `packet-contract-freeze.json`
- packet authorship can proceed without guessing body, layout, or artifact rules

### Gate 3: CI launch gate

Required:

- `Lane C` merged and verified
- `ci-freeze.json`
- workflow command list is frozen and explicit

### Gate 4: final local proof gate

Required:

- `Lane D` merged
- the full local proof loop passes from `ws/m29-int`
- `proof-log.json` records every actual command and exit code
- packet tree remains clean after TypeScript prove and certify

### Gate 5: pushed-SHA CI observation gate

Required:

- push succeeded
- `push-record.json` records the pushed branch and SHA
- the branch CI run is observed on that exact pushed SHA
- `ci-observation.json` records Rust lane green and TypeScript lane green

### Gate 6: closeout gate

Required:

- `closeout.md`
- exact verdict: `EXPAND`, `NARROW`, or `STOP`
- verdict justified by the Gate 4 and Gate 5 results

## Completion Criteria

M29 orchestration is complete only when all are true:

1. recovery restarted from `741a83e`
2. `d10679a` is preserved as blocked history only
3. only `Lane A` and `Lane B` ran in parallel
4. parent remained sole integrator, sole freeze authority, sole push authority, and sole final verifier
5. `Lane C` started only after the post-foundation packet freeze
6. `Lane D` started only after the post-packet CI freeze
7. Rust stayed green and path-stable
8. TypeScript completed smoke, prove, and certify under the locked packet-local pilot
9. the final branch push triggered CI on the exact observed SHA
10. closeout ended with exactly one verdict: `EXPAND`, `NARROW`, or `STOP`
