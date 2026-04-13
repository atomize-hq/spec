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

**Theme:** Formalize the graph model so M9 (cross-library) and M10 (planning) have a clean
foundation. Extend the minimal `SpecGraph` from M7 into a first-class answerable object.

### Core Questions the Graph Must Answer

```
1. What are all the units?                    → graph.units()
2. What are all the molecule tests?           → graph.molecule_tests()
3. What edges exist (dep + covers)?           → graph.edges()
4. Which edges are declared vs. observed?     → edge.kind (Declared | Observed)
5. Which deps are internal vs. external?      → edge.scope (Internal | CrossLibrary)
6. What library does each node belong to?     → node.library_id
7. What is the reverse dependency set?        → graph.reverse_deps(unit_id)
8. What molecule tests cover a given unit?    → graph.tests_covering(unit_id)
```

### graph.build() Contract

`SpecGraph::build(loaded_units, molecule_tests, passports)` constructs the graph from:
- Loaded `.unit.spec` files (units, deps, local_tests)
- Loaded `.test.spec` files (molecule tests, covers edges)
- Passports (observed test results → future observed edges in M9+)

Graph source of truth: **the spec files**. Passports contribute observed status but do not
add or remove edges. Generated code is derived and ephemeral — not a graph input.

### Invalidation Rules

The graph is rebuilt on each command invocation from the current spec files. No persistent
graph state between runs. This avoids staleness. The export bundle captures a snapshot.

### Impact Analysis (foundation for M10)

`graph.impact(unit_id)` returns: all units that transitively depend on `unit_id`, plus all
molecule tests that cover `unit_id`. This is the minimal impact set for a change.

### Success Criteria

- `SpecGraph` lives in `spec-core`, exposed from `lib.rs`.
- `spec export` uses `SpecGraph::build()` instead of ad-hoc edge construction.
- `spec status` uses `graph.impact()` to detect downstream stale units (optional in M8,
  required in M9).
- All M7 molecule test / covers edge behavior migrated to use `SpecGraph`.
- Integration tests for `graph.impact()`, `graph.reverse_deps()`, `graph.tests_covering()`.

---

## M9 — Cross-library Deps

**Theme:** Implement the `shared::money/round` namespace-prefixed dep syntax with
`[libraries]` config mapping.

**Prerequisite:** M8 graph layer complete. Do not implement before SpecGraph supports
library scope on nodes.

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

`SpecGraph::build()` accepts an optional `LibraryContext` that maps namespace aliases to
loaded spec sets from other libraries. Cross-library edges get `scope: CrossLibrary` and
`library_id` pointing at the external library namespace.

### Success Criteria

- `spec validate` accepts `shared::money/round` syntax with `[libraries]` config.
- Cross-library dep generates correct `use` statement in Rust output.
- Cross-library cycle detection catches A→B→A across library boundaries.
- Integration tests: valid cross-library dep, unknown namespace, missing dep, cycle.
- Example project updated with a second spec library demonstrating the feature.

---

## M10 — Planning Boundary as Data

**Theme:** Define the first minimal plan artifact without building a planning UI. Clarifies
the architecture boundary between spec (verification) and planning (intent).

**Prerequisite:** M8 graph layer complete. `graph.impact()` must be reliable.

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
export. It is advisory — it tells the implementer what else to re-test, not what to change.

### CLI

`spec plan validate <path>` — validate plan artifact, compute impact set, report.

`spec plan export <path>` — include plan artifact in export bundle.

No plan execution, no plan tracking, no planning UI in M10. The plan is a data artifact
that AI agents and humans read to understand scope. The value is making the intent explicit
and machine-readable, not automating the work.

### Success Criteria

- `spec plan validate` accepts `.plan.spec` files, validates unit ids exist, computes
  impact via `graph.impact()`.
- Export includes plan artifact and computed `impacted` list.
- Schema documented in AGENTS.md so AI agents can read and write plan artifacts.
- Integration tests: valid plan, unknown unit id, impact computation.

---

## Failure Modes

| Codepath | Production failure mode | Test covers? | Error handling? | Silent? |
|---|---|---|---|---|
| Default output anchored to crate root | crate_root not resolved (no Cargo.toml) | yes (workspace_root_for tests) | bail with clear message | no |
| Evidence preservation in write_passports | passport file corrupted on disk | via serde deserialize | returns None, writes fresh | no |
| 6-state status transitions | clock skew between observed_at and now | N/A | timestamp is informational | no |
| .test.spec covers validation | covers unit deleted after test authored | yes (integration) | SPEC_MOLECULE_COVERS_NOT_FOUND | no |
| graph.impact() | cycle in dep graph (already caught by validate) | yes (cycle tests) | bail before graph build | no |
| Cross-library dep resolution | [libraries] path not found on disk | partial | needs explicit test | **critical gap** |
| Plan artifact impact computation | graph built from stale spec files | no caching | always rebuilds | no |

**Critical gap:** M9 needs an explicit test that `spec validate` fails with a clear error
when a `[libraries]` path does not exist on disk.

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
| Graph-like edges | `export.rs:GraphEdge` (promote to SpecGraph in M8) |
| Error code registry | `commands.rs:spec_error_code` |
| JSON fixture tests | `spec-cli/tests/fixtures/` |

---

## Worktree Parallelization

M6a is a single workstream — all changes are tightly coupled (output path → module prefix
→ test evidence → fixture regeneration). Sequential implementation.

M6b (health model) can run in parallel with the structural PR since they touch different
surfaces (status state machine vs. commands module split).

M7 is sequential: loader → schema → validator → generator → export → tests.

M8, M9, M10 are each sequential within themselves. M9 blocks on M8. M10 blocks on M8.
M9 and M10 can run in parallel worktrees once M8 is merged.

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
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 1 | issues_found | M6 over-bundled, evidence retention vs. truthful pipeline tension, sequencing .test.spec before graph, 6-state status mixes dimensions — all resolved |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | **CLEAR (PLAN)** | 6 issues found, 0 unresolved, 1 critical gap (M9 missing-library test) |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 0 | — | — |

**CODEX:** 6 tensions raised: (1) evidence retention vs. truthful pipeline → resolved by preserve+stale detection together; (2) M6 over-bundled → split into M6a/structural PR/M6b; (3) 6-state status mixes dimensions → explicit precedence rules defined; (4) module prefix config as band-aid → resolved by auto-derivation from crate root; (5) .test.spec before graph layer → resolved by minimal graph in M7 + full graph in M8; (6) declared covers edges can lie → documented as programmer claim, same status as deps.
**UNRESOLVED:** 0
**VERDICT:** ENG CLEARED — implement in order: M6a → structural PR → M6b → M7 → M8 → M9/M10 parallel.
