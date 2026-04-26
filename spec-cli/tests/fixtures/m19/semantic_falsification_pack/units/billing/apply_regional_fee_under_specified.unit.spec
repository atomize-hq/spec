id: billing/apply_regional_fee_under_specified
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
    - output >= subtotal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
local_tests:
  - id: regional_fee_under_specified_basic
    expect: apply_regional_fee_under_specified(Decimal::new(10000, 2), Decimal::new(5, 2)) == Decimal::new(10500, 2)
