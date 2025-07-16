use std::ffi::{c_char, CStr, CString};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::runtime::Runtime;

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
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-m")
        .arg("4096")
        .arg("-cpu")
        .arg("host")
        .arg("-enable-kvm")
        .arg("-smp")
        .arg("4")
        .arg("-hda")
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
