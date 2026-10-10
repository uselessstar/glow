//! Provides high-level and safe access to hardware I/O ports, specifically for serial communication.

use core::fmt::Write;
use raw_api::serial::write;

pub use raw_api::serial::init;

/// Represents a writer for the serial port, allowing safe and structured output to the serial interface.
pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(s);
        Ok(())
    }
}
