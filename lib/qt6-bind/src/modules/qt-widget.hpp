#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to the C++ implementation
typedef struct QtWidgetHandle QtWidgetHandle;

/**
 * @brief Creates a new Qt widget handle.
 */
QtWidgetHandle* create_qt_widget();

/**
 * @brief Sets the title of the widget.
 */
void set_widget_title(QtWidgetHandle* handle, const char* title);

/**
 * @brief Shows the widget.
 */
void show_qt_widget(QtWidgetHandle* handle);

/**
 * @brief Hides the widget.
 */
void hide_qt_widget(QtWidgetHandle* handle);

/**
 * @brief Cleans up all resources associated with the widget handle.
 */
void cleanup_qt_widget(QtWidgetHandle* handle);

#ifdef __cplusplus
}
#endif