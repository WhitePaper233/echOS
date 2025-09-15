use core::arch::global_asm;

use riscv::{
    ExceptionNumber, InterruptNumber,
    interrupt::{Exception, Interrupt, Trap},
    register::{
        scause, sie, stval,
        stvec::{self, Stvec, TrapMode},
    },
};

use crate::{
    debugln, errorln,
    syscall::handle_syscall,
    system::timer::set_next_trigger,
    task::{self, suspend_current_and_run_next, task_manager::TASK_MANAGER},
    trap::context::TrapContext,
};

pub mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        fn __all_traps();
    }
    unsafe {
        // setup trap handler base addr and mode to direct
        let mut stvec_val = Stvec::from_bits(0);
        stvec_val.set_address(__all_traps as usize);
        stvec_val.set_trap_mode(TrapMode::Direct);
        stvec::write(stvec_val);

        // timer interrupt enabled
        sie::set_stimer();
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(exception_number) => {
            match Exception::from_number(exception_number).unwrap() {
                Exception::UserEnvCall => {
                    // move app's pc to next instruction
                    ctx.sepc += 4;
                    ctx.x[10] =
                        handle_syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
                }
                Exception::StoreFault => {
                    errorln!("StoreFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::StorePageFault => {
                    errorln!("PageFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::IllegalInstruction => {
                    errorln!("IllegalInstruction in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                _ => {
                    errorln!(
                        "Unsupported exception {:?}, stval = {:#x} in application, kernel killed it.",
                        scause.cause(),
                        stval
                    );
                    task::exit_current_and_run_next();
                }
            }
        }
        Trap::Interrupt(interrupt_number) => {
            match Interrupt::from_number(interrupt_number).unwrap() {
                Interrupt::SupervisorTimer => {
                    debugln!(
                        "SupervisorTimerInterrupt occurs, switching: app_{} to app_{}",
                        TASK_MANAGER.get_current_task(),
                        TASK_MANAGER
                            .find_next_task()
                            .map(|v| v as isize)
                            .unwrap_or(-1)
                    );
                    set_next_trigger();
                    suspend_current_and_run_next();
                }
                _ => {
                    errorln!(
                        "Unsupported interrupt {:?}, stval = {:#x} in application, kernel killed it.",
                        scause.cause(),
                        stval
                    );
                    task::exit_current_and_run_next();
                }
            }
        }
    }
    ctx
}
