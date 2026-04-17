Summary: Implemented the M10 CLI surface on top of the new `spec-core` plan contract. Added `spec plan validate` and `spec plan export`, the plan-specific library-root resolver, JSON output parity for valid/invalid plan validation, dedicated plan export wiring, and mapping for all locked `SPEC_PLAN_*` machine codes.

Files changed:
- `spec-cli/src/commands.rs`
- `spec-cli/src/config.rs`

Commands run:
- `cargo check -p spec-cli`

Result:
- Passed.

Blockers:
- None.
