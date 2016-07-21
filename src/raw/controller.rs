use super::Frame;

pub enum Controller {}

extern {
    pub fn lm_controller_new() -> *mut Controller;
    pub fn lm_controller_is_connected(this: *mut Controller) -> bool;
    pub fn lm_controller_delete(this: *mut Controller);
    pub fn lm_controller_frame(this: *mut Controller) -> *mut Frame;
}
