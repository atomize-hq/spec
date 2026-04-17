Summary: Implemented the M10 `spec-core` plan contract. Added a dedicated `spec_core::plan` module for authored `.plan.spec` types, schema validation, load support, action-sensitive `build_plan_report`, union-only public `computed_impact`, and plan export primitives. Added plan-specific `SPEC_PLAN_*` error variants in `spec-core` and covered parsing, duplicate change units, modify/remove/add semantics, unresolved add impact, missing molecule test targets, and dedicated plan export bundle behavior.

Files changed:
- `spec-core/src/lib.rs`
- `spec-core/src/loader.rs`
- `spec-core/src/export.rs`
- `spec-core/src/plan.rs`
- `spec-core/src/schema/plan.spec.json`

Commands run:
- `cargo test -p spec-core`

Result:
- Passed.

Blockers:
- None.
