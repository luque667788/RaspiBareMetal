//! UART Driver for Raspberry Pi (Mini UART and PL011 UART)
//!
//! This module provides a simple interface for serial communication using either the Mini UART or the full PL011 UART (uart0) on the Raspberry Pi.
//! It allows you to easily switch between the two UARTs by changing a single constant.
//!
//! # Features
//! - Supports both Mini UART and PL011 UART (uart0)
//! - Simple API for initialization, sending, and receiving data
//! - Blocking read/write operations
//! - Line input with basic editing (backspace)
//!
//! # Example
//! ```rust
//! use crate::drivers::uart;
//!
//! fn main() {
//!     // Initialize the UART (select which one in the driver)
//!     uart::init();
//!
//!     // Write a string
//!     uart::write_string("Hello, UART!\r\n");
//!
//!     // Read a byte (if available)
//!     if let Some(byte) = uart::read_byte() {
//!         uart::write_byte(byte); // Echo it back
//!     }
//!
//!     // Read a line into a buffer
//!     let mut buf = [0u8; 128];
//!     if let Some(len) = uart::read_line(&mut buf) {
//!         // Do something with the input
//!     }
//! }
//! ```
//!
//! # Selecting UART
//! By default, the driver uses the PL011 UART (uart0). To use the Mini UART instead, change the `UART_TYPE` constant at the top of this file.
//!
//! # Notes
//! - Make sure your board's GPIO pins are connected to the correct UART and configured for the right alternate function.
//! - Baud rate and clock settings may need adjustment for your hardware.

use crate::hal::registers::uart::{MINI_UART_REGS, PL011_UART_REGS};
use crate::hal::registers::gpio::GPIO_REGS;
use crate::hal::registers::utils::*;
use crate::hal::registers::auxiliary::AUX_REGS; 

/// Select which UART to use
pub enum UartType {
    Mini,
    Pl011,
}

/// Set this to select which UART implementation to use
const UART_TYPE: UartType = UartType::Pl011; // Change to UartType::Mini for Mini UART

pub fn init() {
    match UART_TYPE {
        UartType::Mini => unsafe {
            // setup the GPIO 14 for TX and GPIO 15 for RX (make them operate in function 5 mode)
            // GPFSELn registers control the function of GPIO pins
            let gpio_regs = &mut *GPIO_REGS;
            
            // Clear FSEL14 (bits 14-12) and FSEL15 (bits 17-15) in GPFSEL1
            gpio_regs.gpfsel1 &= !(0b111 << 12); // Clear FSEL14
            gpio_regs.gpfsel1 &= !(0b111 << 15); // Clear FSEL15
            
            // write the corresponding 3-bit value to its FSEL field. For ALT5, the value is 010
            // The base address for GPIO registers is 0x7e200000. The GPFSEL1 register is at offset 0x04 from the base
            // FSEL14 and FSEL15 fields within the GPFSEL1 register at address 0x7e200000 + 0x04
            gpio_regs.gpfsel1 |= 0b010 << 12; // Set GPIO14 to ALT5 (TX)
            gpio_regs.gpfsel1 |= 0b010 << 15; // Set GPIO15 to ALT5 (RX)

            //The Mini UART is enabled via the AUX_ENABLES register setting Bit 0
            // The Auxiliary register base address is 0x7e215000. The AUX_ENABLES register is at offset 0x04 from this base
            let aux_regs = &mut *AUX_REGS;
            set_bit(&mut aux_regs.aux_enables, 0); // Enable Mini UART

            //Baud Rate: The baud rate is derived from the system clock. You set the baud rate using the AUX_MU_BAUD_REG. 
            //This is a 16-bit register. It can be accessed directly via offset 0x68 from the Auxiliary base address (0x7e215000)
            // Baudrate = System_Clock_Freq / (8 * (Baudrate_Reg + 1)) so if we want 115200 baud, we can set the register to 270 (assuming a 250MHz system clock)
            let mini_uart_regs = &mut *MINI_UART_REGS;
            mini_uart_regs.aux_mu_baud_reg = 270; // Set baud rate to 115200 (assuming 250MHz system clock)

            // set the data size at AUX_MU_LCR_REG bit 0 (If set the UART works in 8-bit mode else it works in 7-bit mode) 
            // offset 0x4c from the Auxiliary base (0x7e215000) = 0x7e21504c
            set_bit(&mut mini_uart_regs.aux_mu_lcr_reg, 0); // Set 8-bit mode

            // the RTS and CTS will be disabled by default, so we can ignore them for now

            // Enable transmitter and receiver by setting the corresponding bits in the AUX_MU_CNTL_REG register
            // The AUX_MU_CNTL_REG is at offset 0x60 from the Auxiliary base address (0x7e215000) 
            // Bit 0 enables the transmitter, and Bit 1 enables the receiver.
            // no need to set them becasue the default is already enabled
            // TODO! provide a proper function implementation to set up all this stuff later
            set_bit(&mut mini_uart_regs.aux_mu_cntl_reg, 0); // Enable receiver
            set_bit(&mut mini_uart_regs.aux_mu_cntl_reg, 1); // Enable transmitter

            // Good practice to clear the FIFOs before using them
            // The AUX_MU_IIR_REG is at offset 0x48 from the Auxiliary base address (0x7e215000)
            // writing 1 to bit 1 clears the receive FIFO, and writing 1 to bit 2 clears the transmit FIFO
            set_bit(&mut mini_uart_regs.aux_mu_iir_reg, 1); // Clear receive FIFO
            set_bit(&mut mini_uart_regs.aux_mu_iir_reg, 2); // Clear transmit FIFO
        },
        UartType::Pl011 => unsafe {
            // Set GPIO14/15 to ALT0 for UART0
            let gpio_regs = &mut *GPIO_REGS;
            gpio_regs.gpfsel1 &= !(0b111 << 12);
            gpio_regs.gpfsel1 &= !(0b111 << 15);
            gpio_regs.gpfsel1 |= 0b100 << 12; // ALT0 for UART0 TX
            gpio_regs.gpfsel1 |= 0b100 << 15; // ALT0 for UART0 RX
            // Disable pull-up/down for pins 14/15 (if needed)
            // ...optional: implement if required for your board...
            let uart = &mut *PL011_UART_REGS;
            // Disable UART0 before config
            uart.cr = 0;
            // Clear pending interrupts
            uart.icr = 0x7FF;
            // Set integer & fractional baud rate (for 115200 baud @ 48MHz UART clock)
            uart.ibrd = 26; // 48_000_000 / (16 * 115200) = 26.04
            uart.fbrd = 3;  // Fractional part: .04 * 64 + 0.5 = 3
            // 8N1, FIFO enabled
            uart.lcrh = (1 << 4) | (1 << 5) | (1 << 6);
            // Enable UART, TX, RX
            uart.cr = (1 << 0) | (1 << 8) | (1 << 9);
        },
    }
}

pub fn write_byte(byte: u8) {
    match UART_TYPE {
        UartType::Mini => unsafe {
            let mini_uart_regs = &mut *MINI_UART_REGS;
            // Wait until the transmit FIFO is not full
            // The Transmitter empty (TXE) flag (AUX_MU_LSR_REG bit 5) (offset is 0x54) indicates if the transmit FIFO can accept at least one byte
            // The Space available flag (AUX_MU_STAT_REG bit 1) (offset is 0x64) )also indicates if the TX FIFO can accept at least one symbol
            // you can check any of these flags to determine if the FIFO is not full meaning we can write to it
            while !is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 5) {
                // Wait for transmitter to be ready
            }

            // after that writing to the AUX_MU_IO_REG (offset is 0x40) will write the byte to the transmit FIFO
            mini_uart_regs.aux_mu_io_reg = byte as u32;
        },
        UartType::Pl011 => unsafe {
            let uart = &mut *PL011_UART_REGS;
            // Wait until TX FIFO not full (FR bit 5 == 0)
            while (uart.fr & (1 << 5)) != 0 {}
            uart.dr = byte as u32;
        },
    }
}

pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}

pub fn read_byte() -> Option<u8> {
    match UART_TYPE {
        UartType::Mini => unsafe {
            let mini_uart_regs = &*MINI_UART_REGS;
            
            // Check if data is ready (AUX_MU_LSR_REG bit 0)
            if is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 0) {
                // Data is available, read from IO register
                Some((mini_uart_regs.aux_mu_io_reg & 0xFF) as u8)
            } else {
                None
            }
        },
        UartType::Pl011 => unsafe {
            let uart = &*PL011_UART_REGS;
            // Check if RX FIFO is empty (FR bit 4 == 1)
            if (uart.fr & (1 << 4)) == 0 {
                Some((uart.dr & 0xFF) as u8)
            } else {
                None
            }
        },
    }
}

pub fn flush() {
    match UART_TYPE {
        UartType::Mini => unsafe {
            let mini_uart_regs = &*MINI_UART_REGS;
            //after that we can poll the Transmitter idle flag (AUX_MU_LSR_REG bit 6) (offset is 0x54)
            // or the Transmitter done flag (AUX_MU_STAT_REG bit 9) (offset is 0x64).
            //Transmitter done (STAT bit 9) is set when the transmitter is idle and the transmit FIFO is empty
            while !is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 6) {
                // Wait for transmitter to be idle
            }
        },
        UartType::Pl011 => unsafe {
            let uart = &*PL011_UART_REGS;
            // Wait until TX FIFO is empty (FR bit 7 == 1)
            while (uart.fr & (1 << 7)) == 0 {}
        },
    }
}

pub fn read_line(buffer: &mut [u8]) -> Option<usize> {
    // Read a line of input from UART (until \r or \n)
    let mut pos = 0;
    while pos < buffer.len() - 1 {
        if let Some(byte) = read_byte() {
            match byte {
                b'\r' | b'\n' => {
                    buffer[pos] = 0; // Null terminate
                    return Some(pos);
                }
                b'\x08' | b'\x7f' => { // Backspace or DEL
                    if pos > 0 {
                        pos -= 1;
                        write_string("\x08 \x08"); // Backspace, space, backspace
                    }
                }
                _ => {
                    buffer[pos] = byte;
                    write_byte(byte); // Echo the character
                    pos += 1;
                }
            }
        }
    }
    buffer[pos] = 0; // Null terminate
    Some(pos)
}

pub fn is_data_ready() -> bool {
    match UART_TYPE {
        UartType::Mini => unsafe {
            let mini_uart_regs = &*MINI_UART_REGS;
            is_bit_set(mini_uart_regs.aux_mu_lsr_reg, 0)
        },
        UartType::Pl011 => unsafe {
            let uart = &*PL011_UART_REGS;
            // RX FIFO not empty (FR bit 4 == 0)
            (uart.fr & (1 << 4)) == 0
        },
    }
}
