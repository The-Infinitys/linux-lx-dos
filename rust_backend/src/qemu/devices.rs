pub trait QemuDevice {
    fn to_qemu_args(&self) -> Vec<String>;
}
