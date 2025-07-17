use crate::LxDosError;
use crate::qt_lx_dos;
use crate::{handle_qt_event, RustEventCallback};

pub fn run() -> Result<(), LxDosError> {
    unsafe {
        qt_lx_dos::register_event_callback(Some(handle_qt_event as RustEventCallback));
        qt_lx_dos::run_qt_app();
    }
    Ok(())
}
