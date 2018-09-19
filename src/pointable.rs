use std::os::raw::c_int;

use super::Vector;
use raw;

pub struct Pointable {
    raw: *mut raw::Pointable,
}

impl Pointable {
    pub unsafe fn from_raw(raw: *mut raw::Pointable) -> Pointable {
        Pointable { raw }
    }

    /// returned id might be negative
    pub fn id(&self) -> i32 {
        unsafe { raw::lm_pointable_id(self.raw) }
    }

    pub fn touch_distance(&self) -> f32 {
        unsafe { raw::lm_pointable_touch_distance(self.raw) }
    }

    pub fn stabilized_tip_position(&self) -> Vector {
        unsafe { Vector::from_raw(raw::lm_pointable_stabilized_tip_position(self.raw)) }
    }

    pub fn is_extended(&self) -> bool {
        unsafe { raw::lm_pointable_is_extended(self.raw) }
    }
}

impl Drop for Pointable {
    fn drop(&mut self) {
        unsafe {
            raw::lm_pointable_delete(self.raw);
        }
    }
}

pub struct PointableList {
    raw: *mut raw::PointableList,
}

impl PointableList {
    pub unsafe fn from_raw(raw: *mut raw::PointableList) -> PointableList {
        PointableList { raw }
    }

    pub fn len(&self) -> usize {
        unsafe { raw::lm_pointable_list_count(self.raw) as usize }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { raw::lm_pointable_list_is_empty(self.raw) }
    }

    pub fn extended(&self) -> PointableList {
        unsafe { PointableList::from_raw(raw::lm_pointable_list_extended(self.raw)) }
    }

    pub fn frontmost(&self) -> Option<Pointable> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Pointable::from_raw(raw::lm_pointable_list_frontmost(
                    self.raw,
                )))
            }
        }
    }

    pub fn leftmost(&self) -> Option<Pointable> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Pointable::from_raw(raw::lm_pointable_list_leftmost(
                    self.raw,
                )))
            }
        }
    }

    pub fn rightmost(&self) -> Option<Pointable> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Pointable::from_raw(raw::lm_pointable_list_rightmost(
                    self.raw,
                )))
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Pointable> {
        unsafe {
            if index < self.len() {
                Some(Pointable::from_raw(raw::lm_pointable_list_at(
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

impl Drop for PointableList {
    fn drop(&mut self) {
        unsafe {
            raw::lm_pointable_list_delete(self.raw);
        }
    }
}

pub struct Iter<'a> {
    list: &'a PointableList,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Pointable;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pointable) = self.list.get(self.index) {
            self.index += 1;
            Some(pointable)
        } else {
            None
        }
    }
}
