
id: pricing/checkout_total
kind: function
intent:
  why: Return the checkout total by reusing the local wrapper, then applying a surcharge and a loyalty discount.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/calculate_total
  - pricing/apply_tax
  - pricing/apply_discount
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = calculate_total(subtotal, discount_rate, tax_rate);
        let surcharged_total = apply_tax(base_total, surcharge_rate);
        apply_discount(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = calculate_total(subtotal, discount_rate, tax_rate);
        const surcharged_total = apply_tax(base_total, surcharge_rate);
        return apply_discount(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: checkout_total_basic
    expect: "checkout_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9801, 2)"
