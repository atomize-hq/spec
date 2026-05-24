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

## Who This Is For

This guide is for a builder or engineer who is new to `spec` and needs the
repo mental model quickly enough to make or review a real change.

It is especially aimed at people doing one of these jobs:

- author or fix a `*.unit.spec`
- explain why `status`, `export`, or a passport says what it says
- figure out whether a behavior is core support, backend support, proof,
  semantic review, or benchmark policy

## What You Should Be Able To Do After Reading This

After this guide, you should be able to:

- identify whether a unit question is about kind, backend, proof, semantic
  review, family, or benchmark claim
- run the first proof loop on a concrete unit without guessing which command
  matters
- know where proof is stored and which commands refresh it
- know when corpus, promotion, and benchmark docs matter, and when they do not

## If You Changed A Spec File

If you just changed a `*.unit.spec` or `.test.spec`, start here instead of
trying to load the whole architecture into your head first:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/apply_tax.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- status examples/ecommerce/units --format json
```

What each command answers:

- `validate`: is the authored source legal `spec` input?
- `build`: can the backend lower and compile it?
- `test`: did it prove, and did it refresh passport plus semantic-review truth?
- `status`: what health state does the repo project right now?

Artifacts to watch:

- generated Rust: `examples/ecommerce/src/generated/`
- unit proof record: `*.spec.passport.json`
- molecule proof record: `*.test.evidence.json`

If you changed an interaction across units, also run the relevant `.test.spec`
molecule test. For the canonical seam flow, that is:

```bash
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
```

## Running Example

Keep `pricing/apply_tax` in your head while reading this doc. It is the clean
function example for the current system:

- `kind:function`
- lowers on Rust, and in the bounded lane on TypeScript
- proves through unit-local atom tests
- can be semantically reviewed into
  `function.arithmetic_leaf.monotone_up.v1`

When seam behavior matters, contrast it with `pricing/pricing_quote`, which is
a `kind:data` seam that can be valid and benchmark-positive without semantic
review understanding it as a function family.

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

## What You Can Ignore On Day 1

If your job is just to author or fix a unit, you can ignore corpus,
recommendation, promotion, and benchmark details until the validate/build/test
loop is green.

Come back to those layers when you are:

- changing semantic-review capability
- deciding whether a new family should be promoted
- making or auditing a public product claim such as Rust V1

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

Most contributors can ignore this section on day 1. It matters when you are
working on family promotion, recommendation honesty, or corpus-program
decisions.

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

Most contributors should only care about benchmarks when they are asking what
the repo can publicly claim today, not when they are just trying to prove a
local fix.

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

Here is the first successful loop for the running example. This is the fastest
way to connect the concepts in this guide to something real.

1. Start with authored truth.
   The source of truth is
   `examples/ecommerce/units/pricing/apply_tax.unit.spec`.

2. Validate the authored unit:

   ```bash
   cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/apply_tax.unit.spec --format json
   ```

   If this fails, you still have a source-shape problem. Nothing about backend
   lowering, proof, semantic review, or benchmarks matters yet.

3. Build the example root:

   ```bash
   cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
   ```

   If this fails, the authored source may be valid, but the backend-lowering or
   compile lane is not.

4. Refresh proof:

   ```bash
   cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
   ```

   This rewrites
   `examples/ecommerce/units/pricing/apply_tax.spec.passport.json`.
   If you also changed cross-unit behavior, run the matching `.test.spec` file
   and refresh the corresponding `*.test.evidence.json`.

5. Project current health:

   ```bash
   cargo run -p spec-cli -- status examples/ecommerce/units --format json
   ```

   Now you are looking at read-side truth. `status` tells you whether the unit
   is `valid`, `stale`, `failing`, `incomplete`, or `untested`; it does not
   create proof on its own.

6. Ask the semantic-review question only after proof exists.
   If the unit is a supported `kind:function` shape, `spec test` may refresh
   semantic-review truth inside the passport. For `pricing/apply_tax`, that can
   include the family
   `function.arithmetic_leaf.monotone_up.v1`.

7. Ask the benchmark question last.
   Benchmark accounting is about whether a proved case counts toward a bounded
   public claim such as Rust V1. It is not the same question as whether the
   unit is valid, compiled, or semantically understood.

That is one flow with several distinct readers, not one monolithic support
engine.

## Where To Look

Use this map when you want the source of truth for a concept:

| If you want to understand... | Start here |
| --- | --- |
| the docs tree itself and which docs are current authority | [`docs/README.md`](./README.md) |
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

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/autoplan` | Scope and strategy | 1 | CLEAR | The guide direction was right, but it needed a much faster newcomer path and a clearer day-1 versus advanced-systems boundary. |
| Codex Review | `codex exec` | Independent second opinion | 1 | ISSUES_FOUND | Outside review agreed the taxonomy was strong but said the doc still did not get a first-time reader to one successful repo action fast enough. |
| Eng Review | `/autoplan` | Commands and artifact pathing | 1 | CLEAR | Added a concrete validate/build/test/status loop, explicit artifact locations, and a running example that ties the concepts to live repo surfaces. |
| Design Review | `/autoplan` | UI and visual UX gaps | 0 | — | No UI scope in this document review. |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | CLEAR | Time to first useful outcome improved from conceptual-only to a five-minute guided loop with commands, expected artifacts, and clear "ignore this for now" boundaries. |

**CODEX:** Independent review said the guide named the system well but front-loaded maintainers' concepts before showing a first successful action.
**UNRESOLVED:** The broader README and docs tree still need a follow-on cleanup so this guide becomes the stable anchor instead of one strong page inside an overloaded docs set.
**VERDICT:** CEO + ENG + DX CLEARED for this revision. Good mental-model anchor, now with a usable newcomer path.
