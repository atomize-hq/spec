id: money/round
kind: function
spec_version: "0.3.0"
intent:
  why: Round a decimal for reuse by sibling spec libraries.
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        value.round_dp(2)
    }
local_tests:
  - id: rounds_half_up
    expect: round(Decimal::new(12345, 3)) == Decimal::new(1235, 2)
