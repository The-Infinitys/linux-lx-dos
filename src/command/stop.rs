use crate::LxDosError;
use std::fs;
use std::io::{self, Read};
use nix::sys::signal::{kill, SIGTERM};
use nix::unistd::Pid;

const PID_FILE: &str = "/tmp/lx-dos.pid";

pub fn stop() -> Result<(), LxDosError> {
    let mut pid_str = String::new();
    let pid = match fs::File::open(PID_FILE) {
        Ok(mut file) => {
            file.read_to_string(&mut pid_str)
                .map_err(|e| LxDosError::Message(format!("Failed to read PID from file: {}", e)))?;
            pid_str.trim().parse::<i32>()
                .map_err(|e| LxDosError::Message(format!("Failed to parse PID: {}", e)))?
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(LxDosError::Message("LX-DOS is not running (PID file not found).".to_string()));
        },
        Err(e) => {
            return Err(LxDosError::Message(format!("Failed to open PID file: {}", e)));
        }
    };

    println!("Attempting to stop LX-DOS with PID: {}", pid);
    match kill(Pid::from_raw(pid), SIGTERM) {
        Ok(_) => {
            println!("Successfully sent SIGTERM to PID: {}", pid);
            fs::remove_file(PID_FILE)
                .map_err(|e| LxDosError::Message(format!("Failed to remove PID file: {}", e)))?;
            println!("LX-DOS stopped.");
        },
        Err(e) => {
            return Err(LxDosError::Message(format!("Failed to send SIGTERM to PID {}: {}", pid, e)));
        }
    }

    Ok(())
}
