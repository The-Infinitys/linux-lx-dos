#pragma once

#include "types.hpp"
#include "events.hpp"

extern "C" {
    QtAppHandle* create_qt_app();
    void set_app_id(QtAppHandle* handle, const char* id);
    void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format);
    int run_qt_app(QtAppHandle* handle, int argc, char* argv[]);
    AppEvent poll_event(QtAppHandle* handle);
    void init_tray_icon(QtAppHandle* handle);
    void add_tray_menu_item(QtAppHandle* handle, const char* text, const char* id);
    void quit_qt_app(QtAppHandle* handle);
    void cleanup_qt_app(QtAppHandle* handle);
}
