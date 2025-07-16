pub mod drive;
pub mod boot;
pub mod input;
pub mod audio;

pub trait QemuDevice {
    fn to_qemu_args(&self) -> Vec<String>;
}
