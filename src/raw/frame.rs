use std::os::raw::c_float;
use super::PointableList;
use super::InteractionBox;

pub enum Frame {}

extern {
    pub fn lm_frame_delete(this: *mut Frame);
    pub fn lm_frame_current_frames_per_second(this: *mut Frame) -> c_float;
    pub fn lm_frame_pointables(this: *mut Frame) -> *mut PointableList;
    pub fn lm_frame_interaction_box(this: *mut Frame) -> *mut InteractionBox;
}
