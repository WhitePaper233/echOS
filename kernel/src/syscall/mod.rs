use crate::{
    errorln,
    syscall::{
        file_system::sys_write,
        process::{sys_exit, sys_get_time_of_day, sys_yield},
        syscall_id::{SYSCALL_EXIT, SYSCALL_GET_TIME_OF_DAY, SYSCALL_WRITE, SYSCALL_YIELD},
    },
};

pub mod file_system;
pub mod process;
pub mod syscall_id;

pub fn handle_syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME_OF_DAY => sys_get_time_of_day(core::ptr::null_mut(), 0),
        _ => {
            errorln!("Unsupported syscall_id: {}", syscall_id);
            -1
        }
    }
}
