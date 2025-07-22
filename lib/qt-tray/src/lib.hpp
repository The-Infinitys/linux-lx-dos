#ifndef LIB_HPP
#define LIB_HPP

#include "modules/tray.hpp"

extern "C" {
    // Function to create a new QtTray instance
    qt_tray::QtTray* create_qt_tray();

    // Function to destroy a QtTray instance
    void destroy_qt_tray(qt_tray::QtTray* tray);

    // Functions to interact with the QtTray instance
    void qt_tray_set_icon(qt_tray::QtTray* tray, const char* icon_path);
    void qt_tray_set_tool_tip(qt_tray::QtTray* tray, const char* tool_tip);

    // Menu related functions
    void qt_tray_add_menu_item(qt_tray::QtTray* tray, const char* text, void (*callback)(void*), void* user_data);

    // Event handling
    void qt_tray_on_activated(qt_tray::QtTray* tray, void (*callback)(int, void*), void* user_data);
}

#endif // LIB_HPP
