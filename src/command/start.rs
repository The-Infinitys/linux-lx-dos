use crate::LxDosError;
use std::process::Command;
use std::fs::File;
use std::io::Write;

const PID_FILE: &str = "/tmp/lx-dos.pid";

pub fn start() -> Result<(), LxDosError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| LxDosError::Message(format!("Failed to get current executable path: {}", e)))?;

    let child = Command::new(current_exe)
        .arg("run")
        .spawn()
        .map_err(|e| LxDosError::Message(format!("Failed to spawn run command: {}", e)))?;

    let pid = child.id();
    let mut file = File::create(PID_FILE)
        .map_err(|e| LxDosError::Message(format!("Failed to create PID file: {}", e)))?;
    file.write_all(pid.to_string().as_bytes())
        .map_err(|e| LxDosError::Message(format!("Failed to write PID to file: {}", e)))?;

    println!("LX-DOS started with PID: {}", pid);
    Ok(())
}
