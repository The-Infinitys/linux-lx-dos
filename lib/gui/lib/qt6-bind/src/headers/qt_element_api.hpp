#pragma once

#include "types.hpp"
#include "events.hpp"

extern "C" {
    void create_qt_element_async(int element_type, const char* id_str, void (*callback)(QtElementHandle*, void*), void* user_data);
    void set_element_text_async(QtElementHandle* element_handle, const char* text);
    void set_element_size_async(QtElementHandle* element_handle, int width, int height);
    void set_element_enabled_async(QtElementHandle* element_handle, bool enabled);
    QtElementEvent poll_element_event(QtElementHandle* handle);
    void cleanup_qt_element(QtElementHandle* handle);
    void add_child_element_to_element(QtElementHandle* parent_handle, QtElementHandle* child_handle);
}
