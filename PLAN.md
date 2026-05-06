# M38 - Architecture Follow-On Trigger Gating After M37

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-05**  
Supersedes: **M37 - Family-Analysis Decision-Kernel Extraction After M36**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260505-184257.md`**  
Frozen baseline commit: **`e04d2fa9059c0010f84bd1f2b150feee6246bb84`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
M37 closeout: **`.runs/m37_decision_kernel_extraction/closeout.md`**  
Execution note: **M38 is a plan-first gating milestone. It does not spend corpus run `1`, does not reopen the arithmetic-ready story, does not move the family-analysis kernel out of `xtask/src/family/`, and does not widen any public artifact schema or CLI surface by default.**

## Objective

Turn the current `author_architecture_follow_on_plan` instruction into one
explicit repo-owned gate:

> what exact evidence would justify a deeper post-M37 family-analysis
> extraction, and what evidence means the repo should keep the current kernel
> local and stop?

M38 is not "do more architecture because the current shape feels close to
general."

M38 is:

1. confirm what M37 actually landed on the live branch
2. record which post-M37 triggers are currently false
3. define the exact evidence that would make any one of them true
4. authorize at most one bounded evidence probe
5. state the exact stop condition if that probe does not produce real pressure

The deliverable is a truthful go / no-go gate for M39+, not a stealth shared
core extraction.

## Executive Verdict

The repo does not currently justify:

1. a generalized multi-wedge decision layer
2. a cross-crate family-analysis shared core
3. public semantic fingerprint fields in emitted JSON

M38 therefore ships as a **trigger-gating milestone**, not as another runtime
architecture move.

The only authorized follow-on probe is a **real non-author maintainer
legibility dry run**. If no real non-author maintainer is available during the
milestone window, M38 stops after the trigger ledger and records that no probe
ran. It does not simulate a maintainer and it does not invent a second
consumer.

## Live Baseline

M37 landed on `feat/corpus-expansion` at
`e04d2fa9059c0010f84bd1f2b150feee6246bb84`.

Live revalidation on 2026-05-05 preserved the closeout floor:

- `cargo test -p xtask` passes `123/123`
- `cargo xtask family recommend --format json` remains
  `recommendation_status = "no_strong_candidate"`
- `cargo xtask family corpus-decision --format json` remains
  `decision_status = "not_recommended"`
- `open_blockers = ["helper_surface_not_promotable"]`
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

The only new runtime-adjacent signal from the live rerun is small:

- `xtask/src/family/helper_surface.rs` emits two `dead_code` warnings for
  `decision_uses_helper_surface_follow_on_tuple(...)` and
  `decision_matches_helper_surface_follow_on_tuple(...)`

That is hygiene pressure. It is not milestone pressure.

## Frozen Premises

1. M37 actually landed and still reproduces on the live branch.
2. The current repo state still contains one durable non-promotable
   helper-surface wedge, not multiple durable wedges.
3. `xtask/src/family/decision_kernel.rs` currently has two real runtime
   consumers:
   - `xtask/src/family/recommend.rs`
   - `xtask/src/family/promotion_artifacts.rs`
4. No current non-`xtask` crate needs family-analysis decision semantics.
5. No current external consumer needs first-class semantic fingerprint fields
   in emitted JSON.
6. Therefore none of the three post-M37 deeper-extraction triggers are true
   today.

## Problem Statement

The repo now has the correct bounded kernel shape, but it does not yet have a
truthful rule for when to go further.

That gap invites three bad moves:

1. extracting a generalized multi-wedge decision layer before a second durable
   wedge exists
2. extracting a cross-crate shared core before a real third-consumer or
   non-`xtask` pressure exists
3. publishing semantic fingerprint fields because the repo can, not because a
   real consumer needs them

All three are plausible future moves. None are justified by current branch
truth.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Family-analysis decision truth | `xtask/src/family/decision_kernel.rs` | Reuse unchanged. M38 does not relocate it. |
| Helper-surface classifier and frozen tuples | `xtask/src/family/helper_surface.rs` | Reuse unchanged except optional warning cleanup. |
| Recommendation emission and latest-byte reuse | `xtask/src/family/recommend.rs` | Reuse unchanged. M38 does not change recommendation policy. |
| Artifact validation | `xtask/src/family/promotion_artifacts.rs` | Reuse unchanged. M38 does not widen schema or add public fingerprint fields. |
| Current operator decision surface | `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json` | Reuse as live truth input. |
| Trigger ledger | `TODOS.md` post-M37 decision-kernel follow-ups | Reuse as the source of candidate deeper moves. |
| Corpus program stop/pivot guidance | `docs/recommendation_corpus_expansion_program_v0.1.md` | Reuse unchanged in substance. M38 must not silently reactivate corpus spending. |
| M37 verification record | `.runs/m37_decision_kernel_extraction/acceptance.md`, `closeout.md`, `proof-log.json` | Reuse as the baseline proof that the bounded extraction is complete. |

### Minimum complete change set

The minimum honest M38 is:

1. replace the finished M37 `PLAN.md` with an M38 plan grounded in live branch
   truth
2. define the exact trigger matrix for the three post-M37 follow-ups:
   - generalized multi-wedge decision layer
   - cross-crate family-analysis shared core
   - public semantic fingerprint fields
3. record the current verdict for each trigger and why it is false today
4. authorize at most one bounded evidence probe
5. state explicit non-goals and stop conditions

Anything beyond that is scope leak unless it directly supports the optional
probe or a tiny warning cleanup.

### Complexity check

M38 should stay almost entirely in planning and documentation surfaces.

The hard boundary is:

- no new crate
- no `spec-core` move
- no new artifact kind
- no schema version bump
- no public fingerprint fields
- no new family recommendation policy
- no corpus manifest expansion
- no synthetic second durable wedge

If a proposed M38 task needs any of those, it is not M38.

### Search check

**[Layer 1]** Reuse the repo's existing trigger ledger in `TODOS.md`.

**[Layer 1]** Reuse the current live decision artifact and M37 closeout as the
source of truth, not stale chat summaries.

**[Layer 3]** The right next move is not "generalize now." It is "write down
what would make generalization real."

**[EUREKA]** A second synthetic consumer created only to prove that a shared
core is needed is fake evidence. A trigger counts only if it comes from a real
repo pressure source, a real maintainer legibility failure, or a real
downstream consumer need.

### TODOS cross-reference

The relevant post-M37 TODOs already exist:

1. `Generalized multi-wedge decision layer`
2. `Cross-crate family-analysis shared core`
3. `Public semantic fingerprint fields`
4. `Run a true non-author maintainer promotion dry run`

M38 should not add new deeper-extraction TODOs unless a genuinely distinct
pressure source appears during the optional probe.

### Completeness check

A vague "we'll know it when we see it" follow-on is not enough.

The complete version is:

- one trigger matrix
- one current verdict per trigger
- one explicit probe contract
- one explicit non-goal list
- one exact rule for when M39 is authorized and when it is not

That is the lake. Boil it.

### Distribution check

No new package, binary, image, or release pipeline is required.

The affected operator surfaces remain:

- `cargo xtask family recommend --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family validate-artifact ...`
- `.semantic-family-artifacts/family-promotion/analysis/*.latest.json`

## Approved Scope

M38 includes exactly four deliverables:

1. this authoritative trigger-gating plan
2. a trigger matrix with current verdicts, evidence rules, and non-evidence
3. one optional non-author maintainer probe contract
4. a verification and closeout checklist that makes the stop condition
   impossible to misread

## NOT in scope

The following work is explicitly deferred:

- generalized decision-engine extraction
  Reason: no second durable wedge exists.
- cross-crate family-analysis shared core
  Reason: no third consumer or non-`xtask` pressure exists.
- public semantic fingerprint fields in JSON
  Reason: no external consumer requires them.
- corpus run `1` activation
  Reason: M38 is a gating milestone, not a corpus-spend milestone.
- arithmetic-ready story reactivation
  Reason: M37 retired that steering path.
- schema or CLI surface expansion
  Reason: M38 must not turn internal proof surfaces into public contract
  accidentally.
- synthetic consumers or fake wedges
  Reason: manufactured pressure would poison the next milestone decision.

## Architecture Review

### System boundary

M38 is intentionally a boundary-preserving milestone:

```text
CURRENT FAMILY-ANALYSIS SHAPE
=============================

xtask/src/family/recommend.rs
        │
        ├──── derives recommendation inputs
        │
        ▼
xtask/src/family/decision_kernel.rs
        │
        ├──── basis snapshot derivation
        ├──── decision derivation
        └──── normalized proof fingerprints
        │
        ├───────────────────────────────┐
        ▼                               ▼
xtask/src/family/promotion_artifacts.rs  live *.latest.json artifacts
        │
        └──── validates emitted truth

xtask/src/family/helper_surface.rs
        │
        └──── helper-surface classification + frozen tuples

M38 RULE:
keep every box in place unless a listed trigger becomes true
```

### Decision flow

```text
LIVE TRUTH → TRIGGER JUDGMENT → NEXT ACTION
===========================================

cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact ...
        │
        ▼
confirm live wedge still equals:
  no_strong_candidate
  not_recommended
  helper_surface_not_promotable
        │
        ▼
evaluate three follow-on triggers:
  multi-wedge?
  cross-crate?
  public fingerprint consumer?
        │
        ├── none true ──► keep kernel local, stop
        │
        └── one true ───► author exact next milestone against that trigger only
```

### Trigger matrix

| Follow-on | Current verdict | Trigger condition | Acceptable evidence | Non-evidence |
|---|---|---|---|---|
| Generalized multi-wedge decision layer | `not triggered` | A second durable non-promotable wedge appears whose path cannot be expressed in `decision_kernel.rs` without branching beyond the helper-surface contract. | Real recommendation-analysis output, real corpus-program decision behavior, or real validator/emitter divergence caused by the single-wedge assumption. | Hypothetical future wedges or a synthetic second wedge authored only to make the abstraction look useful. |
| Cross-crate family-analysis shared core | `not triggered` | At least two non-`recommend.rs` / non-`promotion_artifacts.rs` consumers inside `xtask/src/family/` need the same kernel logic, or a non-`xtask` crate needs the same decision semantics. | A justified third consumer, a real downstream crate or command path blocked on duplicated logic, or a maintainer workflow that becomes unmaintainable because the kernel cannot stay local honestly. | Extracting preemptively because a split "seems likely," or adding an artificial consumer whose only purpose is to satisfy the trigger. |
| Public semantic fingerprint fields | `not triggered` | An external consumer needs first-class semantic fingerprint fields in emitted JSON rather than internal proof gating only. | A real validator, CLI workflow, downstream tool, or review automation that cannot consume current artifact truth honestly without those fields. | "It would be nice for debugging" or exposing internals because the hash already exists. |

## Implementation Plan

### Phase 1 - Baseline confirmation

Goal: confirm that M37 truth still reproduces on the live branch before any
follow-on interpretation.

Steps:

1. run the verification floor in the listed command order
2. confirm the expected semantic outputs still match the live artifacts
3. record any drift from M37 closeout
4. classify any drift as one of:
   - no drift
   - hygiene drift
   - trigger-relevant drift

Exit rule:

- if trigger-relevant drift appears, stop and rewrite M38 around that new truth
- otherwise continue

### Phase 2 - Trigger ledger finalization

Goal: turn the post-M37 follow-ups into explicit gates instead of vibes.

Steps:

1. evaluate each trigger against live truth
2. write the current verdict for each trigger
3. write acceptable evidence and non-evidence for each trigger
4. confirm that none of the three are currently true

Exit rule:

- if any trigger is already true, M38 stops being a gating milestone and
  becomes the authoring input for the next exact milestone
- if none are true, continue

### Phase 3 - Optional evidence probe

Goal: test whether the current architecture hides real maintainer pressure
without manufacturing it.

Authorized probe:

- `non-author maintainer legibility dry run`

Entry criteria:

1. a real maintainer who did not author M35-M37 is available
2. the probe can run on current live branch truth
3. the probe is treated as evidence gathering, not as a reason to pre-approve
   deeper extraction

If any entry criterion is false:

- do not substitute the original author
- do not simulate the role with a second write-up
- record `probe not run, no real non-author maintainer available`
- stop after Phase 2

If all entry criteria are true:

1. run the baseline commands from a clean checkout or worktree
2. ask the maintainer to explain, in writing:
   - why the wedge remains `helper_surface_not_promotable`
   - why the kernel stays in `xtask/src/family/`
   - why corpus run `1` remains unspent
   - why semantic fingerprints stay internal only
3. record hidden context requests, confusion points, and any request for deeper
   extraction
4. map each failure, if any, back to one trigger only

Required output location if the probe runs:

- `.runs/m38_non_author_probe/summary.md`

Probe verdict rules:

- `pass`: maintainer can explain and operate the current path without hidden
  author context and without requesting deeper extraction
- `fail`: repeated legibility or operability failure maps cleanly to a listed
  trigger
- `inconclusive`: probe was interrupted or evidence quality is too weak to map
  honestly to a trigger

### Phase 4 - Closeout

Goal: close M38 with one unambiguous answer.

Closeout statement must be exactly one of:

1. `No deeper extraction justified yet. Keep the kernel local.`
2. `Trigger proven. Author the next milestone against <exact trigger>.`
3. `Probe inconclusive. Do not extract yet. Re-run only with real new evidence.`

## Evidence Probe Decision

M38 authorizes the maintainer legibility dry run and rejects the "deliberate
second consumer" probe for now.

Why this probe wins:

1. it tests whether the current local-kernel shape is actually hard to operate
2. it does not create fake architectural pressure
3. it reuses an already-existing backlog pressure source
4. it can fail honestly into either the cross-crate or multi-consumer trigger
   without widening the runtime surface first

Why the second-consumer probe is rejected in M38:

1. it is too easy to fake
2. it risks spending architecture effort to manufacture the evidence
3. it confuses "can build another consumer" with "must build another consumer"

## Optional Hygiene

M38 may carry one tiny warning-cleanup decision if maintainers want warning-clean
output during the probe:

- keep `decision_uses_helper_surface_follow_on_tuple(...)` and
  `decision_matches_helper_surface_follow_on_tuple(...)` as explicit
  freeze-sentinel runtime helpers, or
- move them behind test-only usage

This is never a milestone driver. If it threatens to expand beyond a tiny local
cleanup, defer it.

## Test And Verification Plan

M38 is plan-first, so the test surface is the verification floor plus probe
truthfulness, not new product behavior.

### Execution coverage

```text
M38 EXECUTION COVERAGE
======================

[1] Baseline reproduction
    ├── cargo test -p xtask
    ├── family recommend --format json
    ├── family corpus-decision --format json
    └── validate-artifact on both *.latest.json artifacts

[2] Trigger evaluation
    ├── multi-wedge trigger
    ├── cross-crate trigger
    └── public fingerprint trigger

[3] Optional probe
    ├── maintainer can explain current wedge
    ├── maintainer can operate current commands
    └── failure maps to one trigger or does not count

[4] Closeout
    └── exact stop/go statement written with no ambiguous middle state
```

### Verification floor

Run these commands in order:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Expected semantic outcome:

- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- `open_blockers = ["helper_surface_not_promotable"]`
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

### Acceptance matrix

| Check | Pass condition | Failure meaning |
|---|---|---|
| Baseline revalidation | All commands pass and semantic outputs match the frozen floor. | M37 truth drifted. Re-baseline before interpreting M38. |
| Trigger evaluation | All three current verdicts remain `not triggered`. | M38 is no longer just a gating milestone. |
| Probe integrity | Probe uses a real non-author maintainer or does not run. | Synthetic evidence invalidates the milestone decision. |
| Closeout | Final statement matches one of the three allowed closeout lines. | The milestone remains ambiguous and is not done. |

## Failure Modes Registry

| Failure mode | Covered by verification? | Error handling exists? | User-visible effect | Critical? | Mitigation |
|---|---|---|---|---|---|
| Baseline commands drift from M37 truth | Yes | Yes | Maintainers would make decisions from stale reality. | Yes | Stop and re-baseline before any trigger judgment. |
| Probe uses synthetic consumer or fake maintainer pressure | Yes | Yes | Repo may over-extract for a problem it invented. | Yes | Reject the evidence and keep kernel local. |
| Probe reveals confusion but it does not map to a listed trigger | Yes | Yes | Team may overfit one vague complaint into a roadmap move. | No | Record as inconclusive, do not authorize extraction. |
| Warning cleanup expands into behavior change | Partially | Yes | A hygiene task could silently become a runtime milestone. | No | Defer cleanup unless it stays tiny and local. |
| Corpus run `1` gets reopened implicitly | Yes | Yes | Team starts spending program budget under the wrong milestone. | Yes | Keep corpus activation explicitly out of scope. |
| Public fingerprint exposure gets bundled "for convenience" | Yes | Yes | Internal proof surface becomes accidental public contract. | Yes | Reject unless a real external consumer is demonstrated. |

No M38 failure mode is allowed to fail silently. Every failure must end in
either stop, defer, or re-baseline.

## Performance And Operability Review

There is no new runtime hot path in M38.

The only meaningful performance and operability considerations are:

1. baseline verification should stay limited to the existing `xtask` floor
2. the optional probe should run against existing commands, not add new heavy
   harnessing
3. warning cleanup, if done, must stay local to `xtask/src/family/helper_surface.rs`
   and must not widen compile or test scope

This is intentionally boring. Good.

## Worktree Parallelization Strategy

Default expectation: **mostly sequential**.

This milestone has only two genuine parallel opportunities, and both are
conditional. If the team is only landing this plan, use one lane and keep it
simple.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Baseline confirmation | `xtask/src/family/`, `.semantic-family-artifacts/`, `.runs/m37_decision_kernel_extraction/` | — |
| Authoritative M38 plan rewrite | repo root planning docs (`PLAN.md`) | baseline confirmation |
| Optional warning cleanup | `xtask/src/family/` | baseline confirmation |
| Optional non-author probe artifact | `.runs/m38_non_author_probe/` | plan rewrite |
| Final closeout statement | `PLAN.md`, optional `.runs/m38_non_author_probe/` | all prior executed steps |

### Parallel lanes

Lane A: baseline confirmation → plan rewrite → final closeout  
Lane B: optional warning cleanup (sequential inside `xtask/src/family/`)  
Lane C: optional non-author probe artifact generation

### Execution order

1. Run Lane A first through baseline confirmation.
2. After baseline confirmation:
   - continue Lane A with the plan rewrite
   - optionally launch Lane B in parallel if the team explicitly wants the tiny
     warning cleanup
3. Launch Lane C only after the plan rewrite is stable, because the probe must
   use the final trigger matrix and closeout rules.
4. Merge Lane B and Lane C back into Lane A, then write the final closeout
   statement.

### Conflict flags

- Lanes A and C are safe in parallel after the plan rewrite because they touch
  different directories.
- Lanes A and B should not run in parallel after the rewrite if both need to
  touch milestone wording about the cleanup decision.
- Lane B must stay single-owner because every step touches the same
  `xtask/src/family/` module lane.

If the team does not take the optional cleanup and does not run the probe, the
correct answer is:

`Sequential implementation, no parallelization opportunity beyond the plan rewrite.`

## Completion Summary

- Step 0 - Scope Challenge: scope accepted as-is for a plan-first gating
  milestone
- Architecture Review: 3 explicit trigger gates, 0 new runtime architecture
  moves authorized
- Code Quality Review: optional helper-surface warning cleanup only, otherwise
  reuse current code unchanged
- Test Review: verification floor defined, probe coverage defined, no new
  product-behavior test surface added
- Performance Review: no new runtime hot path, operability limited to existing
  `xtask` command floor
- NOT in scope: written
- What already exists: written
- TODOS.md handling: existing post-M37 TODOs stay authoritative; no new TODO is
  created unless the probe reveals a genuinely distinct pressure source
- Failure modes: 4 critical stop/defer conditions flagged
- Parallelization: 3 lanes total, with 2 optional conditional lanes
- Lake Score: complete option chosen, explicit trigger matrix + probe contract +
  failure modes + parallelization all included

## Deliverables

M38 is complete only when all of these exist:

1. this authoritative `PLAN.md`
2. one explicit trigger matrix with current verdicts
3. one explicit non-goals section
4. one verification floor and acceptance matrix
5. one failure modes registry
6. one worktree parallelization strategy
7. if the probe ran, `.runs/m38_non_author_probe/summary.md`
8. one exact closeout statement chosen from the allowed list

## Success Criteria

M38 is done only when all of these are true:

1. `PLAN.md` no longer describes the finished M37 extraction and instead
   states the M38 trigger-gating mission.
2. Each post-M37 follow-up has an explicit current verdict, trigger rule,
   acceptable evidence, and non-evidence.
3. The repo names at most one honest evidence probe.
4. The plan makes clear that deeper extraction is unauthorized until one of the
   listed triggers becomes true.
5. The live wedge remains:
   - `recommendation_status = "no_strong_candidate"`
   - `decision_status = "not_recommended"`
   - `open_blockers = ["helper_surface_not_promotable"]`
   - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
   - `decision_basis_code = "durable_non_promotable_helper_surface"`
   - `required_next_action = "author_architecture_follow_on_plan"`

## M39 Authorization Rule

M39 is authorized only if one of these becomes true:

1. a second real durable wedge exists
2. a third real consumer or non-`xtask` consumer exists
3. a real external consumer needs public semantic fingerprint fields
4. the non-author maintainer probe fails in a way that maps cleanly to one of
   the three triggers above

If none of those happen, the repo keeps the kernel local and stops.

## Next Actions

1. Treat this file as the authoritative M38 boundary.
2. Re-run the verification floor before any follow-on discussion.
3. Run the non-author maintainer probe only if a real non-author maintainer is
   available.
4. If the probe does not produce a trigger, stop and keep the kernel local.
5. If the probe does produce a trigger, author the next milestone against that
   exact trigger and nothing broader.
