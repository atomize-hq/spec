<!-- /autoplan restore point: /home/azureuser/.gstack/projects/atomize-hq-spec/codex-i5-support-core-autoplan-restore-20260520-000000.md -->
# I5: Rust V1 Supported-Core Closure Plan

Status: **authoritative implementation plan**  
Iteration: **I5**  
Milestone family: **Rust V1 supported-core closure**  
Implementation readiness: **ready for implementation**  
Plan scope: **close the currently shipped M66 Rust V1 supported core against the frozen M67/M68 benchmark wall, without implementing `BENCH-SERVICE`, widening support rows, or reopening the I3.5/I4 command-scope contract**  
Base branch: **main**  
Working branch: **main**  
Validated at commit: **`1dbff70`**  
Last rewritten: **2026-05-20**

Supersedes:

- the prior `I4: Rust V1 Command-Wall Fixture and Contract-Test Hardening Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- live repo truth run on `main` at `1dbff70`:
  - `cargo run -p spec-cli -- status . --format json`
  - `git branch --show-current`
  - `git rev-parse --short HEAD`

Historical context, not authority:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `ORCH_PLAN.md`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`

Primary repo surfaces:

- `benchmarks/labels.json`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- `examples/ecommerce/units/pricing/pricing_quote.unit.spec`
- `examples/ecommerce/units/pricing/discount_strategy.unit.spec`
- `examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec`
- `examples/crosslib-app/units/**`
- `spec-core/src/benchmark.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

## Executive Summary

I3 landed benchmark mechanics.

I3.5 froze which command surfaces are authoritative.

I4 locked that command wall behind fixtures.

I5 is the next step: convert the already shipped Rust V1 supported-core claim
from an implied story into one explicit closure contract that a maintainer can
run, inspect, and trust.

The live repo already proves most of the story. The problem is that the proof is
still split across separate surfaces:

- `BENCH-ECOM` passes, but its required molecule gate still omits the seam
  business flow that already has evidence
- `BENCH-CROSSLIB` is visible, but two active companion-negative cases are still
  untested
- `BENCH-ECOM` readability review exists on disk, but it is no longer current
  against the live projection digest
- explicit supported-boundary rejections exist in pieces, but I5 has not yet
  frozen one deliberate regression wall for them

I5 closes exactly those gaps and nothing broader.

## Current Validated Truth

Observed on `main` at `1dbff70`:

- `cargo run -p spec-cli -- status . --format json` reports
  `schema_version: 4` and `scope_authority: "inventory_only"`
- `BENCH-ECOM` currently reports:
  - `benchmark_status: "passing"`
  - `gate_status: "satisfied"`
  - `required_for_v1: true`
  - `supported_cases: 7`
  - `supported_valid_cases: 7`
  - `positive_credit_cases: 7`
  - `readability_review_status: "missing"`
- `BENCH-ECOM.required_molecule_ids` currently includes only:
  - `pricing/checkout_flow`
  - `pricing/discount_plus_tax`
- `examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec`
  already exists, and the resulting evidence is already referenced by supported
  case proof refs, but it is not yet part of the required benchmark gate
- `BENCH-CROSSLIB` currently reports:
  - `kind: "companion_negative_proof"`
  - `benchmark_status: "incomplete"`
  - `case_status_counts.untested: 2`
  - the untested active cases are:
    - `pricing/calculate_total`
    - `pricing/checkout_nested_chain3`
- `BENCH-SERVICE` currently reports:
  - `lifecycle: "reserved"`
  - `benchmark_status: "reserved"`
  - `gate_status: "reserved"`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json` exists, but its
  stored projection digest no longer matches the live benchmark projection, so
  the read-side surface truthfully treats the review as missing/currently
  unusable

That means the supported core is not missing. It is partially closed and not yet
locked as one deliberate, maintainable contract.

## Step 0: Scope Challenge

### Premise correction

The problem is not "add more Rust V1 support."

The problem is:

```text
turn the already shipped Rust V1 supported core into one explicit
benchmark + molecule + readability + rejection closure wall
```

If implementation expands beyond that sentence, it has escaped the milestone.

### Scope verdict

The complete version is still cheap here.

I5 does not require a new subsystem, new artifact family, new benchmark roster,
or service workload authoring. It requires tightening the existing benchmark
registry, proof artifacts, regression fixtures, and closeout docs until they
tell one consistent story.

Scope is accepted as-is.

### Complexity check

Expected write scope:

- benchmark registry and snapshot/review artifacts under `benchmarks/`
- proof freshness work in `examples/ecommerce/units/**` and
  `examples/crosslib-app/units/**`
- one dedicated Rust V1 closure regression surface under `spec-cli/tests/`
- targeted `spec-core` or `spec-cli` edits only if a promised closure rule is
  not yet enforced by the current implementation
- `TODOS.md` closeout wording

If implementation starts redesigning projection schemas, repo-root semantics,
service workloads, or general Rust support admission, stop. That is different
scope.

## What Already Exists

| Sub-problem | Existing owner | I5 action |
| --- | --- | --- |
| positive benchmark accounting | `benchmarks/labels.json`, `spec-core/src/benchmark.rs` | reuse and tighten |
| benchmark-root status/export truth | `spec-cli/src/commands.rs`, `spec-core/src/export.rs` | preserve, add closure assertions only |
| supported function and seam cases | `BENCH-ECOM`, `examples/ecommerce/units/**` | reuse as the supported-core baseline |
| seam molecule evidence | `pricing/discount_strategy_checkout_flow.test.spec` and its evidence artifact | promote into required benchmark gate |
| companion-negative visibility | `BENCH-CROSSLIB` | complete and regression-protect |
| readability review surface | `benchmarks/reviews/BENCH-ECOM.readability.review.json` | refresh and lock currentness |
| reserved benchmark visibility | `BENCH-SERVICE` projection and snapshots | preserve exactly as reserved |
| I3.5/I4 command wall | `spec-cli/tests/cli.rs` and benchmark fixtures | preserve unchanged |

## Frozen Decisions

These are locked. I5 implements them and does not reopen them.

1. **I5 is the first implementation slice of `M69 supported-core closure`.**
   - `M65` owns that milestone identity.
   - `TODOS.md` currently uses stale shorthand and must be updated during
     closeout.

2. **I5 does not implement `BENCH-SERVICE`.**
   - `BENCH-SERVICE` stays `kind: positive`, `lifecycle: reserved`,
     `required_for_v1: true`.
   - I5 may preserve or refresh reserved visibility only.
   - I5 may not author `examples/service/**`.

3. **I5 does not widen Rust V1 support rows or interaction claims.**
   - `ROW-GENERIC-BOUNDED` stays deferred.
   - `ROW-ASYNC-IO` stays deferred.
   - no new semantic-family promotions are part of I5.

4. **I5 closes only the already shipped supported narrow core.**
   - `ROW-FN-CORE`
   - `ROW-PIPE-CORE`
   - `ROW-DATA-PLAIN`
   - `ROW-SUM-PLAIN`
   - `INT-FN-PIPE`
   - `INT-LOCAL-CLOSURE`
   - `INT-SEAM-BUSINESS`
   - `INT-PROOF-COVERAGE`

5. **I5 must make every in-scope claim map to one deliberate proof owner.**
   - if a row or interaction is in scope, it gets one named benchmark or
     artifact owner
   - if a rejection boundary is in scope, it gets one named fixture owner
   - if something cannot be mapped honestly, it stays out of I5

6. **I5 reuses the M68 writer/reader boundary.**
   - `benchmarks/labels.json` remains the authored benchmark-accounting source
   - passports and molecule evidence remain the proof writers
   - `spec status`, `spec export`, and `spec benchmark snapshot` remain derived
     or read-side surfaces only

7. **I5 treats readability as a closure gate, not garnish.**
   - readability remains benchmark-scoped and human-authored
   - readability never mutates support classification
   - for `BENCH-ECOM`, currentness must be deliberate and regression-protected

8. **I5 must finish the active companion-negative wall.**
   - active `BENCH-CROSSLIB` cases may not remain untested
   - companion-negative cases remain visible but never count as positive credit

9. **I5 inherits the I3.5/I4 command wall unchanged.**
   - benchmark-root proof commands remain authoritative
   - repo-root `status . --format json` remains `inventory_only`
   - repo-root `export .` remains unsupported for this workspace shape

10. **Minimal diff still wins.**
    - prefer benchmark-label, proof-refresh, and regression-harness tightening
      over product refactors
    - code changes outside those surfaces are allowed only when necessary to
      make the current M66 wall truthful and stable

## Closure Matrix

This is the authoritative I5 claim map. If a claim is not mapped here, it is
not part of the milestone.

| M66 contract item | I5 proof owner | Required proof surface |
| --- | --- | --- |
| `ROW-FN-CORE` | `BENCH-ECOM` function cases: `money/round`, `pricing/apply_discount`, `pricing/apply_tax` | benchmark-root `status` + `export` + fresh passports |
| `ROW-PIPE-CORE` | `pricing/calculate_total`, `pricing/calculate_total_guarded_tax` inside `BENCH-ECOM` | benchmark-root `status` + `export` + fixture assertions |
| `ROW-DATA-PLAIN` | `pricing/pricing_quote` | fresh passport + required seam molecule proof |
| `ROW-SUM-PLAIN` | `pricing/discount_strategy` | fresh passport + required seam molecule proof |
| `INT-FN-PIPE` | `pricing/checkout_flow` and `pricing/discount_plus_tax` | required benchmark molecule proofs |
| `INT-LOCAL-CLOSURE` | `BENCH-ECOM` same-tree positive closure and `BENCH-CROSSLIB` same-tree companion-negative closure | benchmark-root projections + no-positive-credit assertions |
| `INT-SEAM-BUSINESS` | `pricing/discount_strategy_checkout_flow` | required benchmark molecule proof in `BENCH-ECOM.required_molecule_ids` |
| `INT-PROOF-COVERAGE` | passports, molecule evidence, benchmark status/export, benchmark snapshots | fresh proof artifacts + stable read-side projections |
| readable supported-core closure | `benchmarks/reviews/BENCH-ECOM.readability.review.json` | current projection digest + current generated-file set |
| reserved future workload visibility | `BENCH-SERVICE` snapshot/projection | reserved-only read-side visibility |

Deferred and intentionally outside I5:

- `ROW-GENERIC-BOUNDED`
- `ROW-ASYNC-IO`
- `BENCH-SERVICE` authored workload
- any new positive benchmark case outside the current `BENCH-ECOM` roster

## Architecture

```text
                    M66 SUPPORTED-CORE CLAIM
                               |
         +---------------------+----------------------+
         |                                            |
         v                                            v
   positive benchmark wall                    explicit rejection wall
         |                                            |
         v                                            v
      BENCH-ECOM                             closure regression fixtures
         |
         +--------------------+--------------------+
         |                    |                    |
         v                    v                    v
   supported cases      required molecules   readability review
         |                    |                    |
         v                    v                    v
     passports          molecule evidence   current digest + file set
         |                    |                    |
         +--------------------+----------+---------+
                                         |
                                         v
                            benchmark projection core
                                         |
                     +-------------------+-------------------+
                     |                                       |
                     v                                       v
            benchmark-root status/export          benchmark snapshot artifacts
                     |
                     v
                         I5 closure is truthful

    BENCH-CROSSLIB runs beside this wall as active companion-negative proof,
    never as positive credit.

    BENCH-SERVICE stays reserved and visible, but outside implementation scope.
```

## Target Outcome

I5 is done only when all of these are true at the same time:

1. `BENCH-ECOM` still passes as the active positive benchmark.
2. `BENCH-ECOM.required_molecule_ids` explicitly includes
   `pricing/discount_strategy_checkout_flow`.
3. `BENCH-ECOM` reports a current readability review again.
4. `BENCH-CROSSLIB` no longer has active untested cases.
5. companion-negative cases still contribute zero positive credit.
6. every in-scope supported row and interaction in the closure matrix has one
   explicit proof owner.
7. the supported-boundary rejection wall is frozen behind one deliberate
   regression suite instead of being implied by scattered tests.
8. repo-root inventory semantics remain unchanged.
9. `BENCH-SERVICE` remains reserved and clearly unimplemented.

## Implementation Contract

Implementation is four phases. Phases 1-3 do the real closure work. Phase 4 is
the final refresh and closeout pass after the earlier phases are frozen.

### Phase 1: Positive benchmark closure

Objective:

- make the positive supported-core benchmark explicitly require the seam
  business-flow proof it already depends on in practice

Primary write scope:

- `benchmarks/labels.json`
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `spec-cli/tests/fixtures/benchmarks/**`
- `spec-cli/tests/rust_v1_closure.rs` or equivalent dedicated closure suite

Required changes:

- add `pricing/discount_strategy_checkout_flow` to
  `BENCH-ECOM.required_molecule_ids`
- keep the current `BENCH-ECOM` supported case roster unless implementation
  proves one case is mislabeled
- add regression coverage that `BENCH-ECOM` becomes non-passing if that seam
  molecule proof is missing, stale, or failing
- refresh the benchmark snapshot and any benchmark fixture JSON that legitimately
  changes once the gate is frozen

Done when:

- the seam molecule is part of the required gate on purpose, not only via case
  proof refs
- `BENCH-ECOM` still passes with the expanded required-molecule set

### Phase 2: Companion-negative closure

Objective:

- finish the active negative-proof wall so it becomes truthful, current, and
  regression-protected

Primary write scope:

- `examples/crosslib-app/units/**`
- `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- `spec-cli/tests/fixtures/benchmarks/**`
- `spec-cli/tests/rust_v1_closure.rs` or equivalent dedicated closure suite

Required changes:

- refresh proof for the two currently untested active `BENCH-CROSSLIB` cases:
  - `pricing/calculate_total`
  - `pricing/checkout_nested_chain3`
- if either case should not be active companion proof, remove it deliberately
  and explain why in the benchmark-label diff; do not leave it half-active
- add regression coverage that an active companion-negative case without current
  proof makes the benchmark `incomplete`
- add regression coverage that companion-negative cases never increment
  `positive_credit_cases`, even when the carrier unit is `valid`

Done when:

- `BENCH-CROSSLIB` is complete as an active companion-negative benchmark
- its summary still shows `positive_credit_cases: 0`

### Phase 3: Supported-boundary rejection wall

Objective:

- freeze the current detectable supported-boundary rejections for the shipped
  Rust V1 core behind one deliberate closure suite

Primary write scope:

- `spec-cli/tests/rust_v1_closure.rs` or equivalent dedicated closure suite
- `spec-cli/tests/fixtures/**`
- `spec-core/src/validator.rs`
- `spec-core/src/benchmark.rs`
- `spec-cli/src/commands.rs`

Required changes:

- inventory the exact already-detectable closure boundaries that I5 is claiming
  for the shipped supported core
- prefer reusing existing fixture families and unsupported-near-miss packs where
  that already matches the supported-core boundary being frozen
- add one stable regression assertion per in-scope rejection boundary so the
  failure is machine-visible and cannot silently drift into apparent success
- only change implementation code if the current surface does not already emit a
  stable and truthful early rejection

Hard rule:

- I5 does not invent a brand-new rejection taxonomy here
- I5 freezes the already detectable boundary wall for the shipped supported core
- if a category cannot be mapped to one concrete existing or newly-added
  fixture and one stable observable failure contract, it stays out of I5

Done when:

- the closure suite makes the supported-boundary wall observable and
  intentionally maintained

### Phase 4: Readability currentness and closeout

Objective:

- make the read-side closure story current, final, and maintainable after
  Phases 1-3 are settled

Primary write scope:

- `benchmarks/reviews/BENCH-ECOM.readability.review.json`
- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- `spec-cli/tests/fixtures/benchmarks/**`
- `TODOS.md`

Required changes:

- refresh `BENCH-ECOM` readability review against the final live projection
  digest and final `readability_generated_files` set
- add regression coverage for current versus missing/stale readability review
  state
- refresh only the impacted benchmark snapshots after label, molecule, proof,
  and rejection work are final
- update the `TODOS.md` `M69` wording so it matches this closure milestone
  rather than the stale mechanics-expansion shorthand

Hard rule:

- snapshot and review refresh is the final lane
- do not refresh snapshots while benchmark labels or required-molecule
  definitions are still changing

Done when:

- read-side surfaces and closeout docs tell the same story as the benchmark
  registry and proof artifacts

## Implementation Tasks

### Phase 1 tasks

- [ ] Add `pricing/discount_strategy_checkout_flow` to
      `BENCH-ECOM.required_molecule_ids` in `benchmarks/labels.json`.
- [ ] Refresh `BENCH-ECOM` benchmark assertions/fixtures to reflect the expanded
      required molecule set.
- [ ] Add a regression that removing or staling that molecule proof demotes the
      benchmark.

### Phase 2 tasks

- [ ] Refresh proof for `examples/crosslib-app/units/pricing/calculate_total.unit.spec`.
- [ ] Refresh proof for
      `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`.
- [ ] Add regression coverage that an active companion-negative case without
      proof yields `benchmark_status: "incomplete"`.
- [ ] Add regression coverage that companion-negative cases keep
      `positive_credit_cases: 0`.

### Phase 3 tasks

- [ ] Create or extend one dedicated Rust V1 closure suite under `spec-cli/tests/`.
- [ ] Freeze the exact in-scope supported-boundary rejection fixtures for the
      shipped supported core.
- [ ] Assert that each in-scope closure boundary fails via a stable,
      machine-visible contract.
- [ ] Touch `spec-core` or `spec-cli` implementation only where current
      enforcement is not yet truthful or stable.

### Phase 4 tasks

- [ ] Refresh `benchmarks/reviews/BENCH-ECOM.readability.review.json` against
      the final projection digest.
- [ ] Refresh impacted benchmark snapshots only after Phases 1-3 are merged.
- [ ] Update benchmark fixture JSON to match the final closure state.
- [ ] Rewrite the `TODOS.md` `M69` item so it names supported-core closure
      directly.

## Acceptance Commands

The I5 proof wall is:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo run -p spec-cli -- export examples/crosslib-app/units
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
```

Targeted proof refresh commands:

```bash
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec
```

Regression wall:

```bash
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status . --format json
```

The last command is part of acceptance because I5 must preserve repo-root
inventory semantics while tightening benchmark-root proof surfaces.

## Test Strategy

### Coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] Positive benchmark closure
    |
    +- benchmarks/labels.json
    |   +- [CHANGE] add seam molecule to required_molecule_ids
    |   \- [TEST] benchmark gate demotes when required molecule proof is absent
    |
    +- spec-core/src/benchmark.rs projection
    |   +- [TEST] supported cases remain positive only when full gate is current
    |   \- [TEST] companion-negative cases remain visible but zero-credit
    |
    \- benchmark snapshot + export/status
        +- [TEST] BENCH-ECOM stays passing
        \- [TEST] BENCH-CROSSLIB stays complete and zero-credit

[+] Readability currentness
    |
    +- benchmarks/reviews/BENCH-ECOM.readability.review.json
    |   +- [CHANGE] refresh to final projection digest
    |   \- [TEST] current vs stale/missing review status
    |
    \- read-side benchmark surfaces
        \- [TEST] review status is current only when digest + file set match

[+] Supported-boundary rejection wall
    |
    +- dedicated closure fixture/test suite
    |   +- [CHANGE] freeze exact in-scope rejection fixtures
    |   \- [TEST] stable machine-visible rejection per frozen boundary
    |
    \- supported-core guardrail
        \- [TEST] rejected fixtures never appear as supported-core success

USER FLOW COVERAGE
===========================
[+] Maintainer closure flow
    |
    +- refresh ecommerce seam proof
    +- refresh crosslib companion proof
    +- run benchmark-root status/export/snapshot
    \- verify BENCH-ECOM current + BENCH-CROSSLIB complete + BENCH-SERVICE reserved

[+] Failure states
    |
    +- missing seam molecule obligation demotes BENCH-ECOM
    +- stale readability review surfaces as non-current
    +- active companion case without proof yields incomplete benchmark
    \- closure-boundary fixture fails early with stable contract
```

### Mandatory regression tests

1. `BENCH-ECOM` becomes non-passing if
   `pricing/discount_strategy_checkout_flow` is required but missing, stale, or
   failing.
2. `BENCH-ECOM` readability review reports current only when both
   `projection_digest` and `readability_generated_files` match the live
   projection.
3. `BENCH-CROSSLIB` becomes incomplete when an active companion-negative case
   lacks current proof.
4. `BENCH-CROSSLIB` companion-negative cases never increment
   `positive_credit_cases`, even when the carrier unit is `valid`.
5. repo-root `status . --format json` remains `scope_authority: "inventory_only"`
   after I5.
6. each in-scope supported-boundary rejection frozen by Phase 3 fails with a
   stable machine-visible contract.

### Test files

Preferred write shape:

- keep the broad I3.5/I4 command-wall fixtures in `spec-cli/tests/cli.rs`
- add one dedicated integration suite for I5 closure assertions, for example
  `spec-cli/tests/rust_v1_closure.rs`
- keep benchmark fixture JSON under `spec-cli/tests/fixtures/benchmarks/`
- keep any new adversarial closure fixtures under a dedicated Rust V1 closure
  subtree instead of mixing them into unrelated milestone packs

## Failure Modes Registry

| Failure mode | Why it matters | Coverage requirement | Critical gap? |
| --- | --- | --- | --- |
| seam support remains indirectly proven but not required by the benchmark gate | fake green supported-core closure | required-molecule regression around `discount_strategy_checkout_flow` | yes |
| readability review file exists but is not current against the live projection | maintainers think readability is reviewed when the read-side wall disagrees | current vs stale/missing review regression | yes |
| active companion-negative case stays untested | the negative-proof wall stops proving anything while still looking active | active-companion incomplete regression | yes |
| companion-negative cases accidentally count as positive credit | false Rust support claim | zero-credit benchmark-summary regression | yes |
| supported-boundary rejection remains scattered and unowned | closure relies on folklore instead of a maintained regression wall | dedicated closure fixture suite | yes |
| repo-root inventory semantics drift while closing benchmark gaps | I3.5/I4 contract regresses | repo-root scope-authority regression | yes |

## Performance / Complexity Guardrails

I5 should not introduce a meaningful runtime cost increase.

Guardrails:

- benchmark projection remains label-driven and linear in loaded benchmark cases
- no repo-wide discovery beyond already shipped root loading rules
- any new rejection check must operate on already parsed authored-entry surfaces,
  not whole-program Rust analysis
- snapshot and review refresh remain explicit artifact updates, not hidden side
  effects of `status` or `export`

If implementation wants new caches, new registries, or a second projection
subsystem, it is solving the wrong problem.

## Worktree Parallelization Strategy

This section is authoritative for parallel execution. It is intentionally stricter
than the earlier draft so we do not create benchmark-artifact merge churn while
trying to go faster.

### Lane ownership

| Lane | Owned write set | Allowed goal |
| --- | --- | --- |
| Lane A | `benchmarks/labels.json`, `examples/ecommerce/units/**`, `spec-cli/tests/fixtures/benchmarks/**` for `BENCH-ECOM`, `spec-cli/tests/rust_v1_closure.rs` benchmark-gate assertions | Phase 1 positive benchmark closure |
| Lane B | `examples/crosslib-app/units/**`, `spec-cli/tests/fixtures/benchmarks/**` for `BENCH-CROSSLIB`, `spec-cli/tests/rust_v1_closure.rs` companion-negative assertions | Phase 2 companion-negative closure |
| Lane C | `spec-core/src/validator.rs`, `spec-core/src/benchmark.rs`, `spec-cli/src/commands.rs`, dedicated closure fixtures/tests | Phase 3 supported-boundary rejection wall |
| Lane D | `benchmarks/reviews/BENCH-ECOM.readability.review.json`, impacted snapshot files, final benchmark fixtures, `TODOS.md` | Phase 4 final refresh and closeout |

### Non-negotiable ownership rules

- `benchmarks/labels.json` belongs to **Lane A only** during parallel work.
- Lane B may not edit `benchmarks/labels.json` in parallel. If companion-negative
  work discovers a needed label change, queue it for the post-merge integration
  pass instead of creating a parallel conflict.
- benchmark snapshots belong to **Lane D only**.
- readability review artifacts belong to **Lane D only**.
- if Lanes A and B both need the same test file, split assertions so Lane A owns
  benchmark-gate coverage and Lane B owns companion-negative coverage, then
  reconcile in the integration pass.

### Parallel lanes

- `Lane A`: Phase 1 positive benchmark closure
- `Lane B`: Phase 2 companion-negative closure
- `Lane C`: Phase 3 supported-boundary rejection wall
- `Lane D`: Phase 4 final refresh and closeout

### Execution order

1. Launch Lane A and Lane C in parallel.
2. Launch Lane B in parallel with A and C only if it respects the ownership rule
   that `benchmarks/labels.json` stays A-owned.
3. Merge A, B, and C.
4. Run Lane D after that merge, not before.
5. Refresh readability review and impacted snapshots only once the benchmark
   labels, proof artifacts, and rejection wall are frozen.

### Conflict flags

- `spec-cli/tests/rust_v1_closure.rs` is the highest-probability text conflict.
  Keep the suite sectioned by phase so integration is mechanical.
- benchmark fixture JSON can still conflict between Lanes A and B if both rewrite
  broad fixture files. Prefer narrower fixture files or an explicit integration
  pass instead of editing the same full-output fixture in parallel.
- any lane that starts refreshing snapshots early is wrong. Snapshot churn is a
  symptom that Lane D started too soon.

## What Success Looks Like

After I5:

- one maintainer can run the benchmark-root proof wall and see:
  - `BENCH-ECOM` passing with a current readability review
  - `BENCH-CROSSLIB` complete as active companion-negative proof
  - `BENCH-SERVICE` still reserved, visible, and clearly outside this milestone
- seam-shaped support is part of the benchmark gate on purpose, not by folklore
- the supported-boundary wall is backed by one deliberate closure suite
- the next milestone can debate bounded generics, async/IO, or service workload
  from a truthful supported-core baseline instead of archaeology

## NOT in scope

- `BENCH-SERVICE` authored workload or `examples/service/**`
- final V1 proof ratification
- bounded generics admission into V1
- async or IO admission into V1
- repo-root aggregate workspace redesign
- benchmark schema redesign or new artifact families
- broad TypeScript-lane work unrelated to Rust V1 supported-core closure

## Completion Summary

- Step 0: Scope Challenge, accepted as full supported-core closure without
  service-workload widening
- Architecture Review: one explicit closure wall now defined across benchmark,
  molecule, readability, and rejection surfaces
- Code Quality Review: exact file ownership and phase boundaries now frozen
- Test Review: coverage diagram written and six mandatory regression groups
  identified
- Performance Review: no new subsystem justified; keep projection label-driven
  and linear
- Failure modes: six closure-breaking regressions identified and assigned
- Parallelization: four lanes, three parallel-capable, one final sequential
  refresh lane
- Lake Score: 5/5, because the honest complete version is still cheaper than a
  partial story plus future archaeology
