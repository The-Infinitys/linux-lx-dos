// src/modules/lx_dos/vm/device/graphics.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmGraphics {
    driver: GraphicsDriver,
}

#[derive(Debug)]
pub enum GraphicsDriver {
    Qxl,
    Vga,
    VirtIO,
}

impl VmGraphics {
    pub fn new(driver: GraphicsDriver) -> Self {
        Self { driver }
    }
}

impl VmArgs for VmGraphics {
    fn to_vm_args(&self) -> Vec<String> {
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
        let graphics = VmGraphics::new(GraphicsDriver::Qxl);
        assert_eq!(graphics.to_vm_args(), vec!["-vga", "qxl"]);
    }

    #[test]
    fn test_graphics_vga() {
        let graphics = VmGraphics::new(GraphicsDriver::Vga);
        assert_eq!(graphics.to_vm_args(), vec!["-vga", "std"]);
    }

    #[test]
    fn test_graphics_virtio() {
        let graphics = VmGraphics::new(GraphicsDriver::VirtIO);
        assert_eq!(graphics.to_vm_args(), vec!["-vga", "virtio"]);
    }
}
