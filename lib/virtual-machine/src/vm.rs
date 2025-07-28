/// src/vm.rs
pub mod configure;
pub mod device;

use configure::{Architecture, VmCpu, VmMemory};
use device::VmDevice;

pub trait VmArgs {
    fn to_vm_args(&self) -> Vec<String>;
}

#[derive(Debug, Default)]
pub struct VmSystem {
    pub arch: Architecture,
    pub mem: VmMemory,
    pub cpu: VmCpu,
    pub devices: Vec<VmDevice>,
}

impl VmArgs for VmSystem {
    fn to_vm_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // アーキテクチャ (-M)
        args.extend(vec!["-M".to_string(), self.arch.to_string()]);

        // メモリ (-m)
        args.extend(self.mem.to_vm_args());

        // CPU (-cpu, -smp)
        args.extend(self.cpu.to_vm_args());

        // デバイス
        for device in &self.devices {
            args.extend(device.to_vm_args());
        }

        args
    }
}
