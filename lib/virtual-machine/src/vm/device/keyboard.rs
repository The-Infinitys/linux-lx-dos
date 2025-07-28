// src/modules/lx_dos/vm/device/keyboard.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmKeyboard {
    model: KeyboardModel,
}

#[derive(Debug)]
pub enum KeyboardModel {
    PS2,
    VirtIO,
    Usb,
}

impl VmKeyboard {
    pub fn new(model: KeyboardModel) -> Self {
        Self { model }
    }
}

impl VmArgs for VmKeyboard {
    fn to_vm_args(&self) -> Vec<String> {
        match self.model {
            KeyboardModel::PS2 => vec!["-device".to_string(), "ps2-kbd".to_string()],
            KeyboardModel::VirtIO => vec!["-device".to_string(), "virtio-keyboard".to_string()],
            KeyboardModel::Usb => vec!["-device".to_string(), "usb-kbd".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_ps2() {
        let keyboard = VmKeyboard::new(KeyboardModel::PS2);
        assert_eq!(keyboard.to_vm_args(), vec!["-device", "ps2-kbd"]);
    }

    #[test]
    fn test_keyboard_virtio() {
        let keyboard = VmKeyboard::new(KeyboardModel::VirtIO);
        assert_eq!(keyboard.to_vm_args(), vec!["-device", "virtio-keyboard"]);
    }

    #[test]
    fn test_keyboard_usb() {
        let keyboard = VmKeyboard::new(KeyboardModel::Usb);
        assert_eq!(keyboard.to_vm_args(), vec!["-device", "usb-kbd"]);
    }
}
