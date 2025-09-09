use core::arch::asm;
use core::cell::SyncUnsafeCell;
use lazy_static::lazy_static;

use crate::trap::context::TrapContext;
use crate::{debugln, infoln};

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDR: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_init_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(&self, ctx: TrapContext) -> *mut TrapContext {
        let ctx_ptr =
            (self.get_init_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ctx_ptr = ctx;
        }
        ctx_ptr
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_init_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

struct AppManager {
    app_num: usize,
    curr_app: usize,
    app_addrs: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        infoln!("app_num = {}", self.app_num);
        (0..self.app_num).for_each(|i| {
            infoln!(
                "app_{} [{:#x}, {:#x})",
                i,
                self.app_addrs[i],
                self.app_addrs[i + 1]
            );
        });
    }

    pub fn get_current_app(&self) -> usize {
        self.curr_app
    }

    pub fn move_to_next_app(&mut self) {
        self.curr_app += 1;
    }

    fn load_app(&self, app_id: usize) {
        if app_id >= self.app_num {
            panic!("All applications completed!");
        }
        debugln!("Loading app_{}", app_id);

        unsafe {
            // clear app instruction section
            asm!("fence.i");
            core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, APP_SIZE_LIMIT).fill(0);

            // load next app
            let app_src = core::slice::from_raw_parts(
                self.app_addrs[app_id] as *const u8,
                self.app_addrs[app_id + 1] - self.app_addrs[app_id],
            );
            let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);
        }
    }
}

lazy_static! {
    static ref APP_MANAGER: SyncUnsafeCell<AppManager> = unsafe {
        SyncUnsafeCell::new({
            unsafe extern "C" {
                fn app_instruction_section_ptr();
            }

            let app_num_ptr = app_instruction_section_ptr as usize as *const usize;
            let mut app_addrs: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];

            let app_num = app_num_ptr.read_volatile();
            let start_app_raw: &[usize] =
                core::slice::from_raw_parts(app_num_ptr.add(1), app_num + 1);

            app_addrs[..=app_num].copy_from_slice(start_app_raw);

            AppManager {
                app_num,
                curr_app: 0,
                app_addrs,
            }
        })
    };
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

pub fn init() {
    let app_manager = unsafe { &*APP_MANAGER.get() };
    app_manager.print_app_info();
}

pub fn run_next_app() -> ! {
    {
        let app_manager_ptr = APP_MANAGER.get();
        let app_manager = unsafe { &mut *app_manager_ptr };
        let current_app = app_manager.get_current_app();
        app_manager.load_app(current_app);
        app_manager.move_to_next_app();
    }

    unsafe extern "C" {
        fn __restore(ctx: usize);
    }
    unsafe {
        let ctx_addr = KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDR,
            USER_STACK.get_init_sp(),
        )) as usize;
        __restore(ctx_addr);
    }
    unreachable!()
}
