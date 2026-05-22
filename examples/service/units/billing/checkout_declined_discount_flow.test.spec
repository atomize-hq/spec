id: billing/checkout_declined_discount_flow
spec_version: "0.3.0"
intent:
  why: Prove the declined-discount service flow stays aligned when guarded fees clamp negative values to zero.
covers:
  - billing/checkout_net_total_guarded_fee
  - billing/discount_strategy
  - billing/pricing_quote
imports:
  - rust_decimal::Decimal
  - crate::billing::checkout_net_total_guarded_fee::checkout_net_total_guarded_fee
  - crate::billing::pricing_quote::PricingQuote
body:
  rust: |
    {
        let subtotal = Decimal::new(10000, 2);
        let declined = crate::billing::discount_strategy::DiscountStrategy::Declined;
        let quote = PricingQuote::new(subtotal, Decimal::ZERO, Decimal::ZERO);
        let guarded_total =
            checkout_net_total_guarded_fee(subtotal, Decimal::ZERO, Decimal::new(-5, 2));

        assert_eq!(declined.discount_amount(subtotal), Decimal::ZERO);
        assert_eq!(declined.discounted_subtotal(subtotal), subtotal);
        assert_eq!(quote.discounted_subtotal(), subtotal);
        assert_eq!(quote.total(), subtotal);
        assert_eq!(guarded_total, subtotal);
        assert_eq!(guarded_total, quote.total());
    }
