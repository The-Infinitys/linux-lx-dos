#include "qt-widget.hpp"
#include <QWidget>
#include <QDebug>

// --- Internal C++ Implementation ---

#include "qt-widget.hpp"
#include <QWidget>
#include <QDebug>
#include <QApplication>

// --- Internal C++ Implementation ---

class QtWidgetWrapper {
public:
    QtWidgetWrapper() = default;

    ~QtWidgetWrapper() {
        if (widget) {
            delete widget;
            widget = nullptr;
            qDebug() << "QWidget destroyed.";
        }
    }

    void ensureWidgetCreated() {
        if (!widget && QApplication::instance()) {
            widget = new QWidget();
            qDebug() << "QWidget created lazily.";
        }
    }

    void setTitle(const std::string& title) {
        ensureWidgetCreated();
        if (widget) {
            widget->setWindowTitle(QString::fromStdString(title));
            qDebug() << "Widget title set to:" << QString::fromStdString(title);
        }
    }

    void show() {
        ensureWidgetCreated();
        if (widget) {
            widget->show();
            qDebug() << "QWidget shown.";
        }
    }

    void hide() {
        if (widget) {
            widget->hide();
            qDebug() << "QWidget hidden.";
        }
    }

private:
    QWidget* widget = nullptr;
};

// Opaque handle that points to the C++ implementation
struct QtWidgetHandle {
    QtWidgetWrapper* impl;
};


// --- C-style API Implementation ---

extern "C" {

QtWidgetHandle* create_qt_widget() {
    return new QtWidgetHandle{new QtWidgetWrapper()};
}

void set_widget_title(QtWidgetHandle* handle, const char* title) {
    if (handle && handle->impl) {
        handle->impl->setTitle(title);
    }
}

void show_qt_widget(QtWidgetHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->show();
    }
}

void hide_qt_widget(QtWidgetHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->hide();
    }
}

void cleanup_qt_widget(QtWidgetHandle* handle) {
    if (handle) {
        delete handle->impl;
        delete handle;
    }
}

} // extern "C"