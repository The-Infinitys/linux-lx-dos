use crate::qemu::devices::QemuDevice;

pub struct BiosUefiDevice {
    file: Option<String>,
    // Add more options as needed, e.g., for secure boot
}

impl BiosUefiDevice {
    pub fn new(file: Option<String>) -> Self {
        BiosUefiDevice { file }
    }
}

impl QemuDevice for BiosUefiDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(file) = &self.file {
            args.push(format!("-bios {}", file));
        }
        // Add logic for other BIOS/UEFI options
        args
    }
}
