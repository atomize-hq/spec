# spec — Data Category Taxonomy
**Version:** v0.1  
**Status:** Active working map  
**Date:** 2026-05-24

## Purpose

This document is the per-kind follow-on for `kind:data`.

Use it when the higher-level [kind coverage map](./kind_coverage_map_v0.1.md)
is not enough and you need to answer more specific questions such as:

- what named data categories does the repo actually own today?
- which parts of `kind:data` are structural support versus descriptor support?
- what makes a seam qualify for `data.pricing_quote.v1`?
- what is still unnamed pressure inside the data kind?

## Scope

This document tracks the current **product/support surface** for `kind:data`.

It does **not** try to invent future data categories just to make the map feel
complete. If the repo does not currently own a category strongly enough to name
it, this doc should say `Unnamed` rather than pretend the taxonomy is settled.

## Reading Rules

This taxonomy distinguishes four different things that are easy to blur:

- **Structural legality**
  Is a `kind:data` seam valid authored `spec` source at all?
- **Named descriptor category**
  Does the bounded semantic-review system recognize a specific data shape?
- **Proof and benchmark role**
  Does a data seam merely validate, or does it carry benchmark-positive public
  claim weight?
- **Unsupported observation**
  Can the repo honestly say a seam is outside the owned descriptor subset?

## Structural Baseline For `kind:data`

Every current `kind:data` seam starts from the same structural contract:

- authored data fields live under `data.fields`
- shared construction lives under `constructors`
- callable seam behavior lives under `methods`
- top-level `contract` is not authored
- top-level `deps` is not authored
- top-level `imports` is not authored
- top-level `body.rust` must stay empty
- top-level `body.typescript` is not authored today

That means `kind:data` already has a real authored shape even where it does not
yet have a broad category taxonomy.

## Current Named Categories

As of this doc's date, the repo owns exactly one supported named data category
plus one explicit unsupported observation surface:

- `data.pricing_quote.v1`
- `unsupported.data.v1`

That is the main truth to internalize: `kind:data` is real, but its named
descriptor vocabulary is still intentionally narrow.

## Data Category Matrix

| Category | Semantic route | Structural support | Rust lane | TypeScript lane | Atom proof | Molecule role | Benchmark role | Public role | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `data.pricing_quote.v1` | Supported descriptor | Shipped | Yes | No seam-kind execution lane | Yes | Direct and flow-bearing | Positive in `BENCH-ECOM` and `BENCH-SERVICE` | Direct as plain data seam | Canonical quote seam; descriptor-based and namespace-agnostic |
| `unsupported.data.v1` | Terminal unsupported surface | N/A as a supported category | N/A as a supported category | N/A as a supported category | N/A as a supported category | N/A | No positive credit | Observation only | Honest additive fallback when a data seam misses the owned descriptor subset |

## Owned Descriptor: `data.pricing_quote.v1`

`data.pricing_quote.v1` is the only currently supported named data category.

It is descriptor-based rather than path-based. The repo proves that both:

- `pricing/pricing_quote`
- `billing/pricing_quote`

route to the same compatibility key when they preserve the same authored and
executable seam meaning.

### Descriptor requirements

The current semantic-review detector requires:

- exactly three decimal-authored fields:
  - `subtotal`
  - `discount_rate`
  - `tax_rate`
- exactly one constructor:
  - `new`
- that constructor must accept the same three decimal fields as inputs
- required semantic method roles:
  - `discounted_subtotal`
  - `total`
- those required methods must:
  - use `shared_ref` receivers
  - take no additional inputs
  - return decimal values

The repo does **not** currently treat this as a loose “quote-like data seam.”
It is an exact bounded descriptor.

### Execution and proof shape

The canonical ecommerce and service examples show the intended support shape:

- Rust lowering is first-class
- atom proof is first-class through `local_tests`
- molecule proof is first-class for multi-unit checkout flows
- benchmark-positive proof exists in both required positive walls

### Helper and example methods

The owned descriptor is stricter about required semantic roles than it is about
extra helper/example methods.

That means additive helper or example methods may coexist so long as the seam
still preserves the required quote roles and authored/executable meaning.
What the repo does **not** currently own is a separately named helper-only data
category.

## Unsupported Observation: `unsupported.data.v1`

The repo also owns an honest unsupported observation surface for `kind:data`.

This matters because it lets the system say:

- this is a valid `kind:data` seam structurally
- but it is outside the narrow owned descriptor subset

The canonical near-miss example today is renaming descriptor vocabulary, such
as changing `discount_rate` to `discount_percent`. That does **not** produce a
new supported category. It routes to `unsupported.data.v1`.

So the current taxonomy boundary is:

- valid `kind:data` seam support is broader than one descriptor
- supported semantic-category ownership is not

## Benchmark And Claim Role

`data.pricing_quote.v1` is not just structurally supported. It is part of the
current positive benchmark-backed product claim:

- `pricing/pricing_quote` is a positive case in `BENCH-ECOM`
- `billing/pricing_quote` is a positive case in `BENCH-SERVICE`

That makes the current data category map different from a purely exploratory
taxonomy. The one owned category already carries real public-claim weight.

`unsupported.data.v1` carries no such credit. It is observation-only.

## TypeScript Boundary

`kind:data` currently has **no seam-kind TypeScript execution lane**.

That boundary should be stated narrowly:

- the repo can reason about supported data seams in bounded semantic context
- that does **not** mean data seams execute in the shipped TypeScript lane
- current TypeScript execution support remains function-only

So any future `kind:data` TypeScript work would be a **support-dimension
expansion**, not evidence that more data categories already exist today.

## What Is Still Unnamed For `kind:data`

This is still one of the largest open map holes in the repo.

The current unnamed pressure includes at least:

- quote-like data seams that keep the same role pattern but rename fields or
  constructor vocabulary
- data seams with multiple constructors instead of one canonical `new`
- data seams with more than two owned semantic methods
- data seams that are valid structural seams but have no benchmark or semantic
  category ambition
- any future seam kinds that want TypeScript execution rather than Rust-only
  seam lowering

The important constraint is that none of those are repo-owned named categories
yet. They are pressure, not shipped taxonomy.

## What This Taxonomy Clarifies

This document makes a few important truths visible:

1. `kind:data` is already real and claim-bearing even though its named category
   map is thin.
2. The current data taxonomy is descriptor-based, not namespace-based.
3. The repo owns one benchmark-positive supported descriptor today, not a broad
   family of data seams.
4. Unsupported data review is explicit, which is healthier than silently
   stretching `data.pricing_quote.v1` to fit near-misses.
5. The next work for `kind:data` is likely naming categories or support
   boundaries, not re-proving that seam structure exists.

## How To Update This Taxonomy

When `kind:data` truth changes, update this document after checking:

1. `docs/kind_coverage_map_v0.1.md`
2. `README.md`
3. `docs/core_mechanisms_guide_v0.1.md`
4. `PLAN.md`
5. `DECISIONS.md`
6. `benchmarks/labels.json`
7. current semantic review, validator, CLI, and example-root truth

Update the smallest correct thing:

- add or edit a row in the matrix
- tighten the descriptor requirements
- widen or narrow the TypeScript boundary
- clarify which surfaces are benchmark-positive versus observation-only

Do **not** name a new data category just because several seams “feel similar.”
Wait until the repo has:

- explicit checked-in routing truth
- clear authored and executable role requirements
- a stable compatibility key

## Likely Next Follow-On

After this doc, the next natural follow-on is probably
`docs/sum_category_taxonomy_v0.1.md` so seam kinds stop being asymmetric in the
map.
