# M28 Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Integration branch: **`feat/corpus-expansion`**  
Review base: **`main`**  
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m28_shared_backend_boundary`**  
Worktree root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m28-shared-backend-boundary`**  
Last rewritten: **2026-05-02**

## Summary

M28 is a bounded architecture milestone, not an evidence or language-expansion
milestone.

The only implementation objective is to extract one explicit shared
backend-execution boundary in `spec-core`, route the current seam consumers
through it, and prove that current Rust status/export/read-side truth remains
unchanged.

Critical path:

1. Parent captures the live branch/dirty-state baseline and freezes scope from
   the current `PLAN.md`.
2. Parent stabilizes `ORCH_PLAN.md` as the execution contract and records that
   contract in run-state before any runtime freeze occurs.
3. Parent lands the shared seam contract locally in
   `spec-core/src/backend_execution.rs`, exports it from `spec-core/src/lib.rs`,
   and records that contract in `freeze.json`.
4. After the execution contract lock and runtime freeze, workers fan out in
   parallel:
   - Lane A rewires core consumers.
   - Lane B rewires read-side projection and regressions against the frozen
     seam.
   - Lane C audits `xtask` read-only.
5. Parent merges Lane A first, then Lane B, then consumes Lane C before final
   closeout.
6. Parent runs the exact proof loop from `PLAN.md`, records final proof, and
   closes M28 with an explicit M29 go/no-go decision.

Worker runtime policy:

- Intended worker model/class: `GPT-5.4`, `reasoning=high`.
- Maximum worker concurrency after freeze: `3`.
- Parent remains the sole integrator, merger, re-freeze authority, relaunch
  authority, scope interpreter, and final verifier.
- Only the parent may integrate, re-freeze, relaunch, or reinterpret scope.
- Workers may only execute their assigned lane contract. A blocker is never
  permission for a worker to broaden M28 on its own.

Single-parent model:

- The parent is the only integrator, merger, rebase authority, freeze owner,
  run-state owner, and final verifier.
- Workers edit only their assigned files in dedicated worktrees forked from the
  recorded freeze commit.
- `PLAN.md` remains read-only authority throughout this run. If the runtime work
  reveals that `PLAN.md` itself must change, halt and split a follow-on instead
  of broadening M28.

There are no human approval gates in this run. Only hard guards, execution
contract lock gates, freeze gates, lane acceptance gates, and final proof gates
may stop execution.

## Hard Guards

- Scope is locked to the current M28 in `PLAN.md`.
- The milestone is about shared backend-execution boundary extraction inside
  `spec-core`, not about recommendation policy, corpus policy, or language-two
  implementation.
- Preserve current Rust read-side truth for:
  - `spec status`
  - `spec export`
  - passport/backend freshness
  - semantic-review preserved-vs-leaked behavior
- Preserve the closed runtime file contract from `PLAN.md`:
  - `spec-core/src/backend_execution.rs` new
  - `spec-core/src/passport.rs`
  - `spec-core/src/escape_hatch.rs`
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/lib.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/cli.rs`
- Preserve the closed planning contract from `PLAN.md`:
  - `PLAN.md` is authority and remains read-only for this run
  - `ORCH_PLAN.md` is parent-owned execution contract
- `xtask` is audit-only in M28.
  - Read-only targets include `xtask/src/family/coverage.rs`,
    `xtask/src/family/report.rs`, and related proof/report surfaces.
  - If an `xtask` code edit is required to make M28 green, halt and split a
    bounded follow-on. Do not expand scope inside M28.
- Do not widen validator policy beyond
  `methods[].lowering.rust.body` and `backends.rust.derives`.
- Do not change recommendation, corpus, or `money/round` governance semantics.
- Do not add a second-language runtime, lowering path, packet, fixture, or
  scaffold.
- Do not add a new command family, artifact class, or release workflow.
- Never fork worker branches from `main`.
- Never assume the repo is clean. Capture and respect actual dirty state at
  launch.

Stop immediately if any of the following become true:

- `PLAN.md` and the frozen M28 execution contract cannot be reconciled without
  editing `PLAN.md`
- the shared boundary needs files outside the closed runtime contract
- validator policy widening becomes necessary
- `xtask` changes become necessary
- recommendation/corpus semantics drift
- second-language work enters the diff
- the run can no longer end with a credible M29 go/no-go decision

## Canonical Run-State

Parent-owned orchestration state lives only under:

- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m28_shared_backend_boundary`

Canonical parent-owned state files:

- `baseline.json`
  - live branch
  - launch HEAD SHA
  - dirty-file summary
  - lane ownership snapshot against actual dirty state
  - note of any lane delayed or narrowed because of pre-existing dirt
- `tasks.json`
  - ordered task queue
  - owner
  - status
  - dependencies
  - restart count
- `session-log.md`
  - chronological parent decisions
  - worker launch notes
  - relaunch reasons
  - deviations resolved without widening scope
- `docs-contract.json`
  - `PLAN.md` read-only confirmation
  - `ORCH_PLAN.md` stabilization timestamp
  - execution-contract hash or commit reference
  - parent-confirmed worker model and concurrency cap
  - parent-confirmed lane map before runtime freeze
- `freeze.json`
  - `freeze_commit_sha`
  - frozen public functions and types in `spec-core/src/backend_execution.rs`
  - digest/freshness invariants
  - lane ownership map
  - forbidden surfaces per lane
- `merge-log.md`
  - merge order
  - merge attempt result
  - local smoke results after each merge
  - relaunch/reject reasons
  - downstream stale-lane notes
- `integration-state.json`
  - current integrated tasks
  - pending tasks
  - proof-loop status
  - audit disposition
- `acceptance.md`
  - final acceptance checklist against this file and `PLAN.md`
- `final-proof.json`
  - ordered final commands
  - exit codes
  - artifact paths
  - final verdict
- `blocked.json`
  - stop reason
  - blocking task
  - blocking evidence
  - required follow-on plan
- `closeout.md`
  - final narrative closeout
  - explicit M29 decision

Per-task sentinels live under:

- `REPO_ROOT/.runs/task-m28-00-baseline/`
- `REPO_ROOT/.runs/task-m28-01-stabilize-execution-contract/`
- `REPO_ROOT/.runs/task-m28-02-freeze-backend-boundary/`
- `REPO_ROOT/.runs/task-m28-03-core-consumers/`
- `REPO_ROOT/.runs/task-m28-04-read-side-regressions/`
- `REPO_ROOT/.runs/task-m28-05-xtask-audit/`
- `REPO_ROOT/.runs/task-m28-06-integrate-proof/`
- `REPO_ROOT/.runs/task-m28-07-closeout-m29/`

Each task sentinel directory uses:

- `started.json`
- `status.json`
- exactly one terminal file: `done.json` or `blocked.json`

Worker handoff artifacts live under:

- `RUN_ROOT/handoffs/task-m28-03-core-consumers/`
- `RUN_ROOT/handoffs/task-m28-04-read-side-regressions/`
- `RUN_ROOT/handoffs/task-m28-05-xtask-audit/`

Every worker handoff must contain:

- `result.json`
- `handoff.md`
- `commands.json`
- exactly one terminal marker: `done.ok` or `blocked.ok`

Run-state rules:

- `.runs/**` are parent-owned run artifacts, not authored source.
- Workers do not modify `RUN_ROOT/*` or any `.runs/task-m28-*/` sentinel
  directly.
- Parent writes all orchestration state back to `REPO_ROOT`.

## Execution Model And Critical Path

Parent-only serialized phases:

1. `task/m28-00-baseline`
2. `task/m28-01-stabilize-execution-contract`
3. `task/m28-02-freeze-backend-boundary`
4. `task/m28-06-integrate-proof`
5. `task/m28-07-closeout-m29`

Parallel worker lanes after the execution contract lock and runtime freeze:

- Lane A: `task/m28-03-core-consumers`
- Lane B: `task/m28-04-read-side-regressions`
- Lane C: `task/m28-05-xtask-audit`

Why this split is safe:

- Parent owns the new shared seam itself:
  - `spec-core/src/backend_execution.rs`
  - `spec-core/src/lib.rs`
- Parent also owns the only live execution contract:
  - `ORCH_PLAN.md`
  - `docs-contract.json`
- Lane A owns only core consumer rewires.
- Lane B owns only read-side/status/tests rewires against the frozen seam.
- Lane C stays read-only and cannot create merge conflicts by design.

Why this split is strict:

- Worker launch is forbidden until both `docs-contract.json` and `freeze.json`
  exist.
- If parent changes the execution contract or the frozen seam after workers
  launch, affected lanes are stale and must be relaunched from the new freeze.
- If Lane C concludes that `xtask` must change, M28 stops in a blocked state.
  That result does not authorize “small enough to fix now.”

## Parent vs Worker Ownership Model

### Parent-only ownership

Parent-owned files:

- `ORCH_PLAN.md`
- `spec-core/src/backend_execution.rs`
- `spec-core/src/lib.rs`
- all `.runs/**`
- all merges, rebases, and conflict resolution
- `acceptance.md`
- `final-proof.json`
- `blocked.json`
- `closeout.md`

Parent-only responsibilities:

- capture actual baseline branch/SHA/dirty state
- stabilize the execution contract before worker launch
- freeze the shared seam contract before worker launch
- create worker branches and worktrees from `freeze_commit_sha`
- reject stale or out-of-bounds worker output
- merge lanes in order
- run the exact final proof loop
- decide the M29 go/no-go closeout

### Worker lane ownership

Lane A owns only:

- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/semantic_review.rs`

Lane B owns only:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`

Lane C owns no authored files. It is read-only against:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/report.rs`
- related prove/certify/report schemas or fixtures needed to support the audit

Workers never own:

- `PLAN.md`
- `ORCH_PLAN.md`
- `spec-core/src/backend_execution.rs`
- `spec-core/src/lib.rs`
- any `xtask` source file for edit
- `generator.rs`
- `validator.rs`
- `.runs/**`

## Context-Control Rules

Parent active context stays narrow:

- `PLAN.md`
- `ORCH_PLAN.md`
- `RUN_ROOT/baseline.json`
- `RUN_ROOT/tasks.json`
- `RUN_ROOT/docs-contract.json`
- `RUN_ROOT/freeze.json` after freeze
- `RUN_ROOT/session-log.md`
- latest integration diff summary

Worker prompts contain only:

- owned files
- forbidden surfaces
- relevant `PLAN.md` excerpts
- the current `docs-contract.json`
- the current `freeze.json`
- the required commands
- the lane acceptance criteria

Context rules:

- Workers do not get the full repo or unrelated docs by default.
- Workers stop and return a blocker if they need a parent-owned file changed.
- Parent closes workers immediately after merge or rejection.
- There is no worker docs lane in M28.
  - `PLAN.md` never changes.
  - `ORCH_PLAN.md` is parent-owned and is stabilized in an explicit pre-freeze
    parent task.
  - If the parent materially changes `ORCH_PLAN.md` after worker launch, open
    lanes are stale and must be relaunched from a new freeze.

## Worktree And Branch Plan With Concrete Names

Canonical paths:

- `REPO_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m28-shared-backend-boundary`

Integration surface:

- branch: `feat/corpus-expansion`
- worktree: `REPO_ROOT`
- owner: parent only

Worker branches and worktrees, created only after `docs-contract.json` and
`freeze.json` both exist:

- Lane A
  - branch: `codex/m28-core-consumers`
  - worktree: `$WORKTREE_ROOT/core-consumers`
- Lane B
  - branch: `codex/m28-read-side-regressions`
  - worktree: `$WORKTREE_ROOT/read-side-regressions`
- Lane C
  - branch: `codex/m28-xtask-audit`
  - worktree: `$WORKTREE_ROOT/xtask-audit`

Creation commands from `REPO_ROOT` after freeze:

```bash
mkdir -p "$WORKTREE_ROOT" "$RUN_ROOT"
FREEZE_SHA=$(jq -r '.freeze_commit_sha' "$RUN_ROOT/freeze.json")

git worktree add -b codex/m28-core-consumers \
  "$WORKTREE_ROOT/core-consumers" \
  "$FREEZE_SHA"

git worktree add -b codex/m28-read-side-regressions \
  "$WORKTREE_ROOT/read-side-regressions" \
  "$FREEZE_SHA"

git worktree add -b codex/m28-xtask-audit \
  "$WORKTREE_ROOT/xtask-audit" \
  "$FREEZE_SHA"
```

Worktree rules:

- never fork workers from `main`
- never fork workers before the contract-stabilization and freeze tasks complete
- never reuse a dirty worker worktree
- never let workers merge back into `feat/corpus-expansion`

## Freeze Artifact And Restart Rule

The freeze point is the first parent commit where all of the following are true
together:

- `spec-core/src/backend_execution.rs` exists
- `spec-core/src/lib.rs` exports the new module
- the module exposes the parent-approved boundary API for:
  - marker collection
  - marker classification
  - summary construction
  - backend-execution digest computation
  - shared helper/example identity surface
- `docs-contract.json` records the stabilized execution contract
- `freeze.json` records the frozen contract and lane ownership

`freeze.json` is the authoritative runtime worker contract.
`docs-contract.json` is the authoritative execution-contract lock.

Shared-seam or execution-contract change after worker launch means any change
to:

- public function or type signatures in `spec-core/src/backend_execution.rs`
- marker-kind semantics
- digest/freshness invariants
- the export of the module from `spec-core/src/lib.rs`
- lane ownership or forbidden surfaces recorded in `freeze.json`
- lane assumptions or parent-owned execution rules recorded in
  `docs-contract.json`

Mandatory stale-lane rule:

1. If the frozen seam or execution contract changes after any worker starts,
   affected open lanes are stale.
2. Stale worker output must not be merged, cherry-picked, or manually
   reconciled.
3. Parent must:
   - update `docs-contract.json` if execution-contract assumptions changed
   - update `freeze.json`
   - record the new `freeze_commit_sha`
   - mark affected tasks blocked/stale in sentinel state
   - delete stale worker worktrees and branches
   - relaunch fresh worktrees from the new freeze commit
4. Lane C is stale only if the audit scope itself changes.

## Worker Handoff And Parent Review Contract

Every worker handoff package is mandatory and must be complete before the
parent reviews code. The parent does not improvise missing context from git
history or prior chat text.

Generic required files for all worker handoffs:

- `result.json`
  - `task_id`
  - `lane`
  - `freeze_commit_sha`
  - `head_commit_sha`
  - `status`
  - `changed_files`
  - `commands_run`
  - `exit_codes`
  - `assumptions`
  - `blockers`
- `handoff.md`
  - short human-readable summary
  - what changed
  - why the lane believes acceptance passed
  - explicit note of anything not verified
- `commands.json`
  - ordered command list
  - working directories
  - exit codes
- exactly one terminal marker:
  - `done.ok`
  - `blocked.ok`

Parent pre-merge review is mandatory for every lane:

1. verify `freeze_commit_sha` matches current `RUN_ROOT/freeze.json`
2. verify changed files stay inside lane ownership
3. verify required commands were actually run and exited `0`, unless the lane
   returned `blocked.ok`
4. verify the handoff explains assumptions and unresolved risk plainly
5. verify the lane did not edit parent-owned files or `.runs/**`
6. verify the lane is not stale against post-freeze semantic changes already
   recorded in `merge-log.md`

Reject and relaunch rules:

- Reject immediately if the handoff is incomplete.
- Reject immediately if `freeze_commit_sha` is stale.
- Reject immediately if the diff includes out-of-scope edits.
- Reject immediately if required command accounting is missing.
- Reject immediately if the lane depends on a parent reinterpretation of scope.
- Relaunch from the current freeze if the lane is directionally correct but
  stale against updated post-freeze assumptions.
- Stop the entire run instead of relaunching if the reject reason implies M28
  scope expansion.

## Merge Policy

- Parent is the only merger.
- Parent merges only into `feat/corpus-expansion` in `REPO_ROOT`.
- Parent does not ask workers to rebase.
- Parent may do local mechanical conflict resolution only inside worker-owned
  files.
- Any conflict that changes the frozen seam or the locked execution contract is
  a stop-and-relaunch event.

Merge order:

1. Parent completes `task/m28-01-stabilize-execution-contract`.
2. Parent completes `task/m28-02-freeze-backend-boundary`.
3. Launch Lanes A, B, and C from `freeze_commit_sha`.
4. Merge Lane A first.
5. Rerun Lane A post-merge verification locally.
6. Merge Lane B second.
7. Rerun Lane B post-merge verification locally.
8. Consume Lane C audit result before closeout:
   - if `no_leak_found`, continue
   - if `leak_found_follow_on_required`, stop M28 in blocked closeout
9. Parent runs the final proof loop locally.

Post-merge local verification is required after each successful merge attempt:

- After Lane A merge:
  - rerun `cargo test -p spec-core --lib -- --color never`
  - confirm Lane A did not silently invalidate Lane B's fixture or read-side
    assumptions
- After Lane B merge:
  - rerun `cargo test -p spec-cli --test m14_regressions -- --color never`
  - rerun `cargo test -p spec-cli --test cli -- --color never`
  - confirm `status`/`export` parity claims still match the integrated tree
- After Lane C disposition:
  - record whether the audit was accepted as `no_leak_found` or forced a stop

`merge-log.md` must record after every merge attempt:

- lane/task ID
- worker branch and handoff commit SHA
- accepted or rejected disposition
- reason for rejection or relaunch if applicable
- local post-merge commands run
- local post-merge exit codes
- whether downstream lanes became stale

Lane B invalidation rule:

- Even without a seam signature change, Lane B becomes stale if the accepted
  Lane A merge changes semantic assumptions behind Lane B's parity or regression
  expectations.
- In that case the parent must reject or relaunch Lane B from the current
  freeze/integrated state instead of manually patching its output.

Docs-vs-runtime merge rule:

- There is no worker docs merge in M28.
- `ORCH_PLAN.md` must already be stable before worker launch.
- Runtime lane merges do not authorize late plan drift.
- If runtime work appears to require a `PLAN.md` edit, stop rather than
  backfitting the plan after the fact.

Reject a worker lane immediately if it:

- edits a file outside its ownership set
- depends on a parent-owned seam change after launch
- assumes a clean tree and drops unrelated local state
- returns without command and exit-code accounting
- returns a handoff package that is incomplete or stale against `freeze.json`

## Task Graph

```text
task/m28-00-baseline
  ->
task/m28-01-stabilize-execution-contract
  ->
task/m28-02-freeze-backend-boundary
  ->
parallel:
  task/m28-03-core-consumers
  task/m28-04-read-side-regressions
  task/m28-05-xtask-audit
  ->
task/m28-06-integrate-proof
  ->
task/m28-07-closeout-m29
```

## Workstream Plan

### task/m28-00-baseline

Owner:

- parent

Owned surfaces:

- `RUN_ROOT/baseline.json`
- `RUN_ROOT/tasks.json`
- `RUN_ROOT/session-log.md`
- `REPO_ROOT/.runs/task-m28-00-baseline/**`

Required work:

- capture current branch, HEAD SHA, and dirty-state summary
- confirm `PLAN.md` is the M28 source of truth
- record that `PLAN.md` is read-only for this run
- snapshot lane ownership against actual dirty files
- identify whether pre-existing dirt touches any worker-owned surface

Required commands:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
```

Acceptance:

- baseline reflects actual repo state at launch
- lane ownership snapshot exists
- any pre-existing dirt that overlaps owned files is recorded before freeze

### task/m28-01-stabilize-execution-contract

Owner:

- parent

Owned surfaces:

- `ORCH_PLAN.md`
- `RUN_ROOT/docs-contract.json`
- `RUN_ROOT/session-log.md`
- `REPO_ROOT/.runs/task-m28-01-stabilize-execution-contract/**`

Forbidden surfaces:

- runtime implementation files
- any `xtask` source file
- `PLAN.md`

Required work:

- stabilize `ORCH_PLAN.md` as the execution contract for the run
- record that `PLAN.md` is read-only authority and `ORCH_PLAN.md` is the
  parent-owned execution contract
- lock worker model, concurrency cap, lane map, and merge ordering in
  `docs-contract.json`
- record the exact point at which the execution contract became stable enough
  for runtime freeze to proceed

Acceptance:

- `ORCH_PLAN.md` is stable enough that workers can be launched without further
  plan reinterpretation
- `docs-contract.json` records the lane contract and parent-only docs policy
- the runtime freeze task can proceed without another docs pass

### task/m28-02-freeze-backend-boundary

Owner:

- parent

Owned surfaces:

- `spec-core/src/backend_execution.rs`
- `spec-core/src/lib.rs`
- `RUN_ROOT/freeze.json`
- `REPO_ROOT/.runs/task-m28-02-freeze-backend-boundary/**`

Forbidden surfaces:

- Lane A files
- Lane B files
- all `xtask` source files
- `PLAN.md`

Required work:

- add the new shared boundary module
- freeze the boring, explicit API for:
  - backend-execution marker collection
  - marker classification
  - marker summary
  - backend-execution digest computation
  - helper/example identity reuse
- record the contract and invariants in `freeze.json`

Required invariants to record:

- authored-only seam edits do not change backend-execution freshness
- backend-only lowering/derives edits do not change authored freshness
- helper-only and domain-lowering markers remain distinguishable

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
```

Acceptance:

- `backend_execution.rs` exists and is exported
- workers can consume the frozen seam without editing parent-owned files
- `freeze.json` is concrete enough to relaunch stale lanes deterministically

### task/m28-03-core-consumers

Owner:

- worker A

Owned surfaces:

- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/semantic_review.rs`

Forbidden surfaces:

- `spec-core/src/backend_execution.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/**`
- any `xtask` file
- `.runs/**`
- `PLAN.md`
- `ORCH_PLAN.md`

Required work:

- route passport backend digest and marker truth through the frozen boundary
- route escape-hatch gate logic through the frozen boundary
- route semantic-review aligned/preserved/leaked classification through the
  frozen boundary
- preserve current helper-only vs domain-lowering meaning exactly

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
```

Required handoff payload:

- `result.json` with owned-file diff summary, `freeze_commit_sha`, and explicit
  pass/fail status
- `handoff.md` explaining how each owned consumer now uses the frozen boundary
- `commands.json` proving the required test command ran locally
- `done.ok` or `blocked.ok`

Parent pre-merge checks:

- owned files only
- handoff references the current `freeze_commit_sha`
- no independent backend marker scan remains in the owned files
- required `spec-core` test command exited `0`

Acceptance:

- no independent backend marker scan remains in the owned files
- passport authored/backend freshness invariants stay intact
- required proof surfaces remain `atom` + `molecule`
- preserved-vs-leaked semantic-review behavior stays truthful

### task/m28-04-read-side-regressions

Owner:

- worker B

Owned surfaces:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`

Forbidden surfaces:

- `spec-core/src/backend_execution.rs`
- `spec-core/src/lib.rs`
- Lane A files
- any `xtask` file
- `.runs/**`
- `PLAN.md`
- `ORCH_PLAN.md`

Required work:

- preserve export/status parity against the frozen boundary
- preserve CLI health demotion behavior for open escape-hatch gates and
  semantic drift
- land the targeted regressions from `PLAN.md`

Required commands:

```bash
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
```

Required handoff payload:

- `result.json` with parity assumptions, changed files, and
  `freeze_commit_sha`
- `handoff.md` naming the fixtures and invariants relied on
- `commands.json` for both targeted `spec-cli` test runs
- `done.ok` or `blocked.ok`

Parent pre-merge checks:

- owned files only
- handoff references the current `freeze_commit_sha`
- assumptions still hold after accepted Lane A merge
- targeted `spec-cli` tests exited `0`

Acceptance:

- export truth matches status truth for the same fixture
- CLI JSON/text reasons remain truthful
- backend-only preserved vs leaked behavior remains locked by regression tests

### task/m28-05-xtask-audit

Owner:

- worker C

Owned surfaces:

- none for edit

Read-only audit targets:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/report.rs`
- prove/certify/report wording or schemas needed to support the audit

Forbidden surfaces:

- every authored source file for edit
- `.runs/**`
- `PLAN.md`
- `ORCH_PLAN.md`

Required work:

- inspect whether current `xtask` proof/report/coverage surfaces embed a real
  Rust-specific backend semantic leak
- confirm that the frozen runtime extraction leaves coverage output byte-stable
  under unchanged recommendation semantics
- return only one of:
  - `no_leak_found`
  - `leak_found_follow_on_required`

Required commands:

```bash
rg -n "Rust|rust|lowering|backend|escape|semantic" xtask/src/family xtask/src/lib.rs
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Required handoff payload:

- `result.json` with explicit audit disposition and cited files
- `handoff.md` explaining why the audit did or did not force a follow-on
- `commands.json` for the audit commands and exit codes
- `done.ok` or `blocked.ok`

Parent pre-merge checks:

- no authored source edits exist
- audit disposition is one of the two allowed values
- any claimed leak is evidence-backed and cannot be solved inside M28 honestly

Acceptance:

- audit result is explicit
- no `xtask` file changed
- any claimed leak cites the exact file/path and why it cannot be addressed
  inside M28 without expanding scope

### task/m28-06-integrate-proof

Owner:

- parent

Owned surfaces:

- merged integration branch state
- `RUN_ROOT/merge-log.md`
- `RUN_ROOT/integration-state.json`
- `RUN_ROOT/acceptance.md`
- `REPO_ROOT/.runs/task-m28-06-integrate-proof/**`

Required work:

- merge Lane A, then rerun its smoke command locally
- merge Lane B, then rerun its smoke commands locally
- consume Lane C audit disposition before declaring M28 complete
- verify no out-of-scope file drift entered the integration branch
- run the exact proof loop from `PLAN.md`

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Optional full confirmation:

```bash
cargo test -p spec-cli -- --color never
```

Local post-merge verification runbook:

- after Lane A merge:
  - run `cargo test -p spec-core --lib -- --color never`
  - inspect whether Lane B's stated fixtures or invariants are now stale
  - if stale, reject/relaunch Lane B before merging it
- after Lane B merge:
  - run targeted `spec-cli` tests
  - confirm integrated `status`/`export` parity claims still hold
- after Lane C disposition:
  - record audit outcome in `merge-log.md` and `integration-state.json`

Acceptance:

- exact proof loop is green
- coverage JSON is byte-stable
- no recommendation/corpus semantics drift occurred
- no `xtask` edits were needed
- if audit found a leak, the run transitions to blocked closeout instead of
  pretending to complete

### task/m28-07-closeout-m29

Owner:

- parent

Owned surfaces:

- `RUN_ROOT/final-proof.json`
- `RUN_ROOT/blocked.json` if needed
- `RUN_ROOT/closeout.md`
- `REPO_ROOT/.runs/task-m28-07-closeout-m29/**`

Required work:

- write final proof accounting
- write closeout narrative
- decide one explicit M29 outcome:
  - `go`
  - `no_go`

Required M29 `go` conditions:

- all high-criticality runtime consumers share the frozen boundary path
- current Rust status/export/read-side truth stayed intact
- coverage stayed byte-stable
- `xtask` audit disposition is `no_leak_found`
- closeout states that the runtime boundary is now honest enough for a scoped
  M29 pilot, not that M29 begins automatically

Required M29 `no_go` conditions:

- `xtask` leak requires a follow-on plan
- a runtime consumer still derives backend truth independently
- proof loop drifted outside the closed contract
- recommendation/corpus or second-language scope drift occurred
- `closeout.md` names the blocker theme that stops an honest scoped pilot

Acceptance:

- `closeout.md` ends with an explicit M29 decision and rationale
- `no_go` closeout includes a named follow-on blocker theme
- `final-proof.json` and `blocked.json` agree with `closeout.md`
- there is no cleanup-only or “mostly ready” third option

## Integration Sequence

Parent integration order is fixed:

1. Capture baseline.
2. Stabilize the execution contract locally.
3. Freeze the shared seam locally.
4. Launch workers from `freeze_commit_sha`.
5. Merge Lane A.
6. Run Lane A post-merge local verification.
7. Merge Lane B.
8. Run Lane B post-merge local verification.
9. Consume Lane C audit disposition.
10. Run the full final proof loop.
11. Write closeout and M29 go/no-go.

If Lane A changes the frozen seam by necessity, stop, update `freeze.json`, and
relaunch Lane B from the new freeze instead of manually reconciling its output.

If Lane A invalidates Lane B assumptions without changing the seam signature,
reject or relaunch Lane B from the current integrated state rather than merging
it optimistically.

## Acceptance Gates

### Gate 0 - Launch gate

- baseline captured against actual dirty state
- `PLAN.md` authority confirmed
- `PLAN.md` marked read-only for the run

### Gate 1 - Execution-contract lock gate

- `ORCH_PLAN.md` is stable for this run
- `docs-contract.json` exists and records worker model, concurrency cap, lane
  map, and parent-only docs policy

### Gate 2 - Runtime freeze gate

- `backend_execution.rs` exists
- `spec-core/src/lib.rs` exports it
- `freeze.json` records the frozen seam and invariants

### Gate 3A - Core consumer lane gate

- Lane A stays within ownership
- owned files consume the frozen seam
- handoff package is complete
- `cargo test -p spec-core --lib -- --color never` exits `0`

### Gate 3B - Read-side lane gate

- Lane B stays within ownership
- status/export parity and CLI regressions are covered
- handoff package is complete
- targeted `spec-cli` tests exit `0`

### Gate 3C - Audit gate

- Lane C remains read-only
- handoff package is complete
- returns only `no_leak_found` or `leak_found_follow_on_required`
- any leak claim is evidence-backed

### Gate 4 - Final proof gate

- exact proof loop from `PLAN.md` is green
- coverage output is byte-stable
- no out-of-scope files entered the diff

### Gate 5 - M29 decision gate

- `go` only if runtime seam extraction is complete and `xtask` stays read-only
- `go` means M28 leaves the runtime boundary honest enough for a scoped M29
  pilot
- `go` does not mean M29 starts automatically
- `no_go` requires a named follow-on blocker theme in `closeout.md`

## Halt Conditions

Halt immediately if any of the following occur:

- a required code edit falls outside the closed runtime contract
- `PLAN.md` would need to change to justify the runtime diff
- `xtask` change becomes necessary
- validator policy widening becomes necessary
- coverage JSON drifts under unchanged recommendation semantics
- `status` and `export` disagree on the same fixture
- helper-only lowering becomes domain-lowering
- leaked backend-only meaning becomes silently preserved
- second-language work enters the branch
- the run cannot produce an explicit M29 go/no-go outcome

## Closeout Requirements And Blocked-Run Handling

`closeout.md` must state:

- launch branch and freeze commit
- whether pre-existing dirt affected any lane
- exact merge order
- exact proof commands run
- coverage diff result
- whether CLI/status/export wording changed or only internals changed
- whether current Rust read-side truth stayed aligned
- `xtask` audit disposition
- explicit M29 decision with rationale
- if `no_go`, the named follow-on blocker theme

`final-proof.json` must record:

- ordered commands
- exit codes
- relevant artifact paths
- final acceptance verdict

If the run blocks, `blocked.json` must record:

- blocking task ID
- exact halt condition
- evidence file or command output path
- why the blocker cannot be solved inside M28 honestly
- required follow-on plan name

Blocked-run rule:

- A blocked M28 may preserve completed runtime extraction work in local branch
  state for analysis, but it does not count as an accepted milestone close.
- A blocked M28 must still produce final proof/accounting artifacts and an
  explicit M29 `no_go`.

## Tests And Acceptance

Required final ordered sequence from `PLAN.md`:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Optional only if `spec-cli` diff widens:

```bash
cargo test -p spec-cli -- --color never
```

Expected invariants after the final sequence:

- exactly one shared backend-execution boundary exists in `spec-core`
- `passport.rs`, `escape_hatch.rs`, `semantic_review.rs`, `export.rs`, and
  `spec-cli/src/commands.rs` all consume that boundary instead of re-deriving
  backend truth independently
- authored-only seam changes do not alter backend freshness
- backend-only lowering/derive changes do not alter authored freshness
- helper-only markers remain helper-only
- domain-lowering markers remain domain-lowering
- preserved backend-only meaning remains preserved
- leaked backend-only meaning remains failing
- `status` and `export` agree on the same fixtures
- coverage output remains byte-stable
- no recommendation/corpus semantics change occurred
- no second-language work landed

## Assumptions

- `feat/corpus-expansion` remains the live integration branch for M28.
- `PLAN.md` already contains the normative milestone scope and proof loop.
- Current dirty state, including any local `PLAN.md` edits, is real repo state
  and must be recorded rather than overwritten.
- `spec-core/src/backend_execution.rs` can expose a stable enough seam contract
  for workers without needing broader refactors.
- The existing M14 regressions and CLI fixtures are sufficient to prove current
  Rust read-side truth without inventing a new fixture family.
- Any real `xtask` leak discovered by Lane C is a follow-on planning input, not
  permission to expand M28.
