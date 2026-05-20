id: pricing/base_nested_chain3
kind: function
spec_version: "0.3.0"
intent:
  why: Return the shared nested chain3 subtotal before the app root applies its outer surcharge and loyalty discount.
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
  - id: happy_path
    expect: base_nested_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(5, 2), Decimal::new(5, 2)) == Decimal::new(96045, 4)
