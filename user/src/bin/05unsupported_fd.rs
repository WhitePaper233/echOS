#![no_std]
#![no_main]

use user::syscall::system_write;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let str = "This should not be outputed";
    system_write(1, str.as_ptr(), str.len());

    return 0;
}
