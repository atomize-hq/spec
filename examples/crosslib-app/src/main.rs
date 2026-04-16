use anyhow::Result;

mod generated;
pub use generated::*;

fn main() -> Result<()> {
    println!("cross-library example: generate shared first, then app");
    Ok(())
}
