# M40 - Family-Analysis Shared-Core Follow-On Authority Plan

Status: **authority plan**
Milestone family: **shared-core-portability**
Implementation readiness: **authority artifact ready for review**
Next artifact kind: **authority_plan**
Autoplan ready: **yes**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-07**
Supersedes: **M39 - Verification Consumer Probe After M38**

## Executive Verdict

The repo is still not authorized to extract a shared family-analysis core.

The repo is authorized to freeze the exact contract that decides when that extraction becomes honest. That is what M40 does.

M40 is a plan artifact, not an implementation milestone. If this file causes code motion beyond the plan itself, the milestone has failed its own boundary.

## What M40 Freezes

M40 freezes four things:

1. the exact candidate seam under consideration
2. the exact evidence that upgrades the repo from planning-authorized to implementation-authorized
3. the exact surfaces that must remain local even if extraction is later approved
4. the exact set of allowed next milestones, with everything else explicitly blocked

## Live Validated Basis

Revalidated on the live `feat/corpus-expansion` tree on 2026-05-07.

Commands run:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Observed truth:

- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks passed
- `cargo xtask family corpus-decision --format json`
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "not_recommended"`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `required_next_action = "author_architecture_follow_on_plan"`
- `cargo test -p xtask`
  - `136 passed; 0 failed`
  - the only current noise is two dead-code warnings in [`xtask/src/family/helper_surface.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs)

Current milestone truth:

- corpus run `1` remains unspent
- the helper-surface wedge remains a durable non-promotable hold
- [`xtask/src/family/verify.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs) is now a real standing consumer of the bounded decision contract
- no command output authorizes code extraction yet

### What M39 actually proved

M39 proved three narrow things:

- the verifier command is now a real in-tree consumer, not a hypothetical future consumer
- the helper-surface follow-on tuple is frozen tightly enough to validate in read-side form
- standing proof walls can adopt the verifier without breaking parity or regressions

### What M39 did not prove

M39 did not prove any of the things that would justify extraction:

- it did not prove that `verify.rs` alone is enough reuse pressure
- it did not prove that command plumbing belongs in the seam
- it did not prove a cross-crate consumer exists
- it did not prove public fingerprint fields are required
- it did not prove that a new shared crate would reduce complexity rather than spread it

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | M40 decision |
|---|---|---|
| durable helper-surface classifier | [`xtask/src/family/helper_surface.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs) | keep frozen as the classifier contract |
| bounded decision derivation | [`xtask/src/family/decision_kernel.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs) | keep as the local semantic source of truth |
| verifier consumer | [`xtask/src/family/verify.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs) | treat as the proof that consumer pressure is real |
| artifact schema and parity contract | [`xtask/src/family/promotion_artifacts.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs) | keep as the read-side contract boundary |
| command and path plumbing | [`xtask/src/family/mod.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs), [`xtask/src/family/paths.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/paths.rs), [`xtask/src/lib.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs) | explicitly keep out of the shared seam |
| trigger inventory | [`TODOS.md`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md) post-M37 follow-ups | keep as the canonical trigger vocabulary |
| latest closeout evidence | [`.runs/m39_verification_consumer_probe/closeout.md`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe/closeout.md) | use as proof that the third-consumer claim is real |

### Minimum complete M40

M40 is complete only if this file contains all of the following, with no ambiguity:

1. one concrete seam definition
2. one trigger table with exact authorization conditions
3. one proof floor tied to live commands
4. one M41 gate with exact allowed outcomes
5. one explicit non-goal block that prevents stealth implementation
6. one future execution split for the first authorized implementation milestone

### Complexity and scope result

This milestone is intentionally small and explicit.

- Files intentionally changed in M40: `PLAN.md`
- New classes or services: `0`
- New artifact type: `0`
- Distribution work required: none, because M40 is an authority artifact, not a shippable runtime surface

That is the right scope. Anything larger is fake progress.

## Chosen Lane

M40 stays in `shared-core-portability`, but in the narrow sense only.

Here, portability means portability of bounded family-analysis decision semantics across consumers without dragging CLI wiring, artifact latest-path lookup, JSON command rendering, or milestone-specific wording into the seam.

This is not:

- a new Rust family promotion
- corpus expansion
- recommendation-policy work
- second-language backend work
- generalized multi-wedge logic

## Candidate Seam

The candidate seam is the smallest boundary that carries reusable family-analysis semantics:

```text
candidate seam
  helper-surface durable-hold classifier
  bounded corpus-program decision derivation
  normalized decision proof-fingerprint helpers
```

Current code anchors:

- durable classifier and frozen tuples:
  [`xtask/src/family/helper_surface.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs)
- bounded decision derivation and proof normalization:
  [`xtask/src/family/decision_kernel.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs)
- standing consumer that proves reuse pressure is no longer hypothetical:
  [`xtask/src/family/verify.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs)

The following must stay local even after any future seam move:

```text
must stay local
  xtask CLI wiring
  artifact latest-path lookup
  command-specific JSON rendering
  proof-wall file locations
  milestone-specific closeout wording
```

The repo must not extract "all of `xtask/src/family/`". That would be broad because it is adjacent, not because it is coherent.

## Architecture Surface

### Dependency graph

```text
recommendation.latest.json
        │
        ▼
promotion_artifacts.rs
  schema + contract validation
        │
        ├──────────────► helper_surface.rs
        │                frozen durable-hold classifier
        │
        ├──────────────► decision_kernel.rs
        │                basis snapshot + derived decision contract
        │                + normalized proof fingerprints
        │
        ▼
corpus-program-decision.latest.json
        │
        ├──────────────► verify.rs
        │                read-side parity verifier
        │
        ▼
ORCH_PLAN.md / proof walls / future consumers

outside seam, must stay local
  mod.rs
  paths.rs
  lib.rs
  command rendering and latest-path plumbing
```

### Why this is the right boundary

- `helper_surface.rs` already carries frozen semantic tuples, not command flow.
- `decision_kernel.rs` already derives bounded next-action truth from validated artifacts.
- `verify.rs` proves there is now at least one real read-side consumer beyond the original recommendation path.
- `mod.rs`, `paths.rs`, and CLI dispatch are still orchestration glue, not portable semantics.

That is the whole seam. No larger.

## Trigger Table

| Follow-on | Current state after M39 | Exact trigger | Authorized next move | Still does not count |
|---|---|---|---|---|
| local extraction inside `xtask/src/family/` | not triggered | one additional non-`recommend.rs` and non-`promotion_artifacts.rs` consumer inside `xtask/src/family/`, beyond `verify.rs`, reuses the same bounded decision semantics | author an M41 implementation plan for a still-local seam extraction | `verify.rs` alone, dead-code cleanup, or general tidiness |
| cross-crate family-analysis shared core | not triggered | one non-`xtask` crate needs the same bounded decision semantics | author a separate implementation plan that may cross crate boundaries | internal-only reuse pressure |
| generalized multi-wedge decision layer | not triggered | a second durable non-promotable wedge appears and cannot fit the current kernel shape honestly | author a dedicated follow-on plan for multi-wedge logic | hypothetical future wedges |
| public semantic fingerprint fields | not triggered | a real external consumer needs first-class fingerprint fields in emitted JSON | author a narrow export-contract plan | internal proof reuse only |

## M41 Authorization Gate

M41 must end in exactly one of these outcomes:

1. **Local implementation milestone**
   Allowed only if the first trigger-table row becomes true.

2. **Cross-crate implementation milestone**
   Allowed only if a non-`xtask` consumer proves the stronger cross-crate trigger.

3. **Further evidence milestone**
   Allowed if pressure grows but no trigger is yet satisfied.

4. **No new milestone**
   Allowed if the current kernel still serves all real consumers honestly.

M41 may not default to extraction just because M40 exists. A plan file is not evidence.

## Proof Floor

M40 approval stays tied to live repo truth, not to narrative confidence:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

These commands prove:

- the recommendation and decision artifacts still agree with derived truth
- the repo's current next action is still planning, not extraction
- the bounded decision surfaces remain green under regression coverage

## Verification Coverage Map

```text
DECISION CONTRACT COVERAGE
==========================
[+] helper_surface.rs
    ├── [TESTED] classify_helper_surface()
    ├── [TESTED] recommendation_matches_helper_surface_durable_hold_tuple()
    └── [TESTED] decision_matches_helper_surface_follow_on_tuple()

[+] decision_kernel.rs
    ├── [TESTED] corpus_program_basis_snapshot()
    ├── [TESTED] basis_snapshot_requires_helper_surface_follow_on()
    ├── [TESTED] basis_activates_helper_surface_follow_on()
    └── [TESTED] derive_corpus_program_decision_contract()

[+] verify.rs
    ├── [TESTED] artifact load + schema validation
    ├── [TESTED] basis snapshot parity
    ├── [TESTED] derived decision parity
    └── [TESTED] frozen helper-surface floor

[+] live command surface
    ├── [GREEN] cargo xtask family verify-decision-contract --format json
    ├── [GREEN] cargo xtask family corpus-decision --format json
    └── [GREEN] cargo test -p xtask

────────────────────────────────────────────
Coverage verdict: proof floor is already green
Known noise: 2 dead-code warnings in helper_surface.rs
Net new test gap for M40: none, because M40 is plan-only
────────────────────────────────────────────
```

## Error and Rescue Registry

| Failure | Detection surface | Immediate rescue | Why this is enough |
|---|---|---|---|
| latest recommendation artifact missing or invalid | `family verify-decision-contract` fails validation | regenerate or restore the canonical latest artifact before touching any plan decision | avoids inventing authority from stale or broken inputs |
| corpus decision artifact drifts from derived truth | `derived_decision_parity` fails | re-run `cargo xtask family corpus-decision --format json`, inspect tuple drift, then stop if the basis changed semantically | keeps the plan anchored to computed truth, not copied prose |
| helper-surface tuple changes silently | `frozen_helper_surface_floor` fails | compare the frozen tuple in `helper_surface.rs` against the verifier mismatch and treat as a contract change requiring a new milestone | prevents stealth widening of the seam |
| someone treats `verify.rs` alone as extraction approval | trigger table remains false | keep implementation frozen and route the request back through M41 authorization | consumer pressure is real, but still insufficient |

## Failure Modes Registry

| Failure mode | Test covers it | Error handling exists | User-visible outcome | Critical gap |
|---|---|---|---|---|
| future author treats `verify.rs` alone as extraction authority | yes, via proof floor plus trigger table | yes, by explicit trigger gating in this plan | visible because M41 outcome would be blocked | No |
| command plumbing leaks into the seam because it is adjacent | partially, by structural review rather than runtime test | yes, by explicit "must stay local" boundary | visible in review because touched modules would exceed the allowed seam | No |
| corpus work reopens by momentum | yes, `corpus-decision` output still says architecture follow-on | yes, via proof floor and non-goals | visible because decision action would contradict plan | No |
| dead-code cleanup gets promoted into milestone scope | no runtime test needed | yes, via explicit non-goals | visible because diff scope would drift beyond the artifact | No |
| cross-crate extraction is claimed from internal-only reuse | yes, trigger table requires a non-`xtask` consumer | yes, by explicit gate | visible because the stronger trigger would still be false | No |

No critical gaps are currently open. The boundary is tight enough if the repo obeys it.

## Not In Scope

The following were considered and are explicitly deferred:

- moving [`xtask/src/family/decision_kernel.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs) into a new crate, because no cross-crate consumer exists yet
- creating a new shared-core crate, because that would spend an innovation token before the seam is proven
- widening public artifact schemas, because no external consumer requires it yet
- spending corpus run `1`, because the live decision contract still says planning follow-on
- adding a new Rust family wedge, because it does not answer the current architecture question
- folding dead-code warning cleanup into M40, because hygiene is not evidence
- second-language backend work, because the shared-core boundary is still unproven even inside Rust

## Worktree Parallelization Strategy

### M40 itself

Sequential implementation, no parallelization opportunity.

Reason: M40 is a single authority artifact in [`PLAN.md`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md). Splitting one file across worktrees is coordination theater.

### First authorized implementation milestone, if a trigger later fires

#### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| freeze local seam interface | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs`, `xtask/src/family/promotion_artifacts.rs` | — |
| rewire in-tree consumers to the frozen seam | `xtask/src/family/verify.rs`, other in-tree family consumers that prove the trigger | freeze local seam interface |
| docs and closeout sync | `PLAN.md`, `ORCH_PLAN.md`, `.runs/` closeout artifacts, milestone docs | freeze local seam interface |
| command-surface adoption | `xtask/src/family/mod.rs`, `xtask/src/family/paths.rs`, command-facing proof walls | rewire in-tree consumers to the frozen seam |

#### Parallel lanes

- `Lane A`: freeze local seam interface
- `Lane B`: rewire in-tree consumers to the frozen seam, after `Lane A`
- `Lane C`: docs and closeout sync, after `Lane A`
- `Lane D`: command-surface adoption, after `Lane B`

#### Execution order

Launch `Lane A` first.

After `Lane A` lands, launch `Lane B` and `Lane C` in parallel worktrees.

After `Lane B` lands, run `Lane D`.

#### Conflict flags

- `Lane A` and `Lane B` both touch `xtask/src/family/`. They must not run in parallel.
- `Lane B` and `Lane D` both depend on command-facing consumer truth. `Lane D` waits.
- `Lane C` is the safest parallel lane because it stays in docs and closeout artifacts after the seam contract is frozen.

## Deliverables

M40 is done only when this file makes all of the following obvious:

1. what M39 actually proved
2. what M39 did not prove
3. what exact seam is under consideration
4. what evidence authorizes local extraction
5. what stronger evidence authorizes cross-crate extraction
6. what must remain local after any future seam move
7. what live commands revalidate the proof floor
8. how the first authorized implementation milestone would split across worktrees

## Acceptance Checklist

- [x] one concrete seam definition
- [x] one exact trigger table
- [x] one exact M41 authorization gate
- [x] one proof floor tied to live commands
- [x] one explicit non-goal block
- [x] one verification coverage map
- [x] one failure-modes registry
- [x] one worktree parallelization section
- [x] zero implementation authorization beyond the artifact itself

## Next Actions

1. Treat this file as the M40 authority artifact, not as notes about writing another artifact.
2. Keep implementation frozen until one trigger-table row becomes true.
3. When a row becomes true, author the next milestone against this contract instead of reopening the seam argument from scratch.
4. Ignore the two current dead-code warnings for milestone authority purposes unless a later implementation milestone explicitly scopes cleanup.
