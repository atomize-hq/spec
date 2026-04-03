# spec

`spec` is a Rust workspace for authoring semantic units in `*.unit.spec` files, validating them with JSON Schema, normalizing them into internal IR, and generating readable Rust source.

## Workflow

1. Author a `.unit.spec` file.
2. Validate it with the CLI.
3. Generate Rust output.
4. Copy or integrate the generated `.rs` files into your Rust project.

## Workspace

- `spec-core`: parsing, validation, normalization, and generation primitives
- `spec-cli`: CLI for `validate` and `generate`
- `examples/ecommerce`: a small realistic example with pricing units

## Quickstart

```bash
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units
cargo run -p spec-cli -- generate examples/ecommerce/units --output generated/spec
```

After installation, the binary is `spec`:

```bash
cargo install spec-cli
spec validate examples/ecommerce/units
spec generate examples/ecommerce/units --output generated/spec
```

## Spec format

Each unit is a YAML document with these required fields:

- `id`: hierarchical unit id like `pricing/apply_discount`
- `kind`: currently `function`
- `intent.why`: why the unit exists
- `body.rust`: verbatim Rust function body

Optional fields include `contract`, `deps`, `imports`, `local_tests`, and `links`.

## Example

The ecommerce example demonstrates four units across two modules:

- `money/round`
- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`

The example crate is intentionally minimal. It provides a realistic place to keep unit specs and a Rust project scaffold that can host generated output.

## Commands

The CLI currently supports:

```bash
cargo run -p spec-cli -- validate <path>
cargo run -p spec-cli -- validate <path> --no-strict
cargo run -p spec-cli -- generate <path> --output <dir>
```

`validate` checks schema and semantic rules. `--no-strict` downgrades missing internal deps to warnings for validation only. `generate` always remains strict and emits `.rs` files under the output directory while managing `mod.rs` files plus the `.spec-generated` safety marker.

The `--output` path must resolve to a directory inside your project root. Paths that escape the project root are rejected as a safety guardrail to prevent accidental deletion of files outside the project.

## Workspace Config

An optional `spec.toml` at the repo root can relax `local_tests[].expect` validation for trusted workspaces:

```toml
[validation]
allow_unsafe_local_test_expect = false
```

When `allow_unsafe_local_test_expect = true`, `local_tests[].expect` still must parse as a Rust expression, but block, unsafe, closure, and other otherwise-rejected expression forms are allowed.
