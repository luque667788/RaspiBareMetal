//! PL011 UART (UART0) driver for Raspberry Pi
//!
//! Provides blocking read/write and initialization routines for the full UART0 peripheral.
//!
//! # Example
//! ```rust
//! use crate::drivers::uart::uart0;
//!
//! fn main() {
//!     uart0::init();
//!     uart0::write_string("Hello from UART0!\r\n");
//!     if let Some(byte) = uart0::read_byte() {
//!         uart0::write_byte(byte);
//!     }
//! }
//! ```

use crate::hal::registers::uart::PL011_UART_REGS;
use crate::hal::registers::gpio::GPIO_REGS;

pub fn init() {
    unsafe {
        let gpio_regs = &mut *GPIO_REGS;
        gpio_regs.gpfsel1 &= !(0b111 << 12); // Clear FSEL14 (TX)
        gpio_regs.gpfsel1 &= !(0b111 << 15); // Clear FSEL15 (RX)
        gpio_regs.gpfsel1 |= 0b100 << 12; // Set GPIO14 to ALT0 (UART0 TX)
        gpio_regs.gpfsel1 |= 0b100 << 15; // Set GPIO15 to ALT0 (UART0 RX)
        // Optionally: disable pull-up/down for pins 14/15 here if needed
        let uart = &mut *PL011_UART_REGS;
        uart.cr = 0; // Disable UART0 before config
        uart.icr = 0x7FF; // Clear all pending interrupts
        uart.ibrd = 26; // Set integer baud rate divisor (for 115200 baud @ 48MHz)
        uart.fbrd = 3;  // Set fractional baud rate divisor
        uart.lcrh = (1 << 4) | (1 << 5) | (1 << 6); // 8N1, enable FIFOs
        uart.cr = (1 << 0) | (1 << 8) | (1 << 9); // Enable UART, TX, RX
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        let uart = &mut *PL011_UART_REGS;
        while (uart.fr & (1 << 5)) != 0 {} // Wait for TX FIFO to have space
        uart.dr = byte as u32; // Write byte to TX FIFO
    }
}

pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_byte(byte); // Send each byte
    }
}

pub fn read_byte() -> Option<u8> {
    unsafe {
        let uart = &*PL011_UART_REGS;
        if (uart.fr & (1 << 4)) == 0 { // RX FIFO not empty?
            Some((uart.dr & 0xFF) as u8) // Read byte
        } else {
            None // No data available
        }
    }
}

pub fn flush() {
    unsafe {
        let uart = &*PL011_UART_REGS;
        while (uart.fr & (1 << 7)) == 0 {} // Wait for TX FIFO to be empty
    }
}

pub fn read_line(buffer: &mut [u8]) -> Option<usize> {
    let mut pos = 0;
    while pos < buffer.len() - 1 {
        if let Some(byte) = read_byte() {
            match byte {
                b'\r' | b'\n' => { // End of line
                    buffer[pos] = 0; // Null terminate
                    return Some(pos);
                }
                b'\x08' | b'\x7f' => { // Backspace or DEL
                    if pos > 0 {
                        pos -= 1;
                        write_string("\x08 \x08"); // Erase last char on terminal
                    }
                }
                _ => {
                    buffer[pos] = byte; // Store byte
                    write_byte(byte);   // Echo back
                    pos += 1;
                }
            }
        }
    }
    buffer[pos] = 0; // Null terminate
    Some(pos)
}

pub fn is_data_ready() -> bool {
    unsafe {
        let uart = &*PL011_UART_REGS;
        (uart.fr & (1 << 4)) == 0 // RX FIFO has data?
    }
}
