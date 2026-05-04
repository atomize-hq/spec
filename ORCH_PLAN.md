# M32 Orchestration Plan

Status: **authoritative execution contract for the split-worktree M32 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Last rewritten: **2026-05-04**  
Required re-anchor: **`ws/m31-int` at `945284ea7ab6bf788d7202ff674b81581afd47c6` or a merged equivalent proven by the parent**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**

## Summary

- This run is for **M32 one bounded second-language promotion path** only.
- `PLAN.md` remains the milestone authority. This file becomes operative only because the work is intentionally split into worktrees.
- The parent agent is the sole integrator, sole freeze authority, sole stale-lane invalidator, sole push authority, sole CI observer, and sole final verifier.
- The parent owns the critical path locally: re-anchor, baseline capture, authority freeze, `Lane A`, every merge, every freeze checkpoint, runtime promotion-artifact emission, final verification, publish, observe, and closeout.
- `Lane A` is the sequential foundation lane and stays parent-owned. No worker launches before `Lane A` lands and the target-language artifact contract is frozen.
- After `Lane A` is merged and frozen, exactly two worker lanes may run in parallel:
  - `Lane B` = monotone-up packet and harness lock-in
  - `Lane C` = read-side truth alignment across semantic review, passport, export, status, and CLI regressions
- `Lane D` runs last as one bounded worker lane for roadmap and packet-doc closeout after `Lane B` and `Lane C` are merged and re-verified.
- Recommended worker profile for `Lane B`, `Lane C`, and `Lane D` is `GPT-5.4` with `reasoning_effort=high`.
- Maximum worker concurrency is `2`.
- The only primary pilot family is `function.arithmetic_leaf.monotone_up.v1`.
- `function.wrapper.pipeline.v1` is regression pressure only. It must stay green where required, but it is not a second M32 certify target.
- No M26-style Gate 1 or Gate 2 human approvals exist in this run. M32 uses parent-owned freeze checkpoints plus machine verification, not approval pauses.
- `promotion.execution.json` for the monotone-up pilot is a mandatory runtime-generated artifact in this run, not just a schema-covered possibility.
- `blocker.report.json` for the same pilot is a mandatory blocked-path artifact whenever a required post-foundation lane or final verification stops after artifact-capable code exists.
- Parent-owned run-state under `RUN_ROOT` is the only execution truth. Worker memory, stale worktree files, and ad hoc notes are not.

## Hard Guards

- `PLAN.md` wins over this document, worker summaries, stale worktree copies, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen on `ws/m32-int`.
- The live checkout on `feat/corpus-expansion` is a publish target and baseline reference, not the merge surface.
- M32 starts only from a parent-recorded M31-integrated base:
  - exact branch: `ws/m31-int`
  - required anchor commit: `945284ea7ab6bf788d7202ff674b81581afd47c6`
  - allowed alternative: a merged-equivalent commit explicitly recorded in `m31-base-freeze.json`
- The closed implementation surface for M32 before authority freeze is:
  - `xtask/src/lib.rs`
  - `xtask/src/family/prove.rs`
  - `xtask/src/family/certify.rs`
  - `xtask/src/family/report.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/harness.rs`
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
  - `semantic-families/README.md`
  - `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- Allowed mechanical spillover is compile- or fixture-forced only:
  - `xtask/src/family/paths.rs`
  - `xtask/src/family/routing.rs`
  - `spec-core/src/types.rs`
  - `spec-core/src/lib.rs`
  - `spec-core/src/validator.rs`
- `PLAN.md` is authority-only during execution. It is read for lane prompts and verification and is not a worker-owned edit surface.
- `ORCH_PLAN.md` is orchestration authority only during execution. It is not delegated.
- After `authority-freeze.json` is written, both `PLAN.md` and `ORCH_PLAN.md` are frozen. They are no longer part of normal execution output, lane ownership, merge scope, or final diff allowance.
- If either authority file must change after `authority-freeze.json`, stop the run, write blocker state, and restart from a new authority baseline rather than mutating the active run contract.
- No one hand-edits JSON under `ARTIFACT_ROOT`. Derived artifacts are created only by repo commands and validated as produced output.
- The required M32 runtime artifact paths are:
  - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/promotion.execution.json`
  - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`
- M32 is function-only. Stop immediately if any lane requires:
  - `kind:data` second-language execution semantics
  - `kind:sum` second-language execution semantics
  - `.test.spec` target-language execution
  - a second primary pilot family
  - a broad repo-wide TypeScript support claim
  - a new CLI entrypoint instead of the existing `cargo xtask family ... --target-language typescript` surface
- `function.wrapper.pipeline.v1` remains comparator pressure only. Workers may preserve or repair wrapper-pipeline regressions, but may not widen M32 into a second certify target for that family.
- The parent may resolve only syntax-level, import-order, or context-drift merge fallout. Semantic ownership conflicts go back to the owning lane.

## Worktree Layout

Canonical worktrees:

- integration
  - branch: `ws/m32-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion/int`
- `Lane A` target-language artifact foundation
  - branch: `ws/m32-lane-a-target-language-foundation`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion/lane-a-target-language-foundation`
- `Lane B` monotone-up packet and harness
  - branch: `ws/m32-lane-b-monotone-up-packet-harness`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion/lane-b-monotone-up-packet-harness`
- `Lane C` read-side truth alignment
  - branch: `ws/m32-lane-c-read-side-truth`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion/lane-c-read-side-truth`
- `Lane D` roadmap and packet-doc closeout
  - branch: `ws/m32-lane-d-docs-closeout`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion/lane-d-docs-closeout`

Creation rules:

- The parent first proves the M31 base in `m31-base-freeze.json`.
- The parent records live branch, live SHA, dirty state, and overlap before creating any M32 worktree.
- `ws/m32-int` is created from the exact re-anchor commit recorded in `m31-base-freeze.json`, not from an unrecorded live `HEAD`.
- `Lane A` is forked from `ws/m32-int` after `authority-freeze.json` is written.
- `Lane B` and `Lane C` are both forked from the exact post-`Lane A` SHA recorded in `lane-a-freeze.json`.
- `Lane D` is forked from the exact post-merge SHA recorded in `post-bc-freeze.json`.
- No worker is forked from another worker branch.
- If any named branch or worktree already exists and points at stale or conflicting state, the parent removes and recreates it before reuse and records that in `session-log.md`.
- A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent does not hand-forward stale worker branches.

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m32-one-bounded-second-language-promotion`
- `ARTIFACT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`

`RUN_ROOT` is a parent-written control plane. Workers may read it, but they do not create, update, or delete files under `RUN_ROOT`.

Canonical parent-owned files:

- `m31-base-freeze.json`
  - required anchor commit
  - chosen integration base commit
  - whether the base is the exact anchor or a merged equivalent
  - evidence proving the equivalence choice
- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty-state summary
  - overlap check against the M32-owned surface
  - carry-forward decision if the live checkout is not clean
- `integration-base.txt`
  - the exact commit used to seed `ws/m32-int`
  - the only allowed diff base for the closed-surface gate in final verification
- `authority-freeze.json`
  - milestone id `M32`
  - authority paths
  - worker model recommendation
  - concurrency cap
  - lane map
  - hard guards
  - publish target branch
- `run-id.txt`
  - the single canonical M32 run id used for family-promotion artifacts
- `artifact-paths.json`
  - absolute and repo-relative path for monotone-up `recommendation.latest.json`
  - absolute and repo-relative path for `promotion.execution.json`
  - absolute and repo-relative path for `blocker.report.json`
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
  - base proof
  - worktree creation
  - freeze creation
  - worker launch
  - merge results
  - stale-lane invalidations
  - publish and CI observation notes
- `lane-a-freeze.json`
  - exact post-`Lane A` commit
  - frozen target-language artifact/report contract
  - exact recommendation-refresh invocation for the monotone-up pilot
  - exact recommendation validation command for that artifact path
  - exact green-path artifact-emission invocation
  - exact blocked-path artifact-emission invocation
  - exact artifact validation commands
  - exact launch SHA for `Lane B` and `Lane C`
  - command order for the Rust lane and TypeScript lane
- `lane-b-launch.md`
  - reproducible launch packet for `Lane B`
  - exact `PLAN.md` excerpt text
  - exact `ORCH_PLAN.md` excerpt text
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
- `lane-c-launch.md`
  - reproducible launch packet for `Lane C`
  - exact `PLAN.md` excerpt text
  - exact `ORCH_PLAN.md` excerpt text
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
- `post-bc-freeze.json`
  - exact post-merge commit after `Lane B` and `Lane C`
  - frozen monotone-up pilot wording for docs
  - explicit statement that wrapper pipeline remains regression pressure only
  - exact `Lane D` acceptance commands
- `lane-d-launch.md`
  - reproducible launch packet for `Lane D`
  - exact `PLAN.md` excerpt text
  - exact `ORCH_PLAN.md` excerpt text
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
- `promotion-execution-record.json`
  - run id
  - recommendation artifact path
  - family
  - artifact path
  - commands captured into the artifact
  - proof artifact references included
  - validation result
- `proof-log.json`
  - actual final merged-state verification commands
  - exit code per command
  - execution order
- `push-record.json`
  - remote
  - pushed branch
  - pushed SHA
  - push timestamp
  - whether the push was `ws/m32-int` or the publish target
- `ci-observation.json`
  - workflow name
  - run id or URL
  - observed branch
  - observed SHA
  - workspace result
- `blocked.json`
  - blocking task
  - blocking evidence
  - required next decision
- `closeout.md`
  - pilot-family summary
  - shared-vs-target-specific residue summary
  - read-side truth alignment summary
  - roadmap summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-00-reanchor/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-01-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-02-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-a-target-language-foundation/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-03-freeze-post-lane-a/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-b-monotone-up-packet-harness/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-c-read-side-truth/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-04-freeze-post-bc/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-d-docs-closeout/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-05-promotion-execution-artifact/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-06-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-07-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/task-m32-08-closeout/`

Each sentinel directory contains parent-written task state only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m32-00-reanchor
  -> task/m32-01-baseline
      -> task/m32-02-authority-freeze
          -> task/m32-a-target-language-foundation
              -> task/m32-03-freeze-post-lane-a
task/m32-03-freeze-post-lane-a
  -> task/m32-b-monotone-up-packet-harness
  -> task/m32-c-read-side-truth
task/m32-b-monotone-up-packet-harness
  -> task/m32-04-freeze-post-bc
task/m32-c-read-side-truth
  -> task/m32-04-freeze-post-bc
task/m32-04-freeze-post-bc
  -> task/m32-d-docs-closeout
      -> task/m32-05-promotion-execution-artifact
          -> task/m32-06-final-verify
              -> task/m32-07-push-observe
                  -> task/m32-08-closeout
```

Execution meaning:

1. Parent proves the M31 base and records the exact M32 integration seed.
2. Parent captures live branch state and overlap facts.
3. Parent freezes orchestration authority and creates the integration worktree.
4. `Lane A` lands the target-language artifact/report contract and freezes the output truth that `Lane B` and `Lane C` must consume.
5. Parent merges `Lane A`, reruns its acceptance commands from merged state, writes `lane-a-freeze.json`, and forks `Lane B` and `Lane C` from that exact frozen SHA.
6. `Lane B` and `Lane C` run in parallel with disjoint ownership.
7. Parent merges `Lane B`, reruns its acceptance commands, then merges `Lane C`, reruns its acceptance commands, writes `post-bc-freeze.json`, and forks `Lane D`.
8. `Lane D` lands the roadmap and packet-doc closeout last so docs describe the actual landed pilot, not assumptions.
9. Parent merges `Lane D`, writes and validates a runtime `promotion.execution.json` for the monotone-up pilot from merged integration state, then runs the full merged-state verification floor.
10. Parent fast-forwards the publish target only if safe, pushes the exact verified integration SHA, observes CI on that exact SHA, and writes closeout.

## Workstream Plan

### WS-0 Re-anchor on the validated M31 base - parent only

#### `task/m32-00-reanchor`

Required parent actions:

1. Prove that the M32 seed includes the M31 integration boundary.
2. Prefer `ws/m31-int` directly if it contains `945284ea7ab6bf788d7202ff674b81581afd47c6`.
3. If using a merged equivalent, record the exact commit and the proof that it already contains the same M31 contract.
4. Write `m31-base-freeze.json`.

Required commands:

```bash
git rev-parse --verify ws/m31-int
git merge-base --is-ancestor 945284ea7ab6bf788d7202ff674b81581afd47c6 ws/m31-int
git rev-parse ws/m31-int
git show --stat --oneline 945284ea7ab6bf788d7202ff674b81581afd47c6
```

Acceptance:

- `m31-base-freeze.json` exists.
- the chosen M32 integration base is recorded as either the exact anchor commit or a merged equivalent with explicit evidence.
- no code lane may start until this freeze exists.

### WS-1 Baseline capture - parent only

#### `task/m32-01-baseline`

Required parent actions:

1. Confirm the live branch is still `feat/corpus-expansion`.
2. Record the live SHA, dirty state, and overlap with the M32-owned surface.
3. Stop immediately if dirty overlap exists inside the M32-owned surface and no carry-forward decision has been recorded.
4. If unrelated local changes must be preserved, record how they are carried or excluded before seeding worktrees. Do not silently fork from SHA and strand local work.

Required commands:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
git diff --name-only
git diff --name-only --cached
```

Acceptance:

- `baseline.json` exists.
- live branch is `feat/corpus-expansion`.
- dirty overlap is either empty or explicitly blocked.
- the live SHA and carry-forward decision used for orchestration are recorded.

### WS-2 Orchestration freeze - parent only

#### `task/m32-02-authority-freeze`

Required parent actions:

1. Confirm `ORCH_PLAN.md` matches the current M32 authority.
2. Write `authority-freeze.json`.
3. Write `run-id.txt` for the monotone-up pilot family-promotion artifacts.
4. Write `integration-base.txt` from the exact chosen integration base commit recorded in `m31-base-freeze.json`.
5. Write `artifact-paths.json` from that run id.
6. Write `tasks.json`.
7. Create `ws/m32-int` from the commit recorded in `m31-base-freeze.json`.
8. Fork `ws/m32-lane-a-target-language-foundation` from `ws/m32-int`.

Acceptance:

- no worker launches before `authority-freeze.json`.
- `ORCH_PLAN.md`, `authority-freeze.json`, and `tasks.json` agree on lane order, hard guards, publish target, and freeze semantics.
- `run-id.txt`, `integration-base.txt`, and `artifact-paths.json` exist before code work starts.
- after `authority-freeze.json`, `PLAN.md` and `ORCH_PLAN.md` are frozen and may not re-enter runtime scope unless the run is explicitly aborted and replanned.

### WS-3 Target-language artifact foundation - parent only

#### `task/m32-a-target-language-foundation` on `ws/m32-lane-a-target-language-foundation`

Parent mission:

- make `xtask` prove, certify, report, and promotion-artifact truth target-language-aware without widening the milestone beyond the monotone-up pilot.

Parent-owned paths:

- `xtask/src/lib.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/report.rs`
- `xtask/src/family/promotion_artifacts.rs`

Allowed mechanical spillover only if compile-forced:

- `xtask/src/family/paths.rs`
- `xtask/src/family/routing.rs`

Required acceptance commands:

```bash
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/prove.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/certification.report.json
ATTEMPT_PATH=$(ls -t .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/attempt-*.json | head -n 1)
test -n "$ATTEMPT_PATH"
cargo xtask family validate-artifact "$ATTEMPT_PATH"
cargo test -p xtask family_prove_ -- --color never
cargo test -p xtask family_certify_ -- --color never
cargo test -p xtask artifact_schema_ -- --color never
```

`Lane A` must deliver before any worker launch:

- prove and certify artifacts encode the actual target language for the lane that ran.
- Rust-default behavior remains truthful when the flag is omitted.
- artifact validation accepts the new truthful target-language shape.
- one monotone-up recommendation artifact path exists at `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json`, is refreshed by a parent-usable command, and validates truthfully for the M32 pilot.
- one parent-usable runtime artifact emission contract exists for:
  - green-path `promotion.execution.json`
  - blocked-path `blocker.report.json`
- `Lane A` is not done until the parent has exercised the new recommendation-refresh command plus both runtime artifact-emission commands in-lane and frozen their exact argv in `lane-a-freeze.json`.
- `lane-a-freeze.json` records the exact parent commands that refresh the recommendation artifact and generate those artifacts from merged integration state.
- no broad TypeScript support claim is introduced.
- the foundation freeze records the command order. The canonical order is:
  - Rust prove
  - Rust certify
  - TypeScript prove
  - TypeScript certify

### WS-4 Parent merge and post-foundation freeze - parent only

#### `task/m32-03-freeze-post-lane-a`

Strict merge order:

1. merge `ws/m32-lane-a-target-language-foundation` into `ws/m32-int`
2. rerun all `Lane A` acceptance commands from merged state
3. write `lane-a-freeze.json`
4. write `lane-b-launch.md` and `lane-c-launch.md`
5. fork `ws/m32-lane-b-monotone-up-packet-harness` and `ws/m32-lane-c-read-side-truth` from the recorded frozen SHA

Parent may resolve only:

- straightforward import ordering
- mechanical context drift
- compile-local visibility adjustments that do not change the frozen artifact/report contract

Parent must bounce work back to the owning lane for:

- any unfinished target-language artifact schema
- any attempt to move CLI target-language parsing out of `xtask/src/lib.rs`
- any widening beyond the single monotone-up pilot contract
- any attempt to hide actual target-language truth in derived artifacts

Acceptance:

- `Lane A` is merged and re-verified from integration state.
- `lane-a-freeze.json` exists.
- `lane-b-launch.md` and `lane-c-launch.md` exist.
- `Lane B` and `Lane C` both start from the same frozen SHA.
- `lane-a-freeze.json` contains the exact recommendation-refresh contract and artifact-emission contract the parent must use later for `recommendation.latest.json`, `promotion.execution.json`, and `blocker.report.json`.

### WS-5 Parallel post-foundation lanes - workers, concurrency cap 2

#### `task/m32-b-monotone-up-packet-harness` on `ws/m32-lane-b-monotone-up-packet-harness`

Worker mission:

- lock the committed monotone-up packet and harness to the frozen M32 pilot contract without turning wrapper pipeline into a second certification target.

Owned paths:

- `xtask/src/family/harness.rs`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Worker must not do:

- edit `xtask/src/lib.rs`
- edit `xtask/src/family/prove.rs`
- edit `xtask/src/family/certify.rs`
- edit `xtask/src/family/report.rs`
- edit `xtask/src/family/promotion_artifacts.rs`
- edit `semantic-families/README.md`
- edit `docs/**`
- edit `PLAN.md`
- edit `ORCH_PLAN.md`

Required acceptance commands:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test -p xtask monotone_up_ -- --color never
```

`Lane B` must deliver:

- `family smoke` still enforces the committed monotone-up scaffold exactly.
- additive `body.typescript` remains committed packet truth in every bucket it already occupies.
- monotone-up harness suite ownership remains locked.
- no new family is introduced.
- wrapper pipeline remains comparator pressure only.

#### `task/m32-c-read-side-truth` on `ws/m32-lane-c-read-side-truth`

Worker mission:

- make semantic review, passport, export, status, and CLI regressions tell one aligned bounded M32 story for the monotone-up pilot without implying broad TypeScript support.

Owned paths:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`

Allowed mechanical spillover only if compile- or fixture-forced:

- `spec-core/src/types.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/validator.rs`

Worker must not do:

- edit any `xtask/src/**` file
- edit `semantic-families/**`
- edit `docs/**`
- edit `PLAN.md`
- edit `ORCH_PLAN.md`

Required acceptance commands:

```bash
cargo test -p spec-core monotone_up_ -- --color never
cargo test -p spec-core wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli monotone_up_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_ -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce --format json
```

`Lane C` must deliver:

- semantic review still cites authored `body.typescript` truthfully for the monotone-up pilot.
- passport, export, and status agree on the same pilot fixtures.
- no read-side surface implies repo-wide TypeScript support.
- target-specific residue stays visible if it exists.
- wrapper-pipeline regression surfaces stay green as comparator pressure.

### WS-6 Parent merge of parallel lanes and post-BC freeze - parent only

#### `task/m32-04-freeze-post-bc`

Strict merge order:

1. merge `ws/m32-lane-b-monotone-up-packet-harness` into `ws/m32-int`
2. rerun `Lane B` acceptance commands from merged state
3. merge `ws/m32-lane-c-read-side-truth` into `ws/m32-int`
4. rerun `Lane C` acceptance commands from merged state
5. if merge fallout appears, resolve only syntax-level or context-level drift and record it in `merge-log.md`
6. write `post-bc-freeze.json`
7. write `lane-d-launch.md`
8. fork `ws/m32-lane-d-docs-closeout` from the recorded frozen SHA

Parent must bounce work back to the owning lane for:

- disagreement between packet/harness contract and read-side truth surfaces
- any attempt by `Lane C` to redefine the frozen `Lane A` artifact contract
- any attempt by `Lane B` to broaden the pilot beyond monotone-up
- any wording that would make wrapper pipeline look like a second M32 certify target
- any post-`lane-a-freeze.json` lane failure that has not yet been recorded in a validated `blocker.report.json`

Acceptance:

- `Lane B` and `Lane C` are merged and re-verified from integration state.
- `merge-log.md` records merge SHAs, conflicts, and stale-lane decisions.
- `post-bc-freeze.json` exists.
- if any required `Lane B` or `Lane C` merge-time acceptance fails after `lane-a-freeze.json`, the parent emits and validates `blocker.report.json` before stop.

### WS-7 Roadmap and packet-doc closeout - worker

#### `task/m32-d-docs-closeout` on `ws/m32-lane-d-docs-closeout`

Worker mission:

- rewrite public wording last so the roadmap and semantic-family docs describe the actual landed M32 pilot exactly and do not overclaim broad TypeScript support.

Owned paths:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `semantic-families/README.md`

Worker must not do:

- edit any code file
- edit `PLAN.md`
- edit `ORCH_PLAN.md`

Required acceptance commands:

```bash
rg -n "M31|M32|function.arithmetic_leaf.monotone_up.v1|function.wrapper.pipeline.v1|TypeScript|typescript" docs/ai_promotion_and_multilanguage_milestones_v0.1.md semantic-families/README.md PLAN.md
! rg -n "kind:data|kind:sum|repo-wide TypeScript support|broad TypeScript support|all families now support TypeScript" docs/ai_promotion_and_multilanguage_milestones_v0.1.md semantic-families/README.md
```

`Lane D` must deliver:

- roadmap text says `M31` then `M32`.
- roadmap text describes one bounded monotone-up second-language path.
- packet docs describe monotone-up as the M32 pilot family.
- wrapper pipeline remains documented as regression pressure only.
- if `Lane D` acceptance fails after `lane-a-freeze.json`, the parent emits and validates `blocker.report.json` before stop.

### WS-8 Runtime promotion artifact emission - parent only

#### `task/m32-05-promotion-execution-artifact`

Parent mission:

- write one runtime-generated `promotion.execution.json` for the monotone-up pilot from merged `ws/m32-int` state before final verification is allowed to start.

Required parent actions:

1. Read `run-id.txt` and `artifact-paths.json`.
2. Read the exact recommendation-refresh invocation and green-path artifact-emission invocation recorded in `lane-a-freeze.json`.
3. Run the full monotone-up proof loop on merged integration state in the frozen order:
   - Rust prove
   - Rust certify
   - TypeScript prove
   - TypeScript certify
4. Refresh:
   - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json`
5. Generate:
   - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/promotion.execution.json`
6. Validate the generated artifacts with:
   - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json`
   - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/promotion.execution.json`
7. Write `promotion-execution-record.json`.

Required commands:

```bash
RUN_ID=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/run-id.txt)
RECOMMENDATION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json"
PROMOTION_EXECUTION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/${RUN_ID}/promotion.execution.json"
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
test -f "$RECOMMENDATION_PATH"
# Run the exact recommendation-refresh invocation frozen in lane-a-freeze.json.
# Run the exact recommendation validation command frozen in lane-a-freeze.json.
# Run the exact green-path artifact-emission invocation frozen in lane-a-freeze.json.
test -f "$PROMOTION_EXECUTION_PATH"
cargo xtask family validate-artifact "$RECOMMENDATION_PATH"
cargo xtask family validate-artifact "$PROMOTION_EXECUTION_PATH"
```

Acceptance:

- monotone-up `recommendation.latest.json` exists at the frozen family-promotion path.
- `cargo xtask family validate-artifact <recommendation-path>` passes on that artifact.
- `promotion.execution.json` exists at the frozen M32 family-promotion path.
- `cargo xtask family validate-artifact <path>` passes on that artifact.
- `promotion-execution-record.json` records the recommendation path, artifact path, run id, and proof artifacts referenced.

### WS-9 Final verification - parent only

#### `task/m32-06-final-verify`

The parent must run this exact merged-state verification sequence from `ws/m32-int` before calling M32 done:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/prove.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/certification.report.json
ATTEMPT_PATH=$(ls -t .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/attempt-*.json | head -n 1)
test -n "$ATTEMPT_PATH"
cargo xtask family validate-artifact "$ATTEMPT_PATH"
RUN_ID=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/run-id.txt)
RECOMMENDATION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json"
PROMOTION_EXECUTION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/${RUN_ID}/promotion.execution.json"
test -f "$RECOMMENDATION_PATH"
test -f "$PROMOTION_EXECUTION_PATH"
cargo xtask family validate-artifact "$RECOMMENDATION_PATH"
cargo xtask family validate-artifact "$PROMOTION_EXECUTION_PATH"
cargo test -p xtask family_prove_ -- --color never
cargo test -p xtask family_certify_ -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask monotone_up_ -- --color never
cargo test -p spec-core monotone_up_ -- --color never
cargo test -p spec-core wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli monotone_up_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_ -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce --format json
rg -n "M31|M32|function.arithmetic_leaf.monotone_up.v1|function.wrapper.pipeline.v1" docs/ai_promotion_and_multilanguage_milestones_v0.1.md semantic-families/README.md PLAN.md
! rg -n "kind:data|kind:sum|repo-wide TypeScript support|broad TypeScript support|all families now support TypeScript" docs/ai_promotion_and_multilanguage_milestones_v0.1.md semantic-families/README.md
INTEGRATION_BASE_SHA=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m32_one_bounded_second_language_promotion/integration-base.txt)
git diff --name-only "${INTEGRATION_BASE_SHA}...HEAD"
! git diff --name-only "${INTEGRATION_BASE_SHA}...HEAD" | rg -v '^(xtask/src/(lib|family/(prove|certify|report|promotion_artifacts|harness|paths|routing))\.rs|spec-core/src/(semantic_review|passport|export|types|lib|validator)\.rs|spec-cli/src/commands\.rs|spec-cli/tests/(cli|m14_regressions)\.rs|semantic-families/function\.arithmetic_leaf\.monotone_up\.v1/.*|semantic-families/README\.md|docs/ai_promotion_and_multilanguage_milestones_v0\.1\.md)$'
cargo test
```

Rules:

- record every actual command and exit code in `proof-log.json`
- do not substitute broader or different commands for the sequence above
- if any command fails after `lane-a-freeze.json` exists, the parent must also emit and validate a runtime `blocker.report.json` at the frozen family-promotion path before stopping
- M32 is not done if the full floor passes but the diff escapes the closed implementation surface

### WS-10 Publish and CI observation - parent only

#### `task/m32-07-push-observe`

Required parent actions:

1. Confirm the verified `ws/m32-int` commit can fast-forward the publish target branch `feat/corpus-expansion` without discarding or rewriting unrelated work that appeared after baseline.
2. If and only if that fast-forward is safe, update the publish target to the exact verified integration SHA.
3. Push `feat/corpus-expansion`.
4. Record remote, branch, SHA, and timestamp in `push-record.json`.
5. Observe the CI run triggered by that exact pushed SHA.
6. Record workflow name, run id or URL, observed SHA, and workspace result in `ci-observation.json`.

Acceptance:

- publish branch is the exact verified SHA from `ws/m32-int`
- push succeeded
- CI ran on the exact pushed SHA
- workspace CI is green

### WS-11 Closeout - parent only

#### `task/m32-08-closeout`

Closeout must write `closeout.md` and answer plainly:

1. Did M32 prove one bounded second-language promotion path for `function.arithmetic_leaf.monotone_up.v1` and nothing broader?
2. Did Rust-default prove/certify remain green while the TypeScript prove/certify lane also went green?
3. Do prove/certify reports and promotion artifacts now identify actual target language truthfully?
4. Did the promotion chain refresh and validate a monotone-up recommendation artifact before emitting closeout artifacts?
5. Do semantic review, passport, export, and status now tell one aligned bounded story about the same pilot?
6. Did wrapper pipeline remain regression pressure only?
7. Was `promotion.execution.json` generated, validated, and preserved at the frozen M32 family-promotion path?
8. If the run stopped early at any point after `lane-a-freeze.json`, was `blocker.report.json` generated and validated before stop?
9. What remained genuinely shared versus target-specific after the pilot?
10. Did any part of the run need seam-kind widening, a second pilot family, or a broad TypeScript support claim?

Allowed closeout verdicts:

- `EXPAND`
  - M32 landed cleanly and the repo is ready for the next bounded second-language follow-on milestone
- `NARROW`
  - the monotone-up pilot landed, but one bounded truth-surface or artifact follow-on still has to close before the next milestone is honest
- `STOP`
  - the run required scope widening, failed the verification floor, or left the repo overclaiming what TypeScript support means

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
- publish decisions
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

After `authority-freeze.json`, `PLAN.md` and `ORCH_PLAN.md` are read only as frozen authority snapshots. The parent does not mutate them or rely on post-freeze edits during the active run.

Parent-owned information that is offloaded to run-state and must be read from files rather than reconstructed from chat:

- M31 base proof
- baseline capture
- lane launch packets
- merge decisions and outcomes
- stale-lane invalidation history
- final verification logs
- push records
- CI observations
- closeout evidence

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen implementation beyond the M32 closed surface
- they need to change the frozen `Lane A` artifact contract after `lane-a-freeze.json`
- they cannot satisfy acceptance commands with concrete evidence
- they discover overlapping external edits inside their owned surface after launch
- they discover a requirement for seam-kind or molecule-test target-language execution

Worker blocker response:

- stop work
- report the smallest blocking fact with evidence
- do not write or mutate any file under `RUN_ROOT`

Parent blocker response:

- write the sentinel terminal blocked state for the blocked task
- write `blocked.json`
- if `lane-a-freeze.json` exists, the frozen blocked-path invocation must be a real parent-usable command surface added by `Lane A`; schema support alone is not enough.
- if `lane-a-freeze.json` exists, read `run-id.txt`, `artifact-paths.json`, and the blocked-path artifact-emission invocation frozen in `lane-a-freeze.json`, then generate:
  - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`
- validate the blocker artifact with:
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`
- stop downstream launches
- stop publish and closeout
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
  - inferred milestone scope beyond M32
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.
- Worker prompts must include only:
  - owned paths
  - forbidden paths
  - exact authority excerpts
  - exact acceptance commands
  - applicable freeze record path
  - frozen launch SHA
  - applicable hard guards

## Freeze Checkpoints

M32 does not use M26-style human approval gates.

It uses these parent-owned checkpoints instead:

### Checkpoint 0: M31 base freeze

Required:

- `m31-base-freeze.json` exists
- the chosen M32 seed proves inclusion of `945284ea7ab6bf788d7202ff674b81581afd47c6` or an explicitly recorded merged equivalent

### Checkpoint 1: Baseline freeze

Required:

- `baseline.json` exists
- live branch is `feat/corpus-expansion`
- dirty overlap inside the M32-owned surface is either absent or explicitly blocked

### Checkpoint 2: Post-`Lane A` freeze

Required:

- `Lane A` is merged and re-verified on `ws/m32-int`
- `lane-a-freeze.json` exists
- the frozen artifact/report contract is recorded
- `lane-b-launch.md` and `lane-c-launch.md` exist and point at the same frozen SHA

### Checkpoint 3: Post-BC freeze

Required:

- `Lane B` and `Lane C` acceptance commands pass on merged integration state
- `post-bc-freeze.json` exists
- `Lane D` forks from the exact post-BC frozen SHA

### Checkpoint 4: Promotion execution artifact

Required:

- monotone-up `recommendation.latest.json` exists at the frozen family-promotion path
- `cargo xtask family validate-artifact <recommendation-path>` passes on it
- `promotion.execution.json` exists at the frozen family-promotion path for the monotone-up pilot
- `cargo xtask family validate-artifact <path>` passes on it
- `promotion-execution-record.json` exists

### Checkpoint 5: Final verification

Required:

- the exact merged-state verification sequence passes
- `proof-log.json` records every command and exit code
- the final merged diff stays inside the M32 closed surface plus allowed mechanical spillover

## Tests And Acceptance

The required floor is locked:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test -p xtask monotone_up_ -- --color never
cargo test -p spec-core monotone_up_ -- --color never
cargo test -p spec-core wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli monotone_up_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_ -- --color never
cargo test
```

Additional acceptance rules:

- if Rust prove/certify passes but TypeScript prove/certify does not, M32 is incomplete
- if TypeScript passes but artifacts still say `rust`, M32 is incomplete
- if the promotion chain still points at a stale recommendation artifact that does not describe the monotone-up pilot, M32 is incomplete
- if `promotion.execution.json` is missing, unvalidated, or does not reference real monotone-up proof artifacts, M32 is incomplete
- if a required post-foundation failure path does not produce a validated `blocker.report.json`, the run is blocked and incomplete
- if read-side surfaces pass while docs overclaim broad TypeScript support, M32 is incomplete
- if wrapper-pipeline regressions fail while monotone-up is green, M32 is incomplete
- if a second primary family appears anywhere in accepted scope, M32 is blocked

## Assumptions

- `feat/corpus-expansion` remains the publish target branch for this run.
- `ws/m31-int` or its merged equivalent remains available locally when the run starts.
- The existing monotone-up packet already contains the additive TypeScript fixture surface M32 needs.
- `cargo xtask family validate-artifact` remains the stable artifact-truth validator during this run.
- The repo can prove the bounded second-language pilot without reopening seam portability or molecule-test execution policy.

## Freeze And Restart Rules

- No lane launches before the parent writes `authority-freeze.json`.
- `Lane B` and `Lane C` may launch only after `lane-a-freeze.json` exists.
- `Lane D` may launch only after `post-bc-freeze.json` exists.
- If the chosen M31 base changes after `m31-base-freeze.json`, every downstream lane is stale and must be recreated from the new base.
- If `Lane A` changes any frozen artifact or report truth after `Lane B` or `Lane C` is forked, both lanes are stale and must be recreated from the new `lane-a-freeze.json`.
- If `Lane B` changes the monotone-up harness slug, suite names, or committed packet paths that `Lane C` acceptance depends on, `Lane C` is stale and must be recreated from the newest freeze.
- If `Lane C` changes read-side truth vocabulary after `Lane D` is forked, `Lane D` is stale and must be recreated from the newest freeze.
- If overlapping third-party edits land anywhere inside a lane-owned surface after launch, the parent records the overlap, invalidates the affected lanes, and relaunches from the newest relevant freeze.
- The parent does not hand-patch stale worker branches.
- Any request to widen M32 into seam kinds, molecule-test target-language execution, a second pilot family, or broad TypeScript support blocks the run until `PLAN.md` is rewritten.
