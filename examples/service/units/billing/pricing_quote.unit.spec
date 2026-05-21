id: billing/pricing_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a service checkout total from subtotal plus membership discount and regional fee rates.
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    membership_rate:
      type: rust_decimal::Decimal
    regional_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        membership_rate: rust_decimal::Decimal
        regional_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      membership_rate: membership_rate
      regional_rate: regional_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the subtotal after the membership discount and before the regional fee.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - billing/apply_membership_discount
    lowering:
      rust:
        body: |
          {
              apply_membership_discount(self.subtotal, self.membership_rate)
          }
  - id: total
    intent:
      why: Return the final checkout total after the membership discount and regional fee.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - billing/apply_regional_fee
    lowering:
      rust:
        body: |
          {
              apply_regional_fee(self.discounted_subtotal(), self.regional_rate)
          }
local_tests:
  - id: discounted_subtotal_basic
    expect: PricingQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(5, 2)).discounted_subtotal() == rust_decimal::Decimal::new(9000, 2)
  - id: total_basic
    expect: PricingQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(5, 2)).total() == rust_decimal::Decimal::new(9450, 2)
links:
  molecule_tests:
    - billing/checkout_success_flow
    - billing/checkout_declined_discount_flow
    - billing/discount_strategy_quote_flow
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
