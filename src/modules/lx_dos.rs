pub mod vm;

#[derive(Default, Debug)]
pub struct LxDos {
    pub vm: vm::QemuSystem,
}
