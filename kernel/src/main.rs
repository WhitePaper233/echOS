#![no_std]
#![no_main]

use core::arch::global_asm;
#[macro_use]
mod console;
mod language_items;
mod system;

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn kernel_entry() -> ! {
    reset_bss_section();
    debugln!("Here's some debug message");
    infoln!("Hello, world! {}", "Hello OS!");
    warnln!("Do not do this!");
    errorln!("Error occured!");
    panic!("Shutdown Machine!")
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
