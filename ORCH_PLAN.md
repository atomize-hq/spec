# M27.9B Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Primary branch baseline: **`feat/corpus-expansion`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b`**  
Last rewritten: **2026-05-02**

## Summary

This file is the execution contract for completing M27.9B from the current
truthful repo baseline to a final integrated green proof.

M27.9B resolves exactly one remaining ambiguity in the
recommendation/governance layer:

- `money/round` remains visible pressure
- `money/round` is not the next family-promotion target
- the repo stops describing that cluster as generic unresolved overlap
- the repo instead encodes an explicit durable hold for helper-only pressure
  inside already-promoted arithmetic workflows

Required final outcome:

- `recommendation_status = "no_strong_candidate"`
- visible candidate id remains
  `z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0`
- cluster id remains `unsupported_function_surface-e40675da6fa0`
- `hold_reasons = ["helper_surface_not_promotable"]`
- `next_step_status = "durable_hold"`
- `next_step_detail = "helper_surface_not_promotable"`
- recommendation-analysis schema version bumps from `2` to `3`
- coverage leverage for the visible cluster remains `2 / 1 / 0 / 3`
- corpus run `1` remains unspent and unauthorized by default

The parent agent is the sole integrator and the sole owner of run-state truth.
Workers may edit only assigned authored files in assigned worktrees. The parent
alone owns baseline freeze, vocabulary freeze, integration, derived artifact
refresh, final proof, and closeout.

Repository root:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`

Integration branch:

- `feat/corpus-expansion`

Worker branches:

- `codex/m27-9b-ws1-xtask`
- `codex/m27-9b-ws2-governance`

Worker worktrees:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws1_xtask`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws2_governance`

Canonical run root:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b`

## Hard Guards

1. Authored file scope is closed. Only these six authored files may change:
   - `xtask/src/family/coverage.rs`
   - `xtask/src/family/recommend.rs`
   - `xtask/src/family/promotion_artifacts.rs`
   - `xtask/src/lib.rs`
   - `PLAN.md`
   - `docs/recommendation_corpus_expansion_program_v0.1.md`

2. Derived artifacts are parent-owned outputs only:
   - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
   - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

3. No source edits outside those six authored files are allowed. If this
   milestone appears to require edits in `spec-core`, `spec-cli`, examples,
   manifests, packet fixtures, or other docs, halt and re-plan.

4. No new command surfaces may be added. Reuse the existing command path:
   - `cargo xtask family coverage --format json`
   - `cargo xtask family recommend --format json`
   - `cargo xtask family validate-artifact ...`

5. No new artifact family may be added. The contract change lands inside the
   existing recommendation-analysis artifact.

6. No second analysis pass may be added. The durable-hold decision must be
   expressed through the existing coverage and recommendation path.

7. `RecommendationCandidateEntry` must gain required fields:
   - `next_step_status`
   - `next_step_detail`

8. Recommendation-analysis schema version must bump from `2` to `3`.

9. `money/round` remains a visible held candidate. This milestone does not hide
   or delete it.

10. Leverage and cluster invariants must hold:
    - cluster id remains `unsupported_function_surface-e40675da6fa0`
    - `real_example_hits = 2`
    - `promotion_relevant_regression_hits = 1`
    - `boundary_only_hits = 0`
    - `total_units_in_cluster = 3`

11. The top-level recommendation status must remain `no_strong_candidate`.

12. Corpus run `1` remains unspent and unauthorized by default in both
    code-driven artifacts and docs.

13. No M26 wrapper-family content may be imported. M27.9B is not a
    wrapper-family milestone.

14. `coverage.rs` is in scope but not mandatory to change. If current coverage
    truth already supports the durable-hold branch, leave it unchanged and
    record that explicitly in run-state.

## Parent-Owned Run-State Protocol

The canonical run root for this milestone is:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b`

The parent owns run-state truth. Workers may read run-state freely. Workers may
not write run-state except to explicitly assigned worker return paths under
`handoffs/` and `diagnostics/`.

Canonical run-state files:

- `.runs/m27_9b/tasks.json`
- `.runs/m27_9b/session-log.md`
- `.runs/m27_9b/baseline.json`
- `.runs/m27_9b/vocabulary-freeze.json`
- `.runs/m27_9b/integration-state.json`
- `.runs/m27_9b/final-proof.json`
- `.runs/m27_9b/blocked.json`
- `.runs/m27_9b/closeout.md`

Canonical run-state directories:

- `.runs/m27_9b/diagnostics/`
- `.runs/m27_9b/diagnostics/parent/`
- `.runs/m27_9b/diagnostics/ws1_xtask/`
- `.runs/m27_9b/diagnostics/ws2_docs/`
- `.runs/m27_9b/handoffs/`
- `.runs/m27_9b/handoffs/ws1_xtask/`
- `.runs/m27_9b/handoffs/ws2_docs/`

Run-state ownership rules:

- Parent writes and updates:
  - `tasks.json`
  - `session-log.md`
  - `baseline.json`
  - `vocabulary-freeze.json`
  - `integration-state.json`
  - `final-proof.json`
  - `blocked.json`
  - `closeout.md`
- Worker 1 may write only:
  - `.runs/m27_9b/handoffs/ws1_xtask/result.json`
  - `.runs/m27_9b/handoffs/ws1_xtask/handoff.md`
  - `.runs/m27_9b/handoffs/ws1_xtask/commit.txt`
  - `.runs/m27_9b/handoffs/ws1_xtask/done.ok`
  - `.runs/m27_9b/diagnostics/ws1_xtask/**`
- Worker 2 may write only:
  - `.runs/m27_9b/handoffs/ws2_docs/result.json`
  - `.runs/m27_9b/handoffs/ws2_docs/handoff.md`
  - `.runs/m27_9b/handoffs/ws2_docs/commit.txt`
  - `.runs/m27_9b/handoffs/ws2_docs/done.ok`
  - `.runs/m27_9b/diagnostics/ws2_docs/**`

`tasks.json` is the authoritative execution ledger. It tracks:

- `run_id`
- `milestone`
- `integration_branch`
- `status`
- `current_phase`
- `tasks[]`
- `worker_branches`
- `worker_worktrees`
- `blocking_reason`
- `updated_at`

Each task entry in `tasks.json` must include:

- `task_id`
- `phase`
- `owner`
- `status`
- `depends_on`
- `owned_paths`
- `deliverable`
- `start_gate`
- `acceptance`
- `halt_on`

Allowed task status values:

- `pending`
- `ready`
- `in_progress`
- `done`
- `blocked`
- `cancelled`

`session-log.md` is the parent-maintained factual journal. It records only
phase boundaries, key observations, worker dispatches, merge results, proof
results, and blockers. It is not a transcript sink.

`baseline.json` captures the pre-edit anchor truth. It must record at minimum:

- current branch
- current HEAD SHA
- coverage totals
- recommendation-analysis schema version
- top-level recommendation status
- visible candidate id
- visible cluster id
- current hold reasons
- leverage tuple
- timestamp

`vocabulary-freeze.json` is the parent-issued lexical contract that docs must
mirror exactly. It must contain:

- `hold_reason`
- `next_step_status`
- `next_step_detail`
- `recommendation_analysis_schema_version`
- `frozen_from_commit`
- `frozen_at`

`integration-state.json` tracks parent integration state. It must include:

- integrated worker commits
- pending worker commits
- integrated files by worker
- whether derived artifacts have been refreshed
- whether final proof has run
- whether repo is blocked

`final-proof.json` is the authoritative acceptance artifact. It must capture:

- command list
- exit codes
- resulting artifact paths
- resulting artifact SHA-256s
- recommendation-analysis schema version
- top-level recommendation status
- visible candidate excerpt
- leverage excerpt
- docs alignment booleans
- final verdict

`diagnostics/**` stores raw command outputs and lightweight extracted
assertions. It is for evidence, not decision authority.

`blocked.json` is created only if the run cannot reach green. It must contain:

- `phase`
- `task_id`
- `blocking_condition`
- `repo_truth_observed`
- `safe_next_actions`
- `resume_requirements`

## Execution Graph

The execution graph for M27.9B is:

```text
WS-0 KICKOFF
    |
    v
WS-1 BASELINE FREEZE
    |
    +------------------------------+
    |                              |
    v                              v
WS-A XTASK LANE                WS-B DOCS PRE-DRAFT
    |                              |
    |                              |
    v                              |
WS-A VOCABULARY PROVEN -----------+
    |
    v
PARENT VOCABULARY FREEZE
    |
    v
WS-B DOCS FINALIZE
    |
    v
WS-INT INTEGRATION
    |
    v
WS-F FINAL PROOF
    |
    v
WS-C CLOSEOUT
```

Dependency rules:

- `WS-0` must complete before any worker starts.
- `WS-1` baseline freeze must complete before any worker receives a task packet.
- `WS-A` and `WS-B pre-draft` may run in parallel after baseline freeze.
- `WS-B` may pre-draft after baseline freeze because it can identify edit sites
  and prepare bounded language from current repo truth.
- `WS-B` may not finalize or submit merge-ready doc wording until
  `vocabulary-freeze.json` exists.
- `WS-INT` starts only after WS-A is complete and WS-B is complete.
- `WS-F` starts only after parent integration is complete.
- `WS-C` starts only after final proof is green or an unrecoverable blocker is
  recorded.

## Workstream Plan

## WS-0 Kickoff

Task IDs:

- `M27.9B-00-KICKOFF`
- `M27.9B-00-RUNSTATE-INIT`

Owned paths:

- `.runs/m27_9b/**`
- no authored source files

Start gate:

- parent is on `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- branch is `feat/corpus-expansion`
- no worker worktrees for this run already exist, or stale ones are explicitly
  removed

Required actions:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/{diagnostics/parent,diagnostics/ws1_xtask,diagnostics/ws2_docs,handoffs/ws1_xtask,handoffs/ws2_docs}
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse --abbrev-ref HEAD > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/current-branch.txt
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/current-head.txt
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec status --short > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/git-status.txt
```

Parent writes initial run-state:

- `tasks.json` with all tasks seeded as `pending`
- `session-log.md` with kickoff timestamp and branch/HEAD
- `integration-state.json` with `status = "not_started"`

Acceptance:

- `.runs/m27_9b/` exists with required files/directories
- `tasks.json` and `session-log.md` exist
- current branch is recorded
- parent has confirmed the intended integration branch is
  `feat/corpus-expansion`

Blocked path / halt conditions:

- current branch is not `feat/corpus-expansion`
- repo state is unexpectedly dirty in any of the six authored files or derived
  analysis artifacts and parent cannot explain it
- stale run-state from an old M27.9B attempt exists and cannot be safely
  superseded

## WS-1 Baseline Freeze

Task IDs:

- `M27.9B-01-BASELINE-COVERAGE`
- `M27.9B-02-BASELINE-RECOMMEND`
- `M27.9B-03-BASELINE-ASSERT`
- `M27.9B-04-WORKTREE-CREATE`

Owned paths:

- `.runs/m27_9b/**`
- worker worktree roots
- no authored source files

Start gate:

- WS-0 complete
- parent run-state initialized

Required actions:

```bash
cargo xtask family coverage --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.coverage.stdout.json
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.coverage.latest.json
cargo xtask family recommend --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.recommend.stdout.json
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.validate.coverage.txt
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.validate.recommendation.txt
rg -n '\"schema_version\": 2|\"recommendation_status\": \"no_strong_candidate\"|unsupported_function_surface-e40675da6fa0|\"unknown_overlap_family\"' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.recommendation.assertions.txt
rg -n '\"total_units\": 28|\"promoted_family_units\": 17|\"supported_unpromoted_family_units\": 0|\"unsupported_function_units\": 11|unsupported_function_surface-e40675da6fa0' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.coverage.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/baseline.coverage.assertions.txt
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec worktree add -b codex/m27-9b-ws1-xtask /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws1_xtask feat/corpus-expansion
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec worktree add -b codex/m27-9b-ws2-governance /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws2_governance feat/corpus-expansion
```

Parent writes `baseline.json` with:

- branch and HEAD
- coverage tuple `28 / 17 / 0 / 11`
- recommendation-analysis schema version `2`
- top-level `no_strong_candidate`
- visible candidate id
- visible cluster id
- `hold_reasons = ["unknown_overlap_family"]`
- leverage `2 / 1 / 0 / 3`

Acceptance:

- baseline commands are green
- current repo truth matches the locked M27.9A baseline
- both worker worktrees exist
- `baseline.json` is written
- `tasks.json` marks WS-1 done and WS-A / WS-B as `ready`

Blocked path / halt conditions:

- coverage totals differ from `28 / 17 / 0 / 11`
- top-level recommendation status differs from `no_strong_candidate`
- visible candidate or cluster id differs
- current hold reason is not `unknown_overlap_family`
- baseline artifact validation fails
- worker worktree creation fails

## WS-A Xtask Lane

Task IDs:

- `M27.9B-A1-COVERAGE-DECISION`
- `M27.9B-A2-RECOMMEND-CONTRACT`
- `M27.9B-A3-ARTIFACT-SCHEMA`
- `M27.9B-A4-LOCK-TESTS`
- `M27.9B-A5-LOCAL-PROOF`
- `M27.9B-A6-HANDOFF`

Owned paths:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`

Start gate:

- WS-1 baseline freeze complete
- worker worktree exists
- worker packet has been delivered by parent

Worker 1 packet must contain exactly:

- repo root and worktree path
- integration branch and worker branch
- `baseline.json`
- current `PLAN.md`
- current recommendation and coverage artifacts
- allowed file list
- forbidden file rule
- target outcome excerpt
- required proof loop
- worker return contract
- instruction to keep commits clean and bounded to owned files

Required actions:

- inspect current coverage output and current `coverage.rs`
- decide whether `coverage.rs` needs a code change or can remain unchanged
- implement durable-hold resolution in `recommend.rs`
- extend artifact vocabulary and validation in `promotion_artifacts.rs`
- bump recommendation-analysis schema version from `2` to `3`
- update locked command-path tests in `xtask/src/lib.rs`
- run local proof in the worker worktree
- produce one or more clean commits limited to owned files
- write the worker return package

Worker 1 required local commands:

```bash
cargo test -p xtask -- --color never > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/cargo-test.txt
cargo xtask family coverage --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/coverage.stdout.json
cargo xtask family recommend --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/recommendation.stdout.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/validate.coverage.txt
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/validate.recommendation.txt
rg -n '\"schema_version\": 3|\"recommendation_status\": \"no_strong_candidate\"|unsupported_function_surface-e40675da6fa0|\"helper_surface_not_promotable\"|\"durable_hold\"' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/ws1_xtask/recommendation.assertions.txt
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws1_xtask rev-parse HEAD > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/handoffs/ws1_xtask/commit.txt
```

Worker 1 return package must contain:

- `handoffs/ws1_xtask/result.json`
- `handoffs/ws1_xtask/handoff.md`
- `handoffs/ws1_xtask/commit.txt`
- `handoffs/ws1_xtask/done.ok`

`result.json` must contain:

- `status`
- `head_commit`
- `files_changed`
- `coverage_rs_changed`
- `recommendation_analysis_schema_version`
- `candidate_excerpt`
- `proof_commands`
- `proof_status`
- `blockers`

`handoff.md` must be short and bounded. It must include:

- whether `coverage.rs` changed
- why the durable-hold logic is truthful
- what tests were added/updated
- exact files changed
- exact command results
- any unresolved risk

Acceptance:

- worker proof is green
- returned files list is limited to the four owned `xtask` files
- returned candidate excerpt matches the required durable-hold target
- recommendation-analysis schema version is `3`
- no source edits outside the WS-A owned paths are present

Blocked path / halt conditions:

- worker concludes that durable-hold cannot be expressed without edits outside
  the four owned files
- worker proof changes cluster id or leverage tuple
- worker proof changes top-level recommendation status
- worker introduces new recommendation or coverage commands
- worker modifies docs or derived JSON as authored deliverables

## WS-B Docs Lane

Task IDs:

- `M27.9B-B1-PREDRAFT`
- `M27.9B-B2-WAIT-FOR-VOCABULARY`
- `M27.9B-B3-FINALIZE`
- `M27.9B-B4-HANDOFF`

Owned paths:

- `PLAN.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Start gate:

- WS-1 baseline freeze complete
- worker worktree exists
- worker packet has been delivered by parent

Worker 2 packet must contain exactly:

- repo root and worktree path
- integration branch and worker branch
- `baseline.json`
- current `PLAN.md`
- current program tracker doc
- current recommendation and coverage artifacts
- allowed file list
- forbidden file rule
- explicit note that pre-draft may start now but final wording must wait for
  `vocabulary-freeze.json`
- worker return contract

Safe parallelization rule for WS-B:

- WS-B may pre-draft immediately after baseline freeze
- WS-B may identify exact edit sites, structure replacement text, and stage
  non-terminology edits
- WS-B may not finalize the exact durable-hold wording, produce its final
  commit, or mark itself done until `vocabulary-freeze.json` exists

Required actions before vocabulary freeze:

- inspect current `PLAN.md`, program tracker, and baseline artifacts
- identify every location where `money/round` is still framed as unresolved
  overlap or corpus-program pressure
- pre-draft bounded replacements that preserve current facts but leave frozen
  terminology slots to be filled later

Required actions after vocabulary freeze:

- finalize wording using the exact values from `vocabulary-freeze.json`
- ensure both docs say:
  - `money/round` remains visible
  - it is not the next family
  - corpus run `1` remains unspent
  - corpus run `1` remains unauthorized by default
- produce one clean commit limited to the two owned doc files
- write the worker return package

Worker 2 return package must contain:

- `handoffs/ws2_docs/result.json`
- `handoffs/ws2_docs/handoff.md`
- `handoffs/ws2_docs/commit.txt`
- `handoffs/ws2_docs/done.ok`

`result.json` must contain:

- `status`
- `head_commit`
- `files_changed`
- `vocabulary_freeze_consumed`
- `money_round_language_aligned`
- `corpus_run_1_guard_present`
- `blockers`

Acceptance:

- only `PLAN.md` and `docs/recommendation_corpus_expansion_program_v0.1.md`
  changed
- wording matches `vocabulary-freeze.json` exactly where terminology is
  contractual
- docs do not authorize another corpus run by default
- docs do not invent new roadmap gates, M26 content, or wrapper-family
  promotion content

Blocked path / halt conditions:

- docs cannot be aligned without changing the intended durable-hold vocabulary
- docs require changes outside the two owned files
- docs imply another automatic corpus-expansion run for `money/round`
- docs drift into M28 planning beyond the narrow M27.9B outcome

## WS-INT Integration

Task IDs:

- `M27.9B-INT1-REVIEW-WSA`
- `M27.9B-INT2-FREEZE-VOCAB`
- `M27.9B-INT3-REVIEW-WSB`
- `M27.9B-INT4-INTEGRATE-WSA`
- `M27.9B-INT5-INTEGRATE-WSB`
- `M27.9B-INT6-INTEGRATION-CHECK`

Owned paths:

- integration branch
- `.runs/m27_9b/**`
- parent may touch only integrated copies of worker-owned authored files

Start gate:

- WS-A handoff is complete
- WS-B handoff is complete or at least ready to finalize once vocabulary freeze
  is issued

Parent review mechanics:

- parent reviews `result.json`, `handoff.md`, `commit.txt`, and concise
  diagnostics files
- parent does not ingest giant worker transcripts
- parent reads only the bounded files needed to validate ownership, outputs,
  and proof
- parent waits on `done.ok` sentinels or explicit worker completion messages
- parent does not tight-poll worker worktrees

Vocabulary freeze step:

- after WS-A review passes, parent writes `vocabulary-freeze.json`
- WS-B must confirm it consumed that file before final docs handoff is accepted

Preferred integration path:

- first choice: cherry-pick clean worker commit(s) from each worker branch
- fallback: bounded patch-copy by owned files only if cherry-pick is noisy but
  file ownership is still clean
- parent does not merge whole branches blindly

WS-A integration commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec cherry-pick <ws1_commit_sha>
```

WS-B integration commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec cherry-pick <ws2_commit_sha>
```

Fallback bounded patch-copy path if cherry-pick is not clean but ownership is
still valid:

- copy only the worker-owned file versions from the worker worktree into the
  integration branch checkout
- verify file list again
- commit parent-authored integration commit with a bounded message

Integration-state updates required:

- parent writes `vocabulary-freeze.json`
- parent updates `integration-state.json` after WS-A integration
- parent updates `integration-state.json` after WS-B integration
- parent records integrated commit SHAs and exact integrated files

Acceptance:

- WS-A integrated cleanly
- `vocabulary-freeze.json` exists
- WS-B integrated cleanly
- exactly the six authored files changed on the integration branch before
  derived refresh
- `integration-state.json` says all worker changes are integrated and final
  proof is pending

Blocked path / halt conditions:

- conflict inside `xtask/src/family/recommend.rs` or
  `xtask/src/family/promotion_artifacts.rs`
- worker commit touches files outside owned paths
- WS-B wording does not match `vocabulary-freeze.json`
- parent cannot explain a diff in any unexpected file
- cherry-pick conflict in docs is tolerated only if it stays within `PLAN.md`
  and the tracker doc and parent resolves it without changing frozen
  vocabulary
- cherry-pick conflict in WS-A contract surfaces is a hard halt and requires
  re-dispatch or manual bounded re-integration

## WS-F Final Proof

Task IDs:

- `M27.9B-F1-CARGO-TEST`
- `M27.9B-F2-COVERAGE-REFRESH`
- `M27.9B-F3-RECOMMEND-REFRESH`
- `M27.9B-F4-ARTIFACT-VALIDATE`
- `M27.9B-F5-ASSERT-OUTPUT`
- `M27.9B-F6-WRITE-FINAL-PROOF`

Owned paths:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.runs/m27_9b/**`

Start gate:

- WS-INT complete
- integration branch contains final authored changes only

Required actions:

```bash
cargo test -p xtask -- --color never > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.cargo-test.txt
cargo xtask family coverage --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.coverage.stdout.json
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.coverage.latest.json
cargo xtask family recommend --format json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.recommend.stdout.json
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.validate.coverage.txt
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.validate.recommendation.txt
rg -n '\"schema_version\": 3|\"recommendation_status\": \"no_strong_candidate\"|unsupported_function_surface-e40675da6fa0|\"helper_surface_not_promotable\"|\"durable_hold\"' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.recommendation.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.recommendation.assertions.txt
rg -n '\"total_units\": 28|\"promoted_family_units\": 17|\"supported_unpromoted_family_units\": 0|\"unsupported_function_units\": 11|unsupported_function_surface-e40675da6fa0' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.coverage.latest.json > /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_9b/diagnostics/parent/final.coverage.assertions.txt
```

Parent writes `final-proof.json` with:

- all proof commands and exit codes
- artifact paths and SHA-256s
- schema version `3`
- recommendation status `no_strong_candidate`
- visible candidate excerpt
- leverage excerpt
- doc alignment checks
- verdict `green` or `blocked`

Acceptance:

- `cargo test -p xtask -- --color never` passes
- coverage refresh succeeds
- recommendation refresh succeeds
- both analysis artifacts validate
- recommendation-analysis artifact shows the exact durable-hold target
- coverage tuple remains `28 / 17 / 0 / 11`
- visible cluster leverage remains `2 / 1 / 0 / 3`
- docs align with the same durable-hold story
- no authored file outside the six-file scope changed

Blocked path / halt conditions:

- any proof command fails
- recommendation-analysis schema version is not `3`
- top-level status changes away from `no_strong_candidate`
- visible candidate disappears
- hold reason remains `unknown_overlap_family`
- `next_step_status` or `next_step_detail` is missing or wrong
- leverage tuple changes
- docs still leave corpus run `1` implicitly or explicitly authorized

## WS-C Final Closeout

Task IDs:

- `M27.9B-C1-GREEN-CLOSEOUT`
- `M27.9B-C2-BLOCKED-CLOSEOUT`
- `M27.9B-C3-WORKTREE-CLEANUP`

Owned paths:

- `.runs/m27_9b/**`
- worker worktrees and worker branches
- no additional authored source files

Start gate:

- WS-F complete with either a green verdict or a blocked verdict

Green closeout actions:

- write `final-proof.json` with `verdict = "green"`
- update `tasks.json` so all tasks are `done`
- update `integration-state.json` so `final_proof = "green"`
- append a final entry to `session-log.md`
- write `closeout.md` summarizing:
  - integrated worker commits
  - authored files changed
  - derived artifacts refreshed
  - proof commands run
  - acceptance outcome
  - note that corpus run `1` remains unspent and unauthorized by default
- optionally remove worker worktrees and delete local worker branches after
  verifying no unmerged work remains

Suggested cleanup commands after green:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec worktree remove /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws1_xtask
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec worktree remove /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m27_9b_ws2_governance
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec branch -d codex/m27-9b-ws1-xtask
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec branch -d codex/m27-9b-ws2-governance
```

Blocked closeout actions:

- write `blocked.json`
- update `tasks.json` with the first blocking task as `blocked`
- update `integration-state.json` with `status = "blocked"`
- append the blocker and safe next actions to `session-log.md`
- leave worker worktrees in place unless parent intentionally snapshots and
  discards them

Acceptance:

- green path leaves a complete run-state record and optional cleanup
- blocked path leaves enough structured state to resume without re-reading long
  transcripts

Blocked path / halt conditions:

- worker worktree cleanup fails
- parent cannot safely distinguish green from blocked because run-state is
  incomplete
- final verdict is ambiguous

## Safe Parallelization Rules

The safe concurrency in M27.9B is narrow but real.

What can run in parallel after baseline freeze:

- WS-A can start immediately on the four `xtask` files
- WS-B can start immediately on doc inspection and pre-draft because it can
  use the current `PLAN.md`, current artifacts, and baseline facts to locate
  required edits and prepare bounded replacements

Why that concurrency is safe:

- WS-A owns the contractual vocabulary and schema change
- WS-B owns only doc narration
- the two lanes do not share files
- pre-draft doc work does not change artifact truth or integration state
- the only semantic coupling is the exact frozen durable-hold vocabulary, which
  is solved by `vocabulary-freeze.json`

What may not run in parallel:

- no splitting of `recommend.rs` and `promotion_artifacts.rs` across workers
- no final doc commit before vocabulary freeze
- no final proof before parent integration
- no worker integration without parent review

Operational justification for not widening concurrency further:

- the `xtask` contract surfaces are one coupled unit because recommendation
  logic, schema validation, and locked tests must agree exactly
- extra worker splitting inside `xtask/` would add merge risk without
  shortening the critical path
- doc pre-draft concurrency is low risk because parent can reject or rebase
  wording cheaply once the vocabulary freeze lands

Worker waiting protocol:

- workers do not tight-poll for changes
- WS-B waits for `vocabulary-freeze.json` or an explicit parent message before
  finalizing
- parent waits for `done.ok` sentinels or explicit worker completion, not
  continuous transcript monitoring

## Subagent Handoff And Context-Control Rules

The parent controls worker context tightly.

General worker rules:

- each worker receives only the files, facts, and commands needed for its lane
- each worker works only inside its assigned worktree
- each worker returns only a bounded handoff package
- each worker is considered closed after merge; the parent does not keep it
  live for speculative follow-up

WS-A handoff payload:

- `baseline.json`
- current `PLAN.md`
- current recommendation and coverage artifacts
- target durable-hold outcome excerpt
- allowed file list
- forbidden file list
- proof loop commands
- required return package schema

WS-B handoff payload:

- `baseline.json`
- current `PLAN.md`
- current program tracker doc
- current recommendation and coverage artifacts
- note that `vocabulary-freeze.json` will be authoritative for final
  terminology
- allowed file list
- forbidden file list
- required return package schema

Worker return contract:

- one clean commit or a very small bounded set of commits
- `result.json`
- `handoff.md`
- `commit.txt`
- `done.ok`
- raw diagnostics only under assigned `diagnostics/` subtree

Parent review contract:

- read `result.json` first
- read `handoff.md` second
- inspect only the raw diagnostics needed to confirm the worker’s claims
- inspect the worker diff only for owned files
- do not ingest giant transcripts
- do not mine the worker conversation for state that should have been in the
  return package

Completion sentinels:

- WS-A completion sentinel: `.runs/m27_9b/handoffs/ws1_xtask/done.ok`
- WS-B completion sentinel: `.runs/m27_9b/handoffs/ws2_docs/done.ok`

Worker shutdown rule:

- after a worker’s commit is integrated or rejected, the parent records the
  disposition in `integration-state.json`
- after green closeout, the parent removes the worker worktree and optionally
  deletes the local worker branch
- the parent does not reopen a merged worker unless a new task is explicitly
  issued

## Tests And Acceptance

Integrated branch proof loop:

```bash
cargo test -p xtask -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

The run is accepted only if all of the following are true together:

1. `cargo test -p xtask -- --color never` passes.

2. Coverage still proves:
   - `total_units = 28`
   - `promoted_family_units = 17`
   - `supported_unpromoted_family_units = 0`
   - `unsupported_function_units = 11`

3. Recommendation-analysis proves:
   - `schema_version = 3`
   - `recommendation_status = "no_strong_candidate"`
   - visible candidate id remains
     `z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0`
   - `cluster_ids = ["unsupported_function_surface-e40675da6fa0"]`
   - `promotion_readiness = "hold"`
   - `hold_reasons = ["helper_surface_not_promotable"]`
   - `next_step_status = "durable_hold"`
   - `next_step_detail = "helper_surface_not_promotable"`

4. Visible candidate leverage remains:
   - `real_example_hits = 2`
   - `promotion_relevant_regression_hits = 1`
   - `boundary_only_hits = 0`
   - `total_units_in_cluster = 3`

5. Artifact schema validation rejects contradictory combinations, including:
   - durable hold without hold readiness
   - helper-surface-not-promotable without durable hold
   - durable-hold candidate under top-level `ranked`
   - stale schema version pretending to satisfy the new required fields

6. Locked command-path tests in `xtask/src/lib.rs` encode the new truth end to
   end.

7. `PLAN.md` and `docs/recommendation_corpus_expansion_program_v0.1.md` both
   say:
   - `money/round` remains visible
   - `money/round` is not the next family
   - corpus run `1` remains unspent
   - corpus run `1` remains unauthorized by default

8. No authored file outside the six-file scope changed.

Mandatory halt conditions at any point:

- baseline truth diverges from M27.9A locked expectations
- worker output escapes owned paths
- parent sees an unexpected diff in any non-owned file
- recommendation status stops being `no_strong_candidate`
- cluster id or leverage changes
- docs authorize another corpus run by default
- final proof is not fully green

## Assumptions

- `feat/corpus-expansion` already contains the truthful M27.9A baseline that
  this milestone builds on.
- The existing `xtask` coverage and recommendation commands run successfully
  from the repo root.
- The current recommendation-analysis artifact schema is `2`, so M27.9B must
  move it to `3`.
- Worktrees under `.worktrees/` and run-state under `.runs/m27_9b/` are
  acceptable orchestration surfaces for this repo.
- The parent is the sole integrator and may use either clean cherry-pick or
  bounded patch-copy by owned files, but not broad branch merges.
- No external approval gate is required unless repo truth diverges from the
  M27.9B contract.
- If the milestone blocks, the resume source of truth is the run-state under
  `.runs/m27_9b/`, not worker chat transcripts.
