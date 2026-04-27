
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
local_tests:
  - id: basic_tax
    expect: "apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)"
