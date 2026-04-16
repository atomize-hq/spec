# crosslib app example

This example demonstrates M9 direct sibling-library reuse.

From the repo root:

```bash
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo run --manifest-path examples/crosslib-app/Cargo.toml -- check
```
