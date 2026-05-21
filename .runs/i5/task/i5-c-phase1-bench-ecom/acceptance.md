# Acceptance

- Add `pricing/discount_strategy_checkout_flow` to `BENCH-ECOM.required_molecule_ids`.
- Refresh proof for `pricing_quote.unit.spec`, `discount_strategy.unit.spec`, and `discount_strategy_checkout_flow.test.spec`.
- Add closure assertions that missing, stale, or failing required seam proof breaks `BENCH-ECOM`.
- Do not refresh committed snapshots or shared benchmark fixtures.
- Required verification commands:
  - `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec`
  - `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec`
  - `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec`
  - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
  - `cargo run -p spec-cli -- export examples/ecommerce/units`
  - `cargo test -p spec-cli rust_v1_closure`
