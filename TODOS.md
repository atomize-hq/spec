## TODOs for M1 (Release 0.1)

## M101 backlog

- [ ] **Land the category truth contract implementation wedge** — The design anchor is [`docs/category_truth_contract_v0.1.md`](./docs/category_truth_contract_v0.1.md). The failure class is broader than one benchmark bug: read-side consumers must not infer supported category status or positive benchmark credit from partial truth. First adoption scope should cover `sum.discount_strategy.v1`, `data.pricing_quote.v1`, `unsupported.sum.v1`, and `unsupported.data.v1`, with benchmark accounting, `spec status`, `spec export`, and readability/snapshot surfaces consuming the same explicit contract.

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
- [x] **Cross-library dep IMPLEMENTATION** — Completed v0.7.0 (2026-04-16). M9 ships the namespace-prefixed schema (`shared::money/round`) as the repo-scoped direct-library first cut: root-owned `[libraries]` config, typed dep identity shared by validator/generator/export, `use <alias>::...` generation, Cargo dependency alias validation, export schema_version 3 structured dep refs, direct cross-library cycle detection, and explicit rejection of cross-library `.test.spec` covers. Out of scope remains out-of-repo libraries, transitive library discovery, and cross-library graph-query semantics.
- [ ] **M6: Semantic contract-vs-body comparison (LLM eval)** — LLM-powered eval per unit: compare `intent` + `contract` spec against the generated body code. Emit a `semantic_match` score or flag in passport evidence. This catches "body technically compiles and tests pass but the logic doesn't match the intent" — the real governance story. `contract_hash` catches interface drift; this catches semantic drift. Depends on: M5 ships (passport infrastructure + AI-native loop). Needs eval infrastructure (LLM call from spec test, or a separate `spec eval` command). (Added by /plan-eng-review 2026-04-05)
- [x] **M5: Golden JSON fixture tests for --format json stability** — Completed v0.5.0 (2026-04-06). Fixture files added under spec-cli/tests/fixtures/. Integration tests in cli.rs diff against them. (feat/m5)
- [x] **spec build/generate overwrites passport evidence and contract_hash** — Completed v0.12.0 (2026-04-22). Non-test passport writes now preserve existing evidence and freshness anchors, bootstrap a baseline hash only when needed, and reproject current freshness instead of erasing prior `spec test` proof. (spec-cli/src/commands.rs, spec-core/src/passport.rs)
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
- [ ] **CLI test harness: stop spawning orphan-heavy Cargo trees for `spec test` integration coverage** — The M15 semantic-review hang fix removed Cargo from three targeted `spec status` tests, but the broader `spec-cli/tests/cli.rs` harness still has many cases that shell out to `spec test`, which in turn shells out to `cargo build` / `cargo test` with isolated temp target dirs. When those integration tests run concurrently or are interrupted, they can leave large orphaned `spec` / `cargo` / `rustc` process trees behind and create long package-cache lock waits plus sustained fan load. Follow-up scope: (1) inventory the remaining integration tests that only need read-side passport/evidence fixtures and convert them to in-process artifact seeding helpers, (2) for tests that genuinely need end-to-end Cargo execution, route them through a shared test helper that sets a reusable `CARGO_TARGET_DIR` under repo `target/` instead of per-run temp dirs, (3) add a small test-only cleanup/ownership strategy so interrupted runs do not strand nested `spec test` subprocess trees, and (4) document which CLI tests are true pipeline coverage versus artifact/read-path coverage so future additions do not default to spawning Cargo unnecessarily. (Added after M15 semantic-review status test review, 2026-04-23)

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

- [x] **Molecule test status tracking** — Completed v0.9.0 (2026-04-17). Molecule tests now persist `*.test.evidence.json` artifacts, `spec test <file.test.spec>` supports explicit single-test execution, and `spec status` reports a separate molecule-test health plane without contaminating unit status. (Added by /ship feat/m7 2026-04-14)

## M8 Backlog (from M8 eng review)

### Open (from M8 review)

- [ ] **`links.molecule_tests` deprecation** — M8 adds an explicit comment in `SpecGraph::build()` saying `links.molecule_tests` is legacy metadata and `.test.spec` `covers` edges are the authoritative source. This TODO tracks the follow-up: emit `SpecWarning::DeprecatedLinksField` when `links.molecule_tests` is non-empty in a loaded spec, then remove the field from `SpecStruct` and the `Links` struct in a cleanup milestone. **Blocked by:** M8 landing with the comment. **Context:** The `Links` struct lives at `spec-core/src/types.rs:63`. The warning path is `spec-core/src/validator.rs` (alongside other `MissingSpecVersion`-style warnings). (Added by /plan-eng-review main 2026-04-15)

## M11 Follow-ups

- [x] **Ship canonical ecommerce molecule evidence** — Completed main (2026-04-18). `examples/ecommerce` now tracks the generated `pricing/checkout_flow.test.evidence.json` and `pricing/discount_plus_tax.test.evidence.json` artifacts so `spec status .` stays truthful on a fresh clone. Refresh with `spec test examples/ecommerce/units --output examples/ecommerce/src/generated` whenever the molecule specs or covered unit contracts change.

- [x] **Make generated molecule tests warning-clean under strict consumers** — Completed main (2026-04-20). `.test.spec` now supports explicit `imports`, `covers` remains semantic coverage metadata, validate JSON warnings are structured at schema_version 3, and omitted molecule imports take the deprecated legacy fallback with a stable warning code. The canonical ecommerce molecule specs now author explicit imports so `RUSTFLAGS='-D warnings' cargo test --manifest-path examples/ecommerce/Cargo.toml` passes without unused-import failures.

- [ ] **Remove deprecated cover-derived molecule imports fallback** — Follow-up after downstream migration. When all maintained `.test.spec` files author explicit `imports` or `imports: []`, drop the implicit cover-derived import path and remove `SPEC_MOLECULE_IMPLICIT_IMPORTS_DEPRECATED`. This should also remove legacy-only `MoleculeCoversCollision` validation coupling from the fallback path.

## Post-M11 seam follow-ups

- [ ] **Promotion path: nested behaviors → first-class tracked nodes** — The first data seam milestone should keep one top-level seam ID as the tracked truth surface for validation, passports, and status, with constructors/methods modeled as nested behaviors. This TODO captures the deliberate future promotion path when real usage shows seam-level tracking is too coarse. Define the criteria for promotion, what must stay stable in the authored model, and which truth surfaces would change when nested behaviors become independently tracked. **Why:** protects the reduced-scope decision from turning into accidental forever-ontology. **Depends on:** first seam shipping and real usage evidence that seam-level tracking is insufficient.

- [ ] **Escape-hatch gate before second-language work** — Backend-specific escape hatches are allowed only as optional, namespaced, lowering-only details. Before any second-language backend begins, define and enforce the review gate for what qualifies as an allowed escape hatch, what tests must exist for one, and what conditions must be met before Rust-specific lowering details are considered contained enough to not poison the shared core. **Why:** prevents target-specific details from quietly becoming shared semantics. **Depends on:** first seam shipping and actual escape-hatch usage patterns in examples/tests.

- [ ] **Canonical example as compatibility surface** — Once the first data seam ships, treat the canonical example as a maintained contract surface, not demo garnish. When authored seam shape, CLI loop, or escape-hatch rules change, update the example, README commands, AGENTS workflow, and example-backed integration tests together. **Why:** the shipped example is part of the product and agent teaching surface, and example drift creates fake green confidence. **Depends on:** the first data seam example existing.

## Post-M19 follow-ups

- [ ] **Clean M19 falsification fixture unused-variable warnings** — Release QA for v0.13.0 found that `spec build` / `spec test` on `spec-cli/tests/fixtures/m19/semantic_falsification_pack` succeeds but emits Rust unused-variable warnings for intentionally adversarial fixture inputs (`regional_rate` in `checkout_net_total_drift`, `manual_adjustment` in `checkout_net_total_under_specified`). This is not a release blocker and does not invalidate M19. Fix only as fixture hygiene, preferably by underscore-prefixing intentionally unused fixture parameters if that preserves classification. Do not add dummy body statements that widen the Family B syntax envelope, and do not suppress unused warnings generator-wide. **Priority:** Low. **Context:** QA report `.gstack/qa-reports/qa-report-spec-cli-m19-2026-04-26-212920.md`.

## Post-M23 follow-ups

- [x] **Confirm M23 maintainer smoke gate in a clean throwaway checkout** — Completed 2026-04-29 on `codex/m23-contract` at `69b8981f731a6f6d156820c78e4821955601c8a8`. Verified with temp worktrees by deleting `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1`, rerunning `cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1`, confirming `family.toml` stayed byte-for-byte aligned, confirming the four locked pricing starter cases regenerated across `aligned` / `drift` / `under_specified` / `unsupported_near_miss`, confirming the aligned starter spec remained leaf-shaped, and cross-checking `cargo xtask family smoke function.arithmetic_leaf.monotone_down_nonnegative.v1` passed. This retires the M23 maintainer smoke blocker and unblocks M24.

- [x] **M24: promote `function.arithmetic_leaf.monotone_up.v1` as the second real leaf-family packet** — Completed v0.13.1 (2026-04-29). `xtask/src/family/harness.rs` now registers the monotone-up family with locked routing and suite ownership, `xtask/src/family/scaffold.rs` emits the truthful tax starter, `semantic-families/function.arithmetic_leaf.monotone_up.v1/` ships the committed packet, and `cargo xtask family smoke/prove/certify function.arithmetic_leaf.monotone_up.v1` all pass with fresh `.semantic-family-artifacts` proof.

- [ ] **Run a true non-author maintainer promotion dry run** — M23 planning now treats maintainer legibility as a first-class outcome, but the claim is only fully retired once someone other than the original author follows `family new` → packet authoring → prove/certify without hidden context. Do this only after the narrower starter-scaffold smoke gate is explicit and green so the dry run is testing authoring legibility rather than an ambiguous packet diff.

- [ ] **Reduce packet authoring ceremony only after two real leaf promotions exist** — If M23 and M24 both land cleanly, evaluate whether `candidate.md` scaffolding, starter fixtures, or packet metadata can be made lighter without weakening truthfulness. Do not pre-optimize packet ergonomics before the second leaf-family proof exists.

## Post-M37 decision-kernel follow-ups

- [ ] **Generalized multi-wedge decision layer** — trigger: add a second durable non-promotable wedge whose decision path cannot be expressed in `decision_kernel.rs` without branching beyond the current helper-surface contract.

- [ ] **Cross-crate family-analysis shared core** — trigger: at least two non-`recommend.rs` / non-`promotion_artifacts.rs` consumers inside `xtask/src/family/` need the same kernel logic, or a non-`xtask` crate needs the same decision semantics.

- [ ] **Public semantic fingerprint fields** — trigger: an external consumer needs first-class semantic fingerprint fields in emitted JSON, not just internal normalized proof gating.

## Deferred from M46 /autoplan review (2026-05-10)

- [x] **Wrapper TypeScript execution in `spec`** — Completed by M52 (2026-05-12). The bounded Bun-backed TypeScript lane now admits the same-tree `function.wrapper.pipeline.v1` family with the frozen direct local dep tuple `monotone_down_nonnegative -> monotone_up`, while keeping proof additive and target-specific.

- [x] **Bounded same-tree chain3 TypeScript execution in `spec`** — Completed by M54 (2026-05-13). The bounded Bun-backed TypeScript lane now admits the same-tree `function.wrapper.pipeline.chain3.v1` family with the frozen direct local dep tuple `wrapper.pipeline -> monotone_up -> monotone_down_nonnegative`, while keeping proof additive, atom-only, and target-specific.

- [x] **Cross-library TypeScript helper imports** — Completed by M55 (2026-05-13). The bounded Bun-backed TypeScript lane now admits cross-library helper imports in the one legal helper slot for `function.arithmetic_leaf.monotone_up.v1`, and bounded wrapper/chain3 closures may reuse that shared helper transitively once it is in the loaded tree.

- [x] **Direct cross-library wrapper and chain3 TypeScript roots** — Completed by M56 (2026-05-13). The bounded Bun-backed TypeScript lane now admits direct cross-library wrapper roots, direct cross-library chain3 roots, and exact mixed local-plus-shared direct dep tuples for the frozen promoted families, while keeping closure collection bounded and broader TypeScript execution claims deferred.

- [x] **Bounded same-tree nested chain3 TypeScript closure** — Completed by M58 (2026-05-14). The bounded Bun-backed TypeScript lane now admits a same-tree `function.wrapper.pipeline.chain3.v1` in chain3 slot 1, recurses through that validated nested closure inside the same loaded tree, preserves the pre-Bun rejection wall for wrong family, wrong dep order, missing nested `body.typescript`, and cross-library recursion, and leaves slot 2 plus slot 3 frozen.

- [x] **Semantic-review-driven same-tree local TypeScript function graph execution** — Completed by M59 (2026-05-14). The bounded Bun-backed TypeScript lane now admits same-tree local `kind:function` roots across the shipped supported semantic-review families, validates the reachable local closure graph-wide before Bun, dedupes shared same-tree subgraphs, excludes unrelated loaded units, and preserves the existing direct cross-library helper, wrapper, and chain3 portability lanes unchanged.

- [x] **Normalized required-arg wrapper family in semantic review** — Completed by M60 (2026-05-15). M60 adds one supported wrapper family for apply_tax(discounted, tax_rate.max(Decimal::ZERO)); broader required-argument expressions remain unsupported.

- [x] **Recursive local-plus-cross-library TypeScript closure across shipped families** — Completed by M61 (2026-05-15). M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.

- [ ] **Remaining TypeScript oceans after M61** — After M61, the shipped lane covers recursive local-plus-cross-library closure across the already-supported function families. The broader TypeScript oceans still explicitly deferred are arbitrary authored 4+ direct-dep topology parity, new semantic-family promotion, molecule TypeScript execution, and seam-kind TypeScript execution.

## Post-M68 benchmark mechanics

- [x] **M68: Rust V1 benchmark mechanics landing** — Completed main (2026-05-18). The repo now ships `benchmarks/labels.json`, a shared benchmark projection core, additive schema-version-4 `benchmarks[]` in `spec status` / `spec export`, `spec benchmark snapshot <id>`, seeded readability review anchoring for `BENCH-ECOM`, reserved `BENCH-SERVICE` visibility, committed snapshot artifacts, and repo-facing CLI fixtures/docs for the full-versus-partial benchmark contract.

- [x] **M69: supported-core closure after mechanics-only landing** — Completed main (2026-05-21). `BENCH-SERVICE` is now an active single-library benchmark rooted at `examples/service/units`, ships the frozen six-unit / three-molecule roster with fresh proof, keeps benchmark accounting read-only over specs/passports/evidence, and still does not widen the Rust V1 support vocabulary beyond the existing supported-core closure.

## Post-I7 scope closure

- [x] **I7: Rust V1 scope-decision closure** — Completed main (2026-05-22). The repo now freezes one explicit pre-I8 story: bounded generics defer to `V1.1`, Rust V1 stays synchronous-only so async/IO also defer to `V1.1`, `BENCH-CROSSLIB` remains the companion negative-proof wall, and the final I8 wall stays the existing five-command proof run.

- [x] **I8: Rust V1 final proof run** — Completed main (2026-05-23). The frozen five-command wall still holds on the live branch: `BENCH-ECOM` and `BENCH-SERVICE` both reran as passing positive proof walls with current readability state, repo-root `status . --format json` remained the expected `inventory_only` non-green inventory surface with exit code `1`, and the bounded Rust V1 claim stayed narrow without widening scope.
