Summary: Implemented the M8 graph contract in `spec-core/src/graph.rs`. `SpecGraph` now has private storage plus reverse-dependency and test-coverage indexes, public accessors and query methods, public `ImpactSet`, deterministic build ordering, the explicit validated-input / ignore-`links.molecule_tests` contract, and the required graph test matrix.

Files changed:
- `spec-core/src/graph.rs`

Focused verification:
- `cargo test -p spec-core --lib reverse_deps_returns_direct_dependents_sorted`
- Expected compile break is in `spec-core/src/export.rs`, which still reads the now-private `graph.edges` field directly.
