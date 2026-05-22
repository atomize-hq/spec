id: billing/discount_strategy
kind: sum
spec_version: "0.3.0"
intent:
  why: Represent mutually exclusive membership discount strategies for a service checkout flow.
sum:
  variants:
    declined: {}
    percentage:
      fields:
        rate:
          type: rust_decimal::Decimal
    fixed_credit:
      fields:
        amount:
          type: rust_decimal::Decimal
methods:
  - id: discount_amount
    intent:
      why: Return the discount amount to subtract from the subtotal.
    receiver: shared_ref
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
      returns: rust_decimal::Decimal
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::Declined => rust_decimal::Decimal::ZERO,
                  Self::Percentage { rate } => subtotal * *rate,
                  Self::FixedCredit { amount } => (*amount).min(subtotal),
              }
          }
  - id: discounted_subtotal
    intent:
      why: Return the subtotal after applying the selected discount strategy.
    receiver: shared_ref
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
      returns: rust_decimal::Decimal
    lowering:
      rust:
        body: |
          {
              subtotal - self.discount_amount(subtotal)
          }
  - id: declined_example_holds
    intent:
      why: Support direct atom proof for the canonical declined-discount example.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let policy = Self::Declined;
              policy.discount_amount(rust_decimal::Decimal::new(10000, 2))
                  == rust_decimal::Decimal::ZERO
                  && policy.discounted_subtotal(rust_decimal::Decimal::new(10000, 2))
                      == rust_decimal::Decimal::new(10000, 2)
          }
  - id: percentage_example_holds
    intent:
      why: Support direct atom proof for the canonical percentage discount example.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let policy = Self::Percentage {
                  rate: rust_decimal::Decimal::new(10, 2),
              };
              policy.discount_amount(rust_decimal::Decimal::new(10000, 2))
                  == rust_decimal::Decimal::new(1000, 2)
                  && policy.discounted_subtotal(rust_decimal::Decimal::new(10000, 2))
                      == rust_decimal::Decimal::new(9000, 2)
          }
  - id: fixed_credit_capped_behavior_holds
    intent:
      why: Support direct atom proof for capped fixed-credit behavior.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let policy = Self::FixedCredit {
                  amount: rust_decimal::Decimal::new(2000, 2),
              };
              policy.discount_amount(rust_decimal::Decimal::new(1500, 2))
                  == rust_decimal::Decimal::new(1500, 2)
                  && policy.discounted_subtotal(rust_decimal::Decimal::new(1500, 2))
                      == rust_decimal::Decimal::ZERO
          }
local_tests:
  - id: variant_declined
    expect: DiscountStrategy::Declined.declined_example_holds()
  - id: variant_percentage
    expect: DiscountStrategy::Declined.percentage_example_holds()
  - id: behavior_fixed_credit_capped
    expect: DiscountStrategy::Declined.fixed_credit_capped_behavior_holds()
links:
  molecule_tests:
    - billing/checkout_declined_discount_flow
    - billing/discount_strategy_quote_flow
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
