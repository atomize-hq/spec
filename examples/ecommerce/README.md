# ecommerce example

This example shows a small pricing domain authored as `.unit.spec` files.

## Build / run

This crate expects generated Rust to exist at `src/generated/` (gitignored).

From the repo root:

```bash
cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run --manifest-path examples/ecommerce/Cargo.toml -- check
```

Or, if you have `spec` installed, from `examples/ecommerce/`:

```bash
spec generate units --output src/generated
cargo check
```

Files:

- `units/pricing/apply_discount.unit.spec`
- `units/pricing/apply_tax.unit.spec`
- `units/pricing/calculate_total.unit.spec`

The `Cargo.toml` and `src/main.rs` are intentionally minimal. They provide a project scaffold for generated output and local experimentation with the pricing units.
