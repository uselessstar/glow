//! # CLI
//! Command-line interface for xtask.

use clap::{Parser, Subcommand};

use check::check;

mod check;

/// A command-line interface for building and running the project.
#[derive(Subcommand)]
enum Commands {
    /// Checks whether the project is set up correctly and all dependencies are installed.
    Check,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Parses the command-line arguments and executes the appropriate command.
pub fn parse() -> anyhow::Result<()> {
    let d = Cli::parse();

    match d.command {
        Commands::Check => check()?,
    }
    Ok(())
}
