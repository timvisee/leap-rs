use std::os::raw::c_float;

pub enum Controller {}
pub enum Frame {}

extern {
    pub fn lm_controller_new() -> *mut Controller;
    pub fn lm_controller_is_connected(this: *mut Controller) -> bool;
    pub fn lm_controller_delete(this: *mut Controller);
    pub fn lm_controller_frame(this: *mut Controller) -> *mut Frame;
    pub fn lm_frame_delete(this: *mut Frame);
    pub fn lm_frame_current_frames_per_second(this: *mut Frame) -> c_float;
}
