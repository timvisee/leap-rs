#include <Leap.h>
#include "vector.h"

extern "C" {
    LM_Vector lm_vector_new(float x, float y, float z) {
        return new Leap::Vector(x, y, z);
    }

    float lm_vector_x(LM_Vector self) {
        return self->x;
    }

    float lm_vector_y(LM_Vector self) {
        return self->y;
    }

    float lm_vector_z(LM_Vector self) {
        return self->z;
    }

    LM_Vector lm_vector_mul(const LM_Vector self, float factor) {
        return new Leap::Vector(*self * factor);
    }

    void lm_vector_mul_assign(LM_Vector self, float factor) {
        *self *= factor;
    }

    void lm_vector_delete(LM_Vector self) {
        delete self;
    }
}
