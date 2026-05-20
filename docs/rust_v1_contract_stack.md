# Rust V1 Contract Stack

This document is the repo-facing index for the Rust V1 planning artifacts and the
current implementation ladder.

It exists for one reason:

- make the current contract stack and milestone ownership easy to find

It is not a replacement for the underlying artifacts.

## Current Stack

### M65: Planning Anchor

Artifact:
- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`

Owns:
- paper trail from the failed giant contract draft
- artifact split
- rough milestone ladder from `M65` to V1
- forced decisions:
  - `D1` `BENCH-SERVICE`
  - `D2` bounded generics
  - `D3` async / IO
  - `D4` `BENCH-CROSSLIB` role

### M66: Narrow-Core Provisional Rust Support Contract

Artifact:
- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`

Owns:
- narrow-core V1 user claim
- supported / deferred / explicitly-out rows
- supported / deferred / explicitly-out interactions
- fallback policy
- early-failure boundary
- provisional done-state gate for the narrow core

Does not own:
- benchmark schemas
- truth-surface mechanics
- rollout sequencing

### M67: Benchmark / Truth-Surface Companion Spec

Artifact:
- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`

Owns:
- benchmark roster
- benchmark roles
- truth-writer vs read/project boundary
- fallback visibility rules
- readability as an observation surface
- `BENCH-SERVICE` as a required later gate

Resolved here:
- `D4`: `BENCH-CROSSLIB` is a companion negative-proof fixture, not positive
  workload coverage

### M68: Mechanics-Landing Implementation Contract

Artifact:
- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`

Owns:
- concrete benchmark artifact set
- exact writer vs reader boundary
- benchmark-level enums
- canonical `projection_digest`
- exact path-scoped benchmark behavior for `spec status` and `spec export`
- exact readability scope for shared generated files
- anti-laundering rules for benchmark credit

This is the last doc-first milestone before implementation.

## Contract Boundary Summary

Use this routing rule before editing or extending any of the artifacts above:

- if the statement is about what Rust V1 claims, it belongs in `M66`
- if the statement is about how that claim is observed, labeled, reviewed, or
  projected, it belongs in `M67`
- if the statement is about concrete benchmark/read-surface mechanics, it
  belongs in `M68`
- if the statement is about roadmap ordering or milestone ownership, it belongs
  in `M65`

## Implementation Ladder

The current ladder is:

- `M65-M68 -> I3 -> I3.5 -> I4`

### I3: Benchmark Mechanics Baseline

Goal:
- land the benchmark truth-surface mechanics baseline in the repo

Scope:
- benchmark projection rules
- anti-laundering and gate semantics
- benchmark-root status/export proof wall behavior
- supporting snapshots and readability surfaces

Primary outcome:
- the benchmark-root command wall became the trustworthy proof surface

### I3.5: Post-I3 Authority Alignment and Repo-Root Contract Freeze

Goal:
- align the repo to one authoritative command wall without widening scope

Scope:
- restore and normalize this repo-facing contract-stack index
- freeze repo-root `status . --format json` as supported `inventory_only`
- freeze repo-root `export .` as stable unsupported scope with
  `SPEC_UNSUPPORTED_SCOPE`
- preserve benchmark-root `status examples/ecommerce/units --format json` and
  `export examples/ecommerce/units` as the proof wall
- align code, fixtures, docs, help text, README surfaces, changelog, and the
  orchestration runbook to the same contract

Primary outcome:
- I3.5 becomes the repo-root contract freeze milestone between I3 mechanics and
  I4 contract-test hardening

### I4: Schema v4 Fixtures and Contract Tests

Goal:
- lock the frozen machine contract with fixtures and regression tests

Scope:
- fixture coverage for benchmark-root proof commands
- fixture coverage for namespace and single-file partial diagnostics
- fixture coverage for repo-root `inventory_only` status
- fixture coverage for repo-root unsupported export with
  `SPEC_UNSUPPORTED_SCOPE`
- regression coverage that keeps docs/help and CLI behavior aligned

Primary outcome:
- the I3.5 command wall is test-backed and hard to regress

## Repo Note

The planning artifacts above live in the local project planning area under
`~/.gstack/projects/atomize-hq-spec/`.

This repo document is the stable index that points to them and explains what
each one owns. When the ladder changes, update this index rather than teaching
one-off milestone lore in scattered docs.
