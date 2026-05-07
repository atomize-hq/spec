id: pricing/pricing_tax_leaf_drift
kind: function
spec_version: "0.3.0"
intent:
  why: Return the running checkout subtotal after applying the surcharge rate.
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
  typescript: |
    {
        return subtotal + subtotal * rate;
    }
local_tests:
  - id: pricing_tax_leaf_drift_basic
    expect: pricing_tax_leaf_drift(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)
