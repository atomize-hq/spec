# M27 Orchestration Plan

## Summary

- Confirmed plan authority: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` is the correct target plan for this M27 orchestration run.
- `PLAN-04.md` does not apply to this repo. Execute M27 only against `PLAN.md`.
- If any older note, kickoff prompt, or worker suggestion conflicts with `PLAN.md`, `PLAN.md` wins.
- This is a full-session orchestration plan for M27, not a slice outline. The run ends only when:
  - `family coverage` is implemented and deterministic
  - `family recommend` is implemented and deterministic
  - `validate-artifact` accepts both new M27 artifact kinds
  - the corpus manifest and maintainer docs are landed
  - final integration acceptance is green
- Use dedicated worktrees and branches. Do not assume any M27 branch, worktree, or run-state artifact already exists.
- Keep the workspace boundary intact:
  - no new crate
  - no new workspace member
  - no new binary outside existing `xtask`
- Treat `.semantic-family-artifacts/*` and `.runs/*` as derived run artifacts, not assumed tracked source.

## Worker Model

- Parent agent is the only integrator.
- Worker model for every delegated task:
  - model: GPT-5.4
  - reasoning: high
- Maximum concurrency: 2 workers at a time.
- Safe parallelism is intentionally narrow:
  - one `spec-core` worker
  - one `semantic-families` worker
- All `xtask` work is deliberately serialized in one parent-owned worktree because it shares:
  - `xtask/src/lib.rs`
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/promotion_artifacts.rs`

## Hard Guards

- `PLAN.md` is authoritative for:
  - command names
  - artifact paths
  - schema contracts
  - ranking rules
  - corpus rules
- M27 stays inside the existing `cargo xtask family ...` surface.
  - add:
    - `cargo xtask family coverage --format json`
    - `cargo xtask family recommend --format json`
  - extend:
    - `cargo xtask family validate-artifact <path>`
- `cargo xtask family recommend --format json` must recompute coverage in-process first.
  - It must not trust a pre-existing `coverage.latest.json`.
- M27 must not overwrite the existing M26 promotion artifact:
  - `.semantic-family-artifacts/family-promotion/recommendation.latest.json`
- New M27 durable outputs are locked to:
  - `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- Coverage must retain an inventory snapshot under:
  - `.semantic-family-artifacts/family-promotion/inventory/`
- `xtask` must use `spec-core` directly for loading, validation, semantic review, and unsupported fingerprinting.
  - no shell-out to `spec status`
  - no shell-out to `spec validate`
  - no shell-out to `spec export`
- The authored corpus manifest is locked to:
  - `semantic-families/corpus/rust-function.toml`
- Packet fixtures under `semantic-families/**/fixtures/**` are forbidden manifest sources for M27.
- Any edit outside the expected M27 touch set is a stop-and-escalate event:
  - `xtask/Cargo.toml`
  - `xtask/src/lib.rs`
  - `xtask/src/family/mod.rs`
  - `xtask/src/family/coverage.rs`
  - `xtask/src/family/recommend.rs`
  - `xtask/src/family/promotion_artifacts.rs`
  - `xtask/src/family/paths.rs`
  - `spec-core/src/lib.rs`
  - `spec-core/src/semantic_review.rs`
  - `semantic-families/corpus/rust-function.toml`
  - `semantic-families/README.md`

## Locked Assumptions

- `PLAN.md` is the only M27 plan authority because `PLAN-04.md` does not exist here.
- The current checked-out baseline branch is `feat/m26`.
  - Treat that branch only as the baseline commit source, not as an M27 branch.
- The locked M27 corpus basis from `PLAN.md` is still:
  - `examples/ecommerce/units`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`
- Expected corpus counts remain:
  - 6 ecommerce units
  - 12 M19 units
  - 9 M20 units
  - 27 total
- Expected runtime-supported promoted families remain the four-family set described in `PLAN.md`.
- M27 has no M26-style human approval gate.
  - Escalation still applies for authority drift, scope expansion, or touch-set expansion.

## Orchestration State

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WORKTREE_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m27`
- `M27_RUN_ROOT=$PRIMARY_ROOT/.runs/m27`

Canonical parent-owned orchestration surfaces:

- `$M27_RUN_ROOT/tasks.json`
  - authoritative task ledger
  - one record per task id
  - includes branch, worktree, owner, status, and completion sentinel path
- `$M27_RUN_ROOT/session-log.md`
  - terse parent-agent timeline
  - records kickoff, worker launch, merge decisions, escalations, and acceptance milestones
- `$M27_RUN_ROOT/merge-log.md`
  - ordered merge history
  - records source branch, target branch, commit SHAs, and any mechanical conflict resolutions
- `$M27_RUN_ROOT/acceptance.md`
  - final checklist and command results
  - only updated from the parent integration worktree
- `$M27_RUN_ROOT/baseline.json`
  - captured before any worker starts
  - stores baseline branch, baseline sha, inventory result summary, corpus counts, and authority checks

Per-task sentinels:

- each task gets a dedicated sentinel directory under `$PRIMARY_ROOT/.runs/<TASK_ID>/`
- minimum files:
  - `started.json`
  - `status.json`
  - `done.json` or `blocked.json`
- purpose:
  - let the parent track task completion without tight polling
  - keep worker completion state out of commit messages and chat transcripts

Derived product surfaces created by the implementation:

- `$PRIMARY_ROOT/.semantic-family-artifacts/family-promotion/inventory/*.json`
- `$PRIMARY_ROOT/.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `$PRIMARY_ROOT/.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

## Worktree Plan

Branches and worktrees:

- integration:
  - branch: `ws/m27-int`
  - worktree: `$WORKTREE_ROOT/int`
- fingerprint worker:
  - branch: `ws/m27-fingerprint`
  - worktree: `$WORKTREE_ROOT/fingerprint`
- corpus/docs worker:
  - branch: `ws/m27-corpus-docs`
  - worktree: `$WORKTREE_ROOT/corpus-docs`
- serialized xtask implementation:
  - branch: `ws/m27-xtask`
  - worktree: `$WORKTREE_ROOT/xtask`

Creation commands from `PRIMARY_ROOT`:

```bash
mkdir -p "$WORKTREE_ROOT" "$M27_RUN_ROOT"
BASE_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_SHA=$(git rev-parse HEAD)
printf '{\n  "base_branch": "%s",\n  "base_sha": "%s"\n}\n' "$BASE_BRANCH" "$BASE_SHA" > "$M27_RUN_ROOT/baseline.json"

git worktree add -b ws/m27-int "$WORKTREE_ROOT/int" "$BASE_SHA"
git worktree add -b ws/m27-fingerprint "$WORKTREE_ROOT/fingerprint" "$BASE_SHA"
git worktree add -b ws/m27-corpus-docs "$WORKTREE_ROOT/corpus-docs" "$BASE_SHA"
```

Create `ws/m27-xtask` only after the fingerprint and corpus/docs branches are merged into integration:

```bash
INT_SHA=$(git -C "$WORKTREE_ROOT/int" rev-parse HEAD)
git worktree add -b ws/m27-xtask "$WORKTREE_ROOT/xtask" "$INT_SHA"
```

Worktree rules:

- do not reuse dirty worktrees
- do not let workers merge their own branches
- do not create `ws/m27-xtask` before the baseline gate and parallel prerequisites are complete

## Task Graph

Critical path:

1. `task/m27-00-baseline-validate`
2. launch in parallel:
   - `task/m27-a1-fingerprint-helper`
   - `task/m27-a2-corpus-manifest-docs`
3. merge A1 then A2 into integration
4. `task/m27-b1-xtask-cli-wiring`
5. `task/m27-b2-coverage-artifact`
6. `task/m27-b3-recommend-artifact`
7. `task/m27-c-integrate`

Parallel-safe tasks:

- `task/m27-a1-fingerprint-helper`
- `task/m27-a2-corpus-manifest-docs`

Deliberately serialized tasks:

- `task/m27-00-baseline-validate`
- `task/m27-b1-xtask-cli-wiring`
- `task/m27-b2-coverage-artifact`
- `task/m27-b3-recommend-artifact`
- `task/m27-c-integrate`

## Workstream Plan

### WS-BASELINE

#### `task/m27-00-baseline-validate` — parent agent only

Owned files:

- no product source files by default
- parent may write only:
  - `$M27_RUN_ROOT/baseline.json`
  - `$M27_RUN_ROOT/tasks.json`
  - `$M27_RUN_ROOT/session-log.md`
  - `$PRIMARY_ROOT/.runs/task-m27-00-baseline-validate/*`

Forbidden files:

- all M27 product source files

Required commands:

```bash
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
cargo xtask family inventory --format json
find examples/ecommerce/units -name '*.unit.spec' | sort | wc -l
find spec-cli/tests/fixtures/m19/semantic_falsification_pack/units -name '*.unit.spec' | sort | wc -l
find spec-cli/tests/fixtures/m20/unsupported_truth_pack/units -name '*.unit.spec' | sort | wc -l
test ! -e PLAN-04.md
test -f PLAN.md
```

Acceptance:

- confirm `PLAN-04.md` is absent and `PLAN.md` is present
- confirm inventory truth still matches the four-family promoted/runtime-supported basis in `PLAN.md`
- confirm corpus counts still equal `6 / 12 / 9 / 27`
- confirm the command and artifact authority in `PLAN.md` still matches the repo surfaces being targeted
- if inventory truth, corpus counts, or command/artifact authority drift, stop and re-plan before any worker starts

### WS-A

#### `task/m27-a1-fingerprint-helper` — worker 1

Owned files:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs`

Forbidden files:

- all `xtask/*`
- all `semantic-families/*`

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
```

Acceptance:

- public unsupported-function shape fingerprint helper exists in `spec-core`
- helper is derived from existing semantic-review truth, not new `xtask` heuristics
- tests prove fingerprint stability
- tests prove same reason code with different shapes does not collapse into one cluster
- no artifact IO or ranking policy is added to `spec-core`

### WS-B

#### `task/m27-a2-corpus-manifest-docs` — worker 2

Owned files:

- `semantic-families/corpus/rust-function.toml`
- `semantic-families/README.md`

Forbidden files:

- all Rust source files
- all `xtask/*`
- all `spec-core/*`

Required commands:

```bash
test -f semantic-families/README.md
test ! -e semantic-families/corpus/rust-function.toml || true
find examples/ecommerce/units spec-cli/tests/fixtures/m19/semantic_falsification_pack/units spec-cli/tests/fixtures/m20/unsupported_truth_pack/units -name '*.unit.spec' | sort | wc -l
```

Acceptance:

- manifest exists at the locked path
- manifest contains exactly the three locked M27 sources
- manifest excludes packet fixtures
- README documents:
  - `family coverage`
  - `family recommend`
  - `analysis/` artifact directory
  - corpus source kinds
  - bucket leverage rules
  - M26 root recommendation non-overwrite rule

### WS-BUILD

#### `task/m27-b1-xtask-cli-wiring` — parent agent only in `ws/m27-xtask`

Owned files:

- `xtask/Cargo.toml`
- `xtask/src/lib.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`

Forbidden files:

- all `spec-core/*`
- all `semantic-families/*`

Required commands:

```bash
cargo test -p xtask -- --color never
```

Acceptance:

- `xtask` has runtime access to `spec-core` if needed
- `FamilyCommand::{Coverage, Recommend}` exists
- dispatch is wired for both commands
- module graph is in place for `coverage.rs` and `recommend.rs`
- manifest parsing and validation entrypoints exist

#### `task/m27-b2-coverage-artifact` — parent agent only in `ws/m27-xtask`

Owned files:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`

Forbidden files:

- all `spec-core/*`
- all `semantic-families/*`

Required commands:

```bash
cargo test -p xtask -- --color never
tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
```

Acceptance:

- retained inventory snapshot is written under `.semantic-family-artifacts/family-promotion/inventory/`
- coverage artifact is written to the locked `analysis/coverage.latest.json` path
- stdout bytes equal written artifact bytes
- coverage artifact validation passes through `validate-artifact`
- coverage reports:
  - promoted function units
  - supported-unpromoted function units
  - unsupported function units
  - supported non-function units
- unsupported cluster projection uses the `spec-core` fingerprint helper

#### `task/m27-b3-recommend-artifact` — parent agent only in `ws/m27-xtask`

Owned files:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`

Forbidden files:

- all `spec-core/*`
- all `semantic-families/*`

Required commands:

```bash
cargo test -p xtask -- --color never
tmpdir=$(mktemp -d)
cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
```

Acceptance:

- recommend recomputes coverage in-process first
- ranking follows the locked tuple from `PLAN.md`
- only `candidate_status == "rankable"` clusters are ranked
- `recommendation_status` follows the locked evaluation order
- stdout bytes equal written artifact bytes
- recommendation-analysis artifact validation passes
- M26 root recommendation artifact remains untouched

## WS-INT

### `task/m27-c-integrate` — parent agent only

Scope:

- merge all completed task branches into `ws/m27-int`
- run final acceptance only from merged state
- write fresh parent-owned run artifacts only from merged state

Owned files:

- integration branch merge commits
- `$M27_RUN_ROOT/session-log.md`
- `$M27_RUN_ROOT/merge-log.md`
- `$M27_RUN_ROOT/acceptance.md`
- `$PRIMARY_ROOT/.runs/task-m27-c-integrate/*`

Forbidden files:

- no creative semantic edits across lane-owned source during integration

Merge order:

1. `ws/m27-fingerprint` into `ws/m27-int`
2. `ws/m27-corpus-docs` into `ws/m27-int`
3. create `ws/m27-xtask` from integration HEAD and complete B1/B2/B3 there
4. `ws/m27-xtask` into `ws/m27-int`

Merge commands:

```bash
git -C "$WORKTREE_ROOT/int" merge --no-ff ws/m27-fingerprint
git -C "$WORKTREE_ROOT/int" merge --no-ff ws/m27-corpus-docs
git -C "$WORKTREE_ROOT/int" merge --no-ff ws/m27-xtask
```

Integration may resolve mechanically:

- module declarations
- import ordering
- adjacent test additions
- straightforward context drift caused by already-approved ownership boundaries

Integration must bounce back to lane owners:

- fingerprint contract disagreements
- schema contract disagreements
- ranking-rule disagreements
- manifest semantics disagreements
- any conflict requiring reinterpretation of `PLAN.md`

Derived artifact rule:

- regenerate all M27 derived artifacts from the current merged integration state only
- do not trust worker-produced `.semantic-family-artifacts/*`
- do not carry forward stale run-local analysis output into final acceptance

Required commands:

```bash
cargo fmt --all
cargo test -p spec-core --lib -- --color never
cargo test -p xtask -- --color never

tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"

cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
```

Acceptance:

- all prerequisite task branches merge cleanly or with mechanical-only resolutions
- final artifacts are regenerated fresh from merged state
- final acceptance loop is green
- `acceptance.md` records command results and final checklist state
- `merge-log.md` records merge SHAs and any mechanical resolutions performed

## Context-Control Rules

- Worker prompts must contain only:
  - relevant `PLAN.md` excerpt
  - owned files
  - forbidden files
  - required commands
  - acceptance criteria
- Do not paste full `PLAN.md`, full repo maps, or unrelated milestone history into worker prompts.
- Parent reviews only:
  - terse worker summary
  - narrow diff
  - command results
- Parent should not review full worker transcripts unless a task is blocked.
- Prefer completion sentinels under `.runs/<TASK_ID>/` or long waits over tight polling.
- Close workers after merge. Do not keep idle workers attached to the task graph.
- Parent-owned files under `.runs/m27` are the orchestration source of truth. Worker chat is not.

## `tasks.json` Minimum Shape

Use one object per task id with:

- `task_id`
- `branch`
- `worktree`
- `owner`
- `status`
- `sentinel_dir`
- `depends_on`

Seed task ids:

- `task/m27-00-baseline-validate`
- `task/m27-a1-fingerprint-helper`
- `task/m27-a2-corpus-manifest-docs`
- `task/m27-b1-xtask-cli-wiring`
- `task/m27-b2-coverage-artifact`
- `task/m27-b3-recommend-artifact`
- `task/m27-c-integrate`

## Final Acceptance Checklist

M27 is complete only when all of the following are true:

1. `cargo xtask family coverage --format json` exists and is deterministic.
2. `cargo xtask family recommend --format json` exists and is deterministic.
3. Both commands print JSON to stdout and atomically write identical bytes to their locked artifact paths.
4. Coverage writes a retained inventory snapshot and cites its path and sha.
5. Coverage reports:
   - promoted function units
   - supported-but-unpromoted function units
   - unsupported function units
   - supported non-function units
6. Unsupported clustering uses the `spec-core` fingerprint helper rather than duplicated `xtask` heuristics.
7. Recommendation ranks only `rankable` clusters.
8. Recommendation can honestly emit:
   - `ranked`
   - `no_strong_candidate`
   - `insufficient_real_corpus`
9. `cargo xtask family validate-artifact <path>` accepts both new M27 artifact kinds.
10. The M26 root recommendation artifact path is not overwritten.
11. The locked three-source M27 corpus basis is covered by regression tests.
12. README documentation matches landed command names, artifact paths, and corpus rules.

## Stop Conditions

- Stop and re-plan before any worker starts if:
  - live inventory truth drifts from `PLAN.md`
  - corpus counts drift from `6 / 12 / 9 / 27`
  - command or artifact authority in `PLAN.md` no longer matches the repo target
- Stop and escalate during execution if:
  - any task needs files outside the locked touch set
  - any task implies a new crate or workspace member
  - integration reveals a semantic disagreement across lane owners

## Completion and Cleanup

- When final acceptance is green:
  - update `$M27_RUN_ROOT/acceptance.md`
  - update `$M27_RUN_ROOT/merge-log.md`
  - leave `.semantic-family-artifacts/*` and `.runs/*` as run artifacts
- Remove worktrees only after the parent confirms integration is complete:

```bash
git worktree remove "$WORKTREE_ROOT/fingerprint"
git worktree remove "$WORKTREE_ROOT/corpus-docs"
git worktree remove "$WORKTREE_ROOT/xtask"
git worktree remove "$WORKTREE_ROOT/int"
```
