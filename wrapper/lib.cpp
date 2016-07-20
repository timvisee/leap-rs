#include <stdio.h>
#include <Leap.h>
#include <unistd.h>

typedef Leap::Controller* LM_Controller;

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
}
