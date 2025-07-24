#include "qt-app.hpp"
#include <QApplication>
#include <QIcon>
#include <QMenu>
#include <QSystemTrayIcon>
#include <QBuffer>
#include <string>
#include <vector>
#include <memory>

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
        // The QApplication object is now managed by Qt's object tree.
        app = new QApplication(argc, argv);

        if (!appId.empty()) {
            app->setApplicationName(QString::fromStdString(appId));
        }

        QIcon appIcon;
        if (!iconData.isEmpty()) {
            QPixmap pixmap;
            pixmap.loadFromData(iconData, iconFormat.c_str());
            appIcon = QIcon(pixmap);
            app->setWindowIcon(appIcon);
        }

        if (shouldInitTray) {
            if (!QSystemTrayIcon::isSystemTrayAvailable()) {
                return -1; // System tray is not available
            }

            // The tray and menu are also managed by Qt's object tree.
            menu = new QMenu();
            tray = new QSystemTrayIcon(appIcon);
            tray->setContextMenu(menu);
            tray->show();

            QObject::connect(tray, &QSystemTrayIcon::activated, [this](QSystemTrayIcon::ActivationReason reason) {
                if (reason == QSystemTrayIcon::Trigger) {
                    event_queue.push_back({AppEventType::TrayClicked, 0});
                } else if (reason == QSystemTrayIcon::DoubleClick) {
                    event_queue.push_back({AppEventType::TrayDoubleClicked, 0});
                }
            });
        }

        return app->exec();
    }

    AppEvent pollEvent() {
        if (event_queue.empty()) {
            return {AppEventType::None, 0};
        }
        AppEvent event = event_queue.front();
        event_queue.erase(event_queue.begin());
        return event;
    }

    void addTrayMenuItem(const std::string& text, int id) {
        if (menu) {
            QAction* action = menu->addAction(QString::fromStdString(text));
            QObject::connect(action, &QAction::triggered, [this, id]() {
                event_queue.push_back({AppEventType::MenuItemClicked, id});
            });
        }
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
    return {AppEventType::None, 0};
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

void add_tray_menu_item(QtAppHandle* handle, const char* text, int id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    }
}

} // extern "C"

