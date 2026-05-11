# M49 Reusable Seam Semantic-Review Substrate Slice 1 Orchestration Plan

Status: **authoritative orchestration plan for executing M49**  
Supersedes: **the stale M48 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M49: Reusable Seam Semantic-Review Substrate, Slice 1 Implementation Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Kickoff tree expectation: **clean**  
Primary write scope: **`spec-core/src/semantic_review.rs`**  
Proof-wall surfaces: **`spec-core/src/export.rs`**, **`spec-core/src/typescript_backend.rs`**, **`spec-cli/src/commands.rs`**, **`spec-cli/tests/cli.rs`**  
Canonical M49 run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m49_reusable_seam_semantic_review_substrate_slice1/`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum concurrency after freeze: **2 workers**  
Last rewritten: **2026-05-11**

## Summary

- Execute from the repo-root authority lane at `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on branch `feat/m40-plus`.
- Keep the serialized critical path local to the parent agent:
  1. baseline freeze
  2. authority freeze
  3. WS-CONTRACT semantic contract work in `spec-core/src/semantic_review.rs`
  4. contract freeze gate
  5. WS-INT integration
  6. final validation and closeout
- Parallelism is allowed only after the contract freeze gate is written and green. The only honest post-freeze parallel work is:
  - WS-PROOF-CORE on `spec-core/src/export.rs` and `spec-core/src/typescript_backend.rs`
  - WS-PROOF-CLI on `spec-cli/src/commands.rs` and `spec-cli/tests/cli.rs`
- There are no human approval gates in M49. The contract freeze is the only orchestration gate before parallelism.
- The parent agent remains the only integrator and the only writer of orchestration state under `.runs/m49_reusable_seam_semantic_review_substrate_slice1/`.
- Use these concrete lanes, paths, and branches:
  - Parent authority lane: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`
  - WS-PROOF-CORE worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/proof-core` on `ws/m49-proof-core`
  - WS-PROOF-CLI worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/proof-cli` on `ws/m49-proof-cli`
  - WS-INT integration worktree: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/int` on `ws/m49-int`
- Primary planning and proof inputs are:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
  - `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md`

## Hard Guards

- `PLAN.md` is the sole scope authority for M49 execution.
- Kickoff must start from a clean worktree. If `git status --short` is not clean, stop before any source edit.
- Execute on `feat/m40-plus`. If the run begins on another branch, stop and record the divergence instead of improvising a new branch topology.
- Parent-owned production edit surface before parallelism is only `spec-core/src/semantic_review.rs`.
- Lane ownership is strict:
  - WS-CONTRACT owns `spec-core/src/semantic_review.rs`
  - WS-PROOF-CORE owns `spec-core/src/export.rs` and `spec-core/src/typescript_backend.rs`
  - WS-PROOF-CLI owns `spec-cli/src/commands.rs` and `spec-cli/tests/cli.rs`
  - WS-INT owns orchestration artifacts, merge mechanics, and validation captures only
- No worker may edit `spec-core/src/semantic_review.rs`.
- No worker may begin before `contract-freeze.json` exists and records `contract_freeze_commit`.
- The contract freeze gate must lock all six `PLAN.md` decisions:
  - `SupportedSeamFamily` variant names
  - canonical keys `sum.discount_strategy.v1` and `data.pricing_quote.v1`
  - legacy keys `sum.discount_policy.v1` and `data.checkout_quote.v1`
  - preserve matching policy: canonical-or-legacy only for the matching family
  - refresh policy: canonical key only
  - near-miss policy: renamed vocabulary stays unsupported
- M49 does not authorize:
  - `xtask/**`
  - schema changes
  - export JSON contract redesign
  - new CLI flags
  - new crates
  - new workspace members
  - TypeScript product-scope expansion
  - new abstraction layers, registries, or module splits beyond this slice
- The proof-wall surfaces are proof walls, not scope expansion lanes. Prefer tests there. Production edits are allowed only if needed to preserve truthful behavior already required by `PLAN.md`.
- If any lane requires edits outside the five in-scope repo files, stop and re-scope.
- `PLAN.md` and `ORCH_PLAN.md` are authority inputs during execution. They are not runtime deliverables for the run itself.

## Worktree And Branch Inventory

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m49-parent-authority` | `WS-CONTRACT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | baseline, authority freeze, semantic contract work, contract freeze gate |
| `lane/m49-proof-core` | `WS-PROOF-CORE` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/proof-core` | `ws/m49-proof-core` | Worker | export and TypeScript proof-wall work after freeze |
| `lane/m49-proof-cli` | `WS-PROOF-CLI` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/proof-cli` | `ws/m49-proof-cli` | Worker | CLI proof-wall work after freeze |
| `lane/m49-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/int` | `ws/m49-int` | Parent | merge worker lanes, run integrated proof, prepare validated result |
| `lane/m49-parent-closeout` | `finalize` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | fast-forward or merge validated integration result, final closeout captures |

## Canonical Orchestration State

All authoritative M49 run state lives under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m49_reusable_seam_semantic_review_substrate_slice1/`

Canonical file-of-record surfaces:

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, and worktree snapshot | Parent |
| `authority-freeze.json` | frozen scope, lane ownership, and command contract | Parent |
| `contract-freeze.json` | frozen seam contract and worker fork basis | Parent |
| `worktrees.json` | worktree path and branch inventory | Parent |
| `in-scope-files.txt` | exact writable repo surfaces | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | lane queue and dependency state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance.md` | final proof and completion ledger | Parent |
| `blocked.json` | blocker artifact on incomplete termination | Parent |
| `authority-snapshot/PLAN.md` | kickoff authority snapshot | Parent |
| `authority-snapshot/ORCH_PLAN.md` | kickoff orchestration snapshot | Parent |
| `validation/kickoff/*` | baseline command captures | Parent |
| `validation/ws-contract/*` | semantic-review and freeze captures | Parent |
| `validation/ws-proof-core/*` | worker proof-core captures copied into authority root | Parent |
| `validation/ws-proof-cli/*` | worker proof-cli captures copied into authority root | Parent |
| `validation/ws-int/*` | integration merge and proof captures | Parent |
| `validation/final/*` | final branch-state and acceptance captures | Parent |

## Per-Task Sentinel Directories

Every orchestration task has a sentinel root under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m49_reusable_seam_semantic_review_substrate_slice1/tasks/<TASK_ID>/`

Each sentinel directory must contain at least:

| File | Required contents |
| --- | --- |
| `sentinel.json` | `task_id`, `workstream`, `lane`, `owner`, `owned_files`, `status`, `started_at`, `completed_at`, `blocker_status`, `last_command_status`, `notes_summary` |
| `commands.ndjson` | one record per command with `command`, `cwd`, `started_at`, `completed_at`, `exit_code`, `stdout_path`, `stderr_path` |
| `owned-files.txt` | exact owned repo files for that task |
| `result.md` | concise parent-authored summary of outcome, blockers, and next action |

Rules for task sentinels:

- Only the parent writes authoritative sentinel files under `.runs/m49.../tasks/**`.
- Workers may report command results and changed files, but the parent records them into the sentinel directories.
- A task is not complete until its sentinel has `status: done` and `completed_at`.
- A blocked task must set `blocker_status: blocked` and name the exact file or contract leak that stopped progress.

## File-Of-Record Details

`baseline.json` must record:

- `run_id`
- `repo_root`
- `working_branch`
- `kickoff_timestamp`
- `git_status_short`
- `kickoff_commit`
- `kickoff_commit_short`
- `plan_path`
- `orch_plan_path`
- `primary_write_scope`
- `proof_wall_surfaces`
- `test_plan_input`

`authority-freeze.json` must record:

- statement that the parent owns the serialized critical path
- exact lane ownership
- maximum concurrency `2`
- explicit statement that no worker starts before contract freeze
- exact targeted proof commands
- exact conditional broader proof commands
- exact stop rules for cross-lane scope leaks

`contract-freeze.json` must record:

- `run_id`
- `contract_freeze_commit`
- `contract_freeze_commit_short`
- `contract_freeze_branch`
- `supported_seam_family_variants`
- `canonical_keys`
- `legacy_keys`
- `preserve_matching_policy`
- `refresh_policy`
- `near_miss_policy`
- `lane_b_branch`
- `lane_b_path`
- `lane_c_branch`
- `lane_c_path`
- `lane_int_branch`
- `lane_int_path`
- `targeted_commands_green`
- `freeze_summary`

`tasks.json` must record per task:

- `id`
- `workstream`
- `lane`
- `owner`
- `status`
- `depends_on`
- `owned_files`
- `required_commands`
- `writes`
- `started_at`
- `completed_at`
- `notes`

Allowed `tasks.json` statuses are:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `cancelled`

## Workstream Plan

### WS-CONTRACT (`lane/m49-parent-authority`, parent only, sequential)

Purpose: freeze authority, implement the semantic-review contract, and create the only gate before parallelism.

Tasks in WS-CONTRACT:

1. `task-m49-contract-baseline-freeze`
2. `task-m49-contract-authority-freeze`
3. `task-m49-contract-semantic-contract`
4. `task-m49-contract-freeze-gate`

Owned files:

- `spec-core/src/semantic_review.rs`
- all `.runs/m49_reusable_seam_semantic_review_substrate_slice1/**` orchestration artifacts

Required commands:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
git rev-parse --short=7 HEAD
cargo test -p spec-core semantic_review
```

Required work:

- snapshot kickoff `PLAN.md` and `ORCH_PLAN.md`
- write `baseline.json`, `authority-freeze.json`, `worktrees.json`, `in-scope-files.txt`, `out-of-scope-files.txt`, `tasks.json`, and `queue.json`
- implement the M49 contract in `spec-core/src/semantic_review.rs`
- add the semantic-review proofs required by `PLAN.md`
- create the contract-freeze checkpoint commit
- create the worker branches and worker worktrees from the exact freeze commit
- create the integration worktree from the exact freeze commit

WS-CONTRACT acceptance:

- kickoff worktree is clean and branch is `feat/m40-plus`
- `spec-core/src/semantic_review.rs` is the only production edit surface touched before parallelism
- the six contract-freeze decisions are fully locked in code and recorded in `contract-freeze.json`
- `cargo test -p spec-core semantic_review` is green on the freeze commit
- the worker and integration worktrees all point at the exact `contract_freeze_commit`
- no worker has started before `contract-freeze.json` is written

WS-CONTRACT stop rules:

- if the worktree is dirty, stop before M49 begins
- if `cargo test -p spec-core semantic_review` is red, do not parallelize
- if any freeze decision is still moving, do not parallelize
- if creating the exact shared freeze basis fails, collapse the run back to parent-only sequential execution or stop

### WS-PROOF-CORE (`lane/m49-proof-core`, worker, post-freeze only)

Purpose: prove the export and TypeScript read-side truth walls after the semantic contract is frozen.

Task in WS-PROOF-CORE:

- `task-m49-proof-core`

Owned files:

- `spec-core/src/export.rs`
- `spec-core/src/typescript_backend.rs`

Required commands:

```bash
cargo test -p spec-core export
cargo test -p spec-core typescript_backend
```

Required work:

- prove export preserve accepts legacy seam keys only for the matching family during the migration window
- prove export preserve does not invent supported seam truth
- prove refreshed seam truth reads back canonically
- prove bounded TypeScript validation does not regress when family-routed seam support exists in context
- prefer tests over production logic changes
- keep behavior narrow and compatibility-preserving

WS-PROOF-CORE acceptance:

- only the two owned files are changed
- `cargo test -p spec-core export` is green
- `cargo test -p spec-core typescript_backend` is green
- any production logic change stays within existing behavior required by `PLAN.md`
- no canonical key, legacy key, or near-miss policy is redefined in this lane

WS-PROOF-CORE stop rules:

- if work requires `spec-core/src/semantic_review.rs`, stop this worker and return the blocker to the parent
- if work requires a CLI file, stop this worker and return the blocker to the parent
- if the fix implies schema or TypeScript product-scope changes, stop and re-scope

### WS-PROOF-CLI (`lane/m49-proof-cli`, worker, post-freeze only)

Purpose: prove status/export/test CLI truth surfaces against the frozen semantic contract.

Task in WS-PROOF-CLI:

- `task-m49-proof-cli`

Owned files:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Required commands:

```bash
cargo test -p spec-cli cli
```

Required work:

- prove `spec status --format json` preserve behavior for legacy seam passports
- prove `spec export` preserve behavior for legacy seam passports
- prove refresh rewrites seam semantic review keys canonically
- preserve stale and incomplete seam health semantics
- prefer `cli.rs` tests
- change `commands.rs` only if alias-aware preserve logic is duplicated there and must be fixed to remain truthful

WS-PROOF-CLI acceptance:

- only the two owned files are changed
- `cargo test -p spec-cli cli` is green
- CLI status/export preserve matrix covers canonical and legacy seam keys
- refresh behavior is canonical-only
- stale and incomplete seam semantics remain unchanged
- no CLI JSON contract or flag surface changes are introduced

WS-PROOF-CLI stop rules:

- if work requires `spec-core/src/semantic_review.rs`, stop this worker and return the blocker to the parent
- if work requires `spec-core/src/export.rs` or `spec-core/src/typescript_backend.rs`, stop this worker and return the blocker to the parent
- if the fix implies new CLI flags or JSON contract changes, stop and re-scope

### WS-INT (`lane/m49-int`, parent only)

Purpose: integrate the frozen worker lanes in a dedicated integration worktree, run the integrated proof set, and prepare the validated result for `feat/m40-plus`.

Task sequence in WS-INT:

1. `task-m49-int-merge-proof-core`
2. `task-m49-int-merge-proof-cli`
3. `task-m49-int-targeted-proof`
4. `task-m49-int-broader-proof-sweep` when required
5. `task-m49-int-promote-validated-result`

Owned files:

- merge state in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m49/int`
- `.runs/m49_reusable_seam_semantic_review_substrate_slice1/**`
- no net-new product-scope ownership beyond merge mechanics and validation

Merge order:

1. create `ws/m49-int` from `contract_freeze_commit`
2. merge `ws/m49-proof-core` into `ws/m49-int`
3. merge `ws/m49-proof-cli` into `ws/m49-int`
4. run targeted proof commands in the integration worktree
5. run broader proof commands if the trigger conditions are met
6. if green, fast-forward or merge the validated `ws/m49-int` result back onto `feat/m40-plus` from the repo-root authority lane
7. capture final status and acceptance on `feat/m40-plus`

Conflict rules:

- If the conflict is a straightforward merge mechanic in a lane-owned file and the frozen contract is not being reopened, the parent may resolve it in WS-INT.
- If the conflict or failing proof requires semantic changes in a lane-owned file and ownership is clear, bounce the change back to that lane owner and rerun that workstream from the frozen basis.
- If the conflict or failing proof requires reopening `spec-core/src/semantic_review.rs`, canonical keys, legacy keys, preserve policy, refresh policy, or near-miss policy, stop both worker lanes and collapse back to parent-owned serialized repair on `feat/m40-plus`.
- WS-INT does not invent new product scope. It integrates, validates, and either accepts, bounces, or collapses.

Required commands for targeted proof:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-core typescript_backend
cargo test -p spec-cli cli
```

Conditional broader commands when shared projection behavior was touched broadly:

```bash
cargo test -p spec-core
cargo test -p spec-cli
```

WS-INT acceptance:

- the integration worktree starts from the exact `contract_freeze_commit`
- both worker branches merge in the recorded order
- all four targeted proof commands are green on `ws/m49-int`
- the broader proof sweep runs and passes whenever triggered by `PLAN.md` conditions
- no proof-wall diff reopens WS-CONTRACT decisions
- the validated integration result is moved back onto `feat/m40-plus` without additional product-scope edits outside the five in-scope repo files

WS-INT stop rules:

- if merge conflict resolution requires creative semantic redesign, stop and bounce or collapse instead of improvising in WS-INT
- if targeted proof is red, do not promote the integration branch
- if broader proof is required and red, do not promote the integration branch
- if final promotion back to `feat/m40-plus` cannot preserve the validated commit content, stop and record a blocker

## Cross-Lane Scope Leak Policy

- If a worker needs `spec-core/src/semantic_review.rs`, parallelism is no longer honest. Stop that worker immediately. The parent either:
  - collapses back to parent-only serialized repair on `feat/m40-plus`, or
  - stops the run and records a blocker if reopening the freeze contract would exceed M49 scope
- If WS-PROOF-CORE needs a CLI-owned file, or WS-PROOF-CLI needs a proof-core-owned file, the worker must not cross the boundary. The parent either:
  - bounces the requested change to the owning worker if the frozen contract is unchanged, or
  - cancels both workers and moves remaining source work into parent-owned serialized completion if ownership is now coupled
- WS-INT may resolve only merge mechanics and validation fallout that do not change the frozen contract. WS-INT is not a hidden fourth implementation lane.
- Any scope leak into `xtask/**`, schema surfaces, or CLI contract redesign is an immediate stop and re-scope event.

## Worker Prompt And Return Contract

The parent must pass each worker only:

- owned files
- exact relevant `PLAN.md` excerpt
- exact frozen contract excerpt from `contract-freeze.json`
- required commands
- forbidden touch surfaces

Workers must return only:

- changed files
- commands run with exit codes
- blockers
- unresolved assumptions

Worker output rules:

- workers do not write `.runs/m49_reusable_seam_semantic_review_substrate_slice1/**`
- workers do not merge branches
- workers do not reinterpret scope
- workers do not return full transcripts as required context
- the parent reviews narrow diffs and narrow summaries only
- the parent does not ingest full worker transcripts into main context
- the parent records accepted worker outcomes in the task sentinel directories and `session-log.md`

## Tests And Acceptance

Primary proof input:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md`

Targeted proof commands required by `PLAN.md`:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-core typescript_backend
cargo test -p spec-cli cli
```

Conditional broader proof commands required by `PLAN.md` when shared projection behavior was touched broadly:

```bash
cargo test -p spec-core
cargo test -p spec-cli
```

M49 is complete only when all of the following are true:

- WS-CONTRACT froze the seam-family contract and recorded it in `contract-freeze.json`
- WS-PROOF-CORE proved export and TypeScript truth walls without widening behavior
- WS-PROOF-CLI proved CLI preserve/refresh truth surfaces without changing CLI contract shape
- WS-INT merged both worker lanes in the dedicated integration worktree and all targeted proof commands are green there
- the broader proof sweep ran and passed when triggered
- the validated integration result was promoted back onto `feat/m40-plus`
- the final diff stays within:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/export.rs`
  - `spec-core/src/typescript_backend.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `.runs/m49_reusable_seam_semantic_review_substrate_slice1/**`
- `acceptance.md` closes the exact M49 failure modes from `PLAN.md`:
  - unseen seam ids with supported shape route to supported families
  - legacy preserve alias migration is real
  - refresh canonicalization is real
  - wrapper dependency support remains intact
  - renamed-vocabulary near misses stay unsupported
  - TypeScript bounded-lane behavior is not regressed

## Assumptions

- `feat/m40-plus` is the live M49 execution branch and remains the authority branch for final closeout.
- A local checkpoint commit after WS-CONTRACT is required and acceptable so all post-freeze lanes fork from one frozen basis.
- Maximum honest concurrency for M49 is `2` workers. A third worker is not justified by the current `PLAN.md` scope.
- Proof-wall files may end up tests-only. If they require production edits, those edits remain compatibility-preserving and behaviorally narrow.
- No `xtask` changes are allowed or needed for M49.
- `.runs/m49_reusable_seam_semantic_review_substrate_slice1/**` is run-state and audit data, not authored product surface.
