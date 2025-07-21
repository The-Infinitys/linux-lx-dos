pub mod command;
pub mod modules;
pub mod utils;
pub use utils::error::LxDosError;
pub mod qt_tray {
    #[allow(warnings)]
    mod ffi {
        include!(concat!(env!("OUT_DIR"), "/qt-tray_bindings.rs"));
    }
}
