use crate::{
    infoln, system::timer::read_time_ms, task::{exit_current_and_run_next, suspend_current_and_run_next}
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

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[allow(unused)]
pub fn sys_get_time_of_day(tp: *mut TimeVal, tzp: usize) -> isize {
    read_time_ms() as isize
}
