use riscv::register::time;

use crate::config::{CLOCK_FREQ, TICKS_PER_SEC};

const MS_PER_SEC: usize = 1000;
const US_PER_SEC: usize = 1_000_000;


pub fn read_time() -> usize {
    time::read()
}

pub fn read_time_ms() -> usize {
    read_time() / (CLOCK_FREQ / MS_PER_SEC)
}

pub fn read_time_us() -> usize {
    read_time() / (CLOCK_FREQ / US_PER_SEC)
}

pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as u64);
}

pub fn set_next_trigger() {
    // note:
    //                 CLOCK_FREQ - `mtime` register's value increased in 1s
    //              TICKS_PER_SEC - number of interruptions occurring within 1s
    // CLOCK_FREQ / TICKS_PER_SEC - increased value of `mtime` register that next interruption occurrs since now
    set_timer(read_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
