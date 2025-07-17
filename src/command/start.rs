use crate::LxDosError;
use crate::qt_lx_dos;
use crate::handle_qt_event;
use crate::qt_lx_dos::register_event_callback;
pub fn start() -> Result<(), LxDosError> {
    unsafe {
        register_event_callback(Some(handle_qt_event));
        qt_lx_dos::run_qt_app();
    }
    Ok(())
}
