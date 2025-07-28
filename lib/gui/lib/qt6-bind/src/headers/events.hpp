#pragma once

#include <stddef.h> // For size_t
#include "enums.hpp"

typedef struct {
    AppEventType type;
    const char* menu_id_str;
} AppEvent;

typedef struct {
    QtWindowEventType type;
} WindowEvent;

typedef struct {
    QtElementEventType type;
    const char* element_id_str;
    const char* data_str;
} QtElementEvent;
