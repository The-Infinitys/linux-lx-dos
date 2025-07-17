#include "qt-lx-dos.hpp"
#include "../ui/main/main.hpp"
#include "../ui/settings/settings.hpp"
#include "../ui/welcome/welcome.hpp"
#include <QApplication>

static QApplication *app = nullptr;
static Widget *main_window = nullptr;
static SettingsWindow *settings_window = nullptr;
static WelcomeWindow *welcome_window = nullptr;

RustEventCallback global_event_callback = nullptr;

void register_event_callback(RustEventCallback callback) {
    global_event_callback = callback;
}

void send_qt_command(const char* command_name, const char* command_data) {
    // This is where you would handle commands from Rust
    // For now, just print them to debug output
    qDebug("Received command from Rust: %s, data: %s", command_name, command_data);
}

void run_qt_app() {
    if (app) return;
    int argc = 0;
    char **argv = nullptr;
    app = new QApplication(argc, argv);
    QApplication::setQuitOnLastWindowClosed(false);
    main_window = new Widget();
    // main_window->show(); // Start hidden
    app->exec();
    delete main_window;
    delete app;
    main_window = nullptr;
    app = nullptr;
}

void show_main_window() {
    if (main_window) {
        main_window->show();
    }
}

void show_settings_window() {
    if (!settings_window) {
        settings_window = new SettingsWindow();
    }
    settings_window->show();
}

void show_welcome_window() {
    if (!welcome_window) {
        welcome_window = new WelcomeWindow();
    }
    welcome_window->show();
}
