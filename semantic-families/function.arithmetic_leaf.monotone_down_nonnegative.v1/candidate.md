# function.arithmetic_leaf.monotone_down_nonnegative.v1

Summary: Straight-line arithmetic leaf with zero-or-one helper dep and nonnegative clamp semantics.

This packet is the packetized form of the existing `pricing/apply_discount` wedge. It stays narrow:
one straight-line arithmetic leaf, at most one helper dep, and the specific "discount then clamp at
zero" semantics that already ship in the ecommerce example and M14 regression coverage.

Each bucket keeps its locked starter case path and uses a packet-local `money/round` helper with
the same local unit id. The helper is intentionally semantically boring: it exists only so this
packet proves the optional helper-dep shape without depending on units outside the packet.

## Aligned

- `fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec`: truthful lift of `examples/ecommerce/units/pricing/apply_discount.unit.spec`. The body subtracts `subtotal * rate`, clamps at zero, and routes through the optional helper dep.
- `fixtures/aligned/units/money/round_aligned.unit.spec`: packet-local helper that preserves the optional helper-dep shape without adding unrelated behavior. This keeps the aligned case honest to the canonical `apply_discount` wiring.

## Drift

- `fixtures/drift/units/pricing/apply_discount_drift.unit.spec`: reuses the aligned authored truth but swaps in the exact M14 surcharge-style rewrite. The authored story still says "discount", while the executable body increases the subtotal instead.
- `fixtures/drift/units/money/round_drift.unit.spec`: same helper shape as aligned so the bucket isolates semantic drift to the leaf body rather than to dependency topology.

## Under Specified

- `fixtures/under_specified/units/pricing/apply_discount_under_specified.unit.spec`: keeps the aligned executable body but weakens `intent.why` to the exact M14 vague-truth wedge (`todo`). This makes the executable behavior truthful while the authored semantic surface is too weak for honest evaluation.
- `fixtures/under_specified/units/money/round_under_specified.unit.spec`: same helper shape as aligned so the bucket isolates the under-specification to authored intent rather than packet topology.

## Unsupported Near Miss

- `fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec`: seeded from `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_discount_control_flow.unit.spec`. It preserves the discount semantics but uses explicit branching for the clamp, which makes it a real near miss for this family rather than generic filler.
- `fixtures/unsupported_near_miss/units/money/round_unsupported_near_miss.unit.spec`: same helper shape as the supported buckets so the only unsupported surface is the branch-based clamp in the leaf body.
