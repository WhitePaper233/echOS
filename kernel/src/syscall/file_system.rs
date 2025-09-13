use crate::errorln;

type FileDescriptor = usize;
#[allow(unused)]
/// Standard input file descriptor constant.
pub const FD_STDIN: FileDescriptor = 0;
#[allow(unused)]
/// Standard output file descriptor constant.
pub const FD_STDOUT: FileDescriptor = 1;
#[allow(unused)]
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
