use core::panic::PanicInfo;

use crate::{
    console::styles::{Color, FontStyle},
    system,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!(
                "[kernel] [{}{}PANIC{}{}] {}{}{} {}:{}\n                 {}{}{}",
                FontStyle::Bold.to_ansi_code(),
                Color::Red.to_ansi_code(),
                FontStyle::Reset.to_ansi_code(),
                Color::Default.to_ansi_code(),
                FontStyle::Bold.to_ansi_code(),
                Color::Red.to_ansi_code(),
                "System panicked at",
                location.file(),
                location.line(),
                info.message(),
                FontStyle::Reset.to_ansi_code(),
                Color::Default.to_ansi_code(),
            );
        }
        None => {
            println!("System panicked: {}", info.message());
        }
    }
    system::shutdown(true);
}
