#include "lib.hpp"
#include <QAction>
#include <QCoreApplication>

extern "C" {

qt_tray::QtTray* create_qt_tray() {
    // Ensure QCoreApplication is initialized for Qt objects
    if (!QCoreApplication::instance()) {
        static int argc = 0;
        new QCoreApplication(argc, nullptr);
    }
    return new qt_tray::QtTray();
}

void destroy_qt_tray(qt_tray::QtTray* tray) {
    delete tray;
}

void qt_tray_set_icon(qt_tray::QtTray* tray, const char* icon_path) {
    if (tray && icon_path) {
        tray->setIcon(QString::fromUtf8(icon_path));
    }
}

void qt_tray_set_tool_tip(qt_tray::QtTray* tray, const char* tool_tip) {
    if (tray && tool_tip) {
        tray->setToolTip(QString::fromUtf8(tool_tip));
    }
}

void qt_tray_add_menu_item(qt_tray::QtTray* tray, const char* text, void (*callback)(void*), void* user_data) {
    if (tray && text && callback) {
        QAction* action = tray->getMenu()->addAction(QString::fromUtf8(text));
        QObject::connect(action, &QAction::triggered, [callback, user_data]() {
            callback(user_data);
        });
    }
}

void qt_tray_on_activated(qt_tray::QtTray* tray, void (*callback)(int, void*), void* user_data) {
    if (tray && callback) {
        QObject::connect(tray, &qt_tray::QtTray::activated, [callback, user_data](QSystemTrayIcon::ActivationReason reason) {
            callback(static_cast<int>(reason), user_data);
        });
    }
}

}