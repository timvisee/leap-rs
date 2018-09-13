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

#endif
