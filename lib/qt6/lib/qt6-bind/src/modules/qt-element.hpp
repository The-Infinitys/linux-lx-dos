/// src/modules/qt-element.hpp
#pragma once

#include <stddef.h> // For size_t

// --- C-style API declarations (visible to both C and C++) ---
// These enums and structs define the public C API interface.

/**
 * @brief Defines the types of UI elements available for the C API.
 */
enum QtElementType
{
    QtElementType_Button,
    QtElementType_Label,
    QtElementType_LineEdit,
    QtElementType_Widget,
    // Add other element types as needed
};

/**
 * @brief Defines the types of events that can originate from a UI element for the C API.
 */
enum QtElementEventType
{
    QtElementEventType_None,
    QtElementEventType_Clicked,        // For buttons
    QtElementEventType_TextChanged,    // For LineEdit
    QtElementEventType_EditingFinished // For LineEdit
    // Add other event types as needed
};

/**
 * @brief Structure to hold UI element event data for the C API.
 */
typedef struct
{
    enum QtElementEventType type;
    const char *element_id_str; // Unique ID of the element that emitted the event
    const char *data_str;       // Additional data related to the event (e.g., LineEdit text)
} QtElementEvent;

// Opaque pointer for QtElementHandle.
// The actual C++ class definition is in qt-element.cpp.
typedef struct QtElementHandle QtElementHandle;


// --- C-style API function declarations (visible to both C and C++) ---
extern "C"
{
    /**
     * @brief Creates a new Qt UI element handle of the specified type.
     *
     * @param type The type of element to create.
     * @param id_str A unique identifier string for the element.
     * @return A pointer to the created QtElementHandle.
     */
    QtElementHandle *create_qt_element(enum QtElementType type, const char *id_str);

    /**
     * @brief Sets the text of a UI element (e.g., button text, label text).
     *
     * @param handle The element handle.
     * @param text The text string to set.
     */
    void set_element_text(QtElementHandle *handle, const char *text);

    /**
     * @brief Sets the size of a UI element.
     *
     * @param handle The element handle.
     * @param width The width of the element.
     * @param height The height of the element.
     */
    void set_element_size(QtElementHandle *handle, int width, int height);

    /**
     * @brief Sets the enabled/disabled state of a UI element.
     *
     * @param handle The element handle.
     * @param enabled True to enable the element, false to disable.
     */
    void set_element_enabled(QtElementHandle *handle, bool enabled);

    /**
     * @brief Polls for the next event from the UI element.
     *
     * @param handle The element handle.
     * @return The next QtElementEvent. Returns QtElementEventType_None if no event.
     */
    QtElementEvent poll_element_event(QtElementHandle *handle);

    /**
     * @brief Returns the underlying QWidget pointer for a given element handle.
     * This allows other C++ modules (like qt-window.cpp) to interact with the QWidget.
     *
     * @param handle The element handle.
     * @return A pointer to the QWidget, or nullptr if invalid.
     */
    void* get_qt_element_widget(QtElementHandle *handle); // Returns void* to avoid Qt include in HPP

    /**
     * @brief Frees a character pointer allocated on the C++ side.
     * Used for all C++-allocated strings passed to Rust.
     * @param ptr The pointer to free.
     */
    void free_char_ptr(const char *ptr);

    /**
     * @brief Cleans up all resources associated with the handle.
     *
     * @param handle The element handle.
     */
    void cleanup_qt_element(QtElementHandle *handle);

} // extern "C"
