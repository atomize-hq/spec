mod commands;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "spec",
    bin_name = "spec",
    version,
    about = "Validate and generate Rust from .unit.spec files"
)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.run()
}
