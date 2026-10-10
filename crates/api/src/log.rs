//! A simple logger implementation that writes to the serial port.

use core::fmt::Write;

use log::{Level, Log, Metadata};

struct GlowLogger;

static LOGGER: GlowLogger = GlowLogger;

impl Log for GlowLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if cfg!(debug_assertions) {
            metadata.level() <= Level::Trace
        } else {
            metadata.level() <= Level::Info
        }
    }
    fn flush(&self) {}
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let target = record.target();
            let args = record.args();
            writeln!(
                super::serial::SerialWriter,
                "[{}] {}: {}",
                level,
                target,
                args
            )
            .unwrap();
        }
    }
}

/// Initializes the logger and sets the maximum log level based on the build configuration.
pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }
}
