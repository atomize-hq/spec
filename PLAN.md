# I8: Rust V1 Final Proof Run Plan

Status: **authoritative implementation plan**
Iteration: **I8**
Milestone family: **Rust V1 final proof run**
Implementation readiness: **ready to execute**
Plan scope: **rerun the frozen Rust V1 proof wall, preserve repo-root inventory semantics, and close Rust V1 only if the live benchmark surfaces, deferred boundaries, and repo-facing docs still match the ratified narrow-core claim**
Base branch: **main**
Working branch: **`feat/i8-final-proof-run`**
Validated at commit: **`5d849d4`**
Last rewritten: **2026-05-23**

Supersedes:

- the prior `I7: Rust V1 Scope-Decision Closure Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- I7 decision freeze: `.runs/i7/decision-freeze.json`
- I7 handoff packet: `.runs/i7/i8-handoff.json`
- benchmark roster: `benchmarks/labels.json`
- live repo truth on `feat/i8-final-proof-run` at `5d849d4`:
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - `cargo run -p spec-cli -- status . --format json`

Historical context, not authority:

- `README.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `TODOS.md`
- `ORCH_PLAN.md`

Primary repo surfaces for I8 closeout:

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
- `.runs/i8/**`

## Executive Summary

I7 already made the product decisions. Rust V1 stays narrow. Rust V1 stays
synchronous-only. Bounded generics and async or IO-owned boundaries stay
deferred to `V1.1`. `BENCH-CROSSLIB` stays visible as companion negative proof
and never counts as positive supported credit.

That means I8 is not another scope milestone. I8 is the final proof run over
the already-ratified claim. The job is to prove that the live repo still tells
the same story through the same five-command wall, then freeze the evidence and
close Rust V1 without widening scope by prose drift, benchmark drift, or
repo-root misinterpretation.

I8 is complete only when the repo can say this sentence honestly, with checked
evidence:

> Rust V1 is the current narrow-core `spec` surface: synchronous supported
> function families plus plain data and sum seams, proven by BENCH-ECOM and
> BENCH-SERVICE, with BENCH-CROSSLIB preserved as companion negative proof.

## Frozen I8 Contract

These are inherited from I7 and are not open for reinterpretation in I8:

- the active ladder is `I7 -> I8`, not `I7 -> I8 -> I9`
- the Rust V1 claim stays narrow, synchronous, and benchmark-backed
- deferred `V1.1` surfaces are exactly:
  - bounded generics
  - async flows, runtime adapters, and IO-owned boundaries
- `BENCH-ECOM` and `BENCH-SERVICE` are the only positive proof walls required
  for the V1 claim
- `BENCH-CROSSLIB` remains the active companion negative proof wall
- the authoritative I8 proof wall is exactly five commands:
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - `cargo run -p spec-cli -- status . --format json`
- no new slice-specific proof commands are admitted in I8
- repo-root `export .` remains unsupported for this workspace shape and is not
  part of the I8 wall
- repo-root `status . --format json` remains a broad inventory surface with
  `scope_authority: inventory_only`; it is not a green ship gate

## Current Validated Truth

Observed live on `feat/i8-final-proof-run` at commit `5d849d4` on 2026-05-23:

- `status examples/ecommerce/units --format json` passes with:
  - `BENCH-ECOM`
  - `benchmark_status: passing`
  - `gate_status: satisfied`
  - `readability_review_status: current`
- `export examples/ecommerce/units` passes with:
  - `schema_version: 4`
  - `provenance.git_commit_sha: 5d849d4f60676c259793d0592b7c1af07431d9a2`
  - full `BENCH-ECOM` projection, including required molecule proofs and
    readability-generated file inventory
- `status examples/service/units --format json` passes with:
  - `BENCH-SERVICE`
  - `benchmark_status: passing`
  - `gate_status: satisfied`
  - `readability_review_status: current`
- `export examples/service/units` passes with:
  - `schema_version: 4`
  - `provenance.git_commit_sha: 5d849d4f60676c259793d0592b7c1af07431d9a2`
  - full `BENCH-SERVICE` projection, including required molecule proofs and
    readability-generated file inventory
- `status . --format json` exits `1`, which is still correct for I8 because:
  - `scope_authority` is `inventory_only`
  - `BENCH-CROSSLIB` remains visible with `benchmark_status: passing` and
    `gate_status: not_applicable`
  - `BENCH-ECOM` and `BENCH-SERVICE` still project as `passing`
  - intentionally non-green roots still exist in the broad inventory surface,
    including fixtures, semantic-family packets, and unsupported or untested
    diagnostic roots that are outside the positive proof wall

This is the behavior I8 must preserve unless a real truth bug is found.

## Scope Challenge

### Premise correction

I8 is not "final Rust work." I8 is:

```text
prove that the already-ratified Rust V1 claim still holds on the live repo,
with the same benchmark roster, the same deferred boundaries, and the same
five-command wall
```

If the work expands beyond that sentence, the milestone escaped.

### What existing code already solves the problem

The repo already ships everything I8 needs:

- the benchmark roster and roles are already encoded in `benchmarks/labels.json`
- benchmark-aware `status` and `export` already project full truth at schema
  version `4`
- readability anchors already exist for the two positive walls
- I7 already froze the scope decision and wrote the I8 handoff packet
- repo-facing docs already describe the narrow-core story

I8 should reuse those surfaces exactly. It should not invent parallel proof
machinery, new labels, or a new source of authority.

### Minimum change set

If the frozen wall still passes, the minimum complete I8 diff is:

1. create `.runs/i8/` and freeze the run inputs
2. rerun the five-command wall and archive raw outputs
3. compare those outputs against the frozen claim and checked-in docs
4. patch only real authority drift
5. write the closeout packet

Code changes are conditional only. If a proof-wall command fails or a read-side
surface contradicts the ratified claim, fix only the direct blocker. Do not
turn one failing command into a mechanics redesign.

### Complexity check

This milestone should stay boring:

- no new CLI commands
- no new benchmark kinds
- no new support rows
- no new example or service roots
- no new proof writers
- no new artifact classes beyond the bounded `.runs/i8/` closeout records

If the work grows into benchmark schema edits, support-boundary changes, or a
new post-I8 planning milestone, stop. That is not I8.

### Completeness rule

The complete version is cheap here, so take it:

- rerun all five commands, not fragments
- inspect benchmark projections, not just exit codes
- archive raw outputs, not just summaries
- compare docs against live truth, not against memory
- close only after the repo-facing story and the command wall say the same thing

The shortcut version would be "the commands seem fine, ship it." I8 should not
use that shortcut.

### Distribution check

I8 introduces no new distributable artifact. The release surface remains the
existing CLI and checked-in repo authority. Distribution work is unchanged and
out of scope.

## What Already Exists

| Sub-problem | Existing owner | I8 action |
| --- | --- | --- |
| milestone ladder and scope boundary | `docs/rust_v1_contract_stack.md`, `.runs/i7/i8-handoff.json` | reuse exactly; do not infer `I9` |
| supported-vs-deferred Rust claim | `DECISIONS.md`, `.runs/i7/decision-freeze.json` | quote and verify, not reinterpret |
| benchmark role split | `benchmarks/labels.json`, I7 handoff | preserve two positive walls plus one companion negative wall |
| benchmark/readability mechanics | `status`/`export` schema v4, committed readability reviews, snapshot surfaces | verify read-side truth still matches the frozen mechanics |
| positive proof walls | `examples/ecommerce/units`, `examples/service/units` | rerun unchanged |
| companion-negative visibility | `examples/crosslib-app/units`, repo-root `status`, `BENCH-CROSSLIB` labels | verify still visible and still zero-credit |
| repo-root scope semantics | `.runs/i3_5_authority_alignment/**`, current CLI behavior | preserve `inventory_only`; do not try to make repo-root globally green |
| scope-closure packet trail | `.runs/i7/**` | treat as frozen inputs, then append I8 evidence separately under `.runs/i8/**` |

## Final Rust V1 Claim

This is the exact plain-English line I8 is closing:

> Rust V1 is the current narrow-core `spec` surface: synchronous supported
> function families plus plain data and sum seams, proven by BENCH-ECOM and
> BENCH-SERVICE, with BENCH-CROSSLIB preserved as companion negative proof.

Derived consequences:

- supported:
  - synchronous supported function families
  - plain pipeline and wrapper composition inside the shipped supported
    families
  - plain data seams
  - plain sum seams
  - truthful proof surfaces over those supported rows
- deferred to `V1.1`:
  - bounded generics
  - async flows, runtime adapters, and IO-owned boundaries
- visible but non-crediting:
  - companion negative proof in `BENCH-CROSSLIB`

## Artifact Map

The I8 run root is intentionally small and deterministic:

- `.runs/i8/preflight.json`
- `.runs/i8/evidence/ecommerce.status.json`
- `.runs/i8/evidence/ecommerce.export.json`
- `.runs/i8/evidence/service.status.json`
- `.runs/i8/evidence/service.export.json`
- `.runs/i8/evidence/workspace.status.json`
- `.runs/i8/authority-drift.md`
- `.runs/i8/closeout.json`

Artifact contract:

- raw command output files are canonical evidence
- summaries may cite raw outputs but do not replace them
- checked-in authority docs remain the public story
- `.runs/i8/**` is the private closeout packet that proves how that story was
  revalidated

## Architecture And Execution Graph

I8 is a verification-and-ratification run, not a product expansion:

```text
I7 decision freeze
      |
      v
live benchmark roster + live CLI truth
      |
      v
I8 preflight freeze (.runs/i8/)
      |
      +---------------------+----------------------+
      |                     |                      |
      v                     v                      v
BENCH-ECOM rerun      BENCH-SERVICE rerun   workspace inventory rerun
      |                     |                      |
      +---------------------+----------+-----------+
                                     |
                                     v
                        authority drift comparison
                                     |
                     +---------------+---------------+
                     |                               |
                     v                               v
              no drift or doc drift only      truth blocker found
                     |                               |
                     v                               v
              final closeout packet        direct blocker repair only
                     |                               |
                     +---------------+---------------+
                                     |
                                     v
                      truthful Rust V1 done-state claim
```

Critical dependency rule:

- proof reruns may happen before doc edits
- doc edits may not happen until proof outputs are captured and interpreted
- blocker repair exists only if a live command or live projection contradicts
  the frozen claim
- closeout happens only after proof truth and repo-facing prose agree

## Work Phases

| Phase | Goal | Primary outputs | Exit criteria |
| --- | --- | --- | --- |
| 1. Preflight freeze | create a reproducible I8 run root and freeze the inputs | `.runs/i8/preflight.json` | every later step cites frozen inputs and fixed output paths |
| 2. Positive proof rerun | rerun both positive benchmark walls unchanged | ecommerce and service raw status/export outputs | both positive walls still pass with satisfied gates and current readability |
| 3. Broad inventory confirmation | rerun repo-root inventory and confirm companion-negative visibility | workspace raw status output and inventory interpretation | `scope_authority: inventory_only` is preserved and `BENCH-CROSSLIB` remains zero-credit |
| 4. Authority drift ratification | compare live outputs against checked-in authority surfaces | `.runs/i8/authority-drift.md`, doc diff only if needed | all repo-facing authority surfaces teach one identical I8 story |
| 5. Conditional blocker repair | repair only a direct truth blocker if one exists | bounded code or doc diff plus rerun evidence | failing truth surface is fixed without widening support or commands |
| 6. Final closeout | freeze the final evidence packet and milestone verdict | `.runs/i8/closeout.json`, release-note updates if needed | Rust V1 can be stated honestly with no extra caveats |

### Phase detail

#### Phase 1. Preflight freeze

Create `.runs/i8/` and write `preflight.json` with:

- branch
- commit
- timestamp
- authority inputs
- the exact five-command wall
- expected evidence output file paths
- the frozen plain-English Rust V1 claim

No interpretation happens here. This phase freezes the basis only.

#### Phase 2. Positive proof rerun

Run these four commands against the live branch and archive raw stdout:

- `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
- `cargo run -p spec-cli -- export examples/ecommerce/units`
- `cargo run -p spec-cli -- status examples/service/units --format json`
- `cargo run -p spec-cli -- export examples/service/units`

Required interpretation:

- both benchmarks must remain `passing`
- both gates must remain `satisfied`
- both readability reviews must remain `current`
- both exports must remain `schema_version: 4`
- both exports must still project the same positive benchmark role and required
  molecule roster

#### Phase 3. Broad inventory confirmation

Run:

- `cargo run -p spec-cli -- status . --format json`

Expected outcome:

- non-zero exit code is allowed and expected
- `scope_authority` must be `inventory_only`
- `BENCH-CROSSLIB` must remain visible as active companion negative proof
- `BENCH-CROSSLIB` must keep `positive_credit_cases: 0`
- `BENCH-ECOM` and `BENCH-SERVICE` must still project as passing inside the
  broad surface

This phase exists to prevent a false green and to prevent a false red.

#### Phase 4. Authority drift ratification

Compare live outputs against:

- `PLAN.md`
- `ORCH_PLAN.md`
- `docs/rust_v1_contract_stack.md`
- `README.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `TODOS.md`

Allowed outcomes:

- no drift: record that all surfaces already agree
- doc drift only: patch only the prose that drifted

Disallowed outcome:

- reinterpreting the live output so stale prose can stay unchanged

#### Phase 5. Conditional blocker repair

This phase exists only if a live truth surface disagrees with the frozen claim.

Repair policy:

- fix the direct blocker only
- rerun the affected command immediately
- rerun the full five-command wall before closeout
- stop and escalate if the repair would require:
  - new proof commands
  - new support rows
  - benchmark schema redesign
  - widening a deferred `V1.1` surface into Rust V1

#### Phase 6. Final closeout

Write `.runs/i8/closeout.json` with:

- the final Rust V1 claim
- deferred `V1.1` surfaces
- the five-command verdicts
- raw evidence file references
- any doc files changed for drift repair
- final closeout status: `done` or `blocked`

## Proof And Coverage Diagram

This is the exact I8 proof surface. Every row must be observed and archived.

```text
PROOF COMMANDS                                            CLAIM SURFACE
[PASS] cargo run -p spec-cli -- status examples/ecommerce/units --format json
  -> BENCH-ECOM positive wall
  -> benchmark_status: passing
  -> gate_status: satisfied
  -> readability_review_status: current
  -> archive: .runs/i8/evidence/ecommerce.status.json

[PASS] cargo run -p spec-cli -- export examples/ecommerce/units
  -> schema_version: 4 full projection
  -> BENCH-ECOM roster, molecule proofs, readability files visible
  -> archive: .runs/i8/evidence/ecommerce.export.json

[PASS] cargo run -p spec-cli -- status examples/service/units --format json
  -> BENCH-SERVICE positive wall
  -> benchmark_status: passing
  -> gate_status: satisfied
  -> readability_review_status: current
  -> archive: .runs/i8/evidence/service.status.json

[PASS] cargo run -p spec-cli -- export examples/service/units
  -> schema_version: 4 full projection
  -> BENCH-SERVICE roster, molecule proofs, readability files visible
  -> archive: .runs/i8/evidence/service.export.json

[EXPECTED EXIT 1] cargo run -p spec-cli -- status . --format json
  -> scope_authority: inventory_only
  -> BENCH-CROSSLIB visible as companion_negative_proof
  -> positive_credit_cases: 0 for BENCH-CROSSLIB
  -> BENCH-ECOM and BENCH-SERVICE still passing in broad projection
  -> archive: .runs/i8/evidence/workspace.status.json

COVERAGE: 5/5 commands
CLAIM PATHS: 5/5 covered
GAPS: 0 command-surface gaps
```

Interpretation guard:

- a green I8 does not mean repo-root `status .` exits `0`
- a green I8 does mean the two positive walls pass, the broad inventory surface
  still tells the truth, and the checked-in docs match that reality

## Acceptance Checklist

I8 closes only when all of these are true at the same time:

- the plain-English Rust V1 claim in this file still matches:
  - `.runs/i7/decision-freeze.json`
  - `.runs/i7/i8-handoff.json`
  - `DECISIONS.md`
  - `docs/rust_v1_contract_stack.md`
- `BENCH-ECOM` remains `passing` with `gate_status: satisfied`
- `BENCH-SERVICE` remains `passing` with `gate_status: satisfied`
- both positive benchmark exports still emit `schema_version: 4`
- `BENCH-CROSSLIB` remains visible in repo-root inventory and still counts as
  zero positive supported credit
- repo-root `status . --format json` still reports `scope_authority:
  inventory_only`
- no new proof commands were added to make the claim work
- no deferred `V1.1` surface was silently promoted
- no checked-in doc implies a new post-I8 discovery milestone
- `.runs/i8/closeout.json` records the exact final verdict and raw command refs

## Worktree Parallelization Strategy

I8 has three verification steps but only two should run in parallel. The two
positive walls are independent and cheap to split. The repo-root inventory run
is broader, heavier, and easier to misread before the positive walls are
confirmed, so keep it as a parent-owned follow-up step.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| preflight freeze | `.runs/i8/` | — |
| BENCH-ECOM proof rerun | `examples/ecommerce/`, `benchmarks/`, `.runs/i8/evidence/` | preflight freeze |
| BENCH-SERVICE proof rerun | `examples/service/`, `benchmarks/`, `.runs/i8/evidence/` | preflight freeze |
| broad inventory confirmation | repo root inventory surfaces, `examples/crosslib-app/`, `semantic-families/`, `benchmarks/`, `.runs/i8/evidence/` | BENCH-ECOM proof rerun, BENCH-SERVICE proof rerun |
| authority drift ratification | `PLAN.md`, `ORCH_PLAN.md`, `docs/`, `README.md`, `DECISIONS.md`, `CHANGELOG.md`, `TODOS.md` | broad inventory confirmation |
| conditional blocker repair | only the exact modules implicated by a failing command | whichever verification step found the blocker |
| final closeout | `.runs/i8/`, release-note surfaces if changed | authority drift ratification, conditional blocker repair if needed |

### Parallel lanes

- `Lane A`: BENCH-ECOM proof rerun
  - sequential within lane
  - owns only `.runs/i8/evidence/ecommerce.*`
- `Lane B`: BENCH-SERVICE proof rerun
  - sequential within lane
  - owns only `.runs/i8/evidence/service.*`
- `Lane C`: broad inventory confirmation
  - parent-owned
  - must wait for Lane A and Lane B
  - owns only `.runs/i8/evidence/workspace.status.json`
- `Lane D`: authority drift ratification
  - parent-owned
  - must wait for Lane C
  - owns checked-in authority docs only
- `Lane E`: conditional blocker repair
  - exists only if a real truth blocker is found
  - must stay bounded to the direct failing surface

### Execution order

1. Parent creates `.runs/i8/` and freezes inputs.
2. Launch `Lane A` and `Lane B` in parallel worktrees.
3. Parent compares the two positive proof outputs against the frozen claim.
4. Parent runs `Lane C` in the main checkout.
5. Parent launches `Lane D` only after the inventory interpretation is settled.
6. Parent launches `Lane E` only if a real blocker must be repaired.
7. Parent writes the final I8 closeout packet.

### Conflict flags

- `Lane A` and `Lane B` must not write the same evidence files.
- `Lane A` and `Lane B` may still contend on Cargo package or build locks if
  they share the same local cache. That is acceptable, but it means parallel
  worktrees improve operator separation more than raw wall-clock time.
- `Lane C` must not start before both positive walls are understood. Otherwise
  repo-root inventory can be misread in the absence of benchmark context.
- `Lane D` must not ratify docs before the workspace inventory semantics are
  confirmed.
- `Lane E` must not turn a narrow truth repair into a mechanics rewrite. If a
  proposed fix touches benchmark schema, proof writers, or support boundaries,
  stop and escalate.

## Failure Modes

| Failure mode | Consequence | Guard in this plan |
| --- | --- | --- |
| repo-root `status .` exit `1` is misread as I8 failure | a truthful broad inventory surface blocks the release for the wrong reason | require `scope_authority: inventory_only` and benchmark interpretation, not exit-code-only reasoning |
| `BENCH-CROSSLIB` starts counting as positive credit | the V1 claim widens by read-surface drift | require zero positive credit and preserve companion-negative classification |
| positive `status` passes but `export` drifts | downstream machine consumers see a different truth than human readers | require both `status` and `export` for each positive benchmark |
| readability anchors go stale while benchmark status stays green | the plan overstates the reviewability of emitted Rust | require `readability_review_status: current` for both positive walls |
| docs still imply I7 is current or imply I9 exists | milestone ownership becomes ambiguous again | compare every repo-facing authority surface after live reruns, not before |
| a blocker fix adds new proof commands or support rows | I8 quietly becomes another scope milestone | hard-stop any repair that changes the five-command wall or deferred boundaries |
| broad inventory roots are trimmed to force green | the repo loses honest visibility into deferred and fixture surfaces | preserve repo-root inventory behavior exactly |

Critical gap test:

- if either positive benchmark loses `passing` or `satisfied`, I8 is not done
- if any authority doc still needs caveats not present in the frozen claim, I8
  is not done
- if the only way to "pass" is to reinterpret `inventory_only` as proof, I8 is
  not done

## Performance And Operational Review

I8 should not introduce any new runtime or infrastructure cost. The only
meaningful operational concerns are:

- repo-root `status` is a heavier scan than benchmark-root commands, so do not
  treat it as a cheap loop
- raw command outputs must be captured once per closeout run so later doc review
  does not depend on rerunning the wall from memory
- parallelization is helpful only for the two positive walls; forcing more
  concurrency buys little and increases interpretation risk
- concurrent Cargo invocations may wait on package or build locks; that is not
  a blocker, but it means the parent should prefer clarity of ownership over
  chasing marginal runtime wins
- no new caching, queueing, proof-writer parallelism, or benchmark-registry
  work is justified here

Boring-by-default rule:

- if the wall passes, ship the closeout with evidence
- if the wall fails, repair the narrow blocker only

## Implementation Tasks

- [ ] **T1 (P1, human: ~15m / CC: ~5m)** — preflight freeze — create `.runs/i8/`
  and record the frozen inputs, branch, commit, plain-English claim, exact
  five-command wall, and bounded artifact map.
  - Verify: `.runs/i8/preflight.json` names the same authority inputs and evidence paths listed in this plan.
- [ ] **T2 (P1, human: ~15m / CC: ~5m)** — BENCH-ECOM proof rerun — rerun the
  ecommerce `status` and `export` commands and archive raw outputs to
  `.runs/i8/evidence/ecommerce.*`.
  - Verify: `BENCH-ECOM` stays `passing/satisfied/current` and the export remains `schema_version: 4`.
- [ ] **T3 (P1, human: ~15m / CC: ~5m)** — BENCH-SERVICE proof rerun — rerun the
  service `status` and `export` commands and archive raw outputs to
  `.runs/i8/evidence/service.*`.
  - Verify: `BENCH-SERVICE` stays `passing/satisfied/current` and the export remains `schema_version: 4`.
- [ ] **T4 (P1, human: ~10m / CC: ~5m)** — broad inventory confirmation — rerun
  repo-root `status . --format json`, archive it, and confirm the expected
  `inventory_only` interpretation plus companion-negative visibility.
  - Verify: `BENCH-CROSSLIB` remains visible with zero positive credit and repo-root `status` remains non-green by design.
- [ ] **T5 (P1, human: ~30m / CC: ~10m)** — authority drift ratification —
  compare the live outputs against `PLAN.md`, `ORCH_PLAN.md`,
  `docs/rust_v1_contract_stack.md`, `README.md`, `DECISIONS.md`, `CHANGELOG.md`,
  and `TODOS.md`, then patch only real drift.
  - Verify: every checked-in authority surface teaches the same I8 story with no implied `I9`.
- [ ] **T6 (P1, human: ~20m / CC: ~10m)** — final closeout packet — write
  `.runs/i8/closeout.json` with the final claim, deferred surfaces, command
  verdicts, doc-drift summary, and references to raw evidence files.
  - Verify: a future maintainer can reconstruct the exact I8 decision from `.runs/i8/` without relying on conversation context.
- [ ] **T7 (P1, human: ~variable / CC: ~variable)** — conditional blocker repair
  — if any proof command or authority surface contradicts the frozen claim, fix
  the direct blocker only and rerun the affected command plus the final
  five-command wall.
  - Verify: no repair widens support, changes benchmark roles, or adds new proof commands.

## NOT in scope

- bounded generics admission
  - rationale: I7 already deferred this to `V1.1`
- async or IO admission
  - rationale: I7 already froze Rust V1 as synchronous-only
- benchmark schema redesign
  - rationale: M67 and M68 already own benchmark roles and mechanics
- new benchmark roots or new benchmark kinds
  - rationale: the I8 claim closes over the existing three-benchmark roster
- repo-root `export .` support
  - rationale: this workspace shape still truthfully rejects aggregate export
- turning repo-root `status .` into a globally green ship gate
  - rationale: broad inventory is intentionally wider than the positive proof wall
- reopening I3.5 command-wall semantics
  - rationale: I8 consumes that wall as frozen authority
- inventing a checked-in `I9`
  - rationale: the active ladder ends at I8 until a new bounded post-V1 milestone exists

## Immediate Next Move

Execute I8 in this order:

1. create `.runs/i8/` and freeze the exact input set
2. rerun and archive the BENCH-ECOM proof wall
3. rerun and archive the BENCH-SERVICE proof wall
4. rerun and archive repo-root inventory status
5. compare live outputs against the frozen claim and repo-facing docs
6. patch only real drift
7. write the final closeout packet

Do not start by changing code. Start by proving whether the already-ratified
claim still holds on the live branch.
