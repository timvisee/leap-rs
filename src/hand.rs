use raw;
use std::os::raw::c_int;
use Vector;

pub struct Hand {
    raw: *mut raw::Hand,
}

impl Hand {
    pub unsafe fn from_raw(raw: *mut raw::Hand) -> Hand {
        Hand { raw }
    }

    /// returned id might be negative
    pub fn id(&self) -> i32 {
        unsafe { raw::lm_hand_id(self.raw) }
    }

    pub fn pinch_distance(&self) -> f32 {
        unsafe { raw::lm_hand_pinch_distance(self.raw) }
    }

    pub fn stabilized_palm_position(&self) -> Vector {
        unsafe { Vector::from_raw(raw::lm_hand_stabilized_palm_position(self.raw)) }
    }

    pub fn direction(&self) -> Vector {
        unsafe { Vector::from_raw(raw::lm_hand_direction(self.raw)) }
    }
}

impl Drop for Hand {
    fn drop(&mut self) {
        unsafe {
            raw::lm_hand_delete(self.raw);
        }
    }
}

pub struct HandList {
    raw: *mut raw::HandList,
}

impl HandList {
    pub unsafe fn from_raw(raw: *mut raw::HandList) -> HandList {
        HandList { raw }
    }

    pub fn len(&self) -> usize {
        unsafe { raw::lm_hand_list_count(self.raw) as usize }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { raw::lm_hand_list_is_empty(self.raw) }
    }

    pub fn frontmost(&self) -> Option<Hand> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Hand::from_raw(raw::lm_hand_list_frontmost(self.raw)))
            }
        }
    }

    pub fn leftmost(&self) -> Option<Hand> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Hand::from_raw(raw::lm_hand_list_leftmost(self.raw)))
            }
        }
    }

    pub fn rightmost(&self) -> Option<Hand> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Hand::from_raw(raw::lm_hand_list_rightmost(self.raw)))
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Hand> {
        unsafe {
            if index < self.len() {
                Some(Hand::from_raw(raw::lm_hand_list_at(
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

impl Drop for HandList {
    fn drop(&mut self) {
        unsafe {
            raw::lm_hand_list_delete(self.raw);
        }
    }
}

pub struct Iter<'a> {
    list: &'a HandList,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Hand;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hand) = self.list.get(self.index) {
            self.index += 1;
            Some(hand)
        } else {
            None
        }
    }
}
