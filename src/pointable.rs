use std::os::raw::c_int;
use raw;

struct Pointable {
    raw: *mut raw::Pointable
}

impl Pointable {
    pub unsafe fn from_raw(raw: *mut raw::Pointable) -> Pointable {
        Pointable {
            raw: raw
        }
    }

    /// returned id might be negative
    pub fn id(&self) -> i32 {
        unsafe {
            raw::lm_pointable_id(self.raw)
        }
    }

    pub fn touch_distance(&self) -> f32 {
        unsafe {
            raw::lm_pointable_touch_distance(self.raw)
        }
    }
}

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

    pub fn at(&self, index: usize) -> Option<Pointable> {
        unsafe {
            if index < self.count() {
                Some(Pointable::from_raw(raw::lm_pointable_list_at(self.raw, index as c_int)))
            }
            else {
                None
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            list: self,
            index: 0
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

struct Iter<'a> {
    list: &'a PointableList,
    index: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = Pointable;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pointable) = self.list.at(self.index) {
            self.index += 1;
            Some(pointable)
        }
        else {
            None
        }
    }
}
