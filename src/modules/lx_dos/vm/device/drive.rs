// src/modules/lx_dos/vm/device/drive.rs
use std::path::PathBuf;

use super::super::QemuArgs;

#[derive(Debug)]
pub struct QemuDrive {
    file: PathBuf,
    format: DriveFormat,
    media: DriveMedia,
}

#[derive(Debug)]
pub enum DriveFormat {
    Raw,
    QCow2,
}

#[derive(Debug)]
pub enum DriveMedia {
    Disk,
    CdRom,
}

impl QemuDrive {
    pub fn new(file: PathBuf, format: DriveFormat, media: DriveMedia) -> Self {
        Self { file, format, media }
    }
}

impl QemuArgs for QemuDrive {
    fn to_qemu_args(&self) -> Vec<String> {
        let format_str = match self.format {
            DriveFormat::Raw => "raw",
            DriveFormat::QCow2 => "qcow2",
        };
        let media_str = match self.media {
            DriveMedia::Disk => "disk",
            DriveMedia::CdRom => "cdrom",
        };
        vec![
            "-drive".to_string(),
            format!(
                "file={},format={},media={}",
                self.file.display(),
                format_str,
                media_str
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_disk() {
        let drive = QemuDrive::new(
            PathBuf::from("/path/to/disk.img"),
            DriveFormat::QCow2,
            DriveMedia::Disk,
        );
        assert_eq!(
            drive.to_qemu_args(),
            vec!["-drive", "file=/path/to/disk.img,format=qcow2,media=disk"]
        );
    }

    #[test]
    fn test_drive_cdrom() {
        let drive = QemuDrive::new(
            PathBuf::from("/path/to/cdrom.iso"),
            DriveFormat::Raw,
            DriveMedia::CdRom,
        );
        assert_eq!(
            drive.to_qemu_args(),
            vec!["-drive", "file=/path/to/cdrom.iso,format=raw,media=cdrom"]
        );
    }
}
