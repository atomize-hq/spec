# ecommerce example

This example shows a small pricing domain authored as `.unit.spec` files, plus molecule tests and a checked-in plan artifact.

## Build / verify

This crate expects generated Rust to exist at `src/generated/` (gitignored).

If you have `spec` installed, the quickest end-to-end loop from `examples/ecommerce/` is:

```bash
spec build units
spec test units
spec test units/pricing/checkout_flow.test.spec
spec plan validate plans/refactors/checkout-tax-refactor.plan.spec --format json
```

From the repo root:

```bash
cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
cargo check --manifest-path examples/ecommerce/Cargo.toml
cargo test --manifest-path examples/ecommerce/Cargo.toml
```

Or, if you have `spec` installed, from `examples/ecommerce/`:

```bash
spec generate units --output src/generated
cargo check
cargo test
```

Files:

- `units/money/round.unit.spec`
- `units/pricing/apply_discount.unit.spec`
- `units/pricing/apply_tax.unit.spec`
- `units/pricing/calculate_total.unit.spec`
- `units/pricing/checkout_flow.test.spec`
- `units/pricing/discount_plus_tax.test.spec`
- `plans/refactors/checkout-tax-refactor.plan.spec`

Derived artifacts such as `src/generated/`, `*.spec.passport.json`, and `*.test.evidence.json` are generated from those source specs and should not be hand-edited.

The `Cargo.toml` and `src/main.rs` are intentionally minimal. They provide a project scaffold for generated output and local experimentation with the pricing units.
