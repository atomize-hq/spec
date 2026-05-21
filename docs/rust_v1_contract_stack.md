# Rust V1 Contract Stack

This document is the repo-facing index for the Rust V1 contract stack and the
frozen I3.5 command wall.

It exists to answer two questions quickly:

- which artifact owns which Rust V1 claim
- which command surface is proof-authoritative versus diagnostic only

## Current Authority

I3.5 freezes the public contract against these in-repo authority artifacts:

- `.runs/i3_5_authority_alignment/authority-plan.snapshot.md`
- `.runs/i3_5_authority_alignment/phase2-freeze.json`

Those files are the maintained repo authority for the current alignment work.
This index deliberately avoids stale local-user planning paths.

## Historical Stack Ownership

The historical M65-M68 design inputs are carried forward through the I3.5
authority snapshot above. Use this ownership map when deciding where a claim
belongs.

### M65: Planning Anchor

Owns:

- the milestone ladder into Rust V1
- scope framing after the failed giant contract draft
- forced decisions such as `BENCH-SERVICE`, bounded generics, async/IO
  deferral, and the `BENCH-CROSSLIB` role

### M66: Narrow-Core Rust Support Contract

Owns:

- what Rust V1 claims to support
- supported, deferred, and explicitly out interactions
- fallback policy and early-failure boundaries

Does not own:

- benchmark schemas
- truth-surface mechanics
- rollout sequencing

### M67: Benchmark and Truth-Surface Companion

Owns:

- benchmark roster and benchmark roles
- writer-versus-reader boundaries
- readability as an observation surface
- `BENCH-SERVICE` as the required service benchmark that stays within the frozen Rust V1 support boundary

### M68: Mechanics-Landing Implementation Contract

Owns:

- concrete benchmark artifact shapes
- exact path-scoped behavior for `spec status` and `spec export`
- benchmark-level enums, digests, and anti-laundering rules
- readability-selection rules for generated files

## Contract Boundary Summary

Use this routing rule before editing or extending any artifact in the stack:

- if the statement is about what Rust V1 claims, it belongs in `M66`
- if the statement is about how that claim is observed or labeled, it belongs
  in `M67`
- if the statement is about concrete benchmark and read-surface mechanics, it
  belongs in `M68`
- if the statement is about milestone ordering or ownership, it belongs in
  `M65`
- if the statement is about which command wall is authoritative in this repo
  today, it belongs in I3.5

## Implementation Ladder

The current ladder is:

- `M65-M68 -> I3 -> I3.5 -> I4 -> I6`

### I3: Benchmark Mechanics Baseline

Goal:

- land benchmark-aware truth-surface mechanics in the repo

Primary outcome:

- benchmark-root status and export became the trustworthy proof surfaces

### I3.5: Authority Alignment and Repo-Root Contract Freeze

Goal:

- align code, docs, and runbooks to one authoritative command wall without
  widening product scope

Frozen outcomes:

- repo-root `cargo run -p spec-cli -- status . --format json` is supported as
  `inventory_only`
- repo-root `cargo run -p spec-cli -- export .` is unsupported for this
  workspace shape and must fail with `SPEC_UNSUPPORTED_SCOPE`
- benchmark-root commands remain the proof wall:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
```

- namespace and single-file `status` remain partial-scope diagnostics only
- README, example README, changelog, CLI help, and orchestration docs all teach
  the same command wall

Primary outcome:

- I3.5 becomes the milestone that freezes repo-root semantics between I3
  mechanics and I4 regression hardening

### I4: Fixture and Contract-Test Hardening

Goal:

- lock the frozen I3.5 machine contract behind regression fixtures and tests

Scope:

- fixture coverage for benchmark-root proof commands
- fixture coverage for namespace and single-file partial diagnostics
- fixture coverage for repo-root `inventory_only` status
- fixture coverage for repo-root unsupported export with
  `SPEC_UNSUPPORTED_SCOPE`

Primary outcome:

- the I3.5 command wall becomes difficult to regress accidentally

### I6: Rust V1 Service Benchmark Activation

Goal:

- activate `BENCH-SERVICE` as a real single-library proof wall without widening M66 support

Frozen outcomes:

- `examples/service/units` is the benchmark root for the service activation slice
- `BENCH-SERVICE` stays frozen to the six authored service units and three required molecule proofs
- service-root `status` and `export` become proof-authoritative alongside the existing ecommerce wall
- `BENCH-SERVICE` closeout includes a current readability review and a stable committed snapshot

Primary outcome:

- the service benchmark is now a shipped, benchmark-root Rust V1 proof surface instead of a reserved placeholder

## Repo Note

If you need the current execution truth, start with the authority snapshot and
freeze record under `.runs/i3_5_authority_alignment/`, then use this document
as the index that explains what each milestone owns.
