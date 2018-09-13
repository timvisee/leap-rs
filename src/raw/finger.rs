use super::Vector;
use std::os::raw::{c_float, c_int};

pub enum Finger {}
pub enum FingerList {}

extern "C" {
    pub fn lm_finger_id(this: *mut Finger) -> i32;
    pub fn lm_finger_touch_distance(this: *mut Finger) -> c_float;
    pub fn lm_finger_stabilized_tip_position(this: *mut Finger) -> *mut Vector;
    pub fn lm_finger_delete(this: *mut Finger);
    pub fn lm_finger_type(this: *mut Finger) -> u32;
    pub fn lm_finger_is_extended(this: *mut Finger) -> bool;
    pub fn lm_finger_list_count(this: *mut FingerList) -> c_int;
    pub fn lm_finger_list_is_empty(this: *mut FingerList) -> bool;
    pub fn lm_finger_list_frontmost(this: *mut FingerList) -> *mut Finger;
    pub fn lm_finger_list_leftmost(this: *mut FingerList) -> *mut Finger;
    pub fn lm_finger_list_rightmost(this: *mut FingerList) -> *mut Finger;
    pub fn lm_finger_list_delete(this: *mut FingerList);
    pub fn lm_finger_list_at(this: *mut FingerList, index: c_int) -> *mut Finger;
    pub fn lm_finger_list_finger_type(this: *mut FingerList, finger_type: u32) -> *mut FingerList;
    pub fn lm_finger_list_extended(this: *mut FingerList) -> *mut FingerList;
}
