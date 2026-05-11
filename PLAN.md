<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260510-215928.md -->
# M47 Post-M46 Shared-Core Portability Follow-On Authority Plan

Status: **authority plan**  
Milestone family: **shared-core-portability**  
Implementation readiness: **authority artifact ready for review**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Current primary head: **`fff21c5`**  
Last rewritten: **2026-05-11**

Primary source artifacts:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260510-215928.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-221405.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260510-215928.md`

Related repo surfaces:
- `.runs/m46_helper_aware_monotone_up_typescript/closeout.md`
- `.runs/m40_plus_selector_contract_hardening/replay-inputs/restore-point.md`
- `ORCH_PLAN.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `xtask/src/family/analysis_core/`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`

## Executive Verdict

M46 closed one bounded TypeScript truth surface.

It did not settle the broader shared-core portability question. The repo still needs one fresh post-M46 authority artifact that says, without hand-waving, which family-analysis decision semantics are actually shared across consumers and which surfaces remain local to `xtask`, local to Rust, or milestone-specific.

M47 is that artifact. Nothing more.

This is an authority-only milestone. It does not authorize a new family wedge, a broader TypeScript lane, a crate split, or backend widening.

## Goal

Freeze the post-M46 portability boundary tightly enough that a later implementation milestone can be judged honestly.

This plan is complete only if it names, explicitly and in one place:

1. the exact candidate seam
2. the exact surfaces that must stay local
3. the live proof floor
4. the trigger table for any later implementation milestone
5. the exact non-goals that block family churn and backend widening
6. the first honest future execution split, if a trigger ever fires

## Live Validated Basis

### Branch truth, verified on 2026-05-11

The current branch truth is live, not inherited folklore:

- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
- `cargo xtask family corpus-decision --format json`
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_status = "not_recommended"`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask`
  - `146` tests passed

Cargo emitted package-cache and build-directory lock waits during the command runs. That is operational noise, not semantic failure. The semantic outputs above are the truth surface this plan is allowed to rely on.

### Why this matters

The branch is currently doing the honest thing: refusing to invent another family milestone from weak evidence.

That refusal does not answer the portability question by itself. It tells the repo what **not** to do next. It does not define what logic is actually shared, what remains local, or what proof would justify a future portability claim.

That missing contract is the M47 job.

## Scope Challenge

### What already exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| durable helper-surface classification | `xtask/src/family/analysis_core/helper_surface.rs` | shared seam candidate |
| bounded corpus-program decision derivation | `xtask/src/family/analysis_core/decision_contract.rs` | shared seam candidate |
| normalized semantic proof fingerprints | `xtask/src/family/analysis_core/proof_fingerprint.rs` | shared seam candidate |
| write-side caller of shared semantics | `xtask/src/family/recommend.rs` | counts as owner-adjacent producer, not extraction pressure by itself |
| independent read-side consumer | `xtask/src/family/verify.rs` | real reuse pressure, first independent consumer |
| compatibility re-export wrappers | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs` | local shims, not seam owners |
| artifact schemas and validation | `xtask/src/family/promotion_artifacts.rs` | local contract surface, not portability-safe shared core |
| CLI wiring and path lookup | `xtask/src/family/mod.rs`, `xtask/src/family/paths.rs`, `xtask/src/lib.rs` | local orchestration only |
| bounded second-language proof | `.runs/m46_helper_aware_monotone_up_typescript/closeout.md` | proof context only, not broad portability proof |
| prior authority framing | `.runs/m40_plus_selector_contract_hardening/replay-inputs/restore-point.md` | reuse and refresh, do not reinvent |

### Minimum complete M47

M47 is the smallest honest lake only if it delivers all of the following in this file:

1. one concrete seam definition updated for post-M46 truth
2. one explicit shared-vs-local ownership map
3. one exact trigger table for later implementation
4. one proof floor tied to live commands and current artifacts
5. one explicit non-goals block
6. one future implementation split with dependency order and parallelization rules

Anything less is another ambiguous checkpoint note dressed up as strategy.

### Complexity, completeness, distribution

- Files intentionally changed by M47: `PLAN.md`
- New runtime classes or services: `0`
- New distribution work: none
- New backend support: none

This is the correct size.

If the plan grows into backend widening, schema churn, family selection, or cross-crate extraction, it has failed the scope test.

## Authorization Boundary

The live branch stop-state is:

```text
recommendation_status = insufficient_real_corpus
decision_status = not_recommended
decision_action = stop
decision_basis_code = no_actionable_candidate
required_next_action = record_stop_without_new_milestone
```

That stop-state still governs family-selection truth.

### Authorized now

- one authority artifact that refreshes the shared-core portability boundary against post-M46 truth
- one explicit contract for what is shared vs local
- one trigger table for later implementation

### Not authorized now

- another Rust-family wedge
- broader TypeScript execution work
- local seam extraction by momentum
- cross-crate extraction by adjacency
- generic multi-backend architecture work
- renewed corpus or recommendation-policy churn

Critical rule: M47 does **not** turn the current stop-state into implementation approval. It only makes future approval criteria explicit.

## Candidate Seam

The shared seam remains the smallest reusable family-analysis decision boundary:

```text
candidate seam
  helper-surface durable-hold classification
  bounded corpus-program decision derivation
  normalized proof-fingerprint helpers
```

Current code anchors:

- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`

Current consumers:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`

### Shared vs local ownership map

| Surface | Ownership | Why |
| --- | --- | --- |
| `analysis_core/helper_surface.rs` | shared seam | owns durable helper-surface classification semantics |
| `analysis_core/decision_contract.rs` | shared seam | owns bounded next-action derivation and locked stop-state truth |
| `analysis_core/proof_fingerprint.rs` | shared seam | owns semantic fingerprint normalization independent of artifact churn |
| `recommend.rs` | local consumer | uses shared semantics, but also owns command execution, artifact write policy, and recommendation assembly |
| `verify.rs` | local consumer | uses shared semantics, but owns verifier-only JSON output and artifact loading |
| `helper_surface.rs`, `decision_kernel.rs` | local compatibility wrappers | re-export shims only; adjacency is not ownership |
| `promotion_artifacts.rs` | local artifact boundary | schema and validator surface, not portability-safe shared semantics |
| `mod.rs`, `paths.rs`, `lib.rs` | local orchestration | CLI dispatch, path lookup, write locations, command plumbing |
| target-language execution policy | local backend ownership | M46 proved one bounded TypeScript surface, not a shared execution layer |

### Hard boundary

The following must stay local even if a later seam move is authorized:

```text
must stay local
  xtask CLI wiring
  artifact latest-path lookup
  command-specific JSON rendering
  proof-wall file locations
  milestone-specific closeout wording
  backend lowering details
  TypeScript execution policy
  spec generate/build/test ownership
```

The repo must not extract "all of `xtask/src/family/`" just because the files live near each other. That is adjacency bias, not architecture.

## Architecture Surface

### Dependency graph

```text
unsupported coverage truth
        │
        ▼
recommend.rs
  collects coverage and recommendation artifacts
        │
        ▼
analysis_core/helper_surface.rs
  durable helper-surface classification
        │
        ▼
analysis_core/decision_contract.rs
  bounded corpus-program decision derivation
        │
        ▼
analysis_core/proof_fingerprint.rs
  semantic fingerprint normalization
        │
        ├──────────────► recommendation.latest.json
        │
        ├──────────────► corpus-program-decision.latest.json
        │
        └──────────────► verify.rs
                         re-derives and verifies parity against frozen floor

outside seam, must stay local
  promotion_artifacts.rs
  mod.rs
  paths.rs
  lib.rs
  render_json_bytes
  target-language execution policy
```

### Production-failure lens

For each real codepath this plan depends on:

- if `analysis_core/*` semantics drift without the artifacts changing, `verify.rs` must fail parity
- if artifact schemas drift without the shared semantics changing, `promotion_artifacts.rs` may change locally without broadening the seam
- if someone widens TypeScript claims from M46 proof alone, the plan must reject that as a category error

That is the whole architecture game here. Keep shared semantics tiny. Keep everything else local until proof says otherwise.

## Alternatives Considered

| Approach | Pros | Cons | Verdict |
| --- | --- | --- | --- |
| Do nothing after M46 | obeys the stop-state literally | leaves the portability boundary implicit and re-arguable | reject |
| Refresh the bounded authority artifact now | smallest honest lake, resolves ambiguity without fake implementation | requires discipline to stay explicit | choose |
| Broaden TypeScript immediately | visible product motion | skips the shared/local honesty step and spends an innovation token early | reject |
| Reopen Rust-family promotion | familiar workflow | directly contradicts current stop-state | reject |
| Jump to cross-crate extraction | sounds architectural | spends scope before real reuse pressure exists | reject |

## Trigger Table

Current reuse pressure is:

- `recommend.rs` is the owner-adjacent producer
- `verify.rs` is the first independent consumer

That is real signal, but it is still not enough to authorize extraction.

| Follow-on | Current state after M46 | Exact trigger | Authorized next move | Still does not count |
| --- | --- | --- | --- | --- |
| local extraction inside `xtask/src/family/` | not triggered | one additional independent in-tree consumer, beyond `recommend.rs` and `verify.rs`, needs the same bounded `analysis_core/*` semantics | author a local implementation milestone for a still-local seam extraction | `recommend.rs` alone, `verify.rs` alone, compatibility shims, artifact validators, cleanup-only edits |
| cross-crate family-analysis shared core | not triggered | one non-`xtask` crate needs the same bounded semantics without importing local command glue | author a separate implementation plan that may cross crate boundaries | in-tree `xtask` reuse pressure only |
| broader portability or backend claim | not triggered | a concrete backend or portability consumer needs the same bounded semantics **and** the shared/local boundary survives proof without pulling in execution policy | author a separate portability implementation plan | bounded M46 TypeScript proof by itself |
| renewed family-selection work | not triggered | live evidence names a specific next-family winner with stronger proof than the current stop-state | author a separate family-promotion plan | unsupported pressure, historical momentum, or "we should probably keep going" |

## Proof Floor

### Required commands

```bash
.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

### Expected approval truth

- `verify-decision-contract` stays green
- `corpus-decision` stays on `stop` with `record_stop_without_new_milestone`
- `xtask` analysis-core and verifier coverage stays green
- M46 remains the only second-language proof this plan cites

### Live verified subset for this rewrite

As of 2026-05-11:

- `cargo xtask family verify-decision-contract --format json` returned `overall_verdict = "pass"`
- `cargo xtask family corpus-decision --format json` returned `decision_action = "stop"` and `required_next_action = "record_stop_without_new_milestone"`
- `cargo test -p xtask` passed `146` tests

## Validation and Test Strategy

This is an authority-only milestone, so the proof surface is command truth plus manual contract review, not new runtime behavior.

### Coverage map

```text
M47 PROOF FLOOR
================
[GREEN] cargo xtask family verify-decision-contract --format json
[GREEN] cargo xtask family corpus-decision --format json
[GREEN] cargo test -p xtask
[MANUAL] PLAN.md remains authority-only
[MANUAL] PLAN.md keeps analysis_core/* as seam owner
[MANUAL] PLAN.md keeps wrappers, artifact schemas, and CLI glue local
[MANUAL] M46 closeout remains the only second-language proof cited
```

### Test expectations

| Surface | Why it matters | Coverage type | Required result |
| --- | --- | --- | --- |
| `analysis_core/helper_surface.rs` | frozen helper-surface contract | unit tests | still green |
| `analysis_core/decision_contract.rs` | locked stop-state and follow-on derivation | unit tests + command proof | still green |
| `analysis_core/proof_fingerprint.rs` | semantic stability vs artifact churn | unit tests | still green |
| `verify.rs` | first independent consumer and parity gate | unit tests + command proof | still green |
| `PLAN.md` authorization boundary | prevents false implementation claims | manual review | must stay authority-only |
| M46 closeout truth | prevents overclaiming TypeScript portability | artifact review | must stay bounded |

### Regression rule for this milestone

If a future edit changes this plan from "authority-only" to "implementation-authorizing" without satisfying the trigger table, treat that as a regression. The plan is wrong even if the code still compiles.

## Error and Rescue Registry

| Failure | Detection surface | Immediate rescue | Why this is enough |
| --- | --- | --- | --- |
| someone reads `stop` as "no portability work needed" | proof floor vs plan language | restate that stop blocks family churn, not boundary definition | keeps refusal from masquerading as resolution |
| someone reads M46 as broad backend proof | M46 closeout and this plan | point back to the explicit unsupported `.test.spec --target-language typescript` truth | blocks fake portability confidence |
| command glue gets pulled into seam scope | ownership map and architecture review | keep `promotion_artifacts.rs`, `paths.rs`, `mod.rs`, `lib.rs`, and rendering local | prevents adjacency from becoming fake cohesion |
| someone claims extraction from current reuse pressure | trigger table | keep implementation frozen until a second independent consumer appears | prevents stealth architecture work from weak signal |

## Failure Modes Registry

| Failure mode | Test or proof surface | Error handling exists | User-visible outcome | Critical gap |
| --- | --- | --- | --- | --- |
| family churn resumes by momentum | `collect_signals.sh` plus `corpus-decision` | yes | visible, because the stop-state would drift | No |
| M46 is overstated as broad TypeScript support | M46 closeout + plan review | yes | visible, because claims would exceed landed proof | No |
| wrappers are mistaken for seam owners | ownership map + architecture section | yes | visible, because the plan names the true owners | No |
| local extraction is treated as already authorized | trigger table + proof floor | yes | visible, because the trigger row is still false | No |

No critical gaps are open if the repo follows the boundary in this file.

## Not In Scope

The following are explicitly deferred:

- promoting another Rust family
- renewing corpus or recommendation-policy work
- broadening TypeScript beyond what M46 landed
- generic multi-backend execution work
- `spec-core` or cross-crate extraction by default
- schema churn
- command-path refactors justified only by adjacency
- dead-code cleanup unless a later implementation milestone scopes it directly

## Deferred to Existing TODO Surfaces

No new `TODOS.md` item is created by M47.

Reason: this milestone is not missing implementation tasks, it is intentionally refusing to authorize them yet. The deferred work already belongs to later implementation milestones, gated by the trigger table above.

## Worktree Parallelization Strategy

### M47 itself

Sequential implementation, no parallelization opportunity.

Reason: this milestone is one authority artifact in `PLAN.md`. Splitting one boundary document across worktrees would create merge noise without buying real throughput.

### First authorized implementation milestone, if a trigger later fires

#### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| freeze seam interface | `xtask/src/family/analysis_core/` | — |
| rewire in-tree consumers | `xtask/src/family/` | freeze seam interface |
| docs and authority sync | repo-root plans, `.runs/`, docs artifacts | freeze seam interface |
| command-surface adoption | `xtask/src/family/`, `xtask/src/` | rewire in-tree consumers |

#### Parallel lanes

- `Lane A`: freeze seam interface
- `Lane B`: rewire in-tree consumers, after `Lane A`
- `Lane C`: docs and authority sync, after `Lane A`
- `Lane D`: command-surface adoption, after `Lane B`

#### Execution order

Launch `Lane A` first.

After `Lane A` merges or is proven stable, launch `Lane B` and `Lane C` in parallel.

Launch `Lane D` only after `Lane B` lands, because command-surface adoption depends on the consumer rewires being settled.

#### Conflict flags

- `Lane B` and `Lane D` both touch `xtask/src/family/`, so they should stay sequential
- `Lane C` is safe to run beside `Lane B` because it lives in docs and authority artifacts, not the runtime modules

## Acceptance Checklist

- [ ] one concrete seam definition updated for post-M46 truth
- [ ] one explicit shared-vs-local ownership map
- [ ] one exact trigger table
- [ ] one proof floor tied to live commands and current artifacts
- [ ] one explicit non-goals block
- [ ] one future execution split for the first authorized implementation milestone
- [ ] zero implementation authority claimed beyond this artifact

## Next Actions

1. Review this authority plan as the single source of truth for post-M46 portability boundaries.
2. Keep implementation frozen unless a future trigger in this plan turns true.
3. If a trigger turns true, author a separate implementation milestone instead of mutating M47 into code scope.
