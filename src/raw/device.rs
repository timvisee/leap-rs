use std::os::raw::c_int;

pub enum Device {}
pub enum DeviceList {}

extern {
    pub fn lm_device_delete(this: *mut Device);
    pub fn lm_device_list_count(this: *mut DeviceList) -> c_int;
    pub fn lm_device_list_is_empty(this: *mut DeviceList) -> bool;
    pub fn lm_device_list_delete(this: *mut DeviceList);
    pub fn lm_device_list_at(this: *mut DeviceList, index: c_int) -> *mut Device;
}
