<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m2-autoplan-restore-20260403-052737.md -->
# Release 0.2: Harden the Loop

**Generated**: 2026-04-02  
**Status**: Implemented (PR #1 MERGED — 2026-04-02)  
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
- Unresolved internal deps are errors by default (escape hatch `--no-strict` deferred to M3)
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
examples/ecommerce/src/generated` before it can be compiled independently. The
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

---

## /autoplan Review — 2026-04-03 (Retrospective, post-merge)

**Context:** PR #1 MERGED. This is a retrospective pass on the completed M2 plan.
**Mode:** SELECTIVE EXPANSION (auto-decided: iteration on existing system, already implemented)
**UI Scope:** None

### CEO Dual Voices

**CLAUDE SUBAGENT (CEO — strategic independence):**
1. No clear primary user defined (critical) — solo engineer vs. team coordination tool has different M3 priorities
2. Generated code commitment model hybrid/ambiguous (high) — gitignored but required for compile; need binary decision in M3
3. `local_tests.expect` raw string won't scale (high) — M3 structured input model is not optional, it's the product feature
4. Cross-library dep model undesigned before M3 build (high) — sketch schema before building
5. Competitive moat requires evidence/passport model, not better codegen (high) — prioritize over graph resolution
6. LSP surface not examined before committing to CLI-only shape (medium)
7. JSON Schema cross-file constraint ceiling; define CUE trigger condition explicitly (medium)
8. Multi-team output ownership not modeled (medium)
9. `syn full` compile weight (low)

**CODEX SAYS (CEO — strategy challenge):**
1. No ICP defined — correctness tool without adoption funnel
2. "No breadth" is a premise not a strategy for early-stage
3. Thesis says `--no-strict` "escapable" (line 21) but D7 locks it always strict — doc contradiction
4. D8 line 230 reintroduces `src/generated/spec` path (contradicts D4 correction) — doc drift
5. PLAN.md Status still says "Planning" — post-merge drift breaks onboarding
6. `use crate::` hardcoded assumes `pub use generated::*;` pattern in consuming crate — works for ecommerce but non-obvious for drop-in
7. D3 "exactly one top-level function" constraint will be reversed — real users want helper fns, consts, type aliases
8. "Owned subtree" + orphan deletion blocks mixed directories (generated + handwritten glue)
9. Alternative: invert D3 — generate signature from contract, body.rust = body expression only
10. No "drop-in to existing crate" story — requires adding `pub use generated::*;` to crate root
11. Competitive differentiation vs OpenAPI/proto/Smithy not articulated

```
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   Yes     Partial PARTIAL (--no-strict doc contradiction; thesis wording)
  2. Right problem to solve?           Yes     Partial PARTIAL (no ICP defined by either review)
  3. Scope calibration correct?        Yes     Yes     CONFIRMED (M2 hardening was right call)
  4. Alternatives sufficiently explored? No    No      DISAGREE (both flag D3 invert not explored)
  5. Competitive/market risks covered? No      No      CONFIRMED gap (both flag missing differentiation story)
  6. 6-month trajectory sound?         Yes     Partial PARTIAL (local_tests.expect ceiling, dep model undesigned)
═══════════════════════════════════════════════════════════════
CONFIRMED = both agree. DISAGREE = models differ (→ taste decision).
Single critical finding: D3 one-function constraint flagged by Codex (taste decision).
```

### CEO Review Sections

**Step 0A — Premise Challenge:**
All 7 premises confirmed by user. One doc-level contradiction found (not a premise flaw): thesis line 21 says "escapable via `--no-strict`" but D7 always strict — wording should be corrected to "errors by default (escape hatch deferred to M3)."
AUTO-DECISION: Fix wording in PLAN.md thesis (P5 explicit). Logged.

**Step 0B — Existing Code Leverage:**
- D1 orphan cleanup → builds on existing `clean_output_dir` in generator.rs — correct reuse
- D2 imports → extends existing `SpecStruct`/`ResolvedSpec` types in types.rs — correct reuse
- D3 syn validation → new dependency, no existing code to reuse — appropriate
- D4 → builds on existing CLI test infrastructure in cli.rs — correct reuse
- D5 → extends existing `generate_code` in generator.rs — correct reuse
No parallel-flow reconstruction found. No DRY violations.

**Step 0C — Dream State Mapping:**
```
  CURRENT STATE (M2)             THIS PLAN              12-MONTH IDEAL
  ─────────────────────          ───────────────────    ─────────────────────────────
  load→validate→generate         Output safe            Contract type validation
  Rust output compiles           cargo check passes     Evidence/passport model
  No import/dep confusion        Fn name aligned        Cross-library dep resolution
  Stale files possible           Orphan cleanup done    Team adoption
  Deps unvalidated               Strict by default      CI-enforced spec compliance
```
M2 moves correctly toward the 12-month ideal. The trajectory is sound. The gap: evidence model and ICP definition are not in the plan at all — they remain undefined territory.

**Step 0C-bis — Implementation Alternatives:**
```
APPROACH A: Current (semantic hardening first) — SHIPPED
  Summary: Make the compile loop safe before adding features. Validate semantics incrementally with syn.
  Effort:  M (4 weeks delivered in ~1 week with CC)
  Risk:    Low
  Pros:    - Trust before features; compilable output as proof point; no external service dependencies
  Cons:    - No product differentiator yet; ICP undefined; local_tests has ceiling

APPROACH B: Feature-first (skip hardening, add evidence model now)
  Summary: Accept compilation gaps, focus M2 on contract type validation and a minimal passport record.
  Effort:  L
  Risk:    High — broken output tree + unvalidated types = user trust destroyed
  Pros:    - Differentiator earlier
  Cons:    - Unsafe output discredits the tool; early users abandon before trust builds

APPROACH C: Invert D3 (generate from contract, body.rust = body expression)
  Summary: Generate fn signature from contract.inputs, embed body.rust as the function body.
  Effort:  M (replaces D3's syn-validation approach)
  Risk:    Medium — breaking schema change for body.rust convention
  Pros:    - Eliminates fn name drift entirely; no syn policing; cleaner separation of contract and impl
  Cons:    - Breaking change from M2's shipped design; forces users to restructure body.rust
```
RECOMMENDATION: Approach A was correct for M2 (trust before features). Approach C is the right direction for M3 D3 expansion — worth a design spike before adding type validation. AUTO-DECIDED (P3 pragmatic + P1 completeness).

**Step 0D — Mode Analysis (SELECTIVE EXPANSION):**
Expansions surfaced — none accepted (plan already shipped). Candidates for TODOS.md:
- Define ICP explicitly (no effort, 1 paragraph, before M3 scoping)
- Force generated-code commitment decision (binary: commit or ephemeral; before M3)
- Design Approach C invert-D3 spike (2 hours; before M3 D3 expansion)
- Cross-library dep schema sketch (2 hours; before M3 build)
All → deferred to TODOS.md. AUTO-DECIDED (P3 pragmatic, plan is already shipped).

**Step 0E — Temporal Interrogation:**
This is retrospective; temporal interrogation maps to M3 decisions:
- HOUR 1 (M3 foundations): ICP definition must happen first — it gates M3 prioritization
- HOUR 2-3 (M3 core): contract type validation requires resolving Approach C vs current D3 shape
- HOUR 4-5 (M3 integration): evidence model requires defining what constitutes "passing" proof
- HOUR 6+ (M3 polish): generated code commitment model must be resolved before M3 CI story

**Section 1 — Architecture Review:**
```
  CURRENT ARCHITECTURE (post-M2):
  
  .unit.spec files
       │
       ▼
  ┌─────────────┐    ┌──────────────┐    ┌──────────────────┐
  │   loader.rs  │──▶│  normalizer  │──▶ │  validator.rs    │
  │  (YAML→     │    │  (resolve    │    │  (JSON Schema +  │
  │  SpecStruct) │    │  deps/types) │    │  syn + dep check)│
  └─────────────┘    └──────────────┘    └──────────────────┘
                                                   │
                                                   ▼
                                         ┌──────────────────┐
                                         │  generator.rs    │
                                         │  (code + mod.rs  │
                                         │  + orphan clean) │
                                         └──────────────────┘
                                                   │
                                         ┌─────────┴──────────┐
                                         │ per-file atomic    │
                                         │ write (tempfile +  │
                                         │ rename, POSIX safe)│
                                         └────────────────────┘
```
Architecture is clean and well-separated. No coupling concerns. Single point of failure: generator.rs owns both file writing and orphan cleanup — appropriate for M2 scope.

One concern: consuming crates must add `pub use generated::*;` to their crate root for `use crate::X` dep paths to resolve. This is implicit. Examined: working correctly in ecommerce example via main.rs. Not documented for external users. → Flag for TODOS.md doc task.

**Section 2 — Error & Rescue Map:**
```
  CODEPATH                     | WHAT CAN GO WRONG        | HANDLED?
  -----------------------------|--------------------------|----------
  clean_output_dir             | OS error deleting file   | SpecError::Generator (logged, not silent)
  ensure_output_marker         | Symlink escape           | Walks path components, rejects symlinks
  syn::parse_str               | Macro invocations        | Handled: parse as syn::File (not ItemFn)
  cargo subprocess (D4)        | cargo not in PATH        | Silent skip (cargo_available() → return)
  tempfile + rename (D1)       | EXDEV cross-fs rename    | Handled: tempfile in same dir
  dep_to_use_path              | Circular deps            | Not detected (deferred to M3)
```
One confirmed gap from failure modes table: `cargo not in PATH → silent skip`. Codex also flagged this.
AUTO-DECISION: Flag for TODOS.md. Silent skip is acceptable in dev environments where cargo is always present; the concern is limited to unusual CI environments. (P3 pragmatic — not blocking M2 which is shipped)

**Section 3 — Security & Threat Model:**
Known learning (from project learnings): `local_tests.expect` injection was fixed in `43f4c0b` — whitelist approach in `is_safe_expect_expr`. Only binary, call, path, lit, and paren expressions allowed.
Prior learning applied: expect-safe-expr-whitelist (confidence 10/10, 2026-04-03).

New surface from M2:
- Output path safety: symlink-aware path validation before any writes. Solid.
- `syn::File` parse of `body.rust`: parses user-supplied Rust code. Risk: malformed syn input causes parse failure (surfaced as validation error, not panic). Safe.
- No new secrets, no new auth surfaces, no new endpoints.
- Dependency: `syn` with `full` — well-known crate, excellent security track record, no advisory history.

No unaddressed security gaps found. Examined: injection surface in expect, output path, syn parsing.

**Section 4 — Data Flow & Interaction Edge Cases:**
```
  .unit.spec ──▶ YAML parse ──▶ schema validate ──▶ syn parse ──▶ generate ──▶ atomic write
       │              │               │                  │              │
       ▼              ▼               ▼                  ▼              ▼
  [not a file]   [parse error]  [schema error]    [syn error]   [EXDEV? → handled]
  [empty yaml]   [handled]      [handled]         [handled]     [tempfile in same dir]
  [binary file]  [handled]      [handled]         [handled]     [OS error → SpecError]
```
All shadow paths are handled and tested. No unhandled edge case found. CLI integration test confirms error paths surface correctly.

**Section 5 — Code Quality:**
- 786 lines in validator.rs is the biggest file — complexity is there but appropriate for the validation surface
- 1 deferred comment at validator.rs:136 (M3 local_tests config lever) — explicitly tracked in TODOS.md
- DRY: no duplicate logic found across generator/validator
- Naming: clear and consistent — `ensure_output_marker`, `clean_output_dir`, `is_safe_expect_expr`
- No over-engineering found — each function does one thing
- D3's "exactly one top-level item" constraint: Codex flags this will be reversed when users need helper fns/consts. Current implementation at `validate_body_rust` in validator.rs. This is a TASTE DECISION (see gate). Auto-logged.

**Section 6 — Test Review:**
```
NEW CODEPATHS IN M2:
  D1: ensure_output_marker path validation, orphan cleanup, atomic write
  D2: imports field, use statement ordering
  D3: syn fn name check, arg alignment, sibling item check, self-param check
  D4: cargo check subprocess, cargo test subprocess, CARGO_TARGET_DIR isolation
  D5: local_tests codegen, #[cfg(test)] block generation
  D7: strict dep validation in validate + generate paths
```
Test coverage from PR #1: 46 → 76 tests (+30). Coverage gate: 94% (15/16 paths). One known gap: output-is-file defensive bail path. Test pyramid: heavy unit + solid integration. No flakiness risk identified.

**Section 7 — Performance:**
- No N+1 queries (Rust CLI, no DB)
- `syn` parse per unit spec: O(body_size) — acceptable; bodies are tiny
- Orphan cleanup: O(n) file scan — acceptable for M2 file counts
- Cargo subprocess (D4): slow by nature, isolated target dir prevents lock contention

**Section 8 — Observability:**
All errors go to stderr via `SpecError` types. Exit codes are meaningful (0 = clean, 1 = errors). No structured logging — appropriate for a CLI. No dashboards or alerts needed (not a service).

**Section 9 — Deployment:**
CLI binary. No deployment concerns. Version bumped to 0.2.0 in Cargo.toml. CHANGELOG updated. CI/CD in place (GitHub Actions, cross-compilation).

**Section 10 — TODOS.md Items:**
Items to add from this review:
1. Define ICP: solo engineer vs team coordination? (prerequisite for M3 scoping)
2. Binary decision: commit generated output or ephemeral? (prerequisite for M3 CI story)
3. `pub use generated::*` pattern: document as required convention for consuming crates
4. D3 expansion: spike Approach C (generate from contract, body.rust = body expression) before M3 type validation
5. Cross-library dep schema design spike (2 hours, before M3 build)
6. CUE trigger condition: define explicitly (e.g., "when we need cross-file constraint X")
7. cargo silent skip: consider `#[should_panic]` or explicit `skip_reason` log for CI visibility

**NOT in scope (confirmed not in M2):**
- .test.spec, passports, graph resolution, multiple languages, reverse ingestion, IDE/LSP, scheduling, CUE, contract type validation, local_tests structured input, directory-level atomic swap

**What already exists (M2 leverage map):**
- loader.rs → pre-existing, unchanged
- normalizer.rs → pre-existing, minor addition
- validator.rs → extended with syn, dep strictness
- generator.rs → extended with imports, orphan cleanup, atomic writes, local_tests codegen
- cli.rs → extended with safety guards, strict mode

**Dream State Delta:**
M2 leaves us at: compilable, safe, semantically anchored. Distance from 12-month ideal: need ICP, evidence model, contract types. Trajectory is correct.

**CEO Phase Completion Summary:**
```
CEO REVIEW (autoplan retrospective):
  Premises:        7/7 confirmed (1 doc-level wording fix needed in thesis)
  Architecture:    SOUND — clean separation, appropriate coupling
  Security:        SOUND — injection surface addressed, output path safe
  Test Coverage:   94% — 1 known gap (defensive bail path, acceptable)
  Codex voice:     11 findings (3 doc fixes, 5 M3 signals, 2 confirmed gaps, 1 taste decision)
  Claude voice:    9 findings (3 M3 signals, 5 confirmed concerns, 1 low)
  Consensus:       3/6 confirmed, 1 partial (premises), 2 partial (ICP, trajectory)
  Auto-decisions:  8 (all SELECTIVE EXPANSION deferrals or doc fixes)
  Taste decisions: 1 (D3 single-function constraint — will surface at gate)
  User challenges: 0
```

**PHASE 1 COMPLETE.** Codex: 11 findings. Claude subagent: 9 findings. Consensus: 3/6 confirmed, 3 partial. Taste decisions: 1 (D3 constraint). Passing to Phase 3 (skipping Phase 2 — no UI scope).

---

### Phase 3: Eng Review + Dual Voices

**CLAUDE SUBAGENT (Eng — independent review):**
1. `clean_output_dir` vs `ensure_output_marker` use different path-containment logic (High) — `normalized_absolute_path` (lexical) vs `canonicalize` (follows symlinks). Consolidate to single `safe_output_path` utility.
2. Windows rename TOCTOU window (High) — `remove_file` + `rename` is not atomic; comment says "per-file atomic" which is only true on POSIX. Add clarifying comment + Windows test.
3. `validate_body_rust_alignment` misleading error message (Medium) — when body has 1 item that's not a fn, reports `found: 0` instead of `found: 1`. Need separate error variant or pass correct count.
4. `is_safe_expect_expr` does not recurse into sub-expressions (Medium/High Security) — `syn::Expr::Binary` and `syn::Expr::Call` return `true` without checking children. `f({ unsafe { ... } })` bypasses the block/unsafe check.
5. `collect_specs` follows symlinks, `clean_output_dir` does not — undocumented asymmetry; symlink cycle aborts collect with unclear error (Medium)
6. `generate_command` counts only unit files, not mod.rs files in "Generated N files" output (Low)
7. Dep collision check duplicated in validator and generator (Low DRY)
8. `normalized_absolute_path` swallows `current_dir()` failure with `fallback(".")` (Low)
9. Missing idempotency test: two runs same output (Low)
10. Composite key `"file1 | file2"` is fragile for paths containing ` | ` (Low)

**CODEX SAYS (Eng — architecture challenge):**
1. Consumer-module contract (`pub use generated::*`) is implicit and brittle — doc it explicitly as required convention (High)
2. "Owned subtree" + orphan deletion blocks mixed directories — plan should explicitly require dedicated output dir (Medium)
3. Rust keyword identifiers in local_tests[].id → ALREADY HANDLED by validate_rust_keywords (confirmed: False alarm)
4. `local_tests[].id` uniqueness not enforced — duplicate ids → duplicate fn compile error (Medium)
5. Visibility not validated — `fn apply_discount()` (no pub) passes spec validate but fails cargo check via dep (Medium)
6. "Generate writes nothing on error" ordering — validate specs before marker/tempdir creation (Medium — confirmed by reading commands.rs)
7. Plan-doc drift — retrospective notes in same file become anti-documentation (Low)

```
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               Yes     Partial PARTIAL (path containment divergence, mixed dir gap)
  2. Test coverage sufficient?         Partial Partial CONFIRMED gap (is_safe_expect_expr recursion, uniqueness, idempotency)
  3. Performance risks addressed?      Yes     Yes     CONFIRMED (O(n) scans acceptable, CARGO_TARGET_DIR isolated)
  4. Security threats covered?         Partial Partial CONFIRMED gap (is_safe_expect_expr non-recursive)
  5. Error paths handled?              Partial Yes     PARTIAL (current_dir() swallowed, misleading error msg)
  6. Deployment risk manageable?       Yes     Yes     CONFIRMED (single binary, no DB, clean rollback)
═══════════════════════════════════════════════════════════════
CONFIRMED = both agree. DISAGREE = models differ.
Critical gap: is_safe_expect_expr non-recursive (security, both models flag independently).
```

**Section 1 — Architecture ASCII Diagram (post-M2):**
```
  .unit.spec files
       │
       ▼
  ┌──────────────┐   schema JSON  ┌─────────────────────────────────────────┐
  │  loader.rs   │───────────────▶│ validator.rs                            │
  │  (YAML parse)│                │  • JSON Schema (jsonschema crate)        │
  └──────────────┘                │  • semantic: keywords, dep format        │
       │                          │  • syn: fn name, args, self-param check  │
       ▼                          │  • local_tests: id regex + expr safety   │
  ┌──────────────┐                │  • cross-spec: duplicate IDs, dep exists │
  │normalizer.rs │                └─────────────────────────────────────────┘
  │ (dep lookup) │                          │
  └──────────────┘                          ▼
       │                         ┌─────────────────────────────────────────┐
       └────────────────────────▶│ generator.rs                            │
                                 │  • generate_code(): use stmts + body    │
                                 │  • local_tests → #[cfg(test)] block      │
                                 │  • generate_mod_rs(): namespace tree      │
                                 │  • write_generated_file(): atomic rename  │
                                 │  • clean_output_dir(): orphan cleanup     │
                                 │  • ensure_output_marker(): safety guard   │
                                 └─────────────────────────────────────────┘
```
Coupling: generator.rs owns both file writing and orphan cleanup. Appropriate for M2. Single point of failure for output operations, which is intentional (owned subtree model).

**Section 2 — Code Quality:**
- `is_safe_expect_expr` non-recursive at validator.rs:148 — security gap, both models flag it
- `normalized_absolute_path` fallback at generator.rs:317 — swallows real errors
- Dep collision check at generator.rs:274 duplicates validator.rs:81 — remove from generator
- File count reporting at commands.rs:145 — undercounts by not including mod.rs writes
- Naming: clean, consistent, appropriate. No over-engineering.

**Section 3 — Test Review:**
Test diagram and artifacts: see `~/.gstack/projects/atomize-hq-spec/spenquatch-feat-m2-test-plan-20260403-064740.md`

Critical test gap: `is_safe_expect_expr` non-recursive (security). Add tests:
- `expect_with_unsafe_block_in_call_arg_is_rejected` — `f({ unsafe { ... } })` must fail
- `expect_with_block_in_binary_operand_is_rejected` — `a + { exit(1); 2 }` must fail

Medium gaps: local_tests id uniqueness, non-fn body error message, visibility validation.
Low gaps: idempotency, file count, symlink cycle, cargo skip logging, composite key.

**Section 4 — Performance:**
No concerns. syn parse is O(body_size), orphan scan is O(n files), cargo subprocess has isolated target dir. Acceptable at M2 scale.

**NOT in scope (eng view):**
- Parallel generation (--jobs flag): M3+ when file count justifies
- Cross-crate output: requires cross-library dep model (M3)
- Cycle detection: M3 graph resolution

**What already exists:**
- loader.rs: unchanged, pre-existing
- normalizer.rs: minor extension
- validator.rs: syn + dep strictness + local_tests validation
- generator.rs: imports, atomic writes, orphan cleanup, local_tests codegen
- cli.rs: safety guards, strict mode, cargo subprocess

**Failure Modes Registry (updated):**
| Codepath | Failure scenario | Test? | Critical? |
|---------|-----------------|-------|-----------|
| is_safe_expect_expr | Block/Unsafe nested in Call/Binary args | No | **CRITICAL GAP** |
| local_tests id | Duplicate ids within unit | No | Medium gap |
| visibility | Private fn used as dep | No | Medium gap (caught by cargo check) |
| clean_output_dir | Path containment logic diverges from ensure_output_marker | No | High gap |
| normalized_absolute_path | current_dir() fails | No | Low gap |
| collect_specs | Symlink cycle in spec dir | No | Medium gap |
| cargo_available() | Cargo not in PATH at test time | No | Low gap (silent skip) |

**CEO Phase → Eng Phase Cross-Phase Themes:**
1. **`is_safe_expect_expr` injection surface** — CEO (prior learning applied: local-test-expect-injection, confidence 10/10) + Eng (non-recursive check). Both independently reach the same gap via different paths.
2. **Consumer-module contract undocumented** — CEO (pub use generated::* is implicit) + Eng (brittle, blocks drop-in to existing crate). Both flag independently.
3. **`local_tests.expect` design ceiling** — CEO (raw string won't scale as product) + Eng (test gaps in is_safe_expect_expr). Two distinct perspectives, same underlying fragility.

**Eng Phase Completion Summary:**
```
ENG REVIEW (autoplan retrospective):
  Architecture:    SOUND with 2 noted gaps (path containment divergence, mixed dir)
  Security:        CRITICAL GAP — is_safe_expect_expr non-recursive (both models)
  Code quality:    GOOD — 5 minor issues, all low/medium
  Test coverage:   1 critical gap, 4 medium gaps, 5 low gaps
  Test plan:       Written to ~/.gstack/projects/atomize-hq-spec/spenquatch-feat-m2-test-plan-...md
  Auto-decisions:  5 additional (all deferrals or flags)
  Taste decisions: 0 (Eng phase)
  User challenges: 0 (security gap = bug fix, not direction change)
```

**PHASE 3 COMPLETE.** Codex: 7 findings. Claude subagent: 10 findings. Consensus: 3/6 confirmed, 2 partial. Cross-phase themes: 3 (is_safe_expect_expr, consumer-module contract, local_tests fragility). Proceeding to Final Approval Gate.

---

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Mode: SELECTIVE EXPANSION | Mechanical | P3 pragmatic | Iteration on existing implemented system | SCOPE EXPANSION |
| 2 | CEO | Thesis wording fix: "escapable via --no-strict" → "escape hatch deferred to M3" | Mechanical | P5 explicit | Doc contradiction caught by Codex, wording misleads readers | None |
| 3 | CEO | D8 path contradiction (line 230): flag for doc fix | Mechanical | P5 explicit | "src/generated/spec" reintroduced after D4 corrected it to "src/generated" | None |
| 4 | CEO | PLAN.md Status field: flag for update to "Implemented (PR #1 MERGED)" | Mechanical | P5 explicit | Post-merge drift, breaks onboarding | None |
| 5 | CEO | ICP definition → TODOS.md (not added to M2 scope) | Mechanical | P3 pragmatic | M2 already shipped; add as M3 prerequisite | Cherry-pick |
| 6 | CEO | Generated code commitment binary decision → TODOS.md | Mechanical | P3 pragmatic | M2 already shipped; hybrid state acceptable short-term | Cherry-pick |
| 7 | CEO | Approach C (invert D3) design spike → TODOS.md | Mechanical | P3 pragmatic | M2 shipped; design spike before M3 D3 expansion | Cherry-pick |
| 8 | CEO | D3 single-function constraint → TASTE DECISION at gate | Taste | P1/P5 conflict | Claude sees no issue, Codex flags it will be reversed | Accept current |
| 9 | Eng | is_safe_expect_expr non-recursive → flag as critical gap at gate | Mechanical | P1 completeness | Security: injection bypass; both models flag; needs fix in follow-up PR | Defer |
| 10 | Eng | path containment logic divergence → TODOS.md | Mechanical | P5 explicit | clean_output_dir vs ensure_output_marker use different algorithms | Cherry-pick |
| 11 | Eng | local_tests id uniqueness → TODOS.md test gap | Mechanical | P1 completeness | Duplicate ids → compile error, no validator check | Cherry-pick |
| 12 | Eng | visibility not validated → TODOS.md | Mechanical | P3 pragmatic | Caught by cargo check in D4; spec validate warning would be better | Defer to M3 |
| 13 | Eng | consumer-module convention doc → TODOS.md | Mechanical | P5 explicit | pub use generated::* must be documented as required convention | Doc task |
