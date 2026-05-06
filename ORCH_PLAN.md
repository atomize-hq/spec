# M39 Orchestration Plan

Status: **authoritative execution contract for the M39 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Baseline HEAD: **`0f8202c35e29f1db67a0dbc15e1c664175e80eef`** (`0f8202c`)  
Last rewritten: **`2026-05-06`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe`**  
Canonical command: **`cargo xtask family verify-decision-contract --format json`**  
Execution note: **M39 is the verification-consumer probe only. It proves or rejects one read-side consumer claim. It does not move `decision_kernel.rs`, does not widen public schemas, does not rescan raw corpus inputs, does not add path override flags, and does not grow a generic verification framework.**

## Summary

- `PLAN.md` is milestone authority. This file is the operator contract for executing that authority without improvisation.
- The parent agent owns the full critical path: baseline capture, authority freeze, implementation, verifier contract freeze, parity proof, merge, pre-publish acceptance, publish, post-publish verification, and final closeout.
- There is exactly one honest optional parallel seam in M39:
  - after the verifier contract is frozen
  - only if `ORCH_PLAN.md` adoption work is still open
  - only as a dedicated worktree/subagent lane for `ORCH_PLAN.md`
- Concurrency is capped by phase:
  - `0` before baseline capture finishes
  - `1` through authority freeze and verifier contract freeze
  - `2` maximum after optional lane launch
  - `1` again for merge, parity proof, and closeout
- M39 is not complete until the repo proves all of these together:
  - the new verifier command exists and is green on the frozen helper-surface floor
  - parity with the legacy shell ladder is recorded on the same artifact set
  - repo-root orchestration adopts the verifier in standing proof walls
  - `closeout.md` ends with exactly one allowed verdict

## Hard Guards

- `PLAN.md` wins over this file, worker notes, stale worktrees, and run-state summaries if they disagree.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion` is the baseline and publish target, not the merge surface.
- All parent-owned implementation, proof capture, and integration work happens on `ws/m39-int`, not on the live checkout.
- M39 must not introduce:
  - any move of `xtask/src/family/decision_kernel.rs`
  - any new public artifact schema field or schema version bump
  - any new persisted artifact kind
  - any rescan of raw corpus inputs
  - any path override flags for verifier inputs
  - any generic verification framework, trait system, registry, or reusable abstraction beyond this exact command
  - any new recommendation policy path
  - any second semantics path that recomputes coverage or recommendation from source corpus data
- The only authorized command surface for the new consumer is:
  - `cargo xtask family verify-decision-contract --format json`
- The verifier is read-side only. It may read canonical latest artifacts and call existing validators and kernel helpers only.
- `PLAN.md` is frozen after `authority-freeze.json`.
- `ORCH_PLAN.md` is frozen after `authority-freeze.json` except for the explicit M39 adoption edits performed either by the parent or by launched Lane B under this contract.
- Parent-owned canonical run-state remains writable by the parent for the full run. Freeze records and snapshots are immutable once written, but the parent must keep appending or writing later run-state artifacts such as `implementation-contract-freeze.json`, `proof-log.json`, `parity-proof.md`, `acceptance.md`, publish records, and `closeout.md`.
- If `PLAN.md` changes after `authority-freeze.json`, the run stops and restarts from a fresh baseline.
- If Lane B is not launched, only the parent may edit `ORCH_PLAN.md` after `authority-freeze.json`.
- If Lane B is launched, Lane B owns `ORCH_PLAN.md` in its worktree until it returns or is invalidated. The parent remains the only merge authority.
- `.runs/m39_verification_consumer_probe/**` is parent-owned only. Optional lanes do not write canonical run-state.
- No lane edits `.semantic-family-artifacts/**`. Those files are read-only derived inputs.
- If the verifier command string, required JSON contract, required failure reasons, or named proof-wall contract changes after `implementation-contract-freeze.json`, every launched optional lane is stale and must be recreated from a new freeze SHA.
- ORCH adoption is part of proving the consumer. If standing proof walls do not adopt the verifier, the closeout verdict cannot be `third honest consumer proven`.

## Closed Implementation Surface

| Path | M39 responsibility | Owner |
|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | milestone authority and freeze reference | Parent only |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` | execution contract, verifier proof walls, adoption proof surface, closeout discipline | Parent through authority freeze; Optional Lane B only after launch; parent merges final |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs` | CLI wiring for `family verify-decision-contract` | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs` | verifier module registration only | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs` | verifier implementation, local JSON result structs, unit tests | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs` | validator/helper exposure only if strictly required without schema widening | Parent Lane A |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs` | semantic source of truth reused unchanged | Read-only input |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/paths.rs` | canonical latest-path source reused unchanged | Read-only input |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs` | canonical latest-path and artifact relationship reused unchanged | Read-only input |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe/**` | baseline records, freeze records, launch packets, proof capture, acceptance, blocked state, closeout | Parent only |

Rules for the closed surface:

- Any edit outside this table is out of scope unless a merge conflict mechanically forces it and the parent records that in `merge-log.md`.
- Lane B owns exactly one authored file: `ORCH_PLAN.md`.
- There is no honest Lane C in M39. Closeout, acceptance, parity proof, and final verdict remain parent-owned because they depend on merged truth.
- Read-only input paths may be read for parity and validation, but changing them violates M39 scope.

## Branch And Worktree Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Canonical branches and worktrees:

| Role | Branch | Worktree |
|---|---|---|
| Live baseline and publish target | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` |
| Parent integration spine | `ws/m39-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe/int` |
| Optional Lane B, `ORCH_PLAN.md` adoption lane | `ws/m39-lane-b-orch-adoption` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe/lane-b-orch-adoption` |

Creation rules:

1. The parent captures baseline on the live checkout before creating any M39 worktree.
2. `ws/m39-int` is created from the exact SHA recorded in `integration-base.txt`.
3. Lane B may be created only after `implementation-contract-freeze.json` exists and only from the exact freeze SHA recorded there.
4. Lane B is launched only if `ORCH_PLAN.md` adoption work is still open after contract freeze. If adoption is already complete on the freeze commit, write `lane-b-skip.json` and stay sequential.
5. Lane B never forks from the live checkout. It forks only from the frozen integration SHA.
6. If a named M39 worktree already exists with stale state, the parent recreates it and records that in `session.log`.
7. The live branch moving after baseline capture does not silently change the run. The parent must either re-baseline or explicitly merge the new live head into `ws/m39-int` and rerun the full verification wall.
8. No optional lane writes back to the live checkout directly.

Canonical worktree creation commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe/int \
  -b ws/m39-int <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe/lane-b-orch-adoption \
  -b ws/m39-lane-b-orch-adoption <IMPLEMENTATION_CONTRACT_FREEZE_SHA>
```

## Canonical Run-State

Parent-owned orchestration truth uses a phase-specific active root:

- `LIVE_PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `INT_PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe/int`
- `ACTIVE_PARENT_ROOT=LIVE_PRIMARY_ROOT` through `gate-m39-10-authority-freeze`
- `ACTIVE_PARENT_ROOT=INT_PRIMARY_ROOT` from `task-m39-20-create-integration-worktree` through `gate-m39-80-pre-publish-acceptance`
- `ACTIVE_PARENT_ROOT=LIVE_PRIMARY_ROOT` from `task-m39-90-publish-to-live` through `gate-m39-100-final-closeout`
- `ACTIVE_RUN_ROOT=$ACTIVE_PARENT_ROOT/.runs/m39_verification_consumer_probe`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m39-verification-consumer-probe`

Run-state handoff rules:

- Before `task-m39-20-create-integration-worktree`, the live checkout copy is canonical because the integration worktree does not exist yet.
- `task-m39-20-create-integration-worktree` must copy the already-written parent run-state from `LIVE_PRIMARY_ROOT` into `INT_PRIMARY_ROOT` before parent implementation work continues.
- After that copy, the integration worktree copy is canonical for all parent-owned run-state, proof, merge, and pre-publish acceptance artifacts.
- The live checkout copy becomes read-only until `task-m39-90-publish-to-live` or full restart.
- `task-m39-90-publish-to-live` must land the accepted `ws/m39-int` result onto the live checkout and then make the live checkout copy canonical again for post-publish verification and final closeout.

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
- `legacy-shell-baseline.md`
- `implementation-contract-freeze.json`
- `lane-b-launch.md`
- `lane-b-skip.json`
- `merge-log.md`
- `proof-log.json`
- `verify-decision-contract.stdout.json`
- `parity-proof.md`
- `orch-adoption-proof.md`
- `acceptance.md`
- `publish-verification.md`
- `publish-result.json`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `closeout.md`

Required contents:

- `baseline.json`
  - live branch
  - live HEAD SHA
  - dirty-state summary for the closed implementation surface
  - authoritative artifact paths
  - raw byte hashes for the recommendation and decision artifacts
  - frozen helper-surface floor tuple captured from legacy shell proof
- `integration-base.txt`
  - exact SHA used to create `ws/m39-int`
- `publish-head.txt`
  - exact live HEAD SHA captured during baseline
- `authority-freeze.json`
  - snapshot paths for `PLAN.md` and `ORCH_PLAN.md`
  - frozen branch and worktree layout
  - closed implementation surface checksum or literal copy
  - explicit statement that no optional lane is yet authorized
- `run-state.json`
  - `current_task_id`
  - `current_task_status`
  - `active_parent_root`
  - `active_run_root`
  - `live_publish_status: not_started|in_progress|blocked|complete`
  - `last_updated_at`
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
    - `gate-m39-00-baseline-capture`
    - `gate-m39-10-authority-freeze`
    - `task-m39-20-create-integration-worktree`
    - `gate-m39-30-implementation-contract-freeze`
    - `gate-m39-40-optional-lane-launch`
    - `gate-m39-50-merge-and-integration`
    - `gate-m39-60-parity-proof`
    - `gate-m39-70-orch-adoption-proof`
    - `gate-m39-80-pre-publish-acceptance`
    - `task-m39-90-publish-to-live`
    - `gate-m39-95-post-publish-verification`
    - `gate-m39-100-final-closeout`
- `legacy-shell-baseline.md`
  - the exact baseline commands
  - exit codes
  - captured tuple results proving the M38 floor before implementation starts
- `implementation-contract-freeze.json`
  - exact committed `ws/m39-int` SHA after parent verifier work is green
  - frozen command string
  - required top-level JSON keys
  - required `checks` keys
  - required failure reasons
  - explicit `lane_b_status: launch|skip`
  - explicit statement that no wider verifier framework is authorized
- `lane-b-launch.md`
  - owned file: `ORCH_PLAN.md`
  - forbidden files
  - exact verifier command literal
  - exact required proof walls to adopt
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
  - whether verifier contract freeze remained intact
- `proof-log.json`
  - command
  - cwd
  - exit code
  - artifact path if applicable
  - pass/fail
  - semantic interpretation
- `verify-decision-contract.stdout.json`
  - verbatim stdout capture from the canonical verifier command on the accepted run
- `parity-proof.md`
  - legacy shell path commands and results
  - verifier command and result
  - exact artifact hashes proving both ran on the same inputs
- `orch-adoption-proof.md`
  - named proof walls present in `ORCH_PLAN.md`
  - confirmation each standing wall uses the verifier command
  - explicit note whether Lane B was used or skipped
- `acceptance.md`
  - final checklist mapped to M39 acceptance gates
  - baseline capture proof
  - contract freeze proof
  - lane launch or skip proof
  - merge and parity proof
  - ORCH adoption proof
  - publish proof
  - final verdict proof
- `publish-verification.md`
  - live checkout branch and HEAD after publish
  - commands and exit codes for post-publish verification
  - confirmation that live `feat/corpus-expansion` matches the accepted `ws/m39-int` result
- `publish-result.json`
  - `publish_source_branch`
  - `publish_source_sha`
  - `publish_target_branch`
  - `publish_target_sha_before`
  - `publish_target_sha_after`
  - `publish_method: fast_forward|merge`
  - `status: complete|blocked`
- `blocked.json`
  - blocking task id
  - blocking lane
  - blocking reason
  - whether the honest verdict is forced to `keep the kernel local`
- `closeout.md`
  - short narrative of what was proven
  - explicit adoption result
  - exact final verdict as the last non-empty line

## Workstream Plan

### Parent Lane A - critical path only

The parent owns the only non-optional workstream. No subagent is launched before `implementation-contract-freeze.json`.

Before the first gate starts, the parent initializes `run-state.json` and `queue.json`, marks `gate-m39-00-baseline-capture` as `active`, and leaves every later entry as `pending` until promoted by the parent. `queue.json` is the ordering ledger. Prose in this file does not override the ledger.

#### Gate `gate-m39-00-baseline-capture`

Objective:

- prove the current M38 helper-surface floor on the live branch before any M39 implementation begins
- record authoritative artifact hashes and live HEAD

Commands:

```bash
git branch --show-current
git rev-parse HEAD
git status --short PLAN.md ORCH_PLAN.md xtask/src/lib.rs xtask/src/family

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.evidence_summary.missing_evidence == []' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.evidence_summary.stale_evidence == []' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Pass criteria:

- all artifact validation and `jq` checks exit `0`
- the tuple matches the frozen helper-surface floor from `PLAN.md`
- `baseline.json`, `publish-head.txt`, and `legacy-shell-baseline.md` are written before any worktree is created

#### Gate `gate-m39-10-authority-freeze`

Objective:

- freeze the authoritative planning inputs and run layout before implementation starts

Tasks:

1. Snapshot `PLAN.md` and `ORCH_PLAN.md` into `authority-snapshot/`.
2. Write `authority-freeze.json`.
3. Stop the run if `PLAN.md` changes after this point.

Pass criteria:

- `authority-freeze.json` exists
- `PLAN.md` and `ORCH_PLAN.md` snapshots are recorded
- no optional lane is yet authorized

#### Task `task-m39-20-create-integration-worktree`

Objective:

- create the parent integration spine and move canonical run-state into it

Tasks:

1. Write `integration-base.txt` from the baseline SHA.
2. Create `ws/m39-int`.
3. Copy `.runs/m39_verification_consumer_probe/` into the integration worktree.
4. Continue all parent work from `INT_PRIMARY_ROOT`.

#### Gate `gate-m39-30-implementation-contract-freeze`

Objective:

- finish the verifier implementation and lock the exact contract before any optional parallel adoption lane can begin

Parent-owned implementation scope:

1. add the `VerifyDecisionContract { format: String }` CLI surface
2. register `verify.rs`
3. implement the verifier against canonical latest artifacts
4. enforce the frozen helper-surface floor exactly
5. add tests for happy path and all required failure reasons
6. prove non-`json` format rejects

Commands:

```bash
cargo test -p xtask
cargo xtask family verify-decision-contract --help
cargo xtask family verify-decision-contract --format json | tee \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
jq -e '.overall_verdict == "pass"' \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
```

Pass criteria:

- the exact command exists and is green on the frozen helper-surface floor
- stdout JSON includes the required top-level keys, required `checks` keys, and stable failure reasons from `PLAN.md`
- `cargo test -p xtask` is green
- `implementation-contract-freeze.json` is written with the exact freeze SHA and contract

### Optional Lane B - `ORCH_PLAN.md` adoption lane

Lane B exists only after `gate-m39-30-implementation-contract-freeze` passes. It is optional because M39 has only one honest parallel seam.

Launch condition:

- launch only if `ORCH_PLAN.md` adoption work is still open on the contract-freeze commit
- otherwise write `lane-b-skip.json` and continue sequentially

Owned file:

- `ORCH_PLAN.md`

Forbidden files:

- all Rust source
- all `.runs/m39_verification_consumer_probe/**`
- all `.semantic-family-artifacts/**`
- `PLAN.md`

Lane B required changes:

1. ensure the standing proof walls in `ORCH_PLAN.md` use the verifier command instead of repeated shell ladders
2. preserve branch/worktree/run-state/gate semantics from the frozen contract
3. do not expand scope beyond verifier adoption

Lane B required proof walls:

1. `Contract Freeze Verifier Wall`
2. `Integration Verifier Wall`
3. `Final Verification Wall`

Lane B return contract:

- one branch off the exact `implementation-contract-freeze.json` SHA
- one narrow summary of what changed in `ORCH_PLAN.md`
- no run-state edits

Stale-lane invalidation triggers:

- `implementation-contract-freeze.json` SHA changes
- canonical verifier command string changes
- required JSON top-level keys change
- required `checks` keys change
- required failure reasons change
- parent lands conflicting `ORCH_PLAN.md` edits on `ws/m39-int`

#### Gate `gate-m39-40-optional-lane-launch`

Objective:

- make the only honest parallel seam concrete and bounded

Tasks:

1. decide `launch` or `skip` for Lane B
2. if `launch`, write `lane-b-launch.md` and create the lane worktree from the exact freeze SHA
3. if `skip`, write `lane-b-skip.json` with the explicit reason

Pass criteria:

- exactly one of `lane-b-launch.md` or `lane-b-skip.json` exists
- no second optional lane is created

#### Gate `gate-m39-50-merge-and-integration`

Objective:

- merge optional adoption work, if any, back into `ws/m39-int`

Tasks:

1. if Lane B was launched, merge `ws/m39-lane-b-orch-adoption` into `ws/m39-int`
2. resolve only straightforward `ORCH_PLAN.md` merge mechanics
3. if conflict resolution would alter the frozen verifier contract, stop, invalidate Lane B, and recreate it from a new freeze
4. record the result in `merge-log.md`

Pass criteria:

- `ws/m39-int` contains the accepted implementation and any accepted `ORCH_PLAN.md` adoption edits
- `merge-log.md` records whether Lane B was used or skipped

#### Gate `gate-m39-60-parity-proof`

Objective:

- prove the new verifier returns the same green result as the legacy shell ladder on the same artifact set

Commands:

```bash
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.evidence_summary.missing_evidence == []' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.evidence_summary.stale_evidence == []' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

cargo xtask family verify-decision-contract --format json | tee \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
jq -e '.overall_verdict == "pass"' \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
```

Pass criteria:

- the legacy shell path still passes
- the verifier passes on the same artifact hashes
- `parity-proof.md` records the shared input hashes and both result surfaces

#### Gate `gate-m39-70-orch-adoption-proof`

Objective:

- prove that repo-root orchestration adopted the verifier as a standing surface

Named standing proof walls:

1. `Contract Freeze Verifier Wall`
2. `Integration Verifier Wall`
3. `Final Verification Wall`

Required command in each standing wall:

```bash
cargo xtask family verify-decision-contract --format json
```

Proof command:

```bash
rg -n "Contract Freeze Verifier Wall|Integration Verifier Wall|Final Verification Wall|verify-decision-contract --format json" ORCH_PLAN.md
```

Pass criteria:

- all three named standing proof walls exist in `ORCH_PLAN.md`
- each standing wall uses the verifier command
- `orch-adoption-proof.md` explicitly records whether Lane B was used or skipped

#### Gate `gate-m39-80-pre-publish-acceptance`

Objective:

- prove the accepted integration tree is ready to publish back to the live checkout

Tasks:

1. rerun the final verification wall
2. write `acceptance.md`
3. confirm `queue.json` marks gates `00` through `70` as `complete`
4. move `task-m39-90-publish-to-live` to `active`

Failure rule:

- publish is not allowed if implementation, parity, or adoption proof is still blocked

Pass criteria:

- `acceptance.md` exists and maps gates `00` through `70` to pass or blocker outcomes
- the accepted tree on `ws/m39-int` is the exact tree chosen for publish
- `queue.json` records `gate-m39-80-pre-publish-acceptance` as `complete`

#### Task `task-m39-90-publish-to-live`

Objective:

- land the accepted `ws/m39-int` result back onto the live `feat/corpus-expansion` checkout as the end-to-end publish step for this run

Publish rules:

1. publish is allowed only after `gate-m39-80-pre-publish-acceptance` is complete
2. the parent is the only publisher
3. publish lands the accepted integration result as one unit; do not cherry-pick partial M39 surfaces
4. authored source changes and accepted `.runs/m39_verification_consumer_probe/**` proof artifacts from `ws/m39-int` must land together
5. if live `feat/corpus-expansion` has moved since `publish-head.txt`, the parent must first merge that live movement into `ws/m39-int`, rerun gates `50` through `80`, and only then publish
6. publish may use fast-forward only when the live branch still matches `publish-head.txt`; otherwise publish uses a normal merge from the accepted `ws/m39-int` commit
7. publish does not reopen scope; it only lands the already-accepted integration tree

Tasks:

1. read `publish-head.txt` and compare it to live `feat/corpus-expansion`
2. publish the accepted `ws/m39-int` commit set onto the live checkout by fast-forward or merge, as allowed above
3. write `publish-result.json`
4. mark `gate-m39-95-post-publish-verification` as `active` in `queue.json`

#### Gate `gate-m39-95-post-publish-verification`

Objective:

- prove the published live checkout still satisfies the verifier and adoption contract after landing

Commands:

```bash
git branch --show-current
git rev-parse HEAD
cargo test -p xtask
cargo xtask family verify-decision-contract --format json | tee \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
jq -e '.overall_verdict == "pass"' \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
rg -n "Contract Freeze Verifier Wall|Integration Verifier Wall|Final Verification Wall|verify-decision-contract --format json" ORCH_PLAN.md
```

Pass criteria:

- live checkout branch is `feat/corpus-expansion`
- live checkout HEAD equals the published accepted result
- verifier remains green after publish
- `ORCH_PLAN.md` on the live checkout still contains the named standing verifier walls
- `publish-verification.md` and `publish-result.json` are complete

#### Gate `gate-m39-100-final-closeout`

Objective:

- finish with one exact honest verdict and stop

Tasks:

1. confirm `queue.json` marks `task-m39-90-publish-to-live` and `gate-m39-95-post-publish-verification` as `complete`
2. write `closeout.md`
3. mark `gate-m39-100-final-closeout` as `complete`
4. stop the run after the closeout verdict is written

## Context-Control Rules

- The parent is the only integrator, the only run-state author, the only lane launcher, the only stale-lane invalidator, and the only closeout author.
- Lane B is an optimization, not a second source of truth.
- If Lane B is launched, it receives only the frozen contract packet from `lane-b-launch.md`. It does not reinterpret `PLAN.md`.
- If the live baseline artifacts drift after baseline capture, the parent either re-baselines from the new truth or stops. It does not silently continue on mixed evidence.
- If `PLAN.md` changes after authority freeze, all worktrees are stale.
- If `ORCH_PLAN.md` changes on `ws/m39-int` after Lane B launches, Lane B is stale unless the parent explicitly records the conflict as non-overlapping in `session.log`.
- If `cargo xtask family verify-decision-contract --format json` changes shape after contract freeze, Lane B is stale even if its branch still merges cleanly.
- No run-state file is considered authoritative unless it exists under the current `ACTIVE_RUN_ROOT`.

## Tests And Acceptance

### Required implementation-time commands

```bash
cargo test -p xtask
cargo xtask family verify-decision-contract --help
cargo xtask family verify-decision-contract --format json
```

### Legacy baseline capture

The baseline gate must run the pre-M39 shell ladder once and record it in `legacy-shell-baseline.md`. That is the frozen comparison floor, not the standing post-M39 operator surface.

### Contract Freeze Verifier Wall

Run on `ws/m39-int` immediately before writing `implementation-contract-freeze.json`:

```bash
cargo xtask family verify-decision-contract --format json
```

### Integration Verifier Wall

Run after merging Lane B or recording `lane-b-skip.json`:

```bash
cargo test -p xtask
cargo xtask family verify-decision-contract --format json
```

### Final Verification Wall

Run immediately before writing `closeout.md`:

```bash
cargo xtask family verify-decision-contract --format json
```

### Post-Publish Verification Wall

Run on the live `feat/corpus-expansion` checkout immediately after `task-m39-90-publish-to-live`:

```bash
git branch --show-current
git rev-parse HEAD
cargo xtask family verify-decision-contract --format json | tee \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
jq -e '.overall_verdict == "pass"' \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
```

### Acceptance checklist

M39 is not complete until all of these are true:

1. `baseline.json` and `legacy-shell-baseline.md` prove the frozen helper-surface floor on the live baseline SHA.
2. `implementation-contract-freeze.json` proves the exact verifier command contract and freeze SHA.
3. `cargo test -p xtask` is green on the accepted integration tree.
4. `verify-decision-contract.stdout.json` shows `overall_verdict == "pass"` on the accepted integration tree.
5. `parity-proof.md` proves legacy shell parity and verifier parity on the same artifact set.
6. `orch-adoption-proof.md` proves the named standing proof walls use the verifier command.
7. `acceptance.md` maps every gate in this file to a pass or blocker outcome.
8. `publish-result.json` and `publish-verification.md` prove the accepted result landed back on live `feat/corpus-expansion` and stayed green.
9. `closeout.md` ends with exactly one allowed verdict.

## Closeout Rules

Allowed final verdicts:

1. `candidate third consumer observed, but the kernel still stays local`
2. `third honest consumer proven`
3. `keep the kernel local`

Verdict selection rules:

- `third honest consumer proven` is allowed only if:
  - the verifier command is implemented and green
  - parity proof passes
  - `ORCH_PLAN.md` standing proof walls adopt the verifier
- `candidate third consumer observed, but the kernel still stays local` is allowed only if:
  - the verifier command is implemented and green
  - parity proof passes
  - adoption is incomplete, intentionally deferred, or not yet accepted into standing orchestration
- `keep the kernel local` is required if:
  - verifier implementation fails
  - parity proof fails
  - adoption proof fails badly enough that the consumer claim is not honest
  - the command requires scope-violating abstraction, rescanning, schema widening, or path overrides to survive

Final file rule:

- The last non-empty line of `closeout.md` must be exactly one of the three allowed verdict strings above.
- No alternate verdict wording is allowed.
