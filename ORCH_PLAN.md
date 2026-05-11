# M48 Shared-Core Portability Slice 1 Orchestration Plan

## 1. Title + Metadata

Status: **authoritative orchestration plan for executing M48 Lane A**  
Supersedes: **the stale M47 closeout-oriented `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M48: Shared-Core Portability Follow-On, Slice 1 Implementation Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Kickoff branch: **`feat/m40-plus`**  
Kickoff HEAD: **`0283db1be641d04374bceec313c85d230f98c1be`**  
Kickoff short HEAD: **`0283db1`**  
Kickoff tree expectation: **clean**  
Primary write scope: **Lane A only, parent-owned edits inside `xtask/src/family/analysis_core/*`**  
Read-only proof surfaces:  
- **`xtask/src/family/recommend.rs`**
- **`xtask/src/family/verify.rs`**
- **`xtask/src/family/promotion_artifacts.rs`**
- **`xtask/src/family/helper_surface.rs`**
- **`xtask/src/family/decision_kernel.rs`**
Canonical M48 run artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m48_shared_core_portability_slice1_lane_a/`**  
Primary execution mode: **one parent-owned sequential code lane**  
Permitted parallelism: **up to 2 bounded support workers, read-only only, after the parent freezes baseline truth**  
Forbidden parallelism: **any split source-edit lane inside `xtask/src/family/analysis_core/`, any worker-owned truth interpretation, any worker-owned repo source edit**  
Last rewritten: **2026-05-11**

## 2. Summary

M48 is the first execution slice after the M47 authority stop. It is not a new architecture search and it is not a consumer-rewire milestone.

The seam already exists. The only honest critical path is for the parent agent to freeze and prove the existing `xtask/src/family/analysis_core/*` owner surface in one sequential lane:

1. `xtask/src/family/analysis_core/mod.rs`
2. `xtask/src/family/analysis_core/helper_surface.rs`
3. `xtask/src/family/analysis_core/decision_contract.rs`
4. `xtask/src/family/analysis_core/proof_fingerprint.rs`

Everything downstream remains a proof wall, not implementation scope. `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`, and the two shims stay read-only unless the parent explicitly enters the narrow compile-only exception allowed by `PLAN.md`.

The live proof floor is already known at kickoff and must remain true after the slice lands:

- `./.agents/skills/next-milestone/scripts/collect_signals.sh`
  - branch `feat/m40-plus`
  - clean tree
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask`
  - green
  - `146` tests passed at the validated kickoff floor

Support workers are still useful, but only honestly. They may help with read-only downstream audit and acceptance drafting after the parent captures baseline truth. They do not own semantics, commands of record, source edits, or final acceptance.

## 3. Hard Guards

- `PLAN.md` is the sole scope authority for M48 execution.
- M48 executes Lane A only. It does not authorize Lane B consumer rewires, CLI rewiring, path lookup changes, schema changes, backend widening, crate extraction, or new abstraction layers.
- The parent agent is the only source-edit owner for:
  - `xtask/src/family/analysis_core/mod.rs`
  - `xtask/src/family/analysis_core/helper_surface.rs`
  - `xtask/src/family/analysis_core/decision_contract.rs`
  - `xtask/src/family/analysis_core/proof_fingerprint.rs`
- Seam-local tests inside those files are parent-owned. The only allowed non-seam proof-test write surface is `xtask/src/lib.rs`, and only if a narrow existing `xtask` test there must be tightened to prove the frozen seam contract or downstream parity within Lane A.
- `recommend.rs`, `verify.rs`, `promotion_artifacts.rs`, `helper_surface.rs`, and `decision_kernel.rs` are read-only proof surfaces by default.
- The only allowed exception outside `analysis_core/*` is the narrow `PLAN.md` compile-only proof fix:
  - parent-only
  - separately justified in the M48 run root
  - no semantic change
  - no ownership change
  - no routing change
  - no output-meaning change
- No worker may edit repo source, run authoritative `cargo xtask` or `cargo test` commands, or decide whether proof truth is acceptable.
- No worker may reinterpret the stop-state basis or widen scope because files are adjacent.
- No new module, trait, helper layer, schema field, CLI flag, file move, or facade owner file is allowed in this slice.
- `analysis_core/*` is one coupled seam vocabulary and one proof wall. There is no safe code-parallelization opportunity inside it.
- `collect_signals.sh` is advisory only. Raw command outputs remain authoritative over helper-script summaries.
- The parent must stop if the kickoff rerun no longer matches the validated proof floor before any source edit begins.
- `PLAN.md` and `ORCH_PLAN.md` are authority inputs during execution. They are not runtime edit surfaces.
- Historical run roots are read-only inputs only:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_shared_core_portability_follow_on/`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m47_post_m46_shared_core_portability_follow_on/`

## 4. Execution Topology

### 4.1 Lane map

| Lane ID | Branch | Worktree path | Owner | Authority level | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m48-parent-authority` | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | Parent | **authoritative** | capture baseline, freeze run contract, perform all seam edits, run all proof gates, integrate acceptance |
| `lane/m48-worker-proof-audit` | `ws/m48-proof-audit` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m48/proof-audit` | Worker | support-only | read-only downstream-proof audit from parent-captured artifacts and repo files |
| `lane/m48-worker-acceptance` | `ws/m48-acceptance` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m48/acceptance` | Worker | support-only | draft acceptance and closeout prose from parent-captured artifacts |

### 4.2 Topology rules

- `lane/m48-parent-authority` is the only lane allowed to edit repo source.
- `lane/m48-parent-authority` is the only lane allowed to run:
  - `./.agents/skills/next-milestone/scripts/collect_signals.sh`
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo test -p xtask`
- `lane/m48-parent-authority` is the only lane allowed to write canonical authoritative M48 run artifacts outside `drafts/`.
- `lane/m48-worker-proof-audit` may launch only after:
  - baseline branch/head/status is frozen
  - kickoff proof-floor outputs are captured
  - the parent has written the read-only audit inputs under the M48 run root
- `lane/m48-worker-acceptance` may launch only after:
  - the final proof-wall sweep is complete
  - the parent has decided there is no unresolved blocker
- Worker lanes may read any repo path needed for audit, but may write only `drafts/` outputs and parent-requested summaries.
- Worker outputs are advisory. The parent decides whether a flagged issue is real and whether it changes execution.
- Maximum support concurrency is `2` workers.
- If the parent enters the compile-only exception review, all worker activity pauses until the exception is accepted or the run is stopped.

### 4.3 Honest parallelism statement

M48 has one real implementation lane and zero honest code-splitting opportunities.

Reason:

- every real edit lives in the same seam directory
- helper-surface, decision-contract, and proof-fingerprint semantics share one vocabulary and one proof wall
- the downstream acceptance surface is global, not lane-local
- splitting edits across worktrees would trade throughput for merge conflict and semantic skew risk

Support workers are allowed only because they do not own source truth. They compress review time, not implementation time.

## 5. Canonical Run-State And Artifact Surfaces

All canonical M48 execution state lives under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m48_shared_core_portability_slice1_lane_a/`

### 5.1 Canonical artifact set

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch/head/status and proof-floor expectation snapshot | Parent |
| `authority-freeze.json` | frozen writable surface, read-only surface, and lane contract | Parent |
| `in-scope-files.txt` | exact allowed source-edit surfaces | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `queue.json` | live lane and task queue | Parent |
| `tasks.json` | durable task ledger | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance.md` | final proof and acceptance ledger | Parent |
| `closeout.md` | operator-facing closeout | Parent |
| `blocked.json` | blocker artifact if the run stops incomplete | Parent |
| `authority-snapshot/PLAN.md` | kickoff copy of the authority plan input | Parent |
| `authority-snapshot/ORCH_PLAN.md` | kickoff copy of the orchestration input | Parent |
| `validation/baseline/*` | kickoff command captures and notes | Parent |
| `validation/proof-floor/*` | kickoff and final authoritative command captures | Parent |
| `validation/derived-artifacts/*` | pre/post latest-artifact snapshots and diffs | Parent |
| `validation/source-audit/*` | parent-owned seam-contract review notes | Parent |
| `validation/downstream-audit/*` | parent-owned downstream parity review notes | Parent |
| `validation/final/*` | final git, proof, and acceptance checklist captures | Parent |
| `drafts/proof-audit.md` | optional worker downstream audit draft | Worker |
| `drafts/acceptance-outline.md` | optional worker acceptance draft | Worker |
| `drafts/closeout-outline.md` | optional worker closeout draft | Worker |

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
- `git_status_short`
- `expected_proof_floor`
- `allowed_parent_write_surfaces`
- `allowed_support_lanes`
- `read_only_proof_surfaces`
- `historical_reference_roots`

### 5.3 Required `authority-freeze.json` contents

`authority-freeze.json` must record:

- current milestone title from `PLAN.md`
- statement that M48 is Lane A only
- exact parent-owned source-edit surfaces
- exact read-only downstream proof surfaces
- exact worker prohibition list
- canonical run root
- support lane definitions
- explicit no-code-parallelization statement for `analysis_core/*`
- compile-only exception rule from `PLAN.md`
- final proof gate commands

### 5.4 Allowed `tasks.json` states

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

### 5.5 Minimal required validation tree

```text
validation/
  baseline/
    branch.txt
    head.txt
    git-status-short.txt
    kickoff-notes.md
  proof-floor/
    00-collect-signals.txt
    01-verify-decision-contract.json
    02-corpus-decision.json
    03-cargo-test-p-xtask.txt
    10-final-collect-signals.txt
    11-final-verify-decision-contract.json
    12-final-corpus-decision.json
    13-final-cargo-test-p-xtask.txt
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
  source-audit/
    facade-export-inventory.md
    helper-surface-contract.md
    decision-contract-branches.md
    proof-fingerprint-normalization.md
    exception-review.md
  downstream-audit/
    read-only-consumer-check.md
    shim-immutability-check.md
    command-surface-parity.md
  final/
    final-git-status-short.txt
    final-diff-summary.md
    acceptance-checklist.md
```

If any kickoff latest artifact is missing, record the missing state explicitly:

- `pre-coverage.latest.missing.txt`
- `pre-recommendation.latest.missing.txt`
- `pre-corpus-program-decision.latest.missing.txt`

### 5.6 Capture rules

- Every command capture must include:
  - command
  - working directory
  - timestamp
  - exit code
  - raw stdout
  - raw stderr
- `01-verify-decision-contract.json`, `02-corpus-decision.json`, `11-final-verify-decision-contract.json`, and `12-final-corpus-decision.json` must preserve raw JSON exactly as emitted.
- The `cargo test -p xtask` capture files must preserve full terminal output, including the final pass count.
- Latest-artifact pre/post files must be byte-for-byte copies of the live `.latest.json` files.
- Latest-artifact diffs must be generated from the captured pre/post byte copies, not from reformatted JSON.
- Worker lanes may not write under `validation/`, `acceptance.md`, `closeout.md`, or `blocked.json`.

## 6. Workstream Plan

### 6.1 Task order

| Order | Task ID | Lane | Owner | State |
| --- | --- | --- | --- | --- |
| 1 | `gate-m48-00-baseline-freeze` | `lane/m48-parent-authority` | Parent | required |
| 2 | `gate-m48-05-authority-freeze` | `lane/m48-parent-authority` | Parent | required |
| 3 | `task-m48-10-pre-edit-artifact-snapshot` | `lane/m48-parent-authority` | Parent | required |
| 4 | `gate-m48-15-kickoff-proof-floor` | `lane/m48-parent-authority` | Parent | required |
| 5 | `task-m48-20-seam-facade-freeze` | `lane/m48-parent-authority` | Parent | required |
| 6 | `task-m48-25-helper-surface-freeze` | `lane/m48-parent-authority` | Parent | required |
| 7 | `task-m48-30-decision-contract-freeze` | `lane/m48-parent-authority` | Parent | required |
| 8 | `task-m48-35-proof-fingerprint-freeze` | `lane/m48-parent-authority` | Parent | required |
| 9 | `task-m48-40-readonly-downstream-audit` | `lane/m48-worker-proof-audit` | Worker | optional |
| 10 | `gate-m48-42-compile-only-exception-review` | `lane/m48-parent-authority` | Parent | conditional |
| 11 | `gate-m48-45-proof-wall-sweep` | `lane/m48-parent-authority` | Parent | required |
| 12 | `task-m48-50-acceptance-draft` | `lane/m48-worker-acceptance` | Worker | optional |
| 13 | `gate-m48-55-parent-acceptance` | `lane/m48-parent-authority` | Parent | required |
| 14 | `gate-m48-60-closeout` | `lane/m48-parent-authority` | Parent | required |

### 6.2 `gate-m48-00-baseline-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `baseline.json`
- `session-log.md`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `validation/baseline/*`

Required commands:

```bash
git branch --show-current
git rev-parse HEAD
git rev-parse --short=7 HEAD
git status --short
```

Required artifact actions:

- copy the starting `PLAN.md` to `authority-snapshot/PLAN.md`
- copy the starting `ORCH_PLAN.md` to `authority-snapshot/ORCH_PLAN.md`
- treat both copies as read-only audit snapshots for the remainder of the run

Acceptance:

- branch is `feat/m40-plus`
- HEAD is `0283db1be641d04374bceec313c85d230f98c1be`
- kickoff tree is clean or any unexpected dirtiness is recorded before work continues
- the parent has frozen the starting state before any source edit or proof rerun
- the authority inputs are snapshotted before execution for later audit

Stop rule:

- if branch or head does not match the expected kickoff basis and the divergence is not explicitly accepted, stop before M48 begins

### 6.3 `gate-m48-05-authority-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `authority-freeze.json`
- `in-scope-files.txt`
- `out-of-scope-files.txt`
- `queue.json`
- `tasks.json`

Required contents for `in-scope-files.txt`:

- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/lib.rs` only if a narrow existing proof test there becomes necessary to prove the seam contract or downstream parity
- `.runs/m48_shared_core_portability_slice1_lane_a/**`

Required contents for `out-of-scope-files.txt`:

- `PLAN.md`
- `ORCH_PLAN.md`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs` runtime logic outside any unavoidable existing proof tests
- `spec-core/**`
- `semantic-families/**`
- `docs/**`

Acceptance:

- writable scope is frozen to the four seam files plus the narrow proof exceptions allowed by `PLAN.md`
- worker scope is frozen to draft-only support output
- downstream consumers and shims are explicitly locked read-only

### 6.4 `task-m48-10-pre-edit-artifact-snapshot`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/derived-artifacts/pre-*`
- `validation/derived-artifacts/derived-artifact-summary.md`

Kickoff snapshot sources:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Required actions:

- capture byte-for-byte pre-state copies before any proof-floor rerun
- record whether each file was present, missing, or stale by timestamp only
- note whether the `.latest.json` surfaces already reflect the validated kickoff floor

Acceptance:

- pre-edit latest artifacts are frozen before any command refresh
- later artifact churn can be compared against a known pre-edit basis

### 6.5 `gate-m48-15-kickoff-proof-floor`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/proof-floor/00-collect-signals.txt`
- `validation/proof-floor/01-verify-decision-contract.json`
- `validation/proof-floor/02-corpus-decision.json`
- `validation/proof-floor/03-cargo-test-p-xtask.txt`
- `validation/proof-floor/proof-floor-summary.md`

Required commands, run in this exact order:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Expected truth:

- `collect_signals.sh` reports branch `feat/m40-plus`, clean tree, and the known stop-state summary
- `verify-decision-contract` reports `overall_verdict = "pass"`
- `corpus-decision` reports:
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask` is green

Pass-count handling:

- the validated kickoff floor says `146` tests passed
- if the command stays green but the count differs, record the observed count exactly and treat it as an execution-review question, not automatic success
- if the command fails, M48 does not start

Acceptance:

- all four proof-floor captures exist
- the parent has a live baseline before any seam edit begins
- any divergence from the expected stop-state basis is captured before the run proceeds

### 6.6 `task-m48-20-seam-facade-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `xtask/src/family/analysis_core/mod.rs`
- `validation/source-audit/facade-export-inventory.md`

Required inspection commands:

```bash
rg -n "pub use|pub mod" xtask/src/family/analysis_core/mod.rs
rg -n "analysis_core" xtask/src/family/recommend.rs xtask/src/family/verify.rs xtask/src/family/promotion_artifacts.rs xtask/src/family/helper_surface.rs xtask/src/family/decision_kernel.rs
```

Required work:

- make the export inventory explicit
- group exports by semantic concern, not accidental file order
- preserve the frozen facade inventory described by `PLAN.md`
- avoid adding new owner surfaces or silent new exports

Acceptance:

- the facade is the sole approved seam entry point
- every approved seam export remains reachable through `analysis_core`
- no consumer change is required to understand seam ownership

Stop rule:

- if a missing export implies a new owner file or a downstream semantic patch, stop and re-scope

### 6.7 `task-m48-25-helper-surface-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `xtask/src/family/analysis_core/helper_surface.rs`
- `validation/source-audit/helper-surface-contract.md`

Required inspection commands:

```bash
rg -n "classify_helper_surface|durable_non_promotable_helper_surface_candidate_tuple|recommendation_.*helper_surface|HELPER_SURFACE_FINGERPRINT" xtask/src/family/analysis_core/helper_surface.rs
```

Required work:

- preserve the exact durable-hold tuple contract
- preserve the exact helper-surface follow-on tuple contract
- keep `classify_helper_surface()` narrow
- add explicit proof for contradictory inputs and malformed fingerprint inputs

Acceptance:

- wrong primary reason rejects classification
- non-`unknown` overlap rejects classification
- `real_example_hits = 0` rejects classification
- malformed or semantically wrong fingerprints reject classification
- the file remains a classifier, not a policy surface

Stop rule:

- if helper-surface truth now requires broader reason codes, overlap logic, or consumer-specific exceptions, stop and write `blocked.json`

### 6.8 `task-m48-30-decision-contract-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `xtask/src/family/analysis_core/decision_contract.rs`
- `validation/source-audit/decision-contract-branches.md`

Required inspection commands:

```bash
rg -n "decision_contract_stop_state_tuple|corpus_program_basis_snapshot|basis_snapshot_requires_helper_surface_follow_on|derive_corpus_program_decision_contract" xtask/src/family/analysis_core/decision_contract.rs
```

Required work:

- preserve the exact stop-state tuple
- preserve the exact basis snapshot projection
- add explicit proof for the five real branches named by `PLAN.md`
- keep default stop behavior unchanged

Acceptance:

- promotion-ready branch is explicit
- blocked-on-evidence branch is explicit
- helper-surface follow-on branch is explicit
- policy-interpretation blocker branch is explicit
- default stop branch is explicit
- `corpus-decision` still returns the same stop tuple unless the basis actually changes

Stop rule:

- if a new policy surface or a sixth meaningful branch is required to explain current behavior, stop and write `blocked.json`

### 6.9 `task-m48-35-proof-fingerprint-freeze`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `validation/source-audit/proof-fingerprint-normalization.md`

Required inspection commands:

```bash
rg -n "normalized_.*proof_fingerprint|normalized_for_recommend_determinism|fingerprint" xtask/src/family/analysis_core/proof_fingerprint.rs
```

Required work:

- preserve exact normalization fields for coverage, recommendation, and corpus-decision artifacts
- prove timestamp and bookkeeping churn do not change fingerprints when semantics are unchanged
- prove semantic-field drift does change fingerprints
- keep serialization local and boring

Acceptance:

- coverage fingerprint ignores timestamp and path churn only
- recommendation fingerprint ignores `generated_at` and delta churn only
- corpus-decision fingerprint ignores non-semantic churn only
- semantic drift changes the relevant fingerprint
- no schema change or helper-layer introduction is needed to keep fingerprints truthful

Stop rule:

- if normalization requires external helpers, schema widening, or consumer rewrites to stay correct, stop and re-scope

### 6.10 `task-m48-40-readonly-downstream-audit`

Lane: `lane/m48-worker-proof-audit`  
Owner: Worker  
Default: disabled until the parent completes `gate-m48-15-kickoff-proof-floor`

Owned surfaces:

- `drafts/proof-audit.md`

Read-only review inputs:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- the parent-captured kickoff proof-floor outputs
- the parent-captured seam diff summary

Allowed commands:

```bash
rg -n "analysis_core|helper_surface|decision_contract|proof_fingerprint" xtask/src/family/recommend.rs xtask/src/family/verify.rs xtask/src/family/promotion_artifacts.rs xtask/src/family/helper_surface.rs xtask/src/family/decision_kernel.rs
sed -n '1,220p' xtask/src/family/recommend.rs
sed -n '1,220p' xtask/src/family/verify.rs
sed -n '1,220p' xtask/src/family/promotion_artifacts.rs
```

Required output:

- identify any place where downstream behavior appears coupled to unstated seam semantics
- identify whether shims still look compatibility-only
- identify any place where the parent should tighten proof or wording before final acceptance

Worker prohibitions:

- no repo source edits
- no `cargo` commands
- no git mutation
- no final truth claims

Acceptance:

- the parent receives a narrow read-only audit
- any flagged risk is expressed as a bounded concern, not a source edit request

### 6.11 `gate-m48-42-compile-only-exception-review`

Lane: `lane/m48-parent-authority`  
Owner: Parent  
Default: skipped unless the proof wall reveals a compile-only issue outside `analysis_core/*`

Owned surfaces:

- `validation/source-audit/exception-review.md`
- `blocked.json` if the exception is rejected

Required decision record:

- why the issue is compile-only rather than semantic
- exact file touched
- why the change stays inside the narrow `PLAN.md` exception
- why stopping is worse than the bounded fix

Acceptance:

- either the exception is rejected and the run stops
- or the exception is accepted with explicit justification before any out-of-seam edit occurs

### 6.12 `gate-m48-45-proof-wall-sweep`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `validation/proof-floor/10-final-collect-signals.txt`
- `validation/proof-floor/11-final-verify-decision-contract.json`
- `validation/proof-floor/12-final-corpus-decision.json`
- `validation/proof-floor/13-final-cargo-test-p-xtask.txt`
- `validation/derived-artifacts/post-*`
- `validation/derived-artifacts/*.diff`
- `validation/downstream-audit/*`

Required commands, run in this exact order:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Required review outputs:

- `read-only-consumer-check.md`
- `shim-immutability-check.md`
- `command-surface-parity.md`

Acceptance:

- `collect_signals.sh` still lands on the same stop-state summary
- `verify-decision-contract` still passes
- `corpus-decision` still emits:
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask` is green
- latest-artifact churn, if any, is documented and bounded
- downstream read-only surfaces remain unchanged unless the compile-only exception was explicitly accepted

Stop rule:

- if downstream behavior drifts, a read-only surface needs a semantic edit, or the stop tuple changes, stop the run and record the blocker

### 6.13 `task-m48-50-acceptance-draft`

Lane: `lane/m48-worker-acceptance`  
Owner: Worker  
Default: disabled until the parent completes `gate-m48-45-proof-wall-sweep`

Owned surfaces:

- `drafts/acceptance-outline.md`
- `drafts/closeout-outline.md`

Read-only inputs:

- final proof-floor captures
- derived-artifact summary
- parent source diff summary
- worker downstream audit summary, if present

Required output:

- acceptance outline tied to command truth
- closeout outline tied to scope and stop-state preservation
- any wording the parent should tighten before finalizing acceptance

Worker prohibitions:

- no repo source edits
- no command reruns
- no final acceptance decision

### 6.14 `gate-m48-55-parent-acceptance`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `acceptance.md`
- `validation/final/acceptance-checklist.md`
- `validation/final/final-diff-summary.md`
- `validation/final/final-git-status-short.txt`

Acceptance checklist:

- only allowed source surfaces changed
- `analysis_core/mod.rs` is the sole approved seam facade
- helper-surface tuple semantics are frozen and explicitly tested
- decision-contract tuple semantics are frozen and explicitly tested
- proof-fingerprint normalization rules are frozen and explicitly tested
- compatibility shims remain compatibility-only
- `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` still behave the same
- kickoff and final proof floors both support the same stop-state truth
- no scope leakage into consumers, schemas, CLI wiring, path lookup, or backend policy

Acceptance:

- the parent can explain the entire slice as a seam freeze and proof-hardening run, not a hidden consumer rewire
- any optional worker findings are either resolved or explicitly rejected with reason

### 6.15 `gate-m48-60-closeout`

Lane: `lane/m48-parent-authority`  
Owner: Parent

Owned surfaces:

- `closeout.md`
- `blocked.json` if needed
- final `tasks.json`
- final `queue.json`

Required closeout contents:

- actual touched source surfaces
- final proof outcomes
- latest-artifact churn summary
- whether any compile-only exception was invoked
- deferred follow-on items that remain out of scope for M48
- explicit statement that Lane B, extraction, and backend follow-ons remain separate decisions

Acceptance:

- the run can be audited from `baseline.json`, `authority-freeze.json`, `tasks.json`, `acceptance.md`, and `closeout.md` alone
- the closeout does not overclaim future consumer rewires or extraction readiness

## 7. Context-Control Rules

- The parent keeps only these items live in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `tasks.json`
  - latest proof-floor summary
  - current seam diff summary
- Each worker prompt contains only:
  - its owned file set
  - exact relevant `PLAN.md` excerpts
  - allowed commands
  - forbidden touch surfaces
  - parent-captured proof outputs
- Workers return only:
  - files reviewed
  - commands run and exit codes
  - bounded findings
  - blocker notes
- Workers do not write canonical authoritative run artifacts. `drafts/` is the only shared exception.
- The parent reviews worker summaries and narrow diffs only. Full worker transcripts do not become part of the main execution context.
- Close each worker immediately after its draft is consumed.
- Prefer sentinels, explicit handoff files, or long waits over tight polling loops.

## 8. Tests And Acceptance

### 8.1 Required command gates

Kickoff and final proof gates both run:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Required final outcomes:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `decision_basis_code = no_actionable_candidate`
- `required_next_action = record_stop_without_new_milestone`
- `overall_verdict = pass`
- all `xtask` tests green

### 8.2 Source-level acceptance

- `xtask/src/family/analysis_core/mod.rs`
  - export inventory is explicit, grouped, and unchanged in meaning
- `xtask/src/family/analysis_core/helper_surface.rs`
  - contradictory inputs and malformed fingerprint inputs are explicitly proven
- `xtask/src/family/analysis_core/decision_contract.rs`
  - each real branch is explicitly proven
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
  - semantic drift changes fingerprints and bookkeeping churn does not

### 8.3 Downstream acceptance

- read-only consumers remain unchanged in semantics
- command-surface truth remains unchanged
- compatibility shims remain compatibility-only
- any compile-only exception is visibly documented and justified

## 9. Assumptions And Stop Conditions

### 9.1 Assumptions

- the kickoff branch remains `feat/m40-plus`
- the kickoff HEAD remains `0283db1be641d04374bceec313c85d230f98c1be`
- the validated proof floor from 2026-05-11 is still reproducible when M48 begins
- the existing seam files named in `PLAN.md` are still the true owner surfaces
- support workers are optional convenience, not required for correctness

### 9.2 Immediate stop conditions

- kickoff proof floor no longer matches the validated stop-state basis
- a downstream consumer or shim needs a semantic edit
- a new owner surface, helper layer, schema field, CLI change, or backend change appears necessary
- fingerprint normalization cannot stay truthful without widening scope
- the parent cannot explain an out-of-seam change as compile-only under the narrow `PLAN.md` exception
- unexpected dirtiness appears in the worktree and cannot be attributed safely
- external changes land during the run and invalidate the frozen baseline

### 9.3 Completion statement

M48 is complete only when the seam is frozen, the proof wall is green, the downstream stop-state truth is unchanged, and the run artifacts make that claim auditable without reading any worker transcript.
