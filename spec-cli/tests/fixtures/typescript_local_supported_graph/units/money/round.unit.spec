
id: money/round
kind: function
intent:
  why: Round a decimal value for same-tree pricing flows.
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
  typescript: |
    {
        return value;
    }
local_tests:
  - id: round_basic
    expect: "round(Decimal::new(1005, 2)) == Decimal::new(1005, 2)"
