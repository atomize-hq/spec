mod commands;
mod config;

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
    // Exit codes: 0 = success, 1 = validation failures, 2 = internal panic.
    // This lets AI clients distinguish authored spec problems from tool bugs.
    std::panic::set_hook(Box::new(|info| {
        eprintln!("internal error: {info}");
        std::process::exit(2);
    }));
    let cli = Cli::parse();
    cli.command.run()
}
