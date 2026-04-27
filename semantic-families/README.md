# Semantic Family Packets

M21 locks the repo-owned packet contract for promoted `kind:function` semantic families under
`semantic-families/`.

Packet rules for M21:

- `family.toml` is the executable manifest contract.
- `candidate.md` is review context only.
- fixtures are self-contained crate roots under `fixtures/<bucket>/`.
- `xtask` must treat packet fixtures as source of truth and reject symlinks or extra unit files.
- certification outputs live under `.semantic-family-artifacts/` and are never checked-in source.

Frozen exemplar packet for M21:

- `semantic-families/function.wrapper.pipeline.chain3.v1/family.toml`
- `semantic-families/function.wrapper.pipeline.chain3.v1/candidate.md`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/pricing_discount_leaf_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/pricing_tax_leaf_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/checkout_chain3_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/pricing_discount_leaf_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/pricing_tax_leaf_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/checkout_chain3_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/checkout_chain3_unsupported_near_miss.unit.spec`

Locked M21 precedence order:

1. `function.wrapper.pipeline.chain3.v1`
2. `function.wrapper.pipeline.v1`
3. `function.arithmetic_leaf.monotone_down_nonnegative.v1`
4. `function.arithmetic_leaf.monotone_up.v1`
5. `unsupported.function.v1`

Locked M21 semantic test prefixes:

- `m21_chain3_classifier_*`
- `m21_chain3_truth_surface_*`
- `m21_chain3_corpus_*`
- `m21_chain3_regression_*`
