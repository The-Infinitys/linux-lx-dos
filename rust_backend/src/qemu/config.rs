use super::devices::{ide::IdeDevice, usb::UsbDevice};

// Main QEMU configuration structure
pub struct QemuConfig {
    pub machine_type: Option<String>,
    pub cpu_model: Option<String>,
    pub memory_mb: u32,
    pub ide_devices: Vec<IdeDevice>,
    pub usb_devices: Vec<UsbDevice>,
    // Add other configurations as needed
}

impl Default for QemuConfig {
    fn default() -> Self {
        QemuConfig {
            machine_type: Some("pc".to_string()),
            cpu_model: Some("host".to_string()),
            memory_mb: 1024, // Default to 1GB
            ide_devices: Vec::new(),
            usb_devices: Vec::new(),
        }
    }
}

impl QemuConfig {
    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        if let Some(machine) = &self.machine_type {
            args.push("-machine".to_string());
            args.push(machine.clone());
        }

        if let Some(cpu) = &self.cpu_model {
            args.push("-cpu".to_string());
            args.push(cpu.clone());
        }

        args.push("-m".to_string());
        args.push(format!("{}M", self.memory_mb));

        for ide_dev in &self.ide_devices {
            args.push("-drive".to_string());
            args.push(format!("file={},index={},media={}", ide_dev.path, ide_dev.index, ide_dev.media));
        }

        if !self.usb_devices.is_empty() {
            args.push("-usb".to_string());
            for usb_dev in &self.usb_devices {
                args.push("-usbdevice".to_string());
                args.push(usb_dev.device_name.clone());
            }
        }

        args
    }
}
