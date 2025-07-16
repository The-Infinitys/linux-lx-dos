pub struct QemuConfig {
    // Add common QEMU configuration options here
    pub memory: Option<u32>,
    pub cpu_cores: Option<u32>,
    pub enable_kvm: bool,
    pub system_architecture: Option<String>,
}

impl QemuConfig {
    pub fn new() -> Self {
        QemuConfig {
            memory: None,
            cpu_cores: None,
            enable_kvm: false,
            system_architecture: None,
        }
    }

    pub fn to_qemu_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        if let Some(mem) = self.memory {
            args.push("-m".to_string());
            args.push(format!("size={}M", mem));
        }
        if let Some(cores) = self.cpu_cores {
            args.push("-smp".to_string());
            args.push(format!("cores={}", cores));
        }
        if self.enable_kvm {
            args.push("-enable-kvm".to_string());
        }

        args
    }
}
