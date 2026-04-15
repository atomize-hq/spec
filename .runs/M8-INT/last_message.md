Summary: Integrated the M8 graph, public-surface, and export packets and ran the full verification ladder. All targeted gates passed, followed by `cargo test -p spec-core` and `cargo test --all`.

Verification:
- `cargo test -p spec-core reverse_deps_`
- `cargo test -p spec-core tests_covering_`
- `cargo test -p spec-core impact_`
- `cargo test -p spec-core build_ignores_links_molecule_tests_legacy_metadata`
- `cargo test -p spec-core build_export_bundle_graph_edges_correct`
- `cargo test -p spec-cli spec_export_includes_graph_edges -- --exact`
- `cargo test -p spec-cli export_includes_molecule_tests_and_covers_edges -- --exact`
- `cargo test -p spec-cli single_file_export_skips_sibling_molecule_tests -- --exact`
- `cargo test -p spec-core`
- `cargo test --all`
