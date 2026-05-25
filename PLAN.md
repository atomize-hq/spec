# M101: Category Truth Registry and Consumer Qualification Plan

Status: **authoritative implementation plan**  
Milestone: **M101**  
Implementation readiness: **ready to execute**  
Plan scope: **land one explicit category-truth contract for seam-backed support claims across benchmark accounting, `spec status`, `spec export`, and snapshot/readability projections**  
Base branch: **main**  
Working branch: **`feat/m101-category-truth-registry`**  
Last rewritten: **2026-05-25**

Supersedes:

- the prior `I8: Rust V1 Final Proof Run Plan`

Primary authority inputs:

- design anchor: `docs/category_truth_contract_v0.1.md`
- backlog anchor: `TODOS.md` under `M101 backlog`
- current benchmark registry: `benchmarks/labels.json`
- current producer and consumer code:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/benchmark.rs`
  - `spec-core/src/export.rs`
  - `spec-core/src/passport.rs`
  - `spec-cli/src/commands.rs`
- current contract suites and read-side fixtures:
  - `spec-cli/tests/rust_v1_service.rs`
  - `spec-cli/tests/rust_v1_closure.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/benchmarks/*.json`
  - `benchmarks/snapshots/*.snapshot.json`
  - `benchmarks/reviews/*.readability.review.json`

## Executive Summary

The repo already projects semantic-review truth and already computes benchmark,
status, export, and snapshot surfaces. The bug class is not missing data. The
bug class is consumer drift: a read-side surface can still over-credit a seam
category from partial truth such as compatibility key alone, support status
alone, or benchmark label alone.

This wedge fixes that by introducing one producer-owned registry plus one
shared qualification function in `spec-core`, then wiring every current
seam-category consumer to that contract. After this lands, no current consumer
may claim supported category truth or award positive supported benchmark credit
unless qualification returns an explicit supported result.

The most visible product change is intentional: `BENCH-SERVICE` must stop
projecting a full-scope `passing` result when its supported-labeled seam cases
are not producer-qualified as supported.

## Success Sentence

When this plan is complete, the repo can say this sentence honestly:

> Every current seam-category consumer in `spec` uses one shared,
> producer-owned qualification contract. Benchmark labels, health state, and
> compatibility-key folklore can no longer widen supported category claims or
> positive benchmark credit on their own.

## Frozen Decisions

These are not open during implementation:

- the authoritative category registry lives in Rust code under `spec-core`, not
  in a new checked-in JSON registry
- first-scope registry coverage is exactly four rows:
  - `sum.discount_strategy.v1`
  - `data.pricing_quote.v1`
  - `unsupported.sum.v1`
  - `unsupported.data.v1`
- producer truth outranks benchmark labels
- the current service seam mismatch remains visible in this wedge; it is not
  rescued into supported truth
- category qualification is projected read-side truth only and is **not**
  persisted into on-disk passports
- snapshot and readability surfaces consume the same benchmark qualification
  result; readability freshness is not repurposed as an accounting signal
- benchmark labels schema stays at `1`; this wedge changes consumer
  projections, not the label-file format
- `spec status --format json` and `spec export` are published machine surfaces,
  so their schema versions must bump when `category_qualification` is added
- unsupported terminal categories qualify as `unsupported_qualified` without a
  descriptor-id check; descriptor approval is required only for supported rows
- do not ship an unused extra failure vocabulary; only add reason codes the
  first landing actually emits

## Problem Statement

The failure class is any path where a consumer infers category-backed support
or positive benchmark value without explicit qualification.

Current repo reality already shows the gap:

- `benchmarks/labels.json` marks `billing/discount_strategy` and
  `billing/pricing_quote` as `classification: supported`
- producer-owned semantic review routes those units to `unsupported.sum.v1` and
  `unsupported.data.v1`
- current benchmark logic correctly sets
  `counts_as_supported_positive = false` for those cases
- but the enclosing full `BENCH-SERVICE` benchmark still projects
  `benchmark_status = passing`

That last line is too optimistic. The case-level truth is already saying
"this supported claim is not actually producer-qualified." The benchmark-level
truth must stop pretending that full supported closure still passed.

## Scope Challenge

### What already exists

| Sub-problem | Existing owner | Reuse decision |
| --- | --- | --- |
| producer-owned compatibility routing | `spec-core/src/semantic_review.rs` | reuse; do not build a second routing system |
| benchmark projection and accounting skeleton | `spec-core/src/benchmark.rs` | reuse; tighten through qualification rather than rewrite |
| status JSON projection | `spec-cli/src/commands.rs` | reuse; add qualification beside existing semantic review |
| export bundle construction | `spec-core/src/export.rs` | reuse; add additive projected truth rather than new export command |
| projected passport truth assembly | `spec-core/src/passport.rs` | reuse for semantic-review projection only; do not persist category qualification |
| benchmark truth collection from current proof surfaces | duplicated in `spec-cli/src/commands.rs` benchmark and snapshot paths | collapse to one shared helper instead of editing multiple truth carriers independently |
| frozen service/ecommerce benchmark contract suites | `spec-cli/tests/rust_v1_service.rs`, `spec-cli/tests/rust_v1_closure.rs` | keep and update expectations |
| read-side fixtures and snapshots | `spec-cli/tests/fixtures/benchmarks/*.json`, `benchmarks/snapshots/*.snapshot.json` | refresh, do not replace with new artifact families |

### Minimum complete change set

The smallest honest implementation is:

1. add a shared registry and qualification module
2. extend seam semantic review with producer-owned `descriptor_id`
3. thread one shared `CategoryQualification` object through benchmark, status,
   export, and snapshot surfaces
4. invalidate full-scope supported benchmark claims when a supported-labeled
   case is not producer-qualified
5. refresh tests, fixtures, and snapshots so every read-side surface says the
   same thing

Anything smaller leaves at least one current consumer free to infer support
from partial truth.

### Complexity check

This wedge touches one new module, five existing code modules, and several
fixture suites. That is larger than a one-file patch, but it is still the
right-sized diff because the bug class is cross-consumer by definition. A
benchmark-only fix would leave status/export drift alive.

### Search check

This is internal contract consolidation work, not a framework or infrastructure
selection problem. The right move is reuse, not novelty:

- **[Layer 1]** reuse the existing semantic-review, benchmark, status, export,
  and passport projection surfaces
- **[Layer 3]** add one repo-local contract where no shared contract exists yet

No new infrastructure, registry service, or persistence layer belongs in this
plan.

### TODOS cross-reference

This plan is the executable version of the active backlog item already recorded
in `TODOS.md` under `M101 backlog`. It should land the
implementation wedge, not create a second overlapping TODO.

### Completeness check

This plan should ship the complete version now. With the existing producer and
consumer surfaces already in place, the marginal cost of also updating status,
export, and snapshot projections is small compared with the cost of letting
multiple read-side honesty bugs survive.

### Distribution check

This plan introduces no new distributable artifact. Existing CLI build and
release pipelines remain the distribution path. No release automation work is
needed for this wedge.

## NOT In Scope

This wedge does **not** do any of the following:

- widen semantic-review routing so service seam descriptors become supported
- change the four first-scope category ids
- relabel `BENCH-SERVICE` cases in `benchmarks/labels.json`
- add a checked-in external registry file for non-Rust consumers
- persist `category_qualification` into `.spec.passport.json`
- redesign function-family support, benchmark kinds, or benchmark registry shape
- rewrite status health semantics outside the additive qualification field

## Architecture

### Current vs target authority flow

```text
CURRENT
semantic_review
  -> compatibility_key + support_status
  -> benchmark/status/export each decide what that "means"
  -> benchmark label can still overstate supported closure

TARGET
semantic_review
  -> compatibility_key + support_status + descriptor_id
  -> category_truth registry lookup
  -> qualify_category_claim(...)
  -> benchmark/status/export/snapshot all consume the same result
  -> no consumer-local widening
```

### Architecture diagram

```text
spec-core/src/semantic_review.rs
  ├── projects compatibility_key
  ├── projects support_status
  └── projects descriptor_id
              │
              v
spec-core/src/category_truth.rs
  ├── CategoryTruthRegistry
  └── qualify_category_claim(...)
              │
      ┌───────┼─────────────────────────────┐
      │       │                             │
      v       v                             v
benchmark.rs  commands.rs(status JSON)      export.rs(projected_units)
      │       │                             │
      └──────────────> snapshot/readability surfaces
```

### Shared contract additions

Add `spec-core/src/category_truth.rs` and export it from `spec-core/src/lib.rs`.

This module owns:

- `CategoryTruthRegistry`
- `CategoryTruthRow`
- `CategoryKind`
- `ContractSupportStatus`
- `AliasSiblingPolicy`
- `DescriptorSet`
- `PositiveCreditPolicy`
- `ConsumerKind`
- `CategoryQualification`
- `ClaimStatus`
- `PositiveCreditEligibility`
- `QualificationReasonCode`
- `qualify_category_claim(...)`

The implementation should stay explicit. Do not introduce a second layer of
builder types or indirection unless the code proves it is necessary.

### Exact semantic-review change

`SemanticReview` gains one additive field:

```rust
pub struct SemanticReview {
    ...
    pub descriptor_id: Option<String>,
}
```

Rules:

- only producer-owned semantic-review logic may populate `descriptor_id`
- no benchmark, status, export, or snapshot path may synthesize or rewrite it
- canonical ecommerce seam descriptors project:
  - `discount_strategy.ecommerce.v1`
  - `pricing_quote.ecommerce.v1`
- current service seam siblings project their own service descriptor ids and
  remain unqualified for supported rows
- legacy stored semantic reviews without `descriptor_id` stay readable but must
  fail supported qualification explicitly with `descriptor_id_missing`

### Exact first-scope registry rows

| Category | Kind | Contract support | Alias policy | Canonical descriptor | Approved siblings | Positive credit |
| --- | --- | --- | --- | --- | --- | --- |
| `sum.discount_strategy.v1` | `sum` | `supported` | `canonical_only` | `discount_strategy.ecommerce.v1` | none | eligible |
| `data.pricing_quote.v1` | `data` | `supported` | `canonical_only` | `pricing_quote.ecommerce.v1` | none | eligible |
| `unsupported.sum.v1` | `sum` | `unsupported` | `unsupported_terminal` | none | none | ineligible |
| `unsupported.data.v1` | `data` | `unsupported` | `unsupported_terminal` | none | none | ineligible |

### Qualification API contract

Implement one shared function with this exact responsibility split:

```rust
pub fn qualify_category_claim(
    consumer: ConsumerKind,
    semantic_review: Option<&SemanticReview>,
) -> CategoryQualification
```

The first landing should keep the function contract small. Do **not** add a
separate consumer-context struct unless a real requirement appears during
implementation.

The function decides only category truth:

- registry row lookup from `semantic_review.compatibility_key`
- effective support-status match
- descriptor approval for supported rows only
- unsupported terminal qualification for unsupported rows
- positive-credit eligibility

It does **not** decide benchmark lifecycle, full-vs-partial path scope, or gate
status. Those remain benchmark-local rules.

### Stable qualification output

Every consumer must reuse this shape:

```rust
pub struct CategoryQualification {
    pub category_id: Option<String>,
    pub descriptor_id: Option<String>,
    pub claim_status: ClaimStatus,
    pub positive_credit_eligibility: PositiveCreditEligibility,
    pub reason_code: QualificationReasonCode,
}
```

Required first-landing enums:

- `ClaimStatus`
  - `supported_qualified`
  - `unsupported_qualified`
  - `unqualified`
- `PositiveCreditEligibility`
  - `eligible`
  - `ineligible`
- `QualificationReasonCode`
  - `qualified`
  - `semantic_review_missing`
  - `registry_row_missing`
  - `descriptor_id_missing`
  - `descriptor_not_approved`
  - `support_status_mismatch`
  - `positive_credit_disallowed`

Rules that remove ambiguity:

- supported rows require descriptor approval plus `effective_support_status() ==
  Supported`
- unsupported terminal rows require `effective_support_status() ==
  Unsupported`; they do **not** require descriptor approval
- `positive_credit_disallowed` is valid only for a claim that otherwise
  resolves, but is ineligible for positive credit
- `semantic_support_status` may remain visible as a compatibility/debug field,
  but `category_qualification` is the only authoritative claim surface after
  this wedge

## Consumer Behavior Contract

### Benchmark accounting

`spec-core/src/benchmark.rs` is the first adoption point.

Required behavior after the change:

- `BenchmarkCaseTruth` stops carrying only
  `semantic_support_status: Option<SemanticSupportStatus>`
- replace that with `semantic_review: Option<SemanticReview>` so benchmark
  projection receives the exact producer-owned input it needs instead of a new
  bespoke mini-struct
- `BenchmarkCaseProjection` gains `category_qualification`
- `counts_as_supported_positive` requires all of:
  - positive benchmark
  - active lifecycle
  - full path scope
  - valid benchmark accounting
  - `classification == supported`
  - `status == valid`
  - `category_qualification.claim_status == supported_qualified`
  - `category_qualification.positive_credit_eligibility == eligible`
- a supported-labeled case that fails qualification stays visible, but it makes
  the full benchmark accounting invalid
- partial benchmark scope becomes `partial_invalid` on the same mismatch, but
  still does not invent full `benchmark_status` or `gate_status`
- `readability_review_status` remains whatever the readability artifact already
  says; qualification failure must not rewrite it

This is the most important product decision in the plan:

- `BENCH-SERVICE` full projection must stop saying `passing`
- it must become:
  - `accounting_status = invalid`
  - `benchmark_status = invalid`
  - `gate_status = open`
  - unchanged readability freshness status

### `spec status --format json`

`spec-cli/src/commands.rs` must add `category_qualification` to each
`JsonStatusEntry`.

Rules:

- keep current health semantics unchanged
- keep current `semantic_review` projection unchanged except for additive
  `descriptor_id`
- add `category_qualification` beside `semantic_review`
- never infer supported category truth from health or semantic-review presence
  alone

Schema change:

- bump `STATUS_JSON_SCHEMA_VERSION` from `4` to `5`

### `spec export`

`spec-core/src/export.rs` must expose additive projected truth without writing
`category_qualification` into on-disk passports.

Exact plan:

- keep exported `passports` as projected passport truth
- add a new additive `projected_units` array to `ExportBundle`
- define a dedicated read-side struct for it, for example:

```rust
pub struct ProjectedExportUnit {
    pub id: String,
    pub semantic_review: Option<SemanticReview>,
    pub category_qualification: Option<CategoryQualification>,
}
```

- each row must include:
  - `id`
  - `semantic_review`
  - `category_qualification`

That keeps export machine-readable, keeps qualification read-side only, and
avoids mutating `.spec.passport.json` persistence semantics.

Schema change:

- bump `EXPORT_SCHEMA_VERSION` from `4` to `5`

### Snapshot and readability parity

Benchmark snapshot output already reuses benchmark projection. This wedge must
keep that true.

Rules:

- benchmark snapshot output must serialize per-case `category_qualification`
- full-scope invalid service seam claims must yield invalid full snapshots too
- readability review freshness and verdict stay additive and unchanged by
  qualification failure

### Shared read-side plumbing rule

`spec-cli/src/commands.rs` currently constructs benchmark root-case truth in
multiple places. This wedge must collapse that duplication instead of letting
benchmark and snapshot paths drift again.

Implementation rule:

- add one shared helper in `spec-cli/src/commands.rs` that derives read-side
  unit truth from `project_passport_truth_with_context(...)`
- status, benchmark, and snapshot paths should all consume that shared helper
- do not hand-edit three independent call paths with slightly different
  semantic-review extraction logic

## File Blast Radius

### New code

- `spec-core/src/category_truth.rs`

### Existing code that must change

- `spec-core/src/lib.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/benchmark.rs`
- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-cli/src/commands.rs`

### Tests and fixtures that must change

- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/rust_v1_closure.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/benchmarks/*.json`
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`

## Implementation Phases

### Phase 0: Preflight and current-truth lock

Goals:

- capture the current service mismatch behavior before changing it
- verify which fixtures and tests lock the existing optimistic benchmark status

Do:

- run the current benchmark contract suites
- note all current full-scope service expectations that will intentionally flip
  from `passing` to `invalid`
- capture the exact benchmark/status/export/snapshot fixture files that will
  need rewrites

Done when:

- the planned expectation flips are explicit before code edits start

### Phase 1: Add category truth substrate

Files:

- `spec-core/src/category_truth.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/semantic_review.rs`

Deliverables:

- registry structs and enums
- hard-coded first four rows
- `qualify_category_claim(...)`
- `SemanticReview.descriptor_id`
- seam descriptor-id projection tests
- qualification unit tests for:
  - canonical ecommerce sum qualifies supported
  - canonical ecommerce data qualifies supported
  - service sum sibling does not qualify supported
  - service data sibling does not qualify supported
  - unsupported rows qualify only as unsupported
  - missing semantic review fails explicitly
  - missing descriptor id fails explicitly

### Phase 2: Benchmark-core adoption

Files:

- `spec-core/src/benchmark.rs`
- benchmark-focused tests in `spec-core` and `spec-cli/tests/rust_v1_service.rs`
- benchmark JSON fixtures under `spec-cli/tests/fixtures/benchmarks/`

Deliverables:

- `BenchmarkCaseTruth` carries `semantic_review`
- `BenchmarkCaseProjection` gains `category_qualification`
- `counts_as_supported_positive` uses qualification
- benchmark-wide full-scope invalidation on supported-label qualification
  failure
- service benchmark contract tests assert `invalid/open`
- benchmark fixture expectations reflect:
  - ecommerce remains passing
  - service flips to invalid
  - partial mismatch becomes `partial_invalid`

### Phase 3: Export projection adoption

Files:

- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`

Deliverables:

- `ExportBundle.projected_units[]`
- dedicated projected export row type
- export projection reuses shared qualification
- export schema bump `4 -> 5`
- regression coverage that proves qualification stays read-side only and does
  not persist into `.spec.passport.json`

### Phase 4: CLI status and snapshot integration

Files:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`

Deliverables:

- one shared CLI helper for projected unit truth
- `JsonStatusEntry.category_qualification`
- status schema bump `4 -> 5`
- snapshot command emits the same qualification truth as live benchmark output
- service snapshot flips to invalid full benchmark status
- readability freshness remains unchanged by accounting invalidation

### Phase 5: Final verification sweep

Goals:

- prove all current consumers agree
- prove no consumer-local widening remains

Done when:

- benchmark, status, export, and snapshot surfaces tell the same category truth
- all targeted suites pass
- the repo no longer has a full-scope supported benchmark that is invalid at
  the case level but still claims overall `passing`

## Test Diagram

```text
CODE PATHS
[+] spec-core/src/semantic_review.rs
  ├── canonical ecommerce sum -> supported key + descriptor_id
  ├── canonical ecommerce data -> supported key + descriptor_id
  ├── service sum sibling -> unsupported key + service descriptor_id
  └── service data sibling -> unsupported key + service descriptor_id

[+] spec-core/src/category_truth.rs
  ├── registry lookup -> supported canonical row
  ├── registry lookup -> unsupported terminal row
  ├── supported-row descriptor approval passes
  ├── supported-row descriptor approval fails
  ├── unsupported terminal row skips descriptor approval
  ├── missing semantic review -> unqualified
  └── missing descriptor_id -> unqualified

[+] spec-core/src/benchmark.rs
  ├── supported + qualified -> positive credit
  ├── supported label + unsupported qualification -> no credit
  ├── full benchmark with disqualified supported case -> accounting invalid
  └── partial benchmark with disqualified supported case -> partial_invalid

[+] spec-cli/src/commands.rs / spec-core/src/export.rs
  ├── status emits semantic_review + category_qualification
  ├── export emits projected_units + category_qualification
  └── snapshot emits same benchmark qualification as live projection

CONSUMER FLOWS
[+] BENCH-ECOM full benchmark
  ├── canonical seam cases remain supported_qualified
  └── benchmark stays passing

[+] BENCH-SERVICE full benchmark
  ├── service seam cases stay visible
  ├── supported label remains visible
  ├── category_qualification fails explicitly
  └── benchmark flips to invalid/open

[+] spec status / export / snapshot readers
  └── can distinguish supported_qualified vs unsupported_qualified vs unqualified
```

## Required Test Coverage

Add or update tests for these exact behaviors:

- semantic-review descriptor-id projection for canonical supported seams
- semantic-review descriptor-id projection for current service seam siblings
- qualification lookup for all four first-scope rows
- unsupported terminal rows qualify without descriptor approval
- benchmark case projection includes `category_qualification`
- full `BENCH-SERVICE` flips from `passing` to `invalid`
- full `BENCH-ECOM` remains `passing`
- partial benchmark projections use `partial_invalid` without inventing full
  benchmark fields
- status JSON schema version bumps and includes `category_qualification`
- export schema version bumps and includes
  `projected_units[].category_qualification`
- benchmark snapshots serialize the same case qualification as live output
- legacy persisted semantic review without `descriptor_id` fails qualification
  explicitly
- `.spec.passport.json` outputs do **not** persist `category_qualification`

## Failure Modes Registry

| Codepath | Production failure | Test coverage required | User-visible effect | Priority |
| --- | --- | --- | --- | --- |
| registry lookup | missing row causes implicit support fallback | unit test on unknown key -> `registry_row_missing` | downstream consumer silently lies unless blocked | P1 |
| descriptor identity | legacy or missing `descriptor_id` gets treated as supported | regression test -> `descriptor_id_missing` | benchmark/status/export over-credit seam support | P1 |
| unsupported terminal routing | unsupported rows incorrectly require a descriptor and become generic unqualified noise | unit test on `unsupported.sum.v1` and `unsupported.data.v1` | readers cannot distinguish unsupported-qualified truth from missing truth | P1 |
| benchmark invalidation | disqualified supported case still leaves full benchmark `passing` | service benchmark contract tests | maintainer believes supported closure is green when it is not | P1 |
| status/export drift | status and export emit different reason codes or claim states | paired fixture assertions in `cli.rs` and `m14_regressions.rs` | downstream tools disagree about the same unit | P1 |
| readability coupling | qualification failure rewrites readability freshness | snapshot tests | readers lose freshness signal and cannot separate style from truth | P2 |
| passport persistence leak | category qualification gets written into `.spec.passport.json` | export/passport regression | on-disk proof state becomes polluted with read-side claims | P2 |

Critical gap definition for this wedge:

- any path that can still emit supported positive credit or a supported category
  claim without `CategoryQualification == supported_qualified`

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Category substrate | `spec-core/src/category_truth.rs`, `spec-core/src/lib.rs`, `spec-core/src/semantic_review.rs` | — |
| B. Benchmark core adoption | `spec-core/src/benchmark.rs`, benchmark-focused tests, benchmark fixtures | A |
| C. Export projection adoption | `spec-core/src/export.rs`, `spec-core/src/passport.rs`, export-focused tests | A |
| D. CLI status and snapshot integration | `spec-cli/src/commands.rs`, `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs`, snapshots/reviews | B, C |

### Parallel lanes

- Lane A: Step A
- Lane B: Step B after A lands
- Lane C: Step C after A lands
- Lane D: Step D after B and C land

### Execution order

1. Launch Lane A first. It owns the substrate and must land before anything
   else can qualify claims.
2. After A merges, launch Lanes B and C in parallel worktrees.
3. Merge B and C.
4. Run Lane D last to integrate shared `commands.rs` changes, refresh snapshots,
   and freeze final fixtures.

### Conflict flags

- Lanes B and D both affect benchmark-facing projection behavior, but only D
  should touch the final shared CLI truth helper
- Lanes C and D both affect read-side contract surfaces
- Do **not** run B and D in parallel
- Do **not** run C and D in parallel
- Keep `commands.rs` ownership in Lane D to minimize merge conflict risk

If worktree staffing is unavailable, run the same order sequentially.

## Implementation Tasks

Synthesized from the architecture, test, and failure-mode requirements above.

- [ ] **T1 (P1, human: ~2h / CC: ~15min)** — category substrate — add
  `spec-core/src/category_truth.rs` with the first-scope registry rows,
  qualification enums, and `qualify_category_claim(...)`
  - Files: `spec-core/src/category_truth.rs`, `spec-core/src/lib.rs`
  - Verify: `cargo test -p spec-core category_truth`
- [ ] **T2 (P1, human: ~1.5h / CC: ~10min)** — semantic-review producer truth —
  extend seam semantic-review projection with producer-owned `descriptor_id`
  and lock it with unit tests
  - Files: `spec-core/src/semantic_review.rs`
  - Verify: `cargo test -p spec-core semantic_review`
- [ ] **T3 (P1, human: ~2h / CC: ~15min)** — benchmark qualification —
  thread `semantic_review` into benchmark case truth, add
  `category_qualification`, and make positive credit plus full benchmark
  validity depend on qualification
  - Files: `spec-core/src/benchmark.rs`
  - Verify: `cargo test -p spec-core benchmark`
- [ ] **T4 (P1, human: ~1h / CC: ~10min)** — benchmark contract refresh —
  update `BENCH-SERVICE` tests and fixtures to assert
  `accounting_status = invalid`, `benchmark_status = invalid`,
  `gate_status = open`
  - Files: `spec-cli/tests/rust_v1_service.rs`,
    `spec-cli/tests/rust_v1_closure.rs`,
    `spec-cli/tests/fixtures/benchmarks/*.json`
  - Verify: `cargo test -p spec-cli rust_v1_service rust_v1_closure`
- [ ] **T5 (P1, human: ~1.5h / CC: ~10min)** — export contract —
  add `projected_units[]`, surface shared qualification there, and bump export
  schema version to `5`
  - Files: `spec-core/src/export.rs`, `spec-core/src/passport.rs`
  - Verify: `cargo test -p spec-core export`
- [ ] **T6 (P1, human: ~2h / CC: ~15min)** — CLI read-side integration —
  add one shared projected-truth helper, surface `category_qualification` in
  status JSON, and keep snapshots aligned with live benchmark output
  - Files: `spec-cli/src/commands.rs`, `spec-cli/tests/cli.rs`,
    `spec-cli/tests/m14_regressions.rs`
  - Verify: `cargo test -p spec-cli cli m14_regressions`
- [ ] **T7 (P2, human: ~45min / CC: ~5min)** — snapshot and readability freeze —
  refresh benchmark snapshots and readability fixtures so live and frozen
  projections stay in sync without rewriting readability freshness semantics
  - Files: `benchmarks/snapshots/*.snapshot.json`,
    `benchmarks/reviews/*.readability.review.json`
  - Verify: `cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM` and
    `cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE`

## Acceptance Criteria

This plan is complete only when all of the following are true:

1. the repo has one authoritative category truth registry in `spec-core`
2. every current seam-category consumer calls the same qualification function
3. positive benchmark credit is impossible without
   `supported_qualified + eligible`
4. a supported-labeled but producer-unqualified full benchmark becomes
   `invalid`, not `passing`
5. `spec status` and `spec export` both expose additive
   `category_qualification`
6. export keeps category qualification read-side only and does not persist it
   into `.spec.passport.json`
7. snapshot output matches live benchmark qualification output
8. the current `BENCH-SERVICE` mismatch is explicit contract truth, not repo
   folklore

## Verification Commands

Run at minimum:

```bash
cargo test -p spec-core
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo test -p spec-cli m14_regressions
cargo test -p spec-cli cli

cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Expected end state:

- ecommerce full benchmark remains passing
- service full benchmark becomes invalid/open
- status and export both expose the same category qualification for seam rows
- snapshots match live benchmark projections
- `.spec.passport.json` files remain free of persisted `category_qualification`

## Deferred Follow-On Work

After this wedge lands, separate follow-on work may decide whether to:

- widen producer routing so current service seam descriptors become supported
- tighten `BENCH-SERVICE` labels so they stop asking for supported claims the
  producer does not grant
- externalize the registry for non-Rust consumers
- expand category truth beyond the first four seam rows

Those are real follow-ons. They are explicitly **not** prerequisites for this
plan.
