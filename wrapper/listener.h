#ifndef LM_LISTENER_H
#define LM_LISTENER_H
#include <Leap.h>

extern "C" {
    typedef void* LM_Listener;
    typedef void (*LM_ListenerHandler)(LM_Listener, const Leap::Controller &);

    // KEEP IN PERFECT SYNC WITH FFIListener from src/raw/listener.rs
    struct LM_FFIListener {
        LM_Listener handle;
        LM_ListenerHandler on_exit;
        LM_ListenerHandler on_connect;
        LM_ListenerHandler on_frame;
        LM_ListenerHandler on_init;
        LM_ListenerHandler on_device_change;
        LM_ListenerHandler on_device_failure;
        LM_ListenerHandler on_disconnect;
        LM_ListenerHandler on_focus_gained;
        LM_ListenerHandler on_focus_lost;
        LM_ListenerHandler on_images;
        LM_ListenerHandler on_service_change;
        LM_ListenerHandler on_service_connect;
        LM_ListenerHandler on_service_disconnect;
    };
}

class ListenerWrapper: public Leap::Listener {
private:
    LM_FFIListener listener;

public:
    ListenerWrapper(LM_FFIListener);
    void onExit(const Leap::Controller& controller);
    void onConnect(const Leap::Controller& controller);
    void onFrame(const Leap::Controller& controller);
    void onInit(const Leap::Controller& controller);
    void onDeviceChange(const Leap::Controller& controller);
    void onDeviceFailure(const Leap::Controller& controller);
    void onDisconnect(const Leap::Controller& controller);
    void onFocusGained(const Leap::Controller& controller);
    void onFocusLost(const Leap::Controller& controller);
    void onImages(const Leap::Controller& controller);
    void onServiceChange(const Leap::Controller& controller);
    void onServiceConnect(const Leap::Controller& controller);
    void onServiceDisconnect(const Leap::Controller& controller);
};

#endif
