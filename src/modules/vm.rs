#[derive(Default, Debug)]
pub struct QemuSystem {
    pub arch: Architecture,
}
#[derive(Debug)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl Default for Architecture {
    fn default() -> Self {
        match std::env::consts::ARCH {
            "x86_64" => Self::X86_64,
            "arm64" => Self::Arm64,
            // fall back
            _ => Self::X86_64,
        }
    }
}
