
id: money/round
kind: function
intent:
  why: Round monetary values.
body:
  rust: |
    pub fn round(value: Decimal) -> Decimal {
        value
    }
