id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation to support tiered rates without losing checkout coverage."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
        - pricing/calculate_total
        - pricing/checkout_quote
      molecule_tests:
        - pricing/checkout_flow
        - pricing/discount_plus_tax
        - pricing/discount_policy_checkout_flow
      notes:
        - "current blast radius stays fully covered"
notes:
  - "M10 plans are local-library only."
