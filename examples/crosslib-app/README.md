# crosslib app example

This example demonstrates M9 direct sibling-library reuse plus the M61 recursive local-plus-cross-library TypeScript closure across the already-supported function families.

M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.

From the repo root:

```bash
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo check --manifest-path examples/crosslib-app/Cargo.toml
cargo test --manifest-path examples/crosslib-app/Cargo.toml
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
```

The broader TypeScript oceans still explicitly deferred are arbitrary authored 4+ direct-dep topology parity, new semantic-family promotion, molecule TypeScript execution, and seam-kind TypeScript execution.
