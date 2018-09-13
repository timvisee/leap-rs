use raw;
use std::os::raw::c_int;
use Vector;

pub struct Finger {
    raw: *mut raw::Finger,
}

impl Finger {
    pub unsafe fn from_raw(raw: *mut raw::Finger) -> Finger {
        Finger { raw }
    }

    /// returned id might be negative
    pub fn id(&self) -> i32 {
        unsafe { raw::lm_finger_id(self.raw) }
    }

    pub fn touch_distance(&self) -> f32 {
        unsafe { raw::lm_finger_touch_distance(self.raw) }
    }

    pub fn stabilized_tip_position(&self) -> Vector {
        unsafe { Vector::from_raw(raw::lm_finger_stabilized_tip_position(self.raw)) }
    }
}

impl Drop for Finger {
    fn drop(&mut self) {
        unsafe {
            raw::lm_finger_delete(self.raw);
        }
    }
}

pub struct FingerList {
    raw: *mut raw::FingerList,
}

impl FingerList {
    pub unsafe fn from_raw(raw: *mut raw::FingerList) -> FingerList {
        FingerList { raw }
    }

    pub fn len(&self) -> usize {
        unsafe { raw::lm_finger_list_count(self.raw) as usize }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { raw::lm_finger_list_is_empty(self.raw) }
    }

    pub fn frontmost(&self) -> Option<Finger> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Finger::from_raw(raw::lm_finger_list_frontmost(
                    self.raw,
                )))
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Finger> {
        unsafe {
            if index < self.len() {
                Some(Finger::from_raw(raw::lm_finger_list_at(
                    self.raw,
                    index as c_int,
                )))
            } else {
                None
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            list: self,
            index: 0,
        }
    }
}

impl Drop for FingerList {
    fn drop(&mut self) {
        unsafe {
            raw::lm_finger_list_delete(self.raw);
        }
    }
}

pub struct Iter<'a> {
    list: &'a FingerList,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Finger;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(finger) = self.list.get(self.index) {
            self.index += 1;
            Some(finger)
        } else {
            None
        }
    }
}
