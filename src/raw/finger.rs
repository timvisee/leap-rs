use super::Vector;
use std::os::raw::{c_float, c_int};

pub enum Finger {}
pub enum FingerList {}

extern "C" {
    pub fn lm_finger_id(this: *mut Finger) -> i32;
    pub fn lm_finger_touch_distance(this: *mut Finger) -> c_float;
    pub fn lm_finger_stabilized_tip_position(this: *mut Finger) -> *mut Vector;
    pub fn lm_finger_delete(this: *mut Finger);
    pub fn lm_finger_list_count(this: *mut FingerList) -> c_int;
    pub fn lm_finger_list_is_empty(this: *mut FingerList) -> bool;
    pub fn lm_finger_list_frontmost(this: *mut FingerList) -> *mut Finger;
    pub fn lm_finger_list_delete(this: *mut FingerList);
    pub fn lm_finger_list_at(this: *mut FingerList, index: c_int) -> *mut Finger;
}
