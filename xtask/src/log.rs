//! # Log
//! Logging utilities for the xtask CLI.

use log::Level;

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
