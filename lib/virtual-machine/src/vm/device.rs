// src/modules/lx_dos/vm/device.rs
use std::fmt;

pub use super::VmArgs;
pub use audio::VmAudio;
pub use drive::VmDrive;
pub use graphics::VmGraphics;
pub use keyboard::VmKeyboard;
pub use mouse::VmMouse;
pub use usb::VmUsb;
pub use video::VmVideo;

mod audio;
mod drive;
mod graphics;
mod keyboard;
mod mouse;
mod usb;
mod video;

pub enum VmDevice {
    Audio(VmAudio),
    Usb(VmUsb),
    Keyboard(VmKeyboard),
    Mouse(VmMouse),
    Graphics(VmGraphics),
    Video(VmVideo),
    Drive(VmDrive),
}

impl VmArgs for VmDevice {
    fn to_vm_args(&self) -> Vec<String> {
        match self {
            VmDevice::Audio(device) => device.to_vm_args(),
            VmDevice::Usb(device) => device.to_vm_args(),
            VmDevice::Keyboard(device) => device.to_vm_args(),
            VmDevice::Mouse(device) => device.to_vm_args(),
            VmDevice::Graphics(device) => device.to_vm_args(),
            VmDevice::Video(device) => device.to_vm_args(),
            VmDevice::Drive(device) => device.to_vm_args(),
        }
    }
}

impl fmt::Debug for VmDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmDevice::Audio(device) => f
                .debug_struct("VmDevice::Audio")
                .field("device", device)
                .finish(),
            VmDevice::Usb(device) => f
                .debug_struct("VmDevice::Usb")
                .field("device", device)
                .finish(),
            VmDevice::Keyboard(device) => f
                .debug_struct("VmDevice::Keyboard")
                .field("device", device)
                .finish(),
            VmDevice::Mouse(device) => f
                .debug_struct("VmDevice::Mouse")
                .field("device", device)
                .finish(),
            VmDevice::Graphics(device) => f
                .debug_struct("VmDevice::Graphics")
                .field("device", device)
                .finish(),
            VmDevice::Video(device) => f
                .debug_struct("VmDevice::Video")
                .field("device", device)
                .finish(),
            VmDevice::Drive(device) => f
                .debug_struct("VmDevice::Drive")
                .field("device", device)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio::AudioModel;
    use drive::{DriveFormat, DriveMedia};
    use graphics::GraphicsDriver;
    use keyboard::KeyboardModel;
    use mouse::MouseModel;
    use std::path::PathBuf;
    use video::VideoDisplay;

    #[test]
    fn test_audio_device() {
        let device = VmDevice::Audio(VmAudio::new(AudioModel::AC97));
        assert_eq!(device.to_vm_args(), vec!["-soundhw", "ac97"]);
    }

    #[test]
    fn test_usb_device() {
        let device = VmDevice::Usb(VmUsb::new(true, None));
        assert_eq!(device.to_vm_args(), vec!["-usb"]);
    }

    #[test]
    fn test_keyboard_device() {
        let device = VmDevice::Keyboard(VmKeyboard::new(KeyboardModel::VirtIO));
        assert_eq!(device.to_vm_args(), vec!["-device", "virtio-keyboard"]);
    }

    #[test]
    fn test_mouse_device() {
        let device = VmDevice::Mouse(VmMouse::new(MouseModel::Usb));
        assert_eq!(device.to_vm_args(), vec!["-device", "usb-mouse"]);
    }

    #[test]
    fn test_graphics_device() {
        let device = VmDevice::Graphics(VmGraphics::new(GraphicsDriver::Qxl));
        assert_eq!(device.to_vm_args(), vec!["-vga", "qxl"]);
    }

    #[test]
    fn test_video_device() {
        let device = VmDevice::Video(VmVideo::new(VideoDisplay::Vnc { port: 5901 }));
        assert_eq!(device.to_vm_args(), vec!["-vnc", ":1"]);
    }

    #[test]
    fn test_drive_device() {
        let device = VmDevice::Drive(VmDrive::new(
            PathBuf::from("/path/to/disk.img"),
            DriveFormat::QCow2,
            DriveMedia::Disk,
        ));
        assert_eq!(
            device.to_vm_args(),
            vec!["-drive", "file=/path/to/disk.img,format=qcow2,media=disk"]
        );
    }
}
