#include "lib.hpp"
#include <QApplication>
#include <QSystemTrayIcon>
#include <QMenu>
#include <QBuffer>
#include <functional>

static QApplication* app = nullptr;

struct SystemTray {
    QSystemTrayIcon* tray_icon;
    QMenu* menu;
};

SystemTray* system_tray_new(const char* name, const char* id) {
    if (!app) {
        int argc = 0;
        char** argv = nullptr;
        app = new QApplication(argc, argv);
        app->setApplicationName(name);
        app->setOrganizationName(id);
    }
    auto tray = new SystemTray;
    tray->tray_icon = new QSystemTrayIcon();
    tray->menu = new QMenu();
    tray->tray_icon->setContextMenu(tray->menu);
    return tray;
}

void system_tray_delete(SystemTray* tray) {
    delete tray->menu;
    delete tray->tray_icon;
    delete tray;
}

void system_tray_run(SystemTray* tray) {
    tray->tray_icon->show();
    app->exec();
}

void system_tray_exit() {
    if (app) {
        app->exit();
        delete app;
        app = nullptr;
    }
}

void system_tray_set_icon(SystemTray* tray, const unsigned char* data, size_t len, const char* format) {
    QByteArray byte_array(reinterpret_cast<const char*>(data), len);
    QPixmap pixmap;
    pixmap.loadFromData(byte_array, format);
    tray->tray_icon->setIcon(QIcon(pixmap));
}

void system_tray_add_menu_item(SystemTray* tray, const char* text, void (*callback)(void*), void* user_data) {
    auto action = tray->menu->addAction(text);
    QObject::connect(action, &QAction::triggered, [callback, user_data]() {
        callback(user_data);
    });
}
