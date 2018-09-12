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

    // TODO: int lm_finger_list_count(LM_FingerList self) {
    // TODO:     return 5;
    // TODO: }

    void lm_finger_list_delete(LM_FingerList self) {
        delete self;
    }

    // TODO: LM_Finger _finger_list_at(LM_FingerList self, int index) {
    // TODO:     return new Leap::Finger(self[index]);
    // TODO: }

    // LM_LIST_IMPL(Finger, finger)
    // LM_LIST_SPATIAL_IMPL(Finger, finger)
}
