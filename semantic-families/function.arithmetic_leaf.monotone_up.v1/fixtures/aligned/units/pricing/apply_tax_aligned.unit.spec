id: pricing/apply_tax_aligned
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
  - id: apply_tax_aligned_happy_path
    expect: apply_tax_aligned(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
