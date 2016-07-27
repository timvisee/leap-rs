use std::os::raw::{
    c_int,
    c_float
};
use super::Vector;

pub enum Hand {}
pub enum HandList {}

extern {
    pub fn lm_hand_id(this: *mut Hand) -> i32;
    pub fn lm_hand_pinch_distance(this: *mut Hand) -> c_float;
    pub fn lm_hand_stabilized_palm_position(this: *mut Hand) -> *mut Vector;
    pub fn lm_hand_delete(this: *mut Hand);
    pub fn lm_hand_list_count(this: *mut HandList) -> c_int;
    pub fn lm_hand_list_is_empty(this: *mut HandList) -> bool;
    pub fn lm_hand_list_frontmost(this: *mut HandList) -> *mut Hand;
    pub fn lm_hand_list_delete(this: *mut HandList);
    pub fn lm_hand_list_at(this: *mut HandList, index: c_int) -> *mut Hand;
}
