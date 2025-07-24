#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to the C++ implementation
typedef struct QtTrayHandle QtTrayHandle;

// Enum for event types that can be polled from Rust
enum class AppEventType {
    None,
    TrayClicked,
    TrayDoubleClicked,
    MenuItemClicked
};

// Struct to hold event data
typedef struct {
    AppEventType type;
    const char* menu_id_str; // For MenuItemClicked events, now a string
} AppEvent;

/**
 * @brief Creates a new Qt tray handle.
 */
QtTrayHandle* create_qt_tray();

/**
 * @brief Initializes the system tray icon with a menu.
 */
void init_tray(QtTrayHandle* handle, const unsigned char* icon_data, size_t icon_size, const char* icon_format);

/**
 * @brief Polls for the next event from the Qt tray.
 */
AppEvent poll_event(QtTrayHandle* handle);

/**
 * @brief Adds a menu item to the system tray icon's context menu.
 *
 * @param handle The tray handle.
 * @param text The text to display for the menu item.
 * @param id A unique string ID for the menu item, used to identify clicks.
 */
void add_tray_menu_item(QtTrayHandle* handle, const char* text, const char* id);

/**
 * @brief Cleans up all resources associated with the tray handle.
 */
void cleanup_qt_tray(QtTrayHandle* handle);

/**
 * @brief Frees a character pointer allocated by the C++ side.
 */
void free_char_ptr(const char* ptr);

#ifdef __cplusplus
}
#endif