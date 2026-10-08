//! # Log
//! Logging utilities for the xtask CLI.

use log::{Level, LevelFilter, Log, Record};
use owo_colors::OwoColorize;

use styles::{BLOCK, DEBUG, ERROR, INFO, MODULE, TRACE, WARN};

mod styles;

/// Simple logger that prints messages to stdout or stderr.
struct XTaskLogger;

static LOGGER: XTaskLogger = XTaskLogger;

/// Installs the xtask logger and selects a verbosity based on the build profile.
pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    });
    Ok(())
}

impl Log for XTaskLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level().to_level_filter() <= log::max_level()
    }

    fn flush(&self) {}

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        write_message(&format_message(record), record.level());
    }
}

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

/// Formats a log record with the appropriate level and location styles.
fn format_message(record: &Record) -> String {
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
            record.args()
        )
    } else {
        format!("{level} {separator} {}", record.args())
    }
}
