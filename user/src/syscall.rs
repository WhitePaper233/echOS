//! # System Call Interface Module
//!
//! This module provides the low-level system call interface for echOS user-space
//! applications. It implements the RISC-V system call ABI and provides safe
//! wrappers around raw system calls.

use core::arch::asm;

/// System call number for the write operation.
const SYSCALL_WRITE: usize = 64;

/// System call number for the exit operation.
const SYSCALL_EXIT: usize = 93;

/// Performs a raw system call using the RISC-V ecall instruction.
///
/// This function implements the low-level system call interface following
/// the RISC-V calling convention. It places arguments in the appropriate
/// registers and executes the `ecall` instruction to transfer control
/// to the kernel.
pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        )
    }
    ret
}

/// Writes data to a file descriptor using the write system call.
///
/// This function provides a type-safe wrapper around the write system call,
/// allowing applications to output data to file descriptors such as stdout,
/// stderr, or opened files.
///
/// # Arguments
///
/// * `fd` - The file descriptor to write to
/// * `buf` - Pointer to the buffer containing data to write
/// * `len` - Number of bytes to write from the buffer
///
/// # Returns
///
/// Returns the number of bytes successfully written, or a negative error code:
/// - Positive value: Number of bytes written (may be less than requested)
/// - 0: No bytes written (not necessarily an error)
/// - Negative: Error code indicating the type of failure
///
/// # Safety
///
/// The caller must ensure that:
/// - `buf` points to valid memory containing at least `len` bytes
/// - The memory pointed to by `buf` remains valid for the duration of the call
/// - `fd` is a valid file descriptor open for writing
///
/// # Examples
///
/// ```no_run
/// use user::syscall::system_write;
///
/// let message = b"Hello, world!\n";
/// let bytes_written = system_write(1, message.as_ptr(), message.len());
/// ```
pub fn system_write(fd: usize, buf: *const u8, len: usize) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf as usize, len])
}

/// Terminates the current process with the specified exit status.
///
/// This function invokes the exit system call to immediately terminate
/// the calling process. The exit status is passed to the parent process
/// and can be used to indicate success or failure.
///
/// # Arguments
///
/// * `status` - The exit status code:
///   - 0: Success (no errors)
///   - 1-255: Various error conditions (application-defined)
///
/// # Behavior
///
/// This function never returns normally. It transfers control to the kernel
/// which will:
/// 1. Clean up process resources (memory, file descriptors, etc.)
/// 2. Notify the parent process of termination
/// 3. Remove the process from the system
///
/// # Examples
///
/// ```no_run
/// use user::syscall::system_exit;
///
/// // Exit with success status
/// system_exit(0);
///
/// // Exit with error status
/// system_exit(1);
/// ```
pub fn system_exit(status: usize) -> ! {
    syscall(SYSCALL_EXIT, [status, 0, 0]);
    unreachable!()
}

pub fn system_yield() -> isize {
    syscall(SYSCALL_EXIT, [0, 0, 0])
}
