use crate::hal::registers::uart::{MiniUartRegisters, MINI_UART_REGS};
use crate::hal::registers::gpio::GpioRegisters;
use crate::hal::registers::utils::*;
use crate::hal::registers::auxiliary::{AUX_REGS_BASE, AUX_REGS}; 

pub fn init() {
    // Initialize UART driver
    

    // setup the GPIO 14 for TX and GPIO 15 for RX (make them operate in function 5 mode)
    // GPFSELn registers control the function of GPIO pins

    
    
    // write the corresponding 3-bit value to its FSEL field. For ALT5, the value is 010
    // The base address for GPIO registers is 0x7e200000. The GPFSEL1 register is at offset 0x04 from the base
    // FSEL14 and FSEL15 fields within the GPFSEL1 register at address 0x7e200000 + 0x04

    //The Mini UART is enabled via the AUX_ENABLES register setting Bit 0
    // The Auxiliary register base address is 0x7e215000. The AUX_ENABLES register is at offset 0x04 from this base

    //Baud Rate: The baud rate is derived from the system clock. You set the baud rate using the AUX_MU_BAUD_REG. 
    //This is a 16-bit register. It can be accessed directly via offset 0x68 from the Auxiliary base address (0x7e215000)
    // Baudrate = System_Clock_Freq / (8 * (Baudrate_Reg + 1)) so if we want 115200 baud, we can set the register to 270 (assuming a 250MHz system clock)


    // set the data size at AUX_MU_LCR_REG bit 0 (If set the UART works in 8-bit mode else it works in 7-bit mode) 
    // offset 0x4c from the Auxiliary base (0x7e215000) = 0x7e21504c

    // the RTS and CTS will be disabled by default, so we can ignore them for now

    // Enable transmitter and receiver by setting the corresponding bits in the AUX_MU_CNTL_REG register
    // The AUX_MU_CNTL_REG is at offset 0x60 from the Auxiliary base address (0x7e215000) 
    // Bit 0 enables the transmitter, and Bit 1 enables the receiver.
    // no need to set them becasue the default is already enabled
    // TODO! provide a proper function implementation to set up all this stuff later

    // Good practice to clear the FIFOs before using them
    // The AUX_MU_IER_REG is at offset 0x48 from the Auxiliary base address (0x7e215000)
    // writing 1 to bit 1 clears the receive FIFO, and writing 1 to bit 2 clears the transmit FIFO

}

pub fn write_byte(byte: u8) {
    // Write a byte to the UART

    // Wait until the transmit FIFO is not full
    // The Transmitter empty (TXE) flag (AUX_MU_LSR_REG bit 5) (offset is 0x54) indicates if the transmit FIFO can accept at least one byte
    // The Space available flag (AUX_MU_STAT_REG bit 1) (offset is 0x64) )also indicates if the TX FIFO can accept at least one symbol
    // you can check any of these flags to determine if the FIFO is not full meaning we can write to it

    // after that writing to the AUX_MU_IO_REG (offset is 0x40) will write the byte to the transmit FIFO


}


//after that we can poll the Transmitter idle flag (AUX_MU_LSR_REG bit 6) (offset is 0x54)
// or the Transmitter done flag (AUX_MU_STAT_REG bit 9) (offset is 0x64).
//Transmitter done (STAT bit 9) is set when the transmitter is idle and the transmit FIFO is empty
