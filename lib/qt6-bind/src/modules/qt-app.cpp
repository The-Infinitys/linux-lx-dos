#include "qt-app.hpp"
#include "qt-core.hpp"
#include "qt-tray.hpp"
#include <QDebug>
#include <QIcon> // For QIcon usage in QtAppWrapper

// --- Internal C++ Implementation ---

class QtAppWrapper {
public:
    QtAppWrapper(int& argc, char* argv[]) : core(new QtCoreAppWrapper(argc, argv)), tray(nullptr) {
        qDebug() << "QtAppWrapper constructor.";
    }

    ~QtAppWrapper() {
        qDebug() << "QtAppWrapper destructor.";
        delete tray; // Delete tray first as it depends on core's QApplication
        delete core;
    }

    void setAppId(const std::string& id) {
        if (core) {
            core->setAppId(id);
        }
    }

    void setAppIcon(const unsigned char* data, size_t size, const char* format) {
        if (core) {
            core->setAppIcon(data, size, format);
            // Store icon data for tray initialization later
            iconData = QByteArray(reinterpret_cast<const char*>(data), size);
            iconFormat = format;
        }
    }

    void initTray() {
        shouldInitTray = true;
    }

    int run() {
        qDebug() << "QtAppWrapper::run started.";

        // If tray is requested, initialize it here after QApplication is ready
        if (shouldInitTray) {
            if (!core) {
                qWarning() << "Core wrapper is null, cannot initialize tray.";
                return -1;
            }
            tray = new QtTrayWrapper(core);

            // Create QIcon from stored data for the tray
            QIcon appIcon;
            if (!iconData.isEmpty()) {
                QPixmap pixmap;
                if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                    appIcon = QIcon(pixmap);
                } else {
                    qWarning() << "Failed to load application icon for tray from data. Format:" << iconFormat.c_str();
                }
            } else {
                qDebug() << "No icon data provided for tray.";
            }
            tray->initTray(appIcon);
        }

        if (core) {
            return core->run();
        }
        return -1;
    }

    AppEvent pollEvent() {
        if (core) {
            return core->pollEvent();
        }
        return {AppEventType::None, nullptr};
    }

    void addTrayMenuItem(const std::string& text, const std::string& id_str) {
        if (shouldInitTray && tray) {
            tray->addTrayMenuItem(text, id_str);
        } else {
            // If tray is not yet initialized, store pending menu items
            // This assumes initTray() will be called later.
            // If initTray() is never called, these items will not be added.
            pending_tray_menu_items.push_back({text, id_str});
            qDebug() << "Tray not yet initialized, pending menu item for tray:" << QString::fromStdString(text);
        }
    }

    void quitApp() {
        if (core) {
            core->quitApp();
        }
    }

private:
    QtCoreAppWrapper* core;
    QtTrayWrapper* tray;
    bool shouldInitTray = false;

    // Temporary storage for icon data and format, used when creating tray
    QByteArray iconData;
    std::string iconFormat;

    // Store pending menu items if tray is not initialized when addTrayMenuItem is called
    std::vector<std::pair<std::string, std::string>> pending_tray_menu_items;
};

// Opaque handle that points to the C++ implementation
struct QtAppHandle {
    QtAppWrapper* impl;
    // Store argc and argv here to pass to QApplication constructor
    // Note: This is a simplification. In a real application, argc/argv management
    // can be more complex, especially with multiple calls to create_qt_app.
    // For this example, we assume run_qt_app is called once with the main argc/argv.
    int stored_argc;
    char** stored_argv;
};


// --- C-style API Implementation ---

extern "C" {

QtAppHandle* create_qt_app() {
    // We cannot create QApplication here as it needs argc/argv.
    // Instead, we will defer the creation of QtAppWrapper until run_qt_app.
    // For now, return a handle that will store argc/argv.
    QtAppHandle* handle = new QtAppHandle();
    handle->impl = nullptr; // Will be initialized in run_qt_app
    handle->stored_argc = 0; // Initialize to 0
    handle->stored_argv = nullptr; // Initialize to nullptr
    qDebug() << "create_qt_app: Handle created, impl is null.";
    return handle;
}

void set_app_id(QtAppHandle* handle, const char* id) {
    if (handle && handle->impl) {
        handle->impl->setAppId(id);
    } else if (handle) {
        // Store ID if impl not yet created. This is a common pattern for pre-run configs.
        // However, for this refactor, we are assuming set_app_id is called AFTER create_qt_app
        // but potentially BEFORE run_qt_app. The core wrapper handles setting ID on QApplication
        // when it's available.
        // For simplicity, we'll rely on the core wrapper's internal logic for now.
        qWarning() << "set_app_id called before run_qt_app, ID will be applied when QApplication is ready.";
        // A more robust solution might involve storing these values in QtAppHandle
        // and applying them when impl is created.
    }
}

void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format) {
    if (handle && handle->impl) {
        handle->impl->setAppIcon(data, size, format);
    } else if (handle) {
        qWarning() << "set_app_icon_from_data called before run_qt_app, icon will be applied when QApplication is ready.";
        // Similar to set_app_id, rely on core wrapper's internal logic for now.
        // QtAppWrapper now stores icon data temporarily for tray initialization.
    }
}

void init_tray(QtAppHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->initTray();
    } else if (handle) {
        // Mark that tray should be initialized when impl is created in run_qt_app
        // This flag is handled by QtAppWrapper's initTray() method.
        qWarning() << "init_tray called before run_qt_app. Tray will be initialized during run.";
        // The QtAppWrapper's `shouldInitTray` flag will handle this.
    }
}

int run_qt_app(QtAppHandle* handle, int argc, char* argv[]) {
    if (!handle) {
        qCritical() << "run_qt_app: Handle is null.";
        return -1;
    }

    if (!handle->impl) {
        qDebug() << "run_qt_app: Initializing QtAppWrapper with argc/argv.";
        // Store argc and argv for QApplication constructor
        handle->stored_argc = argc;
        handle->stored_argv = argv;
        handle->impl = new QtAppWrapper(handle->stored_argc, handle->stored_argv);

        // Apply any pending configurations that might have been set before run()
        // (e.g., set_app_id, set_app_icon_from_data, init_tray flags are handled internally by QtAppWrapper/QtCoreAppWrapper)
    }

    return handle->impl->run();
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
    if (handle) {
        qDebug() << "cleanup_qt_app: Deleting QtAppHandle and its implementation.";
        delete handle->impl; // This will call ~QtAppWrapper, which cleans up core and tray
        delete handle;
    }
}

void add_tray_menu_item(QtAppHandle* handle, const char* text, const char* id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    } else if (handle) {
        qWarning() << "add_tray_menu_item called before run_qt_app. Item will be added if tray is initialized later.";
        // QtAppWrapper now handles pending menu items if tray is not yet initialized.
    }
}

void free_char_ptr(const char* ptr) {
    free((void*)ptr);
}

} // extern "C"
