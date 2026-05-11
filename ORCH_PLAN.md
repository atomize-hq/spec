# M51 Shared-Core Portability Adoption Orchestration Plan

Status: **authoritative orchestration plan for executing M51**  
Supersedes: **the stale M50 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M51: Shared-Core Portability Adoption Implementation Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Primary execution branch: **`feat/m40-plus`**  
Baseline authority commit: **`709a30f`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum concurrency after freeze: **2 workers**  
Last rewritten: **2026-05-11**

## Summary

- Execute from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus` at `709a30f` unless divergence is explicitly revalidated.
- `PLAN.md` is the sole scope authority. This file is the operator contract derived from it.
- M51 is a bounded shared-core portability adoption milestone. It is not a crate extraction, not a CLI redesign, not a corpus-spend milestone, and not a roadmap rewrite.
- Parent-owned pre-parallel work is exactly the ownership-contract freeze in:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/helper_surface.rs`
  - `xtask/src/family/decision_kernel.rs`
- After that freeze gate passes, fork two strict worker lanes:
  - `WS-PROOF-WALL`
  - `WS-DOCS`
- Parent remains the only integrator and the only writer of authoritative orchestration state.
- Final authoritative proof runs only after both worker lanes merge.

## Hard Guards

- `PLAN.md` is the only scope authority for M51. If a requested change is not authorized by `PLAN.md`, stop.
- `xtask/src/family/analysis_core/*` remains the only semantic owner surface.
- `xtask/src/family/helper_surface.rs` and `xtask/src/family/decision_kernel.rs` may remain only as compatibility passthroughs.
- `xtask/src/family/mod.rs` must stop presenting shim topology as peer semantic truth.
- `xtask/src/lib.rs` must stop calling `decision_kernel::corpus_program_basis_snapshot(...)` as the proof-wall source of truth.
- Command names, flags, JSON schemas, artifact locations, and stop-state outputs do not change.
- `recommend` must still report `insufficient_real_corpus`.
- `corpus-decision` must still report `decision_action = "stop"` and `decision_basis_code = "no_actionable_candidate"`.
- `verify-decision-contract` must still pass all five checks.
- Protected read-only reference scope:
  - `xtask/src/family/analysis_core/*`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/verify.rs`
  - `xtask/src/family/promotion_artifacts.rs`
- Abort and re-scope if any of these become necessary:
  1. A public command name, flag, or JSON schema needs to change.
  2. Stop-state outputs change.
  3. Path, write, or artifact policy has to move into `analysis_core`.
  4. A non-test consumer outside named scope still depends on shims as owners.
  5. Docs cleanup expands into repo-root roadmap or plan-authority rewrites.

## Current Code Truth And Rationale

- `xtask/src/family/recommend.rs`, `xtask/src/family/verify.rs`, and `xtask/src/family/promotion_artifacts.rs` already consume `analysis_core` directly.
- `xtask/src/family/helper_surface.rs` is currently a pure re-export shim over `analysis_core::helper_surface`.
- `xtask/src/family/decision_kernel.rs` is currently a pure re-export shim over `analysis_core::decision_contract`.
- `xtask/src/family/mod.rs` still exports `analysis_core`, `decision_kernel`, and `helper_surface` as peer modules.
- `xtask/src/lib.rs` still calls `decision_kernel::corpus_program_basis_snapshot(...)` at two proof-wall sites.
- Command-facing docs still describe `helper_surface.rs` and `decision_kernel.rs` as current semantic owners.
- M51 exists to make code truth, proof truth, and docs truth tell one story without changing behavior.

## Worktree And Branch Topology

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m51-parent-authority` | `WS-AUTHORITY` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | kickoff capture, authority freeze, ownership-contract freeze, worker fork |
| `lane/m51-proof-wall` | `WS-PROOF-WALL` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m51/proof-wall` | `ws/m51-proof-wall` | Worker | rewire `xtask/src/lib.rs` proof wall |
| `lane/m51-docs` | `WS-DOCS` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m51/docs` | `ws/m51-docs` | Worker | update command-facing ownership docs |
| `lane/m51-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m51/int` | `ws/m51-int` | Parent | merge worker lanes and run authoritative proof |
| `lane/m51-parent-closeout` | `finalize` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | fast-forward validated integration result and write acceptance ledger |

## Canonical Run Root

Use these exact paths:

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m51
RUN_ROOT=$PRIMARY_ROOT/.runs/m51_shared_core_portability_adoption
```

All authoritative orchestration state lives under `RUN_ROOT`.

## Orchestration State

### File-Of-Record Inventory

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, and authority snapshot metadata | Parent |
| `authority-freeze.json` | frozen scope, guards, lane map, proof commands, and abort triggers | Parent |
| `contract-freeze.json` | exact Step 1 decisions, freeze commit, worker fork basis, and conditional docs scope | Parent |
| `worktrees.json` | worktree path and branch inventory | Parent |
| `file-ownership.json` | exact writable repo surfaces per lane | Parent |
| `read-only-references.txt` | explicit protected read-only surfaces | Parent |
| `in-scope-files.txt` | exhaustive in-scope files for M51 | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | dependency queue and task state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance-ledger.md` | final acceptance evidence and signoff ledger | Parent |
| `blocked.json` | blocker artifact on incomplete termination | Parent |
| `authority-snapshot/PLAN.md` | kickoff scope snapshot | Parent |
| `authority-snapshot/ORCH_PLAN.md` | kickoff orchestration snapshot | Parent |
| `authority-snapshot/authority-input.diff` | diff of allowed dirty authority inputs | Parent |
| `validation/kickoff/*` | kickoff command captures | Parent |
| `validation/freeze/*` | Step 1 contract-freeze captures | Parent |
| `validation/proof-wall/*` | `WS-PROOF-WALL` captures copied back by parent | Parent |
| `validation/docs/*` | `WS-DOCS` captures copied back by parent | Parent |
| `validation/int/*` | merge and integrated proof captures | Parent |
| `validation/final/*` | final branch and acceptance captures | Parent |
| `validation/blocked/*` | blocked termination captures and failure evidence | Parent |

### Task Status Vocabulary

Use only these statuses in `tasks.json` and `queue.json`:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `cancelled`

### Sentinel Contract

Every task has a sentinel directory under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m51_shared_core_portability_adoption/tasks/<TASK_ID>/`

Each sentinel directory must contain:

| File | Required contents |
| --- | --- |
| `sentinel.json` | `task_id`, `workstream`, `lane`, `owner`, `owned_files`, `status`, `depends_on`, `started_at`, `completed_at`, `blocker_status`, `submission_commit`, `notes_summary` |
| `commands.ndjson` | one record per command with `command`, `cwd`, `started_at`, `completed_at`, `exit_code`, `stdout_path`, `stderr_path` |
| `owned-files.txt` | exact repo paths owned by that task |
| `result.md` | concise parent-authored outcome, blockers, and next action |

Rules:

- Only the parent writes authoritative sentinel files.
- Workers return narrow summaries plus command outputs; the parent records them into `RUN_ROOT`.
- A task is not complete until `sentinel.json` shows `status: done`.
- A blocked task must identify the exact scope leak, proof failure, or ownership ambiguity.

## Context Control

- Parent keeps only a fixed live orchestration set in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `contract-freeze.json`
  - `file-ownership.json`
  - `tasks.json`
  - `queue.json`
  - narrow validation captures relevant to the active step
- Workers receive only:
  - owned files
  - relevant `PLAN.md` excerpts
  - exact commands
  - exact acceptance criteria
  - exact forbidden-touch surfaces
- Parent reviews narrow diffs and worker summaries only, not full transcripts.
- Close each worker immediately after merge or rejection. Do not keep dormant workers alive.

## Kickoff Rule

Kickoff requires a controlled tree, not a lucky tree.

Required kickoff commands:

```bash
mkdir -p "$RUN_ROOT"/authority-snapshot
mkdir -p "$RUN_ROOT"/validation/{kickoff,freeze,proof-wall,docs,int,final,blocked}
mkdir -p "$RUN_ROOT"/tasks

git branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git rev-parse --short=7 HEAD | tee "$RUN_ROOT/validation/kickoff/head.short.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"

sed -E 's/^...//' "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt" \
  | sed -E 's/^[^ ]+ -> //' \
  > "$RUN_ROOT/validation/kickoff/dirty-paths.txt"

if [ -s "$RUN_ROOT/validation/kickoff/dirty-paths.txt" ] && \
   rg -n -v '^(PLAN\.md|ORCH_PLAN\.md)$' "$RUN_ROOT/validation/kickoff/dirty-paths.txt"; then
  echo "Unexpected dirty or untracked path outside allowed authority inputs" \
    | tee "$RUN_ROOT/validation/kickoff/kickoff-error.txt"
  exit 1
fi

cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/authority-snapshot/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/authority-snapshot/ORCH_PLAN.md"
git diff -- PLAN.md ORCH_PLAN.md > "$RUN_ROOT/authority-snapshot/authority-input.diff"
```

Kickoff acceptance:

- Branch is `feat/m40-plus`.
- `HEAD` short sha is `709a30f`.
- Only `PLAN.md` is dirty, or `PLAN.md` plus `ORCH_PLAN.md`.
- No other modified, deleted, staged, renamed, or untracked path is allowed.
- Authority snapshots are captured before any source edit.
- `baseline.json`, `authority-freeze.json`, `tasks.json`, and `queue.json` exist before implementation starts.

### Current Kickoff-State Validation Warning

The current workspace kickoff-state validation shows dirty passport artifacts outside M51 scope:

- `examples/ecommerce/units/money/round.spec.passport.json`
- `examples/ecommerce/units/pricing/apply_discount.spec.passport.json`
- `examples/ecommerce/units/pricing/apply_tax.spec.passport.json`
- `examples/ecommerce/units/pricing/calculate_total.spec.passport.json`
- `examples/ecommerce/units/pricing/discount_strategy.spec.passport.json`
- `examples/ecommerce/units/pricing/pricing_quote.spec.passport.json`

If those paths remain dirty at actual kickoff, stop before any M51 implementation work. They are not owned by this milestone.

## Freeze-Commit Hygiene

Kickoff may tolerate dirty `PLAN.md` and `ORCH_PLAN.md` because they are authority inputs. The worker fork basis may not.

Rules for the Step 1 freeze commit recorded in `contract-freeze.json`:

- The freeze commit must contain only intended Step 1 changes in:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/helper_surface.rs`
  - `xtask/src/family/decision_kernel.rs`
- The freeze commit must not stage:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - any doc file
  - `xtask/src/lib.rs`
  - any `RUN_ROOT/**` state file
- Worker branches and worktrees must fork from the exact `contract_freeze_commit`, not from `HEAD` and not from a dirty working tree.
- `contract-freeze.json` must record:
  - `contract_freeze_commit`
  - `contract_freeze_commit_short`
  - final `mod.rs` export/presentation decision
  - final shim labeling decision
  - proof-wall direct-import rule for Step 2
  - conditional ownership decision for `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
  - exact worktree and branch fork basis

Required pre-freeze hygiene commands:

```bash
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/freeze/pre-freeze-status.porcelain.txt"
git diff --cached --name-only | tee "$RUN_ROOT/validation/freeze/pre-freeze-staged.txt"
git diff --name-only | tee "$RUN_ROOT/validation/freeze/pre-freeze-unstaged.txt"

if rg -n '^(PLAN\.md|ORCH_PLAN\.md|xtask/src/lib\.rs|docs/)' \
  "$RUN_ROOT/validation/freeze/pre-freeze-staged.txt"; then
  echo "Unexpected non-Step-1 file staged into freeze commit" \
    | tee "$RUN_ROOT/validation/freeze/freeze-hygiene-error.txt"
  exit 1
fi
```

## Parent Sequence Discipline

The parent sequence is serialized and mandatory. Do not reorder these steps.

1. Capture baseline truth.
2. Write authority-freeze outputs.
3. Complete Step 1 edits in `xtask/src/family/{mod.rs,helper_surface.rs,decision_kernel.rs}`.
4. Run Step 1 proof and get a green result.
5. Create the Step 1 freeze commit.
6. Write `contract-freeze.json` with the exact fork basis from that freeze commit.
7. Create all worker and integration worktrees from that exact recorded commit.
8. Launch workers only after worktrees are created and recorded.
9. Merge worker lanes on the integration branch.
10. Run integrated proof and grep gate.
11. Fast-forward `feat/m40-plus` only after integrated acceptance passes.
12. If any step fails irrecoverably, execute blocked termination and do not fast-forward.

## File Ownership

| Workstream | Exact writable repo surfaces |
| --- | --- |
| `WS-AUTHORITY` | `xtask/src/family/mod.rs`; `xtask/src/family/helper_surface.rs`; `xtask/src/family/decision_kernel.rs`; `RUN_ROOT/**` |
| `WS-PROOF-WALL` | `xtask/src/lib.rs` |
| `WS-DOCS` | `docs/semantic_family_capability_corpus_guide_v0.1.md`; `docs/recommendation_corpus_expansion_program_v0.1.md`; `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` only if `contract-freeze.json` marks it writable |
| `WS-INT` | merge mechanics only in `ws/m51-int`; no creative source edits; authoritative validation captures in `RUN_ROOT/**` only |
| `finalize` | fast-forward `feat/m40-plus` to `ws/m51-int`; write `acceptance-ledger.md`; write final captures in `RUN_ROOT/**` |

## Task Ledger

| Task ID | Workstream | Depends on | Purpose |
| --- | --- | --- | --- |
| `task-m51-a0-baseline-capture` | `WS-AUTHORITY` | — | capture kickoff truth and write baseline state |
| `task-m51-a1-authority-freeze` | `WS-AUTHORITY` | `task-m51-a0-baseline-capture` | lock scope, guards, file ownership, proof commands, and abort triggers |
| `task-m51-a2-ownership-contract-freeze` | `WS-AUTHORITY` | `task-m51-a1-authority-freeze` | freeze shim presentation and ownership contract in `xtask/src/family/` |
| `task-m51-a3-step1-proof` | `WS-AUTHORITY` | `task-m51-a2-ownership-contract-freeze` | prove Step 1 is green before creating the worker fork basis |
| `task-m51-a4-freeze-commit-and-fork` | `WS-AUTHORITY` | `task-m51-a3-step1-proof` | create freeze commit, write `contract-freeze.json`, create worktrees from that commit |
| `task-m51-b1-proof-wall` | `WS-PROOF-WALL` | `task-m51-a4-freeze-commit-and-fork` | retarget `xtask/src/lib.rs` proof-wall imports and basis snapshot calls to `analysis_core` |
| `task-m51-c1-docs` | `WS-DOCS` | `task-m51-a4-freeze-commit-and-fork` | rewrite command-facing ownership docs to match the frozen code truth |
| `task-m51-d1-integrate-lanes` | `WS-INT` | `task-m51-b1-proof-wall`, `task-m51-c1-docs` | merge both worker lanes into the integration branch |
| `task-m51-d2-proof-and-grep` | `WS-INT` | `task-m51-d1-integrate-lanes` | run authoritative proof floor and final grep exit gate |
| `task-m51-e1-parent-closeout` | `finalize` | `task-m51-d2-proof-and-grep` | fast-forward validated result and finalize acceptance ledger |

## Worker Prompt Rules

Every worker prompt must contain:

- the worker task id and workstream name
- the exact owned file set for that lane
- the exact forbidden-touch file set
- the relevant `PLAN.md` excerpts for that lane
- the exact `contract-freeze.json` facts for that lane
- the exact required commands for that lane
- the exact acceptance criteria for that lane
- the exact stop rules for that lane
- the requirement to use GPT-5.4 with `reasoning_effort=high`
- the rule that workers do not write `RUN_ROOT/**`
- the rule that workers do not widen scope or reinterpret the frozen contract

Each worker prompt must not contain:

- full prior worker transcripts
- full parent session history
- unrelated repo context
- broad redesign brainstorming
- permission to modify out-of-scope files

Parent review rules:

- The parent reviews summaries plus narrow diffs only.
- The parent records authoritative outcomes into `RUN_ROOT/tasks/<TASK_ID>/`.
- The parent closes each worker immediately after merge or explicit rejection.
- Workers are not kept alive after merge.

## Worker Return Contract

Each worker submission must include only:

- changed files
- commands run, with exit codes
- blockers or unresolved assumptions
- final branch name
- final commit sha

Parent-side submission handling:

- Record the submission into `tasks/<TASK_ID>/result.md`.
- Copy command outputs into the matching `validation/<lane>/` directory.
- Set `sentinel.json.status` to `submitted` before review.
- Set `sentinel.json.status` to `done` only after merge and validation.
- Set `sentinel.json.status` to `blocked` if the submission leaks scope, misses required outputs, or fails acceptance.

## Workstream Plan

### WS-AUTHORITY (`lane/m51-parent-authority`, parent only, sequential)

Purpose: freeze the M51 ownership contract and create the only allowed shared base for parallel work.

Exact operator sequence:

#### A0. Baseline capture

Required commands:

```bash
git branch --show-current
git rev-parse HEAD
git rev-parse --short=7 HEAD
git status --porcelain=v1 -uall
```

Required work:

- Write `baseline.json`.
- Capture kickoff validation outputs under `validation/kickoff/`.
- Create task sentinels for all task ids with initial status.

Acceptance:

- Baseline branch, commit, and dirty-tree truth are captured.
- Kickoff hard guards pass.

#### A1. Authority freeze outputs

Required work:

- Write:
  - `authority-freeze.json`
  - `file-ownership.json`
  - `read-only-references.txt`
  - `in-scope-files.txt`
  - `out-of-scope-files.txt`
  - `tasks.json`
  - `queue.json`
  - initial `session-log.md`

Acceptance:

- Scope, ownership, guards, task order, and abort triggers are recorded before edits begin.

#### A2. Step 1 ownership-contract edits

Required work:

- Edit only:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/helper_surface.rs`
  - `xtask/src/family/decision_kernel.rs`
- Freeze the wording and module-presentation rule that `analysis_core/*` is the sole owner surface.
- Decide the final `mod.rs` presentation:
  - `analysis_core` stays first-class
  - shims either remain exported with explicit compatibility-only framing or stop being elevated as peer truth
- Keep shim behavior identical if they remain.

Acceptance:

- Only the three owned repo files are changed.
- The Step 1 ownership contract is explicit and unambiguous.

#### A3. Step 1 proof run

Required commands:

```bash
cargo test -p xtask
```

Acceptance:

- `cargo test -p xtask` is green on the Step 1 working tree.
- If red, stop before any commit or worker fork.

#### A4. Freeze commit, contract-freeze, and worktree creation

Required commands:

```bash
git add xtask/src/family/mod.rs xtask/src/family/helper_surface.rs xtask/src/family/decision_kernel.rs
git commit -m "Freeze M51 shared-core ownership contract"

FREEZE_COMMIT=$(git rev-parse HEAD)
FREEZE_COMMIT_SHORT=$(git rev-parse --short=7 HEAD)

git worktree add -b ws/m51-proof-wall "$WT_ROOT/proof-wall" "$FREEZE_COMMIT"
git worktree add -b ws/m51-docs "$WT_ROOT/docs" "$FREEZE_COMMIT"
git worktree add -b ws/m51-int "$WT_ROOT/int" "$FREEZE_COMMIT"
```

Required work:

- Write `contract-freeze.json` after the freeze commit exists and before any worker starts.
- Record in `contract-freeze.json`:
  - `contract_freeze_commit`
  - `contract_freeze_commit_short`
  - Step 1 ownership decisions
  - exact branch names
  - exact worktree paths
  - explicit statement that all worker lanes fork from `contract_freeze_commit`
  - conditional write decision for `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- Write `worktrees.json` from the actual created worktrees.

Acceptance:

- The freeze commit exists and is the exact fork basis.
- All worktrees are created from `contract_freeze_commit`, not from a dirty tree and not from an unspecified `HEAD`.
- No worker starts before `contract-freeze.json` and `worktrees.json` exist.

WS-AUTHORITY stop rules:

- If kickoff dirt exists outside allowed authority inputs, stop.
- If `cargo test -p xtask` is red after Step 1, do not fork workers.
- If Step 1 cannot be completed without touching protected read-only files, stop and re-scope.
- If Step 1 discovers a hidden non-test shim owner outside named scope, stop and re-scope.

### WS-PROOF-WALL (`lane/m51-proof-wall`, worker, post-freeze only)

Purpose: make the command-facing proof wall consume the seam directly.

Required commands:

```bash
cargo test -p xtask
rg -n "decision_kernel::corpus_program_basis_snapshot|decision_kernel" xtask/src/lib.rs
```

Required work:

1. Edit only `xtask/src/lib.rs`.
2. Remove the test-module import dependency on `decision_kernel` as owner truth.
3. Retarget both basis-snapshot proof-wall call sites to `family::analysis_core::corpus_program_basis_snapshot(...)`.
4. Update nearby test prose, helper names, or comments only if they still imply shim ownership.
5. Keep all expected stop-state outputs unchanged.

WS-PROOF-WALL acceptance:

- Only `xtask/src/lib.rs` is changed.
- `cargo test -p xtask` is green.
- `xtask/src/lib.rs` no longer imports or calls `decision_kernel::corpus_program_basis_snapshot(...)`.
- No new wrapper or adapter layer is introduced.

WS-PROOF-WALL stop rules:

- If a fix requires touching `xtask/src/family/analysis_core/*`, stop.
- If a fix requires touching any doc or any other `xtask/src/family/*.rs` file, stop.
- If stop-state outputs move, stop and bounce the lane.

### WS-DOCS (`lane/m51-docs`, worker, post-freeze only)

Purpose: make command-facing docs teach the same ownership graph the code implements.

Required commands:

```bash
rg -n "decision_kernel|helper_surface|analysis_core" \
  docs/semantic_family_capability_corpus_guide_v0.1.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/ai_promotion_and_multilanguage_milestones_v0.1.md
```

Required work:

1. Edit only:
   - `docs/semantic_family_capability_corpus_guide_v0.1.md`
   - `docs/recommendation_corpus_expansion_program_v0.1.md`
   - `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` only if the frozen Step 1 truth makes current wording false
2. Replace statements that claim `helper_surface.rs` or `decision_kernel.rs` own current semantics.
3. Rewrite that ownership story to point at `analysis_core/*`.
4. Preserve historical milestone context only where clearly labeled historical or compatibility-only.
5. Do not widen the docs blast radius.

WS-DOCS acceptance:

- Only owned doc files are changed.
- Maintainer-facing current-state ownership text points at `analysis_core/*`.
- Any remaining mention of `helper_surface.rs` or `decision_kernel.rs` as live surfaces is explicitly compatibility-only or historical.
- No repo-root roadmap or plan-authority cleanup is introduced.

WS-DOCS stop rules:

- If docs repair requires code edits, stop and return the blocker.
- If `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` does not become false, leave it unchanged.
- If docs drift expands outside the three named files, stop.

### WS-INT (`lane/m51-int`, parent only)

Purpose: merge both worker lanes and run the only authoritative proof pass.

Required merge order:

1. Merge `ws/m51-proof-wall`.
2. Merge `ws/m51-docs`.
3. Run the authoritative proof floor.
4. Run the final grep exit gate.

Required commands:

```bash
git merge --no-ff ws/m51-proof-wall
git merge --no-ff ws/m51-docs

cargo test -p xtask
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json

rg -n "decision_kernel|helper_surface" \
  xtask/src/lib.rs \
  xtask/src/family \
  docs/semantic_family_capability_corpus_guide_v0.1.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/ai_promotion_and_multilanguage_milestones_v0.1.md
```

WS-INT acceptance:

- Both worker branches merge without widening scope.
- The proof floor is green after both merges.
- Remaining grep hits, if any, are compatibility-only or historical.
- Stop-state outputs remain unchanged.
- `validation/int/` contains captured outputs for every required proof command and the grep gate.

WS-INT stop rules:

- If integration requires creative edits outside conflict resolution, stop and reopen the owning lane.
- If any proof command fails, do not fast-forward `feat/m40-plus`.
- If grep shows a present-day ownership lie, do not close out.

### Finalize (`lane/m51-parent-closeout`, parent only)

Purpose: fast-forward `feat/m40-plus` to the validated integration result, write the acceptance ledger, and capture final branch state.

Preconditions:

- `task-m51-d2-proof-and-grep` is `done`.
- `ws/m51-int` contains the validated integration result.
- No blocked task remains open.

Required commands:

```bash
git -C "$WT_ROOT/int" rev-parse HEAD | tee "$RUN_ROOT/validation/final/ws-m51-int.head.txt"
git -C "$WT_ROOT/int" rev-parse --short=7 HEAD | tee "$RUN_ROOT/validation/final/ws-m51-int.head.short.txt"

git fetch . ws/m51-int:ws/m51-int
git checkout feat/m40-plus
git merge --ff-only ws/m51-int

git rev-parse HEAD | tee "$RUN_ROOT/validation/final/final-head.txt"
git rev-parse --short=7 HEAD | tee "$RUN_ROOT/validation/final/final-head.short.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/final/final-status.porcelain.txt"
```

Required work:

- Write `acceptance-ledger.md` with:
  - baseline commit
  - `contract_freeze_commit`
  - worker branch submission commits
  - integration branch commit
  - final fast-forwarded branch commit
  - required proof commands and exit status
  - expected stop-state outputs
  - grep exit-gate disposition
  - any conditional note about `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- Update `tasks.json`, `queue.json`, and final task sentinels to `done`.
- Append final closeout entry to `session-log.md`.

Finalize acceptance:

- `feat/m40-plus` fast-forwards cleanly to the validated `ws/m51-int` result.
- `acceptance-ledger.md` exists and records the full acceptance chain.
- `validation/final/` captures the final branch sha and clean-status proof.
- No additional source edits are introduced during closeout.

Finalize stop rules:

- If `git merge --ff-only ws/m51-int` fails, stop and record blocked termination.
- If final status shows unexpected dirt outside allowed authority files, stop and record blocked termination.
- Do not rewrite history and do not create a merge commit at closeout.

## Merge And Integration Policy

- Only the parent merges worker branches.
- Worker branches must fork from the exact `contract_freeze_commit`.
- Workers do not merge each other.
- No worker rebases after submission unless the parent requests it.
- Integration edits are limited to mechanical conflict resolution.
- If a conflict requires reinterpreting the Step 1 contract, reject the merge, write `blocked.json`, and reopen the appropriate lane.
- The final authoritative proof run happens once, after both worker branches are merged into `ws/m51-int`.

## Blocked Termination Protocol

Blocked termination is mandatory whenever the run cannot complete without violating scope, proof, or branch hygiene.

### When `blocked.json` must be written

Write `blocked.json` immediately if any of these occur:

- kickoff hard guards fail
- Step 1 proof fails and cannot be repaired within Step 1 scope
- freeze-commit hygiene is violated
- worktree creation cannot use the exact `contract_freeze_commit`
- a worker lane leaks scope or cannot finish within owned files
- integrated proof or grep exit gate fails
- final fast-forward of `feat/m40-plus` fails

### Required `blocked.json` fields

`blocked.json` must contain:

- `run_id`
- `blocked_at`
- `blocked_by_task_id`
- `phase`
- `branch`
- `head_commit`
- `head_commit_short`
- `contract_freeze_commit` if it exists
- `status`
- `blocking_reason`
- `trigger_class`
- `in_scope_work_completed`
- `remaining_open_tasks`
- `touched_files`
- `required_next_action`
- `can_resume_from_current_state`
- `validation_artifacts`
- `notes`

### Required blocked-run captures

At minimum, accompany `blocked.json` with:

- `validation/blocked/git-status.porcelain.txt`
- `validation/blocked/head.txt`
- `validation/blocked/head.short.txt`
- the failing command output or merge error output that caused the block
- copied task-level validation artifacts from the active lane
- a final `session-log.md` entry describing the exact stop point

### Blocked-run branch policy

- Parent does not fast-forward `feat/m40-plus` on blocked termination.
- Parent does not create a closeout merge commit to “save progress.”
- Partial worker results may remain on their branches, but the authoritative run ends blocked.

### Task and queue updates on blocked termination

- Active task sentinel gets `status: blocked`.
- Any dependent not-yet-started tasks move to `cancelled`.
- `queue.json` records the blocked task, the cancellation wave, and the exact blocker reason.
- `tasks.json` reflects final blocked or cancelled status for every remaining task.
- `acceptance-ledger.md` is not written on blocked termination.

## Tests And Acceptance

### Required Proof Commands

```bash
cargo test -p xtask
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
rg -n "decision_kernel|helper_surface" xtask/src/lib.rs xtask/src/family docs/semantic_family_capability_corpus_guide_v0.1.md docs/recommendation_corpus_expansion_program_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md
```

### Expected Semantic Outputs

- `recommendation_status = "insufficient_real_corpus"`
- `decision_status = "not_recommended"`
- `decision_action = "stop"`
- `decision_basis_code = "no_actionable_candidate"`
- `required_next_action = "record_stop_without_new_milestone"`
- `overall_verdict = "pass"`

### Required `verify-decision-contract` Check Outcomes

- `recommendation_analysis_validation = pass`
- `corpus_program_decision_validation = pass`
- `basis_snapshot_parity = pass`
- `derived_decision_parity = pass`
- `frozen_helper_surface_floor = pass`

### Grep Exit Gate

M51 is done only when all of these are true:

1. Command-facing docs no longer claim `xtask/src/family/helper_surface.rs` or `xtask/src/family/decision_kernel.rs` own current semantics.
2. `xtask/src/lib.rs` no longer imports or calls `decision_kernel::corpus_program_basis_snapshot(...)`.
3. Any remaining mention of `helper_surface.rs` or `decision_kernel.rs` as live code surfaces is explicitly compatibility-only or historical.
4. `xtask/src/family/mod.rs` no longer presents shim topology as peer semantic truth.

## Assumptions

- `PLAN.md` remains the sole scope authority throughout execution.
- `709a30f` is the correct baseline authority commit for this orchestration draft; if `HEAD` moves, refresh or explicitly revalidate before execution.
- The current dirty passport artifacts are unrelated operator noise, not M51-owned work.
- `analysis_core/*` already exposes everything needed for the proof-wall rewire; no new shared-core API should be required.
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` is conditionally writable and should stay untouched unless Step 1 or Step 2 makes its current wording false.
- Any hidden non-test shim-owner consumer outside the named write scope is a re-scope event, not a cleanup add-on.
