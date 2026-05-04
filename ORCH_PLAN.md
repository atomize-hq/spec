# M30 Orchestration Plan

Status: **authoritative execution contract for the full M30 run**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Live branch: **`feat/corpus-expansion`**
Review base: **`main`**
Last rewritten: **2026-05-04**
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof`**
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof`**
Locked packet root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/function.wrapper.pipeline.v1`**

## Summary

- This run is for **M30** only. `PLAN.md` is the milestone authority and this file is the authoritative orchestration contract that carries the run from kickoff through final closeout.
- The parent agent remains the sole integrator, sole freeze authority, sole stale-lane invalidator, sole push authority, and sole final verifier.
- Work starts with a parent-owned baseline capture and orchestration freeze, then `Lane A` lands the wrapper packet truth plus scaffold/smoke alignment before any parallel worker starts.
- After `Lane A` is merged and frozen, exactly two worker lanes may run in parallel:
  - `Lane B` = wrapper semantic-review assertion, wrapper truth-surface suite, copied-fixture corpus/regression proof, and any required wrapper harness membership updates
  - `Lane C` = explicit target-language allowlist widening to exactly two families plus the dedicated `wrapper_pipeline_pilot` CI job while preserving `monotone_up_pilot`
- Maximum worker concurrency is `2`. Recommended worker profile is `GPT-5.4` with `high` reasoning for both lanes.
- The implementation surface stays bounded to the M30 closed surface in `PLAN.md`. No repo-wide TypeScript support, no new packet root, no `.test.spec` widening, and no hidden family-specific routing are allowed.
- Parent-owned run-state under `RUN_ROOT` is the source of truth for freezes, launches, merges, verification, push observation, and restart. Worker chat history is not part of execution truth.

## Hard Guards

- `PLAN.md` wins over this document, worker summaries, stale worktrees, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen in `ws/m30-int`.
- The closed implementation surface for M30 is:
  - `semantic-families/function.wrapper.pipeline.v1/**`
  - `spec-core/src/semantic_review.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `xtask/src/family/prove.rs`
  - `xtask/src/family/certify.rs`
  - `xtask/src/family/harness.rs`
  - `xtask/src/family/scaffold.rs`
  - `xtask/src/lib.rs`
  - `.github/workflows/ci.yml`
  - `PLAN.md` is authority only and is not edited during execution
- Allowed mechanical spillover is tightly bounded to compile or expectation fallout directly caused by the primary surface above.
  - Likely spillover sites, only if forced:
    - `spec-core/src/types.rs`
    - `spec-core/src/generator.rs`
    - `spec-core/src/passport.rs`
  - Any semantic broadening outside the primary surface blocks the run until authority is rewritten.
- `semantic-families/function.wrapper.pipeline.v1/` remains the only authoritative packet root for the second proof.
- Committed wrapper packet bytes, not tests, author TypeScript truth.
- The explicit allowlist may contain exactly two promoted families in M30:
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.wrapper.pipeline.v1`
- `family prove` and `family certify` must keep the existing harness, suite slugs, and artifact paths. No TypeScript-specific suite namespace and no new artifact tree are allowed.
- CI must keep failure attribution family-local: `test`, `monotone_up_pilot`, and `wrapper_pipeline_pilot` remain distinct.
- Stop immediately if any lane requires:
  - repo-wide `spec build/test --target-language typescript`
  - widening TypeScript support beyond promoted `kind:function` families
  - `.test.spec` TypeScript support
  - `kind:data` or `kind:sum` TypeScript support
  - a new packet root
  - runtime mutation helpers that synthesize missing wrapper TypeScript bodies

## Worktree Layout

Canonical worktrees:

- integration
  - branch: `ws/m30-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof/int`
- `Lane A`
  - branch: `ws/m30-lane-a`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof/lane-a`
- `Lane B`
  - branch: `ws/m30-lane-b`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof/lane-b`
- `Lane C`
  - branch: `ws/m30-lane-c`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof/lane-c`

Creation rules:

- The parent captures baseline state from the live checkout on `feat/corpus-expansion`, then creates `ws/m30-int` from the current live SHA recorded in `baseline.json`.
- `Lane A` is forked from `ws/m30-int` after the orchestration freeze is written.
- `Lane B` and `Lane C` are both forked from the exact post-`Lane A` SHA recorded in `lane-a-freeze.json`.
- No worker is forked from another worker branch.
- If any named branch or worktree already exists and points at stale or conflicting state, the parent removes and recreates it before reuse and records that in `session-log.md`.
- A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent never hand-forwards a stale worker branch.

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof`

Canonical parent-owned files:

- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty-state summary
  - overlap check against M30-owned paths
- `authority-freeze.json`
  - milestone id `M30`
  - authority paths
  - worker model recommendation
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
- `lane-a-freeze.json`
  - exact post-`Lane A` commit
  - locked packet root
  - locked 12-spec wrapper truth matrix
  - frozen starter-contract expectations
  - exact acceptance commands for `Lane B` and `Lane C`
- `lane-b-launch.md`
  - reproducible launch packet for `Lane B`
  - exact prompt basis excerpt references
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
- `lane-c-launch.md`
  - reproducible launch packet for `Lane C`
  - exact prompt basis excerpt references
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
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
  - `monotone_up_pilot` result
  - `wrapper_pipeline_pilot` result
- `blocked.json`
  - blocking task
  - blocking evidence
  - required next decision
- `closeout.md`
  - contract summary
  - containment summary
  - two-family proof summary
  - CI summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-01-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-a-packet-scaffold/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-02-freeze-post-lane-a/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-b-truth-surfaces/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-c-gate-and-ci/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-03-merge-parallel-lanes/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-04-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-05-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m30_wrapper_second_family_proof/task-m30-06-closeout/`

Each sentinel directory contains:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m30-00-baseline
  -> task/m30-01-authority-freeze
      -> task/m30-a-packet-scaffold
          -> task/m30-02-freeze-post-lane-a
task/m30-02-freeze-post-lane-a
  -> task/m30-b-truth-surfaces
  -> task/m30-c-gate-and-ci
task/m30-b-truth-surfaces
  -> task/m30-03-merge-parallel-lanes
task/m30-c-gate-and-ci
  -> task/m30-03-merge-parallel-lanes
task/m30-03-merge-parallel-lanes
  -> task/m30-04-final-verify
      -> task/m30-05-push-observe
          -> task/m30-06-closeout
```

Execution meaning:

1. Parent captures live branch and overlap facts.
2. Parent freezes orchestration authority and creates the integration worktree.
3. `Lane A` lands the wrapper packet additive `body.typescript` truth across all 12 unit specs and aligns scaffold/smoke expectations.
4. Parent merges `Lane A`, writes `lane-a-freeze.json`, and forks `Lane B` and `Lane C` from that exact frozen SHA.
5. `Lane B` and `Lane C` run in parallel with disjoint ownership.
6. Parent merges both parallel lanes back into `ws/m30-int`, reruns their acceptance commands on merged state, then runs the full merged-state verification floor.
7. Parent pushes, observes CI on the exact pushed SHA, and writes closeout with one verdict.

## Command Contracts

### Parent baseline and orchestration freeze

Owned paths:

- `ORCH_PLAN.md`
- `RUN_ROOT/**`

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse --abbrev-ref HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec status --short
```

Parent must deliver:

- baseline recorded from `feat/corpus-expansion`
- overlap check recorded before any worktree launch
- `authority-freeze.json`, `tasks.json`, and `ORCH_PLAN.md` agreeing on lane order, worker ownership, freezes, and blockers

### Lane A contract: wrapper packet truth + scaffold alignment

Owned paths:

- `semantic-families/function.wrapper.pipeline.v1/**`
- `xtask/src/family/scaffold.rs`
- `xtask/src/lib.rs`

Required edits:

- add additive `body.typescript` to all 12 wrapper packet unit specs:
  - `pricing_discount_leaf_aligned.unit.spec`
  - `pricing_tax_leaf_aligned.unit.spec`
  - `pricing_total_wrapper_aligned.unit.spec`
  - `pricing_discount_leaf_drift.unit.spec`
  - `pricing_tax_leaf_drift.unit.spec`
  - `pricing_total_wrapper_drift.unit.spec`
  - `pricing_discount_leaf_under_specified.unit.spec`
  - `pricing_tax_leaf_under_specified.unit.spec`
  - `pricing_total_wrapper_under_specified.unit.spec`
  - `pricing_discount_leaf_unsupported_near_miss.unit.spec`
  - `pricing_tax_leaf_unsupported_near_miss.unit.spec`
  - `pricing_total_wrapper_unsupported_near_miss.unit.spec`
- keep each TypeScript body bucket-faithful to that bucket's Rust body
- keep wrapper leaf units truthful packet-local deps of wrapper units
- align scaffold starter generation with the committed wrapper packet contract
- align wrapper smoke-contract expectations with the committed starter contract

Required acceptance commands:

```bash
cargo test -p xtask family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces -- --color never
cargo xtask family smoke function.wrapper.pipeline.v1
```

Lane A must deliver:

- committed wrapper packet bytes carry truthful additive TypeScript across `aligned`, `drift`, `under_specified`, and `unsupported_near_miss`
- no test-time helper is required to inject missing wrapper TypeScript bodies
- the wrapper starter contract is no longer Rust-only if the committed packet is not

### Lane B contract: wrapper semantic review + truth surfaces + copied-fixture proof

Owned paths:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `xtask/src/family/harness.rs`

Required edits:

- add a wrapper semantic-review assertion parallel to the monotone-up authored-TypeScript assertion and ensure it cites `body.typescript`
- keep `wrapper_pipeline_classifier_*` green
- keep wrapper truth-surface coverage green:
  - `wrapper_pipeline_truth_surface_command_matrix_preserves_until_spec_test_refresh`
  - `wrapper_pipeline_truth_surface_stale_status_and_export_preserve_last_proven_review`
  - `wrapper_pipeline_truth_surface_unsupported_near_miss_command_matrix_stays_neutral`
- extend copied-wrapper fixture coverage so corpus and regression proof read committed bytes only
- update wrapper harness suite membership only if new wrapper test names make that necessary

Required acceptance commands:

```bash
cargo test -p spec-core --lib wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_regression_ -- --color never
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
```

Lane B must deliver:

- wrapper authored TypeScript is proven visible to semantic review through the shared authored packet surface
- wrapper truth-surface status/export behavior remains honest
- wrapper copied-fixture regressions stay green on committed bytes
- the existing monotone-up TypeScript proof remains green on the shared proof surfaces Lane B touches

### Lane C contract: two-family allowlist gate + dedicated CI pilot

Owned paths:

- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `.github/workflows/ci.yml`

Required edits:

- widen `validate_target_language` only far enough to accept:
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.wrapper.pipeline.v1`
- preserve rejection behavior for every other family
- keep prove/certify suite names, report names, and artifact paths unchanged
- add a dedicated `wrapper_pipeline_pilot` job
- keep `monotone_up_pilot` as a separate job
- update downstream release jobs that currently depend on `[test, monotone_up_pilot]` so they also depend on `wrapper_pipeline_pilot`

Required acceptance commands:

```bash
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Required structural acceptance:

- `.github/workflows/ci.yml` is the only workflow file touched
- `wrapper_pipeline_pilot` exists as a distinct job from `monotone_up_pilot`
- the wrapper pilot encodes exactly:
  - `cargo test -p spec-core --lib wrapper_pipeline_ -- --color never`
  - `cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never`
  - `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never`
  - `cargo xtask family prove function.wrapper.pipeline.v1`
  - `cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript`
  - `cargo xtask family certify function.wrapper.pipeline.v1`
  - `cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript`
- release gating depends on `test`, `monotone_up_pilot`, and `wrapper_pipeline_pilot`

Lane C must deliver:

- TypeScript proof is widened to exactly two promoted function families and nothing more
- wrapper public CI proof is separate and attributable
- the existing monotone-up pilot remains distinct and preserved

## Workstream Plan

### WS-0 Baseline capture - parent only

#### `task/m30-00-baseline`

Required parent actions:

1. Confirm the live branch is still `feat/corpus-expansion`.
2. Record the live SHA, dirty state, and overlap with M30-owned paths.
3. Stop immediately if unresolved dirty overlap exists inside the closed implementation surface.

Acceptance:

- `baseline.json` exists
- overlap is either empty or explicitly blocked
- the live SHA used to seed `ws/m30-int` is recorded

### WS-1 Orchestration freeze - parent only

#### `task/m30-01-authority-freeze`

Required parent actions:

1. Rewrite `ORCH_PLAN.md` to current M30 truth.
2. Write `authority-freeze.json`.
3. Write `tasks.json`.
4. Create `ws/m30-int` from the recorded live SHA.
5. Fork `ws/m30-lane-a` from `ws/m30-int`.

Acceptance:

- no worker launches before `authority-freeze.json`
- `ORCH_PLAN.md`, `authority-freeze.json`, and `tasks.json` agree on lane order and hard guards

### WS-2 Wrapper packet truth and starter alignment - worker

#### `task/m30-a-packet-scaffold` on `ws/m30-lane-a`

Worker mission:

- make the committed wrapper packet bytes truthful first, then align scaffold and smoke contracts to that truth

Worker must not do:

- edit `spec-core/**`
- edit `spec-cli/tests/**`
- edit `xtask/src/family/prove.rs`
- edit `xtask/src/family/certify.rs`
- edit `.github/workflows/ci.yml`

Acceptance:

- all `Lane A` acceptance commands pass
- changed files stay inside `Lane A` ownership
- the 12-spec wrapper matrix is complete before the lane is considered done

### WS-3 Parent merge and post-packet freeze - parent only

#### `task/m30-02-freeze-post-lane-a`

Strict merge order:

1. merge `ws/m30-lane-a` into `ws/m30-int`
2. rerun the `Lane A` acceptance commands from merged state
3. write `lane-a-freeze.json`
4. write `lane-b-launch.md` and `lane-c-launch.md`
5. fork `ws/m30-lane-b` and `ws/m30-lane-c` from the recorded frozen SHA

Parent may resolve only:

- straightforward import ordering
- mechanical context drift
- adjacent test insertion conflicts that do not change meaning

Parent must bounce work back to the owning lane for:

- incomplete or inconsistent wrapper packet TypeScript truth
- disagreement about bucket-faithful TypeScript bodies
- disagreement about wrapper starter contract truth
- any attempt to synthesize wrapper TypeScript at test runtime

Acceptance:

- `Lane A` is merged and re-verified from integration state
- `lane-a-freeze.json` exists
- `lane-b-launch.md` and `lane-c-launch.md` exist
- `Lane B` and `Lane C` both start from the same frozen SHA

### WS-4 Parallel proof lanes - workers, concurrency cap 2

#### `task/m30-b-truth-surfaces` on `ws/m30-lane-b`

Worker mission:

- prove the shared semantic-review and read-side surfaces consume wrapper TypeScript honestly using committed wrapper bytes

Worker must not do:

- edit packet files
- edit `xtask/src/family/prove.rs`
- edit `xtask/src/family/certify.rs`
- edit `.github/workflows/ci.yml`
- add new runtime fixture mutation helpers for wrapper proof

Acceptance:

- all `Lane B` acceptance commands pass
- changed files stay inside `Lane B` ownership
- wrapper proof stays grounded in committed packet bytes

#### `task/m30-c-gate-and-ci` on `ws/m30-lane-c`

Worker mission:

- widen the family prove gate to exactly two families and expose the wrapper proof as a distinct CI signal

Worker must not do:

- edit packet files
- edit `spec-core/**`
- edit `spec-cli/tests/**`
- edit `xtask/src/family/harness.rs`
- introduce a TypeScript-specific artifact namespace

Acceptance:

- all `Lane C` acceptance commands pass
- required structural acceptance for `.github/workflows/ci.yml` is satisfied
- changed files stay inside `Lane C` ownership

### WS-5 Parent merge of parallel lanes - parent only

#### `task/m30-03-merge-parallel-lanes`

Strict merge order:

1. merge `ws/m30-lane-b` into `ws/m30-int`
2. rerun the `Lane B` acceptance commands from merged state
3. merge `ws/m30-lane-c` into `ws/m30-int`
4. rerun the `Lane C` acceptance commands from merged state
5. if merge fallout appears, resolve only syntax-level or context-level drift and record it in `merge-log.md`

Parent must bounce work back to the owning lane for:

- disagreement about wrapper harness membership versus wrapper test reality
- disagreement about the exact two-family allowlist
- disagreement about CI job boundaries or release-gating dependencies
- any broadened TypeScript surface beyond the M30 contract

Acceptance:

- `Lane B` and `Lane C` are merged and re-verified from integration state
- `merge-log.md` records merge SHAs, conflicts, and any stale-lane decisions

### WS-6 Final verification - parent only

#### `task/m30-04-final-verify`

The parent must run this exact merged-state verification sequence from `ws/m30-int` before calling M30 done:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
cargo check --manifest-path examples/ecommerce/Cargo.toml
cargo test -p xtask family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces -- --color never
cargo test -p spec-core --lib wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_regression_ -- --color never
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo xtask family smoke function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Rules:

- record every actual command and exit code in `proof-log.json`
- do not substitute broader or different commands for the sequence above
- if any command fails, stop the run and write `blocked.json`

### WS-7 Push and CI observation - parent only

#### `task/m30-05-push-observe`

Required parent actions:

1. push the final integration branch or designated final candidate branch
2. record remote, branch, SHA, and timestamp in `push-record.json`
3. observe the CI run triggered by that exact pushed SHA
4. record workflow name, run id or URL, observed SHA, and lane results in `ci-observation.json`

Acceptance:

- push succeeded
- CI ran on the exact pushed SHA
- ordinary workspace lane is green
- `monotone_up_pilot` is green
- `wrapper_pipeline_pilot` is green

### WS-8 Closeout - parent only

#### `task/m30-06-closeout`

Closeout must write `closeout.md` and answer plainly:

1. Did one shared authored TypeScript contract survive on two promoted `kind:function` families?
2. Did ordinary CI, `monotone_up_pilot`, and `wrapper_pipeline_pilot` all pass on the pushed SHA?
3. Did wrapper proof reuse the existing family registry, packet root, harness, and artifact paths?
4. What exact question is still unanswered after M30?

Allowed closeout verdicts:

- `EXPAND`
  - the second-family proof passed cleanly and the repo is ready to consider the next bounded TypeScript question
- `NARROW`
  - the wrapper proof worked, but one additional bounded follow-on is still required before broader expansion is honest
- `STOP`
  - the second proof required a new packet root, repo-wide target-language support, hidden family-specific routing, or failed the verification floor

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
- authority rewrites
- merge decisions
- push decisions
- worker chat history as truth source

## Worker Prompt Contract

The parent launches every worker lane from run-state files, not from remembered chat context.

Every worker launch packet must include exactly:

- the lane mission statement from this file
- the exact relevant `PLAN.md` excerpt for that lane
- the exact relevant `ORCH_PLAN.md` excerpt for that lane
- owned paths
- forbidden paths
- exact acceptance commands
- applicable hard guards
- the applicable freeze record path
- the frozen launch SHA
- the required worker return contract

Parent-owned live working context is limited to:

- `PLAN.md`
- `ORCH_PLAN.md`
- `authority-freeze.json`
- the latest freeze record
- the lane-specific launch file being issued
- the current integration diff summary

Parent-owned information that is offloaded to run-state and must be read from files rather than reconstructed from chat:

- baseline capture
- lane launch packets
- merge decisions and merge outcomes
- stale-lane invalidation history
- final verification logs
- push records
- CI observations
- closeout evidence

## Conflict Policy

- The parent does not invent a hybrid milestone contract during merge.
- If a worker result conflicts with `PLAN.md` or the relevant freeze record, the parent must do exactly one of:
  - reject the lane and relaunch from the latest freeze
  - apply the already-frozen authority literally if the lane drifted
  - block the run if the conflict exposes real authority drift
- Parent-resolved merge mechanics are limited to syntax-level or context-level drift.
- Any conflict that changes packet truth, starter truth, harness ownership, allowlist scope, CI job boundaries, or command contracts is a bounce-back, not a creative merge.

## Stale-Lane Invalidation

Automatic invalidation rules:

- If the wrapper packet file set, bucket names, or starter-contract expectations change after `Lane B` or `Lane C` is forked, both lanes are stale.
- If the exact acceptance command floor for `Lane B` or `Lane C` changes after the lane is forked, that lane is stale.
- If a post-fork parent decision changes the exact two-family allowlist or the required CI job contract, `Lane C` is stale.
- `Lane C` is not stale merely because `Lane B` lands first; it becomes stale only if the parent changes the frozen CI or prove/certify contract that `Lane C` was launched against.

Invalidation action:

- discard the stale lane
- recreate the branch and worktree from the newest freeze SHA
- relaunch with the new prompt basis

The parent does not hand-patch stale worker branches.

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen TypeScript support beyond the M30 closed surface
- they need to synthesize wrapper TypeScript truth at test runtime
- they cannot preserve the exact two-family allowlist
- they cannot preserve distinct `monotone_up_pilot` and `wrapper_pipeline_pilot` jobs
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

- Worker authority comes from exactly:
  - the parent prompt
  - the relevant `PLAN.md` excerpt
  - the relevant `ORCH_PLAN.md` excerpt
  - the relevant freeze record
  - the lane-specific launch file under `RUN_ROOT`
- Worker authority does not come from:
  - stale plan snapshots inside seeded worktrees
  - prior worker chat history
  - inferred milestone scope beyond M30
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.
- Worker prompts must include only:
  - owned paths
  - forbidden paths
  - exact authority excerpts
  - exact acceptance commands
  - applicable freeze record path
  - frozen launch SHA
  - applicable hard guards

## Acceptance Gates

### Gate 0: baseline gate

Required:

- `baseline.json` exists
- live branch is `feat/corpus-expansion`
- no unresolved dirty overlap exists inside the M30-owned surface

### Gate 1: post-`Lane A` freeze gate

Required:

- `Lane A` is merged and re-verified on `ws/m30-int`
- `lane-a-freeze.json` exists
- the locked wrapper matrix covers all 12 unit specs across `aligned`, `drift`, `under_specified`, and `unsupported_near_miss`
- `lane-b-launch.md` and `lane-c-launch.md` exist and point at the same frozen SHA
- `Lane B` and `Lane C` both fork from the same recorded SHA

### Gate 2: parallel-lane merge gate

Required:

- `Lane B` acceptance commands pass on merged integration state
- `Lane C` acceptance commands pass on merged integration state
- no broadened TypeScript surface escaped the exact two-family allowlist and bounded CI contract

### Gate 3: final verification gate

Required:

- the exact merged-state verification sequence passes
- `proof-log.json` records every command and exit code
- wrapper Rust and TypeScript family prove/certify both pass
- monotone-up TypeScript prove/certify still passes unchanged

### Gate 4: pushed-SHA CI observation gate

Required:

- push succeeded
- `push-record.json` records the exact pushed SHA
- `ci-observation.json` records that same SHA
- `test`, `monotone_up_pilot`, and `wrapper_pipeline_pilot` are all green

### Gate 5: closeout gate

Required:

- `closeout.md` exists
- closeout answers the four M30 questions in this document
- verdict is exactly one of `EXPAND`, `NARROW`, or `STOP`

## Tests And Acceptance

Operator-facing end-state expectations:

- Packet truth
  - all 12 wrapper packet unit specs under `semantic-families/function.wrapper.pipeline.v1/**` carry additive `body.typescript`
  - the four required buckets remain complete and truthful: `aligned`, `drift`, `under_specified`, `unsupported_near_miss`
  - wrapper leaf packet units remain truthful packet-local dependencies of wrapper units
- Scaffold and smoke honesty
  - scaffold output no longer describes wrapper starters as Rust-only if the committed packet carries TypeScript truth
  - `cargo test -p xtask family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces -- --color never`
  - `cargo xtask family smoke function.wrapper.pipeline.v1`
- Semantic-review and read-side proof coverage
  - wrapper semantic-review assertions prove authored `body.typescript` is read through the shared packet surface
  - wrapper truth-surface coverage remains green for command-matrix, stale-status/export, and unsupported near-miss neutrality
  - wrapper copied-fixture corpus and regression suites stay green on committed bytes only
  - monotone-up proof surfaces touched by M30 remain green
- Two-family allowlist behavior
  - `--target-language typescript` is accepted for exactly:
    - `function.arithmetic_leaf.monotone_up.v1`
    - `function.wrapper.pipeline.v1`
  - all other families still fail fast on that flag
  - prove/certify suite names, artifact paths, and report paths remain unchanged
- CI separation and release gating
  - `test`, `monotone_up_pilot`, and `wrapper_pipeline_pilot` are distinct CI jobs
  - `wrapper_pipeline_pilot` runs the exact wrapper proof commands frozen in this contract
  - downstream release gating depends on all three required jobs
- Workspace boundary and bounded scope
  - no repo-wide TypeScript build/test support is added
  - no `.test.spec`, `kind:data`, or `kind:sum` widening is introduced
  - no new packet root or TypeScript-specific artifact namespace is introduced
  - any spillover remains mechanical and inside the bounded M30 surface
- Run-state and audit completeness
  - `baseline.json`, `authority-freeze.json`, `tasks.json`, `lane-a-freeze.json`, `lane-b-launch.md`, `lane-c-launch.md`, `merge-log.md`, `proof-log.json`, `push-record.json`, `ci-observation.json`, and `closeout.md` exist by the end of a successful run
  - a parent can relaunch `Lane B` or `Lane C` from run-state artifacts alone without consulting worker chat history

## Assumptions

- The live implementation branch for this run remains `feat/corpus-expansion`.
- The parent can create separate worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m30-wrapper-second-family-proof`.
- The wrapper smoke-contract test name in `PLAN.md` is authoritative for M30:
  - `family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces`
- If the CI workflow has multiple downstream release jobs, `Lane C` may update each affected `needs` list inside `.github/workflows/ci.yml`, but it may not otherwise redesign workflow topology.
- If final verification exposes only mechanical merge fallout, the parent may fix that directly on `ws/m30-int`; any semantic change still belongs back in the owning lane.
