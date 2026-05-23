# I7: Rust V1 Scope-Decision Closure Plan

Status: **authoritative implementation plan and closeout record**
Iteration: **I7**
Milestone family: **Rust V1 scope closure and contract ratification**
Implementation readiness: **implemented**
Plan scope: **turn the post-I6 ambiguity into checked-in repo truth by resolving bounded-generics and async/IO admission for honest Rust V1, ratifying the final pre-proof contract line, and freezing I8 as the final proof-run milestone**
Base branch: **main**
Working branch: **`codex/i7-v1-scope-closure`**
Validated on branch: **`main`**
Last rewritten: **2026-05-22**

Supersedes:

- the prior `I6: Rust V1 Service Benchmark Activation Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- live repo truth on `main` after I6 landing:
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - `cargo run -p spec-cli -- status . --format json`

Historical context, not authority:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `/home/azureuser/.gstack/projects/atomize-hq-spec/azureuser-main-design-20260522-001238.md`

Primary repo surfaces:

- `PLAN.md`
- `ORCH_PLAN.md`
- `docs/rust_v1_contract_stack.md`
- `README.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `TODOS.md`
- `benchmarks/labels.json`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`

## Executive Summary

I6 shipped the missing service proof wall. That means the remaining Rust V1 risk
is no longer benchmark mechanics. It is scope drift.

I7 exists to convert the last two ambient "maybe V1, maybe later" topics into
explicit repo-backed truth:

- bounded generics
- async/IO

This milestone is complete only when the repo can answer three questions
without inference:

- what is in Rust V1
- what is out of Rust V1
- what I8 still has to prove

The default posture is narrow and honest. I7 does not widen Rust support unless
one bounded slice passes an explicit admission bar and names its proof burden.
Otherwise it defers to `V1.1` in checked-in authority.

## Frozen I7 Outcome

I7 closed with explicit scope deferral rather than a new admitted Rust slice:

- bounded generics defer to `V1.1`
- Rust V1 remains synchronous-only, so async/IO also defer to `V1.1`
- `BENCH-CROSSLIB` remains the active companion-negative wall
- Lane D does not exist because no new bounded slice was admitted
- I8 inherits the existing five-command proof wall unchanged

## Scope Challenge

### Premise correction

The problem is not "finish Rust V1 by adding more Rust."

The problem is:

```text
turn the remaining post-I6 scope pressure into explicit repo-backed truth
so Rust V1 has one honest, bounded claim and I8 is a proof run instead of
another scope-discovery milestone
```

If implementation expands beyond that sentence, it has escaped I7.

### Milestone posture

I7 is a scope-closure milestone, not a feature-expansion milestone.

That means:

- reuse the shipped I3.5 command wall
- reuse the shipped benchmark mechanics
- reuse the shipped benchmark roster unless a change is explicitly justified
- prefer explicit `V1.1` deferral over ambiguous "probably part of V1"

### Operative rule

I7 must not widen support by atmosphere.

If a surface is not already proven by current repo truth and cannot be admitted
through one small, nameable, explainable slice with believable proof burden, it
defers explicitly to `V1.1`.

## Current Validated Truth

Observed on `main` after the `0.15.2` landing:

- `BENCH-ECOM` is an active positive proof wall.
- `BENCH-SERVICE` is an active positive proof wall.
- `BENCH-CROSSLIB` is the active companion-negative wall.
- repo-root `status . --format json` remains supported as `inventory_only`.
- repo-root `export .` remains unsupported for this workspace shape.
- `M66` still treats bounded generics as deferred.
- `M66` still keeps async/IO outside the narrow-core shipped baseline.
- `docs/rust_v1_contract_stack.md` now records `I7 -> I8` as the active wrap
  path.

In plain English:

- the proof walls are real
- the narrow-core baseline is real
- the final Rust V1 claim still needs explicit ratification

## What Already Exists

| Sub-problem | Existing owner | I7 action |
| --- | --- | --- |
| benchmark-root proof walls | `BENCH-ECOM`, `BENCH-SERVICE`, I3.5 command wall | reuse unchanged as the proof baseline |
| companion-negative boundary | `BENCH-CROSSLIB` | preserve unless I7 records an explicit retirement or demotion decision |
| narrow-core Rust support rows | `M66` | reuse as the starting contract, not the final V1 claim |
| benchmark and readability mechanics | `M67`, `M68`, shipped repo logic | preserve unchanged unless a real truth bug is found |
| milestone ladder and ownership | `M65`, `docs/rust_v1_contract_stack.md` | keep `I7 -> I8` explicit and authoritative |
| execution runbook shape | `ORCH_PLAN.md` | align to the frozen I7 decision flow |

## Frozen Decisions

These decisions are locked for I7 and should not be reopened casually.

1. **I7 owns post-I6 scope closure.**
   - I6 is shipped history.
   - I7 is the current milestone that resolves what remains before honest Rust
     V1 can be declared complete.

2. **I7 does not widen support by default.**
   - bounded generics is not admitted because it is adjacent
   - async/IO is not admitted because it is common backend behavior
   - admission requires an explicit packet, explicit boundary, and explicit
     proof burden

3. **Deferred is a successful outcome when it is explicit.**
   - if bounded generics cannot be admitted cleanly, write `V1.1`
   - if async/IO cannot be admitted cleanly, write `V1.1`
   - honest narrow truth beats aspirational ambiguity

4. **I7 preserves the shipped proof wall unless it records a scoped change.**
   - `BENCH-ECOM` stays active positive proof
   - `BENCH-SERVICE` stays active positive proof
   - `BENCH-CROSSLIB` stays the active companion-negative wall unless I7
     retires or demotes it with replacement rationale

5. **I8 is the final proof-run milestone.**
   - I7 ends by freezing what I8 must prove
   - the repo should not imply a checked-in `I9` queue

6. **Authority merges happen only after decision freeze.**
   - draft reasoning can be parallelized
   - checked-in authority updates happen only after the parent freezes the
     generics and async/IO decisions

## Required Outputs

I7 is done only when all of these exist together:

1. a checked-in bounded-generics decision packet
2. a checked-in async/IO decision packet
3. one ratified plain-English Rust V1 claim
4. one explicit `I8` handoff naming:
   - required benchmark walls
   - authoritative proof commands
   - deferred `V1.1` surfaces
5. repo-facing docs that no longer leave post-I6 state to inference

## Architecture And Decision Flow

The execution flow for I7 is intentionally narrow:

```text
current shipped truth on main
        |
        v
  basis freeze packet
        |
        +-------------------+
        |                   |
        v                   v
bounded-generics      async/IO
decision packet       decision packet
        |                   |
        +---------+---------+
                  |
                  v
           parent decision freeze
                  |
                  v
        authority-surface ratification
                  |
                  v
             I8 handoff freeze
                  |
                  v
          final proof-run target
```

The parent is the only merge authority. Parallel lanes can draft reasoning, but
they do not publish repo truth independently.

## Decision Framework

### D2: Bounded Generics

I7 must answer:

- is there a bounded generics slice small enough, valuable enough, and provable
  enough to join honest Rust V1?

Admit bounded generics into V1 only if all of these are true:

- the slice serves the stated solo Rust backend or business-logic user directly
- the slice is small enough to explain in one paragraph
- the authored boundary is specific and nameable
- the proof burden fits on top of the shipped command wall without new benchmark
  mechanics
- the slice does not quietly smuggle in broader trait, lifetime, macro, or
  abstraction-heavy support

Otherwise:

- keep `ROW-GENERIC-BOUNDED` deferred
- state plainly that bounded generics defer to `V1.1`

Required contents of the bounded-generics packet:

- recommendation: admit or defer
- user-value rationale
- authored boundary
- proof burden
- rejected broader expansions
- exact repo surfaces that change if the recommendation is accepted

### D3: Async / IO

I7 must answer:

- does Rust V1 stay synchronous-only, or is there one bounded async/IO edge
  worth admitting?

Admit async/IO into V1 only if all of these are true:

- the slice is small enough to explain in one paragraph
- it does not require framework lifecycle semantics as the new baseline
- it does not collapse the narrow-core claim into "normal backend Rust"
- it can be proven without turning I7 into a mechanics rewrite

Otherwise:

- keep Rust V1 explicitly synchronous-only
- state plainly that async/IO defers to `V1.1`

Required contents of the async/IO packet:

- recommendation: admit or defer
- user-value rationale
- authored boundary
- proof burden
- rejected broader expansions
- exact repo surfaces that change if the recommendation is accepted

### D4: `BENCH-CROSSLIB` Role

The shipped default is:

- `BENCH-CROSSLIB` remains the companion-negative benchmark because it is still
  performing useful boundary work for the current claim

I7 may retire or demote it only if the repo records:

- what negative-proof function it was serving
- what replaces that function
- why the final V1 proof run becomes clearer rather than weaker

## Work Phases

| Phase | Goal | Primary outputs | Exit criteria |
| --- | --- | --- | --- |
| 1. Basis freeze | capture current post-I6 truth without embellishment | basis packet, current benchmark roster, current deferred rows, current command-wall semantics | all later reasoning cites frozen evidence instead of memory |
| 2. Bounded-generics packet | produce exactly one recommendation on bounded generics | bounded-generics decision packet | packet recommends one outcome and names rationale, boundary, proof burden, rejected expansions |
| 3. Async/IO packet | produce exactly one recommendation on async/IO | async/IO decision packet | packet recommends one outcome and names rationale, boundary, proof burden, rejected expansions |
| 4. Decision freeze | convert the two packets into one frozen milestone posture | decision-freeze record | parent records the final outcomes and closes the "maybe" state |
| 5. Contract ratification | align all repo-facing authority surfaces to the frozen decisions | `PLAN.md`, `ORCH_PLAN.md`, `docs/rust_v1_contract_stack.md`, repo-facing docs | no checked-in doc teaches a conflicting pre-I8 story |
| 6. I8 handoff freeze | define the final proof-run target explicitly | handoff record, validation wall, deferred `V1.1` list | I8 is enumerable, bounded, and no longer discovery-shaped |

## Worktree Parallelization Strategy

This milestone has two independent reasoning workstreams and one dependent
ratification workstream. That means partial parallelization is useful, but only
if the write scopes stay clean.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| basis freeze | `.runs/i7/`, benchmark authority surfaces, top-level docs as read-only inputs | — |
| bounded-generics packet | decision packets, contract docs as read-only inputs | basis freeze |
| async/IO packet | decision packets, contract docs as read-only inputs | basis freeze |
| decision freeze | `.runs/i7/`, parent-owned acceptance surfaces | bounded-generics packet, async/IO packet |
| contract ratification | `PLAN.md`, `ORCH_PLAN.md`, `docs/`, `README.md`, `DECISIONS.md`, `CHANGELOG.md`, `TODOS.md` | decision freeze |
| conditional proof implementation | `spec-core/**`, `spec-cli/**`, `examples/**`, `benchmarks/**` | decision freeze, only if I7 admits a new bounded slice |
| I8 handoff freeze | `.runs/i7/`, `PLAN.md`, `ORCH_PLAN.md`, `docs/` | contract ratification, conditional proof implementation if needed |

### Parallel lanes

- `Lane A`: bounded-generics packet
  - sequential within lane
  - owns only draft decision artifacts until the parent freezes the decision
- `Lane B`: async/IO packet
  - sequential within lane
  - owns only draft decision artifacts until the parent freezes the decision
- `Lane C`: contract ratification
  - must wait for parent decision freeze
  - owns checked-in authority surfaces after the decisions are frozen
- `Lane D`: conditional proof implementation
  - exists only if I7 admits a bounded new slice into V1
  - must wait for decision freeze because proof burden depends on the final
    decision

### Execution order

1. Parent freezes the basis locally.
2. Launch `Lane A` and `Lane B` in parallel.
3. Parent freezes the decisions after both packets return.
4. Launch `Lane C` after decision freeze, or do the ratification work locally.
5. Launch `Lane D` only if the frozen decision admits a new slice that needs
   code proof.
6. Parent closes with the I8 handoff freeze and final validation wall.

### Conflict flags

- `Lane A` and `Lane B` must not both edit checked-in authority docs directly.
  If they do, they will collide in `PLAN.md`, `ORCH_PLAN.md`, and `docs/`.
- `Lane C` and `Lane D` can conflict if a newly admitted slice changes both docs
  and proving code at the same time. In that case, finish the proof slice first
  and ratify the final docs second.
- If no new bounded slice is admitted, `Lane D` does not exist and I7 remains a
  docs-and-decision milestone.

## Failure Modes

| Failure mode | Consequence | Guard in this plan |
| --- | --- | --- |
| bounded generics is admitted by prose drift instead of a packet | Rust V1 claim widens without proof burden | require a bounded-generics packet and parent decision freeze |
| async/IO is admitted as a vibe instead of a boundary | V1 silently turns into "normal backend Rust" | require an async/IO packet with explicit authored boundary |
| `BENCH-CROSSLIB` is retired without replacement logic | negative-proof boundary weakens and final claim gets softer | require replacement rationale before retirement or demotion |
| docs are ratified before decisions are frozen | checked-in authority teaches draft reasoning as truth | authority merges happen only after decision freeze |
| I8 closes without an enumerated proof wall | final milestone reopens scope discovery | require an explicit I8 handoff naming commands, walls, and deferred surfaces |

Critical gap test:

- if any proposed admission has no named proof burden, it is not ready for V1
- if any checked-in doc still implies an inferred `I9`, I7 is not done

## Acceptance Checklist

I7 is complete only when all of these are true at the same time:

- the repo has one explicit answer on bounded generics
- the repo has one explicit answer on async/IO
- the checked-in ladder is `I7 -> I8`
- the repo does not imply a checked-in `I9`
- the proof-authoritative command wall remains the I3.5 wall unless a scoped
  admission adds explicit proving commands
- the benchmark roster still matches the ratified claim
- the final plain-English Rust V1 line can be quoted without hand-waving

## Validation Wall

Minimum truth commands to rerun before declaring I7 closed:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

If I7 admits any new bounded slice into V1, append the exact proving commands
for that slice before closing the milestone.

## Implementation Tasks

These are the minimum build-actionable tasks required to execute I7 cleanly.

- [x] **T1 (P1, human: ~30m / CC: ~5m)** — basis freeze — record the current
  post-I6 benchmark walls, deferred rows, and command-wall semantics in a cited
  basis packet.
  - Verify: basis packet exists and later packets cite it instead of restating
    memory.
- [x] **T2 (P1, human: ~45m / CC: ~10m)** — bounded generics — produce one
  bounded-generics packet that recommends either bounded admission or explicit
  `V1.1` deferral.
  - Verify: packet includes rationale, boundary, proof burden, and rejected
    broader expansions.
- [x] **T3 (P1, human: ~45m / CC: ~10m)** — async/IO — produce one async/IO
  packet that recommends either bounded admission or explicit synchronous-only
  `V1.1` deferral.
  - Verify: packet includes rationale, boundary, proof burden, and rejected
    broader expansions.
- [x] **T4 (P1, human: ~30m / CC: ~10m)** — decision freeze — convert the two
  packets into one frozen milestone posture before any authority docs are
  rewritten.
  - Verify: decision-freeze record names one outcome for bounded generics and
    one outcome for async/IO.
- [x] **T5 (P1, human: ~45m / CC: ~15m)** — contract ratification — update
  `PLAN.md`, `ORCH_PLAN.md`, `docs/rust_v1_contract_stack.md`, and repo-facing
  closeout docs so they all teach the same final pre-I8 story.
  - Verify: no checked-in doc still teaches I6 as current or leaves the V1 line
    to inference.
- [x] **T6 (P1, human: ~30m / CC: ~10m)** — I8 handoff freeze — enumerate the
  final proof walls, authoritative commands, and deferred `V1.1` list.
  - Verify: I8 can be described as a proof run rather than a discovery phase.
- [x] **T7 (P1, human: ~15m / CC: ~5m)** — closeout validation — rerun the
  proof-authoritative commands and add any conditional proof commands required
  by an admitted slice.
  - Verify: validation wall is current and matches the ratified claim.

## What Success Looks Like

After I7:

- the repo can answer "what is honest Rust V1?" in one paragraph
- that answer names both what is included and what is deferred
- no maintainer needs to infer the post-I6 plan from external notes
- I8 is a proof run, not another scope-closure milestone

## NOT in scope

- broad generics support
  - rationale: this milestone resolves the V1 contract line, not the broader
    Rust roadmap
- framework-heavy async support
  - rationale: that would replace the narrow-core posture instead of clarifying
    it
- benchmark-mechanics redesign
  - rationale: I3, I3.5, M67, and M68 already own the command wall and truth
    mechanics
- reopening I3.5 command-wall semantics
  - rationale: I7 consumes that wall as frozen authority
- inventing a checked-in `I9` to hide unresolved I7 decisions
  - rationale: I7 must end by freezing I8 as the final proof milestone

## Immediate Next Move

Execute I7 in this order:

1. freeze the post-I6 basis packet
2. draft the bounded-generics packet
3. draft the async/IO packet
4. freeze the decisions
5. ratify repo-facing authority from the frozen decisions
6. freeze the I8 handoff
7. rerun the validation wall

Do not start "wrapping Rust V1" by widening code scope before the basis and
decision packets exist as repo-backed truth.
