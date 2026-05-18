# Rust V1 Contract Stack

This document is the repo-facing index for the recent Rust V1 planning and
contract artifacts.

It exists for one reason:

- make the current contract stack and implementation ladder easy to find

It is not a replacement for the underlying artifacts.

## Current Stack

### M65: Planning Anchor

Artifact:
- [M65 Rust V1 Contract Decomposition and Rough Roadmap](</Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md>)

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
- [M66 Narrow-Core Provisional Rust Support Contract](</Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md>)

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
- [M67 Benchmark / Truth-Surface Companion Spec](</Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md>)

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
- [M68 Mechanics-Landing Implementation Contract](</Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md>)

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

The project should now move through implementation milestones rather than more
strategy artifacts.

### I1: Benchmark Registry + Shared Projection Core

Goal:
- land the first benchmark mechanics inside the codebase

Scope:
- load and validate `benchmarks/labels.json`
- add benchmark enums and projection structs in `spec-core`
- implement full vs partial benchmark path-scope behavior
- add additive `benchmarks[]` projection to `spec status --format json`
- add additive `benchmarks[]` projection to `spec export`
- project `BENCH-SERVICE` as the explicit reserved gate state

Primary outcome:
- the repo can project benchmark truth without minting it

### I2: Snapshot + Readability Surfaces

Goal:
- land the first stable benchmark snapshot and readability observation surfaces

Scope:
- implement `spec benchmark snapshot <benchmark-id>`
- write `benchmarks/snapshots/<BENCHMARK_ID>.snapshot.json`
- load and project `benchmarks/reviews/<BENCHMARK_ID>.readability.review.json`
- enforce the canonical `projection_digest` contract

Primary outcome:
- benchmark-scoped readability and digest freshness become real surfaces

### I3: Anti-Laundering and Gate Semantics

Goal:
- make the benchmark projection honest under failure, fallback, and missing
  labels

Scope:
- enforce unlabeled-path invalidation
- enforce companion negative-proof visibility
- enforce fallback-backed visibility without native credit
- enforce reserved `BENCH-SERVICE` gate semantics
- keep `status` and `export` from laundering non-native cases into green
  benchmark credit

Primary outcome:
- benchmark credit becomes hard to fake

### I4: Schema v4 Fixtures and Contract Tests

Goal:
- lock the new machine contract with tests

Scope:
- update JSON fixture tests for `schema_version: 4`
- add full-scope benchmark projection tests
- add partial-scope benchmark projection tests
- add reserved benchmark tests
- add readability freshness tests
- add anti-laundering regression tests

Primary outcome:
- the benchmark/truth-surface contract is test-backed

### I5: BENCH-ECOM Activation Closeout

Goal:
- make the live positive benchmark meaningful on the current example

Scope:
- author the real initial `BENCH-ECOM` labels
- make the projected benchmark state honest on current proof artifacts
- ensure readability review can be attached to the canonical generated tree

Primary outcome:
- one active positive benchmark is live and coherent

### I6: BENCH-SERVICE Landing

Goal:
- land the real service-shaped benchmark required for final V1 closure

Scope:
- add the service-shaped benchmark workload
- label it
- wire it into the existing benchmark/snapshot/readability surfaces
- keep its reserved/open state honest until the workload is real

Primary outcome:
- the required final positive benchmark exists

### I7: Scope-Decision Implementation

Goal:
- implement the product-scope decisions still open after the narrow core

Scope:
- bounded generics if admitted into V1
- async / IO if admitted into V1
- or explicit deferral behavior if they remain outside V1

Primary outcome:
- `D2` and `D3` move from planning decisions into real support or real deferral

### I8: Final V1 Proof Closure

Goal:
- close the final Rust V1 proof claim honestly

Scope:
- run the final benchmark truth stack
- verify positive benchmarks and companion fixtures behave correctly
- confirm the support contract, benchmark companion spec, and mechanics layer
  agree

Primary outcome:
- one plain-English Rust V1 claim can be published honestly

## Repo Note

These artifacts currently live under the local project planning area in
`~/.gstack/projects/atomize-hq-spec/`.

This repo doc is the stable index that points to them and explains what each
one owns. If a later milestone supersedes one of the linked artifacts, update
this index rather than scattering one-off references across the repo.
