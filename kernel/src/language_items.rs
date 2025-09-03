#[cfg(not(test))]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
