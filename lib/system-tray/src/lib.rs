mod bind;
mod error;

pub use error::SystemTrayError as Error;
use std::{
    ffi::{c_char, CString},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

#[repr(transparent)]
#[derive(Clone, Copy)]
struct SafeQtAppHandle(*mut bind::QtAppHandle);

unsafe impl Send for SafeQtAppHandle {}

impl SafeQtAppHandle {
    pub unsafe fn new(ptr: *mut bind::QtAppHandle) -> Self {
        Self(ptr)
    }

    pub fn as_ptr(&self) -> *mut bind::QtAppHandle {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    None,
    TrayClicked,
    TrayDoubleClicked,
    MenuItemClicked(String),
}

#[derive(Clone)]
pub struct SystemTray {
    handle: Arc<Mutex<SafeQtAppHandle>>,
    instance: Arc<Mutex<Option<JoinHandle<()>>>>,
}

pub struct Menu {
    text: String,
    id: String,
}

impl Menu {
    pub fn new(text: String, id: String) -> Self {
        Self { text, id }
    }
}

impl SystemTray {
    pub fn new(_name: &str, id: &str) -> Self {
        let c_id = CString::new(id).map_err(Error::Ffi).unwrap();
        let handle = unsafe { bind::create_qt_app() };
        let safe_handle = unsafe { SafeQtAppHandle::new(handle) };
        unsafe {
            bind::set_app_id(safe_handle.as_ptr(), c_id.as_ptr());
            bind::init_tray(safe_handle.as_ptr());
        }
        Self {
            handle: Arc::new(Mutex::new(safe_handle)),
            instance: Arc::new(Mutex::new(None)),
        }
    }

    pub fn menu(self, menu: Menu) -> Self {
        let c_text = CString::new(menu.text).map_err(Error::Ffi).unwrap();
        let c_id = CString::new(menu.id).map_err(Error::Ffi).unwrap();
        unsafe {
            bind::add_tray_menu_item(
                self.handle.lock().unwrap().as_ptr(),
                c_text.as_ptr(),
                c_id.as_ptr(),
            );
        }
        self
    }

    pub fn icon(self, icon_data: &'static [u8], icon_format: &str) -> Self {
        let c_format = CString::new(icon_format).map_err(Error::Ffi).unwrap();
        unsafe {
            bind::set_app_icon_from_data(
                self.handle.lock().unwrap().as_ptr(),
                icon_data.as_ptr(),
                icon_data.len(),
                c_format.as_ptr(),
            );
        }
        self
    }

    pub fn run(&self) {
        let handle = {
            let handle_guard = self.handle.lock().unwrap();
            *handle_guard
        };
        let join_handle = std::thread::spawn(move || {
            let mut argv: Vec<*mut c_char> = Vec::new();
            let result = unsafe { bind::run_qt_app(handle.as_ptr(), 0, argv.as_mut_ptr()) };
            if result != 0 {
                eprintln!("Qt application exited with code: {}", result);
            }
        });
        *self.instance.lock().unwrap() = Some(join_handle);
    }

    pub fn stop(&self) {
        {
            let handle = self.handle.lock().unwrap();
            unsafe {
                bind::quit_qt_app(handle.as_ptr());
            }
        }
        if let Some(join_handle) = self.instance.lock().unwrap().take() {
            join_handle.join().unwrap_or_else(|e| {
                eprintln!("Failed to join Qt thread: {:?}", e);
            });
        }
    }

    pub fn poll_event(&self) -> Result<Event, Error> {
        let handle = self.handle.lock().unwrap();
        let event = unsafe { bind::poll_event(handle.as_ptr()) };
        match event.type_ {
            bind::AppEventType_None => Ok(Event::None),
            bind::AppEventType_TrayClicked => Ok(Event::TrayClicked),
            bind::AppEventType_TrayDoubleClicked => Ok(Event::TrayDoubleClicked),
            bind::AppEventType_MenuItemClicked => {
                let c_str = unsafe { CString::from_raw(event.menu_id_str as *mut c_char) };
                let rust_str = c_str.to_string_lossy().into_owned();
                Ok(Event::MenuItemClicked(rust_str))
            }
            _ => Err(Error::PollEventError("Unknown event type".to_string())),
        }
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new("default", "default")
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        self.stop();
        let handle = self.handle.lock().unwrap();
        if !handle.as_ptr().is_null() {
            unsafe {
                bind::cleanup_qt_app(handle.as_ptr());
            }
        }
    }
}
