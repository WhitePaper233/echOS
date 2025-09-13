use core::arch::asm;

use crate::{
    config, debugln, loader::{kernel_stack::KERNEL_STACK, user_stack::USER_STACK}, trap::context::TrapContext
};

mod kernel_stack;
mod user_stack;

unsafe extern "C" {
    fn _app_nums();
}

pub fn read_base_addr_of_app(app_id: usize) -> usize {
    config::APP_BASE_ADDR + app_id * config::APP_SIZE_LIMIT
}

pub fn count_apps() -> usize {
    unsafe { (_app_nums as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
    let app_num_ptr = _app_nums as usize as *const usize;
    let app_count = count_apps();
    let app_start = unsafe { core::slice::from_raw_parts(app_num_ptr.add(1), app_count + 1) };

    (0..app_count).for_each(|i| {
        // clear region for app
        let app_base_addr = read_base_addr_of_app(i);
        (app_base_addr..app_base_addr + config::APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0);
        });

        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *mut u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { core::slice::from_raw_parts_mut(app_base_addr as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    });

    unsafe {
        asm!("fence.i");
    }
}

pub fn init_app_ctx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        read_base_addr_of_app(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}
