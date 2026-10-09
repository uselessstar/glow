//! Provides low-level access to hardware I/O ports, specifically for serial communication.

use core::arch::asm;

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
