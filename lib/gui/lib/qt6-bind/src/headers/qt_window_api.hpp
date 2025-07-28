#pragma once

#include "types.hpp"
#include "events.hpp"

extern "C" {
    void create_qt_window_async(const char* title, int width, int height, void (*callback)(QtWindowHandle*, void*), void* user_data);
    void show_qt_window_async(QtWindowHandle* handle);
    void add_widget_to_window_async(QtWindowHandle* window_handle, QtElementHandle* element_handle);
    WindowEvent poll_window_event(QtWindowHandle* handle);
    void close_qt_window_async(QtWindowHandle* handle);
    void cleanup_qt_window(QtWindowHandle* handle);
    void set_window_close_callback(QtWindowHandle* handle, void (*callback)(void*), void* user_data);
    void refresh_qt_window_async(QtWindowHandle* handle);
}
