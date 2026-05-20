# Acceptance

- Freeze one machine-visible regression assertion per in-scope supported-boundary rejection.
- Prefer existing unsupported or near-miss fixture families before creating new lane-owned fixtures.
- Keep repo-root `status . --format json` semantics unchanged.
- Required verification commands:
  - `cargo test -p spec-cli rust_v1_closure`
  - `cargo run -p spec-cli -- status . --format json`
