use raw;

pub struct PointableList {
    raw: *mut raw::PointableList
}

impl PointableList {
    pub unsafe fn from_raw(raw: *mut raw::PointableList) -> PointableList {
        PointableList {
            raw: raw
        }
    }

    pub fn count(&self) -> usize {
        unsafe {
            raw::lm_pointable_list_count(self.raw) as usize
        }
    }
}

impl Drop for PointableList {
    fn drop(&mut self) {
        unsafe {
            raw::lm_pointable_list_delete(self.raw);
        }
    }
}