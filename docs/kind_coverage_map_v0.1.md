# spec — Kind Coverage Map
**Version:** v0.1  
**Status:** Active working map  
**Date:** 2026-05-24

## Purpose

This document is the repo's first-pass progress map for `spec` kinds and the
categories inside each kind.

It exists because the repo already has the raw truth needed to answer support
questions, but it does not yet have one place that turns that truth into a
navigation map.

Without that map, the project keeps re-asking vague questions like:

- "what is still missing?"
- "is this a new kind or just a wider function surface?"
- "how far are we from completion for this shape?"

This doc is meant to make those questions concrete.

## What This Map Measures

This map tracks three different things:

1. **Top-level kinds**
   The structural unit shapes the repo currently owns.
2. **Per-kind categories**
   The named buckets or sub-surfaces inside a kind.
3. **Support dimensions**
   The axes that determine how "real" a category is today.

The support dimensions used here are:

- **Core**
  Is the authored shape legal `spec` source?
- **Rust**
  Can the Rust lane lower and execute it?
- **TypeScript**
  Can the bounded TypeScript lane lower and execute it?
- **Proof**
  Does the repo have current atom and/or molecule proof patterns for it?
- **Semantic review**
  Can the bounded reviewer classify it honestly?
- **Benchmark claim**
  Does it contribute directly to a bounded public product claim such as Rust V1?

## Legend

- **Shipped**: repo-owned and live today
- **Bounded**: shipped, but intentionally narrow
- **Deferred**: explicitly named as future work, not part of current truth
- **Unnamed**: clearly a gap, but not yet owned as a named category
- **N/A**: not the right axis for that row

## The Main Rule

Every roadmap move should answer one of these questions explicitly:

1. Is this adding a **new kind**?
2. Is this adding a **new category inside an existing kind**?
3. Is this widening a **support dimension** for an existing category?

If a proposal cannot be placed in one of those buckets, it is probably still
too vague to steer by.

## Top-Level Kind Inventory

| Kind | Structural status | Current category-map status | Rust V1 claim role | Biggest current gap |
| --- | --- | --- | --- | --- |
| `function` | Shipped | Strongest named map in the repo | Direct | Deferred categories are named, but the completion map above them is still thin |
| `data` | Shipped | Thin | Direct as plain seam | No broad category vocabulary beyond one known supported surface |
| `sum` | Shipped | Thin | Direct as plain seam | No broad category vocabulary beyond one known supported surface |
| future kinds | Unnamed | None | None | No checked-in authority for additional top-level kinds today |

## What Is Actually Missing At The Top Level

The repo does **not** currently have a checked-in list of "next kinds."

That means the honest first-pass answer is:

- the current owned kinds are `function`, `data`, and `sum`
- additional top-level kinds are not yet repo-owned enough to appear as real
  roadmap targets
- the larger immediate blind spot is not "which fourth kind is missing?"
- the larger immediate blind spot is "which categories inside `data` and `sum`
  are owned, which are merely hinted, and which support dimensions are still
  absent?"

So the project's map gap is currently more severe **inside kinds** than
**between kinds**.

## Kind: `function`

`kind:function` is the most mature map in the repo.

It already has:

- the richest semantic-review vocabulary
- the richest promotion and corpus-analysis machinery
- the clearest deferred edges
- the clearest distinction between direct claim-bearing families and helper-only
  routes

### Current repo-owned categories

| Category | Current status | Public role | Notes |
| --- | --- | --- | --- |
| `function.arithmetic_leaf.monotone_down_nonnegative.v1` | Shipped | Direct | Canonical discount-like arithmetic leaf |
| `function.arithmetic_leaf.monotone_up.v1` | Shipped | Direct | Canonical tax-like arithmetic leaf |
| `function.wrapper.pipeline.v1` | Shipped | Direct | Straight-line two-call wrapper pipeline |
| `function.wrapper.pipeline.normalized_required_arg.v1` | Shipped | Direct | Narrow required-arg wrapper follow-on |
| `function.wrapper.pipeline.chain3.v1` | Shipped | Direct | Straight-line three-call wrapper pipeline |
| `function.helper.identity_passthrough.v1` | Shipped | Helper-only | Real supported route, but not separately named in the plain-English Rust V1 claim |
| `unsupported.function.v1` | Shipped terminal catch-all | No | Honest additive unsupported surface, not a supported family |

### Explicitly deferred `function` categories

These are already named in repo authority, which is good. The map knows they
exist, even though they are not shipped.

| Deferred category pressure | Current status | Why it matters |
| --- | --- | --- |
| bounded generics for `kind:function` | Deferred to `V1.1` | This is a real authored-shape expansion, not a new top-level kind |
| async flows, runtime adapters, and IO-owned boundaries | Deferred to `V1.1` | This is a backend/runtime/proof expansion for function-shaped units |

### Capability snapshot for `function`

| Dimension | Current truth | Gap shape |
| --- | --- | --- |
| Core | Shipped | Main gaps are not legality; they are category and boundary growth |
| Rust | Strongest lane | Deferred edges are generics and async/IO |
| TypeScript | Bounded | Narrower than Rust and not a parity claim |
| Proof | Strong | Atom proof is first-class; molecule proof exists for interactions |
| Semantic review | Strongest area in repo | Still intentionally narrow, not general code understanding |
| Benchmark claim | Strong | Direct contributor to the benchmark-backed Rust V1 story |

### What is still missing for `function`

The missing map work for `function` is **not** "what is a function?"
The missing map work is:

- an explicit completion matrix for every named function category across Rust,
  TypeScript, proof, semantic review, and benchmark use
- a sharper distinction between categories that are merely supported and
  categories that are central to the product claim
- a decision framework for when a deferred `V1.1` category is worth more than a
  non-function wedge such as migration, recommendation trust, or workflow
  completion

## Kind: `data`

`kind:data` is structurally real and publicly claim-bearing today, but its map
is much thinner than `kind:function`.

Current structural truth is clear:

- `data.fields`
- `constructors`
- `methods`
- no top-level `contract`
- no top-level `deps`
- no top-level `imports`
- top-level `body.rust` must stay empty
- no top-level `body.typescript`

That means the authored shape is real. The missing piece is a broader category
vocabulary above that shape.

### Current repo-owned categories

| Category | Current status | Public role | Notes |
| --- | --- | --- | --- |
| `data.pricing_quote.v1` | Shipped, bounded | Direct as plain data seam | Specific descriptor-based supported data surface |

### Capability snapshot for `data`

| Dimension | Current truth | Gap shape |
| --- | --- | --- |
| Core | Shipped | The structural seam contract is real |
| Rust | Shipped | Part of the current narrow-core Rust V1 claim as a plain data seam |
| TypeScript | No | No seam-kind TypeScript execution today |
| Proof | Shipped | Can be benchmark-positive and proof-bearing today |
| Semantic review | Bounded and specific | Not a broad family system; currently specific supported data surfaces only |
| Benchmark claim | Direct | Plain data seams are part of the current Rust V1 claim |

### What is still missing for `data`

This is one of the biggest map holes in the repo.

What is missing is not basic shape support. What is missing is a named category
taxonomy such as:

- which data-surface descriptors are first-class and worth owning
- which are merely one-off example seams
- which should get descriptor-based semantic review keys
- which should count toward benchmark claims
- which should remain valid core seams without any semantic classification

So the first-pass honest state for `data` is:

- one known supported semantic surface exists
- the general category map for data seams is still mostly **Unnamed**

## Kind: `sum`

`kind:sum` is in the same overall shape as `data`:

- structurally real
- publicly claim-bearing as a plain seam
- much thinner category map than `function`

Current structural truth is clear:

- `sum.variants`
- `methods`
- no top-level `contract`
- no top-level `deps`
- no top-level `imports`
- top-level `body.rust` must stay empty
- no top-level `body.typescript`

### Current repo-owned categories

| Category | Current status | Public role | Notes |
| --- | --- | --- | --- |
| `sum.discount_strategy.v1` | Shipped, bounded | Direct as plain sum seam | Specific descriptor-based supported sum surface |

### Capability snapshot for `sum`

| Dimension | Current truth | Gap shape |
| --- | --- | --- |
| Core | Shipped | The structural seam contract is real |
| Rust | Shipped | Part of the current narrow-core Rust V1 claim as a plain sum seam |
| TypeScript | No | No seam-kind TypeScript execution today |
| Proof | Shipped | Can participate in proof and benchmark-positive roots today |
| Semantic review | Bounded and specific | Not a broad family ecosystem; currently specific supported sum surfaces only |
| Benchmark claim | Direct | Plain sum seams are part of the current Rust V1 claim |

### What is still missing for `sum`

The missing map work mirrors `data`:

- a named category vocabulary beyond one known supported surface
- a rule for which sum descriptors deserve semantic-review keys
- a rule for which sum surfaces are benchmark-relevant versus merely valid
- a progress view that distinguishes "valid sum seam" from "strategically
  important sum seam"

So the first-pass honest state for `sum` is:

- one known supported semantic surface exists
- the general category map for sum seams is still mostly **Unnamed**

## Cross-Kind Gaps

Across all current kinds, the main missing map work is:

1. **A real per-kind category registry**
   `function` mostly has one. `data` and `sum` mostly do not.
2. **A per-category capability matrix**
   Today the repo often knows a category exists, but not all the support
   dimensions that make it feel "complete."
3. **A benchmark-eligibility map**
   Especially important for keeping helper-only routes distinct from direct
   claim-bearing categories.
4. **A completion scoreboard**
   Something that answers:
   - how many named categories exist in this kind?
   - how many are shipped?
   - how many are deferred?
   - how many are still unnamed pressure?

## What This Map Says About "What Next"

This first-pass map suggests a sharper roadmap frame:

- if the project wants to expand `kind:function`, it should say which category
  or deferred edge is being widened
- if the project wants to make seam kinds feel complete, it probably does **not**
  need a new top-level kind first
- it probably needs explicit category taxonomies for `data` and `sum`
- if the project wants a durable progress meter, the next artifact after this
  one should be a machine-readable or table-driven category registry rather than
  another broad roadmap essay

## Immediate Follow-On Artifacts

The next useful docs after this one would be:

1. `docs/function_category_matrix_v0.1.md`
   One row per function category, one column per support dimension.
2. `docs/data_category_taxonomy_v0.1.md`
   Name the currently owned and still-unnamed `kind:data` categories.
3. `docs/sum_category_taxonomy_v0.1.md`
   Name the currently owned and still-unnamed `kind:sum` categories.
4. A future machine-readable registry
   So the repo can compute progress instead of reasoning from prose only.

## Bottom Line

The repo does have a map now, but only in fragments.

Today:

- `function` has the strongest category map
- `data` and `sum` have real structural support but thin category vocabularies
- no additional top-level kinds are currently owned enough to steer by

So the clearest current planning need is:

> build the per-kind category maps to the point where the repo can measure
> completion honestly, instead of talking about support in one overloaded way

