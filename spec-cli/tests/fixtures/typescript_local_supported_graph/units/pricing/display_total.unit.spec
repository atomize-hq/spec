
id: pricing/display_total
kind: function
intent:
  why: Echo a computed total for display without changing the value.
spec_version: "0.3.0"
contract:
  inputs:
    total: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        total
    }
  typescript: |
    {
        return total;
    }
local_tests:
  - id: display_total_basic
    expect: "display_total(Decimal::new(9801, 2)) == Decimal::new(9801, 2)"
