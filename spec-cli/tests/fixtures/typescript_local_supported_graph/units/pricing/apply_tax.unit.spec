
id: pricing/apply_tax
kind: function
intent:
  why: Return the subtotal after applying the tax rate and rounding the total.
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
        round(taxed).max(Decimal::ZERO)
    }
  typescript: |
    {
        const taxed = subtotal.add(subtotal.mul(rate));
        return round(taxed);
    }
local_tests:
  - id: apply_tax_basic
    expect: "apply_tax(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)"
