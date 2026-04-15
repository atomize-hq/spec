Summary: Added the crate-root re-exports for the frozen M8 graph surface in `spec-core/src/lib.rs`. The crate now publicly re-exports `SpecGraph`, `SpecEdge`, `UnitNode`, `MoleculeTestNode`, and `ImpactSet`.

Files changed:
- `spec-core/src/lib.rs`

Integration note:
- End-to-end compilation still depends on `WS-EXPORT` switching export projection from direct `graph.edges` field access to `graph.edges()`.
