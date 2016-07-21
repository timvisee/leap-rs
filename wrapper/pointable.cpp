#include <cstdint>
#include <Leap.h>
#include "pointable.h"
#include "vector.h"

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

    int lm_pointable_list_count(LM_PointableList self) {
        return self->count();
    }

    bool lm_pointable_list_is_empty(LM_PointableList self) {
        return self->isEmpty();
    }

    LM_Pointable lm_pointable_list_frontmost(LM_PointableList self) {
        return new Leap::Pointable(self->frontmost());
    }

    void lm_pointable_list_delete(LM_PointableList self) {
        delete self;
    }

    LM_Pointable lm_pointable_list_at(LM_PointableList self, int index) {
        return new Leap::Pointable((*self)[index]);
    }
}
