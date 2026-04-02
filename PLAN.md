# Release 0.2: Harden the Loop

**Generated**: 2026-04-02  
**Status**: Planning (Eng Review complete)  
**Preceded by**: `.implemented/PLAN-M1-release-0.1.md`  
**Third-party review**: Static review of M1 workspace and specs, 2026-04-02

---

## Thesis

M1 proved the spine: `load → validate → normalize → generate`. The workflow is real.

M2 does not broaden the surface. It makes the loop safe, compilable, and semantically
anchored. The bar for calling M2 done:

- Generating into an unsafe path is rejected before any writes
- Deleting or renaming a unit leaves no stale generated Rust behind
- A mismatched `id` and `body.rust` function name fails validation
- Unresolved internal deps are errors by default (escapable via `--no-strict`)
- External/native imports have an explicit modeled path
- The ecommerce example generates, wires into the crate module tree, and passes `cargo check`
- At least one `local_tests` path executes for real (not just compiles)

---

## Deliverables (sequenced)

### D1 — Output ownership hardening

The highest-priority issue from the third-party review. Right now `ensure_output_marker()`
writes a `.spec-generated` marker before safety checks in `clean_output_dir()` run, and
cleaning is scoped only to the current run's module paths — so stale files from
deleted/renamed units accumulate.

**Model:** owned subtree. The output directory is fully spec-owned. Per-file atomicity:
write each `.rs` to a temp path in the same directory (same filesystem), then `fs::rename`
into place (POSIX atomic per-file). Temp files must be in the same directory as the output
file to avoid EXDEV cross-filesystem rename failures. Use `tempfile::Builder::new()` with
`.prefix(".spec-tmp-").suffix(".rs").tempfile_in(output_dir)`.
After all files are written, scan the output tree for `.rs` files not in the generated
set and delete them. Then remove any empty directories in the output tree (bottom-up walk).
This is the orphan cleanup that replaces the module-scoped clean. "Fully spec-owned" means
no stale files AND no stale empty directories remain.

**Bootstrap safety guard (Eng Review addition):** `ensure_output_marker` must validate
the output path BEFORE creating anything. Reject if:
1. The path is outside the project root (same check already in `clean_output_dir`)
2. The path is non-empty AND has no `.spec-generated` marker (would claim a live directory)

This closes the `spec generate --output src` footgun.

Acceptance:
- `spec generate --output src/` errors before writing anything (non-empty dir, no marker)
- `spec generate --output /tmp/x` errors (outside project root)
- Output path is validated before any filesystem write
- `spec generate` on a spec set that previously had `test/foo` leaves no `test/` behind
  (orphan cleanup, not module-scoped clean)
- ISSUE-002 and ISSUE-004 closed as resolved-by-design

Files: `spec-core/src/generator.rs`, `spec-cli/src/commands.rs`

Tests to add:
- `clean_output_dir_removes_stale_module_from_prior_run` (unit test in generator.rs)
- `generate_rejects_non_empty_dir_without_marker` (CLI integration test)
- `generate_rejects_path_outside_project_root` (CLI integration test)

---

### D2 — Dependency/import model split

Right now `deps` handles everything: internal spec-to-spec calls AND native types like
`Decimal`. These are different things. Without the distinction, generated code looks
readable but isn't fully grounded, and `cargo check` can't pass.

**Decision (locked in Eng Review):** Two separate fields.

```yaml
deps:
  - money/round          # internal: becomes use crate::money::round::round;

imports:
  - rust_decimal::Decimal  # external: becomes use rust_decimal::Decimal;
```

The `imports` field uses a different schema pattern (allows `::` separator, e.g.
`rust_decimal::Decimal`, `std::collections::HashMap`). Schema pattern:
`^[a-zA-Z_][a-zA-Z0-9_]*(::([a-zA-Z_][a-zA-Z0-9_]*))+$` — requires at least one
`::` segment (rejects bare `Decimal`, accepts `rust_decimal::Decimal`).

The `deps` field keeps its existing pattern unchanged. Update `unit.spec.json`,
`SpecStruct`, `ResolvedSpec` (add `imports: Vec<String>` field), and the generator.

**Generated use statement ordering (locked in CEO review 2026-04-02):** imports first
(external), then deps (internal). Matches rustfmt convention (external crates before
`crate::` paths). A blank line separates the two groups when both are present:
```
use rust_decimal::Decimal;

use crate::money::round::round;
```

Acceptance:
- `apply_discount.unit.spec` can declare `Decimal` in `imports` without it being treated as an internal unit
- Generated `use` statements are correct for both kinds
- Schema updated and validated
- `imports: [invalid path]` fails schema validation
- `deps: [money/round]` still generates `use crate::money::round::round;`

Tests to add:
- `imports_field_validates_rust_path` (unit test in validator.rs)
- `imports_field_generates_correct_use_statement` (unit test in generator.rs)

---

### D3 — Rust body parsing and partial semantic alignment (`syn`)

The biggest semantic gap in M1: the spec says `id: pricing/apply_discount` but there
is no enforcement that `body.rust` contains a function named `apply_discount`. They can
drift silently.

**Scope clarification (CEO review 2026-04-02):** D3 is partial semantic alignment, not
complete. It validates fn name and parameter names. It does NOT validate parameter types,
return type, arity, async, generics, or refs/mutability. Those are M3 concerns when
contract type validation lands. The section header is intentionally scoped: "name and
arg alignment," not full signature enforcement.

**Scope (expanded in Eng Review to include arg alignment):** Two passes:

**Pass 1 — fn name:**
- Parse `body.rust` as exactly one `ItemFn`
- Require `fn_name == last id segment`
- Reject extra sibling items (multiple fns, structs, impls)

**Pass 2 — arg alignment against contract.inputs (M2 scope):**
- If `contract.inputs` is present, verify that each key in the map appears as a parameter
  name in the parsed function signature
- Error on mismatch: `contract.inputs has 'subtotal' but body.rust has 'price'`

Acceptance:
- A spec with `id: pricing/apply_discount` and `body.rust: pub fn wrong_name() {}` fails validation with a clear error
- A spec with two function definitions in `body.rust` fails validation
- A spec with `contract.inputs: {subtotal: Decimal}` and body `pub fn apply_discount(price: Decimal)` fails validation
- A spec with `body.rust: pub fn apply_discount(&self, subtotal: Decimal)` fails validation: "body.rust must be a free function (no self parameter)"
- A spec with `body.rust: #[allow(dead_code)] pub fn apply_discount() {}` passes (attributes on the fn are accepted)
- All existing ecommerce specs pass

Crate to add: `syn` (with `full` feature flag) in `[dependencies]` of spec-core (runtime, not dev-dep)

Tests to add:
- `validate_body_fn_name_mismatch` (unit test in validator.rs)
- `validate_body_multiple_fns_rejected` (unit test in validator.rs)
- `validate_contract_arg_name_mismatch` (unit test in validator.rs)

---

### D4 — Cargo check proof

The loop is not honest until we can prove generated output compiles. This deliverable
makes that explicit and repeatable.

- Add `money/round.unit.spec` to the ecommerce example so `apply_discount` and `apply_tax`
  resolve to real units
- Add `imports: [rust_decimal::Decimal]` to all ecommerce specs that use `Decimal` (depends on D2)
- **Wire generated output into the ecommerce crate** (Eng Review addition):
  add `mod generated;` to `examples/ecommerce/src/main.rs`. Generate output goes under
  `examples/ecommerce/src/generated` (NOT `src/generated/spec`) so that `mod generated;`
  in main.rs resolves directly to `src/generated/mod.rs` (generated by the spec tool).
  No intermediate hand-written mod.rs is needed. This is the correct output path (fixed
  in CEO review 2026-04-02: prior refs to `src/generated/spec` were incorrect).
  This makes generated code live code, not dead code. Without this, `cargo check`
  validates nothing.
- Add integration test in `spec-cli/tests/` that:
  1. Runs `spec generate examples/ecommerce/units --output examples/ecommerce/src/generated`
  2. Runs `cargo check` against the ecommerce crate with isolated `CARGO_TARGET_DIR`
     (set to a tempdir to avoid target/ lock contention with the parent workspace)
  3. After D5 lands: runs `cargo test` against the ecommerce crate with same `CARGO_TARGET_DIR`
     (picks up generated `#[cfg(test)]` blocks from D5)
- Run in default test suite (not `#[ignore]`): this is the canonical proof point

Acceptance:
- `cargo test` (spec-cli) runs `cargo check` on generated ecommerce output
- `cargo test` (spec-cli) runs `cargo test` on ecommerce example
- The check passes clean (no unresolved imports, no type errors)
- `cargo check` subprocess uses isolated `CARGO_TARGET_DIR` (no target/ lock contention)
- Generated test functions from `local_tests` execute and pass

---

### D5 — Make `local_tests` real

`local_tests` has been in the schema since M1 but is inert.

**Scope (execution committed in Eng Review):**
- Generate a `#[test]` function per `local_tests` entry in the output `.rs` file
- Test function naming: `test_{id}` format — e.g., `local_tests: [{id: happy_path}]` → `fn test_happy_path()` (locked in CEO review 2026-04-02)
- Schema validation (CEO review 2026-04-02): `local_tests[].id` must match `^[a-z][a-z0-9_]*$`
  (same pattern as ID segments). Add this regex to `unit.spec.json` for the `local_tests.items.properties.id`
  field. Prevents `id: "some case!"` from generating uncompilable `fn test_some case!()`. Add
  to validator test suite: `validate_local_test_id_must_be_valid_identifier`.
- Each `expect` string is wrapped in `assert!(...)` — it's a boolean expression
- The generated `#[cfg(test)] mod tests` block opens with `use super::*;` so the test functions
  can see the unit's function and any imported symbols without explicit re-imports
- D4's integration test runs `cargo test` on the ecommerce example, so generated tests execute

Acceptance:
- A spec with `local_tests` entries produces a `#[cfg(test)]` block in the generated `.rs`
- The generated test block compiles (verified via D4 cargo check)
- Generated test functions execute and pass (verified via D4 cargo test)
- A spec with no `local_tests` generates no test block (no regression)

Tests to add:
- `generate_local_tests_produces_cfg_test_block` (unit test in generator.rs)
- `generate_no_local_tests_produces_no_test_block` (unit test in generator.rs)

---

### D8 — Version bump and changelog note

D7 is a behavioral breaking change. Bump workspace version to `0.2.0` in the root
`Cargo.toml`. Add a CHANGELOG entry:

> **Breaking:** `validate` and `generate` now exit 1 for specs with unresolved internal
> deps. Previously these passed silently. Ensure all deps are defined in the same spec
> set before upgrading.

**ecommerce compile note:** `examples/ecommerce/src/main.rs` will have `mod generated;`
after D4. The ecommerce crate requires `spec generate examples/ecommerce/units --output
examples/ecommerce/src/generated/spec` before it can be compiled independently. The
generated output is gitignored (`generated/` in root .gitignore). The D4 integration
test generates automatically before running `cargo check`. Document this in the
ecommerce crate's README or a comment in main.rs.

---

### D6 — CUE vs JSON Schema: one explicit decision note

The repo docs still reference CUE in several places while the implementation is
JSON Schema. This creates ambiguity for anyone reading the codebase.

Add a decision note to `DECISIONS.md` (or a `## Validation Strategy` section in
README/CLAUDE.md):

> For 0.1 and 0.2, JSON Schema is the implementation path. CUE remains a candidate
> for 0.3+ when cross-file constraints and policy composition justify the complexity.
> Do not design against CUE until then.

Also audit spec comments and YAML for CUE language and update or remove.

This is a doc task — no code changes.

---

### D7 — Dep validation errors (always strict)

Identified in QA 2026-04-02. `validate` currently passes silently when a dep ID isn't
found in the loaded spec set — "✅ valid" even when generate would produce a broken
`use` statement. That's a misleading signal.

**Design (finalized in Eng Review #2):** Always strict. No escape hatch for M2.
The generator hardcodes `use crate::...` for all internal deps. An unresolved dep
cannot produce compilable output. Partial-graph workflows (spec libraries, incremental
authoring) are deferred to M3 when external dep resolution is defined. Allowing generate
to proceed with known-missing deps would overwrite a good output tree with code that
can't compile.

- Unresolved internal deps are errors in both `validate` and `generate` (exit 1)
- Error message: `❌ dep 'money/round' not found in this spec set`
- M3: when cross-library dep composition is introduced, add `--no-strict` with
  defined semantics for external resolution

**Data flow:** `finish_validation` returns `(errors: BTreeMap<...>, warnings: Vec<String>)`.
For M2, missing deps always go into `errors`. The `warnings` path is reserved for
future use.

Acceptance:
- `validate ./units` with a missing dep exits 1 with a clear error
- `generate ./units --output ./out` with a missing dep exits 1, writes nothing
- `validate` on a fully self-contained spec set: exits 0, no errors
- All existing tests pass unchanged

Tests to add:
- `validate_strict_errors_on_missing_dep` (CLI integration test)
- `generate_strict_errors_on_missing_dep` (CLI integration test)

---

## What is NOT in scope

Hold these for M3 or later:

- `.test.spec` (molecule/organism tests)
- Passports and evidence collection
- Full graph resolution and cycle detection (beyond the strict dep check in D7)
- Multiple target languages
- Reverse ingestion
- Planning integration
- IDE/LSP layer
- Rich scheduling or organism-level verification
- Cross-library dep composition (separate `use` path model) and partial-graph workflows (`--no-strict` flag deferred to M3 when external dep resolution is defined)
- Contract type validation (opaque strings in contract.inputs — M3 after D2 lands)
- `local_tests` structured input model (M3 — D5's expect field is raw Rust expression)
- Directory-level atomic swap (file-level atomicity is sufficient for M2 file counts)

---

## Sequencing rationale

```
D1 (output hardening + bootstrap safety) → unblocks safe iteration on everything else
D2 (dep model split: deps + imports)     → required for D4 (cargo check) to be honest
D3 (syn: fn name + arg alignment)        → independent, can land in parallel with D2
D4 (cargo check + wire ecommerce crate)  → depends on D1 + D2; this is the proof point
D5 (local_tests: generate + execute)     → depends on D4 (runs in same cargo test)
D6 (CUE doc decision)                    → any time, no dependencies
D7 (dep errors by default)               → independent, low-risk, can land after D1
D8 (version bump + changelog + ecommerce note) → land with D7 or last
```

Parallel tracks:
- **Track A:** D1 → D2 → D4 → D5
- **Track B:** D3 (runs alongside D2)
- **Track C:** D6, D7 (anytime)

---

## Success criteria (definition of done)

| Check | Pass condition |
|-------|---------------|
| Output safety | `spec generate --output src/` errors before writing anything |
| Bootstrap safety | `spec generate --output /tmp/x` errors before writing anything |
| No stale files | Generate ecommerce, then generate a different spec set — no ecommerce dirs remain |
| Body alignment | `id: pricing/foo` + `pub fn bar()` fails validation |
| Arg alignment | `contract.inputs: {subtotal: Decimal}` + body `fn apply_discount(price: Decimal)` fails |
| Cargo check | `cargo test` runs cargo check on generated ecommerce output and passes |
| Module wiring | `mod generated;` in main.rs resolves to spec-generated mod.rs at src/generated/mod.rs |
| Cargo test | `cargo test` runs ecommerce tests (local_tests executed) and passes |
| Decimal import | `apply_tax.unit.spec` compiles with external Decimal import, no broken use statements |
| Strict mode (validate) | `validate ./units` with a missing dep exits 1 |
| Strict mode (generate) | `generate ./units --output ./out` with a missing dep exits 1, writes nothing |
| local_tests | Generated `.rs` has `#[cfg(test)]` block matching local_tests entries |

---

## Test gaps (to be added during implementation)

All 9 gaps identified in Eng Review:

- `clean_output_dir_removes_stale_module_from_prior_run` (D1, generator.rs)
- `generate_rejects_non_empty_dir_without_marker` (D1, cli.rs)
- `generate_rejects_path_outside_project_root` (D1, cli.rs)
- `imports_field_validates_rust_path` (D2, validator.rs)
- `imports_field_generates_correct_use_statement` (D2, generator.rs)
- `validate_body_fn_name_mismatch` (D3, validator.rs)
- `validate_body_multiple_fns_rejected` (D3, validator.rs)
- `validate_contract_arg_name_mismatch` (D3, validator.rs)
- `generate_cargo_check_on_ecommerce` (D4, spec-cli/tests/cli.rs — new integration test)
- `generate_local_tests_produces_cfg_test_block` (D5, generator.rs)
- `generate_no_local_tests_produces_no_test_block` (D5, generator.rs)
- `validate_strict_errors_on_missing_dep` (D7, cli.rs)
- `generate_strict_errors_on_missing_dep` (D7, cli.rs)
- `generate_rejects_symlinked_output_path` (D1, cli.rs — symlink escape guard)
- `validate_body_with_macros_passes_fn_name_check` (D3, validator.rs — syn::File parse path)
- `deps_unchanged_after_imports_split` (D2, generator.rs — regression: deps: [money/round] still generates use crate::money::round::round; after adding imports field)
- `generate_strict_errors_on_missing_dep` (D7, cli.rs — generate always strict: missing dep exits 1, no output written)
- `generate_local_tests_uses_test_prefix_naming` (D5, generator.rs — confirms `happy_path` → `fn test_happy_path()`)
- `imports_emitted_before_deps_in_use_statements` (D2, generator.rs — external imports precede internal dep use stmts)
- `validate_body_method_rejected` (D3, validator.rs — body.rust with `&self` param fails validation)
- `generate_cargo_check_test_failure_includes_cargo_stderr` (D4, spec-cli/tests/cli.rs — assert message includes cargo output)
- `validate_local_test_id_must_be_valid_identifier` (D5, validator.rs — id with spaces/special chars fails schema)
- `clean_output_dir_removes_empty_dirs_after_orphan_cleanup` (D1, generator.rs — empty stale dirs removed)

---

## Worktree parallelization

| Step | Modules touched | Depends on |
|------|----------------|------------|
| D1 | spec-core/generator, spec-cli/commands | — |
| D2 | spec-core/types, spec-core/schema, spec-core/generator, spec-cli/commands | — |
| D3 | spec-core/validator, spec-core/Cargo.toml | — |
| D4 | spec-cli/tests, examples/ecommerce | D1, D2, D3 |
| D5 | spec-core/generator | D4 |
| D6 | docs only | — |
| D7 | spec-core/validator, spec-cli/commands | D1 |
| D8 | Cargo.toml, CHANGELOG, examples/ecommerce/src/main.rs | D4, D7 |

**Lane A:** D1 → D4 → D5 (sequential, shared generator + commands)
**Lane B:** D2 (can start in parallel with D1, merges before D4)
**Lane C:** D3 (fully independent, merge before D4)
**Lane D:** D6, D7 (anytime, independent)

Launch B + C + D in parallel worktrees. Merge all into main before starting D4.

---

## Failure modes

| Codepath | Failure scenario | Test? | Error handling? | Silent? |
|---------|-----------------|-------|----------------|---------|
| D1 orphan cleanup | OS error deleting stale file (permissions) | No | Yes (SpecError::Generator) | No |
| D1 bootstrap guard | Symlink pointing outside project root | No | Partial (normalizes path) | **CRITICAL GAP** |
| D2 imports generation | Invalid Rust path in imports (e.g., `::Decimal`) | Yes (schema) | Yes (schema rejects) | No |
| D3 syn parse | body.rust contains macro invocations | No | Needs handling | **CRITICAL GAP** |
| D4 cargo check | cargo not in PATH in CI | No | No | Yes — test silently skips |
| D4 cargo check | shared target/ lock contention | Yes (CARGO_TARGET_DIR) | Yes | No — isolated dir per run |
| D5 local_tests | expect string is invalid Rust | No | Caught by D4 cargo check | No |
| D7 dep check | very large spec set, O(n²) dep lookup | No | N/A (acceptable for M2) | No |

**Critical gaps (both resolved in Eng Review):**

1. **Symlink escape in `ensure_output_marker`** — `normalized_absolute_path` resolves `..` but not symlinks.
   **Decision:** Before creating the output dir, walk each existing path component and reject if any is a symlink
   (`path.symlink_metadata()?.file_type().is_symlink()`). Nothing is created on a bad path. Never touches the
   filesystem before validating.

2. **`syn` + macro invocations in `body.rust`** — `syn::parse_str::<ItemFn>` fails on any body containing macros
   (`vec![...]`, `todo!()`, `assert!(...)`, etc.), which would incorrectly reject valid specs.
   **Decision:** Parse as `syn::File` (not bare `ItemFn`). Walk the file's top-level items to find exactly one
   `ItemFn`. Macros inside function bodies are `Stmt::Macro` — not top-level items — so they parse correctly.
   The existing rules (wrong fn name, multiple top-level items) apply only to top-level items, which is correct.

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | issues_open | D4 module wiring fix, D2 ordering locked, D5 naming locked, D1 empty dirs, D5 id validation, CARGO_TARGET_DIR, TODOS.md D7 updated |
| Codex Review | `/codex review` | Independent 2nd opinion | 3 | issues_found | 10 findings; 4 incorporated (D4 path, D1 dirs, D5 id, CARGO_TARGET_DIR), 6 acknowledged |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 2 | CLEAR (PLAN) | 5 issues, 17 test gaps, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**CODEX:** D4 output path corrected to src/generated (module wiring fix), CARGO_TARGET_DIR isolation added, D5 test id validation added to schema, D1 empty dir cleanup added, D7 TODOS.md entry updated, D3 clarified as partial semantic alignment
**CROSS-MODEL:** D4 module wiring (critical build-blocking gap) resolved — output path changed to src/generated. D4/D5 sequencing clarified.
**UNRESOLVED:** 0
**VERDICT:** ENG CLEARED — Eng Review passed. CEO Review complete (HOLD SCOPE). Ready to implement.
