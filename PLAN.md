<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260506-224817.md -->
# M40 - Architecture Shared-Core Follow-On Planning Contract After M39

Status: **authority plan draft**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-07**  
Supersedes: **M39 - Verification Consumer Probe After M38**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260506-101748.md`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
Latest closeout: **`.runs/m39_verification_consumer_probe/closeout.md`**  
Execution note: **M40 is a planning follow-on only. It does not authorize shared-core extraction, corpus run `1`, a new Rust family wedge, or second-language backend work.**

## Executive Verdict

M39 proved that the repo now has a third honest consumer of the current bounded
family-analysis decision contract.

M39 did not prove that a shared-core extraction is now required.

That distinction is the whole job of M40.
This file is the authority draft that locks the seam, the trigger table, the
verification floor, and the exact gate between "planning authorized" and
"implementation authorized."

## Objective

Author the exact follow-on plan that the current live decision surface names:

```text
decision_action = pivot_to_architecture_shared_core_follow_on
required_next_action = author_architecture_follow_on_plan
```

M40 is not an extraction milestone.
It is the artifact that decides:

1. what seam would move if reuse pressure becomes strong enough
2. what evidence upgrades the repo from plan-authorized to implementation-authorized
3. what must remain local even after any future extraction
4. which future milestone is allowed next, and which ones are still blocked

## Live Validated Basis

The current branch was revalidated on **2026-05-07** against the live tree.

Commands run:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Observed results:

- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks passed
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `decision_basis_code = "durable_non_promotable_helper_surface"`
  - `pivot_target_class = "architecture_shared_core_follow_on"`
  - `required_next_action = "author_architecture_follow_on_plan"`
- `cargo test -p xtask`
  - `136 passed; 0 failed`
  - current warnings are limited to dead-code warnings in `xtask/src/family/helper_surface.rs`

Current repo truth that M40 must honor:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- corpus run `1` remains unspent
- the helper-surface wedge remains a durable non-promotable hold
- the verifier command is now a real standing consumer of that bounded contract

## Locked Interpretations

These are not open questions anymore. M40 treats them as defaults unless a later
milestone explicitly reopens them.

1. **M39 proved consumer pressure, not extraction authority.**  
   `verify.rs` is now a real consumer. That does not by itself authorize code motion.

2. **The next honest move is still planning.**  
   The repo's live action contract says `author_architecture_follow_on_plan`. M40 reads that literally.

3. **If future implementation is authorized from internal pressure alone, the first move stays local to `xtask/src/family/`.**  
   No new crate is justified until a non-`xtask` consumer actually appears.

4. **Cross-crate extraction is a stronger claim than local seam extraction.**  
   It stays separately gated by the `Cross-crate family-analysis shared core` trigger in `TODOS.md`.

5. **Warning cleanup in `xtask/src/family/helper_surface.rs` is hygiene only.**  
   It is not a milestone driver and it does not change the decision surface.

6. **The candidate seam includes normalized proof-fingerprint helpers.**  
   That logic already lives in `decision_kernel.rs` and belongs with bounded decision derivation if the seam ever moves.

## Problem Statement

The repo now has enough evidence to say "write the architecture follow-on plan,"
but not enough evidence to say "start extracting shared core now."

If M40 skips that distinction, the repo throws away the discipline it just
earned:

- a bounded recommendation-analysis surface
- a bounded decision-action surface
- a bounded verifier consumer over both

The failure mode would be predictable. One new consumer would get rewritten into
"architecture feels tidy now," and the repo would start moving code without a
real portability trigger.

M40 exists to prevent exactly that.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Durable helper-surface classification | `xtask/src/family/helper_surface.rs` | Reuse as the frozen classifier contract. Do not widen in M40. |
| Bounded decision derivation | `xtask/src/family/decision_kernel.rs` | Reuse as the current local source of truth for stop / spend / pivot logic. |
| Verifier consumer | `xtask/src/family/verify.rs` | Reuse as the new proof that consumer pressure is real, not hypothetical. |
| Artifact contract validation | `xtask/src/family/promotion_artifacts.rs` | Reuse as the existing read-side contract boundary. |
| Latest-path lookup and command surfaces | `xtask/src/family/paths.rs`, `xtask/src/lib.rs`, `xtask/src/family/mod.rs` | Keep local. These are command plumbing, not seam semantics. |
| Standing proof-wall adoption | `ORCH_PLAN.md` and `.runs/m39_verification_consumer_probe/closeout.md` | Reuse as the latest shipped evidence that the verifier is a maintained consumer. |
| Trigger inventory | `TODOS.md` post-M37 follow-ups | Reuse as the source of truth for what must happen before broader extraction claims are honest. |

### Minimum complete M40

The minimum honest M40 deliverable is:

1. one authority-grade `PLAN.md` for the architecture follow-on
2. one exact trigger table mapping future evidence to allowed next moves
3. one explicit seam boundary saying what would move and what must stay local
4. one verification section that ties the plan to live command output
5. one M41 gate that distinguishes local extraction, cross-crate extraction, more evidence, or no milestone
6. one parallelization section so future implementation work can split cleanly if it is ever authorized

### Minimal diff rule

M40 stays narrow on purpose:

- no Rust source edits
- no new public artifact schema
- no new CLI behavior
- no movement of `decision_kernel.rs`
- no reopening of corpus run `1`
- no second-language portability work

If M40 starts touching implementation modules, it has already failed scope control.

## Approaches Considered

### Approach A - Plan-Only M40

Summary:
Author the architecture/shared-core follow-on plan and stop there.
Define the exact seam, the trigger table, the proof floor, and the M41 gate.

Effort: S  
Risk: Low

Pros:

- matches the literal `required_next_action`
- preserves the bounded-decision discipline M39 just proved
- gives the repo a reusable go / no-go contract instead of another architecture argument

Cons:

- no code movement
- feels slower than the branch's recent implementation momentum

### Approach B - Local Extraction Inside `xtask`

Summary:
Treat the verifier as sufficient pressure to start a still-local extraction now.

Effort: M  
Risk: Medium

Pros:

- responds quickly to real reuse pressure
- could reduce duplication sooner

Cons:

- skips over the explicit planning contract
- turns adjacency into authority

### Approach C - Wait Without Writing The Plan

Summary:
Leave the kernel local, do no planning work, and wait for more pressure.

Effort: XS  
Risk: Low

Pros:

- zero architecture churn
- maximally conservative

Cons:

- ignores the explicit next-step contract
- guarantees the repo will have to re-argue the same question later

## Recommended Approach

Choose **Approach A - Plan-Only M40**.

That is the only option that matches all three of these at once:

1. the live command output
2. the frozen decision docs
3. the minimal-diff engineering preference for local, explicit, boring moves first

## Approved Scope

M40 includes exactly these deliverables:

1. this authority plan draft
2. the trigger table below
3. the seam boundary below
4. the verification and failure-mode sections below
5. the M41 authorization gate below
6. the parallelization strategy below

## NOT in scope

The following work is explicitly deferred:

- moving `xtask/src/family/decision_kernel.rs`
  - Reason: M40 decides whether that move is earned, it does not perform it.
- extracting a cross-crate shared core
  - Reason: no non-`xtask` consumer has yet proven that stronger portability claim.
- generalized multi-wedge decision logic
  - Reason: no second durable non-promotable wedge exists.
- public semantic fingerprint fields
  - Reason: no external consumer currently requires them.
- corpus run `1`
  - Reason: the current decision artifact keeps it unspent.
- second-language backend work
  - Reason: the current blocker is portability-boundary honesty, not backend execution support.
- helper-surface warning cleanup as part of M40 scope
  - Reason: hygiene is allowed later, but it is not part of the M40 acceptance contract.

## Architecture Review

### Current dependency graph

The current bounded decision surface is already narrower than "all of family analysis."

```text
CURRENT M39 FLOOR
=================
recommendation.latest.json
        |
        v
promotion_artifacts.rs <-----------------------------+
        |                                            |
        v                                            |
decision_kernel.rs <------ helper_surface.rs         |
        |                                            |
        +------ normalized proof fingerprint helpers |
        |                                            |
        v                                            |
verify.rs -------------------------------------------+
        |
        v
ORCH_PLAN.md proof walls
```

What matters here:

- `decision_kernel.rs` already owns the bounded stop / spend / pivot semantics
- `helper_surface.rs` already owns the durable hold classifier tuple
- `verify.rs` is now a real consumer over that contract
- command wiring, artifact paths, and proof-wall wording are adjacent, but they are not seam semantics

### Candidate seam

The future seam is **not** "move everything under `xtask/src/family/`."

The future seam is the smallest boundary that carries the reusable decision semantics:

```text
POSSIBLE FUTURE LOCAL SEAM
==========================
xtask/src/family/decision_core/
  helper_surface contract
  decision derivation helpers
  normalized proof fingerprint helpers

STAYS LOCAL EVEN AFTER THAT
===========================
xtask CLI wiring
artifact path lookup
command-specific JSON rendering
proof-wall file locations
milestone-specific closeout wording
```

### Local-first extraction rule

If future implementation is authorized by **internal** consumer pressure only,
the first implementation milestone remains a **still-local extraction inside
`xtask/src/family/`**.

If a **non-`xtask` consumer** appears, the repo may then consider a stronger
cross-crate extraction claim.

That two-stage rule keeps the plan boring by default:

- internal pressure -> local seam first
- external pressure -> cross-crate extraction becomes discussable

### Production failure scenarios

| Surface | Realistic failure | Does M40 account for it? | Why the response is sufficient |
|---|---|---|---|
| Decision derivation | `decision_kernel.rs` drifts from the frozen helper-surface hold contract | Yes | `cargo xtask family verify-decision-contract --format json` fails loudly and non-zero. |
| Verifier consumer | `verify.rs` silently stops proving the same semantics as the decision artifact | Yes | parity is enforced by the verifier command itself and backed by `cargo test -p xtask`. |
| Architecture boundary | future author pulls command plumbing into the seam because it is adjacent | Yes | seam boundary above explicitly keeps JSON rendering, path lookup, and ORCH wording local. |
| Portability claim | internal tidiness gets misread as cross-crate portability pressure | Yes | local-first extraction rule blocks cross-crate motion until a non-`xtask` consumer appears. |
| Corpus steering | someone resumes corpus work because the helper surface is still visible | Yes | current artifact still says `pivot_to_architecture_shared_core_follow_on` and keeps corpus run `1` unspent. |

## Trigger Table

This table is the contract M40 exists to freeze.

| Follow-on | Current status after M39 | Trigger that authorizes it | Authorized next move | What still does **not** count |
|---|---|---|---|---|
| **Local decision-core extraction inside `xtask/src/family/`** | not yet triggered | one additional non-`recommend.rs` / non-`promotion_artifacts.rs` consumer inside `xtask/src/family/` beyond `verify.rs`, using the same bounded decision semantics | author an M41 implementation milestone that keeps the seam local to `xtask/src/family/` | `verify.rs` alone, dead-code cleanup, or general architectural tidiness |
| **Cross-crate family-analysis shared core** | not yet triggered | one non-`xtask` crate needs the same bounded decision semantics | author a new implementation plan that may move the seam across crate boundaries | internal-only reuse pressure without a non-`xtask` consumer |
| **Generalized multi-wedge decision layer** | not yet triggered | a second durable non-promotable wedge appears and cannot be expressed cleanly through the current kernel shape | author a dedicated follow-on plan for multi-wedge decision logic | hypothetical future wedges or policy anxiety |
| **Public semantic fingerprint fields** | not yet triggered | a real external consumer needs first-class fingerprint fields in emitted JSON | author a narrow export-surface plan for those public fields | internal proof reuse only |

## M41 Authorization Gate

M41 must choose exactly one of these outcomes:

1. **Local implementation milestone**  
   Allowed only if the first row of the trigger table is satisfied.

2. **Cross-crate implementation milestone**  
   Allowed only if a non-`xtask` consumer satisfies the second row of the trigger table.

3. **Further evidence milestone**  
   Allowed if pressure is growing but no trigger is yet satisfied.

4. **No new milestone yet**  
   Allowed if the current kernel still serves all real consumers honestly.

M41 must not default to extraction just because M40 exists.

## Test Review

M40 is a planning milestone, but it still needs a concrete proof floor.
The acceptance contract must be recheckable by a tired maintainer on the live branch.

### Verification coverage diagram

```text
M40 ACCEPTANCE COVERAGE
=======================
[+] Live decision-contract truth
    |
    +-- [TESTED] verify-decision-contract parity
    |     command: cargo xtask family verify-decision-contract --format json
    |     proves: recommendation + decision artifacts still agree with derived truth
    |
    +-- [TESTED] corpus-program action tuple
    |     command: cargo xtask family corpus-decision --format json
    |     proves: next action is still planning, not extraction
    |
    +-- [TESTED] xtask regression floor
          command: cargo test -p xtask
          proves: decision-kernel, helper-surface, and verifier regressions stay green

[+] M40 authority assertions
    |
    +-- [PLAN GUARD] no code motion authorized in M40
    +-- [PLAN GUARD] corpus run 1 remains unspent
    +-- [PLAN GUARD] local-first extraction rule is explicit
    +-- [PLAN GUARD] cross-crate extraction needs a non-xtask consumer
```

### Required commands while approving M40

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Required tests if M41 implementation is later authorized

If the first trigger fires and M41 becomes a local extraction milestone, the
implementation plan must require all of these:

1. regression tests proving `verify.rs` still matches the derived decision contract
2. regression tests proving the helper-surface durable-hold tuple remains byte-truthful
3. regression tests proving normalized proof-fingerprint behavior is preserved
4. no schema-version bump unless an explicit export contract plan authorizes it
5. no command-surface changes unless they are separately scoped and tested

## Failure Modes Registry

| Failure mode | Test covers it? | Error handling exists? | User-visible outcome | Critical gap? |
|---|---|---|---|---|
| verifier command no longer matches derived decision truth | Yes | Yes | explicit non-zero command failure | No |
| corpus decision action changes without a new plan | Yes | Yes | explicit JSON drift visible in command output | No |
| future author misreads internal reuse pressure as cross-crate authority | Partially, by plan guard not code | Yes, through explicit gate language | review catches it before implementation | No |
| warning cleanup gets silently promoted into milestone scope | N/A, policy not code | Yes, through explicit non-goals | reviewer sees scope drift immediately | No |

There are **0 critical gaps** in the M40 planning contract as written.
The only non-automated guard here is review discipline around scope creep, which
is why the non-goal section and trigger table are first-class deliverables.

## Worktree Parallelization Strategy

### M40 itself

Sequential implementation, no parallelization opportunity.

M40 is one authority artifact.
Multiple people editing the same plan spine in parallel would create more merge
conflict than leverage.

### Prospective M41 parallel lanes if implementation is later authorized

This section exists now so the future implementation milestone starts with a
clean lane split instead of inventing one under pressure.

#### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Freeze local seam foundation | `xtask/src/family/decision_kernel.rs`, `xtask/src/family/helper_surface.rs`, new local seam module under `xtask/src/family/` | none |
| Rewire read-side consumers | `xtask/src/family/verify.rs`, `xtask/src/family/recommend.rs`, `xtask/src/family/promotion_artifacts.rs` | Freeze local seam foundation |
| Command and proof-wall adoption | `xtask/src/lib.rs`, `xtask/src/family/mod.rs`, `ORCH_PLAN.md`, `.runs/**` proof artifacts | Rewire read-side consumers |
| Docs and closeout sync | `PLAN.md`, `TODOS.md`, milestone closeout docs | Freeze local seam foundation |

#### Parallel lanes

- **Lane A:** freeze local seam foundation  
  Sequential. Shared semantic source of truth. This lane must land first.

- **Lane B:** rewire read-side consumers  
  Starts after Lane A freezes the interface.

- **Lane C:** docs and closeout sync  
  Starts after Lane A freezes the interface. Can run in parallel with Lane B.

- **Lane D:** command and proof-wall adoption  
  Starts after Lane B. This is the integration lane.

#### Execution order

```text
Lane A
  |
  +--> Lane B
  |
  +--> Lane C
         |
         +--> Lane D after Lane B finishes
```

Launch **Lane B + Lane C** in parallel only after **Lane A** has frozen the
local seam shape.
Merge both.
Then run **Lane D** as the proof-wall and command-surface integration lane.

#### Conflict flags

- Lane A and Lane B both touch `xtask/src/family/`. They must not overlap in time.
- Lane B and Lane D both touch command-adjacent family surfaces. D waits on B.
- Lane C should avoid editing any code module under `xtask/src/family/`. Keep it docs-only.

## Deliverables

M40 is complete only when all of these are true:

1. `PLAN.md` is the authority-grade draft for the architecture follow-on
2. the trigger table above is present and unambiguous
3. the local-first extraction rule is explicit
4. the cross-crate trigger is separately explicit
5. the verification commands above still pass on the live branch
6. the M41 gate ends in one exact next outcome, not vague momentum language
7. the parallelization section exists so future implementation work can split cleanly if authorized

## Success Criteria

M40 is done only when this file makes all of the following obvious to a new
maintainer:

1. what M39 actually proved
2. what M39 did not prove
3. what seam is on deck if pressure grows
4. what evidence authorizes local extraction
5. what stronger evidence authorizes cross-crate extraction
6. what stays local even after any future seam move
7. what exact commands revalidate the live proof floor

## Next Actions

1. Treat this file as the branch authority draft for M40.
2. Review this file as the artifact, not as a plan-to-write-a-plan memo.
3. If approved, keep implementation frozen until one row in the trigger table becomes true.
4. When a row becomes true, author the next milestone against this contract instead of reopening the architecture argument from scratch.
