#include "lib.hpp"
#include <QApplication>
#include <QSystemTrayIcon>
#include <QMenu>
#include <functional>

struct SystemTray {
    QApplication* app;
    QSystemTrayIcon* tray_icon;
    QMenu* menu;
};

SystemTray* system_tray_new() {
    int argc = 0;
    char** argv = nullptr;
    auto tray = new SystemTray;
    tray->app = new QApplication(argc, argv);
    tray->tray_icon = new QSystemTrayIcon();
    tray->menu = new QMenu();
    tray->tray_icon->setContextMenu(tray->menu);
    tray->tray_icon->show();
    return tray;
}

void system_tray_delete(SystemTray* tray) {
    delete tray->menu;
    delete tray->tray_icon;
    delete tray->app;
    delete tray;
}

void system_tray_run(SystemTray* tray) {
    tray->app->exec();
}

void system_tray_exit(SystemTray* tray) {
    tray->app->exit();
}

void system_tray_set_icon(SystemTray* tray, const char* path) {
    tray->tray_icon->setIcon(QIcon(path));
}

void system_tray_add_menu_item(SystemTray* tray, const char* text, void (*callback)(void*), void* user_data) {
    auto action = tray->menu->addAction(text);
    QObject::connect(action, &QAction::triggered, [callback, user_data]() {
        callback(user_data);
    });
}
