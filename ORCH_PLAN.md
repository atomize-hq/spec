# M56 Bounded Direct Cross-Library Wrapper And Chain3 TypeScript Roots Orchestration Runbook

Status: **authoritative execution runbook**  
Supersedes: **the stale M55 `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Plan title: **`M56: Bounded Direct Cross-Library Wrapper and Chain3 TypeScript Roots Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Primary execution branch: **`feat/m40-plus`**  
Primary execution head at rewrite: **`cbbc388`**  
Base branch: **`main`**  
Authority date: **`2026-05-13`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum safe worker concurrency: **2 concurrent worker lanes, and only after validator contract freeze**  
Rewrite intent: **replace the stale M55 helper-import runbook with an execution-ready M56 parent/worker runbook aligned to the current `PLAN.md` contract**  
Last rewritten: **`2026-05-13`**

## Summary

This runbook turns `PLAN.md` into an operator-safe parent/worker execution flow for M56.

M56 is still bounded. It extends the existing Bun-backed TypeScript lane to allow:

1. direct cross-library wrapper root deps for `function.wrapper.pipeline.v1`
2. direct cross-library chain3 root deps for `function.wrapper.pipeline.chain3.v1`
3. mixed local-plus-shared direct dep tuples only when the exact tuple order and family requirements remain frozen

It does not authorize generic cross-library TypeScript execution. It does not authorize arbitrary multi-dependency graphs. It does not authorize molecule TypeScript, seam kinds, nested chain3 closure members, `spec validate --target-language`, or `spec export --target-language`.

The operator truth for M56 is:

1. `PLAN.md` is the only product-scope authority.
2. The maintained public wrapper proof path is `examples/crosslib-app/units/pricing/calculate_total.unit.spec`.
3. The maintained M55 regression path remains `examples/crosslib-app/units/pricing/apply_tax.unit.spec`.
4. Shared reusable leaves for the maintained wrapper proof live at:
   - `examples/shared-spec/units/pricing/apply_discount.unit.spec`
   - `examples/shared-spec/units/pricing/apply_tax.unit.spec`
5. Chain3 direct cross-library proof belongs in focused CLI or integration coverage, not public example docs.
6. The validator contract in `spec-core/src/validator.rs` freezes first and is parent-owned.
7. Only after that freeze may exactly two worker lanes overlap:
   - backend closure and import rendering
   - shared/app example authoring plus CLI proof wall
8. Docs and backlog updates are strictly last.
9. The parent is the only integrator. Workers never merge each other.
10. No second TypeScript-only resolver is allowed. All direct cross-library dep resolution must reuse already-loaded sibling-library truth.

This runbook is an execution system, not a restatement of the plan. It defines lane ownership, durable run-state artifacts, task gates, worker return rules, stop conditions, integration order, and final proof.

## Hard Guards

- `PLAN.md` is the sole scope authority.
- M56 widens only the library location of already-legal direct root deps for:
  - `function.wrapper.pipeline.v1`
  - `function.wrapper.pipeline.chain3.v1`
- Wrapper direct deps remain exactly two, in this fixed order:
  1. `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  2. `function.arithmetic_leaf.monotone_up.v1`
- Chain3 direct deps remain exactly three, in this fixed order:
  1. `function.wrapper.pipeline.v1`
  2. `function.arithmetic_leaf.monotone_up.v1`
  3. `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- Mixed local-plus-shared tuples are legal only when they still satisfy those exact tuple contracts.
- Nested `function.wrapper.pipeline.chain3.v1` closure members remain unsupported.
- The maintained public M56 example is `examples/crosslib-app/units/pricing/calculate_total.unit.spec`. Do not repurpose `apply_tax.unit.spec` into the new wrapper example.
- The maintained M55 regression path remains `examples/crosslib-app/units/pricing/apply_tax.unit.spec`.
- `spec-core/src/validator.rs` is parent-owned for the full run.
- `spec-core/src/typescript_backend.rs` is single-owner in Lane B.
- `spec-cli/tests/cli.rs` is single-owner in Lane C.
- Workers may edit source `.unit.spec` files only. They must not author `.spec.passport.json`, `.test.evidence.json`, or generated output as source truth.
- Docs stay untouched until the integrated proof wall is green.
- No new crates, services, commands, runtime channels, schema surfaces, or generic resolver stacks.
- Bun-only runtime and atom-only proof remain intact.
- Maximum safe worker concurrency is 2, and only after validator freeze lands on `feat/m40-plus`.
- Parent remains sole integrator onto `feat/m40-plus`.

Preserved negative wall:

- unresolved library alias
- missing imported unit
- wrong dep family
- wrong dep order
- wrong dep count
- missing imported `body.typescript`
- nested chain3 closure still rejected

Stop and re-scope immediately if any of these become true:

1. cross-library root-dep support requires a generic graph executor instead of the current bounded closure collector
2. backend support requires a second TypeScript-only resolver instead of the loaded-unit truth already in memory
3. chain3 proof requires nested chain3 closure-member support
4. passport, export, status, or plan schemas need new fields
5. the only truthful wrapper proof requires test-only mutation or temporary `body.typescript` injection
6. docs would need to claim broad “cross-library TypeScript support” instead of the exact bounded M56 claim

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m56
RUN_ROOT=$PRIMARY_ROOT/.runs/m56_bounded_direct_cross_library_wrapper_chain3_roots
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Primary authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, contract freeze, validator, integration, final proof wall |
| `WS-B-BACKEND` | `$WT_ROOT/ws-b-backend` | `codex/m56-backend-cross-library-roots` | Worker | backend closure collection and import rendering only |
| `WS-C-PROOF` | `$WT_ROOT/ws-c-proof` | `codex/m56-cross-library-proof-wall` | Worker | shared leaves, app wrapper example, CLI proof wall |
| `WS-D-DOCS` | `$WT_ROOT/ws-d-docs` | `codex/m56-docs-last` | Worker | docs and backlog truth after proof is green |

### Worktree creation rules

- Do not create any worker worktree before `M56-02` validator contract freeze is integrated.
- Create `WS-B-BACKEND` and `WS-C-PROOF` from the same frozen `feat/m40-plus` head immediately after `M56-02`.
- Create `WS-D-DOCS` only after `M56-21` is green.
- The current tree is expected clean at kickoff. Record that fact in `baseline.json`; do not assume it forever.
- If the primary tree becomes dirty later, record it. Do not stash or clean by default.

### Recommended creation commands

```bash
mkdir -p "$WT_ROOT"

git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-b-backend" -b codex/m56-backend-cross-library-roots feat/m40-plus
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-c-proof" -b codex/m56-cross-library-proof-wall feat/m40-plus
# create ws-d-docs only after integrated proof wall is green
git -C "$PRIMARY_ROOT" worktree add "$WT_ROOT/ws-d-docs" -b codex/m56-docs-last feat/m40-plus
```

## Durable Orchestration State

All durable session state lives under:

```bash
$PRIMARY_ROOT/.runs/m56_bounded_direct_cross_library_wrapper_chain3_roots
```

This directory is orchestration state, not product truth.

### Required run-state artifacts

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, clean or dirty state, baseline command expectations | Parent |
| `contract-freeze.json` | frozen M56 contract, negative wall, stop rules, unlock rules | Parent |
| `worktrees.json` | exact worktree paths, branches, lane states | Parent |
| `file-ownership.json` | exact owned file map per task and lane | Parent |
| `tasks.json` | durable task definitions, dependencies, states | Parent |
| `queue.json` | runnable queue and current state machine | Parent |
| `session-log.md` | chronological run log with launches, submissions, integrations, and stops | Parent |
| `acceptance-ledger.md` | final signoff checklist and artifact references | Parent |
| `final-proof-manifest.json` | final proof commands, exit codes, and artifact paths | Parent |
| `final-diff-summary.md` | parent-authored summary of landed diffs by lane | Parent |
| `validation/kickoff/` | branch, head, git-status, authority snapshots | Parent |
| `validation/baseline/` | pre-change proof captures, including current failure or absence of M56 direct cross-library root support | Parent |
| `validation/validator/` | validator-focused proof captures | Parent |
| `validation/backend/` | backend-focused proof captures and tree-render evidence | Parent |
| `validation/proof/` | CLI/example proof captures | Parent |
| `validation/docs/` | wording review captures | Parent |
| `validation/final/` | final serial proof wall and closeout evidence | Parent |
| `handoffs/` | worker briefs and worker result summaries | Parent |

### Required `baseline.json` contents

`baseline.json` must include at least:

- `milestone`: `M56`
- `authority_plan_path`
- `authority_plan_title`
- `authority_plan_validated_commit`
- `primary_branch`
- `primary_head_commit`
- `dirty_tree_summary`
- `dirty_tree_files`
- `public_wrapper_example_path`
- `m55_regression_example_path`
- `baseline_commands`
- `baseline_expected_truth`
- `baseline_artifact_paths`
- `stop_rules_version`

### Required `contract-freeze.json` contents

`contract-freeze.json` must include at least:

- `milestone`: `M56`
- `authority_plan_path`
- `authority_plan_head_commit`
- `frozen_at_primary_commit`
- `primary_branch`
- `public_wrapper_example_path`
- `m55_regression_example_path`
- `shared_leaf_paths`
- `exact_scope_claim`
- `locked_decisions`
- `negative_wall`
- `allowed_worker_lanes`
- `file_ownership_version`
- `phase_commands`
- `integration_order`
- `merge_conflict_policy`
- `worker_return_contract`
- `exact_stop_rules`
- `chain3_fixture_policy`

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
- Worker output is incomplete until `commands.txt`, `changed_files.txt`, and `acceptance.md` are populated.
- Chat history is not the durable ledger.
- A task is not done when a worker says “finished”; it is done only after parent integration and gate rerun.

## Context-Control Rules

- The parent owns `PLAN.md`, this runbook, and all `.runs/m56_*` state.
- Workers get only the minimum prompt necessary: goal, scope, owned files, stop rules, acceptance, and exact commands.
- Do not forward one worker’s raw transcript into another worker.
- No worker may expand its write scope mid-flight.
- `validator.rs` is the contract seam. No worker touches it.
- `typescript_backend.rs` is single-owner in Lane B even though it shares `spec-core/src/` with the validator seam.
- `spec-cli/tests/cli.rs` is high-conflict and stays single-owner in Lane C.
- `examples/shared-spec/units/pricing/` and `examples/crosslib-app/units/pricing/` stay in Lane C so authored example truth and CLI proof truth move together.
- Docs worker never edits product code or spec fixtures.
- When validation feedback matters, store machine-readable or targeted command output under `validation/*`; do not rely on chat paraphrases.
- Workers edit source specs, not generated artifacts. Parent enforces the spec workflow:
  - validate from source
  - regenerate through `spec`
  - refresh proof through `spec test`

## File Ownership Map

### Parent-owned throughout

- `spec-core/src/validator.rs`
- all files under `$RUN_ROOT/`
- final integration commits on `feat/m40-plus`

### `WS-B-BACKEND` owned files

- `spec-core/src/typescript_backend.rs`

### `WS-C-PROOF` owned files

- `examples/shared-spec/units/pricing/apply_discount.unit.spec`
- `examples/shared-spec/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/pricing/calculate_total.unit.spec`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `spec-cli/tests/cli.rs`

Rules for proof-lane scope:

- Lane C owns the maintained M56 wrapper example, the maintained M55 regression example, and the focused CLI proof wall together.
- Lane C may add only exact focused test fixtures or helpers that are recorded up front in `file-ownership.json`.
- Lane C must not broaden public docs or create generated artifacts as source truth.

### `WS-D-DOCS` owned files

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

- The parent reviews narrow diffs only, scoped to the lane ownership map.
- The parent reviews command outcomes and blockers, not full worker transcripts.
- The parent integrates one worker lane at a time.
- The parent reruns relevant gates after each integration before touching the next lane.
- After a lane is integrated or rejected, the parent closes that worker. Workers do not remain open as ad hoc follow-up agents.
- If merge feedback requires edits outside the lane ownership map, bounce the lane back with a narrower brief or explicit ownership-map update first.

## Workstream Plan

| ID | Task | Owner | Write scope | Depends on | Unlock condition | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- |
| `M56-00` | Kickoff + baseline capture | Parent | `$RUN_ROOT/**` | none | repo available on `feat/m40-plus` | authority snapshots, clean-tree capture, baseline proofs stored |
| `M56-01` | Contract freeze + ownership map | Parent | `$RUN_ROOT/**` | `M56-00` | baseline recorded | `contract-freeze.json`, `file-ownership.json`, `tasks.json`, `queue.json`, `worktrees.json` frozen |
| `M56-02` | Lane A validator contract freeze | Parent | `spec-core/src/validator.rs` | `M56-01` | contract frozen | validator change integrated on `feat/m40-plus` with focused proof green |
| `M56-10` | Lane B backend closure + import rendering | `WS-B-BACKEND` | `spec-core/src/typescript_backend.rs` | `M56-02` | validator contract integrated | backend lane submitted with tests and no scope drift |
| `M56-20` | Lane C shared leaves + wrapper example + CLI proof wall | `WS-C-PROOF` | proof-lane owned files only | `M56-02` | validator contract integrated | worker submits green proof wall or explicit blocker |
| `M56-21` | Parent integration gate for Lane B then Lane C | Parent | integration on `feat/m40-plus` only | `M56-10`, `M56-20` | both worker lanes submitted | both diffs integrated by parent and post-merge proof reruns green |
| `M56-30` | Lane D docs + backlog sync | `WS-D-DOCS` | docs files only | `M56-21` | integrated proof wall green and wording frozen | docs submitted with exact narrow M56 language |
| `M56-31` | Parent docs integration gate | Parent | integration on `feat/m40-plus` only | `M56-30` | docs lane submitted | docs integrated and wording verified against proof |
| `M56-40` | Final serial proof wall + closeout | Parent | `$RUN_ROOT/**` and minimal fix-forward if required | `M56-31` | all prior tasks integrated | final commands pass, manifests written, closeout recorded |

### `M56-00` Kickoff + baseline capture

Owner: Parent  
Write scope: `$RUN_ROOT/**`

Required captures:

```bash
mkdir -p "$RUN_ROOT"/{validation/{kickoff,baseline,validator,backend,proof,docs,final},tasks,handoffs}

git -C "$PRIMARY_ROOT" branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git -C "$PRIMARY_ROOT" rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git -C "$PRIMARY_ROOT" status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.rewritten.md"
```

Baseline proof captures must record current pre-M56 behavior, not guessed behavior:

```bash
cargo test -p spec-core validator::tests typescript_wrapper -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/spec-core-validator-wrapper.txt"

cargo test -p spec-core validator::tests typescript_chain3 -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/spec-core-validator-chain3.txt"

cargo test -p spec-core typescript_backend::tests cross_library -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/spec-core-backend-cross-library.txt"

cargo test -p spec-cli --test cli typescript_example_apply_tax_single_file_test_succeeds -- --nocapture \
  | tee "$RUN_ROOT/validation/baseline/spec-cli-apply-tax.txt"
```

`baseline.json` must record:

- current branch `feat/m40-plus`
- current head `cbbc388` at kickoff, if unchanged
- clean working tree expectation and actual result
- maintained M55 regression path
- intended maintained M56 wrapper path
- baseline commands
- expected baseline truth:
  - M55 regression path still green or recoverable from current branch truth
  - direct cross-library wrapper and chain3 root support not yet fully green
  - current negative wall may still contain M55-era direct-root rejection wording

Stop rule:

- If baseline already shows M56 fully green, stop and reconcile `PLAN.md` before launching work.

### `M56-01` Contract freeze + ownership map

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

- authority commit references
- current branch
- current primary commit
- clean-tree summary reference
- maintained public wrapper example path
- maintained M55 regression path
- exact M56 scope claim
- locked dep tuples
- negative wall list:
  - unresolved library alias
  - missing imported unit
  - wrong dep family
  - wrong dep order
  - wrong dep count
  - missing imported `body.typescript`
  - nested chain3 closure still rejected
- allowed worker lanes:
  - `WS-B-BACKEND`
  - `WS-C-PROOF`
  - `WS-D-DOCS`
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
- chain3 fixture policy:
  - focused CLI or integration coverage is mandatory
  - no public README example is required for chain3
  - no standalone `cargo run` chain3 fixture command is frozen unless Lane C introduces a stable checked-in fixture path

Exit criteria:

- all artifacts above exist
- all tasks are queued with explicit dependencies
- ownership is exact and finite
- worker prompts can be generated without rereading the full repo

### `M56-02` Lane A validator contract freeze

Owner: Parent  
Write scope: `spec-core/src/validator.rs`

Required outcomes:

- direct shared wrapper deps validate when family, count, order, and `body.typescript` remain exact
- direct shared chain3 deps validate under the same exactness rules
- mixed local-plus-shared tuples validate only when the frozen tuple contract remains satisfied
- unresolved alias rejects clearly
- missing imported unit rejects clearly
- wrong dep family rejects clearly
- wrong dep order rejects clearly
- wrong dep count rejects clearly
- missing imported `body.typescript` rejects clearly
- same-tree wrapper and chain3 positives remain green
- nested chain3 closure ban remains unchanged
- maintained M55 regression path still validates

Required proof surface:

```bash
cargo test -p spec-core validator::tests typescript_wrapper -- --nocapture \
  | tee "$RUN_ROOT/validation/validator/spec-core-validator-wrapper.txt"

cargo test -p spec-core validator::tests typescript_chain3 -- --nocapture \
  | tee "$RUN_ROOT/validation/validator/spec-core-validator-chain3.txt"
```

Exit criteria:

- validator change integrated in `feat/m40-plus`
- validator proof capture green
- `contract-freeze.json` updated if user-facing test strings changed
- launch inputs for Lane B and Lane C recorded

Stop rule:

- If validator work forces edits outside `validator.rs`, stop and decide in parent before any worker starts.

## Concrete Parent-Agent Responsibilities

The parent owns:

- translating `PLAN.md` into the frozen orchestration contract
- creating `RUN_ROOT`, run-state files, task sentinels, and validation directories
- creating worktrees and branches for each worker lane
- generating worker briefs and handoff packets under `$RUN_ROOT/handoffs/`
- freezing exact commands and exact ownership before worker launch
- running and recording kickoff and baseline commands
- executing the entire validator phase in `spec-core/src/validator.rs`
- launching workers only after validator freeze is integrated
- integration ordering:
  - backend first
  - proof lane second
  - docs last
- integrating one lane at a time
- rerunning relevant gates after each merge
- recording every run-state transition in `queue.json`, `tasks.json`, and `session-log.md`
- enforcing ownership boundaries and stop rules
- writing `acceptance-ledger.md`, `final-proof-manifest.json`, and `final-diff-summary.md`
- final acceptance judgment

The parent must not:

- launch workers before `M56-02`
- let workers self-integrate
- batch unresolved lane diffs together
- silently broaden scope during merge feedback
- let docs move ahead of proof

## Concrete Worker-Lane Responsibilities

### `WS-B-BACKEND`

Owned files:

- `spec-core/src/typescript_backend.rs`

Responsibilities:

- replace local-only root-dep parsing for wrapper and chain3 roots with library-aware direct-dep resolution
- reuse loaded-unit truth instead of introducing a second resolver
- keep closure inclusion bounded to the root, its resolved direct deps, and already-supported closure members below those deps
- render stable sibling-library relative imports
- keep shared modules deduplicated
- keep unrelated loaded units out of the generated tree
- preserve helper-import behavior already shipped in M55

Required proof surface:

```bash
cargo test -p spec-core typescript_backend::tests cross_library -- --nocapture
```

Stop if:

- work requires edits to `spec-core/src/validator.rs`
- work requires edits to `spec-cli/tests/cli.rs`
- work implies generic cross-library TypeScript execution
- work requires a second resolver stack
- work needs files outside ownership

### `WS-C-PROOF`

Owned files:

- `examples/shared-spec/units/pricing/apply_discount.unit.spec`
- `examples/shared-spec/units/pricing/apply_tax.unit.spec`
- `examples/crosslib-app/units/pricing/calculate_total.unit.spec`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `spec-cli/tests/cli.rs`

Responsibilities:

- author the shared reusable pricing leaves
- author the maintained M56 wrapper example at `pricing/calculate_total`
- preserve the maintained M55 regression example at `pricing/apply_tax`
- add focused CLI or integration proof for direct cross-library chain3 roots
- add or refresh the negative wall in CLI coverage for:
  - unresolved alias
  - missing imported unit
  - wrong dep family
  - wrong dep order
  - wrong dep count
  - missing imported `body.typescript`
- keep chain3 proof in focused CLI coverage, not public example docs
- avoid generated artifacts as source truth

Required proof surface:

```bash
cargo test -p spec-cli --test cli typescript_example_apply_tax_single_file_test_succeeds -- --nocapture
cargo test -p spec-cli --test cli typescript_cross_library_wrapper_example_executes_with_bun -- --nocapture
cargo test -p spec-cli --test cli typescript_cross_library_chain3_root_executes_with_bun -- --nocapture
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
```

Chain3 fixture command policy:

- The frozen M56 wall requires the named CLI test `typescript_cross_library_chain3_root_executes_with_bun`.
- No separate `cargo run` chain3 fixture command is required at kickoff because chain3 proof is intentionally owned by focused CLI or integration coverage, not public example docs.
- If Lane C introduces a stable checked-in chain3 fixture path, the parent must append the exact `cargo run -p spec-cli -- test <path> --target-language typescript` command to `contract-freeze.json`, this runbook, and final closeout notes before docs begin.

Stop if:

- proof requires edits to `validator.rs` or `typescript_backend.rs`
- proof requires generic multi-dependency or generic cross-library TypeScript behavior
- proof depends on temporary `body.typescript` injection
- proof requires docs edits before Lane D
- proof needs files outside ownership

### `WS-D-DOCS`

Owned files:

- `README.md`
- `examples/crosslib-app/README.md`
- `CHANGELOG.md`
- `TODOS.md`

Responsibilities:

- document only the landed M56 claim
- keep wording consistent across all owned files
- keep broader bans explicit:
  - generic multi-dependency execution
  - molecule TypeScript
  - seam kinds
  - nested chain3 closure support
  - broader cross-library TypeScript claims
- remove or rewrite the M55-era TODO only after proof is green

Stop if:

- docs need to promise broader behavior than integrated proof supports
- docs need product-code edits to become truthful
- docs need files outside ownership

### `M56-10` Lane B backend closure + import rendering

Owner: `WS-B-BACKEND`  
Write scope: `spec-core/src/typescript_backend.rs`

Required outcomes:

- wrapper and chain3 roots resolve direct deps as local or qualified sibling-library deps
- generated tree for `pricing/calculate_total` contains the root plus the two shared pricing leaves exactly once
- generated tree for focused chain3 cross-library proof contains only direct deps plus already-supported bounded closures
- unrelated loaded units are excluded
- nested chain3 closure-member rejection remains intact

Exit criteria:

- worker submits diff limited to `typescript_backend.rs`
- backend proof command and exit code captured
- no validator or CLI file edits
- unresolved ambiguity called out explicitly

### `M56-20` Lane C shared leaves + wrapper example + CLI proof wall

Owner: `WS-C-PROOF`  
Write scope: proof-lane owned files only

Required outcomes:

- shared reusable pricing leaves exist at:
  - `examples/shared-spec/units/pricing/apply_discount.unit.spec`
  - `examples/shared-spec/units/pricing/apply_tax.unit.spec`
- maintained wrapper proof exists at:
  - `examples/crosslib-app/units/pricing/calculate_total.unit.spec`
- maintained M55 regression path remains:
  - `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- direct cross-library wrapper root passes in CLI and direct command proof
- direct cross-library chain3 root passes in focused CLI coverage
- negative wall is present for all preserved failure modes

Exit criteria:

- worker submits diff within approved scope
- proof commands and exit codes captured
- source specs are semantically honest and minimal
- chain3 proof stays out of public README example surfaces

### `M56-21` Parent integration gate for Lane B then Lane C

Owner: Parent  
Write scope: integration on `feat/m40-plus` only

Integration order is fixed:

1. review and integrate `WS-B-BACKEND`
2. rerun backend gates
3. review and integrate `WS-C-PROOF`
4. rerun integrated proof wall

Required post-integration proof surface:

```bash
cargo test -p spec-core validator::tests typescript_wrapper -- --nocapture \
  | tee "$RUN_ROOT/validation/backend/post-merge-validator-wrapper.txt"

cargo test -p spec-core validator::tests typescript_chain3 -- --nocapture \
  | tee "$RUN_ROOT/validation/backend/post-merge-validator-chain3.txt"

cargo test -p spec-core typescript_backend::tests cross_library -- --nocapture \
  | tee "$RUN_ROOT/validation/backend/post-merge-cross-library.txt"

cargo test -p spec-cli --test cli typescript_example_apply_tax_single_file_test_succeeds -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-apply-tax.txt"

cargo test -p spec-cli --test cli typescript_cross_library_wrapper_example_executes_with_bun -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-wrapper-example.txt"

cargo test -p spec-cli --test cli typescript_cross_library_chain3_root_executes_with_bun -- --nocapture \
  | tee "$RUN_ROOT/validation/proof/post-merge-chain3-root.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/proof/post-merge-apply-tax-command.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/proof/post-merge-calculate-total-command.txt"
```

Exit criteria:

- both worker lanes integrated by parent only
- integrated proof wall is green in the primary tree, not just worker worktrees
- no hidden dependency on docs or extra files
- queue states updated to `integrated`

### `M56-30` Lane D docs + backlog sync

Owner: `WS-D-DOCS`  
Write scope: docs files only

Required outcomes:

- `README.md`, `examples/crosslib-app/README.md`, `CHANGELOG.md`, and `TODOS.md` all tell the same narrow M56 story
- docs name the maintained wrapper example truthfully
- docs preserve the maintained M55 regression path truthfully
- docs keep chain3 proof positioned as focused CLI or integration coverage, not public example docs

Required language anchor:

- “bounded direct cross-library wrapper and chain3 roots in the Bun-backed TypeScript lane”

Exit criteria:

- worker submits docs-only diff
- wording is narrow and consistent
- no accidental product-scope widening

### `M56-31` Parent docs integration gate

Owner: Parent  
Write scope: integration on `feat/m40-plus` only

Exit criteria:

- docs integrate cleanly
- parent verifies wording against actual proof captures
- no broader promise lands than what `M56-21` proved

### `M56-40` Final serial proof wall + closeout

Owner: Parent  
Write scope: `$RUN_ROOT/**` and minimal fix-forward only if required

Run serially in `PRIMARY_ROOT`:

```bash
cargo test -p spec-core validator::tests typescript_wrapper -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-core-validator-wrapper.txt"

cargo test -p spec-core validator::tests typescript_chain3 -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-core-validator-chain3.txt"

cargo test -p spec-core typescript_backend::tests cross_library -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-core-backend-cross-library.txt"

cargo test -p spec-cli --test cli typescript_example_apply_tax_single_file_test_succeeds -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-cli-apply-tax.txt"

cargo test -p spec-cli --test cli typescript_cross_library_wrapper_example_executes_with_bun -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-cli-wrapper-example.txt"

cargo test -p spec-cli --test cli typescript_cross_library_chain3_root_executes_with_bun -- --nocapture \
  | tee "$RUN_ROOT/validation/final/spec-cli-chain3-root.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/final/apply-tax-command.txt"

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript \
  | tee "$RUN_ROOT/validation/final/calculate-total-command.txt"
```

If a stable checked-in chain3 fixture path was introduced during Lane C, append the exact command here before closeout:

```bash
cargo run -p spec-cli -- test <stable-chain3-fixture-path>.unit.spec --target-language typescript
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
- final public wrapper example path
- final M55 regression example path
- final primary commit
- whether TypeScript proof is additive in passport or status surfaces
- whether test-only mutation was used, which must be `false`

Exit criteria:

- final serial proof wall is green
- maintained wrapper example is green
- maintained M55 regression path is green
- negative wall is still truthful
- docs are integrated and truthful

## Tests And Acceptance

Required command wall before docs land:

```bash
cargo test -p spec-core validator::tests typescript_wrapper -- --nocapture
cargo test -p spec-core validator::tests typescript_chain3 -- --nocapture
cargo test -p spec-core typescript_backend::tests cross_library -- --nocapture
cargo test -p spec-cli --test cli typescript_example_apply_tax_single_file_test_succeeds -- --nocapture
cargo test -p spec-cli --test cli typescript_cross_library_wrapper_example_executes_with_bun -- --nocapture
cargo test -p spec-cli --test cli typescript_cross_library_chain3_root_executes_with_bun -- --nocapture
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
```

Acceptance checklist:

- direct cross-library wrapper roots pass in the bounded TypeScript lane
- direct cross-library chain3 roots pass in the bounded TypeScript lane
- exact wrapper and chain3 dep tuples stay enforced even when slots are `shared::...`
- wrong dep order, wrong dep count, wrong family, unresolved alias, missing imported unit, and missing imported `body.typescript` all fail before Bun
- nested chain3 closure members still reject
- `examples/crosslib-app/units/pricing/calculate_total.unit.spec` is the maintained M56 wrapper proof path
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec` still passes as the maintained M55 regression path
- same-tree wrapper roots still pass
- same-tree chain3 roots still pass
- generated tree stays bounded and excludes unrelated loaded units
- docs and backlog all tell the same M56 story

## Assumptions

- `PLAN.md` remains the product authority for M56 while this runbook executes.
- The primary branch remains `feat/m40-plus` unless the parent explicitly records a branch change in `baseline.json` and `contract-freeze.json`.
- The current tree is clean at kickoff.
- The maintained wrapper proof uses checked-in example units.
- The direct cross-library chain3 proof continues to live in focused CLI or integration coverage rather than public example docs.
- Existing sibling-library loading and `[libraries]` config remain reusable without new schema or runtime work.
- The repo’s spec workflow remains source-first: edit `.unit.spec`, then validate, build if needed, and refresh proof through `spec test`.

## Parallel Subagent Optimization

This milestone benefits from parallelism only after the validator contract is frozen.

Why concurrency is capped at 2:

1. `validator.rs` is the blast-radius seam. Starting workers before its contract lands creates churn, invalidates tests, and guarantees rework.
2. After validator freeze, the natural split is exactly two independent write domains:
   - Lane B: `spec-core/src/typescript_backend.rs`
   - Lane C: shared leaves, app wrapper example, `spec-cli/tests/cli.rs`
3. A third concurrent implementation worker would either collide with `spec-cli/tests/cli.rs`, collide with the example spec files, or pull docs ahead of proof. None of those are acceptable.
4. Docs have near-zero implementation leverage before proof is green and high risk of getting ahead of reality, so they stay as a separate last lane.

Safe launch order:

1. `M56-00` kickoff and baseline
2. `M56-01` contract freeze
3. `M56-02` parent-owned validator contract freeze
4. launch `WS-B-BACKEND` and `WS-C-PROOF` in parallel from the same frozen head
5. integrate Lane B
6. integrate Lane C
7. rerun integrated proof wall
8. launch `WS-D-DOCS`
9. integrate docs
10. run final serial proof wall and closeout

Conflict flags:

- `validator.rs` must land first and remains single-owner throughout.
- `typescript_backend.rs` remains single-owner in Lane B.
- `spec-cli/tests/cli.rs` remains single-owner in Lane C.
- `examples/shared-spec/units/pricing/` and `examples/crosslib-app/units/pricing/` stay in the same proof lane so example truth and CLI truth do not drift.
- Docs stay last. If docs move earlier, the runbook has already failed.
