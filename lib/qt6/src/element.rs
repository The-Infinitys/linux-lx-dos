use crate::bind;
use crate::Qt6Error;
use std::ffi::{c_char, CString};
use std::marker::PhantomData;

// --- QtElement Wrapper ---

pub struct QtElement<'a> {
    pub handle: *mut bind::QtElementHandle,
    _marker: PhantomData<&'a ()>,
    event_handler: Option<Box<dyn Fn(QtElementEvent) + Send>>,
}

impl<'a> Default for QtElement<'a> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let id = "default_element".to_string();
        let c_id = CString::new(id.clone()).unwrap();

        unsafe {
            let handle =
                bind::create_qt_element(bind::QtElementType_QtElementType_Button, c_id.as_ptr());
            tx.send(handle)
                .expect("Failed to send element handle to Rust channel.");
        }

        let handle = rx
            .recv()
            .expect("Failed to receive element handle from Qt thread.");

        Self {
            handle,
            _marker: PhantomData,
            event_handler: None,
        }
    }
}

impl<'a> From<bind::QtElementType> for QtElement<'a> {
    fn from(element_type: bind::QtElementType) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let id = "".to_string(); // Default empty ID
        let c_id = CString::new(id.clone()).unwrap();

        unsafe {
            let handle = bind::create_qt_element(element_type, c_id.as_ptr());
            tx.send(handle)
                .expect("Failed to send element handle from Qt thread.");
        }

        let handle = rx
            .recv()
            .expect("Failed to receive element handle from Qt thread.");

        Self {
            handle,
            _marker: PhantomData,
            event_handler: None,
        }
    }
}

// Callback function for create_qt_element_async
extern "C" fn on_element_created(
    handle: *mut bind::QtElementHandle,
    user_data: *mut std::os::raw::c_void,
) {
    let tx = unsafe {
        Box::from_raw(user_data as *mut std::sync::mpsc::Sender<*mut bind::QtElementHandle>)
    };
    tx.send(handle)
        .expect("Failed to send element handle to Rust channel.");
}

impl<'a> QtElement<'a> {
    pub fn new(element_type: bind::QtElementType, id: &str) -> Result<Self, Qt6Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let c_id = CString::new(id)?;

        unsafe {
            let handle = bind::create_qt_element(element_type, c_id.as_ptr());
            tx.send(handle)
                .expect("Failed to send element handle from Qt thread.");
        }

        let handle = rx
            .recv()
            .expect("Failed to receive element handle from Qt thread.");

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

    pub fn append(self, element: &QtElement) -> Self {
        unsafe {
            bind::add_child_element_to_element(self.handle, element.handle);
        }
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

#[derive(Clone)]
pub enum QtElementEvent {
    None,
    Clicked(String),
    TextChanged(String, String),
    EditingFinished(String),
}

pub enum QtElementProperty {
    Text(String),
    Size(i32, i32),
    Enabled(bool),
}
