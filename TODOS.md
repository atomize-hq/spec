## TODOs for M1 (Release 0.1)

### Required for Release
- [ ] **Set up GitHub Actions for automated builds and releases**
  - Build matrix: linux/darwin x amd64/arm64
  - Trigger: tag push → GitHub Release with binaries + checksums

- [ ] **Write README with installation, quickstart, and examples**
  - Installation: cargo install spec-cli
  - Quickstart: 3 commands to author, validate, and generate first unit

- [ ] **Create example project with 3-5 realistic units**
  - Location: examples/ecommerce/
  - Units: apply_discount, apply_tax, calculate_total
  - Cargo.toml must include runtime dependencies (e.g., rust_decimal) so `cargo check` passes on generated output

- [ ] **Achieve 100% test coverage: 45 tests**
  - Loader: 8 tests (including recursive dir walk, empty dir, mixed files)
  - Validation: 13 tests (including extra fields, body shape, empty strings, ID format, Rust keyword check, schema meta-validation)
  - Normalizer: 7 tests (including 3+ segment depth, dep fn_name collision, passthrough roundtrip)
  - Generator: 9 tests (including mod.rs generation, deep dep use statements, clean-output-dir, safety marker)
  - Validator aggregate: 1 test (collect-all error aggregation)
  - CLI: 5 integration tests
  - Integration: 3 tests (full pipeline, deps, edge cases)

- [ ] **Implement code generator (.rs file writer)**
  - Generate readable Rust code from .unit.spec
  - Handle dependencies between units
  - Auto-generate mod.rs per directory with pub mod declarations
  - Clean output directory before each generate run (prevent stale files)

- [ ] **Validation test for dep fn_name collisions**
  - Two deps with same last segment (e.g., money/round + math/round) should error at validation
  - Discovered via Codex outside-voice review

### Deferred to M2
- [ ] CUE validation
- [ ] Evidence collection and passports
- [ ] Graph resolution
- [ ] Contract-to-signature enforcement
- [ ] **Add `spec_version` field and schema migration strategy** — When adding new fields or unit kinds in 0.2, need a way to distinguish schema versions and handle backward/forward compatibility. Codex outside-voice finding.
- [ ] **Atomic writes for code generation** — Generate to temp dir, then rename to output dir atomically. Prevents broken output tree on crash or interrupt. Low risk at 1-10 files but matters as project scales. Codex outside-voice finding.
