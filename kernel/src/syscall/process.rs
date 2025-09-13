use crate::{
    infoln,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
};

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_exit(status: i32) -> ! {
    infoln!("Application exited with code {}", status);
    exit_current_and_run_next();
    unreachable!();
}
