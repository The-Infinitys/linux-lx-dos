#include "qt-tray.hpp"
#include <QSystemTrayIcon>
#include <QMenu>
#include <QAction>
#include <QDebug>
#include <QIcon>
#include <QPixmap>
#include <QByteArray>
#include <vector>
#include <string>

// --- Internal C++ Implementation ---

class QtTrayWrapper {
public:
    QtTrayWrapper() = default;

    void initTray(const unsigned char* icon_data, size_t icon_size, const char* icon_format) {
        qDebug() << "Tray initialization requested.";
        if (!QSystemTrayIcon::isSystemTrayAvailable()) {
            qWarning() << "System tray is not available on this system.";
            return; 
        }
        qDebug() << "System tray is available.";

        QIcon trayIcon;
        if (icon_data && icon_size > 0 && icon_format) {
            QPixmap pixmap;
            if (pixmap.loadFromData(QByteArray(reinterpret_cast<const char*>(icon_data), icon_size), icon_format)) {
                trayIcon = QIcon(pixmap);
                qDebug() << "Tray icon loaded from data.";
            } else {
                qWarning() << "Failed to load tray icon from data. Format:" << icon_format;
            }
        } else {
            qDebug() << "No icon data provided for tray.";
        }

        menu = new QMenu();
        tray = new QSystemTrayIcon(trayIcon);
        tray->setContextMenu(menu);
        qDebug() << "Tray icon and menu created and linked.";

        QObject::connect(tray, &QSystemTrayIcon::activated, [this](QSystemTrayIcon::ActivationReason reason) {
            if (reason == QSystemTrayIcon::Context) {
                qDebug() << "Tray icon context menu activated (right-click).";
            } else if (reason == QSystemTrayIcon::Trigger) {
                qDebug() << "Tray icon triggered (left-click).";
                event_queue.push_back({AppEventType::TrayClicked, nullptr});
            } else if (reason == QSystemTrayIcon::DoubleClick) {
                qDebug() << "Tray icon double-clicked.";
                event_queue.push_back({AppEventType::TrayDoubleClicked, nullptr});
            }
        });
        tray->show();
        qDebug() << "Tray icon shown.";

        // Add pending menu items after QMenu is ready
        for (const auto& item : pending_menu_items) {
            addTrayMenuItem(item.first, item.second);
        }
        pending_menu_items.clear(); // Clear the pending list
    }

    AppEvent pollEvent() {
        if (event_queue.empty()) {
            return {AppEventType::None, nullptr};
        }
        AppEvent event = event_queue.front();
        event_queue.erase(event_queue.begin());
        return event;
    }

    void addTrayMenuItem(const std::string& text, const std::string& id_str) {
        qDebug() << "Attempting to add tray menu item:" << QString::fromStdString(text) << "with ID:" << QString::fromStdString(id_str);
        if (!menu) {
            // If menu is null, create it now. This handles cases where menu items are added before initTray().
            menu = new QMenu();
            if (tray) {
                tray->setContextMenu(menu);
                qDebug() << "Created new menu and set it to tray.";
            }
            pending_menu_items.push_back({text, id_str});
            qDebug() << "Menu not yet created, pending menu item.";
            return;
        }

        QAction* action = menu->addAction(QString::fromStdString(text));
        QObject::connect(action, &QAction::triggered, [this, id_str]() {
            qDebug() << "Menu item triggered:" << QString::fromStdString(id_str) << ", adding to event queue.";
            // Duplicate the string so it can be owned by Rust and freed later
            char* id_cstr = strdup(id_str.c_str());
            event_queue.push_back({AppEventType::MenuItemClicked, id_cstr});
        });
        qDebug() << "Successfully added tray menu item.";
    }

private:
    std::vector<AppEvent> event_queue;
    std::vector<std::pair<std::string, std::string>> pending_menu_items; // New: To store menu items added before run()

    // Qt Objects - Let Qt manage their lifetime.
    QMenu* menu = nullptr;
    QSystemTrayIcon* tray = nullptr;
};

// Opaque handle that points to the C++ implementation
struct QtTrayHandle {
    QtTrayWrapper* impl;
};


// --- C-style API Implementation ---

extern "C" {

QtTrayHandle* create_qt_tray() {
    return new QtTrayHandle{new QtTrayWrapper()};
}

void init_tray(QtTrayHandle* handle, const unsigned char* icon_data, size_t icon_size, const char* icon_format) {
    if (handle && handle->impl) {
        handle->impl->initTray(icon_data, icon_size, icon_format);
    }
}

AppEvent poll_event(QtTrayHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->pollEvent();
    }
    return {AppEventType::None, nullptr};
}

void add_tray_menu_item(QtTrayHandle* handle, const char* text, const char* id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    }
}

void cleanup_qt_tray(QtTrayHandle* handle) {
    if (handle) {
        delete handle->impl;
        delete handle;
    }
}

void free_char_ptr(const char* ptr) {
    free((void*)ptr);
}

} // extern "C"