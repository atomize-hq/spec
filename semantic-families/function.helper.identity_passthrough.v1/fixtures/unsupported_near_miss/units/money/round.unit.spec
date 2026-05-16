id: money/round
kind: function
spec_version: "0.3.0"
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
  - rust_decimal::RoundingStrategy
body:
  rust: |
    {
        if value == Decimal::ZERO {
            value
        } else {
            value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
        }
    }
local_tests:
  - id: round_unsupported_near_miss_rounds_nonzero
    expect: round(Decimal::new(12345, 3)) == Decimal::new(1235, 2)
  - id: round_unsupported_near_miss_preserves_zero_branch
    expect: round(Decimal::ZERO) == Decimal::ZERO
