/// src/lib.rs
use std::ffi::{c_char, CString};
use std::marker::PhantomData;

pub mod app;
pub mod window;
pub mod element;
mod bind;
pub use app::{QtApp, QtAppInstance};
pub use window::{QtWindow, QtWindowBuilder, QtWindowEvent};
pub use element::{QtElement, QtElementEvent};


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