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

This wedge does **not** let benchmark labels, stored passports, or other
consumer-local metadata widen producer-owned support truth. A case may still be
labeled `classification: supported` at the benchmark layer and yet remain
category-unqualified when producer truth does not support that claim.

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
- rewrite `BENCH-SERVICE` labels to align with producer truth in the same wedge
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
   The current benchmark-label-versus-producer mismatch is preserved as
   explicit contract truth, not hidden behind vague wording.

6. Producer truth outranks benchmark labels.
   Benchmark `classification` is consumer input, not category authority. It may
   request a supported claim, but only qualification may grant one.

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
    pub descriptor_set: DescriptorSet,
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
4. resolve the producer-owned `descriptor_id` from the same semantic projection
   truth that produced the compatibility key
5. require that `descriptor_id` to match either:
   - `descriptor_set.canonical_descriptor.descriptor_id`
   - or one member of `descriptor_set.approved_siblings[]`
6. require the review's `effective_support_status()` to match the registry row's
   `contract_support_status`
7. apply consumer-specific rules for positive credit or read-side claim display
8. return one stable result object

Suggested result shape:

```rust
pub struct CategoryQualification {
    pub category_id: Option<String>,
    pub descriptor_id: Option<String>,
    pub claim_status: ClaimStatus,
    pub positive_credit_eligibility: PositiveCreditEligibility,
    pub reason_code: QualificationReasonCode,
}
```

This object should be the shared serializable projection shape reused across
benchmark cases, `spec status`, and `spec export`. The wedge should not invent
slightly different per-surface field bundles for the same concept.

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
  - `descriptor_id_missing`
  - `descriptor_not_approved`
  - `support_status_mismatch`
  - `consumer_requirement_failed`
  - `positive_credit_disallowed`
  - `qualified`

The key rule is simple:

> if qualification does not return an explicit qualified result, the consumer
> must not invent one

### Producer-owned descriptor identity

`descriptor_set` is an active contract boundary, not metadata.

So the first wedge must carry one extra producer-owned identity at
qualification time:

- `descriptor_id`

This is a stable per-unit descriptor identity derived from the same producer
truth that already yields the compatibility key. Consumers do not invent it.

First-wedge source of truth:

- seam producer logic in `spec-core/src/semantic_review.rs` (or a minimal
  helper extracted from it) must emit both:
  - `compatibility_key`
  - `descriptor_id`

Bounded implementation rule:

- do **not** redesign the whole semantic review schema
- do add a producer-owned way for the qualification path to obtain
  `descriptor_id` from the semantic projection result or a tightly-coupled
  helper in `spec-core`

Acceptable first-wedge implementation shapes:

1. extend projected semantic truth to carry `descriptor_id`
2. derive `descriptor_id` in `qualify_category_claim(...)` by calling one
   producer-owned helper on the same unit/spec context that produced semantic
   review

The required invariant is the same either way:

> `descriptor_id` must come from producer-owned truth in `spec-core`, not from
> benchmark labels, export readers, status formatting, or other consumer-local
> interpretation

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

In the first implementation-ready wedge:

- supported rows start as `canonical_only`
- unsupported seam rows are `unsupported_terminal`
- `approved_sibling_extension` remains reserved for a later producer-backed
  widening, not for this first landing

`alias_sibling_policy` is only the coarse classification. It is not enough on
its own for machine use. Every supported row that is not `canonical_only` must
also carry an explicit `descriptor_set` so consumers and tests can read the
allowed sibling boundary without relying on prose notes.

### `descriptor_set`

This field is the machine-readable sibling boundary.

Suggested shape:

```json
{
  "canonical_descriptor": {
    "descriptor_id": "pricing_quote.ecommerce.v1",
    "representative_unit_id": "pricing/pricing_quote"
  },
  "approved_siblings": []
}
```

First wedge rules:

- `canonical_only` rows must still declare `canonical_descriptor`
- `approved_sibling_extension` rows must declare both
  `canonical_descriptor` and every approved sibling entry
- `unsupported_terminal` rows use an empty descriptor set
- qualification must fail unless the producer-owned `descriptor_id` matches the
  canonical descriptor or one approved sibling entry
- no consumer may widen beyond `descriptor_set.approved_siblings[]`
- prose notes may explain why a sibling exists, but the sibling boundary itself
  must be derivable from the registry alone

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

| Category | Kind | Contract support | Alias/sibling policy | Approved sibling ids | Positive credit |
| --- | --- | --- | --- | --- | --- |
| `sum.discount_strategy.v1` | `sum` | `supported` | `canonical_only` | none | eligible |
| `data.pricing_quote.v1` | `data` | `supported` | `canonical_only` | none | eligible |
| `unsupported.sum.v1` | `sum` | `unsupported` | `unsupported_terminal` | none | ineligible |
| `unsupported.data.v1` | `data` | `unsupported` | `unsupported_terminal` | none | ineligible |

## First-Scope Policy Notes

### `sum.discount_strategy.v1`

Current repo truth says:

- the canonical semantic-review wording is still narrow around
  `none` / `percentage` / `fixed_amount`
- the service seam sibling (`declined` / `percentage` / `fixed_credit`) is
  still routed by producer truth to `unsupported.sum.v1`
- `BENCH-SERVICE` still labels `billing/discount_strategy` as
  `classification: supported`, but current benchmark fixtures already deny it
  positive credit because semantic-review truth is unsupported

So the first registry row must record:

- `contract_support_status = supported`
- `alias_sibling_policy = canonical_only`
- `descriptor_set.canonical_descriptor.descriptor_id =
  discount_strategy.ecommerce.v1`
- `descriptor_set.approved_siblings = []`
- `notes = canonical ecommerce descriptor only; service sibling remains visible
  but unqualified until producer truth is widened explicitly`

This is intentional. The first wedge is about honest read-side qualification,
not about retroactively upgrading an unsupported producer surface into
supported category truth.

### `data.pricing_quote.v1`

Current repo truth says:

- the canonical semantic-review detector is still narrow around
  `subtotal` / `discount_rate` / `tax_rate`
- the service seam uses
  `subtotal` / `membership_rate` / `regional_rate`
- producer truth still routes that service seam to `unsupported.data.v1`
- `BENCH-SERVICE` still labels `billing/pricing_quote` as
  `classification: supported`, but current benchmark fixtures already deny it
  positive credit because semantic-review truth is unsupported

So the first registry row must record:

- `contract_support_status = supported`
- `alias_sibling_policy = canonical_only`
- `descriptor_set.canonical_descriptor.descriptor_id =
  pricing_quote.ecommerce.v1`
- `descriptor_set.approved_siblings = []`
- `notes = canonical ecommerce descriptor only; service sibling remains visible
  but unqualified until producer truth is widened explicitly`

This keeps the authority chain coherent:

- semantic review stays the producer-owned router
- the registry qualifies only the categories producer truth actually projected
- benchmark labels stay visible but cannot overrule unsupported producer truth

### `unsupported.sum.v1` and `unsupported.data.v1`

These rows exist so unsupported seam truth is explicit and shared.

They must stay:

- visible
- additive
- qualification-bearing as unsupported only
- never positive-credit eligible

### Current service benchmark mismatch

The repo currently has a real split that this wedge must make explicit instead
of papering over:

- `benchmarks/labels.json` marks `billing/discount_strategy` and
  `billing/pricing_quote` as `classification: supported`
- producer-owned semantic review still projects those seam units as
  `unsupported.sum.v1` and `unsupported.data.v1`
- frozen `BENCH-SERVICE` status/export fixtures therefore already show
  `counts_as_supported_positive = false` for both carriers

The first wedge must preserve that checked-in truth honestly:

- the benchmark label remains visible
- the unit remains visible
- the category claim becomes explicitly `unqualified` or
  `unsupported_qualified`, depending on surface
- no new registry row rescues the claim into supported truth

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

- `classification: supported` is necessary input for positive benchmark credit,
  but it is never sufficient authority on its own
- `counts_as_supported_positive` must require
  `CategoryQualification { claim_status: supported_qualified, positive_credit_eligibility: eligible, ... }`
- that qualified result must already have passed descriptor approval against the
  registry row's `descriptor_set`

Additive projection should carry the shared qualification object per case, for
example as:

```json
{
  "category_qualification": {
    "category_id": "sum.discount_strategy.v1",
    "descriptor_id": "discount_strategy.ecommerce.v1",
    "claim_status": "supported_qualified",
    "positive_credit_eligibility": "eligible",
    "reason_code": "qualified"
  }
}
```

For the current service seam mismatch, the same projection stays visible but
reports an unqualified or unsupported result instead of silently relying on the
benchmark label.

If qualification fails:

- positive credit becomes `false`
- the case stays visible
- the failure reason is preserved in qualification output
- if the benchmark projection is `full`, `accounting_status` becomes `invalid`
- if the benchmark projection is `partial`, `accounting_status` becomes
  `partial_invalid`
- `benchmark_status` follows the existing checked-in benchmark semantics:
  because `determine_benchmark_status(...)` treats `accounting_status = invalid`
  as terminal, the full projection becomes `benchmark_status = invalid`
- `gate_status` remains `open`
- `readability_review_status` does **not** get rewritten by qualification
  failure; it stays whatever the readability artifact truth already says
  (`current`, `stale`, `missing`, or `not_applicable`)
- partial projections keep the current checked-in shape: they surface the
  `partial_invalid` accounting result, but do not invent full-scope
  `benchmark_status`, `gate_status`, or `readability_review_status` fields

This keeps honesty stronger than zero-credit-only silence while staying aligned
to the current benchmark status model. The wedge should not overload
`readability_review_status` into a second benchmark-accounting signal.

### `spec status`

Target surface:

- `spec-cli/src/commands.rs`

`spec status` must not infer category-bearing support from base health or raw
semantic-review presence.

First wedge behavior:

- preserve existing health logic
- preserve projected semantic review
- add the shared additive `category_qualification` output for read-side
  consumers
- if qualification fails, show the unit as semantically present but category
  claim unqualified

This is a read-side honesty improvement, not a health-model rewrite.

Because `spec status --format json` is a published machine surface, the wedge
should bump `STATUS_JSON_SCHEMA_VERSION` when this object is added.

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

The first wedge should keep qualification as read-side projected truth only:

- do **not** persist category qualification into passports
- do **not** add benchmark-local or export-local category vocabularies
- do bump `EXPORT_SCHEMA_VERSION` when the shared qualification object is added

### Readability and snapshot surfaces

Target surfaces:

- benchmark snapshot projection in `spec-cli/src/commands.rs`
- snapshot/readability fields derived from `spec-core/src/benchmark.rs`

These surfaces must consume the same qualification result as live benchmark
projection.

No separate snapshot-local support logic is allowed.

First wedge projection rule:

- snapshot and export/readability surfaces must preserve the same per-case
  qualification output as live benchmark projection
- a supported-labeled case that fails qualification must appear with
  `counts_as_supported_positive = false`
- the enclosing full benchmark projection must show
  `accounting_status = invalid`, `benchmark_status = invalid`, and
  `gate_status = open`
- `readability_review_status` remains a freshness/applicability signal only and
  must not be rewritten to encode qualification failure

## Migration Plan

### Phase 1 — Add shared contract substrate

Files:

- `spec-core/src/category_truth.rs` (new)
- `spec-core/src/lib.rs` or module exports
- minimal producer-owned descriptor-id hook in `spec-core` so qualification can
  enforce `descriptor_set`

Deliverables:

- registry structs and enums
- first four rows
- `qualify_category_claim(...)`
- producer-owned `descriptor_id` resolution for first-scope seam categories
- stable qualification reason codes
- unit tests for registry lookup and qualification behavior
- explicit tests that current service seam descriptors do **not** qualify as
  supported in the first wedge

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
- explicit regression coverage that `BENCH-SERVICE` seam cases remain visible,
  labeled `supported`, and still do not earn positive credit while producer
  truth stays unsupported

### Phase 3 — Migrate `spec status` and `spec export`

Files:

- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`

Deliverables:

- additive shared `category_qualification` output on status/export surfaces
- status and export consume the same registry and reason codes
- tests prove supported seam rows qualify and unsupported seam rows remain
  observation-only
- tests prove status/export surface the service benchmark mismatch explicitly
  rather than silently collapsing it

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

It also should not mutate passport on-disk schema in the first landing. The new
truth is read-side projected, not persisted proof state.

## Proof Plan

The implementation is done only when all of these are true.

### Registry proof

- lookup succeeds for all four first-scope category ids
- lookup fails cleanly for unknown category ids
- unsupported seam rows never report positive-credit eligibility
- producer-owned descriptor resolution emits the canonical descriptor id for the
  ecommerce seam exemplars
- service seam exemplars do not qualify for supported rows while producer truth
  still routes them through unsupported surfaces

### Benchmark proof

- a supported benchmark case with missing semantic review gets zero positive
  credit and a qualification failure reason
- a supported benchmark case with unsupported semantic truth gets zero positive
  credit
- a supported benchmark case with missing producer-owned `descriptor_id` gets
  zero positive credit and `descriptor_id_missing`
- a supported benchmark case with the same compatibility key but an unapproved
  descriptor id gets zero positive credit and `descriptor_not_approved`
- a supported benchmark case with category qualification failure forces:
  - full projection `accounting_status = invalid`
  - full projection `benchmark_status = invalid`
  - full projection `gate_status = open`
  - unchanged `readability_review_status`
- a fully qualified supported seam case still earns positive credit
- snapshot output matches live projection output

### Status/export proof

- supported seam rows show qualified support only when their producer-owned
  descriptor id is approved by the row's `descriptor_set`
- unsupported seam rows show qualified unsupported observation
- canonical descriptor qualifies
- current service seam siblings do not qualify for supported rows
- same compatibility key with unapproved descriptor does not qualify
- missing registry or missing semantic review yields `unqualified`, not
  implicit support

### Sibling-policy proof

- supported rows with `approved_sibling_extension` must declare a non-empty
  `descriptor_set.approved_siblings[]`
- no first-scope row uses `approved_sibling_extension` until producer truth
  explicitly widens
- no consumer may widen beyond the registry row's explicit policy

## Acceptance Criteria

This wedge is complete when:

1. the repo has one authoritative category truth registry in `spec-core`
2. benchmark, status, export, and snapshot/readability all call the same
   qualification function
3. positive benchmark credit is impossible without explicit qualified support
4. the four first-scope categories have stable registry rows and tests
5. the current `BENCH-SERVICE` label-vs-producer mismatch is explicit contract
   truth instead of hidden lore
6. `descriptor_set` is actively enforced through producer-owned descriptor
   identity rather than treated as documentation-only metadata
7. no first-scope consumer infers support from health, labels, or
   compatibility-key folklore alone

## Deferred Follow-On Work

After this wedge lands, the next follow-on can decide whether to:

- widen producer-owned semantic routing so specific service seam descriptors
  become truly supported
- tighten `BENCH-SERVICE` labels so they stop asking for supported claims that
  producer truth does not currently grant
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
