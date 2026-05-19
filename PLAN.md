# I3: Rust V1 Contract Stack Mechanics Landing Plan

Status: **authoritative implementation plan**
Iteration: **I3**
Milestone family: **Rust V1 benchmark and truth-surface mechanics**
Implementation readiness: **ready for implementation**
Plan scope: **land the locked M68 mechanics surface on top of the live M61-era repo baseline: benchmark registry, shared benchmark projection core, schema-v4 `spec status --format json`, schema-v4 `spec export`, benchmark snapshots, readability review anchoring, reserved-gate projection for `BENCH-SERVICE`, and the exact anti-laundering/path-scope rules. Preserve all M66 support boundaries and keep M69 supported-core closure out of scope.**
Base branch: **main**
Working branch: **main**
Validated at commit: **`3af2526`**
Last rewritten: **2026-05-18**

Supersedes:

- the prior I2 plan previously maintained at this path
- `ORCH_PLAN.md` as historical execution context only

Locked authority inputs:

- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`

Historical context, not authority:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `ORCH_PLAN.md`

Primary repo surfaces:

- `spec-core/src/lib.rs`
- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/graph.rs`
- `spec-core/src/types.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/*.json`
- `examples/ecommerce/units/**`
- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Executive Summary

The locked M65-M68 stack already answered the strategic questions.

`M65` split the artifact family. `M66` locked the narrow-core support rows.
`M67` locked the benchmark roster and writer-versus-reader wall. `M68` locked
the mechanics. What is still missing is the code.

That is the whole I3 job.

I3 is not a new product direction. It is the live-code landing of the already
locked benchmark and truth-surface contract:

```text
one benchmark registry,
one shared benchmark projection core,
one schema-v4 benchmark surface in status and export,
one read-only snapshot command,
one readability-review anchor surface,
and one exact reserved-gate story for BENCH-SERVICE
```

I3 does not widen M66 support rows. It does not implement `BENCH-SERVICE`. It
does not reopen bounded generics, async/IO, or any larger Rust V1 claim.

## Frozen Implementation Decisions

These are inherited from M65-M68 and are locked for I3.

1. **M66 remains the only source of truth for supported, deferred, and explicitly-out Rust rows and interactions.**
   - I3 may not widen or narrow those rows.
   - I3 may not reinterpret fallback policy.

2. **M67 remains the only source of truth for benchmark roles.**
   - `BENCH-ECOM` is the only active positive benchmark in I3.
   - `BENCH-SERVICE` remains `reserved` and required for final V1 proof.
   - `BENCH-CROSSLIB` remains `companion_negative_proof`, never positive credit.

3. **Benchmark accounting is label-driven, not discovered ad hoc.**
   - `benchmarks/labels.json` is authoritative for benchmark membership and classification.
   - Unlabeled, duplicated, or unknown active-benchmark carriers are accounting failures.

4. **Proof writers stay where they already are.**
   - `.spec.passport.json` remains the authoritative unit proof write surface.
   - `*.test.evidence.json` remains the authoritative molecule proof write surface.
   - `spec status`, `spec export`, and `spec benchmark snapshot` remain read/projection surfaces only.

5. **No benchmark metadata is authored inside workload specs or proof artifacts in I3.**
   - no benchmark fields in `.unit.spec`
   - no benchmark fields in `.test.spec`
   - no benchmark fields in passports
   - no benchmark fields in molecule evidence

6. **Path scope is explicit.**
   - benchmark projection is `full` only when the command loads the entire benchmark root
   - otherwise it is `partial`
   - partial projections never mint positive credit

7. **Reserved-state visibility is mandatory.**
   - repo-broad `spec status . --format json` and repo-broad `spec export` must surface `BENCH-SERVICE`
   - reserved state may not collapse into silence, “missing data”, or implied green

8. **Readability is benchmark-scoped human observation, not proof truth.**
   - positive benchmarks only
   - tied to `projection_digest`
   - never stored in passports or molecule evidence

9. **The shared generated tree stays shared.**
   - readability scope includes shared `mod.rs` and benchmark-relevant `molecule_tests.rs`
   - there is no fake per-benchmark generated-tree split

10. **Public machine-contract bumps are part of the milestone.**
    - `spec status --format json`: `schema_version 3 -> 4`
    - `spec export`: `schema_version 3 -> 4`

## Current Validated Basis

Validated on `main` at `3af2526`.

Observed repo truth:

- `spec-cli/src/commands.rs` currently owns `spec status --format json` and exposes `schema_version: 3`.
- `spec-core/src/export.rs` currently owns `spec export` bundle assembly and exposes `schema_version: 3`.
- `spec-core/src/passport.rs` already projects current unit proof truth, freshness, portability, and semantic review.
- `spec-core/src/molecule_evidence.rs` already persists and reads molecule proof truth.
- `spec-core/src/graph.rs` already provides a typed local graph surface that status/export/projection work can reuse.
- `spec-cli/tests/cli.rs` already carries end-to-end CLI contract tests and fixture-backed schema assertions.
- `examples/ecommerce/units/**` already provides the positive benchmark workload candidates:
  - `money/round`
  - `pricing/apply_discount`
  - `pricing/apply_tax`
  - `pricing/calculate_total`
  - `pricing/calculate_total_guarded_tax`
  - `pricing/discount_strategy`
  - `pricing/pricing_quote`
  - molecule tests `pricing/checkout_flow`, `pricing/discount_plus_tax`, and non-required `pricing/discount_strategy_checkout_flow`
- `examples/crosslib-app/units/**` already provides the maintained companion-negative workload candidates:
  - `pricing/apply_discount`
  - `pricing/apply_tax`
  - `pricing/calculate_total`
  - `pricing/checkout_nested_chain3`
- `examples/shared-spec/units/**` already provides supporting sibling-library inputs that the cross-library workload depends on.
- `benchmarks/` does not exist today.
- there is no benchmark registry today.
- there is no benchmark projection module today.
- there is no benchmark snapshot command today.
- there is no readability review artifact today.
- `README.md` and `CHANGELOG.md` still describe the shipped surface as M61-era TypeScript and family work, not benchmark mechanics.

The gap is therefore structural and implementation-level, not product-discovery-level.

## Step 0: Scope Challenge

### Premise correction

The problem is not “finish Rust V1.”

The problem is narrower:

```text
the repo already has authored workload truth, unit proof truth,
molecule proof truth, semantic projection, and machine-readable status/export,
but it still lacks the benchmark-accounting and read-side projection layer
that makes the Rust V1 contract observable without laundering partial scope,
fallback-backed cases, or reserved gates into fake green state
```

If I3 expands beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | I3 action |
| --- | --- | --- |
| authored workload discovery | `spec-core::loader`, `spec-cli/src/commands.rs` | reuse; do not invent a second discovery model |
| unit proof projection | `spec-core/src/passport.rs` | reuse; benchmark code reads projected truth only |
| molecule proof projection | `spec-core/src/molecule_evidence.rs` plus CLI status paths | reuse; benchmark code reads current molecule truth only |
| machine export bundle | `spec-core/src/export.rs` | extend; append benchmark projection instead of inventing a second export artifact |
| status JSON emission | `spec-cli/src/commands.rs` | extend to schema v4 with additive `benchmarks[]` |
| local graph traversal | `spec-core/src/graph.rs` | reuse for benchmark case iteration and related coverage lookups where helpful |
| proof digests / hash style | `spec-core/src/passport.rs`, `spec-core/src/molecule_evidence.rs`, `spec-core/src/portability.rs` | reuse the existing `sha256:` digest convention |
| benchmark input workloads | `examples/ecommerce`, `examples/crosslib-app`, `examples/shared-spec` | reuse; do not author new benchmark workloads in I3 |
| CLI fixture contract testing | `spec-cli/tests/cli.rs`, `spec-cli/tests/fixtures/*.json` | extend; do not invent a parallel benchmark-only harness |

### Minimum complete slice

The minimum honest I3 slice is:

1. add `benchmarks/labels.json` with strict typed validation
2. add one shared `spec-core` benchmark projection module
3. add deterministic `label_digest` and `projection_digest`
4. add exact path-scope `full` versus `partial` behavior with anti-laundering
5. add schema-v4 additive `benchmarks[]` to `spec status --format json`
6. add schema-v4 additive `benchmarks[]` to `spec export`
7. add `spec benchmark snapshot <benchmark-id>` as a read-only derived writer
8. add readability review artifact loading and projection for positive full-scope benchmarks
9. seed the repo with the initial benchmark registry, readability review anchor, and canonical snapshots
10. add fixture-backed CLI coverage for full, partial, invalid, reserved, and companion-negative cases

Anything smaller is fake done.

Examples:

- adding `benchmarks/labels.json` without shared projection code is fake done
- adding schema-v4 output without the snapshot/readability/digest contract is fake done
- adding a snapshot command without the strict writer-versus-reader wall is fake done
- adding benchmark JSON without reserved `BENCH-SERVICE` visibility is fake done

### Complexity and blast radius

This plan crosses the 8-file smell threshold.

That is acceptable here because the extra files are contract surfaces, not subsystem vanity:

- one new repo-root benchmark artifact tree under `benchmarks/`
- one new shared benchmark module in `spec-core`
- one `spec-core` export integration surface
- one CLI status/export/snapshot integration surface
- one CLI test file expansion
- multiple JSON fixture baselines
- docs sync in `README.md`, `CHANGELOG.md`, and `TODOS.md`

The smaller shortcut is the dangerous version here. It would leave the repo with
another plan that sounds precise while the live code still cannot tell the truth
about benchmarks.

### Search check

Search unavailable for external framework advice, so this plan stays inside live repo truth and known Rust CLI architecture.

In-repo first-principles conclusions:

- **[Layer 1]** Reuse existing loaders, passports, molecule evidence, and CLI fixture harness.
- **[Layer 1]** Reuse the repo’s existing `sha256:` digest convention.
- **[Layer 3]** Do not build a benchmark subsystem. Build one shared projection module and let status, export, and snapshot consume it.

### TODOS cross-reference

`TODOS.md` does not currently carry an honest benchmark-mechanics closure step.

I3 should end with:

- the M68 mechanics landing represented in live code
- `M69 supported-core closure` still explicitly open
- `BENCH-SERVICE` still explicitly reserved and unimplemented

### Completeness and distribution check

No new external distribution surface is introduced.

This remains a CLI and JSON-contract milestone inside the existing `spec`
binary. Completeness here means:

- every benchmark role is explicit
- every broad-scope and narrow-scope projection is honest
- every schema bump is fixture-backed
- every writer-versus-reader boundary is enforced in code and docs

## Architecture Review

### Proposed module shape

| Surface | Action | Why |
| --- | --- | --- |
| `spec-core/src/benchmark.rs` | new | one shared typed home for registry parsing, enums, case projection, path-scope evaluation, digest payloads, readability scope, and snapshot records |
| `spec-core/src/lib.rs` | extend | export the benchmark module to CLI/tests |
| `spec-core/src/export.rs` | extend | append additive benchmark projection into the existing export bundle and bump schema to 4 |
| `spec-cli/src/commands.rs` | extend | own status emission, export wiring, and `benchmark snapshot` command orchestration |
| `spec-cli/tests/cli.rs` | extend | own end-to-end CLI contract coverage for schema-v4 and snapshot behavior |
| `spec-cli/tests/fixtures/*.json` | add/update | freeze full, partial, reserved, and invalid machine surfaces |
| `benchmarks/labels.json` | new | authoritative benchmark accounting input that the projection layer reads |
| `benchmarks/reviews/BENCH-ECOM.readability.review.json` | new | authored readability anchor for the positive benchmark |
| `benchmarks/snapshots/*.snapshot.json` | new | canonical derived full-benchmark projections committed as inspectable truth artifacts |

### Dependency graph

```text
benchmarks/labels.json
        │
        ▼
+---------------------------+
| spec-core::benchmark      |
| - registry types          |
| - label validation        |
| - case projection         |
| - path-scope evaluation   |
| - digest payloads         |
| - readability scope       |
+---------------------------+
        │                 │
        │                 ├───────────────► benchmarks/reviews/*.readability.review.json
        │
        ├───────────────► spec-core/src/export.rs
        │                    │
        │                    ▼
        │               spec export (schema v4)
        │
        └───────────────► spec-cli/src/commands.rs
                             │
                             ├────────────► spec status --format json (schema v4)
                             └────────────► spec benchmark snapshot <id>
                                              │
                                              ▼
                                   benchmarks/snapshots/*.snapshot.json
```

### Shared projection flow

`spec status --format json` and `spec export`:

```text
authored specs + benchmark labels + passports + molecule evidence + readability review
        │
        ▼
shared benchmark projection core
        │
        ├── full benchmark root loaded? yes ──► full projection with status/gate/digests/summary
        │
        └── no ──────────────────────────────► partial projection with no positive credit
```

`spec benchmark snapshot <benchmark-id>`:

```text
benchmark id
   │
   ▼
load full benchmark root + labels + current proof artifacts + readability review
   │
   ▼
shared benchmark projection core (must produce full scope)
   │
   ▼
write one snapshot file only
```

### Exact projection contract

This is the core ambiguity killer. The benchmark core must expose one benchmark
projection model, but that model has two legal shapes depending on path scope.

| Projection shape | Must include | Must omit |
| --- | --- | --- |
| `path_scope: full` | `benchmark_id`, `kind`, `lifecycle`, `required_for_v1`, `root`, `generated_root`, `path_scope`, `accounting_status`, `benchmark_status`, `gate_status`, `label_digest`, `projection_digest`, case projections, benchmark summary, readability review status, readability verdict when present, `readability_generated_files[]` when applicable | nothing from the M68 full-scope contract |
| `path_scope: partial` | `benchmark_id`, `kind`, `lifecycle`, `required_for_v1`, `root`, `generated_root`, `path_scope`, `accounting_status`, intersecting case projections with `counts_as_supported_positive: false` | `benchmark_status`, `gate_status`, `label_digest`, `projection_digest`, benchmark summary, readability review status, readability verdict, `readability_generated_files[]` |

If a partial projection carries any whole-benchmark health or readability claim,
the implementation is wrong.

### Snapshot contract

`spec benchmark snapshot <benchmark-id>` is not a second benchmark subsystem. It
is a single derived writer over the same full-scope projection.

Hard rules:

1. snapshotting always forces full-scope projection
2. if full scope cannot be loaded, the command fails instead of degrading to partial
3. the only write target is `benchmarks/snapshots/<BENCHMARK_ID>.snapshot.json`
4. the command never writes passports, molecule evidence, or readability review files
5. reserved `BENCH-SERVICE` writes the reserved snapshot state without inventing fake cases or generated files

### Architecture decisions

1. **One shared benchmark core, not duplicated status/export logic.**
   - Status, export, and snapshot must consume the same types and projection function.

2. **Keep snapshot writing in CLI, not `spec-core`.**
   - `spec-core` stays projection and validation heavy.
   - `spec-cli` already owns filesystem-facing orchestration.

3. **Seed the actual registry early, not at the very end.**
   - `benchmarks/labels.json` is an input contract, not a finishing touch.
   - Phase 1 should land the real file shape so every downstream phase tests against the real registry contract.

4. **Commit the canonical benchmark artifacts.**
   - This repo already commits derived proof-adjacent artifacts for canonical examples.
   - Committing labels, reviews, and snapshots keeps the benchmark contract inspectable on a fresh clone.

5. **Do not create a second benchmark test harness.**
   - Extend `spec-cli/tests/cli.rs` and fixture baselines that already gate status/export contract changes.

## Code Quality Review

### Guardrails

- Keep all benchmark enums typed in one place. No stringly enums repeated across CLI and core.
- Keep all path-scope logic in the benchmark core. `commands.rs` may decide which command path was loaded, but it must not re-derive benchmark semantics.
- Keep all `sha256:` digest formatting centralized in the benchmark core. No ad hoc hashing in multiple call sites.
- Reuse existing proof projection functions from `passport.rs` and molecule evidence reads from `molecule_evidence.rs`. Benchmark code may not reinterpret support truth independently.
- Keep the CLI layer boring. Parse args, load scope, call the benchmark core, print or write the result.
- Keep the diff boring. One new `spec-core` benchmark module is enough. Do not split into a mini-package tree unless the code proves it is necessary.

### Ownership boundaries

| Concern | Single owner | Why |
| --- | --- | --- |
| label parsing and validation | `spec-core::benchmark` | prevents CLI and export code from drifting |
| full versus partial scope classification | `spec-core::benchmark` | this is contract logic, not command glue |
| digest payload construction | `spec-core::benchmark` | one place to guarantee deterministic sorting |
| JSON schema version bump for export | `spec-core/src/export.rs` | keep export machine contract local to the export bundle |
| JSON schema version bump for status | `spec-cli/src/commands.rs` | keep status machine contract local to the status emitter |
| snapshot file IO | `spec-cli/src/commands.rs` | keeps `spec-core` free of filesystem writes |

### Diagram maintenance

No nearby code comments currently carry benchmark ASCII diagrams, so I3 should
add diagrams in `PLAN.md` only. Do not sprinkle speculative inline code
diagrams into unrelated files.

## Test Review

I3 must land with full benchmark-projection coverage from the start. This is not
a "wire it up, then test it later" milestone.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] benchmark registry load
    │
    ├── [GAP] valid labels file -> typed registry
    ├── [GAP] duplicate / unknown / unlabeled carrier failure
    └── [GAP] reserved positive benchmark with missing root stays legal

[+] status json projection
    │
    ├── [GAP] repo-root full BENCH-ECOM + BENCH-CROSSLIB + reserved BENCH-SERVICE
    ├── [GAP] benchmark-root full BENCH-ECOM
    ├── [GAP] namespace partial BENCH-ECOM
    ├── [GAP] single-file partial BENCH-ECOM
    ├── [GAP] active benchmark accounting invalid -> benchmark invalid/open
    └── [GAP] companion-negative benchmark visible with zero positive credit

[+] export json projection
    │
    ├── [GAP] repo-root full benchmark bundle at schema v4
    ├── [GAP] benchmark-root full benchmark bundle at schema v4
    └── [GAP] single-file partial benchmark bundle omits full-scope fields

[+] snapshot command
    │
    ├── [GAP] BENCH-ECOM full snapshot writes exactly one file
    ├── [GAP] BENCH-CROSSLIB full snapshot writes companion-negative projection
    ├── [GAP] BENCH-SERVICE reserved snapshot writes reserved state without fake generated files
    └── [GAP] partial or missing full-root snapshot invocation fails cleanly

[+] readability review
    │
    ├── [GAP] current review -> current/readable
    ├── [GAP] mismatched projection_digest -> stale
    └── [GAP] missing review on positive benchmark -> incomplete, not passing

[+] anti-laundering
    │
    ├── [GAP] deferred case visible but non-credit
    ├── [GAP] fallback-backed case visible but non-credit
    ├── [GAP] unlabeled active carrier -> accounting invalid
    └── [GAP] partial scope always forces counts_as_supported_positive=false
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Maintainer asks for repo truth
    │
    ├── [GAP] `spec status . --format json` shows full benchmark roster honestly
    └── [GAP] `spec export .` carries the same benchmark truth for downstream tooling

[+] Maintainer inspects one namespace or one file
    │
    ├── [GAP] partial scope keeps intersecting cases visible
    └── [GAP] partial scope withholds full benchmark claims and positive credit

[+] Maintainer refreshes benchmark snapshot
    │
    ├── [GAP] successful BENCH-ECOM snapshot
    ├── [GAP] successful BENCH-CROSSLIB snapshot
    └── [GAP] truthful reserved BENCH-SERVICE snapshot

[+] Reviewer checks generated readability
    │
    ├── [GAP] matching readability review stays current
    └── [GAP] stale or missing review reopens benchmark completeness
```

### Required test matrix

| Surface | Test type | Required coverage |
| --- | --- | --- |
| `spec-core/src/benchmark.rs` | unit | label validation, duplicate detection, reserved-root legality, path-scope classification, deterministic digest ordering, summary formulas, readability file selection |
| `spec-core/src/export.rs` | unit | schema v4 export bundle includes additive `benchmarks[]` and does not regress existing units, passports, graph, or plan export fields |
| `spec-cli/tests/cli.rs` | integration | repo-root full status/export, benchmark-root full status/export, namespace partial, single-file partial, invalid accounting, companion-negative, reserved `BENCH-SERVICE`, snapshot command |
| `spec-cli/tests/fixtures/status-*.json` | fixture | full, partial, reserved, and invalid benchmark status baselines |
| `spec-cli/tests/fixtures/export-*.json` | fixture | full and partial benchmark export baselines |
| snapshot artifact assertions | integration + fixture | output path, deterministic bytes, no writes outside `benchmarks/snapshots/` |

### Failure modes registry

| Failure mode | Surface | Test required | Error handling required | User-visible result |
| --- | --- | --- | --- | --- |
| unlabeled active carrier under benchmark root | registry + status/export | yes | yes | explicit invalid accounting, never silent green |
| duplicate `case_id` or duplicate `carrier_id` in one benchmark | registry load | yes | yes | explicit invalid benchmark, never merged implicitly |
| reserved `BENCH-SERVICE` omitted at repo root | status/export | yes | yes | explicit reserved entry, never silent omission |
| partial scope emits positive credit | status/export | yes | yes | hard false on every partial `counts_as_supported_positive` |
| stale readability review treated as passing | status/export/snapshot | yes | yes | benchmark incomplete, not passing |
| snapshot command mutates passports or evidence | snapshot command | yes | yes | zero writes outside `benchmarks/snapshots/` |
| benchmark digest depends on serialization order | benchmark core | yes | yes | deterministic digest across runs |
| companion-negative benchmark counted as positive | status/export/snapshot | yes | yes | visible but zero positive credit |

Critical gaps:

- Any failure mode with silent positive credit is release-blocking.
- Any failure mode that hides `BENCH-SERVICE` at broad scope is release-blocking.
- Any failure mode that makes snapshotting a proof writer is release-blocking.

### Test artifact to refresh

The implementation should refresh the benchmark-oriented eng-review test plan
artifact at:

- `/home/azureuser/.gstack/projects/atomize-hq-spec/azureuser-main-eng-review-test-plan-20260518-233552.md`

That file already names the user-facing command matrix for status, export, and
snapshot verification. The code change should make that artifact true.

## Performance Review

This is not a throughput milestone, but there are still real performance
guardrails:

- load the registry once per command invocation
- build benchmark projections from already-loaded specs, tests, passports, and evidence, not from repeated filesystem rescans
- compute full digests only for full-scope projections
- sort once inside the benchmark core before digesting or serializing
- treat readability file selection as a bounded walk rooted at declared supported cases, not a blind generated-tree scan

N+1 risks to avoid:

- status and export must not re-read the same passport or molecule evidence per benchmark and per case
- snapshot generation must reuse the same loaded proof inputs as status and export, not rebuild them ad hoc

## Implementation Plan

### Sequence overview

There are five implementation phases, but only one real architectural freeze:

```text
Phase 1: benchmark core + real registry
        │
        ├── freeze projection contract
        │
        ├── Phase 2: export schema-v4 wiring
        ├── Phase 3: status schema-v4 wiring
        └── Phase 4: snapshot + readability wiring
                 │
                 ▼
          Phase 5: canonical artifacts + docs + TODO sync
```

Do not start status, export, or snapshot wiring until the Phase-1 benchmark
projection structs and rules are frozen. That is the seam that keeps the rest of
the work boring.

### Phase 1: Registry and shared benchmark core

Objective: land the new shared benchmark module in `spec-core` and the real
authoritative registry file shape it reads.

Must land:

- typed benchmark enums and structs
- registry loader and validator for `benchmarks/labels.json`
- shared case projection record
- benchmark-level projection record
- path-scope classifier
- summary formulas
- deterministic digest builders
- readability scope file selection
- initial `benchmarks/labels.json` with `BENCH-ECOM`, `BENCH-SERVICE`, and `BENCH-CROSSLIB`

Files:

- `spec-core/src/benchmark.rs` new
- `spec-core/src/lib.rs` export module
- `benchmarks/labels.json` new

Exit gate:

- core types compile
- registry validation tests pass
- the benchmark core can produce full and partial projections without CLI printing logic

### Phase 2: Export integration

Objective: extend the export bundle to schema v4 and append additive
`benchmarks[]`.

Must land:

- `ExportBundle.schema_version = 4`
- additive benchmark bundle projection
- full versus partial benchmark entries mirror the shared core exactly
- no regressions to existing unit, passport, graph, or plan export behavior

Files:

- `spec-core/src/export.rs`
- export tests in `spec-core/src/export.rs`

Exit gate:

- export bundle tests pass
- fixture or assertion coverage proves `benchmarks[]` is additive, not disruptive

### Phase 3: Status integration

Objective: extend CLI JSON status output to schema v4 and append additive
`benchmarks[]`.

Must land:

- `STATUS_JSON_SCHEMA_VERSION = 4`
- repo-root full benchmark projection
- benchmark-root full benchmark projection
- namespace and single-file partial benchmark projection
- explicit broad-scope reserved `BENCH-SERVICE` visibility

Files:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/status-*.json`

Exit gate:

- repo-root, benchmark-root, namespace, and single-file status fixtures all match
- partial scope omits whole-benchmark fields and forces `counts_as_supported_positive=false`

### Phase 4: Snapshot and readability integration

Objective: add the benchmark snapshot command and wire readability-review
loading without changing any proof writer.

Must land:

- `spec benchmark snapshot <benchmark-id>`
- snapshot file writing under `benchmarks/snapshots/`
- readability review status projection
- reserved `BENCH-SERVICE` snapshot behavior
- failure when snapshotting cannot load full scope

Files:

- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `benchmarks/reviews/BENCH-ECOM.readability.review.json`

Exit gate:

- snapshot command writes exactly one file under the snapshot tree
- stale or missing readability review never produces passing positive status
- reserved snapshot writes reserved state without fake generated closure claims

### Phase 5: Canonical artifacts, fixtures, and docs sync

Objective: seed the committed benchmark artifacts and teach every user-facing
truth surface.

Must land:

- `benchmarks/snapshots/BENCH-ECOM.snapshot.json`
- `benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json`
- `benchmarks/snapshots/BENCH-SERVICE.snapshot.json`
- any remaining status/export fixture baselines required by Phases 2-4
- docs updated in `README.md`, `CHANGELOG.md`, and `TODOS.md`

Files:

- `benchmarks/snapshots/*.snapshot.json`
- `spec-cli/tests/fixtures/*.json`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Exit gate:

- canonical snapshots are reproducible from the landing branch
- docs explain the benchmark roster, reserved-gate truth, and writer-versus-reader wall accurately

## Worktree Parallelization Strategy

This plan has one safe parallelization seam: after the benchmark core contract is
frozen. Before that, parallel work is fake parallel because everyone is really
editing the same semantics.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Phase 1 registry + shared core | `spec-core/`, `benchmarks/` | — |
| Phase 2 export integration | `spec-core/` | Phase 1 |
| Phase 3 status integration | `spec-cli/`, `spec-cli/tests/`, fixtures | Phase 1 |
| Phase 4 snapshot + readability | `spec-cli/`, `benchmarks/`, `spec-cli/tests/` | Phase 1 |
| Phase 5 canonical artifacts + docs | `benchmarks/`, repo-root docs, fixtures | Phases 2, 3, and 4 |

### Parallel lanes

Lane A: Phase 1 -> Phase 2
Sequential, shared `spec-core/` ownership. This lane defines and freezes the
benchmark projection contract, then wires export.

Lane B: Phase 3
Depends on the Phase-1 freeze. Owns status JSON benchmark emission plus status
fixture baselines.

Lane C: Phase 4
Depends on the Phase-1 freeze. Owns snapshot routing, snapshot writing,
readability loading, and snapshot-specific assertions.

Lane D: Phase 5
Depends on B + C + the Phase-2 export contract. Owns canonical artifact refresh,
remaining fixture reconciliation, and docs/TODO sync.

### Execution order

1. Launch Lane A first and finish the shared benchmark core.
2. Freeze the benchmark core contract and merge it.
3. Launch Lane B and Lane C in parallel from that freeze.
4. Merge B and C.
5. Launch Lane D after both are green.

### Conflict flags

- Lanes B and C both touch `spec-cli/src/commands.rs`.
  - Recommended split:
    - Lane B owns status JSON benchmark emission and status-only serialization helpers.
    - Lane C owns the new benchmark subcommand branch, snapshot writing helpers, and readability loading helpers.
  - If that split proves noisy in practice, collapse B then C sequentially. Do not fight a merge war inside one giant command file.
- Lanes B and C both touch `spec-cli/tests/cli.rs`.
  - Keep assertions grouped by command surface so merges are mechanical.
  - If the test file becomes a collision magnet, accept one follow-up rebase instead of inventing a second harness.
- Lane D touches artifacts that B and C both read.
  - Do not launch Lane D until both merges are complete and fixture shapes are stable.

### Worktree recommendation

Recommended worktree split:

- Worktree 1: Lane A, then Phase 2 export follow-through
- Worktree 2: Lane B status integration after Lane A merges
- Worktree 3: Lane C snapshot + readability integration after Lane A merges
- Worktree 4: optional short-lived finalization tree for Lane D if artifact/doc sync is noisy

That gives real parallelism without pretending `commands.rs` can support three
independent semantic rewrites at once.

## NOT in scope

These were considered and are explicitly deferred:

- M69 supported-core closure
  - Why: I3 lands mechanics only. It must not reopen row-support arguments.
- bounded generics admission into V1
  - Why: still a forced later scope decision from M65/M66.
- async / IO admission into V1
  - Why: still a forced later scope decision from M65/M66.
- `BENCH-SERVICE` workload implementation
  - Why: reserved gate must stay visible, not silently faked closed.
- benchmark score dashboards, history, or reporting subsystems
  - Why: M68 needs projection truth, not a reporting empire.
- text-mode benchmark summaries as a gating surface
  - Why: M68 explicitly makes JSON the contract surface.
- benchmark metadata inside workload specs, passports, or molecule evidence
  - Why: violates the writer-versus-reader wall.
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
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- export .
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- benchmark snapshot BENCH-ECOM
cargo run -p spec-cli -- benchmark snapshot BENCH-CROSSLIB
cargo run -p spec-cli -- benchmark snapshot BENCH-SERVICE
```

Verification expectations:

- repo-root status and export show `BENCH-ECOM`, `BENCH-CROSSLIB`, and reserved `BENCH-SERVICE`
- benchmark-root status and export show full-scope `BENCH-ECOM`
- namespace and single-file status and export show partial-scope `BENCH-ECOM`
- partial scope never emits positive supported credit
- companion-negative scope never emits positive supported credit
- snapshots write only under `benchmarks/snapshots/`
- snapshotting does not mutate passports or molecule evidence

## Exit Criteria

I3 is done only when all of these are true:

1. `benchmarks/labels.json` exists, validates, and carries the locked `BENCH-ECOM` / `BENCH-SERVICE` / `BENCH-CROSSLIB` roster
2. shared benchmark projection core exists in `spec-core`
3. `spec status --format json` emits schema v4 additive `benchmarks[]`
4. `spec export` emits schema v4 additive `benchmarks[]`
5. path-scoped `full` versus `partial` rules match M68 exactly
6. `BENCH-SERVICE` reserved projection is visible at broad scope and absent from unrelated narrow scope
7. companion-negative cases stay visible but never count as positive
8. readability review state projects correctly for full positive benchmarks
9. benchmark snapshots can be written without mutating proof truth
10. CLI fixtures cover full, partial, invalid, reserved, and companion-negative states
11. repo docs explain the benchmark roster and writer-versus-reader wall truthfully

## Completion Summary

- Step 0: Scope Challenge — scope accepted as the full M68 mechanics landing on the live M61-era codebase, not M69 support expansion
- Architecture Review: one shared benchmark core, one exact full-versus-partial projection contract, one snapshot command, one repo-root benchmark artifact tree
- Code Quality Review: typed enums, centralized path-scope logic, centralized digest logic, and strict CLI/core ownership boundaries
- Test Review: full benchmark projection matrix required across `spec-core`, `spec-cli`, fixtures, and snapshot behavior
- Performance Review: single-load registry and proof inputs, full-scope digesting only, bounded readability selection
- Implementation Plan: five phases with one projection-contract freeze and explicit per-phase exit gates
- NOT in scope: written
- What already exists: written
- Failure modes: eight release-blocking benchmark truth gaps identified and covered
- Parallelization: four lanes, with real parallelism only after the shared-core freeze
- Lake Score: every recommendation chooses the complete mechanics landing over the shortcut
