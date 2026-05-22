# I7 Orchestration Plan

Status: **authoritative execution runbook**
Milestone: **I7 Rust V1 scope-decision closure**
Plan authority: **`/home/azureuser/__Active_Code/atomize-hq/spec/PLAN.md`**
Frozen basis: **current `HEAD` of `main` when the run begins**
Primary workspace: **`/home/azureuser/__Active_Code/atomize-hq/spec`**
Last rewritten: **2026-05-22**

## Summary

- Execute from `/home/azureuser/__Active_Code/atomize-hq/spec`.
- Treat `PLAN.md` as the only milestone authority.
- Treat the old I6 execution shape as shipped history, not current work.
- Keep the parent agent as the only merge authority and the only final
  acceptance authority.
- Keep the critical path local to the parent for:
  - basis freeze
  - decision freeze
  - contract-ratification merge
  - final I8 handoff

I7 is lighter than I6. This milestone is mostly about forcing the last scope
decisions into checked-in authority instead of letting them float.

## Starting Truth

Observed on `main` after the `0.15.2` landing:

- `BENCH-ECOM` is active positive proof.
- `BENCH-SERVICE` is active positive proof.
- `BENCH-CROSSLIB` is the active companion-negative wall.
- repo-root `status . --format json` remains `inventory_only`.
- repo-root `export .` remains unsupported for this workspace shape.
- `M66` still leaves bounded generics and async/IO outside the narrow-core
  shipped baseline.
- the repo now needs a checked-in answer for the final honest Rust V1 line.

## Hard Guards

- Do not reopen I3.5 command-wall semantics.
- Do not redesign benchmark mechanics.
- Do not widen Rust V1 support without an explicit I7 decision packet.
- Do not treat "maybe V1 later" as a substitute for repo-backed truth.
- Keep `I8` as the final proof-run milestone.
- Do not imply a checked-in `I9` queue.

## Worktree And Branch Plan

Use the live primary checkout as the parent lane basis:

- Parent basis branch: `main`
- Parent workspace: `/home/azureuser/__Active_Code/atomize-hq/spec`

Create the I7 worktree root only if parallel lanes are actually needed:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7
```

Recommended parent working branch for the live milestone after basis freeze:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec checkout -b codex/i7-v1-scope-closure
```

Optional worker lanes after basis freeze:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a -b codex/i7-lane-a-generics codex/i7-v1-scope-closure
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b -b codex/i7-lane-b-async codex/i7-v1-scope-closure
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-c -b codex/i7-lane-c-ratify codex/i7-v1-scope-closure
```

If the parent decides that no parallelism is needed, keep I7 single-lane and do
the work locally.

## Orchestration State

Canonical run state lives under:

- `I7_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7`
- basis record: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis.json`
- decision freeze: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/decision-freeze.json`
- final handoff: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/i8-handoff.json`
- session log: `/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/session-log.md`

Suggested task packets:

- `i7-a0-basis-freeze`
- `i7-a1-bounded-generics-decision`
- `i7-a2-async-io-decision`
- `i7-b0-contract-ratification`
- `i7-c0-i8-handoff-freeze`

The parent owns `.runs/i7/**`. Workers do not mutate orchestration state
directly.

## Lane Map

| Lane | Branch | Owned write set | Goal |
| --- | --- | --- | --- |
| Parent | `codex/i7-v1-scope-closure` | `.runs/i7/**`, merge decisions, final acceptance | basis freeze, decisions, final authority |
| Lane A | `codex/i7-lane-a-generics` | decision packet drafts, supporting contract docs | bounded-generics recommendation |
| Lane B | `codex/i7-lane-b-async` | decision packet drafts, supporting contract docs | async/IO recommendation |
| Lane C | `codex/i7-lane-c-ratify` | `PLAN.md`, `ORCH_PLAN.md`, `docs/rust_v1_contract_stack.md`, repo-facing closeout docs | ratified repo authority after parent freezes decisions |

If I7 later admits a bounded new slice that truly requires code proof, create an
additional lane only after the parent records the decision freeze. Do not front-run
that work.

## Gate Model

### Gate 0: Basis Freeze

Owner: parent  
Task id: `i7-a0-basis-freeze`

Advance only when:

- current branch and commit are recorded in `.runs/i7/basis.json`
- the shipped benchmark walls are recorded
- the current deferred rows from M66 are recorded
- the current command-wall semantics are recorded
- any worker lanes, if used, are created from the frozen basis

Reopen if:

- basis drifts before decision work starts
- a worker is launched from an unfrozen commit
- the starting claim set is still paraphrase instead of evidence

### Gate 1: Decision Freeze

Owner: parent  
Task id: `i7-a3-decision-freeze`

Advance only when:

- the bounded-generics packet recommends exactly one outcome
- the async/IO packet recommends exactly one outcome
- each packet names:
  - the user-value rationale
  - the authored boundary
  - the proof burden
  - the rejected broader expansions
- the parent records both decisions in `.runs/i7/decision-freeze.json`

Reopen if:

- a packet still says "maybe"
- a packet quietly widens into broad support
- the proof burden is missing or unbelievable

### Gate 2: Contract Ratification

Owner: parent  
Task id: `i7-b0-contract-ratification`

Advance only when:

- `PLAN.md` teaches the frozen I7 story
- `ORCH_PLAN.md` teaches the frozen I7 execution shape
- `docs/rust_v1_contract_stack.md` teaches `I7 -> I8`
- repo-facing docs no longer imply a post-I6 planning vacuum
- the repo names deferred `V1.1` surfaces explicitly if they remain out

Reopen if:

- any checked-in doc still teaches the old I6-as-current story
- one doc says a surface is in V1 while another leaves it deferred
- `I8` is still undefined as the final proof milestone

### Gate 3: I8 Handoff Freeze

Owner: parent  
Task id: `i7-c0-i8-handoff-freeze`

Advance only when:

- the benchmark walls required for I8 are named explicitly
- the proof-authoritative commands are named explicitly
- the deferred `V1.1` list is named explicitly
- the parent records the target in `.runs/i7/i8-handoff.json`

Reopen if:

- I8 still sounds like "we'll know it when we see it"
- the proof wall is not enumerable
- any deferred surface is still ambient instead of named

## Execution Order

1. Freeze basis locally.
2. Draft bounded-generics and async/IO packets.
3. Parent freezes the decisions.
4. Ratify checked-in docs from the frozen decision state.
5. Freeze the I8 handoff.

Do not reverse steps 3 and 4. The docs should teach frozen decisions, not draft
debate.

## Validation Wall

Before final acceptance, rerun the current proof-authoritative commands:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

If I7 admits a bounded new slice into V1, append its exact proving commands
here before closing the milestone.

## Final Acceptance

I7 lands only when all of these are true:

- the repo has one answer on bounded generics
- the repo has one answer on async/IO
- the checked-in ladder is `I7 -> I8`
- the repo does not imply a checked-in `I9`
- the parent froze the I8 target explicitly
- the proof wall still matches the ratified claim

## Failure Modes

Bounce the milestone if any of these happen:

- "decision packet" becomes code-scope creep
- "ratification" becomes benchmark-mechanics churn
- `V1.1` deferral is treated like failure instead of explicit product truth
- I8 is used as a bucket for unresolved I7 ambiguity

## Immediate Next Move

If starting fresh from this runbook:

1. create `codex/i7-v1-scope-closure`
2. freeze the post-I6 basis into `.runs/i7/basis.json`
3. draft the two scope-decision packets
4. let the parent freeze the decisions before rewriting broader repo-facing docs
