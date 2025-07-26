use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use crate::{Qt6Error, SafeQtAppHandle};
use crate::bind;

// --- QtElement Wrapper ---

pub struct QtElement<'a> {
    pub handle: *mut bind::QtElementHandle,
    _marker: PhantomData<&'a ()>,
    event_handler: Option<Box<dyn Fn(QtElementEvent) + Send>>,
}

impl<'a> Default for QtElement<'a> {
    fn default() -> Self {
        // This default assumes a default app_handle, which might not be ideal.
        // Consider if a default QtElement makes sense without an app_handle.
        // For now, creating a dummy app_handle and element for compilation.
        let dummy_app_handle = unsafe { crate::SafeQtAppHandle::new(std::ptr::null_mut()) };
        QtElement::new(dummy_app_handle, bind::QtElementType_QtElementType_PushButton, "default_element").unwrap()
    }
}

impl<'a> From<bind::QtElementType> for QtElement<'a> {
    fn from(element_type: bind::QtElementType) -> Self {
        // This assumes a default app_handle, which might not be ideal.
        // For now, creating a dummy app_handle for compilation.
        let dummy_app_handle = unsafe { crate::SafeQtAppHandle::new(std::ptr::null_mut()) };
        QtElement::new(dummy_app_handle, element_type, "").unwrap()
    }
}

pub enum QtElementProperty {
    Text(String),
    Size(i32, i32),
    Enabled(bool),
    // Add other properties as needed
}

impl<'a> QtElement<'a> {
    pub fn new(_app_handle: SafeQtAppHandle, element_type: bind::QtElementType, id: &str) -> Result<Self, Qt6Error> {
        let c_id = CString::new(id)?;
        let handle = unsafe { bind::create_qt_element((element_type as i32).try_into().unwrap(), c_id.as_ptr()) };
        Ok(Self {
            handle,
            _marker: PhantomData,
            event_handler: None,
        })
    }

    pub fn property(self, prop: QtElementProperty) -> Result<Self, Qt6Error> {
        match prop {
            QtElementProperty::Text(text) => {
                self.set_text(&text)?;
            }
            QtElementProperty::Size(width, height) => {
                self.set_size(width, height);
            }
            QtElementProperty::Enabled(enabled) => {
                self.set_enabled(enabled);
            }
        }
        Ok(self)
    }

    pub fn event_handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(QtElementEvent) + Send + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
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

    pub fn append(self, _element: &QtElement) -> Self {
        // TODO: Implement actual appending of elements. This will likely require C++ backend changes.
        eprintln!("Warning: Appending elements is not yet fully supported in the C++ backend.");
        self
    }

    pub fn poll_event(&self) -> Result<QtElementEvent, Qt6Error> {
        let event = unsafe { bind::poll_element_event(self.handle) };
        let rust_event = match event.type_ {
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
        };

        if let Ok(e) = &rust_event {
            if let Some(handler) = &self.event_handler {
                handler(e.clone());
            }
        }
        rust_event
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
