#pragma once

// Forward declarations for Qt classes
class QMainWindow;
class QWidget;

namespace qt_bind {
    // Forward declaration of the private implementation class
    class QtWindowPrivate;

    class QtWindow {
    public:
        explicit QtWindow(QWidget *parent = nullptr);
        ~QtWindow();

        void show();

    private:
        QtWindowPrivate *d_ptr; // Opaque pointer to the private implementation
    };
} // namespace qt_bind