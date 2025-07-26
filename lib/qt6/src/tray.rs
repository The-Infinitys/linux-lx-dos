// src/tray.rs

use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use crate::{Qt6Error, SafeQtAppHandle, QtAppEvent, bind};

/// Represents a Qt system tray icon.
pub struct QtTray<'a> {
    handle: *mut bind::QtTrayHandle,
    _marker: PhantomData<&'a ()>,
}

impl<'a> QtTray<'a> {
    /// Creates a new Qt system tray icon.
    /// This requires a reference to the QApplication handle.
    pub fn new(app_handle: SafeQtAppHandle) -> Result<Self, Qt6Error> {
        let handle = unsafe { bind::create_qt_tray(app_handle.as_ptr() as *mut std::os::raw::c_void) };
        Ok(Self {
            handle,
            _marker: PhantomData,
        })
    }

    /// Sets the tray icon from raw binary data.
    pub fn with_icon(self, data: &'static [u8], format: &str) -> Result<Self, Qt6Error> {
        let c_format = CString::new(format)?;
        unsafe {
            bind::set_tray_icon_from_data(
                self.handle,
                data.as_ptr(),
                data.len(),
                c_format.as_ptr(),
            );
        }
        Ok(self)
    }

    /// Initializes and shows the system tray icon with its menu.
    pub fn init(self) -> Self {
        unsafe {
            bind::init_tray(self.handle);
        }
        self
    }

    /// Adds a menu item to the system tray icon's context menu.
    pub fn add_menu_item(&self, text: &str, id: &str) -> Result<(), Qt6Error> {
        let c_text = CString::new(text)?;
        let c_id = CString::new(id)?;
        unsafe {
            bind::add_tray_menu_item_to_tray(self.handle, c_text.as_ptr(), c_id.as_ptr());
        }
        Ok(())
    }

    /// Polls for the next event from the Qt system tray.
    pub fn poll_event(&self) -> Result<QtAppEvent, Qt6Error> {
        let event = unsafe { bind::poll_tray_event(self.handle) };
        match event.type_ {
            bind::QtTrayEventType_QtTrayEventType_None => Ok(QtAppEvent::None),
            bind::QtTrayEventType_QtTrayEventType_TrayClicked => Ok(QtAppEvent::TrayClicked),
            bind::QtTrayEventType_QtTrayEventType_TrayDoubleClicked => Ok(QtAppEvent::TrayDoubleClicked),
            bind::QtTrayEventType_QtTrayEventType_MenuItemClicked => {
                let c_str = unsafe { CString::from_raw(event.menu_id_str as *mut c_char) };
                let rust_str = c_str.to_string_lossy().into_owned();
                Ok(QtAppEvent::MenuItemClicked(rust_str))
            }
            _ => Err(Qt6Error::PollEventError("Unknown tray event type".to_string())),
        }
    }
}

impl<'a> Drop for QtTray<'a> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                bind::cleanup_qt_tray(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}
