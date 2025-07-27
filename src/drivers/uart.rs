//! UART driver module
//!
//! This module re-exports both the Mini UART and PL011 UART (uart0) drivers for the Raspberry Pi.
//!
//! # Usage
//! Use `mini_uart` for the Mini UART peripheral, or `uart0` for the full PL011 UART.
//!
//! ```rust
//! use crate::drivers::uart::mini_uart; // For Mini UART
//! // or
//! use crate::drivers::uart::uart0;     // For PL011 UART0
//! ```
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

pub mod mini_uart;
pub mod uart0;
