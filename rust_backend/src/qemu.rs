pub mod config;
pub mod devices;

use tokio::process::Command;
use crate::qemu::config::QemuConfig;
use crate::qemu::devices::QemuDevice;
use crate::QemuError;

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
