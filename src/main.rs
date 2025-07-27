#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod drivers;
mod hal;
mod log;

use drivers::gpio::GpioPin;
use log::*;



// This is what your assembly boot.S calls
#[no_mangle] // Ensure the function name is not mangled by the compiler
// this is the section main of that the assembly code will jump to
pub extern "C" fn main() -> ! {
    // Initialize UART first otherwise logging will not work
    drivers::uart::uart0::init();
    
    // Send a test message
println!("Hello from Raspberry Pi 4 UART!");
    println!("UART is working!");
    println!("Send any character to see it echoed back!");
    
    // Example: Blink ACT LED (GPIO 42) to confirm kernel is running
    let act_led = GpioPin::new(42);
    act_led.set_output();
    let mut led_on = false;
    
    let mut counter = 0u32;
    loop {
        // Send counter via UART
        println!("Loop count: {}", counter);
        
        // Check for incoming UART data and echo it
        if let Some(received_byte) = drivers::uart::uart0::read_byte() {
            print!("Received: '{}' (0x{:02X})\r\n", received_byte as char, received_byte);
        }
        
        // Blink ACT LED (toggle logic in main)
        if led_on {
            act_led.set_low();
        } else {
            act_led.set_high();
        }
        led_on = !led_on;
        
        // Simple delay
        for _ in 0..1000000 {
            core::hint::spin_loop();
        }
        
        counter += 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}