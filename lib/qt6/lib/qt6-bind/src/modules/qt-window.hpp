#pragma once

#include <cstddef> // For size_t

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer for QtWindowWrapper
struct QtWindowHandle;

// Event types for QtWindow
typedef enum {
    QtWindowEvent_None = 0,
    QtWindowEvent_Closed,
} QtWindowEventType;

// Structure to hold window event data
typedef struct {
    QtWindowEventType type;
} WindowEvent;

// C-style API for QtWindow
QtWindowHandle* create_qt_window(const char* title, int width, int height);
void show_qt_window(QtWindowHandle* handle);
void add_widget_to_window(QtWindowHandle* window_handle, void* element_handle);
WindowEvent poll_window_event(QtWindowHandle* handle);
void cleanup_qt_window(QtWindowHandle* handle);
int is_qt_window_valid(QtWindowHandle* handle);

#ifdef __cplusplus
}
#endif
