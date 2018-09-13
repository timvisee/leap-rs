#include <cstdint>
#include <Leap.h>
#include "pointable.h"
#include "vector.h"
#include "list.h"

extern "C" {
    int32_t lm_pointable_id(LM_Pointable self) {
        return self->id();
    }

    float lm_pointable_touch_distance(LM_Pointable self) {
        return self->touchDistance();
    }

    LM_Vector lm_pointable_stabilized_tip_position(LM_Pointable self) {
        return new Leap::Vector(self->stabilizedTipPosition());
    }

    void lm_pointable_delete(LM_Pointable self) {
        delete self;
    }

    bool lm_pointable_is_extended(LM_Pointable self) {
        return self->isExtended();
    }

    LM_PointableList lm_pointable_list_extended(LM_PointableList self) {
        return new Leap::PointableList(self->extended());
    }

    LM_LIST_IMPL(Pointable, pointable)
    LM_LIST_SPATIAL_IMPL(Pointable, pointable)
}
