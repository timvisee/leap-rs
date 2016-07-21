#include <cstdint>
#include <Leap.h>
#include "pointable.h"

extern "C" {
    int32_t lm_pointable_id(LM_Pointable self) {
        return self->id();
    }

    float lm_pointable_touch_distance(LM_Pointable self) {
        return self->touchDistance();
    }

    int lm_pointable_list_count(LM_PointableList self) {
        return self->count();
    }

    void lm_pointable_list_delete(LM_PointableList self) {
        delete self;
    }

    LM_Pointable lm_pointable_list_at(LM_PointableList self, int index) {
        return new Leap::Pointable((*self)[index]);
    }
}
