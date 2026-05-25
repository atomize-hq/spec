# ecommerce example

This is the canonical single-library `spec` example in the repo.

If the root [`README.md`](../../README.md) tells you what `spec` is, this doc
shows you one truthful place to touch it. Use this example when you want to:

- run the first real validate/build/test/status loop
- inspect both `kind:function` and seam behavior in one library
- see where generated Rust, passports, and molecule evidence actually land
- compare a migrated seam against the pre-`spec` raw Rust baseline

For the broader mental model, read
[`docs/core_mechanisms_guide_v0.1.md`](../../docs/core_mechanisms_guide_v0.1.md)
next to this file, not instead of it.

## Fastest First Run

From the repo root:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/apply_tax.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- status examples/ecommerce/units --format json
```

What changes when you run that loop:

- generated Rust under `examples/ecommerce/src/generated/`
- unit proof in `examples/ecommerce/units/**/*.spec.passport.json`
- molecule proof in `examples/ecommerce/units/**/*.test.evidence.json`

In this example, `src/generated/` is ephemeral and gitignored, but the proof
artifacts are tracked so the example stays truthful on a fresh clone. Re-running
`spec generate` on an unchanged tree should leave those tracked passports alone
instead of churning `generated_at`.

If you have `spec` installed locally, the shorter example-root form is:

```bash
cd examples/ecommerce
spec build units
spec test units/pricing/apply_tax.unit.spec
spec test units/pricing/discount_strategy_checkout_flow.test.spec
spec status units --format json
```

## What This Example Demonstrates

### 1. The canonical pricing function trio

These are the cleanest current `kind:function` examples:

- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`

They are useful because they sit inside the shipped bounded semantic-review
vocabulary:

- `pricing/apply_discount` proves `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `pricing/apply_tax` proves `function.arithmetic_leaf.monotone_up.v1`
- `pricing/calculate_total` proves `function.wrapper.pipeline.v1`

That is a narrow support story on purpose. It is not generic function
understanding.

### 2. A `kind:data` seam

`pricing/pricing_quote.unit.spec` is the clean seam example.

It keeps shared data semantics, constructors, and methods in one authored unit,
then lowers into generated Rust. It is the easiest place to see how seam
support differs from function-family support.

### 3. A `kind:sum` seam with a raw baseline comparison

`pricing/discount_strategy.unit.spec` is the canonical sum-seam migration
wedge.

It exists side by side with the handwritten baseline at:

- `src/raw_baseline/pricing/discount_strategy.rs`

The matching molecule proof,
`pricing/discount_strategy_checkout_flow.test.spec`, checks that the generated
sum seam still agrees with the baseline behavior while composing with the
current pricing flow.

### 4. Molecule tests as cross-unit proof

This example ships three molecule tests:

- `pricing/checkout_flow.test.spec`
- `pricing/discount_plus_tax.test.spec`
- `pricing/discount_strategy_checkout_flow.test.spec`

Use them when the behavior crosses unit boundaries. If a test needs more than
one unit import, it belongs here rather than in `local_tests`.

### 5. A checked-in plan artifact

The canonical example plan lives at:

- `plans/refactors/checkout-tax-refactor.plan.spec`

Use it when you want to inspect the current `.plan.spec` authoring shape
without mixing plan concepts into the normal unit proof loop.

## Command Authority In This Example

The proof-authoritative example root is:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
```

Repo-root status still exists:

```bash
cargo run -p spec-cli -- status . --format json
```

Treat that repo-root view as broad inventory only. It is useful for seeing many
roots at once, but it is not the default green proof wall for this example.

Repo-root export is intentionally unsupported for this workspace shape and
should fail with `SPEC_UNSUPPORTED_SCOPE`.

## Semantic Review Boundary

Two rules matter here:

1. Only `spec test` refreshes semantic-review truth.
2. Unsupported-function truth is explicit through:
   - `semantic_review.support_status`
   - `semantic_review.unsupported_reason_codes`
   - `semantic_review.rewrite_hints`

That means `spec build`, `spec generate`, `spec status`, and `spec export` only
project already-stored semantic-review truth. They do not create new truth.

## Files Worth Reading

Source specs:

- `units/money/round.unit.spec`
- `units/pricing/apply_discount.unit.spec`
- `units/pricing/apply_tax.unit.spec`
- `units/pricing/calculate_total.unit.spec`
- `units/pricing/calculate_total_guarded_tax.unit.spec`
- `units/pricing/pricing_quote.unit.spec`
- `units/pricing/discount_strategy.unit.spec`
- `units/pricing/checkout_flow.test.spec`
- `units/pricing/discount_plus_tax.test.spec`
- `units/pricing/discount_strategy_checkout_flow.test.spec`
- `plans/refactors/checkout-tax-refactor.plan.spec`

Raw baseline comparison:

- `src/raw_baseline/pricing/discount_strategy.rs`
- `src/raw_baseline/pricing/pricing_quote.rs`

Generated and observed artifacts:

- `src/generated/`
- `units/**/*.spec.passport.json`
- `units/**/*.test.evidence.json`

Do not hand-edit generated Rust or proof artifacts. Edit the source specs, then
rerun the relevant `spec` command.

## What To Ignore On Your First Pass

If your only goal is to understand the example, ignore:

- benchmark roster mechanics
- milestone history
- family-recommendation artifacts
- broader repo-root inventory questions

Run the first loop, inspect one unit, inspect one passport, inspect one
molecule evidence file. That is enough to make the rest of the repo much less
mysterious.
