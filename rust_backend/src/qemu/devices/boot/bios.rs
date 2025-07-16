use std::path::PathBuf;

use crate::qemu::devices::QemuDevice;

pub struct BiosDevice {
    file: Option<PathBuf>,
}

impl BiosDevice {
    pub fn new(file: Option<PathBuf>) -> Self {
        BiosDevice { file }
    }
}

impl QemuDevice for BiosDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(file) = &self.file {
            args.push("-bios".to_string());
            args.push(format!("{}", file.display()));
        }
        // Add logic for other BIOS/UEFI options
        args
    }
}
