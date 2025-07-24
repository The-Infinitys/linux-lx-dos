#include "modules/qt-app.hpp"
#include "modules/qt-tray.hpp"
#include "modules/qt-widget.hpp"
#include <vector>
#include <string>
#include <QDebug> // Add for debugging
#include <QThread> // Add for QThread::msleep
#include <QIcon>

// A simple dummy SVG icon for testing using a raw string literal
const std::string svg_icon_data = R"(
<svg width="64" height="64" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
  <rect width="64" height="64" rx="8" ry="8" fill="#2ecc71" />
  <text x="32" y="42" font-family="monospace" font-size="24" fill="white" text-anchor="middle">OK</text>
</svg>
)";

int main(int argc, char* argv[]) {
    QtAppHandle* app = create_qt_app();
    QtTrayHandle* tray = create_qt_tray();
    QtWidgetHandle* widget = create_qt_widget();

    // Initialize QApplication first
    init_qt_application(app, argc, argv);

    set_app_id(app, "com.example.TestApp");

    // Set app icon data (QIcon will be created inside init_qt_application)
    set_app_icon_from_data(
        app, 
        reinterpret_cast<const unsigned char*>(svg_icon_data.c_str()), 
        svg_icon_data.length(), 
        "SVG"
    );

    // Initialize tray with the application icon
    init_tray(tray, reinterpret_cast<const unsigned char*>(svg_icon_data.c_str()), svg_icon_data.length(), "SVG");

    // Add some test menu items to the tray
    add_tray_menu_item(tray, "Open", "open_menu_item");
    qDebug() << "Added menu item: Open";
    add_tray_menu_item(tray, "Settings", "settings_menu_item");
    qDebug() << "Added menu item: Settings";
    add_tray_menu_item(tray, "Quit", "quit_menu_item");
    qDebug() << "Added menu item: Quit";

    // Set up and show a basic widget
    set_widget_title(widget, "My Qt Widget");

    // Event polling loop (for demonstration)
    // This loop will run after the Qt event loop starts, which is not ideal for polling.
    // A better approach would be to use Qt signals/slots or QTimer for event handling.
    // For now, we'll keep it as is to demonstrate the API calls.
    QThread::msleep(100); // Give Qt some time to process events

    int result = run_qt_app(app);

    // Event polling loop (for demonstration)
    while (true) {
        AppEvent event = poll_event(tray);
        if (event.type == AppEventType::MenuItemClicked) {
            qDebug() << "Menu item clicked with ID:" << event.menu_id_str;
            std::string menu_id_str(event.menu_id_str);
            free_char_ptr(event.menu_id_str); // Free the C-string allocated in addTrayMenuItem

            if (menu_id_str == "open_menu_item") { // Open
                qDebug() << "Open action triggered!";
                show_qt_widget(widget);
            } else if (menu_id_str == "settings_menu_item") { // Settings
                qDebug() << "Settings action triggered!";
            } else if (menu_id_str == "quit_menu_item") { // Quit
                qDebug() << "Quit action triggered!";
                quit_qt_app(app);
                break; // Exit loop after quitting
            }
        } else if (event.type == AppEventType::TrayClicked) {
            qDebug() << "Tray icon left-clicked!";
            show_qt_widget(widget);
        } else if (event.type == AppEventType::TrayDoubleClicked) {
            qDebug() << "Tray icon double-clicked!";
            hide_qt_widget(widget);
        } else if (event.type == AppEventType::None) {
            // No event, sleep for a short period to avoid busy-waiting
            // This is a simple example, in a real app, you might use a timer or more sophisticated event handling
            QThread::msleep(100);
        }
    }

    cleanup_qt_app(app);
    cleanup_qt_tray(tray);
    cleanup_qt_widget(widget);

    return result;
}