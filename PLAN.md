# Next Work

Status: **Implementation Ready**

## Current Focus: M5 Completion

**Goal:** Stabilize core infrastructure and ship AI verification foundation for solo engineer/2-5 person AI-heavy teams.

### Revised M5 Scope (Approved via /autoplan)

#### Priority 1: Trust & Stability

**Passport Evidence Preservation** (Effort: S, ~2-3 hours)
- Fix data loss: `spec build`/`spec generate` overwrite passport evidence and contract_hash
- Add merge logic to preserve existing `evidence` and `contract_hash` fields
- Tests for `spec build -> spec test` and `spec test -> spec build` sequences
**Files:** `spec-cli/src/commands.rs`

**Stable External Error Code Namespace** (Effort: S, ~3-4 hours)
- Define stable external error codes (e.g., `SPEC_DEP_NOT_FOUND`) separate from Rust enum variants
- Build mapping from `SpecError` to stable codes
- Add JSON schema versioning strategy for breaking changes
- Verify all variants have mappings with unit tests
**Files:** `spec-core/src/errors.rs`, `spec-cli/src/commands.rs`

#### Priority 2: Minimal Infrastructure

**Evidence Provenance (Passport v3) - MINIMAL SCOPE** (Effort: S, ~2-3 hours)
- Add commit SHA and timestamp to passport evidence
- Update `spec export` to include provenance
- Skip runner identity/env fingerprint (oversold trust gain for complexity)
**Files:** `spec-core/src/passport.rs`, `spec-cli/src/pipeline.rs`, `spec-core/src/export.rs`

**Cargo Timeout Support** (Effort: S, ~2-3 hours)
- Add `wait_timeout` crate
- Add `[pipeline] timeout_secs` config to `spec.toml`
- Prevent indefinite hangs during `spec build`/`spec test`
- Graceful timeout with partial results
**Files:** `spec-cli/src/pipeline.rs`, `Cargo.toml`

#### Priority 3: Code Quality

**Error Handling Refactors** (Effort: XS-S, ~1-2 hours per item)
1. **push_error/push_warning helper** - Extract duplicated 4-line diagnostic collection pattern
2. **test_command passport finalization** - Extract ~60 line shared logic between paths
3. **spec_error_to_json_entry** - Replace 9-tuple with `ErrorFields` struct (if time permits)
**Files:** `spec-cli/src/commands.rs`

**Module Filter Fix** (Effort: XS, ~30 minutes)
- Fix `cargo_test_filter_for` to handle nested output paths correctly
- Use full path relative to output dir instead of just `file_name()`
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

1. **Stable Error Codes** (blocks API contract stability)
2. **Passport Preservation** (fixes user-facing data loss bug)
3. **Minimal Provenance** (adds CI metadata, low complexity)
4. **Timeout Support** (prevents CI hangs, critical for DX)
5. **Refactors** (improve code quality)
6. **Module Filter Fix** (correctness improvement)
7. Optional: `parse_test_output()` optimization (performance)

### Success Criteria

- ✅ Zero regression in existing tests (50+ unit + integration tests)
- ✅ New tests for all added functionality
- ✅ JSON API contracts stable and documented
- ✅ No passport data loss in concurrent runs
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

2. **Begin with Priority 1 (Stable Error Codes):**
   - Read `spec-core/src/errors.rs` to understand current errors
   - Design stable error code mapping
   - Implement mapping function
   - Add tests

3. **After each item, run:**
   ```bash
   cargo test --all
   ```

4. **Ready for review:**
   ```bash
   /ship
   ```

---

**Document version:** 2026-04-10  
**Review status:** Approved via /autoplan  
**Next review checkpoint:** Before /ship
