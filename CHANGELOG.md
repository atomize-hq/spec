# Changelog

## 0.2.1 - 2026-04-03

### Security

- **`is_safe_expect_expr` now recurses into sub-expressions** — Previously, the expression whitelist in `spec validate` only inspected the top-level AST node. A call like `f({ unsafe { ... } })` would pass because the outer `Call` was whitelisted without checking its arguments. All Arms (Binary, Call, MethodCall, Field, Index, Unary, Paren, Cast) now recurse into every sub-expression; `unsafe`, block, closure, and control-flow forms are rejected wherever they appear in the tree. Error message updated from "simple expression" framing to "block, unsafe, closure" framing to accurately describe what is and isn't blocked.

### Testing

- Added 4 regression tests covering recursion through Field, Index, Unary, and Cast arms.

## 0.2.0 - 2026-04-02

### Added
- **`imports` field** — specs can now declare external `use` statements directly (e.g., `imports: [rust_decimal::Decimal]`), which are emitted as `use` items in the generated file. Previously, all external types had to be brought into scope manually via the crate using the generated code.
- **`local_tests` codegen** — specs can embed inline test cases with an `expect` expression; `spec generate` now produces a `#[cfg(test)]` block with one `#[test]` function per entry. Block, unsafe, closure, and control-flow expressions are rejected to keep expect values safe to embed.
- **Body validation** — `body.rust` is now parsed with `syn` at validation time. `spec validate` reports errors for: wrong function name, multiple top-level items, `&self` receivers, and contract input names that don't match function parameters.
- **Ecommerce example** — `examples/ecommerce/` ships a working multi-unit spec set (money/round, pricing/apply_discount, pricing/apply_tax, pricing/calculate_total) with `cargo check` and `cargo test` verified in CI.
- **Cargo check integration test** — the CLI test suite runs `cargo check` and `cargo test` against the ecommerce example on every `cargo test --workspace`, ensuring generated Rust stays valid end-to-end.

### Changed
- **Output safety hardened** — `spec generate --output <dir>` now validates the output path before writing: rejects paths outside the project root (including symlink traversal), rejects non-empty directories missing the `.spec-generated` marker, and cleans orphaned `.rs` files from prior runs. Previously the output directory was trusted as-is.
- **Duplicate ID reporting** — `spec validate` now reports all files with duplicate IDs in a single pass instead of stopping at the first pair.

### Fixed
- `spec validate` and `spec generate` no longer silently pass specs with unresolved internal deps.

### Breaking
- **Strict dep validation** — `validate` and `generate` now exit 1 for specs with unresolved internal deps. Previously these passed silently. Ensure all deps are defined in the same spec set before upgrading. A `--no-strict` flag for partial-graph workflows is planned for M3.
