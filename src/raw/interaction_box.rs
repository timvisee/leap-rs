use super::Vector;

pub enum InteractionBox {}

extern {
    pub fn lm_interaction_box_normalize_point(this: *mut InteractionBox, position: *const Vector, clamp: bool) -> *mut Vector;
    pub fn lm_interaction_box_delete(this: *mut InteractionBox);
}
