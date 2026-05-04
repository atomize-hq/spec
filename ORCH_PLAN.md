# M31 Orchestration Plan

Status: **authoritative execution contract for the split-worktree M31 run**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Live branch: **`feat/corpus-expansion`**
Review base: **`main`**
Last rewritten: **2026-05-04**
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction`**
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction`**

## Summary

- This run is for **M31 shared-core extraction and escape-hatch containment** only.
- `PLAN.md` remains the milestone authority. This file becomes authoritative only if the parent agent actually chooses the split-worktree path described in `PLAN.md`.
- The parent agent is the sole integrator, sole freeze authority, sole stale-lane invalidator, sole push authority, and sole final verifier.
- Work starts with a parent-owned baseline capture and orchestration freeze.
- The parent agent keeps the critical path local: baseline, authority freeze, `Lane A`, all merge/freeze checkpoints, final verification, push/observe, and closeout are parent-owned.
- `Lane A` is the sequential foundation lane and stays on the parent-critical path. No parallel worker launches before `Lane A` lands and its portability API is frozen.
- After `Lane A` is merged and frozen, exactly two worker lanes may run in parallel:
  - `Lane B` = passport projection plus supported seam semantic-review integration
  - `Lane C` = export/status/CLI truth-surface integration
- After `Lane B` and `Lane C` are merged and re-verified, `Lane D` may run last as one bounded worker lane for validator wording and roadmap closeout.
- Recommended worker profile for `Lane B`, `Lane C`, and `Lane D` is `GPT-5.4` with `reasoning_effort=high`.
- Maximum worker concurrency is `2`.
- The implementation surface stays bounded to the M31 closed surface in `PLAN.md`. No M30 wrapper-family work, no second-language execution semantics, no function portability expansion, and no repo-wide target-language policy widening are allowed.
- Parent-owned run-state under `RUN_ROOT` is the only execution truth. Worker memory and stale worktree copies are not.

## Hard Guards

- `PLAN.md` wins over this document, worker summaries, stale worktree files, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen in `ws/m31-int`.
- The closed implementation surface for M31 is:
  - `spec-core/src/portability.rs`
  - `spec-core/src/backend_execution.rs`
  - `spec-core/src/escape_hatch.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/validator.rs`
  - `spec-core/src/export.rs`
  - `spec-core/src/lib.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- Allowed mechanical spillover is compile- or fixture-forced only:
  - `spec-core/src/types.rs`
  - `spec-core/src/molecule_evidence.rs`
  - `spec-core/src/generator.rs`
  - `spec-core/src/graph.rs`
  - `spec-core/src/schema/unit.spec.json`
- `PLAN.md` is authority-only during execution. It is read for lane prompts and verification, not treated as a worker-owned edit surface.
- Any semantic expansion outside that surface blocks the run until authority is rewritten.
- `spec-core/src/portability.rs` becomes the only allowed composition point for seam portability truth.
- `passport.rs`, `semantic_review.rs`, `export.rs`, and `spec-cli/src/commands.rs` may consume the shared projection, but they may not invent fresh portability logic.
- M31 is seam-only. `kind:function` portability policy, second-language execution truth, packet promotion, and CI lane redesign are out of scope.
- Stop immediately if any lane requires:
  - new portability marker classes
  - portability semantics for `kind:function`
  - TypeScript executable semantic review
  - proof-policy changes beyond the current seam `atom` and `molecule` surfaces
  - a new read-side public JSON field without first proving the existing public fields are insufficient

## Worktree Layout

Canonical worktrees:

- integration
  - branch: `ws/m31-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction/int`
- `Lane A` portability foundation
  - branch: `ws/m31-lane-a-portability-foundation`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction/lane-a-portability-foundation`
- `Lane B` passport + semantic-review integration
  - branch: `ws/m31-lane-b-passport-semantic`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction/lane-b-passport-semantic`
- `Lane C` export + status integration
  - branch: `ws/m31-lane-c-export-status`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction/lane-c-export-status`
- `Lane D` validator + roadmap closeout
  - branch: `ws/m31-lane-d-validator-roadmap`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction/lane-d-validator-roadmap`

Creation rules:

- The parent captures baseline state from the live checkout on `feat/corpus-expansion`, then creates `ws/m31-int` from the exact live SHA recorded in `baseline.json`.
- `Lane A` is forked from `ws/m31-int` after the orchestration freeze is written.
- `Lane B` and `Lane C` are both forked from the exact post-`Lane A` SHA recorded in `lane-a-freeze.json`.
- `Lane D` is forked from the exact post-merge SHA recorded in `post-bc-freeze.json`.
- No worker is forked from another worker branch.
- If any named branch or worktree already exists and points at stale or conflicting state, the parent removes and recreates it before reuse and records that in `session-log.md`.
- A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent does not hand-forward stale worker branches.

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m31-shared-core-extraction`

Canonical parent-owned files:

- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty-state summary
  - overlap check against the M31-owned surface
- `authority-freeze.json`
  - milestone id `M31`
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
  - frozen `portability.rs` API surface
  - frozen allowed helper callsites
  - exact acceptance commands for `Lane B` and `Lane C`
- `lane-b-launch.md`
  - reproducible launch packet for `Lane B`
  - exact `PLAN.md` excerpt references
  - exact `ORCH_PLAN.md` excerpt references
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
- `lane-c-launch.md`
  - reproducible launch packet for `Lane C`
  - exact `PLAN.md` excerpt references
  - exact `ORCH_PLAN.md` excerpt references
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
- `post-bc-freeze.json`
  - exact post-merge commit after `Lane B` and `Lane C`
  - final terminology decisions for `Lane D`
  - exact `Lane D` acceptance commands
- `lane-d-launch.md`
  - reproducible launch packet for `Lane D`
  - exact `PLAN.md` excerpt references
  - exact `ORCH_PLAN.md` excerpt references
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
  - workspace result
- `blocked.json`
  - blocking task
  - blocking evidence
  - required next decision
- `closeout.md`
  - contract summary
  - portability boundary summary
  - truth-surface alignment summary
  - roadmap summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-01-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-a-portability-foundation/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-02-freeze-post-lane-a/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-b-passport-semantic/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-c-export-status/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-03-freeze-post-bc/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-d-validator-roadmap/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-04-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-05-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m31_shared_core_extraction/task-m31-06-closeout/`

Each sentinel directory contains:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m31-00-baseline
  -> task/m31-01-authority-freeze
      -> task/m31-a-portability-foundation
          -> task/m31-02-freeze-post-lane-a
task/m31-02-freeze-post-lane-a
  -> task/m31-b-passport-semantic
  -> task/m31-c-export-status
task/m31-b-passport-semantic
  -> task/m31-03-freeze-post-bc
task/m31-c-export-status
  -> task/m31-03-freeze-post-bc
task/m31-03-freeze-post-bc
  -> task/m31-d-validator-roadmap
      -> task/m31-04-final-verify
          -> task/m31-05-push-observe
              -> task/m31-06-closeout
```

Execution meaning:

1. Parent captures live branch, live SHA, and overlap facts.
2. Parent freezes orchestration authority and creates the integration worktree.
3. `Lane A` lands the portability contract module, helper visibility adjustments, and the frozen API shape required for all downstream consumers.
4. Parent merges `Lane A`, reruns its acceptance commands on merged state, writes `lane-a-freeze.json`, and forks `Lane B` and `Lane C` from that exact frozen SHA.
5. `Lane B` and `Lane C` run in parallel with disjoint ownership.
6. Parent merges `Lane B`, reruns its acceptance commands, then merges `Lane C`, reruns its acceptance commands, and writes `post-bc-freeze.json`.
7. `Lane D` lands validator wording and roadmap sequencing after the read-side contract is stable, using `PLAN.md` as fixed terminology authority.
8. Parent merges `Lane D`, runs the full merged-state verification floor, pushes, observes CI on the exact pushed SHA, and writes closeout.

## Workstream Plan

### WS-0 Baseline capture - parent only

#### `task/m31-00-baseline`

Required parent actions:

1. Confirm the live branch is still `feat/corpus-expansion`.
2. Record the live SHA, dirty state, and overlap with the M31 closed surface.
3. Stop immediately if unresolved dirty overlap exists inside the M31-owned surface.

Acceptance:

- `baseline.json` exists.
- overlap is either empty or explicitly blocked.
- the live SHA used to seed `ws/m31-int` is recorded.

### WS-1 Orchestration freeze - parent only

#### `task/m31-01-authority-freeze`

Required parent actions:

1. Rewrite `ORCH_PLAN.md` to current M31 truth.
2. Write `authority-freeze.json`.
3. Write `tasks.json`.
4. Create `ws/m31-int` from the recorded live SHA.
5. Fork `ws/m31-lane-a-portability-foundation` from `ws/m31-int`.

Acceptance:

- no worker launches before `authority-freeze.json`.
- `ORCH_PLAN.md`, `authority-freeze.json`, and `tasks.json` agree on lane order, hard guards, and freeze semantics.

### WS-2 Portability contract foundation - parent only

#### `task/m31-a-portability-foundation` on `ws/m31-lane-a-portability-foundation`

Parent mission:

- create `spec-core/src/portability.rs` as the sole seam portability composition point, freeze its API, and keep `backend_execution.rs` plus `escape_hatch.rs` as reusable helpers rather than read-side orchestrators.

Parent-owned paths:

- `spec-core/src/portability.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/backend_execution.rs`
- `spec-core/src/escape_hatch.rs`

Allowed mechanical spillover only if compile-forced:

- `spec-core/src/types.rs`

Required acceptance commands:

```bash
cargo test -p spec-core portability -- --color never
cargo test -p spec-core backend_execution -- --color never
cargo test -p spec-core escape_hatch -- --color never
```

Lane A must deliver before any worker launch:

- one canonical portability module with frozen projection types and helper entrypoints.
- helper classification and digest behavior unchanged unless intentionally routed through the new contract.
- enough API stability that `Lane B` and `Lane C` can proceed without further `portability.rs` churn.

### WS-3 Parent merge and post-foundation freeze - parent only

#### `task/m31-02-freeze-post-lane-a`

Strict merge order:

1. merge `ws/m31-lane-a-portability-foundation` into `ws/m31-int`
2. rerun the `Lane A` acceptance commands from merged state
3. write `lane-a-freeze.json`
4. write `lane-b-launch.md` and `lane-c-launch.md`
5. fork `ws/m31-lane-b-passport-semantic` and `ws/m31-lane-c-export-status` from the recorded frozen SHA

Parent may resolve only:

- straightforward import ordering
- mechanical context drift
- compile-local visibility adjustments that do not change the frozen portability API

Parent must bounce work back to the owning lane for:

- any unfinished or unstable `portability.rs` API
- any attempt to let downstream consumers recompose portability truth privately
- any new marker taxonomy or scope expansion beyond seam kinds

Acceptance:

- `Lane A` is merged and re-verified from integration state.
- `lane-a-freeze.json` exists.
- `lane-b-launch.md` and `lane-c-launch.md` exist.
- `Lane B` and `Lane C` both start from the same frozen SHA.

### WS-4 Parallel read-side integration lanes - workers, concurrency cap 2

#### `task/m31-b-passport-semantic` on `ws/m31-lane-b-passport-semantic`

Worker mission:

- make passport projection the reference implementation for seam portability truth and make supported seam semantic review consume that shared contract instead of re-deriving portability privately.

Owned paths:

- `spec-core/src/passport.rs`
- `spec-core/src/semantic_review.rs`

Worker must not do:

- edit `spec-core/src/portability.rs`
- edit `spec-core/src/export.rs`
- edit `spec-core/src/validator.rs`
- edit `spec-cli/src/**`
- edit `docs/**`
- edit `PLAN.md`

Required acceptance commands:

```bash
cargo test -p spec-core passport -- --color never
cargo test -p spec-core semantic_review -- --color never
```

Lane B must deliver:

- passport write and read paths consume one seam portability projection.
- `markers`, `proof_coverage`, `escape_hatch_gate`, and `freshness.backend_execution_status` stay truthful through the shared contract.
- supported seam semantic-review verdicts remain aligned, under-specified, backend-only meaning preserved, or backend-only semantics leaked based on shared portability inputs.
- supported-function and unsupported-function truth remain unchanged outside compile-local fallout.

#### `task/m31-c-export-status` on `ws/m31-lane-c-export-status`

Worker mission:

- make export, status, and CLI health surfaces consume the same shared seam portability truth without taking ownership of passport or semantic-review internals.

Owned paths:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Allowed mechanical spillover only if compile- or fixture-forced:

- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/graph.rs`
- `spec-core/src/schema/unit.spec.json`

Worker must not do:

- edit `spec-core/src/portability.rs`
- edit `spec-core/src/passport.rs`
- edit `spec-core/src/semantic_review.rs`
- edit `spec-core/src/validator.rs`
- edit `docs/**`
- edit `PLAN.md`

Required acceptance commands:

```bash
cargo test -p spec-core export -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce --format json
```

Lane C must deliver:

- export-side passport enrichment routes through the shared projection.
- `spec status` preserves the current demotion sinks while consuming shared truth.
- `valid -> incomplete` demotion remains gate-only; `stale`, `failing`, and `invalid` still win when already present.
- export, passport, and status agree on the same seam fixture set after merge.

### WS-5 Parent merge of parallel lanes and post-BC freeze - parent only

#### `task/m31-03-freeze-post-bc`

Strict merge order:

1. merge `ws/m31-lane-b-passport-semantic` into `ws/m31-int`
2. rerun the `Lane B` acceptance commands from merged state
3. merge `ws/m31-lane-c-export-status` into `ws/m31-int`
4. rerun the `Lane C` acceptance commands from merged state
5. if merge fallout appears, resolve only syntax-level or context-level drift and record it in `merge-log.md`
6. write `post-bc-freeze.json`
7. write `lane-d-launch.md`
8. fork `ws/m31-lane-d-validator-roadmap` from the recorded frozen SHA

Parent must bounce work back to the owning lane for:

- disagreement about the meaning of projected passport truth versus export/status truth
- any attempt by `Lane C` to claim passport or semantic-review ownership
- any portability API changes that should have been frozen in `Lane A`
- any semantic broadening beyond seam portability containment

Acceptance:

- `Lane B` and `Lane C` are merged and re-verified from integration state.
- `merge-log.md` records merge SHAs, conflicts, and stale-lane decisions.
- `post-bc-freeze.json` exists.

### WS-6 Validator wording and roadmap closeout - worker

#### `task/m31-d-validator-roadmap` on `ws/m31-lane-d-validator-roadmap`

Worker mission:

- land validator wording only where clarity is missing and rewrite the roadmap so the public narrative matches the landed M31 contract already locked in `PLAN.md`.

Owned paths:

- `spec-core/src/validator.rs`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Worker must not do:

- edit `spec-core/src/portability.rs`
- edit `spec-core/src/passport.rs`
- edit `spec-core/src/semantic_review.rs`
- edit `spec-core/src/export.rs`
- edit `spec-cli/src/**`
- edit `PLAN.md`

Required acceptance commands:

```bash
cargo test -p spec-core validator -- --color never
rg -n "M31|M32" /Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md /Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md
! rg -n "M28|M29" /Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md /Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md
```

Lane D must deliver:

- validator wording distinguishes invalid authored seam shape from valid-but-contaminating backend-specific detail without pretending validation is the full portability decision engine.
- roadmap text says `M31` then `M32`.
- roadmap terminology aligns to the already-authoritative `PLAN.md` and landed code.

### WS-7 Final verification - parent only

#### `task/m31-04-final-verify`

The parent must run this exact merged-state verification sequence from `ws/m31-int` before calling M31 done:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p spec-core backend_execution -- --color never
cargo test -p spec-core escape_hatch -- --color never
cargo test -p spec-core passport -- --color never
cargo test -p spec-core semantic_review -- --color never
cargo test -p spec-core export -- --color never
cargo test -p spec-core validator -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce --format json
rg -n "M31|M32" docs/ai_promotion_and_multilanguage_milestones_v0.1.md PLAN.md
! rg -n "M28|M29" docs/ai_promotion_and_multilanguage_milestones_v0.1.md PLAN.md
cargo test --workspace
```

Rules:

- record every actual command and exit code in `proof-log.json`
- do not substitute broader or different commands for the sequence above
- if any command fails, stop the run and write `blocked.json`

### WS-8 Push and CI observation - parent only

#### `task/m31-05-push-observe`

Required parent actions:

1. push the final integration branch or designated final candidate branch
2. record remote, branch, SHA, and timestamp in `push-record.json`
3. observe the CI run triggered by that exact pushed SHA
4. record workflow name, run id or URL, observed SHA, and workspace result in `ci-observation.json`

Acceptance:

- push succeeded
- CI ran on the exact pushed SHA
- workspace CI is green

### WS-9 Closeout - parent only

#### `task/m31-06-closeout`

Closeout must write `closeout.md` and answer plainly:

1. Did `spec-core/src/portability.rs` become the sole seam portability composition point?
2. Do passport, semantic review, export, and status now tell one aligned seam portability story?
3. Do stale precedence and open-gate demotion still behave exactly as promised?
4. Does the public roadmap now say `M31` then `M32` and describe the same boundary the code ships?
5. What exact executable-truth question remains deferred to M32?

Allowed closeout verdicts:

- `EXPAND`
  - M31 landed cleanly and the repo is ready to take the next executable portability question as a separate M32 milestone
- `NARROW`
  - the shared-core extraction landed, but one bounded follow-on inside the seam portability story still has to close before M32 is honest
- `STOP`
  - the run needed scope expansion beyond seam containment, broke cross-surface truth alignment, or failed the verification floor

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

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen the implementation surface beyond the M31 closed surface
- they need to change the frozen `portability.rs` API after `Lane A` freeze
- they cannot satisfy acceptance commands with concrete evidence
- they find overlapping external edits inside their owned surface after launch

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
  - inferred milestone scope beyond M31
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.
- Worker prompts must include only:
  - owned paths
  - forbidden paths
  - exact authority excerpts
  - exact acceptance commands
  - applicable freeze record path
  - frozen launch SHA
  - applicable hard guards

## Tests And Acceptance

### Gate 0: baseline gate

Required:

- `baseline.json` exists
- live branch is `feat/corpus-expansion`
- no unresolved dirty overlap exists inside the M31-owned surface

### Gate 1: post-`Lane A` freeze gate

Required:

- `Lane A` is merged and re-verified on `ws/m31-int`
- `lane-a-freeze.json` exists
- the frozen `portability.rs` API is recorded
- `lane-b-launch.md` and `lane-c-launch.md` exist and point at the same frozen SHA
- `Lane B` and `Lane C` both fork from the same recorded SHA

### Gate 2: parallel-lane merge gate

Required:

- `Lane B` acceptance commands pass on merged integration state
- `Lane C` acceptance commands pass on merged integration state
- no downstream consumer reintroduced private portability logic

### Gate 3: post-BC freeze gate

Required:

- `post-bc-freeze.json` exists
- terminology needed by `Lane D` is frozen
- `Lane D` forks from the exact post-BC frozen SHA

### Gate 4: final verification gate

Required:

- the exact merged-state verification sequence passes
- `proof-log.json` records every command and exit code
- the final merged diff stays inside the M31 closed surface plus compile-forced spillover

## Assumptions

- The live branch at kickoff remains `feat/corpus-expansion`.
- No unrelated external edits overlap the M31-owned surface after baseline without forcing a parent re-baseline.
- `Lane A` can freeze a stable portability API that `Lane B` and `Lane C` can consume without reopening ownership.
- Canonical seam fixtures under `examples/ecommerce` remain the main proof bed for status and export checks.
- No new workflow topology is required for M31 beyond observing the normal pushed CI result.

## Approval, Freeze, And Restart Rules

- No worker launches before the parent writes `authority-freeze.json`.
- `Lane B` and `Lane C` may launch only after `lane-a-freeze.json` exists.
- `Lane D` may launch only after `post-bc-freeze.json` exists.
- The parent may resolve only syntax-level or context-level merge drift. Semantic conflicts go back to the owning lane.
- If `Lane A` changes the frozen `portability.rs` API after `Lane B` or `Lane C` is forked, both lanes are stale and must be recreated from the new freeze.
- If `Lane B` changes any contract that `Lane C` was explicitly launched to consume, `Lane C` is stale and must be recreated from the newest freeze.
- If overlapping third-party edits land anywhere inside the closed surface after a lane is forked, the parent records the overlap, invalidates the affected lanes, and relaunches from the newest relevant freeze.
- The parent does not hand-patch stale worker branches.
- Any request to widen scope beyond seam containment, change the closed surface, or redefine the M31/M32 boundary blocks the run until the authority plan is rewritten.
