checks:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
    cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
    cargo check --manifest-path examples/ecommerce/Cargo.toml
