# M40 Orchestration Plan

Status: **authoritative execution contract for the M40 planning run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Baseline HEAD: **`e8b1f96d9e8619d3b363529a6f71b254103039dc`** (`e8b1f96`)  
Last rewritten: **`2026-05-07`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_architecture_shared_core_follow_on_plan`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan`**  
Canonical proof commands:  
- **`cargo xtask family verify-decision-contract --format json`**
- **`cargo xtask family corpus-decision --format json`**
- **`cargo test -p xtask`**
Execution note: **M40 is the planning follow-on only. It authors and lands the orchestration contract for the architecture shared-core follow-on decision. It does not authorize shared-core extraction, corpus run `1`, a new Rust family wedge, public schema growth, new CLI behavior, or second-language backend work.**

## Summary

- `PLAN.md` is milestone authority. This file is the operator contract for executing that authority without improvisation.
- The parent agent owns the full critical path: baseline capture, authority freeze, integration worktree creation, M40 `ORCH_PLAN.md` rewrite, proof-floor capture, merge, acceptance, publish, post-publish verification, and final closeout.
- There is exactly one honest optional parallel seam in M40:
  - after the parent finishes the first full M40 draft and writes `draft-contract-freeze.json`
  - only if an independent `ORCH_PLAN.md` consistency audit is still useful
  - only as a dedicated worktree/subagent lane for `ORCH_PLAN.md`
- Concurrency is capped by phase:
  - `0` before baseline capture finishes
  - `1` through authority freeze and draft contract freeze
  - `2` maximum after optional Lane B launch
  - `1` again for merge, acceptance, publish, and closeout
- M40 is not complete until the repo proves all of these together:
  - `ORCH_PLAN.md` is rewritten from the stale M39 execution contract into an M40 planning-run contract
  - the three live authority commands remain green on the accepted tree
  - the accepted contract preserves the exact M40 trigger table, seam boundary, and M41 gate from `PLAN.md`
  - the accepted integration result publishes back onto the live `feat/corpus-expansion` branch
  - `closeout.md` ends with exactly one allowed verdict

## Hard Guards

- `PLAN.md` wins over this file, worker notes, stale worktrees, and run-state summaries if they disagree.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion` is the baseline and publish target, not the merge surface.
- All parent-owned authored work after baseline capture happens on `ws/m40-int`, not on the live checkout.
- M40 must not introduce:
  - any Rust source edits
  - any move of `xtask/src/family/decision_kernel.rs`
  - any new public artifact schema field or schema version bump
  - any new CLI command or CLI behavior
  - any reopening of corpus run `1`
  - any shared-core extraction
  - any cross-crate extraction
  - any second-language backend or portability work
  - any helper-surface warning cleanup justified as milestone scope
  - any reinterpretation of the M40 trigger table into implementation authority
- The only standing proof commands authorized for M40 approval are:
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo test -p xtask`
- M40 cannot claim success if `cargo xtask family corpus-decision --format json` stops returning:
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`
- If the live decision truth changes enough that the current action is no longer `author_architecture_follow_on_plan`, the run stops and restarts from a fresh milestone authority. M40 does not silently continue on outdated assumptions.
- `PLAN.md` is frozen after `authority-freeze.json`.
- `ORCH_PLAN.md` is frozen after `draft-contract-freeze.json` except for the explicit M40 audit edits performed either by the parent or by launched Lane B under this contract.
- Parent-owned canonical run-state remains writable by the parent for the full run. Freeze records and snapshots are immutable once written, but the parent must keep appending or writing later run-state artifacts such as `draft-contract-freeze.json`, proof captures, acceptance, publish records, and `closeout.md`.
- If `PLAN.md`, `TODOS.md`, or the named M39 closeout input change after `authority-freeze.json`, the run stops and restarts from a fresh baseline.
- If Lane B is not launched, only the parent may edit `ORCH_PLAN.md` after `draft-contract-freeze.json`.
- If Lane B is launched, Lane B owns `ORCH_PLAN.md` in its worktree until it returns or is invalidated. The parent remains the only merge authority.
- `.runs/m40_architecture_shared_core_follow_on_plan/**` is parent-owned only. Optional lanes do not write canonical run-state.
- No lane edits `.semantic-family-artifacts/**`. Those files are read-only derived inputs.
- No lane edits `PLAN.md` or `TODOS.md`.
- The candidate future seam stays descriptive only in M40:
  - smallest reusable decision semantics under `xtask/src/family/decision_core/`
  - helper-surface contract
  - bounded decision derivation helpers
  - normalized proof-fingerprint helpers
- The following must stay explicitly local even in the future seam discussion and therefore must not be extracted or reframed by M40:
  - `xtask` CLI wiring
  - artifact path lookup
  - command-specific JSON rendering
  - proof-wall file locations
  - milestone-specific closeout wording

## Closed Planning Surface

| Path | M40 responsibility | Owner |
|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | milestone authority and freeze reference | Parent only, read-only after baseline |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` | execution contract rewrite from M39 to M40, proof-wall adoption for M40, closeout discipline | Parent through draft freeze; Optional Lane B only after launch; parent merges final |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md` | trigger inventory reference only | Read-only input |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe/closeout.md` | latest shipped evidence that M39 proved the third honest consumer | Read-only input |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_architecture_shared_core_follow_on_plan/**` | baseline records, freeze records, launch packets, proof capture, acceptance, publish verification, blocked state, closeout | Parent only |

Rules for the closed planning surface:

- Any edit outside this table is out of scope unless a merge conflict mechanically forces it and the parent records that in `merge-log.md`.
- Lane B owns exactly one authored file: `ORCH_PLAN.md`.
- There is no honest Lane C in M40. Proof capture, acceptance, publish, and final verdict remain parent-owned because they depend on canonical run-state and merged truth.
- Read-only input paths may be read for fidelity checks, but changing them violates M40 scope.
- `.runs/m40_architecture_shared_core_follow_on_plan/**` is a derived proof and orchestration surface. It is not milestone authority.

## Branch And Worktree Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Canonical branches and worktrees:

| Role | Branch | Worktree |
|---|---|---|
| Live baseline and publish target | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` |
| Parent integration spine | `ws/m40-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan/int` |
| Optional Lane B, `ORCH_PLAN.md` audit lane | `ws/m40-lane-b-orch-audit` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan/lane-b-orch-audit` |

Creation rules:

1. The parent captures baseline on the live checkout before creating any M40 worktree.
2. `ws/m40-int` is created from the exact SHA recorded in `integration-base.txt`.
3. Lane B may be created only after `draft-contract-freeze.json` exists and only from the exact freeze SHA recorded there.
4. Lane B is launched only if `ORCH_PLAN.md` still benefits from an independent fidelity audit after the parent draft freeze. If the draft already closes every checklist item cleanly, write `lane-b-skip.json` and stay sequential.
5. Lane B never forks from the live checkout. It forks only from the frozen integration SHA.
6. If a named M40 worktree already exists with stale state, the parent recreates it and records that in `session.log`.
7. The live branch moving after baseline capture does not silently change the run. The parent must either re-baseline or explicitly merge the new live head into `ws/m40-int` and rerun the full M40 verification wall.
8. No optional lane writes back to the live checkout directly.

Canonical worktree creation commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan/int \
  -b ws/m40-int <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan/lane-b-orch-audit \
  -b ws/m40-lane-b-orch-audit <DRAFT_CONTRACT_FREEZE_SHA>
```

## Canonical Run-State

Parent-owned orchestration truth uses a phase-specific active root:

- `LIVE_PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `INT_PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan/int`
- `ACTIVE_PARENT_ROOT=LIVE_PRIMARY_ROOT` through `gate-m40-10-authority-freeze`
- `ACTIVE_PARENT_ROOT=INT_PRIMARY_ROOT` from `task-m40-20-create-integration-worktree` through `gate-m40-80-pre-publish-acceptance`
- `ACTIVE_PARENT_ROOT=LIVE_PRIMARY_ROOT` from `task-m40-90-publish-to-live` through `gate-m40-100-final-closeout`
- `ACTIVE_RUN_ROOT=$ACTIVE_PARENT_ROOT/.runs/m40_architecture_shared_core_follow_on_plan`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-architecture-shared-core-follow-on-plan`

Run-state handoff rules:

- Before `task-m40-20-create-integration-worktree`, the live checkout copy is canonical because the integration worktree does not exist yet.
- `task-m40-20-create-integration-worktree` must copy the already-written parent run-state from `LIVE_PRIMARY_ROOT` into `INT_PRIMARY_ROOT` before parent authored work continues.
- After that copy, the integration worktree copy is canonical for all parent-owned run-state, proof, merge, and pre-publish acceptance artifacts.
- The live checkout copy becomes read-only until `task-m40-90-publish-to-live` or full restart.
- `task-m40-90-publish-to-live` must land the accepted `ws/m40-int` result onto the live checkout and then make the live checkout copy canonical again for post-publish verification and final closeout.

Canonical parent-owned files:

- `baseline.json`
- `integration-base.txt`
- `publish-head.txt`
- `authority-freeze.json`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `authority-snapshot/TODOS.md`
- `authority-snapshot/m39-closeout.md`
- `run-state.json`
- `tasks.json`
- `queue.json`
- `session.log`
- `verify-decision-contract.baseline.json`
- `corpus-decision.baseline.json`
- `xtask-test.baseline.txt`
- `draft-contract-freeze.json`
- `draft-diff-summary.md`
- `lane-b-launch.md`
- `lane-b-skip.json`
- `merge-log.md`
- `proof-log.json`
- `verify-decision-contract.integration.json`
- `corpus-decision.integration.json`
- `xtask-test.integration.txt`
- `plan-contract-audit.md`
- `acceptance.md`
- `publish-verification.md`
- `publish-result.json`
- `verify-decision-contract.post-publish.json`
- `corpus-decision.post-publish.json`
- `xtask-test.post-publish.txt`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `closeout.md`

Required contents:

- `baseline.json`
  - live branch
  - live HEAD SHA
  - dirty-state summary for the closed planning surface
  - authority artifact paths
  - exact command tuple for the three required M40 proof commands
  - captured `decision_action`, `decision_basis_code`, and `required_next_action`
  - recorded last line from the M39 closeout input
- `integration-base.txt`
  - exact SHA used to create `ws/m40-int`
- `publish-head.txt`
  - exact live HEAD SHA captured during baseline
- `authority-freeze.json`
  - snapshot paths for `PLAN.md`, `ORCH_PLAN.md`, `TODOS.md`, and `m39-closeout.md`
  - frozen branch and worktree layout
  - closed planning surface checksum or literal copy
  - explicit statement that no optional lane is yet authorized
  - explicit statement that M40 remains planning-only
- `run-state.json`
  - `current_task_id`
  - `current_task_status`
  - `active_parent_root`
  - `active_run_root`
  - `live_publish_status: not_started|in_progress|blocked|complete`
  - `last_updated_at`
- `tasks.json`
  - static ordered registry of all tasks and gates in this file
  - exact task ids and titles
  - lane ownership for each entry
  - no mutable status authority
- `queue.json`
  - one ordered entry per task or gate in this plan
  - required fields per entry:
    - `id`
    - `kind: task|gate`
    - `title`
    - `owner`
    - `status: pending|active|blocked|skipped|complete`
    - `depends_on`
    - `worktree`
    - `artifacts`
    - `blocking_reason`
    - `started_at`
    - `completed_at`
  - required initial entries:
    - `gate-m40-00-baseline-capture`
    - `gate-m40-10-authority-freeze`
    - `task-m40-20-create-integration-worktree`
    - `gate-m40-30-draft-contract-freeze`
    - `gate-m40-40-optional-lane-launch`
    - `gate-m40-50-proof-floor-capture`
    - `gate-m40-60-merge-and-integration`
    - `gate-m40-70-plan-contract-audit`
    - `gate-m40-80-pre-publish-acceptance`
    - `task-m40-90-publish-to-live`
    - `gate-m40-95-post-publish-verification`
    - `gate-m40-100-final-closeout`
- `session.log`
  - timestamped summary entries for every gate transition
  - worktree creation or recreation notes
  - stale-lane invalidation notes
- `verify-decision-contract.baseline.json`
  - verbatim stdout capture from the canonical verifier command on the live baseline
- `corpus-decision.baseline.json`
  - verbatim stdout capture from the corpus decision command on the live baseline
- `xtask-test.baseline.txt`
  - stdout and stderr capture from `cargo test -p xtask` on the live baseline
- `draft-contract-freeze.json`
  - exact committed `ws/m40-int` SHA after the parent rewrites `ORCH_PLAN.md`
  - required command tuple
  - required M40 section headings that must remain present
  - explicit `lane_b_status: launch|skip`
  - explicit statement that no implementation authority is created by M40
- `draft-diff-summary.md`
  - concise summary of how the accepted M40 contract differs from the stale M39 contract
  - explicit note that the contract remains planning-only
- `lane-b-launch.md`
  - owned file: `ORCH_PLAN.md`
  - forbidden files
  - exact M40 headings and command literals that must remain intact
  - stale-lane invalidation triggers
  - required return contract
- `lane-b-skip.json`
  - `status: skipped`
  - exact reason for no parallel launch
- `merge-log.md`
  - source branch
  - source SHA
  - target SHA before merge
  - target SHA after merge
  - conflicts encountered
  - resolutions applied
  - whether the draft contract freeze remained intact
- `proof-log.json`
  - command
  - cwd
  - exit code
  - captured artifact path
  - pass/fail
  - semantic interpretation
- `verify-decision-contract.integration.json`
  - verbatim stdout capture from the canonical verifier command on the accepted integration tree
- `corpus-decision.integration.json`
  - verbatim stdout capture from the corpus decision command on the accepted integration tree
- `xtask-test.integration.txt`
  - stdout and stderr capture from `cargo test -p xtask` on the accepted integration tree
- `plan-contract-audit.md`
  - named sections present in `ORCH_PLAN.md`
  - confirmation that the trigger table, seam boundary, local-first extraction rule, M41 gate, and future lane model are present
  - confirmation that M40 authorizes no code motion
  - explicit note whether Lane B was used or skipped
- `acceptance.md`
  - final checklist mapped to M40 acceptance gates
  - baseline proof
  - draft freeze proof
  - lane launch or skip proof
  - integration proof
  - plan contract audit proof
  - publish proof
  - final verdict proof
- `publish-verification.md`
  - live checkout branch and HEAD after publish
  - commands and exit codes for post-publish verification
  - confirmation that live `feat/corpus-expansion` matches the accepted `ws/m40-int` result
- `publish-result.json`
  - `publish_source_branch`
  - `publish_source_sha`
  - `publish_target_branch`
  - `publish_target_sha_before`
  - `publish_target_sha_after`
  - `publish_method: fast_forward|merge`
  - `status: complete|blocked`
- `verify-decision-contract.post-publish.json`
  - verbatim stdout capture from the canonical verifier command on the published live tree
- `corpus-decision.post-publish.json`
  - verbatim stdout capture from the corpus decision command on the published live tree
- `xtask-test.post-publish.txt`
  - stdout and stderr capture from `cargo test -p xtask` on the published live tree
- `blocked.json`
  - blocking task id
  - blocking lane
  - blocking reason
  - whether live decision truth drifted
  - whether `keep the kernel local` is forced
- `closeout.md`
  - short narrative of what was proven
  - explicit note that M40 stayed planning-only
  - explicit note whether future M41 authorization remains gated
  - exact final verdict as the last non-empty line

## Seam Boundary

The future seam is not "move everything under `xtask/src/family/`."

The future seam is the smallest boundary that carries the reusable bounded decision semantics:

```text
POSSIBLE FUTURE LOCAL SEAM
==========================
xtask/src/family/decision_core/
  helper_surface contract
  decision derivation helpers
  normalized proof fingerprint helpers

STAYS LOCAL EVEN AFTER THAT
===========================
xtask CLI wiring
artifact path lookup
command-specific JSON rendering
proof-wall file locations
milestone-specific closeout wording
```

### Local-first extraction rule

If future implementation is authorized by internal consumer pressure only, the first implementation milestone remains a still-local extraction inside `xtask/src/family/`.

If a non-`xtask` consumer appears, the repo may then consider the stronger cross-crate extraction claim.

That rule stays frozen throughout M40:

- internal pressure -> local seam first
- external pressure -> cross-crate extraction becomes discussable

## Trigger Table

This table is part of the execution contract, not optional commentary.

| Follow-on | Current status after M39 | Trigger that authorizes it | Authorized next move | What still does **not** count |
|---|---|---|---|---|
| **Local decision-core extraction inside `xtask/src/family/`** | not yet triggered | one additional non-`recommend.rs` / non-`promotion_artifacts.rs` consumer inside `xtask/src/family/` beyond `verify.rs`, using the same bounded decision semantics | author an M41 implementation milestone that keeps the seam local to `xtask/src/family/` | `verify.rs` alone, dead-code cleanup, or general architectural tidiness |
| **Cross-crate family-analysis shared core** | not yet triggered | one non-`xtask` crate needs the same bounded decision semantics | author a new implementation plan that may move the seam across crate boundaries | internal-only reuse pressure without a non-`xtask` consumer |
| **Generalized multi-wedge decision layer** | not yet triggered | a second durable non-promotable wedge appears and cannot be expressed cleanly through the current kernel shape | author a dedicated follow-on plan for multi-wedge decision logic | hypothetical future wedges or policy anxiety |
| **Public semantic fingerprint fields** | not yet triggered | a real external consumer needs first-class fingerprint fields in emitted JSON | author a narrow export-surface plan for those public fields | internal proof reuse only |

## M41 Authorization Gate

M41 must choose exactly one of these outcomes:

1. **Local implementation milestone**  
   Allowed only if the first row of the trigger table is satisfied.

2. **Cross-crate implementation milestone**  
   Allowed only if a non-`xtask` consumer satisfies the second row of the trigger table.

3. **Further evidence milestone**  
   Allowed if pressure is growing but no trigger is yet satisfied.

4. **No new milestone yet**  
   Allowed if the current kernel still serves all real consumers honestly.

M41 must not default to extraction just because M40 exists.

## Workstream Plan

### Parent Lane A - critical path only

The parent owns the only non-optional workstream. No subagent is launched before `draft-contract-freeze.json`.

Before the first gate starts, the parent initializes `run-state.json` and `queue.json`, marks `gate-m40-00-baseline-capture` as `active`, and leaves every later entry as `pending` until promoted by the parent. `queue.json` is the ordering ledger. Prose in this file does not override the ledger.

#### Gate `gate-m40-00-baseline-capture`

Objective:

- prove the live M40 decision surface on the live branch before any M40 authored work begins
- record authoritative command output, input artifact truth, and live HEAD

Commands:

```bash
git branch --show-current
git rev-parse HEAD
git status --short PLAN.md ORCH_PLAN.md TODOS.md .runs/m39_verification_consumer_probe/closeout.md

cargo xtask family verify-decision-contract --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.baseline.json
jq -e '.overall_verdict == "pass"' \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.baseline.json

cargo xtask family corpus-decision --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.baseline.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.baseline.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.baseline.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.baseline.json

cargo test -p xtask | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/xtask-test.baseline.txt

tail -n 1 .runs/m39_verification_consumer_probe/closeout.md
```

Pass criteria:

- all commands exit `0`
- the live branch is `feat/corpus-expansion`
- the current decision tuple matches the frozen M40 basis from `PLAN.md`
- the last non-empty line of the M39 closeout input is `third honest consumer proven`
- `baseline.json`, `publish-head.txt`, and the three baseline proof captures are written before any worktree is created

#### Gate `gate-m40-10-authority-freeze`

Objective:

- freeze the authoritative planning inputs and run layout before authored work starts

Tasks:

1. Snapshot `PLAN.md`, `ORCH_PLAN.md`, `TODOS.md`, and `.runs/m39_verification_consumer_probe/closeout.md` into `authority-snapshot/`.
2. Write `authority-freeze.json`.
3. Stop the run if any frozen authority input changes after this point.

Pass criteria:

- `authority-freeze.json` exists
- all named authority snapshots are recorded
- no optional lane is yet authorized
- M40 is still explicitly marked planning-only

#### Task `task-m40-20-create-integration-worktree`

Objective:

- create the parent integration spine and move canonical run-state into it

Tasks:

1. Write `integration-base.txt` from the baseline SHA.
2. Create `ws/m40-int`.
3. Copy `.runs/m40_architecture_shared_core_follow_on_plan/` into the integration worktree.
4. Continue all parent work from `INT_PRIMARY_ROOT`.

#### Gate `gate-m40-30-draft-contract-freeze`

Objective:

- replace the stale M39 orchestration contract with the full M40 planning-run contract and freeze the first complete draft before any optional audit lane can begin

Parent-owned authored scope:

1. rewrite `ORCH_PLAN.md` from M39 to M40
2. preserve the operator-contract structure: summary, guards, closed surface, worktrees, run-state, queue, workstreams, context control, tests, closeout
3. keep the authored surface narrow: `ORCH_PLAN.md` plus run artifacts only
4. carry forward the exact three-command proof floor
5. carry forward the exact M40 trigger table, seam boundary, M41 gate, and future lane model from `PLAN.md`
6. explicitly record that M40 itself has no implementation authority

Commands:

```bash
git diff -- ORCH_PLAN.md
rg -n '^## ' ORCH_PLAN.md
```

Pass criteria:

- `ORCH_PLAN.md` is fully rewritten for M40 and no longer claims to be the M39 verifier-consumer probe
- no file outside the closed planning surface is edited
- `draft-contract-freeze.json` is written with the exact freeze SHA and required command tuple
- `draft-diff-summary.md` exists and explains the M39 -> M40 contract change without widening scope

### Optional Lane B - `ORCH_PLAN.md` audit lane

Lane B exists only after `gate-m40-30-draft-contract-freeze` passes. It is optional because M40 has only one honest parallel seam.

Launch condition:

- launch only if an independent fidelity audit of `ORCH_PLAN.md` is still useful on the contract-freeze commit
- otherwise write `lane-b-skip.json` and continue sequentially

Owned file:

- `ORCH_PLAN.md`

Forbidden files:

- all Rust source
- all `.runs/m40_architecture_shared_core_follow_on_plan/**`
- all `.semantic-family-artifacts/**`
- `PLAN.md`
- `TODOS.md`

Lane B required changes:

1. improve fidelity of `ORCH_PLAN.md` to `PLAN.md` only where the parent draft missed or misstated a requirement
2. preserve the exact three-command proof floor
3. preserve the exact M40 planning-only scope
4. preserve the trigger table, seam boundary, local-first extraction rule, and M41 gate from `PLAN.md`
5. do not introduce new commands, new files outside scope, or new milestone authority

Lane B required contract surfaces:

1. M40 summary and hard guards
2. the closed planning surface
3. the exact proof-command wall
4. the seam boundary, trigger table, local-first extraction rule, and explicit non-goals
5. the future M41 lane split

Lane B return contract:

- one branch off the exact `draft-contract-freeze.json` SHA
- one narrow summary of what changed in `ORCH_PLAN.md`
- no run-state edits
- no reinterpretation of `PLAN.md`

Stale-lane invalidation triggers:

- `draft-contract-freeze.json` SHA changes
- required proof-command tuple changes
- required M40 section list changes
- required trigger-table or M41 gate language changes
- parent lands conflicting `ORCH_PLAN.md` edits on `ws/m40-int`

#### Gate `gate-m40-40-optional-lane-launch`

Objective:

- make the only honest parallel seam concrete and bounded

Tasks:

1. decide `launch` or `skip` for Lane B
2. if `launch`, write `lane-b-launch.md` and create the lane worktree from the exact freeze SHA
3. if `skip`, write `lane-b-skip.json` with the explicit reason

Pass criteria:

- exactly one of `lane-b-launch.md` or `lane-b-skip.json` exists
- no second optional lane is created

#### Gate `gate-m40-50-proof-floor-capture`

Objective:

- prove that the accepted integration tree still matches the live M40 decision floor while the draft contract remains planning-only

Parallel rule:

- if Lane B is launched, the parent may run this proof capture in parallel because the commands and run-state are parent-owned and Lane B owns only `ORCH_PLAN.md`
- if Lane B proposes any non-`ORCH_PLAN.md` edit, invalidate Lane B immediately

Commands:

```bash
cargo xtask family verify-decision-contract --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.integration.json
jq -e '.overall_verdict == "pass"' \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.integration.json

cargo xtask family corpus-decision --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.integration.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.integration.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.integration.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.integration.json

cargo test -p xtask | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/xtask-test.integration.txt
```

Pass criteria:

- the three proof commands are still green on `ws/m40-int`
- live decision truth still says the next honest move is authoring the architecture follow-on plan, not extraction
- no new implementation authority appears during the run

#### Gate `gate-m40-60-merge-and-integration`

Objective:

- merge optional audit work, if any, back into `ws/m40-int`

Tasks:

1. if Lane B was launched, merge `ws/m40-lane-b-orch-audit` into `ws/m40-int`
2. resolve only straightforward `ORCH_PLAN.md` merge mechanics
3. if conflict resolution would alter the frozen proof-command tuple, the trigger table, or the M41 gate, stop, invalidate Lane B, and recreate it from a new freeze
4. record the result in `merge-log.md`

Pass criteria:

- `ws/m40-int` contains the accepted M40 contract and any accepted audit edits
- `merge-log.md` records whether Lane B was used or skipped
- if Lane B was launched, its diff remains scoped to `ORCH_PLAN.md`

#### Gate `gate-m40-70-plan-contract-audit`

Objective:

- prove that the accepted `ORCH_PLAN.md` is a faithful M40 operator contract and not an implementation plan in disguise

Audit checks:

1. the file is M40-specific, not M39-specific
2. the three-command proof floor is present in baseline, integration, and post-publish walls
3. the trigger table, seam boundary, local-first extraction rule, and M41 gate are present and faithful to `PLAN.md`
4. M40 explicitly authorizes no code motion
5. the future M41 lane model is present as guidance only
6. no out-of-scope files are newly authorized

Proof commands:

```bash
rg -n "^# M40 Orchestration Plan|^## Summary|^## Hard Guards|^## Closed Planning Surface|^## Branch And Worktree Layout|^## Canonical Run-State|^## Seam Boundary|^## Trigger Table|^## M41 Authorization Gate|^## Workstream Plan|^## Context-Control Rules|^## Tests And Acceptance|^## Closeout Rules" ORCH_PLAN.md
rg -n "verify-decision-contract --format json|corpus-decision --format json|cargo test -p xtask|local-first extraction|Cross-crate family-analysis shared core|Local implementation milestone|Cross-crate implementation milestone|Further evidence milestone|No new milestone yet|Lane A|Lane B|Lane C|Lane D" ORCH_PLAN.md
```

Pass criteria:

- `plan-contract-audit.md` exists and records all six audit checks as pass or blocker outcomes
- the accepted `ORCH_PLAN.md` contains the named M40 sections and required future M41 lane guidance
- the audit explicitly records whether Lane B was used or skipped

#### Gate `gate-m40-80-pre-publish-acceptance`

Objective:

- prove the accepted integration tree is ready to publish back to the live checkout

Tasks:

1. confirm `queue.json` marks gates `00` through `70` as `complete`
2. write `acceptance.md`
3. move `task-m40-90-publish-to-live` to `active`

Failure rule:

- publish is not allowed if baseline truth drifted, proof-floor capture is blocked, or the plan contract audit is still blocked

Pass criteria:

- `acceptance.md` exists and maps gates `00` through `70` to pass or blocker outcomes
- the accepted tree on `ws/m40-int` is the exact tree chosen for publish
- `queue.json` records `gate-m40-80-pre-publish-acceptance` as `complete`

#### Task `task-m40-90-publish-to-live`

Objective:

- land the accepted `ws/m40-int` result back onto the live `feat/corpus-expansion` checkout as the end-to-end publish step for this run

Publish rules:

1. publish is allowed only after `gate-m40-80-pre-publish-acceptance` is complete
2. the parent is the only publisher
3. publish lands the accepted integration result as one unit; do not cherry-pick partial M40 surfaces
4. accepted `ORCH_PLAN.md` changes and accepted `.runs/m40_architecture_shared_core_follow_on_plan/**` proof artifacts from `ws/m40-int` must land together
5. if live `feat/corpus-expansion` has moved since `publish-head.txt`, the parent must first merge that live movement into `ws/m40-int`, rerun gates `50` through `80`, and only then publish
6. publish may use fast-forward only when the live branch still matches `publish-head.txt`; otherwise publish uses a normal merge from the accepted `ws/m40-int` commit
7. publish does not reopen scope; it only lands the already-accepted planning contract and proof artifacts

Tasks:

1. read `publish-head.txt` and compare it to live `feat/corpus-expansion`
2. publish the accepted `ws/m40-int` commit set onto the live checkout by fast-forward or merge, as allowed above
3. write `publish-result.json`
4. mark `gate-m40-95-post-publish-verification` as `active` in `queue.json`

#### Gate `gate-m40-95-post-publish-verification`

Objective:

- prove the published live checkout still satisfies the M40 proof floor and contains the accepted contract after landing

Commands:

```bash
git branch --show-current
git rev-parse HEAD

cargo xtask family verify-decision-contract --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.post-publish.json
jq -e '.overall_verdict == "pass"' \
  .runs/m40_architecture_shared_core_follow_on_plan/verify-decision-contract.post-publish.json

cargo xtask family corpus-decision --format json | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.post-publish.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.post-publish.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.post-publish.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .runs/m40_architecture_shared_core_follow_on_plan/corpus-decision.post-publish.json

cargo test -p xtask | tee \
  .runs/m40_architecture_shared_core_follow_on_plan/xtask-test.post-publish.txt

rg -n "^# M40 Orchestration Plan|^## Summary|^## Hard Guards|^## Workstream Plan|Local implementation milestone|Cross-crate implementation milestone|Further evidence milestone|No new milestone yet" ORCH_PLAN.md
rg -n "^## Seam Boundary|^## Trigger Table|^## M41 Authorization Gate" ORCH_PLAN.md
```

Pass criteria:

- live checkout branch is `feat/corpus-expansion`
- live checkout HEAD equals the published accepted result
- the three proof commands remain green after publish
- live `ORCH_PLAN.md` still contains the accepted M40 contract and future M41 gate language
- `publish-verification.md` and `publish-result.json` are complete

#### Gate `gate-m40-100-final-closeout`

Objective:

- finish with one exact honest verdict and stop

Tasks:

1. confirm `queue.json` marks `task-m40-90-publish-to-live` and `gate-m40-95-post-publish-verification` as `complete`
2. write `closeout.md`
3. mark `gate-m40-100-final-closeout` as `complete`
4. stop the run after the closeout verdict is written

### Prospective M41 lane model

This section exists now because `PLAN.md` explicitly requires a future parallelization strategy if implementation is ever authorized. Nothing in this section authorizes M41 by itself.

#### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Freeze local seam foundation | `xtask/src/family/decision_kernel.rs`, `xtask/src/family/helper_surface.rs`, new local seam module under `xtask/src/family/` | none |
| Rewire read-side consumers | `xtask/src/family/verify.rs`, `xtask/src/family/recommend.rs`, `xtask/src/family/promotion_artifacts.rs` | Freeze local seam foundation |
| Command and proof-wall adoption | `xtask/src/lib.rs`, `xtask/src/family/mod.rs`, `ORCH_PLAN.md`, `.runs/**` proof artifacts | Rewire read-side consumers |
| Docs and closeout sync | `PLAN.md`, `TODOS.md`, milestone closeout docs | Freeze local seam foundation |

#### Parallel lanes

- **Lane A:** freeze local seam foundation  
  Sequential. Shared semantic source of truth. This lane must land first.

- **Lane B:** rewire read-side consumers  
  Starts after Lane A freezes the interface.

- **Lane C:** docs and closeout sync  
  Starts after Lane A freezes the interface. Can run in parallel with Lane B.

- **Lane D:** command and proof-wall adoption  
  Starts after Lane B. This is the integration lane.

#### Execution order

```text
Lane A
  |
  +--> Lane B
  |
  +--> Lane C
         |
         +--> Lane D after Lane B finishes
```

Launch **Lane B + Lane C** in parallel only after **Lane A** has frozen the local seam shape. Merge both. Then run **Lane D** as the proof-wall and command-surface integration lane.

#### Conflict flags

- Lane A and Lane B both touch `xtask/src/family/`. They must not overlap in time.
- Lane B and Lane D both touch command-adjacent family surfaces. D waits on B.
- Lane C should avoid editing any code module under `xtask/src/family/`. Keep it docs-only.

## Context-Control Rules

- The parent is the only integrator, the only run-state author, the only lane launcher, the only stale-lane invalidator, the only publisher, and the only closeout author.
- Lane B is an optimization, not a second source of truth.
- If Lane B is launched, it receives only the frozen contract packet from `lane-b-launch.md`. It does not reinterpret `PLAN.md`.
- If live decision truth drifts after baseline capture, the parent either re-baselines from the new truth or stops. It does not silently continue on mixed evidence.
- If `PLAN.md`, `TODOS.md`, or the M39 closeout input change after authority freeze, all worktrees are stale.
- If `ORCH_PLAN.md` changes on `ws/m40-int` after Lane B launches, Lane B is stale unless the parent explicitly records the conflict as non-overlapping in `session.log`.
- If the proof-command tuple changes after draft freeze, Lane B is stale even if its branch still merges cleanly.
- No run-state file is considered authoritative unless it exists under the current `ACTIVE_RUN_ROOT`.
- The parent reviews summaries plus narrow diffs only. It does not treat optional-lane transcript prose as authority.
- Optional lanes are closed immediately after merge or invalidation.

## Tests And Acceptance

### Required planning-time commands

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Baseline Decision Wall

Run on the live `feat/corpus-expansion` checkout before any worktree exists:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Integration Decision Wall

Run on `ws/m40-int` after optional Lane B merge or `lane-b-skip.json`:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Post-Publish Decision Wall

Run on the live `feat/corpus-expansion` checkout immediately after `task-m40-90-publish-to-live`:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Acceptance checklist

M40 is not complete until all of these are true:

1. `baseline.json` and the three baseline proof captures prove the live M40 decision floor on the baseline SHA.
2. `authority-freeze.json` proves the exact authority inputs and branch/worktree layout.
3. `draft-contract-freeze.json` proves the exact M40 draft contract freeze SHA and required proof-command tuple.
4. `verify-decision-contract.integration.json`, `corpus-decision.integration.json`, and `xtask-test.integration.txt` prove the accepted integration tree stayed green.
5. `plan-contract-audit.md` proves the accepted `ORCH_PLAN.md` is a faithful M40 operator contract and still planning-only.
6. `acceptance.md` maps every gate in this file to a pass or blocker outcome.
7. `publish-result.json` and `publish-verification.md` prove the accepted result landed back on live `feat/corpus-expansion` and stayed green.
8. `closeout.md` ends with exactly one allowed verdict.

## Closeout Rules

Allowed final verdicts:

1. `architecture follow-on plan authored; implementation still gated`
2. `author_architecture_follow_on_plan still required`
3. `keep the kernel local`

Verdict selection rules:

- `architecture follow-on plan authored; implementation still gated` is allowed only if:
  - the accepted `ORCH_PLAN.md` fully reflects the M40 planning contract
  - the three proof commands are green on the accepted integration tree
  - publish and post-publish verification both pass
  - M40 never widened scope into implementation authority
- `author_architecture_follow_on_plan still required` is required if:
  - the run cannot land an accepted M40 contract back onto the live branch
  - live decision truth changes enough that the current M40 authority is stale
  - proof-floor capture or plan-contract audit blocks completion before publish
- `keep the kernel local` is required if:
  - the only way to complete the run would be to authorize code motion, schema growth, corpus spend, or cross-crate extraction
  - any attempted fix for the run breaks the local-first extraction rule
  - M40 would otherwise turn planning discipline into accidental implementation authority

Final file rule:

- The last non-empty line of `closeout.md` must be exactly one of the three allowed verdict strings above.
- No alternate verdict wording is allowed.
