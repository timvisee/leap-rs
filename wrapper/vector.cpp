#include <Leap.h>
#include "vector.h"

extern "C" {
    float lm_vector_x(LM_Vector self) {
        return self->x;
    }
    float lm_vector_y(LM_Vector self) {
        return self->y;
    }
    float lm_vector_z(LM_Vector self) {
        return self->z;
    }
    void lm_vector_delete(LM_Vector self) {
        delete self;
    }
}
