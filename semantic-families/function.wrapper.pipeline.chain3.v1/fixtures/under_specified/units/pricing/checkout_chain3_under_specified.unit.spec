id: pricing/checkout_chain3_under_specified
kind: function
spec_version: "0.3.0"
intent:
  why: checkout chain3
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/pricing_total_wrapper_under_specified
  - pricing/pricing_tax_leaf_under_specified
  - pricing/pricing_discount_leaf_under_specified
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = pricing_total_wrapper_under_specified(subtotal, discount_rate, tax_rate);
        let surcharged_total = pricing_tax_leaf_under_specified(base_total, surcharge_rate);
        pricing_discount_leaf_under_specified(surcharged_total, loyalty_rate)
    }
local_tests:
  - id: checkout_chain3_under_specified_basic
    expect: checkout_chain3_under_specified(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9801, 2)
