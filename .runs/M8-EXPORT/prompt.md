Task: M8-EXPORT

Ownership
- You own `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs` only.
- You are not alone in the codebase. Do not revert or overwrite edits from others.

Dependency
- Start only after the graph API is frozen by M8-GRAPH.

Scope
- Project export edges through `graph.edges()` instead of graph internals.
- Keep export as a projection layer.
- Add or update the export regression so mixed `dep` and `covers` edges are asserted in exact sorted order.

Out of scope
- Do not edit any other file.
- Do not change export JSON schema.

Final response
- Summarize exactly what changed.
- List the file path you edited.
