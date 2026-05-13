# M53: Shared-Core Portability Adoption Closeout Implementation Plan

Status: **implementation plan**
Milestone: **M53**
Milestone family: **shared-core-portability**
Implementation readiness: **ready for bounded execution**
Plan scope: **close out command-facing adoption on the frozen `xtask/src/family/analysis_core/*` seam without changing stop-state semantics, CLI shape, JSON contracts, or artifact paths**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-12**

Supersedes:
- the prior repo-root M52 TypeScript execution plan previously maintained at this path
- the reviewed M53 design draft at `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md`

Primary source artifacts:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260512-201140.md`
- `TODOS.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `ORCH_PLAN.md`

Primary repo surfaces:
- `xtask/src/family/mod.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

## Executive Summary

M52 was the TypeScript widening.

M53 is not another semantic-family decision milestone, and it is not more TypeScript work. The current repo truth still says no new family move is authorized, the frozen `analysis_core/*` seam is already the semantic owner, and the remaining debt is the command-facing adoption story around that seam.

That means the honest next wedge is small and specific:

1. make `xtask/src/family/mod.rs` present `analysis_core` as the maintained owner surface
2. keep `decision_kernel.rs` and `helper_surface.rs` only as explicit compatibility passthroughs if they are still needed
3. tighten `xtask/src/lib.rs` proof so retained compatibility surfaces are tested as passthroughs, not mistaken for a second semantic home
4. rerun the frozen stop-state proof floor and confirm no CLI, JSON, or artifact-path behavior changed

If this milestone grows into seam extraction, repo-root plan churn, new family authorization, or generic portability work, it failed.

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` on 2026-05-12.

Commands run:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

Observed command truth:

- `collect_signals.sh`
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family recommend --format json`
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_summary.decision_status = "not_recommended"`
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks passed
- `cargo test -p xtask`
  - `156 passed; 0 failed`

Observed code truth:

- `xtask/src/family/mod.rs`
  - already exports `analysis_core`
  - still exports `decision_kernel` and `helper_surface`
  - already labels both shims as compatibility-only passthroughs
- `xtask/src/family/decision_kernel.rs`
  - is already a pure re-export shim over `analysis_core::decision_contract`
- `xtask/src/family/helper_surface.rs`
  - is already a pure re-export shim over `analysis_core::helper_surface`
- `xtask/src/family/recommend.rs`
  - already imports `analysis_core` directly
- `xtask/src/family/verify.rs`
  - already imports `analysis_core` directly
- `xtask/src/family/promotion_artifacts.rs`
  - already imports `analysis_core` directly
- `xtask/src/lib.rs`
  - already proves most decision semantics through `family::analysis_core::*`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
  - already states `analysis_core/*` is the live owner surface and shims are compatibility-only
- `docs/recommendation_corpus_expansion_program_v0.1.md`
  - already makes the same compatibility-only distinction

That is the actual state.

The semantic move is done. The remaining work is command-facing ownership closeout and proof tightening.

## Decision This Plan Makes

M53 authorizes exactly one bounded milestone:

1. preserve `analysis_core/*` as the only semantic owner surface
2. make the command-facing export story match that frozen boundary
3. keep any retained shim surface explicitly compatibility-only
4. prove that the stop-state contract and command-facing outputs do not move

M53 does not authorize:

- new family promotion work
- corpus run `1`
- second-language backend changes
- repo-root `ORCH_PLAN.md` rewrite
- artifact schema churn
- path ownership migration into `analysis_core`
- crate extraction
- generic shared-core portability beyond this seam closeout

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| semantic owner surface | `xtask/src/family/analysis_core/*` | reuse as-is |
| command consumers | `xtask/src/family/recommend.rs`, `verify.rs`, `promotion_artifacts.rs` | already adopted, do not rework |
| command dispatch surface | `xtask/src/lib.rs` | reuse, tighten proof only |
| compatibility shims | `xtask/src/family/decision_kernel.rs`, `xtask/src/family/helper_surface.rs` | reuse only as explicit passthroughs if retained |
| module presentation | `xtask/src/family/mod.rs` | tighten, do not redesign |
| maintainer stop-state docs | `docs/semantic_family_capability_corpus_guide_v0.1.md`, `docs/recommendation_corpus_expansion_program_v0.1.md` | likely reuse with at most surgical wording sync |
| proof floor | `cargo test -p xtask`, `collect_signals.sh`, `recommend`, `corpus-decision`, `verify-decision-contract` | reuse verbatim |

### Minimum Complete Slice

The minimum honest M53 slice is:

1. `mod.rs` presents `analysis_core` as the maintained surface and retains shims only as compatibility surfaces
2. retained shims remain trivial passthroughs and do not grow new logic
3. `xtask/src/lib.rs` proves both the maintained owner surface and any retained compatibility surface intentionally
4. the frozen stop-state proof floor reruns cleanly with unchanged outputs
5. maintainer docs are only touched if a specific command-facing surface still lies after the code closeout

Anything smaller is fake done.

Anything larger is fake scope growth.

### Scope Reduction Decision

Do not reopen the old "Lane B + Lane D" framing as if a broad consumer migration is still left.

Do not move logic into `analysis_core`.

Do not rewrite repo-root plans as part of this milestone.

Do not touch `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs` unless a proof failure shows that one of them still relies on the old presentation contract.

### Complexity Check

This milestone is below the overbuild threshold.

Expected primary write scope is four files:

- `xtask/src/family/mod.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/lib.rs`

Possible secondary write scope, only if truth requires it:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

No new service, no new crate, no new command, no new schema, no new distribution work.

### Search Check

No unfamiliar platform or new infrastructure is entering the repo.

- **[Layer 1]** reuse the existing Rust module structure, stop-state commands, and xtask proof harness
- **[Layer 1]** reuse the existing compatibility-only shim pattern rather than inventing a new facade layer
- **[Layer 3]** the eureka here is that the semantic migration is already done; the remaining work is presentation and proof, so the plan should stay much smaller than the original draft wording suggested

### TODOS Cross-Reference

Relevant deferred items in `TODOS.md` remain deferred:

- generalized multi-wedge decision layer
- cross-crate family-analysis shared core
- public semantic fingerprint fields

M53 must not silently consume any of those follow-ons.

### Completeness Check

Choose the complete version.

This milestone must land code truth, proof truth, and any necessary maintainer truth together. Do not ship the shortcut where the export surface is cleaned up but proof never directly exercises the retained compatibility path, and do not ship the shortcut where code is right but command-facing docs still teach the old ownership story.

### Distribution Check

No new distributable artifact is introduced in M53.

This is an internal behavior and proof-topology cleanup inside the existing `xtask` binary and repo docs. Existing build and release infrastructure is sufficient.

### Locked Plan Decisions

These are frozen for M53:

1. `analysis_core/*` remains the only semantic owner surface.
2. `decision_kernel.rs` and `helper_surface.rs` are either compatibility-only passthroughs or are removed entirely with proof coverage updated in the same change.
3. command names, flags, JSON schemas, latest-artifact paths, and write behavior stay unchanged.
4. `recommend`, `corpus-decision`, and `verify-decision-contract` must preserve current stop-state semantics exactly.
5. repo-root `PLAN.md` is the only authority artifact being rewritten in this milestone; `ORCH_PLAN.md` is not reopened here.
6. no logic moves into or out of `analysis_core/*` beyond import-path or re-export presentation adjustments needed for truthfulness.

### Abort And Re-scope Triggers

Stop and re-scope if any of these become necessary:

1. a retained consumer outside the expected shim surfaces still depends on historical exports in a way that forces broader module surgery
2. command behavior, JSON output, or latest-artifact paths need to change to complete the adoption cleanup
3. docs require broad historical rewrites rather than narrow wording sync
4. a proposed change starts to extract shared-core logic into a new crate or new module layer
5. the stop-state proof floor changes from `stop / no_actionable_candidate / record_stop_without_new_milestone`

## Target End State

After M53, the repo must tell one consistent story:

- semantic ownership lives in `xtask/src/family/analysis_core/*`
- the command-facing module surface reflects that truth
- any retained shim exists only as a compatibility passthrough
- `xtask/src/lib.rs` proves both the owner surface and the compatibility promise intentionally
- maintainer docs no longer leave room to read shims as live semantic owners
- the family-analysis lane still says stop

## Architecture Review

### Architecture Delta

```text
CURRENT
  analysis_core/*
    -> semantic owner surface
    -> consumed directly by recommend.rs
    -> consumed directly by verify.rs
    -> consumed directly by promotion_artifacts.rs
    -> consumed directly by most xtask proof tests

  mod.rs
    -> exports analysis_core
    -> exports decision_kernel shim
    -> exports helper_surface shim

TARGET M53
  analysis_core/*
    -> still the only semantic owner

  mod.rs
    -> makes analysis_core the obvious maintained seam
    -> keeps shims only if compatibility is still required

  xtask/src/lib.rs
    -> proves owner-surface behavior directly
    -> proves retained shims are passthrough-only

  docs
    -> only updated if they still imply shim ownership after code closeout
```

### Component Boundaries

`xtask/src/family/analysis_core/mod.rs`
- remains the seam-local export surface for semantic ownership
- is not the place to add new command plumbing

`xtask/src/family/mod.rs`
- owns command-facing module presentation
- is the primary code surface M53 is allowed to reinterpret

`xtask/src/family/decision_kernel.rs`
- may only re-export `analysis_core::decision_contract`
- must not accumulate wrapper logic, aliases, or policy branching

`xtask/src/family/helper_surface.rs`
- may only re-export `analysis_core::helper_surface`
- must not become a second semantic owner

`xtask/src/lib.rs`
- owns the proof wall
- must prove the maintained owner story and the retained compatibility story separately

`docs/semantic_family_capability_corpus_guide_v0.1.md`
`docs/recommendation_corpus_expansion_program_v0.1.md`
- are maintainer teaching surfaces only
- may be touched only when they would otherwise contradict the code after M53

### Realistic Failure Scenarios

| Codepath | Realistic failure | Why it matters | Planned protection |
| --- | --- | --- | --- |
| `xtask/src/family/mod.rs` | a shim remains exported in a way that still looks first-class | maintainers keep reading two semantic homes into the codebase | tighten export presentation and add proof around retained compatibility imports |
| `xtask/src/family/decision_kernel.rs` | passthrough re-export drifts from `analysis_core::decision_contract` | downstream compile paths still work but semantics fork | add direct parity tests through the shim path |
| `xtask/src/family/helper_surface.rs` | passthrough re-export drifts from `analysis_core::helper_surface` | same false dual-ownership risk | add direct parity tests through the shim path |
| `xtask/src/lib.rs` | tests only exercise owner paths and never the retained compatibility surface | a future refactor can silently break the promised compat surface | add explicit compatibility-surface regression coverage |
| proof commands | cleanup mutates stop-state outputs while code still compiles | milestone accidentally spends the wrong contract | rerun the full stop-state proof floor as mandatory acceptance |
| docs | wording drifts back to shim ownership language | maintainers get a silent but durable false model | touch docs only if needed and review wording against code after the code diff lands |

### Security And Blast-Radius Contract

No new external surface is added.

The blast radius is local to:

- module exports
- compatibility shims
- xtask proof tests
- maintainer-facing ownership wording

The worst-case failure is not a production security incident. It is governance drift: the repo starts teaching two ownership surfaces again, or a compatibility promise quietly breaks. That is why proof, not refactor cleverness, is the center of M53.

## Code Quality Review

### Guardrails

1. Do not add new semantic logic to the shim files.
2. Do not add a new abstraction layer to describe compatibility.
3. Do not spread ownership comments across unrelated files when one export surface and one proof wall are enough.
4. Prefer explicit re-export and explicit tests over helper macros or generic contract harnesses.
5. Keep the diff minimal. This is adoption closeout, not cleanup theater.

### Naming And Structure Requirements

- if comments change, they must say `compatibility-only` or `semantic owner` plainly
- if `mod.rs` order or grouping changes, it must make the maintained surface obvious without changing module names
- if tests are added, they must name the distinction clearly:
  - owner-surface proof
  - compatibility-surface proof

### Diagram Maintenance

If any nearby ASCII diagram comments are introduced or updated in `xtask/src/lib.rs`, they become part of the change and must reflect the retained compatibility contract accurately. Do not leave a stale topology diagram behind.

## Test Review

### Test Framework Detection

This repo is Rust-first for the affected surface.

Authoritative test command for this milestone:

```bash
cargo test -p xtask
```

Command-level acceptance surfaces:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/analysis_core/*
    ├── [★★★ TESTED] stop-state tuple + decision derivation + helper-surface classifier
    └── [★★★ TESTED] proof fingerprints and parity checks

[+] xtask/src/family/recommend.rs / verify.rs / promotion_artifacts.rs
    ├── [★★★ TESTED] direct owner-surface consumption already exercised by cargo tests
    └── [★★★ TESTED] command outputs pinned by current proof floor

[+] xtask/src/family/mod.rs
    ├── [★★  TESTED] compile surface indirectly covered by cargo tests
    └── [GAP]        explicit regression that retained shims are compatibility-only exports, not peer owner surfaces

[+] xtask/src/family/decision_kernel.rs
    ├── [★★  TESTED] indirect semantic behavior through owner-path tests
    └── [GAP]        direct passthrough parity coverage through shim import path

[+] xtask/src/family/helper_surface.rs
    ├── [★★  TESTED] indirect semantic behavior through owner-path tests
    └── [GAP]        direct passthrough parity coverage through shim import path

[+] xtask/src/lib.rs proof wall
    ├── [★★★ TESTED] stop-state command semantics
    └── [GAP]        explicit compatibility-contract regression block

─────────────────────────────────
COVERAGE: 6/10 paths fully covered today (60%)
QUALITY:  ★★★: 5  ★★: 3  GAP: 4
GAPS: 4 direct compatibility-surface regressions need to be added or tightened
─────────────────────────────────
```

### Maintainer Workflow Coverage

```text
MAINTAINER FLOW COVERAGE
===========================
[+] Stop-state validation workflow
    ├── [★★★ TESTED] collect_signals -> recommend -> corpus-decision -> verify-decision-contract
    └── [★★★ TESTED] cargo test -p xtask

[+] Compatibility promise workflow
    ├── [GAP] maintainers import retained shim surfaces after M53
    └── [GAP] proof explicitly distinguishes owner surface from compat surface

[+] Documentation truth workflow
    ├── [★★  REVIEWED] docs already align today
    └── [GAP] if docs change, final diff review must verify they still say compatibility-only
```

### Required Test Additions

M53 must add or tighten the following tests in `xtask/src/lib.rs`:

1. direct parity coverage through `family::decision_kernel::*` for the retained decision-contract surface
2. direct parity coverage through `family::helper_surface::*` for the retained helper-surface contract
3. one proof block that imports both `family::analysis_core::*` and any retained shim surface intentionally, so the compatibility promise is explicit and future breakage is loud
4. keep the existing command proof floor unchanged and rerun it after the code diff lands

No new CLI command tests are required unless export-surface changes unexpectedly force them.

### Failure-Mode Coverage Matrix

| Failure mode | Test covers it? | Error handling exists? | User sees clear failure? | Critical gap? |
| --- | --- | --- | --- | --- |
| shim export removed or renamed accidentally | planned after M53 test additions | compile/test failure | yes | no |
| shim re-export drifts from owner function | planned after M53 test additions | test failure | yes | no |
| stop-state output mutates silently | yes, via proof floor rerun | verify command + JSON comparison | yes if commands are rerun | no |
| docs drift back to owner-shim ambiguity | manual diff review only | review-only | otherwise silent | no, but keep docs scope narrow |

### Test Plan Artifact

QA-facing artifact for this milestone:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260512-201807.md`

That artifact is generated alongside this plan and should be the primary input for any `/qa` or `/qa-only` pass on M53.

## Performance Review

This milestone does not change product runtime performance.

The only meaningful performance surface is maintainer proof throughput.

What matters:

- do not add new artifact scans or new command hops
- do not rerun the same xtask proof commands redundantly inside tests
- keep acceptance commands sequential in one worktree, because baseline runs already showed occasional package-cache and build-dir lock waiting

No caching or architecture changes are justified here. Boring is correct.

## In-Scope Files

Primary write scope:

- `xtask/src/family/mod.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/lib.rs`

Allowed only if truth requires it:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

## Out-Of-Scope Files

Do not touch unless a blocker proves this plan wrong:

- `ORCH_PLAN.md`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- artifact schema types
- repo-root semantic roadmap docs beyond the two maintainer references named above
- TypeScript execution surfaces

## Implementation Plan

### Step 1: Freeze the current proof floor

Before any edits, capture the exact baseline outputs for:

- `collect_signals.sh`
- `cargo xtask family recommend --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family verify-decision-contract --format json`
- `cargo test -p xtask`

Acceptance:

- baseline command outputs are recorded in the implementation notes or closeout
- no planned code edit starts without a known-good stop-state baseline

### Step 2: Tighten the command-facing export surface

Update `xtask/src/family/mod.rs` so the maintained surface is unmistakable.

Concretely:

- keep `analysis_core` grouped and presented as the maintained owner surface
- retain `decision_kernel` and `helper_surface` only if compatibility still matters
- if retained, keep the compatibility-only comment direct and unambiguous

Acceptance:

- `mod.rs` tells the right ownership story at a glance
- no module names, command names, or import contracts change unless a retained shim is intentionally removed and proven safe in the same milestone

### Step 3: Close out the shim surface without widening it

Update `xtask/src/family/decision_kernel.rs` and `xtask/src/family/helper_surface.rs` only as needed to keep the compatibility contract honest.

Concretely:

- preserve pure passthrough behavior
- remove any stale wording that sounds like ownership
- do not add helper functions, wrappers, or policy logic

Acceptance:

- each shim is still obviously a passthrough or is removed entirely with proof updated
- no semantic code moves out of `analysis_core/*`

### Step 4: Tighten the proof wall

Update `xtask/src/lib.rs` to prove the post-M53 ownership contract intentionally.

Concretely:

- add direct compatibility-path tests for retained shim exports
- keep existing owner-surface tests intact
- keep command-proof assertions centered on unchanged stop-state semantics

Acceptance:

- `cargo test -p xtask` still passes
- retained compatibility surfaces are covered directly, not only indirectly
- future shim drift would fail loudly

### Step 5: Sync maintainer docs only if the code diff requires it

Inspect:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Only edit them if the final code diff would otherwise make their wording false or incomplete.

Acceptance:

- docs remain narrow and accurate
- no broad historical cleanup begins here

### Step 6: Rerun the locked proof floor

After the code diff lands, rerun:

```bash
cargo test -p xtask
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Acceptance:

- stop-state semantics remain exactly:
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- `overall_verdict = "pass"` remains true
- no CLI, JSON, or latest-artifact path drift appears

## Worktree Parallelization Strategy

This milestone has limited but real parallelization.

The code path is mostly sequential. The doc sync, if needed, can split off after the module/export contract is settled.

### Dependency Table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| baseline proof capture | `xtask/`, `.semantic-family-artifacts/`, `.agents/skills/next-milestone/scripts` | — |
| export-surface closeout | `xtask/src/family/` | baseline proof capture |
| shim-proof tightening | `xtask/src/`, `xtask/src/family/` | export-surface closeout |
| maintainer doc sync, only if needed | `docs/` | export-surface closeout |
| final acceptance rerun | `xtask/`, `.semantic-family-artifacts/` | shim-proof tightening, maintainer doc sync |

### Parallel Lanes

- `Lane A`: baseline proof capture -> export-surface closeout -> shim-proof tightening -> final acceptance rerun
- `Lane B`: maintainer doc sync, only if needed, after export-surface closeout

### Execution Order

1. run baseline proof capture serially
2. land the `xtask/src/family/` contract in one worktree first
3. once that contract is stable, run:
   - `Lane A` test/proof tightening
   - `Lane B` doc sync, only if the code diff requires it
4. merge both back together
5. rerun the full locked proof floor serially

### Conflict Flags

- `Lane A` and `Lane B` are safe in parallel only because one is code and one is docs
- do not split `mod.rs`, shim files, and `xtask/src/lib.rs` across multiple code worktrees, because the ownership contract and its proof wall are tightly coupled
- do not run the acceptance cargo commands in parallel across worktrees, because baseline runs already showed package-cache and build-dir lock contention

## NOT In Scope

- repo-root `ORCH_PLAN.md` rewrite
- family-promotion authorization changes
- corpus run `1`
- second-language backend work
- path or artifact-schema redesign
- `analysis_core/*` extraction into a new crate
- generic compatibility cleanup outside the named family-analysis seam
- broad documentation archaeology

## Acceptance Criteria

M53 is complete only if all of the following are true:

1. `analysis_core/*` remains the only semantic owner surface.
2. `xtask/src/family/mod.rs` presents that owner surface cleanly.
3. any retained shim file is visibly compatibility-only and behaviorally trivial.
4. `xtask/src/lib.rs` directly proves any retained compatibility contract.
5. `cargo test -p xtask` passes.
6. `collect_signals.sh`, `recommend`, `corpus-decision`, and `verify-decision-contract` all preserve the current stop-state outputs.
7. no command names, flags, JSON schemas, latest-artifact paths, or write behavior change.
8. docs were either left alone because they were already truthful, or were updated surgically to stay truthful.

## Definition Of Done

The milestone is done when a maintainer can read `xtask/src/family/mod.rs`, inspect the shim files, run the locked proof floor, and come away with exactly one conclusion:

`analysis_core/*` owns the semantics, the stop-state contract still says stop, and any retained shim is just compatibility glue.
