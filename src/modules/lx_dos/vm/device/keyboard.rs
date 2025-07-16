// src/modules/lx_dos/vm/device/keyboard.rs

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuKeyboard {
    model: KeyboardModel,
}

#[derive(Debug)]
pub enum KeyboardModel {
    PS2,
    VirtIO,
    USB,
}

impl QemuKeyboard {
    pub fn new(model: KeyboardModel) -> Self {
        Self { model }
    }
}

impl QemuArgs for QemuKeyboard {
    fn to_qemu_args(&self) -> Vec<String> {
        match self.model {
            KeyboardModel::PS2 => vec!["-device".to_string(), "ps2-kbd".to_string()],
            KeyboardModel::VirtIO => vec!["-device".to_string(), "virtio-keyboard".to_string()],
            KeyboardModel::USB => vec!["-device".to_string(), "usb-kbd".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_ps2() {
        let keyboard = QemuKeyboard::new(KeyboardModel::PS2);
        assert_eq!(keyboard.to_qemu_args(), vec!["-device", "ps2-kbd"]);
    }

    #[test]
    fn test_keyboard_virtio() {
        let keyboard = QemuKeyboard::new(KeyboardModel::VirtIO);
        assert_eq!(keyboard.to_qemu_args(), vec!["-device", "virtio-keyboard"]);
    }

    #[test]
    fn test_keyboard_usb() {
        let keyboard = QemuKeyboard::new(KeyboardModel::USB);
        assert_eq!(keyboard.to_qemu_args(), vec!["-device", "usb-kbd"]);
    }
}
