// src/modules/lx_dos/vm/device/mouse.rs

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuMouse {
    model: MouseModel,
}

#[derive(Debug)]
pub enum MouseModel {
    PS2,
    VirtIO,
    USB,
}

impl QemuMouse {
    pub fn new(model: MouseModel) -> Self {
        Self { model }
    }
}

impl QemuArgs for QemuMouse {
    fn to_qemu_args(&self) -> Vec<String> {
        match self.model {
            MouseModel::PS2 => vec!["-device".to_string(), "ps2-mouse".to_string()],
            MouseModel::VirtIO => vec!["-device".to_string(), "virtio-mouse".to_string()],
            MouseModel::USB => vec!["-device".to_string(), "usb-mouse".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_ps2() {
        let mouse = QemuMouse::new(MouseModel::PS2);
        assert_eq!(mouse.to_qemu_args(), vec!["-device", "ps2-mouse"]);
    }

    #[test]
    fn test_mouse_virtio() {
        let mouse = QemuMouse::new(MouseModel::VirtIO);
        assert_eq!(mouse.to_qemu_args(), vec!["-device", "virtio-mouse"]);
    }

    #[test]
    fn test_mouse_usb() {
        let mouse = QemuMouse::new(MouseModel::USB);
        assert_eq!(mouse.to_qemu_args(), vec!["-device", "usb-mouse"]);
    }
}
