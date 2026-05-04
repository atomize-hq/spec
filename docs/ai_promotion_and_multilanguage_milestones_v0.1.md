# spec — AI Promotion & Multi-Language Milestones
**Version:** v0.1
**Status:** Active roadmap
**Date:** 2026-04-30

> This draft now reflects two things that changed the ladder materially:
> M26 is no longer a proposal, it landed, and M27 narrowed from a vague
> "ranking engine" milestone into a repo-truth coverage-accounting milestone
> with a smaller, more honest first corpus.

## Purpose

This document gives the current landing description for five late-sequence
milestones:

- `M26` — landed
- `M27` — landed
- `M27.5` — next
- `M31` — after recommendation-quality hardening
- `M32` — landed as one bounded second-language pilot for
  `function.arithmetic_leaf.monotone_up.v1`

The shared premise remains the same:

> The promotion workflow is not meant to be human-operated end to end.
> AI should do the work inside hard validation, build, test, and certification
> gates. Humans approve critical boundaries when needed, but the repo should not
> depend on hidden expert ceremony to keep expanding family coverage.

That operator model is consistent with
[`north_star_v0.2.md`](./north_star_v0.2.md), especially the slower, safer,
verify-as-it-builds AI loop.

## Positioning

These milestones assume the repo has already proven:

- the core `validate -> build/generate -> test -> evidence` loop
- packetized semantic family promotion for real Rust family shapes
- smoke / prove / certify as the hard proof gates for a promoted family
- real runtime semantic review for a narrow but honest Rust function wedge

They do **not** assume broad multi-language support already exists.

The repo already landed the narrower proof that had to come first:

- M21 through M24 proved the intent-drift thesis in one narrow Rust wedge
- semantic review can distinguish aligned truth, drift, under-specification, and
  unsupported near-miss shapes across real promoted families
- the runtime supported-family set is now real rather than hypothetical

That changed the next blocker.

The repo is no longer blocked on "can semantic review say anything meaningful at
all?" It now has that proof.

The blocker became operational and then strategic:

- first, how to make family promotion operable by AI under hard gates
- then, how to choose the next family from repo truth instead of gut feel
- then, how to factor the resulting system so second-language work does not
  poison the shared core with Rust-specific escape hatches

That is the ladder below.

## Milestone Sketch

### M26 — Approval-Gated AI Family Promotion Loop
**Status:** Landed

**What landed**

- The repo now has a real approval-gated AI promotion loop rather than a
  hand-operated one.
- `cargo xtask family inventory --format json` exists as a pure projection of
  repo truth.
- promotion artifacts are machine-readable and path-validated through:
  - `cargo xtask family validate-artifact <path>`
- the first live operator-proof family landed as:
  - `function.wrapper.pipeline.v1`
- the hard proof kernel remained in `xtask`:
  - `cargo xtask family smoke`
  - `cargo xtask family prove`
  - `cargo xtask family certify`

**What this proved**

- The repo crossed the important operator boundary:
  family promotion is AI-operable under hard gates, not dependent on a maintainer
  carrying the workflow in their head.
- The recommendation / execution / blocker contract is durable enough that the
  AI loop can stop honestly instead of bluffing through failures.
- The promotion kernel stayed where it belongs, in deterministic repo code, not
  in chat-only policy.

**What landed differently than the earlier draft shape**

- M26 landed narrower and better.
- `inventory` stayed a pure stdout projection and did **not** absorb ranking or
  approval policy.
- artifact validation became a first-class repo command, which is stronger than
  the earlier "artifact shape exists" story.
- the first live proof target resolved concretely to
  `function.wrapper.pipeline.v1`, not just "some supported-but-unpromoted family."

**What this does not prove**

- That next-family selection is already optimal
- That the system is language-portable
- That non-function family promotion is the next right step

### M27 — Coverage Accounting + Next-Family Recommendation Engine
**Status:** Landed

**Current intent**

M27 is no longer a loose "ranking engine" milestone.

It is now a narrower, more honest repo-truth milestone:

- account for how the current checked-in corpus routes today
- separate covered function demand from unsupported function demand
- show supported non-function semantic surfaces without pretending they are
  promotion targets
- cluster unsupported function shapes into deterministic next-family candidates
- rank those candidates from explicit evidence

**What it needs to land on**

- The repo can report which authored unit shapes are:
  - already routed to promoted function families
  - routed to supported-but-unpromoted function families
  - routed to `unsupported.function.v1`
  - routed to supported non-function semantic surfaces that still matter to
    read-side truth
- The repo has a checked-in corpus manifest for the first M27 lane.
- The repo emits durable M27 analysis artifacts:
  - coverage snapshot
  - recommendation analysis
- Recommendation output is evidence-backed, not vibes-backed:
  - real-example frequency
  - promotion-relevant regression frequency
  - unsupported reason-code concentration
  - overlap with existing promoted family structure
  - estimated difficulty
  - explicit low-confidence / no-strong-candidate outcomes

**What changed from the earlier M27 wording**

- M27 got smaller and more concrete.
- The current runtime inventory already has no supported-but-unpromoted runtime
  function family left, so the real M27 question is no longer
  "which known supported family is still unpromoted?"
- The milestone now starts from the actual repo truth:
  four promoted runtime-supported function families and a lot of remaining
  unsupported function demand.
- The first corpus is intentionally small and checked-in:
  - `examples/ecommerce/units`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
  - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`
- Packet fixtures are explicitly excluded from recommendation leverage.
- M27 is allowed to say:
  - `ranked`
  - `no_strong_candidate`
  - `insufficient_real_corpus`

That last point matters. The milestone is supposed to expose confidence gaps,
not hide them.

**What this proves**

- Family expansion can now be driven by measured coverage pressure instead of
  maintainer intuition.
- The AI operator gets a principled, repo-owned input for "what should be
  promoted next?"
- The repo now has a real read on whether it needs another family promotion or a
  corpus-expansion milestone first.

**What this does not prove**

- That the current corpus is large enough to settle the next family forever
- That the same recommendation engine works unchanged across languages
- That the shared family kernel is already clean enough for language #2

### M27.5 — Recommendation Quality Hardening
**Status:** Next

**Why this milestone exists**

M27 did what it was supposed to do mechanically.

The problem is what the first live output taught us:

- a cluster can be discoverable
- a cluster can be weak
- and the current ranking rules can still let it graduate to `ranked`

That is not good enough for roadmap steering.

The repo now needs a narrow follow-up milestone that makes `ranked` mean
"promotion-worthy next family" rather than "the best thing the engine could find
in a small corpus."

**What it needs to land on**

- recommendation policy is stricter than raw cluster discovery
- weak candidates remain visible, but are held back explicitly
- unknown-overlap / hard / thin-evidence candidates no longer cause a top-level
  `ranked` result
- the current locked corpus can honestly yield `no_strong_candidate`
- recommendation artifacts explain why a candidate is being held

**What this proves**

- the engine is not just deterministic, it is selective enough to be trusted
  for next-step decisions
- the repo can distinguish "interesting pressure" from "promote this next"
- the next milestone will be chosen from a stronger recommendation surface

**What this does not prove**

- that corpus expansion is unnecessary
- that the next family is definitely known already
- that shared-core extraction should start immediately afterward

### M31 — Shared-Core Extraction + Escape-Hatch Containment
**Status:** After M27.5

**Why this is the next milestone now**

The next blocker is still architectural, but the code has narrowed it more
precisely than this roadmap originally did.

If M27 and M27.5 land honestly, the repo should know both:

- which unsupported function pressure is real
- whether the recommendation surface is strong enough to trust

That still does **not** justify treating second-language execution as already in
scope.

M31 is the seam-only milestone that has to land first:

- extract the shared core so the seam portability boundary is explicit
- contain Rust-specific lowering and escape-hatch detail instead of letting it
  blur into shared authored shape
- keep the read-side truth honest about when backend-specific detail contaminates
  portability claims

**What it needs to land on**

- The seam portability contract is explicit enough that shared-core extraction is
  a real code boundary, not a slogan.
- Illegal shared-surface authored shapes stay hard validation errors.
- Allowed backend-specific seam detail remains valid authored input, but it is
  not automatically treated as portability-safe.
- Escape-hatch containment is defined before second-language work tries to reuse
  the same proof surfaces.
- The canonical example and corpus inputs are treated as compatibility surfaces,
  not demo garnish, when the shared-core boundary changes.

**What this proves**

- The repo is no longer pretending that "Rust-first" automatically means
  "language-portable."
- The team has identified which seam surfaces are shared truth and which
  backend-specific details must stay contained.
- Second-language work would start from an honest shared-core boundary rather
  than cargo-cult portability language.

**What this does not prove**

- That a second language already works
- That function portability is solved
- That M32 has already been earned

### M32 — One Bounded Second-Language Promotion Path
**Status:** Landed, bounded

After M31 made the seam boundary explicit, M32 stopped being a generic
portability aspiration and became one concrete executable-truth pilot.

M32 proves one bounded second-language promotion path for
`function.arithmetic_leaf.monotone_up.v1` and nothing broader.

**What landed**

- `function.arithmetic_leaf.monotone_up.v1` now carries the bounded
  second-language pilot alongside its existing Rust packet truth.
- TypeScript is explicit for that one monotone-up packet so the repo can test
  the shared promotion flow against a real second-language path.
- packet lifecycle, artifact contracts, approval surfaces, and proof-gate
  semantics stay shared rather than forking into a separate second-language
  workflow.
- `function.wrapper.pipeline.v1` stays in the story as regression pressure only;
  it is not a second M32 certify target.

**What this proves**

- One already-understood family can complete a second-language promotion path
  without re-inventing the promotion workflow from scratch.
- The proof surfaces are now concrete about which parts of the flow stayed
  shared and which assumptions still remain target-specific.
- The M31 containment work was strong enough to support one bounded pilot on a
  real promoted family.

**What this does not prove**

- General second-language family coverage
- Additional M32 certify targets beyond
  `function.arithmetic_leaf.monotone_up.v1`
- Finished parity across arbitrary language features
- That the portability kernel is complete

## Suggested Ordering Logic

1. `M26` first, because the operator model had to become real.
2. `M27` second, because next-family choice should become evidence-driven.
3. `M27.5` third, because evidence-driven is not the same thing as
   recommendation-quality good enough for roadmap steering.
4. `M31` next, because second-language work without shared-core extraction and
   escape-hatch containment would be fake confidence.
5. `M32` after M27.5 and M31, as the bounded
   `function.arithmetic_leaf.monotone_up.v1` second-language pilot and not a
   broader portability claim.

## One-Line Summary

`M26` made family promotion AI-operable, `M27` made next-family choice
evidence-driven, `M27.5` hardens recommendation quality, `M31` isolates
the shared core from Rust-specific escape hatches, and `M32` proves one bounded
second-language promotion path for `function.arithmetic_leaf.monotone_up.v1`
while keeping broader portability claims out of scope.
