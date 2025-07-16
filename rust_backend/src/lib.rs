mod qemu;
mod error;

use std::ffi::{c_char, CStr, CString};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;
use tokio::runtime::Runtime;
use std::process::Stdio;

pub use crate::error::QemuError;
use crate::qemu::{QemuMachine, config::QemuConfig};

// A handle to the running VM, allowing us to stop it.
pub struct VmHandle {
    child: Child,
}

// A thread-safe, shared handle to the VM.
type SharedVmHandle = Arc<Mutex<Option<VmHandle>>>;

// Callback function pointer type for logging.
type LogCallback = extern "C" fn(*const c_char);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_vm(
    disk_image_path: *const c_char,
    log_callback: LogCallback,
) -> *mut SharedVmHandle {
    let path_cstr = unsafe { CStr::from_ptr(disk_image_path) };
    let path = path_cstr.to_str().unwrap().to_owned();

    let vm_handle = Arc::new(Mutex::new(None));
    let vm_handle_clone = vm_handle.clone();

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            run_qemu_process(vm_handle_clone, path, log_callback).await;
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

async fn run_qemu_process(vm_handle: SharedVmHandle, disk_image: String, log_callback: LogCallback) {
    // Create a default QemuConfig for now. In a real application, this would come from user input.
    let mut qemu_config = QemuConfig::new();
    qemu_config.memory = Some(4096);
    qemu_config.cpu_cores = Some(4);
    qemu_config.enable_kvm = true;
    // qemu_config.system_architecture = Some("i386".to_string()); // Example: set a specific architecture

    let qemu_machine = QemuMachine::new(qemu_config);

    let mut cmd = match qemu_machine.build_command() {
        Ok(cmd) => cmd,
        Err(e) => {
            let error_msg = match e {
                QemuError::QemuBinaryNotFound(binary) => format!("QEMU binary not found: {}", binary),
                QemuError::IoError(io_err) => format!("IO Error: {}", io_err),
            };
            let c_error_msg = CString::new(error_msg).unwrap();
            log_callback(c_error_msg.as_ptr());
            return;
        }
    };

    cmd.arg("-hda")
        .arg(&disk_image)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            let error_msg = CString::new(format!("Failed to start QEMU: {}", e)).unwrap();
            log_callback(error_msg.as_ptr());
            return;
        }
    };

    let stdout = child.stdout.take().expect("Failed to open QEMU stdout");
    let stderr = child.stderr.take().expect("Failed to open QEMU stderr");

    *vm_handle.lock().unwrap() = Some(VmHandle { child });

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            Ok(Some(line)) = stdout_reader.next_line() => {
                if let Ok(c_line) = CString::new(line) {
                    log_callback(c_line.as_ptr());
                }
            }
            Ok(Some(line)) = stderr_reader.next_line() => {
                if let Ok(c_line) = CString::new(format!("ERROR: {}", line)) {
                    log_callback(c_line.as_ptr());
                }
            }
            else => break,
        }
    }
}
