use libc::c_void;

use super::Frame;

pub enum Controller {}

extern {
    pub fn lm_controller_new() -> *mut Controller;
    pub fn lm_controller_with_listener(listener: FFIListener) -> *mut Controller;
    pub fn lm_controller_is_connected(this: *mut Controller) -> bool;
    pub fn lm_controller_delete(this: *mut Controller);
    pub fn lm_controller_frame(this: *mut Controller) -> *mut Frame;
}

pub type Listener = *mut c_void;
pub type ListenerHandler = extern fn(Listener, *const Controller);

// KEEP IN PERFECT SYNC WITH LM_FFIListener from wrapper/controller.h
#[repr(C)]
pub struct FFIListener {
    pub handle: Listener,
    pub on_exit: ListenerHandler,
    pub on_connect: ListenerHandler,
    pub on_frame: ListenerHandler,
}
