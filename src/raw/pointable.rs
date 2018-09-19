use std::os::raw::{c_float, c_int};

use super::Vector;

pub enum Pointable {}
pub enum PointableList {}

extern "C" {
    pub fn lm_pointable_id(this: *mut Pointable) -> i32;
    pub fn lm_pointable_touch_distance(this: *mut Pointable) -> c_float;
    pub fn lm_pointable_stabilized_tip_position(this: *mut Pointable) -> *mut Vector;
    pub fn lm_pointable_delete(this: *mut Pointable);
    pub fn lm_pointable_is_extended(this: *mut Pointable) -> bool;
    pub fn lm_pointable_list_count(this: *mut PointableList) -> c_int;
    pub fn lm_pointable_list_is_empty(this: *mut PointableList) -> bool;
    pub fn lm_pointable_list_frontmost(this: *mut PointableList) -> *mut Pointable;
    pub fn lm_pointable_list_leftmost(this: *mut PointableList) -> *mut Pointable;
    pub fn lm_pointable_list_rightmost(this: *mut PointableList) -> *mut Pointable;
    pub fn lm_pointable_list_delete(this: *mut PointableList);
    pub fn lm_pointable_list_at(this: *mut PointableList, index: c_int) -> *mut Pointable;
    pub fn lm_pointable_list_extended(this: *mut PointableList) -> *mut PointableList;
}
