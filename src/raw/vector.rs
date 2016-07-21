
pub enum Vector {}

extern {
    pub fn lm_vector_x(this: *mut Vector) -> f32;
    pub fn lm_vector_y(this: *mut Vector) -> f32;
    pub fn lm_vector_z(this: *mut Vector) -> f32;
    pub fn lm_vector_delete(this: *mut Vector);
}
