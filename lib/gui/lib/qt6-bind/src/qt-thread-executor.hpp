#pragma once

#include <QObject>
#include <QQueue>
#include <functional>
#include <QMutex>

// Forward declarations for types used in async functions
struct QtAppHandle;
struct QtWindowHandle;
struct QtElementHandle;

class QtThreadExecutor : public QObject {
    Q_OBJECT
public:
    explicit QtThreadExecutor(QObject *parent = nullptr);

    // This method will be called from other threads to enqueue tasks
    void enqueue(std::function<void()> task);

signals:
    // Signal to notify the main thread that there are tasks to process
    void tasksAvailable();

public slots:
    // Slot to process tasks on the main thread
    void processTasks();

private:
    QQueue<std::function<void()>> taskQueue;
    QMutex mutex;
};

// C-style API to get the executor instance
extern "C" {
    QtThreadExecutor* get_qt_thread_executor();

    // Thread-safe functions to be called from Rust
    void create_qt_window_async(QtAppHandle* app_handle, const char* title, int width, int height, void (*callback)(QtWindowHandle*));
    void show_qt_window_async(QtWindowHandle* handle);
    void add_widget_to_window_async(QtWindowHandle* window_handle, QtElementHandle* element_handle);
    void set_element_text_async(QtElementHandle* element_handle, const char* text);
    void set_element_size_async(QtElementHandle* element_handle, int width, int height);
    void set_element_enabled_async(QtElementHandle* element_handle, bool enabled);
    void create_qt_element_async(int element_type, const char* id_str, void (*callback)(QtElementHandle*, void*), void* user_data);
    void close_qt_window_async(QtWindowHandle* handle);
    void refresh_qt_window_async(QtWindowHandle* handle);
    void draw_qt_window_async(QtWindowHandle* handle);
}