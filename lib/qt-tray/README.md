# qt-tray

A C++ library for creating system tray icons using Qt, designed with a C-style API for easy integration into various applications.

## Features

-   **System Tray Icon Management:** Easily create and destroy system tray icons.
-   **Customization:** Set custom icons and tooltips for the tray icon.
-   **Context Menus:** Add menu items to the tray icon's context menu with custom callbacks.
-   **Event Handling:** Register callbacks for tray icon activation events (e.g., single click, double click).

## Building

This project uses CMake for its build system. Ensure you have CMake (version 3.16 or higher) and Qt6 development libraries installed.

```bash
mkdir build
cd build
cmake ..
make
```

### CMake Options

-   `-DENABLE_LINTER=ON/OFF`: Enable or disable `clang-tidy` for static analysis (default: `ON`).
-   `-DENABLE_FORMATTER=ON/OFF`: Enable or disable `clang-format` for code formatting (default: `ON`).

## Usage (C API)

The library exposes a C-style API, making it suitable for integration with other languages or environments that can interface with C functions.

### Example

```c
#include <stdio.h>
#include <stdlib.h>
#include "lib.hpp" // Include the C API header

void my_menu_callback(void* user_data) {
    printf("Menu item clicked! User data: %p\n", user_data);
}

void my_activated_callback(int reason, void* user_data) {
    printf("Tray icon activated! Reason: %d, User data: %p\n", reason, user_data);
    if (reason == 3) { // QSystemTrayIcon::Trigger (single click)
        printf("Single click detected.\n");
    }
}

int main(int argc, char *argv[]) {
    // Create a new QtTray instance
    qt_tray::QtTray* tray = create_qt_tray();

    // Set icon (replace with a valid path to an icon file)
    qt_tray_set_icon(tray, "/path/to/your/icon.png");

    // Set tooltip
    qt_tray_set_tool_tip(tray, "My Awesome Tray App");

    // Add menu items
    qt_tray_add_menu_item(tray, "Open", my_menu_callback, (void*)1);
    qt_tray_add_menu_item(tray, "Settings", my_menu_callback, (void*)2);
    qt_tray_add_menu_item(tray, "Quit", [](void* user_data){ QCoreApplication::quit(); }, nullptr);

    // Register activation callback
    qt_tray_on_activated(tray, my_activated_callback, (void*)3);

    // Start the Qt event loop
    QCoreApplication::exec();

    // Destroy the tray instance when done
    destroy_qt_tray(tray);

    return 0;
}
```

## Dependencies

-   Qt6 Widgets
