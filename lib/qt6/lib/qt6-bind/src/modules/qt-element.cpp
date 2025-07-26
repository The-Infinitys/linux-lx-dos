// src/modules/qt-element.cpp
#include "qt-element.hpp" // Include the C-style API header

// --- Internal C++ Implementation (QtElementWrapper class and Qt includes) ---
// All Qt-specific includes and the full class definition are here.

#include <QWidget>
#include <QPushButton>
#include <QLabel>
#include <QLineEdit>
#include <QVBoxLayout> // Added for QVBoxLayout
#include <QDebug>
#include <string>
#include <vector>
#include <queue>
#include <memory> // For std::unique_ptr
#include <QObject> // Required for Q_OBJECT macro and signals/slots
#include <QString> // For QString conversions

// Full definition of QtElementWrapper class
class QtElementWrapper : public QWidget
{
    Q_OBJECT // Qt's meta-object system requires this macro

public:
    // Constructor: Takes the public C-style enum for type
    QtElementWrapper(enum QtElementType type, const std::string &id) : element_id(id), widget(nullptr)
    {
        switch (type)
        {
        case QtElementType_Button:
        {
            QPushButton *button = new QPushButton(QString::fromStdString(id));
            // Connect clicked signal to a lambda that enqueues the event
            QObject::connect(button, &QPushButton::clicked, this, [this, id]() {
                QtElementEvent event = {QtElementEventType_Clicked, strdup(id.c_str()), nullptr};
                event_queue.push(event);
            });
            widget = button;
            break;
        }
        case QtElementType_Label:
        {
            QLabel *label = new QLabel(QString::fromStdString(id));
            widget = label;
            break;
        }
        case QtElementType_LineEdit:
        {
            QLineEdit *lineEdit = new QLineEdit(QString::fromStdString(id));
            // Connect textChanged signal
            QObject::connect(lineEdit, &QLineEdit::textChanged, this, [this, id](const QString &text) {
                QtElementEvent event = {QtElementEventType_TextChanged, strdup(id.c_str()), strdup(text.toStdString().c_str())};
                event_queue.push(event);
            });
            // Connect editingFinished signal
            QObject::connect(lineEdit, &QLineEdit::editingFinished, this, [this, id, lineEdit]() {
                QtElementEvent event = {QtElementEventType_EditingFinished, strdup(id.c_str()), strdup(lineEdit->text().toStdString().c_str())};
                event_queue.push(event);
            });
            widget = lineEdit;
            break;
        }
        default:
            qWarning() << "Unknown QtElementType encountered!";
            break;
        }
    }

    // Destructor
    ~QtElementWrapper()
    {
        if (widget)
        {
            // QWidget's parent-child mechanism handles deletion,
            // but if it has no parent, we must delete it manually.
            // For elements created without a parent, manual deletion is necessary.
            delete widget;
            widget = nullptr;
        }
        // Free any remaining allocated strings in the queue
        while (!event_queue.empty()) {
            QtElementEvent event = event_queue.front();
            event_queue.pop();
            free((void*)event.element_id_str);
            if (event.data_str) {
                free((void*)event.data_str);
            }
        }
    }

    // Get the internal QWidget pointer
    QWidget *getWidget() const
    {
        return widget;
    }

    void setText(const std::string &text)
    {
        if (QPushButton *button = qobject_cast<QPushButton *>(widget))
        {
            button->setText(QString::fromStdString(text));
        }
        else if (QLabel *label = qobject_cast<QLabel *>(widget))
        {
            label->setText(QString::fromStdString(text));
        }
        else if (QLineEdit *lineEdit = qobject_cast<QLineEdit *>(widget))
        {
            lineEdit->setText(QString::fromStdString(text));
        }
    }

    void setSize(int width, int height)
    {
        if (widget)
        {
            widget->setFixedSize(width, height);
        }
    }

    void setEnabled(bool enabled)
    {
        if (widget)
        {
            widget->setEnabled(enabled);
        }
    }

    // Poll for the next event from this element (returns the public C-style struct)
    QtElementEvent pollEvent()
    {
        if (!event_queue.empty())
        {
            QtElementEvent event = event_queue.front();
            event_queue.pop();
            return event;
        }
        return {QtElementEventType_None, nullptr, nullptr};
    }

private:
    std::string element_id;
    QWidget *widget;
    std::queue<QtElementEvent> event_queue;
    public:
    QVBoxLayout* m_layout; // Added for QtElementType_Widget
};

// Define QtElementHandle for C++ compilation (concrete definition)
// This must be here because QtElementWrapper is defined here.
struct QtElementHandle {
    QtElementWrapper* impl;
};


// --- C-style API function implementations ---

extern "C"
{

    QtElementHandle *create_qt_element(QtElementType type, const char *id_str)
    {
        // Allocate QtElementHandle on the heap
        QtElementHandle *handle = new QtElementHandle();
        // Allocate QtElementWrapper on the heap and assign to impl
        handle->impl = new QtElementWrapper(type, id_str);
        return handle;
    }

    void set_element_text(QtElementHandle *handle, const char *text)
    {
        if (handle && handle->impl)
        {
            handle->impl->setText(text);
        }
    }

    void set_element_size(QtElementHandle *handle, int width, int height)
    {
        if (handle && handle->impl)
        {
            handle->impl->setSize(width, height);
        }
    }

    void set_element_enabled(QtElementHandle *handle, bool enabled)
    {
        if (handle && handle->impl)
        {
            handle->impl->setEnabled(enabled);
        }
    }

    QtElementEvent poll_element_event(QtElementHandle *handle)
    {
        if (handle && handle->impl)
        {
            return handle->impl->pollEvent();
        }
        return {QtElementEventType_None, nullptr, nullptr};
    }

    void* get_qt_element_widget(QtElementHandle *handle)
    {
        if (handle && handle->impl)
        {
            return handle->impl->getWidget();
        }
        return nullptr;
    }

    void free_char_ptr(const char *ptr) {
        if (ptr) {
            free((void*)ptr);
        }
    }

    void cleanup_qt_element(QtElementHandle *handle)
    {
        if (handle)
        {
            // Delete the internal C++ object first
            delete handle->impl;
            handle->impl = nullptr; // Prevent double deletion
            // Then delete the handle itself
            delete handle;
        }
    }

    void add_child_element_to_element(QtElementHandle* parent_handle, QtElementHandle* child_handle) {
        if (parent_handle && parent_handle->impl && child_handle && child_handle->impl) {
            if (parent_handle->impl->m_layout) {
                parent_handle->impl->m_layout->addWidget(child_handle->impl->getWidget());
            } else {
                qWarning() << "Parent element does not have a layout to add children to.";
            }
        }
    }

} // extern "C"

// IMPORTANT: Include the MOC generated file here.
// When Q_OBJECT is in a .cpp file, the MOC output is typically named <filename>.moc
// and needs to be explicitly included.
#include "qt-element.moc"
