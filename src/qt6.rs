/// src/qt6.rs
use std::ffi::{c_char, CString};
use std::marker::PhantomData;

// The bindgen-generated bindings will be included here.
// Make sure your build.rs correctly points to qt-app.hpp for binding generation.
#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bind {
    // bindgenがqt-app.hppを処理して生成したバインディングをインクルード
    // OUT_DIRはbuild.rsによって設定されます。
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
    #[error("Qt Instance has already started and wasn't dropped.")]
    QtInstanceError,
}

/// Events that can be received from the Qt application.
#[derive(Clone, Debug)]
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
    /// # Safety
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

/// Represents a running Qt application instance.
pub struct QtAppInstance {
    handle: SafeQtAppHandle,
    _join_handle: Option<std::thread::JoinHandle<()>>, // To keep the thread alive
}

impl QtAppInstance {
    /// Polls for the next event from the Qt application.
    pub fn poll_event(&self) -> Result<QtAppEvent, Qt6Error> {
        // SafeQtAppHandleから生ポインタを取り出す
        let event = unsafe { bind::poll_event(self.handle.as_ptr()) };
        match event.type_ {
            bind::AppEventType_AppEventType_None => Ok(QtAppEvent::None),
            bind::AppEventType_AppEventType_TrayClicked => Ok(QtAppEvent::TrayClicked),
            bind::AppEventType_AppEventType_TrayDoubleClicked => Ok(QtAppEvent::TrayDoubleClicked),
            bind::AppEventType_AppEventType_MenuItemClicked => {
                // C++側でstrdupされた文字列の所有権をRustが受け取り、CString::from_rawで管理する
                // CStringのDrop実装が自動的にfree_char_ptrを呼び出すため、明示的な呼び出しは不要
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

    /// Returns the underlying `SafeQtAppHandle` for the Qt application.
    pub fn get_handle(&self) -> SafeQtAppHandle {
        self.handle
    }

    pub fn join(mut self) -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
        if let Some(handle) = self._join_handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }
}

impl Drop for QtAppInstance {
    fn drop(&mut self) {
        // Qtアプリケーションのリソースをクリーンアップします。
        // このハンドルは run_qt_app に渡されました。
        if !self.handle.as_ptr().is_null() {
            unsafe {
                bind::cleanup_qt_app(self.handle.as_ptr());
            }
            // 二重解放を防ぐため、ハンドルを無効な状態にします
            self.handle = unsafe { SafeQtAppHandle::new(std::ptr::null_mut()) };
        }
        // If the thread handle still exists, it means join() was not called.
        // We should not join here as it would block the drop, but ensure it's cleaned up.
        // The thread will eventually terminate when the Qt app quits.
        self._join_handle.take(); // Consume the handle to prevent double-drop
    }
}

/// Represents a Qt application configuration.
/// This struct uses the builder pattern to set up the application.
/// When dropped, it automatically cleans up the associated C++ resources.
pub struct QtApp<'a> {
    handle: SafeQtAppHandle,
    // Use a phantom lifetime to tie the handle to this struct's lifetime.
    _marker: PhantomData<&'a ()>,
    has_started: bool,
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
unsafe impl Send for QtAppInstance {} // _join_handleがSendなので問題なし

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
            has_started: false,
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
                data.len(), // size_t に対応
                c_format.as_ptr(),
            );
        }
        Ok(self)
    }

    /// Initializes a system tray icon for the application.
    pub fn with_tray(self) -> Self {
        unsafe {
            // C++側の関数名に合わせて init_tray_icon を呼び出す
            bind::init_tray_icon(self.handle.as_ptr());
        }
        self
    }

    /// Adds a menu item to the system tray icon's context menu.
    pub fn add_tray_menu_item(&self, text: &str, id: &str) -> Result<(), Qt6Error> {
        let c_text = CString::new(text)?;
        let c_id = CString::new(id)?;
        unsafe {
            // C++側の関数名に合わせて add_tray_menu_item を呼び出す
            bind::add_tray_menu_item(self.handle.as_ptr(), c_text.as_ptr(), c_id.as_ptr());
        }
        Ok(())
    }

    /// Starts the Qt application event loop in a new thread.
    /// Returns a `QtAppInstance` which can be used to interact with the running app.
    pub fn start(&mut self) -> Result<QtAppInstance, Qt6Error> {
        if self.has_started {
            return Err(Qt6Error::QtInstanceError);
        }
        self.has_started = true;
        // handleはSafeQtAppHandle型になったので、そのまま移動できる
        let handle = self.handle;
        // Prevent `drop` from being called on `self` which would clean up the handle
        // before the new thread takes ownership.
        // `self.handle` を `std::ptr::null_mut()` に設定することで、
        // `QtApp` の `drop` がこのハンドルを二重に解放するのを防ぐ。
        self.handle = unsafe { SafeQtAppHandle::new(std::ptr::null_mut()) };

        let join_handle = std::thread::spawn(move || {
            let mut argv: Vec<*mut c_char> = Vec::new();
            // C++のrun_qt_appはargcとargvを必要とするが、ここではGUIアプリケーションなので通常は0とnullで十分
            // 必要に応じて、実際のコマンドライン引数を渡すことも可能
            let result = unsafe { bind::run_qt_app(handle.as_ptr(), 0, argv.as_mut_ptr()) };
            if result != 0 {
                eprintln!("Qt application exited with code: {}", result);
            }
            // The handle is cleaned up when the original QtApp struct is dropped.
            // We don't want to double-free here.
        });

        Ok(QtAppInstance {
            handle, // SafeQtAppHandleはSendなので、そのまま渡せる
            _join_handle: Some(join_handle),
        })
    }
}

impl<'a> Drop for QtApp<'a> {
    fn drop(&mut self) {
        // handleがnullでないことを確認 (start()でnull_mutに設定されている可能性があるため)
        if !self.handle.as_ptr().is_null() {
            unsafe {
                // Qtアプリケーションのリソースをクリーンアップ
                bind::cleanup_qt_app(self.handle.as_ptr());
            }
            // ドロップされた後、ハンドルを無効な状態にする（二重解放防止）
            self.handle = unsafe { SafeQtAppHandle::new(std::ptr::null_mut()) };
        }
    }
}

// --- QtWindow Wrapper ---

pub struct QtWindow<'a> {
    handle: *mut bind::QtWindowHandle,
    _marker: PhantomData<&'a ()>,
}

impl<'a> QtWindow<'a> {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, Qt6Error> {
        let c_title = CString::new(title)?;
        let handle = unsafe { bind::create_qt_window(c_title.as_ptr(), width, height) };
        Ok(Self {
            handle,
            _marker: PhantomData,
        })
    }

    pub fn show(&self) {
        unsafe { bind::show_qt_window(self.handle) };
    }

    pub fn add_widget(&self, element: &QtElement) {
        unsafe { bind::add_widget_to_window(self.handle, element.handle) };
    }
}

impl<'a> Drop for QtWindow<'a> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                bind::cleanup_qt_window(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}

// --- QtElement Wrapper ---

pub struct QtElement<'a> {
    handle: *mut bind::QtElementHandle,
    _marker: PhantomData<&'a ()>,
}

impl<'a> QtElement<'a> {
    pub fn new(element_type: bind::QtElementType, id: &str) -> Result<Self, Qt6Error> {
        let c_id = CString::new(id)?;
        let handle = unsafe { bind::create_qt_element(element_type, c_id.as_ptr()) };
        Ok(Self {
            handle,
            _marker: PhantomData,
        })
    }

    pub fn set_text(&self, text: &str) -> Result<(), Qt6Error> {
        let c_text = CString::new(text)?;
        unsafe { bind::set_element_text(self.handle, c_text.as_ptr()) };
        Ok(())
    }

    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { bind::set_element_size(self.handle, width, height) };
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { bind::set_element_enabled(self.handle, enabled) };
    }

    pub fn poll_event(&self) -> Result<QtElementEvent, Qt6Error> {
        let event = unsafe { bind::poll_element_event(self.handle) };
        match event.type_ {
            bind::QtElementEventType_QtElementEventType_None => Ok(QtElementEvent::None),
            bind::QtElementEventType_QtElementEventType_Clicked => {
                let id = unsafe { CString::from_raw(event.element_id_str as *mut c_char) }
                    .to_string_lossy()
                    .into_owned();
                Ok(QtElementEvent::Clicked(id))
            }
            bind::QtElementEventType_QtElementEventType_TextChanged => {
                let id = unsafe { CString::from_raw(event.element_id_str as *mut c_char) }
                    .to_string_lossy()
                    .into_owned();
                let text = unsafe { CString::from_raw(event.data_str as *mut c_char) }
                    .to_string_lossy()
                    .into_owned();
                Ok(QtElementEvent::TextChanged(id, text))
            }
            bind::QtElementEventType_QtElementEventType_EditingFinished => {
                let text = unsafe { CString::from_raw(event.data_str as *mut c_char) }
                    .to_string_lossy()
                    .into_owned();
                Ok(QtElementEvent::EditingFinished(text))
            }
            _ => Err(Qt6Error::PollEventError(
                "Unknown element event type".to_string(),
            )),
        }
    }
}

impl<'a> Drop for QtElement<'a> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                bind::cleanup_qt_element(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}

pub enum QtElementEvent {
    None,
    Clicked(String),
    TextChanged(String, String),
    EditingFinished(String),
}
