// src/modules/lx_dos/vm/device/graphics.rs

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuGraphics {
    driver: GraphicsDriver,
}

#[derive(Debug)]
pub enum GraphicsDriver {
    Qxl,
    Vga,
    VirtIO,
}

impl QemuGraphics {
    pub fn new(driver: GraphicsDriver) -> Self {
        Self { driver }
    }
}

impl QemuArgs for QemuGraphics {
    fn to_qemu_args(&self) -> Vec<String> {
        match self.driver {
            GraphicsDriver::Qxl => vec!["-vga".to_string(), "qxl".to_string()],
            GraphicsDriver::Vga => vec!["-vga".to_string(), "std".to_string()],
            GraphicsDriver::VirtIO => vec!["-vga".to_string(), "virtio".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphics_qxl() {
        let graphics = QemuGraphics::new(GraphicsDriver::Qxl);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "qxl"]);
    }

    #[test]
    fn test_graphics_vga() {
        let graphics = QemuGraphics::new(GraphicsDriver::Vga);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "std"]);
    }

    #[test]
    fn test_graphics_virtio() {
        let graphics = QemuGraphics::new(GraphicsDriver::VirtIO);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "virtio"]);
    }
}