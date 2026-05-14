
id: pricing/apply_discount
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
        round(discounted.max(Decimal::ZERO))
    }
  typescript: |
    {
        const discounted = subtotal.add(subtotal.mul(Decimal.new(-1n, 0n).mul(rate)));
        return round(discounted);
    }
local_tests:
  - id: apply_discount_basic
    expect: "apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)"
