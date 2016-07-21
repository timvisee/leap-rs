use std::os::raw::c_int;

pub enum PointableList {}

extern {
    pub fn lm_pointable_list_count(this: *mut PointableList) -> c_int;
    pub fn lm_pointable_list_delete(this: *mut PointableList);
}
