use core::arch::global_asm;

use crate::task::context::TaskContext;

global_asm!(include_str!("switch.S"));

unsafe extern "C" {
    pub fn __switch(curr_task_ctx_ptr: *mut TaskContext, next_task_ctx_ptr: *const TaskContext);
}
