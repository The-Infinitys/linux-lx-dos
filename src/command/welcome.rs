use crate::LxDosError;
use crate::qt_lx_dos;
pub fn welcome() -> Result<(), LxDosError> {
    unsafe {
        qt_lx_dos::show_welcome_window();
    }
    Ok(())
}
