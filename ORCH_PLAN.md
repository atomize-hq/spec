# M36 Orchestration Plan

Status: **authoritative execution contract for the M36 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Last rewritten: **`2026-05-05`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Recommendation artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Decision artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`**  
Execution note: **M36 is one delegated code lane plus one later docs lane. The live checkout on `feat/corpus-expansion` is the baseline and publish target. All integration happens on `ws/m36-int`.**

## Summary

- This run is for **M36 helper-surface follow-on contract consolidation** only.
- `PLAN.md` remains milestone authority. This file is the parent-owned operator contract for landing M36 safely.
- The parent is the sole baseline capturer, sole freeze authority, sole worktree creator, sole lane launcher, sole integrator, sole stale-lane invalidator, sole final verifier, sole publish authority, and sole closeout author.
- The run has **one real code lane** and **one docs lane**:
  - code lane: sequential implementation only, no fake parallelism
  - docs lane: allowed only after API and vocabulary freeze
- Recommended worker profile for every delegated lane is:
  - `GPT-5.4`
  - `reasoning_effort=high`
- Worker concurrency cap is:
  - `0` before `integration-freeze.json`
  - `1` after `integration-freeze.json` and before `api-vocabulary-freeze.json`
  - `2` after `api-vocabulary-freeze.json`
- The parent remains the only integrator. Workers never merge, never publish, never update orchestration state, and never declare the run green.
- The local critical path is fixed:
  1. capture baseline on live `feat/corpus-expansion`
  2. create `ws/m36-int` from the frozen live SHA
  3. launch the single code lane from the frozen integration SHA
  4. integrate code sequentially on `ws/m36-int`
  5. write `api-vocabulary-freeze.json`
  6. launch docs lane from the frozen post-code integration SHA
  7. merge docs into `ws/m36-int`
  8. run full verification on `ws/m36-int`
  9. fast-forward or merge back to live `feat/corpus-expansion` only if green
- `.runs/**` and `.semantic-family-artifacts/**` are run artifacts and derived outputs:
  - not authored source
  - not worker-owned
  - not acceptable substitutes for runtime gates
- The live wedge must remain unchanged end to end:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "not_recommended"`
  - `open_blockers = ["helper_surface_not_promotable"]`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`

## Hard Guards

- `PLAN.md` wins over this file, worker summaries, stale worktrees, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion` is the baseline and publish target, not the merge surface.
- The parent does not integrate on the live checkout. All merges, validations, and final proving happen on:
  - branch: `ws/m36-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`
- The parent records live branch name, head SHA, dirty state, and overlapping local edits before creating any M36 worktree.
- If overlapping local edits exist on M36-owned surfaces before `authority-freeze.json`, the parent either re-anchors around them or blocks the run. It does not overwrite them silently.
- After `authority-freeze.json` is written, both authority files are frozen:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- If either authority file changes after freeze, the run stops and restarts from a fresh baseline.
- M36 stays bounded to `xtask/src/family/` plus these documented surfaces only:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` closeout notes only
- M36 must not introduce:
  - a new crate
  - a schema rewrite
  - a generic decision engine
  - a registry of follow-on contracts
  - a policy DSL
  - a new artifact family
- The follow-on contract lives only in:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `recommend.rs` and `xtask/src/lib.rs` are conflict magnets:
  - `recommend.rs` is single-writer by the code lane only
  - `xtask/src/lib.rs` is single-writer by the code lane only
  - no second code lane may be created to “help”
- The frozen wedge vocabulary stays unchanged:
  - `helper_surface_not_promotable`
  - `durable_non_promotable_helper_surface`
  - `pivot_to_architecture_shared_core_follow_on`
  - `author_architecture_follow_on_plan`
- Stable proof identity is additive and normalized. It is not a public artifact redesign.
- No one hand-edits JSON under `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/`.
- Stop immediately if any lane requires:
  - widening into `spec-core`
  - a second semantic owner for the helper follow-on contract
  - recomputing coverage inside `family corpus-decision`
  - using raw latest-artifact SHA as the closeout gate

## Workstream Plan

## Closed Implementation Surface

Parent-owned or lane-owned work is limited to:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` completion notes only after final green

Allowed mechanical spillover is compile- or module-wire-forced only:

- imports
- module exports
- test names
- test fixtures inside `xtask/src/lib.rs`

## Branch And Worktree Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Canonical branches and worktrees:

| Role | Branch | Worktree |
|---|---|---|
| Live baseline and publish target | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` |
| Parent integration | `ws/m36-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int` |
| Lane A, one real code lane | `ws/m36-lane-a-code` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-a-code` |
| Lane B, docs lane | `ws/m36-lane-b-docs` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-b-docs` |

Creation rules:

1. The parent captures baseline on the live checkout before creating any M36 worktree.
2. `ws/m36-int` is created from the exact SHA recorded in:
   - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/integration-base.txt`
3. `ws/m36-lane-a-code` is created from the exact SHA recorded in:
   - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/integration-freeze.json`
4. `ws/m36-lane-b-docs` is created only after code integration is green, from the exact SHA recorded in:
   - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/api-vocabulary-freeze.json`
5. No worker lane forks from another worker lane.
6. If any named worktree already exists with stale state, the parent recreates it and records the action in `session.log`.
7. If the live branch moves after baseline capture but before publish, the parent either re-baselines or explicitly merges the new live head into `ws/m36-int` and reruns the full verification wall. It does not publish blind.

Canonical worktree creation commands:

```bash
git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int \
  -b ws/m36-int <BASELINE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-a-code \
  -b ws/m36-lane-a-code <INTEGRATION_FREEZE_SHA>

git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-b-docs \
  -b ws/m36-lane-b-docs <API_VOCAB_FREEZE_SHA>
```

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation`
- `ARTIFACT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`

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
- `integration-freeze.json`
- `api-vocabulary-freeze.json`
- `lane-a-launch.md`
- `lane-b-launch.md`
- `merge-log.md`
- `proof-log.json`
- `green-path-record.json`
- `acceptance.md`
- `blocked.json`
- `blocked-failing-command.txt`
- `blocked-failing-exit-code.txt`
- `closeout.md`

Required freeze-record contents:

- `baseline.json`
  - live branch
  - live HEAD SHA
  - dirty-state summary
  - local overlapping edit summary
  - current recommendation artifact path
  - current decision artifact path
- `publish-head.txt`
  - exact live HEAD SHA captured during baseline
  - reused only to detect whether the live publish target moved before Task M36-80
- `integration-freeze.json`
  - exact SHA used to create `ws/m36-int`
  - exact SHA used to create `ws/m36-lane-a-code`
  - explicit statement that one real code lane is now authorized
  - explicit statement that docs lane is still blocked
- `api-vocabulary-freeze.json`
  - exact post-code integration SHA
  - exact helper contract type and function names
  - frozen wedge vocabulary strings
  - explicit docs-lane unblock
  - explicit statement that raw SHA is debug only and normalized fingerprints are authoritative
- `run-state.json`
  - current phase
  - active branch per lane
  - accepted sentinels
  - blocked flag
  - current freeze SHA pointers
- `queue.json`
  - ordered task queue
  - dependency ids
  - lane assignment
  - state: pending, launched, accepted, blocked
- `proof-log.json`
  - command name
  - artifact path
  - raw SHA if captured
  - normalized fingerprint if captured
  - semantic interpretation
- `acceptance.md`
  - final checklist by task
  - final command results
  - publish decision

Per-task sentinel directories:

- `task-m36-00-baseline-capture`
- `task-m36-05-authority-freeze`
- `task-m36-10-create-integration-worktree`
- `task-m36-20-launch-code-lane`
- `task-m36-30-parent-code-integration`
- `task-m36-40-api-vocabulary-freeze`
- `task-m36-50-launch-docs-lane`
- `task-m36-60-parent-docs-integration`
- `task-m36-70-final-verification-wall`
- `task-m36-80-publish-back-to-live`
- `task-m36-90-closeout`

Each task directory contains parent-written markers only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

Workers never write orchestration state, queue files, session logs, sentinels, or parent acceptance records.

## Artifact Handling Rules

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/*.latest.json` are generated artifacts only.
- Workers may read generated artifacts to understand behavior, but they do not edit them.
- Parent reruns generation and validation commands on `ws/m36-int` before accepting any lane.
- Raw SHA of latest artifact bytes may be logged in:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/proof-log.json`
  but raw SHA is never a ship gate.
- Normalized semantic fingerprints are the durable proof identity for closeout.
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/**` are orchestration artifacts only. They never replace green tests, green runtime gates, or parent validation.
- `PLAN.md` is not updated until all final gates pass on `ws/m36-int`.

## Parent Vs Worker Ownership

### Parent-only always

- baseline capture on `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- authority freeze
- creation and recreation of all worktrees
- all files under `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m36_helper_surface_follow_on_contract_consolidation/`
- all merge operations into `ws/m36-int`
- stale-lane invalidation
- final verification wall
- publish-back to `feat/corpus-expansion`
- closeout
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`

### Lane A: one real code lane

Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Owned paths:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/coverage.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs`

Mission:

- extend `helper_surface.rs` to own the derived helper follow-on contract
- rewire `recommend.rs` to consume that owner
- rewire `promotion_artifacts.rs` to consume that owner
- add stable proof-fingerprint helpers and regressions
- preserve the frozen live wedge behavior

Forbidden:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/**`

### Lane B: docs lane

Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Owned paths:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md`

Mission:

- explain one helper follow-on contract owner
- explain that M36 preserves the frozen M35 wedge
- explain that raw latest-artifact SHA is not semantic identity
- explain that normalized semantic fingerprints are the proof surface

Forbidden:

- any `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/**`

## Task Graph

```text
task-m36-00-baseline-capture
  -> task-m36-05-authority-freeze
      -> task-m36-10-create-integration-worktree
          -> task-m36-20-launch-code-lane
              -> task-m36-30-parent-code-integration
                  -> task-m36-40-api-vocabulary-freeze
                      -> task-m36-50-launch-docs-lane
                          -> task-m36-60-parent-docs-integration
                              -> task-m36-70-final-verification-wall
                                  -> task-m36-80-publish-back-to-live
                                      -> task-m36-90-closeout
```

## Task M36-00 - Baseline Capture On Live Branch

**Owner:** Parent  
**Branch:** `feat/corpus-expansion`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Record current branch, HEAD SHA, dirty state, and overlapping local edits in `baseline.json`.
2. Write the same live HEAD SHA to `publish-head.txt` before creating any M36 worktree.
3. Run the live wedge commands on the live checkout.
4. Record the opening truth in `session.log` and `tasks.json`.
5. Write:
   - `task-m36-00-baseline-capture/started.json`
   - `task-m36-00-baseline-capture/status.json`

Runtime gates:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
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

Stop conditions:

- any command fails
- any `jq` assertion fails
- overlapping local edits touch the closed implementation surface and are not explicitly accepted
- the live wedge drifts from the frozen M35 tuple

Acceptance:

- `baseline.json` exists and includes branch, SHA, dirty summary, and overlap summary
- `integration-base.txt` contains the exact live SHA to fork from
- `publish-head.txt` contains the same exact live SHA recorded in `baseline.json`
- `task-m36-00-baseline-capture/done.json` exists
- `task-m36-00-baseline-capture/blocked.json` does not exist
- `queue.json` marks the next task as ready

## Task M36-05 - Authority Freeze

**Owner:** Parent  
**Branch:** `feat/corpus-expansion`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Snapshot `PLAN.md` and `ORCH_PLAN.md` into `authority-snapshot/`.
2. Write `authority-freeze.json`.
3. Mark the implementation surface and branch strategy immutable for the run.
4. Update `run-state.json` and `queue.json`.

Stop conditions:

- either authority file changes after snapshot
- the requested scope no longer matches `PLAN.md`
- new overlapping edits appear on authority files during freeze

Acceptance:

- `authority-freeze.json` exists
- `authority-snapshot/PLAN.md` and `authority-snapshot/ORCH_PLAN.md` exist
- `task-m36-05-authority-freeze/done.json` exists
- `run-state.json` marks docs lane blocked and code lane not yet launched

## Task M36-10 - Create Integration Worktree

**Owner:** Parent  
**Branch:** `ws/m36-int`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`

Actions:

1. Create `ws/m36-int` from the SHA in `integration-base.txt`.
2. Verify branch and worktree point to the recorded baseline SHA.
3. Write `integration-freeze.json`.
4. Record creation details in `session.log`.

Stop conditions:

- worktree cannot be created from the exact baseline SHA
- pre-existing stale `int` worktree cannot be safely recreated
- `ws/m36-int` is not a clean fork from the live baseline SHA

Acceptance:

- `git rev-parse HEAD` in `ws/m36-int` equals `integration-base.txt`
- `integration-freeze.json` exists and authorizes exactly one code lane
- `task-m36-10-create-integration-worktree/done.json` exists
- `run-state.json` lists `ws/m36-int` as the integration spine

## Task M36-20 - Launch The One Real Code Lane

**Owner:** Parent launches, Lane A executes  
**Branch:** `ws/m36-lane-a-code`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-a-code`

Actions:

1. Create `ws/m36-lane-a-code` from the SHA recorded in `integration-freeze.json`.
2. Write `lane-a-launch.md` with:
   - owned paths
   - forbidden paths
   - exact runtime gates
   - return contract
   - stale-lane invalidation triggers
3. Mark the task launched in `queue.json`.

Lane A required checkpoints:

```bash
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask proof_fingerprint -- --color never
```

The new proof-identity regressions added by M36 must include `proof_fingerprint`
in the test name so this checkpoint filter is authoritative.

Stop conditions:

- any attempt to split code work across another branch or worktree
- lane touches forbidden docs or orchestration state
- lane requires new crate, schema rewrite, or generic engine

Acceptance:

- `lane-a-launch.md` exists and is complete
- `ws/m36-lane-a-code` is forked from the frozen integration SHA, not from live
- `task-m36-20-launch-code-lane/done.json` exists
- `queue.json` shows one active worker lane only

## Task M36-30 - Parent Code Integration On `ws/m36-int`

**Owner:** Parent  
**Branch:** `ws/m36-int`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`

Actions:

1. Accept or reject Lane A return based on owned paths and checkpoint evidence.
2. Merge `ws/m36-lane-a-code` into `ws/m36-int`.
3. Rerun code-lane checkpoints on `ws/m36-int`.
4. Rerun the live wedge runtime gates on `ws/m36-int`.
5. Record results in `merge-log.md`, `proof-log.json`, and `session.log`.

Runtime gates after merge:

```bash
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask proof_fingerprint -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
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

Stop conditions:

- `helper_surface.rs`, `recommend.rs`, and `promotion_artifacts.rs` still encode the helper follow-on contract in parallel
- validator and decision derivation disagree on whether the helper wedge is active
- the emitted public tuple changes
- proof-fingerprint tests fail or are missing

Acceptance:

- merge is recorded in `merge-log.md`
- parent reruns all code-lane checkpoints green on `ws/m36-int`
- live wedge runtime assertions still pass on `ws/m36-int`
- `task-m36-30-parent-code-integration/done.json` exists
- docs lane remains blocked until the next freeze task is complete

## Task M36-40 - API And Vocabulary Freeze

**Owner:** Parent  
**Branch:** `ws/m36-int`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`

Actions:

1. Capture the post-code integration SHA.
2. Write `api-vocabulary-freeze.json`.
3. Record the frozen helper contract surface and wedge vocabulary.
4. Unblock the docs lane in `run-state.json` and `queue.json`.

Required frozen facts:

- one semantic owner lives in `xtask/src/family/helper_surface.rs`
- action mapping remains in `xtask/src/family/recommend.rs`
- validation recomputes contract from validated basis in `xtask/src/family/promotion_artifacts.rs`
- raw SHA is debug only
- normalized fingerprints are authoritative proof identity
- vocabulary strings remain unchanged

Stop conditions:

- names or semantics are still moving after code integration
- docs would need to guess or invent wording not frozen in code
- parent cannot name the exact helper contract surface unambiguously

Acceptance:

- `api-vocabulary-freeze.json` exists
- `task-m36-40-api-vocabulary-freeze/done.json` exists
- `queue.json` marks docs lane ready
- `run-state.json` shows concurrency cap raised from `1` to `2`

## Task M36-50 - Launch Docs Lane

**Owner:** Parent launches, Lane B executes  
**Branch:** `ws/m36-lane-b-docs`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/lane-b-docs`

Actions:

1. Create `ws/m36-lane-b-docs` from the SHA recorded in `api-vocabulary-freeze.json`.
2. Write `lane-b-launch.md` with exact frozen vocabulary, owned paths, forbidden paths, and docs acceptance criteria.
3. Keep code surfaces locked.

Docs gate:

```bash
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan|corpus run `1` remains unspent' \
  semantic-families/README.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md
```

Stop conditions:

- docs lane starts before `api-vocabulary-freeze.json`
- docs lane edits any `xtask/src/**` surface
- docs wording reopens public vocabulary or implies raw SHA is authoritative

Acceptance:

- `lane-b-launch.md` exists
- `ws/m36-lane-b-docs` is forked from the frozen post-code integration SHA
- `task-m36-50-launch-docs-lane/done.json` exists
- `queue.json` shows no additional lanes beyond code and docs

## Task M36-60 - Parent Docs Integration

**Owner:** Parent  
**Branch:** `ws/m36-int`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`

Actions:

1. Accept or reject Lane B return based on owned paths and docs gate evidence.
2. Merge `ws/m36-lane-b-docs` into `ws/m36-int`.
3. Rerun the docs grep gate.
4. Confirm docs align with code truth and proof semantics.

Docs gate after merge:

```bash
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan|corpus run `1` remains unspent' \
  semantic-families/README.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md
```

Stop conditions:

- docs imply corpus run `1` was spent
- docs imply raw latest-artifact SHA is the proof gate
- docs drift from the frozen strings or parent code truth

Acceptance:

- merge is recorded in `merge-log.md`
- docs grep gate passes on `ws/m36-int`
- `task-m36-60-parent-docs-integration/done.json` exists
- `ws/m36-int` is now ready for the final verification wall

## Task M36-70 - Final Verification Wall

**Owner:** Parent  
**Branch:** `ws/m36-int`  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation/int`

Actions:

1. Run the full final verification floor from merged integration state.
2. Confirm proof identity is validated by xtask tests, not raw SHA comparisons.
3. Record results in `green-path-record.json`, `proof-log.json`, and `acceptance.md`.

Final verification floor:

```bash
cargo fmt --all --check
cargo clippy -p xtask --all-targets --all-features -- -D warnings
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask recommend -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask proof_fingerprint -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
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
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan|corpus run `1` remains unspent' \
  semantic-families/README.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md
```

Stop conditions:

- any final verification command fails
- any live wedge `jq` assertion fails
- docs grep gate fails
- proof identity is argued from raw SHA instead of normalized fingerprint tests
- the final merged tree requires unexplained manual exceptions

Acceptance:

- every command above exits `0`
- `green-path-record.json` exists and records success
- `task-m36-70-final-verification-wall/done.json` exists
- `blocked.json` does not exist
- parent is authorized to publish back to live

## Task M36-80 - Publish Back To Live Branch

**Owner:** Parent  
**From:** `ws/m36-int`  
**To:** `feat/corpus-expansion`

Actions:

1. Compare current live `feat/corpus-expansion` HEAD with `publish-head.txt` or `baseline.json`.
2. If `PLAN.md` completion notes are required, land them on `ws/m36-int`, rerun Task M36-70 on that updated tree, and publish only the exact reverified SHA.
3. If unchanged, fast-forward the live branch to the exact verified `ws/m36-int` SHA.
4. If live moved:
   - preferred: merge the moved live head into `ws/m36-int`, rerun Task M36-70, then update live
   - fallback: block the run and require a fresh baseline
5. Record the publish action in `session.log` and `acceptance.md`.

Stop conditions:

- live branch moved and parent cannot safely reconcile
- publish would bypass the final verification wall
- publish would add a post-verification live-only commit, including a `PLAN.md` closeout edit that was not reverified on `ws/m36-int`
- parent cannot prove the published tree equals the verified integration tree

Acceptance:

- live `feat/corpus-expansion` points at the verified M36 result
- `task-m36-80-publish-back-to-live/done.json` exists
- `acceptance.md` records whether publish was fast-forward or merge-backed
- no unverified commit is inserted between `ws/m36-int` and the live published result

## Task M36-90 - Closeout

**Owner:** Parent  
**Branch:** `feat/corpus-expansion` after publish  
**Path:** `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Actions:

1. Confirm any required `PLAN.md` completion notes are already present in the published verified tree. Do not create a new live-only source commit in Task M36-90.
2. Write `closeout.md`.
3. Mark final accepted sentinels in `run-state.json`.
4. Ensure all blocked-path files are absent or explicitly empty for green closeout.

Stop conditions:

- `PLAN.md` would require a new post-publish source commit to describe closeout
- required sentinel is missing
- `blocked.json` exists

Acceptance:

- `task-m36-90-closeout/done.json` exists
- `closeout.md` exists
- `acceptance.md` shows every task accepted
- `run-state.json` lists the full accepted sentinel set
- the published live SHA still equals the exact verified integration SHA from Task M36-80
- the run is auditable from `queue.json`, `session.log`, `merge-log.md`, `proof-log.json`, and sentinel files alone

## Context-Control Rules

- Workers only edit their owned paths. No convenience edits outside lane ownership.
- Lane A is the only code lane. It absorbs helper contract extraction, recommend rewiring, validator rewiring, and proof hardening sequentially on one branch.
- Lane B starts only after `api-vocabulary-freeze.json` exists.
- Parent reruns validation on `ws/m36-int` after every merge. Worker-green never bypasses parent-green.
- `queue.json` is the launch order authority. If a lane is not queued and launched there, it does not exist.
- `session.log` is append-only and parent-written. Every worktree creation, merge, relaunch, block, or publish action is logged there with timestamp and SHA.
- A stale lane is invalid if:
  - its launch SHA no longer matches the current parent freeze SHA
  - authority files changed after launch
  - parent detected semantic drift and wrote `blocked.json`
- Invalid lanes are discarded and recreated. The parent does not hand-forward stale worker branches.
- Workers never write:
  - `run-state.json`
  - `tasks.json`
  - `queue.json`
  - `session.log`
  - `merge-log.md`
  - `proof-log.json`
  - `blocked.json`
  - `acceptance.md`
  - any sentinel file
- If code and docs disagree, code on `ws/m36-int` wins and docs are corrected or blocked.
- If runtime truth and normalized proof identity disagree, the run stops until the mismatch is explained with tests.

## Tests And Acceptance

## Required Code Tests

Lane A must land or preserve these test surfaces:

- `cargo test -p xtask helper_surface -- --color never`
- `cargo test -p xtask recommend -- --color never`
- `cargo test -p xtask corpus_decision -- --color never`
- `cargo test -p xtask artifact_schema_ -- --color never`
- `cargo test -p xtask proof_fingerprint -- --color never`

Lane A should name the new proof-identity regressions with `proof_fingerprint`
in the test name so these gates cannot silently skip them.

Required behavioral coverage:

- exact live helper wedge produces `Some(ArchitectureSharedCoreFollowOn)` through the shared helper contract owner
- missing evidence and stale evidence keep the helper contract inactive
- non-helper unsupported pressure does not over-generalize into the architecture follow-on wedge
- `recommend.rs` still emits the frozen architecture follow-on tuple
- `promotion_artifacts.rs` rejects tuple/contract contradictions deterministically
- normalized fingerprints stay stable across harmless `generated_at` and inventory-path churn
- normalized fingerprints still change when semantic meaning changes

## Required Runtime Gates

The parent must prove all of these on `ws/m36-int`:

- `coverage`
- `recommend`
- `corpus-decision`
- `validate-artifact` for coverage
- `validate-artifact` for recommendation
- `validate-artifact` for corpus decision
- all six live wedge `jq` assertions
- docs grep gate

## Task Acceptance Standard

A task is accepted only when all of the following are true:

1. its `started.json` exists,
2. its `status.json` records the expected branch, worktree, and owner,
3. its `done.json` exists,
4. its `blocked.json` does not exist,
5. parent validation for that task passed on the declared worktree,
6. any required freeze file or log entry for that task exists,
7. `queue.json` advances to the next legal state.

Missing sentinel means the task is not accepted.

## Green Run Definition

M36 is green only when all of the following are true:

1. the repo has exactly one semantic owner for the durable helper-surface follow-on contract,
2. `recommend.rs` consumes that owner for the architecture follow-on decision,
3. `promotion_artifacts.rs` consumes that owner for tuple validation,
4. the live wedge still emits the frozen M35 tuple,
5. stable proof identity is demonstrated by normalized fingerprints rather than raw latest-artifact SHA,
6. docs teach the same truth the code enforces,
7. the verified `ws/m36-int` result has been published back to `feat/corpus-expansion`.

## Assumptions

- The established repo naming pattern for milestone worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/` is milestone-root plus lane subdirectories, so M36 uses `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m36-helper-surface-follow-on-contract-consolidation`.
- The live checkout at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` remains the checked-out `feat/corpus-expansion` baseline referenced by `PLAN.md`.
- The repo’s existing orchestration style treats parent-written `.runs/**` state, `queue.json`, `session.log`, and per-task sentinels as the runnable source of coordination truth.
- Publish-back to the live branch is expected after final green; fast-forward is preferred, but an explicit merge-back is acceptable only if the parent reruns the full verification wall on the reconciled integration state.
