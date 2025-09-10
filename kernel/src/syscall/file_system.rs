use crate::{batch::run_next_app, errorln};

type FileDescriptor = usize;
/// Standard input file descriptor constant.
pub const FD_STDIN: FileDescriptor = 0;
/// Standard output file descriptor constant.
pub const FD_STDOUT: FileDescriptor = 1;
/// Standard error file descriptor constant.
pub const FD_STDERR: FileDescriptor = 2;

pub fn sys_write(fd: FileDescriptor, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap_or_default();
            print!("{}", str);
            len as isize
        }
        _ => {
            errorln!("Unsupported fd = {} in sys_write", fd);
            -1
        }
    }
}
