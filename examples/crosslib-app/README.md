# crosslib app example

This example demonstrates M9 direct sibling-library reuse plus M55 cross-library helper imports in the bounded TypeScript lane.

From the repo root:

```bash
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo check --manifest-path examples/crosslib-app/Cargo.toml
cargo test --manifest-path examples/crosslib-app/Cargo.toml
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
```
