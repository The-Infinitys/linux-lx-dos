#include "qt-thread-executor.hpp"
#include "modules/qt-app.hpp"
#include "modules/qt-window.hpp"
#include "modules/qt-element.hpp"
#include <QCoreApplication>
#include <QDebug>

QtThreadExecutor::QtThreadExecutor(QObject *parent) : QObject(parent) {
    // Connect the signal to the slot
    connect(this, &QtThreadExecutor::tasksAvailable, this, &QtThreadExecutor::processTasks, Qt::QueuedConnection);
}

void QtThreadExecutor::enqueue(std::function<void()> task) {
    QMutexLocker locker(&mutex);
    taskQueue.enqueue(task);
    emit tasksAvailable(); // Signal that new tasks are available
}

void QtThreadExecutor::processTasks() {
    QMutexLocker locker(&mutex);
    while (!taskQueue.isEmpty()) {
        std::function<void()> task = taskQueue.dequeue();
        locker.unlock(); // Unlock while executing the task
        task();
        locker.relock(); // Relock for the next task
    }
}

// Global instance of the executor
static QtThreadExecutor* s_qtThreadExecutor = nullptr;

extern "C" {
    QtThreadExecutor* get_qt_thread_executor() {
        if (!s_qtThreadExecutor) {
            // Ensure this is created on the main thread if possible
            s_qtThreadExecutor = new QtThreadExecutor(QCoreApplication::instance());
        }
        return s_qtThreadExecutor;
    }

    void create_qt_window_async(QtAppHandle* app_handle, const char* title, int width, int height, void (*callback)(QtWindowHandle*, void*), void* user_data) {
        get_qt_thread_executor()->enqueue([=]() {
            QtWindowHandle* window_handle = create_qt_window(app_handle, title, width, height);
            if (callback) {
                callback(window_handle, user_data);
            }
        });
    }

    void show_qt_window_async(QtWindowHandle* handle) {
        get_qt_thread_executor()->enqueue([=]() {
            show_qt_window(handle);
        });
    }

    void add_widget_to_window_async(QtWindowHandle* window_handle, QtElementHandle* element_handle) {
        get_qt_thread_executor()->enqueue([=]() {
            add_widget_to_window(window_handle, element_handle);
        });
    }

    void set_element_text_async(QtElementHandle* element_handle, const char* text) {
        get_qt_thread_executor()->enqueue([=]() {
            set_element_text(element_handle, text);
        });
    }

    void set_element_size_async(QtElementHandle* element_handle, int width, int height) {
        get_qt_thread_executor()->enqueue([=]() {
            set_element_size(element_handle, width, height);
        });
    }

    void set_element_enabled_async(QtElementHandle* element_handle, bool enabled) {
        get_qt_thread_executor()->enqueue([=]() {
            set_element_enabled(element_handle, enabled);
        });
    }

    void create_qt_element_async(QtAppHandle* app_handle, int element_type, const char* id_str, void (*callback)(QtElementHandle*, void*), void* user_data) {
        get_qt_thread_executor()->enqueue([=]() {
            QtElementHandle* element_handle = create_qt_element(app_handle, static_cast<QtElementType>(element_type), id_str);
            if (callback) {
                callback(element_handle, user_data);
            }
        });
    }

    void close_qt_window_async(QtWindowHandle* handle) {
        get_qt_thread_executor()->enqueue([=]() {
            // Qt objects are deleted on the GUI thread.
            // The cleanup_qt_window function calls deleteLater() which is thread-safe.
            cleanup_qt_window(handle);
        });
    }

    void refresh_qt_window_async(QtWindowHandle* handle) {
        get_qt_thread_executor()->enqueue([=]() {
            refresh_qt_window(handle);
        });
    }

    void draw_qt_window_async(QtWindowHandle* handle) {
        get_qt_thread_executor()->enqueue([=]() {
            draw_qt_window(handle);
        });
    }
}