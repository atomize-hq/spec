# M27.8R Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Primary branch baseline: **`feat/corpus-expansion`**  
Frozen run artifacts: **`.runs/m27_8/acceptance.md`, `.runs/m27_8/merge-log.md`, `.runs/m27_8/contract-freeze.json`**  
Last rewritten: **2026-05-01**

## Summary

- Execute from the live branch `feat/corpus-expansion`. Treat `PLAN.md` as the implementation contract and the `.runs/m27_8/*` records as the frozen oracle for proof order, recovery commits, and stop conditions.
- Keep the parent agent as the only integrator. The parent owns baseline capture, worker launch, queue state, lane merges, the exact proof loop, acceptance recording, and any blocked closeout.
- Use one narrow parallel window only after baseline capture:
  - Lane A: recover lane-A source truth in `examples/crosslib-app/units/` from `ab11249` / `ws/m27_8-lane-a`
  - Lane B: preserve ranked xtask lock shape in `xtask/src/lib.rs`, add `semantic-families/function.wrapper.pipeline.v1` to the seeded copy list, and keep the test bound to the frozen truth from `7ae58ae` / `ws/m27_8-lane-b`
- Keep lane C sequential and parent-owned. Lane C merges A+B, runs the frozen proof loop from `.runs/m27_8/contract-freeze.json.required_build_order`, and either lands the repair or captures seeded temp-workspace evidence and stops.
- Recommended worktree layout for this run:
  - parent/control: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/corpus-expansion`
  - lane A: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-a` on `ws/m27_8r-lane-a`
  - lane B: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-b` on `ws/m27_8r-lane-b`
  - integration: `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/int` on `ws/m27_8r-int`
- Use GPT-5.4 with `reasoning_effort=high` for both workers. Cap concurrency at `2`.
- Distinguish source from derived state:
  - authored source: exactly three tracked files
  - derived or run-state: passports, generated output, `.semantic-family-artifacts/**`, and parent-owned `.runs/m27_8/**`

## Parent-Owned Run-State Protocol

Canonical run root: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m27_8`

Frozen oracle inputs. Read, cite, preserve:

- `acceptance.md`
  - blocked integrated-run oracle from the prior attempt
  - source for the exact failing invariant, proof results at stop, and locked artifact truth
- `merge-log.md`
  - frozen recovery-source ledger
  - source for `ab11249` and `7ae58ae` ownership and disposition
- `contract-freeze.json`
  - frozen implementation oracle
  - source for the locked touch set, locked `apply_tax` shape, locked `.gitignore` line, locked coverage/recommendation deltas, and required build order

Mutable current-run state. Parent-owned, writable for this execution only:

- `baseline.json`
  - current branch, HEAD SHA, timestamp, and baseline inputs re-read at run start
- `dirty-state.json`
  - exact `git status --short` snapshot at run start, plus any allowlisted pre-existing local state the parent intentionally preserved
- `tasks.json`
  - authoritative task queue and dependency graph for this run
- `session-log.md`
  - append-only run diary for parent decisions, worker launch/return, command outcomes, and stop/land disposition
- `diagnostics/*`
  - current-run diagnostic bundle only
  - empty or absent on a green run
  - populated only if WS-D blocked closeout is triggered

Input/output rule:

- frozen oracle inputs are read-only unless the parent is deliberately recording a new final blocked or landed state for a successor run
- mutable current-run state is parent-owned and may be updated during execution
- workers do not write `.runs/m27_8/**`

## Hard Guards

- `PLAN.md` wins over `ORCH_PLAN.md`, worker suggestions, memory, or stale run notes.
- Current repo reality must be preserved, not normalized. Baseline capture should record the live dirty state before branching. As of rewrite time, `git status --short` shows ` M PLAN.md`.
- Do not revert or overwrite edits you did not make. Anything outside the locked M27.8R touch set is preserve-by-default.
- The locked authored source touch set is exactly:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `examples/crosslib-app/units/.gitignore`
  - `xtask/src/lib.rs`
- The parent may refresh derived proof surfaces during lane C, but workers must not own them:
  - `examples/crosslib-app/units/**/*.spec.passport.json`
  - `examples/shared-spec/units/**/*.spec.passport.json`
  - `examples/shared-crate/src/generated/**`
  - `examples/crosslib-app/src/generated/**`
  - `.semantic-family-artifacts/family-promotion/analysis/**`
  - `.runs/m27_8/**`
- Explicit non-touch source surfaces remain locked unless the diagnostic stop gate proves the contract wrong:
  - `semantic-families/corpus/rust-function.toml`
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/inventory.rs`
  - `.runs/m27_8/*` historical freeze artifacts
- No new harness framework, fixture registry, recommendation policy change, coverage policy change, or corpus expansion work is allowed in this milestone.
- No second speculative fix pass. If the final `cargo test -p xtask -- --color never` still diverges after the packet-root repair, capture seeded temp-workspace evidence from inside the failing test path and stop.
- Workers do not self-merge, rewrite the plan, or invent new owned paths. The parent either merges lane output as-is or bounces it back with a precise correction.

## Source Vs Derived Surfaces

Authored source surfaces for M27.8R:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`
- `xtask/src/lib.rs`

Derived or run-state surfaces for M27.8R:

- `examples/crosslib-app/units/**/*.spec.passport.json`
- `examples/shared-spec/units/**/*.spec.passport.json`
- `examples/shared-crate/src/generated/**`
- `examples/crosslib-app/src/generated/**`
- `.semantic-family-artifacts/family-promotion/analysis/**`
- `.runs/m27_8/**`

Execution rule:

- workers edit authored source only, within their owned path set
- derived surfaces are regenerated only in WS-C by the parent after lane merge
- workers never refresh passports, generated code, coverage artifacts, recommendation artifacts, or `.runs/m27_8/**`
- `acceptance.md` and `contract-freeze.json` are oracle artifacts, not casual logging surfaces and not worker outputs

## Task Graph

```text
task/m27_8r-00-baseline
  -> task/m27_8r-a1-freeze-run-contract
      -> task/m27_8r-b1-lane-a-recovery
      -> task/m27_8r-b2-lane-b-harness
task/m27_8r-b1-lane-a-recovery
task/m27_8r-b2-lane-b-harness
  -> task/m27_8r-c1-integrate-and-proof
      -> task/m27_8r-c2-land-or-stop
```

Execution intent:

1. parent records current branch reality and current-run state
2. parent freezes the run contract for this execution and writes worker prompts
3. lane A and lane B execute in parallel from the same live baseline
4. parent merges both lanes into integration
5. parent alone regenerates derived proof surfaces and runs the exact proof loop
6. parent either lands the repair or writes a blocked diagnostic closeout and stops

## Workstream Plan

### Critical Path

`WS-0 baseline/freeze -> WS-A lane-A recovery + WS-B lane-B harness repair (parallel) -> WS-C parent integration/proof -> land or stop`

Parallelism is useful but intentionally narrow. Lane A and lane B touch disjoint source areas and can run safely in parallel. Everything else is sequential because the expensive and authoritative work is the merged proof loop, not the edits themselves.

### Parallelism Boundary

No third worker lane is justified.

- there are only three authored source files in scope
- lane A and lane B already isolate the only disjoint source ownership boundary
- every derived surface refresh and every acceptance gate depends on both lanes being merged first
- a separate proof, artifact, or diagnostics worker would either duplicate parent context or violate the parent-only integrator rule
- additional lane count adds coordination cost without reducing the critical path

### WS-0 Parent Baseline And Freeze

Parent only. Do not create a separate contract worktree for this run; the contract is already frozen in `PLAN.md` plus `.runs/m27_8/*`, so another control branch adds ceremony without reducing risk.

Task ID: `task/m27_8r-00-baseline`, then `task/m27_8r-a1-freeze-run-contract`

Owned paths:

- `.runs/m27_8/baseline.json`
- `.runs/m27_8/dirty-state.json`
- `.runs/m27_8/tasks.json`
- `.runs/m27_8/session-log.md`
- `.runs/task-m27_8r-00-baseline/**`
- `.runs/task-m27_8r-a1-freeze-run-contract/**`

Required commands / policy:

- parent must run baseline capture commands before any new worktree is created
- parent must read `PLAN.md`, `.runs/m27_8/acceptance.md`, `.runs/m27_8/merge-log.md`, and `.runs/m27_8/contract-freeze.json` before writing `baseline.json`
- no source edits are allowed in WS-0

1. Record live baseline in `.runs/m27_8/` before any implementation branch is created.
   Suggested parent-owned files:
   - `.runs/m27_8/baseline.json`
   - `.runs/m27_8/dirty-state.json`
   - `.runs/m27_8/tasks.json`
   - `.runs/m27_8/session-log.md`
   - `.runs/m27_8/diagnostics/README.md`
2. Capture:
   - `git rev-parse HEAD`
   - `git branch --show-current`
   - `git status --short`
   - confirmation that `PLAN.md`, `.runs/m27_8/acceptance.md`, `.runs/m27_8/merge-log.md`, and `.runs/m27_8/contract-freeze.json` were re-read
3. Treat these as read-only recovery sources:
   - lane-A merge commit `ab11249`
   - lane-B merge commit `7ae58ae`
   - contract freeze commit `b56a7513b96efb6c4e6d554b42163e7eb97ab4af`
4. Create execution worktrees from the live current `feat/corpus-expansion` HEAD, not from the older freeze commit. The freeze commit stays the oracle for expected truth; the live branch stays the execution baseline.
5. Suggested commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r
git worktree add -b ws/m27_8r-lane-a /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-a HEAD
git worktree add -b ws/m27_8r-lane-b /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-b HEAD
git worktree add -b ws/m27_8r-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/int HEAD
```

6. Seed `.runs/m27_8/tasks.json` with one parent-owned queue and one sentinel directory per task. Suggested task IDs:
   - `task/m27_8r-00-baseline`
   - `task/m27_8r-a1-freeze-run-contract`
   - `task/m27_8r-b1-lane-a-recovery`
   - `task/m27_8r-b2-lane-b-harness`
   - `task/m27_8r-c1-integrate-and-proof`
   - `task/m27_8r-c2-land-or-stop`

WS-0 acceptance:

- `baseline.json` records branch, HEAD SHA, timestamp, and re-read oracle inputs
- `dirty-state.json` records the pre-run dirty tree, including the existing `PLAN.md` modification
- `tasks.json` contains the current-run execution graph and owned-path contracts
- worktrees exist for lane A, lane B, and integration
- no source file outside `.runs/**` changed during WS-0
- worker prompts are written only after baseline and task records exist

### Run-Contract Freeze Record

`task/m27_8r-a1-freeze-run-contract` is not a source-freeze rewrite. It is a parent-authored execution record inside current-run state that says:

- the live execution baseline is current `feat/corpus-expansion` HEAD
- frozen oracle inputs are `PLAN.md`, `acceptance.md`, `merge-log.md`, and `contract-freeze.json`
- lane A owns the two crosslib recovery files
- lane B owns `xtask/src/lib.rs`
- lane C owns all derived refresh and acceptance work
- the stop gate is the first unexplained seeded-workspace mismatch after the packet-root fix

### WS-A Lane-A Recovery

Worker 1 on `ws/m27_8r-lane-a` in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-a`.

Task ID: `task/m27_8r-b1-lane-a-recovery`

Owned files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`

Required commands / policy:

- Recover exact authored source truth from `ab11249` or `ws/m27_8-lane-a`.
- Do not re-author a similar unit. The file should be a literal recovery aligned to `.runs/m27_8/contract-freeze.json.locked_apply_tax_shape`.
- Restore exactly the whitelist line `!pricing/apply_tax.spec.passport.json` in `examples/crosslib-app/units/.gitignore`.
- Do not run the full proof loop. Lane A is a source-recovery lane, not an integration lane.
- worker may use read-only diff commands to verify against `ab11249`
- worker must not write passports, generated output, or `.runs/**`

Lane A acceptance:

- diff against `ab11249` for both owned files is empty or explainable only by harmless line-ending normalization
- owned paths and only owned paths changed
- worker summary cites the exact recovery source used

### WS-B Lane-B Harness Repair

Worker 2 on `ws/m27_8r-lane-b` in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/lane-b`.

Task ID: `task/m27_8r-b2-lane-b-harness`

Owned file:

- `xtask/src/lib.rs`

Required commands / policy:

- Start from the ranked lock semantics already proven in `7ae58ae` / `ws/m27_8-lane-b`.
- Repair `seed_locked_recommendation_workspace()` by adding the copied packet root:
  - `semantic-families/function.wrapper.pipeline.v1`
- Add one short comment above the seeded copy list clarifying that promoted packet roots are part of command-path inventory truth in the seeded workspace.
- Preserve the locked command-path assertions from `.runs/m27_8/contract-freeze.json`:
  - source IDs: `examples_ecommerce`, `m19_semantic_falsification_pack`, `m20_unsupported_truth_pack`, `examples_shared_spec`, `examples_crosslib_app`
  - source counts: `6 / 12 / 9 / 1 / 2`
  - function coverage: `28 / 15 / 0 / 13`
  - recommendation status `ranked`
  - arithmetic cluster first and `ready`
  - `money/round` second and `hold`
- Do not add diagnostic capture code proactively. Diagnostic edits are reserved for the parent if the integrated rerun still fails.
- worker may run narrow local verification on `xtask/src/lib.rs` behavior, but must not run the full parent proof loop or write `.runs/**`

Lane B acceptance:

- `xtask/src/lib.rs` is the only changed source file
- ranked lock shape remains intact
- the missing `function.wrapper.pipeline.v1` packet root is present in the seeded copy list
- no edits appear in `xtask/src/family/**`

### WS-C Parent Integration And Proof

Parent only on `ws/m27_8r-int` in `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8r/int`.

Task ID: `task/m27_8r-c1-integrate-and-proof`

Owned paths:

- derived surfaces only:
  - `examples/crosslib-app/units/**/*.spec.passport.json`
  - `examples/shared-spec/units/**/*.spec.passport.json`
  - `examples/shared-crate/src/generated/**`
  - `examples/crosslib-app/src/generated/**`
  - `.semantic-family-artifacts/family-promotion/analysis/**`
- current-run state:
  - `.runs/m27_8/session-log.md`
  - `.runs/m27_8/diagnostics/**` if blocked
  - `.runs/task-m27_8r-c1-integrate-and-proof/**`
  - `.runs/task-m27_8r-c2-land-or-stop/**`

Allowed integration mechanics:

- merge lane A and lane B branches
- resolve straightforward merge mechanics in integration-owned or worker-owned touched files when the final text is already determined by the frozen contract
- discard worker drift outside owned paths
- regenerate derived surfaces through the locked proof loop

Forbidden integration behavior:

- no new substantive source edits outside the three-file contract
- no creative source reconciliation when lane output conflicts with the frozen contract
- no widening into `xtask/src/family/**`, corpus manifests, docs, or new helpers
- no worker-owned source edits introduced by the parent after merge unless the parent is explicitly performing blocked diagnostics inside WS-D

Failure policy before the final xtask lock:

- if a failure is an obvious integration mechanic issue in a derived surface or merge artifact, the parent may correct it and rerun within WS-C
- if a failure implies a new substantive source change outside the locked contract, stop immediately and close out blocked
- if a failure occurs on one of the pre-xtask proof commands because lane output itself is wrong, reject the offending lane result rather than patching around it in integration

1. Merge lane A and lane B output into the integration worktree.
2. Reject any worker drift outside the locked owned paths.
3. Run the frozen proof loop in the exact order from `.runs/m27_8/contract-freeze.json.required_build_order`:

```bash
cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo test --manifest-path examples/crosslib-app/Cargo.toml

cargo xtask family coverage --format json > /tmp/m27_8r-coverage.stdout.json
cmp -s /tmp/m27_8r-coverage.stdout.json .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json

cargo xtask family recommend --format json > /tmp/m27_8r-recommend.stdout.json
cmp -s /tmp/m27_8r-recommend.stdout.json .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

cargo test -p xtask -- --color never
```

4. Record the result in:
   - `.runs/m27_8/session-log.md`
   - `.runs/m27_8/acceptance.md` only if the parent is intentionally updating the current run outcome
5. If the full loop goes green, close the run with the parent as the final integrator and keep the worker branches disposable.

Merge acceptable:

- lane A changes only its two owned files
- lane B changes only `xtask/src/lib.rs`
- merged tree contains no unexpected source drift
- any merge conflict is mechanical and resolved directly to the frozen contract truth

Proof acceptable:

- all commands in the required build order pass in sequence
- coverage stdout bytes equal the checked artifact bytes
- recommendation stdout bytes equal the checked artifact bytes
- both analysis artifacts validate
- final `cargo test -p xtask -- --color never` passes with the locked truth

Blocked termination acceptable:

- session log records the first failing command and exit behavior
- current-run diagnostics bundle is written if and only if the failure is at the final xtask seeded command-path lock
- frozen oracle files are preserved, not rewritten
- parent stops after the first unexplained seeded mismatch

### WS-D Diagnostic Stop Gate

Parent only, and only if WS-C fails at the final `xtask` lock.

Task ID: `task/m27_8r-c2-land-or-stop`

1. Confirm the failure is still on the seeded command-path truth, especially promoted-family count drift.
2. Add temporary diagnostic capture inside the failing test path against `temp_dir.path()`, not repo-root state.
3. Capture the required evidence surfaces named by `PLAN.md`:
   - `inventory::render_snapshot_bytes(temp_dir.path())`
   - `FAMILY_COVERAGE_LATEST_PATH` under `temp_dir.path()`
   - `FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH` under `temp_dir.path()`
4. First write them to the `/tmp/m27_8r-*` files required by the implementation contract, then copy them into:
   - `.runs/m27_8/diagnostics/seeded-inventory.json`
   - `.runs/m27_8/diagnostics/seeded-coverage.json`
   - `.runs/m27_8/diagnostics/seeded-recommendation.json`
   - `.runs/m27_8/diagnostics/stop-summary.md`
5. Revert any temporary diagnostic code before declaring the run blocked, unless the captured evidence proves that the diagnostic helper itself must become permanent.
6. Stop. Do not invent a second missing input path in the same session.

Blocked closeout requirements:

- write `blocked.json` sentinel for `task/m27_8r-c2-land-or-stop`
- preserve the integration tree and current-run diagnostics bundle for handoff
- preserve frozen oracle files unchanged:
  - `.runs/m27_8/acceptance.md`
  - `.runs/m27_8/merge-log.md`
  - `.runs/m27_8/contract-freeze.json`
- do not rewrite recommendation or coverage policy to make the failure disappear
- do not start a second source-edit round in the same run

## Context-Control Rules

- Parent working set stays intentionally small:
  - `PLAN.md`
  - `.runs/m27_8/acceptance.md`
  - `.runs/m27_8/merge-log.md`
  - `.runs/m27_8/contract-freeze.json`
  - current `tasks.json`
  - latest integration diff summary
- Worker prompts should include only:
  - owned paths
  - exact recovery source or locked assertion set
  - forbidden surfaces
  - required commands
  - explicit stop conditions
  - branch/worktree path
  - model requirement: GPT-5.4 with `reasoning_effort=high`
- Workers should return only:
  - changed files
  - commands run with exit codes
  - blockers
  - any mismatch between recovery source and current branch reality
- Parent writes all queue and sentinel artifacts. Suggested sentinel layout:
  - `.runs/task-m27_8r-00-baseline/started.json`
  - `.runs/task-m27_8r-b1-lane-a-recovery/status.json`
  - `.runs/task-m27_8r-b2-lane-b-harness/status.json`
  - `.runs/task-m27_8r-c1-integrate-and-proof/done.json`
  - `.runs/task-m27_8r-c2-land-or-stop/blocked.json`
- Worker transcripts are not durable state. Durable state lives in the code diff plus parent-owned `.runs/m27_8/**`.
- Close worker sessions immediately after merge or rejection. Do not accumulate stale worker context across lanes.

## Sentinel Protocol

Sentinel directories are parent-owned and live under `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-*/`.

Standard files:

- `started.json`
  - written once when the task begins
- `status.json`
  - updated by the parent as the task progresses
- `done.json` or `blocked.json`
  - terminal record for the task

Workers report status to the parent. The parent writes all sentinel files.

Minimum `blocked.json` shape:

- `task_id`
- `blocked_at`
- `branch`
- `worktree`
- `reason`
- `failed_command`
- `required_replan`
- `touched_files`
- `preserved_artifacts`

Recommended `status.json` shape:

- `task_id`
- `status`
- `owner`
- `owned_paths`
- `last_update`
- `notes`

## Tests And Acceptance

- Baseline gate:
  - branch is `feat/corpus-expansion`
  - baseline SHA and current dirty state are recorded before branching
  - existing dirty state outside the touch set is preserved
- Lane A gate:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec` matches frozen lane-A truth
  - `examples/crosslib-app/units/.gitignore` restores `!pricing/apply_tax.spec.passport.json`
- Lane B gate:
  - `xtask/src/lib.rs` retains the ranked command-path lock from lane B
  - seeded workspace copy list includes `semantic-families/function.wrapper.pipeline.v1`
  - no source drift outside `xtask/src/lib.rs`
- Integration proof gate:
  - shared-spec build passes
  - `spec test` for `examples/crosslib-app/units/pricing/apply_tax.unit.spec` passes
  - crosslib build passes
  - `cargo test --manifest-path examples/crosslib-app/Cargo.toml` passes
- Artifact equality gate:
  - coverage stdout bytes match `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - recommendation stdout bytes match `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - both artifacts validate with `cargo xtask family validate-artifact`
- Final xtask lock gate:
  - `cargo test -p xtask -- --color never` passes
  - the seeded command-path lock observes:
    - function coverage `28 / 15 / 0 / 13`
    - recommendation status `ranked`
    - first candidate arithmetic cluster `ready`
    - second candidate `money/round` cluster `hold`
- Stop gate:
  - if the final xtask lock still fails after the packet-root fix, capture seeded temp-workspace evidence from inside the test, persist the diagnostic bundle, and stop without a second speculative code edit

## Assumptions

- `PLAN.md` is already the single M27.8R implementation contract and will remain so during execution. If it changes materially mid-run, the parent should pause and re-read before merging lanes.
- The recovery references remain available locally:
  - `ab11249`
  - `7ae58ae`
  - `ws/m27_8-lane-a`
  - `ws/m27_8-lane-b`
- The accepted causality model in `PLAN.md` is correct enough to justify a first-pass fix limited to the missing promoted packet root in the seeded workspace.
- The only safe useful parallelism is lane A plus lane B. Lane C stays parent-owned because derived artifact refresh and the final xtask truth lock are the real acceptance surface.
- Existing unrelated workspace edits are intentional unless proven otherwise. The parent should record them, avoid them, and not try to clean the tree.
- A successful run does not require inventing any new source touch surface beyond the three-file contract. If it does, this orchestration plan is no longer authoritative and a new plan is required.
