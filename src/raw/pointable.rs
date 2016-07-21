use std::os::raw::{
    c_int,
    c_float
};

pub enum Pointable {}
pub enum PointableList {}

extern {
    pub fn lm_pointable_id(this: *mut Pointable) -> i32;
    pub fn lm_pointable_touch_distance(this: *mut Pointable) -> c_float;
    pub fn lm_pointable_delete(this: *mut Pointable);
    pub fn lm_pointable_list_count(this: *mut PointableList) -> c_int;
    pub fn lm_pointable_list_delete(this: *mut PointableList);
    pub fn lm_pointable_list_at(this: *mut PointableList, index: c_int) -> *mut Pointable;
}
