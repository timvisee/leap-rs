#ifndef LM_LIB_H
#define LM_LIB_H
#include <Leap.h>

extern "C" {
    typedef Leap::Controller* LM_Controller;
    LM_Controller lm_controller_new();
}

#endif
