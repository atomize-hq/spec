id: pricing/pricing_discount_leaf_aligned
kind: function
spec_version: "0.3.0"
intent:
  why: Return the running checkout subtotal after applying the loyalty discount rate and clamping at zero.
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
        (subtotal - subtotal * rate).max(Decimal::ZERO)
    }
local_tests:
  - id: pricing_discount_leaf_aligned_basic
    expect: pricing_discount_leaf_aligned(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
