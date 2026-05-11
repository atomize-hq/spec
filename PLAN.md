# M48: Shared-Core Portability Follow-On, Slice 1 Implementation Plan

Status: **implementation plan**  
Milestone: **M48**  
Milestone family: **shared-core-portability**  
Implementation readiness: **ready for review and bounded execution**  
Plan scope: **Lane A only, freeze seam interface**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Execution precondition: **clean worktree**  
Last rewritten: **2026-05-11**

Primary source artifacts:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-085549.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m47_post_m46_shared_core_portability_follow_on/closeout.md`

Primary repo surfaces:
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

Companion test artifact:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-090108.md`

## Executive Summary

M47 finished the authority freeze. M48 is the first bounded implementation slice that can follow it without lying about what the repo has actually authorized.

The seam already exists in code. Lane A does not invent a new architecture. It freezes the existing `xtask/src/family/analysis_core/*` seam tightly enough that later consumer rewires have one stable semantic target and do not reopen the seam-definition fight.

If Lane A is done correctly, all of the following remain true at the same time:

1. `analysis_core/*` is the only approved semantic owner surface.
2. `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` keep the same behavior.
3. the repo still lands on the same stop-state truth:
   - `decision_action = stop`
   - `decision_basis_code = no_actionable_candidate`
   - `required_next_action = record_stop_without_new_milestone`

## Decision This Plan Makes

This plan authorizes exactly one slice:

1. freeze the seam facade in `analysis_core/mod.rs`
2. freeze helper-surface semantics in `helper_surface.rs`
3. freeze decision-contract semantics in `decision_contract.rs`
4. freeze proof-fingerprint semantics in `proof_fingerprint.rs`
5. prove zero downstream drift with seam-local tests plus command-surface parity

This plan does not authorize:

- consumer rewires
- CLI wiring changes
- latest-path lookup changes
- schema changes
- backend widening
- shim cleanup
- crate extraction
- new abstraction layers

If any of those become necessary to land Lane A, Lane A is scoped incorrectly and must stop.

## Live Validated Basis

Validated from a clean worktree on 2026-05-11.

Commands run:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Observed truth:

- `collect_signals.sh`
  - dirty status: clean
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks passed
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask`
  - `146` tests passed

Lane A is only allowed to tighten interface and proof. It is not allowed to reinterpret this basis, spend corpus run 1, or quietly widen the seam because the files sit next to each other.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| semantic seam facade | `xtask/src/family/analysis_core/mod.rs` | already exists, freeze it rather than reinvent it |
| helper-surface classification and durable-hold tuple | `xtask/src/family/analysis_core/helper_surface.rs` | already exists, harden exact contract |
| basis snapshot and derived decision semantics | `xtask/src/family/analysis_core/decision_contract.rs` | already exists, harden exact contract |
| semantic proof fingerprint normalization | `xtask/src/family/analysis_core/proof_fingerprint.rs` | already exists, harden exact contract |
| write-side consumer | `xtask/src/family/recommend.rs` | proof surface only, do not rewire in Lane A |
| read-side parity consumer | `xtask/src/family/verify.rs` | proof surface only, do not rewire in Lane A |
| local validator consumer | `xtask/src/family/promotion_artifacts.rs` | keep local, do not treat as seam ownership |
| compatibility shims | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs` | preserve unchanged in Lane A |
| CLI wiring, path lookup, JSON rendering | `xtask/src/family/mod.rs`, `xtask/src/family/paths.rs`, `xtask/src/lib.rs`, coverage/recommend rendering helpers | explicitly local, out of scope |

### Minimum complete slice

Lane A is the smallest honest implementation only if it does all of this and nothing more:

1. freezes the `analysis_core` facade and ownership boundary
2. freezes helper-surface, decision-contract, and proof-fingerprint semantics
3. adds or tightens seam-local proof where current tests are too implicit
4. proves downstream command surfaces still land on the same truth

Anything beyond that is a different slice. Anything less leaves the seam under-specified and forces the next slice to rediscover the contract mid-flight.

### Complexity, completeness, and distribution

- Primary write scope: `4` files under `xtask/src/family/analysis_core/`
- Allowed secondary proof touch scope: existing `xtask` tests only if required to keep the proof wall truthful
- New runtime classes or services: `0`
- New infrastructure: `0`
- New distribution work: `0`
- TODO cross-reference: no existing `TODOS.md` item blocks Lane A; broader portability, backend, and extraction work remains deferred there

This is the complete version of the slice, not a shortcut. The alternative shortcut would be to freeze comments and stop at command-level parity. That saves almost nothing and leaves later rewires arguing over behavior that should have been locked now. Not worth it.

### Locked plan decisions

These are resolved. They should not be reopened during implementation:

- Compatibility shims stay unchanged in Lane A. No deprecation comments, no new logic, no ownership shift.
- `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` are proof surfaces, not write scope. If Lane A needs semantic behavior changes there, stop and re-scope.
- No new schema fields, CLI flags, path rules, or rendering helpers are allowed in this slice.
- No new abstraction layer is allowed. Use explicit functions and tests inside the existing `analysis_core/*` files.
- Any implementation that touches more than the four seam files plus existing proof tests is presumed overbuilt until it proves otherwise.

### Abort and re-scope triggers

Stop Lane A and write a follow-on plan instead if any of the following happen:

1. a downstream consumer needs behavior change to compile or stay truthful
2. a shim needs new logic, even if the logic looks tiny
3. command output needs to change to express the seam freeze
4. artifact JSON fields need to change to keep fingerprints or parity working
5. the seam cannot be frozen without introducing a fifth owner file or a new helper layer

## Architecture and Ownership

### Seam ownership matrix

| Surface | Role | Lane A rule |
| --- | --- | --- |
| `analysis_core/mod.rs` | seam facade | sole approved import surface for seam semantics |
| `analysis_core/helper_surface.rs` | helper-surface classification contract | freeze exact classifier and durable-hold tuple semantics |
| `analysis_core/decision_contract.rs` | corpus-decision contract | freeze exact basis snapshot and decision derivation semantics |
| `analysis_core/proof_fingerprint.rs` | semantic fingerprint contract | freeze normalization rules for reuse parity |
| `recommend.rs` | write-side consumer | behavior must remain identical |
| `verify.rs` | read-side parity consumer | behavior must remain identical |
| `promotion_artifacts.rs` | local validator consumer | remains local, not promoted to seam ownership |
| `helper_surface.rs`, `decision_kernel.rs` | compatibility shims | unchanged, compatibility-only |
| CLI / paths / rendering / schemas | local orchestration | out of scope |

### Frozen facade inventory

The facade in `xtask/src/family/analysis_core/mod.rs` must explicitly and stably expose exactly these semantic surfaces, grouped by concern rather than accidental file order:

Decision contract exports:

- `DecisionContractStopStateTuple`
- `DerivedCorpusProgramDecision`
- `basis_activates_helper_surface_follow_on`
- `basis_snapshot_requires_helper_surface_follow_on`
- `corpus_program_basis_snapshot`
- `decision_contract_stop_state_tuple`
- `derive_corpus_program_decision_contract`

Helper-surface exports:

- `HELPER_SURFACE_FINGERPRINT`
- `HelperSurfaceDisposition`
- `HelperSurfaceSignal`
- `classify_helper_surface`
- `durable_non_promotable_helper_surface_candidate_tuple`
- `recommendation_matches_helper_surface_durable_hold_tuple`
- `recommendation_uses_helper_surface_durable_hold_tuple`

Proof-fingerprint exports:

- `normalized_corpus_program_decision_proof_fingerprint`
- `normalized_coverage_proof_fingerprint`
- `normalized_for_recommend_determinism`
- `normalized_recommendation_proof_fingerprint`

Lane A may reorder or comment these exports for clarity. It may not silently add new seam exports or move ownership out of the facade.

### Dependency graph

```text
analysis_core seam
  mod.rs
    ├── helper_surface.rs
    ├── decision_contract.rs
    └── proof_fingerprint.rs
          │
          ├──────────────► recommend.rs
          │                local writer / assembler
          │
          ├──────────────► verify.rs
          │                local parity gate
          │
          ├──────────────► promotion_artifacts.rs
          │                local validator consumer
          │
          ├──────────────► helper_surface.rs shim
          │                compatibility only
          │
          └──────────────► decision_kernel.rs shim
                           compatibility only

outside Lane A
  xtask CLI wiring
  latest-path lookup
  JSON rendering
  artifact schemas
  backend execution policy
```

### Invariants that must still be true after the slice lands

1. `analysis_core/*` remains the only semantic owner surface.
2. downstream consumers still produce the same stop-state truth.
3. proof fingerprints remain semantic, not timestamp-driven.
4. helper-surface handling remains a narrow classifier, not a policy engine.
5. decision derivation remains explicit, branch-bounded, and readable in one sitting.

### Production failure lens

- If `analysis_core/*` semantics drift during cleanup, `verify-decision-contract` must fail.
- If fingerprint normalization starts ignoring real semantic fields, artifact reuse becomes silently wrong.
- If shims or local validators regain semantic ownership, later consumer rewires will target the wrong boundary.
- If a consumer needs behavior change to accommodate Lane A, the seam was not actually frozen and the slice is scoped incorrectly.

## Implementation Contract

### Allowed write scope

Primary write files:

- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`

Allowed secondary proof surfaces only if unavoidable:

- seam-local tests inside those same files
- existing `xtask` tests that prove downstream parity

### Read-only proof surfaces

These files are validation targets, not write scope:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

Allowed exception:

- a compile-only proof fix that does not alter semantics, ownership, routing, or output meaning

If the only way forward is a semantic edit in one of these files, stop.

### Workstream dependency map

| Workstream | Primary files | Depends on | Done when |
| --- | --- | --- | --- |
| 1. Freeze seam facade | `analysis_core/mod.rs` | — | export inventory is explicit, grouped, and unchanged in meaning |
| 2. Freeze helper-surface semantics | `analysis_core/helper_surface.rs` | 1 | classifier and durable-hold tuples are explicit and fully tested |
| 3. Freeze decision-contract semantics | `analysis_core/decision_contract.rs` | 1 | all real branches are proven explicitly with the same stop-state truth |
| 4. Freeze proof-fingerprint semantics | `analysis_core/proof_fingerprint.rs` | 1 | semantic drift changes hashes, bookkeeping churn does not |
| 5. Proof wall sweep | seam files plus existing tests/commands | 1, 2, 3, 4 | all command parity checks stay green with no downstream drift |

### Workstream 1: Freeze the seam facade

Target file:

- `xtask/src/family/analysis_core/mod.rs`

Required work:

1. make the export inventory explicit and stable
2. group exports by semantic area, not accidental file order
3. add module-level ownership comments only if they remove ambiguity
4. confirm all approved seam entry points are re-exported from this facade and only this facade

Done when:

- a later consumer can target `analysis_core::{...}` without guessing which submodule owns which semantics

Stop and re-scope if:

- a missing export implies a new owner surface or a consumer-side ownership patch

### Workstream 2: Freeze helper-surface semantics

Target file:

- `xtask/src/family/analysis_core/helper_surface.rs`

Required work:

1. preserve the exact durable-hold tuple contract
2. preserve the exact helper-surface follow-on tuple contract
3. keep `classify_helper_surface()` narrow:
   - primary reason must be `unsupported_function_surface`
   - overlap family must stay `unknown`
   - `real_example_hits` must be positive
   - shape fingerprint must match the frozen helper-surface shape
4. add explicit tests for contradictory inputs and malformed fingerprint inputs

Done when:

- the file remains an exact classifier, not a policy-expansion surface

Stop and re-scope if:

- the classifier needs additional reason codes, broader overlap logic, or consumer-specific exceptions

### Workstream 3: Freeze decision-contract semantics

Target file:

- `xtask/src/family/analysis_core/decision_contract.rs`

Required work:

1. preserve the exact stop-state tuple returned by `decision_contract_stop_state_tuple()`
2. preserve the exact basis snapshot projection from recommendation analysis artifacts
3. preserve the five real decision branches:
   - promotion-ready candidate
   - plausible candidate blocked on missing or stale evidence
   - helper-surface follow-on
   - policy-interpretation blocker
   - default stop
4. add explicit tests for each branch instead of relying on transitive command coverage

Done when:

- `cargo xtask family corpus-decision --format json` keeps the current stop tuple unless the input basis actually changes

Stop and re-scope if:

- a fifth branch or a new policy interpretation surface is needed to explain existing behavior

### Workstream 4: Freeze proof-fingerprint semantics

Target file:

- `xtask/src/family/analysis_core/proof_fingerprint.rs`

Required work:

1. preserve the exact normalization fields for coverage, recommendation, and corpus decision artifacts
2. prove that timestamp, inventory-path, inventory-sha, and recommendation-delta churn do not change fingerprints when semantics are unchanged
3. prove that semantic-field drift does change fingerprints
4. keep serialization local and boring

Done when:

- artifact reuse remains semantic, not timestamp-driven

Stop and re-scope if:

- normalization requires schema changes or external helper layers to stay truthful

### Workstream 5: Hold downstream behavior fixed

Proof surfaces:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

Required rule:

- these files are read-only for Lane A unless a compile-only proof fix is unavoidable

Done when:

- command output and parity behavior stay unchanged after Lane A

Stop and re-scope if:

- any of these files need semantic edits, new ownership comments, or updated public truth

## Code Quality Guardrails

- No new module, trait, or helper layer.
- No semantic duplication between seam files and consumers.
- No policy broadening hidden inside naming cleanup.
- No file moves.
- Comments are allowed only where they lock ownership or invariants that tests alone do not make obvious.
- If a cleanup makes behavior easier to read but harder to prove, reject the cleanup.
- Bias toward explicit over clever. Four small obvious tests beat one meta-test nobody trusts.

## Validation and Test Strategy

### Required command proof gate

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Required outcomes:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `decision_basis_code = no_actionable_candidate`
- `required_next_action = record_stop_without_new_milestone`
- `overall_verdict = pass`
- all `xtask` tests green

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/analysis_core/mod.rs
    └── facade re-exports
        ├── [PLAN TEST] exact export inventory remains stable
        └── [PLAN TEST] helper, decision, and fingerprint APIs stay reachable from facade

[+] xtask/src/family/analysis_core/helper_surface.rs
    ├── classify_helper_surface()
    │   ├── [PLAN TEST] accepts exact helper-surface signal
    │   ├── [PLAN TEST] rejects wrong primary reason code
    │   ├── [PLAN TEST] rejects non-unknown overlap family
    │   ├── [PLAN TEST] rejects zero real-example hits
    │   └── [PLAN TEST] rejects malformed or non-matching fingerprint
    ├── durable_non_promotable_helper_surface_candidate_tuple()
    │   └── [PLAN TEST] exact tuple fields stay frozen
    └── helper_surface_follow_on_decision_tuple()
        └── [PLAN TEST] exact decision tuple stays frozen

[+] xtask/src/family/analysis_core/decision_contract.rs
    ├── decision_contract_stop_state_tuple()
    │   └── [PLAN TEST] exact stop tuple stays frozen
    ├── corpus_program_basis_snapshot()
    │   └── [PLAN TEST] basis snapshot projection stays exact
    ├── basis_snapshot_requires_helper_surface_follow_on()
    │   ├── [PLAN TEST] exact helper-surface blocker path
    │   └── [PLAN TEST] stale or missing evidence rejects helper-surface follow-on
    └── derive_corpus_program_decision_contract()
        ├── [PLAN TEST] promotion-ready branch
        ├── [PLAN TEST] blocked-on-evidence branch
        ├── [PLAN TEST] helper-surface follow-on branch
        ├── [PLAN TEST] policy-interpretation blocker branch
        └── [PLAN TEST] default stop branch

[+] xtask/src/family/analysis_core/proof_fingerprint.rs
    ├── normalized_coverage_proof_fingerprint()
    │   ├── [PLAN TEST] timestamp/path churn ignored
    │   └── [PLAN TEST] semantic cluster drift changes hash
    ├── normalized_recommendation_proof_fingerprint()
    │   ├── [PLAN TEST] generated_at and delta churn ignored
    │   └── [PLAN TEST] semantic decision drift changes hash
    └── normalized_corpus_program_decision_proof_fingerprint()
        ├── [PLAN TEST] generated_at churn ignored
        └── [PLAN TEST] semantic decision tuple drift changes hash

DOWNSTREAM PROOF SURFACES
===========================
[+] recommend.rs
    └── [PLAN TEST] recommendation.latest.json reuse behavior unchanged

[+] verify.rs
    └── [PLAN TEST] verify-decision-contract remains pass across all checks

[+] promotion_artifacts.rs
    └── [PLAN TEST] local validator contract still accepts frozen seam outputs

─────────────────────────────────
REQUIRED RESULT: all listed tests exist or are tightened, and all command proof
surfaces remain green with unchanged stop-state truth.
─────────────────────────────────
```

### Required test additions by file

| File | Test gap to close | Assertion that must be added |
| --- | --- | --- |
| `analysis_core/mod.rs` | facade inventory is only implied | prove approved exports remain reachable from the facade |
| `helper_surface.rs` | malformed fingerprint rejection is not explicit | malformed JSON and semantically wrong fingerprints must both reject classification |
| `helper_surface.rs` | contradictory signal coverage is incomplete | wrong reason code, non-`unknown` overlap, and `real_example_hits = 0` must all reject classification |
| `decision_contract.rs` | branch proof is too transitive | add one explicit test per real branch plus default stop |
| `proof_fingerprint.rs` | recommendation semantic drift test is missing | semantic decision change must alter recommendation fingerprint even when bookkeeping is stable |
| downstream proof wall | shim immutability is mostly social | verify command parity stays green and shims remain compatibility-only by review and unchanged behavior |

### Failure-mode coverage

| Failure mode | Test or proof surface | Error handling exists | User-visible outcome | Critical gap |
| --- | --- | --- | --- | --- |
| helper-surface cleanup widens classification | helper-surface unit tests + `verify-decision-contract` | yes | visible, stop-state parity fails | No |
| decision-contract cleanup changes stop tuple | decision-contract unit tests + `corpus-decision` | yes | visible, output tuple changes | No |
| recommendation fingerprint ignores real semantic drift | proof-fingerprint unit tests | partial until explicit test exists | silent artifact reuse bug | Yes until test is added |
| shims regain semantic ownership | manual file review + unchanged downstream behavior + green proof wall | partial | silent future boundary drift | Yes if shims are edited |
| consumer behavior changes to accommodate seam freeze | `cargo test -p xtask` + command proof wall | yes | visible in commands or tests | No |

Lane A is not complete if either critical gap remains open.

## Not In Scope

| Item | Why it is deferred |
| --- | --- |
| `recommend.rs` rewires | that is Lane B, not interface freeze |
| `verify.rs` rewires | that is Lane B, not interface freeze |
| `promotion_artifacts.rs` rewires or schema changes | validator policy stays local in Lane A |
| `helper_surface.rs` and `decision_kernel.rs` shim cleanup | deprecation churn is not required to freeze semantics |
| CLI wiring, latest-path lookup, JSON rendering | local orchestration, not seam semantics |
| `ORCH_PLAN.md` rewrite | this plan replaces execution intent for Lane A, not orchestration history |
| cross-crate extraction | current reuse pressure still does not justify it |
| TypeScript or broader backend work | M46 remains bounded proof only |
| recommendation-policy or family-selection follow-on | live basis still says stop |

## TODOS.md Handling

No new `TODOS.md` entry is required for Lane A.

Reason:

- this slice already has a bounded implementation owner and bounded write scope
- broader portability, backend, and extraction work is already deferred elsewhere
- inventing a new TODO here would duplicate existing follow-up surfaces without clarifying scope

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| freeze seam facade | `xtask/src/family/analysis_core/` | — |
| freeze helper-surface contract | `xtask/src/family/analysis_core/` | freeze seam facade |
| freeze decision-contract semantics | `xtask/src/family/analysis_core/` | freeze seam facade |
| freeze fingerprint semantics | `xtask/src/family/analysis_core/` | freeze seam facade |
| seam proof sweep | `xtask/src/family/analysis_core/`, existing `xtask` tests, command proof wall | all prior steps |

### Parallel lanes

Sequential implementation, no safe parallelization opportunity.

Why:

- every real step touches the same primary module directory
- helper-surface, decision-contract, and proof-fingerprint semantics all roll up into one shared seam vocabulary
- the proof wall is not lane-local, it is the same parity surface for every step

Trying to split this across worktrees creates merge conflict and semantic skew risk faster than it creates throughput.

### Execution order

1. freeze the facade contract in `analysis_core/mod.rs`
2. harden `helper_surface.rs`, `decision_contract.rs`, and `proof_fingerprint.rs`
3. run the seam proof sweep
4. stop immediately if any downstream behavior drift appears

### Conflict flags

- Any lane split inside `xtask/src/family/analysis_core/` is a merge-conflict risk.
- Any lane that touches downstream consumers turns this into Lane B and violates the plan.
- Any lane that introduces new helpers to reduce merge conflict is itself overbuilding the slice.

## Acceptance Checklist

- [ ] `analysis_core/mod.rs` is the sole approved seam facade
- [ ] helper-surface tuple semantics are frozen and explicitly tested
- [ ] decision-contract tuple semantics are frozen and explicitly tested
- [ ] proof-fingerprint normalization rules are frozen and explicitly tested
- [ ] compatibility shims remain compatibility-only and unchanged
- [ ] `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` keep the same behavior
- [ ] `collect_signals.sh` still reports the same stop-state summary
- [ ] `cargo xtask family verify-decision-contract --format json` still passes
- [ ] `cargo xtask family corpus-decision --format json` still emits the same stop tuple
- [ ] `cargo test -p xtask` stays green
- [ ] no scope leakage into consumers, CLI surfaces, schemas, or backend policy

## Completion Summary

- Step 0: scope accepted as-is, bounded to `analysis_core/*`
- Architecture: existing seam reused, not reinvented
- Code quality target: explicit over clever, minimal diff, no new abstraction layer
- Test strategy: seam-local branch coverage plus command proof wall
- Failure modes: `2` critical gaps remain open if recommendation semantic-drift proof or shim immutability discipline is skipped
- Not in scope: written
- What already exists: written
- TODOS.md updates: none required
- Parallelization: single lane, sequential only

## Next Step

Implement Lane A only.

Do not reopen consumer rewires, command-surface adoption, docs sync, or backend scope while this slice is in flight. If Lane A lands cleanly, the next honest discussion is whether a separate Lane B plan is still worth doing against the now-frozen seam.
