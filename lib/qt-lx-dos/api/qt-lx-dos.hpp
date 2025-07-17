#ifndef C_API_H
#define C_API_H

#ifdef __cplusplus
extern "C" {
#endif

// Define a callback function type for Rust to receive events from C++
typedef void (*RustEventCallback)(const char* event_name, const char* event_data);

// Global callback instance
extern RustEventCallback global_event_callback;

// Function to register the Rust callback
void register_event_callback(RustEventCallback callback);

// Function for Rust to send commands to C++
void send_qt_command(const char* command_name, const char* command_data);

void run_qt_app();
void show_main_window();
void show_settings_window();
void show_welcome_window();

#ifdef __cplusplus
}
#endif

#endif // C_API_H
