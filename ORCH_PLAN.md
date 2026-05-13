# M54 Bounded Same-Tree Chain3 TypeScript Orchestration Runbook

Status: **authoritative execution runbook**  
Supersedes: **the stale M53 `ORCH_PLAN.md`**  
Authority source: **`PLAN.md`**  
Plan title: **`M54: Bounded Same-Tree Chain3 TypeScript Execution Plan`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Base branch: **`main`**  
Primary execution branch: **`feat/m40-plus`**  
Authority date: **`2026-05-13`**  
Worker model: **GPT-5.4 with `reasoning_effort=high`**  
Maximum safe parallelism: **2 worker lanes at once, and only after validator contract freeze**  
Last rewritten: **`2026-05-13`**

## Summary

Execute M54 from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus`.

`PLAN.md` is the only scope authority. This runbook converts that plan into an operator-safe session flow.

M54 is a bounded execution milestone. It does one thing: extend the existing Bun-backed TypeScript lane to execute exactly `function.wrapper.pipeline.chain3.v1` when the root and all required members live in the same loaded tree. It does not authorize generic multi-dependency TypeScript execution, generic graph execution, cross-library resolution, molecule TypeScript, seam-kind expansion, passport/export schema changes, or new infrastructure.

The orchestration truth is:

1. the validator contract is the critical path and goes first
2. the validator phase is parent-owned in this runbook and stays serialized
3. backend closure work and aligned fixture TypeScript bodies may run in parallel only after the validator contract is frozen in the primary branch
4. the CLI proof wall waits for validator truth, backend truth, and aligned fixture truth
5. docs and backlog sync run last
6. final integration, final proof commands, and signoff are parent-owned

If the implementation starts drifting toward "support any three-dependency TypeScript root" or "resolve helpers across libraries," stop and re-scope. That is not M54.

## Hard Guards

- `PLAN.md` is the sole scope authority.
- The only new TypeScript execution family admitted by M54 is `function.wrapper.pipeline.chain3.v1`.
- Support is keyed by semantic family compatibility key, not by direct dependency count.
- The validator contract is the hinge. No parallel code lanes begin until it is frozen in the primary branch.
- No generic multi-dependency TypeScript support.
- No generic graph executor.
- No cross-library imports or cross-library resolution.
- Molecule TypeScript stays rejected before Bun.
- No passport schema changes.
- No export schema changes.
- No new crates, services, commands, or runtime dependencies unless `PLAN.md` explicitly authorizes them. It does not.
- `spec-core/src/validator.rs` is parent-owned during the validator phase and must not be edited concurrently elsewhere.
- `spec-core/src/typescript_backend.rs` and the aligned chain3 fixture `.unit.spec` files are the only safe parallel surfaces after freeze.
- `spec-cli/tests/cli.rs` waits until validator, backend, and aligned fixture truth are all integrated.
- `README.md`, `CHANGELOG.md`, and `TODOS.md` wait until code paths and proof surfaces are stable.
- Final merge, final proof commands, and acceptance judgment are parent-owned.
- Do not revert, clean, or rewrite other people’s edits.

Stop immediately and re-scope if any of these become true:

1. chain3 support requires a generic graph executor
2. passing the aligned path requires cross-library dependency resolution
3. TypeScript molecule execution becomes necessary
4. passport or export schema changes appear necessary
5. the aligned fixture cannot be classified as `function.wrapper.pipeline.chain3.v1`
6. the work starts broadening into generic multi-root or multi-library TypeScript support

## Concrete Worktree And Branch Layout

Use this exact topology.

```bash
PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec
WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m54
RUN_ROOT=$PRIMARY_ROOT/.runs/m54_bounded_same_tree_chain3
```

### Branch inventory

| Lane | Path | Branch | Owner | Purpose |
| --- | --- | --- | --- | --- |
| Authority + integration | `PRIMARY_ROOT` | `feat/m40-plus` | Parent | kickoff, validator phase, integration, final proofs |
| `WS-BACKEND` | `$WT_ROOT/ws-backend` | `codex/m54-ws-backend` | Worker | bounded TypeScript backend closure only |
| `WS-FIXTURE` | `$WT_ROOT/ws-fixture` | `codex/m54-ws-fixture` | Worker | aligned chain3 fixture TypeScript bodies only |
| `WS-CLI` | `$WT_ROOT/ws-cli` | `codex/m54-ws-cli` | Worker | CLI proof wall after backend + fixture integration |
| `WS-DOCS` | `$WT_ROOT/ws-docs` | `codex/m54-ws-docs` | Worker | docs and backlog sync after proof surfaces stabilize |

Rules:

- There is no separate validator worktree in the default topology. The parent executes the validator contract in `PRIMARY_ROOT`.
- `WS-BACKEND` and `WS-FIXTURE` are the only lanes allowed to overlap.
- `WS-CLI` and `WS-DOCS` are serialized behind earlier gates.
- If the parent chooses to delegate the validator anyway, that worker must be the only active code lane and must finish before any other worker starts.

## Orchestration State

All durable orchestration state lives under:

```bash
RUN_ROOT=$PRIMARY_ROOT/.runs/m54_bounded_same_tree_chain3
```

`RUN_ROOT` and `.runs/m54_*` are execution state, not product source. They are the operator ledger for the session and must not be treated as authored repo truth.

### Run-state inventory

| Path | Purpose | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff branch, commit, dirty-tree, and baseline-proof metadata | Parent |
| `contract-freeze.json` | frozen M54 execution contract, boundaries, and stop rules | Parent |
| `worktrees.json` | active worktree and branch inventory for every lane | Parent |
| `file-ownership.json` | exact owned-file map per task and lane | Parent |
| `tasks.json` | durable task ledger with task definitions and ownership | Parent |
| `queue.json` | runnable-state queue and dependency tracking | Parent |
| `session-log.md` | chronological operator log of launches, integrations, stops, and resumptions | Parent |
| `acceptance-ledger.md` | final acceptance checklist and proof signoff ledger | Parent |
| `validation/kickoff/*` | branch, status, and authority snapshots | Parent |
| `validation/baseline/*` | baseline proof captures before product-code edits | Parent |
| `validation/validator/*` | validator task proof captures | Parent |
| `validation/backend/*` | backend lane proof captures and integration evidence | Parent |
| `validation/fixture/*` | fixture lane proof captures and integration evidence | Parent |
| `validation/cli/*` | CLI lane proof captures and integration evidence | Parent |
| `validation/docs/*` | docs review and wording-validation captures | Parent |
| `validation/final/*` | final proof wall captures and closeout evidence | Parent |

Rules:

- `tasks.json` and `queue.json` are the source of truth for orchestration progress.
- Per-task sentinel directories support the queue; they do not replace it.
- Product truth lives in repo source files and validated proofs, not in run-state notes.

## Per-Task Sentinel Convention

Every task and worker lane gets a dedicated sentinel directory:

```bash
$RUN_ROOT/tasks/<TASK_ID>/
```

Required sentinel files:

- `status.json`
- `owner.txt`
- `branch.txt`
- `commands.txt`
- `changed_files.txt`
- `acceptance.md`
- `blocker.md`

Sentinel file meanings:

- `status.json`: machine-readable task state, timestamps, and current disposition
- `owner.txt`: parent or worker owner label for the task
- `branch.txt`: the branch used for execution or integration
- `commands.txt`: exact commands run plus observed exit codes
- `changed_files.txt`: newline-delimited touched-file list, even when empty
- `acceptance.md`: concise statement of what was proven and what remains open
- `blocker.md`: concrete blockers or unresolved assumptions; empty when not blocked

Rules:

- The parent updates task state in `queue.json` and mirrors key task details in the sentinel directory.
- Workers return narrow summaries that populate `commands.txt`, `changed_files.txt`, and `blocker.md`.
- Chat history is not the run ledger.
- A task is not complete until both its queue state and sentinel acceptance are updated.

## Context-Control Rules

- The parent owns `PLAN.md` and `ORCH_PLAN.md`. Workers do not edit either file.
- The parent owns orchestration state under `RUN_ROOT`.
- One task per worker. No worker prompt may authorize opportunistic side work.
- Each lane has a frozen file-ownership map. If a worker needs a file outside its map, it stops and hands control back to the parent.
- The primary branch is the only integration branch. Workers submit patches or commits; the parent integrates them.
- The parent records all phase transitions in run-state files. Chat history is not the run ledger.
- No worker may silently rebase away conflicts on shared files. Any overlap outside the declared ownership map is a blocker, not an invitation to improvise.
- The parent must freeze exact rejection expectations before CLI work begins so the CLI lane does not invent new contract language.
- The parent must keep negative TypeScript boundaries truthful:
  - cross-library imports rejected
  - molecule TypeScript rejected
  - seam-kind TypeScript rejected
  - generic four-dependency and other out-of-family roots rejected
- The docs lane can describe only behavior already proven in the integrated branch.

## Workstream Plan

| ID | Lane | Owner | Write scope | Depends on | Exit condition |
| --- | --- | --- | --- | --- | --- |
| `M54-00` | Kickoff + baseline | Parent | run-state only | none | branch, dirty-tree, authority snapshots, baseline proofs captured |
| `M54-01` | Contract freeze | Parent | run-state only | `M54-00` | file ownership, task graph, stop rules, and gates frozen |
| `M54-02` | Validator contract | Parent | `spec-core/src/validator.rs` | `M54-01` | exact chain3 validator contract integrated in `feat/m40-plus` with focused proof |
| `M54-10` | `WS-BACKEND` | Worker | `spec-core/src/typescript_backend.rs` | `M54-02` | exact chain3 same-tree closure emission lands cleanly |
| `M54-11` | `WS-FIXTURE` | Worker | aligned chain3 fixture `.unit.spec` files | `M54-02` | aligned chain3 fixture truth has maintained `body.typescript` coverage |
| `M54-12` | Backend + fixture integration gate | Parent | primary branch only | `M54-10`, `M54-11` | both lanes integrated, conflicts resolved, focused proof rerun |
| `M54-20` | `WS-CLI` | Worker | `spec-cli/tests/cli.rs` and explicit small fixture mutations only if required | `M54-12` | aligned pass proof and negative rejects are green |
| `M54-21` | CLI integration gate | Parent | primary branch only | `M54-20` | CLI proof wall integrated and focused proof rerun |
| `M54-30` | `WS-DOCS` | Worker | `README.md`, `CHANGELOG.md`, `TODOS.md` | `M54-21` | docs match exact proven boundary and nothing broader |
| `M54-31` | Docs integration gate | Parent | primary branch only | `M54-30` | docs integrated after proof surface review |
| `M54-40` | Final proof wall + closeout | Parent | none beyond small conflict resolution if needed | `M54-31` | full acceptance commands pass and closeout ledger is complete |

### `M54-00` Kickoff + baseline

- Owner: Parent
- Unlock condition: none
- Owned files: run-state only under `RUN_ROOT`
- Acceptance:
- branch snapshot, head snapshot, and dirty-tree capture exist
- authority snapshots for `PLAN.md` and `ORCH_PLAN.md` exist
- baseline proof captures exist for the focused pre-change commands
- no product-source files were edited during kickoff

### `M54-01` Contract freeze

- Owner: Parent
- Unlock condition: `M54-00` done
- Owned files: `baseline.json`, `contract-freeze.json`, `worktrees.json`, `file-ownership.json`, `tasks.json`, `queue.json`, `session-log.md`
- Acceptance:
- `contract-freeze.json` records the exact M54 boundaries and stop rules
- `file-ownership.json` records lane ownership without overlap beyond planned integration points
- `tasks.json` and `queue.json` define every M54 task and dependency
- validator-first sequencing is explicitly frozen before any worker launch

### `M54-02` Validator contract

- Owner: Parent
- Unlock condition: `M54-01` done
- Owned files: `spec-core/src/validator.rs`
- Acceptance:
- validator admits only `function.wrapper.pipeline.chain3.v1` as the new M54 TypeScript root family
- exact ordered same-tree chain3 dep contract is enforced
- cross-library deps stay rejected
- molecule TypeScript stays rejected
- nested chain3 closure members stay rejected
- focused validator proof capture is green and stored under `validation/validator/`

### `M54-10` Backend lane

- Owner: Worker
- Unlock condition: `M54-02` integrated in `feat/m40-plus`
- Owned files: `spec-core/src/typescript_backend.rs`
- Acceptance:
- backend emits the exact same-tree chain3 closure and nothing broader
- wrapper recursion under a chain3 root works without unrelated unit leakage
- monotone-up and wrapper behavior are preserved
- worker return includes only changed files, commands with exit codes, and blockers or assumptions

### `M54-11` Fixture lane

- Owner: Worker
- Unlock condition: `M54-02` integrated in `feat/m40-plus`
- Owned files: aligned chain3 fixture `.unit.spec` files only
- Acceptance:
- aligned root and required closure members have truthful non-empty `body.typescript`
- no new units are added
- no negative fixtures are widened unless the parent explicitly reassigns them
- worker return includes only changed files, commands with exit codes, and blockers or assumptions

### `M54-12` Backend + fixture integration gate

- Owner: Parent
- Unlock condition: `M54-10` and `M54-11` submitted
- Owned files: primary branch integration surface only
- Acceptance:
- `WS-BACKEND` and `WS-FIXTURE` diffs are reviewed against their ownership maps
- both lanes are integrated by the parent only
- focused backend and validator-adjacent proofs are rerun after integration
- no hidden dependency on CLI files remains

### `M54-20` CLI proof wall

- Owner: Worker
- Unlock condition: `M54-12` done
- Owned files: `spec-cli/tests/cli.rs` plus explicit tiny fixture mutations only if parent-approved
- Acceptance:
- aligned chain3 TypeScript proof succeeds through Bun
- negative chain3-like paths still reject before Bun where required
- proof scope stays bounded to chain3 and existing TypeScript rules
- worker return includes only changed files, commands with exit codes, and blockers or assumptions

### `M54-21` CLI integration gate

- Owner: Parent
- Unlock condition: `M54-20` submitted
- Owned files: primary branch integration surface only
- Acceptance:
- CLI diff stays within the approved ownership map
- parent integrates the lane and reruns `cargo test -p spec-cli typescript_chain3`
- any contract drift discovered here routes back to the parent-owned contract or integration gate, not to an ad hoc worker fix

### `M54-30` Docs and backlog sync

- Owner: Worker
- Unlock condition: `M54-21` done
- Owned files: `README.md`, `CHANGELOG.md`, `TODOS.md`
- Acceptance:
- docs state bounded same-tree chain3 TypeScript support accurately
- docs do not imply generic multi-dependency TypeScript support
- deferred items remain deferred in `TODOS.md`
- worker return includes only changed files, commands with exit codes, and blockers or assumptions

### `M54-31` Docs integration gate

- Owner: Parent
- Unlock condition: `M54-30` submitted
- Owned files: primary branch integration surface only
- Acceptance:
- parent integrates docs only after verifying wording against integrated code and proofs
- any contract drift exposed by docs returns to the parent-owned contract or integration gate
- no broader support claims land accidentally

### `M54-40` Final proof wall + closeout

- Owner: Parent
- Unlock condition: `M54-31` done
- Owned files: final run-state captures and narrow conflict resolution only if required
- Acceptance:
- final proof commands all run serially in `PRIMARY_ROOT`
- aligned chain3 proof passes
- out-of-contract paths still fail with bounded behavior
- `acceptance-ledger.md` records final signoff, residual concerns, and closeout status

## Worker Return Contract

Each worker returns only:

- changed files
- commands run with exit codes
- blockers or unresolved assumptions

Parent review rules:

- the parent reviews worker summaries plus narrow diffs only, not full transcripts
- the parent records accepted results into the task sentinel directory and `queue.json`
- the parent closes workers after integration; no worker remains open after its lane is merged or rejected
- workers do not become ad hoc integration agents after submission

## Concrete Parent-Agent Responsibilities

The parent owns:

- reading `PLAN.md` and translating it into the frozen orchestration contract
- kickoff validation of branch, workspace dirtiness, and baseline state
- creating `RUN_ROOT`, task ledgers, file ownership maps, and worktree inventory
- the full validator phase in `spec-core/src/validator.rs`
- the exact wording of the frozen TypeScript contract and rejection boundaries
- deciding whether `WS-CLI` may mutate fixtures beyond the aligned pass case; default is no
- integrating `WS-BACKEND`, `WS-FIXTURE`, `WS-CLI`, and `WS-DOCS` into `feat/m40-plus`
- rerunning focused gates after every integration step
- stopping the run when scope drifts into generic execution or cross-library resolution
- running final proof commands
- final acceptance judgment and signoff
- all lane integration and conflict resolution at ownership boundaries

The parent must not:

- start `WS-BACKEND` or `WS-FIXTURE` before validator freeze
- start `WS-CLI` before backend and fixture truth are integrated
- start `WS-DOCS` before CLI proof surfaces stabilize
- let workers widen scope through convenience refactors, schema churn, or infra additions
- let workers fix cross-lane drift by editing outside their ownership maps

## Concrete Worker-Lane Responsibilities

### `WS-BACKEND`

Owned files:

- `spec-core/src/typescript_backend.rs`

Responsibilities:

- extend bounded closure collection for a chain3 root only
- preserve monotone-up and wrapper behavior
- emit the exact same-tree closure and exclude unrelated loaded units
- keep import behavior stable and deduped
- avoid validator contract changes

Stop if:

- backend work needs validator rule changes
- backend work needs cross-library resolution
- backend work starts implying nested chain3 support or generic multi-dependency support

### `WS-FIXTURE`

Owned files:

- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`

Responsibilities:

- add and align `body.typescript` exactly where M54 requires it
- keep TypeScript bodies faithful to existing Rust truth
- avoid adding new units or broadening the fixture family
- avoid touching drift, under-specified, or unsupported-near-miss fixtures unless the parent explicitly reassigns them

Stop if:

- the aligned truth seems to require new units
- the aligned truth cannot mirror existing Rust behavior cleanly
- negative fixtures need changes that would alter validator boundary semantics

### `WS-CLI`

Owned files:

- `spec-cli/tests/cli.rs`
- explicit tiny fixture mutations only if the parent approves them during `M54-21` planning

Responsibilities:

- flip the aligned chain3 TypeScript path from reject-before-Bun to success-through-Bun
- preserve or add negative proofs for wrong-family, wrong-order, missing-`body.typescript`, and molecule rejection behavior
- keep the proof wall specific to chain3, not generic multi-dependency TypeScript

Stop if:

- the CLI lane needs to redefine validator or backend semantics
- the CLI lane needs generic fixture expansion
- the negative proof set depends on cross-library runtime behavior instead of validator rejection

### `WS-DOCS`

Owned files:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Responsibilities:

- document bounded same-tree chain3 TypeScript support accurately
- state explicitly that generic multi-dependency TypeScript remains unsupported
- keep cross-library helper imports and broader TypeScript portability deferred in `TODOS.md`

Stop if:

- docs would need to promise anything broader than the integrated code proves
- docs would need to explain new commands, new dependencies, or schema changes

## Exact Sequencing, Gating, Stop Rules, And Final Integration Flow

### Parent-only integration and conflict rules

- The parent is the only integrator.
- If a lane needs files outside its ownership map, stop and bounce it back to the parent.
- Do not resolve creatively across ownership boundaries.
- If `WS-CLI` exposes contract drift, return to the parent-owned contract or integration gate.
- If `WS-DOCS` exposes contract drift, return to the parent-owned contract or integration gate.
- Workers may identify overlap, but they do not resolve it by expanding their scope.

### Phase 0: Kickoff And Baseline

Run only in `PRIMARY_ROOT`.

Required commands:

```bash
mkdir -p "$RUN_ROOT"/validation/{kickoff,baseline,validator,backend,fixture,cli,docs,final}
mkdir -p "$RUN_ROOT"/tasks

git branch --show-current | tee "$RUN_ROOT/validation/kickoff/branch.txt"
git rev-parse HEAD | tee "$RUN_ROOT/validation/kickoff/head.txt"
git status --porcelain=v1 -uall | tee "$RUN_ROOT/validation/kickoff/git-status.porcelain.txt"
cp "$PRIMARY_ROOT/PLAN.md" "$RUN_ROOT/validation/kickoff/PLAN.md"
cp "$PRIMARY_ROOT/ORCH_PLAN.md" "$RUN_ROOT/validation/kickoff/ORCH_PLAN.md"
```

Kickoff acceptance:

- branch is `feat/m40-plus`
- authority snapshots are captured before code work
- tracked dirty files are understood and tolerated only if the parent explicitly records them
- no one cleans or reverts the tree to make kickoff "look clean"

Run baseline proof capture before changing product code:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/baseline/spec-core-typescript-target.txt"
cargo test -p spec-core typescript_tree | tee "$RUN_ROOT/validation/baseline/spec-core-typescript-tree.txt"
cargo test -p spec-cli typescript_chain3 | tee "$RUN_ROOT/validation/baseline/spec-cli-typescript-chain3.txt"
```

Stop if baseline behavior already contradicts `PLAN.md`. That means the authority plan is stale and must be rewritten before execution continues.

### Phase 1: Contract Freeze

The parent writes these run-state artifacts:

- `baseline.json`
- `contract-freeze.json`
- `worktrees.json`
- `file-ownership.json`
- `queue.json`
- `session-log.md`

The freeze must record:

- supported TypeScript families before and after M54
- the exact chain3 dep tuple and order
- same-tree-only enforcement
- rejection of molecule TypeScript, cross-library imports, seam kinds, nested chain3 members, and generic out-of-family roots
- the lane ownership map from this runbook
- the rule that `WS-BACKEND` and `WS-FIXTURE` are the only overlapping lanes

No worker starts before freeze acceptance is recorded.

### Phase 2: Parent-Owned Validator Contract

Parent edits only:

- `spec-core/src/validator.rs`

Required implementation scope:

- add the chain3 TypeScript compatibility key
- extend root-family classification
- enforce family-aware dep-count gates
- add exact ordered chain3 dep-contract validation
- keep same-tree-only behavior
- preserve supported existing families
- reject nested chain3 closure members

Focused proof commands after the validator change:

```bash
cargo test -p spec-core typescript_target | tee "$RUN_ROOT/validation/validator/spec-core-typescript-target.txt"
```

Gate to unlock parallel lanes:

- validator patch is integrated in `feat/m40-plus`
- focused validator proofs are green
- frozen rejection wording is good enough that CLI tests do not need to invent contract language later

Stop if the validator patch starts requiring backend refactors to understand basic scope. Validator is the contract, not a moving guess.

### Phase 3: Safe Parallel Lanes After Validator Freeze

Only now may the parent create worker worktrees.

Recommended creation pattern:

```bash
git worktree add "$WT_ROOT/ws-backend" -b codex/m54-ws-backend feat/m40-plus
git worktree add "$WT_ROOT/ws-fixture" -b codex/m54-ws-fixture feat/m40-plus
```

Active overlap rules:

- `WS-BACKEND` edits only `spec-core/src/typescript_backend.rs`
- `WS-FIXTURE` edits only the aligned chain3 fixture files
- neither lane edits `spec-core/src/validator.rs`
- neither lane edits `spec-cli/tests/cli.rs`

Lane acceptance:

```bash
# WS-BACKEND
cargo test -p spec-core typescript_tree

# WS-FIXTURE
cargo test -p spec-core typescript_target
```

Parent integration flow for `M54-12`:

1. integrate `WS-BACKEND` into `feat/m40-plus`
2. rerun `cargo test -p spec-core typescript_tree`
3. integrate `WS-FIXTURE` into `feat/m40-plus`
4. rerun `cargo test -p spec-core typescript_target`
5. rerun both focused commands once both are integrated

Stop if either lane overlaps outside its ownership map or if either lane exposes a hidden dependency on the CLI test file.

### Phase 4: CLI Proof Wall

Do not start this phase until validator, backend, and aligned fixture truth all exist in `feat/m40-plus`.

Create the worktree only after `M54-12` passes:

```bash
git worktree add "$WT_ROOT/ws-cli" -b codex/m54-ws-cli feat/m40-plus
```

`WS-CLI` responsibilities:

- replace the aligned pre-Bun rejection proof with success-through-Bun proof
- preserve targeted negative rejections
- keep the proof scope bounded to chain3 and existing TypeScript boundaries

Lane acceptance:

```bash
cargo test -p spec-cli typescript_chain3
```

Parent integration flow for `M54-21`:

1. review whether `WS-CLI` changed only allowed files
2. integrate into `feat/m40-plus`
3. rerun `cargo test -p spec-cli typescript_chain3`
4. if negative proof behavior changed unexpectedly, stop and fix the contract before docs begin

### Phase 5: Docs And Backlog Sync

Docs run last, not concurrently with proof-shaping code.

Create the worktree only after `M54-21` passes:

```bash
git worktree add "$WT_ROOT/ws-docs" -b codex/m54-ws-docs feat/m40-plus
```

`WS-DOCS` may now update:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Parent integration flow for `M54-31`:

1. review every wording change against the integrated code and tests
2. integrate into `feat/m40-plus`
3. reject any claim that implies generic multi-dependency TypeScript support

### Phase 6: Final Integration And Proof Wall

This phase is parent-owned and must run serially in `PRIMARY_ROOT`.

Required final proof commands:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/checkout_chain3_unsupported_near_miss.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/checkout_chain3_drift.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/checkout_chain3_under_specified.unit.spec --target-language typescript
```

Final acceptance rules:

- aligned chain3 TypeScript proof passes
- out-of-contract chain3-like paths still fail with bounded-lane rejection behavior
- Bun is not invoked for validator-level rejects
- monotone-up and wrapper TypeScript proofs still pass
- docs match the actual integrated boundary

If any final proof requires reopening validator semantics, stop and return to the parent-owned contract phase. Do not paper over it in docs or CLI tests.

## Tests And Acceptance

### Required focused coverage by phase

Validator phase must cover:

- exact chain3 root acceptance
- wrong dep order rejection
- cross-library dep rejection
- missing dep rejection
- unsupported dep family rejection
- missing `body.typescript` rejection
- generic four-dependency rejection
- nested chain3 closure-member rejection

Backend phase must cover:

- exact same-tree chain3 closure rendering
- wrapper closure recursion under a chain3 root
- deduped emission
- unrelated loaded unit exclusion

CLI phase must cover:

- aligned chain3 success through Bun
- wrong-family rejection before Bun
- missing-`body.typescript` rejection before Bun
- wrong-order rejection before Bun if a small explicit mutation proves it honestly
- molecule TypeScript rejection before Bun

Docs phase acceptance:

- `README.md` states chain3 support narrowly
- `CHANGELOG.md` records bounded same-tree chain3 TypeScript support
- `TODOS.md` still defers cross-library imports and generic multi-dependency TypeScript

### Definition of done

M54 is done only when:

1. the validator admits exactly `function.wrapper.pipeline.chain3.v1` under the frozen same-tree contract
2. backend closure emission includes only the required same-tree closure
3. aligned chain3 fixtures contain honest maintained TypeScript bodies
4. CLI aligned proof passes
5. negative validator boundaries remain intact
6. no schema, command, or dependency drift was introduced
7. docs describe the exact proven boundary and nothing broader

## Assumptions

- `PLAN.md` remains the authority and is not superseded mid-run.
- `feat/m40-plus` remains the primary execution branch for M54.
- The repo may be dirty because other agents or the user are working. The parent inspects and preserves those edits rather than reverting them.
- `cargo test -p spec-core ...` and `cargo test -p spec-cli ...` remain the correct proof entrypoints from `PLAN.md`.
- The aligned chain3 fixture is already semantically classifiable as `function.wrapper.pipeline.chain3.v1`; M54 is about bounded execution truth, not family-analysis expansion.
- No new generated artifact contract needs to be authored for this milestone.
- The parent agent is responsible for deciding whether an unexpected overlap is a real blocker or just stale worker scope. Default to blocker.

## Parallel Subagent Optimization

Safe optimization exists, but only in one place:

- optimize by overlapping `WS-BACKEND` and `WS-FIXTURE` after validator freeze
- do not overlap validator work with anything else
- do not overlap CLI proof-wall work with backend or fixture truth shaping
- do not overlap docs with unstable proof surfaces

This means the honest launch pattern is:

1. parent kickoff and baseline
2. parent contract freeze
3. parent validator contract
4. `WS-BACKEND` + `WS-FIXTURE` in parallel
5. parent integration gate
6. `WS-CLI`
7. parent integration gate
8. `WS-DOCS`
9. parent final proof wall and closeout

Any orchestration that tries to create more concurrency than that is manufacturing merge risk, not saving time.
