use std::os::raw::c_float;

pub enum Frame {}

extern {
    pub fn lm_frame_delete(this: *mut Frame);
    pub fn lm_frame_current_frames_per_second(this: *mut Frame) -> c_float;
}
