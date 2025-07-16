pub mod configure;
use configure::QemuMemory;

use crate::modules::lx_dos::vm::configure::{Architecture, QemuCpu};
#[derive(Default, Debug)]
pub struct QemuSystem {
    pub arch: Architecture,
    pub mem: QemuMemory,
    pub cpu: QemuCpu,
}
