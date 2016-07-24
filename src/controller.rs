use std::ops::Deref;
use std::ptr;
use raw;
use frame::Frame;
use libc::c_void;
use std::mem;

pub struct Controller {
    raw: *mut raw::Controller
}

impl Controller {
    pub fn new() -> Controller {
        unsafe {
            Self::from_raw(raw::lm_controller_new())
        }
    }

    pub unsafe fn from_raw(raw: *mut raw::Controller) -> Controller {
        Controller {
            raw: raw
        }
    }

    pub unsafe fn from_raw_ref(raw: *const raw::Controller) -> Ref {
        Ref::from_raw(raw)
    }

    pub fn with_listener<L: Listener + Send>(listener: L) -> Controller {
        unsafe {
            Self::from_raw(raw::lm_controller_with_listener(listener.into()))
        }
    }

    pub fn is_connected(&self) -> bool {
        unsafe {
            raw::lm_controller_is_connected(self.raw)
        }
    }

    pub fn frame(&self) -> Frame {
        unsafe {
            Frame::from_raw(raw::lm_controller_frame(self.raw))
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            // raw may be null if it is only borrowed
            if !self.raw.is_null() {
                raw::lm_controller_delete(self.raw);
            }
        }
    }
}

pub struct Ref {
    inner: Controller
}

impl Ref {
    fn from_raw(raw: *const raw::Controller) -> Ref {
        unsafe {
            Ref {
                inner: Controller::from_raw(raw as *mut _)
            }
        }
    }
}

impl Deref for Ref {
    type Target = Controller;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        self.inner.raw = ptr::null_mut();
    }
}

pub trait Listener {
    fn on_exit(&mut self, &Controller) {}
    fn on_connect(&mut self, &Controller) {}
    fn on_frame(&mut self, &Controller) {}
    // fn on_init(&mut self, &Controller) {}
    // fn on_device_change(&mut self, &Controller) {}
    // fn on_device_failure(&mut self, &Controller) {}
    // fn on_disconnect(&mut self, &Controller) {}
    // fn on_focus_gained(&mut self, &Controller) {}
    // fn on_focus_lost(&mut self, &Controller) {}
    // fn on_images(&mut self, &Controller) {}
    // fn on_log_message(&mut self, &Controller, MessageSeverity, Timestamp, message: String) {}
    // fn on_service_change(&mut self, Controller) {}
    // fn on_service_connect(&mut self, Controller) {}
    // fn on_service_disconnect(&mut self, Controller) {}
}

trait RawListener: Listener {
    extern fn raw_on_exit(this: *mut c_void, raw_controller: *const raw::Controller) {
        unsafe {
            let this: *mut Self = mem::transmute_copy(&this);
            let mut this = Box::from_raw(this);
            let controller = Controller::from_raw_ref(raw_controller);
            this.on_exit(&controller);
        }
    }

    extern fn raw_on_connect(this: *mut c_void, raw_controller: *const raw::Controller) {
        unsafe {
            let this: &mut Self = mem::transmute_copy(&this);
            let controller = Controller::from_raw_ref(raw_controller);
            this.on_connect(&controller);
        }
    }

    extern fn raw_on_frame(this: *mut c_void, raw_controller: *const raw::Controller) {
        unsafe {
            let this: &mut Self = mem::transmute_copy(&this);
            let controller = Controller::from_raw_ref(raw_controller);
            this.on_frame(&controller);
        }
    }
}

impl<L: Listener> RawListener for L {}

impl<L: Listener> From<L> for raw::FFIListener {
    fn from(listener: L) -> Self {
        raw::FFIListener {
            handle: Box::into_raw(Box::new(listener)) as *mut _,
            on_exit: L::raw_on_exit,
            on_connect: L::raw_on_connect,
            on_frame: L::raw_on_frame,
        }
    }
}
