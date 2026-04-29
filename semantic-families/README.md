# Semantic Family Packets

M23 locks the repo-owned packet contract for promoted `kind:function` semantic families under
`semantic-families/`.

Packet rules for M23:

- `family.toml` is packet-local metadata and validation truth; it does not register or orchestrate family workflows.
- `candidate.md` is review context only.
- fixtures are self-contained crate roots under `fixtures/<bucket>/`.
- orchestration is registry-first in `xtask/src/family/harness.rs`; packet files alone do not enable `cargo xtask family new/smoke/prove/certify` for a new family id.
- `xtask` must treat packet fixtures as source of truth and reject symlinks or extra unit files.
- certification outputs live under `.semantic-family-artifacts/` and are never checked-in source.

Promoted packets:

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
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/candidate.md`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/drift/units/pricing/apply_discount_drift.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/under_specified/units/pricing/apply_discount_under_specified.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec`

The monotone-down packet also carries a packet-local `money/round` helper in every bucket. That
helper keeps the optional helper-dep shape truthful without depending on units outside the packet.

Registered workflow examples:

- `function.wrapper.pipeline.chain3.v1` is registered in `xtask/src/family/harness.rs`, so `cargo xtask family prove ...` and `cargo xtask family certify ...` can run for that id.
- `function.arithmetic_leaf.monotone_down_nonnegative.v1` is also registered, so the same maintainer workflow now lands a real leaf family packet.
- unregistered ids such as `function.wrapper.pipeline.chain4.v1` and `function.arithmetic_leaf.monotone_up.v1` must be added to the Rust harness registry before `cargo xtask family new/smoke/prove/certify` will succeed.

Maintainer smoke-loop note:

- `cargo xtask family new` scaffolds a truthful starter packet. It does not recreate the fully curated committed packet byte-for-byte.
- `cargo xtask family smoke <family-id>` reruns that scaffold logic in a temp workspace and checks only scaffold-owned surfaces.
- For `function.arithmetic_leaf.monotone_down_nonnegative.v1`, the stable smoke invariant is narrower: `family.toml` should regenerate byte-for-byte, the locked pricing starter cases should reappear in all four buckets, and the aligned starter spec should still read like the leaf family (`subtotal`, `rate`, nonnegative invariants, optional `money/round` helper dep).
- A whole-packet diff after deleting and regenerating the committed monotone-down packet is expected to be non-empty because the committed packet adds maintainer-authored rationale in `candidate.md`, packet-local helper units, bucket-local Cargo dependencies, and extra local proof beyond the starter scaffold.

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

Locked semantic test prefixes:

- `m21_chain3_classifier_*`
- `m21_chain3_truth_surface_*`
- `m21_chain3_corpus_*`
- `m21_chain3_regression_*`
- `monotone_down_nonnegative_classifier_*`
- `monotone_down_nonnegative_truth_surface_*`
- `monotone_down_nonnegative_corpus_*`
- `monotone_down_nonnegative_regression_*`
