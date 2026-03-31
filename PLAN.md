# Release 0.1: Source Foundations - Implementation Plan

**Generated**: 2026-03-31 by /plan-eng-review  
**Status**: Reviewed and approved  
**Based on**: design-20260331-001.md

---

## Architecture Summary

**Workflow:**
```
.unit.spec (YAML) → Load → Validate → Normalize → Generate .rs
                                ↓              ↓
                         JSON Schema    Readable Rust
                         (schemars)      (cargo compatible)
```

**Forces AI to validate at every step:**
1. Structured YAML (can't write freeform)
2. Schema validation (JSON Schema)
3. Rust compilation (fmt, clippy, rustc)

**Value**: Small unit files + fast compilation (1-2s overhead) on each increment forces incremental correctness.

---

## Technical Stack - M1

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Language** | Rust | Native performance, strong type system, good codegen support |
| **Schema** | JSON Schema (schemars) | Simpler dependency than CUE, easier for Rust ecosystem |
| **YAML** | serde_yaml | Standard serializer, supports embedded code blocks |
| **Generation** | Custom code writer | Small surface area, readable output |
| **Workspace** | 2 crates (cli + types) | Avoid premature abstraction, extract libraries later |

---

## Component Architecture

**Crate 1: spec-core (library)**
```
src/
  lib.rs           # Re-exports
  loader.rs        # Parse *.unit.spec → SpecStruct
  validator.rs     # JSON Schema validation
  normalizer.rs    # Canonicalize to IR
  generator.rs     # IR → .rs file writer
  types.rs         # Data structures (ID, Contract, Spec, IR)
```

**Crate 2: spec-cli (binary)**
```
src/
  main.rs          # CLI entrypoint
  commands.rs      # validate, generate, etc.
```

---

## Pipeline: 4 Stages

### 1. Load
- Read `*.unit.spec` from filesystem
- Parse YAML using serde → SpecStruct
- Track file paths for error reporting

### 2. Validate (JSON Schema)
- Required fields: `id`, `kind`, `intent`, `body`
- ID format: `[a-zA-Z0-9_/]+` (hierarchical)
- Kind: enum (`function` only in 0.1)
- Additional fields rejected (closed schema)
- **Fail-fast**: stop on first error

### 3. Normalize → IR
- Resolve defaults (`deps: []` if missing)
- Canonicalize IDs
- Parse contract metadata
- Build dependency graph (shallow for 0.1)

### 4. Generate .rs
- Map SpecStruct to Rust module structure
- Extract `body.rust` to function definitions
- Handle deps (use statements, module imports)
- Write readable, formatted code
- Output to `./generated/spec/` by default

---

## File Format: .unit.spec

```yaml
id: pricing/apply_discount
kind: function
intent:
  why: Apply percentage discount preserving nonnegative money
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
        let discounted = subtotal - subtotal * rate;
        round_money(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: happy_path
    expect: apply_discount(100, 0.10) == 90
```

---

## User Workflow

**Author first unit:**
```bash
# 1. Create spec directory
mkdir -p spec/units/pricing
echo 'id: pricing/apply_discount
kind: function
intent: Apply percentage discount
contract: { inputs: { subtotal: Decimal, rate: Decimal }, returns: Decimal }
body:
  rust: |
    pub fn apply_discount(...) { /* impl */ }
' > spec/units/pricing/apply_discount.unit.spec

# 2. Validate spec spec validate spec/units/pricing/
#    → ✅ 1 units valid

# 3. Generate Rust code spec generate spec/units/pricing/
#    → Generated 1 file to ./generated/spec/pricing/apply_discount.rs

# 4. Run Rust toolchain cd generated/spec && cargo fmt && cargo clippy && cargo build
#    → Cargo compiles successfully
```

**On error:**
```bash
# Remove required field
sed -i '/intent:/d' spec/units/apply_discount.unit.spec

spec validate spec/units/
# → ❌ spec/units/apply_discount.unit.spec: missing field: intent

# Fix and re-run → ✅ passes validation
```

---

## Test Coverage: 25 Tests

### Generator Tests (5)
- [ ] Generate valid .rs from complete .unit.spec
- [ ] Handle missing optional fields (deps, tests, contract)
- [ ] Format generated code properly
- [ ] Multiple units generate separate .rs files
- [ ] Detect circular dependencies

### Validation Tests (7)
- [ ] Valid .unit.spec passes all checks
- [ ] Missing required field: id → error
- [ ] Missing required field: kind → error
- [ ] Missing required field: intent → error
- [ ] Missing required field: body → error
- [ ] Invalid id format (spaces, special chars) → error
- [ ] Unknown kind value → error

### Loader Tests (5)
- [ ] Load valid YAML file → SpecStruct
- [ ] File not found → clear error with path
- [ ] Permission denied → clear error with path
- [ ] Invalid YAML syntax → parse error at line X
- [ ] Load directory → Vec<SpecStruct>

### CLI Tests (5)
- [ ] Validate single file success
- [ ] Validate single file error with message
- [ ] Validate directory with multiple files
- [ ] Generate command writes to correct output dir
- [ ] Help and version flags work

### Integration Tests (3)
- [ ] Full workflow: valid spec → generate → cargo build succeeds
- [ ] Round-trip: parse → generate → parse generated Rust module
- [ ] Edge cases: empty deps, empty tests, minimal contract

---

## Success Criteria

- ✓ Can author a valid `.unit.spec` file without internal knowledge
- ✓ Invalid YAML schema fails with clear diagnostics (JSON Schema errors)
- ✓ Valid source normalizes to canonical internal representation (IR)
- ✓ Generated `.rs` files are readable and compile with `cargo build`
- ✓ User can run `rustfmt` and `clippy` on generated code
- ✓ Can process a directory of units and report aggregate status
- ✓ Validation + generation completes in 1-2 seconds for 1-10 units

---

## Decisions Made

| Decision | Choice | Alternative Considered |
|----------|--------|------------------------|
| **Validation** | JSON Schema (schemars) | CUE (deferred to 0.2) |
| **Workspace** | 2 crates (cli + core) | 4-5 crates (overbuilt) |
| **Pipeline stages** | 4 (Load → Validate → Normalize → Generate) | 8 stages (too early) |
| **Fail mode** | Fail-fast | Collect-all (too complex) |
| **Native code** | YAML literal blocks (`\|`) | Markdown fences (```) |
| **Schema org** | Unified (unit.cue) | Per-kind (function.cue) |
| **ID format** | Hierarchical `pricing/apply_discount` | Flat with metadata |

---

## Open Questions Resolved

1. ✅ **CUE organization:** Unified schema, split later when adding 2nd unit kind
2. ✅ **ID format:** Hierarchical with `/` separator
3. ✅ **Native code delimiters:** YAML literal blocks (`\|`)
4. ✅ **Validation ordering:** Fail-fast for 0.1
5. ✅ **CUE runtime:** Shell out to cue binary (deferred to 0.2)
6. ✅ **Workspace structure:** 2 crates (avoid premature abstraction)
7. ✅ **Code generation scope:** In M1 (proves value, not just validation)

---

## NOT in Scope (Deferred)

- CUE validation (M2)
- Evidence collection and passports (M2)
- Graph resolution (M2)
- Multi-language support (M3+)
- IDE/editor integration (M4+)
- Reverse flow (M5+)
- Planning layer (M6+)

---

## Architecture Diagram

```
data-flow: |
  Authoring → Load → Validate → Normalize → Generate → Rust Toolchain → Binary
      ↑           ↓        ↓          ↓         ↓             ↓           ↓
      └───────────┴────────┴──────────┴─────────┴─────────────┴───────────┘
             spec        JSON      Canoni-    .rs files    cargo      cargo
           .unit.spec   Schema      cal IR    (readable)    fmt       build
```

---

## Next Steps

1. Initialize Rust workspace
2. Implement `spec-core` library (types, loader, validator, normalizer, generator)
3. Implement `spec-cli` binary (commands: validate, generate, help)
4. Write tests (25 total)
5. Create example project
6. Write README
7. Setup CI/CD (GitHub Actions)
8. Release 0.1

---

## Files to Create

```
Cargo.toml (workspace)
spec-core/Cargo.toml
spec-core/src/
  lib.rs
  types.rs
  loader.rs
  validator.rs
  normalizer.rs
  generator.rs
spec-cli/Cargo.toml
spec-cli/src/
  main.rs
  commands.rs
src/
  schema/unit.spec.json (JSON Schema)
examples/
  ecommerce/
    units/
      pricing/
        apply_discount.unit.spec
        apply_tax.unit.spec
        calculate_total.unit.spec
    Cargo.toml
    src/
      main.rs
TODOS.md (this file)
README.md
```

---

## External Dependencies

- **Cargo toolchain**: rustfmt, clippy, rustc
- **crates.io**: serde, serde_yaml, serde_json, schemars, clap
- **Optional (0.2)**: cue-lang/tap/cue (external binary)

---

**This plan is reviewed, approved, and ready for implementation.**

**Review**: `/plan-eng-review` on 2026-03-31  
**Design**: `design-20260331-001.md` (by /office-hours)  
**Status**: All decisions resolved, architecture final
