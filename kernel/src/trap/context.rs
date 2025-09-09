use riscv::register::sstatus::{self, SPP, Sstatus};

#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut _sstatus = sstatus::read();
        _sstatus.set_spp(SPP::User);
        let mut ctx = Self {
            x: [0; 32],
            sstatus: _sstatus,
            sepc: entry,
        };
        ctx.set_sp(sp);
        ctx
    }
}
