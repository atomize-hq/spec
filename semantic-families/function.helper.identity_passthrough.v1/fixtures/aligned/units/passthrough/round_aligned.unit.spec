id: passthrough/round
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
body:
  rust: |
    {
        value
    }
local_tests:
  - id: round_aligned_passthrough
    expect: round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)
