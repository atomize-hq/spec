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

I1 lands one narrow but complete wedge:

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
in `I2`, exactly where the ladder puts them.

## Executive Summary

`M65` split the Rust V1 work into clean artifacts. `M66` froze the support
claim. `M67` froze the benchmark roster and the write-vs-read truth boundary.
`M68` closed the mechanics enough to code from.

The repo still has a hole between those frozen docs and the product:

- `spec status --format json` and `spec export` know unit truth and molecule
  truth, but they cannot project benchmark truth
- `BENCH-SERVICE` is a planning requirement, not a machine-visible gate
- path-scoped commands have no benchmark honesty boundary yet
- there is no label-driven anti-laundering wall for positive, deferred,
  fallback-backed, companion-negative, and reserved benchmark states

I1 fixes that hole without widening scope. It keeps the current proof writers
unchanged and adds one shared read-side benchmark projection engine that both
`status` and `export` call.

## Current Validated Baseline

Validated on `feat/m60-plus` at `3561bd1`.

### 1. Current machine surfaces are still schema version 3

Current code facts:

- `STATUS_JSON_SCHEMA_VERSION` is `3` in `spec-cli/src/commands.rs`
- `EXPORT_SCHEMA_VERSION` is `3` in `spec-core/src/export.rs`
- `spec-cli/tests/cli.rs` still asserts schema version `3` for both surfaces

So any benchmark landing that changes the public machine contract must bump both
surfaces together in the same milestone.

### 2. The repo already has the proof truth I1 needs to read

Current read/write wall:

- authored workload lives in `.unit.spec` and `.test.spec`
- unit proof truth lives in `.spec.passport.json`
- molecule proof truth lives in `*.test.evidence.json`
- `spec status` and `spec export` already project those truths but do not mint
  new proof

That means I1 does not need a new writer. It only needs a new reader-side
projection layer.

### 3. `examples/ecommerce` is already a real positive benchmark candidate

Current direct status truth from:

```bash
cargo run -p spec-cli -- status examples/ecommerce --format json
```

Current result:

- all seven authored ecommerce units are `valid`
- all three ecommerce molecule tests are `valid`
- every ecommerce unit currently projects supported narrow-core truth
- there is no benchmark projection today

That makes `examples/ecommerce` the right active positive benchmark root for I1.

### 4. `examples/crosslib-app` already contains the companion negative-proof case

Current direct status truth from:

```bash
cargo run -p spec-cli -- status examples/crosslib-app --format json
```

Current result:

- `pricing/checkout_nested_chain3` is `valid`
- its semantic review projects `support_status: unsupported`
- its `unsupported_reason_codes` include `unsupported_dep_topology`
- the other authored cross-library units remain visible alongside it

That makes `examples/crosslib-app` the right companion benchmark root for I1.

### 5. Scope logic already exists, but benchmark logic does not

Current code already has:

- `resolve_status_roots()` in `spec-cli/src/commands.rs`
- existing file-vs-directory-vs-root scope behavior for `status`
- existing path-scoped `export` behavior

Current code does **not** have:

- benchmark registry loading
- benchmark classification validation
- benchmark-level enums
- full-vs-partial benchmark scope projection
- additive top-level `benchmarks[]`

So the correct move is to extend current scope logic, not replace it.

## Problem Statement

The Rust V1 docs now know what the benchmark layer should mean, but the machine
surfaces do not expose it yet.

That gap creates three bad outcomes:

1. downstream tooling cannot ask benchmark-level questions from the public JSON
   surfaces
2. reserved required proof like `BENCH-SERVICE` can disappear from machine
   output and get forgotten
3. once benchmarks exist, path-scoped commands can easily lie by implying
   whole-benchmark green state from a single-unit query

I1 closes those gaps by making benchmark truth label-driven, shared, additive,
and scope-honest.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner | I1 action |
| --- | --- | --- |
| status scope resolution | `resolve_status_roots()` in `spec-cli/src/commands.rs` | reuse and extend with benchmark root intersection plus full/partial benchmark scope classification |
| unit health projection | `compute_health_status()`, `apply_semantic_review_to_health()`, escape-hatch projection in `spec-cli/src/commands.rs` | reuse as the case-level proof truth source |
| export bundle projection | `spec-core/src/export.rs` | reuse and extend with additive top-level `benchmarks[]` |
| passport + molecule truth projection | `spec-core/src/passport.rs`, `spec-core/src/molecule_evidence.rs` | reuse for case proof refs and required-molecule proof state |
| semantic support truth | `spec-core/src/semantic_review.rs` and projected passport truth | reuse for `semantic_support_status` and anti-laundering credit rules |
| positive workload example | `examples/ecommerce/units` | use as active `BENCH-ECOM` benchmark root |
| companion negative-proof example | `examples/crosslib-app/units` | use as active `BENCH-CROSSLIB` benchmark root |
| JSON contract coverage | `spec-cli/tests/cli.rs` plus fixture JSON under `spec-cli/tests/fixtures/` | bump to schema version 4 and add benchmark assertions in the same wedge |

### Minimum complete slice

The minimum honest I1 slice is:

1. add repo-root `benchmarks/labels.json` and validate it as authoritative
   benchmark-accounting input
2. add one shared benchmark projection module in `spec-core`
3. add explicit benchmark enums and projection structs in that shared module
4. project full vs partial benchmark scope honestly for both `status` and
   `export`
5. emit additive top-level `benchmarks[]` from `spec status --format json`
6. emit additive top-level `benchmarks[]` from `spec export`
7. surface `BENCH-SERVICE` as explicit `reserved` machine state at broad
   full-scope queries
8. add anti-laundering rules so partial, deferred, fallback-backed,
   companion-negative, and reserved cases never mint positive benchmark credit
9. update contract tests and fixtures in the same milestone

Anything smaller is fake done.

Examples:

- adding benchmark logic only to `status` but not `export` is fake done
- adding `benchmarks[]` without full-vs-partial honesty is fake done
- adding labels without unlabeled-root invalidation is fake done
- projecting `BENCH-SERVICE` only in docs and not in machine JSON is fake done

### Complexity check

This wedge touches more than one production surface, but it is still the
minimum engineered slice:

- one new shared production module in `spec-core`
- one existing status path in `spec-cli`
- one existing export path in `spec-core` plus export command wiring
- one authored registry file
- one test surface

That is acceptable. It is not spending an innovation token on a new subsystem.
It is boring extension work on the right seams.

### Search check

I1 should stay Layer 1 and Layer 3, not Layer 2 novelty:

- **[Layer 1]** Reuse existing `status` root resolution, passport projection,
  molecule evidence loading, and export bundle assembly.
- **[Layer 1]** Keep serialization on existing `serde`/`serde_json` machinery.
- **[Layer 3]** Do not infer benchmark truth from directory discovery. Use the
  explicit authored registry because benchmark accounting is product truth, not
  filesystem folklore.

No new config layer, no background cache, no benchmark-specific database, no
auto-discovery magic.

### TODOS cross-reference

Current `TODOS.md` has no blocking item for this wedge.

Important non-blocking consequence:

- I2, I3, and I4 already exist in the implementation ladder and should remain
  ladder-owned follow-ons rather than new ad hoc TODOs

### Completeness check

The complete I1 version is still cheap enough to do now:

- both JSON surfaces move together
- full and partial benchmark scope land together
- reserved gate visibility lands together
- anti-laundering rules land together
- fixtures and tests move in the same commit

Trying to land a half-version would only save minutes and would create a second
contract migration later. Not worth it.

### Distribution check

No new artifact type is introduced for end users.

`benchmarks/labels.json` is authored repo input, not a distributable product
artifact. Distribution pipeline changes are out of scope.

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
                - apply anti-laundering
                - project benchmark/gate status
                     /                       \
                    v                         v
     spec status --format json         spec export --format json
       top-level benchmarks[]            top-level benchmarks[]
```

That shared projector is the whole architecture move. Do not duplicate
benchmark logic separately in `commands.rs` and `export.rs`.

### Module boundaries

| Module / surface | Ownership in I1 | Notes |
| --- | --- | --- |
| `benchmarks/labels.json` | new authored source of benchmark-accounting truth | repo-root, checked in |
| `spec-core/src/benchmarks.rs` | new shared benchmark registry + projection engine | the only place allowed to know benchmark accounting rules |
| `spec-core/src/lib.rs` | export the benchmark module publicly to the CLI crate | thin wiring only |
| `spec-core/src/export.rs` | add additive top-level `benchmarks[]` to `ExportBundle` | reuse shared projector, do not fork rules |
| `spec-cli/src/commands.rs` | load registry, call projector for status JSON, bump schema version | text mode remains non-blocking and unchanged |
| `spec-cli/tests/cli.rs` + fixtures | contract coverage for schema v4 and benchmark projection | full + partial + reserved + invalid cases |

### Concrete shared types

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

### Exact benchmark registry for I1

`benchmarks/labels.json` should be added at repo root with three benchmark
entries.

#### 1. `BENCH-ECOM`

Role:

- active positive benchmark

Roots:

- `root = "examples/ecommerce/units"`
- `generated_root = "examples/ecommerce/src/generated"`

Required molecules:

- `pricing/checkout_flow`
- `pricing/discount_plus_tax`
- `pricing/discount_strategy_checkout_flow`

Initial labeled unit cases:

- `money/round` -> `supported`
- `pricing/apply_discount` -> `supported`
- `pricing/apply_tax` -> `supported`
- `pricing/calculate_total` -> `supported`
- `pricing/calculate_total_guarded_tax` -> `supported`
- `pricing/discount_strategy` -> `supported`
- `pricing/pricing_quote` -> `supported`

#### 2. `BENCH-CROSSLIB`

Role:

- active companion negative-proof benchmark

Roots:

- `root = "examples/crosslib-app/units"`
- `generated_root = "examples/crosslib-app/src/generated"`

Required molecules:

- none

Initial labeled unit cases:

- `pricing/apply_discount` -> `supported`
- `pricing/apply_tax` -> `supported`
- `pricing/calculate_total` -> `supported`
- `pricing/checkout_nested_chain3` -> `companion_negative_proof`

Why label the supported cross-library carriers too:

- because `BENCH-CROSSLIB` is an active benchmark root
- because unlabeled authored carriers under an active benchmark root must make
  accounting invalid
- because only the nested chain3 unit is the companion-negative case, but the
  other carriers still have to be explicitly accounted for

#### 3. `BENCH-SERVICE`

Role:

- reserved positive benchmark required for final V1 closure

Roots:

- `root = "examples/service/units"`
- `generated_root = "examples/service/src/generated"`

Required molecules:

- none

Initial labeled unit cases:

- empty, because the benchmark is reserved

### Exact read-surface contract for I1

I1 is the shared projection wedge, not the snapshot/readability wedge.

So the benchmark JSON contract for I1 is:

- **yes now**:
  - top-level `benchmarks[]`
  - benchmark kind/lifecycle/path-scope/accounting status
  - benchmark status and gate status where full scope makes them honest
  - case projection with proof refs and anti-laundering credit bit
  - required molecule proof projection
  - reserved `BENCH-SERVICE` state
  - schema version bump to 4
- **not yet in I1**:
  - `spec benchmark snapshot`
  - `projection_digest`
  - readability review loading
  - readability verdict/status
  - `readability_generated_files[]`

That means:

- I1 should compute and emit `label_digest` for full-scope entries because it
  is purely registry-owned and audit-friendly
- I1 should **not** emit `projection_digest` yet because the locked digest
  contract includes readability closure that I2 owns

### Exact path-scope behavior

Use the existing command scope and add benchmark intersection rules on top.

Full-scope examples:

- repo-root `spec status . --format json`
- `spec status examples/ecommerce --format json`
- `spec status examples/ecommerce/units --format json`
- `spec export examples/crosslib-app --format json`

Partial-scope examples:

- `spec status examples/ecommerce/units/pricing --format json`
- `spec status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json`
- `spec export examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --format json`

Required rules:

1. benchmarks are emitted only when the command scope intersects their declared
   `root`
2. full scope means the command loaded the entire declared benchmark root
3. partial scope means the command intersects but did not load the whole root
4. every partial case emits `counts_as_supported_positive: false`
5. partial entries omit benchmark-level green-state claims:
   `benchmark_status`, `gate_status`, benchmark summary
6. `BENCH-SERVICE` appears only when the query scope is broad enough to contain
   its declared root, which means repo-root or repo-ancestor queries, not
   narrow ecommerce-only queries

### Reserved benchmark state

At full scope, `BENCH-SERVICE` must project:

- `lifecycle: reserved`
- `path_scope: full`
- `accounting_status: reserved_missing_cases`
- `benchmark_status: reserved`
- `gate_status: reserved`
- `cases: []`
- `required_molecule_proofs: []`

That state must be machine-visible. It is not green, not open-by-implication,
and not droppable.

## Code Quality Review

### DRY requirements

I1 must avoid the classic trap where `status` and `export` each grow their own
slightly different benchmark logic.

Hard rule:

- benchmark validation, case projection, anti-laundering, and full-vs-partial
  rules live once in `spec-core`
- `spec-cli` only adapts command scope and serializes the shared output

### Minimal-diff requirements

Keep the diff boring:

- one new module for benchmark logic
- one new authored registry file
- one additive field on status JSON
- one additive field on export JSON
- one schema version bump

Do **not** add:

- a new config layer
- benchmark-specific CLI flags
- a benchmark cache
- a second registry format
- text-mode benchmark UI in the first wedge

### Explicit-over-clever requirements

Prefer explicit label accounting over inferred heuristics:

- validate duplicate benchmark ids explicitly
- validate duplicate `case_id` and duplicate carrier mapping explicitly
- validate unknown carrier ids explicitly
- validate active-root unlabeled carriers explicitly

Do not try to infer benchmark health from directory names or proof timestamps
alone.

## Test Review

100 percent coverage is the goal for the new benchmark logic because this is a
public machine contract wedge.

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
    └── [GAP] Missing / malformed labels file surfaces command failure clearly

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
    ├── [GAP] Partial entries omit benchmark_status, gate_status, and summary
    └── [GAP] Partial cases always emit counts_as_supported_positive=false

[+] Anti-laundering
    |
    ├── [GAP] Unlabeled active positive unit => accounting_status=invalid, no positive credit
    ├── [GAP] Unlabeled active partial scope => accounting_status=partial_invalid
    ├── [GAP] Deferred case stays visible but never counts green
    ├── [GAP] Fallback-backed case stays visible but never counts green
    ├── [GAP] Companion-negative case stays visible and never counts green
    └── [GAP] Companion benchmark can contain supported carriers without entering positive credit

[+] Benchmark status / gate status
    |
    ├── [GAP] Positive full benchmark passing when supported cases + required molecules are valid
    ├── [GAP] Positive full benchmark incomplete when a supported case is stale/untested/incomplete
    ├── [GAP] Positive full benchmark invalid on accounting failure
    ├── [GAP] Companion benchmark passing when all cases emit and none count positive
    └── [GAP] Reserved BENCH-SERVICE emits reserved gate state

[+] Public schema contract
    |
    ├── [GAP] status JSON fixture(s) bump to schema_version 4 with top-level benchmarks[]
    ├── [GAP] export JSON fixture(s) bump to schema_version 4 with top-level benchmarks[]
    └── [GAP] existing units/passports/graph surfaces remain unchanged aside from additive benchmarks[]

---------------------------------
COVERAGE: 0/23 benchmark paths tested today
GAPS: 23 benchmark paths need tests
CRITICAL: schema-v4 contract is entirely untested until I1 lands
---------------------------------
```

### Required tests to add

#### `spec-core` unit tests

Add focused tests in the new benchmark module for:

- registry normalization and validation
- active-root unlabeled invalidation
- partial-valid vs partial-invalid classification
- case-level `counts_as_supported_positive` rules
- benchmark status and gate status transitions

#### `spec-cli` integration tests

Add CLI coverage in `spec-cli/tests/cli.rs` for:

- repo-root full-scope status benchmark projection
- benchmark-root full-scope status benchmark projection
- nested-directory partial-scope status benchmark projection
- single-file partial-scope status benchmark projection
- full-scope export benchmark projection
- reserved `BENCH-SERVICE` visibility at broad scope only
- malformed or incomplete labels registry failure

#### Fixture updates

Update or add JSON fixtures so the public machine contract is locked at
`schema_version: 4` for:

- status valid
- status untested or incomplete
- export valid

If existing generic fixtures become too awkward to retrofit, add benchmark-aware
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

1. **Do not rescan the filesystem per benchmark case.**
   The projector should operate on already loaded specs, already read passports,
   already read molecule evidence, and already projected semantic truth.

2. **Do not duplicate registry parsing inside status and export.**
   Parse once per command invocation, then pass the shared registry into the
   projector.

3. **Keep lookups map-based.**
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
| active benchmark accounting | unlabeled authored carrier under active root still yields `valid` accounting | yes | yes, benchmark `invalid` / `partial_invalid` | clear invalid benchmark projection |
| partial scope | single-file query implies whole benchmark green | yes | yes, force `partial` and zero positive credit | honest partial projection |
| companion benchmark | companion-negative case disappears from projection | yes | yes | explicit non-native visibility |
| reserved benchmark | `BENCH-SERVICE` omitted from broad scope | yes | yes | explicit reserved gate state |
| status/export divergence | same benchmark root projects differently between `status` and `export` | yes | yes, one shared projector | one consistent machine contract |

Critical gap definition for I1:

- any path that can silently grant positive benchmark credit without full scope
  or without valid accounting is a critical gap

This plan closes those gaps by construction.

## Not in Scope

- `spec benchmark snapshot <benchmark-id>`
- `benchmarks/snapshots/*.snapshot.json`
- `benchmarks/reviews/*.readability.review.json`
- readability review loading on `status` / `export`
- `projection_digest`
- `readability_review_status`
- `readability_verdict`
- `readability_generated_files[]`
- text-mode benchmark summaries
- authored `BENCH-SERVICE` workload content
- benchmark scoring, history, or reporting dashboards

Rationale:

- those belong to `I2` and later by the locked ladder
- forcing them into I1 would turn a clean read-surface wedge into a subsystem
  rewrite

## Implementation Plan

### Step 1: Add the benchmark registry and shared types

Add:

- `benchmarks/labels.json`
- `spec-core/src/benchmarks.rs`
- `pub mod benchmarks;` in `spec-core/src/lib.rs`

Implement:

- registry structs
- enum definitions
- registry parsing and validation
- repo-relative root and generated-root normalization
- `label_digest` canonicalization for full-scope entries

### Step 2: Build the shared projection engine

In `spec-core/src/benchmarks.rs`, implement:

- benchmark root intersection logic
- full vs partial scope classification
- case projection from loaded specs and projected proof truth
- required-molecule proof projection from loaded molecule evidence
- anti-laundering rules
- benchmark status and gate status derivation

Hard rule:

- this engine must take already loaded specs/tests/passports/evidence as input
- it must not re-read passports or molecule evidence on its own

### Step 3: Wire `spec status --format json`

In `spec-cli/src/commands.rs`:

- bump `STATUS_JSON_SCHEMA_VERSION` from `3` to `4`
- add top-level `benchmarks: Vec<BenchmarkProjection>` to the JSON response
- load `benchmarks/labels.json` once for the invocation
- call the shared projector using the resolved scope plus currently loaded truth
- keep text mode unchanged in I1

### Step 4: Wire `spec export`

In `spec-core/src/export.rs` and `spec-cli/src/commands.rs`:

- bump export schema version from `3` to `4`
- add additive top-level `benchmarks[]` to `ExportBundle`
- call the same shared projector used by `status`
- keep all existing export surfaces stable aside from the additive field

### Step 5: Lock reserved and companion semantics

Add targeted coverage for:

- full-scope `BENCH-SERVICE` reserved projection
- companion benchmark visibility
- companion benchmark supported carriers that still never count positive
- partial-scope zero-credit behavior

### Step 6: Update fixtures and contract tests

Update:

- benchmark-aware status fixture coverage
- benchmark-aware export fixture coverage
- schema version assertions in `spec-cli/tests/cli.rs`

Do not land production code before fixture and schema tests are green.

## What Already Exists

- The current status health engine already knows how to compute unit and
  molecule truth. Reuse it.
- The current export bundle already carries units, passports, molecule tests,
  and graph edges. Extend it additively.
- The current example roots already provide the positive and companion workloads
  I1 needs. Do not invent a new fixture corpus.
- The current path-scoped command behavior already separates file, directory,
  and repo-root queries. Extend it with benchmark scope classification instead
  of writing a second scope model.

## Worktree Parallelization Strategy

This plan does have bounded parallelization room once the shared benchmark
contract is frozen.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Registry authoring | `benchmarks/`, `examples/` | - |
| Shared benchmark projection core | `spec-core/src/` | - |
| Status JSON wiring | `spec-cli/src/` | Shared benchmark projection core |
| Export JSON wiring | `spec-core/src/`, `spec-cli/src/` | Shared benchmark projection core |
| Contract tests and fixtures | `spec-cli/tests/`, `spec-cli/tests/fixtures/` | Registry authoring, Status JSON wiring, Export JSON wiring |

### Parallel lanes

- **Lane A:** Registry authoring  
  `benchmarks/labels.json` plus final case roster validation against live
  example truth.

- **Lane B:** Shared projection core -> Export JSON wiring  
  Sequential because both touch `spec-core/src/`.

- **Lane C:** Status JSON wiring  
  Can run after Lane B exposes the shared projection API. Mostly `spec-cli/src/`
  ownership.

- **Lane D:** Contract tests and fixtures  
  Launch after B + C stabilize the machine shape.

### Execution order

1. Launch **Lane A** and the first half of **Lane B** in parallel:
   registry authoring plus shared `spec-core` projection core.
2. Once the shared projection API is stable, launch **Lane C**.
3. Finish **Lane B** export wiring.
4. Launch **Lane D** after B + C are both green enough to freeze schema v4.

### Conflict flags

- Lanes B and C both depend on the exact benchmark projection type shape.
  Freeze the `spec-core` interface before deep CLI assertions.
- Lane A must settle the final case roster before Lane D golden fixtures lock.
- Export wiring shares `spec-core/src/` with the shared projection core, so keep
  those two steps in the same lane to avoid merge churn.

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
- Test Review: coverage diagram produced, 23 benchmark paths must be locked by tests
- Performance Review: keep projection map-based and reuse loaded truth
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none, the ladder already owns I2-I4 follow-ons
- Failure modes: all critical laundering paths explicitly covered in plan
- Parallelization: 4 lanes, with bounded overlap after the shared core freezes
- Lake Score: 5/5 decisions chose the complete version over a fake smaller shortcut
