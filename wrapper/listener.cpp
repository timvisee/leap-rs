#include <Leap.h>
#include "listener.h"

#define IMPL_LISTENER(NAME, FFI_NAME) \
    void ListenerWrapper::NAME(const Leap::Controller& controller) { \
        this->listener.FFI_NAME(this->listener.handle, controller); \
    }

ListenerWrapper::ListenerWrapper(LM_FFIListener listener) {
    this->listener = listener;
}

IMPL_LISTENER(onExit, on_exit)
IMPL_LISTENER(onConnect, on_connect)
IMPL_LISTENER(onFrame, on_frame)
IMPL_LISTENER(onInit, on_init)
IMPL_LISTENER(onDeviceChange, on_device_change)
IMPL_LISTENER(onDeviceFailure, on_device_failure)
IMPL_LISTENER(onDisconnect, on_disconnect)
IMPL_LISTENER(onFocusGained, on_focus_gained)
IMPL_LISTENER(onFocusLost, on_focus_lost)
IMPL_LISTENER(onImages, on_images)
IMPL_LISTENER(onServiceChange, on_service_change)
IMPL_LISTENER(onServiceConnect, on_service_connect)
IMPL_LISTENER(onServiceDisconnect, on_disconnect)
