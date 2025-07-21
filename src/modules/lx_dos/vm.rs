// src/modules/lx_dos/vm.rs

pub mod configure;
pub mod device;

use configure::{Architecture, QemuCpu, QemuMemory};
use device::QemuDevice;

pub trait QemuArgs {
    fn to_qemu_args(&self) -> Vec<String>;
}

#[derive(Debug)]
#[derive(Default)]
pub struct QemuSystem {
    pub arch: Architecture,
    pub mem: QemuMemory,
    pub cpu: QemuCpu,
    pub devices: Vec<QemuDevice>,
}


impl QemuArgs for QemuSystem {
    fn to_qemu_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // アーキテクチャ (-M)
        args.extend(vec!["-M".to_string(), self.arch.to_string()]);

        // メモリ (-m)
        args.extend(self.mem.to_qemu_args());

        // CPU (-cpu, -smp)
        args.extend(self.cpu.to_qemu_args());

        // デバイス
        for device in &self.devices {
            args.extend(device.to_qemu_args());
        }

        args
    }
}
