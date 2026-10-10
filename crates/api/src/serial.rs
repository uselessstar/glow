//! Safe, synchronized serial output built on raw UART access.

use core::arch::asm;
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

pub use raw_api::serial::init;

const POLL_LIMIT: usize = 10_000_000;
static SERIAL_LOCK: AtomicBool = AtomicBool::new(false);

/// Errors that can occur while writing to the serial port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialError {
    LockTimeout,
    TransmitTimeout,
    FormattingError,
}

struct SerialGuard {
    interrupts_enabled: bool,
}

impl SerialGuard {
    fn acquire() -> Result<Self, SerialError> {
        let flags: u64;
        unsafe {
            asm!("pushfq", "pop {}", "cli", out(reg) flags);
        }

        for _ in 0..POLL_LIMIT {
            if SERIAL_LOCK
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return Ok(Self {
                    interrupts_enabled: flags & (1 << 9) != 0,
                });
            }
            core::hint::spin_loop();
        }

        if flags & (1 << 9) != 0 {
            unsafe {
                asm!("sti", options(nomem, nostack));
            }
        }
        Err(SerialError::LockTimeout)
    }
}

impl Drop for SerialGuard {
    fn drop(&mut self) {
        SERIAL_LOCK.store(false, Ordering::Release);
        if self.interrupts_enabled {
            unsafe {
                asm!("sti", options(nomem, nostack));
            }
        }
    }
}

fn write_byte_locked(byte: u8) -> Result<(), SerialError> {
    for _ in 0..POLL_LIMIT {
        if raw_api::serial::transmit_empty() {
            unsafe {
                raw_api::serial::write_byte(byte);
            }
            return Ok(());
        }
        core::hint::spin_loop();
    }

    Err(SerialError::TransmitTimeout)
}

struct FormattedWriter {
    error: Option<SerialError>,
    previous_was_carriage_return: bool,
}

impl Write for FormattedWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n'
                && !self.previous_was_carriage_return
                && let Err(error) = write_byte_locked(b'\r')
            {
                self.error = Some(error);
                return Err(fmt::Error);
            }
            if let Err(error) = write_byte_locked(byte) {
                self.error = Some(error);
                return Err(fmt::Error);
            }
            self.previous_was_carriage_return = byte == b'\r';
        }
        Ok(())
    }
}

/// Writes formatted text as one synchronized operation, converting newlines to CRLF.
pub fn write_fmt(args: fmt::Arguments<'_>) -> Result<(), SerialError> {
    let _guard = SerialGuard::acquire()?;
    let mut writer = FormattedWriter {
        error: None,
        previous_was_carriage_return: false,
    };
    let result = fmt::write(&mut writer, args);

    if let Some(error) = writer.error {
        return Err(error);
    }
    result.map_err(|_| SerialError::FormattingError)
}

/// Writes text as one synchronized operation, converting newlines to CRLF.
pub fn write(s: &str) -> Result<(), SerialError> {
    write_fmt(format_args!("{s}"))
}

/// Writes text followed by a newline as one synchronized operation.
pub fn write_line(s: &str) -> Result<(), SerialError> {
    write_fmt(format_args!("{s}\n"))
}

/// A formatter that writes strings to the serial port with CRLF newlines.
///
/// Each `write_str` call is synchronized; use [`write_fmt`] to keep a complete
/// formatted message from interleaving with other writers.
pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(s).map_err(|_| fmt::Error)
    }
}
