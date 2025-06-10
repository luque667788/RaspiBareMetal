//! GPIO Register definitions.
//!
//! This module provides structures for accessing GPIO peripheral registers
//! on the Raspberry Pi.
//!
//! The addresses and register layouts are based on the BCM2835/BCM2837 ARM Peripherals datasheets.

/// Base address for GPIO registers.
pub const GPIO_REGS_BASE: usize = 0x7E200000; // For RPi models with VideoCore IV (e.g., RPi 2, 3, Zero)
                                             // On RPi 4, this would be 0xFE200000.

/// Pointer to the GPIO registers.
pub const GPIO_REGS: *mut GpioRegisters = GPIO_REGS_BASE as *mut GpioRegisters;

/// Represents the GPIO registers.
///
/// This structure provides access to the GPIO Function Select registers (GPFSELn),
/// Pin Output Set registers (GPSETn), and Pin Output Clear registers (GPCLRn).
#[repr(C)]
pub struct GpioRegisters {
    /// GPIO Function Select 0. Controls GPIO pins 0-9. Each pin uses 3 bits.
    pub gpfsel0: u32, // Offset 0x00
    /// GPIO Function Select 1. Controls GPIO pins 10-19. Each pin uses 3 bits.
    /// Relevant for Mini UART: FSEL14 (bits 14-12) for GPIO14 (TXD1), FSEL15 (bits 17-15) for GPIO15 (RXD1).
    /// ALT5 function (010) should be set for Mini UART.
    pub gpfsel1: u32, // Offset 0x04
    /// GPIO Function Select 2. Controls GPIO pins 20-29.
    pub gpfsel2: u32, // Offset 0x08
    /// GPIO Function Select 3. Controls GPIO pins 30-39.
    pub gpfsel3: u32, // Offset 0x0C
    /// GPIO Function Select 4. Controls GPIO pins 40-49.
    pub gpfsel4: u32, // Offset 0x10
    /// GPIO Function Select 5. Controls GPIO pins 50-53.
    pub gpfsel5: u32, // Offset 0x14
    _reserved0: u32, // Offset 0x18
    /// GPIO Pin Output Set 0. Controls GPIO pins 0-31. Writing 1 sets the pin if configured as output.
    pub gpset0: u32, // Offset 0x1C
    /// GPIO Pin Output Set 1. Controls GPIO pins 32-53. Writing 1 sets the pin if configured as output.
    pub gpset1: u32, // Offset 0x20
    _reserved1: u32, // Offset 0x24
    /// GPIO Pin Output Clear 0. Controls GPIO pins 0-31. Writing 1 clears the pin if configured as output.
    pub gpclr0: u32, // Offset 0x28
    /// GPIO Pin Output Clear 1. Controls GPIO pins 32-53. Writing 1 clears the pin if configured as output.
    pub gpclr1: u32, // Offset 0x2C
    // _reserved2 covers offsets 0x30 to 0x3C
    _reserved2: [u32;3], // Offset 0x30, 0x34, 0x38 (Corrected: gplev0/1 are at 0x34/0x38)
    // The following were based on a standard layout, ensure they match BCM2835 if used.
    // For now, keeping it minimal to what was in the original uart.rs comments.
    // pub gplev0: u32, // Offset 0x34 - GPIO Pin Level 0
    // pub gplev1: u32, // Offset 0x38 - GPIO Pin Level 1
}
