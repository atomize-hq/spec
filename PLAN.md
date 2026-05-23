# I8: Rust V1 Final Proof Run Plan

Status: **authoritative implementation plan**
Iteration: **I8**
Milestone family: **Rust V1 final proof run**
Implementation readiness: **ready to execute**
Plan scope: **rerun the frozen Rust V1 proof wall, preserve repo-root inventory semantics, and close Rust V1 only if the live benchmark surfaces, deferred boundaries, and repo-facing docs still match the ratified narrow-core claim**
Base branch: **main**
Working branch: **`feat/i8-final-proof-run`**
Validated at commit: **`dc61b01`**
Last rewritten: **2026-05-23**

Supersedes:

- the prior `I7: Rust V1 Scope-Decision Closure Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- I7 freeze packet: `.runs/i7/decision-freeze.json`
- I7 handoff packet: `.runs/i7/i8-handoff.json`
- live repo truth on `feat/i8-final-proof-run` at `dc61b01`:
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

I7 already made the hard product decisions. Bounded generics stay deferred to
`V1.1`. Rust V1 stays synchronous-only. `BENCH-CROSSLIB` stays visible as
companion negative proof and never earns positive supported credit.

That means I8 is not a feature milestone. It is the final proof run that must
show the frozen claim still holds on the live repo without widening scope,
changing the command wall, or laundering broad-scope inventory output into
proof.

I8 is complete only when the repo can say one sentence honestly:

> Rust V1 is the current narrow-core `spec` surface: synchronous supported
> function families plus plain data and sum seams, proven by BENCH-ECOM and
> BENCH-SERVICE, with BENCH-CROSSLIB preserved as companion negative proof.

## Frozen I8 Contract

These are inherited, locked, and not open for reinterpretation in I8:

- The milestone ladder is `I7 -> I8`, not `I7 -> I8 -> I9`.
- The Rust V1 claim stays narrow, synchronous, and benchmark-backed.
- Deferred `V1.1` surfaces are exactly:
  - bounded generics
  - async flows, runtime adapters, and IO-owned boundaries
- `BENCH-ECOM` and `BENCH-SERVICE` are the only positive proof walls required
  for the V1 claim.
- `BENCH-CROSSLIB` remains active companion negative proof.
- The authoritative I8 proof wall is exactly five commands:
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo run -p spec-cli -- status examples/service/units --format json`
  - `cargo run -p spec-cli -- export examples/service/units`
  - `cargo run -p spec-cli -- status . --format json`
- No new slice-specific proof commands are admitted in I8.
- Repo-root `export .` remains unsupported for this workspace shape and is not
  part of the I8 wall.

## Current Validated Truth

Observed on `feat/i8-final-proof-run` at `dc61b01`:

- `status examples/ecommerce/units --format json` passes with:
  - `BENCH-ECOM`
  - `benchmark_status: passing`
  - `gate_status: satisfied`
  - `readability_review_status: current`
- `export examples/ecommerce/units` passes with `schema_version: 4` and a full
  benchmark projection for `BENCH-ECOM`.
- `status examples/service/units --format json` passes with:
  - `BENCH-SERVICE`
  - `benchmark_status: passing`
  - `gate_status: satisfied`
  - `readability_review_status: current`
- `export examples/service/units` passes with `schema_version: 4` and a full
  benchmark projection for `BENCH-SERVICE`.
- `status . --format json` exits `1`, which is still correct for I8 because:
  - `scope_authority` is `inventory_only`
  - `BENCH-CROSSLIB` remains visible with `benchmark_status: passing` and
    `gate_status: not_applicable`
  - `BENCH-ECOM` and `BENCH-SERVICE` still project as `passing`
  - non-green fixture and semantic-family roots remain part of the broad
    inventory surface, so repo-root success is not defined as "all roots green"

This is the exact behavior I8 must preserve unless a real truth bug is found.

## Scope Challenge

### Premise correction

I8 is not "final Rust work."

I8 is:

```text
prove that the already-ratified Rust V1 claim still holds on the live repo,
with the same benchmark roster, the same deferred boundaries, and the same
five-command wall
```

If implementation expands beyond that sentence, it has escaped the milestone.

### Minimum change set

If the frozen wall still passes, the minimum complete I8 diff is:

1. capture the final proof outputs under `.runs/i8/`
2. verify the benchmark roster and readability anchors still match the claim
3. patch any stale authority docs or release notes that still drift from the
   frozen claim
4. record the final closeout packet

Code changes are conditional only. If a proof-wall command fails or a read-side
surface contradicts the ratified claim, fix only the direct blocker. Do not
turn one failing command into a mechanics redesign.

### Complexity check

This should stay boring:

- no new benchmark artifacts beyond I8 closeout records under `.runs/i8/`
- no new CLI commands
- no new benchmark kinds
- no new support rows
- no new service/example roots

If the work grows into benchmark schema edits, proof-writer redesign, or a new
support boundary, stop. That is not I8.

### Completeness rule

The complete version is cheap here, so take it:

- rerun all five commands, not fragments
- inspect full benchmark projections, not just exit codes
- capture raw outputs for closeout
- verify docs and decisions against the actual live outputs

The shortcut version would be "commands look fine, ship it." I8 should not use
that shortcut.

### Distribution check

I8 introduces no new distributable artifact. The release surface remains the
existing CLI and checked-in repo authority. Distribution work is unchanged and
out of scope.

## What Already Exists

| Sub-problem | Existing owner | I8 action |
| --- | --- | --- |
| milestone ladder and scope boundary | `M65`, `docs/rust_v1_contract_stack.md`, `.runs/i7/i8-handoff.json` | reuse exactly; do not infer an `I9` |
| supported-vs-deferred Rust claim | `M66`, `DECISIONS.md`, `.runs/i7/decision-freeze.json` | quote and verify, not reinterpret |
| benchmark role split | `M67` | preserve two positive benchmarks plus one companion negative wall |
| benchmark/readability mechanics | `M68`, `benchmarks/labels.json`, snapshots, readability reviews, `status`/`export` schema v4 | verify read-side truth still matches the locked mechanics |
| positive proof walls | `examples/ecommerce/units`, `examples/service/units`, benchmark-aware CLI tests | rerun unchanged |
| companion-negative visibility | `examples/crosslib-app/units`, repo-root `status`, `BENCH-CROSSLIB` labels | verify still visible and still zero-credit |
| repo-root scope semantics | I3.5 authority snapshot plus current CLI behavior | preserve `inventory_only`; do not try to make it globally green |
| scope-closure packet trail | `.runs/i7/**` | treat as frozen inputs, then append I8 evidence separately under `.runs/i8/**` |

## Final Rust V1 Claim

This is the exact plain-English line I8 is trying to close, derived from M65,
M66, the I7 freeze, and the current live repo state:

> Rust V1 is the current narrow-core `spec` surface: synchronous supported
> function families plus plain data and sum seams, proven by BENCH-ECOM and
> BENCH-SERVICE, with BENCH-CROSSLIB preserved as companion negative proof.

Derived consequences:

- supported:
  - synchronous supported function families
  - plain pipeline/wrapper composition inside the shipped supported families
  - plain data seams
  - plain sum seams
  - truthful proof surfaces over those supported rows
- deferred to `V1.1`:
  - bounded generics
  - async flows, runtime adapters, and IO-owned boundaries
- visible but non-crediting:
  - companion negative proof in `BENCH-CROSSLIB`

## Architecture And Execution Graph

The execution graph for I8 is intentionally thinner than I7:

```text
M65 ladder lock
      |
      v
M66 support rows + deferred boundaries
      |
      v
M67 benchmark role split
      |
      v
M68 read-side mechanics
      |
      v
I7 decision freeze + I8 handoff
      |
      v
I8 preflight freeze (.runs/i8/)
      |
      +------------------------------+
      |                              |
      v                              v
positive wall rerun            repo-root inventory check
(ECOM + SERVICE)               (inventory_only + CROSSLIB)
      |                              |
      +---------------+--------------+
                      |
                      v
          authority drift comparison
                      |
                      v
               final closeout packet
                      |
                      v
         truthful Rust V1 done-state claim
```

Critical dependency rule:

- the positive benchmark reruns and the broad inventory rerun may happen in
  parallel
- authority updates may not happen until those outputs are captured and
  compared against the frozen claim

## Work Phases

| Phase | Goal | Primary outputs | Exit criteria |
| --- | --- | --- | --- |
| 1. Preflight freeze | create a reproducible I8 run root and cite frozen inputs | `.runs/i8/preflight.json`, copied authority references, command roster | every later step cites frozen inputs instead of memory |
| 2. Positive proof rerun | rerun the two positive benchmark walls unchanged | raw ecommerce/service status+export outputs, summarized proof ledger | both positive benchmarks still pass with satisfied gates and current readability |
| 3. Broad inventory confirmation | rerun repo-root inventory and confirm companion-negative visibility | raw workspace status output, inventory summary | `scope_authority: inventory_only` is preserved and `BENCH-CROSSLIB` remains visible but zero-credit |
| 4. Authority drift ratification | compare live outputs against docs, labels, decisions, and handoff wording | doc diff only if drift exists | all checked-in authority surfaces teach one identical I8 story |
| 5. Final closeout | freeze the final evidence packet and milestone verdict | `.runs/i8/closeout.json`, release-note updates if needed | Rust V1 can be stated honestly with no extra caveats |

## Proof And Coverage Diagram

This is the I8 proof surface map. Every row below is part of the milestone and
must be captured in the closeout record.

```text
PROOF COMMANDS                                            CLAIM SURFACE
[PASS] cargo run -p spec-cli -- status examples/ecommerce/units --format json
  -> BENCH-ECOM positive wall
  -> benchmark_status: passing
  -> gate_status: satisfied
  -> readability_review_status: current

[PASS] cargo run -p spec-cli -- export examples/ecommerce/units
  -> schema_version: 4 full projection
  -> BENCH-ECOM roster, cases, molecule proofs, readability anchor visible

[PASS] cargo run -p spec-cli -- status examples/service/units --format json
  -> BENCH-SERVICE positive wall
  -> benchmark_status: passing
  -> gate_status: satisfied
  -> readability_review_status: current

[PASS] cargo run -p spec-cli -- export examples/service/units
  -> schema_version: 4 full projection
  -> BENCH-SERVICE roster, cases, molecule proofs, readability anchor visible

[EXPECTED EXIT 1] cargo run -p spec-cli -- status . --format json
  -> scope_authority: inventory_only
  -> BENCH-CROSSLIB visible as companion_negative_proof
  -> BENCH-ECOM and BENCH-SERVICE still passing in broad projection
  -> non-green fixture/semantic-family roots stay inventory-only, not proof failure

COVERAGE: 5/5 commands observed on dc61b01
CLAIM PATHS: 5/5 covered
GAPS: 0 proof-command gaps
PROCEDURAL REQUIREMENT: archive raw outputs under .runs/i8/ so the final claim stays replayable
```

Interpretation guard:

- A green I8 does not mean repo-root `status .` exits `0`.
- A green I8 means the two positive benchmark walls pass, the broad inventory
  surface still tells the truth, and the authority docs match that reality.

## Acceptance Checklist

I8 closes only when all of these are true at the same time:

- the plain-English Rust V1 claim in this file still matches:
  - `M65` provisional claim shape
  - `M66` supported/deferred boundaries
  - `.runs/i7/decision-freeze.json`
  - `.runs/i7/i8-handoff.json`
- `BENCH-ECOM` remains `passing` with `gate_status: satisfied`
- `BENCH-SERVICE` remains `passing` with `gate_status: satisfied`
- both positive benchmark exports still emit `schema_version: 4` full
  projections
- `BENCH-CROSSLIB` remains visible in repo-root inventory and still counts as
  zero positive supported credit
- repo-root `status . --format json` still reports `scope_authority:
  inventory_only`
- no new proof commands were added to make the claim work
- no deferred `V1.1` surface was silently promoted
- no checked-in doc implies a new post-I8 discovery milestone
- `.runs/i8/closeout.json` records the exact final verdict and raw command refs

## Worktree Parallelization Strategy

I8 has two independent verification workstreams and one dependent ratification
workstream. That means limited parallelization is useful.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| preflight freeze | `.runs/i8/`, top-level authority docs as read-only inputs | — |
| positive proof rerun | `examples/ecommerce/`, `examples/service/`, `benchmarks/`, `.runs/i8/evidence/` | preflight freeze |
| broad inventory confirmation | repo root inventory surfaces, `examples/crosslib-app/`, `semantic-families/`, `benchmarks/`, `.runs/i8/evidence/` | preflight freeze |
| authority drift ratification | `PLAN.md`, `ORCH_PLAN.md`, `docs/`, `README.md`, `DECISIONS.md`, `CHANGELOG.md`, `TODOS.md` | positive proof rerun, broad inventory confirmation |
| conditional blocker repair | only the exact modules implicated by a failing command | whichever verification step found the blocker |
| final closeout | `.runs/i8/`, release-note surfaces if changed | authority drift ratification, conditional blocker repair if needed |

### Parallel lanes

- `Lane A`: positive proof rerun
  - sequential within lane
  - owns only `.runs/i8/evidence/ecommerce-*` and `.runs/i8/evidence/service-*`
- `Lane B`: broad inventory confirmation
  - sequential within lane
  - owns only `.runs/i8/evidence/workspace-*` and the inventory summary
- `Lane C`: authority drift ratification
  - must wait for Lane A and Lane B
  - owns checked-in authority docs only
- `Lane D`: conditional blocker repair
  - exists only if Lane A or Lane B finds a real truth regression
  - must stay bounded to the direct failing surface

### Execution order

1. Parent creates `.runs/i8/` and freezes inputs.
2. Launch `Lane A` and `Lane B` in parallel.
3. Parent compares both outputs against the frozen claim.
4. Launch `Lane C` only if the claim and outputs are now fully understood.
5. Launch `Lane D` only if a real blocker must be repaired before closeout.
6. Parent writes the final I8 closeout packet.

### Conflict flags

- `Lane A` and `Lane B` must not both write the same summary file under
  `.runs/i8/`. Split evidence files by benchmark versus workspace.
- `Lane C` must not start before the workspace inventory interpretation is
  settled. Otherwise it risks ratifying the wrong meaning of repo-root `status`.
- `Lane D` must not turn a local truth bug into a broad mechanics rewrite. If
  the repair touches benchmark schema, proof writers, or support rows beyond the
  failing surface, stop and escalate.

## Failure Modes

| Failure mode | Consequence | Guard in this plan |
| --- | --- | --- |
| repo-root `status .` exit `1` is misread as I8 failure | a truthful broad inventory surface blocks the release for the wrong reason | require `scope_authority: inventory_only` and benchmark roster interpretation, not exit-code-only reasoning |
| `BENCH-CROSSLIB` starts counting as positive credit | the V1 claim widens by read-surface drift | verify companion-negative classification and zero-credit behavior in repo-root inventory |
| docs still describe I7 as current or imply I9 | the milestone ladder becomes ambiguous again | ratify all repo-facing authority surfaces only after live output comparison |
| positive status passes but export drifts | downstream machine consumers see a different truth than human readers | require both `status` and `export` for each positive benchmark |
| readability anchors go stale while benchmark status stays green | the claim overstates reviewability of emitted Rust | require `readability_review_status: current` for both positive benchmarks |
| a blocker fix adds new proof commands or support rows | I8 quietly becomes another scope milestone | hard-stop any repair that changes the five-command wall or deferred boundaries |
| broad inventory roots are trimmed to force green | the repo loses honest visibility into deferred and fixture surfaces | preserve the current repo-root inventory behavior exactly |

Critical gap test:

- if any positive benchmark loses `passing` or `satisfied`, I8 is not done
- if any doc still needs caveats not present in the frozen claim, I8 is not done
- if the only way to "pass" is to reinterpret `inventory_only` as proof, I8 is
  not done

## Performance And Operational Review

I8 should not introduce any new runtime or infrastructure cost. The only
meaningful operational concerns are:

- broad repo-root `status` is a heavier scan than benchmark-root commands, so do
  not rerun it unnecessarily during closeout
- full-scope `status` and `export` must be captured as raw outputs once per
  closeout run so later doc review does not depend on rerunning commands by
  memory
- no new caching, queueing, parallel proof writer, or benchmark registry work is
  justified here

Boring-by-default rule:

- if the wall passes, ship the closeout with evidence
- if the wall fails, repair the narrow blocker only

## Implementation Tasks

- [ ] **T1 (P1, human: ~15m / CC: ~5m)** — preflight freeze — create `.runs/i8/`
  and record the frozen inputs, branch, commit, and exact five-command wall.
  - Verify: `.runs/i8/preflight.json` names the same authority inputs listed in this plan.
- [ ] **T2 (P1, human: ~15m / CC: ~5m)** — BENCH-ECOM proof rerun — rerun the
  ecommerce `status` and `export` commands and archive raw outputs.
  - Verify: `BENCH-ECOM` stays `passing/satisfied/current` and the export remains `schema_version: 4`.
- [ ] **T3 (P1, human: ~15m / CC: ~5m)** — BENCH-SERVICE proof rerun — rerun the
  service `status` and `export` commands and archive raw outputs.
  - Verify: `BENCH-SERVICE` stays `passing/satisfied/current` and the export remains `schema_version: 4`.
- [ ] **T4 (P1, human: ~15m / CC: ~5m)** — broad inventory confirmation — rerun
  repo-root `status . --format json`, archive it, and confirm the expected
  `inventory_only` interpretation plus companion-negative visibility.
  - Verify: `BENCH-CROSSLIB` remains visible with zero positive credit and repo-root `status` still exits non-green by design.
- [ ] **T5 (P1, human: ~30m / CC: ~10m)** — authority drift ratification —
  compare the live outputs against `PLAN.md`, `ORCH_PLAN.md`,
  `docs/rust_v1_contract_stack.md`, `README.md`, `DECISIONS.md`, `CHANGELOG.md`,
  and `TODOS.md`, then patch only real drift.
  - Verify: every checked-in authority surface teaches the same I8 story with no implied `I9`.
- [ ] **T6 (P1, human: ~20m / CC: ~10m)** — final closeout packet — write
  `.runs/i8/closeout.json` with the final claim, deferred surfaces, proof-wall
  verdicts, and references to the raw command outputs.
  - Verify: a future maintainer can reconstruct the exact I8 decision from `.runs/i8/` without rerunning the analysis conversation.
- [ ] **T7 (P1, human: ~variable / CC: ~variable)** — conditional blocker repair
  — if any proof command or authority surface contradicts the frozen claim, fix
  the direct blocker only and rerun the affected command subset plus the final
  five-command wall.
  - Verify: no repair widens support, changes benchmark roles, or adds new proof commands.

## NOT in scope

- bounded generics admission
  - rationale: I7 already deferred this to `V1.1`
- async/IO admission
  - rationale: I7 already froze Rust V1 as synchronous-only
- benchmark schema redesign
  - rationale: M67 and M68 already own benchmark roles and mechanics
- new benchmark roots or new benchmark kinds
  - rationale: the I8 claim closes over the existing three-benchmark roster
- repo-root `export .` support
  - rationale: this workspace shape still truthfully rejects aggregate export
- turning repo-root `status .` into a globally green ship gate
  - rationale: broad inventory is intentionally broader than the positive proof wall
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
claim still holds.
