# Release 0.5: AI-Native Loop

**Generated**: 2026-04-06  
**Status**: Planning  
**Preceded by**: `.implemented/PLAN-M4-release-0.4.md`  
**Design doc**: `~/.gstack/projects/atomize-hq-spec/spensermcconnell-main-design-20260405-175421.md`  
**Roadmap reference**: M5 — AI-Native Loop (v0.5.0)  

---

## Thesis

M4 closed the feedback loop for humans. `spec build` / `spec test` run cargo and write evidence into
passports. Every unit now has a machine-readable record of what was declared and what was observed
to pass.

M5 makes spec usable from inside an AI coding agent's loop.

The problem: if you ask Claude Code (or Cursor, or Codex) to implement a spec unit, the agent has
no structured way to understand what needs work, parse failures programmatically, or know whether
its implementation satisfies the contract. It scrapes terminal output and guesses.

The fix is three things shipped together: structured output (so agents can parse instead of scrape),
`spec status` (so agents know what to work on), and a companion skill (so agents know the workflow).

When M5 ships:

- `spec status` gives any AI agent a unit-level status view: valid / invalid / stale / no-evidence
- `spec validate --format json` emits structured failures — agents parse them directly to fix
- `spec test <file>` lets agents run tests against a single unit without touching the whole tree
- Passports track `contract_hash` — stale detection when an agent edits a contract
- AGENTS.md teaches the loop: status → validate → edit → build → test → repeat
- A companion gstack skill (`/spec`) is the distribution mechanism for AI-native spec usage

The bar for M5 done: an AI agent can implement a spec unit from scratch, using `spec validate
--format json` as feedback and `spec test <unit>` to write evidence, without hallucinating scope.

---

## Implementation Order

The deliverables are ordered by dependency. D0 and D1 are documentation — ship first, independently.
D2 (`contract_hash`) is required before D4 (`spec status` stale detection). D3 (`--format json`)
and D5 (single-unit test) are independent. D7 (golden fixtures) depends on D3 and D4 being done.

```
D0 (ICP paragraph) — standalone commit, no deps
D1 (AGENTS.md) — standalone commit, no deps
D2 (contract_hash + read_passport) — spec-core only; required by D4
D3 (--format json on validate) — spec-cli only; independent
D4 (spec status) — depends on D2; independent of D3
D5 (spec test [path]) — spec-core + spec-cli; depends on D2 for passport write
D6 (panic hook) — spec-cli/main.rs only; independent
D7 (golden fixtures) — depends on D3 + D4 passing
D8 (companion skill) — standalone; depends on nothing
D9 (README AI-Native section) — standalone; depends on D3+D4+D5 API being stable
```

Parallel tracks: D0+D1+D6+D8 can ship in any order. D2 → D4 is the only strict chain.

---

## Data Flow Diagram

```
.unit.spec files
     │
     ├── spec validate --format json
     │       │
     │       └── SpecError variants → JSON error objects
     │               {code, field, dep, path, ...}
     │               exit 0 (all valid) / 1 (any invalid)
     │
     ├── spec status [path]
     │       │
     │       ├── collect_specs → validate → errors[]
     │       ├── read_passport(unit) → Passport | None
     │       └── compute_contract_hash(unit) vs passport.contract_hash
     │               → valid | invalid | stale | no-evidence
     │               human text or --format json
     │
     └── spec test <file>
             │
             ├── derive spec_root from file path
             ├── generate ALL units in spec_root
             ├── run_cargo_test(filter = "{module}::{fn}::tests::")
             ├── detect 0-tests-ran → exit non-zero, no passport write
             └── write passport evidence for TARGET unit only
                     (contract_hash = SHA-256(serde_json::to_string(&contract)))
```

---

## D0 — ICP Paragraph in DECISIONS.md

One paragraph. Standalone commit. This is the first thing to ship — it gates all M5 scoping
decisions and is already in the M5 TODOS backlog.

**What to write:** The ICP is the solo engineer or 2-5 person team using AI coding assistants
daily, building systems where correctness matters. They want AI-generated code they can trust —
not just code that compiles. They're already using Claude Code, Cursor, or Codex; spec gives them
contracts + machine evidence to govern the output.

**Location:** Add a new `## ICP (v0.5)` section to `DECISIONS.md`.

**Acceptance:**
- `DECISIONS.md` has a new `## ICP (v0.5)` section with one paragraph
- Committed as a standalone commit before any code changes

---

## D1 — Real AGENTS.md

Append a `## spec Agent Workflow` section to the existing `AGENTS.md` after the current gstack
routing content (which ends around line 18). Do NOT touch the routing section.

**What agents need to know:**
1. What files to touch (`.unit.spec`) and what not to touch (generated output, passports)
2. The 5-step loop: status → validate → edit → build → test → repeat
3. How to read `--format json` output from `spec validate`
4. What a passport is and how to know when a unit is "done"
5. How to interpret stale status (`~` symbol)

**The loop to document:**
```
1. spec status .         → find what needs work (invalid, stale, no-evidence)
2. spec validate [path] --format json
                         → parse structured failures, fix exactly what's wrong
3. edit .unit.spec       → fix intent, contract, body, or local_tests
4. spec build [path]     → compile + generate (catches Rust type errors)
5. spec test [path]      → run tests, write passport evidence
   repeat from 2 until spec status shows all ✓ valid + evidence
```

**Files:**
- `AGENTS.md`: append `## spec Agent Workflow` section

**Acceptance:**
- AGENTS.md still passes gstack routing (existing section untouched)
- The spec agent workflow section is self-contained — a fresh Claude Code session can follow it
  without reading any other documentation
- Committed as a standalone commit

---

## D2 — `contract_hash` in Passport + `read_passport` API

This is the foundation for stale detection in `spec status`. A unit is stale when the contract
in the `.unit.spec` file has changed since the last `spec test` run.

### New dependency

Add to `spec-core/Cargo.toml`:
```toml
sha2 = "0.10"
hex = "0.4"
```

### `Passport` struct change

In `spec-core/src/passport.rs`, add one field:

```rust
pub struct Passport {
    // ... existing fields ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_hash: Option<String>,
}
```

Position: after `evidence` field, at the end of the struct.

**Serialization contract:**
- `contract_hash: None` → field absent from JSON (skip_serializing_if)
- Old passports without the field → deserialize with `contract_hash: None` (serde default)
- Written by `spec test` only. `spec generate` leaves `contract_hash: None`.

### Hash computation

```rust
/// Compute SHA-256 of the serialized contract field.
/// Returns None if contract is None (no contract block in the unit).
/// Hash format: "sha256:{hex}" — prefixed to allow future algorithm migration
/// without a passport schema bump.
pub fn compute_contract_hash(spec: &LoadedSpec) -> Option<String> {
    let contract = spec.spec.contract.as_ref()?;
    let json = serde_json::to_string(contract)
        .expect("contract serialization cannot fail for well-formed spec");
    let hash = sha2::Sha256::digest(json.as_bytes());
    Some(format!("sha256:{}", hex::encode(hash)))
}
```

Key decisions (from design doc + outside voice):
- Uses `serde_json::to_string` with default field order (not sorted)
- `contract.inputs` uses `IndexMap` — insertion order preserved, so reordering inputs changes the hash (intentional: input order affects generated fn parameter order)
- `contract: None` → `contract_hash: None` — no stale detection for units without a contract block
- `contract_hash` tracks interface drift only (contract field). Body/local_test drift is intentionally out of scope. Evidence timestamp covers impl drift.
- Hash format is `sha256:{hex}` (prefixed). Future: detect prefix to determine algorithm. Old
  passports with raw hex (if any were written during development) → prefix mismatch → treat as
  `None` (no detection). Migration: on read, if `contract_hash` doesn't start with `sha256:`,
  discard it (treat as None).

### `passport_path_for` helper

Extract a shared helper (currently inlined in `write_passport`):

```rust
pub fn passport_path_for(source_path: &Path) -> PathBuf {
    // source: "units/pricing/apply_tax.unit.spec"
    // result: "units/pricing/apply_tax.spec.passport.json"
}
```

### `read_passport` API

```rust
/// Read a passport for the given .unit.spec source path.
/// Returns None if the passport file doesn't exist.
/// Returns Err if the file exists but is malformed JSON.
pub fn read_passport(source_path: &Path) -> Result<Option<Passport>>
```

**Error handling:** `read_passport` returns `Err` on malformed JSON. The *caller* (`spec status`)
emits a warning to stderr and treats the unit as no-evidence — it does not abort.

### `build_passport_with_evidence` update

Add `contract_hash: Option<String>` parameter:

```rust
pub fn build_passport_with_evidence(
    spec: &LoadedSpec,
    generated_at: &str,
    evidence: Option<PassportEvidence>,
    contract_hash: Option<String>,  // NEW
) -> Passport
```

**Files:**
- `spec-core/Cargo.toml`: add sha2, hex deps
- `spec-core/src/passport.rs`: `contract_hash` field, `compute_contract_hash()`, `read_passport()`,
  `passport_path_for()` helper
- `spec-cli/src/commands.rs`: update `write_passports` caller to pass `contract_hash`

**Acceptance:**
- `spec test` on a unit with a contract writes `contract_hash` to its passport
- `spec test` on a unit without a contract: `contract_hash` absent from JSON
- `spec generate` does NOT write `contract_hash` (stays None/absent)
- Old passports (no field) load without error
- Editing the contract and re-running `spec test` produces a different `contract_hash`
- Swapping two inputs in the contract produces a different hash (order-sensitive)

**Tests:**

In `spec-core/src/passport.rs` unit tests:
- `test_contract_hash_absent_for_no_contract` — unit without contract → hash is None
- `test_contract_hash_present_for_contract` — unit with contract → hash is Some(hex string)
- `test_contract_hash_changes_on_input_reorder` — inputs [a, b] vs [b, a] → different hashes
- `test_read_passport_returns_none_for_missing` — no passport file → Ok(None)
- `test_read_passport_returns_err_for_malformed` — malformed JSON → Err
- `test_read_passport_roundtrip` — write then read → equal struct

In `spec-cli/tests/cli.rs` integration:
- `spec_test_writes_contract_hash_to_passport` — `spec test` on a unit with a contract → passport
  JSON contains `contract_hash` field
- `spec_generate_does_not_write_contract_hash` — `spec generate` on a unit with a contract →
  passport JSON does NOT contain `contract_hash` field (field absent, not null)

---

## D3 — `--format json` on `spec validate`

AI agents use `spec validate --format json` as their primary feedback mechanism. Human text output
is suppressed; structured JSON goes to stdout; errors/panics go to stderr.

### Clap change

Add `--format` to `ValidateArgs`:

```rust
#[derive(Args, Debug)]
pub struct ValidateArgs {
    pub path: PathBuf,
    #[arg(long)]
    pub no_strict: bool,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
}
```

`OutputFormat` also used by `StatusArgs` (D4) — define once, use in both.

### JSON output schema

```json
{
  "schema_version": 1,
  "status": "invalid",
  "errors": [
    {
      "unit": "shipping/calculate",
      "code": "MissingDep",
      "dep": "currency/convert",
      "path": "units/shipping/calculate.unit.spec"
    },
    {
      "unit": "shipping/calculate",
      "code": "ContractTypeInvalid",
      "field": "contract.inputs.weight",
      "value": "Decmal",
      "path": "units/shipping/calculate.unit.spec"
    }
  ],
  "warnings": []
}
```

**Critical:** `code` values are the PascalCase names of `SpecError` variants from
`spec-core/src/lib.rs`. The complete registry at v0.5:

| Code | When |
|------|------|
| `Io` | IO error reading file |
| `InvalidUtf8` | File is not valid UTF-8 |
| `YamlParse` | YAML parse error |
| `Json` | JSON serialization error |
| `SchemaValidation` | JSON schema validation failed |
| `SemanticValidation` | Semantic validation error |
| `RustKeyword` | ID segment is a Rust reserved keyword |
| `DuplicateId` | Same unit ID in two files |
| `DepCollision` | Two deps resolve to same fn_name |
| `MissingDep` | Dep not found in spec set |
| `CyclicDep` | Cyclic dependency |
| `UseStatementInBody` | use statement in body.rust |
| `BodyRustMustBeBlock` | body.rust failed to parse as a block |
| `BodyRustLooksLikeFnDeclaration` | body.rust looks like a full fn declaration |
| `LocalTestExpectNotExpr` | local_tests[].expect is not a valid Rust expression |
| `DuplicateLocalTestId` | duplicate local_tests[].id within a unit |
| `ContractTypeInvalid` | contract field has invalid Rust type |
| `ContractInputNameInvalid` | contract.inputs key is not a valid Rust identifier |
| `Traversal` | Directory traversal error |
| `Generator` | Code generation error |
| `OutputDir` | Output directory error |
| `MissingMarker` | Missing .spec-generated marker |

**Prior learning applied:** `json-error-code-name-drift` — the design doc draft used `DepNotFound`
and `InvalidType` but the actual variants are `MissingDep` and `ContractTypeInvalid`. The table
above is cross-checked directly against `spec-core/src/lib.rs`.

### JSON error object fields

Not all errors have the same fields. Emit only the fields that are non-null/non-empty:

```rust
#[derive(Serialize)]
struct JsonErrorEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,       // unit ID, looked up from path via id_by_path map
    code: String,               // SpecError variant name
    path: String,               // source file path; for DuplicateId uses file1; "" for Io/Json
    #[serde(skip_serializing_if = "Option::is_none")]
    dep: Option<String>,        // for MissingDep, DepCollision
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,      // for ContractTypeInvalid, ContractInputNameInvalid
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,      // for ContractTypeInvalid (the invalid type string)
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,    // for YamlParse, SchemaValidation, SemanticValidation, Io, Json, Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,         // for DuplicateId, DuplicateLocalTestId, LocalTestExpectNotExpr
    // Explicit structured fields for automation-hostile cases:
    #[serde(skip_serializing_if = "Option::is_none")]
    path2: Option<String>,      // for DuplicateId: the second conflicting file path
    #[serde(skip_serializing_if = "Option::is_none")]
    cycle: Option<Vec<String>>, // for CyclicDep: the full cycle route as a Vec of unit IDs
}
```

Derive a `JsonErrorEntry` from each `SpecError` via a new function:

```rust
fn spec_error_to_json_entry(
    err: &SpecError,
    id_by_path: &HashMap<String, String>,  // file_path → unit_id, from loaded specs
) -> JsonErrorEntry
```

**Special cases in the match (cross-model finding applied):**

- `DuplicateId { id, file1, file2 }`: `path = file1.clone()`, `path2 = Some(file2.clone())`,
  `id = Some(id.clone())`. Both paths are machine-parseable as explicit fields.
- `Io(_)` / `Json(_)`: `path = "".to_string()`, `message = Some(err.to_string())`.
  Known limitation: Io/Json wrapped errors don't carry path context. Agents should treat
  path="" as "file context unavailable."
- `CyclicDep { cycle_path, path }`: `path = path.clone()`,
  `cycle = Some(cycle_path.clone())`. The full cycle route is parseable as a Vec.

**Unit ID derivation:** In `validate_command`, build the map before formatting JSON:
```rust
let id_by_path: HashMap<String, String> = specs.iter()
    .map(|s| (s.source.file_path.clone(), s.spec.id.clone()))
    .collect();
```
Pass to `spec_error_to_json_entry`. For errors without a path match (e.g., `Io`, `Json`),
`unit` field is `None`.

### Exit codes

- exit 0: all units valid
- exit 1: any unit invalid
- exit 2: internal error (panic hook, D6)

When `--format json` is active:
- All human-readable text output suppressed (no println! for validate)
- JSON goes to stdout only
- Internal errors/panics go to stderr (panic hook, D6)

### Files

- `spec-cli/src/commands.rs`: add `OutputFormat` enum, `--format` to `ValidateArgs`,
  `spec_error_to_json_entry()`, JSON output path in `validate_command()`

**Acceptance:**
- `spec validate --format json <valid-path>` → stdout is valid JSON, status "valid", errors []
- `spec validate --format json <invalid-path>` → stdout is valid JSON, status "invalid", errors populated
- Exit codes: 0 (valid), 1 (invalid)
- All JSON output parseable by `serde_json::from_str` in tests
- Human-readable text suppressed when `--format json`
- Warnings array present (may be empty)

**Tests:**

In `spec-cli/tests/cli.rs`:
- `spec_validate_json_all_valid` — all-valid spec dir → exit 0, JSON with status "valid", errors []
- `spec_validate_json_missing_dep` — unit with MissingDep → exit 1, code "MissingDep", path present
- `spec_validate_json_contract_type_invalid` — bad type → exit 1, code "ContractTypeInvalid", field present
- `spec_validate_json_no_human_text_on_stdout` — no human text leaks to stdout when --format json
- `spec_validate_json_zero_units` — empty dir → exit 0, JSON with status "valid", empty arrays
- `spec_validate_json_schema_version_is_1` — schema_version field equals 1

---

## D4 — `spec status [path]`

The entry point for any AI agent starting a work session. Gives a per-unit status view:
valid / invalid / stale / no-evidence.

### Human-readable output

```
✓ pricing/apply_tax       valid  evidence:2026-04-05T14:30:00Z
✓ money/round             valid  evidence:2026-04-05T14:30:01Z
✗ shipping/calculate      invalid  (2 errors)
  · MissingDep: dep 'currency/convert' not found
  · ContractTypeInvalid: contract.inputs.weight: invalid type 'Decmal'
~ pricing/apply_discount  stale  evidence:2026-04-04T09:00:00Z  (contract changed)
— auth/verify             valid  no-evidence
```

Symbols: `✓` = valid+evidence, `✗` = invalid, `~` = stale, `—` = valid but no evidence.

### Stale detection

A unit is stale when:
- `passport.contract_hash` is Some(hash) AND
- `compute_contract_hash(spec)` != `passport.contract_hash`

Not stale when:
- `passport.contract_hash` is None (old passport or unit with no contract) → no detection
- No passport exists → status is no-evidence, not stale

### Clap

```rust
#[derive(Args, Debug)]
pub struct StatusArgs {
    #[arg(value_name = "PATH")]
    pub path: PathBuf,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}
```

Add `Status(StatusArgs)` to the `Command` enum.

### `status_command()` logic

```
1. collect_specs(path)
2. finish_validation(&specs, errors, &options) → (errors_by_file, warnings)
3. build id_by_path: HashMap<file_path, unit_id> from loaded specs
4. for each spec:
   a. read_passport(spec.source_path) → Option<Passport>
      (warn to stderr + treat as None on Err)
   b. compute_contract_hash(&spec) → Option<String>
   c. determine status:
      - has validation errors → "invalid"
      - passport has contract_hash AND live hash != passport.contract_hash → "stale"
      - passport exists with observed_at → "valid" + evidence timestamp
      - otherwise → "valid" + no-evidence
5. emit: human text or JSON
6. if any unit invalid or stale: std::process::exit(1)
```

**Exit code note:** Use `std::process::exit(1)` directly — NOT `bail!()`. Reason: the status table
IS the output; `bail!` would append an anyhow error message after it, making output noisy. Print
the table first, exit 1 if needed. Same pattern as `cargo test` itself.

### JSON output schema (`spec status --format json`)

```json
{
  "schema_version": 1,
  "units": [
    {
      "id": "pricing/apply_tax",
      "status": "valid",
      "errors": [],
      "evidence_at": "2026-04-05T14:30:00Z",
      "stale": false
    },
    {
      "id": "pricing/apply_discount",
      "status": "valid",
      "errors": [],
      "evidence_at": null,
      "stale": false
    },
    {
      "id": "shipping/calculate",
      "status": "invalid",
      "errors": [
        {"code": "MissingDep", "dep": "currency/convert", "path": "..."}
      ],
      "evidence_at": null,
      "stale": false
    }
  ]
}
```

Field notes:
- `status`: `"valid"` | `"invalid"` | `"stale"`
- `stale`: true when contract_hash mismatch; false otherwise (including when no detection possible)
- `evidence_at`: ISO 8601 UTC string or `null`
- `errors`: same entry shape as `validate --format json` errors

Exit codes:
- exit 0: all units valid and not stale
- exit 1: any unit invalid or stale
- exit 2: internal error

**Files:**
- `spec-cli/src/commands.rs`: `StatusArgs`, `status_command()`, `Status` variant in `Command` enum

**Acceptance:**
- `spec status <dir>` human output: shows all units with correct symbols
- `spec status <dir> --format json`: parseable JSON, schema_version 1
- Stale unit (`~`): shown when contract changed since last `spec test`
- Malformed passport: warning to stderr, unit shown as no-evidence (does not abort)
- No passports at all: all units show as no-evidence
- Exit 0 when all valid + not stale; exit 1 when any invalid/stale

**Tests:**

In `spec-cli/tests/cli.rs`:
- `spec_status_all_valid_no_evidence` — fresh spec dir, no passports → exit 0, all no-evidence
- `spec_status_after_spec_test` — after running spec test → exit 0, units show evidence timestamps
- `spec_status_invalid_unit` — unit with validation error → exit 1, `✗` in output / status "invalid" in JSON
- `spec_status_stale_unit` — edit contract after spec test → exit 1, `~` in output / stale:true in JSON
- `spec_status_json_format` — `--format json` → valid JSON, schema_version 1
- `spec_status_malformed_passport_warns_not_aborts` — truncated passport JSON → stderr warning, continues
- `spec_status_single_file_path` — path to a single .unit.spec → shows that unit only

---

## D5 — `spec test [path]` Single-Unit Scoping

AI agents implement one unit at a time. They need to run tests scoped to that unit without
triggering the full tree. Also fixes the UX: currently `spec test` rejects file paths with a bail.

### Remove the file path bail

In `test_command()` (`spec-cli/src/commands.rs`, near line 448), remove:

```rust
if path.is_file() {
    bail!("❌ spec test requires a directory path — pass the units directory, not a single file");
}
```

**Prior learning applied:** `spec-test-file-path-bail` — this bail exists and must be removed.
A regression test is required to verify the file path is now accepted.

### Single-file vs directory dispatch

```
if path.is_file():
    target_spec = load_file(path)
    spec_root = path.parent()
    generate ALL units in spec_root (needed for dep resolution + full compilation)
    filter = cargo_filter_for(target_spec, output_prefix)
    run_cargo_test(crate_root, target_dir, filter=Some(filter))
    detect zero_tests_ran(output) → exit non-zero, no passport write
    write passport for target unit ONLY (with contract_hash)
else (directory):
    existing flow unchanged (all units, no filter)
```

### Cargo test filter derivation

The filter is a substring of the full cargo test name. Given `pricing/apply_tax`:

```
module_path = "pricing"
fn_name = "apply_tax"
filter = "{output_prefix}::pricing::apply_tax::tests::"
       = "spec::pricing::apply_tax::tests::"
```

For a root-level unit `money/round` (no module path):
```
filter = "{output_prefix}::round::tests::"
       = "spec::round::tests::"
```

**Prior learning applied:** `json-filter-text-inconsistency` — use the full module path
(`pricing::apply_tax::tests::`), not a substring of the last ID segment. This is consistent
with `expected_cargo_test_name` in `commands.rs`.

### `run_cargo_test` API change

In `spec-core/src/pipeline.rs`:

```rust
pub fn run_cargo_test(
    crate_root: &Path,
    cargo_target_dir: &Path,
    filter: Option<&str>,
) -> Result<CargoResult>
```

When `filter` is `Some(f)`: pass `-- {f}` after `cargo test` args.
When `filter` is `None`: existing behavior (no filter).

**One caller to update:** `test_command` in `spec-cli/src/commands.rs` (currently passes nothing).

### 0-tests-ran detection

After `run_cargo_test`, parse the output for the test result summary line:

```
test result: ok. 0 passed; 0 failed; 0 ignored; ...
```

If `0 passed` AND `0 failed` (no tests ran): exit non-zero, do NOT write passport evidence.

**Prior learning applied:** `zero-tests-matched-misleading-evidence` — cargo exits 0 when 0 tests
match a filter. Without this guard, spec writes a passport with empty `test_results` and a valid
`observed_at` timestamp — misleading evidence that says the unit was "tested" when it wasn't.

New function in `pipeline.rs`:

```rust
/// Returns true if the cargo test output shows that 0 tests ran
/// (filter matched nothing). Distinct from "all tests passing."
pub fn zero_tests_ran(output: &str) -> bool
```

### Passport write scope

When single-file mode: write passport ONLY for the target unit. Other units in the spec root
retain their existing passports unchanged.

`contract_hash` for the target unit is computed from the loaded spec and written to its passport.

### Files

- `spec-core/src/pipeline.rs`: add `filter: Option<&str>` to `run_cargo_test()`, add `zero_tests_ran()`
- `spec-cli/src/commands.rs`:
  - remove `is_file()` bail from `test_command()`
  - add single-file dispatch path
  - update `run_cargo_test` call to pass filter

**Acceptance:**
- `spec test <path/to/unit.unit.spec>` accepted (not rejected with bail)
- Only the target unit's passport is updated; sibling unit passports unchanged
- If cargo filter matches 0 tests: exit non-zero, no passport written
- `spec test <dir>` (directory path) still works exactly as before

**Tests:**

In `spec-cli/tests/cli.rs`:
- `spec_test_accepts_file_path` — **REGRESSION TEST (CRITICAL)** — `spec test <unit.unit.spec>`
  exits 0, passport written. Positive test: verifies the file path is accepted, not just that
  no error fires. (Was previously bailing with "requires directory path".)
- `spec_test_file_path_only_writes_target_passport` — after `spec test <file>`, only target unit
  passport updated; sibling unit passports unchanged
- `spec_test_zero_tests_matched_exits_nonzero` — **REQUIRED** — filter matching 0 tests → exit
  non-zero, no passport written. Guards against `zero-tests-matched-misleading-evidence` pitfall.
- `spec_test_directory_path_unchanged` — existing directory path behavior still works

In `spec-core/src/pipeline.rs` unit tests:
- `test_zero_tests_ran_detects_empty_run` — "0 passed; 0 failed" → true
- `test_zero_tests_ran_false_for_passing_tests` — "3 passed; 0 failed" → false
- `test_run_cargo_test_with_filter_appends_filter_arg` — filter arg appears in constructed command

---

## D6 — Panic Hook in `main.rs`

Exit code 2 distinguishes internal errors from validation failures (exit 1). Agents can detect
panics programmatically.

Add at the top of `main()` before `Cli::parse()`:

```rust
std::panic::set_hook(Box::new(|info| {
    eprintln!("internal error: {info}");
    std::process::exit(2);
}));
```

**Files:**
- `spec-cli/src/main.rs`

**Acceptance:**
- Panic exits with code 2
- Panic message goes to stderr

**Note on tests:** No automated integration test for the panic hook itself — injecting a panic
in an integration test is unergonomic. Rely on code review and the fact that `std::process::exit(2)`
is unconditional. Add a comment in `main.rs` explaining the exit code contract (0=success,
1=validation errors, 2=internal/panic).

---

## D7 — Golden JSON Fixture Tests

Golden fixtures make `--format json` output shape breakage a test failure. Committed under
`spec-cli/tests/fixtures/`. Integration tests diff actual output against them using `serde_json::Value`
comparison (normalizes whitespace/key order, catches semantic shape changes).

### Fixture files

Four files, each committed as static JSON:

1. `spec-cli/tests/fixtures/validate-valid.json` — `spec validate --format json` on all-valid units
2. `spec-cli/tests/fixtures/validate-invalid.json` — `spec validate --format json` on units with errors
3. `spec-cli/tests/fixtures/status-valid.json` — `spec status --format json` on valid + evidenced units
4. `spec-cli/tests/fixtures/status-stale.json` — `spec status --format json` with a stale unit

**How to generate initial fixtures:** Run the actual commands on a purpose-built test fixture dir
(not ecommerce — use a minimal controlled dir), capture output, verify by hand, commit as golden files.

**Test pattern:**

```rust
#[test]
fn spec_validate_json_golden_valid() {
    // ... set up fixture dir ...
    let output = run(&["validate", "--format", "json", units_dir_str]);
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        include_str!("fixtures/validate-valid.json")
    ).unwrap();
    assert_eq!(actual, expected, "validate --format json output shape changed");
}
```

**How to update fixtures:** When intentional schema changes happen, update the fixtures as part of
the same commit. The diff makes the shape change explicit in review.

**Files:**
- `spec-cli/tests/fixtures/validate-valid.json` (new)
- `spec-cli/tests/fixtures/validate-invalid.json` (new)
- `spec-cli/tests/fixtures/status-valid.json` (new)
- `spec-cli/tests/fixtures/status-stale.json` (new)
- `spec-cli/tests/cli.rs`: 4 golden fixture tests

**Acceptance:**
- All 4 golden fixtures pass on clean build
- Changing `schema_version` from 1 to 2 in the code causes fixture tests to fail
- Fixture files are valid JSON and committed in the repo

---

## D8 — Companion gstack Skill

A gstack skill at `.claude/skills/spec/SKILL.md` in the spec repo. Engineers who've cloned spec
already have it — no separate install step.

**Scope of the skill:**
1. The 5-step agent workflow (same as AGENTS.md, but in skill format so `/spec` loads it)
2. How to read `.unit.spec` files (id, intent, contract, body, local_tests fields)
3. Common validation errors and how to fix them (table: code → what's wrong → fix)
4. How to write valid `local_tests.expect` expressions (must be a Rust expression, not a block)
5. How to interpret passport evidence (what observed_at means, what stale means)

**Format:** Standard gstack SKILL.md format (YAML frontmatter + markdown body).
See `~/.claude/skills/gstack/office-hours/SKILL.md` for template.

**Files:**
- `.claude/skills/spec/SKILL.md` (new file; create parent dir if needed)

**Acceptance:**
- `/spec` loads correctly in a Claude Code session in the spec repo
- The workflow loop is accurate (matches actual CLI behavior shipped in M5)
- Error table includes all named JSON codes from D3

---

## D9 — README AI-Native Usage Section

Document the AI-native workflow for external developers. This bridges "someone clones spec"
and "they know how to use it with their AI agent."

**Add `## AI-Native Usage` section** (after Pipeline, before Contributing):
1. Why: AI writes code faster when it has contracts to validate against
2. The workflow loop
3. `spec validate --format json` output format (brief example)
4. `spec status` symbols reference
5. Companion skill install instructions

**Acceptance:**
- New section in README.md
- `spec validate --format json` example output shown
- `spec status` symbols explained

---

## NOT in Scope (M5)

| Item | Deferred to | Rationale |
|------|-------------|-----------|
| `spec build` / `spec test` structured JSON output | M6 | Build/test errors are already in passport evidence. `--format json` on build/test is lower value than status/validate. |
| Stable external error code namespace (SPEC_DEP_NOT_FOUND) | Pre-M6 | Required before TypeScript generator lands. M5 codes are internal variant names. |
| ValidatedExpr newtype refactor | 0.5.x patch | No M5 feature depends on it. Ships as cleanup. |
| `wait_timeout` for cargo processes | 0.5.x patch | SIGINT still propagates. Documented as known hang. |
| `parse_test_output` HashMap optimization | 0.5.x patch | Correctness first; no performance problem yet. |
| Nextest limitation documentation | 0.5.x patch | Carried from M4. XS effort, deferred as non-blocking. |
| Cross-library dep implementation | M6 | Schema decided in M4 (D6). Implementation is M6 scope. |
| Blueprint layer (planning docs) | M6+ | Solo-engineer ICP doesn't need it yet. |
| Language-agnostic generator (TypeScript, Python, Go) | M6 | M5 infrastructure designed language-agnostically. Second generator is early M6. |
| MCP server integration | Indefinite | Explicit design decision: spec stays a pure CLI. |
| `spec generate` stale detection | M6 | `contract_hash` written by `spec test` only. `spec generate` doesn't run tests. |

---

## What Already Exists

| Existing | Used by | How |
|----------|---------|-----|
| `collect_specs(path)` | D4 spec status | Load all units from dir or file |
| `finish_validation()` | D4 spec status | Run full validation on loaded specs |
| `error_key()` | D3 --format json | Already extracts path string from SpecError |
| `run_cargo_test()` | D5 single-unit | Extended with filter param |
| `parse_cargo_test_output()` | D5 single-unit | Existing; unchanged |
| `build_passport_with_evidence()` | D5 single-unit | Updated to accept contract_hash |
| `write_passport()` | D5 single-unit | Unchanged |
| `rfc3339_now()` | D4, D5 | Unchanged |
| `expected_cargo_test_name()` | D5 filter derivation | Pattern reference for filter construction |
| `OutputFormat` (new in D3) | D3, D4 | Defined once; shared between ValidateArgs and StatusArgs |
| ecommerce example | D7 golden fixtures | Reference spec set for fixture generation |

---

## Files Modified

```
spec-core/Cargo.toml              add sha2, hex deps
spec-core/src/passport.rs         contract_hash field, compute_contract_hash(), read_passport(),
                                  passport_path_for() helper, build_passport_with_evidence update
spec-core/src/pipeline.rs         filter: Option<&str> on run_cargo_test(), zero_tests_ran()
spec-cli/src/main.rs              panic hook + exit code comment
spec-cli/src/commands.rs          OutputFormat enum, --format on ValidateArgs + StatusArgs,
                                  StatusArgs + status_command(), remove file bail from test_command(),
                                  single-unit dispatch in test_command(), spec_error_to_json_entry()
spec-cli/tests/cli.rs             all new integration tests for D2–D7
spec-cli/tests/fixtures/          4 new golden fixture JSON files (D7) [NEW DIR]
AGENTS.md                         append spec agent workflow section (D1)
DECISIONS.md                      ICP paragraph (D0)
.claude/skills/spec/SKILL.md      companion skill (D8) [NEW FILE]
README.md                         AI-Native Usage section (D9)
```

---

## Code Flow Diagrams

### `spec status` data flow

```
status_command(path, format)
    │
    ├── collect_specs(path) ──────────────────────── existing
    │       └── Vec<LoadedSpec>
    │
    ├── finish_validation(&specs, ...) ────────────── existing
    │       └── DiagnosticMap (errors by file)
    │
    └── for each spec in specs:
            │
            ├── read_passport(source_path) ─────────── NEW (passport.rs)
            │       └── Ok(Some(Passport))
            │           Ok(None) if file missing
            │           Err → warn stderr + treat as None
            │
            ├── compute_contract_hash(&spec) ──────── NEW (passport.rs)
            │       └── Option<String>
            │
            └── determine StatusEntry:
                    has errors?  ─────────────────── → "invalid"
                    hash mismatch? ───────────────── → "stale"
                    passport.evidence.observed_at? ─ → "valid" + timestamp
                    otherwise ────────────────────── → "valid" + no-evidence
```

### `spec test <file>` flow

```
test_command(path="units/pricing/apply_tax.unit.spec", ...)
    │
    ├── path.is_file() == true  [new branch; was bailing here]
    │       │
    │       ├── load_file(path) → LoadedSpec (target unit)
    │       ├── spec_root = path.parent()
    │       └── generate_specs(spec_root, output) → all units compiled
    │
    ├── resolve_pipeline_context(spec_root, ...)
    │
    ├── run_cargo_build(crate_root, target_dir) ──── existing
    │
    ├── derive filter:
    │       "spec::pricing::apply_tax::tests::"
    │
    ├── run_cargo_test(crate_root, target_dir, filter=Some("..."))  ← API change
    │
    ├── zero_tests_ran(output)?  ───────────────── NEW guard
    │       true → eprintln! + exit non-zero (no passport)
    │
    ├── parse_cargo_test_output(output) ─────────── existing
    │
    └── write_passports([target_spec_only], contract_hash)
```

---

## Test Coverage Diagram

```
CODE PATH COVERAGE — M5 new code
==================================

[+] spec-core/src/passport.rs
    │
    ├── compute_contract_hash()
    │   ├── [GAP] no contract → None                 (D2 unit test)
    │   ├── [GAP] contract present → Some(hex)       (D2 unit test)
    │   └── [GAP] input reorder changes hash         (D2 unit test)
    │
    ├── read_passport()
    │   ├── [GAP] file missing → Ok(None)            (D2 unit test)
    │   ├── [GAP] malformed JSON → Err               (D2 unit test)
    │   └── [GAP] write+read roundtrip → equal       (D2 unit test)
    │
    └── contract_hash written by spec test
        └── [GAP] passport JSON has contract_hash    (D2 integration)

[+] spec-core/src/pipeline.rs
    │
    ├── run_cargo_test(filter=Some(...))
    │   └── [GAP] filter arg appended to command     (D5 unit test)
    │
    └── zero_tests_ran()
        ├── [GAP] "0 passed; 0 failed" → true        (D5 unit test)
        └── [GAP] "3 passed; 0 failed" → false       (D5 unit test)

[+] spec validate --format json
    │
    ├── all valid → status "valid", errors []         (D3 integration + D7 golden)
    ├── invalid → status "invalid", errors populated  (D3 integration + D7 golden)
    ├── code "MissingDep" correct (not "DepNotFound") (D3 integration)
    ├── no human text on stdout when --format json    (D3 integration)
    ├── zero units → status "valid", empty arrays     (D3 integration)
    └── schema_version = 1                            (D3 integration + D7 golden)

[+] spec status
    │
    ├── all valid, no evidence → no-evidence symbols  (D4 integration)
    ├── after spec test → shows evidence timestamps   (D4 integration)
    ├── invalid unit → exit 1, ✗ symbol               (D4 integration)
    ├── stale unit → exit 1, ~ symbol                 (D4 integration)
    ├── --format json → valid JSON, schema_version 1  (D4 integration + D7 golden)
    ├── malformed passport → warn stderr, continues   (D4 integration)
    └── single file path → shows that unit only       (D4 integration)

[+] spec test single-unit
    │
    ├── [REGRESSION] file path accepted               (D5 integration — CRITICAL)
    ├── only target passport updated                  (D5 integration)
    ├── [REQUIRED] zero tests → exit non-zero,        (D5 integration — CRITICAL)
    │             no passport written
    └── directory path unchanged                      (D5 integration)

[+] golden fixtures
    ├── validate-valid.json shape stable              (D7 golden)
    ├── validate-invalid.json shape stable            (D7 golden)
    ├── status-valid.json shape stable                (D7 golden)
    └── status-stale.json shape stable                (D7 golden)

──────────────────────────────────────────────────────
COVERAGE: All new code paths have required tests
REGRESSION TESTS: 2 critical (spec_test_accepts_file_path,
                               spec_test_zero_tests_matched_exits_nonzero)
──────────────────────────────────────────────────────
```

---

## Failure Modes

| Codepath | Realistic failure | Test? | Error handling? | Silent? |
|----------|-------------------|-------|-----------------|---------|
| `compute_contract_hash` | `serde_json::to_string` fails | No (cannot fail for well-formed spec) | `expect()` → panic (exit 2) | No |
| `read_passport` | Passport truncated mid-write | Yes (D2 unit test) | Returns Err; caller warns + treats as None | No |
| `run_cargo_test` with filter | Filter derivation wrong module path | Yes (D5 unit test) | 0-tests-ran detection catches misses | No |
| `zero_tests_ran` | Cargo changes summary line format in future | Partial (D5 unit tests on current format) | None — detection fails silently | **Yes — document in pipeline.rs** |
| `spec status` malformed passport | Corrupt JSON | Yes (D4 integration) | Warn stderr, treat as no-evidence | No |
| `--format json` schema drift | Format changes without fixture update | Yes (D7 golden) | Golden fixture tests catch this | No |
| `contract_hash` on IndexMap | serde_json serialization order changes between versions | No | None — would cause spurious stale detection | **Yes — low risk, document** |

**Critical gap flagged (medium risk):** `zero_tests_ran` parses cargo's output format. If cargo
changes its summary line format in a future release, the detection silently fails and passports get
misleading evidence. Mitigation: document this fragility in `pipeline.rs` with a comment. The
golden fixture tests would catch a format regression if the ecommerce tests exercise this path.

---

## Parallelization Strategy

Four lanes can run in parallel:

| Lane | Steps | Modules touched | Depends on |
|------|-------|----------------|------------|
| A | D0 + D1 + D6 + D8 + D9 | DECISIONS.md, AGENTS.md, main.rs, .claude/skills/, README.md | — |
| B | D2 | spec-core/src/passport.rs, spec-core/Cargo.toml | — |
| C | D3 | spec-cli/src/commands.rs (validate path only) | — |
| E | D4 | spec-cli/src/commands.rs (status command) | B (read_passport, compute_contract_hash) |
| F | D5 | spec-cli/src/commands.rs (test command), spec-core/src/pipeline.rs | B (contract_hash write) |
| G | D7 | spec-cli/tests/fixtures/, spec-cli/tests/cli.rs | C + E stable |

Execution:
1. Launch A + B + C in parallel
2. When B completes: launch E and F
3. When C + E complete: launch G

**Conflict flag:** C, E, and F all touch `spec-cli/src/commands.rs`. If using parallel worktrees,
merge C before starting E+F, or carefully coordinate. The changes affect different functions
(`validate_command` vs `status_command` vs `test_command`) so merge conflicts are low risk but not zero.

---

## Completion Criteria (M5 Done)

- [ ] D0: ICP paragraph in DECISIONS.md
- [ ] D1: AGENTS.md spec agent workflow section
- [ ] D2: `contract_hash` field in Passport, `read_passport()`, `compute_contract_hash()`
- [ ] D3: `spec validate --format json` working, JSON output matches schema
- [ ] D4: `spec status [path]` command, human + JSON output, stale detection working
- [ ] D5: `spec test <file>` accepted, single-unit passport write, 0-tests-ran guard active
- [ ] D6: Panic hook in main.rs
- [ ] D7: 4 golden JSON fixtures committed, fixture integration tests passing
- [ ] D8: Companion skill at `.claude/skills/spec/SKILL.md`
- [ ] D9: README AI-Native Usage section
- [ ] All integration tests passing on CI (linux + macos matrix)
- [ ] Version bumped to 0.5.0 in `Cargo.toml` workspace

---

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 3 | clean (PLAN) | mode: HOLD_SCOPE, 0 critical gaps |
| Codex Review | `/codex review` | Independent 2nd opinion | 7+ | issues_found | sha256: prefix + explicit JSON fields accepted |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR (PLAN) | 5 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | issues_open | score: 5/10 → 7/10 (on prior plan) |

**CODEX:** sha256: hash prefix + path2/cycle explicit fields — both incorporated.
**CROSS-MODEL:** 2 tensions resolved — both Claude and Codex agree on complete options.
**UNRESOLVED:** 0
**VERDICT:** ENG CLEARED — ready to implement.
