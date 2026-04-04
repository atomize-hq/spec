# Changelog

## 0.3.0 - 2026-04-04

### Added

- **Passport generation** — `spec generate` now emits a `.spec.passport.json` file co-located with each `.unit.spec` source file. Passports are static knowledge artifacts containing the unit's id, intent, contract, deps, local tests, and generation timestamp. They are written atomically only after all Rust code generation succeeds, and gitignored automatically via an appended `**/*.spec.passport.json` entry.
- **`spec_version` field** — Units can now declare `spec_version: "0.3.0"` to indicate which format version they were authored for. `spec validate` and `spec generate` emit a `MissingSpecVersion` warning for units without this field, guiding authors to add it.
- **Cycle detection** — `spec validate` and `spec generate` now detect circular dependencies in the dep graph using DFS. A cycle like `A → B → A` is reported as `❌ cycle detected: A → B → A` and blocks generation.
- **Contract type validation** — `contract.inputs` values and `contract.returns` are now validated as syntactically valid Rust types using `syn`. Invalid types (e.g., `Vec<`) are caught at `spec validate` time. Parameter names (keys) are validated as valid Rust identifiers, catching reserved keywords like `type` or hyphenated names like `bad-name` before they reach codegen.
- **CUE trigger conditions** — DECISIONS.md now documents the explicit conditions under which CUE adoption is warranted, preventing indefinite deferral.

### Changed

- **`body.rust` is now a block expression** — The function body is now specified as a Rust block expression (`{ ... }`, braces included) rather than a complete function declaration. `spec generate` synthesizes the `pub fn` signature from `contract.inputs` and `contract.returns`. This eliminates fn name drift and makes contracts the authoritative source of the function's interface.
- **`contract.inputs` uses ordered map** — Input parameters now preserve YAML declaration order in generated code, using `IndexMap` instead of `HashMap`.
- **`spec generate <file.unit.spec>`** — Single-file generate now correctly writes `.gitignore` to the spec file's parent directory instead of failing with a path error.

### Breaking

- **`body.rust` format** — Units authored for 0.2.x with a full `pub fn` declaration will fail `spec validate` with a migration error. Strip the `pub fn name(params) -> ReturnType` line, keep only the `{ ... }` block, and move parameters into `contract.inputs` and return type into `contract.returns`. See the migration guide in README.md.

## 0.2.2 - 2026-04-03

### Added

- **`--no-strict` flag for `spec validate`** — Downgrades missing-dep errors to warnings and exits 0. Useful for partial-graph workflows where not all deps are present in the local spec set. `spec generate` explicitly rejects `--no-strict` with a helpful error.
- **`spec.toml` workspace config** — Supports `[validation] allow_unsafe_local_test_expect = true` to permit block, unsafe, closure, and other complex Rust expressions in `local_tests[].expect` for trusted environments. Config is discovered by walking ancestors from the target path (same convention as `.gitignore`).
- **`SpecWarning` type** — New non-fatal diagnostic type. Currently emitted for: symlink cycles skipped during directory traversal (`SymlinkCycleSkipped`) and missing deps in non-strict mode (`MissingDep`). Warnings print to stderr and appear in the success message count.

### Fixed

- **Symlink cycle handling** — Directory traversal no longer errors on symlink cycles. Cycles emit a `SymlinkCycleSkipped` warning and traversal continues with the rest of the tree. Previously, a cycle caused `spec validate` and `spec generate` to hard-fail.
- **`safe_output_path` consolidation** — `clean_output_dir` and `ensure_output_marker` previously used divergent path-containment logic (`normalized_absolute_path` lexical vs `canonicalize` symlink-following). Both now use a single `safe_output_path` utility that canonicalizes existing ancestors and rejects paths outside the project root.
- **`local_tests[].id` uniqueness** — Duplicate IDs within a single unit's `local_tests` are now caught at validation time. Previously, duplicate IDs would silently generate duplicate `fn test_{id}()` functions and cause a Rust compile error downstream.
- **`BodyRustSingleItemNotFn` error** — When `body.rust` contains exactly one top-level item that is not a function, the error now says "found 1 item (not a function)" instead of the misleading "found 0 items".

### Internal

- `load_directory_report` promoted from `pub(crate)` test helper to public API. Returns `DirectoryLoadReport` with `specs`, `errors`, `warnings`, and `total_files`.
- `validate_full`, `validate_semantic`, and `validate_deps_exist` each now have `_with_options` variants accepting `ValidationOptions`. The originals are kept as strict-mode convenience wrappers.

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
