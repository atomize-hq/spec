# Release 0.2: Harden the Loop

**Generated**: 2026-04-02  
**Status**: Planning  
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
- Unresolved internal deps fail clearly (with a flag to enforce in CI)
- External/native imports have an explicit modeled path
- The ecommerce example generates and passes `cargo check`
- At least one `local_tests` path executes for real

---

## Deliverables (sequenced)

### D1 — Output ownership hardening

The highest-priority issue from the third-party review. Right now `ensure_output_marker()`
writes a `.spec-generated` marker before safety checks in `clean_output_dir()` run, and
cleaning is scoped only to the current run's module paths — so stale files from
deleted/renamed units accumulate.

**Model:** owned subtree. The output directory is fully spec-owned. Generation is atomic:
write to a temp dir, rename into place. No partial state on crash or interrupt.

Acceptance:
- Output path is validated before any filesystem write
- `spec generate` on a spec set that previously had `test/foo` leaves no `test/` behind
- Interrupt mid-generate leaves the previous output intact (atomic swap)
- ISSUE-002 and ISSUE-004 closed as resolved-by-design

Files: `spec-core/src/generator.rs`, `spec-cli/src/commands.rs`

Related TODOS: atomic writes (Codex finding, already in M2 backlog)

---

### D2 — Dependency/import model split

Right now `deps` handles everything: internal spec-to-spec calls AND native types like
`Decimal`. These are different things. Without the distinction, generated code looks
readable but isn't fully grounded, and `cargo check` can't pass.

Introduce two fields:

```yaml
deps:
  - money/round          # internal: becomes use crate::money::round::round;

imports:
  - rust_decimal::Decimal  # external: becomes use rust_decimal::Decimal;
```

Or a single `deps` with an `external:` prefix convention — the exact shape is a
decision to make in this deliverable, not before. Lock it in and update the JSON Schema.

Acceptance:
- `apply_discount.unit.spec` can declare `Decimal` without it being treated as an internal unit
- Generated `use` statements are correct for both kinds
- Schema updated and validated

---

### D3 — Rust body parsing and semantic alignment (`syn`)

The biggest semantic gap in M1: the spec says `id: pricing/apply_discount` but there
is no enforcement that `body.rust` contains a function named `apply_discount`. They can
drift silently.

Add a validation pass using `syn`:

- Parse `body.rust` as exactly one `ItemFn`
- Require `fn_name == last id segment`
- Reject extra sibling items (multiple fns, structs, impls)
- Optionally in this pass or next: compare arg names against `contract.inputs`

Acceptance:
- A spec with `id: pricing/apply_discount` and `body.rust: pub fn wrong_name() {}` fails validation with a clear error
- A spec with two function definitions in `body.rust` fails validation
- All existing ecommerce specs pass

Crate to add: `syn` with `full` feature flag

---

### D4 — Cargo check proof

The loop is not honest until we can prove generated output compiles. This deliverable
makes that explicit and repeatable.

- Update the ecommerce example so all deps are satisfied: add `money/round.unit.spec`
  so `apply_discount` and `apply_tax` resolve to real units
- Add external imports for `Decimal` (depends on D2)
- Add an integration test that runs `cargo check` on the generated example output and
  fails the test suite if it does not pass
- The test lives in `spec-cli/tests/` and uses `Command::new("cargo").arg("check")`
  against the example crate

Acceptance:
- `cargo test` runs `cargo check` on generated ecommerce output
- The check passes clean (no unresolved imports, no type errors)
- This test is the canonical proof point for every future generate change

---

### D5 — Make `local_tests` real (first cut)

`local_tests` has been in the schema since M1 but is inert. Once a field exists in
authored source, people assume it matters. Time to make one pass real.

Scope: narrow first step only.

- Generate a `#[test]` function per `local_tests` entry in the output `.rs` file
- The `expect` expression becomes the assertion body (`assert!(...)` or `assert_eq!`)
- Verify generated test functions compile as part of the ecommerce `cargo check` (D4)
- Execution (actually running the tests) is in scope if the above lands cleanly;
  otherwise defer execution to M3

Acceptance:
- A spec with `local_tests` entries produces a `#[cfg(test)]` block in the generated `.rs`
- The generated test block compiles
- A spec with no `local_tests` generates no test block (no regression)

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

### D7 — Dep validation warnings + `--strict` flag

Identified in QA 2026-04-02. `validate` currently passes silently when a dep ID isn't
found in the loaded spec set — "✅ valid" even when generate would produce a broken
`use` statement. That's a misleading signal.

- Emit a per-dep warning when an internal dep is not present in the loaded spec set
- Message: `⚠️  dep 'money/round' not found in this spec set (may be resolved externally)`
- Add `--strict` flag to `validate` and `generate` to treat unresolved deps as errors
- Default behavior stays as warning (deps may legitimately live in a separate spec library)

Acceptance:
- `validate ./units` with a missing dep prints a warning and exits 0
- `validate --strict ./units` with a missing dep exits 1 with a clear error
- All existing tests pass unchanged

Related TODOS: two tasks added 2026-04-02 for design decision + implementation

---

## What is NOT in scope

Hold these for M3 or later:

- `.test.spec` (molecule/organism tests)
- Passports and evidence collection
- Full graph resolution and cycle detection (beyond the warning pass in D7)
- Multiple target languages
- Reverse ingestion
- Planning integration
- IDE/LSP layer
- Rich scheduling or organism-level verification

---

## Sequencing rationale

```
D1 (output hardening) → unblocks safe iteration on everything else
D2 (dep model split)  → required for D4 (cargo check) to be honest
D3 (syn body check)   → independent, can land in parallel with D2
D4 (cargo check)      → depends on D1 + D2; this is the proof point
D5 (local_tests)      → depends on D4 (compiles in same check)
D6 (CUE doc decision) → any time, no dependencies
D7 (dep warnings)     → independent, low-risk, can land after D1
```

Parallel tracks:
- **Track A:** D1 → D2 → D4 → D5
- **Track B:** D3 (runs alongside D2)
- **Track C:** D6, D7 (anytime)

---

## Success criteria (definition of done)

| Check | Pass condition |
|-------|---------------|
| Output safety | `spec generate --output /tmp/x` fails before writing anything |
| No stale files | Generate ecommerce, then generate a different spec set — no ecommerce dirs remain |
| Body alignment | `id: pricing/foo` + `pub fn bar()` fails validation |
| Cargo check | `cargo test` in spec-cli runs cargo check on generated ecommerce output and passes |
| Decimal import | `apply_tax.unit.spec` compiles with external Decimal import, no broken use statements |
| Warnings | `validate` on a spec with an unknown dep prints a warning, exits 0 |
| Strict mode | `validate --strict` on same spec exits 1 |
| local_tests | Generated `.rs` has `#[cfg(test)]` block matching local_tests entries |

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | — |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**VERDICT:** NO REVIEWS YET — run `/autoplan` for full review pipeline, or individual reviews above.
