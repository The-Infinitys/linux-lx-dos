#include "qt-window.hpp"
#include "qt-element.cpp"
#include <QApplication>
#include <QWidget>
#include <QVBoxLayout>
#include <string>
#include <vector>
#include <QDebug>

class QtWindowWrapper : public QWidget {
    Q_OBJECT

public:
    QtWindowWrapper(const std::string& title, int width, int height) : QWidget(nullptr) {
        setWindowTitle(QString::fromStdString(title));
        resize(width, height);
        layout = new QVBoxLayout(this);
        setLayout(layout);

        // Connect the QWidget::destroyed signal to capture window close events
        connect(this, &QWidget::destroyed, this, &QtWindowWrapper::onWindowDestroyed);
    }

    ~QtWindowWrapper() {
        // Clear event queue
        event_queue.clear();
    }

    void addWidget(QWidget* widget) {
        if (layout) {
            layout->addWidget(widget);
        }
    }

    WindowEvent pollEvent() {
        if (event_queue.empty()) {
            return {QtWindowEvent_None};
        }
        WindowEvent event = event_queue.front();
        event_queue.erase(event_queue.begin());
        return event;
    }

private slots:
    void onWindowDestroyed() {
        qDebug() << "Window destroyed signal received.";
        event_queue.push_back({QtWindowEvent_Closed});
    }

private:
    QVBoxLayout* layout;
    std::vector<WindowEvent> event_queue;
};

// Opaque pointer for QtWindowWrapper
struct QtWindowHandle {
    QtWindowWrapper* impl;
};

extern "C" {

QtWindowHandle* create_qt_window(const char* title, int width, int height) {
    return new QtWindowHandle{new QtWindowWrapper(title, width, height)};
}

void show_qt_window(QtWindowHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->show();
    }
}

void add_widget_to_window(QtWindowHandle* window_handle, void* element_handle) {
    if (window_handle && window_handle->impl && element_handle) {
        // Assuming element_handle is a pointer to QWidget (QtElementWrapper)
        QtElementHandle* elem_handle = static_cast<QtElementHandle*>(element_handle);
        if (elem_handle && elem_handle->impl) {
            window_handle->impl->addWidget(elem_handle->impl);
        }
    }
}

WindowEvent poll_window_event(QtWindowHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->pollEvent();
    }
    return {QtWindowEvent_None};
}

void cleanup_qt_window(QtWindowHandle* handle) {
    if (handle) {
        // Qt widgets are part of the Qt object tree and are usually deleted
        // when their parent is deleted or when QApplication is destroyed.
        // However, if this window is a top-level window, it needs to be explicitly deleted.
        // Calling deleteLater() is safer for Qt objects.
        if (handle->impl) {
            handle->impl->deleteLater();
        }
        delete handle;
    }
}

int is_qt_window_valid(QtWindowHandle* handle) {
    return (handle && handle->impl && !handle->impl->isWindowType()) ? 1 : 0;
}

} // extern "C"

#include "qt-window.moc"
