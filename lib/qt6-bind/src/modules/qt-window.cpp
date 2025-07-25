#include "qt-window.hpp"
#include <QMainWindow>

namespace qt_bind {

    // Private implementation class
    class QtWindowPrivate : public QMainWindow {
        Q_OBJECT

    public:
        explicit QtWindowPrivate(QWidget *parent = nullptr) : QMainWindow(parent) {}
        ~QtWindowPrivate() {}

        // Add any private members or methods here if needed

        void show() { QMainWindow::show(); }
    };

    QtWindow::QtWindow(QWidget *parent) : d_ptr(new QtWindowPrivate(parent)) {
        // You can set up the window here, e.g., set title, size, etc.
        // d_ptr->setWindowTitle("My Qt Window");
        // d_ptr->resize(800, 600);
    }

    QtWindow::~QtWindow() {
        delete d_ptr;
    }

    void QtWindow::show() {
        d_ptr->show();
    }

} // namespace qt_bind

#include "qt-window.moc"