# spec — AI Promotion & Multi-Language Milestones
**Version:** v0.1  
**Status:** Draft  
**Date:** 2026-04-29

> This draft extends the March north-star and architecture docs with a narrower
> post-M24 milestone ladder. It is meant to capture the currently intended shape,
> not to overwrite the earlier milestone model.

## Purpose

This document gives a quick landing description for three later milestones:

- `M26`
- `M27`
- `M29`

The shared premise is now explicit:

> The promotion workflow is not meant to be human-operated end to end.
> AI should do the work inside hard validation, build, test, and certification
> gates. Humans approve the next target family and the final promoted result.

That operator model is consistent with the north star in
[`north_star_v0.2.md`](./north_star_v0.2.md), especially the "slower, safer,
verify-as-it-builds" AI loop.

## Positioning

These milestones assume the repo has already proven:

- the core `validate -> build/generate -> test -> evidence` loop
- packetized semantic family promotion for real Rust family shapes
- smoke / prove / certify as the hard proof gates for a promoted family

These milestones do **not** assume broad multi-language support already exists.
They are the bridge from "proved by hand in a narrow Rust wedge" to
"AI-operated, language-portable promotion machinery."

That bridge matters for a specific reason.

The repo already landed the narrower proof that had to come first:

- the operator model is the one described in
  [`north_star_v0.2.md`](./north_star_v0.2.md), especially the slower, safer,
  verify-as-it-builds AI loop
- before expanding to other languages, the repo wanted to prove the
  intent-drift thesis in one real target language
- M21 through M24 now give that proof in a narrow Rust wedge: semantic review
  can distinguish aligned truth, drift, under-specification, and unsupported
  near-miss shapes across real promoted families

That means the central problem has changed.

The next blocker is no longer "can semantic review detect meaningful semantic
misalignment at all?" The repo now has that proof.

The next blocker is scale:

- how to cover most of one language without promotion work staying manual
- how to move from broad Rust support to additional languages without rebuilding
  the workflow from scratch

That is why these milestones focus on promotion machinery.

The promotion machinery is not an end in itself. It is the mechanism that should
let the repo move from:

- one narrow Rust proof of the intent-drift thesis

to:

- broad semantic review coverage across most of a language
- then portable semantic review coverage across multiple languages

## Milestone Sketch

### M26 — Approval-Gated AI Family Promotion Loop

**What it needs to land on**

- AI can recommend a candidate next family from repo truth rather than from ad hoc operator choice.
- A human approves the target family before execution starts.
- AI then performs the full promotion loop:
  - scaffold or curate the packet
  - add or adjust family-owned tests
  - run `cargo xtask family smoke`
  - run `cargo xtask family prove`
  - run `cargo xtask family certify`
  - iterate until green or emit a precise blocker
- A human approves the final promoted result after reading a generated promotion report.

**What this proves**

- The repo has moved from "manual promotion workflow exists" to
  "AI can operate the workflow under hard gates."
- The true operator contract is approval-gated AI, not human ceremony.

**What this does not prove**

- That the next-family recommendation logic is globally optimal
- That the workflow is already language-agnostic
- That multi-language porting is cheap yet

### M27 — Coverage Accounting + Next-Family Recommendation Engine

**What it needs to land on**

- The repo can report which unit shapes are:
  - already routed to promoted families
  - routed to supported but unpromoted shapes
  - still falling into unsupported buckets
- The repo can rank the next candidate families by leverage, not gut feel.
- Recommendation output includes evidence:
  - frequency in real corpora
  - unsupported reason-code concentration
  - overlap with existing promoted family structure
  - estimated promotion difficulty

**What this proves**

- Family expansion is now driven by measured coverage pressure.
- The AI operator has a principled input for "what should be promoted next?"

**What this does not prove**

- That the shared family kernel is language-agnostic enough for language #2
- That the same recommendation engine works unchanged across languages

### M29 — Second-Language Promotion Pilot

**What it needs to land on**

- The family promotion core has been factored so a second language can plug into it
  without re-inventing packet lifecycle, prove/certify gates, or promotion reports.
- At least a small set of already-understood family shapes can be promoted or
  re-proved in a second target language.
- The resulting proof surfaces show which semantics are genuinely shared and which
  assumptions are still Rust-specific.

**What this proves**

- The system is becoming a language-portable semantic family platform rather than
  a Rust-only promotion rig.
- The architecture can absorb more than one target language without collapsing
  the shared core into target-specific hacks.

**What this does not prove**

- Broad second-language coverage
- Generalized multi-language expansion
- Full parity across arbitrary language features

## Why `M28` Is Not Frozen Here

This document intentionally does not lock `M28`.

The gap between `M27` and `M29` is likely where the repo will need one focused
kernel-extraction or escape-hatch-containment milestone, but the exact seam should
be chosen after `M26` and `M27` expose the real pressure points.

Prematurely naming `M28` would fake certainty.

## Suggested Ordering Logic

1. `M26` first, because the operator model must become real.
2. `M27` second, because family selection should become evidence-driven.
3. `M29` after the shared-core seam is clearer, because second-language work is
   only useful once the core is ready to learn from it.

## One-Line Summary

`M26` makes family promotion AI-operated, `M27` makes family selection evidence-driven,
and `M29` tests whether the resulting system is actually portable beyond Rust.
