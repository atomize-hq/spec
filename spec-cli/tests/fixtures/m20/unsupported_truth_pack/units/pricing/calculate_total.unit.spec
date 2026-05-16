
id: pricing/calculate_total
kind: function
intent:
  why: Return the total after discounting the subtotal and then applying tax.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/apply_discount
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        apply_tax(apply_discount(subtotal, discount_rate), tax_rate + Decimal::ZERO)
    }
local_tests:
  - id: combined_flow
    expect: "calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(96525, 3)"
