id: pricing/apply_tax_drift
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal - subtotal * rate;
        round(taxed.max(Decimal::ZERO))
    }
local_tests:
  - id: apply_tax_drift_drift
    expect: apply_tax_drift(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(9275, 2)
