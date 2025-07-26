use qemu_system::vm;
#[derive(Default, Debug)]
pub struct LxDos {
    pub vm: vm::QemuSystem,
}
