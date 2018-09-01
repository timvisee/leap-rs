pub enum Vector {}

extern "C" {
    pub fn lm_vector_new(x: f32, y: f32, z: f32) -> *mut Vector;
    pub fn lm_vector_x(this: *mut Vector) -> f32;
    pub fn lm_vector_y(this: *mut Vector) -> f32;
    pub fn lm_vector_z(this: *mut Vector) -> f32;
    pub fn lm_vector_yaw(this: *mut Vector) -> f32;
    pub fn lm_vector_pitch(this: *mut Vector) -> f32;
    pub fn lm_vector_roll(this: *mut Vector) -> f32;
    pub fn lm_vector_mul(this: *const Vector, factor: f32) -> *mut Vector;
    pub fn lm_vector_mul_assign(this: *mut Vector, factor: f32);
    pub fn lm_vector_delete(this: *mut Vector);
}
