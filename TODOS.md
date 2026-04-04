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
- [ ] **Add recursion depth cap to `is_safe_expect_expr`** — The recursive AST walk has no depth limit. Deeply nested input like `((((x))))` or `!!!!!x` (100+ levels) could stack overflow during validation. Add a `depth: usize` parameter and return `false` above a threshold (~128). Low-urgency since `.unit.spec` files are trusted input, but the fix is trivial. Codex adversarial finding, fix/change-is_safe_expect_expr review.

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
- [ ] **Add fn visibility validation** — `spec validate` should warn or error when body.rust function is not `pub`/`pub(crate)` and the unit is used as a dep. Currently caught only by cargo check in D4. (validator.rs)
- [ ] **Add generate idempotency integration test** — Two identical `spec generate` runs on same spec set produce byte-for-byte identical output. Guards against mtime changes on marker file. (cli.rs integration tests)

### Release Engineering
- [x] **Cross-compilation setup for CI** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux-x86_64-musl, linux-aarch64-musl, macos-x86_64, macos-aarch64
  - Uses `cross` crate for Linux targets, native rustup for macOS
  - Idempotent release creation with `gh release view` check

## M4 Backlog

- [ ] **Pipeline wrap: `spec build` / `spec test` config lever** — spec generates, user runs cargo (default). Add workspace config flag and/or CLI flag to enable spec-wrapped cargo execution: `spec build` = validate + generate + cargo build; `spec test` = spec build + cargo test. Default: off. Target: M4.
