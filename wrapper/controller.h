#ifndef LM_CONTROLLER_H
#define LM_CONTROLLER_H
#include <Leap.h>

extern "C" {
    typedef Leap::Controller* LM_Controller;
    typedef void* LM_Listener;
    typedef void (*LM_ListenerHandler)(LM_Listener, const Leap::Controller &);

    // KEEP IN PERFECT SYNC WITH FFIListener from src/raw/controller.rs
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

#endif
