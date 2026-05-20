id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Return the running subtotal after applying the discount rate and clamping at zero.
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
  typescript: |
    {
        return subtotal.add(subtotal.mul(Decimal.new(-1n, 0n).mul(rate)));
    }
local_tests:
  - id: happy_path
    expect: apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
