id: pricing/apply_discount_under_specified
kind: function
spec_version: "0.3.0"
intent:
  why: todo
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
    {
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: apply_discount_under_specified_under_specified
    expect: apply_discount_under_specified(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
  - id: apply_discount_under_specified_clamps_to_zero
    expect: apply_discount_under_specified(Decimal::new(10000, 2), Decimal::new(150, 2)) == Decimal::ZERO
