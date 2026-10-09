//! Provides low-level access to hardware I/O ports, specifically for serial communication.

use core::arch::asm;

/// The I/O port address for the first serial port (COM1).
const COM1: u16 = 0x3F8;

/// Writes a byte to an I/O port.
///
/// # Safety
///
/// This function performs raw I/O port access. The caller must ensure:
///
/// - `port` is a valid I/O port for the target hardware.
/// - Writing `value` to `port` is safe for the current system state.
/// - The operation does not race with other code accessing the same port.
///
/// Incorrect use can corrupt hardware state, cause data loss, or hang the system.
#[inline]
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

/// Reads a byte from an I/O port.
///
/// # Safety
///
/// This function performs raw I/O port access. The caller must ensure:
///
/// - `port` is a valid I/O port for the target hardware.
/// - Reading from `port` is safe for the current system state.
/// - The operation does not race with other code accessing the same port.
///
/// Incorrect use can read garbage, cause data loss, or hang the system.
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port);
    }
    value
}

/// Checks if the transmit buffer of the serial port is empty.
///
/// # Safety
/// This function performs raw I/O port access. The caller must ensure:
/// - The serial port is properly initialized and configured.
/// - The operation does not race with other code accessing the same port.
///
/// Incorrect use can lead to data loss or undefined behavior.
#[inline]
unsafe fn is_transmit_empty() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

/// Initializes the serial port for communication.
pub fn init() {
    unsafe {
        outb(COM1 + 1, 0x00); // Disable interrupts
        outb(COM1 + 3, 0x80); // Enable DLAB
        outb(COM1, 0x03); // 38400 baud (divisor low)
        outb(COM1 + 1, 0x00); // (divisor high)
        outb(COM1 + 3, 0x03); // 8 bits, 1 stop, no parity
        outb(COM1 + 2, 0xC7); // Enable FIFO
        outb(COM1 + 4, 0x0B); // IRQs enabled
    }
}

/// Writes a byte to the serial port.
pub fn write_byte(byte: u8) {
    while !unsafe { is_transmit_empty() } {
        core::hint::spin_loop();
    }
    unsafe { outb(COM1, byte) }
}

/// Writes a string to the serial port.
pub fn write(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}

/// Writes a string followed by a newline to the serial port.
pub fn write_line(s: &str) {
    write(s);
    write("\r\n");
}
