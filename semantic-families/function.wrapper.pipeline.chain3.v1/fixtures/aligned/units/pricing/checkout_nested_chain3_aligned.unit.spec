id: pricing/checkout_nested_chain3_aligned
kind: function
spec_version: "0.3.0"
intent:
  why: Return the final checkout total by recursing through one same-tree nested chain3 before applying the outer surcharge and loyalty discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/base_nested_chain3_aligned
  - pricing/pricing_tax_leaf_aligned
  - pricing/pricing_discount_leaf_aligned
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = base_nested_chain3_aligned(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate);
        let surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
        pricing_discount_leaf_aligned(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = base_nested_chain3_aligned(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate);
        const surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
        return pricing_discount_leaf_aligned(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: checkout_nested_chain3_aligned_basic
    expect: checkout_nested_chain3_aligned(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(970299, 4)
