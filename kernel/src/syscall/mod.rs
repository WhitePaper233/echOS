use crate::{
    errorln,
    syscall::{
        file_system::sys_write,
        process::sys_exit,
        syscall_id::{SYSCALL_EXIT, SYSCALL_WRITE},
    },
};

pub mod file_system;
pub mod process;
pub mod syscall_id;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => {
            errorln!("Unsupported syscall_id: {}", syscall_id);
            -1
        }
    }
}
