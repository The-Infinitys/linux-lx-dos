use crate::LxDosError;
use crate::save_pid;
use std::process::Command;

pub fn start() -> Result<(), LxDosError> {
    save_pid()?;

    let current_exe = std::env::current_exe()
        .map_err(|e| LxDosError::Message(format!("Failed to get current executable path: {}", e)))?;

    // runコマンドをタスクトレイで動かすための準備
    // ここではまだタスクトレイのコードは実装しないが、将来的にここにGTKのタスクトレイアイコンを起動する処理を追加する
    let child = Command::new(current_exe)
        .arg("run")
        .spawn()
        .map_err(|e| LxDosError::Message(format!("Failed to spawn run command: {}", e)))?;

    println!("LX-DOS started with PID: {}", child.id());
    Ok(())
}