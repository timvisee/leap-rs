use raw;

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
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            raw::lm_controller_delete(self.raw);
        }
    }
}
