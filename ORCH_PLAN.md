# M29R Orchestration Plan

Status: **execution contract, authoritative**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Live branch: **`feat/corpus-expansion`**
Review base: **`main`**
Recovery seed: **`741a83e`**
Blocked checkpoint history: **`d10679a`**
Last rewritten: **2026-05-03**
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery`**
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery`**
Locked packet root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/function.arithmetic_leaf.monotone_up.v1`**

## Summary

- This run is for **M29R**, not M29. `PLAN.md` is the only milestone authority.
- The parent agent rewrites and freezes orchestration first, then performs the parent-owned Stage A1 of `Lane A`: the Step 2 shared schema/model repair on the integration worktree before any worker starts.
- After Step 2 lands, the parent writes `contract-freeze.json` and launches exactly two parallel worker lanes from that same frozen SHA:
  - `Lane A` = Step 2 -> Step 3 thread, with Stage A1 parent-owned and Stage A2 worker-owned shared consumer re-anchor in `spec-core`
  - `Lane B` = Step 4 read-side proof and harness alignment in `spec-cli/tests/` and `xtask/`
- `Lane C` owns Step 5 packet replay and waits for both `Lane A` and `Lane B` to merge.
- `Lane D` owns the CI workflow update portion of Step 6 and waits for `Lane C`.
- The parent remains the sole integrator, sole freeze authority, sole stale-lane invalidator, sole push authority, and sole final verifier.
- Maximum worker concurrency is `2`. Only `Lane A` and `Lane B` may run in parallel.
- `.test.spec` remains Rust-only throughout the run. No worker may widen that surface.
- The packet root stays `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`. No `semantic-families-typescript/` tree is allowed.
- Rust remains the default omitted-flag lane. No repo-wide `spec build/test --target-language typescript` work is allowed.
- Parent-owned run-state under `RUN_ROOT` is the orchestration source of truth. Worker chat is not.

## Hard Guards

- `PLAN.md` wins over this document, stale run artifacts, worker summaries, and branch-local drift.
- `PLAN.md` is read-only authority during execution. Closeout lives in run-state, not in-plan edits.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen in `ws/m29r-int`.
- The closed implementation surface for this run is:
  - `spec-core/src/schema/unit.spec.json`
  - `spec-core/src/schema/test.spec.json`
  - `spec-core/src/types.rs`
  - `spec-core/src/validator.rs`
  - `spec-core/src/generator.rs`
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/lib.rs` only if required for export or test plumbing
  - `spec-cli/tests/m14_regressions.rs`
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
- Allowed mechanical compile-fix spillover is tightly bounded to fallout caused by widening `spec-core::types::Body` with `typescript: Option<String>`.
  - Allowed only when the edit is mechanical, typically `typescript: None`.
  - Allowed only when no runtime behavior or milestone scope changes in that file.
  - Likely spillover sites are limited to:
    - `spec-core/src/{export,escape_hatch,generator,graph,molecule_evidence,normalizer,passport,plan,semantic_review,validator}.rs`
    - `spec-cli/src/commands.rs`
  - Any wider spillover blocks the run until the parent rewrites authority.
- `.test.spec` must remain Rust-only. Widening `Body` does not authorize widening `spec-core/src/schema/test.spec.json` or molecule-test authoring.
- `kind:data` and `kind:sum` must continue to reject top-level `body.typescript`.
- The critical path is:
  - shared schema/model repair
  - shared consumer re-anchor
  - read-side proof and harness alignment
  - packet replay
  - CI lane update
  - final merged-state verification
- No new packet root is allowed.
- No repo-wide `spec build/test --target-language typescript` is allowed.
- The required merged-state verification commands are exactly the commands in `PLAN.md` under `## Explicit Verification Sequence`. The parent may run extra local diagnostics during development, but they do not replace that floor.
- Stop immediately if any lane requires:
  - widening `.test.spec`
  - widening `kind:data` or `kind:sum` for TypeScript bodies
  - repo-wide target-language CLI support
  - a new packet root
  - semantic changes outside the closed surface

## Worktree Layout

Canonical worktrees:

- integration
  - branch: `ws/m29r-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery/int`
- `Lane A`
  - branch: `ws/m29r-lane-a`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery/lane-a`
- `Lane B`
  - branch: `ws/m29r-lane-b`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery/lane-b`
- `Lane C`
  - branch: `ws/m29r-lane-c`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery/lane-c`
- `Lane D`
  - branch: `ws/m29r-lane-d`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery/lane-d`

Creation rules:

- The parent captures baseline state from the live checkout on `feat/corpus-expansion`, then creates `ws/m29r-int` from `741a83e`.
- Step 2 is executed by the parent directly on `ws/m29r-int`.
- `Lane A` and `Lane B` are both forked from the exact post-Step-2 SHA recorded in `contract-freeze.json`.
- `Lane C` is forked only after `Lane A` and `Lane B` are merged and `packet-freeze.json` exists.
- `Lane D` is forked only after `Lane C` is merged and `ci-freeze.json` exists.
- Existing stale M29 worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29-typescript-pilot/*` and existing `ws/m29-*` branches are blocked-history surfaces only for M29R.
  - They must not be reused as execution bases, merge bases, or prompt authority for this run.
  - If the parent tears them down, record that teardown in `session-log.md`.
  - If the parent leaves them in place, they are ignored and never reused.
- If any named worktree or branch already exists and points at stale or blocked state, the parent removes and recreates it before reuse and records that teardown in `session-log.md`.
- A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent never hand-forwards a stale worker branch.

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m29r-body-contract-recovery`

Canonical parent-owned files:

- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty-state summary
  - overlap check against M29R-owned surfaces
  - `recovery_seed_sha`
  - `blocked_checkpoint_sha`
- `authority-freeze.json`
  - authority paths
  - live milestone id `M29R`
  - worker model
  - concurrency cap
  - lane map
  - hard guards
- `tasks.json`
  - ordered task ledger
  - `task_id`
  - `owner`
  - `branch`
  - `worktree`
  - `depends_on`
  - `owned_paths`
  - `status`
- `session-log.md`
  - append-only parent timeline
  - worktree creation
  - freeze creation
  - worker launch
  - merge results
  - relaunch decisions
  - push and CI observation notes
- `contract-freeze.json`
  - exact post-Step-2 commit
  - Step 2 changed paths
  - explicit note that `.test.spec` remains Rust-only
  - exact worker prompt basis for `Lane A` and `Lane B`
  - exact `Lane A` and `Lane B` acceptance commands
- `packet-freeze.json`
  - exact post-`Lane A`/`Lane B` merge commit
  - locked packet root
  - locked bucket set:
    - `aligned`
    - `drift`
    - `under_specified`
    - `unsupported_near_miss`
  - exact `Lane C` prove/certify acceptance commands
- `ci-freeze.json`
  - exact post-`Lane C` merge commit
  - exact workflow command list to encode in `.github/workflows/ci.yml`
  - exact `Lane D` prompt basis
- `merge-log.md`
  - ordered merge history
  - merge SHAs
  - conflict notes
  - stale-lane invalidations
- `proof-log.json`
  - actual final merged-state verification commands
  - exit code per command
  - execution order
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
  - workspace lane result
  - monotone-up pilot lane result
- `blocked.json`
  - blocking task
  - blocking evidence
  - required next decision
- `closeout.md`
  - contract summary
  - containment summary
  - Rust-default summary
  - compatibility baggage summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-01-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-02-step2-contract-repair/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-a-step3-consumers/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-b-step4-readside/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-03-freeze-packet/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-c-step5-packet/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-04-freeze-ci/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-d-step6-ci/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-05-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-06-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m29r_additive_body_contract_recovery/task-m29r-07-closeout/`

Each sentinel directory contains:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m29r-00-baseline
  -> task/m29r-01-authority-freeze
      -> task/m29r-02-step2-contract-repair
          -> task/m29r-a-step3-consumers
          -> task/m29r-b-step4-readside
              -> task/m29r-03-freeze-packet
                  -> task/m29r-c-step5-packet
                      -> task/m29r-04-freeze-ci
                          -> task/m29r-d-step6-ci
                              -> task/m29r-05-final-verify
                                  -> task/m29r-06-push-observe
                                      -> task/m29r-07-closeout
```

Execution meaning:

1. Parent captures live branch and recovery facts.
2. Parent rewrites and freezes orchestration authority.
3. Parent lands Step 2 shared schema/model repair and records the post-Step-2 contract freeze.
4. `Lane A` Stage A2 and `Lane B` run in parallel from that same frozen SHA.
5. Parent merges both, writes `packet-freeze.json`, then launches `Lane C`.
6. Parent merges `Lane C`, writes `ci-freeze.json`, then launches `Lane D`.
7. Parent merges `Lane D`, runs the exact merged-state verification sequence from `PLAN.md`, pushes, observes CI on the exact pushed SHA, and closes the milestone with one verdict.

## Command Contracts

### Parent contract: baseline and Lane A Stage A1 freeze

Owned paths:

- `ORCH_PLAN.md`
- `spec-core/src/schema/unit.spec.json`
- `spec-core/src/schema/test.spec.json` as review-only unless explicit Rust-only protection requires an edit
- `spec-core/src/types.rs`
- allowed mechanical compile-fix spillover paths from `PLAN.md`

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
git rev-parse 741a83e
git rev-parse d10679a
cargo test -p spec-core --lib -- --color never
```

Parent Step 2 must deliver:

- `spec-core/src/schema/unit.spec.json` accepts additive `body.typescript` for `kind:function`
- `spec-core/src/types.rs::Body` is widened to include `typescript: Option<String>`
- `ResolvedSpec` carries explicit authored TypeScript truth
- `.test.spec` remains Rust-only
- any compile-fix spillover is mechanical and bounded

### Lane A contract: Step 3 shared consumer re-anchor

Owned paths:

- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs` only if required for test or export plumbing

Required acceptance commands:

```bash
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-core --lib monotone_up_regression_ -- --color never
cargo test -p spec-core --lib -- --color never
```

Lane A must deliver:

- `validate_function_semantic()` allows additive function `body.typescript`
- seam kinds still reject top-level `body.typescript`
- Rust generation remains Rust-only in M29R
- monotone-up semantic review reads explicit authored TypeScript truth
- no `spec_version` sentinel remains necessary for TypeScript body presence

### Lane B contract: Step 4 read-side proof and harness alignment

Owned paths:

- `spec-cli/tests/m14_regressions.rs`
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
cargo test -p spec-cli --test m14_regressions monotone_up_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo test -p xtask family_smoke_accepts_committed_monotone_up_scaffold_surfaces -- --color never
cargo test -p xtask family_smoke_rejects_monotone_up_aligned_starter_shape_drift -- --color never
cargo test -p xtask monotone_up_harness_contract_is_locked -- --color never
cargo test -p xtask monotone_up_suite_ownership_rejects_suite_names_without_locked_slug -- --color never
cargo test -p xtask monotone_up_suite_ownership_rejects_expected_tests_without_locked_slug -- --color never
```

Secondary confidence check, not the primary lane gate:

```bash
cargo test -p xtask -- --color never
```

Lane B must deliver:

- copied monotone-up fixtures that carry additive `body.typescript` load truthfully
- read-side proof surfaces align to the repaired shared contract
- family harness assumptions are re-anchored to the shared contract rather than hidden sentinels
- monotone-up scaffold, harness, and suite-ownership contracts stay locked under the real in-tree `xtask` test surfaces above
- the packet root remains `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
- Rust remains the default omitted-flag lane

### Lane C contract: Step 5 packet replay

Owned paths:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Required acceptance commands:

```bash
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Lane C must deliver:

- packet truth stays under the locked packet root
- the required buckets remain:
  - `aligned`
  - `drift`
  - `under_specified`
  - `unsupported_near_miss`
- packet truth is explicit and additive
- no new packet tree, no checked-in generated output, no checked-in `node_modules`

### Lane D contract: Step 6 CI lane update

Owned paths:

- `.github/workflows/ci.yml`

Required structural acceptance:

- `.github/workflows/ci.yml` is the only changed file
- the workflow encodes the exact merged-state verification commands from `PLAN.md`
- the workflow preserves the ordinary workspace lane and adds the monotone-up pilot lane on the exact pushed SHA
- the workflow does not substitute repo-wide `spec build/test --target-language typescript` for the locked prove/certify commands

Lane D must deliver:

- automatic CI execution on push or PR for the exact pushed SHA
- ordinary workspace health preserved
- targeted monotone-up proof lane preserved
- clear failing signal if the pilot lane regresses

## Workstream Plan

### WS-0 Baseline capture - parent only

#### `task/m29r-00-baseline`

Required parent actions:

1. Confirm the live branch is still `feat/corpus-expansion`.
2. Record the live SHA, dirty state, and overlap with M29R-owned paths.
3. Record `741a83e` as the recovery seed.
4. Record `d10679a` as blocked checkpoint history only.
5. Stop immediately if unresolved dirty overlap exists inside the closed implementation surface.

Acceptance:

- `baseline.json` exists
- overlap is either empty or explicitly blocked
- recovery seed and blocked checkpoint are recorded

### WS-1 Orchestration freeze - parent only

#### `task/m29r-01-authority-freeze`

Required parent actions:

1. Rewrite `ORCH_PLAN.md` to current M29R truth.
2. Write `authority-freeze.json`.
3. Write `tasks.json`.
4. Record the lane map, concurrency cap, hard guards, and blocker protocol.

Acceptance:

- no worker launches before `authority-freeze.json`
- `ORCH_PLAN.md`, `authority-freeze.json`, and `tasks.json` agree on lane order and freeze points

### WS-2 Lane A Stage A1 contract repair - parent only

#### `task/m29r-02-step2-contract-repair`

Mission:

- land the shared schema/model repair before any parallel work begins
- create the contract freeze that both parallel lanes must inherit literally

Required parent actions:

1. Create `ws/m29r-int` from `741a83e`.
2. Land Step 2 on `ws/m29r-int`:
  - widen function-unit schema truth
  - widen `Body`
  - thread `ResolvedSpec.body_typescript`
  - apply only bounded mechanical compile-fix spillover
3. Confirm `.test.spec` remains Rust-only.
4. Run the parent Step 2 command contract.
5. Write `contract-freeze.json`.
6. Fork `ws/m29r-lane-a` and `ws/m29r-lane-b` from the exact `contract_freeze_commit`.

Acceptance:

- Step 2 is landed before any worker starts
- `contract-freeze.json` exists
- both worker branches fork from the same recorded SHA
- the freeze explicitly records that `.test.spec` remains Rust-only and that compile-fix spillover is bounded

### WS-3 Parallel shared-consumer and read-side lanes - workers, concurrency cap 2

#### `task/m29r-a-step3-consumers` on `ws/m29r-lane-a`

Worker mission:

- continue `Lane A` with Step 3 shared consumer re-anchor on top of the parent-frozen Step 2 contract, without widening execution scope

Worker must not do:

- edit packet files
- edit `.github/workflows/ci.yml`
- widen `.test.spec`
- add repo-wide target-language CLI

Acceptance:

- all `Lane A` acceptance commands pass
- changed files stay inside `Lane A` ownership
- no semantic drift from the frozen Step 2 contract

#### `task/m29r-b-step4-readside` on `ws/m29r-lane-b`

Worker mission:

- align read-side proof surfaces and family harness assumptions to the repaired shared contract

Worker must not do:

- edit `spec-core/src/schema/*`
- edit `spec-core/src/types.rs`
- edit packet files
- edit `.github/workflows/ci.yml`
- invent a second packet root

Acceptance:

- all `Lane B` acceptance commands pass
- changed files stay inside `Lane B` ownership
- no packet-root drift or omitted-flag Rust regression is introduced

### WS-4 Parent merge and packet freeze - parent only

#### `task/m29r-03-freeze-packet`

Strict merge order:

1. merge `ws/m29r-lane-a` into `ws/m29r-int`
2. rerun the `Lane A` acceptance commands from merged state
3. merge `ws/m29r-lane-b` into `ws/m29r-int`
4. rerun the `Lane B` acceptance commands from merged state
5. write `packet-freeze.json`
6. fork `ws/m29r-lane-c` from the recorded post-merge SHA

Parent may resolve only:

- straightforward import ordering
- mechanical context drift
- adjacent test insertion conflicts that do not change meaning

Parent must bounce work back to the owning lane for:

- disagreement about how `body.typescript` is carried
- disagreement about seam or `.test.spec` containment
- disagreement about packet root or bucket ownership
- disagreement about harness expectations versus shared contract truth

Acceptance:

- `Lane A` and `Lane B` are merged and re-verified from integration state
- `packet-freeze.json` exists
- `Lane C` receives exact packet ownership, bucket expectations, and prove/certify commands

### WS-5 Packet replay - worker

#### `task/m29r-c-step5-packet` on `ws/m29r-lane-c`

Worker mission:

- replay the monotone-up packet only after the shared contract and read-side surfaces are honest

Required worker actions:

1. Refresh only the required packet fixtures under the locked packet root.
2. Preserve the four required buckets.
3. Keep packet truth explicit and additive.
4. Run the `Lane C` prove/certify command contract.

Acceptance:

- all `Lane C` commands pass
- no files outside the locked packet root change
- packet truth remains single-root and bucket-complete

### WS-6 Parent merge and CI freeze - parent only

#### `task/m29r-04-freeze-ci`

Required parent actions:

1. merge `ws/m29r-lane-c` into `ws/m29r-int`
2. rerun the `Lane C` acceptance commands from merged state
3. write `ci-freeze.json`
4. fork `ws/m29r-lane-d` from the recorded post-packet SHA

Acceptance:

- `Lane C` merged cleanly
- `ci-freeze.json` exists
- the exact workflow command list is frozen before CI work starts

### WS-7 CI lane update - worker

#### `task/m29r-d-step6-ci` on `ws/m29r-lane-d`

Worker mission:

- encode the exact final command floor in CI without widening scope

Required worker actions:

1. Update `.github/workflows/ci.yml` only.
2. Preserve the ordinary workspace lane.
3. Add the monotone-up pilot lane using the exact commands frozen in `ci-freeze.json`.
4. Do not replace the frozen commands with broader repo-wide target-language commands.

Acceptance:

- `.github/workflows/ci.yml` is the only changed file
- the workflow command list matches `ci-freeze.json`
- the worker does not claim success based only on YAML edits; real success waits for pushed-SHA CI observation

### WS-8 Final verification, push, and closeout - parent only

#### `task/m29r-05-final-verify`

The parent must run this exact merged-state verification sequence from `ws/m29r-int` before calling M29R done:

```bash
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-core --lib monotone_up_regression_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test --workspace
```

Rules:

- record every actual command and exit code in `proof-log.json`
- do not substitute broader or different commands for the sequence above
- if any command fails, stop the run and write `blocked.json`

#### `task/m29r-06-push-observe`

Required parent actions:

1. merge `ws/m29r-lane-d` into `ws/m29r-int`
2. push the final integration branch or designated final candidate branch
3. record remote, branch, SHA, and timestamp in `push-record.json`
4. observe the CI run triggered by that exact pushed SHA
5. record workflow name, run id or URL, observed SHA, and lane results in `ci-observation.json`

Acceptance:

- push succeeded
- CI ran on the exact pushed SHA
- ordinary workspace lane is green
- monotone-up pilot lane is green

#### `task/m29r-07-closeout`

Closeout must write `closeout.md` and answer plainly:

1. what changed in the shared contract
2. whether `.test.spec` and seam kinds stayed closed to TypeScript bodies
3. whether Rust-default behavior stayed stable
4. whether any temporary compatibility baggage survived
5. whether CI proved the exact pushed SHA
6. whether the verdict is `EXPAND`, `NARROW`, or `STOP`

Verdict rules:

- `EXPAND` only if:
  - the exact merged-state verification sequence is green
  - pushed-SHA CI is green
  - one honest shared authoring boundary held for Rust and TypeScript
  - `.test.spec` and seam kinds stayed closed
  - no wider architecture breach was required
- `NARROW` only if:
  - the exact merged-state verification sequence is green
  - pushed-SHA CI is green
  - the repaired contract works, but further expansion would require narrower follow-on scope first
- `STOP` if:
  - any required merged-state verification command fails
  - pushed-SHA CI fails or does not run on the exact SHA
  - the run leaks beyond the closed surface
  - `.test.spec` or seam containment is breached

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
- whether that skip blocks merge

Workers do not return:

- new milestone scope
- new authority text
- merge decisions
- push decisions
- speculative redesign proposals in place of blocker facts

## Conflict Policy

- The parent does not invent a hybrid contract during merge.
- If a worker result conflicts with `PLAN.md` or the relevant freeze record, the parent must do exactly one of:
  - reject the lane and relaunch from the latest freeze
  - apply the already-frozen authority literally if the lane drifted
  - block the run if the conflict exposes real authority drift
- Parent-resolved merge mechanics are limited to syntax-level or context-level drift.
- Any conflict that changes milestone meaning, containment, packet ownership, or command contracts is a bounce-back, not a creative merge.

## Stale-Lane Invalidation

Automatic invalidation rules:

- If Step 2 changes after `Lane A` or `Lane B` is forked, both lanes are stale.
- If `Lane A` or `Lane B` changes any packet expectation after `Lane C` is forked, `Lane C` is stale.
- If `Lane C` changes the required final command floor after `Lane D` is forked, `Lane D` is stale.
- If the workflow command list changes after `Lane D` is forked, `Lane D` is stale.

Invalidation action:

- discard the stale lane
- recreate the branch and worktree from the newest freeze SHA
- relaunch with the new prompt basis

The parent does not hand-patch stale worker branches.

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen `.test.spec`
- they need to widen `kind:data` or `kind:sum`
- they need repo-wide target-language CLI support
- they cannot preserve Rust-default behavior
- they cannot satisfy acceptance commands with concrete evidence

Worker blocker response:

- stop work
- write the sentinel terminal blocked state
- report the smallest blocking fact with evidence

Parent blocker response:

- write `blocked.json`
- stop downstream launches
- stop push and closeout
- do not report partial green success

## Context-Control Rules

- `ws/m29r-int` and all descendant worktrees are seeded from `741a83e`, so any `PLAN.md` or `ORCH_PLAN.md` text that happens to exist inside those seeded worktrees may be stale.
- Worker authority comes from exactly:
  - the parent prompt
  - the relevant freeze record
  - the live authority files on the parent branch
- Worker authority does not come from stale plan-doc snapshots inside seeded worktrees.
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.
- Worker prompts must include only:
  - the relevant `PLAN.md` excerpt
  - the relevant `ORCH_PLAN.md` excerpt
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - worker return contract
  - relevant freeze SHA
- The parent keeps only these live orchestration artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - the latest freeze record
  - the latest integration diff summary
- Parent-owned run-state under `RUN_ROOT` is the execution source of truth.
- Workers do not write `RUN_ROOT/*`.
- Parent reviews only:
  - changed files
  - command results
  - blockers
  - unresolved assumptions
- Close each worker immediately after merge or relaunch decision.

## Acceptance Gates

### Gate 0: baseline gate

Required:

- `baseline.json`
- recorded `741a83e`
- recorded `d10679a`
- no unresolved dirty overlap inside the closed implementation surface

### Gate 1: authority freeze gate

Required:

- `ORCH_PLAN.md` rewritten to M29R truth
- `authority-freeze.json`
- `tasks.json`
- concurrency cap fixed at `2`

### Gate 2: contract freeze gate

Required:

- Step 2 landed on `ws/m29r-int`
- `contract-freeze.json`
- both `Lane A` and `Lane B` forked from the same recorded SHA
- `.test.spec` explicitly remains Rust-only

### Gate 3: packet launch gate

Required:

- `Lane A` merged and re-verified
- `Lane B` merged and re-verified
- `packet-freeze.json`
- packet ownership, bucket set, and prove/certify commands are explicit

### Gate 4: CI launch gate

Required:

- `Lane C` merged and re-verified
- `ci-freeze.json`
- workflow command list is frozen and explicit

### Gate 5: final merged-state verification gate

Required:

- `Lane D` merged
- the exact merged-state verification sequence passes
- `proof-log.json` records every command and exit code

### Gate 6: pushed-SHA CI observation gate

Required:

- push succeeded
- `push-record.json` records the pushed branch and SHA
- `ci-observation.json` records the exact observed SHA
- ordinary workspace lane green
- monotone-up pilot lane green

### Gate 7: closeout gate

Required:

- `closeout.md`
- exact verdict: `EXPAND`, `NARROW`, or `STOP`
- verdict justified by Gate 5 and Gate 6 results

## Completion Criteria

M29R orchestration is complete only when all are true:

1. the run restarted from `741a83e`
2. `d10679a` is preserved as blocked history only
3. Step 2 landed before any worker started
4. only `Lane A` and `Lane B` ran in parallel
5. the parent remained sole integrator, sole freeze authority, sole push authority, and sole final verifier
6. `Lane C` started only after the post-`Lane A`/`Lane B` packet freeze
7. `Lane D` started only after the post-`Lane C` CI freeze
8. `.test.spec` remained Rust-only
9. seam kinds remained closed to top-level `body.typescript`
10. the packet root remained `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
11. the exact merged-state verification sequence from `PLAN.md` passed
12. the pushed branch triggered CI on the exact observed SHA
13. closeout ended with exactly one verdict: `EXPAND`, `NARROW`, or `STOP`

## Assumptions

- The existing CI workflow remains a single-file update in `.github/workflows/ci.yml`, not a new workflow family.
- `cargo test -p xtask -- --color never` is an acceptable read-side harness acceptance gate before packet replay because Step 5 is where prove/certify becomes mandatory.
- Step 2 may require bounded mechanical compile-fix spillover exactly as described in `PLAN.md`; no broader semantic spillover is assumed.
