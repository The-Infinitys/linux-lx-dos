use std::ffi::CStr;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod error;
mod qemu;

pub use crate::error::QemuError;
pub use crate::qemu::{LogCallback, QemuConfig, QemuMachine, SharedVmHandle, VmHandle};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_vm(
    disk_image_path: *const std::ffi::c_char,
    log_callback: LogCallback,
) -> *mut SharedVmHandle {
    let path_cstr = unsafe { CStr::from_ptr(disk_image_path) };
    let path = path_cstr.to_str().unwrap().to_owned();

    let vm_handle = SharedVmHandle::new(std::sync::Mutex::new(None));
    let vm_handle_clone = vm_handle.clone();

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Create a default QemuConfig for now. In a real application, this would come from user input.
            let mut qemu_config = QemuConfig::new();
            qemu_config.memory = Some(4096);
            qemu_config.cpu_cores = Some(4);
            qemu_config.enable_kvm = true;
            // qemu_config.system_architecture = Some("i386".to_string()); // Example: set a specific architecture

            let mut qemu_machine = QemuMachine::new(qemu_config);

            // Add devices
            qemu_machine.add_device(Box::new(
                crate::qemu::devices::drive::hard_disk::HardDiskDevice::new(
                    path.clone().into(),
                    "virtio".into(),
                    "qcow2".into(),
                ),
            ));
            qemu_machine.add_device(Box::new(crate::qemu::devices::boot::bios::BiosDevice::new(
                None,
            ))); // Use default BIOS/UEFI
            qemu_machine.add_device(Box::new(crate::qemu::devices::input::InputDevice::new(
                "usb-kbd".to_string(),
            )));
            qemu_machine.add_device(Box::new(crate::qemu::devices::input::InputDevice::new(
                "usb-mouse".to_string(),
            )));
            qemu_machine.add_device(Box::new(crate::qemu::devices::audio::AudioDevice::new(
                "pa".to_string(),
                Some("ac97".to_string()),
            )));

            qemu_machine.run(vm_handle_clone, path, log_callback).await;
        });
    });

    Arc::into_raw(vm_handle) as *mut _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_vm(handle_ptr: *mut SharedVmHandle) {
    if handle_ptr.is_null() {
        return;
    }
    let handle_arc = unsafe { Arc::from_raw(handle_ptr) };
    let mut handle_lock = handle_arc.lock().unwrap();
    if let Some(mut vm_handle) = handle_lock.take() {
        if vm_handle.child.start_kill().is_err() {
            // Could log this error in the future.
        }
    }
}
