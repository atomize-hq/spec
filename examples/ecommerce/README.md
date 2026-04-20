# ecommerce example

This example shows a small pricing domain authored as `.unit.spec` files, plus molecule tests, a checked-in plan artifact, and the canonical M12 migration wedge for `pricing/checkout_quote`.
The canonical in-repo copy also ships tracked molecule evidence for the two pricing molecule tests so `spec status .` stays truthful on a fresh clone.

## M12 migration wedge

`pricing/checkout_quote` exists in two forms on purpose:

- Raw Rust baseline: `src/raw_baseline/pricing/checkout_quote.rs`
- Migrated seam: `units/pricing/checkout_quote.unit.spec`

Both implement the same pricing job: accept a subtotal plus discount and tax rates, expose `discounted_subtotal()`, and expose `total()`. The hand-written module shows the pre-`spec` baseline. The `kind: data` seam shows the M12 authored version that `spec build` lowers into generated Rust.

The existing `units/pricing/checkout_flow.test.spec` molecule test now covers the migrated seam and compares its result against the pre-existing function-based checkout flow.

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
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/checkout_quote.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_quote.unit.spec --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_flow.test.spec --output examples/ecommerce/src/generated
cargo run -p spec-cli -- status examples/ecommerce --format json
```

If you want the shorter generate/check/test loop instead:

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
- `units/pricing/checkout_quote.unit.spec`
- `units/pricing/checkout_flow.test.spec`
- `units/pricing/discount_plus_tax.test.spec`
- `plans/refactors/checkout-tax-refactor.plan.spec`
- `src/raw_baseline/pricing/checkout_quote.rs`

Derived artifacts such as `src/generated/`, `*.spec.passport.json`, and `*.test.evidence.json` are generated from those source specs and should not be hand-edited.
The checked-in `pricing/*.test.evidence.json` files are the canonical generated outputs for this example. Refresh them by rerunning `spec test units --output src/generated` whenever the molecule specs or their covered unit contracts change, then commit the regenerated files.

The `Cargo.toml` and `src/main.rs` are intentionally minimal. They provide a project scaffold for generated output and the side-by-side raw-vs-migrated checkout quote example.
