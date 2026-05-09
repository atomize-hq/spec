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

## M31 / M32 Boundary

- `M31` is the shared-core extraction and escape-hatch containment milestone.
  This README still documents promoted packet truth; it is not the place to
  imply wider portability than the code actually proves.
- `M32` is one bounded second-language promotion path for
  `function.arithmetic_leaf.monotone_up.v1`.
- `function.wrapper.pipeline.v1` remains promoted Rust-family truth plus
  regression pressure for the M32 pilot. It is not a second M32 certify target.
- TypeScript is explicit only inside the bounded
  `function.arithmetic_leaf.monotone_up.v1` pilot surfaces described by this
  repo state.

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

### Recommendation Analysis Status

M27.5 hardens recommendation-analysis so maintainers can distinguish visible
pressure from promotion-worthy pressure without hiding weak candidates.

Each recommendation candidate now exposes:

- `promotion_readiness`: `ready` or `hold`
- `hold_reasons`: zero or more of
  `unknown_overlap_family`, `hard_difficulty`,
  `thin_real_example_support`, `thin_regression_support`

Interpret the top-level recommendation statuses as follows:

- `ranked` means the first candidate is `ready` with `confidence.level`
  `medium` or `high`, so the output is claiming promotion-worthy next-family
  pressure
- `insufficient_real_corpus` means either no discoverable candidates exist, or
  every visible candidate is `hold` and every candidate still has
  `real_example_hits == 0`
- `no_strong_candidate` is an honest outcome when candidates are still visible,
  every current candidate is `hold`, and at least one candidate has some
  real-example pressure
- non-ranked outputs must not include any `ready` candidate

Held candidates are not errors. A candidate may remain visible in the output
with `promotion_readiness = "hold"` so maintainers can see where pressure is
forming without over-claiming that the next family is promotion-ready.

M33 keeps `recommendation_status` as that compatibility layer and adds a
maintainer-facing decision verdict in
`.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`:

- `recommended` means the current top candidate is promotion-worthy now.
- `blocked_for_now` means a plausible candidate exists, but blocker reasons or
  missing/stale evidence still hold it back.
- `not_recommended` means the visible pressure is real but should not drive the
  next family decision.

The current `money/round` helper-surface wedge is the narrow example of that
last state: it stays visible, but the M33 decision surface records it as
`not_recommended` under `helper_surface_not_promotable`. That does not widen
the bounded second-language claim beyond
`function.arithmetic_leaf.monotone_up.v1`.

M34 adds one bounded sibling decision artifact:
`.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`.
Produce it with:

- `cargo xtask family corpus-decision --format json`

That command reads the existing recommendation analysis artifact, validates it,
and emits one explicit next-step contract:

- `stop`
- `spend_corpus_run_1`
- `pivot_to_family_promotion_run`
- `pivot_to_recommendation_policy_run`
- `pivot_to_architecture_shared_core_follow_on`

For the current live helper-surface wedge, the emitted M34 decision is
`pivot_to_architecture_shared_core_follow_on` with
`decision_basis_code = "durable_non_promotable_helper_surface"` and
`required_next_action = "author_architecture_follow_on_plan"`.
That means corpus run `1` remains unspent. It does not mean M34 implemented the
shared-core follow-on.

At the frozen M35 boundary, the helper-surface pressure is still real, but the
non-promotability classification is owned by one shared helper-surface
classifier, not by a widened packet-local or `spec`-core decision path. The
recommendation analysis artifact remains the input truth for that
classification, while `corpus-program-decision.latest.json` remains the
bounded operator-action output that records what to do next with that input.

M36 preserved that frozen M35 wedge exactly. M37 keeps the same wedge outcome
while tightening the code boundary: helper-surface classification still lives
in `xtask/src/family/analysis_core/helper_surface.rs`, family-analysis decision truth now
lives in `xtask/src/family/decision_kernel.rs`, and
`helper_surface_not_promotable`,
`durable_non_promotable_helper_surface`,
`pivot_to_architecture_shared_core_follow_on`, and
`author_architecture_follow_on_plan` stay coupled without implying that corpus
run `1` was spent or that the helper surface became promotable.

M41 retires the visible unsupported-pressure version of that wedge without
promoting a new packet. The runtime semantic reviewer now supports the current
`money/round` helper shape under
`function.helper.identity_passthrough.v1`, the checked-in shared-spec passport
and read-side `status` / `export` surfaces preserve that supported truth, and
`cargo xtask family inventory --format json` now publishes the helper route as
runtime-supported but unpromoted.

That means the helper wedge is no longer counted as live unsupported pressure
by family analysis. `cargo xtask family coverage --format json` now moves the
three current helper units into `supported_unpromoted_family_units`, and
`cargo xtask family recommend --format json` no longer emits
`helper_surface_not_promotable` as a visible candidate hold. This is substrate
truth only; it does not create a new `semantic-families/function.*` packet.

Maintainers should also treat artifact identity semantically, not bytewise:
raw latest-artifact SHA is not semantic identity, and normalized semantic fingerprints are the proof surface.

### Corpus Source Kinds

The Rust function corpus uses explicit source kinds:

- `real_example`: maintained example units such as `examples/ecommerce/units`,
  `examples/shared-spec/units` (Maintained sibling-library optional-helper
  example.), and `examples/crosslib-app/units` (Maintained cross-library
  optional-helper app example.)
- `regression_unsupported`: repo regression packs such as the locked M19 and M20
  sources
- `proof_only`: semantic-family packet fixtures under
  `semantic-families/**/fixtures/**`

The locked M27 manifest contains exactly these five sources, in order:

- `examples/ecommerce/units`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`
- `examples/shared-spec/units`
- `examples/crosslib-app/units`

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

The promoted arithmetic leaf families already cover the helper-aware leaf shape with zero or one
helper dep. The packet-local `money/round` unit models that same optional-helper shape truth in
every bucket without depending on units outside the packet. `shared::money/round` and local
`money/round` are the same optional-helper shape, so the shared-spec and cross-library examples
align with the promoted monotone-down and monotone-up boundaries. Control-flow arithmetic
near-misses remain unsupported.

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
