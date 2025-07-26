use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use crate::{Qt6Error, SafeQtAppHandle, QtAppEvent, bind};

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
