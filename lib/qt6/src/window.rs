use crate::bind;
use crate::{Qt6Error, QtAppEvent, SafeQtAppHandle};
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// A safe wrapper around `*mut bind::QtWindowHandle` that asserts `Send` safety.
/// This is necessary to satisfy Rust's Orphan Rules and explicitly state
/// the thread-safety guarantees.
#[repr(transparent)] // Ensures SafeQtWindowHandle has the same memory layout as *mut bind::QtWindowHandle
#[derive(Clone, Copy)]
pub struct SafeQtWindowHandle(*mut bind::QtWindowHandle);

// Implement Send for SafeQtWindowHandle, asserting that it's safe to transfer
// this handle between threads. This is based on the assumption that
// Qt's QApplication handle, when passed to run_qt_app, is managed exclusively
// by the thread running the event loop.
unsafe impl Send for SafeQtWindowHandle {}

impl SafeQtWindowHandle {
    /// # Safety
    /// Creates a new `SafeQtWindowHandle` from a raw pointer.
    /// This function is unsafe because the caller must ensure the pointer is valid.
    pub unsafe fn new(ptr: *mut bind::QtWindowHandle) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut bind::QtWindowHandle {
        self.0
    }
}

pub struct QtWindow<'a> {
    handle: SafeQtWindowHandle,
    _marker: PhantomData<&'a ()>,
    event_handler: Option<Box<dyn Fn(QtWindowEvent) + Send>>,
}
impl<'a> Default for QtWindow<'a> {
    fn default() -> Self {
        // This default assumes a default app_handle, which might not be ideal.
        // Consider if a default QtWindow makes sense without an app_handle.
        // For now, creating a dummy app_handle for compilation.
        let dummy_app_handle = unsafe { crate::SafeQtAppHandle::new(std::ptr::null_mut()) };
        QtWindowBuilder::new(dummy_app_handle).build().unwrap()
    }
}

impl<'a> QtWindow<'a> {
    pub fn new(
        _app_handle: SafeQtAppHandle,
        title: &str,
        width: i32,
        height: i32,
    ) -> Result<Self, Qt6Error> {
        let c_title = CString::new(title)?;
        let handle = unsafe { bind::create_qt_window(c_title.as_ptr(), width, height) };
        Ok(Self {
            handle: unsafe { SafeQtWindowHandle::new(handle) },
            _marker: PhantomData,
            event_handler: None,
        })
    }

    pub fn event_handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(QtWindowEvent) + Send + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    pub fn default_with_app_handle(app_handle: SafeQtAppHandle) -> Result<Self, Qt6Error> {
        QtWindowBuilder::new(app_handle).build()
    }

    pub fn show(&self) {
        unsafe { bind::show_qt_window(self.handle.as_ptr()) };
    }

    pub fn add_widget(&self, element: &crate::QtElement) {
        unsafe {
            bind::add_widget_to_window(
                self.handle.as_ptr(),
                element.handle as *mut std::os::raw::c_void,
            )
        };
    }

    pub fn poll_event(&self) -> Result<QtWindowEvent, Qt6Error> {
        let event = unsafe { bind::poll_window_event(self.handle.as_ptr()) };
        let rust_event = match event.type_ {
            bind::QtWindowEventType_QtWindowEvent_None => Ok(QtWindowEvent::None),
            bind::QtWindowEventType_QtWindowEvent_Closed => Ok(QtWindowEvent::Closed),
            _ => Err(Qt6Error::PollEventError(
                "Unknown window event type".to_string(),
            )),
        };

        if let Ok(e) = &rust_event {
            if let Some(handler) = &self.event_handler {
                handler(e.clone());
            }
        }
        rust_event
    }

    pub fn builder(app_handle: SafeQtAppHandle) -> QtWindowBuilder<'a> {
        QtWindowBuilder::new(app_handle)
    }

    pub fn start(mut self) -> Result<QtWindowInstance, Qt6Error> {
        // Prevent `drop` from being called on `self` which would clean up the handle
        // before the new thread takes ownership.
        // `self.handle` を `std::ptr::null_mut()` に設定することで、
        // `QtWindow` の `drop` がこのハンドルを二重に解放するのを防ぐ。
        let handle = self.handle;
        self.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) };

        let join_handle = thread::spawn(move || {
            let mut argv: Vec<*mut c_char> = Vec::new();
            let result = unsafe { bind::run_qt_app(handle.as_ptr(), 0, argv.as_mut_ptr()) };
            if result != 0 {
                eprintln!("Qt application exited with code: {}", result);
            }
        });

        Ok(QtWindowInstance {
            handle,
            _join_handle: Some(join_handle),
            event_queue: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn run(mut self) -> Result<(), Qt6Error> {
        // Prevent `drop` from being called on `self` which would clean up the handle
        let handle = self.handle;
        self.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) };

        let mut argv: Vec<*mut c_char> = Vec::new();
        let result = unsafe { bind::run_qt_app(handle.as_ptr(), 0, argv.as_mut_ptr()) };
        if result != 0 {
            eprintln!("Qt application exited with code: {}", result);
            Err(Qt6Error::RunFailed(result))
        } else {
            Ok(())
        }
    }
}

impl<'a> Drop for QtWindow<'a> {
    fn drop(&mut self) {
        if !self.handle.as_ptr().is_null() {
            unsafe {
                bind::cleanup_qt_window(self.handle.as_ptr());
            }
            self.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) };
        }
    }
}

// --- QtWindowBuilder ---

pub struct QtWindowBuilder<'a> {
    app_handle: SafeQtAppHandle,
    title: String,
    width: i32,
    height: i32,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Default for QtWindowBuilder<'a> {
    fn default() -> Self {
        // This default assumes a default app_handle, which might not be ideal.
        // Consider if a default QtWindowBuilder makes sense without an app_handle.
        // For now, creating a dummy app_handle for compilation.
        let dummy_app_handle = unsafe { crate::SafeQtAppHandle::new(std::ptr::null_mut()) };
        Self::new(dummy_app_handle)
    }
}

impl<'a> QtWindowBuilder<'a> {
    pub fn new(app_handle: SafeQtAppHandle) -> Self {
        Self {
            app_handle,
            title: "New Window".to_string(),
            width: 800,
            height: 600,
            _marker: PhantomData,
        }
    }

    pub fn default_with_app_handle(app_handle: SafeQtAppHandle) -> Self {
        Self::new(app_handle)
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self) -> Result<QtWindow<'a>, Qt6Error> {
        QtWindow::new(self.app_handle, &self.title, self.width, self.height)
    }

    pub fn append(self, _element: &crate::QtElement) -> Self {
        // TODO: Implement actual appending of elements to the window builder
        // This will likely require storing a list of elements and passing them to the C++ side during build.
        self
    }
}

/// Events that can be received from a Qt window.
#[derive(Clone, Debug)]
pub enum QtWindowEvent {
    /// No event occurred.
    None,
    /// The window was closed.
    Closed,
}

/// Represents a running Qt window instance.
pub struct QtWindowInstance {
    handle: SafeQtWindowHandle,
    _join_handle: Option<thread::JoinHandle<()>>,
    event_queue: Arc<Mutex<Vec<QtWindowEvent>>>,
    intervals: Arc<Mutex<HashMap<u64, thread::JoinHandle<()>>>>,
    interval_id_counter: Arc<AtomicU64>,
}

impl QtWindowInstance {
    pub(crate) fn new_internal(window: QtWindow) -> Result<Self, Qt6Error> {
        // Ensure the window handle is not dropped when the original QtWindow is dropped
        let handle = window.handle;
        // Set the original window's handle to null to prevent double-free
        // This is a bit hacky, ideally QtWindow::start() would consume self
        // and transfer ownership of the handle directly.
        // For now, we'll rely on the fact that QtWindow::start() already consumes self.
        // window.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) };

        Ok(QtWindowInstance {
            handle,
            _join_handle: None, // This will be set by QtWindow::start()
            event_queue: Arc::new(Mutex::new(Vec::new())),
            intervals: Arc::new(Mutex::new(HashMap::new())),
            interval_id_counter: Arc::new(AtomicU64::new(0)),
        })
    }

    pub fn send_event(&self, event: QtWindowEvent) {
        let mut queue = self.event_queue.lock().unwrap();
        queue.push(event);
    }

    pub fn poll_event(&self) -> Result<QtWindowEvent, Qt6Error> {
        let event = unsafe { bind::poll_window_event(self.handle.as_ptr()) };
        match event.type_ {
            bind::QtWindowEventType_QtWindowEvent_None => Ok(QtWindowEvent::None),
            bind::QtWindowEventType_QtWindowEvent_Closed => Ok(QtWindowEvent::Closed),
            _ => Err(Qt6Error::PollEventError(
                "Unknown window event type".to_string(),
            )),
        }
    }

    pub fn add_interval<F>(&self, interval_ms: u64, callback: F) -> u64
    where
        F: Fn() -> bool + Send + 'static,
    {
        let interval_id = self.interval_id_counter.fetch_add(1, Ordering::SeqCst);
        let intervals_clone = Arc::clone(&self.intervals);

        let join_handle = thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(interval_ms));
                if !callback() {
                    break;
                }
            }
            let mut intervals = intervals_clone.lock().unwrap();
            intervals.remove(&interval_id);
        });

        let mut intervals = self.intervals.lock().unwrap();
        intervals.insert(interval_id, join_handle);
        interval_id
    }

    pub fn del_interval(&self, interval_id: u64) {
        let mut intervals = self.intervals.lock().unwrap();
        if let Some(handle) = intervals.remove(&interval_id) {
            // Attempt to join the thread. If it's already finished, this will return immediately.
            let _ = handle.join();
        }
    }

    pub fn join(mut self) -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
        if let Some(handle) = self._join_handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }
}

impl Drop for QtWindowInstance {
    fn drop(&mut self) {
        if !self.handle.as_ptr().is_null() {
            unsafe {
                bind::cleanup_qt_window(self.handle.as_ptr());
            }
            self.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) };
        }
        self._join_handle.take();
    }
}
