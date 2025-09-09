use core::arch::global_asm;

use riscv::{
    ExceptionNumber,
    interrupt::{Exception, Trap},
    register::{
        scause, stval,
        stvec::{self, Stvec, TrapMode},
    },
};

use crate::{batch, errorln, syscall::syscall, trap::context::TrapContext};

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
                    batch::run_next_app();
                }
                Exception::StorePageFault => {
                    errorln!("PageFault in application, kernel killed it.");
                    batch::run_next_app();
                }
                Exception::IllegalInstruction => {
                    errorln!("IllegalInstruction in application, kernel killed it.");
                    batch::run_next_app();
                }
                _ => {
                    errorln!(
                        "Unsupported trap {:?}, stval = {:#x} in application, kernel killed it.",
                        scause.cause(),
                        stval
                    );
                    batch::run_next_app();
                }
            }
        }
        _ => {
            errorln!(
                "Unsupported trap {:?}, stval = {:#x} in application, kernel killed it.",
                scause.cause(),
                stval
            );
            batch::run_next_app();
        }
    }
    ctx
}
