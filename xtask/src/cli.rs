//! # CLI
//! Command-line interface for xtask.

use clap::{Parser, Subcommand};

/// A command-line interface for building and running the project.
#[derive(Subcommand)]
enum Commands {}

// CMD
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Parses the command-line arguments and executes the appropriate command.
pub fn parse() {
    Cli::parse();
}
