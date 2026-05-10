id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
  typescript: |
    {
        return subtotal.add(subtotal.mul(rate));
    }
local_tests:
  - id: basic_tax
    expect: apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
links:
  molecule_tests:
    - pricing/discount_plus_tax
