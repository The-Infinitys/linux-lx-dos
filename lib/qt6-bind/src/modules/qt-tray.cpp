#include "qt-tray.hpp"
#include <QDebug>

// --- Internal C++ Implementation for System Tray Logic ---

QtTrayWrapper::QtTrayWrapper(QtCoreAppWrapper* core_wrapper) : core(core_wrapper) {
    qDebug() << "QtTrayWrapper constructor.";
}

QtTrayWrapper::~QtTrayWrapper() {
    qDebug() << "QtTrayWrapper destructor.";
    // Qt objects (menu, tray) are parented to QApplication or explicitly deleted.
    // Rely on Qt's cleanup or explicit deletion in cleanup_qt_app if necessary.
}

void QtTrayWrapper::initTray(const QIcon& appIcon) {
    qDebug() << "QtTrayWrapper::initTray: Initializing tray.";
    if (!QSystemTrayIcon::isSystemTrayAvailable()) {
        qWarning() << "System tray is not available on this system.";
        return;
    }
    qDebug() << "System tray is available.";

    menu = new QMenu();
    tray = new QSystemTrayIcon(appIcon);
    tray->setContextMenu(menu);
    qDebug() << "Tray icon and menu created and linked.";

    QObject::connect(tray, &QSystemTrayIcon::activated, [this](QSystemTrayIcon::ActivationReason reason) {
        if (!core) {
            qWarning() << "Core wrapper is null in tray activated signal.";
            return;
        }
        if (reason == QSystemTrayIcon::Context) {
            qDebug() << "Tray icon context menu activated (right-click).";
        } else if (reason == QSystemTrayIcon::Trigger) {
            qDebug() << "Tray icon triggered (left-click).";
            core->addEventToQueue({AppEventType::TrayClicked, nullptr});
        } else if (reason == QSystemTrayIcon::DoubleClick) {
            qDebug() << "Tray icon double-clicked.";
            core->addEventToQueue({AppEventType::TrayDoubleClicked, nullptr});
        }
    });
    tray->show();
    qDebug() << "Tray icon shown.";

    // Add pending menu items now that the menu is ready
    for (const auto& item : pending_menu_items) {
        addTrayMenuItem(item.first, item.second);
    }
    pending_menu_items.clear(); // Clear the pending list
}

void QtTrayWrapper::addTrayMenuItem(const std::string& text, const std::string& id_str) {
    qDebug() << "QtTrayWrapper::addTrayMenuItem: Attempting to add tray menu item:" << QString::fromStdString(text) << "with ID:" << QString::fromStdString(id_str);
    if (!menu) { // If menu is not yet created, store for later
        pending_menu_items.push_back({text, id_str});
        qDebug() << "Menu not yet initialized, pending menu item.";
        return;
    }

    QAction* action = menu->addAction(QString::fromStdString(text));
    QObject::connect(action, &QAction::triggered, [this, id_str]() {
        if (!core) {
            qWarning() << "Core wrapper is null in menu item triggered signal.";
            return;
        }
        qDebug() << "Menu item triggered:" << QString::fromStdString(id_str) << ", adding to event queue.";
        // Duplicate the string so it can be owned by Rust and freed later
        char* id_cstr = strdup(id_str.c_str());
        core->addEventToQueue({AppEventType::MenuItemClicked, id_cstr});
    });
    qDebug() << "Successfully added tray menu item.";
}
