id: pricing/discount_policy_checkout_flow
spec_version: "0.3.0"
intent:
  why: Prove that the sum seam stays aligned with the existing checkout quote and tax flow.
covers:
  - pricing/discount_policy
  - pricing/checkout_quote
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
  - crate::pricing::discount_policy::DiscountPolicy
  - crate::pricing::checkout_quote::CheckoutQuote
  - crate::pricing::apply_tax::apply_tax
body:
  rust: |
    {
        let percentage_policy = DiscountPolicy::Percentage {
            rate: Decimal::new(10, 2),
        };
        let percentage_discounted = percentage_policy.discounted_subtotal(Decimal::new(10000, 2));
        let quote = CheckoutQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );

        assert_eq!(percentage_discounted, Decimal::new(9000, 2));
        assert_eq!(percentage_discounted, quote.discounted_subtotal());
        assert_eq!(apply_tax(percentage_discounted, Decimal::new(725, 4)), quote.total());

        let fixed_policy = DiscountPolicy::FixedAmount {
            amount: Decimal::new(1250, 2),
        };
        let fixed_discounted = fixed_policy.discounted_subtotal(Decimal::new(5000, 2));
        let fixed_taxed = apply_tax(fixed_discounted, Decimal::new(725, 4));

        assert_eq!(fixed_discounted, Decimal::new(3750, 2));
        assert_eq!(fixed_taxed, Decimal::new(40218750, 6));
        assert!(fixed_taxed > fixed_discounted);
    }
