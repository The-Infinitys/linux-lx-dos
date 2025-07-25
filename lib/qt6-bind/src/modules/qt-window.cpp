#pragma once
#include "qt-window.hpp"
#include "qt-element.cpp"
#include <QMainWindow>
#include <QWidget>
#include <QVBoxLayout>
#include <QDebug>

class QtWindowWrapper {
public:
    QtWindowWrapper(const char* title, int width, int height) {
        window = new QMainWindow();
        window->setWindowTitle(title);
        window->resize(width, height);
        
        centralWidget = new QWidget();
        layout = new QVBoxLayout(centralWidget);
        window->setCentralWidget(centralWidget);
    }

    ~QtWindowWrapper() {
        delete window; // QWidget's destructor will delete child widgets and layout
    }

    void show() {
        window->show();
    }

    void addWidget(QtElementHandle* element_handle) {
        if (element_handle && element_handle->impl) {
            QWidget* widget = element_handle->impl->getWidget();
            if (widget) {
                layout->addWidget(widget);
            }
        }
    }

private:
    QMainWindow* window;
    QWidget* centralWidget;
    QVBoxLayout* layout;
};

struct QtWindowHandle {
    QtWindowWrapper* impl;
};

extern "C" {

QtWindowHandle* create_qt_window(const char* title, int width, int height) {
    return new QtWindowHandle{new QtWindowWrapper(title, width, height)};
}

void cleanup_qt_window(QtWindowHandle* handle) {
    if (handle) {
        delete handle->impl;
        delete handle;
    }
}

void show_qt_window(QtWindowHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->show();
    }
}

void add_widget_to_window(QtWindowHandle* window_handle, QtElementHandle* element_handle) {
    if (window_handle && window_handle->impl) {
        window_handle->impl->addWidget(element_handle);
    }
}

}