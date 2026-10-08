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
    let level = match record.level() {
        Level::Trace => binding.style(TRACE),
        Level::Debug => binding.style(DEBUG),
        Level::Info => binding.style(INFO),
        Level::Warn => binding.style(WARN),
        Level::Error => binding.style(ERROR),
    };
    if cfg!(debug_assertions) {
        format!(
            "{}{}{} {}{}{}: {}",
            "[".style(BLOCK),
            level,
            "]".style(BLOCK),
            record.file().unwrap_or("unknown").style(MODULE),
            ":".style(MODULE),
            record.line().unwrap_or(0).to_string().style(MODULE),
            message
        )
    } else {
        format!(
            "{}{}{} {}",
            "[".style(BLOCK),
            level,
            "]".style(BLOCK),
            message
        )
    }
}
