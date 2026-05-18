# I1: Benchmark Registry + Shared Projection Core

Status: **authoritative implementation plan**  
Milestone: **I1**  
Milestone family: **Rust V1 benchmark projection foundation**  
Implementation readiness: **ready for execution**  
Plan scope: **land the benchmark registry, shared read-side benchmark projection core, schema-v4 `status`/`export` surfaces, full-vs-partial benchmark scope honesty, and explicit reserved `BENCH-SERVICE` gate projection without adding new proof writers**  
Base branch: **main**  
Working branch: **feat/m60-plus**  
Validated at commit: **`3561bd1`**  
Last rewritten: **2026-05-18**

Supersedes:

- the previous repo-root M64 authority plan formerly kept at this path

Primary source artifacts:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/rust_v1_contract_stack.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`
- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `examples/ecommerce/units`
- `examples/crosslib-app/units`

## Primary Decision

I1 lands exactly one narrow but complete wedge:

```text
authoritative benchmark registry
        +
shared benchmark projection engine
        +
schema-v4 additive benchmarks[] on status/export
        +
full-vs-partial scope honesty
        +
explicit reserved BENCH-SERVICE gate state
```

That is the whole milestone.

I1 does **not** add a benchmark writer, snapshot command, readability review
loading, `projection_digest`, or generated-file readability closure. Those stay
in `I2` and later, exactly where the ladder puts them.

## Executive Summary

The planning stack is frozen enough to code. The repo is not.

Today, `spec status --format json` and `spec export --format json` know unit
truth and molecule truth, but they cannot project benchmark truth. That leaves
three product holes:

1. downstream tooling cannot ask benchmark-level questions from public JSON
2. reserved required proof like `BENCH-SERVICE` is not machine-visible
3. path-scoped commands can accidentally launder a narrow query into apparent
   whole-benchmark green

I1 fixes those holes with one shared read-side projector. It does not widen the
proof-writing surface. It does not invent new benchmark storage. It does not
split status logic and export logic into parallel implementations.

## Current Validated Baseline

Validated on `feat/m60-plus` at `3561bd1`.

### Machine contract baseline

- `STATUS_JSON_SCHEMA_VERSION` is `3` in `spec-cli/src/commands.rs`
- `EXPORT_SCHEMA_VERSION` is `3` in `spec-core/src/export.rs`
- `spec-cli/tests/cli.rs` still asserts schema version `3` for both surfaces

I1 therefore must bump both machine surfaces together in the same milestone.

### Writer vs reader baseline

- authored workload truth lives in `.unit.spec` and `.test.spec`
- unit proof truth lives in `.spec.passport.json`
- molecule proof truth lives in `*.test.evidence.json`
- `spec status` and `spec export` already project read-side truth without
  minting new proof

That means I1 needs a new shared reader-side layer, not a new writer.

### Live benchmark candidate baseline

`examples/ecommerce/units` already contains:

- 7 unit specs
- 3 molecule tests
- current green proof on the benchmark-candidate workload

`examples/crosslib-app/units` already contains:

- the companion negative-proof `pricing/checkout_nested_chain3`
- 3 supported carrier units that still need explicit benchmark accounting

### Existing scope-resolution baseline

Current code already has:

- `resolve_status_roots()` in `spec-cli/src/commands.rs`
- existing file-vs-directory-vs-root status behavior
- existing path-scoped `export` behavior

Current code does **not** have:

- benchmark registry loading
- benchmark-level projection structs
- benchmark path-scope classification
- additive top-level `benchmarks[]`
- reserved benchmark gate projection

So the correct move is to extend the existing scope model, not replace it.

## Problem Statement

The Rust V1 docs know what the benchmark layer should mean. The machine
surfaces do not expose it yet.

That gap creates fake confidence:

- a benchmark can exist in docs but disappear from machine output
- a partial query can look greener than it should
- deferred, fallback-backed, or companion-negative cases can be mistaken for
  native positive credit

I1 closes that gap by making benchmark truth label-driven, shared, additive,
and scope-honest.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner | I1 action |
| --- | --- | --- |
| status scope resolution | `resolve_status_roots()` in `spec-cli/src/commands.rs` | reuse and extend with benchmark root intersection plus full/partial benchmark scope classification |
| unit health projection | `compute_health_status()` and `apply_semantic_review_to_health()` in `spec-cli/src/commands.rs` | reuse as case-level proof truth input |
| export bundle assembly | `spec-core/src/export.rs` | extend additively with top-level `benchmarks[]` |
| passport and molecule proof loading | `spec-core/src/passport.rs` and `spec-core/src/molecule_evidence.rs` | reuse for case proof refs and required molecule proof state |
| semantic support truth | `spec-core/src/semantic_review.rs` and projected passport truth | reuse for `semantic_support_status` and anti-laundering credit rules |
| positive benchmark candidate | `examples/ecommerce/units` | use as active `BENCH-ECOM` root |
| companion negative-proof candidate | `examples/crosslib-app/units` | use as active `BENCH-CROSSLIB` root |
| JSON contract coverage | `spec-cli/tests/cli.rs` plus fixture JSON under `spec-cli/tests/fixtures/` | bump to schema version `4` and add benchmark assertions in the same wedge |

### Minimum complete slice

The minimum honest I1 slice is:

1. add repo-root `benchmarks/labels.json` and validate it as authoritative
   benchmark-accounting input
2. add one shared benchmark projection module in `spec-core`
3. add explicit benchmark enums and projection structs in that shared module
4. project full vs partial benchmark scope honestly for both `status` and
   `export`
5. emit additive top-level `benchmarks[]` from `spec status --format json`
6. emit additive top-level `benchmarks[]` from `spec export --format json`
7. surface `BENCH-SERVICE` as explicit `reserved` machine state at broad
   full-scope queries
8. enforce anti-laundering rules so partial, deferred, fallback-backed,
   companion-negative, and reserved cases never mint positive benchmark credit
9. update schema assertions, fixtures, and contract tests in the same milestone

Anything smaller is fake done.

### Complexity check

This wedge touches more than one production surface, but it is still the
minimum engineered slice:

- one new shared production module in `spec-core`
- one existing CLI status path in `spec-cli`
- one existing export path in `spec-core`
- one authored registry file
- one existing test surface

That is acceptable. It is boring extension work on existing seams, not a new
subsystem.

### Search check

I1 should stay Layer 1 and Layer 3:

- **[Layer 1]** Reuse current status scope resolution, passport projection,
  molecule evidence loading, and export bundle assembly.
- **[Layer 1]** Keep serialization on existing `serde` and `serde_json`.
- **[Layer 3]** Do not infer benchmark truth from directory discovery. Use an
  explicit authored registry because benchmark accounting is product truth, not
  filesystem folklore.

No benchmark cache, no discovery magic, no second registry format, no benchmark
database.

### TODOS cross-reference

Current `TODOS.md` has no blocking item for this wedge.

The follow-on work is already owned by the implementation ladder:

- I2 owns snapshot and readability surfaces
- I3 owns broader anti-laundering closure
- I4 owns the wider schema-v4 fixture/test closure

Do not create new ad hoc TODOs for those.

### Completeness check

The complete I1 version is still cheap enough to land now:

- both JSON surfaces move together
- full and partial benchmark scope land together
- reserved gate visibility lands together
- anti-laundering lands together
- fixtures and tests land in the same commit

Splitting that into two milestones would save minutes and create a second
public-contract migration. Not worth it.

### Distribution check

No new end-user artifact is introduced.

`benchmarks/labels.json` is authored repo input, not a distributable product
artifact. Release pipeline work is out of scope.

## Architecture Review

### Core design

I1 adds one shared projection engine and keeps every writer unchanged.

```text
authored specs (.unit.spec / .test.spec)
           |
           |      passports / molecule evidence / semantic truth
           |                     |
           v                     v
                spec-core benchmark projector
                - load labels.json
                - validate registry
                - classify scope as full/partial
                - project case truth
                - project required molecules
                - apply anti-laundering
                - derive benchmark + gate state
                     /                       \
                    v                         v
     spec status --format json         spec export --format json
       top-level benchmarks[]            top-level benchmarks[]
```

That shared projector is the whole architecture move.

Hard rule: benchmark validation, scope classification, case projection,
anti-laundering, benchmark status derivation, and reserved-gate projection live
once in `spec-core`. `spec-cli` wires inputs and serializes output. Nothing
else gets to reinterpret benchmark rules.

### Module boundaries

| Module / surface | Ownership in I1 | Notes |
| --- | --- | --- |
| `benchmarks/labels.json` | new authored source of benchmark-accounting truth | repo-root, checked in |
| `spec-core/src/benchmarks.rs` | new shared registry + projection engine | the only place allowed to know benchmark accounting rules |
| `spec-core/src/lib.rs` | export the new benchmark module | thin wiring only |
| `spec-core/src/export.rs` | add additive top-level `benchmarks[]` to `ExportBundle` | reuse shared projector, do not fork rules |
| `spec-cli/src/commands.rs` | load registry, call projector for status JSON, bump schema version | text mode remains unchanged |
| `spec-cli/tests/cli.rs` and `spec-cli/tests/fixtures/` | schema-v4 contract coverage | full, partial, reserved, and invalid cases |

### Shared types and exact public shape

I1 should add explicit shared benchmark types in `spec-core`, not anonymous JSON
maps in `spec-cli`.

Required enums:

- `BenchmarkKind` = `positive | companion_negative_proof`
- `BenchmarkLifecycle` = `active | reserved`
- `BenchmarkClassification` =
  `supported | deferred | fallback_backed | explicitly_out | companion_negative_proof`
- `BenchmarkPathScope` = `full | partial`
- `BenchmarkAccountingStatus` =
  `valid | invalid | reserved_missing_cases | partial_valid | partial_invalid`
- `BenchmarkStatus` =
  `passing | failing | incomplete | invalid | reserved`
- `BenchmarkGateStatus` = `satisfied | open | reserved | not_applicable`

Required projection structs:

- `BenchmarkRegistry`
- `BenchmarkLabel`
- `BenchmarkCaseLabel`
- `BenchmarkProjection`
- `BenchmarkCaseProjection`
- `BenchmarkRequiredMoleculeProjection`

Required `BenchmarkProjection` fields for I1:

- `id`
- `kind`
- `lifecycle`
- `root`
- `generated_root`
- `path_scope`
- `accounting_status`
- `label_digest` for full-scope entries only
- `benchmark_status` for full-scope active/reserved entries only
- `gate_status` for full-scope active/reserved positive benchmarks only
- `cases`
- `required_molecule_proofs`

Required `BenchmarkCaseProjection` fields for I1:

- `case_id`
- `carrier_kind`
- `carrier_id`
- `classification`
- `carrier_status`
- `semantic_support_status`
- `passport_path`
- `counts_as_supported_positive`

Required `BenchmarkRequiredMoleculeProjection` fields for I1:

- `id`
- `status`
- `evidence_path`

No other benchmark summary object should appear in I1. If a field implies
whole-benchmark rollup beyond the items above, it belongs in a later milestone.

### Authoritative benchmark registry for I1

`benchmarks/labels.json` is the sole benchmark-accounting input for I1.

#### `BENCH-ECOM`

| Field | Value |
| --- | --- |
| role | active positive benchmark |
| `root` | `examples/ecommerce/units` |
| `generated_root` | `examples/ecommerce/src/generated` |
| required molecules | `pricing/checkout_flow`, `pricing/discount_plus_tax`, `pricing/discount_strategy_checkout_flow` |

Labeled unit cases:

- `money/round` -> `supported`
- `pricing/apply_discount` -> `supported`
- `pricing/apply_tax` -> `supported`
- `pricing/calculate_total` -> `supported`
- `pricing/calculate_total_guarded_tax` -> `supported`
- `pricing/discount_strategy` -> `supported`
- `pricing/pricing_quote` -> `supported`

#### `BENCH-CROSSLIB`

| Field | Value |
| --- | --- |
| role | active companion negative-proof benchmark |
| `root` | `examples/crosslib-app/units` |
| `generated_root` | `examples/crosslib-app/src/generated` |
| required molecules | none |

Labeled unit cases:

- `pricing/apply_discount` -> `supported`
- `pricing/apply_tax` -> `supported`
- `pricing/calculate_total` -> `supported`
- `pricing/checkout_nested_chain3` -> `companion_negative_proof`

Why the supported carriers must still be labeled:

- `BENCH-CROSSLIB` is an active benchmark root
- unlabeled authored carriers under an active root must invalidate accounting
- only the nested chain3 unit is the companion-negative case, but the other
  authored carriers still need explicit accounting

#### `BENCH-SERVICE`

| Field | Value |
| --- | --- |
| role | reserved positive benchmark required for final V1 closure |
| `root` | `examples/service/units` |
| `generated_root` | `examples/service/src/generated` |
| required molecules | none |
| initial cases | empty, because the benchmark is reserved |

### Registry validation contract

`benchmarks/labels.json` validation must fail clearly for:

- duplicate benchmark ids
- duplicate `case_id` within a benchmark
- duplicate carrier mappings within a benchmark
- unknown classification values
- active benchmarks with malformed roots or generated roots
- active benchmark carriers that do not resolve to authored unit ids

Reserved benchmarks may reference a root that does not exist yet. That is legal
only when `lifecycle == reserved`.

### Exact path-scope contract

Use the existing command scope and add benchmark intersection rules on top.

Full-scope examples:

- `spec status . --format json`
- `spec status examples/ecommerce --format json`
- `spec status examples/ecommerce/units --format json`
- `spec export examples/crosslib-app --format json`

Partial-scope examples:

- `spec status examples/ecommerce/units/pricing --format json`
- `spec status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json`
- `spec export examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --format json`

Required rules:

1. emit benchmark entries only when the command scope intersects the benchmark
   `root`
2. `path_scope = full` only when the command loaded the benchmark's entire
   declared root
3. `path_scope = partial` when the command intersects the root without loading
   the whole root
4. every partial case must emit `counts_as_supported_positive: false`
5. partial entries must omit `benchmark_status`, `gate_status`, and
   `label_digest`
6. `BENCH-SERVICE` appears only when the query scope is broad enough to contain
   its declared root, which means repo-root or repo-ancestor queries, not
   narrow ecommerce-only queries

### Benchmark accounting and anti-laundering rules

Full-scope accounting:

- `valid` when every authored unit under the active benchmark root is
  explicitly labeled and the registry shape itself is valid
- `invalid` when any authored unit under the active root is unlabeled or the
  registry is internally contradictory
- `reserved_missing_cases` only for reserved benchmarks

Partial-scope accounting:

- `partial_valid` when the intersecting authored carriers are all labeled and
  the projection is honest about being partial
- `partial_invalid` when the intersecting query reveals unlabeled carriers or
  other accounting contradictions

Case-level credit rules:

- `supported` may count positive only when `path_scope = full` and the case
  itself projects supported/native truth
- `deferred` never counts positive
- `fallback_backed` never counts positive
- `explicitly_out` never counts positive
- `companion_negative_proof` never counts positive

### Benchmark status and gate-status derivation

Positive benchmarks at full scope:

- `benchmark_status = invalid` when `accounting_status = invalid`
- `benchmark_status = failing` when any supported case or required molecule
  projects `failing` or `invalid`
- `benchmark_status = incomplete` when any supported case or required molecule
  projects `stale`, `untested`, or `incomplete`
- `benchmark_status = passing` only when every supported case and every
  required molecule projects `valid`
- `gate_status = satisfied` only when `benchmark_status = passing`
- `gate_status = open` for active positive benchmarks in every other full-scope
  non-reserved state

Companion benchmarks at full scope:

- `benchmark_status = invalid` when accounting is invalid
- `benchmark_status = failing` when a required carrier fails to project
  benchmark truth at all
- `benchmark_status = passing` when all labeled cases project and none of them
  counts positive
- `gate_status = not_applicable`

Reserved benchmarks at full scope:

- `lifecycle: reserved`
- `path_scope: full`
- `accounting_status: reserved_missing_cases`
- `benchmark_status: reserved`
- `gate_status: reserved`
- `cases: []`
- `required_molecule_proofs: []`

That state must be machine-visible. It is not green, not implied, and not
droppable.

## Code Quality Review

### DRY guardrails

Hard rule:

- benchmark validation, case projection, anti-laundering, and full-vs-partial
  rules live once in `spec-core`
- `spec-cli` only adapts command scope and serializes the shared output

### Minimal-diff guardrails

Keep the diff boring:

- one new shared module for benchmark logic
- one new authored registry file
- one additive field on status JSON
- one additive field on export JSON
- one schema-version bump across both surfaces

Do **not** add:

- a new config layer
- benchmark-specific CLI flags
- a benchmark cache
- a second registry format
- text-mode benchmark UI in I1

### Explicit-over-clever guardrails

Prefer explicit label accounting over heuristics:

- validate duplicate ids explicitly
- validate unknown carriers explicitly
- validate unlabeled active-root carriers explicitly
- derive status and gate state from explicit enums, not ad hoc booleans

## Implementation Plan

### Step 1: Add the registry and shared benchmark types

Touch:

- `benchmarks/labels.json`
- `spec-core/src/benchmarks.rs`
- `spec-core/src/lib.rs`

Implement:

- registry structs
- enum definitions
- registry parsing and validation
- repo-relative root and generated-root normalization
- canonical `label_digest` generation for full-scope entries

Exit condition:

- the registry loads from repo root
- malformed or contradictory registry data fails with clear diagnostics

### Step 2: Build the shared projection engine

Touch:

- `spec-core/src/benchmarks.rs`

Implement:

- benchmark root intersection logic
- full-vs-partial scope classification
- case projection from loaded specs and projected unit truth
- required-molecule projection from loaded molecule evidence
- anti-laundering credit rules
- benchmark status and gate-status derivation

Hard rule:

- the engine takes already loaded specs, molecule tests, passports, molecule
  evidence, and scope metadata as input
- the engine must not re-read passports or molecule evidence on its own

Exit condition:

- one caller contract can serve both `status` and `export`

### Step 3: Wire `spec status --format json`

Touch:

- `spec-cli/src/commands.rs`

Implement:

- bump `STATUS_JSON_SCHEMA_VERSION` from `3` to `4`
- add top-level `benchmarks: Vec<BenchmarkProjection>` to the JSON response
- load `benchmarks/labels.json` once per invocation
- call the shared projector using resolved scope plus current loaded truth
- keep text mode unchanged

Exit condition:

- status JSON emits additive `benchmarks[]` with honest full/partial behavior

### Step 4: Wire `spec export --format json`

Touch:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`

Implement:

- bump export schema version from `3` to `4`
- add top-level additive `benchmarks[]` to `ExportBundle`
- call the same shared projector used by status
- preserve all existing export fields unchanged aside from the additive field

Exit condition:

- status and export project the same benchmark truth for the same scope

### Step 5: Lock reserved, companion, and partial semantics

Touch:

- `spec-core/src/benchmarks.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/`

Implement targeted coverage for:

- full-scope `BENCH-SERVICE` reserved projection
- companion benchmark visibility
- companion benchmark supported carriers that still never count positive
- partial-scope zero-credit behavior
- unlabeled-carrier accounting invalidation

Exit condition:

- the fake-green paths are blocked by tests before the schema-v4 contract ships

### Step 6: Update fixtures and schema assertions

Touch:

- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/status-*.json`
- any new benchmark-aware fixture variants that make the contract clearer

Implement:

- benchmark-aware status fixture coverage
- benchmark-aware export fixture coverage
- schema version `4` assertions in `spec-cli/tests/cli.rs`

Exit condition:

- production code does not land without green schema-v4 fixture coverage

## Test Review

One hundred percent coverage is the goal for this wedge because it changes a
public machine contract.

### Test framework

- runtime: Rust
- unit tests: `cargo test -p spec-core`
- CLI integration tests: `cargo test -p spec-cli --test cli`

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] benchmarks/labels.json loading
    |
    ├── [GAP] Valid registry loads from repo root and normalizes roots/cases
    ├── [GAP] Duplicate benchmark id fails validation
    ├── [GAP] Duplicate case id or duplicate carrier mapping fails validation
    ├── [GAP] Unknown classification fails validation
    └── [GAP] Missing or malformed labels file surfaces command failure clearly

[+] Full-scope benchmark projection
    |
    ├── [GAP] Repo-root status emits BENCH-ECOM full
    ├── [GAP] Repo-root status emits BENCH-CROSSLIB full
    ├── [GAP] Repo-root status emits BENCH-SERVICE reserved full
    ├── [GAP] Ecommerce-root status emits BENCH-ECOM full but not BENCH-SERVICE
    └── [GAP] Export mirrors the same full-scope benchmark entries

[+] Partial-scope benchmark projection
    |
    ├── [GAP] Pricing-subdir status emits BENCH-ECOM partial only
    ├── [GAP] Single-file status emits only intersecting partial case(s)
    ├── [GAP] Partial entries omit benchmark_status, gate_status, and label_digest
    └── [GAP] Partial cases always emit counts_as_supported_positive=false

[+] Anti-laundering
    |
    ├── [GAP] Unlabeled active unit => accounting_status=invalid, no positive credit
    ├── [GAP] Unlabeled active partial scope => accounting_status=partial_invalid
    ├── [GAP] Deferred case stays visible but never counts green
    ├── [GAP] Fallback-backed case stays visible but never counts green
    ├── [GAP] Companion-negative case stays visible and never counts green
    └── [GAP] Companion benchmark can contain supported carriers without becoming positive credit

[+] Benchmark status and gate status
    |
    ├── [GAP] Positive full benchmark passing when supported cases + required molecules are valid
    ├── [GAP] Positive full benchmark incomplete when a supported case is stale/untested/incomplete
    ├── [GAP] Positive full benchmark failing when a supported case or required molecule fails
    ├── [GAP] Positive full benchmark invalid on accounting failure
    ├── [GAP] Companion benchmark passing when all cases emit and none counts positive
    └── [GAP] Reserved BENCH-SERVICE emits reserved gate state

[+] Public schema contract
    |
    ├── [GAP] status JSON fixture(s) bump to schema_version 4 with top-level benchmarks[]
    ├── [GAP] export JSON fixture(s) bump to schema_version 4 with top-level benchmarks[]
    └── [GAP] existing units, passports, molecule_tests, and graph surfaces remain unchanged aside from additive benchmarks[]

---------------------------------
COVERAGE: 0/24 benchmark paths tested today
GAPS: 24 benchmark paths need tests
CRITICAL: schema-v4 contract is entirely untested until I1 lands
---------------------------------
```

### Required tests to add

#### `spec-core` unit tests

Add focused tests for:

- registry normalization and validation
- active-root unlabeled invalidation
- partial-valid vs partial-invalid classification
- case-level `counts_as_supported_positive` rules
- benchmark status and gate-status transitions

#### `spec-cli` integration tests

Add CLI coverage for:

- repo-root full-scope status benchmark projection
- benchmark-root full-scope status benchmark projection
- nested-directory partial-scope status benchmark projection
- single-file partial-scope status benchmark projection
- full-scope export benchmark projection
- reserved `BENCH-SERVICE` visibility at broad scope only
- malformed or incomplete labels registry failure

#### Fixture updates

Update or add fixture coverage so the public machine contract is locked at
`schema_version: 4` for:

- status valid
- status incomplete or stale
- export valid

If current generic fixtures become too awkward to retrofit, add benchmark-aware
fixture variants instead of weakening assertions.

### Regression rule

Any test proving that a previously green JSON consumer no longer sees a stable
shape is a regression test and is mandatory. No debate.

### Test execution commands

Use these during implementation:

```bash
cargo test -p spec-core benchmark
cargo test -p spec-cli --test cli benchmark
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- export examples/crosslib-app --format json
```

## Performance Review

### Watchpoints

1. Do not rescan the filesystem per benchmark case.
   The projector should operate on already loaded specs, already read
   passports, already read molecule evidence, and already projected semantic
   truth.

2. Do not duplicate registry parsing inside status and export.
   Parse once per command invocation, then pass the shared registry into the
   projector.

3. Keep lookups map-based.
   Benchmark case matching should use normalized hash maps keyed by carrier id
   and benchmark id, not nested linear scans across every loaded spec and every
   benchmark case.

### Expected cost profile

`benchmarks/labels.json` is small and static. The new cost should be dominated
by existing proof loading, not by benchmark projection itself.

If benchmark projection adds noticeable latency to `status` or `export`, that
is a bug in the implementation shape.

## Failure Modes Registry

| Code path | Real failure | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| registry load | missing or malformed `benchmarks/labels.json` silently drops benchmark truth | yes | yes, fail command with clear benchmark-registry diagnostic | clear failure, never silent omission |
| active benchmark accounting | unlabeled authored carrier under active root still yields `valid` accounting | yes | yes, benchmark `invalid` or `partial_invalid` | clear invalid benchmark projection |
| partial scope | single-file query implies whole benchmark green | yes | yes, force `partial` and zero positive credit | honest partial projection |
| companion benchmark | companion-negative case disappears from projection | yes | yes | explicit non-native visibility |
| reserved benchmark | `BENCH-SERVICE` omitted from broad scope | yes | yes | explicit reserved gate state |
| status/export divergence | same benchmark root projects differently between `status` and `export` | yes | yes, one shared projector | one consistent machine contract |

Critical gap definition for I1:

- any path that can silently grant positive benchmark credit without full scope
  or without valid accounting is a critical gap

## NOT in Scope

| Deferred item | Why it is deferred |
| --- | --- |
| `spec benchmark snapshot <benchmark-id>` | belongs to I2, not the shared projection wedge |
| `benchmarks/snapshots/*.snapshot.json` | snapshot write surface is not part of I1 |
| `benchmarks/reviews/*.readability.review.json` | readability observation surface is I2 work |
| readability review loading on `status` and `export` | same reason, do not widen the read-side contract now |
| `projection_digest` | locked digest contract depends on snapshot/readability closure |
| `readability_review_status`, `readability_verdict`, `readability_generated_files[]` | all belong to the readability surface, not I1 |
| text-mode benchmark summaries | JSON contract first, text UI later |
| authored `BENCH-SERVICE` workload content | the reserved gate must be visible before the workload exists |
| benchmark scoring, history, or reporting dashboards | product stretch work, not contract foundation |

## What Already Exists

- the current status health engine already knows how to compute unit and
  molecule truth, reuse it
- the current export bundle already carries units, passports, molecule tests,
  and graph edges, extend it additively
- the current example roots already provide the positive and companion
  workloads I1 needs, do not invent a new fixture corpus
- the current path-scoped command behavior already separates file, directory,
  and repo-root queries, extend it with benchmark scope classification instead
  of writing a second scope model

## Worktree Parallelization Strategy

This plan has bounded parallelization room once the shared benchmark contract is
frozen. The shared `spec-core` benchmark API is the barrier. Everything before
that barrier can move in parallel only if it does not also touch `spec-core/src/`.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Registry authoring | `benchmarks/`, `examples/` | - |
| Shared benchmark projection core | `spec-core/src/` | - |
| Export bundle integration | `spec-core/src/`, `spec-cli/src/` | Shared benchmark projection core |
| Status JSON integration | `spec-cli/src/` | Shared benchmark projection core |
| Contract tests and fixtures | `spec-cli/tests/`, `spec-cli/tests/fixtures/` | Registry authoring, Export bundle integration, Status JSON integration |

### Parallel lanes

- **Lane A:** Registry authoring  
  Owns `benchmarks/` plus validation of the final roster against live example
  units and molecule IDs.

- **Lane B:** Shared benchmark projection core -> export bundle integration  
  Sequential in one lane because both steps touch `spec-core/src/` and should
  freeze one benchmark API before anything else depends on it.

- **Lane C:** Status JSON integration  
  Starts only after Lane B freezes the shared projector API. Mostly
  `spec-cli/src/` ownership.

- **Lane D:** Contract tests and fixtures  
  Starts after A, B, and C have stabilized the final schema-v4 shape.

### Execution order

1. Launch **Lane A** and the first half of **Lane B** in parallel:
   registry authoring plus the shared `spec-core` benchmark projector.
2. Finish **Lane B** export-bundle integration and freeze the shared benchmark
   projection API.
3. Launch **Lane C** after that API is stable.
4. Launch **Lane D** after A, B, and C are green enough to lock schema-v4
   fixtures.

### Conflict flags

- Lanes B and C both depend on the exact `BenchmarkProjection` shape. Do not
  let Lane C guess the API before Lane B freezes it.
- Lane A must settle the final case roster before Lane D writes golden
  fixtures.
- Export bundle integration touches both `spec-core/src/` and `spec-cli/src/`.
  Keep it with Lane B so the status-only lane stays narrow.

If the team only has one worker, run sequentially in the same order. That is
still fine.

## Acceptance Criteria

I1 is done only when all of these are true:

1. `benchmarks/labels.json` exists at repo root and validates
2. `spec status --format json` emits top-level `benchmarks[]` at
   `schema_version: 4`
3. `spec export --format json` emits top-level `benchmarks[]` at
   `schema_version: 4`
4. both commands share the same benchmark projection rules
5. full-scope benchmark queries emit honest benchmark status and gate status
6. partial-scope benchmark queries emit only partial entries and never positive
   credit
7. `BENCH-SERVICE` appears as reserved state at broad full scope
8. active-root unlabeled carriers force invalid accounting
9. companion-negative cases remain visible and never count positive
10. all new benchmark contract tests pass

## Completion Summary

- Step 0: Scope Challenge - scope accepted as-is, no widening beyond the locked I1 wedge
- Architecture Review: one shared benchmark projection core, no duplicated status/export logic
- Code Quality Review: explicit enums plus one shared module, no new subsystem
- Test Review: coverage diagram produced, 24 benchmark paths must be locked by tests
- Performance Review: keep projection map-based and reuse loaded truth
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none, the ladder already owns I2-I4 follow-ons
- Failure modes: all critical laundering paths explicitly covered in plan
- Parallelization: 4 lanes, 2 launchable in parallel at the start, then sequential convergence
- Lake Score: 5/5 decisions chose the complete version over a fake smaller shortcut
