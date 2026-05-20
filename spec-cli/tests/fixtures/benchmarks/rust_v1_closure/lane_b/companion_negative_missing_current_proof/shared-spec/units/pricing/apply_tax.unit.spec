id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction and round the total.
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
        round(taxed).max(Decimal::ZERO)
    }
  typescript: |
    {
        const taxed = subtotal.add(subtotal.mul(rate));
        return round(taxed);
    }
local_tests:
  - id: happy_path
    expect: apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
