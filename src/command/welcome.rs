use crate::LxDosError;
use crate::qt_lx_dos;
use crate::WELCOME_WINDOW_CLOSED_SENDER;
use crate::command::start;

pub fn welcome() -> Result<(), LxDosError> {
    unsafe {
        qt_lx_dos::show_welcome_window();
    }

    // Wait for the welcome window to be closed
    WELCOME_WINDOW_CLOSED_SENDER.1.lock().unwrap().recv()
        .map_err(|e| LxDosError::Message(format!("Failed to receive welcome window closed signal: {}", e)))?;

    println!("Welcome window closed. Starting LX-DOS...");
    start::start()
}
