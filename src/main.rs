#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This is what your assembly boot.S calls
#[no_mangle] // Ensure the function name is not mangled by the compiler
// this is the section main of that the assembly code will jump to
pub extern "C" fn main() -> ! {
    // Simple LED blink example - write to GPIO
    // RPi4 GPIO base address is 0xFE200000
    
    let gpio_base = 0xFE200000 as *mut u32;// this is not unsafe becasue we are not dereferencing it yet (just creating a pointer)
    // we get this value from the Raspberry Pi 4 documentation, which specifies the base address for GPIO registers.

    // The GPIO registers are memory-mapped, so we can use raw pointers to access them.
    // The GPIO registers are 32-bit wide, so we can use u32 pointers.
    unsafe {
        // Set GPIO 21 as output (built-in LED on some RPi4 boards)
        // GPFSEL2 register controls GPIO 20-29
        let gpfsel2 = gpio_base.add(2);// the function .add(2) will do pointer arithmetic, moving the pointer 2 * 4 bytes (since each register is 4 bytes) to point to GPFSEL2.
        let mut val = gpfsel2.read_volatile();
        val &= !(0b111 << 3); // Clear GPIO 21 function bits
        val |= 0b001 << 3;    // Set as output
        gpfsel2.write_volatile(val);
        
        // Blink the LED
        let gpset0 = gpio_base.add(7);  // GPIO set register
        let gpclr0 = gpio_base.add(10); // GPIO clear register
        let gpio21_bit = 1 << 21;
        
        loop {
            // Turn LED on
            gpset0.write_volatile(gpio21_bit);
            
            // Simple delay
            for _ in 0..1000000 {
                core::hint::spin_loop();
            }
            
            // Turn LED off
            gpclr0.write_volatile(gpio21_bit);
            
            // Simple delay
            for _ in 0..1000000 {
                core::hint::spin_loop();
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}