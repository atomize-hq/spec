# M40+ Orchestration Plan

Status: **authoritative execution contract for the Shared-Core Portability Follow-On**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**
Milestone: **Shared-Core Portability Follow-On for the Family-Analysis Decision Seam**
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**
Base branch: **`main`**
Working branch: **`feat/m40-plus`**
Last rewritten: **`2026-05-09`**
Canonical run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_shared_core_portability_follow_on`**

## Summary

- `PLAN.md` is the sole authority. This file is the execution runbook for that authority.
- The milestone is a bounded internal seam extraction under `xtask/src/family/analysis_core/`.
- The parent agent remains the sole integrator, gate owner, acceptance owner, and closeout author.
- Default execution is fully sequential on `feat/m40-plus`.
- There is exactly one honest optional split window, and it opens only after the parent freezes the `analysis_core` helper-surface plus decision-contract API and reserves `recommend.rs`.
- If that freeze cannot stay stable, or if `recommend.rs` cannot remain parent-owned, the orchestration collapses back to sequential parent execution.

## Hard Guards

- No new crate.
- No new commands.
- No new artifact schema.
- Do not move `Path`, `fs`, repo-root logic, latest-artifact selection, stdout formatting, or atomic-write behavior into `analysis_core`.
- Do not widen into unrelated `xtask/src/family` cleanup.
- Do not touch unrelated family modules unless a direct compile failure proves it is required.
- Do not preserve `recommend.rs` as a semantic facade. Its semantic re-export block must be deleted by closeout.
- Workers never own gates, never redefine scope, and never merge directly into `feat/m40-plus`.
- `PLAN.md` wins over this file, memory, stale notes, and prior run artifacts if they disagree.
- `.runs/*` is execution evidence only. It is not authority and not a source of semantic truth.

## Locked Target Structure

```text
xtask/src/family/
  analysis_core/
    mod.rs
    helper_surface.rs
    decision_contract.rs
    proof_fingerprint.rs
  coverage.rs
  recommend.rs
  verify.rs
  promotion_artifacts.rs
  paths.rs
  mod.rs
```

## Locked Move Contract

- Move helper-surface types and functions from `helper_surface.rs` to `analysis_core/helper_surface.rs`.
- Move decision derivation from `decision_kernel.rs` to `analysis_core/decision_contract.rs`.
- Move `normalized_recommendation_proof_fingerprint` and `normalized_corpus_program_decision_proof_fingerprint` from `decision_kernel.rs` to `analysis_core/proof_fingerprint.rs`.
- Move `normalized_for_recommend_determinism` and `normalized_coverage_proof_fingerprint` from `coverage.rs` to `analysis_core/proof_fingerprint.rs`.
- Delete the semantic re-export block from `recommend.rs`.
- Add `pub mod analysis_core;` in `family/mod.rs`.

## Preserved Semantic Outputs

The final proof loop must preserve current branch truth:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- `helper_surface_not_promotable` remains the decisive blocker
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `required_next_action = "author_architecture_follow_on_plan"`

## Execution Topology

Canonical paths:

- `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- `WT_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus`
- `RUN_ROOT=$PRIMARY_ROOT/.runs/m40_plus_shared_core_portability_follow_on`

Worktree layout:

| Role | Branch | Worktree | Owner | Status |
|---|---|---|---|---|
| primary execution lane | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | parent | always authoritative |
| optional consumer lane | `codex/m40-plus-consumer` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/consumer` | worker `W1` or parent | starts only after Gate 20 |
| optional proof lane | `codex/m40-plus-proof` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/proof` | worker `W2` or parent | starts only after Gate 20 |
| optional staging lane | `codex/m40-plus-int` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/int` | parent only | optional rehearsal only |

Topology rules:

- The parent remains on `feat/m40-plus`.
- Optional worker branches fork from the exact `contract_freeze_commit` recorded in `analysis-core-freeze.json`.
- Worker branches never merge directly to `feat/m40-plus`.
- The parent cherry-picks or manually integrates worker diffs back through the parent-controlled lane.
- The optional `int` worktree is disposable staging only. Final accepted integration still lands through the parent on `feat/m40-plus`.
- If the split proves dishonest, close worker lanes and continue sequentially on `feat/m40-plus`.

## Canonical Run-State

`RUN_ROOT` is execution evidence only. The parent owns it.

Required run artifacts at kickoff:

- `baseline.json`
- `authority-freeze.json`
- `in-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`

Required run artifacts at API freeze:

- `analysis-core-freeze.json`

Required validation artifacts during execution:

- `validation/cargo-test-baseline.stdout.txt`
- `validation/cargo-test-post-core.stdout.txt`
- `validation/cargo-test-post-integration.stdout.txt`
- `validation/family-recommend.json`
- `validation/family-corpus-decision.json`
- `validation/family-verify-decision-contract.json`
- `validation/tr1-proof-fingerprint-notes.md`
- `validation/tr2-verify-parity-notes.md`
- `validation/tr3-promotion-artifacts-parity-notes.md`
- `validation/tr4-recommend-reuse-notes.md`
- `validation/tr5-purity-audit.md`
- `validation/diff-scope.txt`

Required run artifacts at closeout or block:

- `acceptance.md`
- `closeout.md`
- `blocked.json` on blocked termination

Minimum required contents:

- `baseline.json`
  - branch
  - HEAD SHA
  - dirty-state summary
  - in-scope file list checksum or captured path
  - whether `PLAN.md` differs from `HEAD`
- `authority-freeze.json`
  - authority path
  - hard guards
  - preserved semantic outputs
  - in-scope file set
- `analysis-core-freeze.json`
  - `contract_freeze_commit`
  - exported `analysis_core` symbol summary
  - parent-reserved files
  - whether temporary compatibility forwarders remain active for worker safety
- `blocked.json`
  - failed workstream
  - failed gate
  - branch
  - HEAD SHA
  - blocking evidence
  - restart point
  - whether split lanes were invalidated

## Queue And Gates

| Order | ID | Kind | Owner | Success outputs |
|---|---|---|---|---|
| 1 | `gate-m40p-00-baseline-freeze` | gate | parent | `baseline.json`, `validation/cargo-test-baseline.stdout.txt`, `run-state.json` |
| 2 | `gate-m40p-05-authority-freeze` | gate | parent | `authority-freeze.json`, `queue.json`, `tasks.json` |
| 3 | `task-m40p-10-seam-skeleton` | task | parent | source changes, `session-log.md` |
| 4 | `task-m40p-15-helper-decision-move` | task | parent | source changes, `session-log.md` |
| 5 | `gate-m40p-20-analysis-core-api-lock` | gate | parent | `analysis-core-freeze.json`, `validation/cargo-test-post-core.stdout.txt` |
| 6 | `task-m40p-30-consumer-direct-imports` | task | worker `W1` or parent | lane diff or integrated changes |
| 7 | `task-m40p-35-proof-fingerprint-move` | task | worker `W2` or parent | lane diff or integrated changes |
| 8 | `gate-m40p-40-parent-integration` | gate | parent | integrated tree, `validation/diff-scope.txt`, `validation/cargo-test-post-integration.stdout.txt` |
| 9 | `task-m40p-45-recommend-rewire-regressions` | task | parent | `recommend.rs` rewired, TR-4 and TR-5 landed |
| 10 | `gate-m40p-50-full-proof-loop` | gate | parent | proof-loop validation artifacts |
| 11 | `gate-m40p-55-purity-scope-audit` | gate | parent | `validation/tr5-purity-audit.md`, `acceptance.md` |
| 12 | `gate-m40p-60-closeout` | gate | parent | `closeout.md`, final `run-state.json` |

Queue rules:

- Gates never overlap.
- `task-m40p-30-consumer-direct-imports` and `task-m40p-35-proof-fingerprint-move` may overlap only after Gate 20 passes.
- If Gate 20 does not produce a stable API plus parent-reserved `recommend.rs`, there is no split. Continue sequentially under the parent.

## Workstream Plan

### WS-CORE (`feat/m40-plus`) — parent only, sequential

Workstream purpose:

- establish the seam
- move helper-surface and decision-contract semantics
- freeze the API honestly before any optional split

Task ownership:

| Task ID | Parent-owned files | Required outcome |
|---|---|---|
| `task-m40p-10-seam-skeleton` | `xtask/src/family/mod.rs`, `xtask/src/family/analysis_core/mod.rs` | exact module tree exists and compiles |
| `task-m40p-15-helper-decision-move` | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs`, `xtask/src/family/analysis_core/helper_surface.rs`, `xtask/src/family/analysis_core/decision_contract.rs` | helper-surface and decision-contract semantics move under `analysis_core` |
| `gate-m40p-20-analysis-core-api-lock` | parent may still touch any WS-CORE file | exported seam surface is frozen, parent reserves `recommend.rs`, optional worker boundaries are recorded |

Kickoff commands for Gate 00:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
rg --files xtask/src/family
cargo test -p xtask
```

Write kickoff artifacts:

- `baseline.json`
- `in-scope-files.txt`
- `validation/cargo-test-baseline.stdout.txt`

Authority freeze commands for Gate 05:

```bash
git rev-parse HEAD
rg -n "analysis_core|helper_surface_not_promotable|pivot_to_architecture_shared_core_follow_on|author_architecture_follow_on_plan|TR-1|TR-2|TR-3|TR-4|TR-5" PLAN.md
```

WS-CORE checkpoint commands after each code-moving task:

```bash
cargo test -p xtask
```

API-lock commands for Gate 20:

```bash
cargo test -p xtask
git rev-parse HEAD
```

API-lock acceptance:

- `analysis_core/mod.rs`, `analysis_core/helper_surface.rs`, and `analysis_core/decision_contract.rs` exist.
- Helper-surface and decision-contract exports are stable enough to freeze.
- `recommend.rs` is explicitly reserved to the parent.
- If the parent wants the optional split, compatibility forwarders may remain temporarily in `helper_surface.rs`, `decision_kernel.rs`, or `coverage.rs` only to keep worker branches compiling until WS-INT removes them.
- `analysis-core-freeze.json` records the exact exported symbols and whether those temporary forwarders remain.

### Split Preconditions

The optional split is valid only if all of the following are true at Gate 20:

- the parent has frozen the helper-surface plus decision-contract API in `analysis-core-freeze.json`
- the parent has reserved `xtask/src/family/recommend.rs`
- worker lanes can keep `cargo test -p xtask` green without editing `recommend.rs`
- any temporary compatibility forwarders are already in place and explicitly marked for parent removal in WS-INT

If any one of these is false, do not branch workers. Continue sequentially on `feat/m40-plus`.

### WS-CONSUMER (`codex/m40-plus-consumer`) — optional worker lane after Gate 20

Workstream purpose:

- rewire direct consumers to import shared semantics from `analysis_core`
- land TR-2 and TR-3
- avoid `recommend.rs`

Exact file ownership:

- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- tests in those files only

Forbidden files:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/analysis_core/mod.rs`

Lane start commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus
git worktree add -b codex/m40-plus-consumer /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/consumer <contract_freeze_commit>
```

Lane validation command:

```bash
cargo test -p xtask
```

Lane acceptance:

- `verify.rs` imports shared semantics directly from `analysis_core`
- `promotion_artifacts.rs` imports shared semantics directly from `analysis_core`
- TR-2 exists and passes
- TR-3 exists and passes
- lane does not touch `recommend.rs`

### WS-PROOF (`codex/m40-plus-proof`) — optional worker lane after Gate 20

Workstream purpose:

- move proof-fingerprint ownership under `analysis_core`
- land TR-1
- leave `recommend.rs` parent-owned

Exact file ownership:

- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/decision_kernel.rs`
- tests in those files only

Forbidden files:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/analysis_core/mod.rs`

Lane start commands:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus
git worktree add -b codex/m40-plus-proof /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/proof <contract_freeze_commit>
```

Lane validation command:

```bash
cargo test -p xtask
```

Lane acceptance:

- all three normalized proof helpers live in `analysis_core/proof_fingerprint.rs`
- TR-1 exists and passes
- any temporary compatibility forwarders are narrow and transitional only
- lane does not touch `recommend.rs`

### WS-INT (`feat/m40-plus`, optional `codex/m40-plus-int` staging) — parent only

Workstream purpose:

- integrate worker output or sequentially finish remaining work
- delete `recommend.rs` semantic re-export block
- rewire `recommend.rs`
- land TR-4 and TR-5
- run the full proof loop
- close the milestone

Parent-owned files in WS-INT:

- `xtask/src/family/recommend.rs`
- any in-scope file requiring final glue removal or compatibility-forwarder cleanup
- final regression tests and acceptance notes

Optional staging worktree creation, only if the parent wants a rehearsal branch:

```bash
mkdir -p /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus
git worktree add -b codex/m40-plus-int /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m40-plus/int <contract_freeze_commit>
```

Integration commands for Gate 40:

```bash
cargo test -p xtask
git rev-parse HEAD
git status --short
```

Final proof-loop commands for Gate 50:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

WS-INT acceptance:

- `recommend.rs` imports semantics directly from `analysis_core`
- the semantic re-export block is deleted
- any temporary compatibility forwarders are removed unless they remain as harmless internal shims explicitly allowed by the parent and still keep `recommend.rs` non-facade
- TR-4 exists and passes
- TR-5 is documented and passes as API review plus test where practical
- all four final commands are captured under `RUN_ROOT/validation/`

## Context-Control Rules

- The parent keeps only the following live artifacts in working context:
  - `PLAN.md`
  - `ORCH_PLAN.md`
  - `queue.json`
  - `analysis-core-freeze.json`
  - the latest diff summary
- Each worker receives only:
  - its exact owned file set
  - the relevant `PLAN.md` excerpt
  - the frozen `contract_freeze_commit`
  - required commands
  - forbidden files
  - required TR ownership
- Workers return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not write `RUN_ROOT/*`.
- The parent reviews narrow diffs and summaries only, not full worker transcripts.
- Close worker lanes immediately after integration or cancellation.
- If the parent changes frozen API wording after Gate 20, all worker lanes become stale.

## Blocked And Restart Semantics

- Failure in `WS-CORE` before Gate 20 restarts from the last passing WS-CORE gate.
- If Gate 20 cannot freeze a stable `analysis_core` API, there is no split. Continue sequentially on `feat/m40-plus`.
- If worker lanes have started and the parent discovers `recommend.rs` must change for either lane to compile, the split was dishonest. Close `WS-CONSUMER` and `WS-PROOF` immediately and resume sequentially from Gate 20 under the parent.
- If worker lanes have started and the exported `analysis_core` API changes, all worker lanes become stale. Restart from `task-m40p-15-helper-decision-move`.
- If a worker touches a forbidden file, invalidate that lane and restart the affected work under the parent from the last passing gate.
- If one worker lane is valid and the other proves dishonest, the parent may integrate the valid lane and finish the remaining work sequentially only if Gate 40 has not yet been declared passed.
- If `PLAN.md` changes after Gate 05, restart from Gate 00.
- If `feat/m40-plus` HEAD changes after Gate 20 but before worker integration, invalidate worker lanes and restart from Gate 20.
- If TR-1 fails, reopen WS-PROOF only.
- If TR-2 or TR-3 fails, reopen WS-CONSUMER only.
- If TR-4 or TR-5 fails, reopen WS-INT only.
- If any of the preserved semantic outputs change during Gate 50, stop and reopen WS-INT. Do not accept semantic drift as a refactor side effect.

## Tests And Acceptance

Mandatory regression guards:

| ID | Location | Required assertion |
|---|---|---|
| TR-1 | `analysis_core/proof_fingerprint.rs` tests | moved coverage fingerprint returns the same hash as before the move |
| TR-2 | `verify.rs` tests | verify parity still derives the same basis snapshot and follow-on decision after direct-import rewiring |
| TR-3 | `promotion_artifacts.rs` tests | artifact validation still matches the derived contract after direct-import rewiring |
| TR-4 | `recommend.rs` tests | unchanged semantic fingerprints still reuse prior bytes and avoid rewrite churn |
| TR-5 | `analysis_core` API review plus test where practical | seam surface remains pure and path/fs-free |

Mandatory final commands:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Mandatory final acceptance:

- `analysis_core/` exists with exactly:
  - `mod.rs`
  - `helper_surface.rs`
  - `decision_contract.rs`
  - `proof_fingerprint.rs`
- helper-surface durable-hold classification still yields the exact frozen tuple
- corpus-program decision derivation still yields the same architecture-follow-on contract
- coverage, recommendation, and corpus-decision proof fingerprints all live behind one shared seam
- `recommend.rs` still owns CLI, repo-path, latest-artifact, and write behavior
- `recommend.rs` no longer re-exports semantic helpers
- `verify.rs` and `promotion_artifacts.rs` import semantics directly from `analysis_core`
- `analysis_core` exports accept typed in-memory values, not workspace-root, `Path`, or file-system inputs
- the four final validation commands pass
- the preserved semantic outputs match exactly
- `validation/diff-scope.txt` shows no touched files outside the milestone surface

## Assumptions

- The optional worker split is valid only if temporary compatibility forwarders can keep `cargo test -p xtask` green without giving workers ownership of `recommend.rs`.
- The `WT_ROOT` paths may be created if absent; the parent worktree at `PRIMARY_ROOT` remains the sole authoritative lane throughout.

## Critical assumptions

- Temporary compatibility forwarders are allowed only as transitional execution scaffolding and must not survive closeout in a way that preserves `recommend.rs` as a semantic facade.
- The optional `codex/m40-plus-int` worktree is staging only; final accepted integration still lands through the parent on `feat/m40-plus`.
