id: pricing/apply_tax_under_specified
kind: function
spec_version: "0.3.0"
intent:
  why: todo
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
  - id: apply_tax_under_specified_under_specified
    expect: apply_tax_under_specified(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
