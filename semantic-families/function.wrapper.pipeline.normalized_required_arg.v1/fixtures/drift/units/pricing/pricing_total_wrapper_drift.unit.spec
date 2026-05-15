id: pricing/pricing_total_wrapper_drift
kind: function
spec_version: "0.3.0"
intent:
  why: Return the checkout total after discounting the subtotal and then applying tax.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/pricing_discount_leaf_drift
  - pricing/pricing_tax_leaf_drift
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = pricing_discount_leaf_drift(subtotal, discount_rate);
        pricing_tax_leaf_drift(discounted, discount_rate.max(Decimal::ZERO))
    }
local_tests:
  - id: pricing_total_wrapper_drift_basic
    expect: pricing_total_wrapper_drift(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9900, 2)
