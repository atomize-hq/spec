# M53 Shared-Core Portability Adoption Closeout Orchestration Plan

Status: **authoritative execution plan**  
Supersedes: **the stale M52 `ORCH_PLAN.md`**  
Authority source: **`PLAN.md`**  
Plan title: **`M53: Shared-Core Portability Adoption Closeout Implementation Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Primary execution branch: **`feat/m40-plus`**  
Authority date: **`2026-05-12`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum concurrency after contract freeze: **2 lanes total, but only 1 true code lane; the 2nd lane is conditional docs-only**  
Last rewritten: **`2026-05-12`**

## Summary

Execute from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`.

`PLAN.md` is the only scope authority. This file is the operator runbook derived from it.

M53 is a closeout milestone, not a new semantic-family milestone. The semantic owner surface is already frozen in `xtask/src/family/analysis_core/*`. The remaining work is bounded to command-facing presentation, compatibility-shim honesty, proof tightening, and conditional maintainer wording sync.

The exact milestone is:

1. make `xtask/src/family/mod.rs` present `analysis_core` as the maintained owner surface
2. keep `xtask/src/family/decision_kernel.rs` and `xtask/src/family/helper_surface.rs` compatibility-only passthroughs, or remove them with proof updated in the same change
3. make `xtask/src/lib.rs` prove owner-surface behavior and retained compatibility-surface behavior separately
4. touch maintainer docs only if the final code diff would otherwise make them false
5. rerun the locked proof floor without changing stop-state semantics, CLI shape, JSON contracts, or artifact paths

This plan does not authorize new semantics, new consumers, consumer rewires, schema churn, broader shared-core extraction, or opportunistic cleanup outside the named files.

## Hard Guards

- `PLAN.md` is the sole scope authority.
- `xtask/src/family/analysis_core/*` remains the only semantic owner surface.
- `xtask/src/family/recommend.rs`, `xtask/src/family/verify.rs`, `xtask/src/family/promotion_artifacts.rs`, and `xtask/src/family/paths.rs` are blocker-only escape hatches, not planned write targets.
- `xtask/src/family/mod.rs`, `xtask/src/family/decision_kernel.rs`, `xtask/src/family/helper_surface.rs`, and `xtask/src/lib.rs` stay in one code lane. Do not split them across multiple code worktrees.
- Maintainer docs may be edited only if the parent review after `WS-CLOSEOUT` determines the landed code diff would otherwise make them false.
- Acceptance cargo commands run serially only. Do not run the locked proof floor in parallel across worktrees.
- The final proof floor commands are frozen:
  - `./.agents/skills/next-milestone/scripts/collect_signals.sh`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo test -p xtask`
- Final stop-state semantics must remain exactly:
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- `overall_verdict = "pass"` must remain true for `verify-decision-contract`.
- Do not revert or silently clean other people’s edits.

Stop and re-scope if any of these become necessary:

1. a retained consumer outside the named shim surfaces still depends on the old export topology in a way that forces broader module surgery
2. command behavior, flags, JSON output, latest-artifact paths, or write behavior would need to change
3. docs require broad historical cleanup rather than narrow wording sync
4. semantic logic would need to move into or out of `analysis_core/*`
5. a new crate, new facade layer, or broader shared-core extraction is proposed
6. the stop-state tuple changes away from `stop / no_actionable_candidate / record_stop_without_new_milestone`

## Current Code Truth And Rationale

Current code truth from the live authority docs and named repo surfaces:

- `xtask/src/family/mod.rs` already exports `analysis_core`, `decision_kernel`, and `helper_surface`.
- `xtask/src/family/mod.rs` already labels `decision_kernel` and `helper_surface` as compatibility-only passthroughs.
- `xtask/src/family/decision_kernel.rs` is already a pure passthrough re-export of `crate::family::analysis_core::decision_contract`.
- `xtask/src/family/helper_surface.rs` is already a pure passthrough re-export of `crate::family::analysis_core::helper_surface`.
- `xtask/src/family/recommend.rs`, `xtask/src/family/verify.rs`, and `xtask/src/family/promotion_artifacts.rs` already import `analysis_core` directly and are not planned write targets.
- `xtask/src/lib.rs` already exercises owner-surface behavior heavily through `family::analysis_core::*`.
- `xtask/src/lib.rs` still needs explicit proof that any retained compatibility surface is intentional and drift-detecting.
- `docs/semantic_family_capability_corpus_guide_v0.1.md` and `docs/recommendation_corpus_expansion_program_v0.1.md` already describe `analysis_core/*` as the owner surface, but must not drift back into dual-ownership wording.

Rationale for orchestration shape:

- There is only one true code lane. `mod.rs`, the two shim files, and `xtask/src/lib.rs` define one tightly coupled ownership-and-proof contract.
- The only safe optional parallelism is a docs-only lane after the parent reviews the settled code contract.
- Final proof must stay parent-owned and serial because the proof floor is both behavior-sensitive and build-lock-sensitive.

## Canonical Run Roots

Use these exact paths:

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m53
RUN_ROOT=$PRIMARY_ROOT/.runs/m53_shared_core_portability_closeout
```

All authoritative orchestration state lives under `RUN_ROOT`.

## Parallelization Contract

Before any worker starts, execution is strictly sequential.

After contract freeze:

- only `WS-CLOSEOUT` may start immediately
- `WS-DOC-SYNC` may start only after the parent reviews the settled code contract and explicitly marks docs as required
- `WS-DOC-SYNC` is the only allowed overlap with `WS-CLOSEOUT`, and only because it owns docs only
- `WS-INT` starts only after worker submissions are complete
- the locked proof floor never runs in parallel across worktrees

This milestone has at most one true code worker. Any plan that pretends otherwise is widening scope dishonestly.

## Orchestration State

### File-Of-Record Inventory

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, and baseline-proof metadata | Parent |
| `contract-freeze.json` | frozen M53 contract and no-drift invariants | Parent |
| `worktrees.json` | worktree and branch inventory | Parent |
| `file-ownership.json` | writable surfaces per lane | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | dependency queue and task state | Parent |
| `blocked.json` | blocker capture with file, command, scope leak, and stop reason | Parent |
| `session-log.md` | chronological operator log | Parent |
| `acceptance-ledger.md` | final evidence and signoff ledger | Parent |
| `validation/kickoff/*` | kickoff and branch-state captures | Parent |
| `validation/baseline/*` | baseline proof floor captures | Parent |
| `validation/closeout/*` | code-lane submission captures | Parent |
| `validation/docs/*` | docs-lane review and submission captures | Parent |
| `validation/int/*` | merge and integration captures | Parent |
| `validation/final/*` | final proof floor and signoff captures | Parent |

### Per-Task Sentinel Convention

Every task in the task graph has a sentinel directory under `RUN_ROOT/tasks/<TASK_ID>/`.

Required sentinel files:

- `status.json`
- `owner.txt`
- `branch.txt`
- `started_at.txt`
- `finished_at.txt`
- `commands.txt`
- `changed_files.txt`
- `acceptance.md`
- `blocker.md`

Rules:

- `status.json` is the machine-readable source for task state.
- `commands.txt` records exact commands run and observed exit codes.
- `changed_files.txt` is required for worker tasks even when empty.
- `blocker.md` is empty unless the task is blocked.
- parent updates sentinels when tasks transition state.

### Source-Of-Truth Rule

The parent uses:

- `queue.json`
- `tasks.json`
- `session-log.md`
- per-task sentinels under `RUN_ROOT/tasks/`

as the source of truth for execution state.

Chat history is not the run ledger.

`.runs/m53_shared_core_portability_closeout/*` is run-state only. It is not authored product source and must not be treated as a substitute for repo files.

### Task Status Vocabulary

Use only these statuses:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `skipped`
- `cancelled`

## Kickoff Rule

Kickoff requires a clean execution posture, not a forced-clean tree.

The live workspace is expected to already contain authority-doc edits. In particular, `PLAN.md` is already modified in the live workspace and `ORCH_PLAN.md` may also be in-flight during orchestration authoring.

Required kickoff commands:

```bash
mkdir -p "$RUN_ROOT"/validation/{kickoff,baseline,closeout,docs,int,final}
mkdir -p "$RUN_ROOT"/tasks

git branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Kickoff acceptance:

- branch is `feat/m40-plus`
- authority snapshots are captured before code execution begins
- dirty tracked files are limited to expected authority-doc edits:
  - `PLAN.md`
  - `ORCH_PLAN.md`
- run-root creation is allowed as new untracked execution state
- any other dirty tracked repo file is a stop condition
- no one silently cleans, stashes, reverts, or rewrites the tree to satisfy kickoff

If kickoff fails these conditions, record the blocker under `RUN_ROOT/tasks/M53-00/` and stop for human review.

## Contract Freeze

Purpose: freeze the exact M53 execution contract before workers start.

The freeze must record:

- `analysis_core/*` is the only semantic owner surface
- `decision_kernel.rs` and `helper_surface.rs` are compatibility-only passthroughs or are removed with same-change proof
- `xtask/src/lib.rs` must prove owner-surface and retained compatibility-surface behavior separately
- allowed write scope:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/decision_kernel.rs`
  - `xtask/src/family/helper_surface.rs`
  - `xtask/src/lib.rs`
- conditional write scope only if truth requires it:
  - `docs/semantic_family_capability_corpus_guide_v0.1.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
- blocker-only escape hatches, not planned write scope:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/verify.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
- final proof floor commands
- frozen stop-state tuple
- no CLI shape, JSON contract, artifact-path, or stop-state drift
- serial cargo acceptance rule
- parent-only merge and approval authority

Required freeze outputs:

- `contract-freeze.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`
- `worktrees.json`

Freeze acceptance before `WS-CLOSEOUT` may begin:

- baseline proof floor captures exist for all five locked commands
- `contract-freeze.json` names the exact in-scope and blocker-only files
- `file-ownership.json` matches this runbook exactly
- `tasks.json` contains every task from `M53-00` through `M53-41`
- `queue.json` shows only `M53-03` as the next runnable worker-launch task
- `worktrees.json` reserves only the honest M53 topology:
  - `WS-AUTHORITY`
  - `WS-CLOSEOUT`
  - `WS-INT`
  - `WS-DOC-SYNC` as conditional only
- the parent has explicitly recorded that only one true code lane exists for M53
- no worker has write authority outside its frozen lane
- no worker starts until all of the above are true

## Context-Control Rules

Parent context stays small and authoritative:

- keep only authority docs, run-root state, worker diffs, and proof captures live
- do not keep broad repo history, unrelated docs, or speculative follow-ons in active parent context
- parent decisions are limited to scope enforcement, docs-needed judgment, merge order, blocker recording, and final acceptance

Worker prompts stay narrow:

- `WS-CLOSEOUT` receives only:
  - frozen contract bullets
  - owned files
  - current code truth relevant to those files
  - exact non-goals
  - submit conditions
  - allowed commands
- `WS-DOC-SYNC` receives only:
  - parent-approved code contract summary
  - owned docs
  - exact wording risks to check
  - explicit instruction not to widen scope or perform historical cleanup

Worker return format is narrow:

- changed files
- commands run
- exit codes
- blockers

Workers do not return broad narratives, redesign proposals, or follow-on planning.

Operational rules:

- workers do not self-expand scope
- workers do not run the authoritative final proof floor
- after a worker lane is merged, close that worker and remove it from active context
- chat history is not state; run-root files are state

## Workstream Topology

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m53-parent-authority` | `WS-AUTHORITY` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | kickoff, baseline capture, contract freeze, worker launch, approvals |
| `lane/m53-closeout` | `WS-CLOSEOUT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m53/closeout` | `ws/m53-closeout` | Worker | export-surface closeout, shim honesty, and proof-wall tightening |
| `lane/m53-doc-sync` | `WS-DOC-SYNC` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m53/doc-sync` | `ws/m53-doc-sync` | Worker | conditional docs-only wording sync |
| `lane/m53-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m53/int` | `ws/m53-int` | Parent | parent-only integration and authoritative final proof |

## File Ownership Per Lane

### `WS-CLOSEOUT`

Owns only:

- `xtask/src/family/mod.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/lib.rs`

Must not edit:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `PLAN.md`
- `ORCH_PLAN.md`

### `WS-DOC-SYNC`

Owns only, and only if opened by parent:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Must not edit:

- anything under `xtask/src/`
- `PLAN.md`
- `ORCH_PLAN.md`

### `WS-AUTHORITY`

Owns only:

- `PLAN.md`
- `ORCH_PLAN.md`
- `RUN_ROOT/**`
- worktree creation and removal
- parent-side approval and blocker recording

### `WS-INT`

Owns only:

- integration worktree state
- merge commits
- conflict resolution that is strictly mechanical
- final proof captures
- `acceptance-ledger.md`
- final task and blocker state under `RUN_ROOT`

## Task Graph

| Task ID | Lane | Description | Depends on | Submit when |
| --- | --- | --- | --- | --- |
| `M53-00` | Parent | kickoff capture | — | kickoff artifacts written and tree state validated |
| `M53-01` | Parent | baseline proof floor capture | `M53-00` | all five baseline command captures written |
| `M53-02` | Parent | contract freeze and ownership freeze | `M53-01` | freeze files written and accepted |
| `M53-03` | Parent | create worktrees and launch code lane | `M53-02` | `WS-CLOSEOUT` and `WS-INT` exist and are logged |
| `M53-10` | `WS-CLOSEOUT` | export-surface closeout in `mod.rs` and shims | `M53-03` | `mod.rs` and shim files reflect owner-vs-compatibility truth without new logic |
| `M53-11` | `WS-CLOSEOUT` | proof-wall tightening in `xtask/src/lib.rs` | `M53-10` | owner and compatibility proof are separate and local `cargo test -p xtask` passes |
| `M53-12` | Parent | code-lane review and docs-needed decision | `M53-11` | parent records `docs_required` or `docs_skipped` |
| `M53-20` | `WS-DOC-SYNC` | conditional maintainer wording sync | `M53-12` | docs, if touched, stay narrow and ownership-accurate |
| `M53-30` | Parent | merge closeout lane into integration tree | `M53-12` | `ws/m53-closeout` merged cleanly into `ws/m53-int` |
| `M53-31` | Parent | merge docs lane or mark skipped | `M53-30`, `M53-20` | docs lane merged, or skip recorded if not needed |
| `M53-40` | Parent | authoritative final proof floor | `M53-31` | locked proof floor passes serially and captures are written |
| `M53-41` | Parent | final acceptance and signoff | `M53-40` | acceptance ledger complete |

## Worktree Setup Commands

Run from `PRIMARY_ROOT` after `M53-02` completes:

```bash
mkdir -p "$WT_ROOT"

git worktree add -b ws/m53-closeout "$WT_ROOT/closeout" feat/m40-plus
git worktree add -b ws/m53-int "$WT_ROOT/int" feat/m40-plus
git worktree list --porcelain | tee "$RUN_ROOT/validation/kickoff/worktrees.porcelain.txt"
```

Create the docs worktree only if `M53-12` marks `docs_required`:

```bash
git worktree add -b ws/m53-doc-sync "$WT_ROOT/doc-sync" feat/m40-plus
git worktree list --porcelain | tee "$RUN_ROOT/validation/docs/worktrees-doc-sync.porcelain.txt"
```

If `M53-12` marks `docs_skipped`, do not create the docs worktree.

## Workstream Plan

### `WS-AUTHORITY` — parent only, sequential

This lane owns setup, baseline truth, freeze, worker launch, and the docs-needed decision. It is sequential and authoritative.

#### Task `M53-00` — kickoff capture

Files owned:

- `RUN_ROOT/**`
- `PLAN.md`
- `ORCH_PLAN.md`

Commands:

```bash
mkdir -p "$RUN_ROOT"/validation/{kickoff,baseline,closeout,docs,int,final}
mkdir -p "$RUN_ROOT"/tasks
git branch --show-current
git rev-parse HEAD
git status --porcelain=v1 -uall
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Acceptance:

- branch is `feat/m40-plus`
- dirty tracked files are limited to `PLAN.md` and `ORCH_PLAN.md`
- run-root exists
- sentinel directory `RUN_ROOT/tasks/M53-00/` is populated
- no silent cleanup occurred

#### Task `M53-01` — baseline proof floor capture

Files owned:

- `RUN_ROOT/validation/baseline/*`
- `RUN_ROOT/tasks/M53-01/*`

Commands:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh | tee "$RUN_ROOT/validation/baseline/collect_signals.txt"
cargo xtask family recommend --format json | tee "$RUN_ROOT/validation/baseline/recommend.json"
cargo xtask family corpus-decision --format json | tee "$RUN_ROOT/validation/baseline/corpus-decision.json"
cargo xtask family verify-decision-contract --format json | tee "$RUN_ROOT/validation/baseline/verify-decision-contract.json"
cargo test -p xtask | tee "$RUN_ROOT/validation/baseline/cargo-test-p-xtask.txt"
```

Acceptance:

- all five captures exist
- baseline stop-state tuple is recorded in `baseline.json`
- `verify-decision-contract` is `pass`
- any failure becomes a blocker and stops execution

#### Task `M53-02` — contract freeze and ownership freeze

Files owned:

- `contract-freeze.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`
- `worktrees.json`
- `RUN_ROOT/tasks/M53-02/*`

Commands:

```bash
rg -n "analysis_core|decision_kernel|helper_surface" xtask/src/family/mod.rs xtask/src/family/decision_kernel.rs xtask/src/family/helper_surface.rs xtask/src/lib.rs
rg -n "analysis_core|decision_kernel|helper_surface" docs/semantic_family_capability_corpus_guide_v0.1.md docs/recommendation_corpus_expansion_program_v0.1.md
```

Acceptance:

- freeze files are written
- allowed, conditional, and blocker-only write scope is recorded exactly
- only one true code lane is recorded
- `queue.json` permits `WS-CLOSEOUT` and nothing else
- no worker starts before this task is marked `done`

#### Task `M53-03` — create worktrees and launch code lane

Files owned:

- `worktrees.json`
- `RUN_ROOT/tasks/M53-03/*`

Commands:

```bash
mkdir -p "$WT_ROOT"
git worktree add -b ws/m53-closeout "$WT_ROOT/closeout" feat/m40-plus
git worktree add -b ws/m53-int "$WT_ROOT/int" feat/m40-plus
git worktree list --porcelain | tee "$RUN_ROOT/validation/kickoff/worktrees.porcelain.txt"
```

Acceptance:

- `WS-CLOSEOUT` exists
- `WS-INT` exists
- `WS-DOC-SYNC` is not yet created
- launch is logged in `session-log.md`

#### Task `M53-12` — code-lane review and docs-needed decision

Files owned:

- `RUN_ROOT/validation/closeout/*`
- `RUN_ROOT/validation/docs/*`
- `RUN_ROOT/tasks/M53-12/*`

Commands:

```bash
git -C "$WT_ROOT/closeout" diff -- xtask/src/family/mod.rs xtask/src/family/decision_kernel.rs xtask/src/family/helper_surface.rs xtask/src/lib.rs
git -C "$WT_ROOT/closeout" status --short
```

Acceptance:

- parent confirms `WS-CLOSEOUT` stayed within file ownership
- parent confirms proof coverage distinguishes owner vs compatibility surfaces
- parent decides one of:
  - `docs_required`
  - `docs_skipped`
- if `docs_skipped`, no docs worktree is created
- decision is recorded in `queue.json`, `tasks.json`, and `RUN_ROOT/tasks/M53-12/acceptance.md`

### `WS-CLOSEOUT` — worker code lane

This is the only true code worker. It covers both export-surface closeout and proof-wall tightening. No other code lane exists for M53.

#### Task `M53-10` — export-surface closeout in `mod.rs` and shims

Files owned:

- `xtask/src/family/mod.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/helper_surface.rs`

Commands:

```bash
git status --short
rg -n "analysis_core|decision_kernel|helper_surface" xtask/src/family/mod.rs xtask/src/family/decision_kernel.rs xtask/src/family/helper_surface.rs
git diff -- xtask/src/family/mod.rs xtask/src/family/decision_kernel.rs xtask/src/family/helper_surface.rs
```

Acceptance:

- `mod.rs` presents `analysis_core` as the maintained owner surface
- retained shim files are visibly compatibility-only
- no new semantic logic is added to shim files
- no file outside lane ownership is changed
- `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`, and `paths.rs` remain untouched unless the worker records a blocker and stops instead of editing them opportunistically

#### Task `M53-11` — proof-wall tightening in `xtask/src/lib.rs`

Files owned:

- `xtask/src/lib.rs`

Commands:

```bash
cargo test -p xtask | tee "$RUN_ROOT/validation/closeout/cargo-test-p-xtask.txt"
git diff -- xtask/src/lib.rs
```

Acceptance:

- `xtask/src/lib.rs` proves owner-surface behavior directly
- `xtask/src/lib.rs` proves retained compatibility-surface behavior directly
- future shim drift would fail loudly through explicit proof, not only through incidental indirect coverage
- local `cargo test -p xtask` passes
- `changed_files.txt` for `M53-11` lists only the allowed code files

Submission rule for `WS-CLOSEOUT`:

- worker returns changed files, commands run, exit codes, and blockers only
- worker does not run the authoritative full proof floor
- worker stops and records blocker state if forbidden files appear necessary

### `WS-DOC-SYNC` — conditional worker docs lane

This lane exists only if the parent decides the two named docs are no longer truthful after reviewing `WS-CLOSEOUT`.

Skip condition:

- if the parent review after `WS-CLOSEOUT` concludes `docs/semantic_family_capability_corpus_guide_v0.1.md` and `docs/recommendation_corpus_expansion_program_v0.1.md` are already truthful, this lane is skipped and no docs worktree is created

#### Task `M53-20` — conditional maintainer wording sync

Files owned:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Commands:

```bash
git status --short
rg -n "analysis_core|decision_kernel|helper_surface|compatibility-only|semantic owner" docs/semantic_family_capability_corpus_guide_v0.1.md docs/recommendation_corpus_expansion_program_v0.1.md
git diff -- docs/semantic_family_capability_corpus_guide_v0.1.md docs/recommendation_corpus_expansion_program_v0.1.md
```

Acceptance:

- docs remain narrow and ownership-accurate
- docs do not introduce new policy
- docs do not introduce new scope
- docs do not perform historical cleanup beyond wording required for current truth
- docs do not imply dual ownership or peer-owner shims
- if those constraints cannot be met, the lane records a blocker and stops

Submission rule for `WS-DOC-SYNC`:

- worker returns changed files, commands run, exit codes, and blockers only
- no cargo proof commands are required for this lane
- close the worker after merge or skip

### `WS-INT` — parent only integration lane

This lane merges approved worker output, runs authoritative proof, and either accepts the milestone or records a blocker and stops.

#### Task `M53-30` — merge closeout lane

Files owned:

- integration worktree state
- `RUN_ROOT/validation/int/*`
- `RUN_ROOT/tasks/M53-30/*`

Commands:

```bash
git -C "$WT_ROOT/int" merge --no-ff ws/m53-closeout
```

Acceptance:

- merge is clean or requires only straightforward merge mechanics
- if conflicts imply scope ambiguity or semantic disagreement, stop and record blocker state
- worker is closed after successful merge

#### Task `M53-31` — merge docs lane if opened, else mark skipped

Files owned:

- integration worktree state
- `RUN_ROOT/tasks/M53-31/*`

Commands if docs lane exists:

```bash
git -C "$WT_ROOT/int" merge --no-ff ws/m53-doc-sync
```

Acceptance:

- docs lane merges cleanly or with only straightforward merge mechanics
- if code truth and doc wording conflict, code truth wins and docs are corrected before final proof
- if docs lane was never opened, mark `M53-20` and `M53-31` as `skipped` and continue

#### Task `M53-40` — authoritative final proof floor

Files owned:

- `RUN_ROOT/validation/final/*`
- `RUN_ROOT/tasks/M53-40/*`

Commands:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh | tee "$RUN_ROOT/validation/final/collect_signals.txt"
cargo xtask family recommend --format json | tee "$RUN_ROOT/validation/final/recommend.json"
cargo xtask family corpus-decision --format json | tee "$RUN_ROOT/validation/final/corpus-decision.json"
cargo xtask family verify-decision-contract --format json | tee "$RUN_ROOT/validation/final/verify-decision-contract.json"
cargo test -p xtask | tee "$RUN_ROOT/validation/final/cargo-test-p-xtask.txt"
```

Acceptance:

- all five final captures exist
- final stop-state tuple matches baseline exactly
- `verify-decision-contract` is `pass`
- if any final proof command is red, record blocker state under `RUN_ROOT/tasks/M53-40/` and `blocked.json`, then stop
- do not keep retrying silently

#### Task `M53-41` — final acceptance and signoff

Files owned:

- `acceptance-ledger.md`
- `session-log.md`
- final task sentinels

Acceptance:

- acceptance ledger cites command outputs, not impressions
- all completed and skipped states are reflected in `tasks.json` and `queue.json`
- merged workers are closed and removed from active context
- milestone is either explicitly accepted or explicitly blocked

## Merge And Integration Rules

- Parent is the sole integrator and sole approval authority.
- Integration sequence is fixed:
  1. merge `WS-CLOSEOUT`
  2. merge `WS-DOC-SYNC` if and only if it was opened
  3. resolve only straightforward merge mechanics
  4. if code truth and doc wording conflict, code truth wins and docs are corrected before final proof
- Workers do not merge themselves.
- If merge conflict resolution would require semantic reinterpretation, new scope, or edits outside lane ownership, stop and record blocker state.
- If final proof floor is red, record blocker state under `RUN_ROOT` and stop. Do not continue retrying silently.

## Tests And Acceptance

### Baseline Truth

Baseline truth is captured by `M53-01`.

Required commands:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

Required baseline assertions:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `decision_basis_code = no_actionable_candidate`
- `required_next_action = record_stop_without_new_milestone`
- `overall_verdict = "pass"`

### Closeout Lane Proof

Closeout lane proof is captured by `M53-11`.

Required lane assertions:

- export-surface closeout and proof-wall tightening are both landed in the same code lane
- proof distinguishes owner-surface behavior from retained compatibility-surface behavior
- future shim drift fails loudly
- `cargo test -p xtask` passes locally in `WS-CLOSEOUT`
- forbidden files remain untouched unless blocker escalation occurred

### Docs Truth

Docs truth is captured by `M53-12` and `M53-20`.

Required assertions:

- if docs are already truthful, docs lane is skipped and no docs worktree is created
- if docs are touched, wording stays narrow
- docs do not introduce new policy
- docs do not introduce new scope
- docs do not perform historical cleanup unrelated to M53
- docs do not reintroduce dual-ownership language

### Final Proof Floor

Final proof floor is captured by `M53-40`.

Required commands:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

Required final assertions:

- all five commands pass
- final stop-state tuple matches baseline exactly
- `overall_verdict = "pass"`
- no CLI shape, JSON contract, artifact-path, or stop-state drift appears

### Write-Scope Enforcement

Write-scope enforcement is checked at `M53-12`, `M53-30`, and `M53-41`.

Required assertions:

- `WS-CLOSEOUT` changed only:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/decision_kernel.rs`
  - `xtask/src/family/helper_surface.rs`
  - `xtask/src/lib.rs`
- `WS-DOC-SYNC` changed only:
  - `docs/semantic_family_capability_corpus_guide_v0.1.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
- any attempted change to:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/verify.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
  is blocker-only and requires parent stop plus human review

### Failure-Mode To Gate Mapping

| Failure mode from `PLAN.md` | Gate that catches it | Required signal |
| --- | --- | --- |
| shim drift from `analysis_core` | `M53-11`, `M53-40` | explicit compatibility-path proof fails loudly |
| `mod.rs` still presenting shims as peer owner surfaces | `M53-10`, `M53-12` | parent review of export presentation fails |
| proof wall failing to distinguish owner vs compatibility surfaces | `M53-11`, `M53-12` | `xtask/src/lib.rs` proof is insufficient or indirect |
| stop-state command outputs drifting | `M53-01`, `M53-40` | baseline vs final tuple mismatch |
| docs silently reintroducing dual-ownership language | `M53-12`, `M53-20`, `M53-31` | parent review or docs-lane review fails |

## Assumptions

- `feat/m40-plus` remains the active execution branch for M53.
- `PLAN.md` remains authoritative for M53 until closeout completes.
- `analysis_core/*` itself is frozen and does not need semantic edits for honest closeout.
- no hidden consumer outside the named shims forces rewrites in `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`, or `paths.rs`
- only one true code lane exists for M53 because the owned files are tightly coupled and define one ownership-and-proof contract
- docs may be skipped entirely if the parent determines the landed code diff does not make them false
- the parent can create worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m53`
- serial execution of the locked proof floor is preferred because lock contention matters more than theoretical parallel wall-clock savings here

## Blocker Handling

If a blocker appears:

- capture it under `RUN_ROOT/validation/*`
- record the exact file, command, scope leak, and stop reason in `blocked.json`
- update the affected task sentinel under `RUN_ROOT/tasks/<TASK_ID>/blocker.md`
- set the affected task to `blocked` in `tasks.json` and `queue.json`
- stop instead of widening scope silently

Mandatory escalation cases:

- dirty tracked files outside expected authority-doc edits at kickoff
- any claimed need to edit `xtask/src/family/recommend.rs`, `xtask/src/family/verify.rs`, `xtask/src/family/promotion_artifacts.rs`, or `xtask/src/family/paths.rs`
- any proposed semantic move into or out of `analysis_core/*`
- any merge conflict that is not straightforward mechanics
- any final proof-floor failure

Widened scope in forbidden files requires parent stop plus human review. Workers do not have discretion to spend that scope.

## Done Definition

M53 is done only when:

1. parent-owned sequential phases completed before any worker began
2. the single code lane stayed within its four-file boundary
3. the docs lane was either honestly skipped or stayed within its two-file boundary
4. integration was parent-owned
5. the final locked proof floor passed serially
6. the acceptance ledger records unchanged stop-state semantics and unchanged contract surfaces
7. the repo tells one consistent story: `analysis_core/*` owns the semantics, any retained shim is compatibility glue only, and the family-analysis lane still truthfully says `stop`
