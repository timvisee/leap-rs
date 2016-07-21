#include <Leap.h>
#include "frame.h"

extern "C" {
    LM_Frame lm_frame_delete(LM_Frame self) {
        delete self;
    }

    float lm_frame_current_frames_per_second(LM_Frame self) {
        return self->currentFramesPerSecond();
    }
}
