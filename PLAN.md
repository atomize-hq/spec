# I6: Rust V1 Service Benchmark Activation Plan

Status: **authoritative implementation plan**
Iteration: **I6**
Milestone family: **Rust V1 service-shaped proof closure**
Implementation readiness: **ready for implementation**
Plan scope: **turn `BENCH-SERVICE` from a reserved benchmark into the active service-shaped proof workload required by `M67`, without widening `M66` support rows, reopening `M68` mechanics, or admitting async/IO, generics, traits, lifetimes, or framework-heavy authored surfaces**
Base branch: **main**
Working branch: **`codex/i6-service-benchmark-activation`**
Validated at commit: **`3185b49`**
Last rewritten: **2026-05-21**

Supersedes:

- the prior `I5: Rust V1 Supported-Core Closure Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- live repo truth run on `main` at `3185b49`:
  - `cargo run -p spec-cli -- status . --format json`
  - `git rev-parse --short HEAD`
  - `git branch --show-current`

Historical context, not authority:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/**`

Primary repo surfaces:

- `benchmarks/labels.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
- `examples/service/**` and `examples/service/units/**`
- `spec-core/src/benchmark.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`

## Executive Summary

I5 shipped the supported-core closure wall.

The live repo now has:

- `BENCH-ECOM` passing as the active positive narrow-core benchmark
- `BENCH-CROSSLIB` passing as the active companion-negative wall
- `BENCH-SERVICE` still present only as a reserved, zero-case, zero-review gate

That means the only open Rust V1 proof gap left in the M65-M68 stack is the one
that `M67` named explicitly: the repo still lacks a real service-shaped positive
benchmark.

I6 closes exactly that gap.

It does so by activating `BENCH-SERVICE`, authoring one real single-library
service workload under `examples/service/`, proving both happy-path and
business-failure-path flows inside the already shipped supported narrow core,
and refreshing the read-side benchmark truth until the repo can honestly say
that the current Rust V1 claim is backed by both example-domain proof and a
service-shaped proof workload.

## Current Validated Truth

Observed on `main` at `3185b49`:

- `cargo run -p spec-cli -- status . --format json` reports
  `schema_version: 4` and `scope_authority: "inventory_only"`
- `BENCH-ECOM` currently reports:
  - `lifecycle: "active"`
  - `benchmark_status: "passing"`
  - `gate_status: "satisfied"`
  - `readability_review_status: "current"`
  - `supported_valid_cases: 7`
  - `required_molecule_total: 3`
- `BENCH-CROSSLIB` currently reports:
  - `lifecycle: "active"`
  - `benchmark_status: "passing"`
  - `gate_status: "not_applicable"`
  - `positive_credit_cases: 0`
  - `case_status_counts.valid: 4`
- `BENCH-SERVICE` currently reports:
  - `lifecycle: "reserved"`
  - `accounting_status: "reserved_missing_cases"`
  - `benchmark_status: "reserved"`
  - `gate_status: "reserved"`
  - `readability_review_status: "missing"`
  - `total_cases: 0`
  - `required_molecule_total: 0`
- `benchmarks/labels.json` already declares the future service root as:
  - `root: "examples/service/units"`
  - `generated_root: "examples/service/src/generated"`
  - `readability_scope: "supported_closure"`
  - `cases: []`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json` already exists, but only
  as the reserved-state artifact
- there is currently no `examples/service/` directory in the repo
- service-shaped billing vocabulary already exists, but only inside test
  fixtures:
  - `billing/apply_membership_discount`
  - `billing/apply_regional_fee`
  - `billing/checkout_net_total`
- `docs/rust_v1_contract_stack.md` still teaches the ladder only through I4, so
  the ownership map is usable today but the in-repo milestone ladder needs I6
  closeout cleanup

That means the mechanics are already present.

The missing piece is one deliberate, benchmark-labeled, service-shaped authored
workload with current proof, current readability review, and stable regression
coverage.

## Step 0: Scope Challenge

### Premise correction

The problem is not "add broader Rust support."

The problem is:

```text
turn the reserved BENCH-SERVICE placeholder into one active,
single-library, service-shaped benchmark that proves the already-shipped
Rust V1 narrow core on a real workflow
```

If implementation expands beyond that sentence, it has escaped the milestone.

### Scope verdict

The complete version is still cheap here.

I6 does not need:

- a new benchmark subsystem
- new benchmark schemas
- cross-library service proof
- async or IO semantics
- new semantic-family promotion
- trait, lifetime, or macro-heavy authored support

It needs one truthful service benchmark and the read-side regression wall that
keeps it honest.

Scope is accepted as-is.

### Failure-path definition for I6

`M67` requires a service-shaped failure-path workflow.

For I6, that means a business-path unhappy flow that stays inside the current
supported narrow core, such as:

- a declined or absent discount path represented through the plain sum seam
- a guarded fee path that clamps invalid business input into the supported
  non-negative surface

It does **not** mean:

- network failure
- database failure
- async cancellation
- exception-style service orchestration

Those remain outside `M66` narrow-core support and outside I6 scope.

### Complexity check

Expected write scope:

- `examples/service/**` example scaffold and source specs
- `benchmarks/labels.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`
- targeted `spec-core` / `spec-cli` updates only if the active service benchmark
  exposes a truthful-projection gap
- `README.md`, `docs/rust_v1_contract_stack.md`, `TODOS.md`, and
  `CHANGELOG.md` during closeout

If implementation starts redesigning benchmark projection, widening support
rows, or re-arguing M66/M67/M68, stop. That is different scope.

## What Already Exists

| Sub-problem | Existing owner | I6 action |
| --- | --- | --- |
| positive benchmark mechanics | `benchmarks/labels.json`, `spec-core/src/benchmark.rs`, `spec-cli/src/commands.rs` | reuse and activate `BENCH-SERVICE` |
| benchmark-root truth surfaces | `spec status`, `spec export`, `spec benchmark snapshot` | preserve contract, add service-root coverage |
| supported-core example proof | `BENCH-ECOM` | reuse as the pattern for happy-path benchmark wiring |
| companion-negative wall | `BENCH-CROSSLIB` | preserve unchanged, assert no regression |
| readability review mechanics | `benchmarks/reviews/BENCH-ECOM.readability.review.json` | reuse the same digest/file-set rule for `BENCH-SERVICE` |
| seed billing function vocabulary | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/**` | promote the positive shapes into `examples/service/units/billing/**` |
| plain seam examples | `examples/ecommerce/units/pricing/discount_strategy.unit.spec`, `examples/ecommerce/units/pricing/pricing_quote.unit.spec` | adapt the same supported seam shape into the service domain |
| I5 closure regression wall | `spec-cli/tests/rust_v1_closure.rs` | preserve for ECOM/CROSSLIB, add separate service benchmark suite |
| repo-root inventory semantics | I3.5/I4 fixture wall | preserve unchanged while `BENCH-SERVICE` goes active |

## Frozen Decisions

These are locked. I6 implements them and does not reopen them.

1. **I6 activates `BENCH-SERVICE`.**
   - The milestone is not complete while `BENCH-SERVICE` remains
     `lifecycle: "reserved"`.
   - The benchmark must become an active positive benchmark with real cases,
     real required molecules, current proof, and a current readability review.

2. **I6 does not widen the Rust V1 support contract.**
   - `ROW-GENERIC-BOUNDED` stays deferred.
   - `ROW-ASYNC-IO` stays deferred.
   - no trait-authored, lifetime-heavy, or macro/framework-heavy authored
     surfaces enter the benchmark
   - no new support rows or interaction claims are admitted

3. **I6 stays single-library.**
   - `BENCH-SERVICE` lands under `examples/service/**`
   - no `[libraries]` graph or sibling-library dependency is part of the
     service benchmark
   - the benchmark proves service shape, not cross-library shape

4. **I6 uses only already-supported families and seam kinds.**
   - supported function families already shipped in the repo are allowed
   - plain `kind:data` and `kind:sum` seams are allowed
   - unsupported-function or unsupported-seam cases stay test-fixture-only,
     not positive benchmark cases

5. **The initial positive service roster is fixed in this plan.**
   - `billing/apply_membership_discount`
   - `billing/apply_regional_fee`
   - `billing/checkout_net_total`
   - `billing/checkout_net_total_guarded_fee`
   - `billing/discount_strategy`
   - `billing/pricing_quote`

6. **The initial required molecule roster is fixed in this plan.**
   - `billing/checkout_success_flow`
   - `billing/checkout_declined_discount_flow`
   - `billing/discount_strategy_quote_flow`

7. **`BENCH-SERVICE` continues to use the M68 writer/reader wall.**
   - source specs remain authored truth
   - passports and molecule evidence remain proof writers
   - benchmark labels remain benchmark-accounting truth
   - readability review remains human-authored observation truth
   - `status`, `export`, and `benchmark snapshot` remain read-side projections

8. **A service benchmark that requires widened support is a failed benchmark, not a widened contract.**
   - if any proposed positive service case cannot stay inside the current
     supported rows and interactions, replace the case with another in-family
     service case
   - do not widen M66 to rescue the benchmark

9. **`BENCH-ECOM` and `BENCH-CROSSLIB` stay green and truthful throughout I6.**
   - the service landing may refresh shared fixtures only where the benchmark
     roster legitimately changes
   - it may not regress the already-shipped benchmark wall

10. **Minimal diff still wins.**
    - prefer promoting the existing billing fixture vocabulary plus adapting the
      shipped seam patterns
    - only touch projection code when the active service benchmark exposes a
      real truth gap

11. **I6 executes on a dedicated non-`main` branch.**
    - implementation starts from `main` at `3185b49` or its direct descendant
    - implementation work happens on
      `codex/i6-service-benchmark-activation`
    - do not land incremental service-benchmark edits directly on `main`
    - refresh validation truth on the working branch before declaring I6 done

## Intended Service Workload

I6 authors exactly this service example scaffold:

- `examples/service/Cargo.toml`
- `examples/service/README.md`
- `examples/service/src/main.rs`
- `examples/service/units/.gitignore`
- `examples/service/units/billing/apply_membership_discount.unit.spec`
- `examples/service/units/billing/apply_regional_fee.unit.spec`
- `examples/service/units/billing/checkout_net_total.unit.spec`
- `examples/service/units/billing/checkout_net_total_guarded_fee.unit.spec`
- `examples/service/units/billing/discount_strategy.unit.spec`
- `examples/service/units/billing/pricing_quote.unit.spec`
- `examples/service/units/billing/checkout_success_flow.test.spec`
- `examples/service/units/billing/checkout_declined_discount_flow.test.spec`
- `examples/service/units/billing/discount_strategy_quote_flow.test.spec`

Concrete authored intent for each positive case:

- `billing/apply_membership_discount`
  - promoted from the M19 billing fixture pack
  - monotone-down nonnegative leaf
- `billing/apply_regional_fee`
  - promoted from the M19 billing fixture pack
  - monotone-up leaf
- `billing/checkout_net_total`
  - promoted from the M19 billing fixture pack
  - wrapper pipeline over discount then fee
- `billing/checkout_net_total_guarded_fee`
  - new guarded wrapper over the same leaf pair
  - uses the already-shipped normalized-required-arg shape to clamp
    `regional_rate` at zero
- `billing/discount_strategy`
  - new plain `kind: sum` seam adapted from the shipped ecommerce shape
  - variants: `none`, `percentage { rate }`, `fixed_amount { amount }`
- `billing/pricing_quote`
  - new plain `kind: data` seam adapted from the shipped ecommerce shape
  - fields: `subtotal`, `membership_rate`, `regional_rate`
  - methods expose discounted subtotal and final net total

Concrete molecule obligations:

- `billing/checkout_success_flow`
  - proves the happy-path multi-unit business flow
- `billing/checkout_declined_discount_flow`
  - proves the unhappy-path service flow using supported business semantics
  - this is where "failure path" is made concrete for I6
- `billing/discount_strategy_quote_flow`
  - proves supported seam usage and seam-to-business coherence

## I6 Service Closure Matrix

| Required proof dimension | I6 proof owner | Required proof surface |
| --- | --- | --- |
| real multi-unit business workflow | `billing/checkout_net_total`, `billing/checkout_success_flow` | benchmark-root `status` + `export` + fresh passport/evidence |
| business-path unhappy flow | `billing/checkout_net_total_guarded_fee`, `billing/checkout_declined_discount_flow` | required molecule proof + benchmark gate |
| supported function rows | `billing/apply_membership_discount`, `billing/apply_regional_fee`, `billing/checkout_net_total`, `billing/checkout_net_total_guarded_fee` | fresh passports + benchmark case projections |
| supported seam rows | `billing/discount_strategy`, `billing/pricing_quote` | fresh passports + seam-focused molecule proof |
| supported seam/business interaction | `billing/discount_strategy_quote_flow` | required molecule proof + read-side projection |
| plain boundary pressure | `billing/pricing_quote` generated files and method lowering | readability file-set review + generated Rust inspection |
| truthful read-side projection | `BENCH-SERVICE` benchmark-root `status`, `export`, and snapshot | full-scope projections only; partial scopes remain zero-credit |
| benchmark-scoped readability | `benchmarks/reviews/BENCH-SERVICE.readability.review.json` | current digest + current generated-file set |
| preserved prior closure | `BENCH-ECOM`, `BENCH-CROSSLIB` | regression suite + repo-root inventory projection |

## Architecture

```text
                    M66 NARROW-CORE CLAIM (UNCHANGED)
                                   |
            +----------------------+----------------------+
            |                                             |
            v                                             v
     shipped supported-core wall                 new service-shaped wall
     (BENCH-ECOM + BENCH-CROSSLIB)                  (BENCH-SERVICE)
                                                          |
                         +--------------------------------+-------------------------------+
                         |                                |                               |
                         v                                v                               v
                 positive service cases          required molecule flows           readability review
                         |                                |                               |
                         v                                v                               v
                    fresh passports                 fresh evidence                current digest + file set
                         \___________________________      |      _________________________/
                                                     \     |     /
                                                      v    v    v
                                                  benchmark projection core
                                                           |
                             +-----------------------------+-----------------------------+
                             |                                                           |
                             v                                                           v
               service-root status/export (proof wall)                  BENCH-SERVICE snapshot
                             |
                             v
                repo can honestly claim service-shaped proof exists

    BENCH-ECOM and BENCH-CROSSLIB remain active and must stay green.
    Repo-root status remains inventory_only even after BENCH-SERVICE activates.
```

## Target Outcome

I6 is done only when all of these are true at the same time:

1. `BENCH-SERVICE` is no longer reserved.
2. `benchmarks/labels.json` lists exactly the six positive service cases and
   three required service molecules from this plan.
3. `examples/service/units` exists as a real single-library authored workload.
4. `cargo run -p spec-cli -- status examples/service/units --format json`
   reports:
   - `benchmark_id: "BENCH-SERVICE"`
   - `lifecycle: "active"`
   - `accounting_status: "valid"`
   - `benchmark_status: "passing"`
   - `gate_status: "satisfied"`
   - `readability_review_status: "current"`
   - `summary.total_cases: 6`
   - `summary.supported_valid_cases: 6`
   - `summary.required_molecule_total: 3`
   - `summary.required_molecule_status_counts.valid: 3`
5. `cargo run -p spec-cli -- export examples/service/units` projects the same
   benchmark truth without inventing any new proof.
6. `cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE` writes an active
   snapshot, not a reserved one.
7. `BENCH-ECOM` still passes with current readability truth.
8. `BENCH-CROSSLIB` still passes with zero positive credit.
9. repo-root `status . --format json` still reports `scope_authority:
   "inventory_only"` while showing all three benchmarks.
10. closeout docs teach the same story the benchmark wall now proves.

## Implementation Contract

Implementation is four phases. Phases 1-3 land the service benchmark. Phase 4
is the final closeout pass after the service wall is stable.

### Phase 1: Service scaffold and benchmark activation

Objective:

- create the real service example tree and flip `BENCH-SERVICE` from reserved to
  active with the exact roster frozen above

Primary write scope:

- `examples/service/**`
- `benchmarks/labels.json`

Required changes:

- scaffold `examples/service/` to mirror the single-library shape used by
  `examples/ecommerce/`
- promote the positive M19 billing function fixtures into
  `examples/service/units/billing/**`
- author `billing/checkout_net_total_guarded_fee.unit.spec`
- author `billing/discount_strategy.unit.spec`
- author `billing/pricing_quote.unit.spec`
- switch `BENCH-SERVICE` to `lifecycle: "active"`
- add the six positive cases and the three required molecules
- keep `required_for_v1: true`
- keep `kind: "positive"` and `readability_scope: "supported_closure"`

Hard rules:

- no `[libraries]` graph in the service example
- no `examples/shared-spec` dependency
- no extra positive service cases in I6

Done when:

- `examples/service/units` loads as a real benchmark root
- `BENCH-SERVICE` is active on purpose rather than reserved by placeholder

### Phase 2: Service proof wall

Objective:

- refresh real proof for every positive service case and every required service
  molecule so the active benchmark can pass truthfully

Primary write scope:

- `examples/service/units/**`
- generated proof artifacts under `examples/service/units/**`
- `examples/service/src/generated/**`

Required changes:

- author and prove `billing/checkout_success_flow.test.spec`
- author and prove `billing/checkout_declined_discount_flow.test.spec`
- author and prove `billing/discount_strategy_quote_flow.test.spec`
- run `spec test` for all six positive service units
- run `spec test` for all three service molecule files
- run `spec build examples/service/units --output examples/service/src/generated`
- confirm every positive service case is `status: "valid"`
- confirm every required service molecule is `status: "valid"`

Hard rules:

- if a service case needs unsupported control flow, replace the case
- if a service failure-path idea needs async/IO or fallback, reject the idea
- if a seam shape no longer fits current support truth, adapt it to the shipped
  ecommerce seam pattern instead of widening the contract

Done when:

- `status examples/service/units --format json` can truthfully pass except for
  readability review if Phase 3 has not run yet

### Phase 3: Service benchmark projection, snapshot, readability, and regressions

Objective:

- make the active service benchmark visible, current, and difficult to regress

Primary write scope:

- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`
- targeted `spec-core/src/benchmark.rs` or `spec-cli/src/commands.rs` only if
  the active service benchmark exposes a read-side truth bug

Required changes:

- refresh `BENCH-SERVICE.snapshot.json` from reserved state to active state
- author the first `BENCH-SERVICE` readability review against the final
  projection digest and generated file set
- add a dedicated integration suite for service-benchmark assertions
- refresh or add benchmark fixtures for:
  - service benchmark-root `status`
  - service benchmark-root `export`
  - service benchmark snapshot
  - repo-root inventory showing active `BENCH-SERVICE`
- add regressions that:
  - active `BENCH-SERVICE` becomes non-passing if any required molecule is
    missing, stale, or failing
  - active `BENCH-SERVICE` becomes non-current when the readability digest or
    file set drifts
  - service partial scopes emit zero positive credit
  - `BENCH-ECOM` and `BENCH-CROSSLIB` remain truthful while service activates

Hard rules:

- do not re-open benchmark schema design
- do not merge service activation by only changing fixtures
- projection code changes are allowed only to fix truthful active-benchmark
  behavior

Done when:

- the service benchmark is active, current, snapshot-backed, and regression
  protected

### Phase 4: Docs and closeout

Objective:

- align repo-facing docs and milestone bookkeeping to the now-active service
  benchmark wall

Primary write scope:

- `README.md`
- `docs/rust_v1_contract_stack.md`
- `TODOS.md`
- `CHANGELOG.md`

Required changes:

- update `README.md` benchmark roster so `BENCH-SERVICE` is no longer described
  as reserved-only
- update `docs/rust_v1_contract_stack.md` so the implementation ladder and repo
  note reflect shipped I5 closure and active I6 service proof work
- update `TODOS.md` to retire the stale post-M68 closure placeholder and leave
  only true follow-on oceans
- record the active service benchmark landing in `CHANGELOG.md`
- if the benchmark wall truly closes the final narrow-core proof gate, say that
  explicitly in docs; if not, do not overstate it

Done when:

- docs, labels, snapshots, reviews, and CLI truth all teach the same story

## Implementation Tasks

Synthesized from the plan's locked decisions and risk surfaces.

- [ ] **T1 (P1, human: ~2h / CC: ~20min)** — service example scaffold — create
      `examples/service/` as a single-library benchmark root
  - Surfaced by: Phase 1 — `BENCH-SERVICE` cannot activate without a real root
  - Files: `examples/service/**`, `benchmarks/labels.json`
  - Verify: `cargo run -p spec-cli -- status examples/service/units --format json`

- [ ] **T2 (P1, human: ~2h / CC: ~20min)** — service billing function roster —
      promote the billing leaf and wrapper cases from the M19 fixture pack
  - Surfaced by: Intended Service Workload — seed shapes already exist
  - Files: `examples/service/units/billing/*.unit.spec`
  - Verify: `cargo run -p spec-cli -- test examples/service/units/billing/apply_membership_discount.unit.spec`

- [ ] **T3 (P1, human: ~2h / CC: ~20min)** — service seam roster — author the
      plain `discount_strategy` and `pricing_quote` seams inside the service
      root
  - Surfaced by: I6 Service Closure Matrix — service proof must include seam usage
  - Files: `examples/service/units/billing/discount_strategy.unit.spec`,
    `examples/service/units/billing/pricing_quote.unit.spec`
  - Verify: `cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy.unit.spec`

- [ ] **T4 (P1, human: ~90min / CC: ~15min)** — service molecule gate — author
      and prove the three required service flows
  - Surfaced by: Frozen Decisions 6 — required molecule roster is fixed
  - Files: `examples/service/units/billing/*.test.spec`
  - Verify: `cargo run -p spec-cli -- test examples/service/units/billing/checkout_success_flow.test.spec`

- [ ] **T5 (P1, human: ~60min / CC: ~10min)** — active benchmark labels —
      switch `BENCH-SERVICE` to active and populate the frozen case roster
  - Surfaced by: Frozen Decisions 1 and 5
  - Files: `benchmarks/labels.json`
  - Verify: `cargo run -p spec-cli -- status examples/service/units --format json`

- [ ] **T6 (P1, human: ~2h / CC: ~20min)** — service regression wall — add a
      dedicated `rust_v1_service` integration suite and service benchmark
      fixtures
  - Surfaced by: Phase 3 — active service truth must be hard to regress
  - Files: `spec-cli/tests/rust_v1_service.rs`,
    `spec-cli/tests/fixtures/benchmarks/**`
  - Verify: `cargo test -p spec-cli rust_v1_service`

- [ ] **T7 (P2, human: ~45min / CC: ~10min)** — snapshot plus readability —
      refresh the active service snapshot and author the first service
      readability review
  - Surfaced by: Target Outcome 4 and 6
  - Files: `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`,
    `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
  - Verify: `cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE`

- [ ] **T8 (P2, human: ~45min / CC: ~10min)** — docs closeout — update the repo
      narrative to reflect active service proof
  - Surfaced by: Phase 4 — current docs still teach the reserved gate story
  - Files: `README.md`, `docs/rust_v1_contract_stack.md`, `TODOS.md`,
    `CHANGELOG.md`
  - Verify: manual doc consistency pass against live CLI output

## Acceptance Commands

Service build and proof refresh:

```bash
cargo run -p spec-cli -- build examples/service/units --output examples/service/src/generated
cargo run -p spec-cli -- test examples/service/units/billing/apply_membership_discount.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/apply_regional_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_net_total_guarded_fee.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_success_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/checkout_declined_discount_flow.test.spec
cargo run -p spec-cli -- test examples/service/units/billing/discount_strategy_quote_flow.test.spec
```

Benchmark proof wall:

```bash
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Non-regression wall:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/crosslib-app/units --format json
cargo test -p spec-cli rust_v1_service
cargo test -p spec-cli rust_v1_closure
cargo run -p spec-cli -- status . --format json
```

The last command remains part of acceptance because I6 must preserve repo-root
inventory semantics while activating the service benchmark.

## Test Strategy

### Coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] Service benchmark activation
    |
    +- benchmarks/labels.json
    |   +- [CHANGE] BENCH-SERVICE reserved -> active
    |   +- [CHANGE] add 6 service cases
    |   \- [CHANGE] add 3 required molecules
    |
    +- examples/service/units/billing/**
    |   +- [CHANGE] positive function units
    |   +- [CHANGE] seam units
    |   \- [CHANGE] molecule flows
    |
    +- benchmark projection core
    |   +- [TEST] active BENCH-SERVICE passes only with full current proof
    |   \- [TEST] partial service scope never earns positive credit
    |
    \- snapshot + readability
        +- [CHANGE] active BENCH-SERVICE snapshot
        \- [TEST] current vs stale review truth

[+] Non-regression coverage
    |
    +- BENCH-ECOM
    |   \- [TEST] remains passing/current
    |
    +- BENCH-CROSSLIB
    |   \- [TEST] remains passing/zero-credit
    |
    \- repo-root inventory
        \- [TEST] remains inventory_only while showing active BENCH-SERVICE

USER FLOW COVERAGE
===========================
[+] Maintainer service-proof flow
    |
    +- author service specs
    +- build service generated Rust
    +- refresh unit passports
    +- refresh molecule evidence
    +- run service benchmark-root status/export/snapshot
    \- confirm active current BENCH-SERVICE

[+] Failure states
    |
    +- active benchmark with missing molecule proof becomes non-passing
    +- service review digest drift becomes non-current
    +- partial service scope cannot launder positive credit
    +- attempted unsupported service shape gets rejected from the positive roster
    \- prior benchmarks remain green while service activates
```

### Mandatory regression tests

1. `BENCH-SERVICE` becomes non-passing if any required service molecule proof is
   missing, stale, or failing.
2. `BENCH-SERVICE` reports current readability only when both
   `projection_digest` and `readability_generated_files` match the live
   projection.
3. benchmark-root `status` and `export` agree on the active service benchmark
   summary and case truth.
4. service namespace and single-file scopes emit zero positive supported credit.
5. repo-root `status . --format json` remains `scope_authority:
   "inventory_only"` after service activation.
6. `BENCH-ECOM` still reports passing/current after the service benchmark lands.
7. `BENCH-CROSSLIB` still reports passing with `positive_credit_cases: 0` after
   the service benchmark lands.
8. the unhappy-path service flow remains a supported business-path proof, not a
   fallback-backed or unsupported workaround.

### Test files

Preferred write shape:

- keep broad benchmark command-wall fixtures in `spec-cli/tests/cli.rs`
- keep I5 closure assertions in `spec-cli/tests/rust_v1_closure.rs`
- add a new dedicated suite `spec-cli/tests/rust_v1_service.rs` for the active
  service benchmark
- keep benchmark fixture JSON under `spec-cli/tests/fixtures/benchmarks/`
- if adversarial service fixtures are needed, keep them under a dedicated
  service subtree rather than mixing them into unrelated milestone packs

## Failure Modes Registry

| Failure mode | Why it matters | Coverage requirement | Critical gap? |
| --- | --- | --- | --- |
| service benchmark goes active with the wrong roster or empty cases | fake V1 closure | label-registry assertions plus full benchmark fixture checks | yes |
| service workload quietly depends on unsupported control flow or widened semantics | benchmark claims more than M66 allows | positive-roster review plus service integration tests | yes |
| unhappy-path flow is implemented as fallback or unsupported trickery | failure-path proof becomes dishonest | explicit unhappy-path molecule proof plus benchmark projection checks | yes |
| service readability review exists but is stale | maintainers think service code was reviewed when read-side truth disagrees | current-vs-stale review regression | yes |
| service partial scope emits positive credit | read-side anti-laundering rule regresses | namespace and single-file zero-credit regressions | yes |
| ECOM or CROSSLIB regresses while BENCH-SERVICE activates | I6 breaks shipped proof walls | non-regression suite across all three benchmarks | yes |
| contract-stack doc stays out of sync after I6 lands | repo authority becomes confusing again | closeout doc pass | no |

## Performance / Complexity Guardrails

I6 should not introduce a meaningful runtime cost increase.

Guardrails:

- benchmark projection remains label-driven and linear in loaded benchmark cases
- service example stays single-library and local
- no new benchmark commands are introduced
- no new artifact families are introduced
- no new semantic-family support is introduced
- service failure-path proof must stay in business semantics, not runtime
  orchestration
- snapshot and review refresh remain explicit artifact updates, never hidden
  side effects of `status` or `export`

If implementation wants a new registry, a new service runtime, or a broader Rust
admission story, it is solving the wrong problem.

## Worktree Parallelization Strategy

I6 has parallelization opportunities, but they must stay bounded so that labels,
snapshots, and docs do not churn against each other.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| service scaffold plus positive source specs | `examples/service/`, `benchmarks/labels.json` | — |
| service proof artifacts and generated Rust | `examples/service/units/`, `examples/service/src/generated/` | service scaffold plus positive source specs |
| service regression suite and benchmark fixtures | `spec-cli/tests/`, `spec-cli/tests/fixtures/benchmarks/` | service scaffold plus positive source specs |
| projection/core truth fixes if needed | `spec-core/`, `spec-cli/src/` | service scaffold plus positive source specs |
| snapshot, readability review, and docs closeout | `benchmarks/snapshots/`, `benchmarks/reviews/`, `README.md`, `docs/`, `TODOS.md`, `CHANGELOG.md` | proof artifacts, regression suite, and any projection/core fixes |

### Parallel lanes

Lane A: service scaffold plus positive source specs

Lane B: service regression suite and benchmark fixture scaffolding

Lane C: projection/core truth fixes, but only if activation exposes a real read-side bug

Lane D: snapshot, readability review, and docs closeout

### Execution order

1. Launch Lane A first.
2. Once the service roster is frozen in `benchmarks/labels.json`, launch Lane B.
3. Launch Lane C only if Lane A or B discovers a real projection-truth bug.
4. Merge A, B, and C.
5. Launch Lane D after that merge, not before.

### Conflict flags

- `benchmarks/labels.json` belongs to Lane A only.
- `examples/service/units/**` belongs to Lane A only.
- `spec-cli/tests/fixtures/benchmarks/**` is the highest-probability merge
  conflict between Lanes B and D; D should refresh only after B is merged.
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json` and
  `benchmarks/reviews/BENCH-SERVICE.readability.review.json` belong to Lane D
  only.
- if Lane B needs a label change, queue it for post-merge integration instead
  of editing `benchmarks/labels.json` in parallel.

## What Success Looks Like

After I6:

- `BENCH-SERVICE` is an active, passing, current positive benchmark
- the repo proves both example-domain and service-shaped workloads on the same
  narrow-core contract
- service proof includes both a happy path and a business-path unhappy flow
- the read-side benchmark wall stays truthful under `status`, `export`, and
  snapshot
- the benchmark roster still distinguishes positive proof from companion
  negative proof
- the next milestone can debate bounded generics, async/IO, or broader V1.1
  scope from a fully closed proof baseline instead of from a reserved placeholder

## NOT in scope

- bounded generics admission into V1
- async or IO admission into V1
- trait-authored, lifetime-heavy, or macro/framework-heavy authored support
- cross-library service benchmark work
- benchmark schema redesign
- new benchmark commands or a standalone benchmark subsystem
- repository-wide command-scope redesign
- ORCH plan authoring
- broader TypeScript-lane work unrelated to Rust V1 service proof

## Completion Summary

- Step 0: Scope challenge accepted as service benchmark activation only
- Architecture review: one active service wall now defined beside the shipped
  ECOM and CROSSLIB walls
- Code quality review: exact service roster, molecule roster, and write scope
  now frozen
- Test review: eight mandatory regression groups identified
- Failure modes: six critical benchmark-truth risks named and assigned
- Parallelization: three implementation lanes plus one sequential closeout lane
- Lake Score: 5/5, because the honest complete version is still cheaper than
  another milestone of reserved-state storytelling
