#![no_std]
#![no_main]
mod language_items;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
