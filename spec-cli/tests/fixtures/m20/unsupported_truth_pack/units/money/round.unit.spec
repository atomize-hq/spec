
id: money/round
kind: function
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
spec_version: "0.3.0"
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        value
    }
local_tests:
  - id: basic
    expect: "round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)"
