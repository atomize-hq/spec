# Next Work

Status: **Implementation Ready** (pending PLAN.md corrections)

## CORRECTIONS FROM /plan-eng-review (2026-04-10)

### Removed from scope (already implemented)
- ❌ **Passport Evidence Preservation** — Already implemented in `commands.rs:704-743` and regression-tested
- ❌ **Module Filter Fix** — Already fixed in `commands.rs:1001-1024`

### File path corrections
- `spec-core/src/errors.rs` → Implement in `spec-core/src/lib.rs` (SpecError lives there)
- `spec-cli/src/pipeline.rs` → `spec-core/src/pipeline.rs` (cargo execution lives there)

### New item added
- **Concurrent process detection/warning** — Add warning in `finalize_passports` when multiple spec processes may be writing passports (warn, don't implement full locking)

### Revised Success Criteria
- ✅ Test: `spec build` does not overwrite `spec test` evidence (passport preservation)
- ✅ Test: Timeout configuration respected and terminates hung cargo processes
- ✅ Test: Stable error codes present in `spec validate --format json` output
- ✅ Test: Provenance fields (commit SHA) written when git available

---

## Current Focus: M5 Completion

**Goal:** Stabilize core infrastructure and ship AI verification foundation for solo engineer/2-5 person AI-heavy teams.

### Revised M5 Scope (Approved via /autoplan)

#### Priority 1: Trust & Stability

**Implementation Strategy:** Risk-first. Eliminate the two highest-trust failure modes first: hung cargo subprocesses and unstable machine-readable JSON contracts.

**Feature Brief**
- **Goal:** Make `spec` safe to run inside CI and AI loops by bounding build/test execution time and stabilizing the JSON error/export surface.
- **Why now:** A hung `spec build` or `spec test` blocks all follow-on validation, and unstable JSON codes/schema versions make automation brittle.
- **Primary user:** Solo engineer or 2-5 person AI-heavy team relying on `spec validate --format json`, `spec status --format json`, and exported bundles as machine-readable inputs.
- **In scope:** `[pipeline] timeout_secs` configuration, timeout enforcement in cargo execution, stable `SPEC_*` JSON error codes, numeric `schema_version` alignment, fixture/test refresh, and `AGENTS.md` contract updates.
- **Out of scope:** Evidence provenance, concurrent-process warnings, refactors that do not change trust/stability behavior, and performance-only optimizations.
- **Success criteria:** `spec build`/`spec test` cannot hang indefinitely, JSON error codes are exhaustively mapped to `SPEC_*`, `validate`/`status`/`export` all emit numeric `schema_version`, and regression tests lock the contract.

**Vertical Slices**

**S1. Timeout Guardrail For Cargo Execution** (Effort: S, ~2-3 hours)
- **User value:** `spec build` and `spec test` terminate predictably instead of hanging forever.
- **Scope in:** Config parsing for `pipeline.timeout_secs`, timeout enforcement in cargo execution, explicit timeout error/reporting, and tests covering a hung subprocess path.
- **Scope out:** Retry policies, per-command timeout overrides, and process locking.
- **Acceptance criteria:** A configured timeout stops a hung cargo subprocess, returns a deterministic failure, and preserves partial results/error reporting well enough for the caller to understand what timed out.
- **Verification:** Targeted pipeline/config tests plus `cargo test --all`.
- **Rollout/flags:** No feature flag; use `pipeline.timeout_secs` as the single contract key.

**Atomic Tasks**
- **S1.T1 Config contract for timeout support**
  - **Outcome:** `spec.toml` supports `[pipeline] timeout_secs` with one exact key and one parsed value flowing into the pipeline layer.
  - **Inputs/outputs:** Input: current config loading in `spec-cli/src/config.rs`. Output: concrete config field `pipeline.timeout_secs` with defaulting/validation rules documented in code/tests.
  - **Implementation notes:** Touch `spec-cli/src/config.rs` only for config shape/parsing; do not add CLI flags in this slice.
  - **Acceptance criteria:** Missing key falls back to the existing no-timeout or chosen default behavior; invalid values fail with a clear config error.
  - **Test notes:** Extend config tests for present/missing/invalid timeout cases.
  - **Risk/rollback notes:** Keep the key additive so removing the field restores prior behavior.
- **S1.T2 Timeout enforcement in cargo execution**
  - **Outcome:** The pipeline stops waiting forever on cargo subprocesses.
  - **Inputs/outputs:** Input: parsed timeout value from `S1.T1`. Output: timeout-aware execution path in `spec-core/src/pipeline.rs` and a deterministic surfaced error when timeout is exceeded.
  - **Implementation notes:** Use `wait_timeout` in `spec-core/src/pipeline.rs`; keep stdout/stderr capture and partial-result handling intact.
  - **Acceptance criteria:** A hung build/test path exits on timeout, the subprocess is terminated, and the caller can distinguish timeout from ordinary command failure.
  - **Test notes:** Add/extend pipeline tests for success-under-timeout and timeout-triggered failure.
  - **Risk/rollback notes:** Highest-risk change in Priority 1; isolate to the pipeline layer so rollback is one-file plus dependency removal.
- **S1.T3 End-to-end timeout regression coverage**
  - **Outcome:** The timeout contract is locked with CLI-visible regression tests.
  - **Inputs/outputs:** Input: timeout-aware pipeline behavior from `S1.T2`. Output: tests that prove `spec build`/`spec test` no longer hang indefinitely.
  - **Implementation notes:** Prefer existing test harnesses over bespoke scripts; keep assertions about timeout behavior stable and fast.
  - **Acceptance criteria:** CI has at least one regression test that would fail if cargo waits forever again.
  - **Test notes:** Run targeted timeout tests, then `cargo test --all`.
  - **Risk/rollback notes:** If timing-based assertions prove flaky, replace them with deterministic harness control before shipping.

**S2. Stable JSON Error Contract** (Effort: S, ~3-4 hours)
- **User value:** Automation can key off `SPEC_*` error codes without parsing prose or chasing enum renames.
- **Scope in:** `SpecError` to JSON mapping, exhaustive code coverage, fixture updates for `validate` and `status`, and `AGENTS.md` contract language.
- **Scope out:** Refactoring `spec_error_to_json_entry` structure unless needed to make the mapping exhaustive.
- **Acceptance criteria:** All JSON-visible error variants emit non-empty `SPEC_*` codes, fixtures match the new namespace, and docs instruct consumers to use the stable code field.
- **Verification:** Unit tests for exhaustive mappings, fixture-based CLI tests, and manual inspection that JSON mode stdout stays machine-readable.
- **Rollout/flags:** Breaking contract change; update all fixtures/docs in the same change.

**Atomic Tasks**
- **S2.T1 Define the stable `SPEC_*` namespace**
  - **Outcome:** Every JSON-visible error variant has one explicit SCREAMING_SNAKE_CASE code with `SPEC_` prefix.
  - **Inputs/outputs:** Input: current `SpecError` variants and `spec_error_to_json_entry` implementation. Output: complete mapping table enforced by tests.
  - **Implementation notes:** Touch `spec-core/src/lib.rs` only as needed to understand the variant surface; keep the authoritative mapping in `spec-cli/src/commands.rs`.
  - **Acceptance criteria:** No fallback or empty codes exist; new tests fail if an unmapped variant is added later.
  - **Test notes:** Add/extend a unit test that checks every mapped code begins with `SPEC_` and is non-empty.
  - **Risk/rollback notes:** Contract-breaking rename; ship fixtures/docs/tests together.
- **S2.T2 Lock `validate` and `status` JSON outputs**
  - **Outcome:** CLI JSON fixtures and integration tests match the new stable codes on both happy and error paths.
  - **Inputs/outputs:** Input: stable mapping from `S2.T1`. Output: refreshed fixture files in `spec-cli/tests/fixtures/*.json` and refreshed CLI assertions.
  - **Implementation notes:** Cover `validate --format json` happy + error paths and `status --format json` at least on the happy/stale paths already represented by fixtures.
  - **Acceptance criteria:** Fixture comparisons and JSON parsing tests pass with the new codes and no human-only stdout noise.
  - **Test notes:** Run the existing CLI JSON tests plus any new fixture assertions.
  - **Risk/rollback notes:** Low code risk, high contract visibility; avoid partial fixture updates.
- **S2.T3 Update consumer-facing contract documentation**
  - **Outcome:** `AGENTS.md` and nearby plan text describe the stable `SPEC_*` machine-code contract accurately.
  - **Inputs/outputs:** Input: finalized namespace from `S2.T1`. Output: doc text that tells downstream tooling to read `code` values, not prose.
  - **Implementation notes:** Keep the guidance tight; document the stable prefix and leave examples aligned with the final mapping.
  - **Acceptance criteria:** Documentation matches the emitted JSON contract and does not reference legacy names.
  - **Test notes:** Doc review only.
  - **Risk/rollback notes:** None beyond contract drift if skipped.

**S3. Schema Version Alignment Across Export And JSON Surfaces** (Effort: XS-S, ~1 hour)
- **User value:** Consumers see one numeric schema-version shape instead of a mixed integer/string contract.
- **Scope in:** `spec-core/src/export.rs` type normalization, export tests, and any CLI assertions that currently expect `"1.0"`.
- **Scope out:** Semantic schema-version changes or version bumps beyond normalizing the existing version representation.
- **Acceptance criteria:** Export bundle `schema_version` is serialized as `1` (numeric) and all tests/fixtures agree on that type.
- **Verification:** Export unit tests, CLI export tests, and `cargo test --all`.
- **Rollout/flags:** Fold into Priority 1 so the machine-readable contract changes land together.

**Atomic Tasks**
- **S3.T1 Normalize export schema version to `u8`**
  - **Outcome:** Export bundles no longer serialize `schema_version` as the string `"1.0"`.
  - **Inputs/outputs:** Input: `spec-core/src/export.rs` current `String` field. Output: numeric `schema_version: u8` and updated constructor/tests.
  - **Implementation notes:** Keep `spec_version` untouched; only normalize the export schema field.
  - **Acceptance criteria:** Exported JSON uses `1` and existing tests are updated to distinguish schema version from spec version.
  - **Test notes:** Update `spec-core/src/export.rs` tests and `spec-cli/tests/cli.rs` export assertions.
  - **Risk/rollback notes:** Contract-breaking only for consumers expecting a string; land alongside JSON contract updates.
- **S3.T2 Final contract sweep for numeric schema version**
  - **Outcome:** No Priority 1 JSON surface still emits or expects a string schema version.
  - **Inputs/outputs:** Input: `S2` and `S3.T1` completed. Output: final assertions/fixtures covering `validate`, `status`, and `export`.
  - **Implementation notes:** This is the integration closeout task for the Priority 1 contract surface.
  - **Acceptance criteria:** Repo-wide search and test suite show no remaining `"schema_version": "1.0"` expectation on active JSON outputs.
  - **Test notes:** Run targeted schema-version tests, then `cargo test --all`.
  - **Risk/rollback notes:** Keep this task last within Priority 1 so it validates the complete surface.

**Sub-task Checklists**

**S1.T1 Checklist**
- Add `pipeline.timeout_secs` to the config model in `spec-cli/src/config.rs`.
- Define exact parse/default behavior in tests before wiring the field through.
- Verify existing config loads still succeed when the key is absent.

**S1.T2 Checklist**
- Add `wait_timeout` to `spec-core/Cargo.toml`.
- Update cargo execution in `spec-core/src/pipeline.rs` to wait with the configured timeout.
- Terminate timed-out subprocesses and preserve enough stderr/stdout context for diagnosis.
- Add/adjust pipeline tests for timeout and non-timeout paths.

**S1.T3 Checklist**
- Extend the CLI or pipeline harness with one deterministic hung-process regression case.
- Assert the command exits with timeout-specific failure behavior.
- Run targeted tests and then `cargo test --all`.

**S2.T1 Checklist**
- Enumerate all JSON-visible error variants from the current `SpecError` surface.
- Rename each emitted code to `SPEC_*` in `spec_error_to_json_entry`.
- Add an exhaustive test that fails on missing mappings or non-`SPEC_` codes.

**S2.T2 Checklist**
- Refresh `validate-invalid.json`, `validate-valid.json`, `status-stale.json`, and `status-valid.json`.
- Update CLI JSON tests for happy and error paths to assert the new codes.
- Re-run fixture-backed tests and confirm JSON mode stdout remains parseable.

**S2.T3 Checklist**
- Update `AGENTS.md` to call out stable `SPEC_*` machine codes.
- Remove or rewrite any plan text that still references legacy error-code names.
- Verify docs/examples match the final emitted codes.

**S3.T1 Checklist**
- Change export `schema_version` from `String` to `u8` in `spec-core/src/export.rs`.
- Update constructor logic and export unit tests.
- Update CLI export assertions that currently expect `"1.0"`.

**S3.T2 Checklist**
- Search for lingering string-based schema-version expectations.
- Update final fixtures/assertions across `validate`, `status`, and `export`.
- Run `cargo test --all` as the Priority 1 closeout gate.

**Dependency Graph (text)**
- `S1.T1` blocks `S1.T2`.
- `S1.T2` blocks `S1.T3`.
- `S2.T1` blocks `S2.T2` and `S2.T3`.
- `S3.T1` blocks `S3.T2`.
- `S2` blocks `S3.T2` because the final schema-version sweep must validate the same machine-readable contract surface.
- `S1` blocks the Priority 1 closeout checkpoint because hung commands would undermine all later validation.

**Risks / Unknowns**
- Timeout tests can become flaky if they depend on wall-clock timing alone. De-risk by using the existing harness plus deterministic subprocess control.
- Some error variants may not currently be exercised by fixture-backed CLI tests. De-risk by adding the exhaustive unit test before refreshing fixtures.
- Export consumers may implicitly expect a string schema version. De-risk by landing the numeric change with tests and doc updates in the same Priority 1 batch.

**Milestones**
- **M1:** `S1` complete, so `spec build`/`spec test` cannot hang indefinitely.
- **M2:** `S2` complete, so `validate`/`status` JSON codes are stable for automation.
- **M3:** `S3` complete, so all Priority 1 JSON surfaces share one numeric schema-version contract.

**Workstreams**
- **WS1: Pipeline Guardrail** — Touch surface: `spec-cli/src/config.rs`, `spec-core/src/pipeline.rs`, related config/pipeline tests. Owns `S1.T1`, `S1.T2`, `S1.T3`.
- **WS2: JSON Error Contract** — Touch surface: `spec-cli/src/commands.rs`, `spec-core/src/lib.rs`, `spec-cli/tests/fixtures/*.json`, `AGENTS.md`. Owns `S2.T1`, `S2.T2`, `S2.T3`.
- **WS3: Export Contract Alignment** — Touch surface: `spec-core/src/export.rs`, `spec-cli/tests/cli.rs`. Owns `S3.T1`, `S3.T2`.
- **WS-INT: Priority 1 Integration** — Touch surface: shared contract tests and final `cargo test --all` run. Depends on `WS1`, `WS2`, and `WS3`; owns the final closeout sweep if concurrent work creates test or fixture overlap.

#### Priority 2: Minimal Infrastructure

**Evidence Provenance (Passport v3) - MINIMAL SCOPE** (Effort: S, ~2-3 hours)
- Add commit SHA and timestamp to passport evidence
- Update `spec export` to include provenance
- Skip runner identity/env fingerprint (oversold trust gain for complexity)
**Files:** `spec-core/src/passport.rs`, `spec-core/src/pipeline.rs`, `spec-core/src/export.rs`

#### Priority 3: Code Quality

**Error Handling Refactors** (Effort: XS-S, ~1-2 hours per item)
1. ~~**push_error/push_warning helper**~~ — ✅ Already implemented at `commands.rs:1209-1220`
2. **test_command passport finalization** - Extract ~60 line shared logic between paths
3. **spec_error_to_json_entry** - Replace 9-tuple with `ErrorFields` struct (if time permits)
**Files:** `spec-cli/src/commands.rs`

**Concurrent Process Warning** (Effort: XS, ~30 minutes)
- Add detection/warning when multiple spec processes may be writing passports simultaneously
- Warn instead of implementing full locking (single-writer assumption is valid for ICP)
- Success criterion: warning is emitted when >1 spec process detected — NOT "no data loss"
- Note: detection is best-effort (pgrep/process enumeration is platform-specific and itself racy)
**Files:** `spec-cli/src/commands.rs`

#### Priority 4: Optional Improvements

**parse_test_output() HashMap Optimization** (Effort: XS, ~30 minutes)
- Build HashMap of test IDs before scanning (O(lines) vs O(lines × units))
- Only if benchmarks show benefit for typical repos
**Files:** `spec-cli/src/pipeline.rs`

#### DO NOT SHIP IN M5

**❌ Cross-Library Dependency Implementation** → **DEFERRED TO M7**
- Reason: Premature scaling without package management features (versioning, lockfile, publish)
- Filesystem coupling disguised as architecture
- ICP doesn't need this yet
- Decision: Use exported bundles as package boundary instead

**❌ ValidatedExpr Newtype Refactor (Public API)** → **BLOCKED**
- Reason: `spec-core` is published library; `ResolvedSpec` is public API
- Replacing `String` with `syn::Expr` breaks semver
- Options: Keep internal String + validation, or make ValidatedExpr internal-only
- Revisit in M7 if needed

**❌ Full Evidence Provenance** → **DEFERRED TO M6**
- Runner identity, environment fingerprint add complexity without trust boundary
- Gitignored local JSON files are untrusted medium regardless
- Wait until passports become authoritative artifacts (not cache files)

### Implementation Order

1. **Priority 1 / S1: Timeout Guardrail** (prevents CI hangs — finish before relying on any other M5 validation)
2. **Priority 1 / S2: Stable Error Codes** (breaking rename: update fixtures + `AGENTS.md` in the same change)
3. **Priority 1 / S3: schema_version type fix** (fold into the JSON contract sweep: `export.rs` → `u8`, not String)
4. **Priority 2: Minimal Provenance** (adds CI metadata, low complexity)
5. **Priority 3: Refactors** (improve code quality; note: `push_error`/`push_warning` already done — remove from scope)
6. **Priority 3: Concurrent Process Warning** (warn on concurrent runs; success criterion = warning emitted, not data-loss prevention)
7. **Priority 4:** Optional `parse_test_output()` optimization (performance)

### Success Criteria

- ✅ Zero regression in existing tests (50+ unit + integration tests)
- ✅ New tests for all added functionality
- ✅ JSON API contracts stable and documented
- ✅ Concurrent run warning emitted when >1 spec process detected (warn-only, not a lock)
- ✅ No indefinite hangs during build/test
- ✅ Type-safe error handling patterns established

### Deferred to Future Milestones

**M6 - AI Governance Layer**
- Semantic contract-vs-body comparison (LLM eval)
- Full evidence provenance with CI context
- First AI verification surface (`spec eval` or `spec review`)

**M7 - Scaling & Distribution**
- Cross-library dependencies with namespace-prefixed schema
- Exported bundle-based package boundaries
- ValidatedExpr internal refactor (if API surface permits)

**Future - Performance**
- BTreeMap → HashMap replacement (measure first)
- Other optimizations based on real-world usage

---

## For Reference

- Original TODOS.md backlog: view with `cat TODOS.md` (contains completed/completed items)
- Completed M5 features: see `.implemented/` directory
- Current version: v0.5.1 (post M5 AI-native loop)
- ICP: Solo engineer or 2-5 person AI-heavy team where correctness matters

## To start implementation

1. **Run any pre-implementation checks:**
   ```bash
   cargo test --all
   ```

2. **Begin with Priority 1 / S1 (Timeout Guardrail):**
   - Read `spec-core/src/pipeline.rs` and `spec-cli/src/config.rs`
   - Define the exact `pipeline.timeout_secs` contract
   - Implement timeout enforcement
   - Add regression tests before moving to the JSON contract work

3. **After each item, run:**
   ```bash
   cargo test --all
   ```

4. **Ready for review:**
   ```bash
   /ship
   ```

---

**Document version:** 2026-04-11  
**Review status:** Approved via /plan-eng-review  
**Next review checkpoint:** Before /ship

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 4 | issues_open (stale, e533140) | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 7 | issues_found | schema_version type mismatch, impl order wrong, concurrency criterion misleading |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 8 | **CLEAR (PLAN)** | 7 issues, 1 critical gap, all resolved |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | issues_open (stale) | — |

**CODEX:** 4 cross-model tensions: (1) error code rename vs freeze → user chose rename; (2) schema_version integer/string inconsistency → normalize to integer everywhere; (3) concurrent warning criterion misleading → corrected to "warning emitted"; (4) impl order wrong → timeout moved to Priority 1.
**UNRESOLVED:** 0
**VERDICT:** ENG CLEARED — ready to implement. Run `cargo test --all` before starting, then implement in priority order: Timeout → Error Codes (+ schema_version fix + JSON tests) → Provenance → Refactors → Concurrent Warning.
