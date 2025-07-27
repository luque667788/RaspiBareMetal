// Logging module for UART output.
//
// NOTE: UART must be initialized (e.g., via `drivers::uart::uart0::init()`) BEFORE using any logging macros (print!/println!).
// Using logging before UART is initialized will result in lost or invalid output.
// It is the user's responsibility to ensure correct initialization order (no checks in the code for that).

use core::fmt::Write;

pub struct Logger;

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::drivers::uart::uart0::write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut logger = $crate::log::Logger;
        core::fmt::Write::write_fmt(&mut logger, format_args!($($arg)*)).ok();
    }};
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\r\n");
    }};
}
