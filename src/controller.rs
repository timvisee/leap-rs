use std::{ops::Deref, ptr};

use super::{DeviceList, Frame, Listener};
use raw;

pub struct Controller {
    raw: *mut raw::Controller,
}

impl Controller {
    pub fn new() -> Controller {
        unsafe { Self::from_raw(raw::lm_controller_new()) }
    }

    pub unsafe fn from_raw(raw: *mut raw::Controller) -> Controller {
        Controller { raw }
    }

    pub unsafe fn from_raw_ref(raw: *const raw::Controller) -> Ref {
        Ref::from_raw(raw)
    }

    pub fn with_listener<L: Listener + Send>(listener: L) -> Controller {
        unsafe { Self::from_raw(raw::lm_controller_with_listener(listener.into())) }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { raw::lm_controller_is_connected(self.raw) }
    }

    pub fn frame(&self) -> Frame {
        unsafe { Frame::from_raw(raw::lm_controller_frame(self.raw)) }
    }

    pub fn devices(&self) -> DeviceList {
        unsafe { DeviceList::from_raw(raw::lm_controller_devices(self.raw)) }
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

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Ref {
    inner: Controller,
}

impl Ref {
    fn from_raw(raw: *const raw::Controller) -> Ref {
        unsafe {
            Ref {
                inner: Controller::from_raw(raw as *mut _),
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
