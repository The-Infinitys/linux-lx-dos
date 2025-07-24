#pragma once

#include <string>
#include <vector>
#include <QSystemTrayIcon>
#include <QMenu>
#include "qt-core.hpp" // Include core for AppEvent and QtCoreAppWrapper

// Internal C++ Implementation for System Tray Logic
class QtTrayWrapper {
public:
    // Constructor takes a pointer to the core wrapper to add events
    QtTrayWrapper(QtCoreAppWrapper* core_wrapper);
    ~QtTrayWrapper();

    void initTray(const QIcon& appIcon);
    void addTrayMenuItem(const std::string& text, const std::string& id_str);

private:
    QtCoreAppWrapper* core; // Pointer to the core application wrapper
    QMenu* menu = nullptr;
    QSystemTrayIcon* tray = nullptr;
    std::vector<std::pair<std::string, std::string>> pending_menu_items; // To store menu items added before initTray()
};
