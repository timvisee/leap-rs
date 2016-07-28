#include <Leap.h>
#include "device.h"
#include "list.h"

extern "C" {
    void lm_device_delete(LM_Device self) {
        delete self;
    }

    LM_LIST_IMPL(Device, device)
}
