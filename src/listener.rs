use std::mem;
use libc::c_void;
use raw;
use Controller;

pub trait Listener {
    fn on_exit(&mut self, &Controller) {}
    fn on_connect(&mut self, &Controller) {}
    fn on_frame(&mut self, &Controller) {}
    fn on_init(&mut self, &Controller) {}
    fn on_device_change(&mut self, &Controller) {}
    fn on_device_failure(&mut self, &Controller) {}
    fn on_disconnect(&mut self, &Controller) {}
    // fn on_focus_gained(&mut self, &Controller) {}
    // fn on_focus_lost(&mut self, &Controller) {}
    // fn on_images(&mut self, &Controller) {}
    // fn on_log_message(&mut self, &Controller, MessageSeverity, Timestamp, message: String) {}
    // fn on_service_change(&mut self, Controller) {}
    // fn on_service_connect(&mut self, Controller) {}
    // fn on_service_disconnect(&mut self, Controller) {}
}

macro_rules! raw_listener {
    {$($HANDLER:ident => $FFI_HANDLER:ident),* $(,)*} => {
        trait RawListener: Listener {
            $(
                extern fn $HANDLER(this: *mut c_void, raw_controller: *const raw::Controller) {
                    unsafe {
                        let this: &mut Self = mem::transmute_copy(&this);
                        let controller = Controller::from_raw_ref(raw_controller);
                        this.$FFI_HANDLER(&controller);
                    }
                }
            )*

            extern fn raw_on_exit(this: *mut c_void, raw_controller: *const raw::Controller) {
                unsafe {
                    let this: *mut Self = mem::transmute_copy(&this);
                    let mut this = Box::from_raw(this);
                    let controller = Controller::from_raw_ref(raw_controller);
                    this.on_exit(&controller);
                }
            }
        }
    }
}

raw_listener!{
    raw_on_connect => on_exit,
    raw_on_frame => on_frame,
    raw_on_init => on_init,
    raw_on_device_change => on_device_change,
    raw_on_device_failure => on_device_failure,
    raw_on_disconnect => on_disconnect,
}

impl<L: Listener> RawListener for L {}

impl<L: Listener> From<L> for raw::FFIListener {
    fn from(listener: L) -> Self {
        raw::FFIListener {
            handle: Box::into_raw(Box::new(listener)) as *mut _,
            on_exit: L::raw_on_exit,
            on_connect: L::raw_on_connect,
            on_frame: L::raw_on_frame,
            on_init: L::raw_on_init,
            on_device_change: L::raw_on_device_change,
            on_device_failure: L::raw_on_device_failure,
            on_disconnect: L::raw_on_disconnect,
        }
    }
}
