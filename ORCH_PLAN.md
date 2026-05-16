# M62 Orchestration Plan

Status: **authoritative execution runbook**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Plan title: **`M62: Bounded Corpus Run 1 for the Unsupported Callable-Triple Wrapper Dep-Topology Candidate`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m60-plus`**  
Authority validated commit in `PLAN.md`: **`0518c7a`**  
Base branch: **`main`**  
Authority date: **`2026-05-16`**  
Maximum safe worker concurrency: **2 parallel workers plus the parent integrator**  
Worker model assumption: **`GPT-5.4` with `reasoning_effort=high`**  
Rewrite intent: **replace the stale M61 repo-root orchestration doc with an execution-ready M62 runbook grounded only in `PLAN.md`**  
Last rewritten: **`2026-05-16`**

## Summary

M62 is a bounded corpus-and-analysis run.

It does not widen backend capability, semantic-review support, manifest source
inventory, or recommendation policy.

It adds the smallest exact authored corpus slice needed to move the
callable-triple wrapper dep-topology candidate around
`examples_crosslib_app::pricing/checkout_nested_chain3` off the current
`1 real / 0 regression` floor, then reruns the existing proof wall and accepts
the truthful next decision.

The parent agent owns the critical path and remains the only integrator. The
safe execution shape is fixed:

1. parent freezes the M62 contract from `PLAN.md`
2. parent captures the pre-run analysis basis and dirty-tree baseline
3. **Lane A** and **Lane B** run in parallel because they do not share files
4. parent integrates A + B into one post-authoring baseline
5. **Lane C** runs serially from the integrated baseline for CLI truth-surface
   assertions only
6. parent integrates Lane C
7. parent runs the final proof wall, analysis refresh, decision classification,
   and minimal docs truth-maintenance
8. parent writes the post-run delta and closes only if the result lands in an
   allowed post-run bucket

Historical files are shape references only:

- `docs/m26_orchestration_kickoff_prompt.md`
- the superseded M61 repo-root `ORCH_PLAN.md`

They are not authority for milestone facts, branch names, worktree paths,
acceptance logic, or stop conditions.

## Hard Guards

- `PLAN.md` is the only authority source for milestone scope and facts.
- M62 is not a backend-capability milestone.
- M62 is not a recommendation-policy rewrite milestone.
- M62 is not a semantic-family promotion milestone.
- Prohibited broadening:
  - no edits to `spec-core/src/semantic_review.rs`
  - no edits to `xtask/src/family/coverage.rs`
  - no edits to `xtask/src/family/recommend.rs`
  - no edits to `xtask/src/lib.rs`
  - no edits to `semantic-families/corpus/rust-function.toml`
  - no edits under `semantic-families/**`
  - no new corpus source bucket
  - no units under `examples/ecommerce/units`
  - no widening of TypeScript validator or backend rules
  - no molecule TypeScript execution work
  - no seam-kind TypeScript execution work
- The authored corpus slice is frozen up front:
  - exactly one new maintained real-example unit:
    - `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
  - exactly two new promotion-relevant regression units:
    - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
    - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`
- The only pre-authorized existing source file edit is:
  - `spec-cli/tests/cli.rs`
- Docs may change only after the proof wall and only if an existing surface
  would otherwise become misleading.
- Docs are limited to:
  - `examples/crosslib-app/README.md`
  - `CHANGELOG.md`
  - `TODOS.md`
- Derived artifact rule:
  - `.semantic-family-artifacts/family-promotion/analysis/*.json`
  - `examples/shared-crate/src/generated/**`
  - `examples/crosslib-app/src/generated/**`
  - co-located generated `.rs`
  - `.spec.passport.json`
  - `.test.evidence.json`
  are proof surfaces, not hand-authored source
- Preserve any pre-existing dirty tree exactly as found at kickoff.
- No lane may revert, reset, clean, stash, or overwrite unowned changes.
- No lane may improvise new files to "help" the analysis story beyond the three
  frozen authored additions.

Stop and re-scope immediately if any of these become true:

1. `PLAN.md` changes materially after the parent wrote the authority freeze.
2. Any worker needs to touch a file outside its frozen write scope.
3. The proof wall implies `xtask` or `spec-core` logic must change for M62 to
   land.
4. The target corpus run needs a sixth source, packet leverage, or manifest
   change to move the candidate.
5. `spec-cli/tests/cli.rs` is insufficient and another CLI or backend surface
   needs code edits to keep the public truth wall honest.
6. The final analysis basis still reports `1 real / 0 regression`.
7. The final analysis basis keeps missing-evidence blockers without an exact,
   unit-level explanation of what missed the target cluster.
8. `cargo xtask family corpus-decision --format json` fails on a refreshed valid
   basis.

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m62
RUN_ROOT=$PRIMARY_ROOT/.runs/m62_bounded_corpus_run1
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + state | `PRIMARY_ROOT` | `feat/m60-plus` | Parent | durable run-state, authority docs, final fast-forward target |
| `WS-INT` | `$WT_ROOT/int` | `ws/m62-int` | Parent | integration branch and final proof wall |
| `WS-A` | `$WT_ROOT/real-example` | `ws/m62-real-example` | Worker | maintained cross-library variant authoring |
| `WS-B` | `$WT_ROOT/regressions` | `ws/m62-regressions` | Worker | two M20 nested bad-topology regressions |
| `WS-C` | `$WT_ROOT/cli` | `ws/m62-cli` | Worker | CLI truth-surface assertion widening only |

### Worktree creation rules

- Do not create worker worktrees before `task/m62-00-baseline-freeze` completes.
- Create `WS-INT`, `WS-A`, and `WS-B` first.
- `WS-C` must branch from the integrated post-A+B state in `ws/m62-int`, not
  from the stale primary branch tip.
- There is no separate docs worktree.
- There is no separate proof-artifact worktree.
- Final proof, artifact refresh, decision classification, and minimal docs
  updates all happen in `WS-INT`.
- Record the dirty tree at kickoff and preserve it.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT" "$RUN_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/int" -b ws/m62-int feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/real-example" -b ws/m62-real-example feat/m60-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/regressions" -b ws/m62-regressions feat/m60-plus

# after WS-A and WS-B are integrated into ws/m62-int
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/cli" -b ws/m62-cli ws/m62-int
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
| `authority-freeze.json` | frozen M62 authored contract, file ownership, command wall, stop rules | Parent |
| `worktrees.json` | exact worktree paths, branches, heads, and states | Parent |
| `file-ownership.json` | lane write scopes and global no-touch surfaces | Parent |
| `tasks.json` | canonical task ledger, dependencies, and states | Parent |
| `session-log.md` | chronological launch, integration, rerun, block, and close log | Parent |
| `acceptance-ledger.md` | final gate checklist and proof references | Parent |
| `analysis/pre/coverage.latest.json` | pre-run coverage basis snapshot | Parent |
| `analysis/pre/recommendation.latest.json` | pre-run recommendation basis snapshot | Parent |
| `analysis/pre/corpus-program-decision.latest.json` | pre-run corpus-decision basis snapshot | Parent |
| `analysis/post/coverage.latest.json` | post-run coverage snapshot | Parent |
| `analysis/post/recommendation.latest.json` | post-run recommendation snapshot | Parent |
| `analysis/post/corpus-program-decision.latest.json` | post-run corpus-decision snapshot | Parent |
| `post-run-delta.md` | exact pre/post leverage, blockers, cluster members, and next action | Parent |
| `blocked-summary.md` | exact blocked-state explanation if M62 cannot close | Parent |
| `validation/kickoff/` | kickoff captures and baseline commands | Parent |
| `validation/ws-a/` | maintained-example proof captures | Parent |
| `validation/ws-b/` | M20 regression validate captures | Parent |
| `validation/ws-c/` | CLI assertion proof captures | Parent |
| `validation/final/` | final proof wall captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |
| `tasks/<TASK_ID>/` | per-task sentinels and task-local notes | Parent creates, lane updates |

### Required `baseline.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `historical_shape_refs`
- `observed_primary_surfaces`
- `pre_run_artifact_paths`
- `baseline_commands`
- `run_started_at`

### Required `authority-freeze.json` fields

- `milestone`
- `authority_plan_path`
- `authority_plan_commit`
- `primary_branch`
- `frozen_scope_claim`
- `frozen_authored_additions`
- `frozen_contracts`
- `frozen_cli_assertions`
- `allowed_doc_surfaces`
- `global_no_touch_surfaces`
- `lane_ownership`
- `serialization_points`
- `integration_order`
- `worker_model`
- `worker_return_contract`
- `verification_commands`
- `decision_matrix`
- `stop_rules`

### Required `frozen_contracts` contents

- `pricing/checkout_nested_chain3_variant`
  - exact path
  - exact five-input contract names
  - exact dep tuple
  - exact expected local-test output `Decimal::new(970290, 4)`
- `pricing/base_nested_chain3_bad_dep_topology`
  - exact path
  - exact five-input contract names
  - exact dep tuple
  - exact expected local-test output `Decimal::new(9801, 2)`
- `pricing/checkout_nested_chain3_bad_dep_topology`
  - exact path
  - exact five-input contract names
  - exact dep tuple
  - exact expected local-test output `Decimal::new(970290, 4)`

### Required `frozen_cli_assertions` contents

- test name:
  - `m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix`
- required new unsupported ids:
  - `pricing/base_nested_chain3_bad_dep_topology`
  - `pricing/checkout_nested_chain3_bad_dep_topology`
- required reason code for both:
  - `unsupported_dep_topology`
- test name:
  - `spec_status_repo_root_honors_each_root_workspace_config`
- required crosslib root expectation:
  - 5 units
  - explicit `pricing/checkout_nested_chain3_variant` row
  - status `untested`
  - no `SPEC_UNKNOWN_LIBRARY_NAMESPACE` noise

### Required `worktrees.json` fields

- `milestone`
- `updated_at`
- `primary_root`
- `worktree_root`
- `lanes[]`
  - `lane_id`
  - `path`
  - `branch`
  - `owner`
  - `state`
  - `head_commit`
  - `write_scope`
  - `task_ids`

### Required `tasks.json` fields

- `milestone`
- `updated_at`
- `tasks[]`
  - `task_id`
  - `lane`
  - `state`
  - `owner`
  - `depends_on`
  - `write_scope`
  - `command_wall`
  - `acceptance_summary`
  - `stop_rules`
  - `sentinel_dir`

## Task State And Sentinels

`tasks.json` is the single source of truth for task state.

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

Each task gets a dedicated sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Each sentinel directory should contain:

- `started-at.txt`
- `write-scope.txt`
- `commands.txt`
- `result-summary.md`
- `blocker.md` only when blocked

## Workstream Plan

### WS-BASELINE (`feat/m60-plus` + `ws/m62-int`) — parent agent only, sequential

1. `task/m62-00-baseline-freeze`
- Capture kickoff truth:
  - `git branch --show-current`
  - `git rev-parse --short HEAD`
  - `git status --short`
- Copy the current analysis basis into run-state:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- Write `baseline.json`, `authority-freeze.json`, `file-ownership.json`,
  `tasks.json`, and `worktrees.json`.
- Freeze exact write scopes:
  - `WS-A`: one maintained real-example spec file only
  - `WS-B`: two M20 regression spec files only
  - `WS-C`: `spec-cli/tests/cli.rs` only
  - `WS-INT`: artifact refresh, integration mechanics, and minimal docs
- Create `WS-INT`, `WS-A`, and `WS-B`.
- Record the current dirty tree as preserved baseline, not as lane work.

Acceptance for `task/m62-00-baseline-freeze`:
- every frozen authored addition from `PLAN.md` is present in
  `authority-freeze.json`
- every global no-touch surface from `PLAN.md` is recorded
- pre-run analysis artifacts are snapshotted before any worker edits begin
- the lane write scopes make the A/B parallel split mechanically unambiguous

### Parallel workers after WS-BASELINE is green

2. `task/m62-a-real-example` on `ws/m62-real-example` — worker 1
- Own only:
  - `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
- Author the maintained cross-library variant using the frozen contract
  literally:
  - exact id
  - same five-input `Decimal` contract names
  - exact dep tuple:
    - `shared::pricing/base_nested_chain3`
    - `shared::pricing/apply_tax`
    - `shared::pricing/apply_discount`
  - both `body.rust` and `body.typescript`
  - exact expected local-test output `Decimal::new(970290, 4)`
- Do not edit:
  - `examples/shared-spec/**`
  - `examples/crosslib-app/README.md`
  - any CLI or backend file
- Verify with:
  - `cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json`
  - `cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated`
  - `cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript`

Acceptance for `task/m62-a-real-example`:
- the new maintained unit validates
- the new maintained unit proves in both Rust and TypeScript
- no shared-spec source unit was touched
- the authored body still expresses the target wrapper-like callable-triple
  pressure, not a neighboring supported shape

3. `task/m62-b-regressions` on `ws/m62-regressions` — worker 2
- Own only:
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`
- Author both units using the frozen contract literally:
  - exact ids
  - exact five-input contract names
  - exact dep tuples
  - exact expected local-test outputs
- Keep both units inside the existing M20 pack vocabulary and field order.
- Do not edit:
  - `spec-cli/tests/cli.rs`
  - any `xtask` file
  - any maintained example file
- Verify with:
  - `cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json`

Acceptance for `task/m62-b-regressions`:
- both new M20 regression units validate inside the existing pack
- the units remain in the unsupported callable-triple wrapper neighborhood
- the lane did not widen the pack beyond the two frozen additions

### WS-INT (`ws/m62-int`) — parent agent only, post-A+B integration

4. `task/m62-int-ab`
- Merge `ws/m62-real-example` and `ws/m62-regressions` into `ws/m62-int`.
- Resolve only straightforward merge mechanics.
- If a lane changed an unowned file, reject the lane and bounce it back.
- After merge, verify the integrated authored delta is still exactly:
  - one new maintained real-example file
  - two new M20 regression files
- Create `WS-C` from the integrated `ws/m62-int` tip only after this check
  passes.

Acceptance for `task/m62-int-ab`:
- the integrated tree contains exactly the three frozen authored additions
- no forbidden surface changed
- `ws/m62-cli` forks from the integrated A+B baseline, not from stale primary

### WS-C (`ws/m62-cli`) — worker 3, serialized after A+B integration

5. `task/m62-c-cli`
- Own only:
  - `spec-cli/tests/cli.rs`
- Update only the frozen public truth surfaces:
  - extend
    `m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix`
    with:
    - `pricing/base_nested_chain3_bad_dep_topology`
    - `pricing/checkout_nested_chain3_bad_dep_topology`
    - reason `unsupported_dep_topology` for both
  - update
    `spec_status_repo_root_honors_each_root_workspace_config`
    so the copied `crosslib-app` root expects:
    - 5 units
    - explicit `pricing/checkout_nested_chain3_variant`
    - status `untested`
    - no `SPEC_UNKNOWN_LIBRARY_NAMESPACE` noise
- Do not edit:
  - fixture specs
  - backend code
  - docs
  - analysis artifacts
- Verify with:
  - `cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact`
  - `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`
  - `cargo test -p spec-cli --test cli`

Acceptance for `task/m62-c-cli`:
- `spec-cli/tests/cli.rs` explicitly names the two new M20 ids and the new
  maintained crosslib row
- both targeted exact tests pass
- the broader `spec-cli` CLI truth-surface file stays green

### WS-FINAL (`ws/m62-int`) — parent agent only

6. `task/m62-d-proof-closeout`
- Merge `ws/m62-cli` into `ws/m62-int`.
- Run the full proof wall in this order:

```bash
cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript

cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json
cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact
cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact
cargo test -p spec-cli --test cli

cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq '.unsupported_clusters[] | select(.cluster_id=="unsupported_dep_topology-fbecce0dbe98") | {cluster_id, representative_unit_ids, source_ids, real_example_hits, promotion_relevant_regression_hits, boundary_only_hits}' .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
jq '{recommendation_status, decision_summary, top_candidate:(.ranked_candidates[0] | {candidate_id, promotion_readiness, hold_reasons, confidence, leverage})}' .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq '{decision_action, decision_basis_code, required_next_action, summary}' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

- Copy refreshed analysis artifacts into `analysis/post/`.
- Write `post-run-delta.md` with:
  - pre-run and post-run leverage counts
  - pre-run and post-run blocker lists
  - pre-run and post-run corpus-decision action
  - post-run cluster member ids
  - exact next-action sentence
  - final bucket classification
- Apply docs edits only if the refreshed proof wall makes a current surface
  misleading:
  - `examples/crosslib-app/README.md`
  - `CHANGELOG.md`
  - `TODOS.md`
- If docs do not need truth maintenance, leave them untouched.

Blocked-state behavior for `task/m62-d-proof-closeout`:

- If the maintained example fails Rust or TypeScript proof:
  - stop
  - record failure in `blocked-summary.md`
  - reopen `task/m62-a-real-example`
- If the M20 pack validate step fails:
  - stop
  - record failure in `blocked-summary.md`
  - reopen `task/m62-b-regressions`
- If the targeted or broad CLI truth-surface tests fail:
  - stop
  - record failure in `blocked-summary.md`
  - reopen `task/m62-c-cli`
- If coverage does not show the target cluster at `2 real / 2 regression`:
  - stop
  - record which new unit appears to have missed the target cluster
  - do not close M62
- If refreshed recommendation still keeps missing-evidence blockers:
  - stop
  - record the blocker list and the suspected missed unit or route
  - do not close M62
- If refreshed `corpus-decision` still returns `spend_corpus_run1`:
  - stop
  - record the exact decision payload
  - do not close M62
- If refreshed `corpus-decision` returns `stop`:
  - stop
  - close only if the post-run evidence proves the candidate genuinely fell out
    of the actionable set
  - otherwise treat as red
- If the refreshed basis implies `xtask` or backend code changes are required:
  - stop
  - do not patch around it under M62
  - escalate as a re-scope

Acceptance for `task/m62-d-proof-closeout`:
- the maintained variant proves in both Rust and TypeScript
- the M20 pack stays green
- `spec-cli/tests/cli.rs` proves the new maintained row and both new M20 ids
- all three refreshed analysis artifacts validate
- the target candidate is no longer judged from a `1 real / 0 regression`
  floor
- the final result lands in one of the allowed post-run buckets from `PLAN.md`

7. `task/m62-e-close`
- Close M62 only after the parent classifies the outcome into exactly one
  bucket:

1. **Expected green path**
   - coverage shows `2 real / 2 regression`
   - recommendation becomes `ranked` plus `recommended`
   - corpus decision becomes `pivot_to_family_promotion_run`
   - outcome: M62 succeeded and the next plan, if needed, is a bounded family
     promotion plan

2. **Yellow but acceptable diagnosis**
   - coverage shows `2 real / 2 regression`
   - missing-evidence blockers clear
   - corpus decision pivots to `recommendation_policy_run`
   - outcome: M62 still succeeded as a corpus run, but the follow-up is policy,
     not more corpus spend

3. **Red, do not close**
   - coverage fails to reach `2 real / 2 regression`
   - or the recommendation still reports missing evidence
   - or `decision_action` stays `spend_corpus_run1`

4. **Unexpected stop path**
   - `decision_action = "stop"`
   - close only if the run proves why the candidate is no longer actionable

- Update `acceptance-ledger.md` with the exact bucket and proof references.
- Close worker tasks immediately after merge. Do not keep completed workers
  alive.

Acceptance for `task/m62-e-close`:
- closeout cites exact proof paths, not impressions
- the next action is explicit and consistent with the observed bucket
- no hidden follow-up work is smuggled into M62 after the bucket is known

## Context-Control Rules

- Parent agent keeps only these live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `$RUN_ROOT/tasks.json`
  - `authority-freeze.json`
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
  - the worker return contract
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers must not dump full artifact JSON back into the parent context.
- Workers do not write `analysis/pre/*`, `analysis/post/*`, or acceptance
  ledgers.
- The parent reviews narrow diffs and proof summaries only.
- Close each worker immediately after merge.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Maintained real-example lane
  - `cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json`
  - `cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated`
  - `cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript`
- M20 regression lane
  - `cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json`
- CLI truth-surface lane
  - `cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact`
  - `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`
  - `cargo test -p spec-cli --test cli`
- Analysis wall
  - `cargo xtask family coverage --format json`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- Inspection wall
  - `jq '.unsupported_clusters[] | select(.cluster_id=="unsupported_dep_topology-fbecce0dbe98") | {cluster_id, representative_unit_ids, source_ids, real_example_hits, promotion_relevant_regression_hits, boundary_only_hits}' .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `jq '{recommendation_status, decision_summary, top_candidate:(.ranked_candidates[0] | {candidate_id, promotion_readiness, hold_reasons, confidence, leverage})}' .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `jq '{decision_action, decision_basis_code, required_next_action, summary}' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Final acceptance requires all of the following:

1. exactly three new authored source-spec files landed
2. no sixth corpus source or backend-capability widening was introduced
3. `spec-cli/tests/cli.rs` explicitly asserts the new maintained row and the
   two new unsupported regression ids
4. the target candidate is no longer judged from a `1 real / 0 regression`
   floor
5. `coverage.latest.json` reaches `2 real / 2 regression`
6. missing-evidence blockers are gone
7. `corpus-program-decision.latest.json` is no longer `spend_corpus_run1`
8. the final result is classified into an allowed post-run bucket
9. docs remain truthful and do not overclaim M62 as a product-capability
   milestone
10. `post-run-delta.md` preserves the exact leverage, blocker, and next-action
    delta

## Assumptions

- `feat/m60-plus` remains the live primary branch for M62 execution.
- The validated commit in `PLAN.md` remains `0518c7a` until the run begins.
- The current analysis artifact paths already exist and remain the canonical
  read-side outputs for this milestone.
- Existing `cargo xtask family validate-artifact` support is sufficient for the
  three refreshed analysis artifacts.
- The repo's existing `spec-cli` and `xtask` commands remain the proof owners;
  no browser, UI, or external-service loop is part of M62.
- The only intended concurrency is the two-lane authored corpus split. Adding
  more workers would increase coordination cost without shortening the critical
  path.
