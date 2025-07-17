// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

pub mod command;
pub mod modules;
pub mod utils;
pub mod qt_lx_dos {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use utils::error::LxDosError;

// Define a callback function type for Rust to receive events from C++
pub type RustEventCallback = unsafe extern "C" fn(event_name: *const std::os::raw::c_char, event_data: *const std::os::raw::c_char);

// Callback function to handle events from Qt
extern "C" fn handle_qt_event(event_name: *const std::os::raw::c_char, event_data: *const std::os::raw::c_char) {
    let event_name_str = unsafe { std::ffi::CStr::from_ptr(event_name).to_string_lossy().into_owned() };
    let event_data_str = unsafe { std::ffi::CStr::from_ptr(event_data).to_string_lossy().into_owned() };
    println!("Received event from Qt: name={}, data={}", event_name_str, event_data_str);

    if event_name_str == "main_window_closed" {
        println!("Main window was closed!");
        // ここでRust側での必要な処理を行う
    }
}

