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

- [ ] **Achieve 100% test coverage: 25 tests**
  - Generator: 5 tests
  - Validation: 7 tests  
  - Loader: 5 tests
  - CLI: 5 integration tests
  - Integration: 3 tests

- [ ] **Implement code generator (.rs file writer)**
  - Generate readable Rust code from .unit.spec
  - Handle dependencies between units

### Deferred to M2
- [ ] CUE validation
- [ ] Evidence collection and passports
- [ ] Graph resolution
