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
- [ ] Graph resolution

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
- [ ] **Cross-library dep schema design spike** — Sketch `deps: [money/round@1.2]` or `@org/shared/money/round` schema before M3 build. 2 hours. Prevents breaking schema change.
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

- [ ] **Pipeline wrap: `spec build` / `spec test` config lever** — spec generates, user runs cargo (default). Add workspace config flag and/or CLI flag to enable spec-wrapped cargo execution: `spec build` = validate + generate + cargo build; `spec test` = spec build + cargo test. Default: off. See PLAN.md D1 for workspace-aware crate-root discovery design.
- [ ] **Runtime evidence in passports (D2)** — After `spec test`, update passport with observed test results: `evidence.build_status`, `evidence.test_results[{id, status, reason?}]`, `evidence.parse_confidence`. Evidence is "last observed locally" — not CI-canonical (no commit SHA or runner identity). Provenance deferred to M5.
- [ ] **JSON export v1 (`spec export`) (D3)** — Emit machine-readable bundle: `{schema_version: "1.0", spec_version: "0.4.0", exported_at, units[], passports[], graph:{edges[]}, warnings[]}`. Passports missing = `passport_missing: true` marker, not silent omission. `--output <file>` writes to file; default stdout.
- [ ] **Doc comments in generated Rust (D4)** — `generate_code()` prepends `/// {intent}` above each `pub fn`. Requires: (1) add `intent: Option<String>` to `ResolvedSpec` + update `from_loaded()` in types.rs; (2) prepend `/// {line}\n` per intent line in generator.rs. ~15 lines across 2 files.
- [ ] **D5a: Defense-in-depth sink guard** — Move `is_safe_expect_expr_depth()` to new `spec-core/src/syntax.rs` shared module. Both `validator.rs` and `generator.rs` import from it. Call it in `generate_code()` instead of raw `syn::parse_str`. Error must include unit ID + test ID in context. Raw syn::parse_str overflows on 200+ nested levels — this is the same bug fixed in the validator, now at the generate_code sink. Newtype refactor (ValidatedExpr) deferred to M5.
- [ ] **D5b: README updates (DX checklist)** — 5 new sections: pipeline quickstart, [pipeline] spec.toml config, spec export bundle schema + jq example, escape hatch note (spec generate is first-class), spec test evidence section.
- [ ] **D6: Cross-library dep schema decision** — Design spike (2 hrs): compare namespace prefix (`shared::money/round`), versioned path (`money/round@1.2`), registry path (`org/shared/money/round`). Output: DECISIONS.md entry with chosen schema and rationale. Must complete before M5 build.

- [ ] **D5c: DECISIONS.md — "Generated Output: Ephemeral by Default"** — Close the open "commit vs ephemeral" decision from M3 TODOS. D1 (spec build) resolves it implicitly; write the formal record before M5 engineers re-litigate it. 1-paragraph entry. No code. Effort: XS. (Added by /plan-ceo-review 2026-04-05)
- [ ] **Create spec-core/src/syntax.rs shared module** — Move `is_safe_expect_expr_depth()` from `validator.rs` to a new `syntax.rs` module (pub). Update `validator.rs` and `generator.rs` (D5a) to import from `syntax.rs`. This avoids leaking validator internals into the public API. Also expose from `lib.rs`. Effort: XS (~15 min). Depends on: D5a implementation. (Added by /plan-ceo-review 2026-04-05, Codex tension resolved)
- [ ] **D4: Add `intent: Option<String>` to ResolvedSpec** — `generate_code()` receives `ResolvedSpec` which has no `intent` field. Add `pub intent: Option<String>` to `ResolvedSpec` (types.rs). Update `ResolvedSpec::from_loaded()` to copy `spec.intent.description`. Enables D4 doc comment prepend in `generate_code()`. Effort: XS (~15 min, 2 files). (Added by /plan-eng-review 2026-04-05 — D4 estimate was '1-line change' but requires schema field addition)
- [ ] **Document nextest limitation (D5b addition)** — `spec test` parses standard `cargo test` output format only. nextest uses a completely different format and is not currently supported. Document in README under `## Pipeline` section. Add: "Note: spec test parses standard cargo test output. If your project uses cargo-nextest, use spec generate + cargo test directly for now." Effort: XS. Depends on: D5b implementation. (Added by /plan-eng-review 2026-04-05)

## M5 Backlog (from M4 review)

- [ ] **ICP definition** — One paragraph in DECISIONS.md: who is the v0.x user? Solo engineer, small team, or broader? Gates M5 scoping. (User deferred from M4 — explicit decision 2026-04-04)
- [ ] **Evidence provenance (passport v3)** — Add commit SHA, runner identity, env fingerprint to passport evidence schema. Makes evidence CI-trustworthy, not just locally observed.
- [ ] **D5a newtype refactor (ValidatedExpr)** — Replace `String` expect in `ResolvedSpec` with `ValidatedExpr` newtype wrapping `syn::Expr`. Eliminates double-parse cost and gives type-safe API boundary.
- [ ] **parse_test_output() HashMap optimization** — If implemented with a vec scan (O(lines × units)), refactor to build a HashMap of expected test IDs before the scan (O(lines)). Document as a performance note in pipeline.rs. Effort: XS. Depends on: D2 implementation. (Added by /plan-ceo-review 2026-04-05)
- [ ] **cargo timeout support (wait_timeout)** — `spec build`/`spec test` use `std::process::Command::output()` which blocks indefinitely. Add configurable timeout via the `wait_timeout` crate or similar. For M4, the hang is documented behavior (SIGINT propagates). Effort: S. (Added by /plan-ceo-review 2026-04-05)
- [ ] **Cross-library dep IMPLEMENTATION** — After D6 schema is decided, implement cross-library dep loading, cycle detection across libraries, and use statement generation for external spec units.
