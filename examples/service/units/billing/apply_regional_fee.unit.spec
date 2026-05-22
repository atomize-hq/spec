id: billing/apply_regional_fee
kind: function
spec_version: "0.3.0"
intent:
  why: Return the discounted subtotal after applying the regional fee rate.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
local_tests:
  - id: regional_fee_basic
    expect: apply_regional_fee(Decimal::new(10000, 2), Decimal::new(5, 2)) == Decimal::new(10500, 2)
links:
  molecule_tests:
    - billing/checkout_success_flow
    - billing/discount_strategy_quote_flow
