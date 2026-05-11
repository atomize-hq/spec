# M47 Authority-Plan Completion Orchestration Plan

## 1. Title + Metadata

Status: **authoritative orchestration plan for completing and closing the current M47 `PLAN.md` session**  
Supersedes: **the prior M46 landing-oriented `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Kickoff branch: **`feat/m40-plus`**  
Kickoff HEAD: **`fff21c5d34732cecb61d3fa8a187e2f6096712b7`**  
Observed kickoff dirty source: **`PLAN.md` is already modified in the working tree and must be preserved, not normalized**  
Historical read-only context roots:  
- **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_shared_core_portability_follow_on/`**
- **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/`**
Canonical M47 run artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m47_post_m46_shared_core_portability_follow_on/`**  
Primary execution mode: **one parent-owned sequential authority lane**  
Permitted parallelism: **at most one support worker, launched only after the parent captures the full proof floor, for draft-only review and closeout writing**  
Forbidden parallelism: **any worker lane that reruns authoritative commands, edits repo source, interprets trigger truth independently, or turns M47 into implementation work**  
No landing path: **there is no branch-move, merge, cherry-pick, or integration phase in M47**  
Last rewritten: **2026-05-11**

## 2. Summary

This document is an execution contract for finishing the current M47 authority-plan session. It is not an implementation plan and it is not an extraction plan.

The parent agent owns the only real critical path:

1. capture kickoff truth and existing dirtiness
2. snapshot the current derived family-analysis latest artifacts
3. rerun the exact M47 proof floor
4. audit `PLAN.md` against the live proof floor, live code ownership surfaces, and the bounded M46 closeout truth
5. decide whether the current `PLAN.md` already passes as written or needs one bounded parent-only authority correction
6. finalize acceptance and closeout artifacts, or stop with a blocker record

The only honest worker use in M47 is post-capture drafting. A worker may help draft review-readiness or acceptance prose from parent-captured artifacts. A worker may not own truth. A worker may not rerun cargo commands. A worker may not author or edit `PLAN.md`. Any broader fan-out across `analysis_core`, `recommend`, `verify`, docs, or artifact schemas would be fake parallelism because the current trigger table does not authorize implementation at all.

The session succeeds only if the parent can prove all of the following from observed results:

- the live proof floor still passes on the current branch
- `verify-decision-contract` remains green
- `corpus-decision` remains `stop` with `record_stop_without_new_milestone`
- the candidate shared seam remains bounded to:
  - `xtask/src/family/analysis_core/helper_surface.rs`
  - `xtask/src/family/analysis_core/decision_contract.rs`
  - `xtask/src/family/analysis_core/proof_fingerprint.rs`
- the current consumers remain:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/verify.rs`
- local-only surfaces remain local:
  - wrappers
  - `promotion_artifacts.rs`
  - CLI wiring
  - path lookup
  - rendering
  - backend execution policy
- the trigger table in `PLAN.md` still does not authorize implementation or extraction
- M46 remains bounded second-language proof only
- no authored repo change occurs outside `PLAN.md` and the new M47 run artifacts

## 3. Hard Guards

- `PLAN.md` is the sole scope authority for this session.
- M47 is an authority-plan completion session. It is not a feature sprint, extraction sprint, portability implementation sprint, or family-selection rerun.
- The current trigger table remains non-authorizing until the parent proves otherwise from live outputs. No worker may infer implementation readiness from adjacency, prior milestones, or historical intent.
- Existing dirtiness must be preserved. At kickoff, `git status --short` already reports `M PLAN.md`. No task may clean, reset, or silently overwrite that state.
- The only authored source file that may change during M47 completion is `PLAN.md`, and only the parent may edit it.
- `PLAN.md` may be edited only if the proof-floor rerun or source audit exposes factual drift, stale wording, or a boundary mismatch inside the authority artifact itself.
- If the parent can accept the current `PLAN.md` without change, it must do so. Do not rewrite the plan for style churn.
- No worker may edit:
  - `PLAN.md`
  - `xtask/src/family/**`
  - `spec-core/**`
  - `semantic-families/**`
  - `docs/**`
  - any historical `.runs/` root
- Historical run roots are read-only reference inputs:
  - `.runs/m40_plus_shared_core_portability_follow_on/`
  - `.runs/m46_helper_aware_monotone_up_typescript/`
- The parent must treat direct command outputs as authoritative over helper-script summaries.
- `.agents/skills/next-milestone/scripts/collect_signals.sh` is informative, not authoritative. It does not overrule the raw outputs from:
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo test -p xtask`
- Proof-floor reruns may refresh derived latest artifacts under:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- Derived artifact churn is allowed only at those latest-artifact paths and only if captured explicitly under the M47 run root.
- There is no merge gate, no landing gate, and no branch-move gate in M47.
- If acceptance would require code changes outside `PLAN.md`, stop immediately and write `blocked.json`. Do not smuggle implementation into the authority closeout.

## 4. Execution Topology

### 4.1 Lane map

| Lane ID | Branch | Worktree path | Owner | Authority level | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m47-parent-authority` | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | Parent | **authoritative** | baseline freeze, artifact snapshot, proof-floor reruns, boundary audit, any allowed `PLAN.md` correction, final acceptance |
| `lane/m47-worker-support-draft` | `ws/spec-m47-support-draft` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m47/support-draft` | Worker | support-only, disabled by default | draft-only review-readiness and acceptance prose from parent-captured artifacts |

### 4.2 Topology rules

- `lane/m47-parent-authority` is the only lane allowed to run authoritative commands.
- `lane/m47-parent-authority` is the only lane allowed to interpret whether the proof floor matches the current M47 contract.
- `lane/m47-parent-authority` is the only lane allowed to decide whether `PLAN.md` needs a bounded correction.
- `lane/m47-parent-authority` is the only lane allowed to write:
  - `acceptance.md`
  - `closeout.md`
  - `run-state.json`
  - `blocked.json`
- `lane/m47-worker-support-draft` may launch only after:
  - kickoff baseline is frozen
  - the parent has captured the full proof floor
  - the parent has written read-only review inputs under the M47 run root
- The worker lane may write draft-only artifacts under:
  - `drafts/review-readiness.md`
  - `drafts/acceptance-outline.md`
- The worker lane may not run cargo, spec, git branch movement, or source edits.
- Concurrency cap is `1` worker. If the parent is still interpreting proof truth, the worker must remain disabled.

### 4.3 Honest parallelism statement

M47 critical-path work is sequential-only.

Reason:

- the proof floor is one shared truth surface
- the trigger table is one parent-owned interpretation surface
- the allowed authored source surface is effectively one file: `PLAN.md`
- there is no real module-isolated implementation work to split

The only honest parallelism is after the parent already owns the truth and wants drafting help. Anything broader would create merge noise and false certainty without increasing throughput.

## 5. Canonical Run-State And Artifact Surfaces

All canonical M47 run-state authority lives under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m47_post_m46_shared_core_portability_follow_on/`

### 5.1 Canonical artifact set

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch/head/dirty-state snapshot | Parent |
| `authority-freeze.json` | frozen scope, lane, and writable-surface contract | Parent |
| `in-scope-files.txt` | exact writable surfaces for this session | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `queue.json` | active execution queue and lane state | Parent |
| `tasks.json` | durable task ledger | Parent |
| `run-state.json` | final run summary and verdict | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance.md` | proof-floor and authority-review acceptance ledger | Parent |
| `closeout.md` | operator-facing closeout | Parent |
| `blocked.json` | required if M47 cannot close cleanly | Parent |
| `authority-snapshot/PLAN.md` | kickoff copy of the current authority artifact | Parent |
| `authority-snapshot/ORCH_PLAN.md` | kickoff copy of this orchestration contract | Parent |
| `drafts/review-readiness.md` | optional support-only draft artifact | Worker |
| `drafts/acceptance-outline.md` | optional support-only draft artifact | Worker |
| `validation/` | raw command captures, derived-artifact snapshots, and review notes | Parent-owned tree |

### 5.2 Required `baseline.json` contents

`baseline.json` must record:

- `run_id`
- `kickoff_timestamp`
- `repo_root`
- `authority_plan_path`
- `authority_orch_path`
- `branch`
- `head_sha`
- `head_short_sha`
- `plan_dirty_at_kickoff`
- `git_status_short`
- `historical_reference_roots`
- `proof_floor_commands`
- `allowed_authored_source_surfaces`
- `allowed_derived_artifact_surfaces`

### 5.3 Required `authority-freeze.json` contents

`authority-freeze.json` must record:

- current milestone title from `PLAN.md`
- statement that `PLAN.md` is authority-only and not implementation-authorizing
- exact candidate seam paths
- exact current consumer paths
- exact local-only surfaces that must stay local
- parent-authoritative lane
- optional support-only lane
- worker prohibition list
- run artifact root
- historical read-only inputs
- explicit rule that only `PLAN.md` may be edited as authored source, parent-only, and only if proof review requires it
- explicit rule that all other repo-source edits are forbidden
- explicit rule that there is no landing path in M47

### 5.4 Allowed `tasks.json` statuses

Each `tasks.json` entry must include at least:

- `id`
- `title`
- `lane`
- `owner`
- `status`
- `depends_on`
- `owned_surfaces`
- `required_commands`
- `writes`
- `started_at`
- `completed_at`
- `notes`

Allowed `status` values are:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `cancelled`

### 5.5 Minimal required `validation/` tree

```text
validation/
  baseline/
    branch.txt
    head.txt
    git-status-short.txt
    plan-working.diff
    kickoff-notes.md
  proof-floor/
    00-collect-signals.txt
    01-verify-decision-contract.json
    02-corpus-decision.json
    03-cargo-test-p-xtask.txt
    proof-floor-summary.md
  derived-artifacts/
    pre-coverage.latest.json
    pre-recommendation.latest.json
    pre-corpus-program-decision.latest.json
    post-coverage.latest.json
    post-recommendation.latest.json
    post-corpus-program-decision.latest.json
    coverage.latest.diff
    recommendation.latest.diff
    corpus-program-decision.latest.diff
    derived-artifact-summary.md
  authority-review/
    shared-vs-local-ownership.md
    trigger-table-check.md
    m46-bounded-proof-check.md
    implementation-boundary-check.md
    plan-rewrite-needed.md
  final/
    final-git-status-short.txt
    final-plan.diff
    acceptance-checklist.md
```

If one of the three latest analysis artifacts is missing at kickoff, the parent must write:

- `pre-coverage.latest.missing.txt`
- `pre-recommendation.latest.missing.txt`
- `pre-corpus-program-decision.latest.missing.txt`

instead of fabricating pre-state JSON.

### 5.6 Validation capture rules

- Every command capture file must include:
  - command
  - working directory
  - timestamp
  - exit code
  - raw stdout
  - raw stderr
- `validation/proof-floor/01-verify-decision-contract.json` must preserve the raw JSON output exactly as emitted by `cargo xtask family verify-decision-contract --format json`.
- `validation/proof-floor/02-corpus-decision.json` must preserve the raw JSON output exactly as emitted by `cargo xtask family corpus-decision --format json`.
- `validation/proof-floor/03-cargo-test-p-xtask.txt` must preserve the full `cargo test -p xtask` terminal output, including the final pass count.
- `validation/derived-artifacts/pre-*` and `post-*` files must be byte-for-byte copies of the live latest-artifact files, not reformatted JSON.
- `validation/derived-artifacts/*.diff` must be generated from those byte copies, not from live files after the fact.
- `validation/authority-review/*.md` must cite exact repo paths used in the review.
- Worker lanes may write only under `drafts/`. They may not write under `validation/`, `acceptance.md`, or `closeout.md`.

## 6. Workstream Plan

### 6.1 Task order

| Order | Task ID | Lane | Owner | Default state |
| --- | --- | --- | --- | --- |
| 1 | `gate-m47-00-baseline-freeze` | `lane/m47-parent-authority` | Parent | required |
| 2 | `gate-m47-05-authority-freeze` | `lane/m47-parent-authority` | Parent | required |
| 3 | `task-m47-10-derived-artifact-snapshot` | `lane/m47-parent-authority` | Parent | required |
| 4 | `gate-m47-15-proof-floor` | `lane/m47-parent-authority` | Parent | required |
| 5 | `task-m47-20-authority-boundary-audit` | `lane/m47-parent-authority` | Parent | required |
| 6 | `task-m47-25-support-draft` | `lane/m47-worker-support-draft` | Worker | optional |
| 7 | `gate-m47-30-parent-decision` | `lane/m47-parent-authority` | Parent | required |
| 8 | `task-m47-35-parent-plan-correction` | `lane/m47-parent-authority` | Parent | conditional |
| 9 | `gate-m47-40-final-acceptance` | `lane/m47-parent-authority` | Parent | required |
| 10 | `gate-m47-45-closeout` | `lane/m47-parent-authority` | Parent | required |

### 6.2 `gate-m47-00-baseline-freeze`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `baseline.json`
- `session-log.md`
- `validation/baseline/*`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`

Required commands:

```bash
git branch --show-current
git rev-parse HEAD
git rev-parse --short=7 HEAD
git status --short
git diff -- PLAN.md
```

Required artifact actions:

- copy the current `PLAN.md` into `authority-snapshot/PLAN.md`
- copy the current `ORCH_PLAN.md` into `authority-snapshot/ORCH_PLAN.md`
- record the kickoff `git status --short` exactly as observed
- record the kickoff `git diff -- PLAN.md` exactly as observed

Acceptance:

- branch is recorded as `feat/m40-plus`
- HEAD is recorded as `fff21c5d34732cecb61d3fa8a187e2f6096712b7`
- existing `PLAN.md` dirtiness is captured rather than modified
- the parent has frozen the exact starting authority inputs for later comparison

### 6.3 `gate-m47-05-authority-freeze`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `authority-freeze.json`
- `in-scope-files.txt`
- `out-of-scope-files.txt`
- `queue.json`
- `tasks.json`

Required contents for `in-scope-files.txt`:

- `PLAN.md`
- `.runs/m47_post_m46_shared_core_portability_follow_on/**`

Required contents for `out-of-scope-files.txt`:

- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`
- `spec-core/**`
- `semantic-families/**`
- `docs/**`
- `.runs/m40_plus_shared_core_portability_follow_on/**`
- `.runs/m46_helper_aware_monotone_up_typescript/**`

Acceptance:

- writable scope is frozen to `PLAN.md` plus new M47 run artifacts
- the parent has written an explicit forbidden-touch list
- worker scope is frozen to draft-only outputs

### 6.4 `task-m47-10-derived-artifact-snapshot`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/derived-artifacts/pre-*`
- `validation/derived-artifacts/post-*`
- `validation/derived-artifacts/*.diff`
- `validation/derived-artifacts/derived-artifact-summary.md`

Kickoff snapshot sources:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Required actions:

- capture byte-for-byte pre-state copies before any proof-floor rerun
- after the proof floor, capture byte-for-byte post-state copies
- compute explicit diffs for each latest artifact
- record whether proof-floor churn was:
  - `none`
  - `coverage_only`
  - `recommendation_and_decision_refresh`
  - `unexpected`

Acceptance:

- any latest-artifact refresh is documented exactly
- no artifact churn outside the allowed analysis latest paths is accepted as routine

### 6.5 `gate-m47-15-proof-floor`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/proof-floor/*`
- `session-log.md`

Required commands, run in this exact order:

```bash
.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Expected truth:

- `collect_signals.sh` may summarize the repo state, but the parent must treat it as advisory only
- `verify-decision-contract` must report `overall_verdict = "pass"`
- `corpus-decision` must report:
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask` must be green

Interpretation rule for the `cargo test -p xtask` count:

- the last known live narrative says `146` tests passed
- if the command is still green but the count differs, record the observed count exactly and treat it as a plan-audit question, not automatic failure
- if the command fails, M47 cannot close

Acceptance:

- all four proof-floor captures exist
- raw outputs support the exact stop-state the current M47 plan claims
- any divergence is recorded before the parent begins plan review

### 6.6 `task-m47-20-authority-boundary-audit`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/authority-review/shared-vs-local-ownership.md`
- `validation/authority-review/trigger-table-check.md`
- `validation/authority-review/m46-bounded-proof-check.md`
- `validation/authority-review/implementation-boundary-check.md`
- `validation/authority-review/plan-rewrite-needed.md`

Required review inputs:

- `PLAN.md`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `.runs/m46_helper_aware_monotone_up_typescript/closeout.md`
- `.runs/m40_plus_shared_core_portability_follow_on/acceptance.md`
- `.runs/m40_plus_shared_core_portability_follow_on/closeout.md`
- the M47 proof-floor captures

Required review verdicts:

- `shared-vs-local-ownership.md`
  - confirms the shared seam is still exactly the three `analysis_core/*` files
  - confirms `recommend.rs` and `verify.rs` are current consumers
  - confirms wrappers, artifact schemas, CLI wiring, paths, rendering, and backend execution policy remain local-only
- `trigger-table-check.md`
  - confirms each current trigger row remains untriggered
  - confirms current reuse pressure is real but still insufficient to authorize extraction
- `m46-bounded-proof-check.md`
  - confirms M46 remains bounded TypeScript proof only
  - confirms `.test.spec --target-language typescript` remains unsupported and must not be widened by implication
- `implementation-boundary-check.md`
  - confirms the current `PLAN.md` remains authority-only
  - confirms it does not authorize implementation, extraction, backend widening, or renewed family-selection work
- `plan-rewrite-needed.md`
  - must end in exactly one parent decision:
    - `no_rewrite_required`
    - `parent_rewrite_required`
    - `blocked_external_drift`

Acceptance:

- the parent has one explicit review artifact for each authority question
- the parent has named whether `PLAN.md` is already acceptable as written

### 6.7 `task-m47-25-support-draft`

Lane: `lane/m47-worker-support-draft`  
Owner: Worker  
Default: disabled

Owned surfaces:

- `drafts/review-readiness.md`
- `drafts/acceptance-outline.md`

Allowed inputs:

- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `validation/proof-floor/*`
- `validation/derived-artifacts/derived-artifact-summary.md`
- `validation/authority-review/*`

Forbidden actions:

- no cargo commands
- no git commands other than read-only status if the parent explicitly allows it
- no edits to repo source
- no edits to canonical acceptance or closeout artifacts

Acceptance:

- drafts are grounded only in parent-captured artifacts
- the parent can discard the drafts with zero impact on canonical run truth

### 6.8 `gate-m47-30-parent-decision`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Decision branches:

- `Decision A: accept current PLAN.md`
  - use when `plan-rewrite-needed.md` concludes `no_rewrite_required`
  - skip `task-m47-35-parent-plan-correction`
  - proceed directly to final acceptance
- `Decision B: bounded parent-only PLAN.md correction`
  - use when `plan-rewrite-needed.md` concludes `parent_rewrite_required`
  - correction scope remains authority-only
  - no worker ownership
  - no code or runtime edits
- `Decision C: block the milestone`
  - use when `plan-rewrite-needed.md` concludes `blocked_external_drift`
  - write `blocked.json`
  - stop without improvising implementation

Blocking conditions:

- `verify-decision-contract` no longer passes
- `corpus-decision` no longer returns `stop` plus `record_stop_without_new_milestone`
- the shared-vs-local ownership boundary no longer matches the code
- closing M47 would require edits outside `PLAN.md`
- the trigger table has become true and the correct next move is a new implementation plan rather than an M47 closeout

### 6.9 `task-m47-35-parent-plan-correction`

Lane: `lane/m47-parent-authority`  
Owner: Parent  
Default: conditional

Owned surfaces:

- `PLAN.md`
- `validation/final/final-plan.diff`
- `validation/authority-review/plan-rewrite-needed.md`

Allowed correction scope:

- refresh stale live-proof wording
- tighten ownership or local-only boundary wording
- tighten the trigger-table wording if the current code or proof floor demands it
- correct any authority drift introduced by current branch truth

Forbidden correction scope:

- no new implementation authorization
- no new candidate seam
- no new backend claim
- no new family-promotion authorization
- no schema or runtime edits

Rerun rule after correction:

- if the edit changes claimed live command truth or trigger interpretation, rerun:
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo xtask family corpus-decision --format json`
- if the edit only tightens prose around already-captured truth, do not rerun cargo unnecessarily

Acceptance:

- the final `PLAN.md` is still authority-only
- all edits are bounded to the authority artifact itself

### 6.10 `gate-m47-40-final-acceptance`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `acceptance.md`
- `run-state.json`
- `validation/final/final-git-status-short.txt`
- `validation/final/acceptance-checklist.md`

`acceptance.md` must record:

- branch and final HEAD
- whether `PLAN.md` changed during the session
- the exact proof-floor command list
- per-command observed exit status
- the exact observed `verify-decision-contract` verdict
- the exact observed `corpus-decision` tuple
- the exact observed `cargo test -p xtask` pass/fail result and pass count if green
- derived-artifact churn summary
- shared-vs-local ownership verdict
- trigger-table verdict
- M46 bounded-proof verdict
- final decision:
  - `accepted_without_plan_rewrite`
  - `accepted_with_parent_plan_rewrite`
  - `blocked`

Acceptance:

- the parent has written one clear verdict with machine-evidence support
- all required review questions are answered in canonical artifacts

### 6.11 `gate-m47-45-closeout`

Lane: `lane/m47-parent-authority`  
Owner: Parent

Owned surfaces:

- `closeout.md`
- `tasks.json`
- `queue.json`
- `session-log.md`

`closeout.md` must record:

- concise operator summary of the M47 result
- whether the current `PLAN.md` was accepted as-is or corrected by the parent
- the exact live stop-state preserved at closeout
- the exact bounded candidate seam preserved at closeout
- why M47 used sequential parent ownership
- why broader worker fan-out was intentionally rejected
- any residual follow-up, explicitly marked as outside M47

Final session states:

- `closed_green`
- `closed_green_with_parent_plan_rewrite`
- `blocked_no_authority_closeout`

## 7. Context-Control Rules

- Parent keeps only these live canonical inputs in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - `validation/proof-floor/proof-floor-summary.md`
  - `validation/authority-review/*.md`
- Historical reference artifacts are read-only and should be summarized, not copied into live prompt context wholesale.
- Worker prompts contain only:
  - owned draft files
  - exact input artifact paths
  - forbidden actions
  - the rule that the worker does not own truth
- Workers return only:
  - changed draft files
  - a brief summary
  - blockers or ambiguities
- Workers do not write canonical run-state files.
- Close the worker immediately after its draft is either consumed or discarded.
- Do not poll aggressively. Use task completion sentinels or explicit waits.

## 8. Tests And Acceptance

### 8.1 Proof-floor acceptance

- `cargo xtask family verify-decision-contract --format json` reports `overall_verdict == "pass"`.
- `cargo xtask family corpus-decision --format json` reports:
  - `decision_action == "stop"`
  - `decision_basis_code == "no_actionable_candidate"`
  - `required_next_action == "record_stop_without_new_milestone"`
- `cargo test -p xtask` is green.

### 8.2 Boundary acceptance

- the bounded shared seam remains exactly:
  - `xtask/src/family/analysis_core/helper_surface.rs`
  - `xtask/src/family/analysis_core/decision_contract.rs`
  - `xtask/src/family/analysis_core/proof_fingerprint.rs`
- the current consumers remain exactly:
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/verify.rs`
- local-only surfaces remain local:
  - wrappers
  - `promotion_artifacts.rs`
  - CLI wiring
  - paths
  - rendering
  - backend execution policy

### 8.3 Authority acceptance

- `PLAN.md` remains authority-only.
- The trigger table still does not authorize:
  - local seam extraction
  - cross-crate extraction
  - broader portability implementation
  - renewed family-selection work
- M46 remains bounded second-language proof only.

### 8.4 Scope acceptance

- no repo source changed outside `PLAN.md`
- no worker edited source
- any derived artifact churn stayed confined to:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

## 9. Assumptions And Stop Conditions

### 9.1 Assumptions

- The kickoff branch remains `feat/m40-plus`.
- The kickoff authoritative HEAD remains `fff21c5d34732cecb61d3fa8a187e2f6096712b7`.
- The current working-tree `PLAN.md` modification is intentional repo state and must be preserved.
- Historical `.runs/` roots are useful context but not live authority.
- Cargo command lock waits are operational noise unless they prevent completion.

### 9.2 Immediate stop conditions

Stop the session and write `blocked.json` if any of the following occurs:

- `verify-decision-contract` fails
- `corpus-decision` no longer returns the current stop-state tuple
- `cargo test -p xtask` fails
- the candidate seam or current-consumer set no longer matches the current M47 authority plan
- closing M47 would require edits outside `PLAN.md`
- the correct next move is a new implementation plan rather than an authority closeout
- the parent cannot distinguish historical context from current branch truth

## 10. Future Triggered Implementation Split Reference

This section is preserved only so the current M47 session closes without losing the first honest implementation split promised by `PLAN.md`. It is not executable during M47.

| Step | Modules touched | Depends on |
| --- | --- | --- |
| freeze seam interface | `xtask/src/family/analysis_core/` | — |
| rewire in-tree consumers | `xtask/src/family/` | freeze seam interface |
| docs and authority sync | repo-root plans, `.runs/`, docs artifacts | freeze seam interface |
| command-surface adoption | `xtask/src/family/`, `xtask/src/` | rewire in-tree consumers |

Future lane order, only if a trigger later turns true:

- `Lane A`: freeze seam interface
- `Lane B`: rewire in-tree consumers, after `Lane A`
- `Lane C`: docs and authority sync, after `Lane A`
- `Lane D`: command-surface adoption, after `Lane B`

Current M47 rule:

- keep this split as reference only
- do not pre-launch any of these lanes
- do not turn this future split into present authorization
