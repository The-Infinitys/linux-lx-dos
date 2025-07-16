pub mod devices;
pub mod config;

// Placeholder for QEMU related functions
pub fn build_qemu_command() -> Vec<String> {
    vec!["qemu-system-x86_64".to_string()]
}
