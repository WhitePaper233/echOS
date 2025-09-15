use core::cell::SyncUnsafeCell;

use lazy_static::lazy_static;

use crate::{
    config,
    loader::{count_apps, init_app_ctx},
    task::{TaskControlBlock, TaskStatus, context::TaskContext, switch::__switch},
};

struct TaskManagerInner {
    tasks: [TaskControlBlock; config::MAX_APP_NUM],
    current_task: usize,
}

pub struct TaskManager {
    app_count: usize,
    inner: SyncUnsafeCell<TaskManagerInner>,
}

impl TaskManager {
    pub fn mark_current_suspended(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    pub fn mark_current_exited(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    pub fn find_next_task(&self) -> Option<usize> {
        let inner = unsafe { &mut (*self.inner.get()) };
        let current = inner.current_task;

        // find first ready task after current task
        (current + 1..current + self.app_count + 1)
            .map(|app_id| app_id % self.app_count)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let inner = unsafe { &mut (*self.inner.get()) };
            let current = inner.current_task;
            let current_task_ctx_ptr = &mut inner.tasks[current].task_ctx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_ctx as *mut TaskContext;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;

            unsafe {
                __switch(current_task_ctx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All applications completed!")
        }
    }

    pub fn run_first_task(&self) -> ! {
        let inner = unsafe { &mut (*self.inner.get()) };
        let first_task = inner.tasks.get_mut(0).unwrap_or_else(|| {
            panic!("All applications completed!");
        });
        first_task.task_status = TaskStatus::Running;
        let next_ctx_ptr = &mut first_task.task_ctx as *mut TaskContext;
        let unused_ctx_ptr = &mut TaskContext::zero_init() as *mut TaskContext;

        unsafe {
            __switch(unused_ctx_ptr, next_ctx_ptr);
        }
        unreachable!()
    }

    pub fn get_current_task(&self) -> usize {
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.current_task
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let app_count = count_apps();
        let mut tasks = [TaskControlBlock {
            task_ctx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; config::MAX_APP_NUM];

        for i in 0..app_count {
            // note: init_app_ctx setup the kernel context of app_{i} so __restore can launch the app
            // note: init kernel stack will be:
            // KernelContext {
            //     x: [0; 32]           // all general registers will be 0
            //     sstatus: SPP:User    // cpu is in U mode after trap
            //     spec: app_base_addr  // the app entry addr
            // }
            // note: init_app_ctx returns the sp of kernel stack after kernel context pushed into kernel stack
            //       so now the return value points at the kernel context
            // note: construct_restore_task will construct a task context that execute the __restore function
            //       after being replaced up. It sets ra to __restore's addr, sp to kernel stk and 0 to s regs
            tasks[i].task_ctx = TaskContext::construct_restore_task(init_app_ctx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }

        TaskManager {
            app_count,
            inner: SyncUnsafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    };
}
