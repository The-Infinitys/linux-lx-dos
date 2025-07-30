#include "lib.hpp"
#include <QApplication>
#include <QIcon>
#include <QMenu>
#include <QSystemTrayIcon>
#include <QBuffer>
#include <string>
#include <vector>
#include <memory>
#include <QDebug> // Add for debugging

// --- Internal C++ Implementation ---

class QtAppWrapper {
public:
    QtAppWrapper() = default;

    void setAppId(const std::string& id) {
        appId = id;
    }

    void setAppIcon(const unsigned char* data, size_t size, const char* format) {
        iconData = QByteArray(reinterpret_cast<const char*>(data), size);
        iconFormat = format;
    }

    void initTray() {
        shouldInitTray = true;
    }

    int run(int argc, char* argv[]) {
        // QDebug() << "QtAppWrapper::run started.";
        app = new QApplication(argc, argv);
        // QDebug() << "QApplication created.";

        if (!appId.empty()) {
            app->setApplicationName(QString::fromStdString(appId));
            // QDebug() << "Application ID set to:" << QString::fromStdString(appId);
        }

        QIcon appIcon;
        if (!iconData.isEmpty()) {
            QPixmap pixmap;
            if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                appIcon = QIcon(pixmap);
                app->setWindowIcon(appIcon);
                // QDebug() << "Application icon loaded and set.";
            } else {
                qWarning() << "Failed to load application icon from data. Format:" << iconFormat.c_str();
            }
        } else {
            // QDebug() << "No icon data provided.";
        }

        if (shouldInitTray) {
            // QDebug() << "Tray initialization requested.";
            if (!QSystemTrayIcon::isSystemTrayAvailable()) {
                qWarning() << "System tray is not available on this system.";
                return -1; // System tray is not available
            }
            // QDebug() << "System tray is available.";

            menu = new QMenu();
            tray = new QSystemTrayIcon(appIcon);
            tray->setContextMenu(menu);
            // QDebug() << "Tray icon and menu created and linked.";

            QObject::connect(tray, &QSystemTrayIcon::activated, [this](QSystemTrayIcon::ActivationReason reason) {
                if (reason == QSystemTrayIcon::Context) {
                    // QDebug() << "Tray icon context menu activated (right-click).";
                } else if (reason == QSystemTrayIcon::Trigger) {
                    // QDebug() << "Tray icon triggered (left-click).";
                    event_queue.push_back({AppEventType::TrayClicked, nullptr});
                } else if (reason == QSystemTrayIcon::DoubleClick) {
                    // QDebug() << "Tray icon double-clicked.";
                    event_queue.push_back({AppEventType::TrayDoubleClicked, nullptr});
                }
            });
            tray->show();
            // QDebug() << "Tray icon shown.";

            // Add pending menu items after QApplication and QMenu are ready
            for (const auto& item : pending_menu_items) {
                addTrayMenuItem(item.first, item.second);
            }
            pending_menu_items.clear(); // Clear the pending list
        } else {
            // QDebug() << "Tray initialization not requested.";
        }

        // QDebug() << "Starting QApplication event loop...";
        return app->exec();
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
        // QDebug() << "Attempting to add tray menu item:" << QString::fromStdString(text) << "with ID:" << QString::fromStdString(id_str);
        if (!app) { // If QApplication is not yet created, store for later
            pending_menu_items.push_back({text, id_str});
            // QDebug() << "QApplication not yet created, pending menu item.";
            return;
        }

        if (!menu) {
            // If menu is null, create it now. This handles cases where menu items are added before run().
            menu = new QMenu();
            if (tray) {
                tray->setContextMenu(menu);
                // QDebug() << "Created new menu and set it to tray.";
            }
        }

        QAction* action = menu->addAction(QString::fromStdString(text));
        QObject::connect(action, &QAction::triggered, [this, id_str]() {
            // QDebug() << "Menu item triggered:" << QString::fromStdString(id_str) << ", adding to event queue.";
            // Duplicate the string so it can be owned by Rust and freed later
            char* id_cstr = strdup(id_str.c_str());
            event_queue.push_back({AppEventType::MenuItemClicked, id_cstr});
        });
        // QDebug() << "Successfully added tray menu item.";
    }

    void quitApp() {
        if (app) {
            app->quit();
        }
    }

private:
    // Configuration
    std::string appId;
    QByteArray iconData;
    std::string iconFormat;
    bool shouldInitTray = false;
    std::vector<AppEvent> event_queue;
    std::vector<std::pair<std::string, std::string>> pending_menu_items; // New: To store menu items added before run()

    // Qt Objects - Let Qt manage their lifetime.
    QMenu* menu = nullptr;
    QSystemTrayIcon* tray = nullptr;
    QApplication* app = nullptr;
};

// Opaque handle that points to the C++ implementation
struct QtAppHandle {
    QtAppWrapper* impl;
};


// --- C-style API Implementation ---

extern "C" {

QtAppHandle* create_qt_app() {
    return new QtAppHandle{new QtAppWrapper()};
}

void set_app_id(QtAppHandle* handle, const char* id) {
    if (handle && handle->impl) {
        handle->impl->setAppId(id);
    }
}

void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format) {
    if (handle && handle->impl) {
        handle->impl->setAppIcon(data, size, format);
    }
}

void init_tray(QtAppHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->initTray();
    }
}

int run_qt_app(QtAppHandle* handle, int argc, char* argv[]) {
    if (handle && handle->impl) {
        return handle->impl->run(argc, argv);
    }
    return -1;
}

AppEvent poll_event(QtAppHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->pollEvent();
    }
    return {AppEventType::None, nullptr};
}

void quit_qt_app(QtAppHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->quitApp();
    }
}

void cleanup_qt_app(QtAppHandle* handle) {
    // Let the OS clean up resources on process exit. 
    // Explicitly deleting Qt objects here conflicts with Qt's own cleanup mechanisms
    // after the event loop has finished, causing a crash.
    if (handle) {
        delete handle->impl;
        delete handle;
    }
}

void add_tray_menu_item(QtAppHandle* handle, const char* text, const char* id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    }
}

void free_char_ptr(const char* ptr) {
    free((void*)ptr);
}

} // extern "C"