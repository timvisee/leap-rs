#include <cstdint>
#include <Leap.h>
#include "finger.h"
#include "vector.h"
#include "list.h"

extern "C" {
    int32_t lm_finger_id(LM_Finger self) {
        return self->id();
    }

    float lm_finger_touch_distance(LM_Finger self) {
        return self->touchDistance();
    }

    LM_Vector lm_finger_stabilized_tip_position(LM_Finger self) {
        return new Leap::Vector(self->stabilizedTipPosition());
    }

    void lm_finger_delete(LM_Finger self) {
        delete self;
    }

    LM_LIST_IMPL(Finger, finger)
    LM_LIST_SPATIAL_IMPL(Finger, finger)
}
