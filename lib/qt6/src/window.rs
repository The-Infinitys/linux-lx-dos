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
        // Default window with a generic title and size.
        // This window is not yet associated with a running QApplication.
        let (tx, rx) = std::sync::mpsc::channel();
        let title = "Default Window".to_string();
        let width = 800;
        let height = 600;

        let c_title = CString::new(title.clone()).unwrap();
        let user_data = Box::into_raw(Box::new(tx)) as *mut std::os::raw::c_void;

        unsafe {
            let handle = bind::create_qt_window(c_title.as_ptr(), width, height);
            tx.send(handle).expect("Failed to send window handle from Qt thread.");
        }

        let handle = rx.recv().expect("Failed to receive window handle from Qt thread.");

        Self {
            handle: unsafe { SafeQtWindowHandle::new(handle) },
            _marker: PhantomData,
            event_handler: None,
        }
    }
}

// Callback function for create_qt_window_async
extern "C" fn on_window_created(handle: *mut bind::QtWindowHandle, user_data: *mut std::os::raw::c_void) {
    let tx = unsafe { Box::from_raw(user_data as *mut std::sync::mpsc::Sender<*mut bind::QtWindowHandle>) };
    tx.send(handle).expect("Failed to send window handle to Rust channel.");
}

impl<'a> QtWindow<'a> {
    // Private constructor, use QtWindowBuilder instead
    fn new(
        title: &str,
        width: i32,
        height: i32,
    ) -> Result<Self, Qt6Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let c_title = CString::new(title)?;

        unsafe {
            let handle = bind::create_qt_window(c_title.as_ptr(), width, height);
            tx.send(handle).expect("Failed to send window handle from Qt thread.");
        }

        let handle = rx.recv().expect("Failed to receive window handle from Qt thread.");

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

    pub fn default_with_app_handle(_app_handle: SafeQtAppHandle) -> Result<Self, Qt6Error> {
        Self::default().build()
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

    pub fn builder() -> QtWindowBuilder<'a> {
        QtWindowBuilder::new()
    }

    pub fn start(mut self, app_instance: &crate::QtAppInstance) -> Result<QtWindowInstance, Qt6Error> {
        // Transfer ownership of the handle to QtWindowInstance
        let handle = self.handle;
        self.handle = unsafe { SafeQtWindowHandle::new(std::ptr::null_mut()) }; // Prevent double-free on drop

        let window_instance = QtWindowInstance::new_internal(handle)?;
        app_instance.register_window(window_instance);
        Ok(window_instance)
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
    title: String,
    width: i32,
    height: i32,
    elements: Vec<crate::QtElement<'a>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Default for QtWindowBuilder<'a> {
    fn default() -> Self {
        Self {
            title: "New Window".to_string(),
            width: 800,
            height: 600,
            elements: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<'a> QtWindowBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
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
        let window = QtWindow::new(&self.title, self.width, self.height)?;
        for element in self.elements {
            window.add_widget(&element);
        }
        Ok(window)
    }

    pub fn append(mut self, element: crate::QtElement<'a>) -> Self {
        self.elements.push(element);
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

    pub fn get_handle(&self) -> SafeQtWindowHandle {
        self.handle
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
