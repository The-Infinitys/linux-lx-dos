use std::path::PathBuf;

use crate::qemu::devices::QemuDevice;

pub struct HardDiskDevice {
    path: PathBuf,
    interface: String,
    format: String,
}

impl HardDiskDevice {
    pub fn new(path: PathBuf, interface: String, format: String) -> Self {
        HardDiskDevice {
            path,
            interface,
            format,
        }
    }
}

impl QemuDevice for HardDiskDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        vec![
            "-drive".to_string(),
            format!(
                "format={},file={},if={}",
                self.format,
                self.path.display(),
                self.interface,
            ),
        ]
    }
}
