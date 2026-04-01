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

- [ ] **Achieve 100% test coverage: 49 tests**
  - Loader: 10 tests (including recursive dir walk, empty dir, mixed files, empty file, non-UTF8 file)
  - Validation: 15 tests (including extra fields, body shape, empty strings, ID format, Rust keyword check, schema meta-validation, use-stmt-in-body, duplicate IDs)
  - Normalizer: 6 tests (including 3+ segment depth, dep fn_name collision, passthrough roundtrip)
  - Generator: 11 tests (including mod.rs generation, deep dep use statements with crate:: prefix, scoped clean, safety marker, zero-files, ID-to-path mapping)
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
- [ ] **Cycle detection in normalizer** — Requires full graph resolution (all units loaded). In M1, deps are trusted strings and partial runs don't have visibility into the full graph. Implement alongside graph resolution in M2. CEO review finding.
- [ ] **Validate contract.inputs type names** — In M1, contract.inputs values are unvalidated strings. In M2, validate that type names are valid Rust types (Decimal, String, u64, etc.) or warn on unrecognized types. CEO review finding.
- [ ] **Add `spec_version` field and schema migration strategy** — When adding new fields or unit kinds in 0.2, need a way to distinguish schema versions and handle backward/forward compatibility. Codex outside-voice finding.
- [ ] **Atomic writes for code generation** — Generate to temp dir, then rename to output dir atomically. Prevents broken output tree on crash or interrupt. Low risk at 1-10 files but matters as project scales. Codex outside-voice finding.

### Release Engineering
- [ ] **Cross-compilation setup for CI** — Build matrix (linux/darwin x amd64/arm64) requires cross-compilation tooling. Use `cross` crate or cargo-zigbuild. Linux targets should use musl for portable binaries. Plan for reproducible checksums. CEO review finding. P1 — blocks release.
