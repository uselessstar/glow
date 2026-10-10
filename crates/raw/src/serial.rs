//! Raw UART access through x86 I/O ports.

use core::arch::asm;

const COM1: u16 = 0x3F8;
const LINE_STATUS: u16 = COM1 + 5;
const TRANSMIT_EMPTY: u8 = 1 << 5;

#[inline]
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port);
    }
    value
}

/// Initializes the first UART (COM1) for 38400 baud, 8-N-1.
///
/// This requires permission to access x86 I/O ports and should be called before
/// using the UART.
pub fn init() {
    unsafe {
        outb(COM1 + 1, 0x00); // Disable UART interrupts
        outb(COM1 + 3, 0x80); // Enable divisor-latch access
        outb(COM1, 0x03); // Divisor low: 38400 baud
        outb(COM1 + 1, 0x00); // Divisor high
        outb(COM1 + 3, 0x03); // 8 data bits, 1 stop bit, no parity
        outb(COM1 + 2, 0xC7); // Enable and clear FIFO
        outb(COM1 + 4, 0x0B); // Assert DTR, RTS, and OUT2
    }
}

/// Returns whether the UART can accept another byte.
///
/// This reads the COM1 line-status register directly.
#[inline]
pub fn transmit_empty() -> bool {
    unsafe { inb(LINE_STATUS) & TRANSMIT_EMPTY != 0 }
}

/// Writes a byte directly to COM1 without waiting for the transmitter.
///
/// # Safety
///
/// The caller must ensure that the transmitter is ready (see [`transmit_empty`])
/// and that x86 I/O port access is permitted in the current execution context.
#[inline]
pub unsafe fn write_byte(byte: u8) {
    unsafe {
        outb(COM1, byte);
    }
}
