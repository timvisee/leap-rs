#include <Leap.h>
#include "controller.h"
#include "frame.h"
#include "listener.h"

extern "C" {
    LM_Controller lm_controller_new() {
        return new Leap::Controller();
    }

    LM_Controller lm_controller_with_listener(LM_FFIListener raw_listener) {
        ListenerWrapper* listener = new ListenerWrapper(raw_listener);
        return new Leap::Controller(*listener);
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
