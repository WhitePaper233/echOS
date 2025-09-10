//! # Console Module
//!
//! This module provides console related functions for user-space applications running on echOS.

use core::fmt::{self, Write};

use crate::write;

/// Standard input file descriptor constant.
pub const FD_STDIN: usize = 0;
/// Standard output file descriptor constant.
pub const FD_STDOUT: usize = 1;
/// Standard error file descriptor constant.
pub const FD_STDERR: usize = 2;

/// Console structure that provides formatted output to stdout.
struct Console;

impl Console {
    /// Creates a new Console instance.
    ///
    /// # Returns
    ///
    /// A new `Console` instance ready for writing formatted output.
    pub fn new() -> Self {
        Self {}
    }
}

impl Write for Console {
    /// Writes a string slice to the console (stdout).
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to write to the console
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the write operation succeeded (returned 0)
    /// * `Err(fmt::Error)` if the write operation failed (returned non-zero)
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match write(FD_STDOUT, s.as_bytes()) {
            ..=-1 => Err(fmt::Error {}),
            _ => Ok(()),
        }
    }
}

/// Prints formatted arguments to the console.
///
/// This function takes format arguments (as produced by the `format_args!` macro)
/// and writes them to stdout through a Console instance. It's the underlying
/// implementation used by the `print!` and `println!` macros.
///
/// # Arguments
///
/// * `args` - Format arguments containing the formatted string and values
///
/// # Panics
///
/// Panics if the write operation fails, as this indicates a serious system error.
pub fn print(args: fmt::Arguments) {
    Console::new().write_fmt(args).unwrap();
}

/// Prints formatted text to the console without a trailing newline.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

/// Prints formatted text to the console with a trailing newline.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
