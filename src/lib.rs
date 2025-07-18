use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;

pub mod command;
pub mod modules;
pub mod utils;
pub use utils::error::LxDosError;

fn get_pid_file_path() -> Result<PathBuf, LxDosError> {
    let user = std::env::var("USER").map_err(|e| {
        LxDosError::Message(format!("Failed to get USER environment variable: {}", e))
    })?;
    let tmp_dir = PathBuf::from("/tmp");
    let user_tmp_dir = tmp_dir.join(&user);
    let pid_file_path = user_tmp_dir.join("lx-dos.pid");

    if !user_tmp_dir.exists() {
        fs::create_dir_all(&user_tmp_dir)?;
    }
    Ok(pid_file_path)
}

pub fn save_pid() -> Result<(), LxDosError> {
    let pid_file_path = get_pid_file_path()?;
    let pid = process::id();
    let mut file = fs::File::create(&pid_file_path)?;
    file.write_all(pid.to_string().as_bytes())?;
    Ok(())
}

pub fn read_pid() -> Result<Option<u32>, LxDosError> {
    let pid_file_path = get_pid_file_path()?;
    if pid_file_path.exists() {
        let pid_str = fs::read_to_string(&pid_file_path)?;
        let pid = pid_str
            .trim()
            .parse::<u32>()
            .map_err(|e| LxDosError::Message(format!("Failed to parse PID from file: {}", e)))?;
        Ok(Some(pid))
    } else {
        Ok(None)
    }
}

pub fn delete_pid_file() -> Result<(), LxDosError> {
    let pid_file_path = get_pid_file_path()?;
    if pid_file_path.exists() {
        fs::remove_file(&pid_file_path)?;
    }
    Ok(())
}
