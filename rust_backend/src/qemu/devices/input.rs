use crate::qemu::devices::QemuDevice;

pub struct InputDevice {
    device_type: String, // e.g., "usb-kbd", "usb-mouse"
}

impl InputDevice {
    pub fn new(device_type: String) -> Self {
        InputDevice { device_type }
    }
}

impl QemuDevice for InputDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        vec!["-device".to_string(), format!("{}", self.device_type)]
    }
}
