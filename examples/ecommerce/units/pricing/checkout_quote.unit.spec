id: pricing/checkout_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    discount_rate:
      type: rust_decimal::Decimal
    tax_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        discount_rate: rust_decimal::Decimal
        tax_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      discount_rate: discount_rate
      tax_rate: tax_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the discounted subtotal before tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    lowering:
      rust:
        body: |
          {
              let discounted = self.subtotal - self.subtotal * self.discount_rate;
              discounted.max(rust_decimal::Decimal::ZERO)
          }
  - id: total
    intent:
      why: Return the final checkout total after discount and tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    lowering:
      rust:
        body: |
          {
              let discounted = self.discounted_subtotal();
              discounted + discounted * self.tax_rate
          }
local_tests:
  - id: discounted_subtotal_basic
    expect: CheckoutQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(725, 4)).discounted_subtotal() == rust_decimal::Decimal::new(9000, 2)
  - id: total_basic
    expect: CheckoutQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(725, 4)).total() == rust_decimal::Decimal::new(96525, 3)
links:
  molecule_tests:
    - pricing/checkout_flow
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
