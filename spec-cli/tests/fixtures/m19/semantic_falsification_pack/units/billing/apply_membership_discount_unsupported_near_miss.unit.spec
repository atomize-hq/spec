id: billing/apply_membership_discount_unsupported_near_miss
kind: function
spec_version: "0.3.0"
intent:
  why: Return the subtotal after applying the membership discount rate and clamping at zero.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        (subtotal - subtotal * rate).max(Decimal::ZERO) + Decimal::ZERO
    }
local_tests:
  - id: membership_discount_unsupported_near_miss_basic
    expect: apply_membership_discount_unsupported_near_miss(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
