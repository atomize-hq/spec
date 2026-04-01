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

Optional fields include `contract`, `deps`, `local_tests`, and `links`.

## Example

The ecommerce example demonstrates three pricing units:

- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`

The example crate is intentionally minimal. It provides a realistic place to keep unit specs and a Rust project scaffold that can host generated output.

## Commands

The CLI currently supports:

```bash
cargo run -p spec-cli -- validate <path>
cargo run -p spec-cli -- generate <path> --output <dir>
```

`validate` checks schema and semantic rules. `generate` emits `.rs` files under the output directory and manages `mod.rs` files plus the `.spec-generated` safety marker.
