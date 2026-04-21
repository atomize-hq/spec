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
  - crate::pricing::checkout_quote::CheckoutQuote
  - crate::pricing::apply_tax::apply_tax
body:
  rust: |
    {
        let subtotal = Decimal::new(1500, 2);
        let raw_none = crate::raw_baseline::pricing::discount_policy::DiscountPolicy::None;
        let generated_none = crate::pricing::discount_policy::DiscountPolicy::None;
        assert_eq!(raw_none.discount_amount(subtotal), generated_none.discount_amount(subtotal));
        assert_eq!(
            raw_none.discounted_subtotal(subtotal),
            generated_none.discounted_subtotal(subtotal)
        );

        let percentage_subtotal = Decimal::new(10000, 2);
        let raw_percentage = crate::raw_baseline::pricing::discount_policy::DiscountPolicy::Percentage {
            rate: Decimal::new(10, 2),
        };
        let generated_percentage = crate::pricing::discount_policy::DiscountPolicy::Percentage {
            rate: Decimal::new(10, 2),
        };
        assert_eq!(
            raw_percentage.discount_amount(percentage_subtotal),
            generated_percentage.discount_amount(percentage_subtotal)
        );
        assert_eq!(
            raw_percentage.discounted_subtotal(percentage_subtotal),
            generated_percentage.discounted_subtotal(percentage_subtotal)
        );

        let fixed_subtotal = Decimal::new(5000, 2);
        let raw_fixed = crate::raw_baseline::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(1250, 2),
        };
        let generated_fixed = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(1250, 2),
        };
        assert_eq!(
            raw_fixed.discount_amount(fixed_subtotal),
            generated_fixed.discount_amount(fixed_subtotal)
        );
        assert_eq!(
            raw_fixed.discounted_subtotal(fixed_subtotal),
            generated_fixed.discounted_subtotal(fixed_subtotal)
        );

        let capped_subtotal = Decimal::new(1500, 2);
        let raw_capped = crate::raw_baseline::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };
        let generated_capped = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };
        assert_eq!(
            raw_capped.discount_amount(capped_subtotal),
            generated_capped.discount_amount(capped_subtotal)
        );
        assert_eq!(
            raw_capped.discounted_subtotal(capped_subtotal),
            generated_capped.discounted_subtotal(capped_subtotal)
        );

        let percentage_discounted =
            generated_percentage.discounted_subtotal(percentage_subtotal);
        let quote = CheckoutQuote::new(
            percentage_subtotal,
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );

        assert_eq!(percentage_discounted, Decimal::new(9000, 2));
        assert_eq!(percentage_discounted, quote.discounted_subtotal());
        assert_eq!(apply_tax(percentage_discounted, Decimal::new(725, 4)), quote.total());

        let fixed_policy = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(1250, 2),
        };
        let fixed_discounted = fixed_policy.discounted_subtotal(fixed_subtotal);
        let fixed_taxed = apply_tax(fixed_discounted, Decimal::new(725, 4));

        assert_eq!(fixed_discounted, Decimal::new(3750, 2));
        assert_eq!(fixed_taxed, Decimal::new(40218750, 6));
        assert!(fixed_taxed > fixed_discounted);
    }
