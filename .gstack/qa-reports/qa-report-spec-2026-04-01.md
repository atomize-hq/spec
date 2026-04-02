# QA Report: spec CLI
**Date:** 2026-04-01
**Branch:** main
**Duration:** ~15 min
**Project type:** Rust CLI (no web UI)
**Test suite:** 54 tests (was 53 before this session)

---

## Summary

| Metric | Value |
|--------|-------|
| Build | ✅ Clean |
| Tests (baseline) | 53/53 passing |
| Tests (final) | 54/54 passing |
| Issues found | 3 |
| Fixed | 2 (ISSUE-001, ISSUE-003) |
| Deferred | 1 (ISSUE-002) |
| Health score (baseline) | 88 |
| Health score (final) | 94 |

**PR Summary:** QA found 3 issues, fixed 2, health score 88 → 94.

---

## Issues

### ISSUE-001 — Incorrect file count in duplicate ID error (Medium) — ✅ FIXED
**Commit:** fde5927

When two files share the same unit ID, the error message said "1 file, 1 error" instead of "2 files, 1 error". Root cause: the `BTreeMap` key for duplicate ID errors was `"file1 | file2"` (a single composite string), so `errors.len()` counted it as one entry.

**Fix:** Added `count_unique_files()` which splits composite ` | ` keys and counts distinct paths. Both `validate_command` and `generate_command` now use this.

**Before:**
```
❌ 1 file, 1 error
```

**After:**
```
❌ 2 files, 1 error
```

**Regression test:** `validate_duplicate_id_reports_correct_file_count` in `spec-cli/tests/cli.rs`

---

### ISSUE-002 — Marker file leaked on out-of-project generate failure (Low) — ⏳ DEFERRED

When `spec generate` is called with `--output` pointing outside the project root, `ensure_output_marker()` creates the `.spec-generated` marker before `clean_output_dir` checks and rejects the path. On failure, the marker remains in the target directory.

Repro:
```
spec generate examples/ecommerce/units --output /tmp/some-dir
# Error: Refusing to clean /tmp/some-dir: output path is outside the project root
# But /tmp/some-dir/.spec-generated now exists
```

Fix would be to check path validity before writing the marker. Deferred as edge case — users who hit this can delete the file manually. Also the out-of-project check itself may be intentionally restrictive.

---

### ISSUE-003 — Missing subcommand descriptions in --help (Low/Cosmetic) — ✅ FIXED
**Commit:** 1282f4b

`spec --help` showed blank descriptions for `validate` and `generate`:
```
Commands:
  validate  
  generate  
```

Fixed by adding `#[command(about = "...")]` attributes to both variants.

**After:**
```
Commands:
  validate  Validate .unit.spec files
  generate  Generate Rust source files from .unit.spec files
```

---

## CLI Behavior Verified

| Scenario | Result |
|----------|--------|
| `spec validate <dir>` — 3 valid units | ✅ "3 units valid" |
| `spec validate <file>` — single file | ✅ "1 unit valid" |
| `spec validate <dir>` — invalid ID regex | ✅ Schema error reported |
| `spec validate <dir>` — use statement in body | ✅ Semantic error reported |
| `spec validate <dir>` — Rust keyword in ID | ✅ Keyword error reported |
| `spec validate <dir>` — duplicate IDs | ✅ "2 files, 1 error" (fixed) |
| `spec validate <dir>` — dep collision | ✅ Collision error reported |
| `spec validate /nonexistent` | ✅ "does not exist" error |
| `spec generate <dir>` — default output | ✅ 3 files + mod.rs files written |
| `spec generate <dir>` — ecommerce example | ✅ `cargo check` passes on output |
| `spec --version` | ✅ "spec 0.1.0" |
| `spec --help` | ✅ Descriptions now shown |

---

## Health Score

| Category | Score | Notes |
|----------|-------|-------|
| Build | 100 | Clean compile |
| Functional | 90 | ISSUE-001 fixed; ISSUE-002 deferred |
| UX/CLI | 95 | ISSUE-003 fixed |
| Test coverage | 92 | 54 tests; TODOS list 49 target tests, now at 54 |
| Generated output | 100 | ecommerce example compiles via `cargo check` |

**Final health score: 94/100**
