# I6: Rust V1 Service Benchmark Activation Plan

Status: **authoritative implementation plan**
Iteration: **I6**
Milestone family: **Rust V1 service-shaped proof closure**
Implementation readiness: **ready for implementation**
Plan scope: **turn `BENCH-SERVICE` from a reserved benchmark into the active service-shaped proof workload required by `M67`, without widening `M66` support rows, reopening `M68` mechanics, or admitting async/IO, generics, traits, lifetimes, or framework-heavy authored surfaces**
Base branch: **main**
Working branch: **`codex/i6-service-benchmark-activation`**
Validated on branch: **`codex/i6-service-benchmark-activation`**
Last rewritten: **2026-05-21**

Supersedes:

- the prior `I5: Rust V1 Supported-Core Closure Plan`

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- live repo truth run on branch `codex/i6-service-benchmark-activation` at the then-current `HEAD`:
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
- `spec-cli/tests/rust_v1_closure.rs`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`

## Executive Summary

I5 closed the supported-core benchmark wall.

The remaining Rust V1 proof gap is narrower now than it was when `I5` was first
drafted: the repo already proves the positive narrow-core benchmark
(`BENCH-ECOM`) and the companion-negative wall (`BENCH-CROSSLIB`), while
`BENCH-SERVICE` still exists only as a reserved placeholder with zero cases and
no readability review.

I6 closes exactly that gap and nothing else.

This plan activates `BENCH-SERVICE` as one real single-library benchmark rooted
at `examples/service/units`, backed by six supported positive cases, three
required molecule proofs, a current snapshot, a current readability review, and
a dedicated regression wall. When I6 is complete, the repo can truthfully claim
that the current Rust V1 narrow-core contract is proven by both an example-domain
workload and a service-shaped workload.

## Current Validated Truth

Observed on branch `codex/i6-service-benchmark-activation` at the then-current
`HEAD` on 2026-05-21:

- `cargo run -p spec-cli -- status . --format json` reports:
  - `schema_version: 4`
  - `scope_authority: "inventory_only"`
  - `BENCH-ECOM` is `active`, `passing`, and `readability_review_status:
    "current"`
  - `BENCH-CROSSLIB` is `active`, `passing`, and `positive_credit_cases: 0`
  - `BENCH-SERVICE` is `reserved`, `reserved_missing_cases`, and has
    `total_cases: 0`
- `cargo run -p spec-cli -- status . --format json` currently exits non-zero
  because repo-root inventory includes intentionally non-green fixture surfaces.
  For I6, repo-root status remains a diagnostic read surface; acceptance must
  assert its JSON fields, not require a zero exit code.
- `benchmarks/labels.json` already declares the future service benchmark root:
  - `root: "examples/service/units"`
  - `generated_root: "examples/service/src/generated"`
  - `readability_scope: "supported_closure"`
  - `cases: []`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json` already exists, but only in
  reserved form
- `benchmarks/reviews/` contains `BENCH-ECOM.readability.review.json`, but
  there is no `BENCH-SERVICE.readability.review.json`
- `examples/service/` does not exist yet
- the closest already-shipped service-shape building blocks are:
  - seam examples under `examples/ecommerce/units/pricing/`
  - benchmark fixtures under `spec-cli/tests/fixtures/benchmarks/`
  - closure regression patterns in `spec-cli/tests/rust_v1_closure.rs`
- service-shaped billing vocabulary already exists in fixture form under
  `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/**`
- `docs/rust_v1_contract_stack.md` still teaches the ladder only through `I4`,
  so the implementation ladder needs a closeout update after the benchmark lands

That means the benchmark system is already capable of carrying the claim. The
missing work is the benchmark payload, proof, review artifact, and regression
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
- trait, lifetime, macro, or framework-heavy authored support

It needs one truthful service benchmark and the regression wall that keeps it
honest.

Scope is accepted as-is.

### Failure-path definition for I6

`M67` requires a service-shaped failure-path workflow. For I6, that means a
business-path unhappy flow that stays inside the already-supported narrow core.

Allowed examples:

- a declined or absent discount path expressed through the supported plain sum
  seam
- a guarded-fee path that clamps invalid business input into the supported
  non-negative surface

Disallowed examples:

- network failure
- database failure
- async cancellation
- retry or fallback orchestration
- framework-managed request lifecycles

Those remain outside `M66` and outside I6.

### Complexity check

Expected write scope:

- `examples/service/**`
- `benchmarks/labels.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- `benchmarks/reviews/BENCH-SERVICE.readability.review.json`
- `spec-cli/tests/rust_v1_service.rs`
- `spec-cli/tests/fixtures/benchmarks/**`
- targeted `spec-core/src/benchmark.rs` or `spec-cli/src/commands.rs` only if
  activation exposes a truthful projection gap
- `README.md`, `docs/rust_v1_contract_stack.md`, `TODOS.md`, and
  `CHANGELOG.md` during closeout

If implementation starts redesigning projection, widening support rows, or
re-arguing `M66`/`M67`/`M68`, stop. That is different scope.

## What Already Exists

| Sub-problem | Existing owner | I6 action |
| --- | --- | --- |
| positive benchmark mechanics | `benchmarks/labels.json`, `spec-core/src/benchmark.rs`, `spec-cli/src/commands.rs` | reuse unchanged unless service activation exposes a real truth bug |
| benchmark-root truth surfaces | `spec status`, `spec export`, `spec benchmark snapshot` | preserve contract, add service-root coverage |
| positive narrow-core proof shape | `BENCH-ECOM` | reuse as the primary pattern for labels, required molecules, readability, and snapshot assertions |
| companion-negative wall | `BENCH-CROSSLIB` | preserve unchanged and assert non-regression |
| readability review mechanics | `benchmarks/reviews/BENCH-ECOM.readability.review.json` | reuse the same digest and generated-file-set contract for `BENCH-SERVICE` |
| benchmark fixture pattern | `spec-cli/tests/fixtures/benchmarks/**` | add service-specific fixtures alongside the existing benchmark fixture family |
| benchmark regression pattern | `spec-cli/tests/rust_v1_closure.rs` | preserve this suite and add a dedicated `rust_v1_service` suite rather than overloading closure tests |
| service billing vocabulary | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/**` | promote only the supported positive shapes into `examples/service/units/billing/**` |
| supported seam patterns | `examples/ecommerce/units/pricing/discount_strategy.unit.spec`, `examples/ecommerce/units/pricing/pricing_quote.unit.spec`, and their molecule tests | adapt the same supported seam shape into the service domain |
| repo-root inventory semantics | current repo-root `status . --format json` behavior | preserve as diagnostic-only inventory with benchmark visibility, not as a proof-success command |

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
   - supported seam examples already shipped in ecommerce are allowed
   - unsupported-function or unsupported-seam cases stay fixture-only, not
     positive benchmark cases

5. **The positive service roster is fixed in this plan.**
   - `billing/apply_membership_discount`
   - `billing/apply_regional_fee`
   - `billing/checkout_net_total`
   - `billing/checkout_net_total_guarded_fee`
   - `billing/discount_strategy`
   - `billing/pricing_quote`

6. **The required molecule roster is fixed in this plan.**
   - `billing/checkout_success_flow`
   - `billing/checkout_declined_discount_flow`
   - `billing/discount_strategy_quote_flow`

7. **Source specs remain the only authored truth for service behavior.**
   - edit `.unit.spec` and `.test.spec` files by hand
   - never hand-edit generated Rust, passports, molecule evidence, or snapshots
   - refresh generated Rust with `spec build`
   - refresh passports and molecule evidence with `spec test`
   - refresh benchmark snapshot with `spec benchmark snapshot`

8. **`BENCH-SERVICE` keeps the existing writer/reader contract.**
   - source specs are authored truth
   - passports and molecule evidence are proof writers
   - benchmark labels are benchmark-accounting truth
   - readability review is human-authored observation truth
   - `status`, `export`, and `benchmark snapshot` are read-side projections

9. **A service case that needs widened support is a failed case, not a widened contract.**
   - if any proposed positive case cannot stay inside current supported rows and
     interactions, replace the case
   - do not widen `M66` to rescue the benchmark

10. **`BENCH-ECOM` and `BENCH-CROSSLIB` stay green and truthful throughout I6.**
    - the service landing may refresh shared fixtures only where the benchmark
      roster legitimately changes
    - it may not regress the already-shipped benchmark wall

11. **Minimal diff still wins.**
    - prefer promoting existing billing fixture vocabulary plus adapting shipped
      seam patterns
    - only touch projection code when the active service benchmark exposes a
      real truth gap

12. **Implementation finishes on the current dedicated branch.**
    - work continues on `codex/i6-service-benchmark-activation`
    - refresh validation truth on that branch before declaring I6 done

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
  - supported arithmetic leaf, monotone-down nonnegative
- `billing/apply_regional_fee`
  - promoted from the M19 billing fixture pack
  - supported arithmetic leaf, monotone-up
- `billing/checkout_net_total`
  - promoted from the M19 billing fixture pack
  - supported wrapper pipeline over discount then fee
- `billing/checkout_net_total_guarded_fee`
  - new guarded wrapper over the same leaf pair
  - uses the already-shipped normalized-required-arg shape to clamp
    `regional_rate` at zero
- `billing/discount_strategy`
  - new plain `kind:sum` seam adapted from the ecommerce pattern
  - variants: `none`, `percentage { rate }`, `fixed_amount { amount }`
- `billing/pricing_quote`
  - new plain `kind:data` seam adapted from the ecommerce pattern
  - fields: `subtotal`, `membership_rate`, `regional_rate`
  - methods expose discounted subtotal and final net total

Concrete molecule obligations:

- `billing/checkout_success_flow`
  - proves the happy-path multi-unit business flow
- `billing/checkout_declined_discount_flow`
  - proves the unhappy-path service flow using supported business semantics
  - this is where "failure path" becomes concrete for I6
- `billing/discount_strategy_quote_flow`
  - proves seam usage and seam-to-business coherence

## I6 Service Closure Matrix

| Required proof dimension | I6 proof owner | Required proof surface |
| --- | --- | --- |
| real multi-unit business workflow | `billing/checkout_net_total`, `billing/checkout_success_flow` | benchmark-root `status` + `export` + fresh passport/evidence |
| business-path unhappy flow | `billing/checkout_net_total_guarded_fee`, `billing/checkout_declined_discount_flow` | required molecule proof + benchmark gate |
| supported function rows | `billing/apply_membership_discount`, `billing/apply_regional_fee`, `billing/checkout_net_total`, `billing/checkout_net_total_guarded_fee` | fresh passports + benchmark case projections |
| supported seam rows | `billing/discount_strategy`, `billing/pricing_quote` | fresh passports + seam-focused molecule proof |
| supported seam/business interaction | `billing/discount_strategy_quote_flow` | required molecule proof + read-side projection |
| generated readability pressure | `examples/service/src/generated/**` | readability review + generated Rust inspection |
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

## Authoring and Proof Contract

I6 must follow the repo's spec workflow rather than mutating artifacts directly.

Implementation rules:

- edit `.unit.spec` and `.test.spec` files, not generated Rust or proof artifacts
- use `cargo run -p spec-cli -- validate <unit-or-root> --format json` as the
  first machine-readable failure surface when a spec is invalid
- use `cargo run -p spec-cli -- build examples/service/units --output examples/service/src/generated`
  to regenerate the shared output tree
- use `cargo run -p spec-cli -- test <path>` to refresh the exact passport or
  molecule evidence that changed
- treat `status` as projection and inventory; treat `test` as proof-writer
- author the readability review only after the generated file set and projection
  digest are final

This removes the only dangerous ambiguity in the implementation: which files are
authored truth versus derived truth.

## Target Outcome

I6 is done only when all of these are true at the same time:

1. `BENCH-SERVICE` is no longer reserved.
2. `benchmarks/labels.json` lists exactly the six positive service cases and
   three required service molecules from this plan.
3. `examples/service/units` exists as a real single-library authored workload.
4. `cargo run -p spec-cli -- status examples/service/units --format json`
   exits successfully and reports:
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
   "inventory_only"` and includes all three benchmarks, regardless of whether
   unrelated inventory rows keep the overall command non-green.
10. closeout docs teach the same story the benchmark wall now proves.

## Implementation Contract

Implementation is four phases. Phases 1-3 land the benchmark truth. Phase 4 is
the closeout pass after the service wall is stable.

### Phase 1: Service scaffold and benchmark activation

Objective:

- create the real service example tree and flip `BENCH-SERVICE` from reserved
  to active with the exact roster frozen above

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
- do not author the readability review yet

Exit criteria:

- `examples/service/units` loads as a real benchmark root
- `BENCH-SERVICE` is active on purpose rather than reserved by placeholder
- the label roster matches this plan exactly

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

Exit criteria:

- `status examples/service/units --format json` can truthfully pass except for
  readability review if Phase 3 has not run yet
- `examples/service/src/generated/**` is final enough for readability review

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
- add regressions that prove:
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

Exit criteria:

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
  note reflect shipped `I5` closure and active `I6` service proof work
- update `TODOS.md` to retire stale post-`M68` closure placeholders and leave
  only true follow-on oceans
- record the active service benchmark landing in `CHANGELOG.md`
- if the benchmark wall truly closes the final narrow-core proof gate, say that
  explicitly in docs; if not, do not overstate it

Exit criteria:

- docs, labels, snapshots, reviews, and CLI truth all teach the same story

## Implementation Tasks

Synthesized from the plan's locked decisions and risk surfaces.

- [ ] **T1 (P1, human: ~2h / CC: ~20min)** — service scaffold — create
      `examples/service/` as a single-library benchmark root
  - Surfaced by: Phase 1 — `BENCH-SERVICE` cannot activate without a real root
  - Files: `examples/service/**`, `benchmarks/labels.json`
  - Verify: `cargo run -p spec-cli -- status examples/service/units --format json`

- [ ] **T2 (P1, human: ~2h / CC: ~20min)** — service function roster — promote
      the billing leaf and wrapper cases from the M19 fixture pack
  - Surfaced by: Intended Service Workload — supported billing shapes already
    exist and should be reused rather than re-invented
  - Files: `examples/service/units/billing/*.unit.spec`
  - Verify: `cargo run -p spec-cli -- test examples/service/units/billing/apply_membership_discount.unit.spec`

- [ ] **T3 (P1, human: ~2h / CC: ~20min)** — service seam roster — author the
      plain `discount_strategy` and `pricing_quote` seams inside the service
      root
  - Surfaced by: I6 Service Closure Matrix — service proof must include seam
    usage, not functions only
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
  - Surfaced by: Phase 4 — current docs still teach the pre-I5 ladder
  - Files: `README.md`, `docs/rust_v1_contract_stack.md`, `TODOS.md`,
    `CHANGELOG.md`
  - Verify: manual doc consistency pass against live CLI output

## Acceptance Commands

Phase-2 proof refresh:

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

Acceptance notes:

- the benchmark-root commands above must exit successfully when I6 is complete
- the final repo-root `status . --format json` command is diagnostic; it must
  preserve `scope_authority: "inventory_only"` and show all three benchmarks,
  but it is not required to become a zero-exit proof command for this workspace

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
    +- validate failing specs with `spec validate --format json`
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
| repo-root acceptance wrongly expects zero exit | implementation appears blocked even when contract is preserved | acceptance notes must treat repo-root status as diagnostic inventory | yes |
| ECOM or CROSSLIB regresses while BENCH-SERVICE activates | I6 breaks shipped proof walls | non-regression suite across all three benchmarks | yes |
| contract-stack doc stays out of sync after I6 lands | repo authority becomes confusing again | closeout doc pass | no |

## Performance / Complexity Guardrails

I6 should not introduce a meaningful runtime or maintenance cost increase.

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

I6 has real parallelization opportunities, but they have to be sequenced so the
benchmark roster freezes before tests, snapshots, and docs start depending on
it.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| service scaffold and benchmark activation | `examples/service/`, `benchmarks/` | — |
| service proof refresh | `examples/service/`, `examples/service/src/generated/` | service scaffold and benchmark activation |
| service regression suite | `spec-cli/tests/`, `spec-cli/tests/fixtures/benchmarks/` | service scaffold and benchmark activation |
| projection/core fixes if needed | `spec-core/`, `spec-cli/src/` | service scaffold and benchmark activation |
| snapshot, readability, and docs closeout | `benchmarks/snapshots/`, `benchmarks/reviews/`, `docs/`, repo root docs | service proof refresh, service regression suite, and any projection/core fixes |

### Parallel lanes

Lane A: service scaffold and benchmark activation -> service proof refresh
(sequential, shared `examples/service/`)

Lane B: service regression suite (independent after Lane A freezes the roster)

Lane C: projection/core fixes only if activation exposes a real truth bug
(independent from Lane B, but dependent on Lane A)

Lane D: snapshot, readability, and docs closeout (sequential after A/B/C merge)

### Execution order

1. Launch Lane A first and freeze the benchmark roster.
2. Once Lane A has frozen `BENCH-SERVICE`, launch Lane B in parallel.
3. Launch Lane C only if Lane A or Lane B exposes a real read-side truth bug.
4. Merge A, B, and C.
5. Launch Lane D after that merge, not before.

### Conflict flags

- `benchmarks/labels.json` belongs to Lane A only.
- `examples/service/**` belongs to Lane A only.
- `spec-cli/tests/fixtures/benchmarks/**` belongs to Lane B until fixture shape
  stabilizes; Lane D should not touch it in parallel.
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json` and
  `benchmarks/reviews/BENCH-SERVICE.readability.review.json` belong to Lane D
  only.
- if Lane B discovers it needs a label change, queue that change for post-merge
  integration instead of editing `benchmarks/labels.json` in parallel.

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
- Architecture review structure: one active service wall defined beside the
  shipped ECOM and CROSSLIB walls
- Code quality structure: authored truth, proof writers, and read-side surfaces
  are now separated explicitly
- Test review structure: coverage diagram, mandatory regressions, failure modes,
  and acceptance semantics are all specified
- Parallelization: 4 lanes total, 2 conditional/parallel lanes, 2 sequential
  dependency stages
- Unresolved decisions: none; if new ambiguity appears during implementation,
  the implementation is out of plan and should stop for a scope check
