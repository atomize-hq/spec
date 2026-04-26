# ecommerce example

This example shows a small pricing domain authored as `.unit.spec` files, plus molecule tests, a checked-in plan artifact, and the canonical M13 migration wedge for `pricing/discount_policy`.
The canonical in-repo copy also ships tracked molecule evidence for the pricing molecule tests so `spec status .` stays truthful on a fresh clone.

## M13 migration wedge

`pricing/discount_policy` exists in two forms on purpose:

- Raw Rust baseline: `src/raw_baseline/pricing/discount_policy.rs`
- Migrated seam: `units/pricing/discount_policy.unit.spec`

Both implement the same pricing job: choose one discount strategy, compute `discount_amount(subtotal)`, and expose `discounted_subtotal(subtotal)`. The hand-written enum shows the pre-`spec` baseline implementation of that branching behavior. The `kind: sum` seam shows the M13 authored version that `spec build` lowers into generated Rust.

The new `units/pricing/discount_policy_checkout_flow.test.spec` molecule test covers the M13 `sum` seam together with the existing `pricing/checkout_quote` `data` seam and `pricing/apply_tax` function unit so the example proves a mixed-kind checkout flow, not just enum syntax.

The original M12 `pricing/checkout_quote` seam remains in place as a sibling example.

## M18 semantic review boundary

The pricing trio now demonstrates the bounded `kind:function` semantic-review families shipped in M18:

- `pricing/apply_discount` proves `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `pricing/apply_tax` proves `function.arithmetic_leaf.monotone_up.v1`
- `pricing/calculate_total` proves `function.wrapper.pipeline.v1`

This is still a bounded support story, not generic function understanding. Unsupported near-miss wrappers stay additive-only and non-demoting, and only `spec test` refreshes semantic-review truth. `spec build`, `spec generate`, `spec status`, and `spec export` project stored truth only.

## Locked adversarial score table

Recorded calibration scores for the migration wedge candidates:

- `pricing/discount_policy`: `19`
- `pricing/checkout_quote`: `16`
- `pricing/discount_plus_tax`: `14`

Canonical wedge remains `pricing/discount_policy`.

## Build / verify

This crate expects generated Rust to exist at `src/generated/` (gitignored).

If you have `spec` installed, the quickest end-to-end loop from `examples/ecommerce/` is:

```bash
spec build units
spec test units/pricing/discount_policy.unit.spec
spec test units/pricing/discount_policy_checkout_flow.test.spec
spec plan validate plans/refactors/checkout-tax-refactor.plan.spec --format json
```

From the repo root:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_policy.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec
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
- `units/pricing/discount_policy.unit.spec`
- `units/pricing/discount_policy_checkout_flow.test.spec`
- `units/pricing/checkout_quote.unit.spec`
- `units/pricing/checkout_flow.test.spec`
- `units/pricing/discount_plus_tax.test.spec`
- `plans/refactors/checkout-tax-refactor.plan.spec`
- `src/raw_baseline/pricing/discount_policy.rs`
- `src/raw_baseline/pricing/checkout_quote.rs`

Derived artifacts such as `src/generated/`, `*.spec.passport.json`, and `*.test.evidence.json` are generated from those source specs and should not be hand-edited.
The checked-in `pricing/*.test.evidence.json` files are the canonical generated outputs for this example. Refresh them by rerunning `spec test units --output src/generated` whenever the molecule specs or their covered unit contracts change, then commit the regenerated files.
Single-file `spec test` runs use an isolated internal generated tree, so they do not rewrite this example's checked-out `src/generated/` directory.

The `Cargo.toml` and `src/main.rs` are intentionally minimal. They provide a project scaffold for generated output and the side-by-side raw-vs-migrated pricing seam examples.
