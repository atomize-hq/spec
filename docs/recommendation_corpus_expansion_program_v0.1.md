# Recommendation Corpus Expansion Program
**Version:** v0.1
**Status:** Active tracking doc
**Date:** 2026-05-01

## Purpose

This document tracks the bounded corpus-expansion program that follows M27.75.

It is not a milestone implementation plan.

Per-run implementation still belongs in a milestone-scoped `PLAN.md` or
equivalent plan artifact. This file exists to track the program across multiple
small corpus-expansion runs so the repo can answer one question honestly:

> Do we need more evidence, or is it time to move to M28?

## Why This Exists

M27.5 fixed the recommendation honesty problem.

M27.75 improved the evidence surface, but the current output still says:

- `recommendation_status = "no_strong_candidate"`
- the top visible candidate is still held for `unknown_overlap_family`
- the second visible candidate is still held for
  `thin_real_example_support` and `thin_regression_support`

That means the repo is now good at telling the truth, but the truth is still:
the next-family decision is not settled yet.

The right response is not to pre-commit to 5-10 corpus milestones.
The right response is a bounded iterative program with hard review gates after
each run.

## Relationship To Milestone Plans

Use this division of responsibility:

- This document tracks the multi-run program:
  - current baseline
  - how many evidence-expansion runs have been completed
  - what metrics matter
  - what conditions mean "keep going" vs "stop and pivot"
- Each individual run still gets its own high-rigor implementation plan:
  - exact scope
  - exact files
  - exact expected output deltas
  - exact proof loop

In short:

- `docs/recommendation_corpus_expansion_program_v0.1.md` = program tracker
- `PLAN.md` = one specific run contract

## Program Shape

This is a **bounded iterative evidence-expansion program**.

It is intentionally not an open-ended "just keep adding corpus forever" loop.

### Default operating rule

After each corpus-expansion run:

1. rerun coverage
2. rerun recommendation
3. validate artifacts
4. compare against the prior baseline
5. decide one of:
   - run another evidence milestone
   - pivot to a family-promotion/policy milestone
   - stop corpus work and begin M28

### Recommended cap

- Default cap: **up to 3 additional corpus-expansion runs after M27.75**
- Hard ceiling without explicit re-justification: **5 total additional runs**

The cap is not the goal.

It exists to stop the repo from turning "more evidence" into a permanent excuse
to avoid the architectural decision.

## Current Program State

### Baseline anchor

- Baseline milestone: `M27.75`
- Baseline branch context: `feat/m27`
- Baseline proof date: `2026-05-01`
- Baseline status: `no_strong_candidate`

### Counter

- Completed additional corpus-expansion runs after M27.75: `0`
- Current recommended budget remaining before forced decision: `3`
- Current hard ceiling remaining before re-justification: `5`

## Current Baseline Metrics

These values are the starting truth for the program.

### Coverage baseline

- source ids:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- source unit counts:
  - `6`
  - `12`
  - `9`
  - `1`
  - `1`
- `function_coverage.total_units = 27`
- `function_coverage.promoted_family_units = 15`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 12`

### Recommendation baseline

- `recommendation_status = "no_strong_candidate"`
- visible held candidate count: `2`

Candidate 1:

- cluster id: `unsupported_function_surface-e40675da6fa0`
- overlap family: `unknown`
- promotion readiness: `hold`
- hold reasons:
  - `unknown_overlap_family`
- leverage:
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 3`

Candidate 2:

- cluster id: `unsupported_arithmetic_shape-2694b2baf65b`
- overlap family: `function.arithmetic_leaf.monotone_*`
- promotion readiness: `hold`
- hold reasons:
  - `thin_real_example_support`
  - `thin_regression_support`
- leverage:
  - `real_example_hits = 1`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 2`

## What Counts As a Good Run

A corpus-expansion run is good only if it improves decision quality.

That means at least one of these must happen:

- `unknown_overlap_family` resolves into a real overlap direction
- `real_example_hits` increases for a rankable cluster that matters
- `promotion_relevant_regression_hits` increases for a rankable cluster that matters
- a previously invisible but plausible candidate becomes rankable
- a held candidate becomes `ready`
- the repo gains enough evidence to justify saying corpus growth is no longer the blocker

What does **not** count as a good run:

- adding examples that do not materially change cluster shape
- increasing raw corpus size without improving decision clarity
- creating more visible held candidates without narrowing the next step
- changing policy mid-run and pretending the output improvement came from evidence

## Allowed Run Types

Future runs in this program may do one or more of:

- add maintained repo-owned real examples to the manifest
- add targeted regression examples that improve promotion-relevant evidence
- add examples that help resolve `unknown_overlap_family`
- add examples that pressure-test arithmetic-shape demand

Future runs in this program must **not** do any of:

- change recommendation policy unless the run is explicitly re-scoped away from corpus work
- change artifact schemas as incidental work
- start shared-core extraction as side work
- start second-language proof work
- use packet fixtures as recommendation corpus leverage

## Review Workflow After Each Run

After each new corpus-expansion milestone lands, run:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Then review these questions in order:

1. Is the top-level status still `no_strong_candidate`?
2. Did the top candidate's hold reasons get simpler, stronger, or narrower?
3. Did `unknown_overlap_family` resolve?
4. Did arithmetic-shape evidence become materially thicker?
5. Did any candidate become `ready`?
6. If nothing decisive changed, is the missing evidence still specific enough to justify one more corpus run?

## Stop Rules

Stop the corpus-expansion program early if any of these become true:

### Stop Rule A — promotion path unlocked

- a candidate becomes clearly promotion-worthy
- or the output is now specific enough to justify a narrow promotion-focused milestone

### Stop Rule B — corpus is no longer the blocker

- the same hold reasons survive two successive runs
- and leverage only improves marginally
- and no new overlap clarity appears

That means more corpus is probably not the main problem anymore.

### Stop Rule C — cap reached

- the program reaches the default cap of 3 additional runs after M27.75

At that point, the default next move is:

- begin M28

unless one very specific missing evidence slice is still obviously worth one last run.

### Stop Rule D — hard ceiling reached

- the program reaches 5 total additional runs after M27.75

At that point, continuing corpus work requires an explicit written re-justification.

## Decision Outcomes

Each review checkpoint must end with one of these outcomes:

### Outcome 1 — Run another corpus-expansion milestone

Use this only when:

- the missing evidence is specific
- the next wedge is small
- and the likely payoff is improved decision clarity rather than just a larger corpus

### Outcome 2 — Switch to a promotion/policy milestone

Use this when:

- the evidence is now strong enough
- or the real blocker is no longer corpus size but recommendation interpretation or family choice execution

### Outcome 3 — Begin M28

Use this when:

- the corpus has done enough
- the engine remains honest but indecisive
- and the next bottleneck is now architectural portability rather than family-choice evidence

## Program Run Log

| Run | Milestone / Plan | Date | Corpus delta | Recommendation status | Key candidate change | Decision |
|---|---|---|---|---|---|---|
| Baseline | `M27.75` | 2026-05-01 | `3 -> 5` sources | `no_strong_candidate` | `money/round` gains second real-example hit; arithmetic-shape candidate becomes visible | Continue program |
| 1 | — | — | — | — | — | — |
| 2 | — | — | — | — | — | — |
| 3 | — | — | — | — | — | — |
| 4 (requires justification if beyond default cap) | — | — | — | — | — | — |
| 5 (hard ceiling) | — | — | — | — | — | — |

## Template For Future Run Entries

When a new run completes, update the log with:

- milestone id or plan title
- exact corpus delta
- resulting top-level recommendation status
- change in hold reasons
- change in leverage counts
- explicit next decision:
  - continue corpus program
  - switch to promotion/policy milestone
  - begin M28

## Immediate Next-Step Guidance

As of this baseline, the recommendation is:

- do **not** commit to a long 5-10 milestone corpus roadmap
- do **not** jump to M28 yet
- do one more small evidence-expansion milestone
- review immediately after that run using this document

The current evidence says the repo still needs one more honest attempt at
decision-thickening before it declares the architectural blocker has taken over.
