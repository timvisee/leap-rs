#include <Leap.h>
#include "controller.h"
#include "frame.h"

class ListenerWrapper: public Leap::Listener {
private:
    LM_FFIListener listener;

public:
    ListenerWrapper(LM_FFIListener listener) {
        this->listener = listener;
    }

    void onExit(const Leap::Controller& controller) {
        this->listener.on_exit(this->listener.handle, controller);
    }

    void onConnect(const Leap::Controller& controller) {
        this->listener.on_connect(this->listener.handle, controller);
    }

    void onFrame(const Leap::Controller& controller) {
        this->listener.on_frame(this->listener.handle, controller);
    }

    void onInit(const Leap::Controller& controller) {
        this->listener.on_init(this->listener.handle, controller);
    }

    void onDeviceChange(const Leap::Controller& controller) {
        this->listener.on_device_change(this->listener.handle, controller);
    }

    void onDeviceFailure(const Leap::Controller& controller) {
        this->listener.on_device_failure(this->listener.handle, controller);
    }

    void onDisconnect(const Leap::Controller& controller) {
        this->listener.on_disconnect(this->listener.handle, controller);
    }
};

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
