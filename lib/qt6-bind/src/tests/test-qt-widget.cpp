#include "../modules/qt-widget.hpp"
#include "../modules/qt-app.hpp"
#include <QApplication>
#include <QWidget>
#include <QDebug>
#include <QTimer>

int main(int argc, char* argv[]) {
    // Initialize QApplication
    QtAppHandle* app_handle = create_qt_app();
    init_qt_application(app_handle, argc, argv);

    qDebug() << "Starting qt-widget tests...";

    // Test 1: Create and set title
    QtWidgetHandle* widget_handle = create_qt_widget();
    set_widget_title(widget_handle, "Test Widget Title");
    qDebug() << "Test 1: Widget created and title set.";

    // Test 2: Show and hide widget
    show_qt_widget(widget_handle);
    qDebug() << "Test 2: Widget shown.";

    // Use QTimer to hide the widget after a short delay
    QTimer::singleShot(1000, [=]() {
        hide_qt_widget(widget_handle);
        qDebug() << "Test 2: Widget hidden.";

        // Test 3: Cleanup
        cleanup_qt_widget(widget_handle);
        qDebug() << "Test 3: Widget cleaned up.";

        // Quit the application after tests are done
        quit_qt_app(app_handle);
        cleanup_qt_app(app_handle);
        qDebug() << "All tests completed successfully.";
    });

    return run_qt_app(app_handle);
}