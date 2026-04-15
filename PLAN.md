<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260414-222253.md -->
# Next Work: M6–M10 Roadmap

Status: **Implementation Ready** (M6a approved)

Reviewed via `/plan-eng-review` 2026-04-12. ChatGPT 5.4 Pro + Codex outside voice both consulted.  
M5 and M5 follow-up (v0.5.1) have shipped. This plan covers the next five milestones.

---

## Milestone Summary

```
M6a  Trust Gap Fixes          ← next to implement
M6b  Health Model             ← after M6a
     structural PR            ← commands.rs split (zero behavior change)
M7   .test.spec + minimal graph
M8   Full Graph Layer
M9   Cross-library Deps
M10  Planning Boundary as Data
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
- Integration tests cover: build contract, `reverse_deps()`, `tests_covering()`, `impact()` (including the "molecule test covering downstream, not seed" case), relationship source-of-truth behavior, and unknown-unit-id contracts.
- `build()` doc comment explicitly states "assumes validated input" and "links.molecule_tests is explicitly not read."

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

---

## M9 — Cross-library Deps

**Theme:** Implement the `shared::money/round` namespace-prefixed dep syntax with
`[libraries]` config mapping.

**Prerequisite:** M8 declared graph contract complete. Do not implement M9 until local
`reverse_deps()` / `tests_covering()` / `impact()` semantics are locked and the graph has a
single source of relationship truth.

### spec.toml Contract

```toml
[libraries]
shared = "../shared-spec"   # namespace alias → path to another spec library root
payments = "../../payments/spec"
```

### Dep Syntax

```yaml
deps:
  - money/round              # local dep (same library)
  - shared::money/round      # cross-library dep
```

### Validation

- Unknown library namespace → `SPEC_UNKNOWN_LIBRARY_NAMESPACE`
- Cross-library dep id not found in target library → `SPEC_CROSS_LIBRARY_DEP_NOT_FOUND`
- Cross-library cycles (A depends on B depends on A across library boundary) → `SPEC_CROSS_LIBRARY_CYCLE`
- Legacy local deps (`money/round`) continue to work unchanged.
- `[libraries]` paths are canonicalized before use.
- Self-targeting aliases, duplicate aliases to the same canonical root, and symlink-looped
  external roots are rejected loudly.

### Generation

Cross-library dep generates a `use` statement pointing at the other crate's module:
`use shared::money::round;` where `shared` is the Cargo package name or a configured alias.
The `[libraries]` mapping must also record the Cargo package name (or derive it from the
target spec library's Cargo.toml `[package] name`).

### Cargo Cycle Detection

Semantic cycles (spec dep graph) are already detected. Cross-library deps may also introduce
Cargo build dependency cycles. `spec validate` should check the `[libraries]` dep graph is
a DAG — if library A's spec depends on library B, B must not depend back on A.

### SpecGraph Extension

M9 is where graph metadata becomes cross-library aware. Extend the M8 declared graph with:
- a typed dep IR at the `spec-core` layer that distinguishes local ids from
  `namespace::unit/id`
- `LibraryContext` for loading external spec sets
- cross-library edge metadata (`scope: CrossLibrary`, `library_id`) only after dep identity is real

Do not back-port these fields into M8 as placeholders. M9 owns the first truthful version.
The typed dep IR must be shared by validator, generator, graph, and export. It cannot live
only inside graph code or the repo will keep reparsing raw strings in four places.

### Success Criteria

- `spec validate` accepts `shared::money/round` syntax with `[libraries]` config.
- Cross-library dep generates correct `use` statement in Rust output.
- Cross-library cycle detection catches A→B→A across library boundaries.
- Graph queries (`reverse_deps`, `impact`) work across library boundaries once `LibraryContext` is present.
- Integration tests: valid cross-library dep, unknown namespace, missing dep, cycle,
  missing library path, alias-to-self, duplicate canonical library root, and symlink-cycle external root.
- Example project updated with a second spec library demonstrating the feature.

---

## M10 — Planning Boundary as Data

**Theme:** Define the first minimal plan artifact without building a planning UI. Clarifies
the architecture boundary between spec (verification) and planning (intent).

**Prerequisite:** M8 declared local impact complete. M10 consumes `graph.impact()` as
advisory planning data. If a plan needs cross-library impact, that dependency moves to M9.

### Plan Artifact Schema (.plan.spec)

```yaml
# feature-name.plan.spec
id: checkout-tax-refactor
intent: "Refactor tax calculation to support tiered rates."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance: "apply_tax correctly handles tiered rates per test suite"
  - unit: pricing/tiered_rate
    action: add
    acceptance: "new unit validates and generates"
impacted:
  - pricing/apply_discount  # downstream dep computed by spec
  - pricing/checkout_flow   # molecule test that covers apply_tax
```

The `impacted` list is computed by `graph.impact()` at plan-read time and included in the
export. It is advisory, declared blast radius data. It tells the implementer what else to
re-test, not what to change, and it does **not** affect `spec status`.

For M10, `graph.impact()` is evaluated from the enclosing spec-library root, not from the
individual `.plan.spec` file path. This avoids silently dropping sibling molecule tests or
downstream units because the loader scope was too narrow.

### CLI

`spec plan validate <path>` — validate plan artifact, compute impact set, report.

`spec plan export <path>` — include plan artifact in export bundle.

No plan execution, no plan tracking, no planning UI in M10. The plan is a data artifact
that AI agents and humans read to understand scope. The value is making the intent explicit
and machine-readable, not automating the work.

### Success Criteria

- `spec plan validate` accepts `.plan.spec` files and validates changes action-sensitively:
  - `modify` / `remove` require an existing unit id
  - `add` requires a non-existent unit id plus valid id syntax
- `spec plan validate` computes impact via `graph.impact()` from the enclosing spec-library root.
- Export includes plan artifact and computed `impacted` list.
- Schema documented in AGENTS.md so AI agents can read and write plan artifacts.
- Integration tests: valid plan, unknown unit id for `modify`, invalid duplicate unit id for `add`,
  and impact computation including downstream molecule tests.

---

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
| graph.impact() | downstream molecule tests omitted from retest set | no | plan now defines closure behavior; tests still needed | **yes if untested** |
| graph query API | unknown unit id returns empty and looks valid | no | contract must be explicit (`Result` / `Option`) | **yes if left implicit** |
| Cross-library dep resolution | [libraries] path not found on disk | partial | needs explicit test + loud error | **critical gap** |
| Cross-library dep resolution | alias resolves to self or duplicate canonical root | no | plan now requires rejection | **critical gap** |
| Plan artifact impact computation | graph built from file path instead of library root | no | plan now requires root resolution | **critical gap** |

**Critical gaps:**
- M8 needs explicit tests for impact-closure semantics and unknown-unit-id behavior.
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

| Sub-problem | Existing code |
|---|---|
| Crate root resolution | `pipeline.rs:workspace_root_for` |
| Output path safety | `generator.rs:safe_output_path`, `ensure_output_marker` |
| Stale detection | `commands.rs:contract_hash_for`, `write_passports` |
| Cycle detection | `validator.rs:detect_cycles` |
| Export bundle | `spec-core/src/export.rs:ExportBundle` |
| Minimal declared graph | `spec-core/src/graph.rs:SpecGraph` |
| Error code registry | `commands.rs:spec_error_code` |
| JSON fixture tests | `spec-cli/tests/fixtures/` |
| Relationship validation | `commands.rs:validate_molecule_tests`, `validator.rs:validate_molecule_test_covers` |

---

## Worktree Parallelization

| Step | Modules touched | Depends on |
|---|---|---|
| M8 graph contract + query API | `spec-core/src/graph.rs`, `spec-core/src/lib.rs`, `spec-core/src/export.rs`, graph unit tests | — |
| M8 relationship-source cleanup | `spec-core/src/types.rs`, validator/loader call sites, docs | M8 graph contract decisions |
| M9 typed dep identity | `spec-core/src/types.rs`, validator, generator, export, graph | M8 merged |
| M9 external library loading + validation | loader, validator, config, integration tests | M9 typed dep identity |
| M10 plan artifact + root-scoped impact | new plan command surface, graph query integration, export projection, integration tests | M8 merged, M9 only if cross-library impact needed |

**Parallel lanes**
- `Lane A:` M8 graph contract → M8 relationship-source cleanup
- `Lane B:` M10 plan artifact work after M8 merge, if limited to local-library impact
- `Lane C:` M9 typed dep identity → M9 external library loading + validation

**Execution order**
- Launch `Lane A` first.
- After M8 merges, launch `Lane B` and `Lane C` in parallel worktrees only if M10 remains local-library only.
- If M10 is later expanded to require cross-library impact, it rejoins `Lane C` and becomes sequential behind M9.

**Conflict flags**
- `Lane B` and `Lane C` both want graph query semantics if M10 asks for cross-library impact. Keep M10 local-only or sequence it behind M9.
- M8 relationship-source cleanup and M9 typed dep identity both touch `spec-core/src/types.rs`; do not run those in parallel.

---

## TODOS.md Updates

The following TODOS items are closed or addressed by this plan:

- `spec build/generate overwrites passport evidence` → fixed in M6a
- `pipeline.rs eprintln! forward-compat` → fixed in M6a
- `D5a newtype refactor (ValidatedExpr)` → bundled into structural PR
- `nextest limitation documentation` → M6a doc task
- `Cross-library dep IMPLEMENTATION` → M9
- `M6: Semantic contract-vs-body comparison (LLM eval)` → deferred

New TODOS to add:

- `[M6a investigation] Auto-derive module prefix vs explicit config key: consider whether
  generated_module_prefix config key can be eliminated entirely once auto-derivation is
  validated in practice.`
- `[M9 prerequisite] Explicit test: spec validate fails with clear error when [libraries]
  path does not exist on disk.` (critical gap per failure modes table)
- `[post-M6a] Nextest detection: instead of README note, detect nextest output format and
  emit SPEC_UNSUPPORTED_TEST_RUNNER error rather than producing "unknown" test results.`

---

## Implementation Order

**M6a first. Ship it before doing anything else.**

```
1. spec test ecommerce/units (confirm the bug reproduces)
2. Fix output anchoring + auto-derive prefix
3. Fix evidence preservation in write_passports
4. Fix eprintln! compat in pipeline.rs
5. Add nextest doc to README
6. Regenerate + commit example passports
7. cargo test --all → green
8. /ship
```

Then structural PR (commands.rs split + ValidatedExpr).

Then M6b (status health model).

---

**Document version:** 2026-04-12  
**Review status:** Approved via /plan-eng-review  
**Next review checkpoint:** Before /ship on M6a

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 3 | clean (PLAN via /autoplan) | mode: SELECTIVE_EXPANSION, 0 critical gaps |
| Codex Review | `/codex review` | Independent 2nd opinion | 7+ | issues_found | M8: ImpactSet struct, private fields, trust boundary — all incorporated |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 7 | **CLEAR (PLAN)** | 4 issues found, 0 unresolved, 2 critical gaps (downstream-tests test, diamond dedup test) |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | issues_open | score: 5/10 → 7/10, TTHW: 5min-local/BLOCKED-external |

**CODEX (M8):** 7 findings — ImpactSet struct for `impact()` return type (incorporated), private fields + accessor methods (incorporated), trust boundary doc comment (incorporated), three-copies concern (deferred as expected graph pattern), deterministic output (in implementation notes), export two-path concern (not M8 scope), local tests implicit in unit IDs (documented in plan).
**UNRESOLVED:** 0
**VERDICT:** ENG CLEARED — M8 ready to implement. M6a through M7 shipped. Begin M8 graph layer.
