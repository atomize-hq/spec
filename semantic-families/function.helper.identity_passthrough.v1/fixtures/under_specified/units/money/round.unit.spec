id: money/round
kind: function
spec_version: "0.3.0"
intent:
  why: todo
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
  - id: round_under_specified_passthrough
    expect: round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)
