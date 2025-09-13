#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    pub ra: usize,
    pub sp: usize,
    pub s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn construct_restore_task(kernel_stk_ptr: usize) -> Self {
        unsafe extern "C" {
            unsafe fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kernel_stk_ptr,
            s: [0; 12],
        }
    }
}
