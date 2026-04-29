# function.arithmetic_leaf.monotone_up.v1

Summary: Straight-line arithmetic leaf with zero-or-one helper dep and monotone-up semantics.

This packet is the packetized form of the existing `pricing/apply_tax` wedge. It stays narrow:
one straight-line arithmetic leaf, at most one helper dep, and the specific "add tax to subtotal"
semantics that already ship in the ecommerce example and M14 regression coverage.

Each bucket keeps its locked starter case path and uses a packet-local `money/round` helper with
the same local unit id. The helper is intentionally semantically boring: it exists only so this
packet proves the optional helper-dep shape without depending on units outside the packet.

## Aligned

- `fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec`: truthful lift of `examples/ecommerce/units/pricing/apply_tax.unit.spec`. The body adds `subtotal * rate` to the subtotal and routes through the optional helper dep.
- `fixtures/aligned/units/money/round_aligned.unit.spec`: packet-local helper that preserves the optional helper-dep shape without adding unrelated rounding behavior.

## Drift

- `fixtures/drift/units/pricing/apply_tax_drift.unit.spec`: reuses the existing M14 drift rewrite. The authored story still says tax increases the subtotal, while the executable body subtracts and clamps instead.
- `fixtures/drift/units/money/round_drift.unit.spec`: same helper shape as aligned so the bucket isolates semantic drift to the leaf body rather than to dependency topology.

## Under Specified

- `fixtures/under_specified/units/pricing/apply_tax_under_specified.unit.spec`: keeps the aligned executable body but weakens `intent.why` to the exact vague-truth wedge (`todo`). This keeps the executable behavior truthful while the authored semantic surface is too weak for honest evaluation.
- `fixtures/under_specified/units/money/round_under_specified.unit.spec`: same helper shape as aligned so the bucket isolates the under-specification to authored intent rather than packet topology.

## Unsupported Near Miss

- `fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec`: locked control-flow near miss from the M24 plan. It preserves the monotone-up tax semantics but uses an explicit `if rate == Decimal::ZERO` branch, making it a real near miss for this family instead of a semantic drift duplicate.
- `fixtures/unsupported_near_miss/units/money/round_unsupported_near_miss.unit.spec`: same helper shape as the supported buckets so the only unsupported surface is the control-flow branch in the leaf body.
