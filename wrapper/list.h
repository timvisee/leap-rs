#ifndef LM_LIST_H
#define LM_LIST_H

#define LM_LIST_IMPL(TYPE, IDENT) \
    extern "C" {                                                     \
        int lm_##IDENT##_list_count(LM_##TYPE##List self) { \
            return self->count(); \
        } \
        \
        bool lm_##IDENT##_list_is_empty(LM_##TYPE##List self) { \
            return self->isEmpty(); \
        } \
        \
        void lm_##IDENT##_list_delete(LM_##TYPE##List self) { \
            delete self; \
        } \
        \
        LM_##TYPE lm_##IDENT##_list_at(LM_##TYPE##List self, int index) { \
            return new Leap::TYPE((*self)[index]); \
        } \
    }

#define LM_LIST_SPATIAL_IMPL(TYPE, IDENT) \
    extern "C" {                                                     \
        LM_##TYPE lm_##IDENT##_list_frontmost(LM_##TYPE##List self) { \
            return new Leap::TYPE(self->frontmost()); \
        } \
        \
        LM_##TYPE lm_##IDENT##_list_leftmost(LM_##TYPE##List self) { \
            return new Leap::TYPE(self->leftmost()); \
        } \
        \
        LM_##TYPE lm_##IDENT##_list_rightmost(LM_##TYPE##List self) { \
            return new Leap::TYPE(self->rightmost()); \
        } \
    }

#define LM_LIST_TIPPED_IMPL(TYPE, IDENT) \
    extern "C" {                                                     \
        int32_t lm_##IDENT##_id(LM_##TYPE self) { \
            return self->id(); \
        } \
        \
        float lm_##IDENT##_touch_distance(LM_##TYPE self) { \
            return self->touchDistance(); \
        } \
        \
        void lm_##IDENT##_delete(LM_##TYPE self) { \
            delete self; \
        } \
        \
        LM_Vector lm_##IDENT##_stabilized_tip_position(LM_##TYPE self) { \
            return new Leap::Vector(self->stabilizedTipPosition()); \
        } \
        \
        bool lm_##IDENT##_is_extended(LM_##TYPE self) { \
            return self->isExtended(); \
        } \
        \
        LM_##TYPE##List lm_##IDENT##_list_extended(LM_##TYPE##List self) { \
            return new Leap::TYPE##List(self->extended()); \
        } \
    }

#endif
