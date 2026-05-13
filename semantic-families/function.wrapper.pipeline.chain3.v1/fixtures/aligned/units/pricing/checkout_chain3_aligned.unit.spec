id: pricing/checkout_chain3_aligned
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
  - pricing/pricing_total_wrapper_aligned
  - pricing/pricing_tax_leaf_aligned
  - pricing/pricing_discount_leaf_aligned
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
        let surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
        pricing_discount_leaf_aligned(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
        const surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
        return pricing_discount_leaf_aligned(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: checkout_chain3_aligned_basic
    expect: checkout_chain3_aligned(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9801, 2)
