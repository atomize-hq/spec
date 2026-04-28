# Semantic Family Packets

M21 locks the repo-owned packet contract for promoted `kind:function` semantic families under
`semantic-families/`.

Packet rules for M21:

- `family.toml` is packet-local metadata and validation truth; it does not register or orchestrate family workflows.
- `candidate.md` is review context only.
- fixtures are self-contained crate roots under `fixtures/<bucket>/`.
- orchestration is registry-first in `xtask/src/family/harness.rs`; packet files alone do not enable `cargo xtask family new/prove/certify` for a new family id.
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

Registered M21 workflow example:

- `function.wrapper.pipeline.chain3.v1` is registered in `xtask/src/family/harness.rs`, so `cargo xtask family prove ...` and `cargo xtask family certify ...` can run for that id.
- unregistered ids such as `function.wrapper.pipeline.chain4.v1` must be added to the Rust harness registry before `cargo xtask family new/prove/certify` will succeed.

Packet-local locked routing metadata:

- `family.toml [routing]` and the matching Rust harness entry still define the selected packet's exact locked `precedence` and `must_not_shadow` values.
- Packet-local `must_not_shadow` may include unregistered legacy family ids. Those entries remain part of exact manifest-local equality for the selected family.

Registry-derived xtask routing order surface:

- xtask routing diagnostics and mismatch messages now surface the honest registered-family order only: registered harnesses sorted by precedence, then `unsupported.function.v1`.
- Registry-global coherence ignores unregistered non-terminal `must_not_shadow` entries.
- Registry-global coherence still requires registered-family successors to appear exactly once, in order, and keeps `unsupported.function.v1` terminal.

Artifact schema v3 note:

- `prove.latest.json` now uses `schema_version = 3`.
- In schema v3, `overall_status` and `phase_status` both reflect only the artifact's `required_gates`.
- A successful `prove.latest.json` may therefore show `overall_status = "pass"` while `gates.gate_d.status = "fail"`.

Locked M21 semantic test prefixes:

- `m21_chain3_classifier_*`
- `m21_chain3_truth_surface_*`
- `m21_chain3_corpus_*`
- `m21_chain3_regression_*`
