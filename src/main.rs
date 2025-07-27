#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod drivers;
mod hal;



// This is what your assembly boot.S calls
#[no_mangle] // Ensure the function name is not mangled by the compiler
// this is the section main of that the assembly code will jump to
pub extern "C" fn main() -> ! {
    // Initialize UART first
    drivers::uart::uart0::init();
    
    // Send a test message
    println!("Hello from Raspberry Pi 4 UART!");
    println!("UART is working!");
    println!("Send any character to see it echoed back!");
    
    let mut counter = 0u32;
    loop {
        // Send counter via UART
        println!("Loop count: {}", counter);
        
        // Check for incoming UART data and echo it
        if let Some(received_byte) = drivers::uart::uart0::read_byte() {
            print!("Received: '{}' (0x{:02X})\r\n", received_byte as char, received_byte);
        }
        
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

// Simple print macro for easier UART output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = crate::UartWriter;
        write!(writer, $($arg)*).ok();
    }};
}

#[macro_export]
macro_rules! println {
    () => (print!("\r\n"));
    ($($arg:tt)*) => {{
        print!($($arg)*);
        print!("\r\n");
    }};
}

// Simple wrapper to implement Write trait for UART
pub struct UartWriter;

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        drivers::uart::uart0::write_string(s);
        Ok(())
    }
}