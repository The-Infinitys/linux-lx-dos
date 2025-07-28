// src/modules/lx_dos/vm/device/mouse.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmMouse {
    model: MouseModel,
}

#[derive(Debug)]
pub enum MouseModel {
    PS2,
    VirtIO,
    Usb,
}

impl VmMouse {
    pub fn new(model: MouseModel) -> Self {
        Self { model }
    }
}

impl VmArgs for VmMouse {
    fn to_vm_args(&self) -> Vec<String> {
        match self.model {
            MouseModel::PS2 => vec!["-device".to_string(), "ps2-mouse".to_string()],
            MouseModel::VirtIO => vec!["-device".to_string(), "virtio-mouse".to_string()],
            MouseModel::Usb => vec!["-device".to_string(), "usb-mouse".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_ps2() {
        let mouse = VmMouse::new(MouseModel::PS2);
        assert_eq!(mouse.to_vm_args(), vec!["-device", "ps2-mouse"]);
    }

    #[test]
    fn test_mouse_virtio() {
        let mouse = VmMouse::new(MouseModel::VirtIO);
        assert_eq!(mouse.to_vm_args(), vec!["-device", "virtio-mouse"]);
    }

    #[test]
    fn test_mouse_usb() {
        let mouse = VmMouse::new(MouseModel::Usb);
        assert_eq!(mouse.to_vm_args(), vec!["-device", "usb-mouse"]);
    }
}
