#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod config;
mod language_items;
mod loader;
mod syscall;
mod system;
mod task;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn kernel_entry() -> ! {
    reset_bss_section();
    debugln!("Here's some debug message");
    infoln!("Hello, world! {}", "Hello OS!");
    warnln!("Do not do this!");
    errorln!("Error occured!");

    trap::init();
    loader::load_apps();
    task::run_first_task();
}

fn reset_bss_section() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|ptr| unsafe {
        (ptr as *mut u8).write_volatile(0);
    });
}
