#include <Leap.h>
#include "frame.h"
#include "finger.h"
#include "pointable.h"
#include "hand.h"
#include "interaction_box.h"

extern "C" {
    LM_Frame lm_frame_delete(LM_Frame self) {
        delete self;
    }

    float lm_frame_current_frames_per_second(LM_Frame self) {
        return self->currentFramesPerSecond();
    }

    LM_FingerList lm_frame_fingers(LM_Frame self) {
        return new Leap::FingerList(self->fingers());
    }

    LM_PointableList lm_frame_pointables(LM_Frame self) {
        return new Leap::PointableList(self->pointables());
    }

    LM_HandList lm_frame_hands(LM_Frame self) {
        return new Leap::HandList(self->hands());
    }

    LM_InteractionBox lm_frame_interaction_box(LM_Frame self) {
        return new Leap::InteractionBox(self->interactionBox());
    }
}
