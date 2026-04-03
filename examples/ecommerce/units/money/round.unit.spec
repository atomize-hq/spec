id: money/round
kind: function
intent:
  why: Round monetary values (placeholder for compilation proof).
imports:
  - rust_decimal::Decimal
body:
  rust: |
    pub fn round(value: Decimal) -> Decimal {
        value
    }

