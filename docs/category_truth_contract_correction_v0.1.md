# spec — Category Truth Contract Correction
**Version:** v0.1  
**Status:** Corrective design note  
**Date:** 2026-05-25

## Purpose

This note records the contract correction discovered while reviewing
`feat/m101-category-truth-registry`.

The category-truth milestone direction is still right:

- shared registry-owned category truth
- one shared qualification contract
- benchmark, status, export, and snapshot consumers using the same rules

But the current M101 implementation shape is not safe to land as-is.

It tightened benchmark honesty, then crossed the producer/consumer boundary by
letting read-side surfaces recompute fresh seam category truth.

This note makes the correction explicit so the repo can keep the good parts of
M101 without silently changing the semantic-truth contract.

## Decision

The repo should keep the **stored-truth strict** model.

That means:

1. `semantic_review` remains producer-owned truth.
2. Only `spec test` may refresh `semantic_review`.
3. `category_qualification` is consumer-side interpretation of already-stored
   semantic truth.
4. `spec status`, `spec export`, benchmark projection, snapshot projection, and
   readability projection are consumers, not parallel truth producers.

The repo should **not** introduce a live read-side semantic/category truth plane
inside M101.

If the product ever needs live analysis distinct from passported proof truth,
that must be a later, explicitly named architecture move with its own surface
and contract.

## What Went Wrong In M101

The design in [`category_truth_contract_v0.1.md`](./category_truth_contract_v0.1.md)
correctly identified the failure class:

> consumers must not infer support or positive credit from partial truth

But the first implementation appears to have solved that partly by letting
`status` and `export` derive category-bearing truth from refreshed semantic
projection on read-only paths.

That creates a new failure class:

> a read-side consumer becomes a second semantic/category truth producer

This is the wrong trade.

It makes the system look more honest at the benchmark layer while quietly
weakening the repo's deeper truth boundary.

## Why The Stored-Truth Model Must Win

The repo already has a durable rule:

- passports store semantic review truth from the last proof refresh
- only `spec test` refreshes semantic review truth
- `spec build`, `spec generate`, `spec status`, and `spec export` project
  stored truth

That boundary is valuable because it keeps one answer to:

- where semantic truth comes from
- when it changed
- what evidence anchored it

If M101 lets `status` or `export` recompute fresh category-bearing truth, the
repo no longer has one answer. It has two:

1. proof-time semantic truth
2. read-time semantic/category truth

That is precisely the kind of split that created earlier honesty bugs.

## Correction To The Contract Doc

The contract in [`category_truth_contract_v0.1.md`](./category_truth_contract_v0.1.md)
should be read more narrowly than the first implementation did.

The intended rule must be:

- `category_qualification` may be read-side projected
- but only from preserved/passported `semantic_review`
- not from `SemanticProjectionMode::Refresh` on `status` / `export`
- not from missing-passport synthetic review

In other words:

**read-side projected** does not mean **freshly recomputed semantic truth**.

It means:

**shared consumer interpretation of already-produced truth**.

## Required Implementation Correction

The corrective respin should preserve these M101 ideas:

- the registry substrate in `spec-core`
- stable category qualification enums and reason codes
- benchmark positive-credit gating through qualification
- explicit service-sibling non-qualification in the first wedge

The corrective respin should drop or rewrite these parts:

- any `status` path where refreshed semantic projection feeds
  `category_qualification`
- any `export` path where refreshed semantic projection feeds
  `category_qualification`
- any path where a missing passport can still yield
  `supported_qualified`
- any benchmark/snapshot input path that consumes refreshed seam semantic truth
  instead of preserved truth

## Concrete Respin Plan

Start from the safer pre-M101 line on `feat/i8-final-proof-run` and rebuild the
milestone narrowly.

### Phase 1

Keep the benchmark honesty fix and the shared category-truth substrate.

### Phase 2

Apply qualification at benchmark projection only from preserved semantic truth.

### Phase 3

Add additive `category_qualification` to `status` and `export`, but derive it
only from preserved/passported semantic truth.

### Phase 4

Re-freeze snapshot and readability parity using the same preserved-truth input.

## Acceptance Criteria

The corrected M101 is done only when all of these are true:

1. A unit with no passport does **not** surface `supported_qualified`.
2. `spec status` never emits fresher category truth than the stored passport.
3. `spec export` never emits fresher category truth than the stored passport.
4. Benchmark, snapshot, readability, status, and export all consume the same
   preserved semantic truth when deriving qualification.
5. Service seam siblings remain visible but unqualified until producer truth is
   widened explicitly.

## Not Doing

- introducing a new live analysis truth plane inside this milestone
- mutating passport on-disk schema in the correction wedge
- widening seam semantic support
- solving the broader seam-substrate alias/sibling question in the same patch

## Follow-On If Needed Later

If the repo later decides it truly needs live, recomputed semantic/category
analysis on read-side surfaces, that work should ship as a separate design
packet with:

- a new named output surface
- explicit non-proof status
- a clear distinction from passported semantic truth
- rules for how consumers choose between proof truth and analysis truth

That is a future architecture decision, not part of the M101 correction.
