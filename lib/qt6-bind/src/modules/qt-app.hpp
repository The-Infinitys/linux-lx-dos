#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to the C++ implementation
typedef struct QtAppHandle QtAppHandle;

// Enum for event types that can be polled from Rust
enum class AppEventType {
    None,
    TrayClicked,
    TrayDoubleClicked,
    MenuExitClicked
};

// Struct to hold event data
typedef struct {
    AppEventType type;
} AppEvent;

/**
 * @brief Creates a new Qt application handle.
 */
QtAppHandle* create_qt_app();

/**
 * @brief Sets the application ID.
 */
void set_app_id(QtAppHandle* handle, const char* id);

/**
 * @brief Sets the application icon from raw binary data.
 *
 * @param handle The application handle.
 * @param data Pointer to the raw icon data.
 * @param size The size of the data in bytes.
 * @param format The format of the icon data (e.g., "PNG", "JPG", "SVG").
 */
void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format);

/**
 * @brief Initializes the system tray icon with a menu.
 */
void init_tray(QtAppHandle* handle);

/**
 * @brief Runs the Qt application event loop.
 * This is a blocking call that starts the Qt event loop.
 * It should be called from the thread intended to be the Qt GUI thread.
 */
int run_qt_app(QtAppHandle* handle, int argc, char* argv[]);

/**
 * @brief Quits the Qt application event loop.
 * This can be called from any thread to signal the Qt event loop to exit.
 */
void quit_qt_app(QtAppHandle* handle);

/**
 * @brief Polls for the next event from the Qt application.
 */
AppEvent poll_event(QtAppHandle* handle);

/**
 * @brief Cleans up all resources associated with the handle.
 */
void cleanup_qt_app(QtAppHandle* handle);

#ifdef __cplusplus
}
#endif