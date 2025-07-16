pub mod hard_disk;
pub mod bios_uefi;
pub mod input;
pub mod audio;

pub trait QemuDevice {
    fn to_qemu_args(&self) -> Vec<String>;
}
