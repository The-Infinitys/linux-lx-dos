pub mod configure;
pub mod device;
use configure::QemuMemory;

use crate::modules::lx_dos::vm::configure::{Architecture, QemuCpu};
#[derive(Default, Debug)]
pub struct QemuSystem {
    pub arch: Architecture,
    pub mem: QemuMemory,
    pub cpu: QemuCpu,
}

pub trait QemuArgs {
    fn to_qemu_args(&self) -> Vec<String>;
}
