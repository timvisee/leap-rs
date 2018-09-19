use std::{
    fmt::{self, Display, Formatter},
    os::raw::c_int,
};

use super::Vector;
use raw;

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

    /// Get the finger type as enum
    pub fn type_enum(&self) -> FingerType {
        FingerType::from_id(self.type_id()).unwrap()
    }

    /// Get the finger ID
    ///
    /// The ID is defined by the Leap Motion SDK.
    /// To use a Rust enum, take a look at the `take_enum` method instead.
    pub fn type_id(&self) -> u32 {
        unsafe { raw::lm_finger_type(self.raw) }
    }

    pub fn is_extended(&self) -> bool {
        unsafe { raw::lm_finger_is_extended(self.raw) }
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

    pub fn finger_type(&self, finger_type: FingerType) -> FingerList {
        self.finger_type_id(finger_type.id())
    }

    fn finger_type_id(&self, finger_type: u32) -> FingerList {
        unsafe { FingerList::from_raw(raw::lm_finger_list_finger_type(self.raw, finger_type)) }
    }

    pub fn extended(&self) -> FingerList {
        unsafe { FingerList::from_raw(raw::lm_finger_list_extended(self.raw)) }
    }

    pub fn frontmost(&self) -> Option<Finger> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Finger::from_raw(raw::lm_finger_list_frontmost(self.raw)))
            }
        }
    }

    pub fn leftmost(&self) -> Option<Finger> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Finger::from_raw(raw::lm_finger_list_leftmost(self.raw)))
            }
        }
    }

    pub fn rightmost(&self) -> Option<Finger> {
        unsafe {
            if self.is_empty() {
                None
            } else {
                Some(Finger::from_raw(raw::lm_finger_list_rightmost(self.raw)))
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

/// Finger type
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FingerType {
    /// The thumb
    Thumb,

    /// The index or fore-finger
    Index,

    /// The middle finger
    Middle,

    /// The ring finger
    Ring,

    /// The pinky or little finger
    Pinky,
}

impl FingerType {
    /// Get the finger type from the given `id`,
    /// provided by the Leap Motion library.
    ///
    /// If the `id` is invalid, `None` is returned.
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(FingerType::Thumb),
            1 => Some(FingerType::Index),
            2 => Some(FingerType::Middle),
            3 => Some(FingerType::Ring),
            4 => Some(FingerType::Pinky),
            _ => None,
        }
    }

    /// Get the finger type ID
    pub fn id(&self) -> u32 {
        match self {
            FingerType::Thumb => 0,
            FingerType::Index => 1,
            FingerType::Middle => 2,
            FingerType::Ring => 3,
            FingerType::Pinky => 4,
        }
    }

    /// Get the lowerface finger name
    ///
    /// It will be one of:
    /// - `"thumb"`
    /// - `"index"`
    /// - `"middle"`
    /// - `"ring"`
    /// - `"pinky"`
    pub fn name(&self) -> &'static str {
        match self {
            FingerType::Thumb => "thumb",
            FingerType::Index => "index",
            FingerType::Middle => "middle",
            FingerType::Ring => "ring",
            FingerType::Pinky => "pinky",
        }
    }
}

impl Display for FingerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
