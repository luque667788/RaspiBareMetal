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
pub const PL011_UART_BASE: usize = 0xFE201000; // For RPi 2/3. Use 0x20201000 for RPi 1/Zero.

/// Pointer to the PL011 UART registers.
pub const PL011_UART_REGS: *mut Pl011UartRegisters = PL011_UART_BASE as *mut Pl011UartRegisters;

/// Represents the PL011 UART registers.
/// This struct provides direct access to the full UART (UART0) functionality.
#[repr(C)]
pub struct Pl011UartRegisters {
    /// Data Register (DR) - 0x00
    /// Read: RX FIFO, Write: TX FIFO.
    /// Bits:
    ///   7:0   - Data (read/write)
    ///   8     - Framing error (read)
    ///   9     - Parity error (read)
    ///   10    - Break error (read)
    ///   11    - Overrun error (read)
    ///   31:12 - Reserved
    pub dr: u32,                // 0x00
    /// Receive Status / Error Clear Register (RSRECR) - 0x04
    /// Bits:
    ///   0     - Framing error
    ///   1     - Parity error
    ///   2     - Break error
    ///   3     - Overrun error
    ///   31:4  - Reserved
    /// Write any value to clear errors.
    pub rsrecr: u32,            // 0x04
    _reserved0: [u32; 4],       // 0x08-0x14 (unused)
    /// Flag Register (FR) - 0x18
    /// Bits:
    ///   0     - Clear to send (CTS)
    ///   1     - Data set ready (DSR)
    ///   2     - Data carrier detect (DCD)
    ///   3     - Busy (1 = UART is transmitting data)
    ///   4     - RX FIFO empty (1 = empty)
    ///   5     - TX FIFO full (1 = full)
    ///   6     - RX FIFO full (1 = full)
    ///   7     - TX FIFO empty (1 = empty)
    ///   8     - Ring indicator (RI)
    ///   31:9  - Reserved
    pub fr: u32,                // 0x18
    _reserved1: u32,            // 0x1C
    /// IrDA Low-Power Counter Register (ILPR) - 0x20
    /// Not typically used.
    pub ilpr: u32,              // 0x20
    /// Integer Baud Rate Divisor (IBRD) - 0x24
    /// Bits 15:0: Integer part of baud rate divisor.
    /// Baud rate = UARTCLK / (16 * (IBRD + FBRD/64))
    pub ibrd: u32,              // 0x24
    /// Fractional Baud Rate Divisor (FBRD) - 0x28
    /// Bits 5:0: Fractional part of baud rate divisor.
    pub fbrd: u32,              // 0x28
    /// Line Control Register (LCRH) - 0x2C
    /// Bits:
    ///   0     - Send break (BRK)
    ///   1     - Parity enable (PEN)
    ///   2     - Even parity select (EPS)
    ///   3     - Two stop bits select (STP2)
    ///   4     - Enable FIFOs (FEN)
    ///   5:6   - Word length (WLEN) (00=5 bits, 01=6 bits, 10=7 bits, 11=8 bits)
    ///   7     - Stick parity select (SPS)
    ///   31:8  - Reserved
    pub lcrh: u32,              // 0x2C
    /// Control Register (CR) - 0x30
    /// Bits:
    ///   0     - UART enable (UARTEN)
    ///   1     - SIR enable (SIREN)
    ///   2     - SIR low-power mode (SIRLP)
    ///   8     - Transmit enable (TXE)
    ///   9     - Receive enable (RXE)
    ///   14    - Request to send (RTS)
    ///   15    - RTS hardware flow control enable (RTSEN)
    ///   16    - CTS hardware flow control enable (CTSEN)
    ///   31:17 - Reserved
    pub cr: u32,                // 0x30
    /// Interrupt FIFO Level Select Register (IFLS) - 0x34
    /// Not typically used in basic polling drivers.
    pub ifls: u32,              // 0x34
    /// Interrupt Mask Set/Clear Register (IMSC) - 0x38
    /// Not typically used in basic polling drivers.
    pub imsc: u32,              // 0x38
    /// Raw Interrupt Status Register (RIS) - 0x3C
    pub ris: u32,               // 0x3C
    /// Masked Interrupt Status Register (MIS) - 0x40
    pub mis: u32,               // 0x40
    /// Interrupt Clear Register (ICR) - 0x44
    /// Write 1 to clear corresponding interrupt.
    pub icr: u32,               // 0x44
    /// DMA Control Register (DMACR) - 0x48
    pub dmacr: u32,             // 0x48
    _reserved2: [u32; 13],      // 0x4C-0x7C (unused)
    /// Test Control Register (ITCR) - 0x80
    pub itcr: u32,              // 0x80
    /// Integration Test Input Register (ITIP) - 0x84
    pub itip: u32,              // 0x84
    /// Integration Test Output Register (ITOP) - 0x88
    pub itop: u32,              // 0x88
    /// Test Data Register (TDR) - 0x8C
    pub tdr: u32,               // 0x8C
}