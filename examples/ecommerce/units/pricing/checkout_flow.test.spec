id: pricing/checkout_flow
spec_version: "0.3.0"
intent:
  why: End-to-end checkout price computation — full discount, tax, and rounding chain.
covers:
  - pricing/apply_discount
  - pricing/calculate_total
  - pricing/checkout_quote
body:
  rust: |
    {
        let quote = CheckoutQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );
        let total = calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4));

        assert_eq!(
            quote.discounted_subtotal(),
            apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
        );
        assert_eq!(quote.total(), total);
        assert!(quote.total() > Decimal::ZERO);
    }
