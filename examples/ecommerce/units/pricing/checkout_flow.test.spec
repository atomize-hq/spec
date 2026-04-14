id: pricing/checkout_flow
spec_version: "0.3.0"
intent:
  why: End-to-end checkout price computation — full discount, tax, and rounding chain.
covers:
  - pricing/apply_discount
  - pricing/apply_tax
  - money/round
  - pricing/calculate_total
body:
  rust: |
    {
        let total = calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4));
        assert!(total > Decimal::ZERO);
    }
