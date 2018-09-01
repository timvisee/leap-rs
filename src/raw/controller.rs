use super::{DeviceList, FFIListener, Frame};

pub enum Controller {}

extern "C" {
    pub fn lm_controller_new() -> *mut Controller;
    pub fn lm_controller_with_listener(listener: FFIListener) -> *mut Controller;
    pub fn lm_controller_is_connected(this: *mut Controller) -> bool;
    pub fn lm_controller_delete(this: *mut Controller);
    pub fn lm_controller_frame(this: *mut Controller) -> *mut Frame;
    pub fn lm_controller_devices(this: *mut Controller) -> *mut DeviceList;
}
