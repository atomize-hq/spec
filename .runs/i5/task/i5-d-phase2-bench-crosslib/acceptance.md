# Acceptance

- Refresh proof for `pricing/calculate_total` and `pricing/checkout_nested_chain3`.
- Preserve zero positive credit for companion-negative cases.
- Add closure assertions that missing active companion proof keeps `BENCH-CROSSLIB` incomplete.
- Add closure assertions that companion-negative cases never increment `positive_credit_cases`.
- Required verification commands:
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_discount.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
  - `cargo run -p spec-cli -- status examples/crosslib-app/units --format json`
  - `cargo run -p spec-cli -- export examples/crosslib-app/units`
  - `cargo test -p spec-cli rust_v1_closure`
