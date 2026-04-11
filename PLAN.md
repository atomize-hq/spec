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

**Implementation Strategy:** Dependency-first (minimal). Add the smallest provenance contract that improves traceability for `spec test` and `spec export` without widening the trust surface into runner identity, environment fingerprinting, or other CI-only metadata.

**Feature Brief**
- **Goal:** Let downstream tooling identify which git revision produced passport evidence and export bundles.
- **Why now:** Priority 1 makes the machine-readable surface stable; the next trust gain is being able to correlate a passing or failing passport/export artifact with a concrete repo state.
- **Primary user:** Solo engineer or 2-5 person AI-heavy team using passports and `spec export` as machine-readable context inside local loops or CI.
- **In scope:** One shared provenance contract carrying `git_commit_sha`, reuse of existing timestamp fields (`observed_at`, `generated_at`, `exported_at`) instead of adding duplicate timestamp keys, passport evidence wiring for `spec test`, top-level export provenance, and regression coverage for git-present and git-absent paths.
- **Out of scope:** Runner identity, environment fingerprint, branch name, dirty-worktree status, signed attestations, process locking, and any provenance collection that can fail the primary command.
- **Success criteria:** `spec test` writes provenance when git is available, omits it cleanly when unavailable, `spec build`/`spec generate` preserve prior evidence provenance, `spec export` emits top-level provenance, and regression tests lock both success and no-git behavior.
- **Contract decision:** Add one shared optional object named `provenance` with the exact field `git_commit_sha`. Keep timestamps where they already live: `PassportEvidence.observed_at`, `Passport.generated_at`, and `ExportBundle.exported_at`.

**Vertical Slices**

**S1. Passport Evidence Provenance Contract** (Effort: XS-S, ~1-2 hours)
- **User value:** A passport produced by `spec test` can be traced back to the commit that generated its observed evidence.
- **Scope in:** Provenance schema in passport types, git SHA capture during `spec test`, and preservation of provenance when later `spec generate`/`spec build` rewrites the same passport.
- **Scope out:** New timestamps, runner metadata, export bundle wiring, and any change that makes git availability mandatory.
- **Acceptance criteria:** A successful or failing `spec test` writes `evidence.provenance.git_commit_sha` when the workspace is in git, leaves `provenance` absent when git is unavailable, and later non-test passport rewrites preserve the full `evidence` object.
- **Verification:** Passport serde/unit tests plus CLI integration tests for success, failure, and preservation flows.
- **Rollout/flags:** Additive schema only; no feature flag.

**Atomic Tasks**
- **S1.T1 Define the minimal provenance schema**
  - **Outcome:** Passport evidence has one exact additive contract for provenance.
  - **Inputs/outputs:** Input: `spec-core/src/passport.rs` current `PassportEvidence` struct. Output: a shared `provenance` object with the single field `git_commit_sha`.
  - **Implementation notes:** Use an optional nested object, not a loose top-level string; keep `observed_at` as the only evidence timestamp and do not introduce `captured_at` or similar duplicates.
  - **Acceptance criteria:** Existing passports without provenance continue to deserialize, new passports serialize deterministically, and no empty provenance object is emitted.
  - **Test notes:** Extend passport round-trip and omit-empty-field tests.
  - **Risk/rollback notes:** Low risk; additive serde-only change.
- **S1.T2 Capture git SHA during `spec test`**
  - **Outcome:** Test-generated evidence includes the commit SHA when the command is run inside a git repo.
  - **Inputs/outputs:** Input: `spec-cli/src/commands.rs` test/build failure evidence flow. Output: one resolved SHA per command, threaded into success and failure evidence creation.
  - **Implementation notes:** Resolve the SHA once per command via git, treat lookup failure as non-fatal, and use the same field name on both success and failure paths.
  - **Acceptance criteria:** `spec test` never fails just because git lookup failed; when git is available, the emitted SHA is a full commit hash and appears on every written passport in that run.
  - **Test notes:** Add CLI integration coverage for a git-backed temp repo and a non-git temp dir.
  - **Risk/rollback notes:** Git-backed tests must not depend on global user config; set repo-local config in test setup.
- **S1.T3 Preserve provenance on later non-test rewrites**
  - **Outcome:** `spec generate` and `spec build` do not erase provenance seeded by an earlier `spec test`.
  - **Inputs/outputs:** Input: existing evidence-preservation path in `write_passports`. Output: regression coverage proving provenance survives generate/build rewrites.
  - **Implementation notes:** Reuse the current preserve-existing-evidence behavior rather than adding special-case provenance copying.
  - **Acceptance criteria:** Existing preservation tests fail if provenance disappears after `spec generate` or `spec build`.
  - **Test notes:** Extend the current passport-preservation CLI tests to assert `git_commit_sha` remains present.
  - **Risk/rollback notes:** None if the work stays inside the existing evidence-preservation seam.

**S2. Export Provenance Surface** (Effort: XS-S, ~1 hour)
- **User value:** Consumers inspecting only the export bundle can see the revision context without first opening individual passports.
- **Scope in:** Top-level export provenance contract, wiring from the export command, and regression tests covering export with and without git context.
- **Scope out:** Per-unit export provenance duplication, branch metadata, and changes to warning semantics.
- **Acceptance criteria:** `spec export` emits a top-level `provenance.git_commit_sha` when git is available, omits `provenance` when unavailable, and continues to include passport data and warnings unchanged.
- **Verification:** Export unit tests plus CLI export integration tests.
- **Rollout/flags:** Additive JSON field only; land with test updates.

**Atomic Tasks**
- **S2.T1 Add top-level export provenance**
  - **Outcome:** The export bundle advertises the same minimal provenance contract as passports.
  - **Inputs/outputs:** Input: `spec-core/src/export.rs` current `ExportBundle` and `spec-cli/src/commands.rs` export flow. Output: optional top-level `provenance` object carrying `git_commit_sha`.
  - **Implementation notes:** Use the same field name and struct shape as the passport contract; keep `exported_at` as the export timestamp.
  - **Acceptance criteria:** Export JSON remains stable except for the additive provenance field, and passport loading/warning behavior is unchanged.
  - **Test notes:** Update export unit tests and CLI assertions.
  - **Risk/rollback notes:** Low risk; most likely failure mode is passport/export field-name drift, so share the type where practical.
- **S2.T2 Close out export regression coverage**
  - **Outcome:** The Priority 2 contract is locked across CLI-visible success paths.
  - **Inputs/outputs:** Input: completed passport and export provenance wiring. Output: deterministic tests proving git-present and git-absent export behavior.
  - **Implementation notes:** Prefer existing export test helpers and temp-project fixtures over bespoke harness logic.
  - **Acceptance criteria:** CI has one export assertion that proves provenance is present in a git repo and one that proves omission does not fail export outside git.
  - **Test notes:** Run targeted export/passport provenance tests, then `cargo test --all`.
  - **Risk/rollback notes:** If git setup in tests is flaky, keep the assertions focused on field presence/absence and avoid brittle commit-message assumptions.

**Sub-task Checklists**

**S1.T1 Checklist**
- Add a shared provenance struct in `spec-core/src/passport.rs`.
- Attach it to `PassportEvidence` as `provenance: Option<_>`.
- Add serde tests proving legacy passports still parse and empty provenance is omitted.

**S1.T2 Checklist**
- Add one helper in `spec-cli/src/commands.rs` to resolve `git rev-parse HEAD`.
- Thread the optional SHA through both success and failure evidence builders.
- Add one git-backed CLI test and one non-git CLI test for `spec test`.

**S1.T3 Checklist**
- Extend the existing evidence-preservation tests for `spec generate`.
- Extend the existing evidence-preservation tests for `spec build`.
- Assert `evidence.provenance.git_commit_sha` survives both rewrites unchanged.

**S2.T1 Checklist**
- Add optional top-level `provenance` to `ExportBundle` in `spec-core/src/export.rs`.
- Pass the optional SHA from `export_command` into `build_export_bundle`.
- Update export unit tests so the additive field is covered without changing warning/passport expectations.

**S2.T2 Checklist**
- Extend CLI export tests for git-present provenance.
- Add a no-git export regression test to prove omission is non-fatal.
- Run targeted provenance tests and then `cargo test --all`.

**Dependency Graph (text)**
- `S1.T1` blocks `S1.T2` and `S2.T1`.
- `S1.T2` blocks `S1.T3` because preservation only matters once provenance is written.
- `S2.T1` blocks `S2.T2`.
- `S1.T3` and `S2.T2` together close out Priority 2.

**Risks / Unknowns**
- Git-based integration tests can be flaky if they rely on global git config. De-risk by initializing the temp repo and setting local `user.name` / `user.email` in test setup.
- Passport and export contracts could drift if they define provenance separately. De-risk by using one shared struct/field name.
- Non-git directories are a first-class use case for local experimentation. De-risk by treating git lookup as best-effort and asserting omission, not failure.

**Milestones**
- **M4:** `S1` complete, so passport evidence carries commit provenance and preserves it across later rewrites.
- **M5:** `S2` complete, so export bundles expose the same provenance context for downstream tooling.

**Workstreams**
- **WS1: Passport Provenance** — Touch surface: `spec-core/src/passport.rs`, `spec-cli/src/commands.rs`, passport-focused CLI tests. Owns `S1.T1`, `S1.T2`, `S1.T3`.
- **WS2: Export Provenance** — Touch surface: `spec-core/src/export.rs`, `spec-cli/src/commands.rs`, export-focused CLI tests. Owns `S2.T1`, `S2.T2`.
- **WS-INT: Priority 2 Integration** — Touch surface: shared CLI regression suite and final `cargo test --all` closeout. Depends on `WS1` and `WS2`.

#### Priority 3: Code Quality

**Implementation Strategy:** Maintainability-first. First collapse the duplicated passport-finalization branches in `test_command`, then replace positional JSON error assembly with a typed helper, and only then layer in the warn-only concurrent-process signal at the shared passport-finalization seam.

**Feature Brief**
- **Goal:** Reduce maintenance risk in `spec-cli/src/commands.rs` by removing duplicated control flow, replacing brittle tuple-based JSON assembly with named fields, and surfacing a best-effort warning when concurrent `spec` runs may rewrite passports.
- **Why now:** Priority 1 and Priority 2 both expanded the passport/evidence and JSON-contract surfaces. Leaving the current duplication in place makes later changes error-prone, and the same finalization seam is the right place to add the concurrent-run warning without scattering process checks across commands.
- **Primary user:** Maintainers extending `spec build`, `spec test`, `spec validate --format json`, and passport-writing behavior for solo-engineer or small-team AI-heavy workflows.
- **In scope:** `test_command` refactor around passport finalization, `spec_error_to_json_entry` refactor from positional tuple to named `ErrorFields`, best-effort concurrent-process detection and stderr warning near `finalize_passports`, and regression coverage proving no behavioral drift.
- **Out of scope:** `push_error` / `push_warning` helper work (already shipped), file-locking or single-writer guarantees, JSON schema or error-code renames, performance optimizations, and any platform-specific process-management behavior beyond warn-only detection.
- **Success criteria:** `test_command` preserves current directory-mode and file-path-mode behavior while sharing one passport-finalization path, JSON error entries keep the same external contract while becoming field-name driven internally, concurrent writes produce a warning rather than an abort when detected, and normal single-process workflows remain warning-free.

**Vertical Slices**

**S1. `test_command` Passport Finalization Refactor** (Effort: XS-S, ~2-3 hours)
- **User value:** Maintainers can change test/build failure evidence writing in one place instead of keeping multiple branches in sync.
- **Scope in:** Shared helper/plan for target-spec vs directory-spec passport writes, build-failure and post-test success/failure paths, and preservation of current file-path semantics.
- **Scope out:** Behavioral changes to cargo invocation, provenance resolution, contract-hash computation, or zero-tests handling.
- **Acceptance criteria:** The refactor removes the duplicated `if let Some(target_spec)` / `else` passport-writing branches while preserving all current outcomes for build-fail, test-fail, test-pass, and file-path-targeted runs.
- **Verification:** Existing CLI passport tests plus one focused regression proving directory mode and file-path mode still write the intended passport set.
- **Rollout/flags:** No flag; pure internal refactor.

**Atomic Tasks**
- **S1.T1 Extract shared passport-write planning for `spec test`**
  - **Outcome:** `test_command` has one internal helper or small planning struct that determines passport root, affected specs, evidence map, and contract hashes for both target-file and directory modes.
  - **Inputs/outputs:** Input: current duplicated branches in `spec-cli/src/commands.rs`. Output: one shared seam for preparing `finalize_passports(...)` inputs.
  - **Implementation notes:** Keep ownership/borrowing simple; prefer a small explicit helper over a generic abstraction that obscures the file-path vs directory-path behavior.
  - **Acceptance criteria:** The helper makes the write target explicit and does not change which passports are written in single-file mode.
  - **Test notes:** Add or tighten command-level tests around target-only passport writes.
  - **Risk/rollback notes:** Main risk is accidentally widening single-file writes to sibling units; keep that invariant explicit in tests.
- **S1.T2 Refactor build-failure and post-test flows onto the shared seam**
  - **Outcome:** `test_command` uses the same passport-finalization path after cargo build failure and after test-result parsing.
  - **Inputs/outputs:** Input: shared seam from `S1.T1`, existing `build_failure_evidence`, `build_test_evidence`, and `contract_hashes_for`. Output: one refactored `test_command` with less branch duplication and unchanged CLI-visible behavior.
  - **Implementation notes:** Do not refactor timeout handling or cargo output printing in this slice; keep the change bounded to evidence/passport finalization.
  - **Acceptance criteria:** Build-fail, test-pass, and test-fail flows still write the same evidence and contract hashes they do today.
  - **Test notes:** Run the existing passport evidence/provenance/build-failure/file-path regression suite.
  - **Risk/rollback notes:** If the helper grows beyond the passport seam, split it back down rather than turning Priority 3 into a broad command rewrite.
- **S1.T3 Lock refactor safety with focused regression coverage**
  - **Outcome:** The refactor is protected by tests aimed at the exact branch behavior that used to be duplicated.
  - **Inputs/outputs:** Input: refactored `test_command`. Output: tests covering target-only writes, build failure passport writes, and zero-tests/no-passport behavior.
  - **Implementation notes:** Prefer extending existing CLI tests over introducing a second harness.
  - **Acceptance criteria:** CI fails if a future edit regresses file-path targeting or skips failure-passport finalization.
  - **Test notes:** Run targeted CLI tests, then `cargo test --all`.
  - **Risk/rollback notes:** None beyond missing a branch-specific regression; keep the coverage focused on current invariants.

**S2. Typed JSON Error Entry Construction** (Effort: XS, ~1-2 hours)
- **User value:** Maintainers can add or adjust JSON error fields without relying on tuple position discipline.
- **Scope in:** Replace the 9-field tuple in `spec_error_to_json_entry` with a named `ErrorFields` struct or equivalent typed helper, while keeping `JsonErrorEntry` output unchanged.
- **Scope out:** Error-code namespace changes, fixture churn unrelated to the refactor, or broader `validate`/`status` JSON redesign.
- **Acceptance criteria:** `spec_error_to_json_entry` no longer uses the positional tuple, and existing JSON fixtures/tests remain stable unless a test is added purely to lock field mapping behavior.
- **Verification:** Existing JSON fixture tests plus one or two focused unit tests for variants that populate multiple fields.
- **Rollout/flags:** No flag; internal refactor only.

**Atomic Tasks**
- **S2.T1 Introduce a named field carrier for JSON error assembly**
  - **Outcome:** `spec_error_to_json_entry` assembles data into a local `ErrorFields` struct with named fields instead of the current positional tuple.
  - **Inputs/outputs:** Input: current 9-value tuple match in `spec-cli/src/commands.rs`. Output: a typed intermediary that maps cleanly into `JsonErrorEntry`.
  - **Implementation notes:** Keep the carrier local to `commands.rs`; this is not a public type and should not leak into unrelated modules.
  - **Acceptance criteria:** The code path is easier to audit because each field is assigned by name, not position.
  - **Test notes:** Add a small unit test for representative variants such as `DuplicateId`, `DepCollision`, `ContractTypeInvalid`, or `CyclicDep`.
  - **Risk/rollback notes:** Low risk, but tuple-order regressions can hide easily; the new tests should cover the multi-field cases.
- **S2.T2 Prove no JSON contract drift**
  - **Outcome:** The internal refactor ships with explicit proof that emitted JSON remains unchanged.
  - **Inputs/outputs:** Input: typed field carrier from `S2.T1`. Output: passing fixture-backed `validate` / `status` JSON tests and any targeted unit tests.
  - **Implementation notes:** Avoid refreshing fixtures unless a true output difference is discovered and deliberately accepted.
  - **Acceptance criteria:** Existing machine-readable JSON remains byte-for-byte or semantically identical where current tests already pin it.
  - **Test notes:** Run the current CLI fixture suite plus the new unit assertions.
  - **Risk/rollback notes:** If fixtures move unexpectedly, stop and resolve whether the refactor accidentally changed behavior before accepting the diff.

**S3. Warn-Only Concurrent Passport Write Detection** (Effort: XS, ~1 hour)
- **User value:** Operators get a clear signal when overlapping `spec` runs may both rewrite passports, without falsely implying the tool now guarantees safe concurrent writes.
- **Scope in:** Best-effort process detection, one warning emission path near `finalize_passports`, testability of the detector/warning seam, and documentation of warn-only semantics in the plan.
- **Scope out:** Lock files, retry loops, process coordination, or any success criterion stronger than “warning emitted when concurrent activity is detected.”
- **Acceptance criteria:** When the detector sees another likely `spec` process, the command emits a warning and continues; when detection is unavailable or reports a single process, commands behave exactly as they do today.
- **Verification:** One deterministic test using an injectable/testable detection seam plus a manual sanity check that ordinary runs stay quiet.
- **Rollout/flags:** No flag; best-effort warning only.

**Atomic Tasks**
- **S3.T1 Define the best-effort detection contract**
  - **Outcome:** One helper encapsulates “are concurrent `spec` writers likely active?” and degrades safely when process enumeration is unavailable.
  - **Inputs/outputs:** Input: `finalize_passports` write seam in `spec-cli/src/commands.rs`. Output: helper returning enough information to decide whether to warn, without aborting the command on detection failure.
  - **Implementation notes:** Keep the detector isolated from the write logic and make it easy to substitute in tests; unsupported platforms or missing tools should resolve to “no warning,” not failure.
  - **Acceptance criteria:** Detection failures are silent or debug-only internally; they never block passport writes.
  - **Test notes:** Add a unit-test seam so “concurrent process present” can be simulated without racing real commands.
  - **Risk/rollback notes:** Platform-specific process checks are inherently racy; the contract must stay explicitly best-effort.
- **S3.T2 Emit one clear warning at the passport finalization seam**
  - **Outcome:** Commands that call `finalize_passports` share one consistent concurrent-run warning message.
  - **Inputs/outputs:** Input: detection helper from `S3.T1`. Output: stderr warning emitted before or around passport writes when concurrent activity is detected.
  - **Implementation notes:** Emit the warning once per command, not once per spec file, and keep the text explicit that this is advisory rather than protective locking.
  - **Acceptance criteria:** The warning is human-readable, non-fatal, and absent from ordinary single-process runs.
  - **Test notes:** Cover warning emission through the injected seam and verify command success still holds.
  - **Risk/rollback notes:** Over-warning would erode trust; keep the trigger conservative and scoped to likely concurrent writers.

**Sub-task Checklists**

**S1.T1 Checklist**
- Identify the duplicated `finalize_passports(...)` branches in `test_command`.
- Extract the minimal shared inputs needed for target-only and directory-wide writes.
- Preserve the current `spec_root` vs `path` distinction explicitly in the helper.

**S1.T2 Checklist**
- Route build-failure passport writes through the shared seam.
- Route post-test evidence passport writes through the same seam.
- Confirm cargo output, timeout handling, and zero-tests checks remain untouched.

**S1.T3 Checklist**
- Extend the existing target-only passport regression tests.
- Extend the existing build-failure or test-failure passport regression tests.
- Run targeted CLI coverage and then `cargo test --all`.

**S2.T1 Checklist**
- Add a local `ErrorFields` carrier in `spec-cli/src/commands.rs`.
- Replace tuple-position assignments with named field assignments in the `match`.
- Add focused unit tests for multi-field error variants.

**S2.T2 Checklist**
- Re-run fixture-backed `validate --format json` coverage.
- Re-run fixture-backed `status --format json` coverage.
- Confirm no fixture refresh is needed unless behavior truly changed.

**S3.T1 Checklist**
- Add one helper dedicated to concurrent-process detection.
- Add one test seam so process-presence can be simulated deterministically.
- Ensure detector failure returns “no warning” instead of an error.

**S3.T2 Checklist**
- Call the detector from the shared passport-finalization seam.
- Emit one warning per command when concurrent activity is detected.
- Verify commands still succeed and ordinary runs do not emit the warning.

**Dependency Graph (text)**
- `S1.T1` blocks `S1.T2`.
- `S1.T2` blocks `S1.T3`.
- `S2.T1` blocks `S2.T2`.
- `S3.T1` blocks `S3.T2`.
- `S1` should land before `S3` so the concurrent warning attaches to the final shared passport seam instead of duplicated branches.

**Risks / Unknowns**
- The `test_command` refactor can easily regress single-file behavior by broadening passport writes to sibling specs. De-risk with explicit file-path-mode regression coverage before and after the refactor.
- The `ErrorFields` refactor is low-risk externally but can silently swap field population on multi-field variants. De-risk with focused unit tests for the variants that set several optional fields.
- Concurrent-process detection is inherently racy and platform-specific. De-risk by keeping it advisory, isolating it behind one helper, and making unsupported environments resolve to “no warning.”

**Milestones**
- **M6:** `S1` complete, so passport finalization in `spec test` has one authoritative maintenance seam.
- **M7:** `S2` complete, so JSON error construction is field-name driven and safer to extend.
- **M8:** `S3` complete, so overlapping `spec` runs surface a clear advisory warning without changing command success semantics.

**Workstreams**
- **WS1: Test Command Refactor** — Touch surface: `spec-cli/src/commands.rs`, passport-focused CLI tests. Owns `S1.T1`, `S1.T2`, `S1.T3`.
- **WS2: JSON Error Entry Refactor** — Touch surface: `spec-cli/src/commands.rs`, JSON fixture/unit tests. Owns `S2.T1`, `S2.T2`.
- **WS3: Concurrent Warning** — Touch surface: `spec-cli/src/commands.rs`, warning-focused tests. Owns `S3.T1`, `S3.T2`.
- **WS-INT: Priority 3 Integration** — Touch surface: shared `commands.rs` integration and final `cargo test --all` closeout. Depends on `WS1`, `WS2`, and `WS3`.

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
