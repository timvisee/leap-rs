#include <cstdint>
#include <Leap.h>
#include "finger.h"
#include "vector.h"
#include "list.h"

extern "C" {
    uint32_t lm_finger_type(LM_Finger self) {
        return self->type();
    }

    LM_FingerList lm_finger_list_finger_type(LM_FingerList self, uint32_t type) {
        return new Leap::FingerList(self->fingerType((Leap::Finger::Type) type));
    }

    LM_LIST_IMPL(Finger, finger)
    LM_LIST_SPATIAL_IMPL(Finger, finger)
    LM_LIST_TIPPED_IMPL(Finger, finger)
}
