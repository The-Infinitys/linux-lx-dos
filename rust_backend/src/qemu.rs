pub mod config;
pub mod devices;

use tokio::process::Command;
pub use crate::qemu::config::QemuConfig;
pub use crate::qemu::devices::QemuDevice;
pub use crate::QemuError;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;
use std::process::Stdio;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

pub struct VmHandle {
    pub child: Child,
}

pub type SharedVmHandle = Arc<Mutex<Option<VmHandle>>>;

pub type LogCallback = extern "C" fn(*const std::ffi::c_char);

pub struct QemuMachine {
    pub config: QemuConfig,
    pub devices: Vec<Box<dyn QemuDevice>>,
}

impl QemuMachine {
    pub fn new(config: QemuConfig) -> Self {
        QemuMachine {
            config,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn QemuDevice>) {
        self.devices.push(device);
    }

    pub async fn run(&mut self, vm_handle: SharedVmHandle, disk_image: String, log_callback: LogCallback) {
        let mut cmd = match self.build_command() {
            Ok(cmd) => cmd,
            Err(e) => {
                let error_msg = match e {
                    QemuError::QemuBinaryNotFound(binary) => format!("QEMU binary not found: {}", binary),
                    QemuError::IoError(io_err) => format!("IO Error: {}", io_err),
                };
                let c_error_msg = CString::new(error_msg).unwrap();
                log_callback(c_error_msg.as_ptr());
                return;
            }
        };

        cmd.arg("-hda")
            .arg(&disk_image)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                let error_msg = CString::new(format!("Failed to start QEMU: {}", e)).unwrap();
                log_callback(error_msg.as_ptr());
                return;
            }
        };

        let stdout = child.stdout.take().expect("Failed to open QEMU stdout");
        let stderr = child.stderr.take().expect("Failed to open QEMU stderr");

        *vm_handle.lock().unwrap() = Some(VmHandle { child });

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        loop {
            tokio::select! {
                Ok(Some(line)) = stdout_reader.next_line() => {
                    if let Ok(c_line) = CString::new(line) {
                        log_callback(c_line.as_ptr());
                    }
                }
                Ok(Some(line)) = stderr_reader.next_line() => {
                    if let Ok(c_line) = CString::new(format!("ERROR: {}", line)) {
                        log_callback(c_line.as_ptr());
                    }
                }
                else => break,
            }
        }
    }

    pub fn build_command(&self) -> Result<Command, QemuError> {
        let arch = self.config.system_architecture.as_deref().unwrap_or("x86_64");
        let qemu_binary = format!("qemu-system-{}", arch);

        // Check if the QEMU binary exists in PATH
        if which::which(&qemu_binary).is_err() {
            return Err(QemuError::QemuBinaryNotFound(qemu_binary));
        }

        let mut command = Command::new(qemu_binary);

        // Apply config arguments
        command.args(self.config.to_qemu_args());

        // Apply device arguments
        for device in &self.devices {
            command.args(device.to_qemu_args());
        }

        Ok(command)
    }
}
