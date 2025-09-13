use crate::{config, debugln, trap::context::TrapContext};

pub static KERNEL_STACK: [KernelStack; config::MAX_APP_NUM] = [KernelStack {
    data: [0; config::KERNEL_STACK_SIZE],
}; config::MAX_APP_NUM];

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct KernelStack {
    data: [u8; config::KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + config::KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, trap_ctx: TrapContext) -> usize {
        let trap_ctx_ptr =
            (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_ctx_ptr = trap_ctx;
        }
        debugln!("{:x}", trap_ctx_ptr as usize);
        trap_ctx_ptr as usize
    }
}
