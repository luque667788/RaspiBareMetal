//! UART Register definitions.
//!
//! This module provides structures for accessing UART-specific registers.
//! Currently focuses on Mini UART registers, which are part of the Auxiliary Peripherals block.
//! The addresses and register layouts are based on the BCM2835/BCM2837 ARM Peripherals datasheets.

use super::auxiliary::AUX_REGS_BASE;

/// Pointer to the Mini UART registers (part of Auxiliary Peripherals).
/// This points directly to the Mini UART section within the auxiliary peripheral block.
pub const MINI_UART_REGS: *mut MiniUartRegisters = (AUX_REGS_BASE + 0x40) as *mut MiniUartRegisters;

/// Represents the Mini UART registers portion of the Auxiliary Peripherals.
/// This struct provides direct access to Mini UART functionality.
#[repr(C)]
pub struct MiniUartRegisters {
    /// Mini UART I/O Data. 8-bit register. Reading gets from RX FIFO, writing puts into TX FIFO.
    /// Only the least significant 8 bits are used.
    pub aux_mu_io_reg: u32, // Offset 0x40 from AUX_REGS_BASE
    /// Mini UART Interrupt Enable. Controls which UART events trigger an interrupt.
    /// Bit 0: Enable transmit interrupt (triggered when TX FIFO is empty).
    /// Bit 1: Enable receive interrupt (triggered when RX FIFO holds data).
    pub aux_mu_ier_reg: u32, // Offset 0x44 from AUX_REGS_BASE
    /// Mini UART Interrupt Identify / FIFO Clear.
    /// Read: Bits 2:1 indicate interrupt type (10=TX empty, 01=RX ready). Bit 0 is 0 if interrupt pending.
    /// Write: Bit 1 clears receive FIFO. Bit 2 clears transmit FIFO.
    /// Bits 7:6 show FIFO enabled status (11 = enabled).
    pub aux_mu_iir_reg: u32, // Offset 0x48 from AUX_REGS_BASE
    /// Mini UART Line Control. Controls data size, stop bits.
    /// Bit 0: Data size (0 for 7-bit, 1 for 8-bit).
    /// Bits 1-7 are reserved.
    pub aux_mu_lcr_reg: u32, // Offset 0x4C from AUX_REGS_BASE
    /// Mini UART Modem Control. Controls RTS line.
    /// Bit 1: RTS level (0 for high, 1 for low).
    pub aux_mu_mcr_reg: u32, // Offset 0x50 from AUX_REGS_BASE
    /// Mini UART Line Status. Shows status of transmitter and receiver.
    /// Bit 0: Data ready (RX FIFO has data).
    /// Bit 1: Receiver overrun (new byte received, RX FIFO full).
    /// Bit 5: Transmitter empty (TX FIFO can accept at least one byte).
    /// Bit 6: Transmitter idle (TX FIFO empty and transmitter serial shifter finished).
    pub aux_mu_lsr_reg: u32, // Offset 0x54 from AUX_REGS_BASE (Read-Only)
    /// Mini UART Modem Status. Shows status of CTS line. (Not typically used in simple setups)
    /// Bit 5: CTS line status.
    pub aux_mu_msr_reg: u32, // Offset 0x58 from AUX_REGS_BASE (Read-Only)
    /// Mini UART Scratch. A single byte scratch register for temporary storage.
    pub aux_mu_scratch_reg: u32, // Offset 0x5C from AUX_REGS_BASE
    /// Mini UART Extra Control. Enables transmitter/receiver.
    /// Bit 0: Receiver enable.
    /// Bit 1: Transmitter enable.
    /// Other bits control RTS/CTS auto flow, etc. (defaults are usually fine for basic UART).
    pub aux_mu_cntl_reg: u32, // Offset 0x60 from AUX_REGS_BASE
    /// Mini UART Extra Status. Shows FIFO levels and transmitter status.
    /// Bits 3:0: RX FIFO fill level (number of symbols in RX FIFO).
    /// Bit 1 (Symbol available): TX FIFO can accept at least one symbol. (Alternative to LSR bit 5)
    /// Bit 9 (Transmitter done): Transmitter is idle and TX FIFO is empty. (Alternative to LSR bit 6)
    /// Bits 31:24: Baudrate counter value (read-only).
    pub aux_mu_stat_reg: u32, // Offset 0x64 from AUX_REGS_BASE (Read-Only for most useful bits)
    /// Mini UART Baudrate. 16-bit register for setting baudrate.
    /// Baudrate = system_clock_freq / (8 * (baud_reg + 1)).
    /// Only the lower 16 bits (15:0) are used.
    pub aux_mu_baud_reg: u32, // Offset 0x68 from AUX_REGS_BASE
}

/// Base address for the PL011 UART registers (UART0).
/// This is the standard UART on the BCM2835/BCM2837 SoCs.
pub const PL011_UART_BASE: usize = 0x3F201000; // For RPi 2/3. Use 0x20201000 for RPi 1/Zero.

/// Pointer to the PL011 UART registers.
pub const PL011_UART_REGS: *mut Pl011UartRegisters = PL011_UART_BASE as *mut Pl011UartRegisters;

/// Represents the PL011 UART registers.
/// This struct provides direct access to the full UART (UART0) functionality.
#[repr(C)]
pub struct Pl011UartRegisters {
    /// Data Register. Read: RX FIFO, Write: TX FIFO.
    pub dr: u32,                // 0x00
    /// Receive Status / Error Clear Register.
    pub rsrecr: u32,            // 0x04
    _reserved0: [u32; 4],       // 0x08-0x14 (unused)
    /// Flag Register. Status bits for TX/RX FIFO, busy, etc.
    pub fr: u32,                // 0x18
    _reserved1: u32,            // 0x1C
    /// IrDA Low-Power Counter Register (not typically used).
    pub ilpr: u32,              // 0x20
    /// Integer Baud Rate Divisor.
    pub ibrd: u32,              // 0x24
    /// Fractional Baud Rate Divisor.
    pub fbrd: u32,              // 0x28
    /// Line Control Register. Data size, parity, stop bits, FIFO enable.
    pub lcrh: u32,              // 0x2C
    /// Control Register. Enables UART, TX, RX, etc.
    pub cr: u32,                // 0x30
    /// Interrupt FIFO Level Select Register.
    pub ifls: u32,              // 0x34
    /// Interrupt Mask Set/Clear Register.
    pub imsc: u32,              // 0x38
    /// Raw Interrupt Status Register.
    pub ris: u32,               // 0x3C
    /// Masked Interrupt Status Register.
    pub mis: u32,               // 0x40
    /// Interrupt Clear Register.
    pub icr: u32,               // 0x44
    /// DMA Control Register.
    pub dmacr: u32,             // 0x48
    _reserved2: [u32; 13],      // 0x4C-0x7C (unused)
    /// Test Control Register (not typically used).
    pub itcr: u32,              // 0x80
    /// Integration Test Input Register (not typically used).
    pub itip: u32,              // 0x84
    /// Integration Test Output Register (not typically used).
    pub itop: u32,              // 0x88
    /// Test Data Register (not typically used).
    pub tdr: u32,               // 0x8C
}