//! GPIO driver for Raspberry Pi 4
//!
//! Provides basic GPIO pin control: set as output, set high, set low.
//! This driver is generic and can be used for any GPIO pin (0-53).
//!
//! # Example
//! 
//! ```rust
//! use drivers::gpio::GpioPin;
//!
//! // Create a GPIO pin instance for GPIO 42 (ACT LED)
//! let led = GpioPin::new(42);
//! led.set_output();
//! led.set_high(); // Turn LED on
//! led.set_low();  // Turn LED off
//! ```

use crate::hal::registers::gpio::{GPIO_REGS, GpioRegisters};

/// Represents a GPIO pin number (0-53).
pub struct GpioPin(u8);

impl GpioPin {
    /// Create a new GPIO pin instance.
    /// # Arguments
    /// * `pin` - GPIO pin number (0-53)
    pub const fn new(pin: u8) -> Self {
        GpioPin(pin)
    }

    /// Set this GPIO pin as output.
    /// This configures the pin's function select bits to '001' (output).
    pub fn set_output(&self) {
        let pin = self.0;
        let fsel_index = pin / 10; // Each GPFSEL controls 10 pins
        let fsel_shift = (pin % 10) * 3;
        unsafe {
            let regs = &mut *GPIO_REGS;
            let fsel = match fsel_index {
                0 => &mut regs.gpfsel0,
                1 => &mut regs.gpfsel1,
                2 => &mut regs.gpfsel2,
                3 => &mut regs.gpfsel3,
                4 => &mut regs.gpfsel4,
                5 => &mut regs.gpfsel5,
                _ => return, // Invalid pin
            };
            let mut val = core::ptr::read_volatile(fsel);
            val &= !(0b111 << fsel_shift); // Clear function bits
            val |= 0b001 << fsel_shift;   // Set to output (001)
            core::ptr::write_volatile(fsel, val);
        }
    }

    /// Set this GPIO pin high (logic 1).
    pub fn set_high(&self) {
        let pin = self.0;
        unsafe {
            let regs = &mut *GPIO_REGS;
            if pin < 32 {
                regs.gpset0 = 1 << pin;
            } else {
                regs.gpset1 = 1 << (pin - 32);
            }
        }
    }

    /// Set this GPIO pin low (logic 0).
    pub fn set_low(&self) {
        let pin = self.0;
        unsafe {
            let regs = &mut *GPIO_REGS;
            if pin < 32 {
                regs.gpclr0 = 1 << pin;
            } else {
                regs.gpclr1 = 1 << (pin - 32);
            }
        }
    }
}
