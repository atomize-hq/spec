id: pricing/calculate_total_guarded_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Return the total after discounting the subtotal and then applying tax while clamping the tax rate at zero.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
  invariants:
    - output >= 0
deps:
  - pricing/apply_discount
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_discount(subtotal, discount_rate);
        apply_tax(discounted, tax_rate.max(Decimal::ZERO))
    }
local_tests:
  - id: combined_flow
    expect: calculate_total_guarded_tax(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(96525, 3)
