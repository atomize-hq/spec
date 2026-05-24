# spec — Core Mechanisms Guide
**Version:** v0.1  
**Status:** Active repo guide  
**Date:** 2026-05-24

## Purpose

This document is the repo's "start here" explanation for the main mechanisms
at play in `spec`.

It exists because several different systems are active at once:

- authored semantic units
- backend lowering and execution
- atom and molecule proof
- semantic review
- promoted semantic families
- corpus and recommendation analysis
- benchmark-based product claims

These systems are related, but they are not the same thing. The goal of this
guide is to make the boundaries memorable so repo docs can point back here
instead of re-explaining the whole stack from scratch every time.

## Short Version

If you feel lost, ask these questions in order:

1. What **kind** of unit is this?
2. Can it **lower and execute** on the backend I care about?
3. Does it have **current proof**?
4. Does **semantic review** understand its meaning?
5. If yes, which **family** is it in?
6. Does it count toward a **benchmark/product claim**?
7. If it is not understood yet, does the **corpus** suggest a new family to
   promote?

That is the main stack.

## The Two Layers

The cleanest separation in this repo is:

### 1. Write-path truth

This is the product's semantic-source system.

It owns:

- `*.unit.spec`
- `*.test.spec`
- validation
- normalization
- graph resolution
- generation
- build and test execution
- passports
- molecule evidence
- `status` and `export` projections

This is the layer that changes semantic repo truth.

### 2. Read-side interpretation

This is everything that reads the write-path truth and interprets it.

Examples:

- semantic review
- benchmark accounting
- coverage and recommendation artifacts
- corpus-program decisions

These consumers interpret the graph and its proof state. They do not define the
semantic graph itself.

## The Main Questions

Each unit can be asked several different questions. Most confusion comes from
mixing them together.

| Question | Mechanism | Short meaning |
| --- | --- | --- |
| Is this shape legal `spec` source? | Core support | The authored unit shape is allowed and valid. |
| Can this run on Rust or TypeScript? | Backend support | A target-language lane can lower and execute it. |
| Did it actually prove? | Atom / molecule proof | Current local-test and molecule evidence exists and is fresh. |
| Does the reviewer understand its function meaning? | Semantic review support | The bounded semantic reviewer can classify the function honestly. |
| Which meaning bucket is it in? | Family support | The function matches one shipped semantic family. |
| Does it count toward a public claim? | Benchmark support | It earns positive credit toward a bounded product claim such as Rust V1. |
| What should we teach the reviewer next? | Corpus / recommendation | Checked-in examples create pressure for future family promotion. |

## The Core Mechanisms

### Kinds

Kinds answer:

> What sort of unit is this structurally?

Current top-level authored kinds:

- `function`
- `data`
- `sum`

Kinds are part of `spec` core support. If the repo supports `kind:data`, that
means the authored seam shape is valid and can participate in the normal
pipeline. It does **not** automatically mean semantic review understands that
kind.

### Seams

In current repo language, a seam is usually a non-function unit shape that owns
shared semantics plus nested behavior.

Examples:

- `kind:data`
- `kind:sum`

Seams are about the product model and authored shape. They are not the same as
semantic families.

### Backends

Backends answer:

> Can this authored unit lower and execute on backend X?

Today the important lanes are:

- Rust
- bounded TypeScript

Backend support is narrower than core support. A unit can be valid `spec`
source and still not be executable in every backend lane.

The current TypeScript lane is intentionally smaller than Rust:

- only bounded `kind:function` closure graphs
- only already-supported function-family shapes
- atom-only proof
- no molecule TypeScript execution
- no seam-kind TypeScript execution

### Atom and Molecule Proof

Proof answers:

> Did this thing actually validate, build, and test successfully, and is that
> proof current?

Two proof surfaces matter:

- **atom proof**: local unit-owned tests from `local_tests`
- **molecule proof**: multi-unit interaction proof from `.test.spec`

This is the main runtime truth layer. A unit can be structurally supported and
still have failing or stale proof.

### Passports

Passports answer:

> What is this unit's stored proof record?

Passports are derived per-unit artifacts. They store things like:

- observed test results
- freshness anchors
- semantic review truth from the last proof refresh
- provenance

Passports are about current per-unit proof state. They are not recommendation
artifacts and they are not benchmark declarations.

### Semantic Review

Semantic review answers:

> Does the bounded reviewer understand this function's meaning shape?

Important constraints:

- it is intentionally narrow
- it is not arbitrary code understanding
- it is mainly first-class for `kind:function`
- only `spec test` refreshes semantic review truth

Today it recognizes a small shipped function vocabulary such as arithmetic
leaves and wrapper pipeline forms. It can also explicitly mark a function as
unsupported.

It does **not** currently imply general understanding of seam kinds like
`kind:data` or `kind:sum`.

### Families

Families answer:

> If semantic review understands this function, what supported meaning bucket is
> it in?

A family is a shipped semantic-review bucket, not a seam and not a benchmark.

Examples:

- `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `function.arithmetic_leaf.monotone_up.v1`
- `function.wrapper.pipeline.v1`

Families are the current semantic-review vocabulary.

### Promotion

Promotion answers:

> When does a candidate family become real shipped support?

A family is not supported just because someone can name it or because the
corpus contains several examples of it. It becomes real shipped support only
after the promotion workflow proves and certifies it as an honest boundary.

Promotion is what turns:

- "we see this shape"

into:

- "the reviewer now supports this shape honestly"

### Corpus

The corpus answers:

> What shapes are actually showing up in checked-in examples and regressions?

The corpus is an observation surface. It is used to detect:

- already-supported coverage
- unsupported pressure
- repeated adjacent shapes
- whether a future family might be worth promotion

The corpus does **not** itself expand semantic-review capability.

### Coverage and Recommendation Artifacts

These artifacts answer:

> Given the corpus, what does the repo currently observe and what should happen
> next?

Examples:

- `coverage.latest.json`
- `recommendation.latest.json`
- `corpus-program-decision.latest.json`

These are read-side analysis outputs. They do not define repo truth; they
interpret it.

### Benchmarks

Benchmarks answer:

> Which selected cases count toward a bounded public product claim today?

Benchmarks are curated proof walls, not authored semantic source.

For Rust V1, they back claims like:

- which roots are proof-authoritative
- which cases count as positive credit
- which negative walls stay visible without earning positive credit

Benchmarks read existing proof inputs such as passports and molecule evidence.
They do not mint those proofs themselves.

## What "Supported" Actually Means

"Supported" means different things in different layers. Do not collapse them
into one flag.

### Core support

Can `spec` author, validate, and run this shape honestly?

### Backend support

Can backend X lower and execute this shape?

### Proof support

Do we have current atom and molecule evidence?

### Semantic support

Does the bounded reviewer understand this function meaning?

### Family support

Is this function in one of the currently shipped family buckets?

### Product-claim support

Does this case count toward a benchmark-backed public claim such as Rust V1?

### Future-support pressure

Does the checked-in corpus suggest the next family that should be promoted?

## The Most Common Confusions

### Family vs seam

These are different.

- **Family**: a semantic-review bucket for function meaning
- **Seam**: a product/unit-shape concept such as `kind:data` or `kind:sum`

Example:

- `pricing_quote` being a `data` seam means `spec` core supports that unit
  shape.
- It does **not** mean semantic review supports data seams.
- `apply_tax` being in `function.arithmetic_leaf.monotone_up.v1` means semantic
  review supports that function family.

### Valid proof vs semantic support

A unit can prove successfully without semantic review understanding it.

That is why a seam can be:

- valid in Rust
- benchmark-positive
- current in proof

while still remaining unsupported by semantic review.

### Corpus pressure vs capability

The corpus can show that a shape is common without teaching the reviewer how to
understand it.

More examples increase pressure. They do not automatically increase semantic
understanding.

### Benchmarks vs source truth

Benchmarks do not define what the unit is. They define how some selected units
count toward a public product claim.

## A Simple Flow

When a new authored unit enters the repo, the clean mental flow is:

1. Author the unit in `*.unit.spec`.
2. Validate it as legal `spec` source.
3. Lower and execute it on the backend lane being used.
4. Record atom and molecule proof into passports and evidence files.
5. If it is a function, semantic review may classify it into a supported family
   or mark it unsupported.
6. If the unit is part of a benchmark roster, benchmark accounting may count it
   toward a public claim.
7. Separately, checked-in units may feed corpus analysis that suggests future
   family-promotion work.

That is one flow with several distinct readers, not one monolithic support
engine.

## Where To Look

Use this map when you want the source of truth for a concept:

| If you want to understand... | Start here |
| --- | --- |
| the core authored model and workflow | [`README.md`](../README.md) |
| semantic-family capability, corpus, and promotion | [`semantic_family_capability_corpus_guide_v0.1.md`](./semantic_family_capability_corpus_guide_v0.1.md) |
| the current Rust V1 benchmark and claim boundary | [`rust_v1_contract_stack.md`](./rust_v1_contract_stack.md) |
| long-range product vision | [`north_star_v0.2.md`](./north_star_v0.2.md) |
| high-level system architecture | [`high_level_technical_architecture_v0.2.md`](./high_level_technical_architecture_v0.2.md) |
| milestone sequencing and release shape | [`roadmap_and_release_shape_v0.1.md`](./roadmap_and_release_shape_v0.1.md) |

## One-Page Cheat Sheet

Keep this version in your head:

- **Kinds / seams** answer: what is this unit structurally?
- **Backends** answer: can it run on Rust / TypeScript?
- **Passports / evidence** answer: did it prove?
- **Semantic review / families** answer: does the reviewer understand this
  function shape?
- **Corpus / recommendation** answer: what should we teach the reviewer next?
- **Benchmarks** answer: what can we publicly claim today?

## Non-Goals

This document does not try to:

- replace milestone plans
- replace the README command reference
- define current benchmark rosters exhaustively
- define every current family packet in detail
- become the only architecture doc in the repo

Its job is narrower: keep the mental model clean enough that the deeper docs
stay readable.
