#include "modules/qt-app.hpp"
#include <vector>
#include <string>

// A simple dummy SVG icon for testing using a raw string literal
const std::string svg_icon_data = R"(
<svg width="64" height="64" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
  <rect width="64" height="64" rx="8" ry="8" fill="#2ecc71" />
  <text x="32" y="42" font-family="monospace" font-size="24" fill="white" text-anchor="middle">OK</text>
</svg>
)";

int main(int argc, char* argv[]) {
    QtAppHandle* app = create_qt_app();

    set_app_id(app, "com.example.TestApp");

    set_app_icon_from_data(
        app, 
        reinterpret_cast<const unsigned char*>(svg_icon_data.c_str()), 
        svg_icon_data.length(), 
        "SVG"
    );

    init_tray(app);

    int result = run_qt_app(app, argc, argv);

    // The cleanup function is now responsible for freeing the handle and the wrapper,
    // but not the Qt objects themselves.
    cleanup_qt_app(app);

    return result;
}
