use crate::qemu::devices::QemuDevice;

pub struct HardDiskDevice {
    path: String,
    interface: String,
}

impl HardDiskDevice {
    pub fn new(path: String, interface: String) -> Self {
        HardDiskDevice { path, interface }
    }
}

impl QemuDevice for HardDiskDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        vec![format!("-drive file={},if={}", self.path, self.interface)]
    }
}
