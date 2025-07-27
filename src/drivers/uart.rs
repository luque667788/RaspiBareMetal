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
//!     uart::uart0::init();
//!
//!     // Write a string
//!     uart::uart0::write_string("Hello, UART!\r\n");
//!
//!     // Read a byte (if available)
//!     if let Some(byte) = uart::uart0::read_byte() {
//!         uart::uart0::write_byte(byte); // Echo it back
//!     }
//!
//!     // Read a line into a buffer
//!     let mut buf = [0u8; 128];
//!     if let Some(len) = uart::uart0::read_line(&mut buf) {
//!         // Do something with the input
//!     }
//! }
//! ```
//!


pub mod mini_uart;
pub mod uart0;
