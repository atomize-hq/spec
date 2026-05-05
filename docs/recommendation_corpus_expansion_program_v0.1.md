# Recommendation Corpus Expansion Program
**Version:** v0.1
**Status:** Review checkpoint reached
**Date:** 2026-05-02

## Purpose

This document tracks the bounded corpus-expansion program that follows M27.75.

It is not a milestone implementation plan.

Per-run implementation still belongs in a milestone-scoped `PLAN.md` or
equivalent plan artifact. This file exists to track the program across multiple
small corpus-expansion runs so the repo can answer one question honestly:

> Do we need more evidence, or is it time to move to M28?

## Why This Exists

M27.5 fixed the recommendation honesty problem.

M27.75 improved the evidence surface, and M27.8R then repaired the seeded
command-path harness so the repo now reproduces the truthful post-expansion
output consistently.

The current truthful output says:

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_analysis_schema_version = 4`
- `recommendation_status = "no_strong_candidate"`
- `decision_status = "not_recommended"`
- the arithmetic-ready cluster no longer survives as a ranked next-step driver
- `money/round` remains visible through
  `unsupported_function_surface-e40675da6fa0`
- `money/round` is not the next family
- corpus run `1` remains unspent and unauthorized by default

That means this program is no longer answering "is the engine honest?" or even
"is there any rankable candidate at all?" It is now answering a narrower
question:

> Is any later corpus run worth explicitly justifying, or has the program
> already produced enough truth to leave corpus run `1` unspent by default?

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

## Frozen Terminology Source

This tracker consumes the authoritative M27.9B vocabulary freeze verbatim from
commit `44836f42ea75937f85e9ec72658eb7238db35dd9`.

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

- Program origin milestone: `M27.75`
- Current truthful baseline milestone: `M27.9` stop-state
- Baseline branch context: `feat/corpus-expansion`
- Baseline proof date: `2026-05-02`
- Baseline status: `no_strong_candidate`
- Baseline note: the M27.9 stop-state records semantic implementation success
  plus accounting failure; it does not consume the corpus-run budget and it
  does not authorize another corpus run by default

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
  - `2`
- `function_coverage.total_units = 28`
- `function_coverage.promoted_family_units = 17`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 11`

### Recommendation baseline

- `recommendation_analysis_schema_version = 3`
- `recommendation_status = "no_strong_candidate"`
- ranked candidate count: `0`
- retired historical expectation: `unsupported_arithmetic_shape-2694b2baf65b`
  previously looked `ready`, but that was the wrong accounting target rather
  than the truthful post-fix recommendation result

Next visible held candidate:

- cluster id: `unsupported_function_surface-e40675da6fa0`
- promotion readiness: `hold`
- hold reason: `helper_surface_not_promotable`
- next step status: `durable_hold`
- next step detail: `helper_surface_not_promotable`
- leverage:
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 3`

Current governance meaning:

- `money/round` remains visible
- `money/round` is not the next family
- corpus run `1` remains unspent and unauthorized by default

## What Counts As a Good Run

A corpus-expansion run is good only if it improves decision quality.

That means at least one of these must happen:

- a different candidate gains enough evidence to become the next scoped target
- `real_example_hits` increases for a later explicitly justified candidate
- `promotion_relevant_regression_hits` increases for a later explicitly
  justified candidate
- a previously invisible but plausible candidate becomes rankable
- a held candidate other than the durable-held `money/round` helper surface
  becomes `ready`
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
- add examples that support a later explicitly scoped candidate
- add examples that clarify a later candidate without reopening retired
  arithmetic pressure

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
2. Did the visible held candidate's hold reasons get simpler, stronger, or narrower?
3. Did any non-held candidate become specific enough to justify a scoped
   follow-on milestone?
4. Did any surviving candidate gain materially better promotion clarity without
   reopening `money/round` as the default target?
5. Did any candidate become `ready`?
6. If nothing decisive changed, is the missing evidence still specific enough to justify one more corpus run?

Before asking question 6, apply one additional gate:

7. Did the latest truthful baseline already unlock Stop Rule A without spending
   another corpus run?

## Stop Rules

Stop the corpus-expansion program early if any of these become true:

### Stop Rule A — promotion path unlocked

- a candidate becomes clearly promotion-worthy
- or the output is now specific enough to justify a narrow promotion-focused milestone

Current state: this rule is not met at the M27.9 stop-state because no ranked
candidate survives. `money/round` is still visible under
`unsupported_function_surface-e40675da6fa0`, but the durable hold means it is
not the next family and does not authorize corpus run `1` by default.

### Stop Rule B — corpus is no longer the blocker

- the same hold reasons survive two successive runs
- and leverage only improves marginally
- and no new promotion clarity appears

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
| Checkpoint | `M27.8R` | 2026-05-02 | no corpus delta; harness-truth repair only | `ranked` | arithmetic-shape cluster becomes `ready`; `money/round` remains visible as the held helper surface later frozen as durable hold in M27.9B | Switch to promotion-focused milestone |
| Closeout checkpoint | `M27.9` stop-state | 2026-05-02 | no corpus delta; semantic implementation landed, accounting target failed | `no_strong_candidate` | arithmetic-ready pressure retires; `money/round` remains visible but is not the next family, and corpus run `1` stays unspent pending the M27.9B freeze | Hold corpus program; close out accounting |
| Decision contract | `M34` | 2026-05-05 | no corpus delta; read-side decision contract only | `no_strong_candidate` | `money/round` remains a durable helper-surface hold under `helper_surface_not_promotable`; M35 keeps that non-promotability call on one shared classifier surface | `pivot_to_architecture_shared_core_follow_on`; keep corpus run `1` unspent |
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

As of the M34 decision-contract closeout, the repo now records two distinct
truth surfaces:

- `recommendation.latest.json` is the M33 recommendation-analysis input
- `corpus-program-decision.latest.json` is the M34 stop/spend/pivot output

Run the bounded decision command with:

```bash
cargo xtask family corpus-decision --format json
```

For the current live wedge, that artifact emits:

- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

At the frozen M35 boundary, `money/round` still represents real helper-surface
pressure, but the `helper_surface_not_promotable` classification is owned by
one shared classifier surface. This document treats
`recommendation.latest.json` as the input truth that carries that
classification, while `corpus-program-decision.latest.json` remains the
operator-action output that tells maintainers to stop, spend, or pivot without
claiming broader implementation scope.

M36 preserves that frozen M35 wedge. One shared classifier surface remains the
only helper follow-on contract owner, so
`helper_surface_not_promotable` still feeds
`durable_non_promotable_helper_surface`, which still yields
`pivot_to_architecture_shared_core_follow_on` plus
`author_architecture_follow_on_plan`. Corpus run `1` remains unspent.

The operational guidance is therefore:

- do **not** commit to a long 5-10 milestone corpus roadmap
- do **not** spend corpus run `1` yet
- do **not** reopen arithmetic-ready pressure as the default next-step driver
- do **not** claim that M34 implemented the architecture/shared-core follow-on
- keep this document as the corpus fallback ledger while the repo records the
  truthful bounded pivot decision
- keep `money/round` visible without treating it as the next family

The current evidence no longer supports "one more corpus run by default."
It also no longer supports steering from the retired arithmetic-ready story.
The truthful closeout state is `28 / 17 / 0 / 11` with
`recommendation_status = "no_strong_candidate"` and
`decision_status = "not_recommended"`, while `money/round` remains visible
under `helper_surface_not_promotable`. In the M33 vocabulary, a plausible
future candidate with evidence gaps would read `blocked_for_now`, while a truly
promotion-worthy candidate would read `recommended`. Corpus run `1` remains
unspent, and the explicit M34 next step is
`pivot_to_architecture_shared_core_follow_on`, not silent corpus continuation.
That frozen M35 wording does not promote a new family, spend corpus run `1`, or
move the helper-surface classification out of the shared classifier path.

When reviewing the read-side outputs for that frozen wedge, raw latest-artifact SHA is not semantic identity, and normalized semantic fingerprints are the proof surface. M36 keeps the semantic contract anchored on normalized meaning rather than byte-for-byte churn in `*.latest.json`.
