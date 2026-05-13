# M55 Bounded Cross-Library TypeScript Helper Imports Orchestration Runbook

Status: **authoritative execution runbook**  
Supersedes: **the stale M54 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M55: Bounded Cross-Library TypeScript Helper Imports Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Base branch: **`main`**  
Authority date: **`2026-05-13`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum safe worker concurrency: **2 concurrent worker lanes, and only after validator contract freeze**  
Last rewritten: **`2026-05-13`**

## Summary

This runbook turns `PLAN.md` into an operator-safe parent/worker execution flow for M55.

M55 is narrow. It extends the existing Bun-backed TypeScript lane to allow **bounded cross-library helper imports only**. It does **not** authorize generic cross-library TypeScript execution. It does **not** authorize direct cross-library wrapper roots. It does **not** authorize direct cross-library chain3 roots. It does **not** authorize test-only mutation of the canonical example as an escape hatch.

The operator truth for M55 is:

1. The canonical real-user proof path is `examples/crosslib-app/units/pricing/apply_tax.unit.spec`.
2. The parent owns kickoff, contract freeze, validator changes, all integration, all gates, and final signoff.
3. The validator contract freezes first in `feat/m40-plus`.
4. Only after that freeze may two worker lanes overlap:
   - backend helper resolution and import rendering
   - CLI proof wall plus maintained example authoring and any pre-frozen additive support-fixture work
5. Docs and backlog updates are strictly last.
6. The parent is the only integrator. Workers never merge each other.
7. If the only way to make the canonical example pass is to inject temporary `body.typescript` during tests, stop. That is outside M55.

`PLAN.md` is the only authority for product scope. The existing `ORCH_PLAN.md` is structural reference only.

## Hard Guards

- `PLAN.md` is the sole scope authority.
- M55 supports cross-library imports only when the imported dep is a helper leaf in a helper slot already legal in the bounded TypeScript lane.
- Direct wrapper root deps remain local-only.
- Direct chain3 root deps remain local-only.
- The canonical green path is `examples/crosslib-app/units/pricing/apply_tax.unit.spec`.
- Recursive wrapper and chain3 shared-helper reuse must be proven, but a second maintained public example is not required.
- The parent owns `spec-core/src/validator.rs` for the entire run. No worker edits it.
- The parent is the only integrator onto `feat/m40-plus`.
- Workers may edit source `.unit.spec` files only. They must not author `.spec.passport.json`, `.test.evidence.json`, or generated code as source truth.
- `README.md`, `examples/crosslib-app/README.md`, `CHANGELOG.md`, and `TODOS.md` stay untouched until proof is green.
- `spec-cli/tests/cli.rs` has one owner only.
- No new crates, commands, runtime channels, schema changes, or broad resolver stack.
- Support-fixture ownership for `WS-B-PROOF` must be frozen to exact file paths before that lane starts. Default is an empty list.

Stop and re-scope immediately if any of these become true:

1. The canonical example needs direct cross-library wrapper or chain3 root deps to pass.
2. Backend support requires a second TypeScript-only library resolver instead of loaded-unit truth.
3. The validator or backend change implies generic cross-library TypeScript execution.
4. Recursive proof requires generic graph execution instead of the bounded closure contract.
5. Passport, export, or status schemas need new fields.
6. The canonical example can only pass via `inject_typescript_body_if_missing`, test-only file mutation, or any other temporary body injection trick.
7. Docs would need to promise anything broader than “cross-library helper imports in the bounded TypeScript lane.”

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m55
RUN_ROOT=$PRIMARY_ROOT/.runs/m55_bounded_cross_library_helper_imports
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, validator, integration, final proof wall |
| `WS-A-BACKEND` | `$WT_ROOT/ws-a-backend` | `codex/m55-backend-helper-imports` | Worker | `spec-core/src/typescript_backend.rs` only |
| `WS-B-PROOF` | `$WT_ROOT/ws-b-proof` | `codex/m55-cli-example-proof` | Worker | CLI proof wall, canonical example bodies, exact pre-frozen support fixtures |
| `WS-C-DOCS` | `$WT_ROOT/ws-c-docs` | `codex/m55-docs-last` | Worker | docs and backlog after proof is green |

### Worktree creation rules

- Do not create any worker worktree before `M55-02` validator freeze is integrated in `feat/m40-plus`.
- Create `WS-A-BACKEND` and `WS-B-PROOF` from the same frozen `feat/m40-plus` head.
- Create `WS-C-DOCS` only after `M55-21` is green.
- If the primary tree is dirty, record it in `baseline.json` before creating worktrees. Do not stash or clean by default.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-a-backend" -b codex/m55-backend-helper-imports feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-b-proof" -b codex/m55-cli-example-proof feat/m40-plus
# create ws-c-docs only after proof is green
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-c-docs" -b codex/m55-docs-last feat/m40-plus
```

## Durable Orchestration State

All durable session state lives under:

```bash
$PRIMARY_ROOT/.runs/m55_bounded_cross_library_helper_imports
```

This directory is orchestration state, not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, baseline command expectations | Parent |
| `contract-freeze.json` | frozen M55 contract, negative wall, stop rules, unlock rules | Parent |
| `worktrees.json` | exact worktree paths, branches, and lane states | Parent |
| `file-ownership.json` | exact owned file map per task and lane | Parent |
| `tasks.json` | durable task definitions, dependencies, and states | Parent |
| `queue.json` | runnable queue and current task state machine | Parent |
| `session-log.md` | chronological run log with launches, submissions, integrations, and stops | Parent |
| `acceptance-ledger.md` | final signoff checklist and artifact references | Parent |
| `final-proof-manifest.json` | final proof commands, exit codes, and artifact paths | Parent |
| `final-diff-summary.md` | parent-authored summary of landed diffs by lane | Parent |
| `validation/kickoff/` | branch, head, git-status, authority snapshots | Parent |
| `validation/baseline/` | pre-change proof captures, including expected failure on canonical TS path | Parent |
| `validation/validator/` | validator-focused proof captures | Parent |
| `validation/backend/` | backend-focused proof captures and import-render evidence | Parent |
| `validation/proof/` | CLI/example/support-fixture proof captures | Parent |
| `validation/docs/` | wording review captures | Parent |
| `validation/final/` | final serial proof wall and closeout evidence | Parent |
| `handoffs/` | worker briefs and worker result summaries | Parent |

### Required `baseline.json` contents

`baseline.json` must include at least:

- `milestone`: `M55`
- `authority_plan_path`
- `authority_plan_head_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `canonical_example_path`
- `baseline_commands`
- `baseline_expected_truth`
- `baseline_artifact_paths`
- `stop_rules_version`

### Required `contract-freeze.json` contents

`contract-freeze.json` must include at least:

- `milestone`: `M55`
- `authority_plan_path`
- `authority_plan_head_commit`
- `frozen_at_primary_commit`
- `primary_branch`
- `canonical_example_path`
- `exact_scope_claim`
- `negative_wall`
- `allowed_worker_lanes`
- `file_ownership_version`
- `support_fixture_paths_for_ws_b_proof`
- `phase_commands`
- `integration_order`
- `merge_conflict_policy`
- `worker_return_contract`
- `exact_stop_rules`

### Queue state machine

Every task in `tasks.json` and `queue.json` uses only these states:

- `queued`
- `ready`
- `running`
- `blocked`
- `submitted`
- `integrated`
- `closed`

A worker may move a task only to `submitted` or `blocked`. Only the parent may mark `integrated` or `closed`.

## Per-Task Sentinel Convention

Each task gets a dedicated sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Required files:

- `status.json`
- `owner.txt`
- `branch.txt`
- `write_scope.txt`
- `commands.txt`
- `changed_files.txt`
- `acceptance.md`
- `blocker.md`

### Sentinel file meanings

- `status.json`: `task_id`, `state`, `owner`, `depends_on`, `started_at`, `submitted_at`, `integrated_at`
- `owner.txt`: `parent` or the exact worker lane id
- `branch.txt`: execution branch for the task
- `write_scope.txt`: frozen allowed write scope for the task
- `commands.txt`: exact commands run and exit codes
- `changed_files.txt`: newline-delimited changed file list
- `acceptance.md`: what was proven, what remains open
- `blocker.md`: one concrete blocker or `none`

Sentinel rules:

- The parent creates every sentinel directory before a task starts.
- The worker result is considered incomplete until `commands.txt`, `changed_files.txt`, and `acceptance.md` are populated.
- Chat history is not the durable ledger.
- A task is not done when a worker says “finished”; it is done only after parent integration and gate rerun.

## Context-Control Rules

- The parent owns `PLAN.md`, this runbook, and all `.runs/m55_*` state.
- Workers get only the minimum prompt necessary: goal, scope, owned files, stop rules, acceptance, and the exact commands they should run.
- Do not forward one worker’s raw transcript into another worker.
- Worker outputs must stay narrow.
- No worker may expand its write scope mid-flight. Any new file request goes back to the parent.
- `spec-core/src/` is a contract seam. Only the parent touches `validator.rs`; only `WS-A-BACKEND` touches `typescript_backend.rs`.
- `spec-cli/tests/cli.rs` is conflict-prone. Only `WS-B-PROOF` touches it.
- The canonical example source files stay owned by `WS-B-PROOF`, not docs.
- Docs worker never edits product code or spec fixtures.
- The parent keeps summaries small and updates `queue.json` instead of carrying state in chat.
- When validation feedback matters, use machine-readable or targeted test outputs and store them under `validation/*`; do not rely on memory or paraphrased stderr.

## File Ownership Map

### Parent-owned throughout

- `spec-core/src/validator.rs`
- all files under `$RUN_ROOT/`
- final integration commits on `feat/m40-plus`

### `WS-A-BACKEND` owned files

- `spec-core/src/typescript_backend.rs`

### `WS-B-PROOF` owned files

Always owned:

- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`

Conditionally owned:

- exact additive support-fixture file paths frozen by the parent in `file-ownership.json` before launch

Rules for support fixtures:

- Default is `[]`.
- The parent must decide before launch whether support fixtures are needed.
- If needed, the parent must list exact fixture file paths, not globs.
- If later evidence shows additional fixture files are required, stop `WS-B-PROOF`, update `file-ownership.json`, and relaunch or re-brief. Do not silently broaden scope mid-lane.

### `WS-C-DOCS` owned files

- `README.md`
- `examples/crosslib-app/README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Worker Return Contract

Each worker returns only:

- changed files
- commands run with exit codes
- blockers or unresolved assumptions

Each worker return must be written into its handoff packet under `$RUN_ROOT/handoffs/<TASK_ID>.md` and mirrored into the task sentinel files.

Parent review rules:

- The parent reviews narrow diffs only, scoped to the lane’s ownership map.
- The parent reviews command outcomes and blockers, not full worker transcripts.
- The parent integrates one worker lane at a time.
- The parent reruns the relevant gates after each integration before touching the next lane.
- After a lane is integrated or rejected, the parent closes that worker. Workers do not remain open as ad hoc follow-up agents.
- If merge feedback requires edits outside the lane’s ownership map, bounce the lane back with a narrower brief or explicit ownership-map update first. Do not silently broaden scope.

## Workstream Plan

| ID | Task | Owner | Write scope | Depends on | Unlock condition | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- |
| `M55-00` | Kickoff + baseline capture | Parent | `$RUN_ROOT/**` | none | repo available on `feat/m40-plus` | authority snapshots, dirty-tree capture, baseline proofs stored |
| `M55-01` | Contract freeze + ownership map | Parent | `$RUN_ROOT/**` | `M55-00` | baseline recorded | `contract-freeze.json`, `file-ownership.json`, `tasks.json`, `queue.json`, `worktrees.json` frozen |
| `M55-02` | Validator contract | Parent | `spec-core/src/validator.rs` | `M55-01` | contract frozen | validator change integrated on `feat/m40-plus` with focused proof green |
| `M55-10` | Backend helper resolution + import rendering | `WS-A-BACKEND` | `spec-core/src/typescript_backend.rs` | `M55-02` | validator contract integrated | backend lane submitted with tests and no scope drift |
| `M55-20` | CLI proof wall + canonical example truth + exact support fixtures | `WS-B-PROOF` | `spec-cli/tests/cli.rs`, canonical example specs, exact pre-frozen support fixtures | `M55-02` | validator contract integrated and fixture paths frozen | worker submits green proof wall or explicit blocker |
| `M55-21` | Parent integration gate for backend then proof lane | Parent | `feat/m40-plus` integration only | `M55-10`, `M55-20` | both worker lanes submitted | both diffs integrated by parent and post-merge proof reruns green |
| `M55-30` | Docs + backlog sync | `WS-C-DOCS` | docs files only | `M55-21` | proof green and wording frozen | docs submitted with exact narrow M55 language |
| `M55-31` | Parent docs integration gate | Parent | `feat/m40-plus` integration only | `M55-30` | docs lane submitted | docs integrated and wording verified against proof |
| `M55-40` | Final serial proof wall + closeout | Parent | `$RUN_ROOT/**` and minimal fix-forward if required | `M55-31` | all prior tasks integrated | final commands pass, manifests written, closeout recorded |

### `M55-00` Kickoff + baseline capture

Owner: Parent  
Write scope: `$RUN_ROOT/**`

Required captures:

```bash
mkdir -p "$RUN_ROOT"/{validation/{kickoff,baseline,validator,backend,proof,docs,final},tasks,handoffs}

git -C "$PRIMARY_ROOT" branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git -C "$PRIMARY_ROOT" rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git -C "$PRIMARY_ROOT" status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.previous.md"
```

Baseline proof captures:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/baseline/spec-core-typescript-target.txt"
cargo test -p spec-core typescript_tree | tee "$RUN_ROOT/validation/baseline/spec-core-typescript-tree.txt"
cargo test -p spec-cli typescript | tee "$RUN_ROOT/validation/baseline/spec-cli-typescript.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/baseline/canonical-apply-tax-typescript-test.txt"

cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json \
  | tee "$RUN_ROOT/validation/baseline/canonical-crosslib-status-typescript.json"
```

`baseline.json` must record:

- authority commit from `PLAN.md`
- current branch
- current HEAD commit
- dirty-tree summary and file list
- canonical example path
- exact baseline commands
- expected baseline truth:
  - current canonical TypeScript path is not yet green
  - current M55 negative wall is not yet fully landed
- artifact paths for all kickoff and baseline captures
- exact stop rules snapshot

Exit criteria:

- kickoff snapshots exist
- current branch is `feat/m40-plus`
- baseline captures prove current pre-M55 behavior
- any existing dirty files are recorded, not “cleaned up”

Stop rule:

- If baseline already shows the canonical TypeScript path green, stop and reconcile `PLAN.md` first.

### `M55-01` Contract freeze + ownership map

Owner: Parent  
Write scope: `$RUN_ROOT/**`

Required artifacts:

- `baseline.json`
- `contract-freeze.json`
- `worktrees.json`
- `file-ownership.json`
- `tasks.json`
- `queue.json`
- `session-log.md`

`contract-freeze.json` must freeze:

- authority commit
- current branch
- current primary commit
- dirty-tree summary reference
- canonical example path
- exact M55 scope claim
- negative wall list:
  - unresolved alias
  - missing shared helper
  - wrong helper family
  - missing helper `body.typescript`
  - direct shared wrapper root dep
  - direct shared chain3 root dep
- allowed worker lanes:
  - `WS-A-BACKEND`
  - `WS-B-PROOF`
  - `WS-C-DOCS`
- exact commands for each phase
- exact stop rules
- integration order:
  - parent validator
  - backend lane
  - proof lane
  - docs lane
  - final closeout
- merge conflict policy
- worker return contract
- exact support-fixture paths for `WS-B-PROOF`, or `[]` if none

Additional freeze requirements:

- `file-ownership.json` must list lane-owned files exactly.
- `worktrees.json` must include intended paths and branches before any worktree is created.
- `tasks.json` and `queue.json` must already reflect the serialized gates.

Exit criteria:

- all artifacts above exist
- all tasks are queued with explicit dependencies
- support-fixture ownership for `WS-B-PROOF` is either exact and finite or explicitly empty
- worker prompts can be generated without re-reading the whole repo

### `M55-02` Validator contract

Owner: Parent  
Write scope: `spec-core/src/validator.rs`

Required outcomes:

- legal shared helper dep accepted in legal helper slot
- unresolved alias rejected clearly
- missing shared helper rejected clearly
- wrong helper family rejected clearly
- missing helper `body.typescript` rejected clearly
- direct shared wrapper dep rejected before Bun
- direct shared chain3 dep rejected before Bun
- user-facing wording says the narrow M55 thing, not broad cross-library TS support

Required proof surface:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/validator/spec-core-typescript-target.txt"
```

Exit criteria:

- validator change committed in `feat/m40-plus`
- validator proof capture is green
- `contract-freeze.json` is updated with final wording if test strings changed
- parent records worker launch brief inputs

Stop rule:

- If validator work forces edits outside `validator.rs`, pause and decide in parent before any worker starts.

## Concrete Parent-Agent Responsibilities

The parent owns:

- reading `PLAN.md` and translating it into the frozen orchestration contract
- creating `RUN_ROOT`, run-state files, task sentinels, and validation directories
- creating worktrees and branches for each worker lane
- generating worker briefs and handoff packets under `$RUN_ROOT/handoffs/`
- freezing exact support-fixture ownership before launching `WS-B-PROOF`
- running and recording kickoff and baseline commands
- freezing exact phase commands in `contract-freeze.json`
- executing the entire validator phase in `spec-core/src/validator.rs`
- launching workers only after validator freeze is integrated
- integration ordering:
  - backend first
  - proof lane second
  - docs last
- integrating one lane at a time
- rerunning the relevant gates after each merge
- recording every run-state transition in `queue.json`, `tasks.json`, and `session-log.md`
- enforcing ownership boundaries and stop rules
- writing `acceptance-ledger.md`, `final-proof-manifest.json`, and `final-diff-summary.md`
- final acceptance judgment

The parent must not:

- launch workers before `M55-02`
- leave support-fixture ownership ambiguous for `WS-B-PROOF`
- batch unresolved worker diffs together in one merge step
- let workers self-integrate
- silently absorb scope changes discovered during merge feedback

## Concrete Worker-Lane Responsibilities

### `WS-A-BACKEND`

Owned files:

- `spec-core/src/typescript_backend.rs`

Responsibilities:

- implement helper resolution and relative import rendering inside the bounded M55 contract
- reuse loaded-unit truth rather than introducing a second resolver
- preserve deterministic and deduplicated bounded closure emission
- keep changes limited to backend logic and backend-facing tests only if already co-located in the same file scope
- return only changed files, commands with exit codes, and blockers

Stop if:

- work requires edits to `spec-core/src/validator.rs`
- work requires edits to `spec-cli/tests/cli.rs`
- work implies generic cross-library TypeScript execution
- work requires a second import resolver stack
- work needs files outside its ownership map

### `WS-B-PROOF`

Owned files:

- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`
- exact support-fixture files frozen in `file-ownership.json` before launch, or none

Responsibilities:

- make the canonical example green through authored source truth
- add or update the M55 CLI proof wall
- prove recursive wrapper and chain3 shared-helper reuse
- keep support-fixture edits additive, minimal, and limited to exact pre-frozen paths
- avoid test-only canonical mutation
- return only changed files, commands with exit codes, and blockers

Stop if:

- no pre-frozen support fixture exists and a new one is needed
- more fixture files are needed than were frozen in `file-ownership.json`
- work requires edits to `spec-core/src/typescript_backend.rs` or `validator.rs`
- work requires broadening the canonical public example story
- work depends on temporary `body.typescript` injection
- work needs files outside its ownership map

### `WS-C-DOCS`

Owned files:

- `README.md`
- `examples/crosslib-app/README.md`
- `CHANGELOG.md`
- `TODOS.md`

Responsibilities:

- document only the landed M55 claim
- keep docs wording consistent across all owned files
- point to the canonical command that actually passed
- preserve explicit deferred items in `TODOS.md`
- return only changed files, commands with exit codes, and blockers

Stop if:

- docs need to promise broader behavior than the integrated proof supports
- docs need product-code edits to become truthful
- docs require changes outside owned files

### `M55-10` Backend helper resolution + import rendering

Owner: `WS-A-BACKEND`  
Write scope: `spec-core/src/typescript_backend.rs`

Required outcomes:

- helper dep resolution accepts library-qualified helper ids only where validator has already allowed them
- backend resolves shared helpers from loaded units, not a second resolver
- sibling-library relative import path is correct
- bounded closure stays deduplicated
- unrelated loaded units do not leak into output
- recursive wrapper/chain3 helper reuse is covered here only if the CLI lane would otherwise need awkward fixture inflation

Required proof surface:

```bash
cargo test -p spec-core typescript_tree
cargo test -p spec-core typescript_target
```

Exit criteria:

- worker submits diff limited to `typescript_backend.rs`
- backend proof commands and exit codes are captured
- no validator or CLI file edits
- any unresolved ambiguity is called out explicitly

Stop rules:

- if `validator.rs` must change, stop and return to parent
- if backend needs a second import resolver, stop
- if backend broadens root contract beyond helper-slot reuse, stop

### `M55-20` CLI proof wall + canonical example truth + exact support fixtures

Owner: `WS-B-PROOF`  
Write scope: `spec-cli/tests/cli.rs`, canonical example `.unit.spec` files, exact pre-frozen support fixture files only

Required outcomes:

- canonical green path passes through Bun using authored truth in:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `examples/shared-spec/units/money/round.unit.spec`
- target-specific status remains truthful after the TypeScript proof
- recursive wrapper shared-helper reuse is proven
- recursive chain3 shared-helper reuse is proven
- negative wall exists for:
  - unresolved alias
  - missing shared helper
  - wrong helper family
  - missing helper `body.typescript`
  - direct shared wrapper root dep
  - direct shared chain3 root dep
- no test-only injection on canonical example

Required proof surface:

```bash
cargo test -p spec-cli typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json
```

Exit criteria:

- worker submits diff within approved scope
- proof commands and exit codes captured
- canonical example bodies, if changed, are minimal and semantically honest
- support fixture usage is additive and explicitly documented in `acceptance.md`

Stop rules:

- if worker needs to mutate the canonical example only during test execution, stop
- if proof requires editing docs early, stop
- if proof requires generic multi-dependency or generic cross-library TypeScript behavior, stop
- if any needed support-fixture file is not already frozen in `file-ownership.json`, stop and return to parent

### `M55-21` Parent integration gate for backend then proof lane

Owner: Parent  
Write scope: `feat/m40-plus` integration only

Integration order is fixed:

1. Review and integrate `WS-A-BACKEND`
2. Rerun backend/core gates
3. Review and integrate `WS-B-PROOF`
4. Rerun CLI and canonical example gates

Integration rules:

- Integrate one lane at a time.
- Never batch unresolved lane diffs together.
- Review each lane diff against `file-ownership.json` before merge.
- If merge feedback requires edits outside the lane ownership map, bounce the lane back with a narrower brief or explicit ownership-map update first.
- Do not silently “fix it in parent” by broadening scope unless the parent first records that change in run-state artifacts and reopens the task explicitly.

Required post-integration proof surface:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/backend/post-merge-spec-core-typescript-target.txt"
cargo test -p spec-core typescript_tree | tee "$RUN_ROOT/validation/backend/post-merge-spec-core-typescript-tree.txt"

cargo test -p spec-cli typescript | tee "$RUN_ROOT/validation/proof/post-merge-spec-cli-typescript.txt"
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/proof/post-merge-canonical-apply-tax-typescript-test.txt"
cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json \
  | tee "$RUN_ROOT/validation/proof/post-merge-canonical-crosslib-status-typescript.json"
```

Exit criteria:

- both worker lanes integrated by parent only
- canonical proof is green after integration, not just in a worker worktree
- no remaining hidden dependency on docs or other files
- queue states updated to `integrated`

### `M55-30` Docs + backlog sync

Owner: `WS-C-DOCS`  
Write scope: docs files only

Required outcomes:

- `README.md`, `examples/crosslib-app/README.md`, and `CHANGELOG.md` all describe the same narrow M55 claim
- `TODOS.md` still explicitly defers direct shared wrapper roots, direct shared chain3 roots, and generic multi-dependency TypeScript execution
- docs point at the canonical command that just passed

Required doc language anchor:

- “cross-library helper imports in the bounded TypeScript lane”

Exit criteria:

- worker submits docs-only diff
- wording is narrow and consistent
- no accidental product-scope widening

Stop rule:

- If docs need to mention broader cross-library TypeScript support to stay readable, stop and bounce to parent. The code/proof contract is not yet clear enough.

### `M55-31` Parent docs integration gate

Owner: Parent  
Write scope: `feat/m40-plus` integration only

Exit criteria:

- docs integrate cleanly
- parent verifies wording against actual proof captures
- no broader promise lands than what `M55-21` proved

### `M55-40` Final serial proof wall + closeout

Owner: Parent  
Write scope: `$RUN_ROOT/**` and minimal fix-forward only if required

Run serially in `PRIMARY_ROOT`:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/final/spec-core-typescript-target.txt"
cargo test -p spec-core typescript_tree | tee "$RUN_ROOT/validation/final/spec-core-typescript-tree.txt"
cargo test -p spec-cli typescript | tee "$RUN_ROOT/validation/final/spec-cli-typescript.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/final/canonical-apply-tax-typescript-test.txt"

cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json \
  | tee "$RUN_ROOT/validation/final/canonical-crosslib-status-typescript.json"

cp "$PRIMARY_ROOT/examples/crosslib-app/units/pricing/apply_tax.spec.passport.json" \
  "$RUN_ROOT/validation/final/apply_tax.spec.passport.json"
```

Closeout rules:

- Parent runs final proofs only after all lane tasks are already `integrated`.
- Parent does not batch unresolved post-merge fixes from multiple lanes.
- If final proof feedback requires edits outside the last lane’s ownership map, reopen the appropriate task explicitly instead of silently broadening parent integration scope.
- Parent writes:
  - `acceptance-ledger.md`
  - `final-proof-manifest.json`
  - `final-diff-summary.md`

`final-proof-manifest.json` must include:

- final command list
- exit codes
- artifact paths
- final canonical example path
- final primary commit
- whether TypeScript proof is additive in passport/status
- whether test-only canonical mutation was used, which must be `false`

Exit criteria:

- final serial proof wall is green
- canonical example passport contains additive TypeScript proof
- target-specific status is valid for the canonical path
- docs are integrated and truthful
- closeout artifacts are complete and internally consistent

## Tests And Acceptance

### Acceptance matrix

| Acceptance item | Proof source | Owner | Pass condition |
| --- | --- | --- | --- |
| Canonical real-user green path | `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript` | Parent final gate | exits `0`; Bun build/test succeeds |
| Target-specific status is additive and truthful | `cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json` | Parent final gate | canonical unit row is `valid`; status reflects TypeScript proof |
| Additive passport proof exists | copied passport artifact | Parent final gate | `target_proofs.typescript` present for `pricing/apply_tax` |
| Backend import rendering is correct | `cargo test -p spec-core typescript_tree` | `WS-A-BACKEND`, then Parent | green after integration |
| Validator contract is exact | `cargo test -p spec-core typescript_target` | Parent | green with explicit failure classes |
| Recursive wrapper shared-helper reuse is proven | CLI proof wall | `WS-B-PROOF`, then Parent | concrete passing test exists |
| Recursive chain3 shared-helper reuse is proven | CLI proof wall | `WS-B-PROOF`, then Parent | concrete passing test exists |
| Negative wall stays red before Bun | CLI proof wall | `WS-B-PROOF`, then Parent | each out-of-scope case rejects |
| Docs are truthful and narrow | docs diff review + proof captures | `WS-C-DOCS`, then Parent | no broader claim than landed behavior |

### Mandatory negative cases

These must be explicitly covered in the proof wall:

- unresolved library alias
- missing shared helper unit
- wrong helper family
- missing shared helper `body.typescript`
- direct shared wrapper root dep
- direct shared chain3 root dep

### Definition of done

M55 is done only when all of the following are true:

- the parent-integrated `feat/m40-plus` branch passes the final serial proof wall
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec` is green for `--target-language typescript`
- direct shared wrapper and chain3 root deps are still rejected
- recursive wrapper and chain3 shared-helper reuse is proven somewhere concrete
- docs describe only the landed bounded behavior
- `acceptance-ledger.md`, `final-proof-manifest.json`, and `final-diff-summary.md` are complete
- no test-only canonical body injection was used

## Assumptions

- `feat/m40-plus` is the active primary branch for M55 execution.
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec` and `examples/shared-spec/units/money/round.unit.spec` remain the canonical maintained example pair.
- If the canonical example lacks truthful `body.typescript`, adding minimal authored TypeScript to those source specs is allowed and in scope.
- Recursive wrapper and chain3 shared-helper proof may use a minimal additive support fixture if existing maintained examples do not already cover that path.
- `cargo test -p spec-core typescript_target`, `cargo test -p spec-core typescript_tree`, and `cargo test -p spec-cli typescript` remain useful focused command filters. If any filter is too broad or stale, the parent must freeze a narrower exact command list in `contract-freeze.json` before launching workers.
- Existing dirty files outside owned scope may exist; they are tolerated if recorded and not overlapped.

## Parallel Subagent Optimization

The optimal launch pattern is:

1. No workers during kickoff, baseline, contract freeze, or validator work.
2. After `M55-02`, launch exactly two workers:
   - `WS-A-BACKEND`
   - `WS-B-PROOF`
3. Keep them isolated by file ownership:
   - one worker for `spec-core/src/typescript_backend.rs`
   - one worker for `spec-cli/tests/cli.rs` plus canonical example/spec fixtures
4. Do not split `spec-cli/tests/cli.rs` across workers.
5. Do not split `spec-core/src/` across multiple workers.
6. Launch docs only after both earlier lanes are integrated and green.

Maximum safe worker concurrency is `2`.

Why this is the safe maximum:

- before validator freeze, concurrency creates contract churn
- after validator freeze, backend and proof work are separable
- docs before proof creates lying documentation
- a third concurrent code lane would only increase merge risk without shortening the critical path

Parent polling rule:

- poll workers for narrow result summaries only
- do not reopen scope while they are running
- integrate one lane at a time into `feat/m40-plus`
- rerun gates after each integration before proceeding
