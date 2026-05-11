# ecommerce example

This example shows a small pricing domain authored as `.unit.spec` files, plus molecule tests, a checked-in plan artifact, and the canonical migration wedge for `pricing/discount_strategy`.
The canonical in-repo copy also ships tracked molecule evidence for the pricing molecule tests so `spec status .` stays truthful on a fresh clone.

## M13 migration wedge

`pricing/discount_strategy` exists in two forms on purpose:

- Raw Rust baseline: `src/raw_baseline/pricing/discount_strategy.rs`
- Migrated seam: `units/pricing/discount_strategy.unit.spec`

Both implement the same pricing job: choose one discount strategy, compute `discount_amount(subtotal)`, and expose `discounted_subtotal(subtotal)`. The hand-written enum shows the pre-`spec` baseline implementation of that branching behavior. The `kind: sum` seam shows the authored version that `spec build` lowers into generated Rust.

The `units/pricing/discount_strategy_checkout_flow.test.spec` molecule test covers the `pricing/discount_strategy` `sum` seam together with the existing `pricing/pricing_quote` `data` seam and `pricing/apply_tax` function unit so the example proves a mixed-kind checkout flow, not just enum syntax.

The `pricing/pricing_quote` seam remains in place as a sibling example.

## M20 semantic review boundary

The pricing trio demonstrates the bounded `kind:function` semantic-review families currently proved with unseen examples and stricter wrapper-flow checks:

- `pricing/apply_discount` proves `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `pricing/apply_tax` proves `function.arithmetic_leaf.monotone_up.v1`
- `pricing/calculate_total` proves `function.wrapper.pipeline.v1`

This is still a bounded support story, not generic function understanding, and M20 adds no new supported family. Unsupported near-miss wrappers stay keyed as `unsupported.function.v1`, additive-only, and health-neutral.

M20 also makes unsupported-function truth explicit. The public fields are exactly `semantic_review.support_status`, `semantic_review.unsupported_reason_codes`, and `semantic_review.rewrite_hints`. New supported reviews write `support_status: supported`; unsupported function reviews write `support_status: unsupported`. Consumers should branch on `semantic_review.support_status == "unsupported"` rather than infer unsupported state from `verdict` or `evaluator_scope`, though legacy reviews without `support_status` still fall back to `evaluator_scope` plus `unsupported.*.v1` inference.

Only `spec test` refreshes semantic-review truth. `spec build`, `spec generate`, `spec status`, and `spec export` project stored truth only. Fresh unsupported function proof is preserved on read-side surfaces such as `spec status` and `spec export`; stale unsupported function proof is dropped there while the unit's freshness/stale health still reports normally.

## Locked adversarial score table

Recorded calibration scores for the migration wedge candidates:

- `pricing/discount_strategy`: `19`
- `pricing/pricing_quote`: `16`
- `pricing/discount_plus_tax`: `14`

Canonical wedge remains `pricing/discount_strategy`.

## Build / verify

This crate expects generated Rust to exist at `src/generated/` (gitignored).

If you have `spec` installed, the quickest end-to-end loop from `examples/ecommerce/` is:

```bash
spec build units
spec test units/pricing/discount_strategy.unit.spec
spec test units/pricing/discount_strategy_checkout_flow.test.spec
spec plan validate plans/refactors/checkout-tax-refactor.plan.spec --format json
```

From the repo root:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_strategy.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
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
- `units/pricing/discount_strategy.unit.spec`
- `units/pricing/discount_strategy_checkout_flow.test.spec`
- `units/pricing/pricing_quote.unit.spec`
- `units/pricing/checkout_flow.test.spec`
- `units/pricing/discount_plus_tax.test.spec`
- `plans/refactors/checkout-tax-refactor.plan.spec`
- `src/raw_baseline/pricing/discount_strategy.rs`
- `src/raw_baseline/pricing/pricing_quote.rs`

Derived artifacts such as `src/generated/`, `*.spec.passport.json`, and `*.test.evidence.json` are generated from those source specs and should not be hand-edited.
The checked-in `pricing/*.test.evidence.json` files are the canonical generated outputs for this example. Refresh them by rerunning `spec test units --output src/generated` whenever the molecule specs or their covered unit contracts change, then commit the regenerated files.
Single-file `spec test` runs use an isolated internal generated tree, so they do not rewrite this example's checked-out `src/generated/` directory.

The `Cargo.toml` and `src/main.rs` are intentionally minimal. They provide a project scaffold for generated output and the side-by-side raw-vs-migrated pricing seam examples.
