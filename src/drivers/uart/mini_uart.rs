//! Mini UART driver for Raspberry Pi
//!
//! Provides blocking read/write and initialization routines for the Mini UART peripheral.
//!
//! # Example
//! ```rust
//! use crate::drivers::uart::mini_uart;
//!
//! fn main() {
//!     mini_uart::init();
//!     mini_uart::write_string("Hello from Mini UART!\r\n");
//!     if let Some(byte) = mini_uart::read_byte() {
//!         mini_uart::write_byte(byte);
//!     }
//! }
//! ```

use crate::hal::registers::uart::MINI_UART_REGS;
use crate::hal::registers::gpio::GPIO_REGS;
use crate::hal::registers::utils::*;
use crate::hal::registers::auxiliary::AUX_REGS;

pub fn init() {
    unsafe {
        let gpio_regs = &mut *GPIO_REGS;
        gpio_regs.gpfsel1 &= !(0b111 << 12); // Clear FSEL14 (TX)
        gpio_regs.gpfsel1 &= !(0b111 << 15); // Clear FSEL15 (RX)
        gpio_regs.gpfsel1 |= 0b010 << 12; // Set GPIO14 to ALT5 (Mini UART TX)
        gpio_regs.gpfsel1 |= 0b010 << 15; // Set GPIO15 to ALT5 (Mini UART RX)
        let aux_regs = &mut *AUX_REGS;
        set_bit(&mut aux_regs.aux_enables, 0); // Enable Mini UART peripheral
        let mini_uart_regs = &mut *MINI_UART_REGS;
        mini_uart_regs.aux_mu_baud_reg = 270; // Set baud rate to 115200 (assuming 250MHz clock)
        set_bit(&mut mini_uart_regs.aux_mu_lcr_reg, 0); // 8-bit mode
        set_bit(&mut mini_uart_regs.aux_mu_cntl_reg, 0); // Enable receiver
        set_bit(&mut mini_uart_regs.aux_mu_cntl_reg, 1); // Enable transmitter
        set_bit(&mut mini_uart_regs.aux_mu_iir_reg, 1); // Clear receive FIFO
        set_bit(&mut mini_uart_regs.aux_mu_iir_reg, 2); // Clear transmit FIFO
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        let mini_uart_regs = &mut *MINI_UART_REGS;
        while !is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 5) {} // Wait for TX FIFO to have space
        mini_uart_regs.aux_mu_io_reg = byte as u32; // Write byte to TX FIFO
    }
}

pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_byte(byte); // Send each byte
    }
}

pub fn read_byte() -> Option<u8> {
    unsafe {
        let mini_uart_regs = &*MINI_UART_REGS;
        if is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 0) { // Data ready in RX FIFO?
            Some((mini_uart_regs.aux_mu_io_reg & 0xFF) as u8) // Read byte
        } else {
            None // No data available
        }
    }
}

pub fn flush() {
    unsafe {
        let mini_uart_regs = &*MINI_UART_REGS;
        while !is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 6) {} // Wait for transmitter to be idle
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
        let mini_uart_regs = &*MINI_UART_REGS;
        is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 0) // RX FIFO has data?
    }
}
