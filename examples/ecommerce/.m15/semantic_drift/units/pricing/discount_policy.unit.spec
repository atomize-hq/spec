id: pricing/discount_policy
kind: sum
spec_version: "0.3.0"
intent:
  why: Represent checkout discount strategies that cap fixed discounts at the subtotal.
sum:
  variants:
    none: {}
    percentage:
      fields:
        rate:
          type: rust_decimal::Decimal
    fixed_amount:
      fields:
        amount:
          type: rust_decimal::Decimal
methods:
  - id: discount_amount
    intent:
      why: Return the capped discount amount to subtract from the subtotal.
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
                  Self::None => rust_decimal::Decimal::ZERO,
                  Self::Percentage { rate } => subtotal * *rate,
                  Self::FixedAmount { amount } => *amount,
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
  - id: percentage_example
    intent:
      why: Prove the canonical percentage discount example for semantic review.
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
  - id: fixed_amount_example
    intent:
      why: Prove the current fixed-amount discount example for semantic review.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let policy = Self::FixedAmount {
                  amount: rust_decimal::Decimal::new(1250, 2),
              };
              policy.discount_amount(rust_decimal::Decimal::new(5000, 2))
                  == rust_decimal::Decimal::new(1250, 2)
                  && policy.discounted_subtotal(rust_decimal::Decimal::new(5000, 2))
                      == rust_decimal::Decimal::new(3750, 2)
          }
  - id: fixed_amount_capped_example
    intent:
      why: Prove the current fixed-amount executable example for semantic review.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let policy = Self::FixedAmount {
                  amount: rust_decimal::Decimal::new(2000, 2),
              };
              policy.discount_amount(rust_decimal::Decimal::new(1500, 2))
                  == rust_decimal::Decimal::new(2000, 2)
                  && policy.discounted_subtotal(rust_decimal::Decimal::new(1500, 2))
                      == rust_decimal::Decimal::new(-500, 2)
          }
local_tests:
  - id: variant_none
    expect: 'DiscountPolicy::None.discount_amount(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::ZERO && DiscountPolicy::None.discounted_subtotal(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::new(1500, 2)'
  - id: variant_percentage
    expect: DiscountPolicy::None.percentage_example()
  - id: variant_fixed_amount
    expect: DiscountPolicy::None.fixed_amount_example()
  - id: behavior_fixed_amount_capped
    expect: DiscountPolicy::None.fixed_amount_capped_example()
