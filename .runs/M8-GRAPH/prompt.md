Task: M8-GRAPH

Ownership
- You own `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs` only.
- You are not alone in the codebase. Do not revert or overwrite edits from others.

Scope
- Implement the locked M8 graph contract from `PLAN.md` only.
- Make `SpecGraph` fields private.
- Add private `rev_dep_index` and `test_coverage_index`.
- Add public `ImpactSet`.
- Add `units()`, `molecule_tests()`, `edges()`, `reverse_deps()`, `tests_covering()`, `impact()`.
- Keep `build()` infallible and document that it assumes validated input and explicitly ignores `links.molecule_tests`.
- Add the required graph.rs unit tests:
  - `reverse_deps_returns_direct_dependents_sorted`
  - `reverse_deps_unknown_unit_returns_none`
  - `tests_covering_returns_multiple_tests_sorted`
  - `tests_covering_unknown_unit_returns_none`
  - `impact_includes_seed_reverse_dep_closure_and_covering_tests`
  - `impact_includes_downstream_covering_tests_not_just_seed_tests`
  - `impact_deduplicates_diamond_reverse_deps`
  - `build_ignores_links_molecule_tests_legacy_metadata`

Out of scope
- Do not edit `spec-core/src/lib.rs`.
- Do not edit `spec-core/src/export.rs`.
- Do not edit `types.rs`, validator, loader, CLI, docs, or plan files.
- Do not widen scope into M9 or M10.

Acceptance
- `spec-core/src/graph.rs` compiles against the current repo.
- API matches the locked M8 signatures.
- Ordering is deterministic.
- Unknown-ID semantics match the plan.
- Legacy `links.molecule_tests` is ignored by graph construction and proven by test.

Final response
- Summarize exactly what changed.
- List the file path you edited.
