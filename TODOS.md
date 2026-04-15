## TODOs for M1 (Release 0.1)

### Required for Release
- [x] **Set up GitHub Actions for automated builds and releases** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux/darwin x amd64/arm64 using `cross` crate
  - Trigger: tag push → GitHub Release with binaries + checksums

- [x] **Write README with installation, quickstart, and examples** — Completed v0.2.0 (2026-04-02)
  - Installation: cargo install spec-cli
  - Quickstart: 3 commands to author, validate, and generate first unit

- [x] **Create example project with 3-5 realistic units** — Completed v0.2.0 (2026-04-02)
  - Location: examples/ecommerce/
  - Units: apply_discount, apply_tax, calculate_total, money/round
  - Cargo.toml includes runtime dependencies (rust_decimal); cargo check + cargo test pass

- [x] **Achieve 100% test coverage** — Completed v0.2.0 (2026-04-02)

  - 60 unit tests + 14 integration tests (76 total after M2 fixes)

- [x] **Implement code generator (.rs file writer)** — Completed v0.2.0 (2026-04-02)
  - Generates readable Rust from .unit.spec
  - Handles deps with crate:: use statements
  - Auto-generates mod.rs per directory
  - Cleans orphaned .rs files from prior runs

- [x] **Validation test for dep fn_name collisions** — Completed v0.2.0 (2026-04-02)
  - Tested in validator::tests::test_validate_dep_collision

### Deferred to M2
- [x] **Implement dep validation: always strict (no flag)** — Completed v0.2.0 (2026-04-02)
  - `finish_validation` validates all dep IDs exist in the loaded spec set
  - Error: `❌ dep 'money/round' not found in this spec set`
  - `--no-strict` flag deferred to M3

- [ ] CUE validation (candidate 0.3+; JSON Schema for 0.1/0.2 — see DECISIONS.md)
- [x] **Evidence collection and passports** — Completed v0.3.0 (2026-04-04). `spec generate` emits `.spec.passport.json` per unit, gitignored automatically. Passport contains id, intent, contract, deps, local_tests, generated_at. (passport.rs)
- [x] **Graph resolution** — Completed v0.6.0 (2026-04-15). `SpecGraph` now ships as the M8 declared graph layer with sorted accessors, reverse dependency queries, direct covering-test queries, and `ImpactSet`-based impact analysis in `spec-core`. (spec-core/src/graph.rs, spec-core/src/export.rs, spec-core/src/lib.rs)

- [x] **Contract-to-signature enforcement (full)** — Completed v0.3.0 (2026-04-04). D3 inverted: `body.rust` is now a block, `spec generate` synthesizes fn signature from `contract.inputs`/`contract.returns`. `validate_contract_input_types` validates parameter names as Rust identifiers and type strings as valid Rust types. (validator.rs, generator.rs)
- [x] **Cycle detection in normalizer** — Completed v0.3.0 (2026-04-04). `detect_cycles` DFS detects cycles across the full loaded spec set. `validate_deps_exist_with_options` runs cycle detection after missing-dep checks. (validator.rs)
- [x] **Validate contract.inputs type names** — Completed v0.3.0 (2026-04-04). `validate_contract_input_types` validates both parameter names (as `syn::Ident`) and type strings (as `syn::Type`). (validator.rs)
- [x] **Add `spec_version` field and schema migration strategy** — Completed v0.3.0 (2026-04-04). `spec_version` optional field added to schema and types. `check_spec_versions` emits `MissingSpecVersion` warning. Migration guide in README.md. (types.rs, validator.rs, README.md)
- [x] **Atomic writes for code generation** — Completed in D1 (v0.2.0). tempfile::Builder + rename into place (POSIX atomic per-file). Temp in same dir to avoid EXDEV cross-fs errors.

### Deferred to M3
- [x] **`--no-strict` flag for `validate` and `generate`** — Completed v0.2.2 (2026-04-03). `spec validate --no-strict` downgrades missing-dep errors to warnings and exits 0. `spec generate --no-strict` is explicitly rejected with a helpful message. ship adversarial review 2026-04-02.
- [x] **`local_tests.expect` config lever** — Completed v0.2.2 (2026-04-03). `spec.toml` workspace config with `[validation] allow_unsafe_local_test_expect = true` permits block/unsafe expressions in trusted environments. ship adversarial review 2026-04-02.

### Deferred from autoplan retrospective (2026-04-03)

#### Security / Correctness (High Priority)
- [x] **Fix `is_safe_expect_expr` to recurse into sub-expressions** — Completed fix/change-is_safe_expect_expr. `f({ unsafe { ... } })` now rejected. Added regression tests: `expect_with_unsafe_block_in_call_arg_is_rejected`, `expect_with_block_in_binary_operand_is_rejected`, `expect_with_unsafe_block_in_method_call_arg_is_rejected`. (validator.rs:148)

#### Architecture Fixes (Medium Priority)
- [x] **Consolidate path-containment logic** — Completed v0.2.2 (2026-04-03). `safe_output_path` utility extracted; both `clean_output_dir` and `ensure_output_marker` now use it. (generator.rs, commands.rs)
- [x] **Add `local_tests[].id` uniqueness validation** — Completed v0.2.2 (2026-04-03). Duplicate IDs within a unit caught at validation time with `DuplicateLocalTestId` error. (validator.rs)
- [ ] **Document `pub use generated::*` as required consuming-crate convention** — Internal deps generate `use crate::X` which only works if consuming crate re-exports generated modules at root. Document in README/DECISIONS.md. Not a code change.

#### Architecture (Medium Priority)
- [ ] **Defense-in-depth: validate local_tests[].expect at the sink** — `generate_code` (generator.rs:19) is a public library function that embeds `local_test.expect` verbatim with no validation. The CLI path always validates first via the loader, but a direct library API caller constructing a `ResolvedSpec` manually bypasses all expression validation. Consider: (a) validate at the `generate_code` sink, (b) use a newtype wrapper for validated expect strings, or (c) emit the generated assert!() from the validated syn::Expr AST instead of the raw string. Codex outside-voice finding, fix/change-is_safe_expect_expr review.
- [x] **Add recursion depth cap to `is_safe_expect_expr`** — Completed v0.3.0+ (2026-04-04). `MAX_EXPECT_EXPR_DEPTH=128` added; `is_safe_expect_expr_depth` returns false at threshold. Regression test added. Fixed by /qa on main (2026-04-04). (validator.rs)

#### M3 Prerequisites (Design Spikes, Before Build)
- [ ] **Define ICP: solo engineer vs. team coordination tool** — Changes M3 priority order completely. One paragraph, before M3 scoping.
- [ ] **Force binary decision: commit generated output vs ephemeral** — Current hybrid (gitignored but required to compile) is unstable for real adopters. Decide and document in DECISIONS.md.
- [x] **Approach C design spike: invert D3** — Completed v0.3.0 (2026-04-04). D3 inversion shipped: `body.rust` is now a block, fn signature generated from `contract.inputs`. (validator.rs, generator.rs)
- [x] **Cross-library dep schema design spike** — Completed 2026-04-05 by D6 decision record. Cross-library deps are locked to namespace-prefixed syntax (`shared::money/round`) with future `[libraries]` config mapping in `spec.toml`. Versioned and registry-qualified alternatives are deferred. (DECISIONS.md)
- [x] **Define CUE trigger condition explicitly** — Completed v0.3.0 (2026-04-04). DECISIONS.md documents explicit trigger conditions for CUE adoption. (DECISIONS.md)

#### Low-Priority Fixes
- [x] **Fix `generate` file count message** — Completed v0.2.2 (2026-04-03). `resolved_specs.len() + namespaces.len()` used so mod.rs files are included in the count. (commands.rs)
- [x] **Fix non-fn body error message** — Completed v0.2.2 (2026-04-03). `BodyRustSingleItemNotFn` error variant emits "found 1 item (not a function)". (validator.rs)
- [x] **Handle symlink cycles in collect_specs gracefully** — Completed v0.2.2 (2026-04-03). `load_directory_report` emits `SpecWarning::SymlinkCycleSkipped` and continues. CLI surfaces warnings in stderr and success message. (loader.rs, commands.rs)
- [x] **Add fn visibility validation** — Obsolete as of D3 inversion (v0.3.0). `generate_code` always emits `pub fn`; there is no user-authored fn signature to validate. Closed by /qa on main (2026-04-04).
- [x] **Add generate idempotency integration test** — Completed v0.3.0 (2026-04-04, feat/m3). `generate_is_idempotent_for_same_spec_tree` added in cli.rs.
- [x] **Omit `-> ()` for void functions** — Completed v0.3.0+ (2026-04-04). `build_fn_signature` now omits the return annotation when `contract.returns` is absent. Fixed by /qa on main (2026-04-04). (generator.rs)

### Release Engineering
- [x] **Cross-compilation setup for CI** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux-x86_64-musl, linux-aarch64-musl, macos-x86_64, macos-aarch64
  - Uses `cross` crate for Linux targets, native rustup for macOS
  - Idempotent release creation with `gh release view` check

## M4 Backlog (current sprint)

- [x] **Pipeline wrap: `spec build` / `spec test` config lever** — Completed v0.4.0 (2026-04-05). `spec build` = validate + generate + cargo build; `spec test` = spec build + cargo test. `[pipeline]` config in spec.toml. **Completed:** v0.4.0 (2026-04-05)
- [x] **Runtime evidence in passports (D2)** — Completed v0.4.0 (2026-04-05). `spec test` writes evidence with `build_status`, `test_results[{id, status, reason?}]`, and `observed_at` to passports. **Completed:** v0.4.0 (2026-04-05)
- [x] **JSON export v1 (`spec export`) (D3)** — Completed v0.4.0 (2026-04-05). Emits `{schema_version, spec_version, exported_at, units[], passports[], graph:{edges[]}, warnings[]}`. `--output <file>` or stdout. **Completed:** v0.4.0 (2026-04-05)
- [x] **Doc comments in generated Rust (D4)** — Completed v0.4.0 (2026-04-05). `generate_code()` emits `/// {intent}` doc comments above each `pub fn`. `intent` added to `ResolvedSpec`. **Completed:** v0.4.0 (2026-04-05)
- [x] **D5a: Defense-in-depth sink guard** — Completed v0.4.0 (2026-04-05). `syntax.rs` shared module created; both `validator.rs` and `generator.rs` import from it. `generate_code()` validates expect exprs at sink. **Completed:** v0.4.0 (2026-04-05)
- [x] **D5b: README updates (DX checklist)** — Completed v0.4.0 (2026-04-05). `pub use generated::*` convention documented in README. **Completed:** v0.4.0 (2026-04-05)
- [x] **D6: Cross-library dep schema decision** — Completed 2026-04-05. `DECISIONS.md` now locks the future cross-library dep syntax to namespace-prefixed ids (`shared::money/round`) with `[libraries]` config mapping, plus tradeoff analysis and explicit deferrals. M4 remains design-only; M5 implements behavior.

- [x] **D5c: DECISIONS.md — "Generated Output: Ephemeral by Default"** — Completed v0.4.0 (2026-04-05). DECISIONS.md documents that `spec build`/`spec test` treat generated Rust as ephemeral output. **Completed:** v0.4.0 (2026-04-05)
- [x] **Create spec-core/src/syntax.rs shared module** — Completed v0.4.0 (2026-04-05). `syntax.rs` created; `validator.rs` and `generator.rs` import from it. Exposed from `lib.rs`. **Completed:** v0.4.0 (2026-04-05)
- [x] **D4: Add `intent: Option<String>` to ResolvedSpec** — Completed v0.4.0 (2026-04-05). `ResolvedSpec.intent` added; `from_loaded()` copies `spec.intent.why`. **Completed:** v0.4.0 (2026-04-05)
- [x] **Document nextest limitation (D5b addition)** — Completed v0.5.2 (2026-04-12). README Pipeline section now explicitly calls out that `cargo nextest` format is unsupported and produces `status: "unknown"`. (feat/m6a) **Completed:** v0.5.2 (2026-04-12)

## M5 Backlog (from M4 review)

- [x] **ICP definition** — Completed v0.5.0 (2026-04-06). ICP paragraph written in DECISIONS.md: solo engineer or 2-5 person AI-heavy team where correctness matters. (feat/m5)
- [x] **Evidence provenance (passport v3)** — Completed feat/m5-follow-up (2026-04-11). Additive `provenance.git_commit_sha` in `PassportEvidence` and top-level `ExportBundle`. Best-effort; omitted when not in a git repo. (spec-core/src/passport.rs, spec-core/src/export.rs, spec-cli/src/commands.rs)
- [ ] **D5a newtype refactor (ValidatedExpr)** — Replace `String` expect in `ResolvedSpec` with `ValidatedExpr` newtype wrapping `syn::Expr`. Eliminates double-parse cost and gives type-safe API boundary.
- [x] **parse_test_output() HashMap optimization** — Completed feat/m5-follow-up (2026-04-11). `parse_cargo_test_output` and `build_test_evidence` now use `HashMap`. Benchmark harness added at `#[ignore]` for manual validation. (spec-core/src/pipeline.rs, spec-cli/src/commands.rs)
- [x] **cargo timeout support (wait_timeout)** — Completed feat/m5-follow-up (2026-04-11). `[pipeline] timeout_secs` config key; enforced in `run_cargo_build`/`run_cargo_test` via `wait_timeout` crate. Exit code 124. (spec-core/src/pipeline.rs, spec-cli/src/config.rs)
- [ ] **Cross-library dep IMPLEMENTATION** — Implement the chosen namespace-prefixed schema (`shared::money/round`) using `[libraries]` config mapping in `spec.toml`, including cross-library dep loading, validation, use statement generation, and cycle detection across libraries.
- [ ] **M6: Semantic contract-vs-body comparison (LLM eval)** — LLM-powered eval per unit: compare `intent` + `contract` spec against the generated body code. Emit a `semantic_match` score or flag in passport evidence. This catches "body technically compiles and tests pass but the logic doesn't match the intent" — the real governance story. `contract_hash` catches interface drift; this catches semantic drift. Depends on: M5 ships (passport infrastructure + AI-native loop). Needs eval infrastructure (LLM call from spec test, or a separate `spec eval` command). (Added by /plan-eng-review 2026-04-05)
- [x] **M5: Golden JSON fixture tests for --format json stability** — Completed v0.5.0 (2026-04-06). Fixture files added under spec-cli/tests/fixtures/. Integration tests in cli.rs diff against them. (feat/m5)
- [ ] **spec build/generate overwrites passport evidence and contract_hash** — `spec build` (and `spec generate`) call `finalize_passports` with `evidence=None` and `contract_hash=None`, which overwrites any existing passport including one written by a prior `spec test`. Running `spec build` after `spec test [file]` silently erases the contract_hash and test evidence; `spec status` then reports the unit as "valid, no-evidence" and never stale even if the contract changes. Fix: in `write_passports` (commands.rs), read the existing passport before overwriting and preserve `evidence` and `contract_hash` when the new call provides neither. Alternatively, `spec generate` and `spec build` should not write passports at all — passport writing belongs exclusively to `spec test`. Effort: S. Side effect: any code that reads passports after `spec generate` expecting static metadata (id, contract, etc.) would need updating. (Added by /review 2026-04-06)
- [x] **Pre-M6: Stable external error code namespace for --format json** — Completed feat/m5-follow-up (2026-04-11). All `SpecError` variants now emit `SPEC_*` screaming-snake codes. Exhaustiveness test added. Fixtures and AGENTS.md updated. (spec-cli/src/commands.rs)
- [x] **Refactor: spec_error_to_json_entry two-pass match** — Completed feat/m5-follow-up (2026-04-11). Replaced 9-tuple with named `ErrorFields` struct. `spec_error_code` extracted as standalone function. (spec-cli/src/commands.rs)
- [ ] **Refactor: extract push_error/push_warning loop helper** — The 4-line pattern (push loader errors + push validation errors + push warnings) is duplicated verbatim in validate_command, export_command, and generate_specs. Extract a collect_diagnostics helper. Effort: XS. (Added by /ship 2026-04-06)
- [x] **Refactor: test_command passport finalization duplication** — Completed feat/m5-follow-up (2026-04-11). `PassportWritePlan` struct + `finalize_test_passports` helper unify build-failure and post-test paths. (spec-cli/src/commands.rs)
- [x] **Performance: BTreeMap → HashMap in parse_cargo_test_output and DiagnosticMap** — Completed feat/m5-follow-up (2026-04-11). `parse_cargo_test_output` returns `HashMap`; `build_test_evidence` lookup is O(1). Benchmark harness included (run with `cargo test -- --ignored`). (spec-core/src/pipeline.rs, spec-cli/src/commands.rs)
- [x] **spec_error_to_json_entry emits empty path for Io/Json errors** — Completed v0.5.1 (2026-04-07). Changed `path: String` → `path: Option<String>` with `skip_serializing_if = Option::is_none` in `JsonErrorEntry`. Four pathless variants (Io, Json, Generator, OutputDir) now omit `path` entirely; all others emit `Some(path)`. (feat/m5)
- [x] **spec test single-file module filter uses only last output path segment** — Already fixed in v0.5.1 (2026-04-07). `output_module_prefix` now uses full path components, not just `file_name()`. Filter correctly handles nested outputs like `src/generated/spec`. (feat/m5)
- [x] **stale detection bypassed by passports without contract_hash** — Fixed v0.5.1 (2026-04-07). `write_passports` now computes and stores an initial `contract_hash` baseline on first `spec generate` when none exists on disk. Subsequent `spec generate` calls preserve the existing hash (test-written or prior baseline). Stale detection fires correctly when the contract changes without a matching `spec test` run. (feat/m5)
- [x] **Concurrent process detection/warning** — Completed feat/m5-follow-up (2026-04-11). `ConcurrentPassportWriteGuard` writes a temp-dir marker on `finalize_passports` entry, warns when other active markers exist. Best-effort / warn-only; TTL 5 min. (spec-cli/src/commands.rs)
- [ ] **pipeline.rs eprintln! forward-compat for --format json** — `run_cargo_build` (pipeline.rs:70) and `run_cargo_test` (pipeline.rs:79) emit unconditional `eprintln!` status lines. If build/test ever get `--format json` support, these will contaminate stderr and bypass the machine-readable contract (per operational learning eprintln-json-mode-bypass). Fix: thread an `OutputFormat` parameter through `run_cargo_build`/`run_cargo_test` and gate eprintln! on `OutputFormat::Text`. Effort: XS. Do this before adding --format json to build/test commands. (Added by /plan-eng-review 2026-04-11)
- [x] **push_error/push_warning helper extracted** — Already implemented at commands.rs:1209-1220 (marked as pending in Priority 3 plan section in error — was completed before plan was written). (Confirmed by /plan-eng-review 2026-04-11)

## M6 Backlog (from M5 review)

- [x] **Default output path `src/generated`** — Completed v0.5.2 (2026-04-12). `spec generate`, `spec build`, and `spec test` derive the output dir from crate root via `spec.toml` or ancestor `Cargo.toml` walk; default is `src/generated`. `--output` still accepted for legacy paths. (feat/m6, commands.rs)
- [x] **`generated_module_prefix` config key** — Completed v0.5.2 (2026-04-12). `[pipeline] generated_module_prefix` in `spec.toml` overrides auto-derived module path for non-standard layouts. (feat/m6, config.rs)
- [x] **`Verbosity` enum in pipeline API** — Completed v0.5.2 (2026-04-12). `Verbosity::Normal` preserves existing stderr output; `Verbosity::Silent` suppresses it. Prepares `run_cargo_build`/`run_cargo_test` for future `--format json` mode. (feat/m6, pipeline.rs)
- [x] **Module prefix evidence mismatch** — Completed v0.5.2 (2026-04-12). Effective prefix now computed once and shared between cargo filter and evidence lookup. Fixes "all tests unknown" when derivation paths disagreed. (feat/m6, commands.rs)
- [x] **Document nextest limitation** — Completed v0.5.2 (2026-04-12). README Pipeline section explicitly calls out that `cargo nextest` format is unsupported and produces `status: "unknown"`. (feat/m6a, README.md)

## M7 Backlog (from M6 review)

- [x] **Molecule tests (`.test.spec`) — first-class support** — Completed v0.5.3 (2026-04-13). `spec validate`, `generate`, `build`, `test`, and `export` all handle `.test.spec` files. Each test declares `covers` units and a full Rust block body. Generated as `#[test]` functions in `molecule_tests.rs` per namespace, gated with `#[cfg(test)]`. (feat/m7, commands.rs, generator.rs, loader.rs)
- [x] **`SpecGraph` in spec-core** — Completed v0.5.3 (2026-04-13). Minimal typed graph (`UnitNode`, `MoleculeTestNode`, `SpecEdge`) built from loaded specs and molecule tests. Foundation for M8 full graph layer. (feat/m7, graph.rs)
- [x] **`spec export` schema_version 2** — Completed v0.5.3 (2026-04-13). `ExportEdge` changed to tagged enum with `kind` field (`"dep"` / `"covers"`). `molecule_tests` array added to bundle. (feat/m7, export.rs)
- [x] **`spec status` schema_version 2 with new health states** — Completed v0.5.3 (2026-04-13). Added `failing`, `incomplete`, `untested` states. `stale: bool` removed; `reason: Option<String>` added. Non-valid units exit 1. (feat/m7, commands.rs)
- [x] **Single-file CLI scope fix for sibling molecules** — Completed v0.5.3 (2026-04-13). File-path invocations of `validate`, `generate`, `export` no longer load sibling `.test.spec` files. Directory invocations load all. (feat/m7, commands.rs)
- [x] **Reserved `molecule_tests` namespace segment** — Completed v0.5.3 (2026-04-13). Unit IDs and molecule test IDs containing `molecule_tests` as any segment are rejected at validation time. (feat/m7, validator.rs)

### Open (from M7 review)

- [ ] **Molecule test status tracking** — AGENTS.md documents that molecule test failures don't propagate to unit status. A failing molecule test currently has no tracked status of its own. Future milestone should track per-molecule-test pass/fail in a `molecule_tests_evidence` field (or similar) so CI can surface molecule test regressions without confusing unit health. (Added by /ship feat/m7 2026-04-14)

## M8 Backlog (from M8 eng review)

### Open (from M8 review)

- [ ] **`links.molecule_tests` deprecation** — M8 adds an explicit comment in `SpecGraph::build()` saying `links.molecule_tests` is legacy metadata and `.test.spec` `covers` edges are the authoritative source. This TODO tracks the follow-up: emit `SpecWarning::DeprecatedLinksField` when `links.molecule_tests` is non-empty in a loaded spec, then remove the field from `SpecStruct` and the `Links` struct in a cleanup milestone. **Blocked by:** M8 landing with the comment. **Context:** The `Links` struct lives at `spec-core/src/types.rs:63`. The warning path is `spec-core/src/validator.rs` (alongside other `MissingSpecVersion`-style warnings). (Added by /plan-eng-review main 2026-04-15)
