# M42 Orchestration Plan

Status: **authoritative execution contract for M42 "Decision-Contract Verifier Stop-State Parity"**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Milestone: **Decision-Contract Verifier Stop-State Parity**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Working branch: **`feat/m40-plus`**  
Last rewritten: **`2026-05-09`**  
Canonical run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m42_decision_contract_verifier_stop_state_parity`**

## Summary

- `PLAN.md` is the only product authority. This file is the execution contract for completing that authority without widening the milestone.
- M42 is a narrow verifier-truth repair.
- The live decision kernel already tells the truthful post-M41 story:
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `open_blockers = []`
  - `missing_evidence = []`
  - `stale_evidence = []`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- The only broken surface is `cargo xtask family verify-decision-contract --format json`, which still freezes a retired helper-surface floor and therefore fails on truthful HEAD artifacts.
- The honest implementation surface is:
  - `xtask/src/family/analysis_core/decision_contract.rs`
  - `xtask/src/family/analysis_core/mod.rs`
  - `xtask/src/family/verify.rs`
  - `xtask/src/lib.rs` only if command-facing tests actually need refresh
- The parent remains the sole integrator, sole proof owner, sole gate owner, and sole closeout author.
- There is no honest parallel worker split for M42.

## Hard Guards

- No new schema version.
- No new CLI flags.
- No command rename.
- `family verify-decision-contract` remains JSON-only.
- Keep the public JSON check key `checks.frozen_helper_surface_floor`.
- Keep the public failure reason `frozen_helper_surface_floor_mismatch`.
- No edits to `xtask/src/family/recommend.rs`.
- No decision-kernel widening beyond exposing the already-truthful stop-state as shared verifier truth.
- No edits to `xtask/src/family/helper_surface.rs`.
- No packet work.
- No `semantic-families/**` changes.
- No docs or changelog work in M42.
- No worker worktrees.
- No new branches for sub-work.
- `PLAN.md` wins over this file, stale `ORCH_PLAN.md`, and `.runs/*` evidence if they disagree.
- `.runs/*` is execution evidence only. It is never authority.

## Locked Outcome Contract

Post-M42 repo truth must be:

- `cargo xtask family recommend --format json` remains truthful and unchanged in meaning.
- `cargo xtask family corpus-decision --format json` remains truthful and unchanged in meaning.
- `cargo xtask family verify-decision-contract --format json` passes on the truthful stop-state instead of failing on a retired helper-surface tuple.
- The verifier still proves these five surfaces:
  - recommendation analysis validation
  - corpus program decision validation
  - basis snapshot parity
  - derived decision parity
  - frozen floor parity
- The verifier still fails loudly on real drift.
- The verifier no longer owns a stale verifier-local copy of retired helper-surface policy truth.

### Locked Truthful Stop-State

The shared frozen floor introduced by M42 must encode exactly this tuple:

```text
recommendation_status = insufficient_real_corpus
decision_status = not_recommended
open_blockers = []
missing_evidence = []
stale_evidence = []
decision_action = stop
decision_basis_code = no_actionable_candidate
required_next_action = record_stop_without_new_milestone
```

### Locked Public Verifier Surface

The public verifier surface stays stable in M42:

- command name unchanged: `family verify-decision-contract`
- supported format unchanged: `--format json` only
- JSON check key unchanged: `checks.frozen_helper_surface_floor`
- failure reason unchanged: `frozen_helper_surface_floor_mismatch`

Internal Rust helper names and test names may become stop-state-honest. Public machine-readable names must not.

## Locked File Surface

| Area | Files | Rule |
|---|---|---|
| shared stop-state truth | `xtask/src/family/analysis_core/decision_contract.rs` | required |
| shared export seam | `xtask/src/family/analysis_core/mod.rs` | required if verifier imports through seam |
| verifier parity, fixtures, tests | `xtask/src/family/verify.rs` | required |
| CLI dispatch tests | `xtask/src/lib.rs` | conditional only |
| recommendation policy | `xtask/src/family/recommend.rs` | forbidden |
| helper-surface policy | `xtask/src/family/helper_surface.rs` | forbidden |
| packet and family assets | `semantic-families/**` | forbidden |
| docs, changelog, orchestration rewrites beyond this file | `README.md`, `CHANGELOG.md`, unrelated docs | out of scope |

### Escalation-Only Surface

If any of these become necessary, stop and rewrite orchestration instead of widening silently:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- any schema-bearing artifact type outside the verifier-local check logic
- any production file outside the locked file surface

## Locked Implementation Order

Execution is serialized. M42 does not permit lane splitting.

```text
Gate 00 baseline capture
  ->
Gate 05 authority freeze
  ->
Task 10 shared truthful stop-state helper
  ->
Gate 20 contract lock
  ->
Task 30 verifier rewiring + fixture reseed + regression-proof refresh
  ->
Task 35 optional CLI dispatch audit in the same lane
  ->
Gate 40 full proof loop + public-surface lock + scope guard capture
  ->
Gate 50 closeout
```

Order rules:

- Baseline capture is mandatory before any code edit.
- Authority freeze is mandatory before any code edit.
- The shared stop-state helper lands before verifier rewiring.
- Regression-proof capture is part of the critical path, not a side effect.
- `xtask/src/lib.rs` is audited only after verifier work is green, and if it needs a touch it remains in the same serialized lane.
- Closeout is forbidden until the baseline failure and final pass are both recorded in `RUN_ROOT`.

## Execution Topology

Canonical paths:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m42_decision_contract_verifier_stop_state_parity`

Worktree layout:

| Role | Branch | Worktree | Owner | Status |
|---|---|---|---|---|
| primary execution lane | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | authoritative |
| worker lanes | none | none | none | intentionally disabled |

## No-Parallelism Operating Mode

There is no honest parallel split for M42.

Use this operating mode instead:

- The parent does all code and proof work locally on `feat/m40-plus`.
- No worker worktrees are authorized.
- No subagent may own a file, test lane, or proof artifact.
- If `xtask/src/lib.rs` needs a touch, it remains in the same serialized lane after verifier work.
- If a surprise scope expansion appears, halt and write `blocked.json` rather than inventing lanes or splitting ownership.
- If the change stops being a three-file verifier-truth repair, the run is no longer valid under this plan.

## Canonical Run-State Artifacts

`RUN_ROOT` is execution evidence only. The parent owns it.

Required kickoff artifacts:

- `baseline.json`
- `authority-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`

Required contract-freeze artifact:

- `stop-state-contract-freeze.json`

Required validation artifacts tied to gates:

| Artifact | Phase | Gate | Meaning |
|---|---|---|---|
| `validation/baseline-recommend.json` | pre-fix | Gate 00 | live recommend truth before edits |
| `validation/baseline-corpus-decision.json` | pre-fix | Gate 00 | live corpus-decision truth before edits |
| `validation/baseline-verify-decision-contract.json` | pre-fix | Gate 00 | verifier JSON output before edits |
| `validation/baseline-verify-decision-contract.stderr.txt` | pre-fix | Gate 00 | verifier stderr before edits |
| `validation/baseline-verify-decision-contract.exit.txt` | pre-fix | Gate 00 | must record non-zero baseline failure |
| `validation/baseline-cargo-test-p-xtask-verify.stdout.txt` | pre-fix | Gate 00 | starting targeted test state |
| `validation/contract-helper-tests.stdout.txt` | mid-run | Gate 20 | helper-level proof after shared truth lands |
| `validation/postfix-cargo-test-p-xtask-verify.stdout.txt` | post-fix | Gate 40 | targeted verifier regression proof |
| `validation/postfix-cargo-test-p-xtask.stdout.txt` | post-fix | Gate 40 | broad xtask regression proof |
| `validation/family-recommend.json` | post-fix | Gate 40 | live recommend truth after edits |
| `validation/family-corpus-decision.json` | post-fix | Gate 40 | live corpus-decision truth after edits |
| `validation/family-verify-decision-contract.json` | post-fix | Gate 40 | verifier JSON output after edits |
| `validation/family-verify-decision-contract.stderr.txt` | post-fix | Gate 40 | verifier stderr after edits |
| `validation/family-verify-decision-contract.exit.txt` | post-fix | Gate 40 | must record exit `0` |
| `validation/final-public-surface-grep.txt` | post-fix | Gate 40 | evidence that outward key / failure-reason names remain present |
| `validation/final-diff-name-only.txt` | post-fix | Gate 40 | scope guard: changed file list |
| `validation/final-diff-stat.txt` | post-fix | Gate 40 | scope guard: change footprint |

Conditional validation artifacts:

- `validation/cargo-test-p-xtask-lib-dispatch.stdout.txt` if `xtask/src/lib.rs` changes or targeted dispatch reruns are needed
- `validation/scope-escalation.md` if the locked file surface proves insufficient

Required closeout or blocked artifacts:

- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination

## Queue And Gates

| Order | ID | Kind | Owner |
|---|---|---|---|
| 1 | `gate-m42-00-baseline-freeze` | gate | parent |
| 2 | `gate-m42-05-authority-freeze` | gate | parent |
| 3 | `task-m42-10-shared-stop-state-contract` | task | parent |
| 4 | `gate-m42-20-contract-lock` | gate | parent |
| 5 | `task-m42-30-verifier-parity-refresh` | task | parent |
| 6 | `task-m42-35-cli-dispatch-audit` | task | parent |
| 7 | `gate-m42-40-full-proof-loop` | gate | parent |
| 8 | `gate-m42-50-closeout` | gate | parent |

Queue rules:

- Gates never overlap.
- All tasks are sequential.
- `task-m42-35-cli-dispatch-audit` is conditional and may be a no-op.
- No gate may be skipped.
- If any gate writes `blocked.json`, the run stops immediately.

## Gate And Task Procedures

### `gate-m42-00-baseline-freeze`

Purpose:

- capture the real pre-fix failure signature
- prove M42 is a regression repair, not speculative cleanup

Owned files:

- no source files
- `RUN_ROOT/**` only

Exact commands to run:

```bash
mkdir -p "$RUN_ROOT/validation"
git branch --show-current
git rev-parse HEAD
git status --short
cargo test -p xtask verify > "$RUN_ROOT/validation/baseline-cargo-test-p-xtask-verify.stdout.txt" 2>&1
cargo xtask family recommend --format json > "$RUN_ROOT/validation/baseline-recommend.json" 2> "$RUN_ROOT/validation/baseline-recommend.stderr.txt"
cargo xtask family corpus-decision --format json > "$RUN_ROOT/validation/baseline-corpus-decision.json" 2> "$RUN_ROOT/validation/baseline-corpus-decision.stderr.txt"
bash -lc 'cargo xtask family verify-decision-contract --format json > "$RUN_ROOT/validation/baseline-verify-decision-contract.json" 2> "$RUN_ROOT/validation/baseline-verify-decision-contract.stderr.txt"; printf "%s\n" "$?" > "$RUN_ROOT/validation/baseline-verify-decision-contract.exit.txt"'
git diff --name-only > "$RUN_ROOT/validation/baseline-diff-name-only.txt"
```

Artifacts written:

- `baseline.json`
- `validation/baseline-*`
- `validation/baseline-diff-name-only.txt`
- `run-state.json`
- `session-log.md`

Blocked conditions:

- baseline `recommend` or `corpus-decision` does not show the locked truthful stop-state
- baseline verifier command exits `0`
- baseline verifier command does not emit the expected false-failure shape
- repo state is already widened beyond the locked file surface by unrelated in-flight work that makes proof ambiguous

### `gate-m42-05-authority-freeze`

Purpose:

- freeze the execution contract before edits
- lock scope, public surface, and truthful stop-state tuple

Owned files:

- no source files
- `RUN_ROOT/**` only

Exact commands to run:

```bash
git rev-parse HEAD
rg -n "Decision-Contract Verifier Stop-State Parity|verify-decision-contract|frozen_helper_surface_floor|frozen_helper_surface_floor_mismatch" PLAN.md xtask/src/family/verify.rs xtask/src/family/analysis_core/decision_contract.rs xtask/src/family/analysis_core/mod.rs xtask/src/lib.rs TODOS.md > "$RUN_ROOT/validation/authority-anchor-grep.txt"
printf "%s\n" "xtask/src/family/analysis_core/decision_contract.rs" "xtask/src/family/analysis_core/mod.rs" "xtask/src/family/verify.rs" "xtask/src/lib.rs" > "$RUN_ROOT/in-scope-files.txt"
```

Artifacts written:

- `authority-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `validation/authority-anchor-grep.txt`

Blocked conditions:

- `PLAN.md` conflicts with the locked M42 thesis
- scope cannot be expressed within the locked file surface
- authority ambiguity remains about the truthful stop-state tuple or public verifier surface

### `task-m42-10-shared-stop-state-contract`

Purpose:

- add the one shared truthful stop-state helper beside the live decision-contract seam
- remove the need for verifier-local stale truth

Owned files:

- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/mod.rs`

Exact commands to run:

```bash
cargo test -p xtask decision_contract > "$RUN_ROOT/validation/contract-helper-tests.stdout.txt" 2>&1
```

Artifacts written:

- updated source files
- `validation/contract-helper-tests.stdout.txt`
- `session-log.md`

Blocked conditions:

- the helper cannot be added without changing live decision policy
- the seam requires edits outside `decision_contract.rs` and `analysis_core/mod.rs`
- the shared helper would force a public schema or command-surface change

### `gate-m42-20-contract-lock`

Purpose:

- confirm the shared stop-state helper is the right truth source before touching verifier logic
- prevent downstream edits on an unstable contract

Owned files:

- no new source files
- `RUN_ROOT/**` only

Exact commands to run:

```bash
rg -n "insufficient_real_corpus|not_recommended|no_actionable_candidate|record_stop_without_new_milestone|frozen_helper_surface_floor" xtask/src/family/analysis_core/decision_contract.rs xtask/src/family/analysis_core/mod.rs xtask/src/family/verify.rs > "$RUN_ROOT/validation/contract-lock-grep.txt"
git diff --name-only > "$RUN_ROOT/validation/contract-lock-diff-name-only.txt"
```

Artifacts written:

- `stop-state-contract-freeze.json`
- `validation/contract-lock-grep.txt`
- `validation/contract-lock-diff-name-only.txt`

Blocked conditions:

- the helper tuple does not exactly match the locked truthful stop-state
- contract truth still appears duplicated in verifier-local literal form
- changed files already exceed the locked implementation surface
- any temptation appears to widen into `recommend.rs` or other policy code

Restart point if blocked:

- restart from `task-m42-10-shared-stop-state-contract`

### `task-m42-30-verifier-parity-refresh`

Purpose:

- rewire `verify.rs` to consume the shared truthful stop-state
- reseed fixtures away from helper-surface durable-hold assumptions
- land regression-proof coverage as part of the implementation itself

Owned files:

- `xtask/src/family/verify.rs`

Exact commands to run:

```bash
cargo test -p xtask verify > "$RUN_ROOT/validation/postfix-cargo-test-p-xtask-verify.stdout.txt" 2>&1
```

Artifacts written:

- updated `xtask/src/family/verify.rs`
- `validation/postfix-cargo-test-p-xtask-verify.stdout.txt`
- `session-log.md`

Blocked conditions:

- the verifier can only be made green by changing outward JSON names
- the verifier can only be made green by deleting the frozen-floor check
- fixture reseeding requires policy changes outside the locked file surface
- regression coverage cannot express the truthful stop-state failure/pass transition clearly

### `task-m42-35-cli-dispatch-audit`

Purpose:

- confirm the command dispatch surface remains locked
- touch `xtask/src/lib.rs` only if existing command-facing assertions require it

Owned files:

- `xtask/src/lib.rs` only if needed
- otherwise no source files

Exact commands to run:

```bash
cargo test -p xtask family_verify_decision_contract > "$RUN_ROOT/validation/cargo-test-p-xtask-lib-dispatch.stdout.txt" 2>&1
```

Artifacts written:

- maybe updated `xtask/src/lib.rs`
- `validation/cargo-test-p-xtask-lib-dispatch.stdout.txt`

Blocked conditions:

- command dispatch unexpectedly requires broader CLI wording cleanup
- a required `xtask/src/lib.rs` touch implies public surface drift rather than test refresh
- dispatch coverage reveals a second unrelated verifier command issue

Restart point if blocked:

- restart from `task-m42-30-verifier-parity-refresh` after scope review

### `gate-m42-40-full-proof-loop`

Purpose:

- prove the full milestone end to end on live commands
- record before/after parity
- record scope guard and public-surface lock evidence

Owned files:

- no new source files
- `RUN_ROOT/**` only

Exact commands to run:

```bash
cargo test -p xtask > "$RUN_ROOT/validation/postfix-cargo-test-p-xtask.stdout.txt" 2>&1
cargo xtask family recommend --format json > "$RUN_ROOT/validation/family-recommend.json" 2> "$RUN_ROOT/validation/family-recommend.stderr.txt"
cargo xtask family corpus-decision --format json > "$RUN_ROOT/validation/family-corpus-decision.json" 2> "$RUN_ROOT/validation/family-corpus-decision.stderr.txt"
bash -lc 'cargo xtask family verify-decision-contract --format json > "$RUN_ROOT/validation/family-verify-decision-contract.json" 2> "$RUN_ROOT/validation/family-verify-decision-contract.stderr.txt"; printf "%s\n" "$?" > "$RUN_ROOT/validation/family-verify-decision-contract.exit.txt"'
rg -n "frozen_helper_surface_floor|FrozenHelperSurfaceFloorMismatch|verify-decision-contract" xtask/src/family/verify.rs xtask/src/lib.rs > "$RUN_ROOT/validation/final-public-surface-grep.txt"
git diff --name-only > "$RUN_ROOT/validation/final-diff-name-only.txt"
git diff --stat > "$RUN_ROOT/validation/final-diff-stat.txt"
```

Artifacts written:

- all `validation/postfix-*`
- all `validation/family-*`
- `validation/final-public-surface-grep.txt`
- `validation/final-diff-name-only.txt`
- `validation/final-diff-stat.txt`

Blocked conditions:

- `cargo test -p xtask` fails
- final `recommend` or `corpus-decision` no longer matches the locked truthful stop-state
- final verifier command does not exit `0`
- outward public names drifted
- changed files exceed the locked file surface

Restart point if blocked:

- restart from `task-m42-30-verifier-parity-refresh` if the failure is inside the locked surface
- write `blocked.json` and stop if the failure implies real scope escape outside the locked surface

### `gate-m42-50-closeout`

Purpose:

- certify the run as complete or blocked
- finalize acceptance, scope, and restart semantics

Owned files:

- no source files
- `RUN_ROOT/**` only

Exact commands to run:

```bash
git rev-parse HEAD
git status --short
```

Artifacts written:

- `acceptance.md`
- `closeout.md`
- final `run-state.json`

Blocked conditions:

- baseline failure and final pass are not both recorded
- acceptance cannot be written without caveats against the locked outcome contract
- final diff scope is not cleanly explainable within the locked file surface

## Workstream Plan With Explicit Ownership

| Workstream | Purpose | Owner | Files |
|---|---|---|---|
| `WS0-baseline` | capture real pre-fix failure | parent | none; `RUN_ROOT/**` only |
| `WS1-contract` | add shared truthful stop-state helper | parent | `xtask/src/family/analysis_core/decision_contract.rs`, `xtask/src/family/analysis_core/mod.rs` |
| `WS2-verifier` | rewire verifier and refresh fixtures/tests | parent | `xtask/src/family/verify.rs` |
| `WS3-cli-audit` | audit dispatch surface, touch only if needed | parent | `xtask/src/lib.rs` conditionally |
| `WS4-proof` | run full proof loop and capture scope/public-surface evidence | parent | none; `RUN_ROOT/**` only |
| `WS5-closeout` | record acceptance and restart state | parent | none; `RUN_ROOT/**` only |

Rules:

- The parent owns every workstream.
- No workstream may be delegated.
- No workstream may run in parallel.
- `WS3-cli-audit` does not create a new lane; it is a conditional step inside the same serialized execution.

## Parent Context Budget

The parent should keep this live in working context:

- `PLAN.md`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/lib.rs`
- `TODOS.md`
- the run-state artifacts under `RUN_ROOT`

The parent must ignore:

- stale M41 orchestration content as authority
- skill-definition files
- unrelated repo subsystems
- packet or family authoring surfaces
- deferred `TODOS.md` items except as explicit anti-scope guards

Authority hygiene rules:

- The old `ORCH_PLAN.md` may be used only as a structural formatting baseline.
- M41 helper-surface orchestration does not define M42 scope, gates, or ownership.
- If stale orchestration language conflicts with `PLAN.md`, `PLAN.md` wins immediately.
- If the parent notices itself importing broader context than the locked file surface requires, it should trim context before continuing rather than compensating with more process.

## Failure / Restart Rules

Write `blocked.json` when:

- any gate’s blocked condition is met
- a required source edit escapes the locked file surface
- the truthful stop-state cannot be represented without widening policy code
- outward verifier public names would need to change to make M42 green
- baseline or final proof becomes ambiguous because of unrelated repo drift

Restart rules:

- If `gate-m42-20-contract-lock` fails, restart from `task-m42-10-shared-stop-state-contract`.
- If `gate-m42-40-full-proof-loop` fails inside the locked file surface, restart from `task-m42-30-verifier-parity-refresh`.
- If `gate-m42-40-full-proof-loop` fails because the solution now needs files outside the locked surface, do not restart within this plan; write `blocked.json` and reopen orchestration.

Real scope escapes that invalidate the run:

- needing `xtask/src/family/recommend.rs`
- needing `xtask/src/family/helper_surface.rs`
- needing packet or schema changes
- needing a public JSON key rename
- needing a failure-reason rename
- needing worker lanes to complete the milestone honestly

## Tests And Acceptance

### Required proof loop

Run exactly this loop before closeout:

```bash
cargo test -p xtask verify
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

### Acceptance contract

M42 is complete only if all of the following are true:

1. Baseline capture proves `cargo xtask family verify-decision-contract --format json` failed before the fix.
2. Final capture proves `cargo xtask family verify-decision-contract --format json` passes after the fix.
3. Final `recommend` output still shows:
   - `recommendation_status = insufficient_real_corpus`
   - `decision_status = not_recommended`
   - `open_blockers = []`
   - `missing_evidence = []`
   - `stale_evidence = []`
4. Final `corpus-decision` output still shows:
   - `decision_action = stop`
   - `decision_basis_code = no_actionable_candidate`
   - `required_next_action = record_stop_without_new_milestone`
5. `cargo test -p xtask verify` passes with refreshed stop-state fixtures and regression coverage.
6. `cargo test -p xtask` passes.
7. Changed production files stay within the locked file surface.
8. No outward JSON key rename slipped in.
9. No outward failure-reason rename slipped in.
10. The verifier still retains all five check planes, including `checks.frozen_helper_surface_floor`.

### Scope-guard acceptance

`validation/final-diff-name-only.txt` must show only:

- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/lib.rs` only if needed

Any other production file means the run is blocked until orchestration is rewritten.

## Assumptions

- `PLAN.md` remains the sole authority for M42 throughout the run.
- The truthful live decision state is already the locked stop-state tuple above.
- The false failure is isolated to verifier stale frozen-floor logic.
- The public verifier surface should remain stable even if internal names become more honest.
- `xtask/src/lib.rs` will likely remain untouched, but a minimal dispatch-test refresh is allowed.
- Deferred `TODOS.md` items such as generalized multi-wedge decision logic and cross-crate family-analysis shared core remain deferred and must not be partially implemented here.
- No honest benefit exists from worker lanes, staging branches, or additional worktrees for this milestone.
- If any assumption breaks, stop and rewrite orchestration before widening scope.
