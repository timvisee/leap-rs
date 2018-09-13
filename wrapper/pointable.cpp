#include <cstdint>
#include <Leap.h>
#include "pointable.h"
#include "vector.h"
#include "list.h"

extern "C" {
    LM_LIST_IMPL(Pointable, pointable)
    LM_LIST_SPATIAL_IMPL(Pointable, pointable)
    LM_LIST_TIPPED_IMPL(Pointable, pointable)
}
