id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount while importing the shared round helper from a sibling spec library.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
deps:
  - shared::money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: happy_path
    expect: apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
