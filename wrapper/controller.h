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
    };
}

#endif
