id: pricing/calculate_total
kind: function
intent:
  why: Combine discount and tax so a checkout flow can produce the final price.
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
    pub fn calculate_total(subtotal: Decimal, discount_rate: Decimal, tax_rate: Decimal) -> Decimal {
        let discounted = apply_discount(subtotal, discount_rate);
        apply_tax(discounted, tax_rate)
    }
local_tests:
  - id: combined_flow
    expect: calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(96525, 3)
links:
  molecule_tests:
    - pricing/checkout_total
