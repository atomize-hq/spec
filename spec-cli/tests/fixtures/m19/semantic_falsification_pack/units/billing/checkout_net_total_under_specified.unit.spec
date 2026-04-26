id: billing/checkout_net_total_under_specified
kind: function
spec_version: "0.3.0"
intent:
  why: Return the net checkout total after discounting the subtotal and then applying the regional fee.
contract:
  inputs:
    subtotal: Decimal
    membership_rate: Decimal
    regional_rate: Decimal
    manual_adjustment: Decimal
  returns: Decimal
deps:
  - billing/apply_membership_discount
  - billing/apply_regional_fee
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_membership_discount(subtotal, membership_rate);
        apply_regional_fee(discounted, regional_rate)
    }
local_tests:
  - id: checkout_net_total_under_specified_basic
    expect: checkout_net_total_under_specified(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(5, 2), Decimal::ZERO) == Decimal::new(9450, 2)
