id: pricing/checkout_chain3_unsupported_near_miss
kind: function
spec_version: "0.3.0"
intent:
  why: Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/pricing_total_wrapper_unsupported_near_miss
  - pricing/pricing_tax_leaf_unsupported_near_miss
  - pricing/pricing_discount_leaf_unsupported_near_miss
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        pricing_discount_leaf_unsupported_near_miss(
            pricing_tax_leaf_unsupported_near_miss(
                pricing_total_wrapper_unsupported_near_miss(subtotal, discount_rate, tax_rate),
                surcharge_rate,
            ),
            loyalty_rate,
        )
    }
local_tests:
  - id: checkout_chain3_unsupported_near_miss_basic
    expect: checkout_chain3_unsupported_near_miss(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9801, 2)
