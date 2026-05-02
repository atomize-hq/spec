# M27.9A Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Primary branch baseline: **`feat/corpus-expansion`**  
Frozen evidence basis: **`.runs/m27_9/session-log.md`, `.runs/m27_9/diagnostics/blocked-summary.md`, `.runs/m27_9/diagnostics/coverage.actual.json`, `.runs/m27_9/diagnostics/recommendation.actual.json`**  
Authoritative source worktree: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int`**  
Last rewritten: **2026-05-02**

## Summary

- Execute from the live branch `feat/corpus-expansion`. `PLAN.md` is the only implementation authority. `ORCH_PLAN.md` defines execution order, workstream ownership, integration mechanics, and stop conditions only.
- M27.9A is a closeout-and-recalibration run. It does four things and only four things:
  - land the already-authored M27.9 source truth from the integration worktree
  - reproduce the blocked stop-path evidence on `feat/corpus-expansion`
  - refresh the locked `xtask` analysis contract to the truthful post-fix baseline
  - rewrite the planning and program ledger so the repo records M27.9 as implementation success plus accounting failure
- The truthful target state is:
  - `function_coverage = 28 / 17 / 0 / 11`
  - `recommendation_status = "no_strong_candidate"`
  - no ranked arithmetic-ready candidate remains
  - `unsupported_function_surface-e40675da6fa0` remains the visible held candidate for `unknown_overlap_family`, concretely `money/round`
- The authoritative authored source truth lives in the live content of `ws/m27_9-int`, not merely its committed tip. Source landing must read from `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int`.
- Use existing worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/{int,m20-cli,docs}` as provisional inputs, not assumed-ready execution lanes:
  - `ws/m27_9-int`
  - `ws/m27_9-m20-cli`
  - `ws/m27_9-docs`
- The current live state matters:
  - `ws/m27_9-int` is authoritative because its live worktree contains the authored source delta, even though it is dirty by design with both authored and derived changes.
  - `ws/m27_9-m20-cli` may remain dirty because it is reference-only.
  - `ws/m27_9-docs` is not reusable as-is for Lane B if it is still pinned to the frozen `cc12c859d99d409a4f861be64b9d7df7a653caba` baseline or still carries the stray `semantic-families/README.md` change.
- Use GPT-5.4 with `reasoning_effort=high` for workers. Cap worker concurrency at `1`. The parent remains the sole integrator and the only writer of run-state and derived proof surfaces.
- Maximum safe concurrency is intentionally narrow:
  - Lane A is parent-only and serialized because it owns branch truth, source import, and reproduced stop-state proof.
  - Lane B is the only safe concurrent worker lane before reproduced stop-state because its scope is bounded to docs/program closeout wording over frozen evidence.
  - A second post-reproduction worker lane is intentionally forbidden. After reproduced stop-state, the only remaining authored code surface is `xtask/src/lib.rs`, which must lock exactly the parent-produced outputs and would gain nothing from a second integration cycle.
- `ws/m27_9-m20-cli` may be inspected for corroboration from the earlier blocked run, but it is not an authoritative source of truth for M27.9A source landing.

## Hard Guards

- `PLAN.md` wins over memory, stale orchestration notes, worker suggestions, and previous run summaries.
- The old `28 / 18 / 0 / 10` expectation is historical diagnosis only. It is never an execution target in M27.9A.
- Source landing must import the exact authored source set from the live `ws/m27_9-int` worktree content:
  - `spec-core/src/semantic_review.rs`
  - delete `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
  - add `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_control_flow.unit.spec`
  - `spec-cli/tests/cli.rs`
  - `semantic-families/README.md`
- Source landing mechanism is locked:
  - use direct file sync or patch-copy of the exact authored files above from `ws/m27_9-int`
  - do not use branch merge as the landing mechanism
  - do not use whole-worktree checkout or blanket sync
  - do not cherry-pick `ws/m27_9-int` as a branch-level unit, because the authoritative truth includes uncommitted worktree content and excludes derived passports
- Recalibration ownership is limited to:
  - `xtask/src/lib.rs`
  - `PLAN.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
- Derived proof surfaces are downstream only. Do not hand-copy, stage as authored truth, or use as source-import inputs:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `examples/shared-crate/src/generated/**`
  - `examples/crosslib-app/units/pricing/*.spec.passport.json`
  - `.runs/m27_9/**`
- The two modified passport files visible in `ws/m27_9-int` are derived proof only:
  - `examples/crosslib-app/units/pricing/apply_discount.spec.passport.json`
  - `examples/crosslib-app/units/pricing/apply_tax.spec.passport.json`
  They must not be hand-copied into `feat/corpus-expansion`.
- Lane B may not start from a stale or dirty docs worktree. Before any worker writes docs/program wording, the parent must prove one of these states:
  - the existing `ws/m27_9-docs` worktree is rebased or recreated from the current `feat/corpus-expansion` head and is clean except for Lane B owned paths, or
  - a replacement docs worktree at the same path/role has been created from the current `feat/corpus-expansion` head.
- If `ws/m27_9-docs` contains changes outside `PLAN.md` and `docs/recommendation_corpus_expansion_program_v0.1.md`, reject it as a worker base and recreate or cleanly replace it before Lane B starts.
- `PLAN.md` is not open for a fresh rewrite. Lane B may adjust closeout wording only so the file accurately records:
  - implementation success
  - accounting failure
  - truthful target state
  - truthful next-step pressure
  It must not reopen scope, redefine steps `0-5`, or invent new milestone work.
- No new family packet.
- No new corpus run.
- No recommendation-policy rewrite.
- No artifact schema rewrite.
- No shared-core or M28 work.
- `xtask/src/lib.rs` is blocked until Lane A confirms reproduced branch truth is exactly:
  - `function_coverage = 28 / 17 / 0 / 11`
  - `recommendation_status = "no_strong_candidate"`
- If reproduced branch truth does not match that stop-state exactly:
  - stop M27.9A before editing `xtask/src/lib.rs`
  - keep Lane B draft-only
  - do not finalize closeout wording as settled repo truth
  - write machine-evidence diagnostics under `.runs/m27_9/diagnostics/`
  - require a fresh planning decision instead of forcing the lock or hand-explaining the drift away

## Source Vs Derived Surfaces

### Authored source truth that must land

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec` deleted
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_control_flow.unit.spec` added
- `spec-cli/tests/cli.rs`
- `semantic-families/README.md`

### Recalibration surfaces owned by M27.9A

- `xtask/src/lib.rs`
- `PLAN.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

### Current authoritative integration-worktree status to import from

- `M semantic-families/README.md`
- `M spec-cli/tests/cli.rs`
- `D spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
- `?? spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_control_flow.unit.spec`
- `M spec-core/src/semantic_review.rs`
- `M examples/crosslib-app/units/pricing/apply_discount.spec.passport.json`
- `M examples/crosslib-app/units/pricing/apply_tax.spec.passport.json`

Import rule:

- The first five entries above are the authored source delta.
- The last two entries are derived proof and excluded from source landing.

### Downstream-only proof surfaces

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `examples/shared-crate/src/generated/**`
- `examples/crosslib-app/units/pricing/*.spec.passport.json`
- `.runs/m27_9/**`

## Parent-Owned Run-State Protocol

Canonical roots:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `M27_9_RUN_ROOT=$PRIMARY_ROOT/.runs/m27_9`
- `M27_9_ANALYSIS_ROOT=$PRIMARY_ROOT/.semantic-family-artifacts/family-promotion/analysis`
- `M27_9_INT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/int`
- `M27_9_DOCS_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/docs`
- `M27_9_M20CLI_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_9/m20-cli`

Parent-owned mutable run-state:

- queue: `$M27_9_RUN_ROOT/tasks.json`
- session log: `$M27_9_RUN_ROOT/session-log.md`
- baseline snapshot: `$M27_9_RUN_ROOT/baseline.json`
- frozen evidence acceptance record: `$M27_9_RUN_ROOT/evidence-acceptance.json`
- source import contract and completion record: `$M27_9_RUN_ROOT/source-import.json`
- reproduced stop-state record: `$M27_9_RUN_ROOT/reproduced-stop-state.json`
- integration record: `$M27_9_RUN_ROOT/integration-state.json`
- final proof record: `$M27_9_RUN_ROOT/final-proof.json`
- diagnostics: `$M27_9_RUN_ROOT/diagnostics/**`
- per-task sentinels: `$PRIMARY_ROOT/.runs/<TASK_ID>/`

Required queue and log exactness:

- `tasks.json` is seeded before any authored file edit and records for each task:
  - `id`
  - `owner`
  - `worktree`
  - `depends_on`
  - `owned_paths`
  - `status`
  - `gate`
- `session-log.md` is append-only and records:
  - timestamp
  - task id
  - actor
  - branch or worktree
  - commands run
  - exit codes
  - stop reason or completion note
- `baseline.json` records:
  - live branch name
  - live `git rev-parse HEAD`
  - live `git status --short`
  - freeze-time blocked-run HEAD `cc12c859d99d409a4f861be64b9d7df7a653caba`
- `source-import.json` is initialized before edits with:
  - expected authored source paths
  - excluded derived paths
  - mechanism `pending_direct_file_sync`
  and completed after landing with:
  - actual imported paths
  - actual excluded paths
  - source worktree
  - completion timestamp

Frozen evidence basis that must be copied into run-state, not paraphrased from memory:

- freeze-time observed HEAD: `cc12c859d99d409a4f861be64b9d7df7a653caba`
- semantic gate passed in `spec-core`
- worker lane B repaired M20 plus CLI truth
- worker lane C updated `semantic-families/README.md`
- integration gate passed on `ws/m27_9-int` for `fmt + spec-core + spec-cli`
- parent refreshed derived proof surfaces needed for merged CLI truth
- stop rule triggered because actual coverage was `28 / 17 / 0 / 11` while the old expected coverage was wrong
- actual recommendation was `no_strong_candidate`
- no `xtask/src/lib.rs` edit was made in the blocked run
- current parent workspace still lacks the full merged source truth

Run-state rules:

- The parent writes all `.runs/m27_9/**` files.
- Workers may read run-state but never write it.
- The parent records both:
  - the frozen blocked-run basis commit
  - the live execution-start commit on `feat/corpus-expansion`
- If those differ in ways that can explain a reproduction mismatch, the parent records drift explicitly before any `xtask` work begins.

## Task Graph

```text
task/m27_9a-00-kickoff
  -> task/m27_9a-01-seed-run-state
      -> task/m27_9a-02-accept-frozen-evidence
          -> task/m27_9a-03-verify-worktree-viability
              -> task/m27_9a-a1-import-authored-source-truth
              -> task/m27_9a-a2-land-source-truth-on-parent
                  -> task/m27_9a-a3-reproduce-stop-state
                      -> task/m27_9a-b2-finalize-ledger-wording
                          -> task/m27_9a-i1-integrate-docs-lane
                              -> task/m27_9a-i2-pre-xtask-integration-gate
                                  -> task/m27_9a-c1-refresh-xtask-lock
                                      -> task/m27_9a-d1-final-proof
                                          -> task/m27_9a-d2-closeout-or-stop

task/m27_9a-03-verify-worktree-viability
  -> task/m27_9a-b0-prepare-docs-lane
      -> task/m27_9a-b1-draft-ledger-rewrite
      -> task/m27_9a-b2-finalize-ledger-wording
```

Execution meaning:

1. WS-0 seeds the run-state and freezes the kickoff contract before any authored file edit.
2. WS-0 also proves that each reused worktree is viable for its role and repairs or recreates the docs lane before any worker writes against it.
3. Lane A imports the live `ws/m27_9-int` authored truth by direct file sync, lands it on `feat/corpus-expansion`, and proves reproduced stop-state.
4. Lane B may draft bounded closeout wording in parallel once frozen evidence is accepted and the docs lane has been prepared, but it cannot finalize or merge until Lane A confirms reproduced stop-state.
5. WS-INT is a dedicated parent-owned integration phase on `feat/corpus-expansion`. It integrates Lane B back into the already-landed parent truth and runs a pre-`xtask` integration gate.
6. Lane C remains parent-only and blocked until source truth is landed, stop-state is reproduced, and integration is complete.
7. If any gate fails after Lane B exists, the parent writes deterministic blocked-closeout artifacts instead of reopening scope informally.

## Workstream Plan

### WS-0 Kickoff And Run-State Seed (`feat/corpus-expansion`) - parent only

Task IDs:

- `task/m27_9a-00-kickoff`
- `task/m27_9a-01-seed-run-state`
- `task/m27_9a-02-accept-frozen-evidence`
- `task/m27_9a-03-verify-worktree-viability`

Required parent actions:

1. Re-read:
   - `PLAN.md`
   - `ORCH_PLAN.md`
   - `.runs/m27_9/session-log.md`
   - `.runs/m27_9/diagnostics/blocked-summary.md`
   - `.runs/m27_9/diagnostics/coverage.actual.json`
   - `.runs/m27_9/diagnostics/recommendation.actual.json`
2. Record in `baseline.json`:
   - live branch name
   - live `git rev-parse HEAD`
   - live `git status --short`
   - freeze-time HEAD `cc12c859d99d409a4f861be64b9d7df7a653caba`
3. Seed `tasks.json` with the full task graph, owners, dependencies, and owned-path contracts.
4. Initialize `source-import.json` with:
   - expected authored source paths
   - excluded derived paths
   - mechanism `pending_direct_file_sync`
5. Write `evidence-acceptance.json` that locks the accepted frozen story:
   - implementation success
   - accounting failure
   - truthful target state `28 / 17 / 0 / 11`
   - truthful recommendation `no_strong_candidate`
   - surviving held candidate `unsupported_function_surface-e40675da6fa0`
6. Inspect all three existing worktrees directly. Do not substitute committed branch state for live worktree content.
   - `ws/m27_9-int` status is captured as source-authority input.
   - `ws/m27_9-m20-cli` status is captured as reference-only corroboration.
   - `ws/m27_9-docs` status is captured as lane-viability input.
7. Record in run-state the branch, HEAD, and dirty-path snapshot for each worktree, plus an explicit viability decision:
   - `ws/m27_9-int = authoritative_dirty_allowed`
   - `ws/m27_9-m20-cli = reference_only_dirty_allowed`
   - `ws/m27_9-docs = ready|needs_recreation|needs_cleanup`
8. If `ws/m27_9-docs` is not already based on the current `feat/corpus-expansion` HEAD or contains out-of-scope dirty paths, prepare Lane B before any worker starts by recreating or cleanly replacing the docs worktree from current `feat/corpus-expansion`.
   - The prepared docs lane must begin from the current M27.9A `PLAN.md`, not the frozen M27.9 plan text.
   - The prepared docs lane must be clean before the worker edits owned files.

WS-0 acceptance:

- `tasks.json`, `baseline.json`, `evidence-acceptance.json`, and initialized `source-import.json` all exist before any authored file edit
- frozen evidence is captured as machine-readable parent-owned run-state
- the authoritative integration-worktree status is captured before import begins
- worktree viability is captured before import begins, including whether the docs lane was reused or recreated
- the kickoff contract clearly distinguishes authored source, derived proof, and recalibration surfaces
- no authored source files change during WS-0

### WS-A Source Landing And Stop-State Reproduction (`feat/corpus-expansion` using `ws/m27_9-int` as source authority) - parent only

Task IDs:

- `task/m27_9a-a1-import-authored-source-truth`
- `task/m27_9a-a2-land-source-truth-on-parent`
- `task/m27_9a-a3-reproduce-stop-state`

Owned authored paths on the parent branch:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_control_flow.unit.spec`
- `spec-cli/tests/cli.rs`
- `semantic-families/README.md`

Locked import mechanism:

- Parent imports authored source by direct file sync or patch-copy of the exact files above from the live `ws/m27_9-int` worktree.
- Parent does not merge the worktree branch and does not bulk-check out the worktree tree.
- Parent records the chosen mechanism and touched files in `source-import.json`.

Required parent actions:

1. Import the exact authored source truth from the live `ws/m27_9-int` worktree content.
2. Confirm the imported source set excludes both crosslib passport files.
3. Complete `source-import.json` with actual landed paths and excluded derived paths.
4. Run the source-truth gate:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
```

5. Reproduce the stop-state on the merged parent branch:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

6. Record the reproduced outputs in `reproduced-stop-state.json` and in diagnostics copies under `.runs/m27_9/diagnostics/`.

WS-A acceptance:

- the parent branch now contains the full authored M27.9 source truth, not just the earlier partial semantic edit
- only the five authored source surfaces land in this phase
- both crosslib passport files remain excluded from source landing
- the reproduced branch truth is exactly:
  - `function_coverage = 28 / 17 / 0 / 11`
  - `recommendation_status = "no_strong_candidate"`
- no ranked arithmetic-ready candidate remains
- `unsupported_function_surface-e40675da6fa0` remains held for `unknown_overlap_family`

WS-A blocked path:

- If coverage or recommendation differ from the expected reproduced stop-state, stop immediately.
- Write or refresh:
  - `.runs/m27_9/diagnostics/blocked-summary.md`
  - `.runs/m27_9/diagnostics/coverage.actual.json`
  - `.runs/m27_9/diagnostics/recommendation.actual.json`
  - `.runs/m27_9/diagnostics/reproduction-failure.json`
- Mark Lane B as draft-only pending replanning.
- Do not edit `xtask/src/lib.rs`.
- Do not finalize plan/program language that claims the reproduction was confirmed.

### WS-B Docs/Program Closeout Wording (`ws/m27_9-docs`) - single worker lane

Task IDs:

- `task/m27_9a-b0-prepare-docs-lane`
- `task/m27_9a-b1-draft-ledger-rewrite`
- `task/m27_9a-b2-finalize-ledger-wording`

Owned files:

- `PLAN.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Start gate:

- WS-B may start only after:
  - `task/m27_9a-02-accept-frozen-evidence` is complete
  - `task/m27_9a-03-verify-worktree-viability` is complete
  - `task/m27_9a-b0-prepare-docs-lane` has produced a clean docs worktree based on the current `feat/corpus-expansion` HEAD

Finalization gate:

- WS-B may be finalized and merged only after `task/m27_9a-a3-reproduce-stop-state` confirms the branch really reproduces the frozen stop-state.

Bounded scope rule:

- `PLAN.md` edits are limited to closeout wording adjustments inside the existing M27.9A contract.
- WS-B must not materially rewrite the milestone shape, file contract, step ordering, or implementation scope.
- The worker is aligning repo-recorded closeout language, not authoring a new plan.

Required worker rules:

- Treat `.runs/m27_9` blocked evidence as authoritative input.
- Treat the prepared docs worktree baseline as authoritative, not the old frozen `cc12c859d99d409a4f861be64b9d7df7a653caba` tree state.
- Draft from frozen evidence first, then reconcile the final wording against the reproduced parent-branch stop-state before return or merge.
- Record M27.9 as implementation success plus accounting failure.
- Retire arithmetic-ready pressure as a live next-step driver.
- Keep `money/round` as the next visible held candidate.
- Do not author any language that implies:
  - a new family packet is part of this milestone
  - another corpus run is part of this milestone
  - recommendation policy is being revised here
- Do not use the wrong `28 / 18 / 0 / 10` gate except to describe the obsolete expectation that caused the stop.

WS-B acceptance:

- changed files are exactly:
  - `PLAN.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
- no other path changes are accepted from this lane
- the worker base for `ws/m27_9-docs` is the current `feat/corpus-expansion` branch state, not the stale frozen M27.9 branch state
- `PLAN.md` remains structurally the same milestone and step contract, with closeout wording only
- both documents lock the truthful target state `28 / 17 / 0 / 11` plus `no_strong_candidate`
- both documents describe the surviving held candidate consistently
- wording tracks reproduced stop-state, not merely the frozen run summary

### WS-INT Parent Integration (`feat/corpus-expansion`) - parent only

Task IDs:

- `task/m27_9a-i1-integrate-docs-lane`
- `task/m27_9a-i2-pre-xtask-integration-gate`

Purpose:

- WS-INT is the explicit integration phase for M27.9A.
- There is no separate long-lived integration branch for this run.
- The parent integrates onto `feat/corpus-expansion` directly after WS-A and before any `xtask` lock refresh.

Integration ownership and mechanism:

- Source truth from `ws/m27_9-int` is already landed by parent direct file sync in WS-A.
- Lane B returns a narrow docs/program diff from the prepared `ws/m27_9-docs` lane.
- Parent integrates Lane B back into `feat/corpus-expansion` by:
  - preferred path: cherry-pick clean docs-only worker commit(s)
  - fallback path: manual patch-copy of only `PLAN.md` and `docs/recommendation_corpus_expansion_program_v0.1.md` after reviewing the worker diff
- Parent does not use `git merge` for the whole docs worktree.

Conflict handling rules:

- If cherry-pick conflicts, the parent resolves only within:
  - `PLAN.md`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
- Conflict resolution preserves parent branch factual truth from WS-A first:
  - `28 / 17 / 0 / 11`
  - `no_strong_candidate`
  - arithmetic-ready pressure retired
  - `money/round` held
- If worker wording conflicts with reproduced branch truth, branch truth wins and the wording is rewritten narrowly.
- No conflict may be resolved by changing `spec-core`, `spec-cli`, fixtures, README, or `xtask`.

Required parent actions:

1. Review Lane B diff and confirm it touches only the two owned docs/program files.
2. Integrate Lane B onto `feat/corpus-expansion` using the preferred or fallback mechanism above.
3. Write `integration-state.json` with:
   - integration inputs
   - chosen integration mechanism
   - worker commit ids if any
   - changed paths
   - conflict resolution mode `none|manual_docs_only`
4. Run the pre-`xtask` integration gate:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
```

5. Verify integrated branch state before `xtask` work begins:
   - source-truth files from WS-A still match intended authored state
   - docs/program files are present and aligned to reproduced stop-state
   - `xtask/src/lib.rs` is still untouched

WS-INT acceptance:

- parent branch contains:
  - the five authored source-truth surfaces from WS-A
  - the two bounded docs/program closeout surfaces from WS-B
- no unrelated path changes are introduced during integration
- `xtask/src/lib.rs` remains unchanged before the integration gate completes
- pre-`xtask` integration gate passes on the integrated branch
- `integration-state.json` records the actual integration mechanism and result

WS-INT blocked path:

- If Lane B proposes changes outside its owned paths, reject the lane and stop.
- If Lane B was started from a stale docs base or from a docs worktree still carrying out-of-scope dirty paths, reject the lane and stop.
- If cherry-pick fails and docs-only manual conflict resolution cannot preserve reproduced truth cleanly, stop.
- If the pre-`xtask` integration gate fails, stop before `xtask` work begins.
- On any WS-INT failure, write or refresh:
  - `.runs/m27_9/diagnostics/blocked-summary.md`
  - `.runs/m27_9/diagnostics/integration-failure.json`
  - `.runs/m27_9/diagnostics/coverage.actual.json` when coverage was rerun
  - `.runs/m27_9/diagnostics/recommendation.actual.json` when recommendation was rerun
- Do not proceed to `xtask`.

### WS-C xtask Lock Refresh (`feat/corpus-expansion`) - parent only

Task ID:

- `task/m27_9a-c1-refresh-xtask-lock`

Owned file:

- `xtask/src/lib.rs`

Hard start gate:

- WS-C is forbidden until all of the following are true:
  - WS-A reproduced exactly `28 / 17 / 0 / 11`
  - WS-A reproduced exactly `no_strong_candidate`
  - WS-INT integrated Lane B and passed the pre-`xtask` integration gate

Required parent actions:

1. Reuse the reproduced coverage and recommendation outputs from WS-A as the locking basis.
2. Update `xtask/src/lib.rs` to assert:
   - coverage at `28 / 17 / 0 / 11`
   - `RecommendationStatus::NoStrongCandidate`
   - arithmetic-ready ranking removed
   - `unsupported_function_surface-e40675da6fa0` remains the visible held candidate for `unknown_overlap_family`
3. Do not rewrite ranking policy, artifact schema, or other `xtask/src/family/*` surfaces.
4. Verify with:

```bash
cargo test -p xtask -- --color never
```

WS-C acceptance:

- changed file is exactly `xtask/src/lib.rs`
- the lock surface reflects reproduced merged-branch truth only
- no policy or schema code outside the lock surface changes
- locked tests pass against the recalibrated baseline

WS-C blocked path:

- If WS-A or WS-INT did not complete cleanly, WS-C does not start.
- If `cargo test -p xtask` fails after the lock edit, stop and diagnose the lock surface only.
- On failure, write or refresh:
  - `.runs/m27_9/diagnostics/blocked-summary.md`
  - `.runs/m27_9/diagnostics/xtask-lock-failure.json`
- Do not widen scope into policy code.

### WS-D Final Proof And Stop-Path Closeout (`feat/corpus-expansion`) - parent only

Task IDs:

- `task/m27_9a-d1-final-proof`
- `task/m27_9a-d2-closeout-or-stop`

Preconditions:

- WS-A accepted
- WS-B accepted and integrated through WS-INT
- WS-C accepted

Run the final proof loop in this exact order:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo test -p xtask -- --color never
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Required parent actions:

1. Write `final-proof.json` with command list, exit codes, and artifact-validation results.
2. Refresh diagnostics copies of actual coverage and recommendation outputs if those commands rerun during final proof.
3. Record final closeout in `session-log.md` as either:
   - clean stop-path closeout
   - blocked closeout with gate failure

WS-D acceptance:

- `spec-core` passes
- `spec-cli` CLI truth passes
- `xtask` passes
- coverage artifact validates
- recommendation artifact validates
- final outputs still read:
  - `function_coverage = 28 / 17 / 0 / 11`
  - `recommendation_status = "no_strong_candidate"`
- the remaining visible held candidate is still `unsupported_function_surface-e40675da6fa0`
- M27.9 is closed as implementation success plus accounting failure, not as a failed semantic fix

WS-D blocked closeout:

- Stop at the first unexplained mismatch.
- Write or refresh the exact machine-evidence closeout bundle:
  - `.runs/m27_9/diagnostics/blocked-summary.md`
  - `.runs/m27_9/diagnostics/final-proof-failure.json`
  - `.runs/m27_9/diagnostics/coverage.actual.json`
  - `.runs/m27_9/diagnostics/recommendation.actual.json`
  - `.runs/m27_9/final-proof.json`
- Record the failed gate explicitly:
  - `reproduction`
  - `integration`
  - `xtask_lock`
  - `final_proof`
- Do not hand-tune `xtask` a second time to chase the failure.
- Do not silently retry with broadened scope.
- Require a fresh planning decision if final proof diverges from the reproduced stop-state.

## Worker Protocol

- Worker prompt contents are fixed:
  - owned file set
  - relevant frozen-evidence excerpt
  - reproduced stop-state once available
  - allowed scope
  - forbidden surfaces
  - required return format
- Worker return contents are fixed:
  - changed files
  - commands run
  - exit codes
  - blockers or unresolved assumptions
  - whether the lane is draft-only or finalizable
- Workers must not write:
  - `.runs/**`
  - `.semantic-family-artifacts/**`
  - generated output
  - passport files
  - source-truth code surfaces outside their owned paths
- The parent reviews only narrow diffs and structured returns, not full worker transcripts.
- Each worker is closed immediately after merge or rejection. No long-lived worker remains attached after its lane is integrated or stopped.

## Context-Control Rules

- Parent context stays bounded to:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `.runs/m27_9/session-log.md`
  - `.runs/m27_9/diagnostics/blocked-summary.md`
  - the latest reproduced-stop-state summary
  - the latest integration summary
- `ws/m27_9-int` is the read-side source authority for authored M27.9 landing.
- `ws/m27_9-docs` is the only writing worker lane in this milestone, but only after the parent has recreated or cleaned it onto the current `feat/corpus-expansion` base.
- `ws/m27_9-m20-cli` is non-authoritative reference context only for M27.9A. It does not own source landing, integration, or final truth.
- Parent-only phases are:
  - WS-0 kickoff and run-state seed
  - WS-A source landing and reproduced stop-state
  - WS-INT integration
  - WS-C `xtask` lock refresh
  - WS-D final proof and closeout
- Worker concurrency remains capped at `1` for the entire run. The orchestration does not create a second worker lane after reproduction because there is no independent authored surface left that can move safely without forcing another integration gate.

## Tests And Acceptance

- Kickoff and run-state
  - parent-owned run-state is initialized before any authored edits
  - task queue, baseline, frozen evidence, and source-import contract are all recorded explicitly
- Source landing
  - `spec-core/src/semantic_review.rs` preserves the cross-library arithmetic fix exactly as proven in the integration worktree
  - `spec-cli/tests/cli.rs` preserves the supported cross-library truth surfaces and the M20 unsupported whole-pack reason matrix
  - `semantic-families/README.md` lands with the authored helper-aware explanation from the integration worktree
  - derived passports remain excluded from source landing
- Reproduced stop-state
  - merged branch truth is `28 / 17 / 0 / 11`
  - recommendation is `no_strong_candidate`
  - arithmetic-ready ranking is absent
  - `money/round` remains the next visible held candidate
- Docs/program closeout
  - `PLAN.md` receives bounded closeout wording only
  - `docs/recommendation_corpus_expansion_program_v0.1.md` retires arithmetic pressure and points future work at `money/round`
  - no other path changes come from the worker lane
- Integration
  - parent integrates Lane B explicitly and records how
  - pre-`xtask` integration gate passes on integrated branch state
  - `xtask/src/lib.rs` is still untouched when integration completes
- Locked analysis contract
  - `xtask/src/lib.rs` locks the truthful post-fix baseline and no longer encodes arithmetic-ready pressure
  - no ranking-policy or artifact-schema rewrites occur
- Final proof
  - all commands in the final proof loop succeed in order
  - both analysis artifacts validate
  - final outputs match the reproduced stop-state exactly

## Assumptions

- The live execution branch remains `feat/corpus-expansion`.
- The blocked evidence bundle in `.runs/m27_9/**` is still present and trustworthy enough to serve as the frozen basis record.
- The current `ws/m27_9-int` worktree content remains the authoritative authored M27.9 source truth even if the branch tip alone is incomplete.
- Existing worktrees `ws/m27_9-int`, `ws/m27_9-m20-cli`, and `ws/m27_9-docs` may be reused only if their live state matches their lane role. In the current repo state, `ws/m27_9-docs` may require recreation or clean replacement before use because it can be stale relative to `feat/corpus-expansion` and can carry an out-of-scope README edit.
- Unrelated local edits may exist elsewhere in the repo. This run integrates around them and never reverts them.
