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
- [ ] Evidence collection and passports
- [ ] Graph resolution

- [ ] Contract-to-signature enforcement
- [ ] **Contract-to-signature enforcement (full)** — D3 in M2 covered fn name + arg names (partial). Full type validation (param types, return type, arity, async, generics) deferred to M3.
- [ ] **Cycle detection in normalizer** — Requires full graph resolution (all units loaded). In M1, deps are trusted strings and partial runs don't have visibility into the full graph. Implement alongside graph resolution in M2. CEO review finding.
- [ ] **Validate contract.inputs type names** — In M1, contract.inputs values are unvalidated strings. In M2, validate that type names are valid Rust types (Decimal, String, u64, etc.) or warn on unrecognized types. CEO review finding.
- [ ] **Add `spec_version` field and schema migration strategy** — When adding new fields or unit kinds in 0.2, need a way to distinguish schema versions and handle backward/forward compatibility. Codex outside-voice finding.
- [x] **Atomic writes for code generation** — Completed in D1 (v0.2.0). tempfile::Builder + rename into place (POSIX atomic per-file). Temp in same dir to avoid EXDEV cross-fs errors.

### Deferred to M3
- [ ] **`--no-strict` flag for `validate` and `generate`** — Deferred from M2. When cross-library dep composition is introduced, partial-graph workflows need a way to downgrade missing-dep errors to warnings (exit 0). The `finish_validation` return type already scaffolds `(errors, warnings)` for this. ship adversarial review 2026-04-02.
- [ ] **`local_tests.expect` config lever** — Currently restricted to simple expressions (binary, call, path, literal) for safety. Add a workspace config flag to allow block/unsafe expressions for trusted environments. ship adversarial review 2026-04-02.

### Deferred from autoplan retrospective (2026-04-03)

#### Security / Correctness (High Priority)
- [ ] **Fix `is_safe_expect_expr` to recurse into sub-expressions** — Current implementation checks top-level expression variant but not children. `f({ unsafe { ... } })` passes because outer `Call` returns true without inspecting args. Fix: recurse into `Binary`, `Call`, `MethodCall`, `Field`, `Index`, `Unary`, `Cast` sub-expressions. Add tests: `expect_with_unsafe_block_in_call_arg_is_rejected`, `expect_with_block_in_binary_operand_is_rejected`. (validator.rs:148)

#### Architecture Fixes (Medium Priority)
- [ ] **Consolidate path-containment logic** — `clean_output_dir` uses `normalized_absolute_path` (lexical) and `ensure_output_marker` uses `canonicalize` (symlink-following). Divergent logic = future maintenance hazard. Extract single `safe_output_path(path) -> Result<PathBuf>` utility. (generator.rs, commands.rs)
- [ ] **Add `local_tests[].id` uniqueness validation** — Duplicate ids within a unit → duplicate `fn test_{id}()` → compile error. Validate uniqueness in `validate_local_test_expects`. (validator.rs)
- [ ] **Document `pub use generated::*` as required consuming-crate convention** — Internal deps generate `use crate::X` which only works if consuming crate re-exports generated modules at root. Document in README/DECISIONS.md. Not a code change.

#### M3 Prerequisites (Design Spikes, Before Build)
- [ ] **Define ICP: solo engineer vs. team coordination tool** — Changes M3 priority order completely. One paragraph, before M3 scoping.
- [ ] **Force binary decision: commit generated output vs ephemeral** — Current hybrid (gitignored but required to compile) is unstable for real adopters. Decide and document in DECISIONS.md.
- [ ] **Approach C design spike: invert D3** — Instead of validating body.rust fn name, generate fn signature from `contract.inputs` and treat `body.rust` as the function body expression. Eliminates fn name drift entirely. 2-hour design spike before M3 D3 expansion.
- [ ] **Cross-library dep schema design spike** — Sketch `deps: [money/round@1.2]` or `@org/shared/money/round` schema before M3 build. 2 hours. Prevents breaking schema change.
- [ ] **Define CUE trigger condition explicitly** — Current DECISIONS.md says "CUE when we need cross-file constraints." Specify the exact constraint that triggers the switch (e.g., "when we need dep-exists validation across spec libraries"). Prevents indefinite deferral.

#### Low-Priority Fixes
- [ ] **Fix `generate` file count message** — "Generated N files" only counts unit files, not mod.rs files written. Fix: count `resolved_specs.len() + namespaces.len()`. (commands.rs:145)
- [ ] **Fix non-fn body error message** — When body.rust has exactly 1 item that's not a fn, `BodyRustMustBeSingleFn { found: 0 }` is misleading. Should report found=1 with note "not a function". (validator.rs:175)
- [ ] **Handle symlink cycles in collect_specs gracefully** — `WalkDir::follow_links(true)` with symlink cycle emits error that aborts entire spec collection. Log warning and skip instead. (commands.rs)
- [ ] **Add fn visibility validation** — `spec validate` should warn or error when body.rust function is not `pub`/`pub(crate)` and the unit is used as a dep. Currently caught only by cargo check in D4. (validator.rs)
- [ ] **Add generate idempotency integration test** — Two identical `spec generate` runs on same spec set produce byte-for-byte identical output. Guards against mtime changes on marker file. (cli.rs integration tests)

### Release Engineering
- [x] **Cross-compilation setup for CI** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux-x86_64-musl, linux-aarch64-musl, macos-x86_64, macos-aarch64
  - Uses `cross` crate for Linux targets, native rustup for macOS
  - Idempotent release creation with `gh release view` check
