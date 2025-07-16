// src/modules/lx_dos/vm/device/graphics.rs

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuGraphics {
    driver: GraphicsDriver,
}

#[derive(Debug)]
pub enum GraphicsDriver {
    QXL,
    VGA,
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
            GraphicsDriver::QXL => vec!["-vga".to_string(), "qxl".to_string()],
            GraphicsDriver::VGA => vec!["-vga".to_string(), "std".to_string()],
            GraphicsDriver::VirtIO => vec!["-vga".to_string(), "virtio".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphics_qxl() {
        let graphics = QemuGraphics::new(GraphicsDriver::QXL);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "qxl"]);
    }

    #[test]
    fn test_graphics_vga() {
        let graphics = QemuGraphics::new(GraphicsDriver::VGA);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "std"]);
    }

    #[test]
    fn test_graphics_virtio() {
        let graphics = QemuGraphics::new(GraphicsDriver::VirtIO);
        assert_eq!(graphics.to_qemu_args(), vec!["-vga", "virtio"]);
    }
}