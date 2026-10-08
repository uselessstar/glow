//! # Log
//! Logging utilities for the xtask CLI.

use log::{Level, Record};
use owo_colors::OwoColorize;

use styles::{BLOCK, DEBUG, ERROR, INFO, TRACE, WARN};

use crate::log::styles::MODULE;

mod styles;

/// Simple logger that prints messages to stdout or stderr.
struct XTaskLogger;

/// Writes the message to stdout or stderr depending on the log level.
///
/// # Arguments
/// * `message` - The message to write.
/// * `level` - The log level of the message.
fn write_message(message: &str, level: Level) {
    match level {
        Level::Trace | Level::Debug | Level::Info => {
            println!("{}", message);
        }
        Level::Warn | Level::Error => {
            eprintln!("{}", message);
        }
    }
}

/// Formats the message with the appropriate style based on the log level.
///
/// # Arguments
/// * `message` - The message to format.
/// * `record` - The log record containing the log level and other useful info.
fn format_message(message: &str, record: Record) -> String {
    let binding = record.level();
    let (level_style, separator_style) = match record.level() {
        Level::Trace => (TRACE, BLOCK.dimmed()),
        Level::Debug => (DEBUG, BLOCK.dimmed()),
        Level::Info => (INFO, BLOCK.green()),
        Level::Warn => (WARN, BLOCK.yellow()),
        Level::Error => (ERROR, BLOCK.red()),
    };
    let level = binding.style(level_style);
    let separator = "|".style(separator_style);
    if cfg!(debug_assertions) {
        format!(
            "{} {} {}{}{}: {}",
            level,
            separator,
            record.file().unwrap_or("unknown").style(MODULE),
            ":".style(MODULE),
            record.line().unwrap_or(0).to_string().style(MODULE),
            message
        )
    } else {
        format!("{level} {separator} {message}")
    }
}
