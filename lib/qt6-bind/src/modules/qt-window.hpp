#pragma once

#include <stddef.h>
#include "qt-element.hpp"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct QtWindowHandle QtWindowHandle;

QtWindowHandle* create_qt_window(const char* title, int width, int height);
void cleanup_qt_window(QtWindowHandle* handle);
void show_qt_window(QtWindowHandle* handle);
void add_widget_to_window(QtWindowHandle* window_handle, QtElementHandle* element_handle);

#ifdef __cplusplus
}
#endif