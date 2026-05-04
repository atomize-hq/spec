id: pricing/pricing_total_wrapper_under_specified
kind: function
spec_version: "0.3.0"
intent:
  why: Adjust the checkout total using the current pricing inputs.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/pricing_discount_leaf_under_specified
  - pricing/pricing_tax_leaf_under_specified
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = pricing_discount_leaf_under_specified(subtotal, discount_rate);
        pricing_tax_leaf_under_specified(discounted, tax_rate)
    }
  typescript: |
    {
        const discounted = pricing_discount_leaf_under_specified(subtotal, discount_rate);
        return pricing_tax_leaf_under_specified(discounted, tax_rate);
    }
local_tests:
  - id: pricing_total_wrapper_under_specified_basic
    expect: pricing_total_wrapper_under_specified(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9900, 2)
