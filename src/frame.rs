use raw;

pub struct Frame {
    raw: *mut raw::Frame
}

impl Frame {
    pub fn from_raw(raw: *mut raw::Frame) -> Frame {
        Frame {
            raw: raw
        }
    }

    pub fn current_fps(&self) -> f32 {
        unsafe {
            raw::lm_frame_current_frames_per_second(self.raw)
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            raw::lm_frame_delete(self.raw);
        }
    }
}
