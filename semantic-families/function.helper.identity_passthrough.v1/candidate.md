# function.helper.identity_passthrough.v1

Summary: Straight-line unary Decimal helper with no deps and either passthrough or round-like body.

This packet captures the existing `money/round` helper surface that runtime semantic review
already supports. The contract stays frozen and narrow: `fn_name == round`, one `Decimal` input,
`Decimal` return, no deps, no invariants, and no control flow in supported cases.

The packet is self-contained and ships all four required buckets. The aligned bucket proves both
honest supported lanes without widening the contract: one `round` fixture with round-like intent
and round-like body, and one `round` fixture with passthrough intent and direct-passthrough body.

## Aligned

- `fixtures/aligned/units/money/round.unit.spec`: round-like intent plus round-like body, matching the currently supported helper lane for a real rounding implementation.
- `fixtures/aligned/units/passthrough/round.unit.spec`: passthrough intent plus direct-passthrough body, proving the second supported aligned lane under the same frozen helper contract.

## Drift

- `fixtures/drift/units/money/round.unit.spec`: passthrough intent paired with a round-like body so the executable behavior contradicts the authored helper story while staying inside the helper-shaped contract.

## Under Specified

- `fixtures/under_specified/units/money/round.unit.spec`: keeps an otherwise-supported direct-passthrough body but weakens `intent.why` to `todo`, making the semantic surface too vague for honest alignment.

## Unsupported Near Miss

- `fixtures/unsupported_near_miss/units/money/round.unit.spec`: keeps round-like intent and an otherwise helper-shaped body, but wraps the rounding expression in explicit control flow so it stays a real near miss for this family instead of semantic drift.
