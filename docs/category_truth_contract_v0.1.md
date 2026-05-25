# spec — Category Truth Registry and Consumer Qualification Contract
**Version:** v0.1  
**Status:** Implementation-ready design  
**Date:** 2026-05-25

## Purpose

This design closes the cross-consumer honesty gap exposed by commit `47dcd1f`
("Require explicit support for benchmark positive credit").

That fix proved the repo had a real bug class:

> a read-side consumer can over-credit support, category ownership, or positive
> benchmark value from partial truth instead of one explicit contract

This document turns the earlier contract memo into an implementation-ready
design for the next wedge:

- a machine-readable category truth registry
- one shared consumer qualification contract
- first adoption in benchmark accounting, `spec status`, `spec export`, and
  readability/snapshot surfaces

This doc does not reopen `PLAN.md` or `ORCH_PLAN.md`. It narrows one bounded
artifact into an executable implementation shape.

## Decision Summary

The first wedge will ship two new shared primitives inside `spec-core`:

1. `CategoryTruthRegistry`
   A typed, serializable registry that declares repo-owned category truth for
   claim-bearing seams.

2. `qualify_category_claim(...)`
   One shared function that combines a registry row with projected semantic
   review truth and consumer context to decide:
   - whether a consumer may make a supported-category claim
   - whether a consumer may make an unsupported-category claim
   - whether a consumer may award positive benchmark credit
   - why not, when qualification fails

The authoritative source for this first wedge lives in Rust code under
`spec-core`, not in a freestanding JSON file. That keeps the first adoption
small, compile-time checked, and shared by all current Rust consumers.

The registry must still be machine-readable:

- rows use serializable structs and stable enums
- tests and read-side artifacts may render the registry or qualification output
  as JSON
- a file-based external registry can come later if non-Rust consumers need a
  direct checked-in data artifact

## Why This Is The Right Scope

The producer boundary already exists:

- `spec-core/src/semantic_review.rs` owns category routing and effective support
  status

The consumer drift risk also already exists:

- `spec-core/src/benchmark.rs` decides positive credit from projected support
  truth
- `spec-cli/src/commands.rs` projects benchmark/status/export surfaces from
  passport truth
- `spec-core/src/export.rs` preserves read-side semantic review truth for
  downstream consumers
- snapshot and readability surfaces reuse benchmark projections and can inherit
  the same optimism bug if qualification rules drift

So the next honest move is not another one-off fix. It is to centralize the
claim contract at the producer/consumer seam.

## Non-Goals

This wedge does not:

- change semantic-review routing semantics for function families
- promote new categories
- widen seam support beyond the currently shipped four-category first scope
- add a new public file format for external tools
- resolve the broader seam-substrate cleanup by itself

The broader seam-substrate ambiguity stays visible. This wedge makes consumers
honest about it.

## Failure Class

The failure class is any path where a consumer infers support or positive value
without explicit qualification.

Examples:

- awarding positive benchmark credit because a case is labeled `supported` and
  the unit is otherwise valid, even if semantic support is absent or
  unsupported
- treating `support_status`, `compatibility_key`, and sibling handling as
  consumer-local interpretation instead of shared repo truth
- letting `status`, `export`, benchmark projection, and snapshot/readability
  surfaces disagree about whether a category is support-bearing or
  observation-only

## Current Producer And Consumer Surfaces

### Producer

`spec-core/src/semantic_review.rs`

- owns supported seam compatibility keys:
  - `sum.discount_strategy.v1`
  - `data.pricing_quote.v1`
- owns terminal unsupported seam routing:
  - `unsupported.sum.v1`
  - `unsupported.data.v1`
- owns `effective_support_status()`
- already preserves the distinction between supported surfaces and unsupported
  observation surfaces

### First consumers

1. `spec-core/src/benchmark.rs`
   Current risk: positive credit and support-bearing benchmark interpretation.

2. `spec-cli/src/commands.rs` status projection
   Current risk: read-side category interpretation from projected semantic truth
   without one shared qualification vocabulary.

3. `spec-core/src/export.rs` and `spec-cli/src/commands.rs` export projection
   Current risk: downstream readers can inherit raw semantic truth but still
   invent category meaning locally.

4. benchmark readability and snapshot surfaces
   Current risk: they echo benchmark conclusions and can preserve optimism bugs
   if qualification is not shared.

## Design Principles

1. No inference by default.
   Missing contract truth never upgrades to supported or positive-credit truth.

2. Producer owns category routing.
   Consumers do not reinterpret compatibility keys, evaluator scopes, or
   sibling boundaries.

3. Qualification is additive and explicit.
   Consumers receive a qualified answer plus a failure reason, not a suggestion
   to infer.

4. Unsupported categories stay visible.
   Observation-only categories remain readable, but they never count as
   supported or positive-credit truth.

5. Transitional ambiguity must be named.
   The current sum-sibling mismatch is preserved as explicit contract truth,
   not hidden behind vague wording.

## Core Design

### 1. Category Truth Registry

Add a new typed registry module in `spec-core`, for example:

- `spec-core/src/category_truth.rs`

It will expose serializable types roughly shaped like this:

```rust
pub struct CategoryTruthRegistry {
    pub schema_version: u8,
    pub categories: &'static [CategoryTruthRow],
}

pub struct CategoryTruthRow {
    pub category_id: &'static str,
    pub kind: CategoryKind,
    pub contract_support_status: ContractSupportStatus,
    pub producer_surface: ProducerSurface,
    pub alias_sibling_policy: AliasSiblingPolicy,
    pub positive_credit_policy: PositiveCreditPolicy,
    pub consumer_requirements: ConsumerRequirements,
    pub notes: &'static str,
}
```

This is machine-readable repo truth, not just documentation.

### 2. Consumer Qualification Contract

Add one shared qualification function in `spec-core`, for example:

```rust
pub fn qualify_category_claim(
    consumer: ConsumerKind,
    semantic_review: Option<&SemanticReview>,
    consumer_context: ConsumerQualificationContext,
) -> CategoryQualification
```

The function must:

1. resolve the registry row from `semantic_review.compatibility_key`
2. reject qualification when semantic review is missing
3. reject qualification when the registry row is missing
4. require the review's `effective_support_status()` to match the registry row's
   `contract_support_status`
5. apply consumer-specific rules for positive credit or read-side claim display
6. return one stable result object

Suggested result shape:

```rust
pub struct CategoryQualification {
    pub category_id: Option<String>,
    pub claim_status: ClaimStatus,
    pub positive_credit_eligibility: PositiveCreditEligibility,
    pub reason_code: QualificationReasonCode,
}
```

With stable enums:

- `ClaimStatus`
  - `supported_qualified`
  - `unsupported_qualified`
  - `unqualified`
- `PositiveCreditEligibility`
  - `eligible`
  - `ineligible`
- `QualificationReasonCode`
  - `semantic_review_missing`
  - `registry_row_missing`
  - `support_status_mismatch`
  - `consumer_requirement_failed`
  - `positive_credit_disallowed`
  - `qualified`

The key rule is simple:

> if qualification does not return an explicit qualified result, the consumer
> must not invent one

## Registry Schema

The first wedge uses this exact vocabulary.

### `kind`

- `sum`
- `data`

### `contract_support_status`

- `supported`
- `unsupported`

This is static category capability truth from the registry row, not a per-unit
runtime health signal.

### `producer_surface`

```json
{
  "owner": "semantic_review",
  "compatibility_key": "sum.discount_strategy.v1",
  "evaluator_scope": "supported_sum_surface"
}
```

First wedge rule:

- all first-scope rows are owned by `semantic_review`
- consumers must key by `compatibility_key`
- consumers may not infer category support from `evaluator_scope` alone

### `alias_sibling_policy`

This field freezes the repo vocabulary for sibling and alias handling.

Allowed values in this wedge:

- `canonical_only`
  The category means exactly the canonical producer-owned descriptor.

- `approved_sibling_extension`
  The repo intentionally ships more than one sibling descriptor under the same
  category. Consumers may honor that category only because the registry row
  says so. They may not widen it further.

- `unsupported_terminal`
  The category is an observation-only unsupported sink. It is never
  support-bearing or positive-credit eligible.

The sum seam starts as `approved_sibling_extension`.
The data seam starts as `canonical_only`.
Both unsupported seam rows are `unsupported_terminal`.

### `positive_credit_policy`

Suggested shape:

```json
{
  "eligible": true,
  "requires_supported_case_classification": true,
  "requires_full_scope_projection": true,
  "requires_valid_benchmark_accounting": true
}
```

For unsupported categories:

```json
{
  "eligible": false
}
```

### `consumer_requirements`

First wedge requirements are explicit and shared:

```json
{
  "require_semantic_review": true,
  "require_matching_compatibility_key": true,
  "allow_health_only_inference": false,
  "allow_label_only_inference": false
}
```

This is the contract that closes the recent bug class.

## First Registry Rows

The first adoption scope is intentionally small.

| Category | Kind | Contract support | Alias/sibling policy | Positive credit |
| --- | --- | --- | --- | --- |
| `sum.discount_strategy.v1` | `sum` | `supported` | `approved_sibling_extension` | eligible |
| `data.pricing_quote.v1` | `data` | `supported` | `canonical_only` | eligible |
| `unsupported.sum.v1` | `sum` | `unsupported` | `unsupported_terminal` | ineligible |
| `unsupported.data.v1` | `data` | `unsupported` | `unsupported_terminal` | ineligible |

## First-Scope Policy Notes

### `sum.discount_strategy.v1`

This row is the one explicit transitional nuance.

Current repo truth says:

- the canonical semantic-review wording is still narrow around
  `none` / `percentage` / `fixed_amount`
- broader checked-in repo surfaces already treat the service sibling
  (`declined` / `percentage` / `fixed_credit`) as part of the same category

So the first registry row must record:

- `contract_support_status = supported`
- `alias_sibling_policy = approved_sibling_extension`
- `notes = transitional canonical/service sibling mismatch; do not widen beyond
  the shipped sibling set`

This is not permission for consumers to invent new siblings. It is permission
for consumers to respect the one shipped sibling extension the repo already
claims.

### `data.pricing_quote.v1`

This row is simpler:

- exact bounded descriptor
- no approved sibling extension in the first wedge
- positive-credit eligible when fully qualified

### `unsupported.sum.v1` and `unsupported.data.v1`

These rows exist so unsupported seam truth is explicit and shared.

They must stay:

- visible
- additive
- qualification-bearing as unsupported only
- never positive-credit eligible

## Consumer Behavior Contract

### Benchmark accounting

Target surface:

- `spec-core/src/benchmark.rs`

Current positive-credit logic already checks:

- benchmark kind
- lifecycle
- path scope
- accounting validity
- case classification
- projected semantic support status

The new rule is stricter:

- `counts_as_supported_positive` must require
  `CategoryQualification { claim_status: supported_qualified, positive_credit_eligibility: eligible, ... }`

If qualification fails:

- positive credit becomes `false`
- the case stays visible
- the failure reason is preserved in qualification output
- benchmark accounting status should degrade to non-clean when a case labeled
  `supported` cannot qualify

This keeps honesty stronger than zero-credit-only silence.

### `spec status`

Target surface:

- `spec-cli/src/commands.rs`

`spec status` must not infer category-bearing support from base health or raw
semantic-review presence.

First wedge behavior:

- preserve existing health logic
- preserve projected semantic review
- add additive qualification output for read-side consumers
- if qualification fails, show the unit as semantically present but category
  claim unqualified

This is a read-side honesty improvement, not a health-model rewrite.

### `spec export`

Target surfaces:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`

`spec export` must preserve projected semantic review and add the same additive
qualification output used by `status`.

Reason:

- downstream consumers currently receive raw semantic truth and can still drift
- export should be the main machine-readable place where the repo says
  "this category claim is qualified" versus "this row exists but is not
  support-bearing"

### Readability and snapshot surfaces

Target surfaces:

- benchmark snapshot projection in `spec-cli/src/commands.rs`
- snapshot/readability fields derived from `spec-core/src/benchmark.rs`

These surfaces must consume the same qualification result as live benchmark
projection.

No separate snapshot-local support logic is allowed.

## Migration Plan

### Phase 1 — Add shared contract substrate

Files:

- `spec-core/src/category_truth.rs` (new)
- `spec-core/src/lib.rs` or module exports

Deliverables:

- registry structs and enums
- first four rows
- `qualify_category_claim(...)`
- stable qualification reason codes
- unit tests for registry lookup and qualification behavior

### Phase 2 — Migrate benchmark accounting first

Files:

- `spec-core/src/benchmark.rs`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/rust_v1_closure.rs`
- snapshot fixtures under `benchmarks/snapshots/` and
  `spec-cli/tests/fixtures/benchmarks/`

Deliverables:

- benchmark positive credit gated through qualification
- category qualification echoed in case projection output
- explicit regression coverage for the dishonesty bug class

### Phase 3 — Migrate `spec status` and `spec export`

Files:

- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`

Deliverables:

- additive qualification output on status/export surfaces
- status and export consume the same registry and reason codes
- tests prove supported seam rows qualify and unsupported seam rows remain
  observation-only

### Phase 4 — Freeze snapshot/readability parity

Files:

- `spec-cli/src/commands.rs`
- `spec-core/src/benchmark.rs`
- benchmark fixtures and snapshots

Deliverables:

- live and snapshot benchmark projections share the same qualification path
- readability surfaces cannot silently reintroduce optimistic category claims

## Expected File Blast Radius

The first implementation should stay near this boundary:

- `spec-core/src/semantic_review.rs`
  Reuse existing compatibility keys; do not reopen routing semantics unless the
  contract demands a minimal extraction point.

- `spec-core/src/benchmark.rs`
  Main positive-credit adoption point.

- `spec-core/src/export.rs`
  Main machine-readable read-side adoption point.

- `spec-cli/src/commands.rs`
  Main `status`, `export`, and snapshot consumption point.

- tests and JSON fixtures for benchmark/status/export parity

This wedge should not sprawl into unrelated family-analysis or corpus files.

## Proof Plan

The implementation is done only when all of these are true.

### Registry proof

- lookup succeeds for all four first-scope category ids
- lookup fails cleanly for unknown category ids
- unsupported seam rows never report positive-credit eligibility

### Benchmark proof

- a supported benchmark case with missing semantic review gets zero positive
  credit and a qualification failure reason
- a supported benchmark case with unsupported semantic truth gets zero positive
  credit
- a fully qualified supported seam case still earns positive credit
- snapshot output matches live projection output

### Status/export proof

- supported seam rows show qualified support
- unsupported seam rows show qualified unsupported observation
- missing registry or missing semantic review yields `unqualified`, not
  implicit support

### Sibling-policy proof

- `sum.discount_strategy.v1` is the only first-scope row allowed to use
  `approved_sibling_extension`
- no consumer may widen beyond the registry row's explicit policy

## Acceptance Criteria

This wedge is complete when:

1. the repo has one authoritative category truth registry in `spec-core`
2. benchmark, status, export, and snapshot/readability all call the same
   qualification function
3. positive benchmark credit is impossible without explicit qualified support
4. the four first-scope categories have stable registry rows and tests
5. the sum sibling mismatch is explicit contract truth instead of hidden lore
6. no first-scope consumer infers support from health, labels, or
   compatibility-key folklore alone

## Deferred Follow-On Work

After this wedge lands, the next follow-on can decide whether to:

- normalize the canonical `sum.discount_strategy.v1` detector wording to match
  the shipped sibling truth
- externalize the registry into a checked-in JSON artifact for non-Rust
  consumers
- expand the registry beyond the first four seam categories

Those are real follow-ons, but they are not prerequisites for this wedge.

## Recommended Implementation Order

1. add the registry and qualification substrate
2. land benchmark-accounting adoption and regressions
3. land `status` / `export` additive qualification output
4. lock snapshot/readability parity

That order keeps the original bug class closed first, then prevents the same
dishonesty from surviving elsewhere.
