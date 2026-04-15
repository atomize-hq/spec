Summary: `spec-core/src/export.rs` now treats `SpecGraph` as a read-only source and projects edges through `graph.edges()` instead of reading private internals. Export-side ordering now follows the graph contract, and the regression asserts the exact mixed `dep` and `covers` edge sequence without changing the export JSON shape.

Files changed:
- `spec-core/src/export.rs`

Integration note:
- Ready for WS-INT verification against the M8 graph changes.
