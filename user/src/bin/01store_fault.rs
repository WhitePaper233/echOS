#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[app_1] Into Test store_fault, we will insert an invalid store operation...");
    println!("[app_1] Kernel should kill this application!");

    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }

    return 0;
}
