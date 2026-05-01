# M27.8 Orchestration Plan

Status: **execution contract**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` only**  
Working branch baseline: **`feat/corpus-expansion`**  
Last rewritten: **2026-05-01**

## Summary

- Execute from the current branch `feat/corpus-expansion`, because that is both the
  authority-file baseline and the live checked-out branch in this workspace.
- Keep the critical path local to the parent agent for baseline capture, dirty-state
  recording, contract freeze, worker launch, integration, derived-artifact
  regeneration, final acceptance, and landing.
- Use exactly one safe parallel window after the parent freezes the M27.8 contract:
  - Lane A: author `examples/crosslib-app/units/pricing/apply_tax.unit.spec` and
    update `examples/crosslib-app/units/.gitignore`
  - Lane B: update the command-path lock in `xtask/src/lib.rs`
- Use dedicated worktrees under
  `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8/{contract,lane-a,lane-b,int}`
  with workstream branches:
  - `ws/m27_8-contract`
  - `ws/m27_8-lane-a`
  - `ws/m27_8-lane-b`
  - `ws/m27_8-int`
- Use GPT-5.4 with `reasoning_effort=high` for both workers. Cap concurrency at `2`.
  The parent agent remains the only integrator.
- Keep orchestration state in one canonical location owned by the parent agent:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8`
  - `RUN_ROOT=$PRIMARY_ROOT/.runs/m27_8`
  - baseline: `$RUN_ROOT/baseline.json`
  - dirty-state: `$RUN_ROOT/dirty-state.json`
  - queue: `$RUN_ROOT/tasks.json`
  - session log: `$RUN_ROOT/session-log.md`
  - merge log: `$RUN_ROOT/merge-log.md`
  - contract freeze: `$RUN_ROOT/contract-freeze.json`
  - worker prompts: `$RUN_ROOT/prompts/lane-a.md`, `$RUN_ROOT/prompts/lane-b.md`
  - acceptance record: `$RUN_ROOT/acceptance.md`
  - per-task sentinels: `$PRIMARY_ROOT/.runs/<task-id>/`
- Treat `$RUN_ROOT/*`,
  `examples/crosslib-app/units/*.spec.passport.json`,
  `examples/shared-spec/units/*.spec.passport.json`, and
  `.semantic-family-artifacts/family-promotion/analysis/*` as run-state or derived
  proof surfaces, not authored source.

## Hard Guards

- `PLAN.md` is the sole authority for M27.8. If any stale orchestration note, older
  `ORCH_PLAN.md`, branch-local memory, or worker suggestion disagrees, `PLAN.md` wins.
- The milestone scope is exactly three tracked source-file edits:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `examples/crosslib-app/units/.gitignore`
  - `xtask/src/lib.rs`
- Explicit non-touch surfaces for this run:
  - `semantic-families/corpus/rust-function.toml`
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `docs/recommendation_corpus_expansion_program_v0.1.md`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/**`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/**`
  - `semantic-families/README.md`
- There are no human approval gates in M27.8. The only intentional pauses are
  stop-and-replan events.
- Parent-owned build order is fixed and must not be reordered:
  1. build `examples/shared-spec/units` into `examples/shared-crate/src/generated`
  2. run exact-unit proof for `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  3. build `examples/crosslib-app/units` into `examples/crosslib-app/src/generated`
  4. run `examples/crosslib-app` crate tests
  5. rerun coverage
  6. rerun recommendation
  7. validate both artifacts
  8. run `xtask` tests
- Worker lanes must not regenerate passports, coverage artifacts, or recommendation
  artifacts. Those writes are integration-only.
- If implementation starts requiring source edits outside the locked touch set, stop
  and re-plan.
- If any lane discovers it must edit `coverage.rs`, `recommend.rs`,
  `promotion_artifacts.rs`, the corpus manifest, docs, or fixture packs, stop and
  re-plan.
- If the first integrated rerun does not match the locked ranked truth from
  `PLAN.md`, stop immediately.
  - Do not silently rewrite expectations.
  - Do not widen the milestone.
  - Re-plan from the mismatch.
- If the baseline worktree is dirty in the locked source touch set or derived proof
  surfaces, stop before branching.
- Pre-existing unrelated local state is allowed only as recorded baseline context.
  The currently known unrelated state is:
  - modified: `PLAN.md`
  - untracked: `diagrams.md`
  - untracked: `docs/semantic_family_capability_corpus_guide_v0.1.md`

## Discarded Stale Assumptions

This M27.8 run must not inherit stale M27.75 assumptions.

- no manifest-edit lane
- no docs lane
- no `semantic-families/README.md` work
- no corpus source expansion work in `semantic-families/corpus/rust-function.toml`
- no recommendation, coverage, or promotion-artifacts logic edits
- no five-source-manifest transition work, because M27.8 operates on the already-expanded
  corpus and adds one new real example inside `examples_crosslib_app`

## Integrator Model

- Parent agent is the only integrator.
- Parent agent is the only authority for:
  - baseline capture
  - dirty-state recording
  - contract freeze
  - worker prompt generation
  - worker launch
  - merge decisions
  - conflict resolution
  - derived-artifact regeneration
  - acceptance recording
  - landing or blocked closeout
- Maximum active workers: `2`
- Safe worker layout:
  - Lane A: crosslib spec authoring
  - Lane B: `xtask` command-path lock
- Parallelism is intentionally bounded. There is exactly one approved parallel phase,
  and it starts only after the parent freezes the exact contract.

## Worktree And Branch Strategy

All execution branches fork from the exact current `feat/corpus-expansion` baseline
SHA. Do not fork from `main`. Do not develop directly on `feat/corpus-expansion`
once execution starts.

Canonical orchestration roots:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27_8`
- `RUN_ROOT=$PRIMARY_ROOT/.runs/m27_8`

Canonical branches:

- contract: `ws/m27_8-contract`
- Lane A: `ws/m27_8-lane-a`
- Lane B: `ws/m27_8-lane-b`
- integration: `ws/m27_8-int`

Canonical worktrees:

- contract: `$WORKTREE_ROOT/contract`
- Lane A: `$WORKTREE_ROOT/lane-a`
- Lane B: `$WORKTREE_ROOT/lane-b`
- integration: `$WORKTREE_ROOT/int`

Creation commands from `PRIMARY_ROOT`:

```bash
mkdir -p "$WORKTREE_ROOT" "$RUN_ROOT" "$RUN_ROOT/prompts"
BASE_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_SHA=$(git rev-parse HEAD)
git worktree add -b ws/m27_8-contract "$WORKTREE_ROOT/contract" "$BASE_SHA"
```

After baseline capture and contract freeze are written:

```bash
FREEZE_SHA=$(jq -r '.contract_freeze_commit' "$RUN_ROOT/contract-freeze.json")
git worktree add -b ws/m27_8-lane-a "$WORKTREE_ROOT/lane-a" "$FREEZE_SHA"
git worktree add -b ws/m27_8-lane-b "$WORKTREE_ROOT/lane-b" "$FREEZE_SHA"
git worktree add -b ws/m27_8-int "$WORKTREE_ROOT/int" "$FREEZE_SHA"
```

Worktree rules:

- do not reuse dirty worktrees
- do not let workers self-merge
- do not merge worker branches directly into `feat/corpus-expansion`
- do not create extra side branches beyond the four locked branches
- do not branch workers from anything earlier than `contract_freeze_commit`
- if unrelated local changes expand beyond the recorded dirty-state allowlist before
  branching, stop and re-record the baseline before proceeding

## Parent-Owned Run State

Parent-managed orchestration state lives under `$RUN_ROOT`:

- `baseline.json`
- `dirty-state.json`
- `tasks.json`
- `session-log.md`
- `merge-log.md`
- `contract-freeze.json`
- `prompts/lane-a.md`
- `prompts/lane-b.md`
- `acceptance.md`

Minimum `tasks.json` shape:

- `task_id`
- `branch`
- `worktree`
- `owner`
- `status`
- `depends_on`
- `owned_paths`
- `required_commands`
- `sentinel_dir`

Per-task sentinel directories live under `PRIMARY_ROOT/.runs/<task-id>/` and contain:

- `started.json`
- `status.json`
- `done.json` or `blocked.json`

Sentinel rules:

- Parent writes every sentinel file, including worker-task sentinels.
- Workers report status; they do not author sentinel files directly.
- `blocked.json` must include:
  - `task_id`
  - `blocked_at`
  - `reason`
  - `required_replan`
  - `touched_files`

Worker chat is not the source of truth. Parent-owned run artifacts are.

## Task Graph

```text
task/m27_8-00-baseline
  -> task/m27_8-a1-freeze-contract
      -> task/m27_8-b1-crosslib-unit
      -> task/m27_8-b2-xtask-lock
task/m27_8-b1-crosslib-unit
task/m27_8-b2-xtask-lock
  -> task/m27_8-c1-integrate-and-rerun
      -> task/m27_8-c2-land-or-stop
```

Execution intent:

1. parent captures the exact baseline, branch, SHA, and dirty state
2. parent freezes the M27.8 contract and writes worker prompts
3. Lane A and Lane B branch from the same frozen commit and run in parallel
4. parent merges both lanes into integration
5. parent alone regenerates passports and analysis artifacts from merged state
6. parent runs the full proof loop in the locked order
7. parent records acceptance and either lands or stops

Serialized parent-owned tasks:

- `task/m27_8-00-baseline`
- `task/m27_8-a1-freeze-contract`
- `task/m27_8-c1-integrate-and-rerun`
- `task/m27_8-c2-land-or-stop`

Parallel-safe window:

- `task/m27_8-b1-crosslib-unit`
- `task/m27_8-b2-xtask-lock`

That is the only approved parallel phase.

## Lane And Ownership Boundaries

### WS-CONTRACT — parent only

Purpose:

- record the exact frozen contract from `PLAN.md`
- capture the allowed dirty state
- write the worker prompts and merge rules
- record the freeze commit other lanes must branch from

Owned files:

- `$RUN_ROOT/baseline.json`
- `$RUN_ROOT/dirty-state.json`
- `$RUN_ROOT/tasks.json`
- `$RUN_ROOT/session-log.md`
- `$RUN_ROOT/contract-freeze.json`
- `$RUN_ROOT/prompts/lane-a.md`
- `$RUN_ROOT/prompts/lane-b.md`
- parent-owned sentinel files only

Forbidden files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`
- `xtask/src/lib.rs`
- all derived proof artifacts
- all explicit non-touch surfaces

### Lane A — crosslib unit worker

Purpose:

- add the new maintained cross-library unit with the exact locked shape
- whitelist the new passport intentionally

Owned files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`

Forbidden files:

- `xtask/src/lib.rs`
- `.semantic-family-artifacts/family-promotion/analysis/*`
- `examples/crosslib-app/units/*.spec.passport.json`
- `examples/shared-spec/units/*.spec.passport.json`
- all explicit non-touch surfaces

### Lane B — `xtask` proof worker

Purpose:

- update the single existing command-path lock to the ranked M27.8 truth

Owned files:

- `xtask/src/lib.rs`

Forbidden files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`
- `.semantic-family-artifacts/family-promotion/analysis/*`
- `examples/crosslib-app/units/*.spec.passport.json`
- `examples/shared-spec/units/*.spec.passport.json`
- all explicit non-touch surfaces

### WS-INT — parent only

Purpose:

- merge completed worker branches
- regenerate all derived proof surfaces from merged state only
- run final validation
- write merge and acceptance artifacts
- land the integrated result or stop with a blocked record

Parent integration must not invent new product semantics. Any conflict that requires
reinterpretation of `PLAN.md` is a stop-and-bounce event back to the owning lane.

## Merge Rules

- Parent merges only into `ws/m27_8-int`.
- Merge order is fixed:
  1. `ws/m27_8-lane-a`
  2. `ws/m27_8-lane-b`
- A worker branch is merge-eligible only if:
  - touched files are within its owned paths
  - required lane-local commands were run
  - no derived artifacts were written
  - no non-touch surface changed
- Parent resolves only mechanical conflicts.
  - path moves
  - import-order or formatting collisions inside owned files
- Parent does not resolve semantic conflicts creatively.
  - If Lane B's assertions no longer match the frozen `apply_tax` shape, bounce back.
  - If Lane A needs to broaden crosslib source changes, bounce back.
- If either lane includes unrelated edits because the codebase moved underneath it,
  stop and re-plan rather than silently absorbing them.
- `merge-log.md` must record:
  - merged branch
  - merge commit or cherry-picked commit
  - owned paths verified
  - conflicts encountered
  - disposition

## Worker Prompt Contract

Each worker prompt must be written by the parent under `$RUN_ROOT/prompts/` and must
contain only:

- the exact `contract_freeze_commit`
- the worker's owned file set
- the exact `PLAN.md` excerpt relevant to that lane
- forbidden touch surfaces
- required commands
- acceptance conditions
- stop conditions
- return format

Each worker must return only:

- changed files
- commands run and exit codes
- blockers or unresolved assumptions

Workers must not return or rely on:

- full transcript dumps
- derived-artifact contents
- self-authored run-state files
- creative scope expansions

## Task Contracts

### `task/m27_8-00-baseline` — parent only

Owned files:

- `$RUN_ROOT/baseline.json`
- `$RUN_ROOT/dirty-state.json`
- `$RUN_ROOT/tasks.json`
- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_8-00-baseline/*`

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
test -f PLAN.md
test -f ORCH_PLAN.md
test -f examples/crosslib-app/units/.gitignore
test -f xtask/src/lib.rs
```

Acceptance:

- current branch is `feat/corpus-expansion`
- baseline SHA is captured
- dirty state is recorded verbatim
- the known unrelated dirty files are recorded as allowed local state
- no dirty file exists in the locked source touch set
- no dirty file exists in derived proof surfaces
- `PLAN.md` is present and treated as sole authority
- task graph, branches, and worktrees are recorded in `tasks.json`

Stop conditions:

- current branch is not `feat/corpus-expansion`
- any locked source touch file is already dirty
- any expected derived artifact is already dirty
- `PLAN.md` is missing or materially inconsistent with the live repo layout

### `task/m27_8-a1-freeze-contract` — parent only

Owned files:

- `$RUN_ROOT/contract-freeze.json`
- `$RUN_ROOT/prompts/lane-a.md`
- `$RUN_ROOT/prompts/lane-b.md`
- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_8-a1-freeze-contract/*`

Must do:

- record `contract_freeze_commit` equal to the accepted baseline SHA
- record the exact locked M27.8 touch set
- record the explicit non-touch list
- record the exact `apply_tax` unit shape from `PLAN.md`
- record the required `.gitignore` whitelist line
- record the exact ranked output deltas Lane B must lock
- record the required parent-owned build order
- write prompts for Lane A and Lane B using only frozen contract data
- create worker and integration worktrees from `contract_freeze_commit`

Must not do:

- edit milestone source files
- edit derived artifacts
- reinterpret any locked expected values

Acceptance:

- `contract-freeze.json` contains:
  - `contract_freeze_commit`
  - `working_branch`
  - `locked_source_touch_set`
  - `explicit_non_touch_set`
  - `locked_apply_tax_shape`
  - `locked_gitignore_line`
  - `locked_coverage_deltas`
  - `locked_recommendation_deltas`
  - `required_build_order`
- worker prompts exist and are lane-specific
- all worker and integration branches fork from the same frozen commit

Stop conditions:

- the `PLAN.md` contract cannot be reduced to disjoint lane ownership
- the frozen unit shape is ambiguous enough that Lane B cannot lock against it
- baseline moves before worker branches are created

### `task/m27_8-b1-crosslib-unit` — Lane A

Owned files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/.gitignore`
- `PRIMARY_ROOT/.runs/task-m27_8-b1-crosslib-unit/*` via parent only

Must do:

- create `examples/crosslib-app/units/pricing/apply_tax.unit.spec` with the exact
  authored shape frozen in `contract-freeze.json`
- add exactly one line to `examples/crosslib-app/units/.gitignore`:
  - `!pricing/apply_tax.spec.passport.json`
- keep the change limited to the owned paths
- validate the new unit source without generating derived artifacts

Must not do:

- edit `examples/crosslib-app/spec.toml`
- run `spec build` for shared-spec or crosslib-app
- run `spec test` for the new unit
- write passport artifacts
- edit any file outside Lane A ownership

Required commands:

```bash
cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/apply_tax.unit.spec --format json
```

Acceptance:

- the new unit file exists with the frozen YAML shape
- `.gitignore` changes by exactly one whitelist line
- `spec validate` succeeds for the new unit
- no non-owned file changes are present

Stop conditions:

- unit validation requires changes to shared-spec, spec.toml, manifest, or fixtures
- the authored shape from `PLAN.md` does not validate as written
- Lane A needs to touch generated output or passports to look green

### `task/m27_8-b2-xtask-lock` — Lane B

Owned files:

- `xtask/src/lib.rs`
- `PRIMARY_ROOT/.runs/task-m27_8-b2-xtask-lock/*` via parent only

Must do:

- rename
  `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
  to
  `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held()`
- keep the existing stdout-bytes-versus-written-artifact checks
- keep the existing first-run-versus-second-run byte-stability checks
- update assertions to the exact M27.8 ranked truth locked in `PLAN.md`
- consume the frozen `apply_tax` unit shape literally from `contract-freeze.json`

Must assert:

- coverage source ids remain:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- coverage source unit counts are:
  - `6`
  - `12`
  - `9`
  - `1`
  - `2`
- `function_coverage.total_units == 28`
- `function_coverage.promoted_family_units == 15`
- `function_coverage.supported_unpromoted_family_units == 0`
- `function_coverage.unsupported_function_units == 13`
- `recommendation_status == ranked`
- ranked candidate count is `2`
- first candidate is `unsupported_arithmetic_shape-2694b2baf65b`
- first candidate is `ready` with:
  - `hold_reasons == []`
  - `real_example_hits == 2`
  - `promotion_relevant_regression_hits == 1`
  - `boundary_only_hits == 0`
  - `total_units_in_cluster == 3`
  - `difficulty.tier == Adjacent`
  - `confidence.level == Medium`
- second candidate is `unsupported_function_surface-e40675da6fa0`
- second candidate remains `hold` with:
  - `hold_reasons == [UnknownOverlapFamily]`
  - `real_example_hits == 2`
  - `promotion_relevant_regression_hits == 1`
  - `boundary_only_hits == 0`
  - `total_units_in_cluster == 3`
  - `difficulty.tier == Hard`
  - `confidence.level == Low`

Must not do:

- edit any file outside `xtask/src/lib.rs`
- add a second parallel command-path lock for the same flow
- edit coverage, recommendation, or artifact logic
- write final derived artifacts

Required commands:

```bash
cargo test -p xtask recommendation_command_path --no-run
cargo test -p xtask --no-run
```

Acceptance:

- the updated lock compiles cleanly
- only `xtask/src/lib.rs` changes
- no policy logic changes are introduced
- Lane B is prepared for the integrated green run without requiring non-owned files

Stop conditions:

- the lock can only be made truthful by changing policy or artifact code
- Lane B needs to edit example files or fixtures
- Lane B cannot compile the updated test shape from the frozen contract

### `task/m27_8-c1-integrate-and-rerun` — parent only

Owned files:

- `examples/crosslib-app/units/pricing/apply_tax.spec.passport.json`
- `examples/crosslib-app/units/pricing/apply_discount.spec.passport.json`
- `examples/shared-spec/units/money/round.spec.passport.json`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `$RUN_ROOT/merge-log.md`
- `$RUN_ROOT/acceptance.md`
- `PRIMARY_ROOT/.runs/task-m27_8-c1-integrate-and-rerun/*`

Must do:

- merge `ws/m27_8-lane-a` and `ws/m27_8-lane-b` into `ws/m27_8-int`
- verify each worker touched only owned files before merge
- regenerate all derived proof surfaces from merged state only
- run the exact proof loop in the locked order
- compare coverage and recommendation stdout bytes to the written artifact bytes
- validate both artifacts through the path-aware `xtask` validator
- confirm all locked output deltas from `PLAN.md`

Required commands:

```bash
cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo test --manifest-path examples/crosslib-app/Cargo.toml

tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"

cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.rerun.json"
cmp -s "$tmpdir/recommend.stdout.json" "$tmpdir/recommend.stdout.rerun.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"

cargo test -p xtask -- --color never
```

Acceptance:

- exactly three tracked source files changed for the milestone
- only the expected derived proof artifacts changed beyond the source touch set
- the shared-library-first build order was preserved
- `apply_tax.unit.spec` proves successfully
- the crosslib crate tests clean
- coverage stdout bytes equal the written `coverage.latest.json`
- recommendation stdout bytes equal the written `recommendation.latest.json`
- rerunning recommendation is byte-stable
- coverage source unit counts are `6 / 12 / 9 / 1 / 2`
- `function_coverage.total_units == 28`
- `function_coverage.promoted_family_units == 15`
- `function_coverage.supported_unpromoted_family_units == 0`
- `function_coverage.unsupported_function_units == 13`
- recommendation status is `ranked`
- ranked candidate count is `2`
- arithmetic cluster is first and `ready`
- `money/round` remains second and held for `unknown_overlap_family`
- `xtask` tests lock the ranked arithmetic result and the held `money/round` truth

Stop conditions:

- any integrated proof command fails
- byte equality fails for coverage or recommendation
- recommendation rerun is not byte-stable
- a non-touch source file must change to make acceptance pass
- the integrated output does not match the frozen M27.8 deltas

### `task/m27_8-c2-land-or-stop` — parent only

Owned files:

- `$RUN_ROOT/acceptance.md`
- `$RUN_ROOT/merge-log.md`
- `$RUN_ROOT/session-log.md`
- `PRIMARY_ROOT/.runs/task-m27_8-c2-land-or-stop/*`

Must do on green:

- record the final accepted command results and invariants
- merge `ws/m27_8-int` back onto `feat/corpus-expansion`
- record the landing SHA in `acceptance.md`

Must do on red:

- write a blocked closeout with the exact failing invariant
- record the first failing command, the first failing invariant, and the exact diff
  scope in `acceptance.md`
- leave the frozen contract and merge log intact
- do not bounce into ad hoc follow-up edits under the same run
- stop without broadening scope

Acceptance:

- final disposition is recorded as `landed` or `blocked`
- landed state references the integration SHA and baseline branch
- blocked state references the first failing invariant and required re-plan

Stop conditions:

- parent cannot land without extra source edits
- the accepted integration SHA diverges from the recorded contract freeze ancestry

## Context-Control Rules

- Parent agent keeps only five live artifacts in working context:
  - `PLAN.md`
  - `$RUN_ROOT/tasks.json`
  - `$RUN_ROOT/contract-freeze.json`
  - the acceptance checklist
  - the latest integration diff summary
- Each worker prompt contains only:
  - owned file set
  - exact `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
  - the recorded `contract_freeze_commit`
  - stop conditions
- Workers do not write `$RUN_ROOT/*`.
- Workers do not write passports or analysis artifacts.
- Parent reviews worker summaries plus narrow diffs only.
- Parent closes each worker immediately after merge.
- Use sentinels and explicit completion reports, not tight polling.

## Tests And Acceptance

- Lane A source validation
  - `cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/apply_tax.unit.spec --format json`
  - no `spec build`
  - no `spec test`
- Lane B compile proof
  - `cargo test -p xtask recommendation_command_path --no-run`
  - `cargo test -p xtask --no-run`
- Parent integrated proof loop
  - `cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated`
  - `cargo test --manifest-path examples/crosslib-app/Cargo.toml`
  - `cargo xtask family coverage --format json`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `cargo test -p xtask -- --color never`

Required acceptance invariants:

- the five corpus source ids stay unchanged and in the same order
- source unit counts become `6 / 12 / 9 / 1 / 2`
- `examples_crosslib_app` grows from `1` to `2`
- first candidate cluster id is `unsupported_arithmetic_shape-2694b2baf65b`
- first candidate `promotion_readiness = Ready`
- first candidate `hold_reasons = []`
- first candidate `difficulty.tier = Adjacent`
- first candidate `confidence.level = Medium`
- arithmetic cluster representative units are:
  - `examples_crosslib_app::pricing/apply_discount`
  - `examples_crosslib_app::pricing/apply_tax`
  - `m20_unsupported_truth_pack::pricing/apply_tax_arithmetic_shape`
- arithmetic cluster leverage becomes:
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
- second candidate cluster id is `unsupported_function_surface-e40675da6fa0`
- second candidate `hold_reasons = ["unknown_overlap_family"]`
- second candidate `difficulty.tier = Hard`
- second candidate `confidence.level = Low`
- unknown-overlap `money/round` cluster remains materially unchanged and held
- recommendation output becomes `ranked` with exactly two candidates
- derived artifacts remain validator-clean and byte-stable

## Assumptions

- Worktree naming follows the repo's existing `spec-m27_*` pattern, so
  `spec-m27_8` and `ws/m27_8-*` are the correct concrete names for this run.
- The authority-file header `Base branch: main` is historical context, not the
  execution baseline. M27.8 execution uses `Working branch: feat/corpus-expansion`.
- Lane B can update and compile the command-path lock from the frozen `apply_tax`
  contract without owning the new spec file itself; the first full green run
  happens only after integration.
- The known dirty planning files are unrelated local state and not milestone
  deliverables unless they change again during execution.
