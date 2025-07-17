use crate::LxDosError;
use crate::qt_lx_dos;
pub fn start() -> Result<(), LxDosError> {
    unsafe {
        qt_lx_dos::run_qt_app();
    }
    Ok(())
}
