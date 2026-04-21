id: pricing/discount_policy
kind: sum
spec_version: "0.3.0"
intent:
  why: Represent mutually exclusive discount strategies for checkout pricing.
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
                  Self::None => rust_decimal::Decimal::ZERO,
                  Self::Percentage { rate } => subtotal * *rate,
                  Self::FixedAmount { amount } => (*amount).min(subtotal),
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
local_tests:
  - id: none_has_zero_discount_amount
    expect: DiscountPolicy::None.discount_amount(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::ZERO
  - id: none_leaves_subtotal_unchanged
    expect: DiscountPolicy::None.discounted_subtotal(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::new(1500, 2)
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
