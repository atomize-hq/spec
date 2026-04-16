<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260416-194312.md -->
# Next Work: M6–M10 Roadmap

Status: **M9 Delivered** (2026-04-16). M10 is the next implementation milestone, narrowed to a local-library plan contract that turns authored change intent into truthful derived impact.

Reviewed via `/autoplan` 2026-04-16 for the M10 solidification. Codex outside voices consulted;
delegated subagents were unavailable in this thread by session policy. M5 through M9 have
shipped. This plan covers the next implementation milestone plus the already-shipped historical
context that constrains it.

---

## Milestone Summary

```
M6a  Trust Gap Fixes          ✓ shipped
M6b  Health Model             ✓ shipped
     structural PR            ✓ shipped
M7   .test.spec + minimal graph ✓ shipped
M8   Full Graph Layer         ✓ shipped
M9   Cross-library Deps       ✓ shipped
M10  Planning Boundary as Data ← next to implement
```

**Explicitly deferred (do not front-load):**
- TypeScript / Python / Go targets
- Semantic eval / embeddings
- LLM semantic contract-vs-body scoring
- Planning UX
- CUE
- Reverse ingestion

---

## M6a — Trust Gap Fixes

**Theme:** Make the pipeline truthful end-to-end. Close the confirmed bug where spec test
generates code to the wrong location, compiles different code than what it generated, and
produces all-"unknown" test results in passports.

### The Root Cause (confirmed by tracing commands.rs)

Default `--output generated/spec` is relative to CWD. Cargo runs in the resolved crate root.
These are different directories. `spec test examples/ecommerce/units` from the repo root:

```
BEFORE (broken):
  generates to:  {repo_root}/generated/spec/pricing/apply_tax.rs   ← gitignored, disconnected
  cargo sees:    examples/ecommerce/src/generated/                  ← prior run's code
  module prefix: "generated::spec"                                  ← wrong (has ::spec:: segment)
  test names:    "generated::spec::pricing::apply_tax::tests::..."  ← never found in cargo output
  result:        all local tests → status: "unknown"

AFTER (fixed):
  generates to:  {crate_root}/src/generated/pricing/apply_tax.rs   ← cargo sees THIS
  cargo sees:    examples/ecommerce/src/generated/                  ← freshly generated code
  module prefix: "generated"                                        ← derived from strip(crate_root/src/)
  test names:    "generated::pricing::apply_tax::tests::..."        ← found, matched
  result:        local tests → status: "pass"
```

### Changes

**1. Anchor default output to crate root (breaking behavior, correct fix)**

Change the default `--output` convention from CWD-relative `generated/spec` to
`{crate_root}/src/generated`. The crate root is already resolved via `workspace_root_for`
or `pipeline.crate_root` in spec.toml.

- Drop the `spec` subdirectory from the default. It added `::spec::` noise and no convention used it.
- New default: `{crate_root}/src/generated` (relative to resolved crate root, not CWD).
- Update `--output` default_value in all three command arg structs (generate, build, test).

**2. Auto-derive module prefix from output path relative to crate root**

Replace the current `output_module_prefix(output)` derivation (which uses the raw output path)
with derivation from `output.strip_prefix({crate_root}/src/)`:

```
output = {crate_root}/src/generated     →  prefix = "generated"
output = {crate_root}/src/generated/spec → prefix = "generated::spec"
output = {crate_root}/src/api/gen        → prefix = "api::gen"
```

The `src` strip is now anchored to the crate root, not guessed from the first path component.

**3. Add `[pipeline] generated_module_prefix` as explicit override**

For non-standard layouts (e.g., crate imports generated code via re-export rather than
direct `mod`), allow explicit override:

```toml
[pipeline]
generated_module_prefix = "my_custom_name"
```

When present, this overrides auto-derivation. When absent (the common case), auto-derive.

**4. Preserve evidence in write_passports**

Fix the TODOS item: `spec build` and `spec generate` currently overwrite `evidence` and
`contract_hash` fields in passports, silently erasing `spec test` results.

Fix: in `write_passports`, read the existing passport before writing. If the new call
provides `evidence = None` and `contract_hash = None`, carry forward the existing values.

**Important:** this does NOT manufacture false freshness. The 6-state model (M6b) ensures
a rebuilt unit is never shown as `valid` unless:
- `contract_hash` still matches (contract hasn't changed)
- Evidence exists and all tests show `pass` or `ok`

If the contract changed after `spec build`, status = `stale` (hash mismatch). Evidence is
preserved but the stale flag is accurate. M6a ships evidence preservation; M6b ships the
status model that makes it safe.

**5. Thread OutputFormat through pipeline.rs eprintln!**

`run_cargo_build` and `run_cargo_test` emit unconditional `eprintln!` status lines. These
will contaminate machine-readable output if `--format json` is ever added to build/test.
Fix now (XS, clear deadline):

```rust
// before: eprintln!("spec: running cargo build in {}", crate_root.display());
// after:
if matches!(format, OutputFormat::Text) {
    eprintln!("spec: running cargo build in {}", crate_root.display());
}
```

Thread `OutputFormat` parameter through `run_cargo_build` and `run_cargo_test`. One caller
each in commands.rs. No behavior change in Text mode.

**6. Nextest limitation documented**

Add to README under `## Pipeline`:
> `spec test` parses standard `cargo test` output format only. `cargo nextest` uses a
> different output format and is not supported. Running `spec test` in a project configured
> for nextest will produce `status: "unknown"` for all local tests. Use standard `cargo test`.

Close the TODOS item that has been outstanding since M4.

**7. Regenerate example ecommerce passports**

After all fixes land, run `spec test examples/ecommerce/units` and commit the resulting
passports. All local tests should show `status: "pass"` (not `"unknown"`). The committed
passports become a regression artifact proving the trust gap is closed.

### Dependency Order

```
1. Anchor default output + auto-derive prefix  (commands.rs + pipeline.rs/config.rs)
2. Evidence preservation in write_passports    (commands.rs)
3. eprintln! compat in pipeline.rs             (pipeline.rs)
4. Nextest doc                                 (README.md)
5. Regenerate + commit example passports       (examples/)
```

### Success Criteria

- `spec test examples/ecommerce/units` produces passports with all test results `pass`,
  not `unknown`. This is the regression test for the entire trust gap fix.
- A new integration test: `spec test <dir>` with `crate_root` configured correctly →
  `build_test_evidence` maps test names using the auto-derived prefix → results match.
- Existing tests all pass (`cargo test --all`).
- Fixture files updated if output path changes affect JSON snapshots.

### What NOT in M6a Scope

- Status state machine changes (M6b)
- schema_version bump (M6b)
- commands.rs split (structural PR, between M6a and M6b)
- ValidatedExpr newtype (structural PR)

---

## Structural PR (between M6a and M6b)

**Zero behavior change. All tests pass before and after.**

Split `spec-cli/src/commands.rs` (2433 lines) into a module directory:

```
spec-cli/src/commands/
  mod.rs          ← CLI dispatch (Cli::run match arm)
  validate.rs     ← validate_command
  generate.rs     ← generate_command + generate_specs + finalize_passports
  build.rs        ← build_command
  test.rs         ← test_command + build_test_evidence + passport_write_plan
  status.rs       ← status_command
  export.rs       ← export_command
  helpers.rs      ← output_module_prefix, expected_cargo_test_name,
                     cargo_test_filter_for, resolve_git_provenance,
                     rfc3339_now, timeout_suffix, etc.
```

Bundle `D5a ValidatedExpr` newtype into this PR:
- Replace `expect: String` in `ResolvedSpec` with `ValidatedExpr(syn::Expr)` newtype.
- `ValidatedExpr` wraps a parsed `syn::Expr` — eliminates double-parse in `generator.rs`.
- `generate_code` receives `ValidatedExpr`, calls `.into_token_stream()` directly.
- Removes the last gap where a direct `ResolvedSpec` constructor could bypass validation.

**Success criterion:** `cargo test --all` passes before and after. No new behavior.

---

## M6b — Health Model

**Theme:** Make `spec status` a real evidence-health surface, not just validation + staleness.

### 6-State Status Machine

```
  untested     no passport / no evidence field
      │
  incomplete   evidence exists but ≥1 test result is "unknown"
      │
  failing      build_status = "fail" OR "timeout" OR any test_result.status = "fail"
      │   ↘
  stale        contract_hash mismatch (contract changed since last spec test)
  valid        all: build_status pass, all tests pass/ok, hash matches, no unknowns
  invalid      validation errors (schema/semantic), regardless of evidence
```

**Precedence (highest to lowest):** invalid > failing > stale > incomplete > untested > valid

`valid` is only reached when ALL conditions are met: validation clean, build passed,
all test results observed (none "unknown"), all tests pass, contract hash matches.

### JSON Contract Change

This is a breaking change. Bump `schema_version` from 1 to 2.

Old (schema_version 1):
```json
{"status": "stale", "stale": true}
```

New (schema_version 2):
```json
{
  "schema_version": 2,
  "status": "incomplete",
  "reason": "1 local test not observed in cargo output"
}
```

**Migration plan:**
- Old passports (without `schema_version` or with `schema_version: 1`) deserialize with
  backward-compatible serde defaults. The status computation upgrades them on read.
- Mixed-version repos: each unit computes its own status from its own passport.
  No cross-unit version dependency.
- CLI consumers: the JSON `status` string values change (new values: `incomplete`, `untested`,
  `failing`). Bump `schema_version` in `spec status --format json` output so consumers can
  detect the change. Document in AGENTS.md and CHANGELOG.
- Old consumers reading `schema_version: 1` responses: existing `valid/invalid/stale` still
  valid state names. New state names are additive. Old code will see `schema_version: 2` and
  can guard on it.

### Human-readable `spec status` output

```
✓ money/round             valid       evidence:2026-04-12T02:56:17Z
✓ pricing/apply_tax       valid       evidence:2026-04-12T02:56:17Z
~ pricing/apply_discount  stale       contract changed since last test
? shipping/calculate      incomplete  1 test not observed
✗ auth/verify             failing     build failed
— new_unit/foo            untested    no evidence
✗ inventory/check         invalid     2 validation errors
```

### Success Criteria

- Each new state has at least one test that reaches it via a real code path.
- `spec status --format json` emits `schema_version: 2`.
- Fixture files updated for all new status values.
- Old passports still parse correctly (serde backward-compat test).
- AGENTS.md updated: document new state names and schema_version: 2 contract.

---

## M7 — .test.spec + Minimal Graph

**Theme:** First-class molecule tests with declared covers edges. Add just enough graph
structure to represent the unit/test/edge model without over-engineering it.

### .test.spec File Format

```yaml
# pricing.test.spec
id: pricing/checkout_flow
intent: "Verify discount + tax chain produces correct totals end-to-end."
covers:
  - pricing/apply_discount
  - pricing/apply_tax
  - money/round
body:
  rust: |
    let discounted = apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2));
    let total = apply_tax(discounted, Decimal::new(725, 4));
    assert_eq!(total, Decimal::new(10725, 2));
```

- `id`: same namespace as unit ids, conventionally `{namespace}/test_name`
- `intent`: why this molecule test exists
- `covers`: declared unit ids. spec validates all ids exist in the loaded spec set.
  These are programmer claims, not observed coverage — same epistemic status as `deps`.
- `body.rust`: test function body. spec generates a `#[test]` function. This IS code
  injection — spec validates it compiles and the declared units are importable; it does not
  validate semantic correctness beyond that.

### Validation Rules

- All ids in `covers` must exist in the loaded spec set. Error: `SPEC_MOLECULE_COVERS_NOT_FOUND`.
- Duplicate `.test.spec` ids are rejected. Error: `SPEC_DUPLICATE_MOLECULE_ID`.
- Body validation: same `is_safe_expr` rules as local test `expect` (block expression,
  no unsafe).
- A `.test.spec` file that declares no `covers` is a warning, not an error.

### Generation

`spec generate` and `spec build` process `.test.spec` files alongside `.unit.spec` files.
Each molecule test generates a `#[test]` function in a dedicated `molecule_tests.rs` file
(or per-namespace `{namespace}/molecule_tests.rs`). The generated function imports all
covered units and runs the body.

### Minimal Graph in spec-core

Rather than raw JSON arrays or a full graph abstraction, introduce a minimal `SpecGraph`
struct in `spec-core` that represents the current loaded world:

```rust
pub struct SpecGraph {
    pub units: Vec<UnitNode>,
    pub molecule_tests: Vec<MoleculeTestNode>,
    pub edges: Vec<SpecEdge>,
}

pub struct UnitNode { pub id: String, pub deps: Vec<String> }
pub struct MoleculeTestNode { pub id: String, pub covers: Vec<String> }

pub enum SpecEdge {
    Dep { from: String, to: String },
    Covers { test: String, unit: String },
}
```

This is not a full graph database. It's a typed representation of what the loader found.
It answers: what units? what molecule tests? what edges? M8 extends this.

### Export

`spec export` includes molecule tests and covers edges:

```json
{
  "schema_version": 2,
  "units": [...],
  "molecule_tests": [
    {
      "id": "pricing/checkout_flow",
      "intent": "...",
      "covers": ["pricing/apply_discount", "pricing/apply_tax", "money/round"]
    }
  ],
  "graph": {
    "edges": [
      {"kind": "dep",    "from": "pricing/apply_tax", "to": "money/round"},
      {"kind": "covers", "test": "pricing/checkout_flow", "unit": "pricing/apply_discount"}
    ]
  }
}
```

### Status Propagation Rule

Molecule test failure does NOT propagate to unit status. A failing molecule test changes
the molecule test's own status (in a future `spec status` extension for molecule tests).
Unit status is determined solely by:
- unit validation
- `spec test` evidence for that unit's local tests
- contract_hash staleness

This avoids the "five units fail because one molecule test failed" ambiguity Codex raised.
Document this boundary explicitly in AGENTS.md.

### Atom/Molecule Boundary

- **Atom tests**: inline `local_tests` in `.unit.spec`. Test one unit's behavior.
  Generated inside the unit's `#[cfg(test)]` module.
- **Molecule tests**: `.test.spec` files. Test interactions between units.
  Generated as standalone `#[test]` functions that call multiple units.
- **The boundary**: if a test needs to import more than one unit, it belongs in `.test.spec`.
  If it tests only the current unit's behavior, it belongs in `local_tests`.

### Success Criteria

- `spec validate`, `spec build`, `spec test`, `spec export` all handle `.test.spec` files.
- `covers` validation rejects unknown unit ids with a stable `SPEC_*` error code.
- Generated molecule test compiles and `cargo test` runs it.
- Export includes `molecule_tests` array and `covers` edges in `graph.edges`.
- At least two molecule tests added to `examples/ecommerce/`.
- Integration tests in `cli.rs` cover: valid molecule test, unknown covers id, generation,
  export shape.

---

## M8 — Full Graph Layer in spec-core

**Theme:** Promote the minimal M7 graph into a first-class **declared relationship contract**
that answers impact questions truthfully. M8 is not an observation system and not a status
engine. It is the clean declared-graph foundation that M9 and M10 can build on.

### Core Questions the Graph Must Answer

```
1. What are all the units?                    → graph.units()
2. What are all the molecule tests?           → graph.molecule_tests()
3. What edges exist (dep + covers)?           → graph.edges()
4. What is the reverse dependency set?        → graph.reverse_deps(unit_id)
5. What molecule tests cover a given unit?    → graph.tests_covering(unit_id)
6. What is the local declared blast radius?   → graph.impact(unit_id)
7. What is the authoritative relationship source? → deps + covers only
8. What export shape should reuse the graph?  → export projects from SpecGraph
```

### graph.build() Contract

`SpecGraph::build(loaded_units, molecule_tests)` constructs the graph from:
- Loaded `.unit.spec` files (units, deps, local_tests)
- Loaded `.test.spec` files (molecule tests, covers edges)

Graph source of truth: **the authored spec files**. In M8:
- `.unit.spec` `deps` are the only authoritative dependency edges
- `.test.spec` `covers` are the only authoritative molecule-test coverage edges
- passports are **not** graph input
- generated Rust is derived and ephemeral, never graph input

`links.molecule_tests` on unit specs is legacy metadata, not relationship truth. **Decision
(locked in M8 eng review 2026-04-15):** `build()` explicitly ignores it with a code comment;
a TODOS entry tracks the follow-up validator warning + field removal. It must not silently
compete with `.test.spec` `covers`.

### Invalidation Rules

The graph is rebuilt on each command invocation from the current spec files. No persistent
graph state between runs. This avoids staleness. The export bundle captures a snapshot.

### Impact Analysis (foundation for M10)

`graph.impact(unit_id)` returns the **local declared retest set** as a structured type:

```rust
pub struct ImpactSet {
    pub units: Vec<String>,          // unit IDs in the retest closure (seed + all reverse deps)
    pub molecule_tests: Vec<String>, // molecule tests covering any unit in that set
}

fn impact(&self, unit_id: &str) -> Option<ImpactSet>
// None  → seed unit not in graph
// Some  → ImpactSet (units always includes the seed; both vecs are sorted)
```

Unit IDs and molecule test IDs share the same string format, so the structured return type
is required to let callers (M10 plan artifact, AI agents) distinguish "units to re-implement"
from "molecule tests to run."

`impact()` returns **unit IDs**, not individual local test cases. The contract is: callers
pass unit IDs to `spec test`, which handles local tests per unit. Local test cases are
implicitly included through the unit ID.

`impact()` is implemented via BFS over `rev_dep_index` with a `HashSet<String>` for
deduplication (handles diamond dependencies). M8: local-library declared impact only.
Advisory planning data, not runtime status.

### API Contract (locked in M8 eng review 2026-04-15)

```rust
// SpecGraph fields are private. Accessor methods are the public API.
// build() assumes validated input (all dep IDs and covers IDs exist in the spec set).

fn units(&self) -> &[UnitNode]
fn molecule_tests(&self) -> &[MoleculeTestNode]
fn edges(&self) -> &[SpecEdge]           // sorted
fn reverse_deps(&self, unit_id: &str) -> Option<Vec<String>>
// None → unit not in graph; Some([]) → exists, no dependents; Some([...]) → sorted dependents
fn tests_covering(&self, unit_id: &str) -> Option<Vec<String>>
// None → unit not in graph; Some([]) → exists, no covering tests; Some([...]) → sorted
fn impact(&self, unit_id: &str) -> Option<ImpactSet>
// None → seed not in graph
```

Internal fields (`rev_dep_index`, `test_coverage_index`) are `HashMap<String, Vec<String>>`,
private to the struct. Export calls `graph.edges()` (not the field directly).

### Implementation Slices (locked for M8)

```text
LoadedSpec + LoadedMoleculeTest
        │
        ▼
SpecGraph::build()
  ├── sorted UnitNode / MoleculeTestNode vectors
  ├── sorted SpecEdge vector
  ├── rev_dep_index: unit_id -> direct dependents
  └── test_coverage_index: unit_id -> covering molecule tests
        │
        ├── accessors: units() / molecule_tests() / edges()
        ├── queries: reverse_deps() / tests_covering() / impact()
        └── export projection through graph.edges()
```

**Slice A. Graph core in `spec-core/src/graph.rs`**

- Keep `SpecGraph::build()` as the single constructor. It accepts validated input and stays infallible in M8.
- Make `units`, `molecule_tests`, and `edges` private. Add private `rev_dep_index` and `test_coverage_index`.
- Build all public vectors in deterministic order during construction:
  - `units` sorted by `id`
  - `molecule_tests` sorted by `id`
  - `edges` sorted lexicographically by enum payload
  - each index vec sorted and deduplicated once during `build()`
- `reverse_deps(unit_id)` returns **direct** dependents only. Transitive closure belongs to `impact()`, not this accessor.
- `tests_covering(unit_id)` returns molecule tests that directly declare the unit in `covers`.
- `impact(unit_id)` performs BFS over `rev_dep_index`, collecting the seed plus all transitive reverse deps, then unions molecule tests covering any unit in that closure.
- `build()` carries an explicit doc comment: "assumes validated input" and "does not read `links.molecule_tests`."

**Slice B. Public surface and file boundaries**

- `spec-core/src/lib.rs`: re-export `SpecGraph`, `SpecEdge`, `UnitNode`, `MoleculeTestNode`, and `ImpactSet`.
- `spec-core/src/export.rs`: remain a projection layer. It may call `graph.edges()`, but it must not read graph internals or serialize index state.
- `spec-core/src/types.rs`: no schema change in M8. `Links.molecule_tests` stays as legacy parsed metadata only; field removal is a later cleanup milestone.

**Slice C. Exact test work required before shipping M8**

- `spec-core/src/graph.rs` unit tests:
  - `reverse_deps_returns_direct_dependents_sorted`
  - `reverse_deps_unknown_unit_returns_none`
  - `tests_covering_returns_multiple_tests_sorted`
  - `tests_covering_unknown_unit_returns_none`
  - `impact_includes_seed_reverse_dep_closure_and_covering_tests`
  - `impact_includes_downstream_covering_tests_not_just_seed_tests`
  - `impact_deduplicates_diamond_reverse_deps`
  - `build_ignores_links_molecule_tests_legacy_metadata`
- `spec-core/src/export.rs` regression test:
  - export still projects sorted `graph.edges()` correctly after graph internals become private and indexed.
- End-of-milestone verification:
  - `cargo test -p spec-core`
  - `cargo test --all`

### Explicit Non-Goals for M8

- No `Declared | Observed` edge taxonomy
- No edge-level runtime evidence
- No `spec status` downstream stale propagation
- No cross-library node metadata (`library_id`, `scope`) before M9 defines typed dep identity
- No export schema growth beyond what current consumers need

### Success Criteria

- `SpecGraph` lives in `spec-core`, exposed from `lib.rs`.
- `SpecGraph::build()` consumes only loaded unit specs and loaded molecule tests.
- `spec export` uses `SpecGraph::build()` — already satisfied by M7 (`export.rs:92`).
- All M7 molecule test / covers edge behavior in `SpecGraph` confirmed as declared graph truth.
- `graph.reverse_deps()`, `graph.tests_covering()`, and `graph.impact()` ship for local-library declared relationships per the API contract above.
- `spec status` remains passport-driven in M8. No downstream stale propagation is added.
- `SpecGraph` fields are private; public API is accessor methods only.
- `ImpactSet` struct is public from `spec-core`.
- Tests cover: build contract, `reverse_deps()`, `tests_covering()`, `impact()` (including the downstream-covering-test case and diamond dedup case), relationship source-of-truth behavior, export projection regression, and unknown-unit-id contracts.
- `build()` doc comment explicitly states "assumes validated input" and "links.molecule_tests is explicitly not read."

### Delivery Status

**Delivered 2026-04-15 in v0.6.0.**

What shipped:
- `SpecGraph` now exposes the declared graph API from `spec-core`, including `reverse_deps()`, `tests_covering()`, and `impact()`.
- `ImpactSet` shipped as the structured return type for local declared blast-radius queries.
- Graph internals are private; export projects through the public graph surface.
- `links.molecule_tests` is explicitly ignored in `build()` as legacy metadata, with follow-up cleanup deferred.
- Graph and export regression coverage landed, including downstream-covering-test and diamond-dedup cases.

Post-ship verification:
- `cargo test --all` passed on the shipped branch.
- `spec export examples/ecommerce/units` emits `schema_version: 2` with 4 units, 2 molecule tests, and 11 graph edges.
- Example ecommerce passports were refreshed after ship so the checked-in regression artifacts now show `pass` rather than `incomplete`.

### M8 /autoplan Review (2026-04-14)

**Review scope:** `PLAN.md` M8 section, grounded against [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1), [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1), [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1), [spec-core/src/passport.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/passport.rs:1), and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:514).

**UI scope:** No. This is backend and data-model planning only, so Phase 2 design review is skipped.

#### Step 0A. Premise Challenge

1. The real user problem is not "we need a full graph layer." The real user problem is "we need trustworthy impact analysis for cross-library deps and plan artifacts." Right now M8 names the abstraction before it proves the user win.
2. `edge.kind (Declared | Observed)` is not supported by the current evidence model. Passports contain per-unit build and local-test evidence plus `contract_hash`; they do not contain edge-level runtime facts. Shipping "observed" edges in M8 would encode fake precision.
3. `spec status` currently computes truth from validation errors, passport evidence, and contract hash. Using `graph.impact()` to mark downstream stale units is a product-semantics change, not a plumbing cleanup. That deserves its own explicit contract.
4. M9's hard problem is typed cross-library dep identity and cycle truth, not `library_id` on nodes. Front-loading graph metadata before the dep identity model is fixed risks building the wrong foundation.
5. The schema still carries two relationship stories: `.test.spec` `covers` and `links.molecule_tests` on unit specs. M8 should not harden the graph until one relationship source of truth is chosen.

#### Step 0B. What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Declared unit and molecule-test edges | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1) | Reuse the current minimal graph as the seed, do not rebuild from scratch. |
| Export graph serialization | [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83) | Existing consumer proves M8 already has one downstream caller. Keep export as a consumer, not the reason for extra schema growth. |
| Unit health and staleness truth | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:514) | Reuse current passport-hash status model. Do not silently merge inferred blast radius into this surface in M8. |
| Molecule relationship validation | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1835) and [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:543) | Reuse current `covers` validation as the source of declared molecule-test edges. |
| Relationship schema debt | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:61) | `links.molecule_tests` still exists. M8 must either deprecate or explicitly ignore it. |
| Cross-library dep identity | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:10) | No typed dep identity exists yet. Current dep strings are local-only. This is an M9 blocker, not something graph metadata can wish away. |

#### Step 0C. Dream State Mapping

```text
CURRENT STATE                         THIS PLAN AS WRITTEN                     12-MONTH IDEAL
Minimal declared-edge graph           Broad "full graph layer" milestone       Trusted impact engine with explicit
used mostly by export.                mixing queries, future metadata,         declared relationships, typed cross-
Status truth lives in passports.      and implied status semantics.            library identities, and evidence-backed
                                      Some planned facts do not exist yet.     observations where instrumentation exists.
```

**Dream state delta:** M8 should move the repo from "graph as export helper" to "graph as trusted declared-relationship query layer." It should not jump all the way to observed edges or downstream status semantics before the evidence model and dep identity model exist.

#### Step 0C-bis. Implementation Alternatives

```text
APPROACH A: Query-Only Layer
  Summary: Keep SpecGraph minimal and private, add reverse lookup helpers over current local IDs.
  Effort:  S
  Risk:    Low
  Pros:    Small blast radius; unlocks impact queries quickly; minimal schema churn.
  Cons:    Leaves M9 to solve typed cross-library identity later; weak long-term contract; risks another rewrite.
  Reuses:  Existing graph.rs, export.rs, validator coverage.

APPROACH B: Contract-First Declared Graph (RECOMMENDED)
  Summary: Promote SpecGraph into a first-class declared-relationship model with explicit node/edge types and query APIs, while deferring observed edges and downstream stale propagation.
  Effort:  M
  Risk:    Medium
  Pros:    Gives M9/M10 a real foundation; avoids fake "observed" precision; keeps status semantics trustworthy.
  Cons:    Requires tighter contract decisions now; forces explicit deferrals in the roadmap.
  Reuses:  Existing SpecGraph, export consumer, current passport-based status model.

APPROACH C: Full Platform Graph Now
  Summary: Ship declared + observed edge taxonomy, cross-library-ready metadata, and status integration in one milestone.
  Effort:  L
  Risk:    High
  Pros:    Ambitious platform story; fewer future public API pivots if guessed correctly.
  Cons:    Encodes facts the repo cannot currently observe; couples M8 to unresolved M9 semantics; highest migration debt.
  Reuses:  Existing graph/export code only as scaffolding.
```

**Recommendation:** Choose **Approach B** because it is the complete version of what M8 can honestly promise today: trusted declared graph answers, not pretend observations.

#### Step 0D. SELECTIVE_EXPANSION Analysis

**Complexity check:** As written, M8 touches at least `spec-core/src/graph.rs`, `spec-core/src/export.rs`, `spec-core/src/lib.rs`, `spec-core/src/types.rs`, `spec-cli/src/commands.rs`, and integration/unit tests. That is already a medium-sized milestone. It should not also absorb status-semantics changes and future evidence concepts.

**Minimum set that achieves the goal:**
- Define the declared graph contract: node kinds, edge kinds, query methods, and rebuild rules.
- Migrate export and M7 molecule-test handling to the declared graph.
- Add `reverse_deps`, `tests_covering`, and `impact` for local-library declared relationships.
- Test the graph queries directly in `spec-core` plus one integration path through export.

**Expansion scan:**
- `library_id` and cross-library edge scope on public node/edge types.
- "Observed" edges sourced from runtime evidence.
- Downstream stale propagation in `spec status`.
- Additional graph queries such as SCC / topological ordering.
- Public export schema widening beyond what current consumers need.

**Cherry-pick decisions (auto-decided per /autoplan principles):**
- **Accepted into M8:** first-class declared graph API, local-library `impact()`, `reverse_deps()`, `tests_covering()`, export migration, and explicit rebuild/no-cache contract.
- **Deferred to M9:** typed cross-library dep identity, `library_id`, cross-library `scope`, and any graph semantics that depend on external libraries.
- **Deferred to later milestone:** observed edges, molecule-test runtime evidence, downstream stale propagation, and any export-schema expansion not needed by a named consumer.

#### Step 0E. Temporal Interrogation

- **HOUR 1 foundations:** decide whether M8's graph is declared-only or declared+observed. This cannot stay fuzzy.
- **HOUR 2-3 core logic:** decide the canonical relationship source. If `.test.spec` `covers` is truth, `links.molecule_tests` must be deprecated or explicitly non-authoritative.
- **HOUR 4-5 integration:** decide whether export consumes public graph structs or a projection. This affects schema churn and consumer stability.
- **HOUR 6+ polish/tests:** decide whether `impact()` is local-library only in M8. If that answer is "yes," the plan must say so plainly or implementers will overbuild for M9.

#### Step 0F. Mode Selection Confirmation

**Selected mode:** `SELECTIVE_EXPANSION`

**Chosen approach under this mode:** `APPROACH B: Contract-First Declared Graph`

**Premise gate outcome:** user selected the contract-first path and explicitly requested that
all cascades into M9 and M10 be reflected in `PLAN.md`.

This keeps the milestone complete, explicit, and honest:
- build the declared graph contract now
- do not ship fake observed edges
- do not mutate `spec status` semantics in the same milestone
- do not hard-block M9 on metadata that only M9 can define correctly

#### CEO Outside Voice

**CLAUDE SUBAGENT (CEO — strategic independence):** unavailable in this run. Session policy for this thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

**CODEX SAYS (CEO — strategy challenge):**

- M8 is currently framed as a platform milestone, but the real unlock is trustworthy impact analysis for M9 and M10.
- `Declared | Observed` edges are premature because passport evidence has no edge-level runtime facts.
- Hard-blocking M9 and M10 on a "full graph layer" is likely over-scoping the abstraction before dep identity is solved.
- Reusing `graph.impact()` to mark downstream stale units would blend inferred blast radius with observed unit health and make `spec status` less trustworthy.
- `links.molecule_tests` remains unresolved schema debt and should not silently coexist with `.test.spec` `covers` as equal graph truth sources.

#### CEO Dual Voices — Consensus Table

```text
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   N/A     No     single-model concern
  2. Right problem to solve?           N/A     No     single-model concern
  3. Scope calibration correct?        N/A     No     single-model concern
  4. Alternatives sufficiently explored?N/A    No     single-model concern
  5. Competitive/market risks covered? N/A     Partial single-model concern
  6. 6-month trajectory sound?         N/A     No     single-model concern
═══════════════════════════════════════════════════════════════
```

**Single-model verdict:** strong strategic signal to reframe M8 from "full graph layer" to "declared graph contract + impact queries."

#### NOT in Scope (CEO pass)

- Edge-level observed facts in M8, because the current evidence model cannot produce them truthfully.
- Downstream stale propagation in `spec status`, because that changes product semantics and should not piggyback on graph plumbing.
- Cross-library node metadata in M8, because typed dep identity is an M9 concern and is still undefined.
- Export schema growth beyond what current consumers require, because public schema churn without a named consumer is avoidable debt.

#### Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Review M8 as a contract-first graph milestone, not a generic platform rewrite | Taste | P1 + P5 | This preserves the real foundation while avoiding abstractions that the evidence model cannot support yet. | Treat M8 as a full graph platform milestone |
| 2 | CEO | Skip Phase 2 design review | Mechanical | P3 | M8 has no meaningful UI scope; design review would be noise. | Running UI/design review on backend graph planning |
| 3 | CEO | Recommend Approach B over A/C | Mechanical | P1 + P5 | It is the complete version that stays explicit and honest about current repo truth. | Query-only shortcut, full-platform overreach |
| 4 | CEO | Defer observed edges out of M8 | Mechanical | P5 | The repo has no edge-level observation artifact today. | Encoding fake "observed" precision from passports |
| 5 | CEO | Defer downstream stale propagation out of M8 | Mechanical | P3 + P5 | `spec status` currently reports observed unit truth; mixing inferred blast radius would muddy the contract. | Folding status semantics into the graph milestone |
| 6 | CEO | Cascade M8 scope changes into M9 and M10 prerequisites/success criteria | Mechanical | P1 | The roadmap must stay internally consistent or implementation will drift immediately. | Leaving later milestones on the old assumptions |
| 7 | CEO | Keep M10 local-library only even after M9 shipped | Taste | P3 + P5 | The repo has truthful local graph queries today, but not truthful cross-library query semantics. The complete near-term move is to prove the planning contract on one library before widening the blast radius. | Expanding M10 straight into cross-library planning |
| 8 | CEO | Reframe M10 around change intent + derived impact, not a passive YAML note | Mechanical | P1 | The user job is understanding what changed, why, and what else to retest. A file format alone does not solve that job. | Keeping M10 as a thin parseable note format |
| 9 | CEO | Replace authored `impacted` with derived `computed_impact` | Mechanical | P5 | Source and derived data must not share one field or the plan will rot immediately. | Authoring and exporting the same flat `impacted` list |
| 10 | Eng | Make acceptance criteria structured and machine-readable | Mechanical | P1 + P5 | Linking acceptance to unit ids and molecule tests gives AI and humans a real contract instead of YAML-shaped prose. | Free-text-only acceptance strings |
| 11 | Eng | Resolve plan graph scope from the enclosing library root, never from the plan file path | Mechanical | P5 | Existing file-path loaders are intentionally narrow. Reusing them for plans would under-report impact and drop sibling molecule tests. | Reusing single-file spec loading for plan impact |
| 12 | Eng | Define action-sensitive impact semantics: `modify/remove` = current graph, `add` = unknown | Mechanical | P5 | The graph can only answer questions about nodes that already exist. Fabricating impact for `add` would be a lie. | Pretending `graph.impact()` works for all actions |
| 13 | Eng | Use a dedicated `spec plan export` bundle instead of mutating `spec export` in M10 | Taste | P3 + P5 | The existing export bundle is already consumer-facing. A dedicated plan export is the smaller, cleaner first cut while the plan surface is still stabilizing. | Bumping the main export bundle schema for a single-plan feature |

---

## M9 — Cross-library Deps (Contract-First, Repo-Scoped)

**Theme:** Let one spec library reuse units from a sibling spec library in the same git repo
without copy-pasting code, while keeping `spec validate`, generated Rust imports, and export
truthful. M9 is not a package manager, not cross-library planning, and not a graph-query
expansion milestone.

**Milestone verdict:** M9 is the first truthful shared-library slice. It solves direct sibling
library reuse with one identity story across validation, generation, and export. It does **not**
expand planning semantics, graph-query scope, or trust boundaries beyond the repo.

**User job:**
- A root library can author `shared::money/round` and get real validation/build behavior,
  not stringly best-effort.
- A team can split shared units into a sibling spec library without losing trust in
  generated Rust or `spec validate`.
- M10 plan artifacts remain local-library only. Cross-library planning stays deferred.

**Prerequisite:** M8 declared graph contract complete. Do not implement M9 until local
`reverse_deps()` / `tests_covering()` / `impact()` semantics are locked and the graph has a
single source of relationship truth.

### Locked Boundary

- Only direct cross-library deps authored by the root library being validated/generated.
- `[libraries]` targets must resolve inside the same git repo as the invoking library.
- Only the root library's `spec.toml` is authoritative. Imported libraries do **not**
  recursively load their own `[libraries]` entries in M9.
- Cross-library `.test.spec` `covers` are out of scope and rejected loudly.
- `SpecGraph::reverse_deps()`, `tests_covering()`, and `impact()` stay local-library only in M9.
- M10 remains local-library only even after M9 lands.

### What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Author-facing cross-library syntax decision | [DECISIONS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/DECISIONS.md:56) | Reuse the locked `shared::money/round` syntax. Do not reopen author-facing syntax in M9. |
| Local dep identity and duplicate-id validation | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:13) and [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303) | Current dep identity is plain local strings. M9 must add typed identity before it loads multiple libraries. |
| Local graph/export contract | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:37) and [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83) | Reuse the public graph/export boundary. Export stays a projection, not a second source of truth. |
| Generated import contract | [spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475) and [README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/README.md:226) | Local deps already rely on `use crate::...`. Cross-library imports must extend that model without inventing a second identity. |
| Root config loading | [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1) | Current config lookup is single-root nearest-ancestor. Keep one authoritative root config in M9. |
| Cargo/crate-root truth | [spec-core/src/pipeline.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/pipeline.rs:37) | Reuse the existing "build what Cargo actually sees" principle. M9 must validate the Rust dependency alias before codegen lies. |

### Authoritative Contract

#### `spec.toml`

```toml
[libraries]
shared = "../shared-spec"
payments = "../../payments/spec"
```

The namespace alias is authoritative for:
- authored dep syntax (`shared::money/round`)
- generated Rust import paths (`use shared::money::round::round;`)
- root-scoped graph/export references in M9

M9 does **not** read a target crate's Cargo `[package] name` to invent a second identity.
If the consuming crate wants to import `shared::...`, its `Cargo.toml` must expose a dependency
named `shared`.

```toml
[dependencies]
shared = { path = "../shared-crate" }
payments = { path = "../../payments/crate" }
```

#### Authored dep syntax

```yaml
deps:
  - money/round              # local dep (same library)
  - shared::money/round      # cross-library dep
```

#### Typed identity

```rust
enum DepRef {
    Local { unit_id: String },
    External { library: String, unit_id: String },
}

struct QualifiedUnitRef {
    library: Option<String>, // None = root library, Some("shared") = external alias
    id: String,
}
```

- Local root-library units keep their existing slash-delimited unit ids.
- External refs use the root config's namespace alias plus the unit id.
- Canonicalized filesystem paths are used for trust checks and duplicate-root rejection,
  not as authored ids or generated Rust module names.
- The namespace alias is the only public cross-library identity in M9. Cargo package names,
  canonical paths, and inferred crate names remain implementation details.

### Architecture Review

```text
root spec library
    │
    ├── root spec.toml [libraries]
    │       │
    │       └── repo-scoped library resolver
    │               │
    │               ├── typed DepRef / QualifiedUnitRef
    │               ├── validator + cycle checks
    │               ├── generator import path selection
    │               └── export schema v3 projection
    │
    └── local graph queries remain local-only in M9
```

**Architecture constraints:**
- Root `spec.toml` is the only authoritative `[libraries]` config in M9.
- The same alias must satisfy authored syntax, generated `use <alias>::...` imports, and the
  consuming crate's Cargo dependency name.
- `SpecGraph` may carry typed dep refs internally, but public query semantics remain local-only.
- Recursive library discovery stays out of scope. One authoritative root config keeps validation,
  loading, and cycle detection deterministic.

### Validation

- Unknown library namespace → `SPEC_UNKNOWN_LIBRARY_NAMESPACE`
- Target library path missing on disk → `SPEC_LIBRARY_PATH_NOT_FOUND`
- Target library path escapes the repo root → `SPEC_LIBRARY_OUT_OF_ROOT`
- Alias points back to the root library → `SPEC_LIBRARY_ALIAS_SELF`
- Two aliases resolve to the same canonical library root → `SPEC_DUPLICATE_LIBRARY_ROOT`
- Cross-library dep id not found in target library → `SPEC_CROSS_LIBRARY_DEP_NOT_FOUND`
- Cross-library cycle across the direct library graph → `SPEC_CROSS_LIBRARY_CYCLE`
- Root crate lacks a Cargo dependency keyed by the same alias → `SPEC_LIBRARY_CRATE_ALIAS_MISSING`
- Legacy local deps (`money/round`) continue to work unchanged.
- Duplicate unit ids across different libraries are allowed. Duplicate ids within the same
  resolved library remain errors.

### Generator Contract

- Local deps keep the current `use crate::...` contract.
- Cross-library deps emit `use <alias>::...` where `<alias>` is the namespace key from
  the root library's `[libraries]` config.
- Cross-library callable-name collisions are rejected with a stable error in M9. Automatic
  import alias rewriting is deferred until the authored `body.rust` contract has a story for
  those alias names.

### Graph + Export Contract

M9 is where dep identity becomes typed. It is **not** where cross-library graph queries become
public API.

- Validator, generator, graph, and export all consume the same typed dep IR.
- `SpecGraph` may store typed cross-library dep refs internally, but public query semantics remain
  local-library only in M9.
- `spec export` bumps `schema_version` to 3 and encodes dep endpoints as structured refs:

```json
{
  "kind": "dep",
  "from": {"library": null, "id": "pricing/apply_tax"},
  "to": {"library": "shared", "id": "money/round"}
}
```

Export remains a projection over the public contract. It must not serialize raw graph internals.

### Implementation Plan

**Slice 1. Typed dep identity**
- Primary files: [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1), [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303)
- Add typed dep IR to `spec-core` and normalize authored dep strings once.
- Keep the existing local-only dep path backward compatible.
- Make same-library duplicate-id validation stay local to the resolved library, while allowing
  the same unit id to exist in two different libraries.

**Slice 2. Root-owned library resolution**
- Primary files: [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1), [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797)
- Extend `spec.toml` parsing with `[libraries]`.
- Add a repo-scoped resolver that canonicalizes library roots, rejects out-of-root targets,
  rejects alias-to-self, and rejects duplicate canonical roots.
- Keep only the invoking root library's config authoritative. Imported libraries do not recursively
  widen the graph in M9.

**Slice 3. Validation and cycle truth**
- Primary files: [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303), [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797)
- Resolve direct external libraries before dep-existence checks run.
- Extend cycle detection to the direct root-library plus imported-library graph.
- Reject cross-library `.test.spec` coverage loudly instead of silently treating it as local.

**Slice 4. Generator and compiler truth**
- Primary files: [spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475), root `Cargo.toml` fixtures/examples
- Emit `use <alias>::...` imports for external deps.
- Validate that the consuming crate exposes the same alias in `Cargo.toml`.
- Reject callable-name collisions across local and external deps with a stable error. Do not try
  to invent automatic import alias rewriting in M9.

**Slice 5. Export and fixtures**
- Primary files: [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83), spec CLI/export fixtures
- Bump export to `schema_version: 3` with structured dep refs.
- Add mixed local/cross-library dep fixtures and regression tests.
- Keep export a projection over the public typed dep contract. Do not leak graph internals.

**Slice 6. Example repo proof + verification**
- Add an in-repo second spec library and matching crate dependency alias proof.
- Verification commands:
  - `cargo test -p spec-core`
  - `cargo test -p spec-cli`
  - `cargo test --all`

### Parallelization / Lanes

M9 is only partially parallelizable. The first slice is the gate:

- **Gate:** `Slice 1` typed dep identity must land first. Validator, generator, export, and
  resolver work all need the same dep identity contract before they can move safely.

After `Slice 1` lands, split into two lanes:

- **Lane A. Resolution + validation**
  - `Slice 2` root-owned library resolution
  - `Slice 3` validation and direct cross-library cycle truth
  - Primary surfaces: `spec-cli/src/config.rs`, `spec-cli/src/commands.rs`,
    `spec-core/src/validator.rs`

- **Lane B. Generator + export**
  - `Slice 4` generator and compiler truth
  - `Slice 5` export schema v3 and fixtures
  - Primary surfaces: `spec-core/src/generator.rs`, `spec-core/src/export.rs`, export fixtures

Then reconverge for the final integration lane:

- **Lane C. Example proof + regression**
  - `Slice 6` example sibling library, Cargo alias proof, end-to-end regression coverage,
    and milestone verification commands

**Do not parallelize across these boundaries:**
- Do not start Lane A or Lane B before `Slice 1` lands.
- Do not run Lane C until Lane A and Lane B are both merged, because the example proof and
  regression suite need the final validator, generator, and export contracts together.

### Test Review

```text
CODE PATH COVERAGE
===========================
[+] spec-cli/src/config.rs
    ├── parse [libraries] table
    ├── alias-to-self rejection
    ├── duplicate canonical root rejection
    └── out-of-root path rejection

[+] spec-core/src/types.rs / validator.rs
    ├── typed dep IR parsing
    ├── same-library duplicate ids still rejected
    ├── same local id across two libraries allowed
    └── direct cross-library cycle detection

[+] spec-core/src/generator.rs
    ├── external deps emit use <alias>::...
    ├── missing Cargo dependency alias fails loudly
    └── callable-name collisions across local/external deps

[+] spec-core/src/export.rs
    ├── schema_version 3 dep ref encoding
    └── mixed local/cross-library fixture coverage
```

### Failure Modes Registry

| Codepath | Production failure mode | Planned handling | Silent? |
|---|---|---|---|
| `[libraries]` resolution | Path escapes repo root | `SPEC_LIBRARY_OUT_OF_ROOT` | no |
| `[libraries]` resolution | Alias resolves back to root library | `SPEC_LIBRARY_ALIAS_SELF` | no |
| `[libraries]` resolution | Two aliases resolve to the same canonical library root | `SPEC_DUPLICATE_LIBRARY_ROOT` | no |
| dep identity | Two libraries both define `money/round` | Typed `{library?, id}` contract keeps the dep target unambiguous | no |
| generator import path | Config alias does not match Cargo dependency name | `SPEC_LIBRARY_CRATE_ALIAS_MISSING` | no |
| generator import path | Local and external deps share the same callable name | Stable collision error, no auto alias rewriting | no |
| export | Cross-library dep serialized as a plain string edge | `schema_version: 3` structured dep refs | no |
| molecule coverage | External `.test.spec` cover silently treated as local | Dedicated rejection in M9 | no |

### Success Criteria

- `spec validate` accepts `shared::money/round` syntax with `[libraries]` config.
- `[libraries]` targets outside the repo root are rejected loudly.
- Cross-library deps generate `use <alias>::...` imports and fail validation if the root crate
  does not expose that alias in `Cargo.toml`.
- Cross-library cycle detection catches direct A→B→A across library boundaries.
- Export bumps to `schema_version: 3` and represents cross-library dep endpoints without ambiguity.
- Integration tests cover: valid direct cross-library dep, unknown namespace, missing dep,
  missing library path, out-of-root path, alias-to-self, duplicate canonical root, missing Cargo
  dependency alias, and direct cross-library cycle.
- Example project updated with a second spec library in-repo demonstrating the feature.

### Review-Locked Decisions

- Keep M9 as the next milestone, but narrow it to repo-scoped direct deps.
- Make the namespace alias the only public cross-library identity in M9.
- Keep root `spec.toml` authoritative for `[libraries]`.
- Keep cross-library graph queries out of M9.
- Reject cross-library callable-name collisions instead of inventing automatic aliases.
- Bump export to `schema_version: 3` for structured dep refs.

### What NOT in M9 Scope

- Out-of-repo libraries
- Recursive/transitive library discovery
- Cross-library `.test.spec` covers
- Cross-library `reverse_deps()` / `tests_covering()` / `impact()` semantics
- Package-name-derived import identity

---

## M10 — Planning Boundary as Data (Change Intent + Derived Impact)

**Theme:** Ship the first truthful plan contract after M9. M10 is not a planning UI and not
cross-library change intelligence. It is the minimal authored change-set artifact that lets a
human or AI say "these are the units I intend to change" and receive a derived local-library
retest set without scraping prose.

**Milestone verdict:** M10 should prove one clean boundary:
- authored plan source = intended changes + structured acceptance targets
- derived plan output = advisory impact, computed from the current local graph

That keeps planning explicit without pretending the repo already knows future state.

**User job:**
- A developer can author a local refactor plan and immediately see which existing units and
  molecule tests are in the current blast radius.
- An AI agent can parse one file, validate the intended changes, and get a machine-readable
  impact result instead of guessing from filenames and prose.
- The system stays honest about uncertainty: existing units get derived impact, new units do not.

**Prerequisite:** M9 shipped direct cross-library dep truth, but public graph queries are still
local-library only. M10 consumes the current local `SpecGraph` contract exactly as shipped in M8/M9.
If a future milestone wants cross-library plan impact, it must first define truthful
cross-library `reverse_deps()` / `impact()` semantics.

### Locked Boundary

- One plan file at a time. M10 validates or exports a single `.plan.spec` file by explicit path.
- The plan file must live under a resolved spec-library root. Directory-scoped graph loading is
  anchored to that library root, never to the plan file path.
- `changes[].unit` is local-library only in M10. Any authored `shared::...` unit ref is rejected.
- `computed_impact` is derived output only. It is not authored in `.plan.spec`.
- `modify` and `remove` compute current-graph impact. `add` reports `impact_status: unknown`
  unless a later milestone adds future-edge authoring.
- No plan execution, no progress tracking, no status mutation, no planning UI.
- Do not widen the existing `spec export` bundle contract in M10. Plan export gets its own bundle.

### Authored Schema (`.plan.spec`)

```yaml
# checkout-tax-refactor.plan.spec
id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation to support tiered rates without losing checkout coverage."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
      molecule_tests:
        - pricing/checkout_flow
      notes:
        - "tiered-rate behavior is covered by checkout_flow"
  - unit: pricing/tiered_rate
    action: add
    acceptance:
      validate:
        - pricing/tiered_rate
notes:
  - "M10 plans are local-library only."
```

**Authoring rules:**
- `id` is unique per plan file.
- `intent.why` is required.
- `changes` must be non-empty.
- `changes[].unit` must be a valid local unit id, not a cross-library ref.
- `changes[].unit` values must be unique within one plan file.
- `action` is one of `add | modify | remove`.
- `modify` / `remove` require the unit to exist in the current library graph.
- `add` requires the unit id to be absent from the current library graph while still passing
  unit-id syntax validation.
- `acceptance.validate` lists unit ids that must validate when the work is done.
- `acceptance.molecule_tests` lists existing molecule-test ids that must still pass.
- `notes` fields are optional human guidance, not machine-derived truth.

### Derived Impact Output (`validate` / `export` only)

`computed_impact` is the machine-readable answer to "what current work should I re-check?"

```json
{
  "plan_id": "checkout-tax-refactor",
  "computed_impact": {
    "status": "partial",
    "units": ["pricing/apply_tax", "pricing/calculate_total"],
    "molecule_tests": ["pricing/checkout_flow"],
    "unresolved": [
      {
        "unit": "pricing/tiered_rate",
        "action": "add",
        "reason": "current graph has no node for action=add"
      }
    ]
  }
}
```

**Derived-impact contract:**
- `modify` / `remove` use `graph.impact(unit_id)` from the enclosing library root.
- Changed seed units stay in `computed_impact.units`. They are part of the retest set.
- `add` contributes an unresolved entry, not a fabricated impact set.
- Union impact across multiple changes is sorted and deduplicated.
- `computed_impact` is advisory planning data only. It does **not** mutate `spec status`.

### CLI Contract

`spec plan validate <file>`
- accepts one `.plan.spec` file path
- rejects directories
- resolves the enclosing library root before loading units or molecule tests
- validates authored shape plus action-specific rules
- computes per-change and union `computed_impact`
- should support `--format json` from the first cut so agents do not scrape terminal prose

`spec plan export <file>`
- emits a dedicated `PlanExportBundle`, not the existing `ExportBundle`
- includes the authored plan plus derived `computed_impact`
- keeps plan export schema evolution decoupled from the unit export contract

No plan discovery in M10. The caller passes one plan file explicitly.

### Dedicated Export Shape

```json
{
  "schema_version": 1,
  "spec_version": "0.3.0",
  "exported_at": "2026-04-16T00:00:00Z",
  "plan": { "...authored plan..." },
  "computed_impact": { "...derived output..." },
  "warnings": []
}
```

This is intentionally separate from `spec export`. The existing export bundle is already a
consumer-facing contract for units, molecule tests, passports, and graph edges. M10 should not
take on unrelated schema churn just to ship one plan artifact.

### What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Local declared impact queries | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:49) | Reuse `ImpactSet` as the current-graph truth for `modify/remove`. Do not re-derive impact with ad hoc traversal in CLI code. |
| Workspace + repo boundary knowledge | [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1) | Reuse resolved workspace and repo roots when anchoring plan scope. M10 should extend that trust boundary, not invent a second one. |
| Validation + JSON diagnostics contract | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:56) | Mirror the existing `--format json` posture instead of inventing prose-only output. |
| Directory spec loading | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:2162) | Reuse after adding a dedicated plan-root resolver. File-scoped loading is intentionally too narrow. |
| Molecule test loading | [spec-core/src/loader.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/loader.rs:232) | Reuse for local-library test discovery once the root is resolved. |
| Existing export versioning pattern | [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:22) | Reuse the versioned bundle pattern, but keep M10 in a dedicated plan export surface. |

### Architecture Review

```text
.plan.spec
    │
    ├── authored change intent
    │       └── validate change ids + actions + acceptance targets
    │
    └── spec plan validate/export
            │
            ├── resolve enclosing library root (canonical, repo-bounded)
            ├── load units + molecule tests from that root
            ├── validate against current local graph
            ├── run graph.impact() per supported action
            └── emit PlanReport / PlanExportBundle
```

**Architecture constraints:**
- Plan scope resolution must reuse the existing workspace-root and repo-root truth from
  [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
  but with a dedicated plan-root resolver instead of the current single-file spec loader.
- The plan layer consumes the current public `SpecGraph` contract from
  [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:56),
  not graph internals.
- Symlink traversal and out-of-root paths must be rejected or skipped explicitly during
  plan-root scanning. M10 cannot widen trust boundaries by accident.

### Error & Rescue Registry

| Scenario | What fails | User-visible rescue |
|---|---|---|
| Plan file sits outside any resolved library root | The command cannot know which units/tests define the local graph | Fail with an explicit path-to-root error and tell the caller to move the plan under a library root or pass a path inside one. |
| `changes[].unit` names a missing unit with `action=modify/remove` | The derived impact would be fiction | Fail validation with a stable machine code. No fallback. |
| `changes[].unit` names an already-existing unit with `action=add` | The authored intent conflicts with current graph truth | Fail validation and show the existing unit id. |
| `action=add` asks for impact on a not-yet-existing node | The graph has nothing truthful to traverse | Return `unresolved[]` with `reason`, keep the rest of the plan valid, and mark the overall impact `partial`. |
| Plan consumer wants one machine-readable bundle | Reusing `spec export` would create unrelated schema churn | Emit a dedicated `PlanExportBundle` from `spec plan export`. |

### Code Quality Review

- Keep the first cut explicit. Do not front-load a CLI refactor just to make room for `spec plan`.
  The command can land in the current CLI surface and move later if the command split happens.
- Keep authored plan types and derived-impact types separate. `computed_impact` must be derived
  data, not a field round-tripped through author input.
- Reuse existing JSON error and warning patterns. M10 is a new command surface, not a second
  diagnostics dialect.
- Prefer small dedicated plan types over widening generic export or graph types prematurely.
  The plan contract is new. The graph contract is already shipped.

### Implementation Slices

1. **Plan schema + parser contract**
   - Add typed `.plan.spec` structs for authored fields only.
   - Validate required keys, unique `changes[].unit`, and action enum shape before touching the graph.

2. **Plan-root resolution**
   - Resolve the enclosing library root from the plan file path.
   - Load the full local library spec set from that root, not from the plan file directory.
   - Reject directory input for `spec plan validate/export`; M10 is single-file invocation only.

3. **Action-sensitive validation + derived impact**
   - `modify/remove` require an existing local node and call `graph.impact(unit_id)`.
   - `add` requires a syntactically valid but currently missing unit id and emits unresolved impact.
   - Union and dedupe the per-change `ImpactSet` results deterministically.

4. **CLI contract + JSON output**
   - Add `spec plan validate <file>` with text and `--format json`.
   - Return stable machine-readable validation failures and a structured `computed_impact` payload.

5. **Plan export + docs**
   - Add `spec plan export <file>` with a dedicated versioned bundle.
   - Document the schema in AGENTS.md and README-level machine-readable docs.
   - Keep the existing `spec export` surface untouched.

6. **Regression suite**
   - Add integration tests for root resolution, symlink escape handling, cross-library rejection,
     add/modify/remove action semantics, and deterministic impact union/export ordering.

### Test Review

**Test diagram**

| Codepath / behavior | Test layer | Required coverage |
|---|---|---|
| Parse one `.plan.spec` file and reject directories | CLI integration | `spec plan validate <dir>` fails cleanly; `spec plan validate <file>` succeeds on a valid plan. |
| Resolve enclosing library root from nested plan path | CLI integration | Nested plan file still loads sibling units and molecule tests from the enclosing library root. |
| Validate `modify/remove` against current graph truth | CLI integration + unit | Missing unit id fails with a stable code; existing local unit id passes. |
| Validate `add` against absence-in-graph truth | CLI integration + unit | Existing unit id with `add` fails; missing id yields unresolved impact, not fabricated impact. |
| Reject cross-library `changes[].unit` refs | CLI integration | `shared::pricing/apply_tax` fails loudly in M10. |
| Derive union impact deterministically | spec-core unit + CLI integration | Changed seed units remain in the set, downstream units dedupe, molecule tests dedupe, ordering is stable. |
| Protect root/repo boundary on scan | CLI integration | Symlink escape or out-of-root path is rejected or skipped explicitly with warning/error coverage. |
| Export one plan bundle | CLI integration + fixture | Bundle schema, version, ordering, warnings, and `computed_impact` shape stay stable. |

**Test artifact:** [spensermcconnell-main-m10-test-plan-20260416-191129.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m10-test-plan-20260416-191129.md)

### Performance Review

- The expensive operation in M10 is graph loading, not impact traversal. Keep work scoped to one
  resolved library root and build the graph once per invocation.
- `graph.impact()` already returns sorted, deduped `ImpactSet` data. Reuse it instead of
  recomputing traversals per export projection.
- Root scanning must stay repo-bounded. A fast command that silently walks outside the repo is
  worse than a slower truthful one.
- No caching layer in M10. The local-library graph is small enough, and caching would make root
  correctness harder to reason about in the first cut.

### Parallelization / Lanes

M10 is partially parallelizable, but only after the contract gate is locked.

**Gate 0, do this first and sequentially**
- Lock the authored schema, derived-impact shape, and plan-root resolution rules in the code and
  docs before splitting work.

**Lane A, spec-core contract lane**
- Plan structs and derived-impact types
- Plan export bundle + serializer
- Unit tests for action semantics and deterministic impact projection

**Lane B, spec-cli command lane**
- `spec plan validate/export` command wiring
- Plan-root resolver
- Validation diagnostics and `--format json`

**Join lane, run after A and B land**
- End-to-end integration tests
- README + AGENTS.md updates
- Fixture refresh and final CLI shape polish

**Do not parallelize across these boundaries**
- Do not let both lanes invent their own plan result types. The shared data contract is the gate.
- Do not start export fixtures before the validation payload and bundle schema are locked.
- Do not widen M10 into cross-library impact while Lane B is in flight. That collapses back into a
  sequential post-M9 graph-query milestone.

### Failure Modes

| Codepath | Failure mode | Test covers? | Error handling? | Silent? |
|---|---|---|---|---|
| plan root resolution | plan file outside any resolved library root | no | fail with explicit path/root error | no |
| plan root scan | symlink escapes the library or repo root | no | reject or skip with explicit warning/error | **critical gap** |
| single-file invocation | graph built from the plan file path instead of the library root | no | dedicated resolver required | **critical gap** |
| `computed_impact` projection | authored and derived impact shapes drift | no | derived-only contract | **critical gap** |
| `action=add` | fake impact reported for a unit that is not yet in the graph | no | unresolved entry + partial status | no |
| plan export | existing unit export schema churns for one new artifact | yes (by contract choice) | separate bundle | no |
| conflicting changes | same unit listed twice with incompatible actions | no | fail validation | no |

### What NOT in M10 Scope

- Cross-library plan changes or cross-library impact queries
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

### Implementation Order

```text
1. Lock plan schema, derived-impact shape, and root-resolution contract
2. Implement plan structs + command parsing
3. Implement plan-root resolver and graph loading from enclosing library root
4. Implement action-sensitive validation and ImpactSet projection
5. Add `spec plan validate --format json`
6. Add dedicated `spec plan export` bundle
7. Land integration tests, fixtures, and docs
8. Re-review before widening scope beyond local-library truth
```

### Success Criteria

- `spec plan validate <file>` accepts one `.plan.spec` file and rejects directories.
- Plan validation resolves the enclosing library root before loading the graph.
- `modify` / `remove` require an existing local unit id.
- `add` requires a missing local unit id and reports derived impact as unresolved/unknown.
- Cross-library unit ids in `changes[].unit` are rejected in M10.
- `computed_impact` is derived-only, structured as `{status, units, molecule_tests, unresolved}`.
- `spec plan export <file>` emits a dedicated versioned plan export bundle.
- Schema is documented in AGENTS.md and README-level machine-readable docs, not only agent prompts.
- Integration tests cover:
  - valid local-only modify plan
  - valid mixed modify/add plan
  - unknown unit id for `modify`
  - duplicate/conflicting `changes[].unit`
  - cross-library unit ref rejected in a plan
  - single-file nested plan path still loads the full library graph
  - symlink escape rejected or skipped explicitly
  - impact union includes downstream molecule tests and keeps changed seed units
  - plan export schema/version behavior

---

## M10 Review Record (2026-04-16)

`/autoplan` was run against the refreshed M10 scope and grounded against
[docs/north_star_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/north_star_v0.2.md:101),
[docs/high_level_technical_architecture_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/high_level_technical_architecture_v0.2.md:102),
[docs/roadmap_and_release_shape_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/roadmap_and_release_shape_v0.1.md:413),
[spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:56),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:22),
[spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
[spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:56),
and [spec-core/src/loader.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/loader.rs:232).

Outcome:
- CEO correction: M10 should solve change intelligence for one library, not merely introduce a
  file extension.
- Eng correction: root resolution, symlink boundaries, action-sensitive impact semantics, and a
  dedicated plan export contract must be explicit in the milestone, not left to implementer taste.
- Design review skipped, no UI scope.
- Outside voice: Codex ran twice (CEO + Eng). Delegated subagents were unavailable in this thread
  by session policy.
- Test artifact: [spensermcconnell-main-m10-test-plan-20260416-191129.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m10-test-plan-20260416-191129.md)
- Review-time taste choices are now resolved in the milestone text above:
  keep M10 local-library only, and ship a dedicated plan export bundle.

### CEO Dual Voices — Consensus Table

```text
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   N/A     Partial single-model concern
  2. Right problem to solve?           N/A     No      single-model concern
  3. Scope calibration correct?        N/A     Partial taste disagreement
  4. Alternatives sufficiently explored?N/A    No      single-model concern
  5. Competitive/market risks covered? N/A     Partial single-model concern
  6. 6-month trajectory sound?         N/A     Partial taste disagreement
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (CEO — strategy challenge):**
- Do not ship YAML theater. M10 must change how developers and AI understand intended change.
- Free-text acceptance and authored `impacted` lists would rot immediately.
- The roadmap is more credible if M10 proves a local-library planning contract first, then opens a
  separate cross-library change-intelligence milestone.

**CLAUDE SUBAGENT (CEO — independent review):** unavailable in this run. Session policy for this
thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Design Review

Skipped, no UI scope. M10 is a CLI/data-artifact milestone.

### ENG Dual Voices — Consensus Table

```text
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               N/A     Partial single-model concern
  2. Test coverage sufficient?         N/A     No      single-model concern
  3. Performance risks addressed?      N/A     Yes     single-model positive
  4. Security threats covered?         N/A     No      single-model concern
  5. Error paths handled?              N/A     No      single-model concern
  6. Deployment risk manageable?       N/A     Yes     single-model positive
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (eng — architecture challenge):**
- Reusing single-file loaders for `spec plan validate <file>` would under-report sibling units and
  molecule tests.
- The plan layer widens a real trust boundary unless root-scoped path resolution and symlink
  handling are made explicit.
- `action=add` cannot truthfully use current-graph impact and must report uncertainty explicitly.
- Plan export needs a stable bundle contract now, not an implied future schema bump.

**CLAUDE SUBAGENT (eng — independent review):** unavailable in this run. Session policy for this
thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Cross-Phase Themes

- **Truth before convenience** — both passes converged on the same rule: do not author or export
  derived impact as if it were source truth.
- **Scope from roots, not files** — both passes independently pushed the same implementation
  constraint: plan validation must resolve the library root first or it will lie.

### NOT in Scope (M10 pass)

- Cross-library plan changes or cross-library impact queries
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

### Completion Summary

```text
  +====================================================================+
  |                M10 /autoplan REVIEW — COMPLETION SUMMARY           |
  +====================================================================+
  | Mode selected        | SELECTIVE_EXPANSION                         |
  | Premise gate         | implicit via "solidify M10 after M9 landed" |
  | Section 1  (Arch)    | 4 contract issues fixed in-plan             |
  | Section 2  (Errors)  | failure modes updated, 3 critical gaps      |
  | Section 3  (Security)| 2 path/root boundary issues named           |
  | Section 4  (Data/UX) | skipped, no UI scope                        |
  | Section 5  (Quality) | 3 schema/contract drift issues fixed        |
  | Section 6  (Tests)   | diagram + QA artifact produced              |
  | Section 7  (Perf)    | no new runtime hotspot beyond root scan     |
  | Section 8  (Observ)  | skipped, no runtime surface in M10          |
  | Section 9  (Deploy)  | no deploy surface                           |
  | Section 10 (Future)  | post-M10 cross-library follow-on named      |
  | Section 11 (Design)  | SKIPPED (no UI scope)                       |
  +--------------------------------------------------------------------+
  | NOT in scope         | written (5 items)                           |
  | What already exists  | written                                     |
  | Failure modes        | 7 rows, 3 critical gaps                     |
  | Test artifact        | written                                     |
  | Outside voice        | ran (codex-only)                            |
  | Unresolved decisions | 2 taste choices, 0 blockers                 |
  +====================================================================+
```

The M10 section above is now the authoritative source of truth. This review record stays only as
historical evidence for why the boundary and contract were locked this way.

---

## M9 Review Record (2026-04-15)

`/autoplan` was run against the refreshed M9 scope and grounded against
[DECISIONS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/DECISIONS.md:56),
[spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1),
[spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1),
[spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475),
[spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303),
[spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797).

Outcome:
- CEO correction: keep M9 next, but narrow it to direct repo-scoped shared-library reuse.
- Eng correction: make dep identity, root-owned config, Cargo alias validation, and export schema
  v3 explicit in the milestone contract.
- Design review skipped, no UI scope.
- Outside voice: Codex ran, delegated subagents were unavailable in this thread by policy.
- Test artifact: [spensermcconnell-main-m9-test-plan-20260415-211200.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m9-test-plan-20260415-211200.md)
- Unresolved plan decisions: 0

The M9 section above is now the authoritative source of truth. This review record stays only as
historical evidence for why the scope and boundary were locked this way.

## M8-M10 /autoplan Eng Review (2026-04-14)

**Review scope:** updated M8/M9/M10 roadmap sections, checked against current graph/export/status
implementation in [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1),
[spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1),
[spec-core/src/passport.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/passport.rs:1),
and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:61).

### Architecture Review

**System architecture**

```text
Loaded .unit.spec + .test.spec
        │
        ▼
  validator (authoritative integrity gate)
        │
        ▼
 validated graph input
        │
        ▼
     SpecGraph
   ├── units
   ├── molecule_tests
   ├── declared dep edges
   ├── declared covers edges
   └── reverse indexes
        │
        ├── export projection
        ├── planning impact queries
        └── future M9 cross-library extension
```

**Architecture finding:** `SpecGraph::build()` should not become a public blind copier over raw
loaded specs. Today graph integrity checks live in CLI validation, not inside `graph.rs`. M8
must either build from validated input or return a fallible result.

**Architecture finding:** export must remain a projection over graph, not a serialization of
graph structs directly. Otherwise M9 graph evolution will become export-schema churn.

### Code Quality Review

- Current graph storage is flat vectors only. That is acceptable for M7 export, but not for the
  repeated `reverse_deps`, `tests_covering`, and `impact` queries M8/M10 want. The plan now
  needs reverse indexes baked into construction.
- The repo still carries `links.molecule_tests` as legacy metadata in `SpecStruct`. M8 must name
  its treatment explicitly so there is one relationship contract, not two.
- Cross-library dep parsing cannot stay stringly typed. M9 now explicitly owns a typed dep IR in
  `spec-core`, not a graph-only patch.

### Test Review

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/graph.rs
    │
    ├── [★   TESTED] build() creates dep + covers edges
    ├── [GAP]         reverse_deps() direct dependent lookup
    ├── [GAP]         reverse_deps() transitive closure
    ├── [GAP]         tests_covering() direct and multiple tests
    ├── [GAP]         impact() includes downstream units + their covering tests
    ├── [GAP]         unknown unit id contract (Result/Option vs silent empty)
    └── [GAP]         large fan-out/fan-in indexing behavior

[+] spec-core/src/export.rs
    │
    ├── [★★  TESTED] export builds graph edges through SpecGraph
    ├── [GAP]         export remains projection when graph adds new fields
    └── [GAP]         deterministic projection with graph query indexes present

[+] M9 cross-library dep layer
    │
    ├── [GAP]         parsed DepId IR round-trip from authored YAML
    ├── [GAP]         unknown namespace
    ├── [GAP]         missing canonicalized path
    ├── [GAP]         alias-to-self / duplicate canonical root
    ├── [GAP]         symlink-cycle external root
    └── [GAP]         cross-library cycle in graph + generator integration

[+] M10 plan artifact layer
    │
    ├── [GAP]         action=modify requires existing unit
    ├── [GAP]         action=add requires non-existent unit
    ├── [GAP]         graph scope resolves from enclosing spec-library root
    └── [GAP]         impact includes downstream molecule tests, not just direct seed tests

─────────────────────────────────
COVERAGE: existing tests prove seed graph construction and export projection basics.
GAPS: graph query semantics, typed dep identity, plan action validation, and external-library path trust boundaries.
─────────────────────────────────
```

### Performance Review

- Repeated graph queries over flat `Vec` scans will degrade once M9 loads multiple libraries.
  The plan now requires reverse indexes built once during graph construction.
- Deterministic ordering is part of the performance and correctness contract, because export
  snapshots and planning artifacts should not flap.

### ENG Dual Voices — Consensus Table

```text
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               N/A     Partial single-model concern
  2. Test coverage sufficient?         N/A     No      single-model concern
  3. Performance risks addressed?      N/A     Partial single-model concern
  4. Security threats covered?         N/A     Partial single-model concern
  5. Error paths handled?              N/A     Partial single-model concern
  6. Deployment risk manageable?       N/A     Yes     single-model positive
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (eng — architecture challenge):**
- `impact()` was under-specified and would under-report downstream molecule tests.
- M10 `action: add` contradicted the existing-unit validation rule.
- M9 needed typed dep identity at the `spec-core` layer, not just extra graph metadata.
- Graph scope resolution for plan commands had to be anchored at the enclosing library root.
- Graph query APIs needed explicit unknown-id behavior and indexed internals.

**CLAUDE SUBAGENT (eng — independent review):** unavailable in this run. Session policy for
this thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Cross-Phase Themes

- **Truth over convenience** — Phase 1 and Phase 3 both flagged the same risk: do not let M8
  pretend to know more than the repo can currently observe.
- **Type identity before metadata** — Phase 1 and Phase 3 both converged on the same M9 rule:
  cross-library identity must become a typed core contract before graph decorations land.

### Test Plan Artifact

- QA handoff written to [spensermcconnell-main-eng-review-test-plan-20260414-223534.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-eng-review-test-plan-20260414-223534.md)

### Completion Summary

```text
  +====================================================================+
  |            M8-M10 /autoplan REVIEW — COMPLETION SUMMARY            |
  +====================================================================+
  | Mode selected        | SELECTIVE_EXPANSION                         |
  | System Audit         | M8 reframed as declared graph contract      |
  | Step 0               | premise gate passed with user option A      |
  | Section 1  (Arch)    | 4 issues found                              |
  | Section 2  (Errors)  | failure modes updated, 3 critical gaps      |
  | Section 3  (Security)| 2 filesystem trust-boundary issues          |
  | Section 4  (Data/UX) | skipped, no UI scope                        |
  | Section 5  (Quality) | 3 contract-drift issues found               |
  | Section 6  (Tests)   | diagram produced, major gaps identified     |
  | Section 7  (Perf)    | 1 index/query-shape issue found             |
  | Section 8  (Observ)  | skipped, no new runtime surface in M8       |
  | Section 9  (Deploy)  | roadmap-only, no new deploy gate required   |
  | Section 10 (Future)  | M9/M10 cascades updated                     |
  | Section 11 (Design)  | SKIPPED (no UI scope)                       |
  +--------------------------------------------------------------------+
  | NOT in scope         | written and refreshed                       |
  | What already exists  | written and refreshed                       |
  | Error/rescue registry| failure modes table updated                 |
  | Failure modes        | 5 rows, 3 critical gaps                     |
  | TODOS.md updates     | roadmap TODO section updated in-plan        |
  | Scope proposals      | 3 evaluated, contract-first path accepted   |
  | CEO plan             | not externalized; review captured in plan   |
  | Outside voice        | ran (codex-only)                            |
  | Lake Score           | 6/6 major decisions chose complete option   |
  | Diagrams produced    | architecture, test coverage                 |
  | Stale diagrams found | 0                                           |
  | Unresolved decisions | 0 user-blocking, 2 roadmap clarifications   |
  +====================================================================+
```


## Failure Modes

| Codepath | Production failure mode | Test covers? | Error handling? | Silent? |
|---|---|---|---|---|
| Default output anchored to crate root | crate_root not resolved (no Cargo.toml) | yes (workspace_root_for tests) | bail with clear message | no |
| Evidence preservation in write_passports | passport file corrupted on disk | via serde deserialize | returns None, writes fresh | no |
| 6-state status transitions | clock skew between observed_at and now | N/A | timestamp is informational | no |
| .test.spec covers validation | covers unit deleted after test authored | yes (integration) | SPEC_MOLECULE_COVERS_NOT_FOUND | no |
| graph.impact() | downstream molecule tests omitted from retest set | yes (planned `impact_includes_downstream_covering_tests_not_just_seed_tests`) | `ImpactSet` contract + BFS closure over reverse deps | no |
| graph query API | unknown unit id returns empty and looks valid | yes (planned `*_unknown_unit_returns_none` tests) | explicit `Option` contract on all graph query methods | no |
| Cross-library dep resolution | [libraries] path not found on disk | partial | needs explicit test + loud error | **critical gap** |
| Cross-library dep resolution | alias resolves to self or duplicate canonical root | no | plan now requires rejection | **critical gap** |
| Plan artifact impact computation | graph built from file path instead of library root | no | plan now requires root resolution | **critical gap** |

**Critical gaps:**
- M9 needs explicit tests for missing library path, alias-to-self, duplicate canonical root,
  and symlink-looped external roots.
- M10 needs explicit tests proving plan validation resolves graph scope from the enclosing
  library root and handles `action: add` differently from `modify/remove`.

---

## NOT in Scope (Deferred)

- TypeScript / Python / Go generator targets (moved from M5 design doc; re-evaluate after M8)
- `ValidatedExpr` as a public library type (bundled into structural PR as internal refactor only)
- Observed coverage edges (molecule tests declare coverage; observation requires instrumentation)
- Molecule test passports / evidence tracking (molecule tests run via cargo test, but status
  tracking for them deferred until M8 graph is solid)
- Nextest support (detect nextest format and surface clear error rather than "unknown" — nice-to-have after M6a)
- LLM semantic contract-vs-body scoring
- CUE
- Reverse ingestion

---

## What Already Exists (reuse, don't rebuild)

The authoritative M10 reuse map now lives inside the milestone section above. Keep reusing:
- existing workspace + repo boundary resolution in `spec-cli/src/config.rs`
- local impact truth in `spec-core/src/graph.rs`
- versioned export-bundle patterns in `spec-core/src/export.rs`
- existing JSON fixture and CLI integration-test posture in `spec-cli/tests/`

---

## Worktree Parallelization

| Step | Modules touched | Depends on |
|---|---|---|
| Contract gate | `PLAN.md`, plan schema types, root-resolution contract notes | — |
| Lane A: spec-core plan contract | `spec-core` plan types, derived-impact types, plan export builder, unit tests | Contract gate |
| Lane B: spec-cli plan commands | `spec-cli` command wiring, plan-root resolver, validation diagnostics, CLI integration scaffolding | Contract gate |
| Join lane | integration tests, fixtures, README, AGENTS.md | Lane A + Lane B |

**Parallel lanes**
- `Lane A:` shared plan data contract in `spec-core`
- `Lane B:` CLI validate/export surface in `spec-cli`
- `Join lane:` end-to-end verification and docs after both land

**Execution order**
- Lock the schema and resolver contract first.
- Launch `Lane A` and `Lane B` in parallel only after that gate.
- Run the join lane last for integration coverage, fixture updates, and docs.

**Conflict flags**
- Both lanes depend on one shared `computed_impact` contract. Do not let each lane invent its own shape.
- Do not start fixture churn before the validate/export payloads are locked.
- If M10 scope expands into cross-library impact, stop parallelization and re-plan the milestone.

---

## TODOS.md Updates

This pass does not reopen shipped M6-M9 work. New M10-specific follow-ups to add:

- `[M10] Add stable error codes for plan outside library root, duplicate plan change ids,
  cross-library plan refs, modify/remove on missing unit, and add on existing unit.`
- `[M10] Add CLI fixtures for \`spec plan validate --format json\` and
  \`spec plan export\` schema_version 1 ordering.`
- `[post-M10] Decide whether future-edge authoring for \`action=add\` becomes a first-class plan
  feature or stays unresolved until a later graph-query milestone.`
- `[post-M10] Cross-library plan impact semantics need their own milestone after local-library
  plan truth is proven.`

---

## Implementation Order

**Current milestone: M10. M6a through M9 are shipped.**

```text
1. Lock M10 plan schema + root-resolution contract
   - single-file invocation only
   - local-library authored ids only
   - derived impact remains output-only

2. Implement spec-core plan contract
   - typed authored-plan structs
   - typed derived-impact structs
   - dedicated plan export bundle

3. Implement spec-cli plan commands
   - `spec plan validate <file>`
   - `spec plan export <file>`
   - root-scoped plan loading and validation diagnostics

4. Add regression suite
   - action-specific validation coverage
   - nested plan-path root resolution
   - symlink escape / root-boundary enforcement
   - deterministic impact union + export fixtures

5. Verification
   - cargo test -p spec-core
   - cargo test -p spec-cli
   - cargo test --all

6. Re-review before widening
   - keep M10 local-library scoped unless a later milestone expands query semantics

7. /ship when implementation lands
```

**Do not front-load into this PR:**
- Cross-library plan refs or cross-library impact
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

---

**Document version:** 2026-04-16
**Review status:** M10 consolidated into one implementation-ready plan section
**Next review checkpoint:** After M10 command surface lands, before any scope widening

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 5 | clean (PLAN via /autoplan) | M10 narrowed to truthful local-library change intent plus derived impact, not planning theater |
| Codex Review | `/codex review` | Independent 2nd opinion | 10+ | issues_found | M10: root resolution, action-sensitive impact semantics, dedicated plan export, and trust-boundary clarity |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 10 | **CLEAR (PLAN)** | M10 gaps made explicit: root-scoped loading, failure modes, test coverage, and parallelization |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | issues_open | score: 5/10 → 7/10, TTHW: 5min-local/BLOCKED-external |

**CODEX (M10):** flagged the real missing pieces: root-scoped graph loading, explicit `action=add`
uncertainty, stable plan JSON/export contracts, and path-boundary handling that does not widen
the repo trust surface by accident.
**UNRESOLVED:** 0
**VERDICT:** PLAN LOCKED — start with the M10 schema and root-resolution contract, then land the
`spec-core` plan types, then the `spec-cli` validate/export surface, then the regression suite
before `/ship`.
