mod bind;
mod error;

use std::{ffi::CString, sync::{mpsc, Arc, Mutex}, thread};
pub use error::SystemTrayError as Error;

struct ThreadSafeSystemTray(*mut bind::SystemTray);
unsafe impl Send for ThreadSafeSystemTray {}
unsafe impl Sync for ThreadSafeSystemTray {}

pub struct SystemTray {
    tray: Arc<Mutex<ThreadSafeSystemTray>>,
    event_sender: mpsc::Sender<Event>,
    event_receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
}

pub enum Event {
    Click,
    Menu(String),
}

pub struct EventSender {
    sender: mpsc::Sender<Event>,
}

impl EventSender {
    pub fn send(&self, event: Event) -> Result<(), Error> {
        self.sender.send(event).map_err(|_| Error::SendError)
    }
}

impl From<&SystemTray> for EventSender {
    fn from(value: &SystemTray) -> Self {
        Self {
            sender: value.event_sender.clone(),
        }
    }
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
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel();
        let tray = unsafe { bind::system_tray_new() };
        Self {
            tray: Arc::new(Mutex::new(ThreadSafeSystemTray(tray))),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
        }
    }

    pub fn menu(self, menu: Menu) -> Self {
        let text = CString::new(menu.text).unwrap();
        let id = CString::new(menu.id).unwrap();
        let sender = self.event_sender.clone();

        unsafe {
            bind::system_tray_add_menu_item(
                self.tray.lock().unwrap().0,
                text.as_ptr(),
                Some(Self::menu_callback),
                Box::into_raw(Box::new((sender, id))) as *mut _,
            );
        }
        self
    }

    extern "C" fn menu_callback(user_data: *mut std::ffi::c_void) {
        let (sender, id) = unsafe { *Box::from_raw(user_data as *mut (mpsc::Sender<Event>, CString)) };
        sender
            .send(Event::Menu(id.into_string().unwrap()))
            .unwrap();
    }

    pub fn icon(self, icon_path: &str) -> Self {
        let icon_path = CString::new(icon_path).unwrap();
        unsafe {
            bind::system_tray_set_icon(self.tray.lock().unwrap().0, icon_path.as_ptr());
        }
        self
    }

    pub fn handle_event<F: Fn(Event) + Send + 'static>(self, handle_function: F) {
        let receiver = self.event_receiver.clone();
        thread::spawn(move || {
            let receiver = receiver.lock().unwrap();
            for event in receiver.iter() {
                handle_function(event);
            }
        });
    }

    pub fn run(&mut self) {
        unsafe {
            bind::system_tray_run(self.tray.lock().unwrap().0);
        }
    }

    pub fn start(&mut self) {
        let tray = self.tray.clone();
        thread::spawn(move || unsafe {
            bind::system_tray_run(tray.lock().unwrap().0);
        });
    }

    pub fn stop(&mut self) {
        unsafe {
            bind::system_tray_exit(self.tray.lock().unwrap().0);
        }
    }

    pub fn event_sender(&self) -> EventSender {
        EventSender::from(self)
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        unsafe {
            bind::system_tray_delete(self.tray.lock().unwrap().0);
        }
    }
}