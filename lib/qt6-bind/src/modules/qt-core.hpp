#pragma once

#include <stddef.h>
#include <string>
#include <vector>
#include <QApplication>
#include <QByteArray>
#ifdef __cplusplus
extern "C" {
#endif

// Enum for event types that can be polled from Rust
enum class AppEventType {
    None,
    TrayClicked,
    TrayDoubleClicked,
    MenuItemClicked
};

// Struct to hold event data
typedef struct {
    AppEventType type;
    const char* menu_id_str; // For MenuItemClicked events, now a string
} AppEvent;

#ifdef __cplusplus
}
#endif

// Internal C++ Implementation for Core Application Logic
class QtCoreAppWrapper {
public:
    QtCoreAppWrapper(int& argc, char* argv[]);
    ~QtCoreAppWrapper();

    void setAppId(const std::string& id);
    void setAppIcon(const unsigned char* data, size_t size, const char* format);
    int run();
    void quitApp();
    AppEvent pollEvent();

    // Method to allow other modules to add events to the queue
    void addEventToQueue(AppEvent event);

private:
    QApplication* app;
    std::string appId;
    QByteArray iconData;
    std::string iconFormat;
    std::vector<AppEvent> event_queue;
};

