#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use riscv::register::sstatus::SPP;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[app_4] Try to access privileged CSR in U Mode");
    println!("[app_4] Kernel should kill this application!");
    unsafe {
        riscv::register::sstatus::set_spp(SPP::User);
    }
    return 0;
}
