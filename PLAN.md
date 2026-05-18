<!-- /autoplan restore point: /home/azureuser/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260518-191503.md -->
# I2: Rust V1 Contract Stack Mechanics Landing Plan

Status: **authoritative implementation plan**  
Iteration: **I2**  
Milestone family: **Rust V1 benchmark and truth-surface mechanics**  
Implementation readiness: **ready for implementation**  
Plan scope: **land the full M68 mechanics surface on top of the locked M65-M67 contract stack: benchmark registry, shared benchmark projection, schema-v4 `spec status --format json`, schema-v4 `spec export`, benchmark snapshots, readability review anchoring, reserved-gate projection for `BENCH-SERVICE`, and the exact anti-laundering/path-scope rules. Preserve all M66 support boundaries and defer supported-core expansion to M69.**  
Base branch: **main**  
Working branch: **main**  
Validated at commit: **`aca0307`**  
Last rewritten: **2026-05-18**

Supersedes:

- the stale M61 TypeScript authority plan previously maintained at this path
- `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-rust-completion-execution-plan-20260517-161417.md` as execution context only

Locked authority inputs:

- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`

Historical context, not authority:

- `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-rust-completion-execution-plan-20260517-161417.md`
- `CLAUDE.md`
- `TODOS.md`

Primary repo surfaces:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/*.json`
- `spec-core/src/lib.rs`
- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/types.rs`
- `spec-core/src/graph.rs`
- `benchmarks/**`
- `examples/ecommerce/units/**`
- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Executive Summary

The repo already has the hard parts of proof truth.

It can author specs, validate them, build them, test them, persist unit proof in
`.spec.passport.json`, persist molecule proof in `*.test.evidence.json`, and
project current truth through `spec status --format json` and `spec export`.
What it does not have is the benchmark-accounting layer that turns those proof
surfaces into an honest Rust V1 product claim.

That is the whole I2 job.

I2 does not widen M66 support rows. It does not re-open bounded generics,
async, IO, traits, lifetimes, or macro-heavy authored surfaces. It does not add
benchmark fields to `.unit.spec`, `.test.spec`, passports, or molecule
evidence. It adds one benchmark registry, one shared read-side projection core,
one snapshot writer, one readability review anchor, and one schema-v4 machine
surface for `status` and `export`.

After I2, the repo can finally say this truthfully:

```text
the Rust V1 contract stack has an explicit benchmark roster,
explicit writer-vs-reader boundaries,
explicit anti-laundering rules,
explicit reserved-gate projection for BENCH-SERVICE,
and one machine-readable read-side benchmark surface shared by status and export
```

That closes M68.

It does not close M69.

## Frozen Implementation Decisions

These are inherited from M65-M68 and are locked for I2.

1. **M66 remains the only source of truth for supported, deferred, and explicitly-out Rust rows and interactions.**
   - I2 may not widen or narrow those rows.
   - I2 may not reinterpret fallback policy.

2. **M67 remains the only source of truth for benchmark roles.**
   - `BENCH-ECOM` is the only active positive benchmark in I2.
   - `BENCH-SERVICE` remains `reserved` and required for final V1 proof.
   - `BENCH-CROSSLIB` remains `companion_negative_proof`, never positive credit.

3. **Benchmark accounting is label-driven, not discovered ad hoc.**
   - `benchmarks/labels.json` is authoritative for benchmark membership and classification.
   - Unlabeled or duplicated carriers under an active benchmark root are benchmark-accounting failures.

4. **Proof writers stay where they already are.**
   - `.spec.passport.json` remains the authoritative unit proof write surface.
   - `*.test.evidence.json` remains the authoritative molecule proof write surface.
   - `spec status`, `spec export`, and benchmark snapshotting stay read/projection surfaces only.

5. **No benchmark metadata is authored inside workload specs or proof artifacts in I2.**
   - no benchmark fields in `.unit.spec`
   - no benchmark fields in `.test.spec`
   - no benchmark fields in passports
   - no benchmark fields in molecule evidence

6. **Path scope is explicit.**
   - benchmark projection is `full` only when the command scope loads the entire benchmark root
   - otherwise it is `partial`
   - partial projections never mint positive credit

7. **Reserved-state visibility is mandatory.**
   - repo-broad `spec status . --format json` and repo-broad `spec export` must surface `BENCH-SERVICE`
   - reserved state may not collapse into silence or “missing data”

8. **Readability is benchmark-scoped human observation, not proof truth.**
   - positive benchmarks only
   - tied to `projection_digest`
   - never stored in passports or molecule evidence

9. **The shared generated tree stays shared.**
   - readability scope includes shared `mod.rs` and benchmark-relevant `molecule_tests.rs`
   - there is no fake per-benchmark generated tree split

10. **Schema bumps are part of the milestone.**
    - `spec status --format json`: `schema_version 3 -> 4`
    - `spec export`: `schema_version 3 -> 4`

## Current Validated Basis

Validated on `main` at `aca0307`.

Observed repo truth:

- `spec-cli/src/commands.rs` already owns both `spec status --format json` and
  `spec export --format json` shape emission.
- `spec-core/src/export.rs` already builds the export bundle from authored
  specs, passports, molecule evidence, and graph edges.
- `spec-core/src/passport.rs` already projects current unit truth from authored
  specs plus stored proof.
- `spec-core/src/molecule_evidence.rs` already persists and reads molecule proof
  truth.
- `spec-cli/tests/fixtures/` already carries JSON fixture baselines for status
  and export schema contracts.
- `examples/ecommerce/units/**` already gives one positive benchmark candidate
  with current unit passports and checked-in molecule evidence for
  `pricing/checkout_flow` and `pricing/discount_plus_tax`.
- `examples/crosslib-app/units/**` already gives one maintained companion
  negative-proof workload root.
- `examples/service/units` does not exist today, which is acceptable only
  because `BENCH-SERVICE` is still a reserved benchmark and must remain
  machine-visible without pretending the workload exists yet.
- there is no `benchmarks/` directory today
- there is no benchmark registry today
- there is no shared benchmark projection module today
- there is no benchmark snapshot command today
- there is no readability review artifact today
- `spec status --format json` and `spec export` have no benchmark surface today

The gap is therefore structural, not speculative.

## Step 0: Scope Challenge

### Premise correction

The problem is not “finish Rust V1.”

The problem is narrower:

```text
the repo already has workload truth and proof truth,
but it still lacks the benchmark-accounting and projection layer
that makes the Rust V1 contract observable without laundering fallback,
partial scope, or reserved gates into fake green state
```

If I2 expands beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | I2 action |
| --- | --- | --- |
| authored workload discovery | `spec-core::loader`, `spec-cli/src/commands.rs` | reuse; do not add a second discovery model |
| unit proof persistence | `spec-core/src/passport.rs` | reuse as-is; benchmark code only reads projected truth |
| molecule proof persistence | `spec-core/src/molecule_evidence.rs` | reuse as-is; benchmark code only reads current molecule evidence |
| semantic truth projection | `spec-core/src/passport.rs`, `spec-core/src/export.rs` | reuse; benchmark projection consumes projected semantic support status, it does not re-evaluate semantics separately |
| status JSON emission | `spec-cli/src/commands.rs` | extend to schema v4 with additive `benchmarks[]` |
| export JSON emission | `spec-core/src/export.rs` plus `spec-cli/src/commands.rs` | extend to schema v4 with additive `benchmarks[]` |
| current canonical positive workload | `examples/ecommerce/units/**` | reuse as `BENCH-ECOM`; add label accounting, readability scope, and snapshot support |
| current canonical cross-library pressure workload | `examples/crosslib-app/units/**` | reuse as `BENCH-CROSSLIB`; project as companion negative proof only |
| JSON fixture contract testing | `spec-cli/tests/cli.rs`, `spec-cli/tests/fixtures/*.json` | extend; do not invent a second snapshot-only test harness |

### Minimum complete slice

The minimum honest I2 slice is:

1. add `benchmarks/labels.json` and strict registry validation
2. add one shared benchmark projection core in `spec-core`
3. add canonical `label_digest` and `projection_digest`
4. add path-scope `full` versus `partial` behavior with explicit anti-laundering
5. add schema-v4 additive `benchmarks[]` to both `spec status --format json`
   and `spec export`
6. add `spec benchmark snapshot <benchmark-id>` as a read-only derived writer
7. add readability review artifact loading and projection for positive
   full-scope benchmarks
8. add the reserved `BENCH-SERVICE` gate state and keep it visible at broad
   scope
9. add fixture-backed CLI coverage for full, partial, invalid, reserved, and
   companion-negative cases
10. seed the repo with the initial benchmark registry and initial readability
    review anchor for `BENCH-ECOM`

Anything smaller is fake done.

Examples:

- adding `benchmarks/labels.json` without path-scope rules is fake done
- adding path-scope rules without shared projection code is fake done
- adding schema-v4 output without digest stability is fake done
- adding a snapshot command without a strict writer-vs-reader wall is fake done
- adding benchmark JSON without reserved `BENCH-SERVICE` projection is fake done

### Complexity and blast radius

This plan crosses the 8-file smell threshold.

That is acceptable here because the extra files are contract surfaces, not
infrastructure vanity:

- one new benchmark registry under `benchmarks/`
- one new core projection module
- one core export integration surface
- one CLI status/export/snapshot surface
- one CLI integration test file
- multiple JSON fixture baselines
- one benchmark review artifact
- one doc-surface refresh for `README.md`, `TODOS.md`, and `CHANGELOG.md`

The smaller shortcut would leave the repo with yet another half-contract.
Boil the lake.

### Search check

No framework built-in replaces this work.

- **[Layer 1]** Reuse the existing authored-spec loaders
- **[Layer 1]** Reuse the existing passport and molecule-evidence truth writers
- **[Layer 1]** Reuse the existing status/export JSON fixture harness
- **[Layer 3]** The right architecture is not a benchmark subsystem rewrite, it
  is one shared projection layer that reads authoritative proof truth and emits
  benchmark truth without writing any new proof

### TODOS cross-reference

`TODOS.md` already carries the long-term Rust-completion direction, but it does
not yet name the benchmark registry or benchmark-projection mechanics as a
separate closure step.

I2 should end with:

- `M68 mechanics landing` closed
- `M69 supported-core closure` still open
- `BENCH-SERVICE` still explicitly reserved

### Completeness and distribution check

No new repo-external artifact is introduced.

This remains a CLI and JSON-contract milestone inside the existing `spec`
distribution surface. Completeness here means:

- every benchmark role is explicit
- every broad-scope and narrow-scope projection is honest
- every schema change is fixture-tested
- every writer-versus-reader boundary is enforced

## Milestone Contract

### Exact shipped behavior after I2

After I2:

- `benchmarks/labels.json` exists and is the authoritative benchmark-accounting file
- `benchmarks/snapshots/<BENCHMARK_ID>.snapshot.json` exists as a derived
  artifact written only by `spec benchmark snapshot <benchmark-id>`
- `benchmarks/reviews/<BENCHMARK_ID>.readability.review.json` exists as the
  benchmark-scoped readability verdict surface
- `spec status --format json` emits `schema_version: 4` with additive
  top-level `benchmarks[]`
- `spec export` emits `schema_version: 4` with additive top-level
  `benchmarks[]`
- both surfaces use the same projection engine, the same enums, the same
  path-scope rules, and the same anti-laundering rules
- `BENCH-ECOM` can project `full` or `partial` benchmark truth depending on
  command scope
- `BENCH-CROSSLIB` stays visible but never contributes positive credit
- `BENCH-SERVICE` stays visible at broad scope as `reserved`, never positive
  and never silently omitted
- readability review status is benchmark-scoped and projection-digest-bound
- `status`, `export`, and snapshotting remain read/projection surfaces only

### Exact initial benchmark roster

I2 seeds exactly this roster:

| Benchmark id | Kind | Lifecycle | Required for V1 | Root | Generated root | Readability scope |
| --- | --- | --- | --- | --- | --- | --- |
| `BENCH-ECOM` | `positive` | `active` | `true` | `examples/ecommerce/units` | `examples/ecommerce/src/generated` | `supported_closure` |
| `BENCH-SERVICE` | `positive` | `reserved` | `true` | `examples/service/units` | `examples/service/src/generated` | `supported_closure` |
| `BENCH-CROSSLIB` | `companion_negative_proof` | `active` | `false` | `examples/crosslib-app/units` | `examples/crosslib-app/src/generated` | `none` |

Initial `BENCH-ECOM` required molecule proofs:

- `pricing/checkout_flow`
- `pricing/discount_plus_tax`

Initial `BENCH-ECOM` supported unit carriers:

- `money/round`
- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`
- `pricing/calculate_total_guarded_tax`
- `pricing/discount_strategy`
- `pricing/pricing_quote`

Initial `BENCH-CROSSLIB` companion-negative unit carriers:

- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`
- `pricing/checkout_nested_chain3`

### Exact preserved boundaries

These must still be true after I2:

- no benchmark fields in authored workload specs
- no benchmark fields in passports
- no benchmark fields in molecule evidence
- `spec build` does not mint benchmark truth
- `spec generate` does not mint benchmark truth
- `spec status` does not mint proof truth
- `spec export` does not mint proof truth
- benchmark snapshots do not refresh proof truth
- `M66` support rows do not change
- `BENCH-SERVICE` does not become implemented
- `BENCH-CROSSLIB` never becomes positive-credit workload proof
- partial-scope benchmark queries never emit positive supported credit

### Exact machine contract

This plan is the implementation contract.

An implementer should not need to reopen the upstream M68 design doc to know
what the JSON and enum surfaces have to do.

#### Registry contract

`benchmarks/labels.json` is `schema_version: 1` and each benchmark entry must
declare:

- `id`
- `kind`
- `lifecycle`
- `required_for_v1`
- `root`
- `generated_root`
- `readability_scope`
- `required_molecule_ids[]`
- `cases[]`

Exact allowed values:

- `kind`: `positive`, `companion_negative_proof`
- `lifecycle`: `active`, `reserved`
- `classification`: `supported`, `deferred`, `fallback_backed`,
  `explicitly_out`, `companion_negative_proof`
- `readability_scope`: `supported_closure`, `none`

Hard rules:

- `cases[]` may contain only unit carriers in I2
- molecule tests are benchmark obligations only through
  `required_molecule_ids[]`, never through `cases[]`
- `BENCH-ECOM` and `BENCH-SERVICE` are `kind: positive`
- `BENCH-CROSSLIB` is `kind: companion_negative_proof`
- a `reserved` benchmark may legally have `cases: []` and a missing on-disk
  root
- an `active` benchmark may not silently rely on unlabeled authored units under
  its root

#### Projection contract

Every full-scope benchmark projection in `status`, `export`, and snapshotting
uses one shared shape with these fields:

- `benchmark_id`
- `kind`
- `lifecycle`
- `required_for_v1`
- `path_scope`
- `accounting_status`
- `benchmark_status`
- `gate_status`
- `readability_review_status`
- `readability_verdict` when a review exists
- `label_digest`
- `projection_digest`
- `summary`
- `required_molecule_proofs[]`
- `cases[]`
- `readability_generated_files[]` when readability applies

Each projected case must carry:

- `case_id`
- `carrier_kind`
- `carrier_id`
- `classification`
- `status`
- `reason`
- `semantic_support_status` when present
- `proof_refs.passport` when present
- `proof_refs.covering_molecule_evidence[]` when present
- `counts_as_supported_positive`

`counts_as_supported_positive` is true only when all of these are true:

1. parent benchmark kind is `positive`
2. parent benchmark lifecycle is `active`
3. `path_scope == full`
4. `accounting_status == valid`
5. case classification is `supported`
6. case status is `valid`
7. semantic support status is absent or exactly `supported`

Otherwise it is false. No exceptions.

#### Enum contract

Exact benchmark-level enums:

- `accounting_status`: `valid`, `invalid`, `reserved_missing_cases`,
  `partial_valid`, `partial_invalid`
- `benchmark_status`: `passing`, `failing`, `incomplete`, `invalid`,
  `reserved`
- `gate_status`: `satisfied`, `open`, `reserved`, `not_applicable`
- `readability_review_status`: `current`, `stale`, `missing`,
  `not_applicable`

The reserved `BENCH-SERVICE` full-scope state is locked:

- `lifecycle: reserved`
- `accounting_status: reserved_missing_cases`
- `benchmark_status: reserved`
- `gate_status: reserved`

That state must never be collapsed into “missing”, omitted from broad scope, or
coerced into green.

#### Path-scope contract

Benchmark projection is in scope when the command path equals the benchmark
root, is an ancestor of it, is a descendant inside it, or is a single
`.unit.spec` / `.test.spec` file under it.

`path_scope: full` is allowed only when the command loaded the entire benchmark
root.

`path_scope: partial` is required for namespace and single-file projections and
must omit:

- `benchmark_status`
- `gate_status`
- `label_digest`
- `projection_digest`
- `summary`
- `readability_review_status`
- `readability_verdict`
- `readability_generated_files[]`

Partial scope may emit only `partial_valid` or `partial_invalid`, and every
partial-scope case must emit `counts_as_supported_positive: false`.

#### Digest contract

`label_digest` and `projection_digest` must both be deterministic SHA-256 over
canonical JSON payloads.

Implementation rules:

- no hashing pretty JSON
- no hashing map iteration order
- no hashing temp paths
- sort `required_molecule_ids[]`, case lists, proof-ref lists, and readability
  file lists before canonical encoding
- `projection_digest` excludes `generated_at`, readability review verdict
  fields, snapshot output location, and any ambient runtime-only path detail

#### Snapshot and readability contract

`spec benchmark snapshot <benchmark-id>` is full-scope only.

It may read authored specs, labels, passports, molecule evidence, readability
review files, and generated output. It writes only:

- `benchmarks/snapshots/<BENCHMARK_ID>.snapshot.json`

It may not write:

- passports
- molecule evidence
- semantic review
- readability review files

For active positive benchmarks it must validate that every path listed in
`readability_generated_files[]` exists. For `BENCH-SERVICE`, it must write the
reserved snapshot state without trying to fake generated-file freshness.

## Architecture Review

### Chosen architecture

One shared projection core.

Not one benchmark implementation for `status`, one for `export`, and one for
snapshotting. That would drift immediately.

The implementation should introduce exactly one new `spec-core` module for
benchmark accounting, projection, digesting, and snapshot assembly. CLI
commands own parsing arguments and writing files. Core owns truth.

### Dependency graph

```text
                           authored inputs
                    +---------------------------+
                    | benchmarks/labels.json    |
                    | .unit.spec / .test.spec   |
                    +-------------+-------------+
                                  |
                                  v
                       +----------------------+
                       | loader + scope set   |
                       | existing commands.rs |
                       +----------+-----------+
                                  |
                +-----------------+------------------+
                |                                    |
                v                                    v
   +---------------------------+         +---------------------------+
   | passport projected truth  |         | molecule evidence truth   |
   | spec-core/passport.rs     |         | spec-core/molecule_*.rs   |
   +-------------+-------------+         +-------------+-------------+
                 \                               /
                  \                             /
                   \                           /
                    v                         v
                +-----------------------------------+
                | spec-core benchmark projection    |
                | - label validation                |
                | - path_scope full/partial         |
                | - case accounting                 |
                | - digest computation              |
                | - reserved gate projection        |
                | - readability file selection      |
                +----------------+------------------+
                                 |
                +----------------+-----------------+
                |                                  |
                v                                  v
     +-------------------------+      +------------------------------+
     | spec status --format    |      | spec export / benchmark      |
     | schema v4 benchmarks[]  |      | snapshot writer              |
     +-------------------------+      +------------------------------+
```

### Module layout

Preferred minimal-diff layout:

| Surface | Ownership | Change |
| --- | --- | --- |
| `spec-core/src/benchmark.rs` | new | benchmark label schema, enums, projection structs, digest helpers, readability file selection, snapshot struct |
| `spec-core/src/lib.rs` | existing | export new benchmark module |
| `spec-core/src/export.rs` | existing | thread benchmark projections into export bundle and schema v4 |
| `spec-cli/src/commands.rs` | existing | load registry, compute benchmark scope, emit schema v4 `benchmarks[]`, add `spec benchmark snapshot <benchmark-id>` |
| `spec-cli/tests/cli.rs` | existing | full/partial/reserved/invalid benchmark integration tests |
| `spec-cli/tests/fixtures/*.json` | existing | schema-v4 benchmark fixtures |
| `benchmarks/labels.json` | new | initial authoritative benchmark registry |
| `benchmarks/reviews/BENCH-ECOM.readability.review.json` | new | initial readability review anchor |

Avoid introducing both `benchmark.rs` and `benchmark_projection.rs` unless the
single module becomes unreadable during implementation. Minimal diff wins here.

### File-by-file change map

This is the concrete ownership map for the implementation, not just a module
wishlist.

| File or directory | Exact responsibility | Must not do |
| --- | --- | --- |
| `spec-core/src/benchmark.rs` | own label parsing, validation, full/partial projection, enums, digests, readability file selection, snapshot assembly structs | duplicate CLI path parsing or write files |
| `spec-core/src/lib.rs` | export the new benchmark module cleanly | add side effects |
| `spec-core/src/export.rs` | append shared benchmark projections into export bundle schema v4 | reimplement benchmark logic locally |
| `spec-cli/src/commands.rs` | compute command scope, call shared projection core, serialize schema v4 output, own snapshot subcommand file writing | classify cases or compute digests inline |
| `spec-cli/tests/cli.rs` | own end-to-end repo-root, root-path, subtree, single-file, reserved, invalid-registry, and snapshot behavior coverage | become the only place benchmark rules are specified |
| `spec-cli/tests/fixtures/*.json` | lock the exact schema-v4 machine surfaces | drift from the shared projection contract |
| `benchmarks/labels.json` | seed the canonical roster and case classifications | encode proof truth or readability verdicts |
| `benchmarks/reviews/BENCH-ECOM.readability.review.json` | seed the initial human readability anchor for one projection digest | masquerade as generated data |
| `benchmarks/snapshots/` | hold derived snapshot artifacts only | become source-of-truth inputs |
| `README.md`, `CHANGELOG.md`, `TODOS.md` | document the benchmark roster, reserved gate behavior, and M68 closure/M69 deferral clearly | widen the product claim beyond this exact contract |

### Security and trust boundaries

The critical trust wall is simple:

- authored specs define workload truth
- passports and molecule evidence define proof truth
- benchmark labels define accounting truth
- readability review defines readability truth
- status/export/snapshot only read and project those truths

If any implementation path lets `status`, `export`, or snapshotting write
passports, molecule evidence, or benchmark labels, the milestone failed.

## Code Quality Review

### Non-negotiable code quality rules

1. **One projection implementation, many consumers.**
   - `status`, `export`, and snapshotting must not each implement their own
     benchmark case classification logic.

2. **Enums, not string soup.**
   - `accounting_status`
   - `benchmark_status`
   - `gate_status`
   - `readability_review_status`
   - `classification`
   must be typed in core code, then serialized.

3. **Digest code must be isolated and deterministic.**
   - no hashing pretty JSON
   - no hashing map iteration order
   - no hashing temp paths

4. **Label validation must fail loudly.**
   - duplicate cases
   - unknown classification
   - carrier outside root
   - required molecule id not under benchmark root
   - unlabeled loaded carrier under active benchmark root
   must produce explicit benchmark-accounting failure, not silent omission

5. **Readability selection logic stays benchmark-local.**
   - shared `mod.rs` reuse is expected
   - do not invent fake ownership splits

6. **Reserved-state logic stays explicit.**
   - no “if root missing, drop benchmark”
   - reserved is a first-class state, not an error shortcut

### Naming and API guidance

Use names that describe the contract, not the implementation accident.

Good:

- `BenchmarkLabelRegistry`
- `BenchmarkProjection`
- `BenchmarkCaseProjection`
- `BenchmarkPathScope`
- `BenchmarkAccountingStatus`
- `BenchmarkSnapshot`

Bad:

- `BenchData`
- `ScopeInfo`
- `MaybeGreen`
- `ProjectionHelper2`

### Diagram maintenance

This plan introduces new benchmark concepts, not just code.

If implementation adds or updates nearby ASCII diagrams in `commands.rs`,
`export.rs`, or the new benchmark module, those diagrams must ship accurate in
the same change. Stale diagrams are worse than none.

## Test Review

Rust test framework is already present and authoritative in-repo:

- unit tests in `spec-core`
- integration tests in `spec-cli/tests/cli.rs`
- JSON fixture baselines in `spec-cli/tests/fixtures/*.json`

I2 must land with full benchmark-projection coverage from the start.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/benchmark.rs
    |
    ├── load_labels()
    │   ├── [GAP] valid registry parse
    │   ├── [GAP] duplicate benchmark id rejection
    │   ├── [GAP] duplicate case id rejection within one benchmark
    │   ├── [GAP] unknown classification rejection
    │   └── [GAP] reserved benchmark with non-empty cases rejection
    |
    ├── project_benchmark(scope = full)
    │   ├── [GAP] active positive benchmark passes
    │   ├── [GAP] active positive benchmark incomplete
    │   ├── [GAP] active positive benchmark invalid accounting
    │   ├── [GAP] companion-negative benchmark passes without positive credit
    │   └── [GAP] reserved benchmark emits reserved gate state
    |
    ├── project_benchmark(scope = partial)
    │   ├── [GAP] partial_valid when selected carriers are fully labeled
    │   ├── [GAP] partial_invalid when selected carrier is unlabeled
    │   └── [GAP] partial projections always emit counts_as_supported_positive = false
    |
    ├── compute_label_digest()
    │   ├── [GAP] stable across outer benchmark ordering
    │   └── [GAP] stable across case ordering
    |
    ├── compute_projection_digest()
    │   ├── [GAP] excludes generated_at
    │   ├── [GAP] excludes readability verdict
    │   └── [GAP] changes when cases/summary/proof refs change
    |
    └── readability_generated_files()
        ├── [GAP] includes supported case files
        ├── [GAP] includes ancestor mod.rs chain
        ├── [GAP] includes benchmark-required molecule_tests.rs
        └── [GAP] excludes deferred/fallback/companion files

[+] spec-cli/src/commands.rs
    |
    ├── status --format json
    │   ├── [GAP] repo-root full BENCH-ECOM + reserved BENCH-SERVICE + BENCH-CROSSLIB
    │   ├── [GAP] benchmark root full projection
    │   ├── [GAP] subtree partial projection
    │   └── [GAP] single-file partial projection
    |
    ├── export
    │   ├── [GAP] schema_version 4
    │   ├── [GAP] same benchmark projection as status
    │   └── [GAP] reserved benchmark visible at broad scope only
    |
    └── benchmark snapshot <benchmark-id>
        ├── [GAP] writes snapshot only
        ├── [GAP] full-scope active benchmark snapshot
        ├── [GAP] reserved benchmark snapshot
        └── [GAP] fails when readability-generated files are missing for active benchmark
```

### Developer-flow coverage

```text
DEVELOPER FLOW COVERAGE
===========================
[+] Registry authoring
    ├── [GAP] invalid label file produces machine-readable failure
    └── [GAP] valid label file projects through both status and export

[+] Broad-scope proof query
    ├── [GAP] repo-root status shows BENCH-SERVICE reserved gate
    ├── [GAP] repo-root export shows same benchmark truth
    └── [GAP] companion-negative benchmark stays visible without positive credit

[+] Narrow-scope proof query
    ├── [GAP] benchmark root path gets full projection
    ├── [GAP] namespace path gets partial projection
    └── [GAP] single-unit path gets partial projection with no positive credit

[+] Snapshot maintenance
    ├── [GAP] BENCH-ECOM snapshot generation
    ├── [GAP] BENCH-CROSSLIB snapshot generation
    └── [GAP] BENCH-SERVICE reserved snapshot generation

[+] Readability review anchoring
    ├── [GAP] matching projection_digest => current
    ├── [GAP] mismatched projection_digest => stale
    └── [GAP] missing review => missing
```

### Required test files and assertions

1. `spec-core` unit tests for:
   - registry parsing and validation
   - digest determinism
   - full/partial projection rules
   - reserved-state projection
   - readability file set selection

2. `spec-cli/tests/cli.rs` integration tests for:
   - repo-root full-scope `status --format json`
   - benchmark-root full-scope `status --format json`
   - subtree partial `status --format json`
   - single-file partial `export`
   - snapshot command write behavior
   - invalid registry failure path

3. JSON fixture updates for:
   - full-scope positive benchmark
   - partial-scope positive benchmark
   - reserved benchmark projection
   - companion-negative benchmark projection
   - schema-v4 export bundle

### Regression rule

Any divergence between `status` and `export` benchmark projection for the same
scope is a regression and requires a regression test in the same milestone.

Same rule for:

- reserved benchmark omission
- partial-scope positive credit leakage
- companion-negative positive credit leakage
- digest instability from ordering-only changes

## Performance Review

This is not a throughput milestone, but there are still real footguns.

### Expected hot paths

1. loading and validating the benchmark registry
2. mapping loaded authored carriers to label entries
3. computing digests for full-scope projections
4. assembling readability-generated file sets
5. emitting large schema-v4 JSON fixtures

### Performance decisions

1. Build carrier lookups once per command scope.
   - Do not rescan labels benchmark-by-benchmark with repeated path walks.

2. Keep projection assembly in-memory and map-backed.
   - The repo size is small now, but O(n^2) benchmark-case matching is still
     needless churn.

3. Compute `projection_digest` only for full-scope entries.
   - Partial scope explicitly omits it. Use that.

4. Resolve readability files from projected supported cases, not by walking the
   generated tree blindly.
   - This is both faster and more truthful.

5. Snapshot command may validate file existence, but it must not rebuild
   generated output or rerun proof commands.

### Production failure scenario

If projection assembly silently recomputes benchmark membership from directory
layout instead of labels, a new spec file under `examples/ecommerce/units` can
start counting toward V1 without any explicit benchmark classification.

That is a correctness bug disguised as convenience.

## Failure Modes Registry

| Failure mode | Where it happens | User-visible impact | Test required | Error handling required | Critical |
| --- | --- | --- | --- | --- | --- |
| unlabeled unit under active benchmark root | registry validation / full projection | benchmark turns fake green or silently drops workload | yes | yes, `accounting_status: invalid` plus explicit error | yes |
| duplicate case id or duplicate carrier mapping | registry validation | benchmark accounting becomes ambiguous | yes | yes, hard validation failure | yes |
| partial path leaks positive credit | partial projection | single green unit looks like benchmark closure | yes | yes, force `counts_as_supported_positive: false` | yes |
| `BENCH-SERVICE` omitted at repo-root scope | broad status/export | reserved final gate disappears | yes | yes, explicit reserved projection | yes |
| `BENCH-CROSSLIB` counts toward positive denominator | benchmark summary | negative fixture launders into product claim | yes | yes, hard classification wall | yes |
| readability review matches wrong generated file set | full positive benchmark | stale review looks current | yes | yes, `projection_digest` gate and exact path set match | yes |
| snapshot command refreshes passports or molecule evidence | snapshot CLI | reader becomes writer and mutates proof truth | yes | yes, write-surface wall in tests | yes |
| export and status diverge on same benchmark scope | read-side consumers | machine clients see conflicting truth | yes | yes, shared core and integration regression tests | yes |

There are no acceptable silent failures in this milestone.

## Implementation Plan

### Phase 1: Registry and core projection types

Deliver:

- `spec-core/src/benchmark.rs`
- registry loader and validator
- typed enums and structs
- carrier-to-label matching
- benchmark scope intersection logic

Acceptance:

- full parse/validation unit tests pass
- invalid registry surfaces explicit machine-readable failure

### Phase 2: Shared projection engine and digests

Deliver:

- full/partial projection builder
- case projection with anti-laundering
- summary computation
- `label_digest`
- `projection_digest`
- readability-generated file selection

Acceptance:

- full positive, companion-negative, and reserved benchmark unit tests pass
- digest stability tests pass

### Phase 3: `status` and `export` schema-v4 integration

Deliver:

- additive top-level `benchmarks[]` in `spec status --format json`
- additive top-level `benchmarks[]` in `spec export`
- shared projection core wired into both
- repo-root and path-scoped scope handling

Acceptance:

- schema-v4 fixture baselines pass
- repo-root full scope and single-file partial scope behave exactly as locked

### Phase 4: Snapshot writer and readability review loading

Deliver:

- `spec benchmark snapshot <benchmark-id>`
- snapshot artifact writing under `benchmarks/snapshots/`
- readability review file loading
- full-scope readability status and verdict projection

Acceptance:

- snapshot command writes only snapshot files
- matching and stale readability review states are test-covered

### Phase 5: Repo seeding and docs sync

Deliver:

- initial `benchmarks/labels.json`
- initial `BENCH-ECOM` readability review record
- generated snapshots for seeded benchmarks
- README, TODOS, and CHANGELOG updates

Acceptance:

- fresh repo commands show benchmark truth without manual spelunking
- docs describe benchmark roles and schema-v4 change accurately

### Phase handoff contract

The milestone is sequential until the shared projection contract freezes.

The exact freeze point is the end of Phase 2. Before any parallel lane starts,
the branch must have these stable and reviewed:

- the benchmark enums
- the full versus partial projection field set
- the digest payload rules
- the reserved benchmark state contract
- the case-level anti-laundering rule

After that freeze:

- Phase 3 may wire schema-v4 `status` and `export`
- Phase 4 may wire snapshot writing and readability loading
- neither phase may mutate the shared core contract without rebasing on a
  deliberate Phase-2 contract change

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Phase 1 registry types | `spec-core/`, `benchmarks/` | — |
| Phase 2 projection core | `spec-core/` | Phase 1 |
| Phase 3 status/export integration | `spec-cli/`, `spec-core/` | Phase 2 |
| Phase 4 snapshot + readability | `spec-cli/`, `benchmarks/`, `spec-core/` | Phase 2 |
| Phase 5 fixtures and docs | `spec-cli/tests/`, `spec-cli/tests/fixtures/`, repo-root docs | Phase 3 for schema shape, Phase 4 for snapshot/readability wording |

### Parallel lanes

Lane A: Phase 1 -> Phase 2  
Sequential, shared `spec-core/`

Lane B: Phase 3  
Depends on Lane A, owns `spec-cli/` command integration and schema-v4 read surfaces

Lane C: Phase 4  
Depends on Lane A, owns snapshot mechanics, readability review loading, and benchmark artifact IO

Lane D: Phase 5  
Depends on Lane B and Lane C, owns fixtures and docs sync

### Execution order

1. Launch Lane A first and finish it completely.
2. Launch Lane B and Lane C in parallel from the post-A integration state.
3. Merge B and C.
4. Launch Lane D after both are green.

### Conflict flags

- Lanes B and C both touch `spec-cli/src/commands.rs` if snapshot subcommand and
  status/export integration live in one file. That is a merge-conflict hotspot.
  To keep them parallel, freeze a narrow ownership split:
  - Lane B owns `status` and `export` wiring plus schema-v4 emission
  - Lane C owns new `benchmark snapshot` argument parsing and file-writing path
- Lanes B and C both consume shared `spec-core` benchmark types. Do not let
  either lane mutate core projection contracts after A freezes them.

## NOT in scope

These were considered and are explicitly deferred:

- M69 supported-core closure
  - Why: I2 lands mechanics only. It must not reopen row-support arguments.
- bounded generics admission into V1
  - Why: still a forced later scope decision from M65/M66.
- async / IO admission into V1
  - Why: still a forced later scope decision from M65/M66.
- `BENCH-SERVICE` implementation
  - Why: reserved gate must stay visible, not silently faked closed.
- benchmark score reports or historical benchmark dashboards
  - Why: M68 needs projection truth, not a reporting subsystem.
- text-mode benchmark summaries as a gating surface
  - Why: M68 explicitly makes JSON the contract surface.
- benchmark metadata inside workload specs, passports, or molecule evidence
  - Why: violates the writer-vs-reader wall.
- generated-tree isolation per benchmark
  - Why: the repo already uses shared generated trees and M68 freezes that choice.

## Acceptance Commands

The implementation is not done until all of these pass on the landing branch:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Verification expectations:

- repo-root status/export show `BENCH-ECOM`, `BENCH-CROSSLIB`, and reserved `BENCH-SERVICE`
- benchmark-root status/export show full-scope `BENCH-ECOM`
- namespace and single-file status/export show partial-scope `BENCH-ECOM`
- partial scope never emits positive supported credit
- companion-negative scope never emits positive supported credit
- snapshots write only under `benchmarks/snapshots/`

## Exit Criteria

I2 is done only when all of these are true:

1. `benchmarks/labels.json` exists and validates
2. shared benchmark projection core exists in `spec-core`
3. `status` and `export` both emit schema-v4 additive `benchmarks[]`
4. path-scoped `full` versus `partial` rules match M68 exactly
5. `BENCH-SERVICE` reserved projection is visible at broad scope
6. companion-negative cases stay visible but never count as positive
7. readability review state projects correctly for full positive benchmarks
8. benchmark snapshots can be written without mutating proof truth
9. JSON fixtures cover full, partial, invalid, reserved, and companion-negative states
10. docs explain the benchmark roster and writer-vs-reader wall truthfully

## Completion Summary

- Step 0: Scope Challenge — scope accepted as the full M68 mechanics landing, not M69 support expansion
- Architecture Review: one shared projection core, one registry, one snapshot writer, one schema-v4 benchmark surface
- Code Quality Review: typed enums, deterministic digests, strict label validation, no duplicate read-side logic
- Test Review: full benchmark projection matrix required across core and CLI layers
- Performance Review: map-backed projection, full-scope digesting only, no generated-tree blind scans
- NOT in scope: written
- What already exists: written
- Failure modes: eight critical gaps explicitly blocked
- Parallelization: four lanes, two parallel after core freeze
- Lake Score: every recommendation chooses the complete mechanics landing over the shortcut
