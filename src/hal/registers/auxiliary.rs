//! Auxiliary Peripherals Register definitions.
//!
//! This module provides base address and common definitions for Auxiliary peripheral registers
//! on the Raspberry Pi, including those for the Mini UART, SPI1, and SPI2.
//!
//! The addresses and register layouts are based on the BCM2835/BCM2837 ARM Peripherals datasheets.

/// Base address for Auxiliary Peripherals (Mini UART, SPI1, SPI2).
pub const AUX_REGS_BASE: usize = 0xFE215000; // On RPi 4, this would be 0xFE215000.

/// Pointer to the Auxiliary Peripherals registers.
pub const AUX_REGS: *mut AuxiliaryRegisters = AUX_REGS_BASE as *mut AuxiliaryRegisters;

/// Represents the common Auxiliary Peripherals registers block.
/// This includes the shared control registers but not the specific peripheral registers.
#[repr(C)]
pub struct AuxiliaryRegisters {
    /// Auxiliary Interrupt Status. Shows status of Mini UART (bit 0), SPI1 (bit 1), SPI2 (bit 2) interrupts.
    pub aux_irq: u32, // Offset 0x00
    /// Auxiliary Enables. Bit 0 enables Mini UART. Bit 1 enables SPI1. Bit 2 enables SPI2.
    pub aux_enables: u32, // Offset 0x04
    // Note: Individual peripheral registers (Mini UART, SPI1, SPI2) are defined in their respective modules
}
