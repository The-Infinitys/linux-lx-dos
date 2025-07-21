// src/modules/lx_dos/vm/device.rs
use std::fmt;

pub use super::QemuArgs;
pub use audio::QemuAudio;
pub use drive::QemuDrive;
pub use graphics::QemuGraphics;
pub use keyboard::QemuKeyboard;
pub use mouse::QemuMouse;
pub use usb::QemuUsb;
pub use video::QemuVideo;

mod audio;
mod drive;
mod graphics;
mod keyboard;
mod mouse;
mod usb;
mod video;

pub enum QemuDevice {
    Audio(QemuAudio),
    Usb(QemuUsb),
    Keyboard(QemuKeyboard),
    Mouse(QemuMouse),
    Graphics(QemuGraphics),
    Video(QemuVideo),
    Drive(QemuDrive),
}

impl QemuArgs for QemuDevice {
    fn to_qemu_args(&self) -> Vec<String> {
        match self {
            QemuDevice::Audio(device) => device.to_qemu_args(),
            QemuDevice::Usb(device) => device.to_qemu_args(),
            QemuDevice::Keyboard(device) => device.to_qemu_args(),
            QemuDevice::Mouse(device) => device.to_qemu_args(),
            QemuDevice::Graphics(device) => device.to_qemu_args(),
            QemuDevice::Video(device) => device.to_qemu_args(),
            QemuDevice::Drive(device) => device.to_qemu_args(),
        }
    }
}

impl fmt::Debug for QemuDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QemuDevice::Audio(device) => f.debug_struct("QemuDevice::Audio").field("device", device).finish(),
            QemuDevice::Usb(device) => f.debug_struct("QemuDevice::Usb").field("device", device).finish(),
            QemuDevice::Keyboard(device) => f.debug_struct("QemuDevice::Keyboard").field("device", device).finish(),
            QemuDevice::Mouse(device) => f.debug_struct("QemuDevice::Mouse").field("device", device).finish(),
            QemuDevice::Graphics(device) => f.debug_struct("QemuDevice::Graphics").field("device", device).finish(),
            QemuDevice::Video(device) => f.debug_struct("QemuDevice::Video").field("device", device).finish(),
            QemuDevice::Drive(device) => f.debug_struct("QemuDevice::Drive").field("device", device).finish(),
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
        let device = QemuDevice::Audio(QemuAudio::new(AudioModel::AC97));
        assert_eq!(device.to_qemu_args(), vec!["-soundhw", "ac97"]);
    }

    #[test]
    fn test_usb_device() {
        let device = QemuDevice::Usb(QemuUsb::new(true, None));
        assert_eq!(device.to_qemu_args(), vec!["-usb"]);
    }

    #[test]
    fn test_keyboard_device() {
        let device = QemuDevice::Keyboard(QemuKeyboard::new(KeyboardModel::VirtIO));
        assert_eq!(device.to_qemu_args(), vec!["-device", "virtio-keyboard"]);
    }

    #[test]
    fn test_mouse_device() {
        let device = QemuDevice::Mouse(QemuMouse::new(MouseModel::Usb));
        assert_eq!(device.to_qemu_args(), vec!["-device", "usb-mouse"]);
    }

    #[test]
    fn test_graphics_device() {
        let device = QemuDevice::Graphics(QemuGraphics::new(GraphicsDriver::Qxl));
        assert_eq!(device.to_qemu_args(), vec!["-vga", "qxl"]);
    }

    #[test]
    fn test_video_device() {
        let device = QemuDevice::Video(QemuVideo::new(VideoDisplay::Vnc { port: 5901 }));
        assert_eq!(device.to_qemu_args(), vec!["-vnc", ":1"]);
    }

    #[test]
    fn test_drive_device() {
        let device = QemuDevice::Drive(QemuDrive::new(
            PathBuf::from("/path/to/disk.img"),
            DriveFormat::QCow2,
            DriveMedia::Disk,
        ));
        assert_eq!(
            device.to_qemu_args(),
            vec!["-drive", "file=/path/to/disk.img,format=qcow2,media=disk"]
        );
    }
}