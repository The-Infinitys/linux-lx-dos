use std::ffi::{c_char, CString};
use std::marker::PhantomData;

// The bindgen-generated bindings will be included here.
#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bind {
    include!(concat!(env!("OUT_DIR"), "/qt6-bind.rs"));
}

#[derive(Debug, thiserror::Error)]
pub enum Qt6Error {
    #[error("Failed to create C-style string: {0}")]
    Ffi(#[from] std::ffi::NulError),
    #[error("Qt application failed to run, returned exit code {0}")]
    RunFailed(i32),
    #[error("Failed to poll event: {0}")]
    PollEventError(String),
    #[error("Qt Instance has already made an didn't dropped.")]
    QtInstanceError,
}

/// Events that can be received from the Qt application.
pub enum QtAppEvent {
    /// No event occurred.
    None,
    /// The system tray icon was clicked.
    TrayClicked,
    /// The system tray icon was double-clicked.
    TrayDoubleClicked,
    /// A menu item in the system tray was clicked, with the given ID.
    MenuItemClicked(String),
}

// ★ここから新しいラッパー型の定義★
/// A safe wrapper around `*mut bind::QtAppHandle` that asserts `Send` safety.
/// This is necessary to satisfy Rust's Orphan Rules and explicitly state
/// the thread-safety guarantees.
#[repr(transparent)] // Ensures SafeQtAppHandle has the same memory layout as *mut bind::QtAppHandle
#[derive(Clone, Copy)]
pub struct SafeQtAppHandle(*mut bind::QtAppHandle);

// Implement Send for SafeQtAppHandle, asserting that it's safe to transfer
// this handle between threads. This is based on the assumption that
// Qt's QApplication handle, when passed to run_qt_app, is managed exclusively
// by the thread running the event loop.
unsafe impl Send for SafeQtAppHandle {}
// unsafe impl Sync for SafeQtAppHandle {} // If you need to share references across threads

impl SafeQtAppHandle {
    /// Creates a new `SafeQtAppHandle` from a raw pointer.
    /// This function is unsafe because the caller must ensure the pointer is valid.
    pub unsafe fn new(ptr: *mut bind::QtAppHandle) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut bind::QtAppHandle {
        self.0
    }
}
// ★ここまで新しいラッパー型の定義★

/// Represents a running Qt application instance.
pub struct QtAppInstance {
    // 型をSafeQtAppHandleに変更
    handle: SafeQtAppHandle,
    _join_handle: std::thread::JoinHandle<()>, // To keep the thread alive
}

impl QtAppInstance {
    /// Polls for the next event from the Qt application.
    pub fn poll_event(&self) -> Result<QtAppEvent, Qt6Error> {
        // SafeQtAppHandleから生ポインタを取り出す
        let event = unsafe { bind::poll_event(self.handle.as_ptr()) };
        match event.type_ {
            bind::AppEventType_None => Ok(QtAppEvent::None),
            bind::AppEventType_TrayClicked => Ok(QtAppEvent::TrayClicked),
            bind::AppEventType_TrayDoubleClicked => Ok(QtAppEvent::TrayDoubleClicked),
            bind::AppEventType_MenuItemClicked => {
                let c_str = unsafe { CString::from_raw(event.menu_id_str as *mut c_char) };
                let rust_str = c_str.to_string_lossy().into_owned();
                Ok(QtAppEvent::MenuItemClicked(rust_str))
            }
            _ => Err(Qt6Error::PollEventError("Unknown event type".to_string())),
        }
    }

    /// Signals the Qt application to quit.
    pub fn quit(&self) {
        // SafeQtAppHandleから生ポインタを取り出す
        unsafe {
            bind::quit_qt_app(self.handle.as_ptr());
        }
    }
}

impl Drop for QtAppInstance {
    fn drop(&mut self) {
        // Ensure the Qt app thread is joined when the instance is dropped.
        // This might block if the Qt app is still running.
        // In a real application, you might want a more graceful shutdown.
        // For now, we just ensure the thread is cleaned up.
        // Note: The C++ side handles cleanup of Qt objects when the app quits.
        // We only need to ensure the thread is joined.
        // The `cleanup_qt_app` is called when `QtApp` is dropped.
    }
}

/// Represents a Qt application configuration.
/// This struct uses the builder pattern to set up the application.
/// When dropped, it automatically cleans up the associated C++ resources.
pub struct QtApp<'a> {
    // 型をSafeQtAppHandleに変更
    pub handle: SafeQtAppHandle,
    // Use a phantom lifetime to tie the handle to this struct's lifetime.
    _marker: PhantomData<&'a ()>,
}

impl<'a> std::fmt::Debug for QtApp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QtApp")
            .field("handle", &self.handle.as_ptr()) // デバッグ出力も生ポインタを取り出す
            .finish()
    }
}

// QtAppInstanceはSafeQtAppHandleを含むので、SafeQtAppHandleがSendなら、QtAppInstanceもSendです。
// (ただし、_join_handleがSendであることも前提です)
unsafe impl Send for QtAppInstance {} // これは_join_handleがSendなので問題なし

// QtAppもSafeQtAppHandleを含むので、SafeQtAppHandleがSendなら、QtAppもSendです。
unsafe impl<'a> Send for QtApp<'a> {}

impl<'a> Default for QtApp<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> QtApp<'a> {
    /// Creates a new Qt application configuration.
    pub fn new() -> Self {
        let handle = unsafe { bind::create_qt_app() };
        // 生ポインタをSafeQtAppHandleでラップ
        Self {
            handle: unsafe { SafeQtAppHandle::new(handle) },
            _marker: PhantomData,
        }
    }

    /// Sets the application ID (e.g., "com.example.myapp").
    pub fn with_id(self, id: &str) -> Result<Self, Qt6Error> {
        let c_id = CString::new(id)?;
        unsafe {
            // SafeQtAppHandleから生ポインタを取り出す
            bind::set_app_id(self.handle.as_ptr(), c_id.as_ptr());
        }
        Ok(self)
    }

    /// Sets the application icon from raw binary data.
    ///
    /// # Arguments
    /// * `data` - The raw byte data of the icon (e.g., from `include_bytes!`).
    /// * `format` - The format of the icon, e.g., "PNG", "SVG".
    pub fn with_icon(self, data: &'static [u8], format: &str) -> Result<Self, Qt6Error> {
        let c_format = CString::new(format)?;
        unsafe {
            // SafeQtAppHandleから生ポインタを取り出す
            bind::set_app_icon_from_data(
                self.handle.as_ptr(),
                data.as_ptr(),
                data.len() as usize,
                c_format.as_ptr(),
            );
        }
        Ok(self)
    }

    /// Initializes a system tray icon for the application.
    pub fn with_tray(self) -> Self {
        unsafe {
            // SafeQtAppHandleから生ポインタを取り出す
            bind::init_tray(self.handle.as_ptr());
        }
        self
    }

    /// Adds a menu item to the system tray icon's context menu.
    pub fn add_tray_menu_item(&self, text: &str, id: &str) -> Result<(), Qt6Error> {
        let c_text = CString::new(text)?;
        let c_id = CString::new(id)?;
        unsafe {
            // SafeQtAppHandleから生ポインタを取り出す
            bind::add_tray_menu_item(self.handle.as_ptr(), c_text.as_ptr(), c_id.as_ptr());
        }
        Ok(())
    }

    /// Starts the Qt application event loop in a new thread.
    /// Returns a `QtAppInstance` which can be used to interact with the running app.
    pub fn start(&self) -> Result<QtAppInstance, Qt6Error> {
        // handleはSafeQtAppHandle型になったので、そのまま移動できる
        let handle = self.handle;
        // Prevent `drop` from being called on `self` which would clean up the handle
        // before the new thread takes ownership.

        let join_handle = std::thread::spawn(move || {
            let mut argv: Vec<*mut c_char> = Vec::new();
            // SafeQtAppHandleから生ポインタを取り出す
            let result = unsafe { bind::run_qt_app(handle.as_ptr(), 0, argv.as_mut_ptr()) };
            if result != 0 {
                eprintln!("Qt application exited with code: {}", result);
            }
            // The handle is cleaned up when the original QtApp struct is dropped.
            // We don't want to double-free here.
        });

        Ok(QtAppInstance {
            handle, // SafeQtAppHandleはSendなので、そのまま渡せる
            _join_handle: join_handle,
        })
    }
}

impl<'a> Drop for QtApp<'a> {
    fn drop(&mut self) {
        // SafeQtAppHandleから生ポインタを取り出す
        if !self.handle.as_ptr().is_null() {
            unsafe {
                bind::cleanup_qt_app(self.handle.as_ptr());
            }
            // ドロップされた後、ハンドルを無効な状態にする
            self.handle = unsafe { SafeQtAppHandle::new(std::ptr::null_mut()) };
        }
    }
}
