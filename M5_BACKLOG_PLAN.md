# M5 Backlog - Remaining Items Plan

## M5 Backlog Items (Remaining)

### High Priority

#### Evidence Provenance (Passport v3)
Add commit SHA, runner identity, and environment fingerprint to passport evidence schema. This makes evidence CI-trustworthy, not just locally observed.

**Implementation:**
- Extend passport schema in `spec-core/src/passport.rs` to include provenance fields
- Modify evidence collection in `spec-cli/src/pipeline.rs` to capture CI environment
- Update `spec export` to include provenance data
- Write integration tests verifying CI context detection

**Files:** passport.rs, pipeline.rs, commands/export.rs, tests/

**Estimated effort:** M (medium, ~1 day)

#### D5a Newtype Refactor (ValidatedExpr)
Replace `String` expect in `ResolvedSpec` with `ValidatedExpr` newtype wrapping `syn::Expr`. This eliminates double-parse cost and provides a type-safe API boundary.

**Implementation:**
- Create `ValidatedExpr` newtype in spec-core
- Update `ResolvedSpec` struct to use `ValidatedExpr` instead of `String` for expect expressions
- Modify validator to construct `ValidatedExpr` early in the pipeline
- Refactor generator to use the parsed AST directly, eliminating re-parse

**Files:** types.rs, validator.rs, generator.rs, syntax.rs

**Estimated effort:** M (medium, ~1 day)

#### Cross-Library Dependency Implementation
Implement the namespace-prefixed cross-library dependency schema (`shared::money/round`) using `[libraries]` config in `spec.toml`, including:
- Cross-library dep loading
- Validation across library boundaries
- Use statement generation
- Cycle detection across libraries

**Implementation:**
- Add `[libraries]` section to spec.toml schema
- Modify loader to resolve cross-library deps from local directories or paths
- Update validator to check external deps exist within loaded libraries
- Generate proper use statements (`use shared::money::round`)
- Extend cycle detection to follow cross-library edges
- Add integration tests with 2+ library workspaces

**Files:** loader.rs, validator.rs, generator.rs, commands/*.rs

**Estimated effort:** L (large, ~3-5 days)

### Medium Priority

#### Passport Evidence Preservation
Fix: `spec build` and `spec generate` overwrite passport evidence and contract_hash.

**Implementation:**
- Modify `write_passports` in commands.rs to read existing passport before overwriting
- Preserve `evidence` and `contract_hash` fields when present
- Only write passports from `spec test` exclusively OR merge metadata intelligently
- Add tests for `spec build -> spec test` and `spec test -> spec build` sequences

**Files:** commands.rs

**Estimated effort:** S (small, ~2-3 hours)

#### Stable External Error Code Namespace
Define a stable external namespace for JSON error codes (e.g., `SPEC_DEP_NOT_FOUND`) separate from Rust enum variant names. Map internal variants to stable codes.

**Implementation:**
- Define stable error code enum/constant set
- Create mapping function from `SpecError` to stable codes
- Update JSON output format to use stable codes
- Bump schema_version when code mappings change
- Add test verifying all SpecError variants have mappings

**Files:** errors.rs, commands/export.rs

**Estimated effort:** S (small, ~3-4 hours)

#### Refactor: Error Handling Cleanup

1. **spec_error_to_json_entry two-pass match** (Effort: S)
   - Replace 9-tuple return with `ErrorFields` struct
   - Use `Default::default()` for absent fields
   - Improves maintainability and compiler safety

2. **extract push_error/push_warning loop helper** (Effort: XS)
   - Extract 4-line error/warning collection pattern
   - Deduplicate in validate_command, export_command, generate_specs
   - Cleaner, more testable diagnostic collection

3. **test_command passport finalization duplication** (Effort: XS)
   - Extract ~60 lines of shared passport finalization logic
   - Create `finalize_for_spec` helper
   - Reduce duplication between build-failure and test-success paths

#### Performance Optimizations

1. **parse_test_output() HashMap optimization** (Effort: XS)
   - Build HashMap of expected test IDs before scanning output
   - Reduce from O(lines × units) to O(lines)
   - Document performance improvement

2. **cargo timeout support** (Effort: S)
   - Add `wait_timeout` crate or similar
   - Add `[pipeline] timeout_secs` config to spec.toml
   - Prevent indefinite hangs during build/test
   - Graceful timeout with partial results preserved

3. **BTreeMap → HashMap** (Effort: XS)
   - Replace BTreeMap with HashMap in pipeline.rs, commands.rs
   - O(1) insert/lookup where sorted order not needed
   - Low priority but straightforward

4. **spec test module filter fix** (Effort: XS)
   - Fix `cargo_test_filter_for` to handle nested output paths correctly
   - Currently uses only `file_name()` last segment
   - Should use full path relative to output dir

### Low Priority / Deferred

#### M6: Semantic Contract-vs-Body Comparison (LLM Eval)
LLM-powered eval: compare `intent` + `contract` spec against generated body code. Emit `semantic_match` score in passport. Catches "body technically compiles but logic doesn't match intent" - the real governance story.

**Note:** Depends on M5 shipping first. Requires eval infrastructure (LLM call from spec test or separate `spec eval` command).

**Status:** Deferred to M6

### Implementation Order

1. D5a Newtype Refactor (blocks other work on ResolvedSpec)
2. Evidence Provenance (adds CI-trustworthiness)
3. Passport Evidence Preservation (fixes data loss bug)
4. Stable External Error Code Namespace (API contract)
5. Cross-Library Dependency Implementation (largest feature)
6. Refactoring and performance items (can be interleaved)

### Success Criteria

- All high and medium priority items completed
- Zero regression in existing tests
- New tests for all new functionality
- Documentation updates for user-facing changes
- Performance benchmarks show improvements where claimed

---

## /autoplan Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Defer cross-library dependencies to M7 | User Challenge | P2 (boil lakes) | Premature scaling; filesystem coupling without package management | — |
| 2 | CEO | Reprioritize M5 around verification + stability | Taste | P1 (completeness) | Semantic verification is product wedge; infrastructure is table stakes | Original sequential order |
| 3 | CEO | Evidence provenance → minimal scope | Taste | P3 (pragmatic) | Weak trust gain for complexity; local JSON is untrusted medium | Full CI env detection |
| 4 | Eng | ValidatedExpr is public API break | Mechanical | P5 (explicit) | spec-core is published library; ResolvedSpec public → semver break | — |
| 5 | Eng | Cross-library codegen undefined | Mechanical | P5 (explicit) | No Rust import-path contract; would break first nontrivial workspace | — |
| 6 | Eng | Extract push_error helper before refactors | Mechanical | P4 (DRY) | 4-line pattern duplicated in 3+ commands | — |
| 7 | Eng | Defer HashMap optimization to future | Mechanical | P3 (pragmatic) | BTreeMap overhead negligible for small repos; not on critical path | — |

## /autoplan Approval

**Status:** APPROVED as-is
**Timestamp:** '$(date)'  
**Branch:** $(git branch --show-current)
**Commit:** $(git rev-parse --short HEAD)

**Approved reframing:**
1. ✅ Reframe M5 to prioritize semantic verification + stability
2. ✅ Defer cross-library dependencies to M7
3. ✅ Reprioritize implementation order (stability before internals)
4. ✅ Cut evidence provenance scope to minimal viable fields

**Next steps:** Update PLAN.md with revised M5 scope, then run `/ship` when ready to create PR.
