#include <Leap.h>
#include "device.h"
#include "list.h"

extern "C" {
    void lm_device_delete(LM_Device self) {
        delete self;
    }

    float lm_device_baseline(LM_Device self) {
        return self->baseline();
    }

    float lm_device_horizontal_view_angle(LM_Device self) {
        return self->horizontalViewAngle();
    }

    bool lm_device_is_embedded(LM_Device self) {
        return self->isEmbedded();
    }

    // Blocked by 3.0 release
    // bool lm_device_is_lighting_bad(LM_Device self) {
    //     return self->isLightingBad();
    // }

    // Blocked by 3.0 release
    // bool lm_device_is_smudged(LM_Device self) {
    //     return self->isSmudged();
    // }

    bool lm_device_is_streaming(LM_Device self) {
        return self->isStreaming();
    }

    bool lm_device_is_valid(LM_Device self) {
        return self->isValid();
    }

    float lm_device_range(LM_Device self) {
        return self->range();
    }

    float lm_device_vertical_view_angle(LM_Device self) {
        return self->verticalViewAngle();
    }

    LM_LIST_IMPL(Device, device)
}
