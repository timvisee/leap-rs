use raw;

pub struct Vector {
    raw: *mut raw::Vector
}

impl Vector {
    pub unsafe fn from_raw(raw: *mut raw::Vector) -> Vector {
        Vector {
            raw: raw
        }
    }

    pub fn x(&self) -> f32 {
        unsafe {
            raw::lm_vector_x(self.raw)
        }
    }

    pub fn y(&self) -> f32 {
        unsafe {
            raw::lm_vector_y(self.raw)
        }
    }

    pub fn z(&self) -> f32 {
        unsafe {
            raw::lm_vector_z(self.raw)
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
