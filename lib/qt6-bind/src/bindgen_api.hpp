#pragma once

#include <stddef.h> // For size_t

// Opaque pointers for Qt handles
typedef struct QtAppHandle QtAppHandle;
typedef struct QtWindowHandle QtWindowHandle;
typedef struct QtElementHandle QtElementHandle;

// Enums for event types and element types
typedef enum AppEventType {
    AppEventType_None,
    AppEventType_TrayClicked,
    AppEventType_TrayDoubleClicked,
    AppEventType_MenuItemClicked
} AppEventType;

typedef struct {
    AppEventType type;
    const char* menu_id_str;
} AppEvent;

typedef enum QtWindowEventType {
    QtWindowEvent_None,
    QtWindowEvent_Closed
} QtWindowEventType;

typedef struct {
    QtWindowEventType type;
} WindowEvent;

typedef enum QtElementType {
    QtElementType_Button,
    QtElementType_Label,
    QtElementType_LineEdit
} QtElementType;

typedef enum QtElementEventType {
    QtElementEventType_None,
    QtElementEventType_Clicked,
    QtElementEventType_TextChanged,
    QtElementEventType_EditingFinished
} QtElementEventType;

typedef struct {
    QtElementEventType type;
    const char* element_id_str;
    const char* data_str;
} QtElementEvent;

// C-style API for QtApp
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

    // C-style API for QtWindow (async versions)
    void create_qt_window_async(const char* title, int width, int height, void (*callback)(QtWindowHandle*, void*), void* user_data);
    void show_qt_window_async(QtWindowHandle* handle);
    void add_widget_to_window_async(QtWindowHandle* window_handle, QtElementHandle* element_handle);
    WindowEvent poll_window_event(QtWindowHandle* handle);
    void close_qt_window_async(QtWindowHandle* handle);
    void cleanup_qt_window(QtWindowHandle* handle);
    void set_window_close_callback(QtWindowHandle* handle, void (*callback)(void*), void* user_data);
    void refresh_qt_window_async(QtWindowHandle* handle);

    // C-style API for QtElement (async versions)
    void create_qt_element_async(int element_type, const char* id_str, void (*callback)(QtElementHandle*, void*), void* user_data);
    void set_element_text_async(QtElementHandle* element_handle, const char* text);
    void set_element_size_async(QtElementHandle* element_handle, int width, int height);
    void set_element_enabled_async(QtElementHandle* element_handle, bool enabled);
    QtElementEvent poll_element_event(QtElementHandle* handle);
    void cleanup_qt_element(QtElementHandle* handle);
}