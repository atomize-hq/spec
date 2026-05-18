# M64 Orchestration Plan

Status: **authoritative execution runbook**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Plan title: **`M64: Retire the False Same-Tree Nested Chain3 Regression Thesis, Preserve the Honest Cross-Library Candidate, and Refresh Truthful Family Analysis`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m60-plus`**  
Authority validated commit in `PLAN.md`: **`a761e28`**  
Base branch: **`main`**  
Authority date: **`2026-05-17`**  
Maximum safe worker concurrency: **2 parallel workers plus the parent integrator**  
Worker model assumption: **`GPT-5.4` with `reasoning_effort=high`**  
Rewrite intent: **replace the stale M63 runbook with an execution-ready M64 orchestration plan grounded only in `PLAN.md`**  
Last rewritten: **`2026-05-17`**

## Summary

- Execute from the current checked-out branch `feat/m60-plus` because that is
  the live workspace baseline and the branch named in `PLAN.md`.
- Keep the critical path local to the parent agent for baseline freeze,
  worktree setup, integration, the proof wall, analysis refresh, and final
  closeout.
- Use workers only for the two isolated authored lanes:
  - semantic-review proof in `spec-core/src/semantic_review.rs`
  - CLI truth-surface proof in `spec-cli/tests/cli.rs`
- Use dedicated worktrees under
  `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m64/{int,semantic-review,cli}`
  with branches `ws/m64-int`, `ws/m64-semantic-review`, and `ws/m64-cli`.
- Keep one durable orchestration source of truth:
  - run root: `.runs/m64_truth_reset_run1/`
  - task ledger: `.runs/m64_truth_reset_run1/tasks.json`
  - session log: `.runs/m64_truth_reset_run1/session-log.md`
  - per-task sentinels: `.runs/m64_truth_reset_run1/tasks/<TASK_ID>/`
- Historical orchestration docs are shape references only.
  - They are not authority for M64 facts, worktree layout, commands,
    acceptance, or stop conditions.
- Treat `.semantic-family-artifacts/family-promotion/analysis/*.json`,
  generated `.rs`, `.spec.passport.json`, and `.test.evidence.json` as derived
  proof surfaces, not authored source.

## Hard Guards

- `PLAN.md` is the only authority for M64 scope, order, and acceptance.
- Preserve the current dirty tree exactly as found at kickoff.
  - `PLAN.md` is already modified in the primary workspace and must not be
    reverted, reformatted, or rewritten by execution of this runbook.
- M64 is a truth-reset and read-side correction milestone.
  - no backend widening
  - no semantic-review widening beyond the smallest fix forced by failing proof
  - no new family key
  - no route-precedence reorder
  - no manifest or packet changes
  - no new committed `.unit.spec` or `.test.spec` fixtures
  - no recommendation-policy rewrite
- Core authored source scope is frozen to exactly these two surfaces:
  - `spec-core/src/semantic_review.rs`
  - `spec-cli/tests/cli.rs`
- Optional truth-maintenance scope is frozen to exactly one surface and only
  after proof plus analysis refresh:
  - `TODOS.md`
- Explicit no-touch authored surfaces:
  - `examples/crosslib-app/units/**`
  - `examples/shared-spec/units/**`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/**`
  - `semantic-families/**`
  - `xtask/src/family/**`
  - `spec-core/src/typescript_backend.rs`
  - `spec-core/src/validator.rs`
- Worker lanes must not author or refresh analysis artifacts.
  - the parent integrator is the only lane allowed to rerun
    `coverage/recommend/corpus-decision`
  - the parent integrator is the only lane allowed to touch `TODOS.md`
- No human approval gates exist in M64.
  - all gates are proof-based and must be satisfied from the merged repo state

Stop immediately and write a blocked summary if any of these occur:

1. `PLAN.md` changes materially after the baseline freeze is written.
2. Any worker needs to edit a file outside its frozen write scope.
3. `spec-core/src/semantic_review.rs` proof requires cross-library widening,
   a new family, or route-order changes to go green.
4. `spec-cli/tests/cli.rs` proves insufficient and another CLI or backend file
   would need edits for the public truth wall to stay honest.
5. Any attempt to prove the same-tree thesis depends on committed fixture work
   under `examples/**` or `spec-cli/tests/fixtures/**`.
6. The analysis wall cannot be refreshed cleanly after the proof wall is green.
7. A green outcome depends on asserting export `units[].semantic_review`
   instead of exported `passports[].semantic_review`.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m64
RUN_ROOT=$PRIMARY_ROOT/.runs/m64_truth_reset_run1
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + state | `PRIMARY_ROOT` | `feat/m60-plus` | Parent | durable run-state, authority docs, final fast-forward target |
| `WS-INT` | `$WT_ROOT/int` | `ws/m64-int` | Parent | integration branch, proof wall, analysis refresh, closeout |
| `WS-A` | `$WT_ROOT/semantic-review` | `ws/m64-semantic-review` | Worker | semantic-review proof in `spec-core/src/semantic_review.rs` |
| `WS-B` | `$WT_ROOT/cli` | `ws/m64-cli` | Worker | CLI truth-surface proof in `spec-cli/tests/cli.rs` |

### Worktree creation rules

- Do not create worker worktrees until `task/m64-00-baseline-freeze` is green.
- Create `WS-INT`, `WS-A`, and `WS-B` from `feat/m60-plus`.
- There is no separate docs worktree.
- There is no separate analysis-artifact worktree.
- Final formatting, proof, artifact validation, and optional `TODOS.md`
  truth-maintenance happen only in `WS-INT`.
- Record the dirty tree at kickoff and preserve it.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT" "$RUN_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b ws/m64-int feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/semantic-review" -b ws/m64-semantic-review feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/cli" -b ws/m64-cli feat/m60-plus
```

## Durable Orchestration State

All durable orchestration state lives under:

```bash
$RUN_ROOT
```

This directory is run-state only. It is not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, HEAD, dirty-tree snapshot, authority commit | Parent |
| `authority-freeze.json` | frozen M64 scope, truth contracts, file ownership, stop rules | Parent |
| `worktrees.json` | exact worktree paths, branches, heads, and lifecycle state | Parent |
| `file-ownership.json` | lane write scopes and global no-touch surfaces | Parent |
| `tasks.json` | canonical task ledger, dependencies, and states | Parent |
| `session-log.md` | chronological kickoff, launch, integration, proof, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `analysis/pre/coverage.latest.json` | pre-run coverage basis snapshot | Parent |
| `analysis/pre/recommendation.latest.json` | pre-run recommendation basis snapshot | Parent |
| `analysis/pre/corpus-program-decision.latest.json` | pre-run corpus-decision basis snapshot | Parent |
| `analysis/post/coverage.latest.json` | post-run coverage snapshot | Parent |
| `analysis/post/recommendation.latest.json` | post-run recommendation snapshot | Parent |
| `analysis/post/corpus-program-decision.latest.json` | post-run corpus-decision snapshot | Parent |
| `post-run-delta.md` | exact pre/post truth delta, blockers, candidate status, and next action | Parent |
| `blocked-summary.md` | exact blocked-state explanation if M64 cannot close cleanly | Parent |
| `validation/kickoff/` | kickoff captures and baseline commands | Parent |
| `validation/ws-a/` | semantic-review proof captures | Parent |
| `validation/ws-b/` | CLI truth-surface proof captures | Parent |
| `validation/final/` | final merged proof wall and artifact refresh captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |
| `tasks/<TASK_ID>/` | per-task sentinels and task-local notes | Parent creates, lane updates |

### Required `authority-freeze.json` contents

- `milestone`
- `authority_plan_path`
- `authority_plan_commit`
- `primary_branch`
- `frozen_scope_claim`
- `truth_contracts`
- `allowed_source_surfaces`
- `allowed_optional_surfaces`
- `global_no_touch_surfaces`
- `lane_ownership`
- `serialization_points`
- `integration_order`
- `worker_model`
- `worker_return_contract`
- `verification_commands`
- `closeout_matrix`
- `stop_rules`

### Required `truth_contracts` contents

- `same_tree_inner_supported_chain3`
  - test name:
    `same_tree_nested_chain3_inner_routes_to_supported_chain3`
  - required assertions:
    - `verdict == aligned`
    - `compatibility_key == function.wrapper.pipeline.chain3.v1`
    - `support_status == supported`
- `same_tree_outer_supported_chain3_under_specified`
  - test name:
    `same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified`
  - required assertions:
    - `verdict == under_specified`
    - `compatibility_key == function.wrapper.pipeline.chain3.v1`
    - `support_status == supported`
    - `reason_codes` contains `OutsideHonestSupportedSubset`
- `cli_same_tree_truth_surfaces`
  - test name:
    `nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth`
  - required assertions:
    - inner unit publishes supported `chain3`
    - outer unit publishes supported `chain3` with `under_specified`
    - neither unit publishes `unsupported_dep_topology`
    - export assertions read from `passports[]`
- `direct_crosslib_truth_surface`
  - test name:
    `direct_crosslib_nested_chain3_unsupported_truth_stays_pinned`
  - required assertions:
    - `status == valid`
    - `compatibility_key == unsupported.function.v1`
    - `support_status == unsupported`
    - `unsupported_reason_codes == ["unsupported_dep_topology"]`
    - export assertions read from the passport for
      `pricing/checkout_nested_chain3`
- `repo_root_copied_truth_surface`
  - strengthened existing test:
    `spec_status_repo_root_honors_each_root_workspace_config`
  - required assertions:
    - `crosslib-app` still has exactly `4` units
    - `pricing/checkout_nested_chain3` still appears
    - copied repo-root status remains `untested` for that unit
    - `SPEC_UNKNOWN_LIBRARY_NAMESPACE` remains absent

### Required `tasks.json` states

Allowed states:

- `queued`
- `ready`
- `running`
- `blocked`
- `submitted`
- `integrated`
- `closed`
- `skipped`

Only the parent may set `integrated`, `closed`, or `skipped`.
Workers may move only between `running`, `blocked`, and `submitted`.

Each task sentinel directory must contain:

- `status.json`
- `notes.md`
- `commands.log`
- `started-at.txt`
- `finished-at.txt`
- `handoff.md` for worker-owned tasks

## Workstream Plan

### WS-PARENT (`feat/m60-plus` then `ws/m64-int`) - parent agent only

#### 1. `task/m64-00-baseline-freeze`

- Record the live baseline before any worker launch.
- Snapshot:
  - current branch `feat/m60-plus`
  - `HEAD = a761e28`
  - dirty tree summary
  - current `PLAN.md` authority facts
  - current `ORCH_PLAN.md` replacement timestamp
  - pre-run analysis artifact paths and copies
- Write:
  - `baseline.json`
  - `authority-freeze.json`
  - `file-ownership.json`
  - initial `tasks.json`
  - initial `session-log.md`
- Copy the current analysis basis into `analysis/pre/`.

Acceptance:

- baseline captures the live dirty-tree state exactly
- all frozen truth contracts and file scopes match `PLAN.md`
- pre-run analysis snapshots exist before any authored edits begin

#### 2. `task/m64-01-worktree-setup`

- Create `WS-INT`, `WS-A`, and `WS-B`.
- Record their branch names, paths, and starting commits in `worktrees.json`.
- Create worker handoff packets under `handoffs/`.

Acceptance:

- all three worktrees exist at the expected paths
- both worker branches start from `feat/m60-plus`
- worker briefs contain only owned surfaces, relevant `PLAN.md` excerpts, and
  required commands

### Parallel workers after WS-PARENT freeze is green

#### 3. `task/m64-a-semantic-review-proof` on `ws/m64-semantic-review` - worker 1

Own only:

- `spec-core/src/semantic_review.rs`

Required work:

- add exact test
  `same_tree_nested_chain3_inner_routes_to_supported_chain3`
- add exact test
  `same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified`
- reuse the existing fixture style already present in the file
  - specifically the existing `chain3`-family semantic-review helpers instead
    of inventing a new one-off builder
- keep the diff test-only if both tests pass without production changes
- if proof fails, change only the smallest semantic-review surface needed to
  make shipped behavior match the already observed blocked-state truth
- do not reorder routes
- do not add a family
- do not widen cross-library support

Required commands:

- `cargo test -p spec-core same_tree_nested_chain3_inner_routes_to_supported_chain3 -- --exact`
- `cargo test -p spec-core same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified -- --exact`
- `cargo test -p spec-core semantic_review`

Acceptance:

- the inner same-tree shape proves supported `chain3` with `aligned`
- the outer same-tree shape proves supported `chain3` with `under_specified`
- `OutsideHonestSupportedSubset` is pinned on the outer test
- no authored file outside `spec-core/src/semantic_review.rs` changes

#### 4. `task/m64-b-cli-truth-proof` on `ws/m64-cli` - worker 2

Own only:

- `spec-cli/tests/cli.rs`

Required work:

- add exact test
  `nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth`
  - guard with `if !cargo_available() { return; }`
  - use existing helpers:
    - `temp_repo_dir`
    - `write_spec`
    - `run_in`
    - `parse_stdout_json`
    - `read_passport_json`
  - author the same-tree inner and outer units in a temp project inside the
    test
  - run:
    - `spec test units --output src/generated --crate-root .`
    - `spec status units --format json`
    - `spec export units`
  - assert:
    - inner publishes supported `chain3`
    - outer publishes supported `chain3` plus `under_specified`
    - neither unit publishes `unsupported_dep_topology`
    - export assertions read from `passports[]`, not `units[]`
- add exact test
  `direct_crosslib_nested_chain3_unsupported_truth_stays_pinned`
  - copy `examples/crosslib-app` and `examples/shared-spec` into a temp area
  - run direct-root commands against copied `examples/crosslib-app`
  - assert direct-root status and exported passport truth for
    `pricing/checkout_nested_chain3`
- strengthen existing test
  `spec_status_repo_root_honors_each_root_workspace_config`
  - keep it focused on workspace-config discovery and namespace hygiene
  - do not let it become the only proof of direct-root semantic truth
- do not touch:
  - `spec-cli/tests/fixtures/**`
  - `examples/**`
  - `spec-core/**`

Required commands:

- `cargo test -p spec-cli --test cli nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth -- --exact`
- `cargo test -p spec-cli --test cli direct_crosslib_nested_chain3_unsupported_truth_stays_pinned -- --exact`
- `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`
- `cargo test -p spec-cli --test cli`

Acceptance:

- same-tree CLI test proves honest supported truth through status plus passports
- direct cross-library CLI test pins the maintained unsupported candidate
- repo-root copied proof stays explicit about `4` units, `untested`, and absent
  `SPEC_UNKNOWN_LIBRARY_NAMESPACE`
- no authored file outside `spec-cli/tests/cli.rs` changes

### WS-INT (`ws/m64-int`) - parent agent only

#### 5. `task/m64-c-integrate-and-proof-wall`

- Merge `ws/m64-semantic-review` into `ws/m64-int`.
- Merge `ws/m64-cli` into `ws/m64-int`.
- Resolve only straightforward merge mechanics in integration-owned surfaces.
- If either worker diff tries to pull in scope beyond its frozen file set:
  - stop
  - bounce the lane back to its owner, or
  - apply the `PLAN.md` contract literally
- After merging, run:
  - `cargo fmt --all`
  - `cargo test -p spec-core same_tree_nested_chain3_inner_routes_to_supported_chain3 -- --exact`
  - `cargo test -p spec-core same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified -- --exact`
  - `cargo test -p spec-cli --test cli nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth -- --exact`
  - `cargo test -p spec-cli --test cli direct_crosslib_nested_chain3_unsupported_truth_stays_pinned -- --exact`
  - `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`
  - `cargo test -p spec-core semantic_review`
  - `cargo test -p spec-cli --test cli`
- If any proof stays red:
  - write `blocked-summary.md`
  - capture failing commands under `validation/final/`
  - stop

Acceptance:

- merged state contains only the two authorized authored source diffs
- exact selectors are green in the merged state
- broad `spec-core semantic_review` and `spec-cli --test cli` are green before
  analysis refresh begins

#### 6. `task/m64-d-analysis-refresh-and-closeout`

- After the proof wall is green, rerun:
  - `cargo xtask family coverage --format json`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- Copy refreshed artifacts into `analysis/post/`.
- Compare pre/post truth for:
  - `cluster_id = unsupported_dep_topology-fbecce0dbe98`
  - representative unit ids
  - `real_example_hits`
  - `promotion_relevant_regression_hits`
  - `recommendation_status`
  - `top_candidate_id`
  - `decision_action`
- Write `post-run-delta.md` with one of two allowed conclusions:
  - the same cross-library unsupported candidate remains the live honest wedge,
    or
  - analysis moved to a different truthful next wedge
- Update `TODOS.md` only if the refreshed analysis reveals a follow-up that
  would otherwise be lost.
  - do not carry forward the retired same-tree regression thesis
- Write `acceptance-ledger.md` and close the run.

Acceptance:

- all three analysis artifacts refresh and pass `validate-artifact`
- pre/post delta is explicit and references real artifact paths
- closeout states the truthful next action without reviving the retired
  same-tree thesis
- `TODOS.md` changes only if needed and only for the truthful next wedge

## Context-Control Rules

- Parent agent keeps only five live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `.runs/m64_truth_reset_run1/tasks.json`
  - the acceptance checklist
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- The parent agent reviews summaries plus narrow diffs only.
  - it does not ingest full worker transcripts into the main context
- Close each worker immediately after merge.
- Use sentinels or bounded waits, not tight polling.

## Tests And Acceptance

### Focused proof wall

- Step 1:
  - `cargo test -p spec-core same_tree_nested_chain3_inner_routes_to_supported_chain3 -- --exact`
  - `cargo test -p spec-core same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified -- --exact`
- Step 2A:
  - `cargo test -p spec-cli --test cli nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth -- --exact`
- Step 2B:
  - `cargo test -p spec-cli --test cli direct_crosslib_nested_chain3_unsupported_truth_stays_pinned -- --exact`
- Step 2C:
  - `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`

### Broad local proof after focused tests pass

- `cargo test -p spec-core semantic_review`
- `cargo test -p spec-cli --test cli`

### Analysis wall

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

### Acceptance checklist

- same-tree inner proof stays supported `chain3`
- same-tree outer proof stays supported `chain3` and `under_specified`
- no same-tree read-side surface publishes `unsupported_dep_topology`
- direct-root cross-library proof still publishes unsupported dep-topology truth
- repo-root copied proof remains separate and honest about `untested`
- all three analysis artifacts are refreshed from the merged green state
- closeout names the truthful next wedge without smuggling the retired thesis

## Failure Modes Registry

| Codepath | Real production-style failure | Test covers it? | Error handling exists? | Silent if missed? |
| --- | --- | --- | --- | --- |
| same-tree inner routing | reviewer silently regresses this shape back to unsupported | yes, Step 1 exact test | yes, semantic-review surfaces expose it | yes, analysis would lie again |
| same-tree outer routing | outer shape flips to aligned or unsupported instead of supported `under_specified` | yes, Step 1 + Step 2A | yes, public surfaces expose it | yes, public proof would claim the wrong family |
| direct cross-library example | maintained unsupported candidate silently flips to supported | yes, Step 2B | yes, status JSON and exported passports expose it | yes, the next milestone would chase the wrong wedge |
| repo-root copied proof | `untested` copied-root status gets mistaken for direct-root semantic authority | yes, Step 2C | no automatic guard outside tests | yes, future planning could drift again |
| export assertion surface | tests assert `units[]` instead of `passports[]` and report false truth | yes, Step 2A and 2B require passport reads | no | yes, the wrong read-side surface would pass |
| analysis refresh | proof passes but the checked-in artifacts stay stale or invalid | yes, validate-artifact commands | yes, validators fail | yes, the closeout would be based on old data |
| scope control | same-tree proof work leaks into fixtures, packets, or example units | guarded by frozen file ownership | no | yes, the milestone would claim a broader change than it made |

Critical gap rule:

- if any same-tree shape still publishes `unsupported_dep_topology` after the
  Step 2A test, stop the milestone
- if the cross-library maintained example stops publishing
  `unsupported_dep_topology` in Step 2B without an intentional reviewer change,
  stop the milestone
- if a green test requires asserting export `units[].semantic_review`, stop and
  re-scope because the proof surface is wrong

## Performance Review

No product runtime changes. This is proof and analysis work.

The only performance risk is avoidable local and CI drag.

Rules:

- land targeted exact tests first
- use exact selectors while iterating
- do not run the full CLI wall on every edit
- refresh the three analysis artifacts only after the semantic-review and CLI
  truth tests are green
- prefer temp-project proof and copied-example proof over broad committed-fixture
  churn

## NOT in scope

- making `examples_crosslib_app::pricing/checkout_nested_chain3` supported
  `chain3`
- widening semantic review to support cross-library nested callable-triple
  topology
- reviving the M63 same-tree regression idea as committed corpus or fixture
  pressure
- changing `spec-cli/tests/fixtures/m20/unsupported_truth_pack` into a mixed
  supported-and-unsupported pack
- backend, validator, or TypeScript execution changes
- manifest, family packet, or promotion-registry changes
- a new corpus-rerun story that still depends on the retired same-tree thesis

## Worktree Parallelization Strategy

This plan has two implementation lanes, then one integration lane.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| semantic-review proof | `spec-core/src/semantic_review.rs` | — |
| CLI truth-surface proof | `spec-cli/tests/cli.rs` | — |
| analysis refresh and closeout | `.semantic-family-artifacts/`, `TODOS.md` | semantic-review proof, CLI truth-surface proof |

### Parallel lanes

- `Lane A`: semantic-review proof in `spec-core/src/semantic_review.rs`
- `Lane B`: CLI truth-surface proof in `spec-cli/tests/cli.rs`
- `Lane C`: integration, proof wall, artifact refresh, delta capture, and
  optional `TODOS.md` truth maintenance after A + B merge

### Execution order

1. Parent freezes baseline and launches `Lane A` plus `Lane B` in parallel.
2. Parent integrates both lanes into `ws/m64-int`.
3. Parent reruns the merged proof wall.
4. Parent refreshes the analysis wall from the merged green state only.
5. Parent closes with a truthful next-action delta.

### Conflict flags

- `Lane A` and `Lane B` touch different module directories, so merge conflict
  risk is low
- `Lane B` itself must stay sequential inside one worker because the same file
  owns the temp same-tree proof, direct cross-library proof, and repo-root
  copied proof
- `Lane C` must stay sequential because analysis artifacts must be generated
  from merged final truth, not partial state

## Exit Criteria

M64 is successful only if all of the following are true:

1. `spec-core/src/semantic_review.rs` has focused exact tests proving the
   same-tree nested pair publishes supported `chain3` truth.
2. `spec-cli/tests/cli.rs` has a focused temp-project truth test proving the
   same-tree pair publishes supported `chain3` truth through CLI status and
   exported passports.
3. `spec-cli/tests/cli.rs` has a direct cross-library truth test proving
   `pricing/checkout_nested_chain3` remains `unsupported_dep_topology` in
   direct-root status and exported passports.
4. `spec_status_repo_root_honors_each_root_workspace_config` remains explicit
   about repo-root discovery and namespace hygiene without pretending to be the
   direct-root semantic authority.
5. The three family-analysis artifacts are rerun from the merged green state
   and pass validation.
6. The closeout records whether the same cross-library candidate remains live or
   whether a different truthful next wedge emerged.
7. No same-tree unsupported regression thesis survives in code, tests, docs, or
   artifacts.

## Completion Summary

- Step 0: Scope Challenge, resolved to a narrower and more truthful split
- Architecture Review: no new architecture, just a corrected truth wall and a
  clearer separation of proof surfaces
- Code Quality Review: minimal diff, no new committed fixtures, assert against
  exported passports instead of the wrong export unit surface
- Test Review: exact semantic-review proof, exact temp CLI proof, exact direct
  cross-library proof, explicit repo-root config proof, refreshed analysis wall
- Performance Review: targeted test execution only, no runtime impact
- NOT in scope: written
- Failure modes: explicit, with stop conditions
- Parallelization: 3 lanes total, 2 parallel then 1 sequential
- Lake Score: the complete option wins, because skipping the direct
  cross-library proof or the analysis refresh would save minutes and cost the
  next milestone its truth
