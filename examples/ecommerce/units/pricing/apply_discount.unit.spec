id: pricing/apply_discount
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
  typescript: |
    {
        const discounted = subtotal.add(subtotal.mul(Decimal.new(-1n, 0n).mul(rate)));
        return round(discounted);
    }
local_tests:
  - id: happy_path
    expect: apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
links:
  molecule_tests:
    - pricing/discount_plus_tax
