id: pricing/apply_discount_aligned
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
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: apply_discount_aligned_happy_path
    expect: apply_discount_aligned(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
  - id: apply_discount_aligned_clamps_to_zero
    expect: apply_discount_aligned(Decimal::new(10000, 2), Decimal::new(150, 2)) == Decimal::ZERO
