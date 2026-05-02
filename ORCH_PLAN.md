# M28 Orchestration Plan

Status: **execution contract**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Primary branch baseline: **`feat/corpus-expansion`**
Run root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m28_shared_core`**
Last rewritten: **2026-05-02**

## Summary

This file is the execution contract for M28.

M28 starts from the truthful post-M27.9B baseline and does one bounded thing:

- extract one shared backend-execution runtime boundary in `spec-core`
- route current seam consumers through it
- preserve current Rust status/export read-side truth
- audit `xtask` proof and coverage surfaces without rewriting them by habit

Required final outcome:

- one new `spec-core/src/backend_execution.rs` module exists
- `passport`, `escape_hatch`, `semantic_review`, and `export` all consume it
- targeted `spec-cli` regressions are green
- frozen `xtask` coverage output remains no-drift against the current baseline
- no recommendation or corpus semantics change
- no second-language implementation starts
- `xtask` either stays read-only or triggers a documented halt/split

The parent agent is the sole integrator and sole owner of run-state truth.
Workers may only edit assigned files in assigned worktrees.

## Hard Guards

1. Runtime implementation scope is closed to these files:
   - `spec-core/src/backend_execution.rs`
   - `spec-core/src/passport.rs`
   - `spec-core/src/escape_hatch.rs`
   - `spec-core/src/semantic_review.rs`
   - `spec-core/src/lib.rs`
   - `spec-cli/src/commands.rs`
   - `spec-core/src/export.rs`
   - `spec-cli/tests/m14_regressions.rs`
   - `spec-cli/tests/cli.rs`

2. Planning/orchestration scope is closed to:
   - `PLAN.md`
   - `ORCH_PLAN.md`

3. `xtask/src/family/*` is audit-only.
   If any `xtask` file must change to make M28 green, halt and split a follow-on
   plan. Do not improvise the scope expansion.

4. No new command family may be added.

5. No recommendation-analysis, corpus, or `money/round` governance behavior may
   change.

6. No second-language packet, lowering, runtime, or fixture work may land.

7. No validator policy widening is allowed.
   `methods[].lowering.rust.body` and `backends.rust.derives` remain the only
   Rust-specific seam escape hatches.

8. `cargo xtask family coverage --format json` must remain byte-stable against
   `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
   unless recommendation semantics intentionally change.

## Canonical Run-State

Canonical run root:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m28_shared_core`

Parent-owned files:

- `.runs/m28_shared_core/tasks.json`
- `.runs/m28_shared_core/session-log.md`
- `.runs/m28_shared_core/baseline.json`
- `.runs/m28_shared_core/integration-state.json`
- `.runs/m28_shared_core/final-proof.json`
- `.runs/m28_shared_core/blocked.json`
- `.runs/m28_shared_core/closeout.md`

Parent-owned directories:

- `.runs/m28_shared_core/diagnostics/parent/`
- `.runs/m28_shared_core/diagnostics/ws_runtime/`
- `.runs/m28_shared_core/diagnostics/ws_docs/`
- `.runs/m28_shared_core/diagnostics/ws_audit/`
- `.runs/m28_shared_core/handoffs/ws_runtime/`
- `.runs/m28_shared_core/handoffs/ws_docs/`
- `.runs/m28_shared_core/handoffs/ws_audit/`

## Worker Lanes

### Lane A - Runtime boundary extraction

Branch:

- `codex/m28-runtime-boundary`

Worktree:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m28_runtime_boundary`

Owned files:

- `spec-core/src/backend_execution.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs`
- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
```

Required handoff package:

- `result.json`
- `handoff.md`
- `commit.txt`
- `done.ok`

### Lane B - Plan/orchestration docs

Branch:

- `codex/m28-docs`

Worktree:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m28_docs`

Owned files:

- `PLAN.md`
- `ORCH_PLAN.md`

Required commands:

- none beyond consistency checks and diff review

Required handoff package:

- `result.json`
- `handoff.md`
- `commit.txt`
- `done.ok`

### Lane C - xtask audit only

Branch:

- `codex/m28-xtask-audit`

Worktree:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.worktrees/m28_xtask_audit`

Owned files:

- none by default

Read-only targets:

- `xtask/src/family/report.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs`
- any prove/certify artifact fixtures or tests needed to support the audit

Required commands:

```bash
rg -n "Rust|rust|target_language|lowering|escape-hatch|backend-only" xtask/src/family xtask/src/lib.rs
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Required handoff package:

- `result.json`
- `handoff.md`
- `done.ok`

If the audit concludes that an `xtask` edit is necessary, the handoff must say
so explicitly and the parent must halt integration pending a follow-on plan.

## Parallelization Contract

Launch order:

1. Launch Lane A and Lane B in parallel.
2. Launch Lane C in parallel as read-only audit.
3. Merge Lane B at any time.
4. Merge Lane A only after its proof loop is green.
5. Process Lane C before final closeout:
   - if no leak, record read-only outcome
   - if leak, halt and split follow-on

Conflict profile:

- Lanes A and B are disjoint
- Lane C must remain read-only
- Any write in Lane C is a scope break, not a merge conflict to resolve casually

## Parent Execution Sequence

### Phase 1 - Baseline freeze

Parent records:

- current branch and HEAD SHA
- current M27.9B proof references
- current coverage baseline path and checksum target
- diffstat before any M28 edits
- whether the design authority exists and matches the active plan

### Phase 2 - Worker dispatch

Dispatch:

- Lane A runtime worker
- Lane B docs worker
- Lane C audit worker

### Phase 3 - Runtime merge

Merge Lane A only if:

- all required commands exit `0`
- handoff explains the shared boundary clearly
- handoff states how `status` and `export` parity was preserved
- no out-of-scope file drift exists

### Phase 4 - Docs merge

Merge Lane B if:

- `PLAN.md` and `ORCH_PLAN.md` align
- no stale M27.9B instructions remain

### Phase 5 - Audit disposition

Read Lane C handoff.

If audit result is:

- `no_leak_found`
  - record that `xtask` remained read-only for M28
- `leak_found_follow_on_required`
  - write `blocked.json`
  - stop before final closeout

### Phase 6 - Final proof

Parent final proof loop:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Optional full confirmation if runtime scope expanded within `spec-cli`:

```bash
cargo test -p spec-cli -- --color never
```

## Acceptance Gate

The run is accepted only if all of the following are true:

1. Lane A introduced exactly the shared runtime boundary the plan requires
2. Lane B kept the planning docs aligned
3. Lane C either:
   - found no `xtask` leak, or
   - forced a documented halt before any `xtask` code mutation
4. final proof loop is green
5. frozen coverage output is no-drift against the baseline
6. no recommendation, corpus, or second-language drift occurred

## Halt Conditions

Halt immediately if any of these happen:

- `xtask` edit becomes necessary
- validator policy widening appears necessary
- recommendation/corpus semantics drift
- runtime proof loop requires changes outside the closed file contract
- second-language work starts sneaking in through fixtures or packet scaffolds

## Closeout Requirements

`closeout.md` must state:

- whether `xtask` remained read-only
- exact proof commands run
- coverage diff result against the frozen baseline
- whether CLI truth surfaces changed in wording or only in internals
- whether `status` and `export` stayed aligned for the closeout fixtures
- whether the exact M29 closeout probe passed or failed
- whether any follow-on portability work was discovered but deferred

`final-proof.json` must record:

- command list
- exit codes
- final artifact paths, if any
- final verdict
