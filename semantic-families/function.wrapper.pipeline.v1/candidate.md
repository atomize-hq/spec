# function.wrapper.pipeline.v1

Summary: Straight-line two-call wrapper pipeline over supported semantic deps.

This packet isolates the dedicated two-step pricing wrapper shape: one monotone-down discount leaf,
one monotone-up tax leaf, and a wrapper that is supposed to apply discount first and tax second.
Each bucket stays packet-local and carries exactly those three pricing units, so the family does
not depend on the chain3 checkout extension or any other external helper.

## Aligned

- `fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`: packet-local monotone-down nonnegative leaf for the discount step.
- `fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`: packet-local monotone-up leaf for the tax step.
- `fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`: truthful dedicated wrapper that discounts first, then taxes the discounted subtotal.

## Drift

- `fixtures/drift/units/pricing/pricing_discount_leaf_drift.unit.spec`: same packet-local discount leaf shape as aligned.
- `fixtures/drift/units/pricing/pricing_tax_leaf_drift.unit.spec`: same packet-local tax leaf shape as aligned.
- `fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec`: authored as discount-then-tax, but the body reverses the order and taxes before discounting.

## Under Specified

- `fixtures/under_specified/units/pricing/pricing_discount_leaf_under_specified.unit.spec`: same packet-local discount leaf shape as aligned.
- `fixtures/under_specified/units/pricing/pricing_tax_leaf_under_specified.unit.spec`: same packet-local tax leaf shape as aligned.
- `fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec`: keeps the aligned body, but weakens the authored semantic surface to vague pricing truth.

## Unsupported Near Miss

- `fixtures/unsupported_near_miss/units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec`: same packet-local discount leaf shape as aligned.
- `fixtures/unsupported_near_miss/units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec`: same packet-local tax leaf shape as aligned.
- `fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec`: stays semantically close to the honest wrapper, but leaves the supported subset by threading `tax_rate.max(Decimal::ZERO)` into the tax call.
