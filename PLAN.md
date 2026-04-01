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
| **YAML** | serde_yaml_bw | Maintained fork of serde_yaml (deprecated). Panic-free, hardened against Billion Laughs. Same Value API. |
| **Dir traversal** | walkdir | Standard recursive directory walk crate |
| **Errors** | thiserror + anyhow | thiserror for typed library errors, anyhow for CLI error propagation |
| **Generation** | Custom code writer | Small surface area, readable output |
| **Workspace** | 2 crates (cli + types) | Avoid premature abstraction, extract libraries later |
| **MSRV** | Rust 1.91.1 (floor: 1.89) | Pin `rust-version` in Cargo.toml. jsonschema requires ≥1.83 |

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

## JSON Schema: unit.spec.json

The schema file lives at `spec-core/src/schema/unit.spec.json`, embedded at compile time via `include_str!`. Uses JSON Schema Draft 7 (fully supported by the `jsonschema` crate).

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Unit Spec",
  "description": "Schema for .unit.spec semantic unit files",
  "type": "object",
  "required": ["id", "kind", "intent", "body"],
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string",
      "pattern": "^[a-z][a-z0-9_]*/[a-z][a-z0-9_]*(/[a-z][a-z0-9_]*)*$",
      "description": "Hierarchical unit ID. Each segment must be a valid Rust identifier (no keywords)."
    },
    "kind": {
      "type": "string",
      "enum": ["function"]
    },
    "intent": {
      "type": "object",
      "required": ["why"],
      "additionalProperties": false,
      "properties": {
        "why": {
          "type": "string",
          "minLength": 1
        }
      }
    },
    "contract": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "inputs": {
          "type": "object",
          "additionalProperties": { "type": "string" }
        },
        "returns": {
          "type": "string"
        },
        "invariants": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "deps": {
      "type": "array",
      "items": {
        "type": "string",
        "pattern": "^[a-z][a-z0-9_]*/[a-z][a-z0-9_]*(/[a-z][a-z0-9_]*)*$"
      }
    },
    "body": {
      "type": "object",
      "required": ["rust"],
      "additionalProperties": false,
      "properties": {
        "rust": {
          "type": "string",
          "minLength": 1
        }
      }
    },
    "local_tests": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "expect"],
        "additionalProperties": false,
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "expect": {
            "type": "string",
            "minLength": 1
          }
        }
      }
    },
    "links": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "molecule_tests": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    }
  }
}
```

**Note:** The JSON Schema `pattern` validates regex format but does NOT check for Rust reserved keywords. Keyword validation is a post-schema check in the validator (see Pipeline Stage 2 below).

---

## Type Definitions

### SpecStruct (raw parsed form, mirrors YAML)

```rust
struct SpecStruct {
    id: String,
    kind: String,
    intent: Intent,
    contract: Option<Contract>,
    deps: Vec<String>,         // defaults to empty vec if absent
    body: Body,
    local_tests: Vec<LocalTest>, // defaults to empty vec if absent
    links: Option<Links>,
}

struct Intent {
    why: String,
}

struct Body {
    rust: String,
}

struct Contract {
    inputs: Option<HashMap<String, String>>,  // type names as strings
    returns: Option<String>,
    invariants: Vec<String>,                  // human-readable invariant expressions
}

struct LocalTest {
    id: String,
    expect: String,   // human-readable assertion expression
}

struct Links {
    molecule_tests: Vec<String>,  // IDs of related molecule-level tests
}
```

`SpecStruct` is deserialized from the validated JSON Value (after schema validation passes). Fields with defaults use `#[serde(default)]` in implementation.

---

## Internal Representation (IR)

The normalizer produces `ResolvedSpec`, which the generator consumes. Defined in `types.rs`:

```rust
struct ResolvedSpec {
    id: String,                       // canonical: "pricing/apply_discount"
    fn_name: String,                  // last segment: "apply_discount"
    module_path: String,              // everything before last segment: "pricing"
    deps: Vec<String>,                // fully resolved dep IDs (empty vec if none)
    body_rust: String,                // raw Rust code from body.rust block
    contract: Option<Contract>,       // metadata only, not used for codegen in M1
    local_tests: Vec<LocalTest>,      // stored, not executed in M1 (preserved for M2)
    links: Option<Links>,             // stored, not used in M1 (preserved for M2)
}
```

`SpecStruct` is the raw parsed form (close to YAML). `ResolvedSpec` is the normalized form with derived fields computed and defaults applied.

---

## Pipeline: 4 Stages

### 1. Load
- Read `*.unit.spec` from filesystem (recursive directory walk)
- **UTF-8 check**: Read file as bytes, verify valid UTF-8 before YAML parsing. Error: `File is not valid UTF-8: {path}`
- Parse YAML to `serde_yaml::Value` (preserves raw author input for schema validation)
- Track file paths for error reporting

### 2. Validate (JSON Schema)
- Convert `serde_yaml::Value` → `serde_json::Value` for JSON Schema validation (validates raw author input, not re-serialized struct)
- Required fields: `id`, `kind`, `intent`, `body`
- ID format: snake_case segments separated by `/` — regex `[a-z][a-z0-9_]*/[a-z][a-z0-9_]*(/[a-z][a-z0-9_]*)*` — **must contain `/`**, each segment must be a valid Rust identifier (e.g., `pricing/apply_discount`)
- Kind: enum (`function` only in 0.1)
- Additional fields rejected (closed schema)
- **Rust keyword check** (post-schema): reject IDs where any segment is a Rust reserved keyword (`type`, `mod`, `crate`, `self`, `super`, `fn`, `struct`, `enum`, `impl`, `trait`, `pub`, `use`, `let`, `mut`, `const`, `static`, `ref`, `return`, `if`, `else`, `match`, `for`, `while`, `loop`, `break`, `continue`, `move`, `async`, `await`, `dyn`, `where`, `as`, `in`, `extern`, `unsafe`). Error: `ID segment "type" is a Rust reserved keyword`
- Dep fn_name collision check (two deps with same last segment → error)
- **body.rust must NOT contain `use` statements** (post-schema check). All imports come from `deps` declarations. Error: `body.rust must not contain use statements; declare imports via deps instead`
- **Duplicate ID check** (post-load): after loading all files in a directory, reject duplicate IDs across files. Error: `Duplicate ID "pricing/apply_discount" in foo.unit.spec and bar.unit.spec`
- **Single file**: fail-fast (stop on first error within the file)
- **Directory**: collect-all (validate every file, report all errors, aggregate status). **Error output format**: errors grouped by file path, summary line at end: `❌ 3 files, 7 errors`
- After validation passes, deserialize `serde_json::Value` → `SpecStruct`
- Schema embedded at compile time via `include_str!("../schema/unit.spec.json")` — lives at `spec-core/src/schema/unit.spec.json`, no runtime file dependency

### 3. Normalize → IR
- Resolve defaults (`deps: []` if missing)
- **Canonicalize IDs**: strip any leading/trailing whitespace. IDs are already constrained by regex; canonicalization ensures no invisible characters survive deserialization. Verify regex match post-normalization as a defensive check.
- Derive `fn_name` (last segment) and `module_path` (all preceding segments)
- Parse contract metadata
- **Deps are trusted strings in M1.** No graph resolution, no cycle detection, no referential integrity check. Dep IDs become `use` statements directly. The Rust compiler catches missing modules. (Graph resolution and cycle detection deferred to M2.)

### 4. Generate .rs
- Map ResolvedSpec to Rust module structure
- Extract `body.rust` to function definitions
- Handle deps: split ID on `/`, last segment = fn_name, all preceding segments = module path. **All use statements use `crate::` prefix** (required for Rust 2018+ edition). Examples: `money/round` → `use crate::money::round::round;`, `utils/math/round` → `use crate::utils::math::round::round;`
- Detect dep fn_name collisions (e.g., deps on both `money/round` and `math/round`) → error at validation, not codegen
- **Contract metadata is stored in IR but NOT used for code generation in M1.** `body.rust` is the sole codegen input.
- **`body.rust` is a verbatim Rust function definition** (signature + body, NO `use` statements — those come from `deps`). The generator wraps it with `use` statements and module structure but does not transform the code itself.
- Write readable, formatted code
- Output to `./generated/spec/{module}/{fn_name}.rs` by default
- Auto-generate `mod.rs` per directory with `pub mod` declarations for each unit in that namespace
- **spec owns the entire output directory.** All files in `generated/spec/` are managed by spec. Do not place hand-written files there; they will be deleted on next generate.
- **Scoped clean before generate:** Only clean files within module directories that the current generate run will produce. Stale `.rs` files in affected dirs are removed; other dirs are untouched. This supports partial workflows (e.g., `spec generate pricing/` then `spec generate money/`).
- **Safety rails for clean:** Before deleting, verify: (1) the output dir path is inside the project root, and (2) the output dir contains a `.spec-generated` marker file. Marker is created during first generate and **recreated after each clean step**. Refuse to clean if either check fails. Error: `Refusing to clean {path}: missing .spec-generated marker`
- **Zero files case:** If no `.unit.spec` files are found, succeed with message: `0 units found, nothing to generate.` Exit code 0.
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
- `deps` — unit IDs; `money/round` resolves to `use crate::money::round::round;` in generated Rust

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

## Test Coverage: 49 Tests

### Loader Tests (10)
- [ ] Load valid YAML file → serde_yaml::Value → SpecStruct
- [ ] File not found → clear error with path
- [ ] Permission denied → clear error with path
- [ ] Invalid YAML syntax → parse error at line X
- [ ] Load directory recursively → Vec<SpecStruct>
- [ ] Empty directory → empty vec (not an error)
- [ ] Directory with mixed files (non-.unit.spec files skipped)
- [ ] Nested subdirectories loaded recursively
- [ ] Empty .unit.spec file → clear error (not crash)
- [ ] Non-UTF8 / binary .unit.spec file → "File is not valid UTF-8: {path}"

### Validation Tests (15)
- [ ] Valid .unit.spec passes all checks
- [ ] Missing required field: id → error
- [ ] Missing required field: kind → error
- [ ] Missing required field: intent → error
- [ ] Missing required field: body → error
- [ ] Invalid id format (uppercase, leading digits, spaces) → error
- [ ] ID without `/` (e.g., `id: foo`) → error: must be hierarchical
- [ ] Unknown kind value → error
- [ ] Extra/unknown field rejected (closed schema) → error
- [ ] body exists but missing `rust` key → error
- [ ] Empty string values (id: "", intent.why: "") → error
- [ ] ID segment is Rust keyword (e.g., `pricing/type`) → error
- [ ] Embedded JSON Schema is itself valid (meta-validation of unit.spec.json)
- [ ] body.rust containing `use` statement → error (imports come from deps)
- [ ] Duplicate ID across files in same directory → error with both file paths

### Normalizer Tests (6)
- [ ] ID canonicalization (whitespace stripped, regex re-verified)
- [ ] fn_name derived from last path segment
- [ ] module_path derived from prefix segments (including 3+ segment depth)
- [ ] Missing deps defaults to empty Vec
- [ ] Dep fn_name collision (two deps with same last segment) → error
- [ ] contract, local_tests, links survive normalization (passthrough roundtrip)

### Generator Tests (11)
- [ ] Generate valid .rs from complete ResolvedSpec
- [ ] Handle missing optional fields (contract is None)
- [ ] Format generated code properly (indentation, braces)
- [ ] Multiple units generate separate .rs files under correct module paths
- [ ] mod.rs generated with pub mod declarations for each unit in namespace
- [ ] Dep use statement correct at 3+ segment depth (uses `crate::` prefix)
- [ ] Output dir not writable → clear error with path
- [ ] Scoped clean removes stale files in affected module dirs only
- [ ] Clean refuses to delete dir without `.spec-generated` marker
- [ ] Zero .unit.spec files → succeed with message, exit code 0
- [ ] ID-to-file-path mapping correctness (including 3+ segment depth)

### Validator Aggregate Tests (1)
- [ ] Directory validation collects all errors across files (collect-all mode)

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
| **deps resolution** | Generalized: split on `/`, last = fn_name, prefix = module path (works at any depth) | Flattened `money_round` style |
| **deps collision** | Error at validation if two deps share fn_name | Auto-alias (`as money_round`) |
| **Fail mode (dir)** | Collect-all for directories, fail-fast for single files | Fail-fast everywhere |
| **Validation pipeline** | YAML → Value → JSON Schema → SpecStruct (validate raw input) | Deserialize first, validate struct |
| **Dir loading** | Recursive walk | Flat (immediate files only) |
| **Stale files** | Clean output dir before each generate run | Manifest-based tracking |
| **mod.rs** | Auto-generate with pub mod per unit in namespace | Manual wiring by user |
| **local_tests/links in IR** | Stored in ResolvedSpec, not executed in M1 | Stripped during normalize |
| **Generated output** | .rs files only, user provides Cargo project | Full Cargo project (deferred — adds stale-file complexity before format is validated) |
| **Schema location** | `spec-core/src/schema/unit.spec.json` (embedded via include_str!) | Runtime file path |
| **YAML crate** | `serde_yaml_bw` (maintained fork, panic-free) | `serde_yaml` (deprecated), `serde-saphyr` (no Value type) |
| **Error handling** | `thiserror` (library) + `anyhow` (CLI) | Custom error types (boilerplate) |
| **ID keyword check** | Reject Rust reserved keywords in ID segments | Allow keywords + use r#raw identifiers in codegen |
| **Output dir safety** | `.spec-generated` marker file + path prefix check | No safety (risk of accidental deletion) |
| **Output dir ownership** | spec owns `generated/spec/` entirely | Allow mixed hand-written + generated files |
| **JSON Schema draft** | Draft 7 (fully supported by `jsonschema` crate) | Draft 2020-12 (partial support) |
| **Dep resolution (M1)** | Deps are trusted strings → `use crate::` statements | Validate deps exist (requires full graph) |
| **body.rust imports** | body.rust must NOT contain `use` statements; deps own imports | Allow use statements in body.rust (dedup complexity) |
| **Duplicate IDs** | Error on duplicate IDs across files in same load | Last-file-wins silently (data loss risk) |
| **Cycle detection** | Deferred to M2 with graph resolution | Include in M1 normalizer (can't work with partial runs) |
| **Output dir clean** | Scoped clean: only affected module dirs | Full clean of entire output dir (breaks partial workflows) |
| **Zero files** | Succeed with message, exit 0 | Error out (breaks idempotent scripts) |
| **Error output format** | Grouped by file, summary line at end | Flat list (harder to scan) |
| **Use statement prefix** | `crate::` prefix (Rust 2018+) | No prefix (ambiguous with external crates) |
| **MSRV** | 1.91.1 (floor 1.89) | Unspecified (confusing build failures for users) |

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
4. Write tests (49 total)
5. Create example project
6. Write README
7. Setup CI/CD (GitHub Actions)
8. Release 0.1

---

## Worktree Parallelization Strategy

| Step | Modules touched | Depends on |
|------|----------------|------------|
| spec-core library | spec-core/src/ | — |
| spec-cli binary | spec-cli/src/ | spec-core (types/interfaces) |
| Example project | examples/ | spec-core, spec-cli |
| CI/CD + README | .github/, README.md | spec-cli (for install instructions) |

**Lane A:** spec-core (types, loader, validator, normalizer, generator + all unit tests)
**Lane B:** spec-cli (main.rs, commands.rs + CLI integration tests) — depends on Lane A's public API, not full implementation
**Lane C:** CI/CD + README — independent, can start in parallel with Lane A

```
Time →
Lane A: [workspace init] → [spec-core: types → loader → validator → normalizer → generator] → [unit tests]
Lane C:                    [CI/CD scaffold + README draft]─────────────────────────────────────────────────→
Lane B:                                        [spec-cli: commands, integration tests]────────────────────→
Merge:                                                                                   [example project]
```

**Execution:** Launch A + C in parallel. Start B once A's public types/traits are defined (doesn't need full implementation). Example project last, needs working CLI.

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
- **crates.io**: serde, serde_yaml_bw, serde_json, jsonschema, clap, walkdir, thiserror, anyhow
- **Optional (0.2)**: cue-lang/tap/cue (external binary)

---

**This plan is reviewed, approved, and ready for implementation.**

**Review**: `/plan-ceo-review` on 2026-04-01 (HOLD SCOPE), `/plan-eng-review` on 2026-03-31 (fourth pass)
**Design**: `design-20260331-001.md` (by /office-hours)
**Status**: All decisions resolved, architecture final, 49 tests planned

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | HOLD SCOPE. 10 decisions made. crate:: prefix fix (critical). Scoped clean. 49 tests. 3 TODOs added. |
| Codex Review | `/codex review` | Independent 2nd opinion | 4 | CLEAR | Pass 4 (via CEO review): import path bug, clean scope, cycle detection, scope drift, release engineering. 3 acted on, 2 deferred to TODOS. |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 3 | CLEAR (PLAN) | Pass 4: 5 issues (deps, JSON Schema, types, body.rust clarity, keyword validation). All resolved. |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0
**VERDICT:** CEO + ENG + CODEX CLEARED — 49 tests planned, all decisions resolved, architecture final.
