# M57 Shared-Core Portability Adoption Closeout Orchestration Runbook

Status: **authoritative execution runbook**  
Supersedes: **the stale M56 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M57: Shared-Core Portability Adoption Closeout Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Primary execution head at rewrite: **`1c841f2`**  
Authority validated commit in `PLAN.md`: **`504b1e3`**  
Base branch: **`main`**  
Authority date: **`2026-05-13`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum safe worker concurrency: **2 total, but default operating mode is sequential with only 1 active worker lane until proof repair is integrated**  
Rewrite intent: **replace the stale M56 TypeScript-oriented runbook with an execution-ready M57 closeout runbook aligned to the current `PLAN.md` truth**  
Last rewritten: **`2026-05-13`**

## Summary

This runbook turns the current M57 `PLAN.md` into a bounded parent/worker execution system.

M57 is a small closeout milestone. It is not a new architecture push. It does not authorize new shared-core extraction, new crates, new commands, schema churn, artifact-path churn, TypeScript work, consumer rewires, or broader semantic-review policy work.

The minimum honest completion slice is fixed:

1. keep repo-root authority aligned to the current M57 closeout contract
2. verify `xtask/src/family/mod.rs` and the two compatibility shims still tell one ownership story
3. repair the stale locked proof expectation in `xtask/src/lib.rs`
4. rerun the frozen stop-state proof floor
5. sync wording or docs only if the audit proves current wording false

The parent remains the only integrator. Workers never merge each other. Most of the run is sequential because the milestone is small and the real critical path is the stale `xtask/src/lib.rs` proof wall.

## Hard Guards

- `PLAN.md` is the only scope authority.
- M57 remains a closeout/truth-sync milestone around an already-frozen owner seam.
- `xtask/src/family/analysis_core/*` remains the only semantic owner surface.
- `xtask/src/family/helper_surface.rs` and `xtask/src/family/decision_kernel.rs` remain compatibility-only passthrough shims.
- `xtask/src/family/recommend.rs`, `xtask/src/family/verify.rs`, and `xtask/src/family/promotion_artifacts.rs` are read-only proof surfaces for M57.
- The concrete code write target is primarily `xtask/src/lib.rs`.
- Optional wording or docs edits are allowed only if the audit proves a current falsehood.
- Preserve stop-state truth exactly:
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_status = "not_recommended"`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo xtask family verify-decision-contract --format json` must remain `pass`.
- The known stale failing test is:
  - `tests::recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held`
- The stale expected source unit counts are:
  - old locked expectation: `[6, 12, 9, 1, 2]`
  - current locked truth from `PLAN.md`: `[6, 12, 9, 3, 3]`
- Do not “fix” the proof wall by weakening assertions or switching to dynamic filesystem-derived expectations.
- Do not add new commands, new schema fields, new artifact paths, or new proof surfaces.
- Do not let docs move ahead of code truth and proof truth.
- Parent remains sole integrator onto `feat/m40-plus`.

Stop and re-scope immediately if any of these become true:

1. greening `cargo test -p xtask` requires semantic edits in `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`
2. the only way to green the suite is to loosen or delete the locked assertion instead of updating it to current locked truth
3. owner-surface truth now requires a new abstraction layer or helper module
4. wording sync would require new product claims rather than correction of false wording
5. proof parity would require CLI flag changes, command output changes, or artifact JSON changes
6. unrelated `xtask` failures appear and are not caused by the M57 closeout blast radius

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m57
RUN_ROOT=$PRIMARY_ROOT/.runs/m57_shared_core_portability_adoption_closeout
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, audit, integration, final proof wall |
| `WS-A-PROOF` | `$WT_ROOT/ws-a-proof` | `codex/m57-xtask-proof-closeout` | Worker | stale proof-wall repair in `xtask/src/lib.rs` |
| `WS-B-SYNC` | `$WT_ROOT/ws-b-sync` | `codex/m57-ownership-doc-sync` | Worker, optional | narrow wording/doc sync only if audit proves a current falsehood |

### Worktree creation rules

- Do not create any worker worktree before `M57-02` audit freeze completes.
- Create `WS-A-PROOF` immediately after `M57-02`.
- Create `WS-B-SYNC` only if the audit or post-proof diff proves a specific wording or doc falsehood.
- Do not spin up `WS-B-SYNC` speculatively.
- Default mode is sequential. Only run two workers in parallel if:
  - `WS-A-PROOF` has a clearly isolated `xtask/src/lib.rs` task
  - the parent audit has already proven a separate narrow falsehood for `WS-B-SYNC`
- If the primary tree becomes dirty later, record it in run-state. Do not stash or clean by default.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-a-proof" -b codex/m57-xtask-proof-closeout feat/m40-plus

# create only if the parent audit proves a current wording or doc falsehood
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-b-sync" -b codex/m57-ownership-doc-sync feat/m40-plus
```

## Durable Orchestration State

All durable session state lives under:

```bash
$RUN_ROOT
```

This directory is orchestration state, not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, head, tree state, baseline proof truth | Parent |
| `contract-freeze.json` | frozen M57 scope, stop rules, command wall, lane policy | Parent |
| `worktrees.json` | exact worktree paths, branches, and lane states | Parent |
| `file-ownership.json` | exact owned file map per lane | Parent |
| `tasks.json` | canonical task definitions, dependencies, and live states | Parent |
| `queue.json` | optional parent-authored scheduling projection derived from `tasks.json` | Parent |
| `session-log.md` | chronological launch, submission, integration, and stop log | Parent |
| `acceptance-ledger.md` | final signoff checklist and artifact references | Parent |
| `final-proof-manifest.json` | exact final proof commands, exit codes, and artifact paths | Parent |
| `final-diff-summary.md` | parent-authored landed diff summary | Parent |
| `validation/kickoff/` | branch, head, status, authority snapshots | Parent |
| `validation/baseline/` | pre-change proof captures, including the known red proof wall | Parent |
| `validation/audit/` | owner-surface and wording audit captures | Parent |
| `validation/proof/` | targeted proof-repair captures | Parent |
| `validation/final/` | final serial proof-wall captures | Parent |
| `handoffs/` | worker briefs and worker return packets | Parent |

### Required `baseline.json` contents

`baseline.json` must include at least:

- `milestone`: `M57`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_validated_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `baseline_commands`
- `baseline_expected_truth`
- `known_red_test`
- `stale_expected_source_unit_counts`
- `current_locked_source_unit_counts`
- `stop_rules_version`

### Required `contract-freeze.json` contents

`contract-freeze.json` must include at least:

- `milestone`: `M57`
- `authority_plan_path`
- `authority_plan_head_commit`
- `frozen_at_primary_commit`
- `primary_branch`
- `exact_scope_claim`
- `locked_decisions`
- `preserved_stop_state_truth`
- `known_red_test`
- `allowed_worker_lanes`
- `optional_lane_triggers`
- `phase_commands`
- `integration_order`
- `merge_conflict_policy`
- `worker_return_contract`
- `exact_stop_rules`

### Queue state machine

`tasks.json` is the canonical task-state source of truth. `queue.json`, if present, is a derived scheduling projection only and must never become an independent task ledger.

Every task in `tasks.json` uses only these states:

- `queued`
- `ready`
- `running`
- `blocked`
- `submitted`
- `integrated`
- `closed`

A worker may move a task only to `submitted` or `blocked`. Only the parent may mark `integrated` or `closed`.

## Per-Task Sentinel Convention

Each task gets a dedicated sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Required files:

- `status.json`
- `owner.txt`
- `branch.txt`
- `write_scope.txt`
- `commands.txt`
- `changed_files.txt`
- `acceptance.md`
- `blocker.md`

Sentinel rules:

- The parent creates every sentinel before work starts.
- Worker output is incomplete until `commands.txt`, `changed_files.txt`, and `acceptance.md` are populated.
- Chat history is not the durable ledger.
- A task is not done when a worker says “done.” It is done only after parent integration and gate rerun.

## Context-Control Rules

- The parent owns `PLAN.md`, `ORCH_PLAN.md`, and all files under `$RUN_ROOT/`.
- Workers get only the minimum prompt necessary: goal, scope, owned files, stop rules, acceptance, and exact commands.
- Do not forward one worker’s raw transcript into another worker.
- No worker may expand its write scope mid-flight.
- `xtask/src/lib.rs` is single-owner in `WS-A-PROOF`.
- `xtask/src/family/mod.rs`, `xtask/src/family/helper_surface.rs`, and `xtask/src/family/decision_kernel.rs` stay parent-owned unless the audit proves they are false and the parent explicitly reassigns them.
- Docs stay parent-owned unless the parent explicitly assigns an exact subset to `WS-B-SYNC`.
- `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` are read-only context surfaces only.
- When validation feedback matters, store command output under `validation/*`; do not rely on chat paraphrases.
- The parent integrates one lane at a time and reruns relevant gates after each merge.

## File Ownership Map

### Parent-owned throughout

- `PLAN.md`
- `ORCH_PLAN.md`
- all files under `$RUN_ROOT/`
- `xtask/src/family/mod.rs` unless reassigned after audit
- `xtask/src/family/helper_surface.rs` unless reassigned after audit
- `xtask/src/family/decision_kernel.rs` unless reassigned after audit
- `xtask/src/family/recommend.rs` read-only
- `xtask/src/family/verify.rs` read-only
- `xtask/src/family/promotion_artifacts.rs` read-only
- final integration commits on `feat/m40-plus`

### `WS-A-PROOF` owned files

- `xtask/src/lib.rs`

### `WS-B-SYNC` owned files, only if explicitly assigned after audit

Potential files:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Rules for `WS-B-SYNC` scope:

- The parent assigns only the exact subset proven false.
- `WS-B-SYNC` must not touch `xtask/src/lib.rs`.
- `WS-B-SYNC` must not touch `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`.
- If the audit shows current wording is already truthful, `WS-B-SYNC` is skipped entirely.

## Worker Return Contract

Each worker returns only:

- changed files
- commands run with exit codes
- blockers or unresolved assumptions

Each return must be written into `$RUN_ROOT/handoffs/<TASK_ID>.md` and mirrored into the task sentinel files.

Parent review rules:

- review narrow diffs only, scoped to the ownership map
- review command outcomes and blockers, not raw worker transcripts
- integrate one worker lane at a time
- rerun relevant gates after each integration
- close the lane after integration or rejection; do not keep workers open indefinitely
- if merge feedback requires edits outside the lane scope, bounce the lane back or update ownership explicitly first

## Workstream Plan

| ID | Task | Owner | Write scope | Depends on | Unlock condition | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- |
| `M57-00` | Kickoff + baseline capture | Parent | `$RUN_ROOT/**` | none | repo available on `feat/m40-plus` | authority snapshots and baseline proof captures stored |
| `M57-01` | Contract freeze + ownership map | Parent | `$RUN_ROOT/**` | `M57-00` | baseline recorded | frozen contract, queue, sentinels, ownership map written |
| `M57-02` | Parent authority + owner-surface audit | Parent | read-only audit plus `$RUN_ROOT/**` | `M57-01` | contract frozen | audit result recorded, optional lane decision made |
| `M57-10` | Lane A stale proof-wall repair | `WS-A-PROOF` | `xtask/src/lib.rs` | `M57-02` | audit complete | worker submits narrow proof repair or explicit blocker |
| `M57-11` | Parent integration gate for Lane A | Parent | integration on `feat/m40-plus` only | `M57-10` | worker submitted | proof repair integrated, targeted and full proof reruns green or blocked with evidence |
| `M57-20` | Optional Lane B wording/doc sync | `WS-B-SYNC` | exact files assigned after audit | `M57-11` by default, or `M57-02` only if parent explicitly allows parallel launch | audit proved a current falsehood | worker submits exact narrow sync or is skipped |
| `M57-21` | Parent optional sync integration gate | Parent | integration on `feat/m40-plus` only | `M57-20` if launched | worker submitted | optional wording/doc diff integrated or lane closed as skipped |
| `M57-30` | Final serial proof wall + closeout | Parent | `$RUN_ROOT/**` and minimal fix-forward only if required | `M57-11` and `M57-21` if launched | all required work integrated | final commands pass, manifests written, closeout recorded |

## Task Execution Details

### `M57-00` Kickoff + baseline capture

Owner: Parent  
Write scope: `$RUN_ROOT/**`

Required captures:

```bash
mkdir -p "$RUN_ROOT"/{validation/{kickoff,baseline,audit,proof,final},tasks,handoffs}

git -C "$PRIMARY_ROOT" branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git -C "$PRIMARY_ROOT" rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git -C "$PRIMARY_ROOT" status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Required baseline proof captures:

```bash
cargo test -p xtask recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/targeted-red-test.txt"

cargo test -p xtask family::analysis_core::helper_surface::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/helper-surface-tests.txt"

cargo test -p xtask family::analysis_core::decision_contract::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/decision-contract-tests.txt"

cargo test -p xtask family::analysis_core::proof_fingerprint::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/proof-fingerprint-tests.txt"

./.agents/skills/next-milestone/scripts/collect_signals.sh \
  | tee "$RUN_ROOT/validation/baseline/collect-signals.txt"

cargo xtask family recommend --format json \
  | tee "$RUN_ROOT/validation/baseline/recommend.json"

cargo xtask family corpus-decision --format json \
  | tee "$RUN_ROOT/validation/baseline/corpus-decision.json"

cargo xtask family verify-decision-contract --format json \
  | tee "$RUN_ROOT/validation/baseline/verify-decision-contract.json"

cargo test -p xtask \
  | tee "$RUN_ROOT/validation/baseline/full-xtask-suite.txt"
```

Baseline truth to record:

- current branch is `feat/m40-plus`
- stop-state commands already report `stop` / `no_actionable_candidate` / `record_stop_without_new_milestone`
- `verify-decision-contract` already passes
- the known red wall is the single stale locked recommendation coverage assertion in `xtask/src/lib.rs`
- expected stale counts are `[6, 12, 9, 1, 2]`
- current locked truth is `[6, 12, 9, 3, 3]`

Stop rule:

- If baseline reveals additional failures outside the M57 blast radius, stop and classify before launching a worker.
- If stop-state command truth already drifted away from the frozen `PLAN.md` contract, stop and re-plan.

### `M57-01` Contract freeze + ownership map

Owner: Parent  
Write scope: `$RUN_ROOT/**`

Required artifacts:

- `baseline.json`
- `contract-freeze.json`
- `worktrees.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`
- `session-log.md`

`contract-freeze.json` must freeze:

- current authority commit references
- current primary commit
- exact M57 scope claim
- locked stop-state truth
- the known red test name
- the current locked count truth `[6, 12, 9, 3, 3]`
- allowed worker lanes:
  - `WS-A-PROOF`
  - `WS-B-SYNC`
- optional lane trigger rules
- exact proof-wall command list
- integration order:
  - parent audit
  - proof lane
  - optional sync lane only if needed
  - final closeout
- merge conflict policy
- worker return contract
- stop rules

Exit criteria:

- all artifacts above exist
- all tasks are queued with explicit dependencies
- ownership is exact and finite
- worker prompts can be generated without rereading the full repo

### `M57-02` Parent authority + owner-surface audit

Owner: Parent  
Write scope: read-only audit plus `$RUN_ROOT/**`

Audit surfaces:

- `PLAN.md`
- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Audit goals:

1. confirm `PLAN.md` is already truthful M57 authority and needs no further scope rewrite
2. confirm `analysis_core/*` is still presented as the only semantic owner surface
3. confirm both shims are still pure passthroughs with no semantic logic
4. confirm current docs still say the same ownership truth
5. decide whether `WS-B-SYNC` is unnecessary, code-wording-only, docs-only, or mixed

Required output:

- `validation/audit/audit-findings.md`
- `validation/audit/owned-falsehoods.json`

`owned-falsehoods.json` must enumerate one of:

- `no_falsehoods_found`
- exact code files requiring wording-only correction
- exact docs files requiring wording-only correction

Exit criteria:

- the parent has a binary launch decision for `WS-B-SYNC`
- any potential sync scope is exact and narrow
- if the audit implies semantic changes rather than wording corrections, stop and re-scope

### `M57-10` Lane A stale proof-wall repair

Owner: `WS-A-PROOF`  
Write scope: `xtask/src/lib.rs`

Responsibilities:

- repair only the stale locked proof expectation in `xtask/src/lib.rs`
- keep the assertion explicit and locked
- preserve expected source ids:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- update source unit counts to current locked truth if confirmed:
  - `[6, 12, 9, 3, 3]`
- preserve the existing proof value of the test
- do not add dynamic discovery logic
- do not broaden the blast radius into nearby unrelated cleanup

Required proof surface for worker submission:

```bash
cargo test -p xtask recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held -- --nocapture
cargo test -p xtask
```

Exit criteria:

- worker submits a narrow diff limited to `xtask/src/lib.rs`
- worker captures both commands with exit codes
- worker records any remaining failure as an explicit blocker, not an assumption

Stop if:

- the repair requires files outside `xtask/src/lib.rs`
- the repair requires weaker assertions
- the repair implies drift in stop-state command truth
- the repair exposes unrelated suite failures not attributable to the stale count wall

### `M57-11` Parent integration gate for Lane A

Owner: Parent  
Write scope: integration on `feat/m40-plus` only

Integration order is fixed:

1. review `WS-A-PROOF`
2. integrate onto `feat/m40-plus`
3. rerun targeted proof
4. rerun full frozen proof floor
5. decide whether optional sync is still needed

Required post-integration proof surface:

```bash
cargo test -p xtask recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-targeted-red-test.txt"

cargo test -p xtask family::analysis_core::helper_surface::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-helper-surface-tests.txt"

cargo test -p xtask family::analysis_core::decision_contract::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-decision-contract-tests.txt"

cargo test -p xtask family::analysis_core::proof_fingerprint::tests -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-proof-fingerprint-tests.txt"

./.agents/skills/next-milestone/scripts/collect_signals.sh \
  | tee "$RUN_ROOT/validation/proof/post-merge-collect-signals.txt"

cargo xtask family recommend --format json \
  | tee "$RUN_ROOT/validation/proof/post-merge-recommend.json"

cargo xtask family corpus-decision --format json \
  | tee "$RUN_ROOT/validation/proof/post-merge-corpus-decision.json"

cargo xtask family verify-decision-contract --format json \
  | tee "$RUN_ROOT/validation/proof/post-merge-verify-decision-contract.json"

cargo test -p xtask \
  | tee "$RUN_ROOT/validation/proof/post-merge-full-xtask-suite.txt"
```

Exit criteria:

- proof-lane diff integrated by parent only
- targeted stale test passes
- full `cargo test -p xtask` is green
- stop-state command truth remains frozen
- `verify-decision-contract` remains `pass`

If `WS-B-SYNC` was not proven necessary, skip directly to `M57-30`.

### `M57-20` Optional Lane B wording/doc sync

Owner: `WS-B-SYNC`  
Write scope: only the exact files assigned after audit

Default rule:

- skip this lane unless the parent audit proves a current falsehood

Possible responsibilities, only if explicitly assigned:

- tighten wording in `xtask/src/family/mod.rs`
- tighten wording in `xtask/src/family/helper_surface.rs`
- tighten wording in `xtask/src/family/decision_kernel.rs`
- sync narrow maintainer wording in the two docs files

Guardrails:

- any code-file change must be wording-only or presentation-only
- no semantic logic moves into shims
- no new abstraction layer appears
- docs may only be corrected to match already-proven truth
- no roadmap reopening
- no new milestone claims

Required proof surface when Rust source files are changed:

```bash
cargo test -p xtask family::analysis_core::helper_surface::tests -- --nocapture
cargo test -p xtask family::analysis_core::decision_contract::tests -- --nocapture
cargo test -p xtask family::analysis_core::proof_fingerprint::tests -- --nocapture
```

If `WS-B-SYNC` is docs-only, no worker cargo proof is required beyond parent final proof wall.

Exit criteria:

- worker submits only the exact approved file subset
- any Rust-source change remains wording-only
- commands and exit codes are captured
- if the audit was wrong and no falsehood exists, the lane is closed without edits

Stop if:

- the lane needs `xtask/src/lib.rs`
- the lane needs `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`
- the lane needs behavioral rather than wording changes

### `M57-21` Parent optional sync integration gate

Owner: Parent  
Write scope: integration on `feat/m40-plus` only

Exit criteria:

- optional sync diff integrates cleanly or is explicitly skipped
- any Rust wording change still leaves analysis-core tests green
- no broader product claims land than what the proof wall supports

## Final Serial Proof Wall

### `M57-30` Final serial proof wall + closeout

Owner: Parent  
Write scope: `$RUN_ROOT/**` and minimal fix-forward only if required

Run these exact commands before calling the milestone complete:

```bash
cargo test -p xtask recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held -- --nocapture
cargo test -p xtask family::analysis_core::helper_surface::tests -- --nocapture
cargo test -p xtask family::analysis_core::decision_contract::tests -- --nocapture
cargo test -p xtask family::analysis_core::proof_fingerprint::tests -- --nocapture
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

Record all outputs under `validation/final/` and `final-proof-manifest.json`.

Final acceptance truth:

- `recommendation_status = "insufficient_real_corpus"`
- `decision_status = "not_recommended"`
- `decision_action = "stop"`
- `decision_basis_code = "no_actionable_candidate"`
- `required_next_action = "record_stop_without_new_milestone"`
- `cargo xtask family verify-decision-contract --format json` remains `pass`
- `cargo test -p xtask` is green
- if docs changed, they now match the code truth exactly
- if docs did not change, they were already truthful

## Stop And Re-Scope Triggers

Stop M57 and write a new plan instead if any of these become true:

1. the proof repair requires consumer rewires in `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`
2. the proof repair requires new commands, new flags, new schemas, or new artifact paths
3. owner-surface truth now needs shared-core extraction or any new crate/module work
4. docs would need to claim new product behavior rather than current truth
5. the stop-state changes away from `stop` / `no_actionable_candidate` / `record_stop_without_new_milestone`
6. `verify-decision-contract` stops passing
7. more `xtask` failures appear and are not directly caused by the stale locked count expectation

## Acceptance Ledger

- [ ] kickoff branch, head, and tree state captured under `$RUN_ROOT/validation/kickoff/`
- [ ] baseline proof captures stored under `$RUN_ROOT/validation/baseline/`
- [ ] `contract-freeze.json`, `file-ownership.json`, `tasks.json`, and `queue.json` written
- [ ] parent audit completed and optional sync decision recorded
- [ ] `xtask/src/lib.rs` stale proof wall repaired without weakening the assertion
- [ ] expected source ids remain fixed
- [ ] expected source unit counts reflect current locked truth `[6, 12, 9, 3, 3]`
- [ ] `analysis_core/*` remains the only semantic owner surface
- [ ] `helper_surface.rs` and `decision_kernel.rs` remain compatibility-only passthroughs
- [ ] optional wording/doc sync was either skipped because current wording was already true or landed as a narrow correction only
- [ ] `collect_signals.sh` still reports the same stop-state summary
- [ ] `cargo xtask family recommend --format json` still reports `insufficient_real_corpus` and `not_recommended`
- [ ] `cargo xtask family corpus-decision --format json` still reports `stop` and `no_actionable_candidate`
- [ ] `cargo xtask family verify-decision-contract --format json` still passes
- [ ] `cargo test -p xtask` is green
- [ ] `acceptance-ledger.md`, `final-proof-manifest.json`, and `final-diff-summary.md` written

## Execution Posture

This milestone is intentionally small. The honest default is:

1. parent kickoff and audit
2. one worker lane for `xtask/src/lib.rs`
3. parent integration and proof wall
4. optional sync lane only if the audit proves it is needed
5. parent final closeout

Do not create extra workstreams just because the prior runbook had more of them. For M57, unnecessary parallelism is overhead, not leverage.
