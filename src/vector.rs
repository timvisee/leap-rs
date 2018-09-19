use std::ops::{Mul, MulAssign};

use raw;

pub struct Vector {
    raw: *mut raw::Vector,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        unsafe { Self::from_raw(raw::lm_vector_new(x, y, z)) }
    }

    pub unsafe fn from_raw(raw: *mut raw::Vector) -> Vector {
        Vector { raw }
    }

    pub fn x(&self) -> f32 {
        unsafe { raw::lm_vector_x(self.raw) }
    }

    pub fn y(&self) -> f32 {
        unsafe { raw::lm_vector_y(self.raw) }
    }

    pub fn z(&self) -> f32 {
        unsafe { raw::lm_vector_z(self.raw) }
    }

    pub fn yaw(&self) -> f32 {
        unsafe { raw::lm_vector_yaw(self.raw) }
    }

    pub fn pitch(&self) -> f32 {
        unsafe { raw::lm_vector_pitch(self.raw) }
    }

    pub fn roll(&self) -> f32 {
        unsafe { raw::lm_vector_roll(self.raw) }
    }

    pub fn as_raw(&self) -> *mut raw::Vector {
        self.raw
    }
}

impl<'a> Mul<f32> for &'a Vector {
    type Output = Vector;
    fn mul(self, factor: f32) -> Self::Output {
        unsafe { Vector::from_raw(raw::lm_vector_mul(self.raw, factor)) }
    }
}

impl<'a> Mul<&'a Vector> for f32 {
    type Output = Vector;
    fn mul(self, vector: &'a Vector) -> Self::Output {
        unsafe { Vector::from_raw(raw::lm_vector_mul(vector.raw, self)) }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, factor: f32) {
        unsafe {
            raw::lm_vector_mul_assign(self.raw, factor);
        }
    }
}

impl Drop for Vector {
    fn drop(&mut self) {
        unsafe {
            raw::lm_vector_delete(self.raw);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn mul() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = &v * 2.0;

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);

        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vector::new(1.0, 2.0, 3.0);

        v *= 2.0;

        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 4.0);
        assert_eq!(v.z(), 6.0);
    }
}
