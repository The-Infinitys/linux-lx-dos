#include "qt-core.hpp"
#include <QDebug>
#include <QPixmap>
#include <QApplication>
#include <QIcon>
#include <QByteArray>


// --- Internal C++ Implementation for Core Application Logic ---

QtCoreAppWrapper::QtCoreAppWrapper(int& argc, char* argv[]) {
    qDebug() << "QtCoreAppWrapper constructor: Creating QApplication.";
    app = new QApplication(argc, argv);
}

QtCoreAppWrapper::~QtCoreAppWrapper() {
    qDebug() << "QtCoreAppWrapper destructor: Deleting QApplication.";
    // QApplication is typically deleted by Qt's internal mechanisms when the event loop exits.
    // Explicitly deleting it here can cause issues if the event loop is still running or
    // if Qt objects are still alive. For now, rely on OS cleanup.
    // delete app; // Avoid explicit deletion here
}

void QtCoreAppWrapper::setAppId(const std::string& id) {
    appId = id;
    if (app) {
        app->setApplicationName(QString::fromStdString(appId));
        qDebug() << "Application ID set to:" << QString::fromStdString(appId);
    } else {
        qDebug() << "QApplication not yet created, App ID will be set after run().";
    }
}

void QtCoreAppWrapper::setAppIcon(const unsigned char* data, size_t size, const char* format) {
    iconData = QByteArray(reinterpret_cast<const char*>(data), size);
    iconFormat = format;
    if (app && !iconData.isEmpty()) {
        QPixmap pixmap;
        if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
            app->setWindowIcon(QIcon(pixmap));
            qDebug() << "Application icon loaded and set.";
        } else {
            qWarning() << "Failed to load application icon from data. Format:" << iconFormat.c_str();
        }
    } else {
        qDebug() << "No icon data provided or QApplication not yet created.";
    }
}

int QtCoreAppWrapper::run() {
    qDebug() << "QtCoreAppWrapper::run: Starting QApplication event loop...";
    if (appId.empty()) {
        qWarning() << "Application ID is not set. It's recommended to set it using setAppId().";
    }
    if (iconData.isEmpty()) {
        qWarning() << "Application icon is not set. It's recommended to set it using setAppIcon().";
    }
    return app->exec();
}

void QtCoreAppWrapper::quitApp() {
    if (app) {
        qDebug() << "QtCoreAppWrapper::quitApp: Quitting QApplication.";
        app->quit();
    }
}

AppEvent QtCoreAppWrapper::pollEvent() {
    if (event_queue.empty()) {
        return {AppEventType::None, nullptr};
    }
    AppEvent event = event_queue.front();
    event_queue.erase(event_queue.begin());
    return event;
}

void QtCoreAppWrapper::addEventToQueue(AppEvent event) {
    event_queue.push_back(event);
    qDebug() << "Event added to queue. Type:" << static_cast<int>(event.type);
}
