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

## M27 Corpus Analysis

M27 adds two maintainer-facing analysis commands for the Rust `kind:function` lane:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`

Both commands load the authored corpus manifest at
`semantic-families/corpus/rust-function.toml`. `family coverage` evaluates the
current manifest sources and writes
`.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`.
`family recommend` recomputes that coverage in-process first, then writes
`.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`.

The new M27 outputs live only under the `analysis/` artifact directory. They do
not overwrite the M26 approval-gated root artifact at
`.semantic-family-artifacts/family-promotion/recommendation.latest.json`.

### Corpus Source Kinds

The Rust function corpus uses explicit source kinds:

- `real_example`: maintained example units such as `examples/ecommerce/units`
- `regression_unsupported`: repo regression packs such as the locked M19 and M20
  sources
- `proof_only`: semantic-family packet fixtures under
  `semantic-families/**/fixtures/**`

The locked M27 manifest contains exactly these three sources:

- `examples/ecommerce/units`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`

Packet fixtures are `proof_only`. They remain useful for packet certification,
but they are excluded from the M27 manifest and never act as recommendation
input.

### Bucket Leverage Rules

M27 derives each unit's bucket from its filename:

- `*_unsupported_near_miss.unit.spec` -> `unsupported_near_miss`
- `*_under_specified.unit.spec` -> `under_specified`
- `*_drift.unit.spec` -> `drift`
- everything else -> `aligned_or_real`

Recommendation leverage depends on both source kind and bucket:

- `real_example`: all function units count toward leverage
- `regression_unsupported`: `unsupported_near_miss` does not add leverage, while
  `drift`, `under_specified`, and `aligned_or_real` can add leverage
- `proof_only`: never counts toward leverage

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
- `semantic-families/function.wrapper.pipeline.v1/candidate.md`
- `semantic-families/function.wrapper.pipeline.v1/family.toml`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/aligned/Cargo.toml`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/aligned/src/main.rs`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/drift/Cargo.toml`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/drift/src/main.rs`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/drift/units/pricing/pricing_discount_leaf_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/drift/units/pricing/pricing_tax_leaf_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/under_specified/Cargo.toml`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/under_specified/src/main.rs`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/under_specified/units/pricing/pricing_discount_leaf_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/under_specified/units/pricing/pricing_tax_leaf_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/Cargo.toml`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/src/main.rs`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/candidate.md`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/drift/units/pricing/apply_discount_drift.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/under_specified/units/pricing/apply_discount_under_specified.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/candidate.md`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/drift/units/pricing/apply_tax_drift.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/under_specified/units/pricing/apply_tax_under_specified.unit.spec`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec`

The monotone-down packet also carries a packet-local `money/round` helper in every bucket. That
helper keeps the optional helper-dep shape truthful without depending on units outside the packet.
The monotone-up packet follows the same packet-local helper pattern for the canonical `apply_tax`
family.

Registered workflow examples:

- `function.wrapper.pipeline.chain3.v1` is registered in `xtask/src/family/harness.rs`, so `cargo xtask family prove ...` and `cargo xtask family certify ...` can run for that id.
- `function.wrapper.pipeline.v1` is also registered and promoted, so the same registry-first workflow now lands the dedicated two-step wrapper packet between chain3 and the arithmetic leaves.
- `function.arithmetic_leaf.monotone_down_nonnegative.v1` is also registered, so the same maintainer workflow now lands a real leaf family packet.
- `function.arithmetic_leaf.monotone_up.v1` is also registered and promoted, so the same registry-first workflow now lands the direct sibling arithmetic leaf packet for `pricing/apply_tax`.
- unregistered ids such as `function.wrapper.pipeline.chain4.v1` must still be added to the Rust harness registry before `cargo xtask family new/smoke/prove/certify` will succeed.

Maintainer smoke-loop note:

- `cargo xtask family new` scaffolds a truthful starter packet. It does not recreate the fully curated committed packet byte-for-byte.
- `cargo xtask family smoke <family-id>` reruns that scaffold logic in a temp workspace and checks only scaffold-owned surfaces.
- For `function.wrapper.pipeline.v1`, the stable smoke invariant is the dedicated wrapper sibling of chain3: `family.toml` should regenerate byte-for-byte, the locked packet-local wrapper cases should reappear in all four buckets, and the aligned starter spec should still read like the two-step discount-then-tax wrapper family.
- For `function.arithmetic_leaf.monotone_down_nonnegative.v1`, the stable smoke invariant is narrower: `family.toml` should regenerate byte-for-byte, the locked pricing starter cases should reappear in all four buckets, and the aligned starter spec should still read like the leaf family (`subtotal`, `rate`, nonnegative invariants, optional `money/round` helper dep).
- For `function.arithmetic_leaf.monotone_up.v1`, the stable smoke invariant is the sibling version of that same contract: `family.toml` should regenerate byte-for-byte, the locked pricing starter cases should reappear in all four buckets, and the aligned starter spec should still read like the tax family (`subtotal`, `rate`, `output >= subtotal`, optional `money/round` helper dep).
- A whole-packet diff after deleting and regenerating the committed monotone-down packet is expected to be non-empty because the committed packet adds maintainer-authored rationale in `candidate.md`, packet-local helper units, bucket-local Cargo dependencies, and extra local proof beyond the starter scaffold.
- The same whole-packet caveat now applies to the committed monotone-up packet because its committed form also adds maintainer-authored rationale, packet-local helper units, and bucket-local Cargo dependencies beyond the starter scaffold.

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
- `wrapper_pipeline_classifier_*`
- `wrapper_pipeline_truth_surface_*`
- `wrapper_pipeline_corpus_*`
- `wrapper_pipeline_regression_*`
- `monotone_down_nonnegative_classifier_*`
- `monotone_down_nonnegative_truth_surface_*`
- `monotone_down_nonnegative_corpus_*`
- `monotone_down_nonnegative_regression_*`
- `monotone_up_classifier_*`
- `monotone_up_truth_surface_*`
- `monotone_up_corpus_*`
- `monotone_up_regression_*`
