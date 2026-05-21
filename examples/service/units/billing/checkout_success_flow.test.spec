id: billing/checkout_success_flow
spec_version: "0.3.0"
intent:
  why: End-to-end service checkout pricing for the approved membership discount and regional fee path.
covers:
  - billing/apply_membership_discount
  - billing/apply_regional_fee
  - billing/checkout_net_total
  - billing/pricing_quote
imports:
  - rust_decimal::Decimal
  - crate::billing::apply_membership_discount::apply_membership_discount
  - crate::billing::apply_regional_fee::apply_regional_fee
  - crate::billing::checkout_net_total::checkout_net_total
  - crate::billing::pricing_quote::PricingQuote
body:
  rust: |
    {
        let quote = PricingQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(5, 2),
        );
        let total = checkout_net_total(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(5, 2),
        );

        assert_eq!(
            quote.discounted_subtotal(),
            apply_membership_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
        );
        assert_eq!(quote.total(), total);
        assert_eq!(
            apply_regional_fee(quote.discounted_subtotal(), Decimal::new(5, 2)),
            total
        );
        assert!(quote.total() > quote.discounted_subtotal());
    }
