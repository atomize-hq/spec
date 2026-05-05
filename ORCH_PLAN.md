# M34 Orchestration Plan

Status: **authoritative execution contract for the M34 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Last rewritten: **`2026-05-05`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Analysis basis path: **`.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Decision artifact path: **`.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`**  
Execution note: **M34 is a bounded read-side extension. The parent stays on the code critical path and launches no worker until the basis wedge and the new artifact contract are frozen.**

## Summary

- This run is for **M34 stop-spend-pivot decision contract** only.
- `PLAN.md` remains milestone authority. `ORCH_PLAN.md` is the parent-owned execution contract for the session that lands M34 safely.
- The parent agent is the sole baseline capturer, sole freeze authority, sole schema/command integrator, sole stale-lane invalidator, sole merge authority, sole blocker emitter, sole final verifier, sole push authority, and sole closeout author.
- The code lane stays **strictly sequential** on `ws/m34-int` because `xtask/src/lib.rs`, `xtask/src/family/recommend.rs`, `xtask/src/family/promotion_artifacts.rs`, and `xtask/src/family/paths.rs` define one coupled artifact pipeline.
- Parallelism is allowed only where it is truly disjoint:
  - optional late docs sync lane
  - only after `artifact-freeze.json` exists
  - only if the parent has already produced and validated the first real `corpus-program-decision.latest.json`
- Recommended worker profile for the optional docs lane is:
  - `GPT-5.4`
  - `reasoning_effort=high`
- Worker concurrency cap is:
  - `0` before `artifact-freeze.json`
  - `1` after `artifact-freeze.json`, and only for the optional docs lane
- The live wedge is mandatory and must stay explicit end to end:
  - basis artifact remains `recommendation.latest.json`
  - current basis must still read `no_strong_candidate` + `not_recommended`
  - current blocker must still be `helper_surface_not_promotable`
  - current missing/stale evidence must both be empty
  - `cargo xtask family corpus-decision --format json` must therefore emit `pivot_to_architecture_shared_core_follow_on`
- M34 does not execute corpus work. It decides whether corpus run `1` should remain unspent, be explicitly spent, or be superseded by a pivot class.

## Hard Guards

- `PLAN.md` wins over this file, worker summaries, stale worktree copies, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen on `ws/m34-int`.
- The live checkout on `feat/corpus-expansion` is the publish target and baseline reference, not the merge surface.
- The parent records live branch name, head SHA, dirty state, and overlapping local edits before creating any M34 worktree.
- If local or incoming edits overlap the M34-owned surface before `authority-freeze.json`, the parent must either re-anchor around them or block the run. It must not silently overwrite them.
- After `authority-freeze.json` is written, both `PLAN.md` and `ORCH_PLAN.md` are frozen. If either authority file must change after that point, stop the run, emit blocker state, and restart from a new authority baseline.
- The M34 input contract is fixed:
  - read `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - validate that basis before deriving the decision
  - write `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- `cargo xtask family corpus-decision --format json` is the only new command surface in scope.
- `family corpus-decision` must not rescan the corpus and must not recompute recommendation from raw sources.
- No one hand-edits JSON under `.semantic-family-artifacts/`. Derived artifacts are created only by repo commands and validated as produced output.
- M34 must preserve these plan guards exactly:
  - no corpus execution
  - no recommendation-policy redesign
  - no family promotion execution changes
  - no shared-core implementation
  - no markdown program-tracker parsing at runtime
  - no new runtime crate
  - no new artifact directory outside `analysis/`
- The closed implementation surface for M34 is:
  - `xtask/src/lib.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
  - `semantic-families/README.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
  - `docs/semantic_family_capability_corpus_guide_v0.1.md`
- Allowed mechanical spillover is compile-, module-wire-, or test-harness-forced only:
  - `xtask/src/family/mod.rs`
  - existing xtask test modules inside already-owned xtask files
- Stop immediately if any lane requires edits to:
  - `semantic-families/corpus/rust-function.toml`
  - any `spec-core/src/**` file
  - any promoted family packet directory under `semantic-families/**`
  - `xtask/src/family/harness.rs`
  - family prove/certify runtime behavior
- Stop immediately if work widens into:
  - corpus leverage-accounting policy changes
  - recommendation decision-policy reinterpretation beyond the M34 rules table
  - promotion execution or blocker artifact mechanics
  - architecture/shared-core implementation
  - runtime ingestion of markdown program ledgers

## Worktree Layout

Canonical worktrees:

- integration and parent-owned code lane
  - branch: `ws/m34-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract/int`
- optional docs lane
  - branch: `ws/m34-lane-b-docs-sync`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract/docs`

Creation rules:

1. The parent captures baseline on the live branch before creating any M34 worktree.
2. `ws/m34-int` is created from the exact SHA recorded in `integration-base.txt`, not from an unrecorded moving `HEAD`.
3. The parent writes `authority-freeze.json` before any worker lane exists.
4. The optional docs lane is forked only from the exact SHA recorded in `artifact-freeze.json`.
5. The parent writes `docs-launch.md` before launching the docs worker.
6. The docs worker launches only from `docs-launch.md` plus the exact freeze-record excerpts named there.
7. No worker is forked from another worker branch.
8. If any named branch or worktree already exists and points at stale or conflicting state, the parent removes and recreates it before reuse and records that in `session-log.md`.
9. A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent does not hand-forward stale worker branches.
10. If the live branch moves after baseline capture, the parent either refreshes baseline and rebuilds the orchestration state or blocks publish. It does not publish over a moved branch tip.

## Parent vs Worker Ownership

### Parent-owned always

- baseline capture
- authority freeze
- schema/path freeze
- command dispatch integration
- validator integration
- deterministic-write integration
- all code edits on the critical path
- first validated `corpus-program-decision.latest.json`
- `artifact-freeze.json`
- `code-freeze.json`
- final merge
- green-path validation
- blocked-path capture
- final verification
- publish and CI observation
- closeout

### Optional worker-owned docs lane

The only safe worker lane is docs sync, and only after `artifact-freeze.json`.
Recommended worker profile:

- `GPT-5.4`
- `reasoning_effort=high`

Allowed lane-B owned paths:

- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`

Forbidden to lane B:

- `PLAN.md`
- `ORCH_PLAN.md`
- `.runs/**`
- `.semantic-family-artifacts/**`
- all `xtask/src/**`
- any file outside the three docs paths above

Lane B mission is narrow:

- explain recommendation analysis as the M33 truth input
- explain corpus-program decision as the M34 next-step output
- explain exact stop vs spend vs pivot meanings
- explain that the current live wedge keeps corpus run `1` unspent and points to an architecture follow-on class
- avoid implying that M34 implemented the follow-on work

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract`
- `ARTIFACT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`

`RUN_ROOT` is a parent-written control plane. Workers may read it, but they do not create, update, or delete files under `RUN_ROOT`.

Canonical parent-owned files:

- `baseline.json`
  - live branch name
  - live checkout SHA
  - dirty-state summary
  - overlap check against the M34-owned surface
  - whether the live branch still matches the expected M33 closed-surface basis
- `integration-base.txt`
  - the exact commit used to seed `ws/m34-int`
  - the frozen execution `--diff-base` for final diff checks
- `publish-head.txt`
  - the exact live branch head captured during baseline
  - the branch tip the parent must preserve at publish time
- `closed-surface-base.txt`
  - the exact commit against which the final M34-owned diff is measured
- `authority-freeze.json`
  - milestone id `M34`
  - authority paths
  - concurrency cap
  - lane map
  - hard guards
  - publish target branch
- `artifact-paths.json`
  - coverage analysis path
  - recommendation analysis path
  - corpus-program decision path
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
  - baseline capture
  - worktree creation
  - freeze creation
  - worker launch
  - merge results
  - stale-lane invalidations
  - publish and CI observation notes
- `schema-freeze.json`
  - frozen path constants
  - frozen artifact kind names
  - frozen action vocabulary
  - frozen basis-code vocabulary
  - frozen required-next-action vocabulary
  - frozen pivot-target-class vocabulary
  - frozen validator invariants
- `basis-freeze.json`
  - exact `recommendation.latest.json` SHA
  - `recommendation_status`
  - `decision_status`
  - `top_candidate_id`
  - `open_blockers`
  - `missing_evidence`
  - `stale_evidence`
  - explicit proof that the live basis still maps to the helper-surface wedge required by `PLAN.md`
- `artifact-freeze.json`
  - exact post-command-integration commit SHA
  - exact `corpus-program-decision.latest.json` SHA
  - emitted `decision_action`
  - emitted `decision_basis_code`
  - emitted `pivot_target_class`
  - emitted `required_next_action`
  - explicit capture that the current basis emits `pivot_to_architecture_shared_core_follow_on`
  - exact launch SHA for the optional docs lane
- `code-freeze.json`
  - exact code-lane post-test commit SHA
  - exact green-path commands
  - exact blocked-path commands
  - exact final verification floor
  - closed diff allowlist
- `docs-launch.md`
  - reproducible parent-owned launch packet for the optional docs lane
  - exact `PLAN.md` excerpt
  - exact `ORCH_PLAN.md` excerpt
  - exact `artifact-freeze.json` excerpt required by the worker
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - freeze record path and frozen SHA
  - required worker return contract
- `merge-log.md`
  - ordered merge history
  - merge SHAs
  - conflict notes
  - stale-lane invalidations
- `green-path-record.json`
  - analysis artifact path and hash
  - decision artifact path and hash
  - commands run
  - validation results
- `proof-log.json`
  - every final verification command
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
  - failing command
  - exit code
  - whether the decision artifact was unchanged or absent
  - required next decision
- `closeout.md`
  - stop/spend/pivot outcome summary
  - wedge summary
  - docs alignment summary
  - scope-control summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-01-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-a1-schema-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-a2-code-integration/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-a3-artifact-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-b-docs-sync/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-02-code-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-03-docs-merge/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-04-green-path/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-05-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-06-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/task-m34-07-closeout/`

Each sentinel directory contains parent-written task state only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m34-00-baseline
  -> task/m34-01-authority-freeze
      -> task/m34-a1-schema-freeze
          -> task/m34-a2-code-integration
              -> task/m34-a3-artifact-freeze
                  -> task/m34-02-code-freeze
task/m34-a3-artifact-freeze
  -> task/m34-b-docs-sync (optional)
task/m34-02-code-freeze
  -> task/m34-03-docs-merge
task/m34-b-docs-sync
  -> task/m34-03-docs-merge
task/m34-03-docs-merge
  -> task/m34-04-green-path
      -> task/m34-05-final-verify
          -> task/m34-06-push-observe
              -> task/m34-07-closeout
```

Execution meaning:

1. Parent proves the current branch still carries the M33 wedge that M34 is supposed to interpret.
2. Parent freezes authority and creates the integration worktree.
3. Parent freezes vocabulary and path before code edits.
4. Parent lands the sequential code lane on `ws/m34-int`.
5. Parent produces and validates the first real decision artifact before any worker starts.
6. Only then may the optional docs lane launch from the frozen SHA.
7. Docs never own code files and never merge before the code lane reaches `code-freeze.json`.
8. Parent then runs the green-path command floor from merged integration state.
9. If any step after `code-freeze.json` fails, the parent emits `blocked.json` and stops publish.
10. Parent publishes only the exact verified `ws/m34-int` SHA.

## Workstream Plan

### WS-0 Baseline capture and wedge proof - parent only

#### `task/m34-00-baseline`

Parent mission:

- capture the live branch baseline and prove that the current M33 truth still matches the M34 helper-surface wedge.

Required parent actions:

1. Confirm the live branch is `feat/corpus-expansion`.
2. Record live head SHA and dirty state.
3. Check for overlapping local edits inside the M34-owned surface.
4. Validate the current recommendation artifact.
5. Prove the current basis still matches the exact M34 wedge from `PLAN.md`.
6. Write `baseline.json`, `integration-base.txt`, `publish-head.txt`, `closed-surface-base.txt`, and `basis-freeze.json`.

Required commands:

```bash
git branch --show-current
git rev-parse --verify HEAD
git status --short
git diff --name-only
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact "$ANALYSIS_PATH"
jq -e '.recommendation_status == "no_strong_candidate"' "$ANALYSIS_PATH"
jq -e '.decision_summary.decision_status == "not_recommended"' "$ANALYSIS_PATH"
jq -e '.decision_summary.top_candidate_id == "z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0"' "$ANALYSIS_PATH"
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' "$ANALYSIS_PATH"
jq -e '.evidence_summary.missing_evidence == [] and .evidence_summary.stale_evidence == []' "$ANALYSIS_PATH"
shasum -a 256 "$ANALYSIS_PATH"
```

Acceptance:

- Live branch is `feat/corpus-expansion`.
- The recommendation artifact validates.
- The live basis still matches the exact helper-surface wedge required by `PLAN.md`.
- Any overlapping local edits inside the M34-owned surface are either absent or explicitly block the run.

### WS-1 Authority freeze and worktree creation - parent only

#### `task/m34-01-authority-freeze`

Parent mission:

- freeze the orchestration contract and create the single integration worktree from the recorded baseline SHA.

Required parent actions:

1. Create `RUN_ROOT` and sentinel directories.
2. Create `ws/m34-int` from `integration-base.txt`.
3. Write `authority-freeze.json`, `artifact-paths.json`, and `tasks.json`.
4. Record the lane map and concurrency cap.
5. Freeze the publish target branch and critical-path ownership.

Required commands:

```bash
BASE_SHA=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/integration-base.txt)
git worktree add -b ws/m34-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract/int "$BASE_SHA"
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m34-stop-spend-pivot-decision-contract/int rev-parse --verify HEAD
```

Acceptance:

- `authority-freeze.json` exists.
- `artifact-paths.json` exists.
- `tasks.json` exists.
- `ws/m34-int` exists and points at the recorded baseline SHA.
- No worker launches before this checkpoint completes.

### WS-2 Schema and path freeze - parent only

#### `task/m34-a1-schema-freeze`

Parent mission:

- freeze the M34 contract vocabulary before any code edits land.

Required parent actions:

1. Freeze the new path:
   - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
2. Freeze the new command:
   - `cargo xtask family corpus-decision --format json`
3. Freeze the artifact kind:
   - `corpus_program_decision`
4. Freeze the schema constant:
   - `CORPUS_PROGRAM_DECISION_SCHEMA_VERSION = 1`
5. Freeze the action vocabulary:
   - `stop`
   - `spend_corpus_run_1`
   - `pivot_to_family_promotion_run`
   - `pivot_to_recommendation_policy_run`
   - `pivot_to_architecture_shared_core_follow_on`
6. Freeze the basis-code vocabulary:
   - `promotion_ready_candidate`
   - `plausible_candidate_missing_evidence`
   - `durable_non_promotable_helper_surface`
   - `no_actionable_candidate`
   - `policy_interpretation_blocker`
7. Freeze the required-next-action vocabulary:
   - `record_stop_without_new_milestone`
   - `author_corpus_expansion_plan`
   - `author_family_promotion_plan`
   - `author_recommendation_policy_plan`
   - `author_architecture_follow_on_plan`
8. Freeze the only valid live output for the current basis:
   - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
   - `decision_basis_code = "durable_non_promotable_helper_surface"`
   - `pivot_target_class = "architecture_shared_core_follow_on"`
   - `required_next_action = "author_architecture_follow_on_plan"`
9. Write `schema-freeze.json`.

Acceptance:

- `schema-freeze.json` exists.
- The path, command, vocabulary, and live wedge are explicitly frozen.
- The docs lane is still disallowed at this checkpoint.

### WS-3 Sequential code integration and first artifact - parent only

#### `task/m34-a2-code-integration`

Parent mission:

- land the coupled code changes on `ws/m34-int` without splitting the lane.

Required parent actions:

1. Extend `xtask/src/family/paths.rs` with the new analysis artifact path constant.
2. Extend `xtask/src/family/promotion_artifacts.rs` with:
   - the new artifact kind
   - the new schema version constant
   - the new serde structs
   - validator logic
   - artifact classification support for `family validate-artifact`
3. Extend `xtask/src/lib.rs` with `FamilyCommand::CorpusDecision`.
4. Extend `xtask/src/family/recommend.rs` with:
   - basis load
   - basis validation
   - bounded decision derivation
   - deterministic latest-byte reuse
5. Keep the code lane sequential. Do not fork a second code worktree.

Acceptance:

- The new command is wired through the existing `xtask family` dispatch surface.
- The new artifact validator lives inside the current `family validate-artifact` path.
- No new crate, runtime parser, or artifact tree is introduced.

#### `task/m34-a3-artifact-freeze`

Parent mission:

- produce the first validated M34 decision artifact and freeze the exact emitted wedge before any docs worker exists.

Required parent actions:

1. Run the current analysis refresh floor from `ws/m34-int`.
2. Run `cargo xtask family corpus-decision --format json`.
3. Validate the new artifact at the exact new path.
4. Assert the emitted live wedge fields.
5. Hash the new artifact and write `artifact-freeze.json`.

Required commands:

```bash
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family corpus-decision --format json
test -f "$DECISION_PATH"
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.artifact_kind == "corpus_program_decision"' "$DECISION_PATH"
jq -e '.analysis_basis_path == ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"' "$DECISION_PATH"
jq -e '.basis_snapshot.recommendation_status == "no_strong_candidate"' "$DECISION_PATH"
jq -e '.basis_snapshot.decision_status == "not_recommended"' "$DECISION_PATH"
jq -e '.basis_snapshot.top_candidate_id == "z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0"' "$DECISION_PATH"
jq -e '.basis_snapshot.open_blockers == ["helper_surface_not_promotable"]' "$DECISION_PATH"
jq -e '.basis_snapshot.missing_evidence == [] and .basis_snapshot.stale_evidence == []' "$DECISION_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.pivot_target_class == "architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
shasum -a 256 "$DECISION_PATH"
```

Acceptance:

- `corpus-program-decision.latest.json` exists at the exact new path.
- The new artifact validates through `cargo xtask family validate-artifact`.
- The current live basis emits `pivot_to_architecture_shared_core_follow_on`.
- `artifact-freeze.json` records the exact artifact SHA and emitted wedge.

### WS-4 Optional docs lane - worker

#### `task/m34-b-docs-sync`

This lane is optional. It exists only if the parent decides docs can safely proceed after `artifact-freeze.json`.

Worker mission:

- align the three maintainer docs to the frozen M34 vocabulary without changing code or authority.

Launch rules:

- the parent launches the worker from `RUN_ROOT/docs-launch.md`
- worker model/profile is pinned to:
  - `GPT-5.4`
  - `reasoning_effort=high`
- the worker may read repo files for local context if needed
- the worker may edit only the three owned docs files
- authority comes only from:
  - `RUN_ROOT/docs-launch.md`
  - the exact excerpts embedded in `docs-launch.md`
  - the frozen `artifact-freeze.json` record referenced by the launch file
- the worker does not infer authority from seeded worktree copies of project docs, independent repo reinterpretation of the wedge, or prior chat context

Required acceptance commands for the worker lane:

```bash
rg -n "corpus-decision|corpus-program-decision.latest.json|stop|spend_corpus_run_1|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
! rg -n "M34 implements the follow-on|corpus run 1 is spent by default|runtime parses the markdown program tracker|new runtime crate|shared-core implementation landed in M34" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
```

Acceptance:

- Only the three owned docs files changed.
- Docs explicitly distinguish recommendation analysis input from corpus-program decision output.
- Docs do not imply that M34 performs corpus execution or shared-core implementation.

### WS-5 Code freeze and docs merge - parent only

#### `task/m34-02-code-freeze`

Parent mission:

- freeze the code lane only after the new command, validator, and tests all pass on `ws/m34-int`.

Required parent actions:

1. Run the targeted M34 xtask tests.
2. Re-run the decision command to prove deterministic bytes on unchanged basis.
3. Freeze the exact green-path, blocked-path, and final-verification commands in `code-freeze.json`.

Required commands:

```bash
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
PRE_SHA=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
cargo xtask family corpus-decision --format json
POST_SHA=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
test "$PRE_SHA" = "$POST_SHA"
```

Acceptance:

- The targeted xtask test floor passes.
- Re-running `cargo xtask family corpus-decision --format json` preserves byte-identical output on unchanged basis input.
- `code-freeze.json` exists before any docs merge.

#### `task/m34-03-docs-merge`

Parent mission:

- merge the optional docs lane only after the code lane has reached `code-freeze.json`.

Acceptance:

- `merge-log.md` exists.
- If the docs lane was launched, it merged only after `code-freeze.json`.
- If the docs lane was skipped, `merge-log.md` records the skip explicitly and `task-m34-b-docs-sync` closes as a no-op.
- No doc wording conflicts with `artifact-freeze.json` or `code-freeze.json`.

### WS-6 Green-path validation and blocked path - parent only

#### `task/m34-04-green-path`

Parent mission:

- run the exact merged-state green-path floor and validate the new artifact at the exact new path.

Required green-path commands:

```bash
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family corpus-decision --format json
test -f "$DECISION_PATH"
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.analysis_basis_path == ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"' "$DECISION_PATH"
jq -e '.basis_snapshot.recommendation_status == "no_strong_candidate"' "$DECISION_PATH"
jq -e '.basis_snapshot.decision_status == "not_recommended"' "$DECISION_PATH"
jq -e '.basis_snapshot.open_blockers == ["helper_surface_not_promotable"]' "$DECISION_PATH"
jq -e '.basis_snapshot.missing_evidence == [] and .basis_snapshot.stale_evidence == []' "$DECISION_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.pivot_target_class == "architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
```

Green-path acceptance:

- The recommendation artifact validates at the existing analysis path.
- The new decision artifact validates at `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`.
- The new artifact preserves the exact live wedge required by `PLAN.md`.
- `green-path-record.json` records both artifact hashes and the command floor.

Canonical blocked-path evidence-capture commands if any post-`code-freeze.json` green-path step fails:

```bash
RUN_ROOT="/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract"
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
FAILING_COMMAND="${FAILING_COMMAND:?set FAILING_COMMAND to the exact command that failed}"
FAILING_EXIT_CODE="${FAILING_EXIT_CODE:?set FAILING_EXIT_CODE to the exact non-zero exit code}"
printf '%s\n' "$FAILING_COMMAND" > "$RUN_ROOT/blocked-failing-command.txt"
printf '%s\n' "$FAILING_EXIT_CODE" > "$RUN_ROOT/blocked-failing-exit-code.txt"
ANALYSIS_SHA_BEFORE=$(shasum -a 256 "$ANALYSIS_PATH" | awk '{print $1}')
printf '%s\n' "$ANALYSIS_SHA_BEFORE" > "$RUN_ROOT/blocked-analysis.sha256"
DECISION_PRESENT_BEFORE=0
DECISION_SHA_BEFORE=""
if [ -f "$DECISION_PATH" ]; then
  DECISION_PRESENT_BEFORE=1
  DECISION_SHA_BEFORE=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
fi
printf '%s\n' "$DECISION_PRESENT_BEFORE" > "$RUN_ROOT/blocked-decision.present-before"
printf '%s\n' "$DECISION_SHA_BEFORE" > "$RUN_ROOT/blocked-decision.sha-before"
cargo xtask family validate-artifact "$ANALYSIS_PATH"
ANALYSIS_SHA_AFTER=$(shasum -a 256 "$ANALYSIS_PATH" | awk '{print $1}')
printf '%s\n' "$ANALYSIS_SHA_AFTER" > "$RUN_ROOT/blocked-analysis.sha256.after"
test "$ANALYSIS_SHA_BEFORE" = "$ANALYSIS_SHA_AFTER"
if [ -f "$DECISION_PATH" ]; then
  cargo xtask family validate-artifact "$DECISION_PATH"
  DECISION_PRESENT_AFTER=1
  DECISION_SHA_AFTER=$(shasum -a 256 "$DECISION_PATH" | awk '{print $1}')
else
  DECISION_PRESENT_AFTER=0
  DECISION_SHA_AFTER=""
fi
printf '%s\n' "$DECISION_PRESENT_AFTER" > "$RUN_ROOT/blocked-decision.present-after"
printf '%s\n' "$DECISION_SHA_AFTER" > "$RUN_ROOT/blocked-decision.sha-after"
if [ "$DECISION_PRESENT_BEFORE" = "1" ] && [ "$DECISION_PRESENT_AFTER" = "1" ]; then
  if [ "$DECISION_SHA_BEFORE" = "$DECISION_SHA_AFTER" ]; then
    printf 'stable\n' > "$RUN_ROOT/blocked-decision.change-status"
  else
    printf 'changed_unexpectedly\n' > "$RUN_ROOT/blocked-decision.change-status"
  fi
elif [ "$DECISION_PRESENT_BEFORE" = "0" ] && [ "$DECISION_PRESENT_AFTER" = "1" ]; then
  printf 'appeared_during_failure_window\n' > "$RUN_ROOT/blocked-decision.change-status"
elif [ "$DECISION_PRESENT_BEFORE" = "1" ] && [ "$DECISION_PRESENT_AFTER" = "0" ]; then
  printf 'disappeared_during_failure_window\n' > "$RUN_ROOT/blocked-decision.change-status"
else
  printf 'absent_both_before_and_after\n' > "$RUN_ROOT/blocked-decision.change-status"
fi
test "$FAILING_EXIT_CODE" -ne 0
```

Blocked-path rules:

- The parent writes the sentinel terminal blocked state for the failing task.
- The parent writes `blocked.json`.
- The parent preserves the actual failing command and exit code from the step that failed.
- The parent preserves and validates the current analysis artifact.
- The parent preserves and validates the current decision artifact if it exists.
- The parent records whether the decision artifact remained stable, changed unexpectedly, appeared, disappeared, or was absent throughout the failure window.
- The parent stops downstream publish and closeout.
- The parent does not report partial green success.

### WS-7 Final verification - parent only

#### `task/m34-05-final-verify`

The parent must run this exact merged-state verification floor from `ws/m34-int` before calling M34 done:

```bash
cargo fmt --all --check
cargo clippy -p xtask --all-targets --all-features -- -D warnings
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.recommendation_status == "no_strong_candidate"' "$ANALYSIS_PATH"
jq -e '.decision_summary.decision_status == "not_recommended"' "$ANALYSIS_PATH"
jq -e '.decision_summary.top_candidate_id == "z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0"' "$ANALYSIS_PATH"
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' "$ANALYSIS_PATH"
jq -e '.evidence_summary.missing_evidence == [] and .evidence_summary.stale_evidence == []' "$ANALYSIS_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.pivot_target_class == "architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
cargo test -p xtask -- --color never
rg -n "corpus-decision|corpus-program-decision.latest.json|stop|spend_corpus_run_1|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md PLAN.md
! rg -n "M34 implements the follow-on|corpus run 1 is spent by default|runtime parses the markdown program tracker|new runtime crate|shared-core implementation landed in M34" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md
CLOSED_SURFACE_BASE_SHA=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m34_stop_spend_pivot_decision_contract/closed-surface-base.txt)
git diff --name-only "${CLOSED_SURFACE_BASE_SHA}...HEAD"
! git diff --name-only "${CLOSED_SURFACE_BASE_SHA}...HEAD" | rg -v '^(xtask/src/(lib|family/(recommend|promotion_artifacts|paths|mod))\.rs|semantic-families/README\.md|docs/(recommendation_corpus_expansion_program_v0\.1|semantic_family_capability_corpus_guide_v0\.1)\.md)$'
```

Rules:

- Record every actual command and exit code in `proof-log.json`.
- Do not substitute broader or different commands for the sequence above.
- If any command fails after `code-freeze.json` exists, the parent must emit `blocked.json` before stopping.
- M34 is not done if the floor passes but the diff escapes the closed implementation surface.

### WS-8 Publish and CI observation - parent only

#### `task/m34-06-push-observe`

Required parent actions:

1. Confirm the verified `ws/m34-int` commit is a descendant of the preserved live branch head recorded in `publish-head.txt` and can fast-forward `feat/corpus-expansion` without discarding unrelated work.
2. If and only if that fast-forward is safe, update the publish target to the exact verified integration SHA.
3. Push `feat/corpus-expansion`.
4. Record remote, branch, SHA, and timestamp in `push-record.json`.
5. Observe the CI run triggered by that exact pushed SHA.
6. Record workflow name, run id or URL, observed SHA, and workspace result in `ci-observation.json`.

Acceptance:

- Publish branch is the exact verified SHA from `ws/m34-int`.
- Push succeeded.
- CI ran on the exact pushed SHA.
- Workspace CI is green.

### WS-9 Closeout - parent only

#### `task/m34-07-closeout`

Closeout must write `closeout.md` and answer plainly:

1. Does the repo now emit one bounded machine-readable stop/spend/pivot decision without hidden chat context?
2. Does the current helper-surface wedge deterministically map to `pivot_to_architecture_shared_core_follow_on`?
3. Does the new artifact validate at `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`?
4. Is the basis snapshot explicit enough to explain why corpus run `1` remains unspent?
5. Do the docs use the same vocabulary the new artifact emits?
6. Did the run avoid corpus execution, recommendation-policy redesign, family promotion execution changes, shared-core implementation, markdown runtime parsing, new runtime crate work, and new artifact trees?
7. If the run stopped after `code-freeze.json`, was `blocked.json` emitted before stop?

Allowed closeout verdicts:

- `PROCEED`
  - M34 landed cleanly and the repo now has an authoritative next-step decision contract
- `NARROW`
  - M34 landed materially, but one bounded wording or verification follow-on still blocks a clean next milestone
- `STOP`
  - The run widened scope, failed verification, or left the repo overclaiming what M34 actually did

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
- the exact relevant freeze-record excerpt for that lane
- owned paths
- forbidden paths
- exact acceptance commands
- applicable hard guards
- worker model/profile:
  - `GPT-5.4`
  - `reasoning_effort=high`
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

For the optional docs lane specifically:

- the parent must issue `RUN_ROOT/docs-launch.md`
- that file is the only operative worker launch packet
- it must embed the exact `PLAN.md`, `ORCH_PLAN.md`, and `artifact-freeze.json` excerpts the worker is allowed to rely on
- it must pin worker model/profile to `GPT-5.4` with `reasoning_effort=high`
- the worker may read repo files for local context, but it does not reconstruct wedge wording or scope from the repo independently when the launch file already freezes them

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
  - inferred milestone scope beyond M34
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen implementation beyond the M34 closed surface
- they need to change the frozen schema vocabulary after `schema-freeze.json`
- they need to change the frozen wedge wording after `artifact-freeze.json`
- they cannot satisfy acceptance commands with concrete evidence
- they discover overlapping external edits inside their owned surface after launch
- they discover a need to touch corpus policy, `spec-core`, promoted family packets, runtime markdown parsing, or shared-core implementation

Parent blocker response:

1. Write the sentinel terminal blocked state for the blocked task.
2. Write `blocked.json`.
3. Preserve the failing command, exit code, analysis-artifact evidence, and decision-artifact state evidence in `RUN_ROOT`.
4. Stop downstream launches, publish, and closeout.
5. Do not report partial green success.

## Freeze Checkpoints

### Checkpoint 0: Baseline freeze

Required:

- `baseline.json` exists
- live branch is `feat/corpus-expansion`
- dirty overlap inside the M34-owned surface is either absent or explicitly blocked
- `basis-freeze.json` proves the M33 helper-surface wedge still matches `PLAN.md`

### Checkpoint 1: Authority freeze

Required:

- `authority-freeze.json` exists
- `artifact-paths.json` exists
- `publish-head.txt` exists
- `closed-surface-base.txt` exists
- `ws/m34-int` was created from the recorded baseline SHA

### Checkpoint 2: Schema freeze

Required:

- `schema-freeze.json` exists
- frozen schema versions, path, and vocabulary are recorded
- the docs lane is still disallowed at this checkpoint

### Checkpoint 3: Artifact freeze

Required:

- `artifact-freeze.json` exists
- the current decision artifact validates at the exact new path
- the current basis emits `pivot_to_architecture_shared_core_follow_on`
- if the docs lane is launched, it starts from the exact artifact-freeze SHA
- `docs-launch.md` exists and points at `artifact-freeze.json`

### Checkpoint 4: Code freeze

Required:

- `code-freeze.json` exists
- the targeted xtask test floor passes
- deterministic re-run behavior is proven on unchanged basis input

### Checkpoint 5: Docs merge

Required:

- if the docs lane was launched, its acceptance commands pass on merged integration state
- `merge-log.md` records the merge result or explicit skip

### Checkpoint 6: Green-path validation

Required:

- coverage, recommendation, and corpus-decision paths validate
- `green-path-record.json` exists
- the exact new artifact path `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json` has been validated in merged state

### Checkpoint 7: Final verification

Required:

- the exact merged-state verification floor passes
- `proof-log.json` records every command and exit code
- the final merged diff stays inside the M34 closed surface plus allowed mechanical spillover

## Tests And Acceptance

The required command floor is locked:

```bash
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
DECISION_PATH=".semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family validate-artifact "$DECISION_PATH"
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' "$DECISION_PATH"
jq -e '.pivot_target_class == "architecture_shared_core_follow_on"' "$DECISION_PATH"
jq -e '.required_next_action == "author_architecture_follow_on_plan"' "$DECISION_PATH"
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate -- --color never
```

Additional acceptance rules:

- if the live recommendation artifact no longer matches the helper-surface wedge, M34 is blocked until authority is refreshed
- if the decision artifact is written anywhere except `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`, M34 is incomplete
- if the decision artifact validates but does not emit `pivot_to_architecture_shared_core_follow_on` on the current basis, M34 is incomplete
- if contradictory action/basis combinations are not rejected by validation, M34 is incomplete
- if rerunning the command on unchanged basis input changes bytes, M34 is incomplete
- if docs imply that M34 executed corpus work or implemented the architecture follow-on, M34 is incomplete
- if the diff touches corpus manifest policy, `spec-core`, family packets, prove/certify execution, runtime markdown parsing, or a new crate, the run is blocked

## Assumptions

- `feat/corpus-expansion` remains the publish target branch for this run.
- The current recommendation artifact remains the authoritative M33 analysis input for this run.
- `cargo xtask family validate-artifact` remains the stable validator entrypoint during this run.
- The current live basis continues to expose the helper-surface durable-hold wedge at run start.
- The optional docs lane can be skipped entirely if timing, drift, or merge risk makes it unsafe.
- No new family promotion, corpus-run execution, or shared-core implementation is required to make the M34 contract honest.

## Freeze And Restart Rules

- No lane launches before the parent writes `authority-freeze.json`.
- The optional docs lane may launch only after `artifact-freeze.json` exists and `docs-launch.md` has been written.
- If baseline changes after `baseline.json`, every downstream lane is stale and must be recreated from the new baseline.
- If `schema-freeze.json` changes before `artifact-freeze.json`, no docs lane may launch until a fresh artifact freeze is created on top of the new schema.
- If `artifact-freeze.json` changes after the docs lane is forked, the docs lane is stale and must be recreated from the new frozen SHA.
- If `code-freeze.json` changes any field name, command contract, artifact path, or current-wedge wording after the docs lane is forked, the docs lane is stale and must be recreated.
- If `feat/corpus-expansion` moves after baseline capture, the parent must either refresh baseline and replay planning against the new head or block publish. The parent does not force-publish over a moved branch tip.
- If overlapping third-party edits land anywhere inside a lane-owned surface after launch, the parent records the overlap, invalidates the affected lanes, and relaunches from the newest relevant freeze.
- The parent does not hand-patch stale worker branches.
- Any request to widen M34 into corpus execution, recommendation-policy redesign, family promotion execution changes, runtime markdown parsing, new crate work, or shared-core implementation blocks the run until `PLAN.md` is rewritten.
