# M45 Orchestration Plan

Status: **authoritative kickoff and execution contract for M45 bounded TypeScript execution**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Execute from current branch: **`feat/m40-plus`**  
Scope diff anchor commit: **`295f9c5`**  
Last rewritten: **`2026-05-10`**

## Summary

- Execute from the current repo root on branch `feat/m40-plus`.
- Use `295f9c5` as the fixed scope diff anchor for M45 boundary checks. It is the plan-authoring tip, not a required starting `HEAD`.
- Keep the true critical path in the parent lane for:
  - baseline capture
  - authority freeze
  - target-language and bounded-lane freeze
  - merge preview
  - merge sequencing
  - final proof wall
  - closeout
- Launch exactly two early worker lanes in parallel after the freeze:
  - generation
  - execution and proof routing
- Launch one late worker lane only after the parent has merged the first two lanes into a stable preview commit:
  - acceptance and docs
- Worker concurrency cap is **2** before merge preview and **1** after merge preview.
- Worker model assumption is fixed for all worker lanes:
  - `model = GPT-5.4`
  - `reasoning_effort = high`
- Use dedicated `spec-m45` worktrees and branches:
  - primary baseline: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `ws/spec-m45-target-freeze` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze`
  - `ws/spec-m45-generation` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation`
  - `ws/spec-m45-execution-proof` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof`
  - `ws/spec-m45-acceptance-docs` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs`
  - `ws/spec-m45-integration` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration`
- Keep orchestration state in one canonical parent-owned run root:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `M45_RUN_ROOT=$PRIMARY_ROOT/.runs/m45_bounded_typescript_lane`
  - `queue=$M45_RUN_ROOT/queue.json`
  - `tasks=$M45_RUN_ROOT/tasks.json`
  - `session_log=$M45_RUN_ROOT/session-log.md`
  - `target_freeze=$M45_RUN_ROOT/target-freeze.json`
  - `acceptance=$M45_RUN_ROOT/acceptance.md`
  - `merge_log=$M45_RUN_ROOT/merge-log.md`
- Treat authored source, run-state artifacts, and derived proof artifacts as different classes:
  - authored source is the milestone deliverable
  - `.runs/**` is parent-owned orchestration state only
  - refreshed passports, export captures, and proof-wall command logs are derived proof output only
- A clean tree is not required to start. The parent must capture the actual starting `HEAD` and dirty state before opening worktrees.
- `PLAN.md` is already dirty in the current workspace. That is authority input, not M45 output.

## Hard Guards

- `PLAN.md` is the sole scope authority. `ORCH_PLAN.md` is the execution contract, not a second specification.
- M45 scope is exactly one first-class TypeScript execution lane:
  - `kind:function`
  - compatibility key `function.arithmetic_leaf.monotone_up.v1`
  - `deps: []`
  - generated `.ts` output only
  - Bun build and Bun execution only
  - atom tests only
- `spec validate` does not gain `--target-language` support in M45.
- `spec export` does not gain `--target-language` support in M45.
- `.test.spec` remains unsupported for TypeScript in M45 and must fail before Bun runs.
- Wrapper execution remains out of scope even though packet proof already exists in `xtask`.
- No dependency-bearing unit is allowed into the TypeScript lane.
- No seam kinds are allowed into the TypeScript lane.
- No `number`-based decimal shortcut is allowed. The lane must use the bounded fixed-point `bigint` helper.
- TypeScript proof is additive only:
  - `target_proofs.rust`
  - `target_proofs.typescript`
- Top-level Rust-facing proof mirrors remain Rust-only compatibility surfaces. They must not become a merged cross-target truth surface.
- `spec status --target-language typescript` must read only `target_proofs.typescript`. It must never inherit Rust proof.
- The accepted atom-test translation floor is exactly the bounded AST contract from `PLAN.md`. No best-effort fallback and no raw-string translation loophole.
- Generated TypeScript helper filenames are frozen after the parent target-freeze gate:
  - `__spec_ts/runtime.ts`
  - `__spec_ts/build_entry.ts`
  - `__spec_ts/local_tests.ts`
- The parent is the sole owner of:
  - `M45_RUN_ROOT/**`
  - baseline and authority artifacts
  - target-freeze artifacts
  - merge preview
  - merge sequencing
  - final integration
  - proof-wall execution
  - closeout
- No worker lane may edit:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `.runs/**`
  - another lane's owned files
- No worker may widen M45 into:
  - generic TypeScript support
  - wrapper parity
  - molecule parity
  - seam-kind parity
  - `spec validate --target-language`
  - runtime or package-manager abstraction
  - proof-schema redesign beyond additive `target_proofs`
- Fixture and example source files are read-only by default in M45:
  - `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`
  - `examples/ecommerce/units/pricing/**`
  If the proof wall or bounded-lane regressions prove one of those sources is incomplete for M45, the parent may reopen that surface narrowly. Any such reopening must be recorded explicitly in `authority-freeze.json` before a worker touches it, and the reopened files must get an explicit owner instead of drifting into shared custody.
- Do not revert or overwrite unrelated user changes. Integrate around the current repo state.

## Execution Topology

| Role | Branch | Worktree | Owner | Scope |
|---|---|---|---|---|
| primary baseline | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | authority, run-state, final landing |
| target freeze | `ws/spec-m45-target-freeze` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze` | parent | `TargetLanguage`, CLI flag surface, bounded TS eligibility, frozen helper filenames and proof-field names |
| generation lane | `ws/spec-m45-generation` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation` | worker A | `spec-core/src/typescript_backend.rs`, `spec-core/src/generator.rs`, `spec-core/src/lib.rs` |
| execution and proof lane | `ws/spec-m45-execution-proof` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof` | worker B | `spec-core/src/pipeline.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, post-freeze `spec-cli/src/commands.rs` routing only |
| acceptance and docs lane | `ws/spec-m45-acceptance-docs` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs` | worker C | `spec-cli/tests/cli.rs`, `README.md`, `CHANGELOG.md` |
| integration | `ws/spec-m45-integration` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration` | parent | merge preview, worker merge intake, parity cleanup, proof wall, final acceptance |

Rules:

- Worker A and worker B must fork from the exact `target_freeze_commit` recorded in `target-freeze.json`.
- Worker C must fork from the exact `acceptance_base_commit` recorded by the parent during merge preview. It must not fork directly from `feat/m40-plus`.
- `feat/m40-plus` is the canonical landing branch. After the proof wall passes on `ws/spec-m45-integration`, the parent fast-forwards `feat/m40-plus` to that integrated commit before closeout.
- The parent is the sole integrator.
- Default merge order is:
  1. `ws/spec-m45-generation`
  2. `ws/spec-m45-execution-proof`
  3. `ws/spec-m45-acceptance-docs`
- The merge order is intentional:
  - lane A freezes generated-tree reality first
  - lane B threads Bun and proof routing through that frozen tree shape
  - lane C locks tests and docs only after code truth exists
- If lane B believes the frozen filenames, flag surface, or proof-field names are wrong, it must stop and bounce to the parent. It may not silently re-freeze the lane.
- If lane C discovers missing code behavior rather than missing tests or docs, it must stop and bounce to the parent. It may not patch code ownership surfaces itself.

## Canonical Run-State And Artifact Surfaces

### Authored source deliverables

Only these authored surfaces are in-bounds by default for M45:

- `spec-core/src/types.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/pipeline.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `README.md`
- `CHANGELOG.md`

Read-only authority inputs for M45 unless the parent reopens scope explicitly:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`
- `examples/ecommerce/units/pricing/**`

If the parent reopens either surface, the reopened files become authored M45 deliverables for that run and must be added to `in-scope-files.txt`, lane ownership, and final boundary checks immediately.

Inline tests follow the ownership of the file they live in.

### Parent-owned run-state artifacts

Canonical parent-owned run root:

- `M45_RUN_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane`

Required parent-owned artifacts:

- `baseline.json`
- `authority-freeze.json`
- `target-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`
- `merge-log.md`
- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination
- `validation/**`

Execution-record expectations:

- `baseline.json`
  - baseline branch
  - starting `HEAD`
  - scope diff anchor commit
  - initial `git status --short`
  - explicit note that `PLAN.md` was already dirty at baseline if still true
- `authority-freeze.json`
  - authority `PLAN.md` path
  - frozen in-scope authored surfaces
  - frozen read-only fixture and example surfaces
  - frozen out-of-scope surfaces
  - hard-guard summary
- `target-freeze.json`
  - `target_freeze_commit`
  - frozen `TargetLanguage` enum spelling
  - commands that accept `--target-language`
  - commands that do not accept `--target-language`
  - frozen helper filenames
  - frozen TypeScript proof field names
  - frozen unsupported-lane rule summary
  - banned post-freeze drift
- `merge-log.md`
  - merge preview commit
  - `acceptance_base_commit`
  - merge order
  - conflicts encountered
  - exact files manually repaired by the parent
  - explicit note if the parent relaunched a lane
- `acceptance.md`
  - proof-wall command outcomes
  - target-proof separation checklist
  - bounded-lane checklist
  - final diff boundary verdict
- `closeout.md`
  - landed scope summary
  - remaining risks
  - blocked items, if any
  - deferred follow-ups outside M45

Expected `M45_RUN_ROOT/validation/` records:

- `validation/baseline/git-status.short.txt`
- `validation/baseline/git-diff.scope-anchor-name-only.txt`
- `validation/baseline/git-diff.scope-anchor-stat.txt`
- `validation/authority/in-scope-files.txt`
- `validation/authority/out-of-scope-files.txt`
- `validation/target-freeze/spec-core-tests.txt`
- `validation/target-freeze/spec-cli-tests.txt`
- `validation/merge-preview/git-merge-status.txt`
- `validation/merge/final-name-only.diff`
- `validation/merge/final-stat.diff`
- `validation/proof-wall/cargo-test.txt`
- `validation/proof-wall/spec-generate-aligned-typescript.txt`
- `validation/proof-wall/spec-build-aligned-typescript.txt`
- `validation/proof-wall/spec-test-aligned-typescript.txt`
- `validation/proof-wall/spec-test-drift-typescript.txt`
- `validation/proof-wall/spec-test-unsupported-near-miss-typescript.txt`
- `validation/proof-wall/spec-test-example-apply-tax-typescript.txt`
- `validation/proof-wall/spec-test-molecule-negative-typescript.txt`
- `validation/proof-wall/spec-status-typescript.json`
- `validation/proof-wall/spec-export.json`
- `validation/proof-wall/family-prove-typescript.txt`
- `validation/closeout/final-git-status.short.txt`

The parent may add more evidence files, but these are the minimum expected record surfaces.

### Per-task sentinels

Each gate or task gets a sentinel directory under `.runs/`:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-00-baseline/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-05-authority-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-a1-target-freeze/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-15-worker-launch/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-b-generation/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-c-execution-proof/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-35-merge-preview/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-37-acceptance-launch/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-d-acceptance-docs/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-40-merge-window/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-f-integration/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-50-proof-wall/`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-60-closeout/`

Each sentinel directory may contain:

- `started.json`
- `status.json`
- `done.json`
- `blocked.json`

### Derived proof artifacts

These are derived outputs, not authored deliverables:

- refreshed `*.spec.passport.json` files touched by:
  - `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/**`
  - `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- proof-wall JSON or text captures under `M45_RUN_ROOT/validation/`
- temporary generated TypeScript trees emitted by `spec generate/build/test`

Rules:

- Derived proof artifacts may change only because proof or validation commands ran.
- No one hand-edits passports or command captures.
- Parent acceptance is based on command results and repo truth, not on preserving generated output as authored source.

## Queue And Gates

| Order | ID | Kind | Owner | Worktree | Opens when |
|---|---|---|---|---|---|
| 1 | `task-m45-00-baseline` | gate | parent | primary | repo baseline and dirty-state capture complete |
| 2 | `task-m45-05-authority-freeze` | gate | parent | primary | `PLAN.md` scope, in-scope files, and hard guards are recorded |
| 3 | `task-m45-a1-target-freeze` | task | parent | `ws/spec-m45-target-freeze` | baseline and authority freeze are complete |
| 4 | `task-m45-15-worker-launch` | gate | parent | primary | `target-freeze.json` exists with commit, frozen helper names, and banned drift |
| 5 | `task-m45-b-generation` | task | worker A | `ws/spec-m45-generation` | worker launch gate is open |
| 6 | `task-m45-c-execution-proof` | task | worker B | `ws/spec-m45-execution-proof` | worker launch gate is open |
| 7 | `task-m45-35-merge-preview` | gate | parent | `ws/spec-m45-integration` | lanes A and B are submitted or explicitly blocked |
| 8 | `task-m45-37-acceptance-launch` | gate | parent | primary | `acceptance_base_commit` exists and lane C scope is still bounded |
| 9 | `task-m45-d-acceptance-docs` | task | worker C | `ws/spec-m45-acceptance-docs` | acceptance launch gate is open |
| 10 | `task-m45-40-merge-window` | gate | parent | primary | worker C handoff is submitted or explicitly blocked |
| 11 | `task-m45-f-integration` | task | parent | `ws/spec-m45-integration` | merge window is open |
| 12 | `task-m45-50-proof-wall` | gate | parent | `ws/spec-m45-integration` | integrated branch is merged and locally consistent |
| 13 | `task-m45-60-closeout` | gate | parent | primary | full proof wall is green, `feat/m40-plus` is fast-forwarded, and acceptance is recorded |

## Workstream Plan

### `task-m45-00-baseline` - parent only

Purpose:

- capture the execution baseline before any M45 worktree opens

Owned files and artifacts:

- `M45_RUN_ROOT/baseline.json`
- `M45_RUN_ROOT/run-state.json`
- `M45_RUN_ROOT/validation/baseline/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-00-baseline/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane/validation/baseline
git rev-parse --abbrev-ref HEAD
git rev-parse --short HEAD
git merge-base --is-ancestor 295f9c5 HEAD
git status --short
git diff --name-only 295f9c5..HEAD
git diff --stat 295f9c5..HEAD
```

Artifacts written:

- `baseline.json`
- `validation/baseline/git-status.short.txt`
- `validation/baseline/git-diff.scope-anchor-name-only.txt`
- `validation/baseline/git-diff.scope-anchor-stat.txt`
- `task-m45-00-baseline/started.json`
- `task-m45-00-baseline/done.json`

Blocked conditions:

- current branch is not `feat/m40-plus`
- current `HEAD` is not a descendant of `295f9c5`
- repo state is ambiguous enough that the parent cannot distinguish pre-existing unrelated edits from M45 execution state

Restart point if blocked:

- stop before worktree creation
- record the reason in `task-m45-00-baseline/blocked.json`
- restart from `task-m45-00-baseline` after the parent re-establishes the correct branch and baseline

### `task-m45-05-authority-freeze` - parent only

Purpose:

- freeze the authoritative M45 scope, owned authored surfaces, read-only input surfaces, and non-negotiable guards

Owned files and artifacts:

- `M45_RUN_ROOT/authority-freeze.json`
- `M45_RUN_ROOT/in-scope-files.txt`
- `M45_RUN_ROOT/validation/authority/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-05-authority-freeze/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane/validation/authority
sed -n '1,666p' PLAN.md
git ls-files spec-core/src spec-cli/src README.md CHANGELOG.md semantic-families examples
```

Artifacts written:

- `authority-freeze.json`
- `in-scope-files.txt`
- `validation/authority/in-scope-files.txt`
- `validation/authority/out-of-scope-files.txt`
- `task-m45-05-authority-freeze/started.json`
- `task-m45-05-authority-freeze/done.json`

Blocked conditions:

- `PLAN.md` scope is unclear, contradictory, or changes mid-run
- the parent cannot freeze the read-only fixture and example surfaces cleanly
- the parent identifies mandatory authored files outside the currently authorized M45 surface

Restart point if blocked:

- stop before editing the target-freeze lane
- record the blocker in `task-m45-05-authority-freeze/blocked.json`
- restart from `task-m45-05-authority-freeze` after the parent resolves scope authority

### `task-m45-a1-target-freeze` - parent only

Purpose:

- freeze the shared target-language contract and bounded-lane eligibility before parallel implementation begins

Owned files and directories:

- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`

Required commands:

```bash
if git worktree list --porcelain | grep -F "worktree /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze" >/dev/null; then
  git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze status --short
elif git show-ref --verify --quiet refs/heads/ws/spec-m45-target-freeze; then
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze ws/spec-m45-target-freeze
else
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/target-freeze -b ws/spec-m45-target-freeze feat/m40-plus
fi
cargo test -p spec-core -- --color never
cargo test -p spec-cli -- --color never
git status --short
```

Artifacts written:

- `target-freeze.json`
- `validation/target-freeze/spec-core-tests.txt`
- `validation/target-freeze/spec-cli-tests.txt`
- `task-m45-a1-target-freeze/started.json`
- `task-m45-a1-target-freeze/done.json`

Acceptance:

- One shared `TargetLanguage` enum exists and is frozen for downstream use.
- `--target-language` is frozen to `rust|typescript`.
- Rust remains the default target at the command boundary.
- Only these commands gain `--target-language` in M45:
  - `spec generate`
  - `spec build`
  - `spec test`
  - `spec status`
- `spec validate` and `spec export` are explicitly recorded as no-flag surfaces in `target-freeze.json`.
- The bounded TypeScript eligibility contract is frozen:
  - family must be `function.arithmetic_leaf.monotone_up.v1`
  - `deps` must be empty
  - `.test.spec` is rejected
  - atom-test `expect` must match the bounded AST grammar
- Frozen generated file names are recorded:
  - `__spec_ts/runtime.ts`
  - `__spec_ts/build_entry.ts`
  - `__spec_ts/local_tests.ts`
- Frozen proof field names are recorded:
  - `target_proofs.rust`
  - `target_proofs.typescript`
- Frozen unsupported-lane wording is specific enough that downstream lanes can test against it without guessing.

Blocked conditions:

- the parent cannot stabilize enum spelling, helper filenames, or proof-field names cleanly enough for parallel work
- the bounded TS gate still requires generation or pipeline code to decide eligibility
- the lane needs broader file rewiring to make the freeze compile

Restart point if blocked:

- stop before worker launch
- record the blocker in `task-m45-a1-target-freeze/blocked.json`
- restart from `task-m45-a1-target-freeze` after the parent resolves the contract locally

### `task-m45-15-worker-launch` - parent only

Purpose:

- launch the first two worker lanes from one frozen commit and record exact file ownership plus banned drift

Owned files and artifacts:

- `M45_RUN_ROOT/tasks.json`
- `M45_RUN_ROOT/queue.json`
- `M45_RUN_ROOT/session-log.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-15-worker-launch/**`

Required commands:

```bash
git rev-parse --short ws/spec-m45-target-freeze
if git worktree list --porcelain | grep -F "worktree /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation" >/dev/null; then
  git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation status --short
elif git show-ref --verify --quiet refs/heads/ws/spec-m45-generation; then
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation ws/spec-m45-generation
else
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/generation -b ws/spec-m45-generation ws/spec-m45-target-freeze
fi
if git worktree list --porcelain | grep -F "worktree /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof" >/dev/null; then
  git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof status --short
elif git show-ref --verify --quiet refs/heads/ws/spec-m45-execution-proof; then
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof ws/spec-m45-execution-proof
else
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/execution-proof -b ws/spec-m45-execution-proof ws/spec-m45-target-freeze
fi
git worktree list
```

Artifacts written:

- updated `tasks.json`
- updated `queue.json`
- worker-launch note in `session-log.md`
- `task-m45-15-worker-launch/started.json`
- `task-m45-15-worker-launch/done.json`

Blocked conditions:

- `target-freeze.json` is missing or does not identify a single frozen commit with helper filenames, proof fields, and command-flag boundaries
- worker worktrees or branches cannot be created cleanly from the frozen commit
- lane ownership is still ambiguous enough that a worker would need to guess file scope

Restart point if blocked:

- stop before issuing worker prompts
- record the blocker in `task-m45-15-worker-launch/blocked.json`
- restart from `task-m45-15-worker-launch` after worktrees and lane contracts are clean

### `task-m45-b-generation` - worker A

Purpose:

- make authored TypeScript executable by emitting the bounded M45 TypeScript tree without owning Bun execution or proof routing

Owned files and directories:

- `spec-core/src/typescript_backend.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/lib.rs`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m45-b-generation/started.json`
- `task-m45-b-generation/status.json`
- `task-m45-b-generation/done.json` or `blocked.json`

Acceptance:

- `spec-core/src/typescript_backend.rs` exists and is explicitly documented as the bounded M45 lane, not generic TS parity.
- `generator.rs` dispatches by target and no longer ignores authored TypeScript when the bounded lane is eligible.
- The generated tree contains:
  - one `.ts` module per eligible unit
  - `__spec_ts/runtime.ts`
  - `__spec_ts/build_entry.ts`
  - `__spec_ts/local_tests.ts`
- Helper and harness files are emitted once per output root, not once per unit.
- The old regression proving TS is ignored is replaced by one proving authored TS is emitted into the bounded lane.
- The lane does not add Bun execution, proof writes, or status routing.

Blocked conditions:

- the frozen helper filenames or proof contract no longer suffice for generation output
- the lane needs to edit pipeline, passport, export, commands, docs, or read-only fixture sources to make progress
- a failing command indicates eligibility or routing drift rather than generation drift

Restart point if blocked:

- stop in `ws/spec-m45-generation`
- hand the blocker to the parent with the failing command and missing frozen assumption
- restart from `task-m45-b-generation` after the parent republishes a valid freeze or reassigns the cross-lane fix

### `task-m45-c-execution-proof` - worker B

Purpose:

- add Bun build and test execution plus additive TypeScript proof routing without rewriting the frozen lane boundary

Owned files and directories:

- `spec-core/src/pipeline.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m45-c-execution-proof/started.json`
- `task-m45-c-execution-proof/status.json`
- `task-m45-c-execution-proof/done.json` or `blocked.json`

Acceptance:

- `pipeline.rs` owns Bun build and Bun execution helpers for the TS lane.
- Bun is the only TypeScript execution contract. No runtime auto-detection layer appears.
- `commands.rs` routes `generate`, `build`, `test`, and `status` through the frozen target-language surface and nothing else.
- `passport.rs` stores additive TypeScript proof truth under `target_proofs.typescript` and leaves Rust truth untouched.
- `export.rs` carries additive target-proof data honestly.
- `status --target-language typescript` reads TS proof only and reports `untested` when TS proof is missing.
- The lane does not widen validate support, molecule execution, wrapper execution, dependency support, or docs.

Blocked conditions:

- the lane needs to rename frozen helper filenames, proof fields, or CLI flag spelling
- the lane needs to edit generator or backend files to make Bun or proof routing work
- the lane would need read-only fixture source edits to prove the bounded lane

Restart point if blocked:

- stop in `ws/spec-m45-execution-proof`
- hand the blocker to the parent with the failing command, missing symbol, or frozen-name conflict
- restart from `task-m45-c-execution-proof` after the parent resolves the cross-lane issue

### `task-m45-35-merge-preview` - parent only

Purpose:

- merge the first two lanes into a stable preview commit and freeze the acceptance base for worker C

Owned files and artifacts:

- `M45_RUN_ROOT/merge-log.md`
- `M45_RUN_ROOT/validation/merge-preview/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-35-merge-preview/**`

Required commands:

```bash
if git worktree list --porcelain | grep -F "worktree /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration" >/dev/null; then
  git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration status --short
elif git show-ref --verify --quiet refs/heads/ws/spec-m45-integration; then
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration ws/spec-m45-integration
else
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration -b ws/spec-m45-integration ws/spec-m45-target-freeze
fi
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration merge --no-ff ws/spec-m45-generation
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration merge --no-ff ws/spec-m45-execution-proof
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration status --short
```

Artifacts written:

- merge-preview note in `merge-log.md`
- `validation/merge-preview/git-merge-status.txt`
- `task-m45-35-merge-preview/started.json`
- `task-m45-35-merge-preview/done.json` or `blocked.json`

Acceptance:

- lanes A and B merge in the declared order
- the merged preview commit is recorded as `acceptance_base_commit`
- parent repairs during preview are limited to:
  - merge mechanics
  - line-level conflict resolution
  - narrow in-scope drift between generation output shape and execution/proof routing
- the preview is stable enough that worker C can write tests and docs without guessing filenames or proof shape

Blocked conditions:

- merge conflicts cannot be resolved without reopening file ownership or widening scope
- A and B still disagree on helper filenames, command routing, or proof-field shape
- the preview still requires read-only fixture source edits that were never reopened by the parent

Restart point if blocked:

- stop in `ws/spec-m45-integration`
- record the blocker in `task-m45-35-merge-preview/blocked.json` and `merge-log.md`
- restart from `task-m45-15-worker-launch` if a lane must be relaunched, otherwise restart from `task-m45-35-merge-preview`

### `task-m45-37-acceptance-launch` - parent only

Purpose:

- launch worker C from the merged preview commit, not from speculative code

Owned files and artifacts:

- `M45_RUN_ROOT/tasks.json`
- `M45_RUN_ROOT/queue.json`
- `M45_RUN_ROOT/session-log.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-37-acceptance-launch/**`

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration rev-parse --short HEAD
if git worktree list --porcelain | grep -F "worktree /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs" >/dev/null; then
  git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs status --short
elif git show-ref --verify --quiet refs/heads/ws/spec-m45-acceptance-docs; then
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs ws/spec-m45-acceptance-docs
else
  git worktree add /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/acceptance-docs -b ws/spec-m45-acceptance-docs "$(git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration rev-parse HEAD)"
fi
git worktree list
```

Artifacts written:

- updated `tasks.json`
- updated `queue.json`
- acceptance-launch note in `session-log.md`
- `task-m45-37-acceptance-launch/started.json`
- `task-m45-37-acceptance-launch/done.json`

Blocked conditions:

- `acceptance_base_commit` was not recorded during merge preview
- the preview branch is still unstable enough that worker C would need to guess product truth
- worker C ownership would need to extend into code files to complete the lane

Restart point if blocked:

- stop before issuing the worker C prompt
- record the blocker in `task-m45-37-acceptance-launch/blocked.json`
- restart from `task-m45-35-merge-preview` after the parent re-stabilizes the preview

### `task-m45-d-acceptance-docs` - worker C

Purpose:

- lock end-to-end bounded-lane regressions and write only the docs that the landed code actually justifies

Owned files and directories:

- `spec-cli/tests/cli.rs`
- `README.md`
- `CHANGELOG.md`

Required commands:

```bash
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
git status --short
```

Artifacts written:

- worker handoff summary in lane-local notes
- `task-m45-d-acceptance-docs/started.json`
- `task-m45-d-acceptance-docs/status.json`
- `task-m45-d-acceptance-docs/done.json` or `blocked.json`

Acceptance:

- `spec-cli/tests/cli.rs` locks the product surface for:
  - aligned fixture success
  - drift fixture behavior
  - unsupported near-miss failure before Bun
  - example-unit success for `pricing/apply_tax`
  - molecule negative failure for `pricing/discount_plus_tax.test.spec`
  - target-specific status behavior
  - additive proof export behavior
  - absence of TypeScript support for `spec validate`
- `README.md` documents:
  - Bun as the only TypeScript prerequisite
  - monotone-up only boundary
  - zero-dependency requirement
  - atom-only boundary
  - bounded atom-test grammar limits
- `CHANGELOG.md` records first-class bounded TS execution and nothing broader.
- The lane does not edit code ownership files or read-only fixture sources.

Blocked conditions:

- CLI regressions expose missing code behavior rather than missing tests
- truthful docs would require broader claims than the landed code allows
- the lane appears to require fixture-source or example-source edits

Restart point if blocked:

- stop in `ws/spec-m45-acceptance-docs`
- hand the blocker to the parent with the failing command or wording gap
- restart from `task-m45-d-acceptance-docs` after the parent resolves the upstream issue

### `task-m45-40-merge-window` - parent only

Purpose:

- verify worker handoffs are complete, merge-safe, and still inside frozen ownership before final integration

Owned files and artifacts:

- `M45_RUN_ROOT/merge-log.md`
- `M45_RUN_ROOT/tasks.json`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-40-merge-window/**`

Required commands:

```bash
git rev-parse --short ws/spec-m45-generation
git rev-parse --short ws/spec-m45-execution-proof
git rev-parse --short ws/spec-m45-acceptance-docs
git diff --name-only ws/spec-m45-target-freeze..ws/spec-m45-generation
git diff --name-only ws/spec-m45-target-freeze..ws/spec-m45-execution-proof
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration diff --name-only HEAD..ws/spec-m45-acceptance-docs
```

Artifacts written:

- merge-window note in `merge-log.md`
- updated `tasks.json`
- `task-m45-40-merge-window/started.json`
- `task-m45-40-merge-window/done.json`

Blocked conditions:

- a worker lane changed files outside its frozen ownership
- a worker lane is incomplete and not explicitly blocked
- handoff notes are missing enough detail that the parent cannot merge safely

Restart point if blocked:

- stop before final integration work begins
- record the blocker in `task-m45-40-merge-window/blocked.json`
- restart from `task-m45-15-worker-launch` or `task-m45-37-acceptance-launch` depending on which lane must be relaunched

### `task-m45-f-integration` - parent only

Purpose:

- merge worker C onto the preview branch, repair narrow in-scope drift, and prepare the exact tree that will face the proof wall

Owned files and directories:

- merged in-scope authored files from all lanes
- parent-owned run-state artifacts under `M45_RUN_ROOT/**`

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration merge --no-ff ws/spec-m45-acceptance-docs
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration diff --name-only 295f9c5..HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration diff --stat 295f9c5..HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/integration status --short
```

Artifacts written:

- updated `merge-log.md`
- `validation/merge/final-name-only.diff`
- `validation/merge/final-stat.diff`
- `task-m45-f-integration/started.json`
- `task-m45-f-integration/status.json`
- `task-m45-f-integration/done.json` or `blocked.json`

Acceptance:

- parent repair stays inside the frozen authored surfaces
- no silent drift is introduced in:
  - frozen helper filenames
  - frozen proof fields
  - command flag boundaries
  - bounded unsupported-lane rules
- the integrated tree still does not widen into wrapper, molecule, seam, dependency, or validate-target support
- docs remain bounded to what the code now does

Blocked conditions:

- merge conflicts cannot be resolved without reopening ownership or widening scope
- the integrated tree requires fixture or example source edits that were never reopened
- proof-wall preparation now depends on out-of-scope files

Restart point if blocked:

- stop in `ws/spec-m45-integration`
- record the blocker in `task-m45-f-integration/blocked.json` and `merge-log.md`
- restart from `task-m45-40-merge-window` if a lane must be relaunched, otherwise restart from `task-m45-f-integration`

### `task-m45-50-proof-wall` - parent only

Purpose:

- run and record the exact M45 proof wall on the integrated branch and treat any failure as a stop condition

Owned files and artifacts:

- `M45_RUN_ROOT/validation/proof-wall/**`
- `M45_RUN_ROOT/acceptance.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-50-proof-wall/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane/validation/proof-wall
cargo test
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/drift/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Artifacts written:

- `validation/proof-wall/cargo-test.txt`
- `validation/proof-wall/spec-generate-aligned-typescript.txt`
- `validation/proof-wall/spec-build-aligned-typescript.txt`
- `validation/proof-wall/spec-test-aligned-typescript.txt`
- `validation/proof-wall/spec-test-drift-typescript.txt`
- `validation/proof-wall/spec-test-unsupported-near-miss-typescript.txt`
- `validation/proof-wall/spec-test-example-apply-tax-typescript.txt`
- `validation/proof-wall/spec-test-molecule-negative-typescript.txt`
- `validation/proof-wall/spec-status-typescript.json`
- `validation/proof-wall/spec-export.json`
- `validation/proof-wall/family-prove-typescript.txt`
- proof-wall summary in `acceptance.md`
- `task-m45-50-proof-wall/started.json`
- `task-m45-50-proof-wall/done.json` or `blocked.json`

Blocked conditions:

- any proof-wall command fails
- TypeScript proof overwrites Rust proof or status mirrors Rust truth
- unsupported units reach Bun instead of failing at the bounded gate
- fixing the failure would require out-of-scope files or a widened milestone

Restart point if blocked:

- stop with the integrated branch intact
- record the blocker in `task-m45-50-proof-wall/blocked.json` and `acceptance.md`
- restart from `task-m45-f-integration` after the parent resolves the narrow in-scope proof drift

### `task-m45-60-closeout` - parent only

Purpose:

- fast-forward the proven integration branch onto `feat/m40-plus` and close the run only after scope, proof, and docs all agree

Owned files and artifacts:

- `M45_RUN_ROOT/acceptance.md`
- `M45_RUN_ROOT/closeout.md`
- `M45_RUN_ROOT/validation/closeout/**`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/task-m45-60-closeout/**`

Required commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane/validation/closeout
git rev-parse --abbrev-ref HEAD
test "$(git rev-parse --abbrev-ref HEAD)" = "feat/m40-plus"
git merge --ff-only ws/spec-m45-integration
git status --short
git diff --name-only 295f9c5..HEAD
git diff --stat 295f9c5..HEAD
```

Artifacts written:

- `closeout.md`
- `validation/closeout/final-git-status.short.txt`
- final acceptance note in `acceptance.md`
- `task-m45-60-closeout/started.json`
- `task-m45-60-closeout/done.json` or `blocked.json`

Blocked conditions:

- closeout is not running from the primary `feat/m40-plus` worktree
- final diff contains out-of-bounds authored files
- `feat/m40-plus` cannot fast-forward cleanly to `ws/spec-m45-integration`
- acceptance still relies on unresolved blockers or undocumented scope deviations

Restart point if blocked:

- stop before declaring M45 complete
- record the blocker in `task-m45-60-closeout/blocked.json`
- restart from `task-m45-f-integration` if code drift caused the issue, otherwise restart from `task-m45-40-merge-window`

## Scope-Boundary Checks

The parent must prove that the final diff stayed inside the frozen M45 authored surfaces plus derived proof output and parent-owned run state.

Required checks:

- capture baseline diff surfaces during `task-m45-00-baseline`
- capture per-lane name-only diffs during `task-m45-40-merge-window`
- capture final integrated name-only and stat diffs during `task-m45-f-integration`
- re-check final name-only and stat diffs during `task-m45-60-closeout`

Required commands:

```bash
git diff --name-only 295f9c5..HEAD
git diff --stat 295f9c5..HEAD
```

Boundary rule:

- every changed file in the final diff must be one of:
  - a frozen M45 authored surface
  - a derived proof artifact changed only by proof commands
  - a parent-owned `.runs/**` execution artifact

Blocked rule:

- any out-of-bounds authored diff is a blocked condition, not a follow-up idea
- do not silently absorb opportunistic refactors, broader TypeScript ambition, or unrelated cleanup into M45
- if a required fix truly needs a new authored surface, stop and reopen authority explicitly instead of normalizing it during integration

## Conflict-Control Rules

M45 has one serial freeze and then one real shared-code seam: generated-tree shape versus execution/proof routing.

Primary conflict flags:

- lane A and lane B both depend on the frozen generated helper filenames
- lane B is the only post-freeze lane allowed to touch `spec-cli/src/commands.rs`
- lane B depends on the parent-held validator and target-language freeze to be complete
- lane C depends on A and B for final error strings, status shape, and export shape
- lane C can easily expose code gaps that look like test or docs work but are not

Resolution rules:

- only the parent may change `spec-core/src/types.rs` or `spec-core/src/validator.rs` after `task-m45-15-worker-launch` opens
- only lane A may change `spec-core/src/typescript_backend.rs`, `spec-core/src/generator.rs`, or `spec-core/src/lib.rs`
- only lane B may change `spec-core/src/pipeline.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, or post-freeze `spec-cli/src/commands.rs`
- only lane C may change `spec-cli/tests/cli.rs`, `README.md`, or `CHANGELOG.md`
- if lane B needs a frozen filename, proof-field, or flag-surface change, the parent must reopen `task-m45-a1-target-freeze`
- if lane C needs a code change, it files a blocker instead of editing another lane's files
- once the parent starts `task-m45-35-merge-preview`, workers stop rebasing and hand off only summaries plus commit pointers

## Context-Control Rules

### Parent prompt rules

- Every worker prompt must include only:
  - the relevant `PLAN.md` excerpts
  - the lane's owned files
  - the frozen names from `target-freeze.json` that matter for that lane
  - the lane's required commands
  - the lane's acceptance criteria
  - the hard guards that matter for that lane
- Do not paste the full repo state into every worker prompt.
- Do not give any worker permission to edit `PLAN.md`, `ORCH_PLAN.md`, `.runs/**`, or another lane's files.
- Record every scope exception, freeze reopening, or merge-order deviation in `session-log.md` and `merge-log.md`.

### Worker prompt rules

- Work only in the assigned `spec-m45` worktree.
- Treat `PLAN.md` plus the frozen excerpts as authoritative.
- Keep edits inside the assigned files and their inline tests.
- Run only the lane-local commands unless the parent explicitly asks for broader proof.
- If a required fix crosses lane ownership, stop and report:
  - the blocked file
  - the missing symbol, frozen-name conflict, or boundary gap
  - the exact command that exposed it
- Return concise handoff notes:
  - changed files
  - commands run
  - remaining risk
  - whether the lane is safe to merge

## Tests And Acceptance

### Required proof wall

The parent integration lane must run this exact M45 wall before closeout:

```bash
cargo test
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/drift/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

### Mandatory non-goal checks

These are release-critical and must be covered by tests or command-path assertions even though they are not separate proof-wall commands:

- `spec validate` still has no TypeScript target support
- `.test.spec --target-language typescript` fails before Bun runs
- units with non-empty `deps` fail before Bun runs
- units outside `function.arithmetic_leaf.monotone_up.v1` fail before Bun runs
- unsupported `local_tests.expect` grammar fails before generation or Bun
- `status --target-language typescript` reports `untested` when TS proof is absent

### Lane-local regression expectations

- target-freeze lane proves flag-surface and bounded-eligibility stability
- generation lane proves emitted TypeScript tree and bounded runtime helper emission
- execution and proof lane proves Bun routing plus additive proof storage and read-side selection
- acceptance and docs lane proves command-path behavior and truthful user-facing wording

### Milestone acceptance checklist

M45 is complete only if all of the following are true:

1. `spec generate`, `spec build`, `spec test`, and `spec status` accept `--target-language`.
2. Rust remains the default path with no behavior regression.
3. `spec validate` does not gain TypeScript target support.
4. `spec generate --target-language typescript` emits a real TS tree for eligible monotone-up units.
5. The TS tree contains exactly one generated runtime helper and one generated local-test harness per output root.
6. `spec build --target-language typescript` uses Bun.
7. `spec test --target-language typescript` uses Bun and refreshes only `target_proofs.typescript`.
8. Rust proof remains untouched by TypeScript execution.
9. `spec status --target-language typescript` reads only TS proof and never mirrors Rust.
10. `spec export` carries additive target-proof data honestly.
11. Units outside the bounded lane fail before Bun runs.
12. `.test.spec --target-language typescript` fails with a stable unsupported message.
13. No wrapper, molecule, seam, dependency-bearing, or validate-target widening lands.
14. `README.md` and `CHANGELOG.md` describe only the bounded lane that actually landed.
15. The proof wall passes.

## Assumptions

- The parent launches from the current baseline branch `feat/m40-plus`, with `HEAD` at or ahead of `295f9c5`, and uses `295f9c5` only as the fixed diff anchor.
- The current M45 `PLAN.md` remains the only scope authority during execution.
- The repo can support disposable worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m45/`.
- Bun is available by the time the proof wall runs, or missing-Bun messaging is explicit enough that the failure is actionable.
- Existing monotone-up fixtures and ecommerce example sources are already good enough authority inputs for M45 unless the parent explicitly reopens them.
- The late acceptance/docs lane can safely fork from the parent-created merge preview commit without needing additional code ownership.
- No schema bump is required beyond additive `target_proofs`.

## Completion Summary

This orchestration plan keeps the only dangerous scope decisions in the parent lane:

- freeze the target-language and bounded-lane contract once
- let generation and execution/proof lanes parallelize against that freeze
- merge them into one preview commit before tests and docs harden the surface
- rerun the exact M45 proof wall locally
- fast-forward `feat/m40-plus` only after proof, scope, and docs all agree

That gives M45 the maximum safe parallelism the file ownership actually allows. The milestone stays bounded to one monotone-up, zero-dependency, atom-only, Bun-only TypeScript lane with additive target proof truth and no fake widening.
