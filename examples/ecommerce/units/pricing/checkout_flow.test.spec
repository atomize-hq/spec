id: pricing/checkout_flow
spec_version: "0.3.0"
intent:
  why: End-to-end checkout price computation — full discount, tax, and rounding chain.
covers:
  - pricing/apply_discount
  - pricing/calculate_total
  - pricing/pricing_quote
imports:
  - rust_decimal::Decimal
  - crate::pricing::apply_discount::apply_discount
  - crate::pricing::calculate_total::calculate_total
  - crate::pricing::pricing_quote::PricingQuote
body:
  rust: |
    {
        let quote = PricingQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );
        let total =
            calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4));
        let rounding_sensitive_quote = PricingQuote::new(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );
        let rounding_sensitive_total = calculate_total(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );

        assert_eq!(
            quote.discounted_subtotal(),
            apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
        );
        assert_eq!(quote.total(), total);
        assert!(quote.total() > Decimal::ZERO);
        assert_eq!(
            rounding_sensitive_quote.discounted_subtotal(),
            apply_discount(Decimal::new(1001, 2), Decimal::new(3333, 4))
        );
        assert_eq!(rounding_sensitive_quote.total(), rounding_sensitive_total);
        assert!(rounding_sensitive_quote.total() > Decimal::ZERO);
    }
