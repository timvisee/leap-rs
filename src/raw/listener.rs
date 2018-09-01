use super::Controller;
use libc::c_void;

pub type Listener = *mut c_void;
pub type ListenerHandler = unsafe extern "C" fn(Listener, *const Controller);

// KEEP IN PERFECT SYNC WITH LM_FFIListener from wrapper/listener.h
#[repr(C)]
pub struct FFIListener {
    pub handle: Listener,
    pub on_exit: ListenerHandler,
    pub on_connect: ListenerHandler,
    pub on_frame: ListenerHandler,
    pub on_init: ListenerHandler,
    pub on_device_change: ListenerHandler,
    pub on_device_failure: ListenerHandler,
    pub on_disconnect: ListenerHandler,
    pub on_focus_gained: ListenerHandler,
    pub on_focus_lost: ListenerHandler,
    pub on_images: ListenerHandler,
    pub on_service_change: ListenerHandler,
    pub on_service_connect: ListenerHandler,
    pub on_service_disconnect: ListenerHandler,
}
