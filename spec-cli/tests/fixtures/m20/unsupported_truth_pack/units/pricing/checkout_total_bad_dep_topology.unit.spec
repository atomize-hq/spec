
id: pricing/checkout_total_bad_dep_topology
kind: function
intent:
  why: Return the total after discounting the subtotal and then applying tax.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
  returns: Decimal
deps:
  - pricing/checkout_total
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let total = checkout_total(subtotal, discount_rate, tax_rate);
        apply_tax(total, surcharge_rate)
    }
local_tests:
  - id: bad_dep_topology
    expect: "checkout_total_bad_dep_topology(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4), Decimal::new(5, 2)) == Decimal::new(10135125, 5)"
