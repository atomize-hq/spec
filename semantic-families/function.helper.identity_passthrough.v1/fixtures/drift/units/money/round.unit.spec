id: money/round
kind: function
spec_version: "0.3.0"
intent:
  why: Echo the provided value unchanged for downstream pricing flows.
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
  - id: round_drift_round_like_body
    expect: round(Decimal::new(12345, 3)) == Decimal::new(1235, 2)
