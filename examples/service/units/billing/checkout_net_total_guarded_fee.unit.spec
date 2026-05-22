id: billing/checkout_net_total_guarded_fee
kind: function
spec_version: "0.3.0"
intent:
  why: Return the net checkout total after discounting the subtotal and applying a nonnegative regional fee only.
contract:
  inputs:
    subtotal: Decimal
    membership_rate: Decimal
    regional_rate: Decimal
  returns: Decimal
  invariants:
    - output >= 0
deps:
  - billing/apply_membership_discount
  - billing/apply_regional_fee
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_membership_discount(subtotal, membership_rate);
        apply_regional_fee(discounted, regional_rate.max(Decimal::ZERO))
    }
local_tests:
  - id: checkout_net_total_guarded_fee_clamps_negative_rates
    expect: checkout_net_total_guarded_fee(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(-5, 2)) == Decimal::new(9000, 2)
links:
  molecule_tests:
    - billing/checkout_declined_discount_flow
