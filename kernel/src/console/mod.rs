use core::fmt::{self, Write};

use sbi_rt::Physical;

pub mod logger;
pub mod styles;

struct Console;

impl Console {
    fn new() -> Self {
        Self {}
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let phys_addr = s.as_ptr() as usize;
        let phys_addr_lo = phys_addr & 0xFFFF_FFFF;
        let phys_addr_hi = phys_addr >> 32;

        let ret = sbi_rt::console_write(Physical::new(s.len(), phys_addr_lo, phys_addr_hi));
        if ret.is_ok() {
            Ok(())
        } else {
            Err(fmt::Error {})
        }
    }
}

pub fn print(args: fmt::Arguments) {
    Console::new().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
