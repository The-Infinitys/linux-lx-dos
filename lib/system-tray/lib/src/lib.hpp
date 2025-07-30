#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct SystemTray SystemTray;

    SystemTray* system_tray_new(const char* name, const char* id);
    void system_tray_delete(SystemTray* tray);
    void system_tray_run(SystemTray* tray);
    void system_tray_exit();
    void system_tray_set_icon(SystemTray* tray, const unsigned char* data, size_t len, const char* format);
    void system_tray_add_menu_item(SystemTray* tray, const char* text, void (*callback)(void*), void* user_data);

#ifdef __cplusplus
}
#endif