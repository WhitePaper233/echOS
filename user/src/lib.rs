//! # echOS User Space Library
//!
//! This crate provides the fundamental user-space runtime and system call interface
//! for applications running on the echOS operating system. It implements the basic
//! functionality needed for user programs to interact with the kernel.
#![no_std]
#![feature(linkage)]

use crate::syscall::system_yield;

#[macro_use]
pub mod console;
mod language_items;
pub mod syscall;

/// Writes data to a file descriptor.
///
/// # Arguments
///
/// * `fd` - The file descriptor to write to (e.g., 1 for stdout, 2 for stderr)
/// * `buf` - A byte slice containing the data to write
///
/// # Returns
///
/// Returns the number of bytes written on success, or a negative error code on failure.
pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::system_write(fd, buf.as_ptr(), buf.len())
}

/// Terminates the current process with the specified exit status.
///
/// # Arguments
///
/// * `status` - The exit status code to return to the parent process
pub fn exit(status: i32) -> ! {
    syscall::system_exit(status as usize)
}

pub fn yield_() -> isize {
    system_yield()
}

/// Application entry point called by the kernel when starting a user process.
///
/// This function serves as the real entry point for user applications. It performs
/// necessary runtime initialization before calling the user's main function.
/// The kernel jumps to this function when loading and starting a user program.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn app_entry() -> ! {
    reset_bss_section();
    exit(main())
}

/// Default weak implementation of the main function.
///
/// This provides a default `main()` function that panics if the user application
/// doesn't define its own `main()` function. User applications should override
/// this by defining their own `main()` function.
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Main function does not exist.");
}

/// Initializes the BSS section by zeroing all uninitialized global variables.
///
/// The BSS (Block Started by Symbol) section contains uninitialized global and
/// static variables that should be zero-initialized at program startup. This
/// function iterates through the BSS section and sets all bytes to zero.
fn reset_bss_section() {
    unsafe extern "C" {
        fn bbss();
        fn ebss();
    }
    (bbss as usize..ebss as usize).for_each(|ptr| unsafe {
        (ptr as *mut u8).write_volatile(0);
    });
}
