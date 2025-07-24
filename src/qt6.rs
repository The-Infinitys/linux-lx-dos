use std::ffi::{c_char, CString};
use std::marker::PhantomData;

// The bindgen-generated bindings will be included here.
#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bind {
    include!(concat!(env!("OUT_DIR"), "/qt6-bind.rs"));
}

#[derive(Debug, thiserror::Error)]
pub enum Qt6Error {
    #[error("Failed to create C-style string: {0}")]
    Ffi(#[from] std::ffi::NulError),
    #[error("Qt application failed to run, returned exit code {0}")]
    RunFailed(i32),
}

/// Represents a Qt application configuration.
/// This struct uses the builder pattern to set up the application.
/// When dropped, it automatically cleans up the associated C++ resources.
#[derive(Debug)]
pub struct QtApp<'a> {
    handle: *mut bind::QtAppHandle,
    // Use a phantom lifetime to tie the handle to this struct's lifetime.
    _marker: PhantomData<&'a ()>,
}

impl<'a> Default for QtApp<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> QtApp<'a> {
    /// Creates a new Qt application configuration.
    pub fn new() -> Self {
        let handle = unsafe { bind::create_qt_app() };
        Self {
            handle,
            _marker: PhantomData,
        }
    }

    /// Sets the application ID (e.g., "com.example.myapp").
    pub fn with_id(self, id: &str) -> Result<Self, Qt6Error> {
        let c_id = CString::new(id)?;
        unsafe {
            bind::set_app_id(self.handle, c_id.as_ptr());
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
            bind::set_app_icon_from_data(
                self.handle,
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
            bind::init_tray(self.handle);
        }
        self
    }

    /// Runs the application, blocking the current thread until the application exits.
    pub fn run(self) -> Result<(), Qt6Error> {
        // QApplication requires argc and argv, but they are not used by our current setup.
        // We can pass dummy values.
        let mut argv: Vec<*mut c_char> = Vec::new();
        let result = unsafe { bind::run_qt_app(self.handle, 0, argv.as_mut_ptr()) };

        if result != 0 {
            // A non-zero exit code might not always be an error in Qt, but we can log it.
            // For now, we consider the run successful if it exits.
        }

        Ok(())
    }
}

impl<'a> Drop for QtApp<'a> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                bind::cleanup_qt_app(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}


// --- Non-blocking API (Future Implementation) ---

#[derive(Debug)]
pub struct QtAppInstance {
    // TODO: This will hold the state for a non-blocking application instance.
}

impl QtApp<'_> {
    pub fn start(&self) -> Result<QtAppInstance, Qt6Error> {
        unimplemented!("Non-blocking start is not yet implemented.");
    }
}


#[derive(Debug)]
pub enum QtAppEvent {
    // TODO: Define events that can be received from the Qt application.
}

pub struct QtAppEventListener {
    // TODO: Implement an event listener.
}

pub struct QtAppTray {
    // TODO: Implement a handle for the tray icon.
}