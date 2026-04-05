<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260404-165359.md -->
# Release 0.4: Pipeline, Evidence & Exports

**Generated**: 2026-04-04  
**Status**: Planning  
**Preceded by**: `.implemented/PLAN-M2-release-0.2.md` (also see: gstack M3 design doc)  
**Roadmap reference**: Release 0.4 — Broader Verification & Exports  

---

## Thesis

M3 proved the semantic unit can be authored, validated, and lowered into correct, compilable Rust.
`body.rust` is a block. The fn signature comes from the contract. Passports capture what was
declared. The pipeline is honest.

M4 closes the feedback loop. spec wraps the build and test commands, collects runtime evidence,
and exports machine-readable artifacts. When M4 ships, every unit will have a passport that
distinguishes what was declared from what was observed to pass.

The bar for calling M4 done:

- `spec build` runs validate → generate → cargo build and surfaces cargo errors through spec
- `spec test` runs spec build → cargo test and updates passports with observed test results
- Passports v2 distinguish "declared" (local_tests) from "observed" (test results from last run)
- `spec export` emits a JSON bundle: units, passports, dependency graph
- Generated Rust files include `///` doc comments from each unit's `intent` field
- Carryover debt from M2/M3 retrospective is cleared
- Cross-library dep schema is decided (design record, no code)

---

## Deliverables (sequenced)

### D1 — Pipeline wrap (`spec build` / `spec test`)

M3 design doc explicitly defers this: "spec generates, user runs cargo. Pipeline wrap is a
config/flag lever for M4." The carryover is now unblocked — passports v1 exist, all M3 work
is done.

**Design:**

```
spec build <path> [--output <dir>]
  = validate_full(path)
  → generate_command(path, output)
  → cargo build (in the consuming crate that includes generated output)
  → exit 0 if all pass, exit 1 on first failure

spec test <path> [--output <dir>]
  = spec build <path>
  → cargo test (in the same consuming crate)
  → collect test results
  → update passports with runtime evidence  ← ALWAYS, regardless of cargo exit code
  → propagate cargo's exit code (0 = all pass, 1 = any failure)
```

**CRITICAL: spec test evidence write order.** Evidence is written to passports BEFORE spec test exits. If cargo test returns non-zero (test failures), spec test still writes fail evidence to the relevant passports, then exits 1. The write-then-exit order ensures evidence reflects the actual run, including failures. Failing to write on non-zero exit would silently leave passports with stale "last-passing" evidence.

**Cargo execution:**
- Uses existing `cargo_available()` guard pattern (from cli.rs integration tests)
- Discovers cargo project root from `--crate-root <path>` flag or config `[pipeline] crate_root`
- Default if unconfigured: the nearest ancestor of `<path>` that contains a `Cargo.toml`
- Isolated `CARGO_TARGET_DIR`: use `spec.toml [pipeline] cargo_target_dir` or system tempdir
- Captures cargo stdout/stderr and forwards to spec's stdout/stderr
- On cargo failure: print cargo's output verbatim, exit 1 with `❌ cargo build failed`
- **Path scope guard**: `spec build` and `spec test` require a directory path. If a single-file path is given, exit 1 with `❌ spec build requires a directory path — pass the units directory, not a single file`
- **Progress signal**: emit `spec: running cargo build in <crate_root>` to stderr before invoking the subprocess. This makes cargo hang visible (user sees the last spec output). No timeout in M4 — SIGINT propagates to cargo.

**Config (`spec.toml`):**
```toml
[pipeline]
crate_root = "examples/ecommerce"       # optional: default = nearest Cargo.toml ancestor
cargo_target_dir = "/tmp/spec-target"  # optional: default = $CARGO_TARGET_DIR or tempdir
```

**Files:**
- `spec-cli/src/commands.rs`: add `Build` and `Test` variants to `Command` enum
- `spec-core/src/pipeline.rs`: new module — `run_cargo_build()`, `run_cargo_test()`, `CargoResult`
- `spec-core/src/lib.rs`: expose `pipeline` module
- `spec-core/src/config.rs` (or `spec-cli/src/config.rs`): add `[pipeline]` section to `SpecConfig`

Acceptance:
- `spec build examples/ecommerce/units --output examples/ecommerce/src/generated` validates, generates, and runs `cargo build` on the ecommerce crate
- `spec test examples/ecommerce/units --output examples/ecommerce/src/generated` also runs `cargo test` and captures results
- Failing a spec validation before cargo is reached: exit 1, no cargo invoked
- cargo not in PATH: exit 1 with `❌ cargo not found — install Rust or ensure cargo is on PATH`
- Non-zero cargo exit: spec exits 1 and forwards cargo's stderr
- Single-file path given: exit 1 with `❌ spec build requires a directory path — pass the units directory, not a single file`
- No Cargo.toml ancestor and no `--crate-root`: exit 1 with `❌ could not find crate root — run from inside a Cargo project, or pass --crate-root <path>`

Tests to add:
- `spec_build_validates_and_runs_cargo_build` (CLI integration test)
- `spec_build_fails_on_validation_error_before_cargo` (CLI integration test)
- `spec_build_unavailable_cargo_exits_cleanly` (CLI integration test — gate test behind cargo_available())
- `spec_test_runs_cargo_test` (CLI integration test)
- `spec_test_forwards_cargo_stderr_on_failure` (CLI integration test)
- `spec_build_rejects_single_file_path` (CLI integration test)
- `spec_test_no_local_tests_produces_empty_evidence` (CLI integration test — unit with no local_tests → test_results: [])

---

### D2 — Runtime evidence in passports (passports v2)

After `spec test` runs cargo test, update the passport with observed results. Passports become
the canonical bridge between declared tests and observed passes.

**Evidence schema addition (passport.schema.json):**
```json
"evidence": {
  "build_status": "pass",
  "last_built_at": "2026-04-04T17:00:00Z",
  "test_results": [
    { "id": "happy_path", "status": "pass" }
  ],
  "observed_at": "2026-04-04T17:00:00Z"
}
```

- `evidence` is optional — absent if `spec generate` was used (not `spec test`)
- `build_status`: `"pass"` | `"fail"` | `"unknown"` (unknown = cargo not available or skipped)
- `test_results[].status`: `"pass"` | `"fail"` | `"error"`
- Parse cargo test output (line-by-line `test X ... ok|FAILED` format, `--test-output immediate` mode)
- Match cargo test names back to local_tests ids by the `test_{id}` naming convention

**Files:**
- `spec-core/src/passport.rs`: add `PassportEvidence` struct, update `Passport` with `evidence: Option<PassportEvidence>`
- `spec-cli/src/commands.rs`: in `test_command()`, after cargo test completes, build evidence and write passports
- `spec-core/src/pipeline.rs`: add `parse_cargo_test_output()` → returns `Vec<TestResult>`

Acceptance:
- After `spec test`, passport files contain an `evidence` field with build_status and test_results
- After `spec generate` (not test), passports have no `evidence` field (no regression on static passports)
- A failing test in ecommerce produces `"status": "fail"` in the matching test_result
- `spec test` on a clean ecommerce example → all test_results are `"pass"`

Tests to add:
- `spec_test_writes_evidence_to_passport` (CLI integration test)
- `spec_generate_passport_has_no_evidence` (unit test in passport.rs — regression)
- `parse_cargo_test_output_parses_pass_and_fail` (unit test in pipeline.rs)
- `spec_test_failure_writes_fail_status_to_passport` (CLI integration test)

---

### D3 — JSON export v1 (`spec export`)

Emit a machine-readable bundle from the loaded spec set and any co-located passports.
Makes spec artifacts consumable by other tools without re-parsing raw source files.

**Command:**
```
spec export <path> [--output <file>]
```
Output defaults to stdout (piped to jq, tools). If `--output <file>` is given, writes to file.

**Bundle schema:**
```json
{
  "spec_version": "0.4.0",
  "exported_at": "2026-04-04T17:00:00Z",
  "units": [
    {
      "id": "pricing/apply_tax",
      "intent": "...",
      "contract": { "inputs": [...], "returns": "..." },
      "deps": [...],
      "local_tests": [...],
      "source_file": "..."
    }
  ],
  "passports": [...],
  "graph": {
    "edges": [
      { "from": "pricing/apply_tax", "to": "money/round" }
    ]
  }
}
```

- Loads passports from disk (co-located `.spec.passport.json`) if they exist
- Graph is the dependency edge list derived from the loaded spec set
- If no passports exist (e.g. generate not run yet), `passports` is an empty array
- `serde_json` is already a dependency

**Files:**
- `spec-core/src/export.rs`: new module — `ExportBundle`, `build_export_bundle()`, `load_passports_for_specs()`
- `spec-core/src/lib.rs`: expose `export` module
- `spec-cli/src/commands.rs`: add `Export` variant to `Command` enum with `export_command()`

Acceptance:
- `spec export examples/ecommerce/units | jq '.units | length'` returns 4 (ecommerce has 4 units)
- `spec export examples/ecommerce/units --output bundle.json` writes valid JSON to file
- Bundle includes graph edges for apply_discount → money/round, apply_tax → money/round, etc.
- If passports exist: bundle includes them. If not: `passports: []`
- Invalid spec path: exit 1 with clear error (same pattern as validate/generate)
- `--output <dir>` (directory given): exit 1 with `❌ --output must be a file path, not a directory`
- `--output <file>` where parent dir does not exist: exit 1 with `❌ output directory does not exist: <parent>`
- Empty spec dir (no .unit.spec files): valid JSON with `units: [], passports: [], graph: {edges: []}`

Tests to add:
- `spec_export_emits_valid_json_bundle` (CLI integration test)
- `spec_export_includes_graph_edges` (CLI integration test)
- `spec_export_includes_passports_if_present` (CLI integration test)
- `build_export_bundle_graph_edges_correct` (unit test in export.rs)
- `spec_export_output_path_rejects_directory` (CLI integration test)
- `spec_export_output_parent_dir_missing_exits_cleanly` (CLI integration test)
- `spec_export_empty_directory_emits_valid_empty_bundle` (CLI integration test)

---

### D4 — Doc comments in generated Rust (`intent` → `///`)

When `spec generate` produces a `.rs` file, prefix the generated `pub fn` with a `///` doc
comment from the unit's `intent` field. Low effort, immediate value: generated code becomes
self-documenting.

**Generated output example:**

```rust
use rust_decimal::Decimal;

use crate::money::round::round;

/// Add sales tax to a subtotal using a rate expressed as a decimal fraction.
pub fn apply_tax(subtotal: Decimal, rate: Decimal) -> Decimal {
    let taxed = subtotal + subtotal * rate;
    round(taxed)
}
```

- No `intent` → no doc comment (no regression for units without intent)
- Multi-line intent: each line prefixed with `/// `
- Escaping: `intent` content is not Rust code — no escaping needed, doc comments are plain text

**Files:**
- `spec-core/src/generator.rs`: update `generate_code()` to prepend `/// {intent}\n` before the fn

Acceptance:
- Generated `.rs` for a unit with `intent` has a `///` doc comment above the function
- Generated `.rs` for a unit without `intent` has no doc comment (no regression)
- Multi-line intent produces multi-line `///` comments
- `cargo doc` on the ecommerce crate succeeds with doc comments present

Tests to add:
- `generate_code_includes_doc_comment_from_intent` (unit test in generator.rs)
- `generate_code_no_doc_comment_when_intent_absent` (unit test in generator.rs)
- `generate_code_multiline_intent_produces_multiline_doc_comment` (unit test in generator.rs)

---

### D5 — Carryover debt (M2/M3 retrospective items)

Three open items from the autoplan retrospective that belong in M4:

**D5a — Defense-in-depth: validate `local_tests[].expect` at the `generate_code` sink**

`generate_code` in `spec-core/src/generator.rs` is a public library function. It embeds
`local_test.expect` verbatim (as `assert!(expr)`) with no validation. A direct API caller
constructing a `ResolvedSpec` manually bypasses all expression validation.

**Decision: approach (c) from the retrospective — emit the assert!() from the validated
`syn::Expr` AST instead of the raw string.**

Implementation:
- Change `local_test.expect` field in `ResolvedSpec` from `String` to `ValidatedExpr` newtype
- `ValidatedExpr` wraps a `syn::Expr` parsed and validated at the validator stage
- `generate_code` uses `ValidatedExpr` to emit via `quote::quote!` or `prettyplease::unparse`
- Add `quote` and `prettyplease` to spec-core Cargo.toml (both are lightweight, no build script)
- OR: keep expect as String but parse in generate_code and fail if invalid — simpler, less correct

Simpler alternative: keep String but call `syn::parse_str::<syn::Expr>` inside `generate_code()`,
return `SpecError::Generator` if invalid. This is one guard, not a newtype refactor.

**CONFIRMED DECISION**: Use the simpler alternative for M4 (inline guard in generate_code). Newtype refactor deferred to M5 when the API surface grows.

**CRITICAL IMPLEMENTATION NOTE**: Do NOT call raw `syn::parse_str::<syn::Expr>` in `generate_code()`. The depth-check helper must be called at this sink too. Raw syn::parse_str overflows its own call stack on deeply nested expressions (200+ levels) — this was already fixed in the validator path but the generate_code sink is a separate call site.

**Boundary decision (Codex tension resolved):** Do NOT make `is_safe_expect_expr_depth()` pub on `validator.rs` — that leaks an internal validator helper into the public API. Instead, move the depth-check function to a new `spec-core/src/syntax.rs` shared module. Both `validator.rs` and `generator.rs` import from `syntax.rs`. This is the correct boundary.

Files: `spec-core/src/generator.rs`, `spec-core/src/syntax.rs` (new shared module), `spec-core/src/validator.rs` (update import), `spec-core/src/lib.rs` (expose syntax module)

Tests to add:
- `generate_code_rejects_unsafe_expect_at_sink` (unit test in generator.rs — direct API call with unsafe expect, no prior validation)
- `generate_code_rejects_deeply_nested_expect_at_sink` (unit test in generator.rs — 200+ nesting levels, must not panic)

**D5b — Document `pub use generated::*` convention**

Internal deps generate `use crate::X` paths. These only resolve if the consuming crate's root
re-exports generated modules (e.g. `pub use generated::*;` in `main.rs` or `lib.rs`).
This requirement is implicit and not documented.

Action: add a `## Consuming Generated Code` section to README.md explaining the convention,
with a code example showing the `pub use generated::*;` pattern.

Files: `README.md`

**D5c — Close the "commit vs ephemeral" open decision**

This was flagged in TODOS.md since M3 as an open prerequisite. D1 (`spec build`) implicitly
resolves it — generated output is ephemeral, regenerated on each `spec build` invocation, not
committed to git. Add a DECISIONS.md entry to close this explicitly.

```markdown
## Generated Output: Ephemeral by Default (0.4.0 decision record)

`spec build` and `spec test` treat generated output as ephemeral — generated on each run into
the `--output` directory, consumed by cargo, not committed to git. The output dir is fully
spec-owned (existing `.spec-generated` marker convention applies).

If you want committed generated output (for diffing in CI, or IDE discoverability), use
`spec generate` and commit manually. `spec build`/`spec test` are optimized for the ephemeral
case. A `--no-regen` flag or equivalent "committed mode" is deferred to M5.
```

Files: `DECISIONS.md`

---

### D6 — Cross-library dep schema design decision

Cycle detection in M3 is in-tree only. Cross-library dep units (`money/round` from a different
spec library) are not loaded in the same spec set. This limitation is documented in the M3
error output. Before M5 (plan-aware workflow), the schema must be decided and written down, but
M4 still ships no runtime behavior change for cross-library deps.

**Chosen schema** (design only, no code in M4):

- Local dep: `money/round`
- Cross-library dep: `shared::money/round`

Future config contract:

```toml
[libraries]
shared = "../shared-spec"
```

`shared` is a namespace alias defined by the consuming workspace. The mapped path points at the
root of another spec library. Version pins and registry/org-qualified paths are explicitly
deferred.

Output: add a `DECISIONS.md` record with the tradeoff matrix, chosen syntax, invalid examples, and
an explicit note that M5 owns resolution, validation, use-path generation, and cross-library cycle
detection.

---

### D7 — Version bump and changelog note

Bump workspace version to `0.4.0` in root `Cargo.toml`. Add a CHANGELOG entry.

Breaking changes in 0.4.0:
- Passport schema v2: adds optional `evidence` field. Parsers of passport JSON should
  tolerate unknown fields and treat absent `evidence` as "no runtime evidence available."

Non-breaking:
- New commands: `spec build`, `spec test`, `spec export`
- Generated `.rs` files gain `///` doc comments (no compile impact)

---

## What is NOT in scope

Hold these for M5 or later:

- Molecule/organism test support (`.test.spec` — requires multi-unit test assembly)
- Docs generation as a separate output format (`.md` docs for each unit) — M4.x
- Impact graph and dependency inspection views — M5
- Plan artifact schema and plan-aware workflow — M5
- Second target language — M6
- Cross-library dep IMPLEMENTATION (D6 decides the schema; M5 builds it)
- Reverse ingestion — M6
- ICP definition (one-paragraph DECISIONS.md entry for "who is the v0.x user") — deferred to M5 scoping

---

## Sequencing rationale

```
D5 (carryover debt) → defense-in-depth + pub use convention doc, no code dependencies
D6 (cross-lib dep schema) → design only, no dependencies, but resolve before 0.5
D4 (doc comments) → independent, can land anytime, no schema changes
D1 (pipeline wrap) → independent of passport changes; requires cargo path discovery design
D2 (evidence passports) → depends on D1 (cargo test output available)
D3 (JSON export) → depends on D2 (passports with evidence make export more useful, but can ship without)
D7 (version bump + changelog) → last
```

Parallel tracks:
- **Track A:** D1 → D2 → D3 (sequential: pipeline → evidence → export)
- **Track B:** D4, D5, D6 (anytime, independent)

---

## Success criteria (definition of done)

| Check | Pass condition |
|-------|---------------|
| Pipeline build | `spec build examples/ecommerce/units --output examples/ecommerce/src/generated` runs and passes |
| Pipeline test | `spec test examples/ecommerce/units --output examples/ecommerce/src/generated` runs all cargo tests |
| Evidence passports | After spec test, passport for apply_tax has `evidence.test_results` with pass/fail per local_test |
| Static passports unchanged | `spec generate` still works and produces passports without `evidence` field |
| JSON export | `spec export examples/ecommerce/units` emits valid JSON with 4 units and correct graph edges |
| Doc comments | Generated apply_tax.rs has `/// Add sales tax...` before `pub fn apply_tax` |
| Library sink guard | Direct call to `generate_code()` with unsafe expect fails at the function boundary |
| Cross-lib schema | DECISIONS.md has a cross-library dep schema decision record |

---

## Test gaps (to be added during implementation)

### From autoplan review (26 original)
- `spec_build_validates_and_runs_cargo_build` (D1, spec-cli/tests/cli.rs)
- `spec_build_fails_on_validation_error_before_cargo` (D1, spec-cli/tests/cli.rs)
- `spec_build_unavailable_cargo_exits_cleanly` (D1, spec-cli/tests/cli.rs)
- `spec_test_runs_cargo_test` (D1, spec-cli/tests/cli.rs)
- `spec_test_forwards_cargo_stderr_on_failure` (D1, spec-cli/tests/cli.rs)
- `spec_test_writes_evidence_to_passport` (D2, spec-cli/tests/cli.rs)
- `spec_generate_passport_has_no_evidence` (D2, spec-core/src/passport.rs)
- `parse_cargo_test_output_parses_pass_and_fail` (D2, spec-core/src/pipeline.rs)
- `spec_test_failure_writes_fail_status_to_passport` (D2, spec-cli/tests/cli.rs)
- `spec_export_emits_valid_json_bundle` (D3, spec-cli/tests/cli.rs)
- `spec_export_includes_graph_edges` (D3, spec-cli/tests/cli.rs)
- `spec_export_includes_passports_if_present` (D3, spec-cli/tests/cli.rs)
- `build_export_bundle_graph_edges_correct` (D3, spec-core/src/export.rs)
- `generate_code_includes_doc_comment_from_intent` (D4, spec-core/src/generator.rs)
- `generate_code_no_doc_comment_when_intent_absent` (D4, spec-core/src/generator.rs)
- `generate_code_multiline_intent_produces_multiline_doc_comment` (D4, spec-core/src/generator.rs)
- `generate_code_rejects_unsafe_expect_at_sink` (D5a, spec-core/src/generator.rs)
- `spec_build_crate_root_workspace_resolution` (D1, spec-cli/tests/cli.rs)
- `spec_build_crate_root_config_vs_flag_precedence` (D1, spec-cli/tests/cli.rs)
- `spec_build_no_cargo_toml_exits_with_error` (D1, spec-cli/tests/cli.rs)
- `parse_cargo_test_output_ignores_non_test_lines` (D2, spec-core/src/pipeline.rs)
- `parse_cargo_test_output_handles_duplicate_test_ids_across_units` (D2, spec-core/src/pipeline.rs)
- `spec_export_partial_passports_marked_missing` (D3, spec-cli/tests/cli.rs)
- `spec_export_output_path_rejects_directory` (D3, spec-cli/tests/cli.rs)
- `spec_export_schema_version_separate_from_spec_version` (D3, spec-core/src/export.rs)
- `spec_test_writes_evidence_atomically` (D2, spec-cli/tests/cli.rs)

### Added by /plan-ceo-review 2026-04-05 (4 new)
- `generate_code_rejects_deeply_nested_expect_at_sink` (D5a, spec-core/src/generator.rs — 200+ nesting levels, must not panic)
- `spec_export_output_parent_dir_missing_exits_cleanly` (D3, spec-cli/tests/cli.rs)
- `spec_build_rejects_single_file_path` (D1, spec-cli/tests/cli.rs)
- `spec_test_no_local_tests_produces_empty_evidence` (D2, spec-cli/tests/cli.rs)
- `spec_export_empty_directory_emits_valid_empty_bundle` (D3, spec-cli/tests/cli.rs)

### Added by /plan-eng-review 2026-04-05 (8 new)
- `spec_build_bare_crate_no_workspace_uses_package_toml` (D1, spec-cli/tests/cli.rs — bare crate fallback in workspace_root_for())
- `spec_export_malformed_passport_json_produces_warning_not_crash` (D3, spec-core/src/export.rs — truncated JSON → warning + skip, not panic)
- `spec_build_prints_crate_root_to_stderr` (D1, spec-cli/tests/cli.rs — decision #18 DX requirement)
- `generate_code_sink_guard_includes_unit_and_test_id_in_error` (D5a, spec-core/src/generator.rs — error context quality)
- `spec_test_build_failure_writes_fail_build_status_to_passport` (D2, spec-cli/tests/cli.rs — cargo build fails before cargo test runs)
- `spec_test_evidence_matches_non_default_output_module_name` (D2, spec-cli/tests/cli.rs — module prefix derived from --output path, not hardcoded "generated")
- `generate_code_includes_intent_as_doc_comment` (D4, spec-core/src/generator.rs — ResolvedSpec.intent field is used)
- `spec_build_stops_at_nearest_package_toml_not_workspace_root` (D1, spec-cli/tests/cli.rs — two-step walk correctness)

**Total: 39 test gaps**

---

## Worktree parallelization

| Step | Modules touched | Depends on |
|------|----------------|------------|
| D1 | spec-cli/commands, spec-core/pipeline (new), config | — |
| D2 | spec-core/passport, spec-cli/commands | D1 |
| D3 | spec-core/export (new), spec-cli/commands | D2 (passports with evidence) |
| D4 | spec-core/generator, spec-core/types | — |
| D5 | spec-core/syntax (new), spec-core/generator, spec-core/validator, spec-core/lib, DECISIONS.md, README.md | — (D5 internal order: syntax.rs → validator.rs import → generator.rs guard) |
| D6 | DECISIONS.md | — |
| D7 | Cargo.toml, CHANGELOG | D1, D2, D3, D4 |

**Lane A:** D1 → D2 → D3 (sequential pipeline)
**Lane B:** D4, D5, D6 (all independent, can run in parallel with Lane A)

---

## Failure modes

| Codepath | Failure scenario | Test? | Error handling? | Silent? |
|---------|-----------------|-------|----------------|---------|
| spec build: cargo discovery | Cargo.toml not found in ancestors | No | Needs error | Yes — would attempt cargo in cwd |
| spec build: cargo subprocess | cargo not in PATH | No | Needs check (existing pattern) | Yes |
| spec build: cargo subprocess | cargo hangs (deadlocked build script) | Manual only | Document: print "spec: running cargo build..." to stderr first; SIGINT propagates | Partially — user sees last spec output |
| spec build: path scope | Single-file path given instead of directory | No | Needs error: ❌ spec build requires a directory path | Yes |
| spec test: cargo test parse | Unexpected test output format | No | Needs fallback | Yes — evidence silently wrong |
| spec test: evidence | Unit has zero local_tests → test_results: [] | No | Correct behavior, untested | No |
| spec export: passport load | Passport JSON malformed / truncated | No | Needs graceful skip | Yes |
| spec export: --output path | Parent directory does not exist | No | Needs check: path.parent().exists() | Yes — confusing OS error |
| D5a sink guard | syn::parse_str failure on expect | No | Needs SpecError::Generator with unit+test ID context | Yes |
| D5a sink guard | Deeply nested expect (200+ levels) → syn stack overflow | No | Must call is_safe_expect_expr_depth() — NOT raw syn::parse_str | Yes |
| D2 evidence write | Passport write fails partway through | No | Needs atomic write (tempfile + rename) | Yes — partial evidence written |

**Critical gaps:**
1. **Cargo.toml discovery** — if `--crate-root` is not specified and there's no Cargo.toml ancestor, spec build would either fail confusingly or run cargo in an arbitrary directory. Need: explicit discovery with clear error if not found.
2. **Cargo test output parse** — cargo's test output format is not a documented API. `test X ... ok` is stable but output in verbose mode differs. Need: test the parser against real cargo test output, not a mock.
3. **D5a syn overflow at sink** — `generate_code()` must call `is_safe_expect_expr_depth()` (from validator.rs, must be made `pub`) instead of raw `syn::parse_str::<syn::Expr>` for the D5a inline guard. Raw syn::parse_str overflows its own call stack on 200+ levels of nesting. The existing validator cap does NOT protect the generate_code() call site.
4. **spec build single-file scope** — scope `spec build` and `spec test` to directory paths only. Add explicit error: `❌ spec build requires a directory path, not a single file`.
5. **--output parent dir missing** — `spec export --output <file>` must check `path.parent().exists()` before calling serde_json write. Emit: `❌ output directory does not exist: <parent>`.


---

## /autoplan Review — 2026-04-04 (M4 Plan Draft Review)

**Context:** New M4 plan drafted inline by /autoplan. M2+M3 fully shipped.
**Mode:** SELECTIVE EXPANSION (new plan on proven system, concrete deliverables)
**UI Scope:** None. DX Scope: Yes (this is a developer CLI tool).

### Phase 1: CEO Review + Dual Voices

**CLAUDE SUBAGENT (CEO — strategic independence):**
1. D2 cargo test parser is unvalidated against real cargo output — evidence model is unreliable until this is tested with real corpus (High)
2. ICP undefined; D3 export schema and D6 cross-lib schema shape depend on it (High)
3. Generated output ephemeral vs. committed decision still unresolved — D1 (spec build) implicitly assumes one answer (Medium)
4. D1 framed as "convenience wrapper" — it is the evidence enforcement layer; reframe in thesis (Medium)
5. D5a auto-decision parenthetical not formally confirmed — close it with a DECISIONS.md note (Medium)
6. D6 schema recommendation (namespace prefix) lacks comparative tradeoff analysis (Medium)
7. D4 doc comments too small to be a peer deliverable at same level as D1-D3 (Low)

**CODEX SAYS (CEO — strategy challenge):**
1. No buyer defined — ICP deferred while D3/D6 depend on it is a strategic contradiction (Critical)
2. Evidence model overstated — no commit SHA, env fingerprint, runner identity; calling it "canonical" creates false trust (High)
3. Cross-library reuse is the core product problem being punted — single-tree pipeline polish is "local CLI varnish" in 6 months (High)
4. spec export has no named downstream consumer — schema-first without a consumer = dead artifact (High)
5. spec build/spec test is potentially the wrong wedge — cargo orchestration is easy to imitate; the moat is CI provenance and distribution semantics (Medium)
6. Scope discipline weak — doc comments in same release as unresolved customer definition (Medium)
7. Validation bar is toy-only — all acceptance criteria based on ecommerce example (Medium)
8. Plan internally inconsistent on ICP — sequencing rationale mentioned it, D5 section omits it, success criteria referenced it (now fixed) (Medium — fixed by auto-decision #4)

```
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   Mostly  Partial PARTIAL (evidence quality underspecified; ICP deferred)
  2. Right problem to solve?           Yes     Partial DISAGREE (Codex: pipeline is wrong wedge; user confirmed scope)
  3. Scope calibration correct?        Yes     Partial PARTIAL (D4 small; doc/README in big release)
  4. Alternatives sufficiently explored? Partial No   DISAGREE (D6 analysis thin; Codex: cross-lib first)
  5. Competitive/market risks covered? Partial No     CONFIRMED gap (no named consumer for export; no buyer defined)
  6. 6-month trajectory sound?         Mostly  Partial PARTIAL (local evidence ≠ canonical; provenance unspecified)
═══════════════════════════════════════════════════════════════
CONFIRMED = both agree. DISAGREE = models differ (→ taste decision or user challenge).
User confirmed scope at premise gate — D1 wedge disagreement becomes TASTE DECISION, not USER CHALLENGE.
```

### CEO Review Sections

**Step 0A — Premise Challenge:**
7 premises evaluated. 4 confirmed solid (M3 shipped, pipeline deferred, evidence requires pipeline, doc comments high-value). 2 partially confirmed (D3 export shape reasonable but needs consumer; D6 schema defensible but needs analysis). 1 removed (ICP — user deferred to M5). Auto-decisions applied.

**Step 0B — Existing Code Leverage:**
- D1 pipeline → `cargo_available()` + `run_cargo()` at cli.rs:875-893 — exact reuse pattern
- D2 evidence → extends `Passport` struct at passport.rs:44 — additive struct field
- D3 export → `LoadedSpec`, `ResolvedSpec` types in types.rs — zero new input types
- D4 doc comments → `generate_code()` in generator.rs — 1-line addition
- D5a sink guard → same `generate_code()` — inline parse, no new abstraction
No DRY violations found. No parallel reconstruction. Lean plan.

**Step 0C — Dream State:**
```
  CURRENT (post-M3)              THIS PLAN (M4)            12-MONTH IDEAL
  ─────────────────────          ──────────────────────    ──────────────────────────
  validate + generate            spec build / spec test    Plan-aware workflow (M5)
  Static passports only          Runtime evidence passports Cross-team governance
  No machine-readable export     JSON bundle export         AI agent consumption
  No doc comments                /// intent comments        Multi-language support
  Cross-lib schema undecided     Cross-lib schema decided   Cross-lib dep IMPL (M5)
  Generated output: ambiguous    Commitment decision needed  CI provenance model
```
Gap: evidence provenance (commit SHA, env) not in M4 passport schema. Both models flag this independently. Tracked as a test gap and future TODOS.md item.

**Step 0C-bis — Implementation Alternatives:**
```
APPROACH A: New top-level commands (spec build / spec test) — current plan
  Effort: S | Risk: Low
  Pros: Distinct concerns, discoverable, testable independently
  Cons: Pipeline is easy to imitate; moat needs to be in governance, not orchestration
  Completeness: 8/10

APPROACH B: Flags on generate (--build, --test)
  Effort: XS | Risk: Low
  Pros: Less surface area
  Cons: Conflates lowering with building — wrong coupling
  Completeness: 6/10 (less correct conceptually)

APPROACH C: Config-only auto-build (spec.toml [pipeline] auto_build = true)
  Effort: XS | Risk: Medium
  Pros: Opt-in, no new commands
  Cons: Inflexible, harder to test, no explicit spec test command
  Completeness: 5/10
```
AUTO-DECISION: A is correct (P5 explicit, P1 completeness). Codex's moat concern is noted and valid for M5+ design, not an M4 scope change. Decision #5 logged.

**Step 0D — Mode: SELECTIVE EXPANSION confirmed.** Treat each scope challenge as an individual decision. No silent expansion or reduction.

**Step 0E — Temporal interrogation:**
- HOUR 1 (D4, D5, D6): doc comments, README, cross-lib design — unblocked, independent
- HOUR 2-4 (D1): pipeline wrap commands — cargo subprocess, config, error handling
- HOUR 5-6 (D2): evidence — cargo test text parsing, passport update
- HOUR 7 (D3): JSON export — new module, export command
- HOUR 8 (D7): version bump, changelog

**Step 0F:** Mode confirmed SELECTIVE EXPANSION.

**Section 1 — Architecture (post-M4 addition):**
```
  CURRENT (post-M3):                M4 ADDS:
  
  .unit.spec → validate → normalize → generate → passport (static)
                                         │
                                         └──▶ [M4] pipeline.rs
                                                   │
                                              spec build ──▶ cargo build
                                              spec test  ──▶ cargo test
                                                                  │
                                                         parse test output
                                                                  │
                                                         passport (+ evidence)
                                                         
  [M4] export.rs: loads specs + passports → JSON bundle
  [M4] generator.rs: prepend /// intent to each pub fn
```
New coupling: `spec test` creates a coupling between the spec validation/gen path and the cargo test result. This is intentional. Single point of failure: if cargo test output format changes, evidence collection silently degrades. Must be documented.

**Section 2 — Error & Rescue Map (M4 additions):**
```
  CODEPATH                     | WHAT CAN GO WRONG          | HANDLED?
  -----------------------------|----------------------------|----------
  spec build: crate discovery  | No Cargo.toml ancestor     | NO — CRITICAL GAP
  spec build: cargo subprocess | cargo not in PATH          | Pattern exists (cargo_available)
  spec test: output parse      | Unknown output format      | NO — CRITICAL GAP (evidence silent)
  spec export: passport load   | Malformed JSON             | NO — needs graceful skip
  D5a: generate_code sink      | Unsafe expect at API level | Will be handled by inline syn parse
  D2 evidence write            | Partial write on interrupt | NO — needs atomic write pattern
```
Two critical gaps: crate discovery and cargo test output reliability. Both must be resolved in D1/D2 implementation.

**Section 3 — Security:**
- `spec build`/`spec test` run cargo, which runs the user's Rust code. This is expected and appropriate for a dev tool. No new privilege escalation surface.
- D5a (sink guard) improves security: `generate_code` public API now rejects unsafe expect strings
- D3 export: serializes user-authored content to JSON. Content is field values from .unit.spec — already validated at this point. Safe.
- No new auth surfaces, no new network requests, no new credential handling.
Examined: subprocess execution, export serialization, sink guard. No unaddressed security gaps found.

**Section 4 — Data Flow (M4 new flows):**
```
  spec build: .unit.spec → validate → generate → cargo build ← CARGO_TARGET_DIR
                                                      │
                                              [exit code + stderr]
                                                      ↓
                                            spec exit 0/1
                                            
  spec test: spec build + cargo test ──→ text output parse ──→ evidence struct ──→ passport write
  
  spec export: .unit.spec dir → validate + load → load_passports_for_specs → ExportBundle → JSON
```
Shadow paths: empty spec set → spec export emits `{"units": [], "passports": [], "graph": {"edges": []}}` (valid). Cargo not available → spec build/test exits 1 cleanly. No passports on disk → export emits `"passports": []`.

**Section 5 — Code Quality:**
- D1 adds new commands to the `Command` enum — 2 new variants, consistent with existing pattern
- `pipeline.rs` new module — keep it thin: `run_cargo_build()`, `run_cargo_test()`, `parse_test_output()`, `CargoResult`
- D3 `export.rs` new module — keep it thin: `ExportBundle`, `build_export_bundle()`, `load_passports_for_specs()`
- No DRY violations in the new modules design
- D4 doc comment: 1-line change in `generate_code()` — minimal diff, appropriate
- Naming: `spec build`, `spec test`, `spec export` follow existing `spec validate`, `spec generate` pattern

**Section 6 — Test Review:**
17 test gaps identified in draft plan. Critical additions from CEO review:
- `parse_cargo_test_output_against_real_corpus` — run parser against multiple real cargo test output formats (not just ecommerce mock). Must include: verbose mode, test binary names with path prefixes, multiple test binaries in workspace, FAILED with panic output.
- `spec_build_no_cargo_toml_exits_with_error` — crate discovery: no Cargo.toml in ancestors → clear error message, no crash
- `spec_test_writes_evidence_atomically` — evidence write must be all-or-nothing per unit

Total test gaps: 20.

**Section 7 — Performance:**
- D1/D2 cargo subprocess: inherently slow. No spec control over cargo build time. CARGO_TARGET_DIR isolation pattern already established.
- D3 export: loads all passports from disk + all specs. O(n) file reads. Acceptable.
- D4 doc comments: O(len(intent)) string allocation per unit. Negligible.

**Section 8 — Observability:**
Same model as M1-M3: all errors to stderr via `SpecError` types, exit codes meaningful. New for D1/D2: cargo subprocess output forwarded verbatim to spec's stdout/stderr. No structured logging needed (CLI, not a service).

**Section 9 — Deployment:**
Single binary. New commands appear in `spec --help`. CHANGELOG updated with new commands. Version bump to 0.4.0. Breaking change: passport schema v2 with optional `evidence` field (non-breaking to parsers that handle unknown fields).

**Section 10 — TODOS.md items from CEO review:**
1. Evidence provenance (M5): add commit SHA, runner ID, environment fingerprint to passport evidence schema — current passport writes local-machine-only evidence without provenance
2. cargo export named consumer (M4 implementation): document the intended consumer before D3 is implemented — what tool or workflow will read the bundle?
3. Cross-library dep schema analysis (D6): produce a written tradeoff matrix for the 3 candidates before committing to namespace prefix

**NOT in scope (CEO confirmed):**
- ICP definition — deferred to M5 scoping (user decision)
- Evidence provenance (commit SHA, CI fingerprint) — deferred to M5 passport schema v3
- Pipeline as moat vs. governance/distribution (Codex challenge) — noted as M5+ design direction, not M4 scope change
- Cross-library dep IMPLEMENTATION — M5

**What already exists (M4 leverage map):**
- `cargo_available()` at cli.rs:875 → reuse pattern in D1
- `run_cargo()` at cli.rs:886 → reuse in `pipeline.rs`
- `Passport` struct at passport.rs:44 → extend with `evidence: Option<PassportEvidence>`
- `LoadedSpec` / `ResolvedSpec` types → direct input to D3 export bundle
- `serde_json` already a dependency → no new crates needed for D3

**Dream State Delta:**
M4 leaves us at: pipeline wrap + runtime evidence + JSON export. Distance from 12-month ideal: need plan-aware workflow (M5), CI provenance (M5), cross-library dep implementation (M5), ICP definition (M5 prerequisite).

**CEO Phase Completion Summary:**
```
CEO REVIEW (autoplan 2026-04-04):
  Premises:        5/7 confirmed (2 auto-decisions applied: ICP deferred, scope confirmed)
  Architecture:    SOUND — lean new modules, consistent with existing patterns
  Security:        SOUND — no new attack surface; D5a improves security
  Test Coverage:   20 gaps identified (17 original + 3 from CEO review)
  Codex voice:     8 findings (1 critical ICP, 3 high, 4 medium)
  Claude voice:    7 findings (2 high, 4 medium, 1 low)
  Consensus:       1/6 confirmed, 2 partial, 2 disagree (taste decisions), 2 gap confirmed
  Auto-decisions:  6 (doc fixes, approach selection, ICP deferral confirmed)
  Taste decisions: 2 (D1 wedge, D6 schema analysis depth)
  User challenges: 0 (user confirmed scope at premise gate; ICP deferred by explicit user choice)
```

**PHASE 1 COMPLETE.** Codex: 8 findings. Claude subagent: 7 findings. Consensus: 1/6 confirmed, 4 partial/disagree. Taste decisions: 2. Passing to Phase 3 (skipping Phase 2 — no UI scope).

---


---

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Mode: SELECTIVE EXPANSION | Mechanical | P3 pragmatic | New plan on proven system; scope already confirmed by user | SCOPE EXPANSION |
| 2 | CEO | ICP deferral confirmed (user explicit decision at premise gate) | Mechanical | P3 pragmatic | User chose "Skip ICP for now"; ICP removed from D5 and success criteria | D5c addition |
| 3 | CEO | Fix plan consistency: sequencing rationale D5 + success criteria ICP refs | Mechanical | P5 explicit | Doc contradiction found by Codex: ICP mentioned in 3 places after user deferred it | None |
| 4 | CEO | D1 framing update: pipeline is evidence enforcement layer, not convenience | Mechanical | P5 explicit | Both models note D1 is undersold in thesis; no scope change, framing change only | None |
| 5 | CEO | D1 approach: new commands (spec build/test) vs flags vs config | Mechanical | P5 explicit | New commands are correct: distinct concerns, discoverable, independently testable | Flags, config-only |
| 6 | CEO | D5a approach: inline syn parse in generate_code vs ValidatedExpr newtype | Mechanical | P3 pragmatic | Simpler approach is appropriate for M4; newtype deferred to M5 when API surface grows | ValidatedExpr newtype |
| 7 | CEO | Evidence provenance (commit SHA, runner ID) → TODOS.md | Mechanical | P3 pragmatic | Both models flag this; provenance is M5+ when CI integration is designed | Cherry-pick |
| 8 | CEO | D1/D2 wedge concern (Codex) → TASTE DECISION at gate | Taste | P1/P6 conflict | Codex: pipeline is wrong wedge. User confirmed scope. Surface at final gate. | Accept current |
| 9 | CEO | D6 schema analysis depth → expand to 3-way comparison in D6 section | Mechanical | P1 completeness | Both models flag thin analysis; add tradeoff matrix before committing | None |
| 10 | Eng | D1 workspace root discovery: walk until [workspace] found, not first Cargo.toml | Mechanical | P1 completeness | Both models flag: member Cargo.toml found first in workspace = wrong crate | Accept member |
| 11 | Eng | D2 test matching: use module-qualified path, not bare test_id | Mechanical | P5 explicit | Bare test_{id} not unique across units; must qualify with generated module path | Bare test_id |
| 12 | Eng | D2 evidence schema: add parse_confidence field | Mechanical | P1 completeness | Codex: silent degradation architectural; need "unparsed/ambiguous" status | Omit field |
| 13 | Eng | D3 export schema: add schema_version (separate from spec_version) + warnings array | Mechanical | P5 explicit | Downstream consumers need schema version separate from tool version | Use spec_version |
| 14 | Eng | D3 --output: validate !path.is_dir() before write | Mechanical | P1 completeness | serde_json write to directory fails confusingly | None |
| 15 | Eng | move run_cargo() from test-only to pipeline.rs production code | Mechanical | P3 pragmatic | run_cargo() pattern is correct but test-only — must be production code for D1 | Keep in tests |
| 16 | Eng | D3 partial passports: emit passport_missing marker, not silent omission | Mechanical | P5 explicit | Claude subagent: silent mismatch in units vs passports arrays | Silent omission |
| 17 | DX | D5b scope expanded to include 5 README sections (quickstart, pipeline config, export schema, escape hatch, evidence) | Mechanical | P1 completeness | Both models: docs fail for new developers; 4 confirmed gaps | Minimal D5b |
| 18 | DX | D1: print resolved crate root to stderr | Mechanical | P5 explicit | Both models: silent auto-discovery creates confusion when wrong root chosen | Silent |
| 19 | DX | D1 acceptance: add missing crate-root error message | Mechanical | P1 completeness | Claude subagent: new user hits generic anyhow error; Codex confirms | Omit |
| 20 | DX | D2: unmatched test → emit "unknown" with reason, not silent omission | Mechanical | P5 explicit | Both models flag silent DX; "unknown" is honest; silent is misleading | Silent drop |
| 21 | DX | D7: CHANGELOG 0.3→0.4 migration notes (evidence is additive) | Mechanical | P1 completeness | Codex: upgrade path not fear-free; need explicit compatibility statement | None |
| 22 | DX | --output disambiguation in help text (not flag rename) | Mechanical | P5 explicit | Codex: --output means different things; help text fixes this cheaply | Flag rename |
| 23 | Eng | workspace_root_for() uses two-step walk: [workspace] first, else nearest [package] — handles workspace AND bare crates | Mechanical | P1 completeness | Bare crate has no [workspace]; algorithm must fall back to nearest [package] Cargo.toml | Walk fails on bare crates |
| 24 | Eng | D4: add intent: Option<String> to ResolvedSpec + update from_loaded() | Mechanical | P5 explicit | generate_code() receives ResolvedSpec which has no intent field; "1-line change" was wrong estimate (~15 lines, 2 files) | Add intent param to generate_code() |
| 25 | Eng | D2 build failure: write build_status:"fail" + test_results:[] on cargo build non-zero | Mechanical | P5 explicit | cargo build failure before cargo test → passports should reflect broken build state, not be left stale | Skip evidence write on build failure |
| 26 | Eng | D2 module prefix: derive from --output path last component, not hardcoded "generated" | Mechanical | P5 explicit | Consuming crate may mount output under any module name; prefix must match actual mount | Hardcode "generated" prefix |
| 27 | Eng | D5b: add nextest limitation note to README | Mechanical | P5 explicit | spec test parses standard cargo test output only; nextest format is different and unsupported | No documentation |


---

### Phase 3: Eng Review + Dual Voices

**CLAUDE SUBAGENT (Eng — independent review):**
1. D1 workspace Cargo.toml walk stops at first member, not workspace root — workspace builds run in wrong crate (High)
2. D2 multi-binary cargo test output not handled — parser doesn't account for workspace builds with multiple test binaries (High)
3. D3 partial passports silently omitted — new unit added since last generate = `units` and `passports` arrays silently mismatched (Medium)
4. D5a double-parse cost + opaque error at API boundary — generator re-parses what validator already parsed; error misses unit/test context (Medium)
5. Test plan gaps: workspace root test, partial passport test, non-test-line parse test (Medium)
6. D3 `--output` path not validated against directory — `serde_json` write to directory path fails confusingly (Low)

**CODEX SAYS (Eng — architecture challenge):**
1. D1 cargo-root discovery unresolved: `--crate-root` flag vs config vs ancestor walk are three different things and none are fully specified; `run_cargo()` is test-only code (High)
2. D2 test matching unreliable: `test_{id}` is not unique across units/binaries; evidence can silently assign to wrong passport (High)
3. D2 evidence schema has no uncertainty representation: no `unparsed`, `ambiguous_match`, or `expected N / observed M` — silent degradation is architectural (High)
4. D3 export schema underspecified: `spec_version` is tool version not schema version; no `warnings/skips` field; graph edges unstable until D6 resolved (High)
5. Test plan missing: workspace vs member manifests, `--crate-root` precedence, duplicate IDs across units, compile vs test failure evidence, malformed passport export (Medium)

```
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               Partial Partial CONFIRMED gap (D1 workspace; D2 matching; D3 schema)
  2. Test coverage sufficient?         Partial Partial CONFIRMED gap (24 gaps identified vs 17 original)
  3. Performance risks addressed?      Yes     Yes     CONFIRMED (O(n) scans, cargo target isolation established)
  4. Security threats covered?         Partial Partial CONFIRMED gap (D3 --output path; pipeline runs user cargo expected)
  5. Error paths handled?              Partial Partial CONFIRMED gap (D2 uncertainty, D3 malformed passports, D1 discovery)
  6. Deployment risk manageable?       Yes     Yes     CONFIRMED (single binary, opt-in new commands)
═══════════════════════════════════════════════════════════════
CONFIRMED = both agree. All 4 findings confirmed high-confidence.
```

**Section 1 — Architecture ASCII Diagram (post-M4):**
```
  .unit.spec files
       │
       ▼
  ┌──────────────┐   schema JSON  ┌─────────────────────────────────────────┐
  │  loader.rs   │───────────────▶│ validator.rs                            │
  │  (YAML parse)│                │  • JSON Schema + syn + dep strictness   │
  └──────────────┘                │  • cycle detection, contract types       │
       │                          └─────────────────────────────────────────┘
       ▼                                        │
  ┌──────────────┐                              ▼
  │normalizer.rs │    ┌───────────────────────────────────────────────────┐
  │ (dep lookup) │──▶ │ generator.rs                                       │
  └──────────────┘    │  • generate_code() → pub fn + /// intent + tests  │
                      │  • generate_mod_rs(), atomic write + orphan clean  │
                      │  • passport.rs → .spec.passport.json (static)     │
                      └───────────────────────────────────────────────────┘
                                   │
                        [M4 NEW]   ▼
                      ┌───────────────────────────────────────────────────┐
                      │ pipeline.rs                                        │
                      │  • spec build: run_cargo_build(crate_root)        │
                      │  • spec test:  run_cargo_test(crate_root)         │
                      │  • parse_test_output() → PassportEvidence         │
                      │  • workspace_root_discovery(path) → PathBuf       │
                      └───────────────────────────────────────────────────┘
                                   │
                        [M4 NEW]   ▼
                      ┌───────────────────────────────────────────────────┐
                      │ export.rs                                          │
                      │  • ExportBundle: units, passports, graph, warnings│
                      │  • load_passports_for_specs() (graceful missing)   │
                      │  • emit JSON to stdout or --output <file>          │
                      └───────────────────────────────────────────────────┘
```

Coupling: `pipeline.rs` creates a hard dependency on cargo being available. `export.rs` is stateless (read-only). Both are thin orchestration layers over existing types. Appropriate for M4.

**Section 2 — Code Quality:**
- D1: cargo root discovery must be extracted to `workspace_root_for(path: &Path) -> Result<PathBuf>` — two-step walk: find nearest ancestor Cargo.toml with `[workspace]`; if none found, fall back to nearest ancestor Cargo.toml with `[package]`. Handles workspace projects AND bare crates correctly. Add test: `spec_build_bare_crate_no_workspace_uses_package_toml`.
- D2: test matching must use module-qualified path, but derive the root module prefix from the `--output` path's last component (e.g. `--output src/generated` → prefix `generated`). Not hardcoded to `"generated"`. Pattern: `{prefix}::{namespace}::{unit}::tests::test_{id}`. Add test: `spec_test_evidence_matches_non_default_output_module_name`.
- D2: build failure evidence write — if `cargo build` exits non-zero (before cargo test runs), write `evidence.build_status: "fail"` with `test_results: []` to all passports before exiting 1. Add test: `spec_test_build_failure_writes_fail_build_status_to_passport`.
- D3: export schema needs: `"schema_version": "1.0"` (separate from tool version), `"warnings": [...]` array for skipped/malformed passports. D3 bundle schema example must be updated to include `schema_version` alongside `spec_version`.
- D4: **NOT a 1-line change.** `ResolvedSpec` (types.rs:70) has no `intent` field. Add `pub intent: Option<String>` to `ResolvedSpec`. Update `ResolvedSpec::from_loaded()` to copy `spec.intent.description`. Then `generate_code()` prepends `/// {line}\n` for each line of intent. Files affected: `types.rs`, `generator.rs`.
- D5a: `generate_code()` error message must include unit ID and local_test ID when failing — error context is required for usability. Add test: `generate_code_sink_guard_includes_unit_and_test_id_in_error`.
- D5 internal sequencing (within Track B, must follow this order):
  1. Create `spec-core/src/syntax.rs`, move `is_safe_expect_expr_depth()` + `MAX_EXPECT_EXPR_DEPTH`, expose from `lib.rs`
  2. Update `validator.rs` to import from `syntax.rs` (compile check: both steps together compile)
  3. Add sink guard in `generate_code()` (generator.rs) importing from `syntax.rs`
  Do NOT attempt all three in one diff — intermediate state won't compile.
- Naming: `pipeline.rs`, `export.rs` are clean module names. Consistent with existing `loader.rs`, `validator.rs`, `generator.rs`, `passport.rs` pattern.

**Section 3 — Test Review:**
See test plan artifact: `~/.gstack/projects/atomize-hq-spec/spenquatch-main-m4-test-plan-20260404-172327.md`

24 test gaps identified (17 original + 7 from dual voice review):

New gaps added:
- `spec_build_crate_root_workspace_resolution` (D1)
- `spec_build_crate_root_config_vs_flag_precedence` (D1)
- `spec_build_no_cargo_toml_exits_with_error` (D1)
- `parse_cargo_test_output_ignores_non_test_lines` (D2)
- `parse_cargo_test_output_handles_duplicate_test_ids_across_units` (D2)
- `spec_export_partial_passports_marked_missing` (D3)
- `spec_export_output_path_rejects_directory` (D3)
- `spec_export_schema_version_separate_from_spec_version` (D3)
- `spec_test_writes_evidence_atomically` (D2)

Total: 26 test gaps.

Critical test gaps (both models flag):
- Workspace root discovery (D1)
- Duplicate test_id matching across units (D2)
- Evidence parse confidence field (D2)

**Section 4 — Performance:**
- D1/D2: cargo subprocesses are slow by design. Isolated CARGO_TARGET_DIR established in M2 pattern.
- D3 export: loads all passports from disk. O(n) file reads. Acceptable at M4 unit counts.
- D4: O(len(intent)) string allocation per unit at generate time. Negligible.
- D5a: syn re-parse adds O(body_size) per unit generate call. Noted; deferred to M5.

**Section 5 — Security:**
- D1/D2: `spec build`/`spec test` run user's cargo, which compiles user code. This is expected behavior for a dev tool. No new privilege escalation.
- D3: `--output <file>` path validation: add `assert!(!path.is_dir())` before write. One line.
- D5a: sink guard improves security surface — direct API callers now blocked from unsafe expect strings.
No new auth surfaces, no network requests, no credential handling.

**NOT in scope (eng view):**
- Multi-binary workspace cargo test aggregation: scope spec test to one crate root per invocation
- Evidence provenance (commit SHA, env fingerprint): M5 passport v3
- Cross-library dep implementation: M5 — D6 decides schema only

**What already exists (M4 leverage map):**
- `cargo_available()` at cli.rs:875 and `run_cargo()` at cli.rs:886 — must be moved to `pipeline.rs` (not just referenced as test helpers)
- `Passport` struct at passport.rs:44 — extend with `evidence: Option<PassportEvidence>`
- `serde_json` dependency already present — zero new crates for D3
- `LoadedSpec`, `ResolvedSpec` types → direct inputs to `build_export_bundle()`

**Failure Modes Registry (M4 additions):**
| Codepath | Failure scenario | Test? | Critical? |
|---------|-----------------|-------|-----------|
| workspace_root_for() | Member Cargo.toml found, not workspace root | No | **HIGH** |
| parse_test_output() | Duplicate test_id across units → wrong evidence | No | **HIGH** |
| parse_test_output() | Unexpected output format → silent wrong evidence | No | **HIGH** |
| export: passport load | Malformed JSON → silent omission without warning | No | **HIGH** |
| D3 --output | Directory path → confusing error from serde_json | No | Medium |
| D5a sink guard | syn parse error context missing unit/test ID | No | Medium |
| D2 evidence write | Partial write (interrupt) → stale+updated passports | No | Medium |

**Eng Phase Completion Summary:**
```
ENG REVIEW (autoplan 2026-04-04):
  Architecture:    3 HIGH gaps resolved in plan: workspace discovery, test matching, evidence schema
  Security:        D3 --output path validation added; D5a improves existing surface
  Code quality:    `run_cargo()` must move to pipeline.rs (not test-only); 5 naming/context fixes
  Test coverage:   26 gaps total; 4 critical, 9 high, 13 medium/low
  Test plan:       Written to ~/.gstack/projects/atomize-hq-spec/spenquatch-main-m4-test-plan-20260404-172327.md
  Auto-decisions:  8 (workspace discovery, matching strategy, schema versioning, --output validation)
  Taste decisions: 0 (all architectural gaps have one clearly right fix)
  User challenges: 0
```

**PHASE 3 COMPLETE.** Codex: 5 findings. Claude subagent: 6 findings. Consensus: 4/6 confirmed gaps, 2 confirmed safe. Cross-phase themes (from Phase 1 + Phase 3): evidence quality (CEO + Eng both flag), export schema stability (CEO + Eng). Proceeding to Phase 3.5 (DX Review — DX scope confirmed).

---



---

### Phase 3.5: DX Review

**DX scope: Yes** — spec is a developer CLI tool; the primary user IS the developer.

**CLAUDE SUBAGENT (DX — independent review):**
1. TTHW: 7-10 minutes, not 5 — README Quickstart ends at `spec generate`; spec build/test not in quickstart (High)
2. Silent crate-root auto-discovery: no output telling developer which crate root spec chose (Medium)
3. Missing error spec for no Cargo.toml ancestor: new user hits generic anyhow error (High)
4. D2 unmatched test behavior undefined: silent omission vs "unknown" not specified (Medium)
5. README missing: pipeline config section, spec export bundle schema, spec test evidence section, escape hatch doc (Medium)
6. `spec generate` not documented as first-class escape hatch — implied to be deprecated (Medium)

**CODEX SAYS (DX — developer experience challenge):**
1. TTHW: 10-20 min — assumes spec path, output dir, consuming crate knowledge; `pub use generated::*` still undiscovered (Fail)
2. Error messages: mostly fail — silent failures in D1/D2/D3; "canonical bridge" language contradicts known provenance gaps (High)
3. CLI ergonomics: mixed — names guessable; `--output` means different things for build/test vs export (Medium)
4. Docs: fail — no quickstart for new commands; no named consumer for spec export (High)
5. Upgrade path not fear-free: passport schema v2 breaking change without migration guide or compatibility examples (High)

```
DX DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Getting started < 5 min?          No (7-10) No (10-20) CONFIRMED fail — README not updated
  2. API/CLI naming guessable?         Mostly  Mixed  PARTIAL (names good; --output semantics differ)
  3. Error messages actionable?        Partial Partial CONFIRMED gap (crate-root miss; D2 silent; passports)
  4. Docs findable & complete?         Partial No     CONFIRMED gap (4 new README sections missing)
  5. Upgrade path safe?                Medium  No     CONFIRMED gap (passport v2; no migration guide)
  6. Dev environment friction-free?    Medium  Medium PARTIAL (pub use convention; spec build auto-discovery)
═══════════════════════════════════════════════════════════════
CONFIRMED = both agree (4/6 confirmed gaps — strong DX signal).
```

**Developer Journey Map:**

| Stage | Current DX | M4 Target |
|-------|-----------|-----------|
| 1. Install | `cargo install spec-cli` | Same |
| 2. Author units | Write .unit.spec files | Same |
| 3. Validate | `spec validate ./units` | Same |
| 4. Generate | `spec generate ./units --output ./src/generated` | Same |
| 5. Wire into crate | Add `mod generated;` + `pub use generated::*;` | **Must be documented** |
| 6. Build with spec | — | `spec build ./units --output ./src/generated` |
| 7. Run spec test | — | `spec test ./units --output ./src/generated` |
| 8. Check passport | Open .spec.passport.json | Passport now has `evidence` field |
| 9. Export bundle | — | `spec export ./units` |

**TTHW Assessment:** Initial: ~15 min. Target with M4 DX fixes: ~7 min. Can reach 5 min only if `pub use generated::*` step is eliminated (would require a different crate integration model — M5 scope).

**DX Auto-Decisions:**

1. **D5b scope expansion** (AUTO-DECIDE P1): README.md must include:
   - Pipeline quickstart block: `spec build` + `spec test` invocations
   - `[pipeline]` config section documentation
   - `spec export` bundle schema (top-level keys, example jq query)
   - Escape hatch note: "spec generate remains standalone, spec build/test are opt-in"
   - spec test evidence section: what the passport evidence field contains

2. **D1: print resolved crate root** (AUTO-DECIDE P5): On `spec build`/`spec test`, emit `spec: using crate root <path>` to stderr. Suppressible with `--quiet`. Makes auto-discovery transparent.

3. **D1 acceptance criteria: add missing crate-root error** (AUTO-DECIDE P1): `❌ could not find crate root — run from inside a Cargo project, or pass --crate-root <path>`

4. **D2: define unmatched test behavior** (AUTO-DECIDE P5): Unmatched `test_{id}` in cargo output → emit `"status": "unknown", "reason": "test not found in cargo output"` in evidence. Not silent omission.

5. **D7: add 0.3.0 → 0.4.0 migration notes** (AUTO-DECIDE P1): CHANGELOG must explicitly state: passport schema v2 adds optional `evidence` field (non-breaking to parsers using `serde_json::from_value` with default). No file migration needed.

6. **`--output` disambiguation** (AUTO-DECIDE P5): spec export help text must say `--output <file>  Write JSON bundle to FILE instead of stdout`. build/test help text: `--output <dir>  Directory for generated Rust files (same as spec generate)`. Documented in help text; no flag rename needed.

**DX Scorecard:**

| Dimension | Initial Score | Target Score | Key Fix |
|-----------|--------------|--------------|---------|
| Getting started | 3/10 | 7/10 | Quickstart + pub use documented |
| API/CLI naming | 7/10 | 8/10 | --output disambiguation in help |
| Error messages | 4/10 | 8/10 | Crate-root error + D2 unmatched behavior |
| Docs completeness | 4/10 | 7/10 | 5 new README sections |
| Upgrade path | 5/10 | 8/10 | Migration note in CHANGELOG |
| Dev env friction | 6/10 | 7/10 | Print crate root, pub use documented |
| **Overall** | **4.8/10** | **7.5/10** | |

**DX Implementation Checklist (must ship with M4):**
- [ ] README: pipeline quickstart (spec build + spec test example)
- [ ] README: [pipeline] spec.toml config section
- [ ] README: spec export bundle top-level keys + jq example
- [ ] README: escape hatch note (spec generate is first-class)
- [ ] README: spec test evidence section (what the passport evidence field contains)
- [ ] D1: print resolved crate root to stderr
- [ ] D1: explicit error for no Cargo.toml ancestor in chain
- [ ] D2: define unmatched test behavior (unknown with reason, not silent)
- [x] D7: CHANGELOG 0.3.0→0.4.0 migration note (evidence field is additive)

**PHASE 3.5 COMPLETE.** DX initial: 4.8/10 → target: 7.5/10. TTHW: 15 min → 7 min. Both models agree on 4 critical DX gaps. 9-item DX checklist added to plan. Passing to Phase 4 (Final Gate).

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 3 | CLEAR (HOLD_SCOPE) | autoplan: 2 critical; 2026-04-05: 5 new gaps + 2 Codex tensions resolved |
| Codex Review | `/codex review` | Independent 2nd opinion | 3 | issues_found | 7 findings; 4 substantive (D4 schema, build-fail evidence, module prefix, workspace algo) resolved |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 2 | CLEAR (PLAN) | 7 issues found + resolved: workspace bare-crate fallback, D5 ordering, D4 ResolvedSpec, build-fail evidence, module prefix, 3 new tests |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | CLEAR (PLAN via /autoplan) | score: 4.8/10 → 7.5/10, 9 DX items resolved |

**CODEX (2026-04-05):** D4 requires ResolvedSpec schema change; build failure path needs evidence write spec; module prefix must derive from --output; workspace algo clarified (different repos, two-step walk correct)
**UNRESOLVED:** 0
**VERDICT:** ENG + CEO CLEARED — ready to implement. Run `/ship` when done.

---

### CEO Review Run 1 — autoplan 2026-04-04 (SELECTIVE EXPANSION)

ICP scope creep removed; evidence quality bar raised; workspace root discovery, test_id matching, evidence schema, export schema stability all resolved. D5c (D1 framing, auto-decisions) confirmed. TTHW: 15 min → 7 min via DX fixes.

### CEO Review Run 2 — /plan-ceo-review 2026-04-05 (HOLD SCOPE)

**New gaps found and resolved in plan:**

| Gap | Description | Resolved |
|-----|-------------|---------|
| GAP 1 | cargo hang: no observability or timeout | Documented: `spec: running cargo build...` to stderr; SIGINT propagates; timeout deferred |
| GAP 2 | `spec export --output` parent dir missing → confusing OS error | Fixed: add parent.exists() check + clear error |
| GAP 3 | D5a uses raw syn::parse_str → syn stack overflow on 200+ nested levels | Fixed: must call is_safe_expect_expr_depth() (make pub in validator.rs) |
| GAP A | spec build accepts single-file path → gitignore guard pitfall | Fixed: scope to directory paths only, add error |
| GAP B | spec test with zero local_tests: untested and unstated | Fixed: added test + acceptance criteria |
| D5c | "commit vs ephemeral" open decision never formally closed | Fixed: DECISIONS.md entry added to D5 |

**New tests added:** 5 (31 total from 26)
**New decisions:** 1 (D5c commit-vs-ephemeral formal record)
**Unresolved:** 0
