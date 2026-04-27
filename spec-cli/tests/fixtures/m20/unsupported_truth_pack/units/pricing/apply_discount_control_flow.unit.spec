
id: pricing/apply_discount_control_flow
kind: function
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
spec_version: "0.3.0"
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
        if discounted < Decimal::ZERO {
            Decimal::ZERO
        } else {
            round(discounted)
        }
    }
local_tests:
  - id: control_flow
    expect: "apply_discount_control_flow(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)"
