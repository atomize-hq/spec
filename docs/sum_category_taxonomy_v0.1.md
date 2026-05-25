# spec — Sum Category Taxonomy
**Version:** v0.1  
**Status:** Active working map  
**Date:** 2026-05-24

## Purpose

This document is the per-kind follow-on for `kind:sum`.

Use it when the higher-level [kind coverage map](./kind_coverage_map_v0.1.md)
is not enough and you need to answer more specific questions such as:

- what named sum categories does the repo actually own today?
- which parts of `kind:sum` are structural support versus descriptor support?
- what makes a seam qualify for `sum.discount_strategy.v1`?
- what is still unnamed pressure inside the sum kind?

## Scope

This document tracks the current **product/support surface** for `kind:sum`.

It does **not** try to invent future sum categories just to make the map feel
complete. If the repo does not currently own a category strongly enough to name
it, this doc should say `Unnamed` rather than pretend the taxonomy is settled.

## Reading Rules

This taxonomy distinguishes four different things that are easy to blur:

- **Structural legality**
  Is a `kind:sum` seam valid authored `spec` source at all?
- **Named descriptor category**
  Does the bounded semantic-review system recognize a specific sum shape?
- **Proof and benchmark role**
  Does a sum seam merely validate, or does it carry benchmark-positive public
  claim weight?
- **Unsupported observation**
  Can the repo honestly say a seam is outside the owned descriptor subset?

## Structural Baseline For `kind:sum`

Every current `kind:sum` seam starts from the same structural contract:

- authored variants live under `sum.variants`
- callable seam behavior lives under `methods`
- top-level `contract` is not authored
- top-level `deps` is not authored
- top-level `imports` is not authored
- top-level `body.rust` must stay empty
- top-level `body.typescript` is not authored today

That means `kind:sum` already has a real authored shape even where it does not
yet have a broad category taxonomy.

## Current Named Categories

As of this doc's date, the repo owns exactly one supported named sum category
plus one explicit unsupported observation surface:

- `sum.discount_strategy.v1`
- `unsupported.sum.v1`

That is the main truth to internalize: `kind:sum` is real, but its named
descriptor vocabulary is still intentionally narrow.

## Sum Category Matrix

| Category | Semantic route | Structural support | Rust lane | TypeScript lane | Atom proof | Molecule role | Benchmark role | Public role | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `sum.discount_strategy.v1` | Supported descriptor | Shipped | Yes | No seam-kind execution lane | Yes | Direct and flow-bearing | Positive in `BENCH-ECOM` and `BENCH-SERVICE` | Direct as plain sum seam | Canonical discount-policy seam; current repo truth includes both ecommerce and service siblings under the same category |
| `unsupported.sum.v1` | Terminal unsupported surface | N/A as a supported category | N/A as a supported category | N/A as a supported category | N/A as a supported category | N/A | No positive credit | Observation only | Honest additive fallback when a sum seam misses the owned descriptor subset |

## Owned Descriptor: `sum.discount_strategy.v1`

`sum.discount_strategy.v1` is the only currently supported named sum category.

It is descriptor-based rather than path-based. The repo treats both:

- `pricing/discount_strategy`
- `billing/discount_strategy`

as part of the same owned product surface.

### Core required roles

Across current repo truth, the owned category requires two semantic method
roles:

- `discount_amount`
- `discounted_subtotal`

Those required methods must:

- use `shared_ref` receivers
- take exactly one decimal input named `subtotal`
- return decimal values

That is the most stable part of the sum descriptor today.

### Canonical authored shape in semantic-review code

The current semantic-review detector and canonical tests are written around this
canonical variant vocabulary:

- `none`
- `percentage`
  - decimal field `rate`
- `fixed_amount`
  - decimal field `amount`

And the canonical aligned executable body expects:

- the `none` arm to produce decimal zero
- the `percentage` arm to compute `subtotal * rate`
- the `fixed_amount` arm to cap the discount at the subtotal

That is the narrowest exact detector-owned descriptor in checked-in
semantic-review code.

### Important current-truth nuance

Current benchmark and export surfaces also carry the service sibling
`billing/discount_strategy` under the same compatibility key even though its
authored vocabulary is:

- `declined`
- `percentage`
- `fixed_credit`

So the honest repo-level statement today is:

- the canonical semantic-review implementation is still phrased around the
  `none` / `fixed_amount` vocabulary
- the broader checked-in product and benchmark surfaces already treat the
  service sibling as part of the same owned `sum.discount_strategy.v1` category

This is worth documenting explicitly so future cleanup can decide whether to:

- normalize the detector wording to match both siblings
- or tighten the public surfaces back to the canonical descriptor

## Execution and Proof Shape

The canonical ecommerce and service examples show the intended support shape:

- Rust lowering is first-class
- atom proof is first-class through `local_tests`
- molecule proof is first-class for multi-unit checkout flows
- benchmark-positive proof exists in both required positive walls

For `kind:sum`, the molecule layer matters especially because the canonical
seam story is not just “can the enum lower?” but “does the lowered policy stay
aligned with the broader pricing or service flow?”

## Helper and Example Methods

The owned descriptor is stricter about required semantic roles than it is about
extra helper/example methods.

That means additive helper or example methods may coexist so long as the seam
still preserves the required discount-strategy roles and authored/executable
meaning. What the repo does **not** currently own is a separately named
helper-only sum category.

## Unsupported Observation: `unsupported.sum.v1`

The repo also owns an honest unsupported observation surface for `kind:sum`.

This matters because it lets the system say:

- this is a valid `kind:sum` seam structurally
- but it is outside the narrow owned descriptor subset

The canonical near-miss example today is renaming descriptor vocabulary, such
as changing `percentage` to `percent`. That does **not** produce a new
supported category. It routes to `unsupported.sum.v1`.

So the current taxonomy boundary is:

- valid `kind:sum` seam support is broader than one descriptor
- supported semantic-category ownership is not

## Benchmark And Claim Role

`sum.discount_strategy.v1` is not just structurally supported. It is part of
the current positive benchmark-backed product claim:

- `pricing/discount_strategy` is a positive case in `BENCH-ECOM`
- `billing/discount_strategy` is a positive case in `BENCH-SERVICE`

That makes the current sum category map different from a purely exploratory
taxonomy. The one owned category already carries real public-claim weight.

`unsupported.sum.v1` carries no such credit. It is observation-only.

## TypeScript Boundary

`kind:sum` currently has **no seam-kind TypeScript execution lane**.

That boundary should be stated narrowly:

- the repo can reason about supported sum seams in bounded semantic context
- that does **not** mean sum seams execute in the shipped TypeScript lane
- current TypeScript execution support remains function-only

So any future `kind:sum` TypeScript work would be a **support-dimension
expansion**, not evidence that more sum categories already exist today.

## What Is Still Unnamed For `kind:sum`

This is still one of the largest open map holes in the repo.

The current unnamed pressure includes at least:

- policy-like sum seams that keep the same role pattern but rename variant or
  field vocabulary
- sum seams with more than three variants
- sum seams with the same structural shape but different public-role ambition
- sum seams that are valid structural seams but have no benchmark or semantic
  category ambition
- any future seam kinds that want TypeScript execution rather than Rust-only
  seam lowering

The important constraint is that none of those are repo-owned named categories
yet. They are pressure, not shipped taxonomy.

## What This Taxonomy Clarifies

This document makes a few important truths visible:

1. `kind:sum` is already real and claim-bearing even though its named category
   map is thin.
2. The current sum taxonomy is descriptor-based, not namespace-based.
3. The repo owns one benchmark-positive supported descriptor today, not a broad
   family of sum seams.
4. Unsupported sum review is explicit, which is healthier than silently
   stretching `sum.discount_strategy.v1` to fit near-misses.
5. There is a current repo-truth split between the canonical semantic-review
   wording and the broader benchmark/export product surface for the service
   sibling.

## How To Update This Taxonomy

When `kind:sum` truth changes, update this document after checking:

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
- resolve or document any split between canonical detector wording and broader
  product surfaces

Do **not** name a new sum category just because several seams “feel similar.”
Wait until the repo has:

- explicit checked-in routing truth
- clear authored and executable role requirements
- a stable compatibility key

## Likely Next Follow-On

After this doc, the next natural follow-on is probably a machine-readable
kind/category registry so the repo can compute coverage status instead of
reconstructing it from prose docs.
