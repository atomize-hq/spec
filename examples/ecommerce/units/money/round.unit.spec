id: money/round
kind: function
intent:
  why: Round monetary values (placeholder for compilation proof).
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

