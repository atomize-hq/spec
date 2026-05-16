
id: pricing/calculate_total
kind: function
intent:
  why: Return the total after discounting the subtotal and then applying tax.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/apply_discount
  - pricing/apply_tax
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
  - id: calculate_total_basic
    expect: "calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9900, 2)"
