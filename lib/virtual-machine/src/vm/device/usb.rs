// src/modules/lx_dos/vm/device/usb.rs

use super::super::VmArgs;

#[derive(Debug)]
pub struct VmUsb {
    enable: bool,
    host_device: Option<String>,
}

impl VmUsb {
    pub fn new(enable: bool, host_device: Option<String>) -> Self {
        Self {
            enable,
            host_device,
        }
    }
}

impl VmArgs for VmUsb {
    fn to_vm_args(&self) -> Vec<String> {
        let mut args = if self.enable {
            vec!["-usb".to_string()]
        } else {
            vec![]
        };
        if let Some(device_id) = &self.host_device {
            args.extend(vec![
                "-device".to_string(),
                format!("usb-host,hostdevice={}", device_id),
            ]);
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_enabled() {
        let usb = VmUsb::new(true, None);
        assert_eq!(usb.to_vm_args(), vec!["-usb"]);
    }

    #[test]
    fn test_usb_with_host_device() {
        let usb = VmUsb::new(true, Some("/dev/bus/usb/001/002".to_string()));
        assert_eq!(
            usb.to_vm_args(),
            vec![
                "-usb",
                "-device",
                "usb-host,hostdevice=/dev/bus/usb/001/002"
            ]
        );
    }
}
