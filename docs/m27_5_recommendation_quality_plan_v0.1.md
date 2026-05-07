# M27.5 - Recommendation Quality Hardening

Status: **implementation contract**
Base branch: **ws/m27-int**
Working branch: **next**
Last written: **2026-04-30**

## Plan Authority

This file is the authoritative M27.5 plan.

Primary milestone source:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Repo truth checked while writing this plan:

- current `ws/m27-int` M27 implementation and acceptance artifact
- current `xtask/src/family/coverage.rs`
- current `xtask/src/family/recommend.rs`
- current `xtask/src/family/promotion_artifacts.rs`
- current `semantic-families/corpus/rust-function.toml`
- current `coverage.latest.json` and `recommendation.latest.json` behavior on the
  locked three-source corpus

If any older draft or milestone note disagrees with this file, this file wins
for M27.5.

## Problem Statement

M27 landed the coverage and recommendation engine.

That part worked.

The repo can now:

- account for real checked-in corpus coverage
- cluster unsupported function pressure
- rank rankable candidates
- emit deterministic machine-readable artifacts

But the first real output exposed the next problem:

- the engine is willing to emit `recommendation_status = "ranked"`
- even when the top candidate is:
  - `overlap_family = "unknown"`
  - `difficulty = "hard"`
  - backed by only `1` real-example hit
  - backed by only `1` promotion-relevant regression hit

That is mechanically consistent with the current M27 policy.

It is not good enough for roadmap steering.

M27.5 is the milestone that makes the recommendation engine conservative enough
that a `ranked` result actually means "this looks like the next family," not
"the engine found the least-bad thing in a small corpus."

## Current Failure Case

The motivating live case from `ws/m27-int` is the current top candidate:

- `primary_reason_code = unsupported_function_surface`
- representative pressure centered on `money/round`
- `overlap_family = "unknown"`
- `difficulty.tier = "hard"`
- `confidence.level = "medium"` under current M27 rules
- `recommendation_status = "ranked"`

This is exactly the kind of result M27.5 must demote.

## Milestone Outcome

When M27.5 lands, the repo can truthfully claim:

- `family recommend` still works and stays deterministic
- recommendation output is stricter than raw cluster discovery
- `ranked` means promotion-worthy pressure, not merely visible pressure
- candidates with thin evidence, unknown overlap, or high implementation risk
  are held back explicitly
- the current locked corpus can honestly yield `no_strong_candidate` even when a
  discoverable cluster exists

M27.5 does **not** claim:

- the corpus is now large enough forever
- the ranking policy is globally optimal
- the next family is definitely known
- shared-core extraction is solved
- second-language work should start

## Scope

### In Scope

- tighten recommendation-quality policy only
- keep the existing M27 command surface:
  - `cargo xtask family coverage --format json`
  - `cargo xtask family recommend --format json`
  - `cargo xtask family validate-artifact <path>`
- add an explicit promotion-readiness gate on top of M27 cluster ranking
- make hold reasons machine-readable in recommendation artifacts
- update recommendation artifact validation for the tightened schema
- add regression coverage for the current `money/round` false-positive ranking
- update maintainer docs to explain what `ranked` now means

### NOT In Scope

- adding new corpus sources
- promoting the next family packet
- changing coverage accounting rules
- changing unsupported-function fingerprint logic
- ranking non-function seams
- beginning M28 shared-core extraction
- starting second-language work

## Step 0 - Scope Challenge

### What M27 already proved

- The command surfaces are real.
- The analysis artifacts are real.
- The engine can separate promoted coverage from unsupported pressure.
- The engine can surface honest low-confidence outcomes in principle.

### What M27 did not prove

- That the threshold for `ranked` is strict enough.
- That `confidence = "medium"` always means "promote next."
- That unknown-overlap hard candidates should outrank "wait."

### Minimum honest change

The minimum honest M27.5 diff is:

1. keep M27 cluster discovery intact
2. add an explicit promotion-readiness policy layer
3. make hold reasons visible in the recommendation artifact
4. prove that the current `money/round` output no longer graduates to `ranked`

Anything less is just nicer prose around the same weak recommendation behavior.

## Locked Decisions

| Decision | Lock |
|---|---|
| M27 command names change | **Rejected.** Keep the M27 command surface unchanged. |
| Coverage artifact schema changes | **Rejected.** M27.5 is recommendation-quality work, not coverage-accounting work. |
| Recommendation artifact may stay semantically vague | **Rejected.** M27.5 must expose why a candidate is held back. |
| Unknown-overlap candidates may still become `ranked` on thin evidence | **Rejected.** That is the live failure case M27.5 exists to stop. |
| Hard candidates may become `ranked` with only one real example | **Rejected.** M27.5 must require a stronger bar. |
| Corpus expansion is folded into M27.5 | **Rejected.** If more corpus is needed later, make that a separate milestone. |

## Locked M27.5 Contract

### Command Surfaces

M27.5 keeps exactly these subcommands:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact <path>`

No new command is introduced.

### Recommendation Policy Split

M27.5 explicitly separates two ideas that M27 blurred:

1. **Discoverable candidate**
   A cluster that is visible and worth surfacing in analysis output.

2. **Promotion-ready candidate**
   A cluster strong enough to justify `recommendation_status = "ranked"`.

M27 discovered clusters correctly.

M27.5 adds the missing gate between "discovered" and "promote next."

### Recommendation Artifact Contract

M27.5 upgrades only the recommendation-analysis artifact.

Path stays:

- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

Artifact kind stays:

- `family_recommendation_analysis`

Schema version becomes:

- `2`

Each `ranked_candidates[]` entry must still include the existing M27 fields, and
now also includes:

- `promotion_readiness`
- `hold_reasons[]`

Allowed `promotion_readiness` values:

- `ready`
- `hold`

Allowed `hold_reasons[]` values:

- `unknown_overlap_family`
- `hard_difficulty`
- `thin_real_example_support`
- `thin_regression_support`
- `single_source_pressure`

Rules:

- `promotion_readiness = "ready"` requires `hold_reasons[] == []`
- `promotion_readiness = "hold"` requires `hold_reasons[]` to be non-empty
- `ranked_candidates[]` may include both ready and hold candidates
- top-level `recommendation_status = "ranked"` is allowed only when the first
  ranked candidate has `promotion_readiness = "ready"`

### Promotion-Readiness Rules

A candidate must be forced to `promotion_readiness = "hold"` when any of the
following is true:

- `overlap_family == "unknown"`
- `difficulty.tier == "hard"` and `real_example_hits < 2`
- `real_example_hits == 0`
- `real_example_hits == 1` and `promotion_relevant_regression_hits < 3`
- `promotion_relevant_regression_hits <= 1` and `real_example_hits <= 1`
- all promotion-relevant pressure comes from exactly one source id and
  `real_example_hits == 0`

Mapped hold reasons:

- `overlap_family == "unknown"` -> `unknown_overlap_family`
- `difficulty.tier == "hard"` with insufficient real-example support ->
  `hard_difficulty`
- `real_example_hits == 0` or `== 1` at the weak-evidence bar ->
  `thin_real_example_support`
- `promotion_relevant_regression_hits <= 1` ->
  `thin_regression_support`
- single-source non-real pressure ->
  `single_source_pressure`

### Confidence Rules

M27.5 tightens confidence.

`confidence.level = "high"` only when:

- `real_example_hits >= 3`, and
- `overlap_family != "unknown"`

`confidence.level = "medium"` only when one of these holds:

- `real_example_hits >= 2`, and `overlap_family != "unknown"`
- `real_example_hits == 1`, `promotion_relevant_regression_hits >= 3`,
  `difficulty.tier != "hard"`, and `overlap_family != "unknown"`

Otherwise:

- `confidence.level = "low"`

This is intentionally stricter than M27.

### Recommendation Status Rules

Evaluate in this order:

- `ranked` when at least one candidate is `promotion_readiness = "ready"` and
  the top ready candidate has `confidence.level` of `medium` or `high`
- `no_strong_candidate` when at least one discoverable candidate exists, but
  every candidate is `promotion_readiness = "hold"`
- `insufficient_real_corpus` when every discoverable candidate is on hold and
  every candidate has `real_example_hits == 0`

### Current-Corpus Locked Expectation

The locked three-source corpus on the current `ws/m27-int` truth must now
produce:

- `recommendation_status = "no_strong_candidate"`

And the current `unsupported_function_surface` / `money/round` candidate must
remain visible but be demoted to:

- `promotion_readiness = "hold"`

With at least these hold reasons:

- `unknown_overlap_family`
- `hard_difficulty`
- `thin_real_example_support`

That regression is the whole point of M27.5.

## Files and Responsibilities

| Area | M27.5 responsibility | Must not happen |
|---|---|---|
| `xtask/src/family/recommend.rs` | implement tightened readiness and status policy | do not re-implement coverage accounting |
| `xtask/src/family/promotion_artifacts.rs` | validate recommendation-analysis schema v2 | do not loosen M27 artifact path rules |
| `xtask/src/lib.rs` tests | add regression coverage for demoted weak candidates | do not silently change unrelated M26/M27 tests |
| `semantic-families/README.md` | explain what `ranked` now means | do not turn README into milestone theory |

## Validation Matrix

Required tests:

- unit test: unknown-overlap hard candidate with `1` real example becomes hold
- unit test: no discoverable candidates -> `insufficient_real_corpus`
- unit test: discoverable-but-held candidates -> `no_strong_candidate`
- unit test: known-overlap adjacent candidate with strong evidence -> `ranked`
- artifact validation test: recommendation-analysis schema v2 accepts
  `promotion_readiness` and `hold_reasons[]`
- regression test: current locked corpus no longer returns `ranked`

## Acceptance Gates

M27.5 is complete only when all of the following are true:

1. `cargo xtask family recommend --format json` remains deterministic.
2. Recommendation artifacts still print to stdout and write identical bytes to
   the locked artifact path.
3. `cargo xtask family validate-artifact <path>` accepts the tightened
   recommendation-analysis artifact schema.
4. Weak candidates remain visible in analysis output instead of disappearing.
5. A candidate with `overlap_family = "unknown"` and `difficulty = "hard"` does
   not cause top-level `recommendation_status = "ranked"` on thin evidence.
6. The current locked three-source corpus yields `no_strong_candidate`.
7. A stronger adjacent known-overlap candidate can still yield `ranked`.
8. M27.5 does not alter M27 coverage artifact semantics.

## Post-M27.5 Branch Rule

M27.5 should end with one of exactly two honest next steps:

- if the tightened engine still yields `no_strong_candidate`, the next milestone
  should be a small corpus-expansion pack or a deliberate human policy choice,
  not automatic M28
- if the tightened engine yields a genuinely promotion-ready candidate, the next
  milestone should be that family promotion

M28 begins only after the repo either:

- promotes the next family from a trustworthy recommendation, or
- explicitly decides that architectural portability work now matters more than
  further Rust-family expansion

That decision must be explicit. M27.5 exists to stop accidental optimism.
