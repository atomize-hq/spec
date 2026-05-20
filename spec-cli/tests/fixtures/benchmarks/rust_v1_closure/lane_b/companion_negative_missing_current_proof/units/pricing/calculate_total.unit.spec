id: pricing/calculate_total
kind: function
spec_version: "0.3.0"
intent:
  why: Return the checkout total after discounting the subtotal and then applying tax with shared pricing leaves.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - shared::pricing/apply_discount
  - shared::pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_discount(subtotal, discount_rate);
        apply_tax(discounted, tax_rate)
    }
  typescript: |
    {
        const discounted = apply_discount(subtotal, discount_rate);
        return apply_tax(discounted, tax_rate);
    }
local_tests:
  - id: happy_path
    expect: calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(9653, 2)
