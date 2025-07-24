#include "qt-app.hpp"
#include <QApplication>
#include <QIcon>
#include <QBuffer>
#include <string>
#include <vector>
#include <memory>
#include <QDebug> // Add for debugging

// --- Internal C++ Implementation ---

class QtAppWrapper {
public:
    QtAppWrapper() = default;

    void initApplication(int argc, char* argv[]) {
        app = new QApplication(argc, argv);
        qDebug() << "QApplication created.";

        if (!appId.empty()) {
            app->setApplicationName(QString::fromStdString(appId));
            qDebug() << "Application ID set to:" << QString::fromStdString(appId);
        }

        if (!iconData.isEmpty()) {
            QPixmap pixmap;
            if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                appIcon = QIcon(pixmap);
                app->setWindowIcon(appIcon);
                qDebug() << "Application icon loaded and set.";
            } else {
                qWarning() << "Failed to load application icon from data. Format:" << iconFormat.c_str();
            }
        } else {
            qDebug() << "No icon data provided.";
        }
    }

    void setAppId(const std::string& id) {
        appId = id;
    }

    void setAppIcon(const unsigned char* data, size_t size, const char* format) {
        iconData = QByteArray(reinterpret_cast<const char*>(data), size);
        iconFormat = format;
    }

    QIcon* getAppIcon() {
        if (appIcon.isNull()) {
            return nullptr;
        }
        return &appIcon;
    }

    int run() {
        qDebug() << "Starting QApplication event loop...";
        return app->exec();
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
    QIcon appIcon;

    // Qt Objects - Let Qt manage their lifetime.
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

void init_qt_application(QtAppHandle* handle, int argc, char* argv[]) {
    if (handle && handle->impl) {
        handle->impl->initApplication(argc, argv);
    }
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

void* get_app_icon(QtAppHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->getAppIcon();
    }
    return nullptr;
}

int run_qt_app(QtAppHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->run();
    }
    return -1;
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

} // extern "C"