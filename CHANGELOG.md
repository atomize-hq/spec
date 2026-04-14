# Changelog

## 0.5.3 - 2026-04-13

### Fixed

- **Single-file CLI scope no longer breaks on sibling molecule tests** — `spec validate <file.unit.spec>`, `spec generate <file.unit.spec>`, and `spec export <file.unit.spec>` now stay scoped to the requested unit. Sibling `.test.spec` files are only loaded for directory invocations, preserving the exact-unit authoring loop used by agents and README workflows.

### Breaking

- **`spec status` non-valid units now exit `1`** — `untested`, `incomplete`, and `failing` units now produce a non-zero exit code alongside `invalid` and `stale`. Consumers that previously treated those states as soft-success need to update.
- **`stale: bool` removed from `spec status --format json` units** — Machine consumers should switch to the `status` and `reason` fields instead of reading a separate stale flag.
- **`spec status --format json` now emits `schema_version: 2`** — Parsers should gate on version `2` for the new health-state contract.

Migration: for `spec status`, treat any unit whose `status` is not `"valid"` as a failing health result, and replace any `stale: bool` logic with `status`/`reason` handling.

## 0.5.2 - 2026-04-12

### Changed

- **Default output path is now `{crate_root}/src/generated`** — `spec generate`, `spec build`, and `spec test` no longer require `--output`. When omitted, the output directory is derived from the crate root (via `spec.toml` or ancestor `Cargo.toml` walk) and defaults to `src/generated` inside that crate. Projects using the old default `generated/spec` can pass `--output generated/spec` explicitly to preserve prior behavior.
- **`generated_module_prefix` config key** — Add `[pipeline] generated_module_prefix = "custom::prefix"` to `spec.toml` for non-standard output layouts where auto-derivation produces a wrong module path.
- **`Verbosity` enum in pipeline API** — `run_cargo_build` and `run_cargo_test` now accept a `Verbosity` parameter. `Verbosity::Normal` preserves existing `spec: running cargo …` stderr output; `Verbosity::Silent` suppresses it (reserved for future `--format json` mode).

### Fixed

- **Module prefix evidence mismatch** — `spec test` was computing the generated module prefix twice: once for the cargo filter and once for evidence lookup, using different values. All tests showed `status: "unknown"` when the two derivations disagreed. Fixed by computing the effective prefix once and passing it to both sites.

## 0.5.1 - 2026-04-11

### Added

- **Pipeline timeout** — Configure `[pipeline] timeout_secs` in `spec.toml` to bound `spec build` and `spec test` execution time. Hung cargo processes are killed after the deadline; passports record `build_status: "timeout"` so downstream agents see a clean signal instead of a stale run.
- **Git provenance in passports and exports** — `spec test` now records the current `git_commit_sha` in passport evidence when run inside a git repository. `spec export` includes top-level provenance in the bundle. Passports from pre-provenance runs deserialize cleanly (field is optional, absent passports remain valid).
- **Concurrent write warning** — When multiple `spec` processes write passports at the same time (a risk in multi-agent CI), a warning is emitted to stderr. The guard is advisory (warn-only, no blocking lock), matching the M5 trust-not-lock design.

### Changed

- **Stable `SPEC_*` JSON error codes** — `spec validate --format json` and `spec status --format json` now emit stable, namespaced error codes (`SPEC_MISSING_DEP`, `SPEC_INVALID_CONTRACT_TYPE`, etc.) instead of bare CamelCase names. Machine consumers can write stable matchers against these codes. `AGENTS.md` updated to reflect the new contract.
- **`schema_version` is now a JSON integer** — Export bundles and JSON status/validate responses emit `"schema_version": 1` (integer) instead of `"schema_version": "1.0"` (string). Consumers that compare against the string `"1.0"` need to update to compare against the integer `1`.
- **Faster cargo test result lookups** — `parse_cargo_test_output` now returns a `HashMap` instead of `BTreeMap`, reducing evidence-building overhead for large test suites.

### Fixed

- **Timeout process tree** — After killing cargo on timeout, pipe reader threads are no longer joined. Grandchildren (rustc, test binaries) that inherit pipe write-ends no longer cause `spec` to hang past the configured timeout.

## 0.5.0 - 2026-04-06

### Added

- **`spec status [path]`** — New command showing per-unit validation, passport, and staleness status in both human-readable and `--format json` modes. AI agents read this to know what to work on.
- **`--format json` on `validate` and `status`** — Structured JSON output on stdout with `schema_version`, `status`, `errors[]`, and `warnings[]` fields. Each error carries a `path` field pointing directly to the source `.unit.spec` file. Loader errors surface in `status --format json` via a `loader_errors` field so AI agents receive a single parseable signal even when files are malformed.
- **`spec test [path]` single-unit scoping** — Pass a `.unit.spec` file path to `spec test` to run only that unit's cargo tests using a derived module path filter (`pricing::apply_tax::tests::`). New passport evidence is written per-run.
- **Contract hash in passports** — `spec test` now writes a `contract_hash` (SHA-256 of the serialized contract) to the passport. `spec status` compares the live contract hash against the stored hash to detect stale units — those where the contract changed but tests haven't been re-run.
- **AGENTS.md spec workflow** — Real agent workflow guide added to AGENTS.md: a 5-step validate → edit → build → test → check loop for AI coding agents working with spec units.
- **Companion gstack skill** — `.claude/skills/spec/SKILL.md` teaches any Claude Code session the spec workflow, common validation errors, and how to interpret passport evidence.
- **ICP definition** — Who spec v0.5 is for: a solo engineer or 2-5 person team using AI coding assistants daily where contract clarity and correctness matter. Written in DECISIONS.md.
- **Golden JSON fixture tests** — `spec-cli/tests/fixtures/` contains reference JSON outputs for `spec validate --format json` and `spec status --format json`. Shape breakage = test failure.

### Fixed

- **Zero-tests detection** — `spec test` now correctly detects when a filter matches 0 tests in a multi-binary crate (checks all binaries, returns true only when none ran matching tests). Previously would silently write evidence with empty test results.
- **JSON status loader errors** — `spec status --format json` no longer emits text diagnostics to stdout when loader errors occur; errors now appear in the JSON response's `loader_errors` field.
- **`status_command` zero-unit edge case** — `spec status` no longer incorrectly prints "0 units found" when loader errors are present.
- **JSON error field completeness** — Several `SpecError` variants (`RustKeyword`, `DepCollision`, `BodyRustMustBeBlock`, `LocalTestExpectNotExpr`) now correctly populate all JSON error fields instead of emitting `null` for known values.

### Migration

- **No authored unit format change** — `.unit.spec` authors should continue using `spec_version: "0.3.0"`.
- **Passport schema v3** — Passports may now include an optional `contract_hash` field (SHA-256, prefixed `sha256:`). Parsers should tolerate its absence; missing hash means "no stale detection available for this unit."

## 0.4.0 - 2026-04-05

### Added

- **Pipeline commands** — `spec build` now runs validate → generate → `cargo build`, and `spec test` runs the same pipeline followed by `cargo test`.
- **JSON export** — `spec export` emits a machine-readable bundle with units, passports, graph edges, and export warnings.

### Changed

- **Generated Rust doc comments** — `spec generate` now emits `///` doc comments from each unit's `intent.why` field above the generated function.
- **Passport runtime evidence** — `spec test` now records observed build/test results under an optional `evidence` field in co-located passports.

### Breaking

- **Passport schema v2** — Passport JSON may now include an optional `evidence` field containing locally observed runtime results.

### Migration

- **Passport evidence is additive** — No file migration is required. Parsers should tolerate absent `evidence` and treat it as "no runtime evidence available".
- **Authored unit format version remains `0.3.0`** — The crate release is `0.4.0`, but `.unit.spec` authors should continue using `spec_version: "0.3.0"` because the unit-file wire format did not change in this release.

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
