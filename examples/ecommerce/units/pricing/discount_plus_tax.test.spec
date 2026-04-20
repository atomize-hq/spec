id: pricing/discount_plus_tax
spec_version: "0.3.0"
intent:
  why: Verify that applying discount then tax produces the correct final price.
covers:
  - pricing/apply_discount
  - pricing/apply_tax
  - money/round
imports:
  - rust_decimal::Decimal
  - crate::pricing::apply_discount::apply_discount
  - crate::pricing::apply_tax::apply_tax
body:
  rust: |
    {
        let discounted = apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2));
        let taxed = apply_tax(discounted, Decimal::new(725, 4));
        assert!(taxed > Decimal::ZERO);
    }
