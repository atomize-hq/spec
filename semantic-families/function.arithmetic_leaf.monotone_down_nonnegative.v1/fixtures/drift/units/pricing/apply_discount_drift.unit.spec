id: pricing/apply_discount_drift
kind: function
spec_version: "0.3.0"
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
    {
        round(subtotal + subtotal * rate)
    }
local_tests:
  - id: apply_discount_drift_drift
    expect: apply_discount_drift(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)
  - id: apply_discount_drift_over_one_rate_still_grows
    expect: apply_discount_drift(Decimal::new(10000, 2), Decimal::new(150, 2)) == Decimal::new(25000, 2)
