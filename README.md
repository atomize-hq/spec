# spec

`spec` is a Rust workspace for authoring semantic units in `*.unit.spec` files, validating them with JSON Schema, normalizing them into internal IR, and generating readable Rust source.

## Workflow

1. Author a `.unit.spec` file.
2. Validate it with the CLI.
3. Build: `spec build` validates, generates, and compiles in one step.
4. Test: `spec test` runs the full pipeline and writes observed evidence to passports.
5. Export: `spec export` emits a machine-readable JSON bundle for downstream tooling.

## Workspace

- `spec-core`: parsing, validation, normalization, generation, pipeline, and export primitives
- `spec-cli`: CLI for `validate`, `generate`, `build`, `test`, and `export`
- `examples/ecommerce`: a small realistic example with pricing units

## Quickstart

```bash
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units
cargo run -p spec-cli -- generate examples/ecommerce/units
```

After installation, the binary is `spec`:

```bash
cargo install spec-cli
spec validate examples/ecommerce/units
spec generate examples/ecommerce/units
```

## Spec format

Each unit is a YAML document with these required fields:

- `id`: hierarchical unit id like `pricing/apply_discount`
- `kind`: currently `function`
- `intent.why`: why the unit exists
- `body.rust`: the function body as a Rust block expression (`{ ... }`, braces included)

Optional fields include `contract`, `deps`, `imports`, `local_tests`, and `links`.

`spec` generates the complete `pub fn` signature from `contract.inputs` and `contract.returns`. A minimal unit with a contract looks like:

```yaml
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

This generates:

```rust
pub fn apply_tax(subtotal: Decimal, rate: Decimal) -> Decimal {
    let taxed = subtotal + subtotal * rate;
    round(taxed)
}
```

## Migrating from 0.2.x

In 0.2.x, `body.rust` contained the full function declaration:

```yaml
# 0.2.x format (no longer valid)
body:
  rust: |
    pub fn apply_tax(subtotal: Decimal, rate: Decimal) -> Decimal {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

In 0.3.0, strip the `pub fn name(params) -> ReturnType` line and keep only the body block. Move the parameter names and types into `contract.inputs`, and the return type into `contract.returns`:

```yaml
# 0.3.0 format
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

Running `spec validate` on a 0.2.x unit will emit a clear migration error pointing to the file.

## Example

The ecommerce example demonstrates four units across two modules:

- `money/round`
- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`

The example crate is intentionally minimal. It provides a realistic place to keep unit specs and a Rust project scaffold that can host generated output.

## Commands

```bash
spec validate <path>                      # schema + semantic validation
spec validate <path> --no-strict          # downgrade missing deps to warnings
spec validate <path> --format json        # machine-readable JSON output for agents
spec generate <path>                      # emit .rs files (default: {crate_root}/src/generated)
spec generate <path> --output <dir>       # emit .rs files to explicit directory

spec build <path>                         # validate → generate → cargo build
spec build <path> --output <dir>          # explicit output directory
spec test  <path>                         # spec build → cargo test, writes evidence to passports
spec test  <path> --output <dir>          # explicit output directory
spec test  <path/to/unit.unit.spec>       # scope to a single unit (filter by module path)

spec status <path>                        # per-unit health: valid/invalid/stale/incomplete/untested/failing
spec status <path> --format json          # machine-readable status for agents

spec export <path>                        # emit JSON bundle to stdout
spec export <path> --output <file>        # write JSON bundle to file
```

`validate` checks schema and semantic rules. `--no-strict` downgrades missing internal deps to warnings for validation only. `generate` always remains strict and emits `.rs` files under the output directory while managing `mod.rs` files plus the `.spec-generated` safety marker.

`spec build` and `spec test` wrap the full pipeline so you can validate, generate, and compile in one step. `spec test` also updates each unit's `.spec.passport.json` with observed pass/fail evidence.

When you pass a single `.unit.spec` file to `spec validate`, `spec generate`, `spec test`, or `spec export`, the CLI stays scoped to that exact unit. Sibling `.test.spec` files are directory-scoped and are only loaded for directory invocations.

`spec export` emits a machine-readable JSON bundle containing all units, passports, dependency graph edges, and warnings for any passports that could not be read.

The `--output` path for `generate`/`build`/`test` must resolve to a directory inside your project root. Paths that escape the project root are rejected as a safety guardrail to prevent accidental deletion of files outside the project.

**Nextest:** `spec test` parses standard `cargo test` output format only. `cargo nextest` uses a different output format and is not supported. Running `spec test` in a project configured for nextest will produce `status: "unknown"` for all local tests. Use standard `cargo test`.

For both `.unit.spec` and directory-scoped `.test.spec` validation, the path segment `molecule_tests` is reserved. Molecule tests generate `molecule_tests.rs` per namespace, so allowing that literal segment in an authored spec ID would create module/file collisions in generated output.

## AI-Native Usage

`spec` is especially useful when an AI agent is the one making the edit loop. The toolchain gives the agent a structured contract to follow, a machine-readable validation result to fix against, and a passport trail that records what was actually observed to pass.

The loop is simple: inspect status, validate the exact unit, edit the `.unit.spec`, build to catch Rust-level issues, then test to write fresh evidence. Single-file `validate`, `generate`, and `test` stay on that unit and do not pull sibling molecule tests into the run.

```bash
spec validate examples/ecommerce/units --format json
```

```json
{
  "schema_version": 2,
  "status": "invalid",
  "errors": [
    {
      "unit": "shipping/calculate",
      "code": "SPEC_MISSING_DEP",
      "dep": "currency/convert",
      "path": "units/shipping/calculate.unit.spec"
    }
  ],
  "warnings": []
}
```

That JSON form is meant for agents: parse `status`, `errors`, and `warnings` instead of scraping terminal prose. Pre-validation workspace-config failures, including broken `[libraries]` entries, also stay in this JSON contract for `validate --format json`.

`spec status` uses simple symbols so you can scan a whole tree quickly. Any unit whose
status is not `valid` exits with code `1`.

For `spec status --format json`, workspace-config failures that happen before any unit row can be computed surface as top-level `loader_errors` entries instead of raw stderr text.

- `✓` valid
- `✗` invalid or failing
- `~` stale
- `?` incomplete
- `—` untested

Use the companion skill at [`.claude/skills/spec/SKILL.md`](.claude/skills/spec/SKILL.md) when you want the full workflow spelled out for an AI coding session.

## Validation error codes

`spec validate --format json` returns error objects with a `code` field. These are the recognized codes returned by the current CLI:

| Code | Description |
|------|-------------|
| `SPEC_IO` | Filesystem I/O error |
| `SPEC_INVALID_UTF8` | File is not valid UTF-8 |
| `SPEC_YAML_PARSE` | YAML syntax error in the unit file |
| `SPEC_JSON` | JSON serialization/deserialization error |
| `SPEC_SCHEMA_VALIDATION` | Unit file failed JSON Schema validation |
| `SPEC_SEMANTIC_VALIDATION` | Unit passed schema but failed a semantic rule |
| `SPEC_RUST_KEYWORD` | An `id` segment is a Rust reserved keyword |
| `SPEC_DUPLICATE_ID` | Two unit files share the same `id` |
| `SPEC_DEP_COLLISION` | Two deps resolve to the same generated function name |
| `SPEC_MISSING_DEP` | A declared dep has no matching unit in the spec set |
| `SPEC_UNKNOWN_LIBRARY_NAMESPACE` | A dep references a library alias that is not configured in `[libraries]` |
| `SPEC_CROSS_LIBRARY_DEP_NOT_FOUND` | A cross-library dep has no matching unit in the resolved library set |
| `SPEC_LIBRARY_CRATE_ALIAS_MISSING` | The root crate is missing the Cargo dependency alias required by a cross-library dep |
| `SPEC_LIBRARY_PATH_NOT_FOUND` | A `[libraries]` entry points to a path that does not exist |
| `SPEC_LIBRARY_OUT_OF_ROOT` | A `[libraries]` entry resolves outside the repo root |
| `SPEC_LIBRARY_ALIAS_SELF` | A `[libraries]` entry points back to the invoking library root |
| `SPEC_DUPLICATE_LIBRARY_ROOT` | Two `[libraries]` aliases resolve to the same canonical root |
| `SPEC_CYCLIC_DEP` | Units form a dependency cycle |
| `SPEC_CROSS_LIBRARY_CYCLE` | Units form a dependency cycle across library boundaries |
| `SPEC_USE_STATEMENT_IN_BODY` | `body.rust` contains a `use` statement — move it to `imports` or `deps` |
| `SPEC_BODY_RUST_MUST_BE_BLOCK` | `body.rust` failed to parse as a Rust block expression |
| `SPEC_BODY_RUST_LOOKS_LIKE_FN_DECLARATION` | `body.rust` contains the full `pub fn` signature — keep only the body block (see migration guide) |
| `SPEC_LOCAL_TEST_EXPECT_NOT_EXPR` | A `local_tests[].expect` value is not a valid Rust expression |
| `SPEC_DUPLICATE_LOCAL_TEST_ID` | Two local tests in the same unit share the same `id` |
| `SPEC_CONTRACT_TYPE_INVALID` | A `contract.inputs` or `contract.returns` type is not valid Rust |
| `SPEC_CONTRACT_INPUT_NAME_INVALID` | A `contract.inputs` key is not a valid Rust identifier |
| `SPEC_TRAVERSAL` | Error walking the units directory tree |
| `SPEC_GENERATOR` | Code generation failure |
| `SPEC_OUTPUT_DIR` | Output directory creation or safety check failed |
| `SPEC_MISSING_MARKER` | Output dir lacks the `.spec-generated` marker — refusing to clean |
| `SPEC_RESERVED_UNIT_NAME` | A slash-delimited spec `id` contains a reserved segment such as `molecule_tests` |

## Consuming Generated Code

Generated units import internal deps with `use crate::...` paths. The consuming crate must
re-export the generated module tree from its root so those paths resolve consistently:

```rust
mod generated;
pub use generated::*;
```

The ecommerce example uses this pattern in
[`examples/ecommerce/src/main.rs`](examples/ecommerce/src/main.rs).

## Workspace Config

An optional `spec.toml` at the repo root can relax `local_tests[].expect` validation for trusted workspaces and configure pipeline defaults:

```toml
[validation]
allow_unsafe_local_test_expect = false

[pipeline]
crate_root = "."          # path to Cargo.toml containing your crate (default: auto-detected)
cargo_target_dir = "target"  # cargo target dir (default: temp dir per run)
timeout_secs = 60         # abort cargo build/test if it exceeds this many seconds
```

When `allow_unsafe_local_test_expect = true`, `local_tests[].expect` still must parse as a Rust expression, but block, unsafe, closure, and other otherwise-rejected expression forms are allowed.

`spec build` and `spec test` auto-detect the nearest member crate (`[package]` Cargo.toml without `[workspace]`) to scope cargo to the right crate in a workspace. Override with `--crate-root <path>` or `[pipeline].crate_root` in `spec.toml`.
