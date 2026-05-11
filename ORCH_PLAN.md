# M50 Canonical Seam Family Migration Orchestration Plan

Status: **authoritative orchestration plan for executing M50**  
Supersedes: **the stale M49 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M50: Canonical Seam Family Migration Implementation Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Primary execution branch: **`feat/m40-plus`**  
Kickoff tree expectation: **controlled dirty authority inputs only**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum concurrency after freeze: **3 workers**  
Last rewritten: **2026-05-11**

## Summary

- Execute from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`.
- `PLAN.md` is the sole scope authority. `ORCH_PLAN.md` is the execution contract derived from it.
- Keep the serialized critical path local to the parent agent until the rename map and semantic contract are frozen.
- Parent-owned pre-parallel work is exactly:
  - `spec-core/src/semantic_review.rs`
  - canonical ecommerce source specs and molecule specs under `examples/ecommerce/units/pricing/`
  - raw baseline pricing files under `examples/ecommerce/src/raw_baseline/pricing/`
  - `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`
  - authoritative orchestration state under `.runs/m50_canonical_seam_family_migration/`
- After the freeze gate passes, fork three strict worker lanes:
  - `WS-PROOF-CORE`
  - `WS-PROOF-CLI`
  - `WS-DOCS`
- Parent remains the only integrator and the only writer of authoritative orchestration state.
- Merge `WS-PROOF-CORE` and `WS-PROOF-CLI` first, run targeted proof, then merge `WS-DOCS`, then run the full authoritative proof loop and the final grep exit gate from `PLAN.md`.

## Hard Guards

- `PLAN.md` is the only scope authority for M50. If a requested change is not authorized by `PLAN.md`, stop.
- Kickoff does not require a perfectly clean tree. It does require controlled dirt:
  - allowed dirty tracked paths at kickoff: `PLAN.md`
  - also allowed: `ORCH_PLAN.md` if this replacement plan has been written into place and not yet committed
  - no other modified, deleted, renamed, staged, or untracked path is allowed
- Execute on `feat/m40-plus`. If the starting branch differs, stop and record divergence.
- Before parallelism, no worker exists and no one edits outside the parent-owned pre-freeze surfaces.
- No worker may start before `contract-freeze.json` exists and records the exact `contract_freeze_commit`, rename map, artifact map, and removed legacy keys.
- Generated artifacts are refreshed from source specs only. Never hand-edit:
  - `*.spec.passport.json`
  - `*.test.evidence.json`
  - `examples/ecommerce/src/generated/**`
- M50 does not authorize:
  - new supported seam families
  - new supported function families
  - generic alias or synonym support
  - export schema changes
  - CLI JSON redesign
  - TypeScript scope expansion
  - new crates, workspace members, or module registries
- `WS-PROOF-CORE` must not reopen `semantic_review.rs`.
- `WS-PROOF-CLI` must not decide canonical names by test repair.
- `WS-DOCS` must not edit source specs, proof files, or generated artifacts.
- The final grep exit gate is mandatory. M50 is not complete without it.

## Worktree And Branch Topology

| Lane | Workstream | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m50-parent-authority` | `WS-AUTHORITY` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | kickoff capture, semantic contract freeze, canonical rename, tracked artifact refresh, worker fork |
| `lane/m50-proof-core` | `WS-PROOF-CORE` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m50/proof-core` | `ws/m50-proof-core` | Worker | proof-wall rewiring in `spec-core` |
| `lane/m50-proof-cli` | `WS-PROOF-CLI` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m50/proof-cli` | `ws/m50-proof-cli` | Worker | CLI and regression rewiring |
| `lane/m50-docs` | `WS-DOCS` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m50/docs` | `ws/m50-docs` | Worker | teaching-surface updates |
| `lane/m50-int` | `WS-INT` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m50/int` | `ws/m50-int` | Parent | merge worker lanes, run proof loops, prepare validated result |
| `lane/m50-parent-closeout` | `finalize` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `feat/m40-plus` | Parent | fast-forward validated integration result and write acceptance ledger |

## Canonical Run Root

Use these exact paths:

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m50
RUN_ROOT=$PRIMARY_ROOT/.runs/m50_canonical_seam_family_migration
```

All authoritative M50 orchestration state lives under `RUN_ROOT`.

### File-Of-Record Inventory

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, and dirty-input snapshot | Parent |
| `authority-freeze.json` | frozen scope, lane ownership, guardrails, and command contract | Parent |
| `contract-freeze.json` | frozen rename map, removed keys, freeze commit, worker fork basis | Parent |
| `worktrees.json` | worktree path and branch inventory | Parent |
| `file-ownership.json` | exact writable repo surfaces by lane | Parent |
| `in-scope-files.txt` | exhaustive in-scope files for M50 | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `tasks.json` | durable task ledger | Parent |
| `queue.json` | dependency queue and task state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `acceptance-ledger.md` | final acceptance evidence and signoff ledger | Parent |
| `blocked.json` | blocker artifact on incomplete termination | Parent |
| `authority-snapshot/PLAN.md` | kickoff scope snapshot | Parent |
| `authority-snapshot/ORCH_PLAN.md` | kickoff orchestration snapshot | Parent |
| `authority-snapshot/authority-input.diff` | diff of allowed dirty authority inputs | Parent |
| `validation/kickoff/*` | kickoff command captures | Parent |
| `validation/lane-a/*` | contract-freeze and rename captures | Parent |
| `validation/proof-core/*` | `WS-PROOF-CORE` captures copied back by parent | Parent |
| `validation/proof-cli/*` | `WS-PROOF-CLI` captures copied back by parent | Parent |
| `validation/docs/*` | `WS-DOCS` captures copied back by parent | Parent |
| `validation/int/*` | merge and integrated proof captures | Parent |
| `validation/final/*` | final branch and acceptance captures | Parent |

### Per-Task Sentinel Contract

Every task has a sentinel directory under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m50_canonical_seam_family_migration/tasks/<TASK_ID>/`

Each sentinel directory must contain:

| File | Required contents |
| --- | --- |
| `sentinel.json` | `task_id`, `workstream`, `lane`, `owner`, `owned_files`, `status`, `depends_on`, `started_at`, `completed_at`, `blocker_status`, `submission_commit`, `notes_summary` |
| `commands.ndjson` | one record per command with `command`, `cwd`, `started_at`, `completed_at`, `exit_code`, `stdout_path`, `stderr_path` |
| `owned-files.txt` | exact repo paths owned by that task |
| `result.md` | concise parent-authored outcome, blockers, and next action |

Rules:

- Only the parent writes authoritative sentinel files.
- Workers return narrow summaries plus command outputs; the parent records them into `RUN_ROOT`.
- A task is not complete until `sentinel.json` shows `status: done`.
- A blocked task must set `status: blocked` and identify the exact contract leak or file leak.

### Task Status Vocabulary

Use only these task statuses in `tasks.json` and `queue.json`:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `done`
- `cancelled`

## Kickoff Rule

Dirty kickoff is allowed only for authority inputs. This run must not stop merely because `PLAN.md` is already modified. It must stop if any unexpected tracked or untracked path is dirty.

Exact kickoff commands:

```bash
mkdir -p "$RUN_ROOT"/authority-snapshot
mkdir -p "$RUN_ROOT"/validation/{kickoff,lane-a,proof-core,proof-cli,docs,int,final}
mkdir -p "$RUN_ROOT"/tasks

git branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git rev-parse --short=7 HEAD | tee "$RUN_ROOT/validation/kickoff/head.short.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"

sed -E 's/^...//' "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt" \
  | sed -E 's/^[^ ]+ -> //' \
  > "$RUN_ROOT/validation/kickoff/dirty-paths.txt"

if [ -s "$RUN_ROOT/validation/kickoff/dirty-paths.txt" ] && \
   rg -n -v '^(PLAN\.md|ORCH_PLAN\.md)$' "$RUN_ROOT/validation/kickoff/dirty-paths.txt"; then
  echo "Unexpected dirty or untracked path outside allowed authority inputs" \
    | tee "$RUN_ROOT/validation/kickoff/kickoff-error.txt"
  exit 1
fi

cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/authority-snapshot/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/authority-snapshot/ORCH_PLAN.md"
git diff -- PLAN.md ORCH_PLAN.md > "$RUN_ROOT/authority-snapshot/authority-input.diff"
```

Kickoff acceptance:

- branch is `feat/m40-plus`
- only `PLAN.md` is dirty, or `PLAN.md` plus `ORCH_PLAN.md`
- no unexpected untracked files exist
- authority snapshots are captured before any source edit
- `baseline.json`, `authority-freeze.json`, `tasks.json`, and `queue.json` are written before implementation starts

## Freeze-Commit Hygiene

Kickoff may tolerate dirty `PLAN.md` and `ORCH_PLAN.md` because they are authority inputs. The worker fork basis may not.

Rules for the freeze commit recorded in `contract-freeze.json`:

- The freeze commit must contain only intended M50 source, spec, baseline, plan-spec, and regenerated artifact changes.
- The freeze commit must not accidentally absorb unrelated edits to `PLAN.md`, `ORCH_PLAN.md`, or any other authority-input or orchestration-only file unless the parent explicitly intends to include them.
- `RUN_ROOT/**` state is not part of the freeze commit.
- Before creating the freeze commit, the parent must verify the staged set contains only lane-A source/artifact changes.
- If `PLAN.md` or `ORCH_PLAN.md` are still dirty and not intentionally part of the worker fork basis, they must be left unstaged and excluded from the freeze commit.
- Worker branches and worktrees must fork from the exact freeze commit, not from a dirty working tree snapshot.

Required pre-freeze hygiene commands:

```bash
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/lane-a/pre-freeze-status.porcelain.txt"
git diff --cached --name-only | tee "$RUN_ROOT/validation/lane-a/pre-freeze-staged.txt"
git diff --name-only | tee "$RUN_ROOT/validation/lane-a/pre-freeze-unstaged.txt"

if rg -n '^(PLAN\.md|ORCH_PLAN\.md)$' "$RUN_ROOT/validation/lane-a/pre-freeze-staged.txt"; then
  echo "Authority-input file staged into freeze commit; explicit intent required" \
    | tee "$RUN_ROOT/validation/lane-a/freeze-hygiene-warning.txt"
fi
```

Freeze-commit acceptance:

- the staged set matches the intended M50 lane-A source/artifact surfaces
- no accidental authority-input edits are included
- the resulting commit is a clean, shareable fork basis for all workers

## File Ownership

| Workstream | Exact writable repo surfaces |
| --- | --- |
| `WS-AUTHORITY` | `spec-core/src/semantic_review.rs`; `examples/ecommerce/units/pricing/discount_policy.unit.spec`; `examples/ecommerce/units/pricing/checkout_quote.unit.spec`; `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`; `examples/ecommerce/units/pricing/checkout_flow.test.spec`; tracked canonical example artifacts under `examples/ecommerce/units/pricing/`; `examples/ecommerce/src/raw_baseline/pricing/discount_policy.rs`; `examples/ecommerce/src/raw_baseline/pricing/checkout_quote.rs`; `examples/ecommerce/src/raw_baseline/pricing/mod.rs`; `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`; `RUN_ROOT/**` |
| `WS-PROOF-CORE` | `spec-core/src/export.rs`; `spec-core/src/passport.rs`; `spec-core/src/generator.rs`; `spec-core/src/molecule_evidence.rs`; `spec-core/src/escape_hatch.rs` |
| `WS-PROOF-CLI` | `spec-cli/src/commands.rs`; `spec-cli/tests/cli.rs`; `spec-cli/tests/m14_regressions.rs`; `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`; `spec-cli/tests/fixtures/plan-export-valid-mixed.json` |
| `WS-DOCS` | `README.md`; `examples/ecommerce/README.md`; `examples/ecommerce/src/main.rs`; `AGENTS.md` |
| `WS-INT` | merge mechanics only in `ws/m50-int`; no creative source edits; authoritative validation captures in `RUN_ROOT/**` only |
| `finalize` | fast-forward `feat/m40-plus` to `ws/m50-int`; write `acceptance-ledger.md`; write final captures in `RUN_ROOT/**` |

## Task Ledger

| Task ID | Workstream | Depends on | Purpose |
| --- | --- | --- | --- |
| `task-m50-a0-baseline-freeze` | `WS-AUTHORITY` | — | capture kickoff truth and write baseline state |
| `task-m50-a1-authority-freeze` | `WS-AUTHORITY` | `task-m50-a0-baseline-freeze` | lock scope, ownership, guards, and exact commands |
| `task-m50-a2-semantic-contract-freeze` | `WS-AUTHORITY` | `task-m50-a1-authority-freeze` | remove legacy preserve window and freeze canonical seam-family key policy |
| `task-m50-a3-example-tree-rename-refresh` | `WS-AUTHORITY` | `task-m50-a2-semantic-contract-freeze` | rename canonical example tree, raw baseline, plan refs, and regenerate tracked artifacts from source specs |
| `task-m50-a4-worker-fork` | `WS-AUTHORITY` | `task-m50-a3-example-tree-rename-refresh` | write `contract-freeze.json`, create worker and integration worktrees from freeze commit |
| `task-m50-b1-proof-core` | `WS-PROOF-CORE` | `task-m50-a4-worker-fork` | retarget `spec-core` proof walls |
| `task-m50-c1-proof-cli` | `WS-PROOF-CLI` | `task-m50-a4-worker-fork` | retarget CLI and regression surfaces |
| `task-m50-d1-docs` | `WS-DOCS` | `task-m50-a4-worker-fork` | update teaching surfaces |
| `task-m50-e1-integrate-core-cli` | `WS-INT` | `task-m50-b1-proof-core`, `task-m50-c1-proof-cli` | merge proof lanes and run targeted proof suites |
| `task-m50-e2-integrate-docs` | `WS-INT` | `task-m50-e1-integrate-core-cli`, `task-m50-d1-docs` | merge docs lane onto validated integration branch |
| `task-m50-e3-full-proof-loop` | `WS-INT` | `task-m50-e2-integrate-docs` | run full authoritative proof loop and final grep gate |
| `task-m50-f1-parent-closeout` | `finalize` | `task-m50-e3-full-proof-loop` | fast-forward validated result and finalize acceptance ledger |

## Worker Prompt And Context-Control Rules

The parent agent owns orchestration state and controls context hygiene. Workers operate with bounded prompts and bounded outputs only.

### What each worker prompt must contain

Every worker prompt must contain exactly these categories of information:

- the worker task id and workstream name
- the exact owned file set for that lane
- the exact forbidden-touch file set
- the exact `PLAN.md` excerpts relevant to that lane
- the exact `contract-freeze.json` facts relevant to that lane:
  - `contract_freeze_commit`
  - canonical rename map
  - artifact rename map
  - removed legacy seam-family keys
  - canonical seam-family keys
- the exact required commands for that lane
- the exact acceptance criteria for that lane
- the exact stop rules for that lane
- the requirement to use GPT-5.4 with `reasoning_effort=high`
- the instruction that generated artifacts are not to be hand-edited
- the instruction that the worker must not widen scope or reinterpret the freeze contract

Each worker prompt must not contain:

- full prior worker transcripts
- full parent session history
- unrelated repo context
- broad brainstorming about alternative rename strategies
- permission to modify out-of-scope files

### What each worker must return

Each worker must return only:

- changed files
- commands run, with exit codes
- blockers or unresolved assumptions
- final branch name and final commit sha for the lane

Workers must not return:

- full raw transcripts
- repeated copies of the prompt
- speculative redesign proposals outside owned files
- new orchestration state files under `RUN_ROOT/**`

### Parent review rules

- The parent reviews summaries plus narrow diffs only, not full worker transcripts.
- The parent records authoritative outcomes into `RUN_ROOT/tasks/<TASK_ID>/`.
- The parent closes each worker immediately after that worker is merged or explicitly rejected.
- Prefer completion sentinels or long waits over tight polling loops when checking worker completion.
- Workers are not kept alive after merge “just in case.” Any new work requires a new bounded prompt.

## Worker Submission Requirements

Every worker lane submission is incomplete unless it includes all of these:

- changed files
- commands run with exit codes
- blockers or unresolved assumptions
- final branch name for the lane
- final commit sha for the lane

Parent-side submission handling:

- record the submission into `tasks/<TASK_ID>/result.md`
- copy command outputs into the matching `validation/<lane>/` directory
- set `sentinel.json.status` to `submitted` before review
- set `sentinel.json.status` to `done` only after merge and validation
- set `sentinel.json.status` to `blocked` if the submission leaks scope, misses required outputs, or fails acceptance

## Workstream Plan

### WS-AUTHORITY (`lane/m50-parent-authority`, parent only, sequential)

Purpose: freeze the rename contract, perform the canonical rename, refresh tracked artifacts from source specs, and create the only gate before parallelism.

Required commands:

```bash
git branch --show-current
git status --porcelain=v1 -uall
git rev-parse HEAD
git rev-parse --short=7 HEAD

cargo test -p spec-core semantic_review

cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_strategy.unit.spec --format json
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/pricing_quote.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_flow.test.spec
cargo run -p spec-cli -- plan validate examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec --format json
```

Required work:

- Write `baseline.json`, `authority-freeze.json`, `worktrees.json`, `file-ownership.json`, `in-scope-files.txt`, `out-of-scope-files.txt`, `tasks.json`, and `queue.json`.
- Freeze the semantic contract in `spec-core/src/semantic_review.rs`:
  - canonical keys remain `sum.discount_strategy.v1` and `data.pricing_quote.v1`
  - legacy preserve keys `sum.discount_policy.v1` and `data.checkout_quote.v1` are removed
  - retained old-name literals are allowed only inside narrow rejection assertions
- Rename the canonical example source tree:
  - `pricing/discount_policy` -> `pricing/discount_strategy`
  - `pricing/checkout_quote` -> `pricing/pricing_quote`
  - `discount_policy.unit.spec` -> `discount_strategy.unit.spec`
  - `checkout_quote.unit.spec` -> `pricing_quote.unit.spec`
  - `discount_policy_checkout_flow.test.spec` -> `discount_strategy_checkout_flow.test.spec`
  - `pricing/discount_policy_checkout_flow` -> `pricing/discount_strategy_checkout_flow`
- Update `checkout_flow.test.spec` in place so its data-seam references move to `pricing/pricing_quote` while the filename stays the same.
- Rename the raw baseline pricing modules and update `examples/ecommerce/src/raw_baseline/pricing/mod.rs`.
- Update `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`.
- Refresh tracked canonical artifacts by running `spec build` and targeted `spec test`. Do not hand-edit artifact JSON.
- Remove obsolete old-path tracked artifacts after new ones are truthfully regenerated.
- Create a single freeze commit that contains:
  - semantic contract freeze
  - example rename
  - raw baseline rename
  - refreshed tracked artifacts
  - plan-spec acceptance rename

WS-AUTHORITY acceptance:

- `semantic_review.rs` rejects legacy seam-family preserve keys and still emits the canonical keys on refresh.
- The canonical example tree loads truthfully under the renamed ids and paths.
- Tracked artifacts exist at the new canonical paths because `spec build` and `spec test` regenerated them.
- `contract-freeze.json` records:
  - `contract_freeze_commit`
  - `contract_freeze_commit_short`
  - `rename_map`
  - `artifact_map`
  - `removed_legacy_keys`
  - `canonical_family_keys`
  - `targeted_commands_green`
  - worker and integration branch names and paths
- No worker has started before `contract-freeze.json` is written.

WS-AUTHORITY stop rules:

- If any dirty or untracked path exists outside `PLAN.md` or `ORCH_PLAN.md`, stop.
- If `cargo test -p spec-core semantic_review` is red after the semantic freeze, do not parallelize.
- If the example tree cannot regenerate truthful new artifacts from source specs, do not parallelize.
- If the rename map changes after worker branches are created, invalidate all worker branches, write the reason to `blocked.json`, and refork from a new freeze commit.

### WS-PROOF-CORE (`lane/m50-proof-core`, worker, post-freeze only)

Purpose: retarget `spec-core` proof walls to the frozen canonical names and artifact paths.

Required commands:

```bash
cargo test -p spec-core export
cargo test -p spec-core passport
cargo test -p spec-core generator
cargo test -p spec-core molecule_evidence
cargo test -p spec-core escape_hatch
```

Required work:

- Retarget canonical fixture ids, file paths, molecule ids, and artifact paths in:
  - `spec-core/src/export.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/generator.rs`
  - `spec-core/src/molecule_evidence.rs`
  - `spec-core/src/escape_hatch.rs`
- Keep old-name literals only where the assertion is intentionally about rejection or historical compatibility behavior.
- Consume the freeze contract literally. This lane does not reinterpret canonical naming.

WS-PROOF-CORE acceptance:

- Only the five owned files are changed.
- All five targeted `cargo test -p spec-core ...` commands are green.
- No edit reopens `semantic_review.rs`.
- No edit invents new alias behavior or new family semantics.

WS-PROOF-CORE stop rules:

- If a fix requires `semantic_review.rs`, bounce it back to the parent and stop the lane.
- If a fix requires `spec-cli/**` or docs surfaces, stop and return the blocker.
- If a failing assertion implies schema or CLI behavior changes beyond rename completion, stop and re-scope.

### WS-PROOF-CLI (`lane/m50-proof-cli`, worker, post-freeze only)

Purpose: retarget command, CLI, regression, and plan-fixture surfaces to the frozen canonical names.

Required commands:

```bash
cargo test -p spec-cli cli
cargo test -p spec-cli m14_regressions
```

Required work:

- Retarget current-state fixtures and assertions in:
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`
  - `spec-cli/tests/fixtures/plan-export-valid-mixed.json`
- Rewrite current-state tests away from legacy seam-family keys as default truth.
- Keep retained old-name literals only in narrow legacy-specific rejection assertions.

WS-PROOF-CLI acceptance:

- Only the five owned files are changed.
- `cargo test -p spec-cli cli` is green.
- `cargo test -p spec-cli m14_regressions` is green.
- No rename decision is made implicitly through test repair; it must match `contract-freeze.json`.

WS-PROOF-CLI stop rules:

- If a fix requires example source changes, bounce it back to `WS-AUTHORITY`.
- If a fix requires `semantic_review.rs`, stop and return the blocker.
- If a fix requires docs edits, leave that to `WS-DOCS`.

### WS-DOCS (`lane/m50-docs`, worker, post-freeze only)

Purpose: make all maintained teaching surfaces speak only the canonical M50 vocabulary.

Required commands:

```bash
rg -n "pricing/discount_policy|pricing/checkout_quote|sum.discount_policy.v1|data.checkout_quote.v1" \
  README.md AGENTS.md examples/ecommerce/README.md examples/ecommerce/src/main.rs
```

Required work:

- Update:
  - `README.md`
  - `examples/ecommerce/README.md`
  - `examples/ecommerce/src/main.rs`
  - `AGENTS.md`
- Retarget:
  - canonical file inventories
  - maintainer commands
  - narrative text
  - nearby ASCII diagrams and workflow snippets
- Remove stale current-state references to old canonical names.

WS-DOCS acceptance:

- Only the four owned files are changed.
- The grep command above returns no stale current-state old-name literals in docs surfaces.
- Commands and file paths in docs resolve to the renamed canonical files.

WS-DOCS stop rules:

- If docs repair requires source-spec or proof-wall edits, stop and bounce the mismatch to the owning lane.
- Do not touch generated artifacts or code fixtures.

### WS-INT (`lane/m50-int`, parent only)

Purpose: merge validated worker lanes, run integrated proof, and produce the final authoritative result.

Required merge order:

1. Merge `ws/m50-proof-core`.
2. Merge `ws/m50-proof-cli`.
3. Run targeted proof suites.
4. Merge `ws/m50-docs`.
5. Run the full authoritative proof loop.
6. Run the final grep exit gate.

Exact integration commands:

```bash
git merge --no-ff ws/m50-proof-core
git merge --no-ff ws/m50-proof-cli

cargo test -p spec-core export
cargo test -p spec-core passport
cargo test -p spec-core generator
cargo test -p spec-core molecule_evidence
cargo test -p spec-core escape_hatch
cargo test -p spec-cli cli
cargo test -p spec-cli m14_regressions

git merge --no-ff ws/m50-docs

cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_strategy.unit.spec --format json
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/pricing_quote.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_flow.test.spec
cargo run -p spec-cli -- plan validate examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec --format json
cargo run -p spec-cli -- status examples/ecommerce --format json

cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-core passport
cargo test -p spec-core generator
cargo test -p spec-core molecule_evidence
cargo test -p spec-core escape_hatch
cargo test -p spec-cli cli
cargo test -p spec-cli m14_regressions
```

Broader suites are conditional. Run them only if targeted fallout leaks outside the named proof walls:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo test --manifest-path examples/ecommerce/Cargo.toml
```

Final grep exit gate from `PLAN.md`:

```bash
rg -n "pricing/discount_policy|pricing/checkout_quote|sum.discount_policy.v1|data.checkout_quote.v1" \
  README.md AGENTS.md examples/ecommerce spec-core spec-cli/tests spec-cli/src/commands.rs spec-core/src
```

WS-INT acceptance:

- `ws/m50-int` descends from the recorded `contract_freeze_commit`.
- `WS-PROOF-CORE` and `WS-PROOF-CLI` merge cleanly or are bounced back for repair.
- The targeted proof suites are green before docs merge.
- The full authoritative proof loop is green after docs merge.
- The grep exit gate is green, or every remaining hit is intentional, legacy-specific, and logged in `acceptance-ledger.md`.

## Worktree Creation Commands

Run these only after `task-m50-a4-worker-fork` has written `contract-freeze.json` and the freeze commit exists:

```bash
FREEZE_COMMIT=$(jq -r '.contract_freeze_commit' "$RUN_ROOT/contract-freeze.json")

git worktree add -b ws/m50-proof-core "$WT_ROOT/proof-core" "$FREEZE_COMMIT"
git worktree add -b ws/m50-proof-cli "$WT_ROOT/proof-cli" "$FREEZE_COMMIT"
git worktree add -b ws/m50-docs "$WT_ROOT/docs" "$FREEZE_COMMIT"
git worktree add -b ws/m50-int "$WT_ROOT/int" "$FREEZE_COMMIT"
```

Rules:

- All worker and integration branches must fork from exactly `FREEZE_COMMIT`.
- No worker may branch from a later or earlier `feat/m40-plus` HEAD.
- If the freeze basis changes, delete and recreate all worker worktrees from the new freeze commit.

## Merge Rules

- Parent is the only integrator.
- Workers submit code changes and narrow summaries only.
- Parent records submissions in `tasks/<TASK_ID>/`.
- Merge worker lanes into `ws/m50-int`, never directly into `feat/m40-plus`.
- Use `git merge --no-ff` for worker merges into `ws/m50-int`.
- Use `git merge --ff-only ws/m50-int` when updating `feat/m40-plus` during closeout.
- If `git merge --ff-only ws/m50-int` fails, stop and record divergence. Do not invent a manual closeout merge on the parent lane.

## Conflict Bounce-Back Rules

- If a merge conflict touches a lane-owned file, bounce the conflict back to that lane owner. Integration does not repair it creatively.
- If `WS-PROOF-CORE` or `WS-PROOF-CLI` needs a rename-map change, invalidate the freeze and return to `WS-AUTHORITY`.
- If a conflict occurs only because generated artifacts changed in parallel, resolve by preserving source edits, rerunning the authoritative `spec build` and `spec test` loop, and recommitting regenerated truth in `ws/m50-int`.
- If docs reveal a source mismatch, bounce it back to the owning code lane. Docs do not become a hidden source-edit lane.
- If a worker touches an out-of-scope file, mark the task `blocked`, reject the submission, and relaunch the lane from the freeze commit.

## Escalation Rules

Stop and write `blocked.json` if any of these becomes true:

- a change requires export schema evolution
- a change requires new CLI semantics beyond rename completion
- the rename cannot be expressed without adding generic alias support
- a worker cannot finish without editing a file owned by another lane
- the canonical example rename breaks public behavior outside the surfaces named in `PLAN.md`
- `spec plan validate` requires semantics beyond local-id rename
- the final grep gate still finds unclassified old-name literals in current-state surfaces
- targeted proof remains red after the owning lane has consumed the frozen contract literally

`blocked.json` must record:

- `run_id`
- `blocked_task_id`
- `freeze_commit`
- `blocking_surface`
- `reason`
- `required_rescope`
- `recommended_next_action`

## Final Acceptance Criteria

M50 is complete only when all of these are true:

- `feat/m40-plus` contains the validated integrated result.
- Canonical seam ids are:
  - `pricing/discount_strategy`
  - `pricing/pricing_quote`
- Canonical seam-family keys are:
  - `sum.discount_strategy.v1`
  - `data.pricing_quote.v1`
- Legacy seam-family preserve keys are gone from current-state behavior:
  - `sum.discount_policy.v1`
  - `data.checkout_quote.v1`
- Canonical source files exist at the renamed paths.
- Canonical raw baseline modules exist at the renamed paths.
- The mixed-kind molecule is renamed to `discount_strategy_checkout_flow`.
- `checkout_flow.test.spec` remains in place but points at `pricing/pricing_quote`.
- Tracked canonical artifacts were refreshed from source specs, not hand-edited.
- `cargo run -p spec-cli -- status examples/ecommerce --format json` is green for the authoritative example root.
- All targeted proof commands listed in `WS-INT` are green.
- Any broader suite run during escalation is green and captured in `RUN_ROOT/validation/final/`.
- The final grep exit gate passes, or every remaining hit is intentionally historical and explicitly logged in `acceptance-ledger.md`.
- `acceptance-ledger.md` records:
  - final branch and commit
  - freeze commit
  - merged worker commits
  - commands run
  - pass/fail outcomes
  - any intentional retained legacy-only literals
  - closeout timestamp

## Parent Closeout

Exact closeout commands:

```bash
git checkout feat/m40-plus
git merge --ff-only ws/m50-int

git rev-parse HEAD | tee "$RUN_ROOT/validation/final/final-head.txt"
git rev-parse --short=7 HEAD | tee "$RUN_ROOT/validation/final/final-head.short.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/final/final-status.porcelain.txt"
```

Closeout acceptance:

- `feat/m40-plus` fast-forwards to the validated integration result
- final captures are written under `RUN_ROOT/validation/final/`
- `acceptance-ledger.md` is complete
- worker branches can be pruned later, but pruning is not part of M50 acceptance

## Recommended Next Action

Execute `WS-AUTHORITY` first. Freeze the rename map, remove the legacy seam-key preserve window, rename the canonical example tree, refresh the tracked artifacts from source specs, and fork all worker lanes from that single frozen commit. Until that gate is green, parallelism is premature.
