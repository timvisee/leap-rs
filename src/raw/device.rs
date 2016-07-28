use std::os::raw::c_int;

pub enum Device {}
pub enum DeviceList {}

extern {
    pub fn lm_device_delete(this: *mut Device);
    pub fn lm_device_baseline(this: *mut Device) -> f32;
    pub fn lm_device_horizontal_view_angle(this: *mut Device) -> f32;
    pub fn lm_device_is_embedded(this: *mut Device) -> bool;
    // pub fn lm_device_is_lighting_bad(this: *mut Device) -> bool;
    // pub fn lm_device_is_smudged(this: *mut Device) -> bool;
    pub fn lm_device_is_streaming(this: *mut Device) -> bool;
    pub fn lm_device_is_valid(this: *mut Device) -> bool;
    pub fn lm_device_range(this: *mut Device) -> f32;
    pub fn lm_device_vertical_view_angle(this: *mut Device) -> f32;
    pub fn lm_device_list_count(this: *mut DeviceList) -> c_int;
    pub fn lm_device_list_is_empty(this: *mut DeviceList) -> bool;
    pub fn lm_device_list_delete(this: *mut DeviceList);
    pub fn lm_device_list_at(this: *mut DeviceList, index: c_int) -> *mut Device;
}
