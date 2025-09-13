use crate::config;

pub static USER_STACK: [UserStack; config::MAX_APP_NUM] = [UserStack {
    data: [0; config::USER_STACK_SIZE],
}; config::MAX_APP_NUM];

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct UserStack {
    data: [u8; config::USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + config::USER_STACK_SIZE
    }
}
