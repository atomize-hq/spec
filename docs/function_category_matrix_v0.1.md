# spec — Function Category Matrix
**Version:** v0.1  
**Status:** Active working map  
**Date:** 2026-05-24

## Purpose

This document is the per-category follow-on for `kind:function`.

Use it when the higher-level [kind coverage map](./kind_coverage_map_v0.1.md)
is not enough and you need to answer more specific questions such as:

- which function categories are actually shipped?
- which ones run in Rust?
- which ones run in the bounded TypeScript lane?
- which ones carry benchmark-positive public claim weight?
- which ones are helper-only or companion-negative only?

## Scope

This document tracks the **product/support surface** for `kind:function`.

It does **not** try to document every maintainer command separately.
That distinction matters because some maintainer-facing paths are narrower than
the shipped product surface. In particular:

- the bounded `spec test --target-language typescript` lane covers more
  supported function categories than the narrower maintainer-facing
  `cargo xtask family prove --target-language typescript` path
- this matrix tracks the repo's shipped support surface first
- command-specific maintenance constraints are called out in notes when they
  matter

## Reading Rules

This matrix distinguishes four different ideas that are easy to blur:

- **Promoted packet**
  Is there a real promoted semantic-family packet?
- **Runtime route**
  Does current runtime/semantic routing recognize the category as supported?
- **TypeScript lane**
  Can the bounded product TypeScript lane execute roots or local graphs for it?
- **Benchmark role**
  Does current benchmark evidence make it part of the positive Rust V1 proof
  wall, helper-only closure support, companion-negative visibility, or no claim
  at all?

## Inventory Snapshot

As of this doc's date, `cargo xtask family inventory --format json` reports:

- promoted families:
  - `function.wrapper.pipeline.chain3.v1`
  - `function.wrapper.pipeline.normalized_required_arg.v1`
  - `function.wrapper.pipeline.v1`
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.helper.identity_passthrough.v1`
- runtime-supported routes:
  - the same six categories above
- supported-but-unpromoted families:
  - none

That means the current `kind:function` map has no runtime-supported shadow
category waiting outside promotion. The main gaps are deferred or still unnamed,
not "supported but unpromoted."

## Matrix Legend

- **Yes**: shipped and repo-owned today
- **Bounded**: shipped, but intentionally narrow
- **Indirect**: not the main claim carrier, but participates in proof or
  closure around a claim-bearing category
- **Helper-only**: real support, but not the main category being claimed
- **Companion-negative**: visible in negative proof only, zero positive credit
- **Deferred**: explicitly named future boundary
- **No**: not part of the current shipped truth
- **N/A**: not the right axis for that row

## Function Category Matrix

| Category | Promoted packet | Runtime route | Rust lane | TypeScript lane | Atom proof | Molecule role | Semantic review | Benchmark role | Public role | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `function.arithmetic_leaf.monotone_down_nonnegative.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Indirect | Yes | Positive in `BENCH-ECOM` and `BENCH-SERVICE`; companion-negative visible in `BENCH-CROSSLIB` | Direct | Canonical discount-like leaf; supports zero-or-one helper dep shapes in current truth |
| `function.arithmetic_leaf.monotone_up.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Indirect | Yes | Positive in `BENCH-ECOM` and `BENCH-SERVICE`; companion-negative visible in `BENCH-CROSSLIB` | Direct | Canonical tax-like leaf; also the explicit M32 second-language pilot anchor |
| `function.wrapper.pipeline.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Indirect and flow-bearing | Yes | Positive in `BENCH-ECOM` and `BENCH-SERVICE`; companion-negative visible in `BENCH-CROSSLIB` | Direct | Straight-line two-call wrapper; current benchmark-positive wrapper baseline |
| `function.wrapper.pipeline.normalized_required_arg.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Indirect and flow-bearing | Yes | Positive in `BENCH-ECOM` and `BENCH-SERVICE` | Direct | Narrow required-arg wrapper follow-on; TypeScript root eligibility is covered in validator truth even though examples lean more on wrapper and chain3 |
| `function.wrapper.pipeline.chain3.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Indirect and flow-bearing | Yes | Companion-negative visible in `BENCH-CROSSLIB`; not part of current positive `BENCH-ECOM` or `BENCH-SERVICE` walls | Shipped but not positive-wall anchored | Straight-line three-call wrapper pipeline; current product support is ahead of current positive benchmark anchoring |
| `function.helper.identity_passthrough.v1` | Yes | Yes | Yes | Yes, bounded | Yes | Helper-only | Yes | Positive closure support inside `BENCH-ECOM`; helper-only marker in benchmark evidence | Helper-only | Real shipped function category, but not separately named in the plain-English Rust V1 claim |
| `unsupported.function.v1` | No | Terminal catch-all | N/A as a supported category | N/A as a supported category | N/A as a supported category | N/A | Yes, as unsupported verdict | No positive credit | Observation only | Honest additive unsupported surface; may coexist with real backend/proof truth for a unit, but never upgrades to supported-family credit |
| bounded generics for `kind:function` | No | No | Deferred | Deferred | Deferred | Deferred | Deferred | No | Deferred | Explicit `V1.1` authored-shape expansion; not a new kind |
| async flows, runtime adapters, and IO-owned boundaries | No | No | Deferred | Deferred | Deferred | Deferred | Deferred | No | Deferred | Explicit `V1.1` backend/runtime/proof expansion; Rust V1 remains synchronous-only today |

## TypeScript Lane Notes

The bounded TypeScript lane for `kind:function` currently has these repo-level
constraints:

- function-only, not seam kinds
- atom-only proof
- no `.test.spec` molecule execution
- bounded local-graph and cross-library closure rules
- every reachable unit must classify to a supported semantic review
- every reachable unit must author `body.typescript`

That means the TypeScript column in the matrix is not a broad "JavaScript
backend parity" claim. It is a bounded execution lane over already-supported
function categories.

### Important maintainer-path caveat

The maintainer-facing family workflow is narrower than the product lane:

- `cargo xtask family prove --target-language typescript` currently allows only
  `function.arithmetic_leaf.monotone_up.v1` and `function.wrapper.pipeline.v1`

That does **not** shrink the product-lane truth recorded above. It means the
family-maintainer command surface is a stricter path than the broader bounded
`spec` execution lane.

## What This Matrix Clarifies

This matrix makes a few important truths visible:

1. `kind:function` is not one undifferentiated support blob.
   Some categories are positive-wall anchored, some are helper-only, and some
   are shipped but not yet positive benchmark anchors.
2. The main `function` gap is no longer "do we have families?"
   The main gap is category completion, benchmark anchoring, and deferred-edge
   decisions.
3. `chain3` is a real shipped function category, but current public proof walls
   do not anchor it the way they anchor the arithmetic, wrapper, and
   normalized-required-arg categories.
4. The bounded TypeScript lane is broader than the narrow maintainer
   `xtask family prove --target-language typescript` path.
5. Deferred `V1.1` surfaces are support-dimension expansions for
   `kind:function`, not candidates for a fourth top-level kind.

## How To Update This Matrix

When function truth changes, update this document after checking:

1. `cargo xtask family inventory --format json`
2. `README.md`
3. `docs/core_mechanisms_guide_v0.1.md`
4. `PLAN.md`
5. `DECISIONS.md`
6. benchmark labels and current benchmark evidence
7. current validator / semantic review / CLI test truth for TypeScript and proof

Update the smallest correct thing:

- a row in the matrix
- a note about benchmark role
- the TypeScript notes
- the inventory snapshot

Do **not** mark a function category as positive-wall anchored unless the current
benchmark labels and evidence actually include it in `BENCH-ECOM` or
`BENCH-SERVICE`.

Do **not** mark a category as helper-only unless the benchmark and claim surfaces
really treat it as closure support rather than the thing being claimed.

## Likely Next Follow-On

If `kind:function` keeps growing, the next artifact after this one should
probably be machine-readable rather than another prose-only doc:

- a checked-in category registry with columns for:
  - category id
  - promoted packet status
  - runtime route status
  - Rust lane
  - TypeScript lane
  - atom proof
  - molecule role
  - semantic-review status
  - benchmark role
  - public role

That would let the repo compute progress instead of only describing it.
