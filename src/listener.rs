use libc::c_void;

use super::Controller;
use raw;

pub trait Listener: Sized {
    fn on_exit(&mut self, &Controller) {}
    fn on_connect(&mut self, &Controller) {}
    fn on_frame(&mut self, &Controller) {}
    fn on_init(&mut self, &Controller) {}
    fn on_device_change(&mut self, &Controller) {}
    fn on_device_failure(&mut self, &Controller) {}
    fn on_disconnect(&mut self, &Controller) {}
    fn on_focus_gained(&mut self, &Controller) {}
    fn on_focus_lost(&mut self, &Controller) {}
    fn on_images(&mut self, &Controller) {}
    fn on_service_change(&mut self, &Controller) {}
    fn on_service_connect(&mut self, &Controller) {}
    fn on_service_disconnect(&mut self, &Controller) {}
    // fn on_log_message(&mut self, &Controller, MessageSeverity, Timestamp, message: String) {}
}

macro_rules! raw_listener {
    {$($HANDLER:ident => $FFI_HANDLER:ident),* $(,)*} => {
        trait RawListener: Listener + Sized {
            $(
                unsafe extern fn $HANDLER(this: *mut c_void, raw_controller: *const raw::Controller) {
                    let this = &mut *(this as *mut Self);
                    let controller = Controller::from_raw_ref(raw_controller);
                    this.$FFI_HANDLER(&controller);
                }
            )*

            unsafe extern fn raw_on_exit(this: *mut c_void, raw_controller: *const raw::Controller) {
                let this: *mut Self = this as *mut Self;
                let mut this = Box::from_raw(this);
                let controller = Controller::from_raw_ref(raw_controller);
                this.on_exit(&controller);
            }
        }
    }
}

raw_listener!{
    raw_on_connect => on_connect,
    raw_on_frame => on_frame,
    raw_on_init => on_init,
    raw_on_device_change => on_device_change,
    raw_on_device_failure => on_device_failure,
    raw_on_disconnect => on_disconnect,
    raw_on_focus_gained => on_focus_gained,
    raw_on_focus_lost => on_focus_lost,
    raw_on_images => on_images,
    raw_on_service_change => on_service_change,
    raw_on_service_connect => on_service_connect,
    raw_on_service_disconnect => on_service_disconnect,
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
            on_focus_gained: L::raw_on_focus_gained,
            on_focus_lost: L::raw_on_focus_lost,
            on_images: L::raw_on_images,
            on_service_change: L::raw_on_service_change,
            on_service_connect: L::raw_on_service_connect,
            on_service_disconnect: L::raw_on_service_disconnect,
        }
    }
}
