id: money/round
kind: function
spec_version: "0.3.0"
intent:
  why: Echo the provided decimal value so this packet can exercise the optional helper-dep shape without adding unrelated rounding semantics.
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
  - id: round_unsupported_near_miss_identity
    expect: round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)
