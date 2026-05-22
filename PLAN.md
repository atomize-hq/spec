# I7: Rust V1 Scope-Decision Closure Plan

Status: **authoritative implementation plan**
Iteration: **I7**
Milestone family: **Rust V1 scope closure and contract ratification**
Implementation readiness: **ready for implementation**
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

I6 shipped the missing service proof wall.

That means the repo is no longer blocked on benchmark mechanics or on whether
`BENCH-SERVICE` is real. The remaining Rust V1 pressure is now narrower and
more dangerous: bounded generics and async/IO are still floating as ambient
"maybe V1, maybe later" scope.

That is how V1 definitions go stale. Not because the code is unclear, but
because the last 10 percent never gets said out loud.

I7 closes that gap.

This milestone does not start by widening Rust support. It starts by forcing the
scope decisions that M65 deliberately left open:

- does bounded generics join honest Rust V1 in a bounded slice, or defer
  explicitly to `V1.1`?
- does Rust V1 stay synchronous-only, or admit a bounded async/IO edge?

The milestone is complete only when the repo can answer those questions without
guessing, the answers are checked in as authority, and `I8` is frozen as the
final proof-run milestone.

## Current Validated Truth

Observed on `main` after the `0.15.2` landing:

- `CHANGELOG.md` says Rust V1 now ships an active service benchmark wall.
- `README.md` teaches both benchmark-root proof walls:
  - `examples/ecommerce/units`
  - `examples/service/units`
- `TODOS.md` marks `M68` complete on `2026-05-18` and `M69` complete on
  `2026-05-21`.
- `docs/rust_v1_contract_stack.md` previously stopped the implementation ladder
  at `I6`, which made the post-I6 state feel under-authored even though the repo
  had moved on.
- `M66` still treats bounded generics as a deferred row and still keeps async/IO
  outside the narrow-core contract.
- `BENCH-CROSSLIB` currently exists as the active companion-negative benchmark
  wall, not as a dead placeholder.

In plain English:

- the proof walls are real
- the narrow-core support line is real
- the final honest V1 scope line is not fully ratified yet

That is the work.

## Step 0: Scope Challenge

### Premise correction

The problem is not "finish Rust V1 by adding more Rust."

The problem is:

```text
turn the remaining post-I6 scope pressure into explicit repo-backed truth
so the final V1 claim is honest, bounded, and ready for one last proof run
```

If implementation expands beyond that sentence, it has escaped the milestone.

### Scope verdict

The complete version is still cheap here because the repo already has the proof
machinery and the benchmark walls.

I7 does not need:

- a new benchmark subsystem
- a new command wall
- a new benchmark roster
- speculative `I9` planning
- a broad Rust-support expansion disguised as "just finishing V1"

It needs:

- one explicit bounded-generics decision
- one explicit async/IO decision
- one ratified plain-English Rust V1 line
- one checked-in handoff to `I8`

### Default rule for unresolved pressure

I7 must not widen support by atmosphere.

If a surface is not already proven by current repo truth and cannot be admitted
through one small, bounded, well-explained slice, it defers explicitly to
`V1.1`.

That rule matters because otherwise "maybe later in this milestone" quietly
turns into "accidentally part of V1."

### Complexity check

Expected primary write scope:

- `PLAN.md`
- `ORCH_PLAN.md`
- `docs/rust_v1_contract_stack.md`
- `README.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `TODOS.md`

Possible conditional write scope only if I7 explicitly admits a bounded new
slice into V1 and the parent accepts the proof burden:

- `spec-core/**`
- `spec-cli/**`
- benchmark artifacts
- example roots needed to prove the admitted slice

If implementation starts redesigning benchmark mechanics, reopening I3.5, or
silently widening M66 without a forced decision record, stop. That is different
scope.

## What Already Exists

| Sub-problem | Current owner | I7 action |
| --- | --- | --- |
| benchmark-root proof walls | `BENCH-ECOM`, `BENCH-SERVICE`, I3.5 command wall | reuse unchanged as the proof baseline |
| companion-negative boundary | `BENCH-CROSSLIB` | preserve unless I7 records an explicit retirement decision |
| narrow-core Rust support rows | `M66` | reuse as the starting contract, not the final V1 ratification |
| benchmark and readability semantics | `M67`, `M68`, shipped repo mechanics | preserve unchanged unless a real truth bug appears |
| post-I6 milestone ambiguity | stale ladder and missing checked-in follow-on | replace with explicit `I7 -> I8` authority |

## Frozen Decisions

These are locked. I7 implements them and does not reopen them casually.

1. **I7 owns post-I6 scope closure.**
   - I6 is shipped.
   - I7 is the first checked-in milestone that answers what remains before
     honest Rust V1 can be declared complete.

2. **I7 does not widen support by default.**
   - bounded generics is not admitted just because it would be nice
   - async/IO is not admitted just because it feels adjacent
   - admission requires an explicit decision, bounded authored shape, and a
     believable proof burden

3. **Deferred is a real outcome.**
   - if bounded generics cannot be admitted cleanly, write down `V1.1`
   - if async/IO cannot be admitted cleanly, write down `V1.1`
   - the repo must prefer honest narrow truth over aspirational ambiguity

4. **I7 preserves the shipped proof wall unless it records a scoped change.**
   - `BENCH-ECOM` stays active positive proof
   - `BENCH-SERVICE` stays active positive proof
   - `BENCH-CROSSLIB` stays the active companion-negative wall unless I7
     explicitly retires or demotes it with replacement rationale

5. **I8 is the final proof-run milestone.**
   - I7 ends by freezing what I8 must prove
   - the repo should not imply a checked-in `I9` queue unless one is later
     authored on purpose

## Required Outputs

I7 is done only when all of these exist together:

1. a checked-in bounded-generics decision
2. a checked-in async/IO decision
3. a ratified plain-English Rust V1 claim
4. an explicit `I8` handoff that names the proof wall and the deferred `V1.1`
   list
5. repo-facing docs that no longer leave post-I6 state to inference

The exact artifact locations may vary during execution, but the repo must finish
with a maintainer-readable answer to:

- what is in Rust V1?
- what is out of Rust V1?
- what does I8 still need to prove?

## Decision Framework

### D2: Bounded Generics

I7 must answer:

- is there a bounded generic slice that is part of the honest Rust V1 claim?

Admit bounded generics into V1 only if all of these are true:

- the slice serves the stated solo Rust backend/business-logic user directly
- the authored shape is small and nameable
- the proof burden can be expressed on top of the shipped command wall without
  new benchmark mechanics
- the slice does not quietly smuggle in broader trait, lifetime, macro, or
  abstraction-heavy support

Otherwise:

- keep `ROW-GENERIC-BOUNDED` deferred
- state plainly that generics defer to `V1.1`

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

### D4: `BENCH-CROSSLIB` role

I7 does not treat this as ambient ambiguity anymore.

The shipped default is:

- `BENCH-CROSSLIB` remains the companion-negative benchmark because it is still
  doing useful boundary work for the current claim

I7 may retire or demote it only if the repo records:

- what negative-proof function it was serving
- what replaces that function
- why the final V1 proof run becomes clearer, not weaker

## Work Phases

### Phase 1: Basis Freeze

Capture the current post-I6 truth without embellishment:

- record the current benchmark-root proof commands and outputs
- record the current deferred rows from M66
- record the currently shipped benchmark roster and roles
- record the current plain-English README and changelog teaching surfaces

Deliverable:

- one basis packet that later I7 decisions can cite instead of paraphrasing from
  memory

### Phase 2: Bounded-Generics Decision Packet

Do exactly one of:

- admit one bounded generic slice into V1 with explicit authored boundaries and
  proof expectations
- defer bounded generics to `V1.1` explicitly

Required contents:

- user-value rationale
- authored boundary
- proof burden
- rejected broader expansions

### Phase 3: Async / IO Decision Packet

Do exactly one of:

- admit one bounded async/IO slice into V1 with explicit authored boundaries and
  proof expectations
- keep Rust V1 synchronous-only and defer async/IO to `V1.1`

Required contents:

- user-value rationale
- authored boundary
- proof burden
- rejected broader expansions

### Phase 4: Contract Ratification Pass

Update repo-facing authority surfaces so they all teach the same final pre-I8
story:

- the implementation ladder in `docs/rust_v1_contract_stack.md`
- the active milestone authority in `PLAN.md`
- the active execution runbook in `ORCH_PLAN.md`
- the plain-English Rust V1 line in repo-facing docs
- the explicit deferred `V1.1` list

This is the part that prevents the next maintainer from having to reconstruct
our intent from changelog archaeology.

### Phase 5: I8 Handoff Freeze

Before I7 closes, freeze exactly what I8 must prove:

- which benchmark walls are required
- which commands are authoritative
- which deferred surfaces are explicitly out of scope for V1
- which documentation line I8 is trying to make honest

If I8's target is still fuzzy at the end of I7, I7 is not done.

## Acceptance Checklist

I7 is complete only when all of these are true at the same time:

- the repo has one explicit answer on bounded generics
- the repo has one explicit answer on async/IO
- the post-I6 ladder is checked in as `I7 -> I8`
- the repo no longer implies an inferred `I9`
- the proof-authoritative command wall remains the I3.5 wall
- the benchmark roster still matches the ratified claim
- the final plain-English Rust V1 line can be quoted without hand-waving

## Validation Wall

Minimum truth commands to run before declaring I7 closed:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

If I7 admits any new bounded slice into V1, it must add the exact proving
commands required for that slice before the milestone can close.

## What Success Looks Like

After I7:

- the repo can answer "what is honest Rust V1?" in one paragraph
- the answer names both what is included and what is deferred
- no maintainer needs to infer the post-I6 plan from external notes
- I8 is a proof run, not another scope-discovery exercise

## NOT in scope

- broad generics support
- framework-heavy async support
- reopening benchmark mechanics for style reasons
- reopening I3.5 command-wall semantics
- inventing an `I9` milestone to hide unresolved decisions

## Open Questions

These remain open until I7 resolves them explicitly:

- Is there a bounded generic slice small enough and valuable enough to join V1?
- Is there a bounded async/IO edge small enough and valuable enough to join V1?
- Does `BENCH-CROSSLIB` still earn its benchmark slot once the final V1 line is
  ratified?

## Immediate Next Move

Treat this document as the current authority.

The next work is:

1. freeze the post-I6 basis packet
2. write the bounded-generics decision packet
3. write the async/IO decision packet
4. ratify the final pre-I8 V1 line in checked-in docs

Do not start "wrapping V1" by broadening code scope before those decisions are
repo-backed truth.
