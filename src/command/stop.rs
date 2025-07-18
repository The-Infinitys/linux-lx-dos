use crate::LxDosError;
use crate::{read_pid, delete_pid_file};
use nix::sys::signal::{kill, SIGTERM};
use nix::unistd::Pid;

pub fn stop() -> Result<(), LxDosError> {
    let pid = match read_pid()? {
        Some(p) => p,
        None => return Err(LxDosError::Message("LX-DOS is not running (PID file not found).".to_string())),
    };

    println!("Attempting to stop LX-DOS with PID: {}", pid);
    match kill(Pid::from_raw(pid as i32), SIGTERM) {
        Ok(_) => {
            println!("Successfully sent SIGTERM to PID: {}", pid);
            delete_pid_file()?;
            println!("LX-DOS stopped.");
        },
        Err(e) => {
            return Err(LxDosError::Message(format!("Failed to send SIGTERM to PID {}: {}", pid, e)));
        }
    }

    Ok(())
}