id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: happy_path
    expect: apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
links:
  molecule_tests:
    - pricing/discount_plus_tax
