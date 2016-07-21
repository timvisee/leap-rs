#include <Leap.h>
#include "controller.h"
#include "frame.h"

extern "C" {
    LM_Controller lm_controller_new() {
        return new Leap::Controller();
    }

    bool lm_controller_is_connected(LM_Controller self) {
        return self->isConnected();
    }

    void lm_controller_delete(LM_Controller self) {
        delete self;
    }

    LM_Frame lm_controller_frame(LM_Controller self) {
        return new Leap::Frame(self->frame());
    }
}
