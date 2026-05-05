# M33 Orchestration Plan

Status: **authoritative execution contract for the M33 run**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Live branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Last rewritten: **`2026-05-04`**  
Required re-anchor: **publish SHA `6a1051b601487710d631031171cfde92810f1581` or a direct descendant explicitly proven by the parent to preserve closed M32 artifact truth**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m33-recommendation-quality-promotion-decisions`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**

## Summary

- This run is for **M33 recommendation-quality promotion decisions** only.
- `PLAN.md` remains milestone authority. `ORCH_PLAN.md` becomes the parent-owned execution contract.
- The parent agent is the sole integrator, sole freeze authority, sole stale-lane invalidator, sole merge authority, sole push authority, sole CI observer, sole blocker emitter, and sole final verifier.
- The core implementation lane stays **mostly sequential** because `xtask/src/family/recommend.rs`, `xtask/src/family/promotion_artifacts.rs`, `xtask/src/family/paths.rs`, and `xtask/src/lib.rs` all share one tightly coupled artifact contract.
- The parent owns the critical path locally on `ws/m33-int`: M32 re-anchor, baseline capture, authority freeze, schema freeze, decision projection, downstream artifact propagation, code freeze, green-path artifact emission, final verification, publish, observe, and closeout.
- At most one narrow worker lane is allowed: a late docs closeout lane. It may start only after `analysis-freeze.json` exists because the docs need the frozen M33 wedge, the real emitted artifact wording, and the recorded current `money/round` interpretation, not just frozen enum names.
- Recommended worker profile for the optional docs lane is **`GPT-5.4` with `reasoning_effort=high`**.
- Worker concurrency cap is **`0`** before `analysis-freeze.json` and **`1`** after `analysis-freeze.json` if the optional docs lane is launched.
- The one real downstream validation family for M33 is still the bounded M32 path: `function.arithmetic_leaf.monotone_up.v1` with `--target-language typescript`.
- M33 does not reopen proof policy. It improves the decision surface and carries that truth through the existing family-promotion artifact chain.
- Parent-owned run-state under `RUN_ROOT` is the only execution truth. Worker memory, stale worktree files, and ad hoc notes are not.

## Hard Guards

- `PLAN.md` wins over this file, worker summaries, stale worktree copies, and run-state notes if they disagree.
- `ORCH_PLAN.md` is parent-owned only. Workers do not edit it.
- The parent does not integrate on the live checkout. All merges and final verification happen on `ws/m33-int`.
- The live checkout on `feat/corpus-expansion` is the publish target and baseline reference, not the merge surface.
- M33 starts only from the M32 publish anchor `6a1051b601487710d631031171cfde92810f1581` or a direct descendant explicitly recorded in `m32-base-freeze.json` as preserving the closed M32 artifact chain.
- The primary analysis artifact path stays fixed at `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`.
- The M26 root artifact path `.semantic-family-artifacts/family-promotion/recommendation.latest.json` is not repurposed into the M33 decision surface.
- No one hand-edits JSON under `ARTIFACT_ROOT`. Derived artifacts are created only by repo commands and validated as produced output.
- After `authority-freeze.json` is written, both `PLAN.md` and `ORCH_PLAN.md` are frozen. If either authority file must change after that point, stop the run, write blocker state, and restart from a new authority baseline.
- The closed implementation surface for M33 is:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
  - `xtask/src/lib.rs`
  - `semantic-families/README.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
  - `docs/semantic_family_capability_corpus_guide_v0.1.md`
  - `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
  - `CHANGELOG.md`
- Allowed mechanical spillover is compile-, test-, or module-wire-forced only:
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/coverage.rs`
- Stop immediately if any lane requires edits to:
  - `semantic-families/corpus/rust-function.toml`
  - any `spec-core/src/**` file
  - any promoted family packet directory
  - `xtask/src/family/harness.rs`
  - prove/certify runtime semantics
- Stop immediately if work widens into:
  - corpus-accounting policy redesign
  - source-kind leverage policy changes
  - `spec-core` semantic capability expansion
  - family promotion or certification of a new family
  - broad target-language or repo-wide TypeScript claims
- The parent may resolve only syntax-level, formatting, import-order, or context-drift merge fallout. Semantic ownership conflicts go back to the owning lane.

## Worktree Layout

Canonical worktrees:

- integration and sequential code lane
  - branch: `ws/m33-int`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m33-recommendation-quality-promotion-decisions/int`
- optional docs lane
  - branch: `ws/m33-lane-b-docs-closeout`
  - path: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m33-recommendation-quality-promotion-decisions/docs`

Creation rules:

1. The parent first proves the M32 base in `m32-base-freeze.json`.
2. The parent records live branch, live SHA, dirty state, and overlap before creating any M33 worktree.
3. `ws/m33-int` is created from the exact commit recorded in `m32-base-freeze.json`, not from an unrecorded live `HEAD`.
4. The optional docs lane is forked only from the exact analysis-freeze SHA recorded in `analysis-freeze.json`.
5. The parent writes `docs-launch.md` before launching the docs worker.
6. The docs worker launches only from `docs-launch.md` plus the frozen authority excerpts and freeze record referenced by that file.
7. No worker is forked from another worker branch.
8. If any named branch or worktree already exists and points at stale or conflicting state, the parent removes and recreates it before reuse and records that in `session-log.md`.
9. A stale lane is discarded and recreated from the newest relevant freeze SHA. The parent does not hand-forward stale worker branches.

## Canonical Run-State

Parent-owned orchestration truth lives under:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m33-recommendation-quality-promotion-decisions`
- `ARTIFACT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`

`RUN_ROOT` is a parent-written control plane. Workers may read it, but they do not create, update, or delete files under `RUN_ROOT`.

Canonical parent-owned files:

- `m32-base-freeze.json`
  - required publish anchor SHA
  - chosen integration seed SHA
  - proof that the chosen seed still preserves closed M32 monotone-up artifact truth
- `baseline.json`
  - live branch name
  - live checkout SHA
  - live dirty-state summary
  - overlap check against the M33-owned surface
- `integration-base.txt`
  - the exact commit used to seed `ws/m33-int`
  - the only allowed diff base for the final closed-surface gate
- `authority-freeze.json`
  - milestone id `M33`
  - authority paths
  - concurrency cap
  - lane map
  - hard guards
  - publish target branch
- `run-id.txt`
  - the single canonical M33 run id selected by the parent
  - parent-owned convention: `{UTC-basic-timestamp}-function.arithmetic_leaf.monotone_up.v1`
  - this convention is chosen because it is compatible with the existing family-promotion artifact command surface, not because `RUN_ROOT` itself defines schema
- `artifact-paths.json`
  - analysis coverage path
  - analysis recommendation path
  - family-scoped monotone-up recommendation path
  - green-path execution artifact path
  - blocked-path blocker artifact path
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
- `schema-freeze.json`
  - frozen schema versions
  - expected artifact field names
  - expected enum vocabulary
  - expected artifact paths
- `analysis-freeze.json`
  - exact post-Step-2 commit SHA
  - exact current merged-state analysis artifact SHA
  - `recommendation_status`
  - `decision_status`
  - top candidate id
  - blocker reasons
  - evidence summary
  - delta summary
  - explicit capture of the current `money/round` helper-surface wedge
  - exact launch SHA for the optional docs lane
- `code-freeze.json`
  - exact code-lane post-Step-3 commit
  - exact analysis refresh and validation commands
  - exact family recommendation refresh and validation commands
  - exact green-path execution-emission command
  - exact blocked-path blocker-emission command template
  - exact final verification floor
  - closed diff allowlist
- `docs-launch.md`
  - reproducible parent-owned launch packet for the optional docs lane
  - exact `PLAN.md` excerpt
  - exact `ORCH_PLAN.md` excerpt
  - exact `analysis-freeze.json` excerpt required by the worker
  - owned paths
  - forbidden paths
  - exact acceptance commands
  - applicable hard guards
  - worker model/profile: `GPT-5.4` with `reasoning_effort=high`
  - freeze record path and frozen SHA
  - required worker return contract
- `merge-log.md`
  - ordered merge history
  - merge SHAs
  - conflict notes
  - stale-lane invalidations
- `green-path-record.json`
  - run id
  - family
  - target language
  - analysis path and hash
  - family recommendation path and hash
  - execution artifact path and hash
  - diff base
  - commands run
  - validation results
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
  - decision-surface summary
  - downstream artifact-chain summary
  - docs alignment summary
  - final verdict

Per-task sentinel directories:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-00-reanchor/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-01-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-02-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-a1-schema-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-a2-analysis-decision/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-a3-downstream-artifacts/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-b-docs-closeout/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-03-code-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-04-docs-merge/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-05-green-path-artifacts/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-06-final-verify/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-07-push-observe/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/task-m33-08-closeout/`

Each sentinel directory contains parent-written task state only:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

## Task Graph

```text
task/m33-00-reanchor
  -> task/m33-01-baseline
      -> task/m33-02-authority-freeze
          -> task/m33-a1-schema-freeze
              -> task/m33-a2-analysis-decision
                  -> task/m33-a3-downstream-artifacts
                      -> task/m33-03-code-freeze
task/m33-a2-analysis-decision
  -> task/m33-b-docs-closeout (optional)
task/m33-03-code-freeze
  -> task/m33-04-docs-merge
task/m33-b-docs-closeout
  -> task/m33-04-docs-merge
task/m33-04-docs-merge
  -> task/m33-05-green-path-artifacts
      -> task/m33-06-final-verify
          -> task/m33-07-push-observe
              -> task/m33-08-closeout
```

Execution meaning:

1. Parent proves the closed M32 base and records the exact M33 integration seed.
2. Parent captures live branch state and overlap facts.
3. Parent freezes orchestration authority and creates the integration worktree.
4. Parent lands the sequential code lane on `ws/m33-int`.
5. After `analysis-freeze.json`, the parent may optionally fork a docs lane from the frozen analysis SHA.
6. Docs never own code files and never merge before the code lane reaches `code-freeze.json`.
7. Parent merges docs only after the code lane is complete and re-verified.
8. Parent then emits the real downstream green-path artifacts from merged integration state.
9. If any post-`code-freeze.json` step fails, the parent emits and validates a real blocker artifact before stopping.
10. Parent publishes only the exact verified `ws/m33-int` SHA.

## Workstream Plan

### WS-0 Re-anchor on the validated M32 base - parent only

#### `task/m33-00-reanchor`

Required parent actions:

1. Confirm the live branch is `feat/corpus-expansion`.
2. Confirm the chosen seed commit is either `6a1051b601487710d631031171cfde92810f1581` or a direct descendant that still preserves M32 artifact truth.
3. Validate the closed M32 monotone-up proof artifacts from the chosen seed.
4. Validate the current M27/M32 analysis artifacts from the chosen seed.
5. Write `m32-base-freeze.json`.

Required commands:

```bash
git rev-parse --verify 6a1051b601487710d631031171cfde92810f1581
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/prove.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/certification.report.json
ATTEMPT_PATH=$(ls -t .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/attempt-*.json | head -n 1)
test -n "$ATTEMPT_PATH"
cargo xtask family validate-artifact "$ATTEMPT_PATH"
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Acceptance:

- `m32-base-freeze.json` exists.
- The chosen integration seed is recorded exactly.
- The bounded monotone-up proof chain validates from that seed.
- The existing analysis artifacts validate from that seed.
- No code lane may start until this freeze exists.

### WS-1 Baseline capture - parent only

#### `task/m33-01-baseline`

Required parent actions:

1. Record live branch, live SHA, and dirty-state summary.
2. Record overlap against the M33 closed implementation surface.
3. Record whether unrelated dirty work exists outside M33 scope.
4. Write `baseline.json`.

Acceptance:

- `baseline.json` exists.
- Live branch is `feat/corpus-expansion`.
- Dirty overlap inside the M33-owned surface is either absent or explicitly blocked before integration starts.

### WS-2 Orchestration freeze - parent only

#### `task/m33-02-authority-freeze`

Required parent actions:

1. Create `RUN_ROOT`.
2. Write `authority-freeze.json`.
3. Write `tasks.json`.
4. Write `artifact-paths.json`.
5. Write `run-id.txt`.
6. Write `integration-base.txt` from the exact seed recorded in `m32-base-freeze.json`.
7. Create `ws/m33-int` from that exact seed commit.

Artifact path contract to freeze:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/promotion.execution.json`
- `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`

Acceptance:

- No worker launches before `authority-freeze.json`.
- `ORCH_PLAN.md`, `authority-freeze.json`, and `tasks.json` agree on lane order, hard guards, publish target, and freeze semantics.
- After `authority-freeze.json`, `PLAN.md` and `ORCH_PLAN.md` are frozen and may not re-enter runtime scope unless the run is explicitly aborted and replanned.

### WS-3 Sequential code lane - parent only on `ws/m33-int`

#### `task/m33-a1-schema-freeze`

Mission:

- freeze the M33 artifact contract before any downstream code or docs work depends on it.

Owned paths:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`

Required parent actions:

1. Land the schema and validator changes on `ws/m33-int`.
2. Freeze the expected M33 schema versions:
   - analysis artifact `schema_version = 4`
   - family-scoped recommendation artifact `schema_version = 2`
   - execution artifact `schema_version = 2`
   - blocker artifact `schema_version = 2`
3. Freeze the new decision vocabulary:
   - `recommended`
   - `blocked_for_now`
   - `not_recommended`
4. Freeze the new blocker and evidence vocabulary from `PLAN.md`.
5. Write `schema-freeze.json`.

Required acceptance commands:

```bash
cargo test -p xtask artifact_schema_ -- --color never
```

Acceptance:

- Artifact paths remain unchanged.
- Contradictory artifact combinations now fail in validator tests.
- `schema-freeze.json` exists and records the frozen field vocabulary.
- No docs lane may launch from schema freeze alone.

#### `task/m33-a2-analysis-decision`

Mission:

- make the primary analysis artifact tell the full M33 decision story.

Owned paths:

- `xtask/src/family/recommend.rs`
- allowed spillover only if forced:
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/mod.rs`

Required parent actions:

1. Implement the decision projection in `recommend.rs`.
2. Keep `recommendation_status` as the compatibility field.
3. Make missing and stale evidence explicit.
4. Make delta deterministic against the previous validated analysis artifact at the same path.
5. Recompute the live analysis artifact from merged state.
6. Write `analysis-freeze.json`.
7. If useful, create `docs-launch.md` and fork `ws/m33-lane-b-docs-closeout` from the exact SHA recorded in `analysis-freeze.json`.

Required acceptance commands:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo test -p xtask recommendation_ -- --color never
```

Acceptance:

- The current repo wedge renders as:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "not_recommended"`
  - visible top candidate `unsupported_function_surface-e40675da6fa0`
  - durable blocker `helper_surface_not_promotable`
- The analysis artifact exposes explicit missing/stale evidence fields even when empty.
- The analysis artifact exposes `delta_from_previous`.
- `analysis-freeze.json` records the exact current wedge and artifact SHA.
- If the docs lane is launched, it launches only after `analysis-freeze.json` and only from the SHA recorded there.
- `docs-launch.md` exists before any docs worker starts.
- A plausible held candidate path is covered in tests as `blocked_for_now`.
- A promotion-ready path is covered in tests as `recommended`.

#### `task/m33-a3-downstream-artifacts`

Mission:

- thread the analysis basis through the downstream artifact chain without duplicating policy.

Owned paths:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- allowed spillover only if forced:
  - `xtask/src/family/mod.rs`

Required parent actions:

1. Update family-scoped recommendation emission to carry analysis-basis truth.
2. Update execution and blocker emission to carry the same basis.
3. Preserve bounded M32 target-language honesty on the monotone-up path.
4. Prove the downstream schema through tests and merged-state command execution.
5. Write `code-freeze.json`.

Required acceptance commands:

```bash
cargo xtask family refresh-promotion-recommendation function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json
cargo test -p xtask family_refresh_promotion_recommendation -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask -- --color never
```

Acceptance:

- The family-scoped recommendation artifact cites the analysis basis directly.
- The downstream artifacts carry basis truth instead of recomputing policy.
- No downstream artifact implies repo-wide target-language readiness.
- `code-freeze.json` exists.
- `code-freeze.json` records the exact argv for:
  - analysis refresh and validation
  - family recommendation refresh and validation
  - green-path execution emission
  - blocked-path blocker emission

### WS-4 Optional docs lane - worker

#### `task/m33-b-docs-closeout` on `ws/m33-lane-b-docs-closeout`

Worker mission:

- align the human-facing docs with the frozen M33 vocabulary and the real landed wedge, without widening scope.

Worker profile:

- model: `GPT-5.4`
- `reasoning_effort=high`

Owned paths:

- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `CHANGELOG.md`

Worker must not do:

- edit any `xtask/src/**` file
- edit any `spec-core/src/**` file
- edit any packet directory
- edit `PLAN.md`
- edit `ORCH_PLAN.md`

Worker launch authority:

- the parent launches the worker from `RUN_ROOT/docs-launch.md`
- that launch file must point at the frozen `analysis-freeze.json`
- the worker may use only:
  - `docs-launch.md`
  - the exact `PLAN.md` excerpt embedded in that file
  - the exact `ORCH_PLAN.md` excerpt embedded in that file
  - the exact `analysis-freeze.json` excerpt embedded in that file
- the worker does not infer authority from the seeded worktree copy of project docs or from prior chat context

Required acceptance commands:

```bash
rg -n "recommended|blocked_for_now|not_recommended|money/round|function.arithmetic_leaf.monotone_up.v1|recommendation.latest.json|bounded second-language" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md CHANGELOG.md
! rg -n "repo-wide TypeScript support|broad TypeScript support|all families now support TypeScript|new promoted family|corpus run 1 spent by M33|spec-core capability expansion" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md CHANGELOG.md
```

Acceptance:

- Docs use the frozen M33 decision vocabulary exactly.
- Docs describe the live `money/round` wedge as visible but not the next family.
- Docs preserve the M32 bounded monotone-up target-language claim and nothing broader.

### WS-5 Parent docs merge - parent only

#### `task/m33-04-docs-merge`

Strict merge order:

1. Finish the sequential code lane on `ws/m33-int`.
2. Re-run all code-lane acceptance commands from merged state.
3. If the docs lane exists, merge `ws/m33-lane-b-docs-closeout` into `ws/m33-int`.
4. Re-run docs acceptance commands from merged state.
5. Record merge SHAs, conflicts, and stale-lane decisions in `merge-log.md`.

Parent may resolve only:

- formatting drift
- line-local doc merge drift
- wording updates required to match the frozen field names and current wedge exactly

Parent must bounce work back to the owning lane for:

- any post-analysis-freeze vocabulary change
- any post-analysis-freeze wedge interpretation change without a refreshed `analysis-freeze.json`
- any attempt by docs to widen M32 into broad target-language readiness
- any code change
- any change to the frozen closed surface after `code-freeze.json`

Acceptance:

- `merge-log.md` exists.
- If the docs lane was launched, it merged only after the code lane reached `code-freeze.json`.
- If the docs lane was skipped, `merge-log.md` records the skip explicitly and `task-m33-b-docs-closeout` closes as a no-op.
- No doc wording conflicts with `analysis-freeze.json` or `code-freeze.json`.

### WS-6 Runtime green-path artifact emission - parent only

#### `task/m33-05-green-path-artifacts`

Parent mission:

- emit the real downstream M33 artifacts from merged `ws/m33-int` state for the bounded monotone-up path.

Required parent actions:

1. Read `run-id.txt`, `artifact-paths.json`, and `code-freeze.json`.
2. Refresh the analysis artifacts from merged state.
3. Refresh the monotone-up family recommendation with `--target-language typescript`.
4. Emit the green-path `promotion.execution.json` from merged state using the frozen `diff_base`.
5. Validate the generated artifacts.
6. Write `green-path-record.json`.

Required commands:

```bash
RUN_ID=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/run-id.txt)
DIFF_BASE=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/integration-base.txt)
ANALYSIS_PATH=".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
FAMILY_RECOMMENDATION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json"
EXECUTION_PATH=".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/${RUN_ID}/promotion.execution.json"
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact "$ANALYSIS_PATH"
cargo xtask family refresh-promotion-recommendation function.arithmetic_leaf.monotone_up.v1 --target-language typescript
test -f "$FAMILY_RECOMMENDATION_PATH"
cargo xtask family validate-artifact "$FAMILY_RECOMMENDATION_PATH"
cargo xtask family emit-promotion-execution function.arithmetic_leaf.monotone_up.v1 "$RUN_ID" "$FAMILY_RECOMMENDATION_PATH" --target-language typescript --diff-base "$DIFF_BASE"
test -f "$EXECUTION_PATH"
cargo xtask family validate-artifact "$EXECUTION_PATH"
```

Acceptance:

- The analysis artifact validates at the existing path.
- The family-scoped monotone-up recommendation artifact validates at the existing path.
- `promotion.execution.json` exists at the frozen family path and validates.
- `green-path-record.json` records the analysis basis, family recommendation basis, and emitted execution artifact path.

### WS-7 Final verification - parent only

#### `task/m33-06-final-verify`

The parent must run this exact merged-state verification floor from `ws/m33-int` before calling M33 done:

```bash
cargo fmt --all --check
cargo clippy -p xtask --all-targets --all-features -- -D warnings
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/prove.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/certification.report.json
ATTEMPT_PATH=$(ls -t .semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/attempt-*.json | head -n 1)
test -n "$ATTEMPT_PATH"
cargo xtask family validate-artifact "$ATTEMPT_PATH"
cargo xtask family refresh-promotion-recommendation function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json
RUN_ID=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/run-id.txt)
DIFF_BASE=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/integration-base.txt)
cargo xtask family emit-promotion-execution function.arithmetic_leaf.monotone_up.v1 "$RUN_ID" ".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json" --target-language typescript --diff-base "$DIFF_BASE"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/${RUN_ID}/promotion.execution.json"
cargo test -p xtask family_refresh_promotion_recommendation -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask recommendation_ -- --color never
cargo test -p xtask -- --color never
rg -n "recommended|blocked_for_now|not_recommended|money/round|function.arithmetic_leaf.monotone_up.v1|recommendation.latest.json|bounded second-language" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md CHANGELOG.md PLAN.md
! rg -n "repo-wide TypeScript support|broad TypeScript support|all families now support TypeScript|new promoted family|corpus run 1 spent by M33|spec-core capability expansion" semantic-families/README.md docs/recommendation_corpus_expansion_program_v0.1.md docs/semantic_family_capability_corpus_guide_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md CHANGELOG.md
INTEGRATION_BASE_SHA=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/integration-base.txt)
git diff --name-only "${INTEGRATION_BASE_SHA}...HEAD"
! git diff --name-only "${INTEGRATION_BASE_SHA}...HEAD" | rg -v '^(xtask/src/(lib|family/(recommend|promotion_artifacts|paths|mod|coverage))\.rs|semantic-families/README\.md|docs/(recommendation_corpus_expansion_program_v0\.1|semantic_family_capability_corpus_guide_v0\.1|ai_promotion_and_multilanguage_milestones_v0\.1)\.md|CHANGELOG\.md)$'
```

Rules:

- Record every actual command and exit code in `proof-log.json`.
- Do not substitute broader or different commands for the sequence above.
- If any command fails after `code-freeze.json` exists, the parent must emit and validate a real blocker artifact before stopping.
- M33 is not done if the floor passes but the diff escapes the closed implementation surface.

### WS-8 Publish and CI observation - parent only

#### `task/m33-07-push-observe`

Required parent actions:

1. Confirm the verified `ws/m33-int` commit can fast-forward `feat/corpus-expansion` without discarding unrelated work.
2. If and only if that fast-forward is safe, update the publish target to the exact verified integration SHA.
3. Push `feat/corpus-expansion`.
4. Record remote, branch, SHA, and timestamp in `push-record.json`.
5. Observe the CI run triggered by that exact pushed SHA.
6. Record workflow name, run id or URL, observed SHA, and workspace result in `ci-observation.json`.

Acceptance:

- Publish branch is the exact verified SHA from `ws/m33-int`.
- Push succeeded.
- CI ran on the exact pushed SHA.
- Workspace CI is green.

### WS-9 Closeout - parent only

#### `task/m33-08-closeout`

Closeout must write `closeout.md` and answer plainly:

1. Does the analysis artifact now answer the five M33 objective questions from `PLAN.md` without hidden chat context?
2. Does the current `money/round` helper-surface wedge read as `not_recommended` rather than as a vague held cluster?
3. Are missing evidence and stale evidence explicit in the analysis artifact?
4. Does `delta_from_previous` tell the maintainer what changed?
5. Does the family-scoped recommendation artifact carry the same analysis basis truth?
6. Does `promotion.execution.json` preserve the starting decision basis and bounded target-language truth?
7. If the run stopped after `code-freeze.json`, was `blocker.report.json` emitted and validated before stop?
8. Do the docs use the same vocabulary the artifacts emit?
9. Did the run avoid scope leak into corpus accounting, `spec-core` capability, new family promotion, or broad target-language claims?

Allowed closeout verdicts:

- `PROCEED`
  - M33 landed cleanly and the repo now has an honest maintainer-facing decision surface
- `NARROW`
  - M33 landed materially, but one bounded artifact or wording follow-on still blocks a clean next milestone
- `STOP`
  - The run widened scope, failed verification, or left the repo overclaiming capability

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
- it must embed the exact `PLAN.md`, `ORCH_PLAN.md`, and `analysis-freeze.json` excerpts the worker is allowed to rely on
- the worker does not reconstruct wedge wording from the repo independently when the launch file already freezes it

## Blocker Protocol

Workers must stop and return a blocker when:

- they need a file outside owned paths
- they need to widen implementation beyond the M33 closed surface
- they need to change the frozen schema vocabulary after `schema-freeze.json`
- they need to change the frozen wedge wording or interpretation after `analysis-freeze.json`
- they cannot satisfy acceptance commands with concrete evidence
- they discover overlapping external edits inside their owned surface after launch
- they discover a need to touch corpus-accounting policy, `spec-core`, family packets, or target-language proof semantics

Parent blocker response:

1. Write the sentinel terminal blocked state for the blocked task.
2. Write `blocked.json`.
3. If `code-freeze.json` exists, emit a real blocker artifact at:
   - `.semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`
4. Validate the blocker artifact with:
   - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/<run-id>/blocker.report.json`
5. Stop downstream launches, publish, and closeout.
6. Do not report partial green success.

Canonical blocked-path command template to freeze in `code-freeze.json`:

```bash
RUN_ID=$(cat /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m33_recommendation_quality_promotion_decisions/run-id.txt)
cargo xtask family emit-promotion-blocker function.arithmetic_leaf.monotone_up.v1 "$RUN_ID" \
  --target-language typescript \
  --blocking-step certify \
  --blocker-kind human-decision-required \
  --summary "$BLOCKER_SUMMARY" \
  --required-human-action "$REQUIRED_HUMAN_ACTION" \
  --safe-next-action "$SAFE_NEXT_ACTION_1" \
  --safe-next-action "$SAFE_NEXT_ACTION_2" \
  --evidence-command "$FAILING_COMMAND" \
  --evidence-exit-code "$FAILING_EXIT_CODE" \
  --evidence-note "$EVIDENCE_NOTE"
```

If a real artifact or diff path is the best evidence, include `--evidence-path "$EVIDENCE_PATH"` in the frozen invocation.

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
  - inferred milestone scope beyond M33
- If a seeded worktree copy of `PLAN.md` or `ORCH_PLAN.md` disagrees with the parent prompt or freeze records, the seeded copy is ignored.

## Freeze Checkpoints

### Checkpoint 0: M32 base freeze

Required:

- `m32-base-freeze.json` exists
- the chosen M33 seed proves inclusion of `6a1051b601487710d631031171cfde92810f1581` or an explicitly recorded direct descendant with the same closed M32 artifact truth

### Checkpoint 1: Baseline freeze

Required:

- `baseline.json` exists
- live branch is `feat/corpus-expansion`
- dirty overlap inside the M33-owned surface is either absent or explicitly blocked

### Checkpoint 2: Authority freeze

Required:

- `authority-freeze.json` exists
- `artifact-paths.json` exists
- `run-id.txt` exists
- `ws/m33-int` was created from the recorded seed SHA

### Checkpoint 3: Schema freeze

Required:

- `schema-freeze.json` exists
- frozen schema versions and vocabulary are recorded
- the docs lane is still disallowed at this checkpoint

### Checkpoint 4: Analysis freeze

Required:

- `analysis-freeze.json` exists
- the current wedge, artifact wording, and `money/round` interpretation are frozen
- if the docs lane is launched, it starts from the exact analysis-freeze SHA
- `docs-launch.md` exists and points at `analysis-freeze.json`

### Checkpoint 5: Code freeze

Required:

- `code-freeze.json` exists
- the sequential code lane acceptance commands pass on merged integration state

### Checkpoint 6: Docs merge

Required:

- if the docs lane was launched, its acceptance commands pass on merged integration state
- `merge-log.md` records the merge result or explicit skip

### Checkpoint 7: Green-path artifact emission

Required:

- analysis artifacts validate at the frozen paths
- the family-scoped monotone-up recommendation artifact validates at the frozen path
- `promotion.execution.json` validates at the frozen run path
- `green-path-record.json` exists

### Checkpoint 8: Final verification

Required:

- the exact merged-state verification floor passes
- `proof-log.json` records every command and exit code
- the final merged diff stays inside the M33 closed surface plus allowed mechanical spillover

## Tests And Acceptance

The required floor is locked:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family refresh-promotion-recommendation function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.arithmetic_leaf.monotone_up.v1/recommendation.latest.json
cargo test -p xtask family_refresh_promotion_recommendation -- --color never
cargo test -p xtask artifact_schema_ -- --color never
cargo test -p xtask recommendation_ -- --color never
cargo test -p xtask -- --color never
```

Additional acceptance rules:

- if the live analysis artifact cannot explain the `money/round` wedge without extra maintainer interpretation, M33 is incomplete
- if `decision_status`, `evidence_summary`, or `delta_from_previous` is absent from the primary analysis artifact, M33 is incomplete
- if family-scoped recommendation, execution, or blocker artifacts do not preserve the analysis basis fields, M33 is incomplete
- if downstream artifacts preserve basis truth but widen M32 into repo-wide target-language support, M33 is incomplete
- if the blocked path is needed after `code-freeze.json` and no validated `blocker.report.json` is emitted, the run is blocked and incomplete
- if the diff touches corpus manifest policy, `spec-core`, family packets, or prove/certify semantics, the run is blocked
- if docs diverge from the emitted artifact vocabulary or from `analysis-freeze.json`, M33 is incomplete

## Assumptions

- `feat/corpus-expansion` remains the publish target branch for this run.
- The chosen seed commit still preserves the closed M32 monotone-up proof artifacts at run start.
- `cargo xtask family validate-artifact` remains the stable artifact-truth validator during this run.
- The downstream real path for M33 remains the bounded monotone-up TypeScript pilot from M32.
- No new family promotion is required to make the M33 decision surface honest.

## Freeze And Restart Rules

- No lane launches before the parent writes `authority-freeze.json`.
- The optional docs lane may launch only after `analysis-freeze.json` exists and `docs-launch.md` has been written.
- If the chosen M32 base changes after `m32-base-freeze.json`, every downstream lane is stale and must be recreated from the new base.
- If `schema-freeze.json` changes before `analysis-freeze.json`, no docs lane may launch until a fresh analysis freeze is created on top of the new schema.
- If `analysis-freeze.json` changes after the docs lane is forked, the docs lane is stale and must be recreated from the new frozen SHA.
- If `code-freeze.json` changes any field name, command contract, artifact path, or current-wedge wording after the docs lane is forked, the docs lane is stale and must be recreated.
- If overlapping third-party edits land anywhere inside a lane-owned surface after launch, the parent records the overlap, invalidates the affected lanes, and relaunches from the newest relevant freeze.
- The parent does not hand-patch stale worker branches.
- Any request to widen M33 into corpus accounting redesign, `spec-core` capability expansion, new family promotion, or broad target-language claims blocks the run until `PLAN.md` is rewritten.

