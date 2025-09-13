use core::arch::global_asm;

use riscv::{
    ExceptionNumber,
    interrupt::{Exception, Trap},
    register::{
        scause, stval,
        stvec::{self, Stvec, TrapMode},
    },
};

use crate::{errorln, syscall::syscall, task, trap::context::TrapContext};

pub mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        fn __all_traps();
    }
    unsafe {
        let mut stvec_val = Stvec::from_bits(0);
        stvec_val.set_address(__all_traps as usize);
        stvec_val.set_trap_mode(TrapMode::Direct);
        stvec::write(stvec_val);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(exception_code) => {
            match Exception::from_number(exception_code).unwrap() {
                Exception::UserEnvCall => {
                    // move app's pc to next instruction
                    ctx.sepc += 4;
                    ctx.x[10] = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
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
        _ => {
            errorln!(
                "Unsupported interrupt {:?}, stval = {:#x} in application, kernel killed it.",
                scause.cause(),
                stval
            );
            task::exit_current_and_run_next();
        }
    }
    ctx
}
