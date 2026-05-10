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
        value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
    }
local_tests:
  - id: round_aligned_round_like
    expect: round(Decimal::new(12345, 3)) == Decimal::new(1235, 2)
