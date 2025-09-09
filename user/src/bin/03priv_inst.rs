#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[app_3] Try to execute privileged instruction in U Mode");
    println!("[app_3] Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
    return 0;
}
