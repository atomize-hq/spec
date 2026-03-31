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
                         (jsonschema)    (cargo compatible)
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
| **Schema** | JSON Schema (static file + `jsonschema` crate) | schemars derives schemas; `jsonschema` validates YAML against a schema |
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

## Internal Representation (IR)

The normalizer produces `ResolvedSpec`, which the generator consumes. Defined in `types.rs`:

```rust
struct ResolvedSpec {
    id: String,           // canonical: "pricing/apply_discount"
    fn_name: String,      // last segment: "apply_discount"
    module_path: String,  // everything before last segment: "pricing"
    deps: Vec<String>,    // fully resolved dep IDs (empty vec if none)
    body_rust: String,    // raw Rust code from body.rust block
    contract: Option<Contract>,
}
```

`SpecStruct` is the raw parsed form (close to YAML). `ResolvedSpec` is the normalized form with derived fields computed and defaults applied.

---

## Pipeline: 4 Stages

### 1. Load
- Read `*.unit.spec` from filesystem
- Parse YAML using serde → SpecStruct
- Track file paths for error reporting

### 2. Validate (JSON Schema)
- Required fields: `id`, `kind`, `intent`, `body`
- ID format: `[a-zA-Z0-9_/]+` (hierarchical) — **must contain `/`** (e.g., `pricing/apply_discount`)
- Kind: enum (`function` only in 0.1)
- Additional fields rejected (closed schema)
- **Fail-fast**: stop on first error
- Schema embedded at compile time via `include_str!("../schema/unit.spec.json")` — lives at `spec-core/src/schema/unit.spec.json`, no runtime file dependency

### 3. Normalize → IR
- Resolve defaults (`deps: []` if missing)
- Canonicalize IDs
- Parse contract metadata
- Build dependency graph (shallow for 0.1)

### 4. Generate .rs
- Map ResolvedSpec to Rust module structure
- Extract `body.rust` to function definitions
- Handle deps: `money/round` → `use money::round::round;` (last segment = fn_name rule)
- Write readable, formatted code
- Output to `./generated/spec/{module}/{fn_name}.rs` by default
- Auto-creates output dir if missing; errors if not writable
- **Does not** generate Cargo.toml or lib.rs (user provides the containing project)

---

## File Format: .unit.spec

```yaml
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount while preserving nonnegative money values.
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
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: happy_path
    expect: apply_discount(100, 0.10) == 90
  - id: zero_rate
    expect: apply_discount(100, 0.0) == 100
links:
  molecule_tests:
    - pricing/discount_plus_tax
```

**Field semantics:**
- `intent.why` — required string; explains *why* this unit exists, not *what* it does
- `local_tests` — atom-level tests owned by this unit; included in closed schema
- `links` — optional; declares molecule/organism test relationships; included in schema, unused until M2
- `deps` — unit IDs; `money/round` resolves to `use money::round::round;` in generated Rust

---

## User Workflow

**Author first unit:**
```bash
# 1. Create spec directory
mkdir -p spec/units/pricing
cat > spec/units/pricing/apply_discount.unit.spec << 'EOF'
id: pricing/apply_discount
kind: function
intent:
  why: Apply a percentage discount preserving nonnegative money values.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
body:
  rust: |
    pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
        subtotal - subtotal * rate
    }
EOF

# 2. Validate
spec validate spec/units/pricing/
#    → ✅ 1 unit valid

# 3. Generate Rust code
spec generate spec/units/pricing/
#    → Generated 1 file: generated/spec/pricing/apply_discount.rs

# 4. Add generated file to your Cargo project and build
#    cp generated/spec/pricing/apply_discount.rs your-crate/src/pricing/
#    cargo build
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

## Test Coverage: 32 Tests

### Loader Tests (5)
- [ ] Load valid YAML file → SpecStruct
- [ ] File not found → clear error with path
- [ ] Permission denied → clear error with path
- [ ] Invalid YAML syntax → parse error at line X
- [ ] Load directory → Vec<SpecStruct>

### Validation Tests (8)
- [ ] Valid .unit.spec passes all checks
- [ ] Missing required field: id → error
- [ ] Missing required field: kind → error
- [ ] Missing required field: intent → error
- [ ] Missing required field: body → error
- [ ] Invalid id format (spaces, special chars) → error
- [ ] ID without `/` (e.g., `id: foo`) → error: must be hierarchical
- [ ] Unknown kind value → error

### Normalizer Tests (5)
- [ ] ID canonicalization (no leading/trailing slash)
- [ ] fn_name derived from last path segment
- [ ] module_path derived from prefix segments
- [ ] Missing deps defaults to empty Vec
- [ ] Detect circular dependencies → error

### Generator Tests (5)
- [ ] Generate valid .rs from complete ResolvedSpec
- [ ] Handle missing optional fields (contract is None)
- [ ] Format generated code properly (indentation, braces)
- [ ] Multiple units generate separate .rs files under correct module paths
- [ ] Output dir not writable → clear error with path

### CLI Tests (5)
- [ ] Validate single file success
- [ ] Validate single file error with message
- [ ] Validate directory with multiple files
- [ ] Generate command writes .rs files to correct output dir
- [ ] Help and version flags work

### Integration Tests (3)
- [ ] Full workflow: valid spec → generate → cargo check passes on generated output
- [ ] Multiple units with deps: generated use statements are correct
- [ ] Edge cases: empty deps, empty contract, minimal unit

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
| **Validation** | JSON Schema (`jsonschema` crate + static schema file) | CUE (deferred to 0.2) |
| **Workspace** | 2 crates (cli + core) | 4-5 crates (overbuilt) |
| **Pipeline stages** | 4 (Load → Validate → Normalize → Generate) | 8 stages (too early) |
| **Fail mode** | Fail-fast | Collect-all (too complex) |
| **Native code** | YAML literal blocks (`\|`) | Markdown fences (```) |
| **Schema org** | Unified (unit.cue) | Per-kind (function.cue) |
| **ID format** | Hierarchical `pricing/apply_discount` | Flat with metadata |
| **intent shape** | Object `{ why: string }` | Plain string (rejected — not aligned with architecture) |
| **deps resolution** | Last segment = fn_name (`money/round` → `round`) | Flattened `money_round` style |
| **Generated output** | .rs files only, user provides Cargo project | Full Cargo project (deferred — adds stale-file complexity before format is validated) |
| **Schema location** | `spec-core/src/schema/unit.spec.json` (embedded via include_str!) | Runtime file path |

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
4. Write tests (32 total)
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
spec-core/src/schema/unit.spec.json (JSON Schema — embedded via include_str!)
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
- **crates.io**: serde, serde_yaml, serde_json, jsonschema, clap
- **Optional (0.2)**: cue-lang/tap/cue (external binary)

---

**This plan is reviewed, approved, and ready for implementation.**

**Review**: `/plan-eng-review` on 2026-03-31 (second pass)
**Design**: `design-20260331-001.md` (by /office-hours)
**Status**: All decisions resolved, architecture final

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 1 | issues_found → all resolved | 8 issues resolved in this session |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 2 | CLEAR (PLAN) | 8 issues found and resolved |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**VERDICT:** ENG CLEARED — second pass complete, all Codex findings resolved.
