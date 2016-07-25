#include <Leap.h>
#include "interaction_box.h"
#include "vector.h"

extern "C" {
    LM_Vector lm_interaction_box_normalize_point(LM_InteractionBox self, const LM_Vector position, bool clamp) {
        return new Leap::Vector(self->normalizePoint(*position, clamp));
    }

    float lm_interaction_box_width(LM_InteractionBox self) {
        return self->width();
    }

    float lm_interaction_box_height(LM_InteractionBox self) {
        return self->height();
    }

    float lm_interaction_box_depth(LM_InteractionBox self) {
        return self->depth();
    }

    LM_InteractionBox lm_interaction_box_delete(LM_InteractionBox self) {
        delete self;
    }
}
