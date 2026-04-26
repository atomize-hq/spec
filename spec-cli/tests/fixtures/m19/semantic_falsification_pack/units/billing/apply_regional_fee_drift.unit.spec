id: billing/apply_regional_fee_drift
kind: function
spec_version: "0.3.0"
intent:
  why: Return the subtotal after applying the regional fee rate.
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
        (subtotal + subtotal * rate).max(Decimal::ZERO)
    }
local_tests:
  - id: regional_fee_drift_basic
    expect: apply_regional_fee_drift(Decimal::new(10000, 2), Decimal::new(5, 2)) == Decimal::new(10500, 2)
