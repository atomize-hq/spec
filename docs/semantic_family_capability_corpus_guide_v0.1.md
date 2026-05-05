# Semantic Family Capability, Corpus, and Promotion Guide
**Version:** v0.1  
**Status:** Draft  
**Date:** 2026-05-01

## Purpose

This document explains a distinction that is easy to blur in the current M27.x
family-analysis work:

- what `spec` core is
- what semantic review is
- what a promoted semantic family is
- what the corpus is doing
- what passports are doing
- what the recommendation engine is actually allowed to claim

The goal is a simple mental model, not a milestone plan.

---

## Short Version

There are **three different axes** at play:

1. **Capability**
   What shapes the semantic reviewer currently understands.

2. **Corpus pressure**
   What unsupported or supported shapes the checked-in corpus actually contains.

3. **Promotion proof**
   What family boundaries have been proven and promoted strongly enough to ship
   as supported review behavior.

These are related, but they are not the same thing.

---

## The Cleanest Mental Model

Think of the system like this:

- **`spec` core** is the semantic source system
- **semantic review** is one classifier inside that system
- **promoted semantic families** are the currently shipped buckets that semantic
  review knows how to classify honestly
- **the corpus** is a checked-in observation set used to study where the current
  supported set fits and where unsupported demand is appearing
- **passports** are per-unit proof records
- **family coverage / recommendation artifacts** are read-side analysis outputs
  over the corpus

That means:

- the graph system is not the same thing as the corpus
- passports are not the same thing as recommendation
- a family is not supported just because the corpus contains many examples of it
- more examples can increase pressure without increasing understanding

---

## The Main Objects

### 1. `spec` core

This is the source-of-truth semantic system.

It owns:

- `*.unit.spec`
- `*.test.spec`
- validation
- normalization
- graph resolution
- generation
- compile/test execution
- evidence collection
- passport emission
- export/status projections

This is the write path that changes semantic repo truth.

### 2. Semantic review

Semantic review is one runtime interpretation layer for authored units.

For `kind:function`, it is intentionally bounded to a **small shipped family
vocabulary**, not arbitrary function understanding.

That means semantic review can currently do things like:

- recognize some arithmetic leaf shapes
- recognize some wrapper pipeline shapes
- reject or mark unsupported shapes outside that narrow subset

So semantic review is not “general code understanding.” It is a narrow,
truth-constrained reviewer.

### 3. Promoted semantic families

A promoted semantic family is a supported interpretation packet that the system
is willing to ship as real review behavior.

A family is “promoted” when it is no longer just an idea or a corpus pattern.
It has:

- a packet
- a boundary
- proof buckets
- routing truth
- certification history

This is what turns “we think this shape matters” into “the system now supports
this shape honestly.”

### 4. Corpus

The corpus is the checked-in set of repo-owned sources used by the M27.x family
analysis lane.

Its job is to answer questions like:

- which current units land in already-supported families?
- which current units fall outside support?
- what unsupported shapes appear repeatedly?
- which unsupported demand looks adjacent to current families?

The corpus is an **observation surface**, not a promotion switch.

### 5. Passports

Passports are derived per-unit proof records.

They answer questions like:

- did this unit validate?
- did it build?
- did its local tests pass?
- what proof is current vs stale?

Passports are about the truth of a unit’s current proof state.

They are not the same artifact family as corpus recommendation.

### 6. Coverage and recommendation artifacts

These are read-side analysis outputs built over the corpus.

- `coverage.latest.json` says what current supported coverage and unsupported
  pressure the corpus shows
- `recommendation.latest.json` says what next family, if any, looks worthy of
  promotion under the current rules

Under the M33 decision surface, that same
`recommendation.latest.json` artifact also carries a top-level decision verdict:

- `recommended`
- `blocked_for_now`
- `not_recommended`

These artifacts interpret the corpus. They do not define the semantic graph.

---

## The Three Axes

## Axis 1 — Capability

**Question:** what shapes does semantic review currently understand?

This is the supported-family axis.

Examples of capability dimensions:

- arithmetic leaf vs wrapper pipeline
- dep topology
- straight-line only vs unsupported control flow
- argument-threading restrictions
- required supported deps or not
- monotone-up vs monotone-down style semantics

If capability expands, the system can honestly classify more authored shapes as
supported.

This is mostly a **reviewer + family-promotion** question.

It is **not** mainly a corpus-size question.

## Axis 2 — Corpus pressure

**Question:** what shapes do we actually see in checked-in repo examples and
regressions?

This is the evidence-pressure axis.

Examples:

- how many real examples hit a cluster?
- how many promotion-relevant regressions hit it?
- is the cluster only boundary noise?
- does the unsupported shape keep showing up?

If corpus pressure grows, the system learns that a shape is more or less common
or important.

But that still does **not** mean the system understands it better.

It may only mean:

- “we see this often”
- not
- “we now know how to support it”

## Axis 3 — Promotion proof

**Question:** have we proven a family boundary well enough to ship it as
supported?

This is the proof-and-certification axis.

It includes things like:

- family packet definition
- required proof buckets
- smoke/prove/certify gates
- artifact validation
- routing and precedence truth

This is what upgrades a candidate family from “interesting pressure” to
“supported semantic review behavior.”

---

## What “Supported” Actually Means

A family is supported when the system can honestly say:

1. semantic review knows how to classify this shape
2. the family boundary has been encoded explicitly
3. the proof and certification workflow has passed
4. the runtime review path now treats that family as shipped behavior

So:

- **support** comes from capability + promotion proof
- **not** from raw corpus count alone

That is the key correction.

---

## What the Corpus Can and Cannot Do

### The corpus can do this

- show that a current supported family is appearing in real code
- show that unsupported shapes are appearing repeatedly
- show that a candidate unsupported shape is adjacent to an existing family
- make the case that a new family is worth promoting next

### The corpus cannot do this by itself

- make the reviewer understand a new shape
- promote a family automatically
- rewrite family boundaries
- convert an unknown unsupported shape into a supported family without reviewer
  and promotion work

So the corpus is a **pressure signal**, not a **support authorizer**.

---

## The Most Common Confusion

The usual mistaken model is:

> If we have enough examples in the corpus, then the family becomes supported.

That is wrong.

The correct model is:

> If we have enough examples in the corpus, then we may have enough evidence to
> justify promoting a family into supported review behavior.

That extra step matters.

It includes implementation, proof, and boundary definition.

---

## Why `money/round` Is the Current Example

This is the cleanest live example of the distinction.

Current repo truth says the `money/round` cluster is:

- visible
- rankable
- real enough to matter
- still `overlap_family = "unknown"`
- `decision_status = "not_recommended"`

That means:

- corpus pressure is real
- capability is still missing
- the current blocker is `helper_surface_not_promotable`, not lack of
  visibility

So if you add more examples of that same unsupported shape, you may increase:

- `real_example_hits`
- confidence that the shape is common

But you do **not** necessarily increase:

- reviewer understanding
- family overlap clarity

That is why more corpus can produce **louder uncertainty** instead of
**better understanding**.

In M33 terms, this is exactly the difference between `blocked_for_now` and
`not_recommended`: `money/round` is not the next family move even though the
pressure is still visible.

---

## Why Arithmetic Was Different

The arithmetic-shape candidate behaves differently because the system already
knows which family direction it is near.

That means:

- capability adjacency already exists
- overlap is already known
- one more real example can change promotion readiness materially

So corpus work can help more there, because the missing piece is mostly evidence,
not interpretation.

---

## A Simple Decision Table

| Situation | What is missing? | More corpus likely helps? |
|---|---|---|
| known family overlap, thin evidence | evidence | yes |
| known family overlap, strong evidence, no proof packet yet | promotion proof | not by itself |
| unknown overlap, repeated same-shape examples | capability / interpretation | usually not much |
| already supported family appears in many examples | confirmation / usage signal | yes, but it does not change support |

---

## The One-Sentence Summary

**Corpus tells us what demand exists. Semantic review capability tells us what
the system currently understands. Promotion proof is what turns one of those
demand patterns into newly supported behavior.**

## Practical Bottleneck Rule

**Corpus is the main upstream fuel for coverage expansion, but not the only
downstream lever. Once enough pressure already exists, the bottleneck can shift
from more corpus to better interpretation or promotion work.**

---

## Practical Questions To Ask

When looking at a family-analysis result, ask these in order:

1. Is this a capability problem or just an evidence-thin problem?
2. Does the system already know what family direction this cluster is near?
3. Would more examples change only leverage counts, or would they change a real
   decision boundary?
4. If the shape is still “unknown,” are we actually missing corpus, or are we
   missing reviewer understanding?

Those four questions usually separate “do another corpus run” from “stop and do
promotion/reviewer work instead.”

---

## Relationship To Other Docs

- `README.md` explains the shipped user-facing workflow
- `docs/north_star_v0.2.md` explains the long-term product vision
- `docs/high_level_technical_architecture_v0.2.md` explains the broader system
  layering
- `semantic-families/README.md` explains the family packet and M27 analysis lane
- `PLAN.md` captures the current single-run corpus-expansion contract

For visual versions of the same distinctions, see the repo-root
`diagrams.md`.
