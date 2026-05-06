# M38 Orchestration Plan

Status: **authoritative execution contract for the M38 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Baseline HEAD: **`e04d2fa9059c0010f84bd1f2b150feee6246bb84`** (`e04d2fa`)  
Last rewritten: **`2026-05-05`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_trigger_gating`**  
Optional probe root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_non_author_probe`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Recommendation artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Decision artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`**  
Execution note: **M38 is the architecture follow-on trigger-gating milestone only. It is not a new runtime extraction, not a `spec-core` move, not a schema or artifact expansion, not corpus run `1` activation, and not a synthetic pressure exercise.**

## Summary

- This run is for **M38 - Architecture Follow-On Trigger Gating After M37** only.
- `PLAN.md` is milestone authority. This file is the parent-owned operator contract for executing that authority without improvisation.
- The parent agent is the sole baseline capturer, sole authority freeze owner, sole worktree creator, sole stale-lane invalidator, sole verification gatekeeper, sole merge authority, sole final verifier, and sole closeout author.
- Honest concurrency is conditional and capped:
  - `0` before `authority-freeze.json`
  - `1` through baseline confirmation and parent plan spine work
  - `2` maximum after `rewrite-freeze.json`, and only if one or both optional lanes are explicitly activated
  - `1` again for final verification and closeout
- The critical path is fixed:
  1. capture baseline on live `feat/corpus-expansion`
  2. freeze `PLAN.md` and `ORCH_PLAN.md`
  3. create the parent integration spine worktree
  4. execute the parent plan spine: confirm the M37 semantic floor, lock the M38 trigger ledger, and decide whether optional lanes exist at all
  5. write `rewrite-freeze.json`
  6. optionally launch Lane B and/or Lane C from that exact freeze SHA
  7. merge or accept optional lane outputs back into `ws/m38-int`
  8. rerun the full verification wall
  9. write one exact allowed closeout statement and stop
- The frozen semantic floor must remain exact throughout the run:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "not_recommended"`
  - `open_blockers = ["helper_surface_not_promotable"]`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`

## Hard Guards

- `PLAN.md` wins over this file, worker notes, stale worktrees, and run-state summaries if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Optional lanes do not edit it.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion` is the baseline and publish target, not the merge surface.
- All proving happens on `ws/m38-int`, not on the live checkout.
- M38 must not introduce:
  - any new crate
  - any `spec-core` move
  - any new artifact kind
  - any schema version bump
  - any public semantic fingerprint fields
  - any corpus run `1` activation
  - any synthetic second durable wedge
  - any synthetic second consumer
  - any new runtime extraction framed as "small enough to sneak in"
- The only authorized deliverables are:
  - authoritative trigger gating
  - baseline verification
  - optional real non-author maintainer probe evidence
  - optional tiny local helper-surface warning cleanup, if explicitly chosen
- Optional warning cleanup is hygiene only. It must stay tiny, local to `xtask/src/family/helper_surface.rs`, and non-milestone-defining.
- Optional probe evidence is honest only if a real non-author maintainer is available. Substituting the original author, simulating a second maintainer, or inventing a second consumer invalidates the run.
- No lane edits anything under:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`, except parent-owned authority-plan alignment or final closeout note if explicitly required
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `.runs/m38_trigger_gating/**` is parent-owned only.
- `.runs/m38_non_author_probe/summary.md` is the only non-parent run artifact an optional worker lane may author, and only if Lane C is explicitly launched.
- If `PLAN.md` or `ORCH_PLAN.md` changes after `authority-freeze.json`, the run stops and restarts from a fresh baseline.
- If overlapping local edits exist on owned surfaces before freeze, the parent either re-anchors around them or blocks the run. It does not overwrite them silently.
- If no warning cleanup is chosen and no real non-author probe can run, the run stays sequential. The parent does not fabricate work just to justify parallelism.

## Closed Implementation Surface

| Path | M38 responsibility | Lane owner |
|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | authority file for the trigger matrix, probe contract, verification floor, and allowed closeout lines | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` | execution contract, branch/worktree rules, freeze rules, stale-lane rules, verification wall, closeout discipline | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs` | optional tiny warning cleanup only; no semantic policy change, no new helpers outside the existing local surface | Optional Lane B |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_trigger_gating/**` | canonical run-state, launch packets, proof log, acceptance record, blocked state, closeout record | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_non_author_probe/summary.md` | optional probe outcome, evidence mapping, skip or inconclusive recording | Optional Lane C |

Rules for the closed surface:

- Any edit outside this table is out of scope unless it is mechanically forced by merge conflict resolution and is recorded in `merge-log.md`.
- Lane B owns exactly one source file. If it needs any second source file, it is out of scope for M38 and must stop.
- Lane C owns exactly one summary artifact. It does not rewrite the trigger matrix, reopen scope, or add a synthetic consumer story.
- `.semantic-family-artifacts/**` are read-only derived inputs for baseline and final verification.

## Branch And Worktree Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Canonical branches and worktrees:

| Role | Branch | Worktree |
|---|---|---|
| Live baseline and publish target | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` |
| Parent integration spine | `ws/m38-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int` |
| Optional Lane B, helper-surface warning cleanup | `ws/m38-lane-b-warning-cleanup` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-b-warning-cleanup` |
| Optional Lane C, non-author probe artifact lane | `ws/m38-lane-c-non-author-probe` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-c-non-author-probe` |

Creation rules:

1. The parent captures baseline on the live checkout before creating any M38 worktree.
2. `ws/m38-int` is created from the exact SHA recorded in `integration-base.txt`.
3. Optional lanes are created only after `rewrite-freeze.json`, and only from the exact integrated SHA recorded there.
4. Lane B is created only if maintainers explicitly choose the tiny warning cleanup.
5. Lane C is created only if a real non-author maintainer is available and the probe will actually run.
6. No optional lane forks from another optional lane.
7. No optional lane forks from the live checkout.
8. If no optional lanes are activated, no optional worktrees are created.
9. If any named worktree already exists with stale state, the parent recreates it and records that action in `session.log`.
10. If the live branch moves after baseline capture but before publish, the parent either re-baselines or explicitly merges the new live head into `ws/m38-int` and reruns the full verification wall.

Canonical worktree creation commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int \
  -b ws/m38-int <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-b-warning-cleanup \
  -b ws/m38-lane-b-warning-cleanup <REWRITE_FREEZE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-c-non-author-probe \
  -b ws/m38-lane-c-non-author-probe <REWRITE_FREEZE_SHA>
```

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_trigger_gating`
- `PROBE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_non_author_probe`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating`

Canonical parent-owned files:

- `baseline.json`
- `integration-base.txt`
- `publish-head.txt`
- `authority-freeze.json`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `run-state.json`
- `tasks.json`
- `queue.json`
- `session.log`
- `rewrite-freeze.json`
- `lane-b-launch.md`
- `lane-c-launch.md`
- `lane-b-skip.json`
- `lane-c-skip.json`
- `merge-log.md`
- `proof-log.json`
- `acceptance.md`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `closeout.md`

Required contents:

- `baseline.json`
  - live branch
  - live HEAD SHA
  - dirty-state summary
  - overlapping local edit summary on the closed implementation surface
  - current recommendation artifact path
  - current decision artifact path
  - current raw byte hashes for recommendation and decision artifacts
  - current frozen semantic output values
  - drift classification: `none` | `hygiene_only` | `trigger_relevant`
- `integration-base.txt`
  - exact SHA used to create `ws/m38-int`
- `publish-head.txt`
  - exact live HEAD SHA captured during baseline
- `authority-freeze.json`
  - snapshot paths for `PLAN.md` and `ORCH_PLAN.md`
  - frozen branch and worktree layout
  - closed implementation surface literal copy or checksum
  - explicit statement that no optional lanes are yet authorized
- `rewrite-freeze.json`
  - exact `ws/m38-int` SHA after parent plan spine work
  - exact trigger matrix values accepted by the parent
  - explicit statement whether Lane B is `launch` or `skip`
  - explicit statement whether Lane C is `launch` or `skip`
  - explicit statement that no deeper extraction is authorized by the freeze itself
  - if Lane C is `launch`, the exact probe prompt boundary and required closeout rules
- `lane-b-launch.md`
  - owned file
  - forbidden files
  - exact `PLAN.md` excerpts copied into the worker packet
  - exact commands to run
  - stale-lane invalidation triggers
  - required return contract
- `lane-c-launch.md`
  - owned file
  - forbidden files
  - exact `PLAN.md` excerpts copied into the worker packet
  - exact probe entry criteria
  - exact commands or evidence steps
  - stale-lane invalidation triggers
  - required return contract
- `lane-b-skip.json`
  - `status: skipped`
  - explicit reason for not taking optional cleanup
- `lane-c-skip.json`
  - `status: skipped`
  - explicit reason, including `no real non-author maintainer available` when applicable
- `merge-log.md`
  - source branch
  - source SHA
  - target SHA before merge
  - target SHA after merge
  - conflicts encountered
  - resolutions applied
  - whether the merge preserved the frozen semantic outputs
- `proof-log.json`
  - command
  - cwd
  - exit code
  - artifact path if applicable
  - raw byte hash if captured
  - semantic interpretation
  - pass/fail
- `acceptance.md`
  - final checklist mapped to M38 acceptance criteria
  - baseline revalidation proof
  - trigger evaluation proof
  - optional lane skip or run proof
  - final semantic output proof
  - final closeout outcome proof
- `blocked.json`
  - blocking task id
  - blocking lane
  - exact violated guard
  - restart requirement
- `closeout.md`
  - final integrated SHA
  - final live publish SHA
  - commands run
  - optional lane disposition
  - accepted deltas
  - exact final closeout statement
  - any deferred next action if a trigger was proven

Per-task sentinel directories:

- `task-m38-00-baseline-capture`
- `task-m38-05-authority-freeze`
- `task-m38-10-create-spine-worktree`
- `task-m38-20-parent-plan-spine`
- `task-m38-25-rewrite-freeze`
- `task-m38-30-launch-lane-b`
- `task-m38-31-launch-lane-c`
- `task-m38-40-merge-lane-b`
- `task-m38-45-accept-lane-c`
- `task-m38-60-final-verification-wall`
- `task-m38-65-publish-back-to-live`
- `task-m38-70-closeout`

Each task directory contains parent-written markers only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Workstream Plan

Task graph:

```text
task-m38-00-baseline-capture
  -> task-m38-05-authority-freeze
      -> task-m38-10-create-spine-worktree
          -> task-m38-20-parent-plan-spine
              -> task-m38-25-rewrite-freeze
                  -> task-m38-30-launch-lane-b (optional)
                  -> task-m38-31-launch-lane-c (optional)
                      -> task-m38-40-merge-lane-b (if launched)
                      -> task-m38-45-accept-lane-c (if launched)
                          -> task-m38-60-final-verification-wall
                              -> task-m38-65-publish-back-to-live
                                  -> task-m38-70-closeout
```

### Parent Task 1 - Baseline Capture

Owner: `Parent`  
Branch: `feat/corpus-expansion`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Record current branch, HEAD SHA, dirty state, and overlapping local edits in `baseline.json`.
2. Write the same live HEAD SHA to both `integration-base.txt` and `publish-head.txt`.
3. Capture current recommendation and decision artifact raw byte hashes.
4. Run the baseline verification floor and verify the frozen semantic outputs.
5. Classify drift as `none`, `hygiene_only`, or `trigger_relevant`.
6. Update `session.log`, `tasks.json`, and `queue.json`.

Minimum command wall:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
shasum -a 256 .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Stop conditions:

- any command fails
- any semantic assertion fails
- overlapping local edits touch the closed implementation surface and are not explicitly accepted
- drift is `trigger_relevant`

### Parent Task 2 - Authority Freeze

Owner: `Parent`  
Branch: `feat/corpus-expansion`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Snapshot `PLAN.md` and `ORCH_PLAN.md` into `authority-snapshot/`.
2. Write `authority-freeze.json`.
3. Freeze branch layout, worktree layout, implementation surface, and guard set for the run.
4. Mark all optional lanes blocked until `rewrite-freeze.json`.

Minimum command wall:

```bash
git diff -- PLAN.md ORCH_PLAN.md
git rev-parse HEAD
test -f PLAN.md
test -f ORCH_PLAN.md
```

Stop conditions:

- either authority file changes during freeze
- scope no longer matches `PLAN.md`
- new overlapping edits appear on authority files during freeze

### Parent Task 3 - Create Spine Worktree

Owner: `Parent`  
Branch: `ws/m38-int`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int`

Actions:

1. Create `ws/m38-int` from the SHA in `integration-base.txt`.
2. Verify the worktree is an exact fork from the frozen baseline.
3. Record creation details in `session.log`.

Minimum command wall:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int \
  -b ws/m38-int "$(cat .runs/m38_trigger_gating/integration-base.txt)"
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int status --short
```

Stop conditions:

- the worktree cannot be created from the exact baseline SHA
- the worktree points at the wrong SHA
- the worktree starts with unexpected dirt

### Parent Task 4 - Parent Plan Spine

Owner: `Parent only`  
Branch: `ws/m38-int`  
Path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int`

Actions:

1. Confirm the M38 authority plan states the three trigger gates, the optional probe contract, the hard boundary, and the three exact allowed closeout lines.
2. Confirm the M37 semantic floor is still the live truth being gated.
3. Decide whether Lane B is `launch` or `skip`.
4. Decide whether Lane C is `launch` or `skip`.
5. Reject any attempt to turn M38 into runtime extraction, corpus spend, or synthetic evidence generation.
6. Prepare optional launch packets or skip markers.

Minimum command wall:

```bash
git rev-parse HEAD
rg -n "no_strong_candidate|not_recommended|helper_surface_not_promotable|author_architecture_follow_on_plan" \
  PLAN.md ORCH_PLAN.md
rg -n "No deeper extraction justified yet\\. Keep the kernel local\\.|Trigger proven\\. Author the next milestone against <exact trigger>\\.|Probe inconclusive\\. Do not extract yet\\. Re-run only with real new evidence\\." \
  PLAN.md
```

Stop conditions:

- any trigger is already proven by current live truth
- the authority files disagree on the trigger matrix or closeout rules
- optional lane scope cannot be expressed inside the closed implementation surface

### Parent Task 5 - Rewrite Freeze

Owner: `Parent only`  
Branch: `ws/m38-int`

Actions:

1. Write `rewrite-freeze.json`.
2. Record the accepted parent spine SHA.
3. Record Lane B and Lane C as `launch` or `skip`.
4. Freeze the optional lane boundaries and stale-lane rules.

Minimum command wall:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int rev-parse HEAD
test -f /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_trigger_gating/authority-freeze.json
```

Stop conditions:

- optional lane status is ambiguous
- the accepted spine SHA is not recorded
- stale-lane rules are missing from the freeze

### Parent Task 6 - Launch Optional Lanes

Owner: `Parent only`  
Branch base: `rewrite-freeze.json`

Actions:

1. Create only the optional worktrees whose status is `launch`.
2. Write `lane-b-launch.md` and/or `lane-c-launch.md` when launched.
3. Write `lane-b-skip.json` and/or `lane-c-skip.json` when skipped.
4. Mark each optional lane started or skipped in `tasks.json` and `queue.json`.

Minimum command wall:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-b-warning-cleanup \
  -b ws/m38-lane-b-warning-cleanup <REWRITE_FREEZE_SHA>
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-c-non-author-probe \
  -b ws/m38-lane-c-non-author-probe <REWRITE_FREEZE_SHA>
```

Launch interpretation:

- Run only the commands that correspond to lanes whose status is `launch`.
- If Lane B is `skip`, do not create its worktree.
- If Lane C is `skip`, do not create its worktree.

Stop conditions:

- a lane worktree is created despite being marked `skip`
- a launch packet omits owned-file boundaries or stale-lane rules
- a lane is launched against the wrong freeze SHA

### Parent Task 7 - Merge Lane B

Owner: `Parent only`  
Branch: `ws/m38-int`

Actions:

1. Inspect Lane B diff against the launch contract.
2. Merge `ws/m38-lane-b-warning-cleanup` into `ws/m38-int` only if it stayed inside one-file hygiene scope.
3. Rerun `cargo test -p xtask` and the semantic floor if the merge occurs.
4. Record the merge in `merge-log.md`.

Minimum command wall:

```bash
git diff --stat ws/m38-int..ws/m38-lane-b-warning-cleanup
git merge --no-ff ws/m38-lane-b-warning-cleanup
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Stop conditions:

- Lane B edited any file other than `xtask/src/family/helper_surface.rs`
- Lane B changes semantic outputs
- Lane B turns a warning cleanup into a behavior or architecture change

### Parent Task 8 - Accept Lane C

Owner: `Parent only`  
Branch: `ws/m38-int`

Actions:

1. Inspect Lane C output against the launch contract.
2. Accept the probe summary only if a real non-author maintainer ran it and the result maps cleanly to one trigger or to an allowed inconclusive outcome.
3. Merge `ws/m38-lane-c-non-author-probe` into `ws/m38-int` only if the summary artifact is the only delta.
4. Record the result in `merge-log.md`.

Minimum command wall:

```bash
git diff --stat ws/m38-int..ws/m38-lane-c-non-author-probe
git merge --no-ff ws/m38-lane-c-non-author-probe
test -f .runs/m38_non_author_probe/summary.md
```

Stop conditions:

- Lane C lacks a real non-author maintainer
- Lane C introduces any file outside `.runs/m38_non_author_probe/summary.md`
- Lane C tries to prove more than one trigger
- Lane C relies on synthetic consumers, synthetic wedges, or vague evidence that maps to nothing

### Parent Task 9 - Final Verification Wall

Owner: `Parent only`  
Branch: `ws/m38-int`

Ordered command wall:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

What this proves:

- the baseline verification floor still holds after any optional lane work
- recommendation and decision artifacts still validate
- the semantic floor stayed frozen
- no hidden runtime extraction or public surface expansion slipped in under M38

Stop conditions:

- any command fails
- any semantic assertion fails
- optional warning cleanup changed runtime truth
- the probe result is being used to justify a broader extraction than the exact trigger permits

### Parent Task 10 - Publish Back To Live

Owner: `Parent only`  
Branches: `ws/m38-int` -> `feat/corpus-expansion`

Actions:

1. Verify the live branch still matches `publish-head.txt`, or reconcile and rerun the full verification wall.
2. Publish only from `ws/m38-int`.
3. Record the final integrated SHA and final live SHA.

Minimum command wall:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
```

Stop conditions:

- live branch moved incompatibly and was not reconciled
- `ws/m38-int` is not fully green
- publish cannot happen cleanly from the accepted integration branch

### Parent Task 11 - Closeout

Owner: `Parent only`  
Branch: `feat/corpus-expansion`

Actions:

1. Write `acceptance.md` and `closeout.md`.
2. Mark the queue complete.
3. Record exact optional lane dispositions: `skipped`, `merged`, `accepted`, or `not run`.
4. Write one exact allowed final closeout statement.

Minimum command wall:

```bash
git rev-parse HEAD
test -f .runs/m38_trigger_gating/acceptance.md
test -f .runs/m38_trigger_gating/closeout.md
```

Stop conditions:

- acceptance evidence is incomplete
- the final closeout line is not one of the three allowed outcomes
- closeout attempts to authorize broader work than the exact trigger permits

## Lane Definitions

### Lane A - Parent Spine

Owner: `Parent only`  
Branch: `ws/m38-int`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/int`  
Starts after: `task-m38-10-create-spine-worktree`  
Concurrency during this lane: `1`

Mission:

- own baseline confirmation and authority freeze
- own trigger-ledger confirmation and optional-lane decisions
- own all run-state artifacts under `.runs/m38_trigger_gating`
- own all merge and verification gates
- own the final closeout statement

Owned files:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_trigger_gating/**`

Acceptance criteria for Lane A:

- baseline floor is reproduced
- authority freeze is written before any optional lane launch
- `rewrite-freeze.json` records explicit `launch` or `skip` for both optional lanes
- no synthetic pressure source is authorized
- final closeout uses one exact allowed statement

### Lane B - Optional Helper-Surface Warning Cleanup

Owner: `Worker, GPT-5.4 high`  
Branch: `ws/m38-lane-b-warning-cleanup`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-b-warning-cleanup`  
Starts after: `rewrite-freeze.json`, only if `Lane B = launch`  
Concurrency during this lane: at most one optional lane

Mission:

- keep the warning cleanup tiny and local
- touch only `xtask/src/family/helper_surface.rs`
- preserve the exact semantic outputs
- stop immediately if the cleanup wants to become real milestone work

Owned file:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`

Readable but not writable:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`

Lane command wall:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Acceptance criteria for Lane B:

- only `helper_surface.rs` changed
- no new runtime surface or trigger policy was introduced
- the exact semantic floor is preserved
- no second file was required

### Lane C - Optional Non-Author Probe Artifact

Owner: `Worker, GPT-5.4 high`  
Branch: `ws/m38-lane-c-non-author-probe`  
Worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating/lane-c-non-author-probe`  
Starts after: `rewrite-freeze.json`, only if `Lane C = launch`  
Concurrency during this lane: at most one optional lane

Mission:

- run the real non-author maintainer legibility dry run
- record the result without inventing new pressure
- map any failure to exactly one listed trigger or return `inconclusive`
- stop rather than generalize

Owned file:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m38_non_author_probe/summary.md`

Readable but not writable:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Lane evidence steps:

1. confirm a real non-author maintainer is actually available
2. run the baseline command floor from the frozen M38 branch state
3. ask the maintainer to explain:
   - why the wedge remains `helper_surface_not_promotable`
   - why the kernel stays in `xtask/src/family/`
   - why corpus run `1` remains unspent
   - why semantic fingerprints stay internal only
4. record confusion, hidden context requests, or extraction requests
5. map any failure to exactly one trigger or mark the run `inconclusive`

Acceptance criteria for Lane C:

- exactly one file changed
- the participant is a real non-author maintainer
- result is `pass`, `fail`, or `inconclusive`
- any `fail` maps to exactly one trigger
- no synthetic consumer or synthetic wedge evidence was used

## Worker Launch Packets

Each optional worker launch packet is a parent-authored, single-source execution note. It must be written to the corresponding `lane-*.md` file and delivered verbatim.

Required prompt ingredients for every worker packet:

1. Milestone title: `M38 - Architecture Follow-On Trigger Gating After M37`.
2. Authority path: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`.
3. Launch freeze SHA from `rewrite-freeze.json`.
4. Worker branch name and worktree path.
5. Worker model requirement:
   - `GPT-5.4`
   - `reasoning_effort=high`
6. Exact owned files.
7. Exact forbidden files.
8. Exact readable reference files.
9. Exact `PLAN.md` excerpt list for that lane.
10. Exact command wall or evidence steps.
11. Hard guards copied verbatim:
   - no new crate
   - no `spec-core` move
   - no new artifact kind
   - no schema version bump
   - no public fingerprint fields
   - no corpus run `1` activation
   - no synthetic second wedge
   - no synthetic second consumer
12. Merge policy: worker does not merge or publish.
13. Stale-lane rules.
14. Required return format.

Required return format for every optional worker:

```text
RESULT
- status: ready-to-merge | skipped | blocked | inconclusive
- branch: <worker-branch>
- base-freeze-sha: <rewrite-freeze-sha>
- head-sha: <worker-head-sha>

FILES
- <absolute path>

COMMANDS
- <command or evidence step> -> <exit/result summary>

CHECKS
- owned-surface-only: yes|no
- synthetic-pressure-used: yes|no
- exact-trigger-mapping-or-not-applicable: yes|no

NOTES
- <brief operator note>

BLOCKERS
- <exact blocker or "none">
```

Worker stale-lane rules:

- If `rewrite-freeze.json` changes after packet launch, every launched optional lane is stale.
- If `PLAN.md` or `ORCH_PLAN.md` changes after packet launch, every launched optional lane is stale.
- If Lane B needs more than `helper_surface.rs`, Lane B is stale and stops.
- If Lane C cannot secure a real non-author maintainer, Lane C becomes `skipped`, not stretched into synthetic evidence generation.
- If any optional lane edits a forbidden file, that lane is stale.
- Stale lanes are not repaired in place. The parent recreates their worktrees from the new freeze SHA and relaunches them if still justified.

## Context-Control Rules

Every optional worker launch note must contain:

- the exact branch and worktree path
- the exact frozen base SHA from `rewrite-freeze.json`
- the exact owned file set
- the exact forbidden file set
- the exact commands or evidence steps the worker is expected to run
- the exact return contract
- the exact stale-lane invalidation rules
- verbatim copies of the relevant `PLAN.md` excerpts listed below

Required `PLAN.md` excerpts by worker:

- Lane B packet must include:
  - `Optional Hygiene`
  - `Verification floor`
  - `Acceptance matrix`
  - `Failure Modes Registry`
- Lane C packet must include:
  - `Phase 3 - Optional evidence probe`
  - `Probe verdict rules`
  - `Acceptance matrix`
  - `M39 Authorization Rule`

Shared worker prohibitions:

- no worker edits `PLAN.md`
- no worker edits `ORCH_PLAN.md`
- no worker edits `.semantic-family-artifacts/**`
- no worker merges branches
- no worker broadens the trigger matrix
- no worker spends corpus run `1`
- no worker creates or requests a synthetic second wedge or synthetic second consumer

## Diff Inspection Before Merge

The parent must inspect every optional worker diff before merge.

For Lane B, confirm:

- only `helper_surface.rs` changed
- the change is warning hygiene only
- no semantic floor drift is visible
- no follow-on extraction justification was smuggled in

For Lane C, confirm:

- only `.runs/m38_non_author_probe/summary.md` changed
- the summary names the real participant role without substituting the original author
- any failure maps to exactly one trigger
- any inconclusive outcome stays inconclusive and does not authorize extraction

## Conflict Rules

- `PLAN.md` and `ORCH_PLAN.md` are parent-owned only.
- Lane B is the single writer for `helper_surface.rs` only if it is explicitly launched.
- Lane C is the single writer for `.runs/m38_non_author_probe/summary.md` only if it is explicitly launched.
- If Lane B asks for a second file or behavior change, the lane is stale immediately.
- If Lane C asks to revise the trigger matrix, the lane is stale immediately.
- If any merge conflict touches `PLAN.md` or `ORCH_PLAN.md` after `authority-freeze.json`, the run stops and restarts from a fresh baseline.
- If any merge conflict touches `helper_surface.rs`, the parent resolves it only if the result remains one-file hygiene. Otherwise the run stops.
- If any merge conflict touches `.runs/m38_non_author_probe/summary.md`, the parent either accepts the single summary artifact or relaunches Lane C. It does not hand-edit synthetic evidence into place.

## Verification Wall And Acceptance Wall

### Required verification commands

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

### Required unchanged-semantic assertions

```bash
jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

### Acceptance checklist

1. Baseline revalidation passed on the live branch.
2. The semantic floor remained exact through final verification.
3. All three trigger verdicts remain explicit and current.
4. Lane B is either skipped or merged as one-file hygiene only.
5. Lane C is either skipped because no real non-author maintainer was available, accepted as `pass`, accepted as a one-trigger `fail`, or accepted as `inconclusive`.
6. No synthetic second wedge, synthetic second consumer, public fingerprint field, schema change, or corpus run `1` activation was introduced.
7. The final closeout line is exactly one allowed outcome.

## Blocking Rules

- Stop immediately if the baseline verification floor drifts from the frozen semantic outputs.
- Stop immediately if any trigger is already true before optional lanes start. In that case M38 closes as authoring input for the exact next milestone, not as a deeper execution run.
- Stop immediately if Lane B expands beyond tiny local warning cleanup.
- Stop immediately if Lane C cannot prove the participant is a real non-author maintainer.
- Stop immediately if any optional lane relies on synthetic evidence.
- Stop immediately if the final closeout statement is ambiguous or names more than one trigger.
- Restart from a fresh baseline if the live branch moves and the parent cannot safely reconcile it into `ws/m38-int` with a rerun of the full verification wall.

## Closeout Contract

The final closeout statement must be exactly one of:

1. `No deeper extraction justified yet. Keep the kernel local.`
2. `Trigger proven. Author the next milestone against <exact trigger>.`
3. `Probe inconclusive. Do not extract yet. Re-run only with real new evidence.`

Allowed `<exact trigger>` values are:

- `generalized multi-wedge decision layer`
- `cross-crate family-analysis shared core`
- `public semantic fingerprint fields`

Closeout mapping rules:

- Outcome `1` is allowed when baseline truth stayed frozen and no trigger was proven. The probe may be skipped or may pass.
- Outcome `2` is allowed only when exactly one trigger was proven by real evidence. It does not authorize any broader extraction than that named trigger.
- Outcome `3` is allowed only when the probe actually ran and the evidence quality was too weak to map honestly to one trigger.
- `probe not run, no real non-author maintainer available` is a valid lane disposition, but it still closes with outcome `1`, not with a special fourth outcome.
- No closeout line may reopen corpus run `1`, authorize multiple triggers, or smuggle in a "small" shared-core move.

## Assumptions

- `PLAN.md` at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` remains the M38 authority for the duration of the run.
- The baseline branch is `feat/corpus-expansion` at `e04d2fa9059c0010f84bd1f2b150feee6246bb84`.
- The repo can create the listed worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m38-trigger-gating`.
- The required proving surface is the five-command verification floor plus the exact semantic assertions.
- The current local `PLAN.md` state is accepted as authority input and is not rewritten by optional lanes.
- A real non-author maintainer may not be available during the milestone window. If not, Lane C is skipped and the run stops after trigger-ledger completion and final verification.
