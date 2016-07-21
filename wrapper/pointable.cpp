#include <Leap.h>
#include "pointable.h"

extern "C" {
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
