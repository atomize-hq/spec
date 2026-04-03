## TODOs for M1 (Release 0.1)

### Required for Release
- [x] **Set up GitHub Actions for automated builds and releases** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux/darwin x amd64/arm64 using `cross` crate
  - Trigger: tag push → GitHub Release with binaries + checksums

- [ ] **Write README with installation, quickstart, and examples**
  - Installation: cargo install spec-cli
  - Quickstart: 3 commands to author, validate, and generate first unit

- [x] **Create example project with 3-5 realistic units** — Completed v0.2.0 (2026-04-02)
  - Location: examples/ecommerce/
  - Units: apply_discount, apply_tax, calculate_total, money/round
  - Cargo.toml includes runtime dependencies (rust_decimal); cargo check + cargo test pass

- [x] **Achieve 100% test coverage** — Completed v0.2.0 (2026-04-02)
  - 60 unit tests + 14 integration tests (74 total, exceeds 49-test target)

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
- [ ] **Cycle detection in normalizer** — Requires full graph resolution (all units loaded). In M1, deps are trusted strings and partial runs don't have visibility into the full graph. Implement alongside graph resolution in M2. CEO review finding.
- [ ] **Validate contract.inputs type names** — In M1, contract.inputs values are unvalidated strings. In M2, validate that type names are valid Rust types (Decimal, String, u64, etc.) or warn on unrecognized types. CEO review finding.
- [ ] **Add `spec_version` field and schema migration strategy** — When adding new fields or unit kinds in 0.2, need a way to distinguish schema versions and handle backward/forward compatibility. Codex outside-voice finding.
- [ ] **Atomic writes for code generation** — Generate to temp dir, then rename to output dir atomically. Prevents broken output tree on crash or interrupt. Low risk at 1-10 files but matters as project scales. Codex outside-voice finding.

### Deferred to M3
- [ ] **`--no-strict` flag for `validate` and `generate`** — Deferred from M2. When cross-library dep composition is introduced, partial-graph workflows need a way to downgrade missing-dep errors to warnings (exit 0). The `finish_validation` return type already scaffolds `(errors, warnings)` for this. ship adversarial review 2026-04-02.
- [ ] **`local_tests.expect` config lever** — Currently restricted to simple expressions (binary, call, path, literal) for safety. Add a workspace config flag to allow block/unsafe expressions for trusted environments. ship adversarial review 2026-04-02.
- [ ] **`--no-strict` flag** — `spec validate --no-strict` and `spec generate --no-strict` downgrade missing-dep errors to warnings (exit 0) for partial-graph workflows. The `finish_validation` tuple return is already scaffolded.

### Release Engineering
- [x] **Cross-compilation setup for CI** — Completed v0.2.0 (2026-04-02)
  - Build matrix: linux-x86_64-musl, linux-aarch64-musl, macos-x86_64, macos-aarch64
  - Uses `cross` crate for Linux targets, native rustup for macOS
  - Idempotent release creation with `gh release view` check
