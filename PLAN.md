<!-- /autoplan restore point: /home/azureuser/.gstack/projects/atomize-hq-spec/feat-i8-final-proof-run-autoplan-restore-20260525-190351.md -->
# Corrected Stored-Truth-Strict Category Qualification Respin Plan

Status: **authoritative implementation plan**  
Milestone family: **`operator-consumer-tooling`**  
Executable wedge: **`corrected stored-truth-strict category qualification respin for benchmark/status/export/snapshot consumers`**  
Implementation readiness: **ready-now**  
Base branch: **`feat/i8-final-proof-run`**  
Planning date: **2026-05-25**

Primary authority inputs:

- handoff: `.codex/handoffs/2026-05-25-183145-m101-correction-respin.md`
- corrective design note: `docs/category_truth_contract_correction_v0.1.md`
- original contract doc: `docs/category_truth_contract_v0.1.md`
- backlog anchor: `TODOS.md` under `M101 backlog`
- current authority context: `PLAN.md` and `ORCH_PLAN.md` from the prior M101 attempt
- live repo state observed on this branch:
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - both currently report `BENCH-SERVICE` as `benchmark_status=passing`, `accounting_status=valid`, `gate_status=open`, `positive_credit_cases=4`

## Why This Respin Exists

The milestone is still right. The implementation shape was not.

The old M101 direction correctly aimed for:

- one registry-owned category truth substrate
- one shared qualification vocabulary
- benchmark, status, export, and snapshot/readability parity

But the reviewed implementation crossed the repo's honesty boundary by allowing
read-side consumers to derive category-bearing truth from refreshed semantic
projection. That would create a second truth producer outside `spec test`.

This respin keeps the shared contract and benchmark honesty gains, then rebuilds
consumer projections so every read-side surface interprets stored semantic truth
only.

## Frozen Decisions

These are fixed for this wedge:

1. `semantic_review` remains producer-owned truth.
2. Only `spec test` may refresh semantic review truth.
3. `category_qualification` is read-side interpretation of stored semantic truth.
4. No read-only path may use `SemanticProjectionMode::Refresh` to mint fresher
   category truth for benchmark, status, export, or snapshot output.
5. The first-scope registry remains the same four seam rows:
   - `sum.discount_strategy.v1`
   - `data.pricing_quote.v1`
   - `unsupported.sum.v1`
   - `unsupported.data.v1`
6. Service seam siblings stay visible but unqualified in this wedge.
7. `benchmarks/labels.json` is not relabeled in this wedge.
8. `.spec.passport.json` schema is not widened to persist
   `category_qualification`.
9. If `spec status` and `spec export` gain additive
   `category_qualification`, their public JSON schema versions bump.
10. For seam category-claim candidates, positive benchmark credit must require
    `category_qualification.claim_status == supported_qualified` plus
    `positive_credit_eligibility == eligible`.
11. Legacy `semantic_support_status` fallback may remain only for non-seam
    benchmark rows. It must not award positive credit to seam rows when
    `category_qualification` is absent.
12. `spec export` must choose one exact no-passport behavior for seam rows:
    if no passported semantic review exists, export may still emit the unit row,
    but it must emit no supported semantic/category claim. The allowed first
    landing shape is:
    - `projected_units[]` row present
    - `semantic_review = null`
    - `category_qualification = null`
    - never `supported_qualified`
13. The refresh audit is owned by named read-side helpers, not whole-file grep.
    The implementation must identify the exact benchmark/status/export/snapshot
    read-side projection helpers and prove those helpers use preserved semantic
    truth only.

## Exact File Scope

### Expected code edits

- `spec-core/src/category_truth.rs`
  - keep the registry and qualification substrate
  - tighten it around preserved semantic truth only
- `spec-core/src/lib.rs`
  - export the corrected category-truth module
- `spec-core/src/semantic_review.rs`
  - keep producer-owned routing and descriptor hooks
  - do not widen read-side refresh semantics
- `spec-core/src/benchmark.rs`
  - keep positive-credit gating and benchmark invalidation logic
  - consume preserved semantic review truth only
- `spec-core/src/export.rs`
  - add or keep additive qualification on export surfaces only if it is derived
    from preserved/passported semantic review
  - remove any refresh-based category truth minting path
- `spec-cli/src/commands.rs`
  - route benchmark/status/export/snapshot consumers through one preserved-truth
    qualification helper
  - remove any `SemanticProjectionMode::Refresh` path that feeds
    `category_qualification` on read-side surfaces

### Expected tests and artifact refresh

- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/rust_v1_closure.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-service-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-service-billing-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-service-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-service-billing-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`

### Explicitly out of scope

- `benchmarks/labels.json`
- `.spec.passport.json` schema changes
- new live analysis surfaces
- widened support for service seam siblings
- broader seam alias/sibling cleanup
- recommendation-policy or corpus-run work
- ORCH plan expansion beyond what this corrected wedge needs

## Keep / Drop Commit Intent

The old branch is reference material, not a branch to land.

### Keep or adapt

| Commit | Intent | Use in respin |
| --- | --- | --- |
| `397837e` | add category truth contract spine | strongest salvage candidate; keep the registry and qualification types after a preserve-only audit |
| `688d673` | pass semantic review into benchmark truth builders | keep the idea; benchmark projection needs semantic-review input, but only preserved truth |
| `81aa154` | invalidate service benchmark category mismatches | keep the benchmark honesty outcome and test expectations |

### Drop or re-implement narrowly

| Commit | Why it does not land as-is | Respin instruction |
| --- | --- | --- |
| `c24d4f7` | projected export truth was built in the same implementation wave that introduced read-side refresh risk | rebuild export from preserved passport truth only |
| `29fd15b` | CLI truth integration touched the risky status/export paths directly | do not cherry-pick wholesale; re-thread through one preserved-truth helper |
| `701019e` | fixture refresh captured the old export shape | regenerate after corrected export semantics are in place |
| `3d81942` | snapshot preservation landed after the read-side truth drift already existed | rebuild snapshot parity after live benchmark projection is corrected |
| `e2af919` | refreshed semantic-review benchmark inputs are suspect under the corrected contract | re-evaluate case by case; default to drop |
| `3ee00b4` | compile-surface fix was tied to the exploratory branch line | re-apply only if the corrected respin actually needs it |

## Corrected Contract Shape

The implementation contract for this respin is:

- producer truth:
  - authored unit specs
  - passports
  - molecule evidence
  - `semantic_review` refreshed only by `spec test`
- consumer truth:
  - `CategoryQualification`
  - benchmark case accounting
  - `spec status --format json`
  - `spec export`
  - snapshot/readability projection

The consumer rule is simple:

> if preserved semantic truth is missing, stale, unsupported, or descriptor-mismatched,
> the consumer may surface that failure, but it may not mint a fresher supported claim.

## Implementation Slices

### Slice 1: Re-establish the contract spine

Deliverables:

- land `spec-core/src/category_truth.rs` as the shared registry and stable
  reason-code surface
- keep the qualification vocabulary small:
  - `supported_qualified`
  - `unsupported_qualified`
  - `unqualified`
- ensure qualification takes preserved semantic-review input, not a refresh-mode
  projection
- keep descriptor approval narrow so service siblings remain unqualified

Proof:

- `cargo test -p spec-core`

### Slice 2: Restore benchmark honesty first

Deliverables:

- thread preserved semantic-review truth into benchmark case qualification
- keep `counts_as_supported_positive` gated on explicit supported qualification
- remove or fence the seam-row legacy fallback so
  `semantic_support_status` alone cannot award positive credit for seam
  category candidates
- make full positive benchmarks invalid when supported-labeled seam cases are
  present but unqualified
- preserve unsupported visibility without positive credit

Expected contract outcome:

- `BENCH-ECOM` full stays passing
- `BENCH-SERVICE` full flips away from `passing/valid`
- target state for `BENCH-SERVICE` full:
  - `accounting_status = invalid`
  - `benchmark_status = invalid`
  - `gate_status = open`

Proof:

- `cargo test -p spec-cli --test rust_v1_service`
- `cargo test -p spec-cli --test rust_v1_closure`
- `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
- `cargo run -p spec-cli -- status examples/service/units --format json`

### Slice 3: Rebuild status and export as stored-truth consumers

Deliverables:

- add additive `category_qualification` only where the surface is reading
  preserved/passported semantic review
- remove any status/export path that reaches `SemanticProjectionMode::Refresh`
  for category qualification
- ensure a missing passport never yields `supported_qualified`
- pin exact no-passport export behavior for seam rows:
  - keep the `projected_units[]` row
  - do not synthesize `semantic_review`
  - do not synthesize `supported_qualified`
  - allow only `semantic_review = null` with `category_qualification = null`
- ensure stale stored truth remains stale instead of being silently refreshed by
  a consumer
- bump public JSON schema versions when additive qualification is introduced

Proof:

- `cargo test -p spec-cli --test cli`
- `cargo test -p spec-cli --test m14_regressions`
- `cargo run -p spec-cli -- export examples/ecommerce/units`
- `cargo run -p spec-cli -- export examples/service/units`

### Slice 4: Re-freeze snapshot and artifact parity

Deliverables:

- snapshot output reuses the same benchmark qualification result as live
  projection
- readability artifacts continue to report readability state, not smuggled
  category-truth state
- fixture refresh happens only after slices 1 through 3 are green

Proof:

- `cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM`
- `cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE`

## Required Regression Cases

These are mandatory. The respin is not done without all of them.

| Case | Why it exists | Expected result |
| --- | --- | --- |
| missing passport seam unit | protects the no-synthetic-truth boundary | export keeps the `projected_units[]` row but emits `semantic_review = null`, `category_qualification = null`, and never `supported_qualified` |
| stale stored seam truth | protects against read-side refresh minting | consumer output may stay stale or unqualified, but may not become fresher than stored truth |
| service sibling with supported benchmark label | protects benchmark honesty | case remains visible, unqualified, and non-credit-bearing; full `BENCH-SERVICE` becomes invalid/open |
| canonical ecommerce seam row | protects the happy path | remains `supported_qualified` and credit-eligible |
| terminal unsupported seam row | protects additive unsupported visibility | may be `unsupported_qualified`, but never positive-credit eligible |

## Failure Modes Registry

| Risk | Trigger | Guard |
| --- | --- | --- |
| read-side truth minting returns through a helper | status/export/snapshot accidentally reuse refresh projection | name the exact read-side helpers and prove they use preserve-mode only, then cover with fixture regressions |
| benchmark honesty fixed but status/export still drift | only benchmark code is corrected | require parity assertions across service status and export fixtures |
| seam rows still receive positive credit through legacy fallback | benchmark keeps `semantic_support_status` shortcut for seam candidates | add a targeted benchmark regression that seam candidates require `category_qualification`, and keep legacy fallback non-seam-only |
| missing passport path still looks supported | export/status silently synthesize semantic review | add explicit no-passport service fixture assertions |
| stale proof gets hidden by live recompute | read-side consumer re-evaluates current authored truth | add stale-passport regression and compare against stored output |
| fixture refresh masks a logic bug | artifacts are updated before invariants are asserted | refresh fixtures only after targeted tests and live CLI checks pass |

## Proof Commands

Run these in order before calling the wedge complete:

```bash
cargo test -p spec-core
cargo test -p spec-cli --test rust_v1_service
cargo test -p spec-cli --test rust_v1_closure
cargo test -p spec-cli --test cli
cargo test -p spec-cli --test m14_regressions
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Required code-review audit before calling the wedge complete:

- name the exact benchmark/status/export/snapshot read-side helpers that project
  semantic/category truth
- verify those helpers use preserved semantic truth only
- verify any remaining `SemanticProjectionMode::Refresh` callsites are limited to
  write-path or explicitly non-read-side flows

## Done Means

This respin is complete only when all of the following are true:

1. the repo has one shared category-truth registry and qualification contract in
   `spec-core`
2. benchmark, status, export, and snapshot consumers all derive qualification
   from preserved semantic truth only
3. a seam benchmark row cannot receive positive credit through legacy
   `semantic_support_status` fallback alone
4. a unit with no passport does not surface `supported_qualified`
   and does not synthesize seam semantic review on export
5. the no-passport export shape is stable and explicit for machine readers
   rather than left to consumer guesswork:
   - `projected_units[]` row present
   - `semantic_review = null`
   - `category_qualification = null`
6. stale stored semantic truth is never upgraded by a read-side consumer into
   fresher category truth
7. `BENCH-ECOM` remains a passing positive wall
8. `BENCH-SERVICE` no longer reports a full supported-positive success while
   its seam siblings remain unqualified
9. status/export schema bumps and fixture refreshes are complete wherever
   additive `category_qualification` is exposed
10. `.spec.passport.json` remains free of persisted `category_qualification`

## Not Doing In This Plan

- landing the old `feat/m101-category-truth-registry` branch
- inventing a second semantic/category truth plane
- widening supported seam truth to rescue service siblings
- relabeling benchmark cases to avoid the mismatch
- mixing this contract repair with separate family-analysis or seam-substrate
  roadmap work
