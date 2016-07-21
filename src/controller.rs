use raw;
use frame::Frame;

pub struct Controller {
    raw: *mut raw::Controller
}

impl Controller {
    pub fn new() -> Controller {
        unsafe {
            Controller {
                raw: raw::lm_controller_new()
            }
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
            raw::lm_controller_delete(self.raw);
        }
    }
}

trait Listener {
    fn on_connect(&mut self, &Controller) {}
    fn on_init(&mut self, &Controller) {}
    fn on_exit(&mut self, &Controller) {}
    fn on_device_change(&mut self, &Controller) {}
    fn on_device_failure(&mut self, &Controller) {}
    fn on_disconnect(&mut self, &Controller) {}
    fn on_focus_gained(&mut self, &Controller) {}
    fn on_focus_lost(&mut self, &Controller) {}
    fn on_frame(&mut self, &Controller) {}
    fn on_images(&mut self, &Controller) {}
    // fn on_log_message(&mut self, &Controller, MessageSeverity, Timestamp, message: String) {}
    fn on_service_change(&mut self, Controller) {}
    fn on_service_connect(&mut self, Controller) {}
    fn on_service_disconnect(&mut self, Controller) {}
}
