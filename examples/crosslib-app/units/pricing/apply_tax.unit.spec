id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Apply tax while importing the shared round helper from a sibling spec library.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
deps:
  - shared::money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed).max(Decimal::ZERO)
    }
local_tests:
  - id: happy_path
    expect: apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
