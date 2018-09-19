use super::{FingerList, HandList, InteractionBox, PointableList};
use raw;

pub struct Frame {
    raw: *mut raw::Frame,
}

impl Frame {
    pub unsafe fn from_raw(raw: *mut raw::Frame) -> Frame {
        Frame { raw }
    }

    pub fn current_fps(&self) -> f32 {
        unsafe { raw::lm_frame_current_frames_per_second(self.raw) }
    }

    pub fn fingers(&self) -> FingerList {
        unsafe { FingerList::from_raw(raw::lm_frame_fingers(self.raw)) }
    }

    pub fn pointables(&self) -> PointableList {
        unsafe { PointableList::from_raw(raw::lm_frame_pointables(self.raw)) }
    }

    pub fn hands(&self) -> HandList {
        unsafe { HandList::from_raw(raw::lm_frame_hands(self.raw)) }
    }

    pub fn interaction_box(&self) -> InteractionBox {
        unsafe { InteractionBox::from_raw(raw::lm_frame_interaction_box(self.raw)) }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            raw::lm_frame_delete(self.raw);
        }
    }
}
