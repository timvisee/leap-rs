use super::Vector;

pub enum InteractionBox {}

extern "C" {
    pub fn lm_interaction_box_normalize_point(
        this: *mut InteractionBox,
        position: *const Vector,
        clamp: bool,
    ) -> *mut Vector;
    pub fn lm_interaction_box_width(this: *const InteractionBox) -> f32;
    pub fn lm_interaction_box_height(this: *const InteractionBox) -> f32;
    pub fn lm_interaction_box_depth(this: *const InteractionBox) -> f32;
    pub fn lm_interaction_box_delete(this: *mut InteractionBox);
}
