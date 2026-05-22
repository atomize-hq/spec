id: billing/discount_strategy_quote_flow
spec_version: "0.3.0"
intent:
  why: Prove that the service discount seam stays aligned with the quote and fee flow for supported discount strategies.
covers:
  - billing/discount_strategy
  - billing/pricing_quote
  - billing/apply_regional_fee
imports:
  - rust_decimal::Decimal
  - crate::billing::apply_regional_fee::apply_regional_fee
  - crate::billing::pricing_quote::PricingQuote
body:
  rust: |
    {
        let subtotal = Decimal::new(10000, 2);
        let raw_declined = crate::raw_baseline::billing::discount_strategy::DiscountStrategy::Declined;
        let generated_declined = crate::billing::discount_strategy::DiscountStrategy::Declined;
        assert_eq!(raw_declined.discount_amount(subtotal), generated_declined.discount_amount(subtotal));
        assert_eq!(
            raw_declined.discounted_subtotal(subtotal),
            generated_declined.discounted_subtotal(subtotal)
        );

        let raw_percentage =
            crate::raw_baseline::billing::discount_strategy::DiscountStrategy::Percentage {
                rate: Decimal::new(10, 2),
            };
        let generated_percentage =
            crate::billing::discount_strategy::DiscountStrategy::Percentage {
                rate: Decimal::new(10, 2),
            };
        assert_eq!(
            raw_percentage.discount_amount(subtotal),
            generated_percentage.discount_amount(subtotal)
        );
        assert_eq!(
            raw_percentage.discounted_subtotal(subtotal),
            generated_percentage.discounted_subtotal(subtotal)
        );

        let raw_fixed =
            crate::raw_baseline::billing::discount_strategy::DiscountStrategy::FixedCredit {
                amount: Decimal::new(1250, 2),
            };
        let generated_fixed =
            crate::billing::discount_strategy::DiscountStrategy::FixedCredit {
                amount: Decimal::new(1250, 2),
            };
        assert_eq!(
            raw_fixed.discount_amount(Decimal::new(5000, 2)),
            generated_fixed.discount_amount(Decimal::new(5000, 2))
        );
        assert_eq!(
            raw_fixed.discounted_subtotal(Decimal::new(5000, 2)),
            generated_fixed.discounted_subtotal(Decimal::new(5000, 2))
        );

        let quote = PricingQuote::new(
            subtotal,
            Decimal::new(10, 2),
            Decimal::new(5, 2),
        );
        let percentage_discounted = generated_percentage.discounted_subtotal(subtotal);

        assert_eq!(percentage_discounted, Decimal::new(9000, 2));
        assert_eq!(percentage_discounted, quote.discounted_subtotal());
        assert_eq!(apply_regional_fee(percentage_discounted, Decimal::new(5, 2)), quote.total());
        assert_eq!(
            generated_fixed.discounted_subtotal(Decimal::new(5000, 2)),
            Decimal::new(3750, 2)
        );
    }
