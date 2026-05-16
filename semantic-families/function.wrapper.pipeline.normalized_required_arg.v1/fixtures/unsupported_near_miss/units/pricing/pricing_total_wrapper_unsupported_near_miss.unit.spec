id: pricing/pricing_total_wrapper_unsupported_near_miss
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
  - pricing/pricing_discount_leaf_unsupported_near_miss
  - pricing/pricing_tax_leaf_unsupported_near_miss
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = pricing_discount_leaf_unsupported_near_miss(subtotal, discount_rate);
        pricing_tax_leaf_unsupported_near_miss(
            discounted,
            tax_rate.max(Decimal::ZERO).round_dp(4),
        )
    }
local_tests:
  - id: pricing_total_wrapper_unsupported_near_miss_basic
    expect: pricing_total_wrapper_unsupported_near_miss(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9900, 2)
