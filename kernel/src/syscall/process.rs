use crate::{batch, debugln};

pub fn sys_exit(status: i32) -> ! {
    debugln!("Application exited with status {}", status);
    batch::run_next_app();
}
