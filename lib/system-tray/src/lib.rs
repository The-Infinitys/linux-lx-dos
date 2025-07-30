mod bind;
mod error;

pub use error::SystemTrayError as Error;
use std::{
    ffi::CString,
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct TrayWrapper(*mut bind::SystemTray);
unsafe impl Send for TrayWrapper {}
unsafe impl Sync for TrayWrapper {}

#[derive(Clone)]
pub struct SystemTray {
    tray: Arc<Mutex<TrayWrapper>>,
    event_sender: mpsc::Sender<Event>,
    event_receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Click,
    Menu(String),
}
#[derive(Debug, Default, Eq, PartialEq)]
pub enum EventHandleReturn {
    #[default]
    Continue,
    Break,
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
    pub fn new(name: &str, id: &str) -> Self {
        let (event_sender, event_receiver) = mpsc::channel();
        let c_name = CString::new(name).unwrap();
        let c_id = CString::new(id).unwrap();
        let tray = unsafe { bind::system_tray_new(c_name.as_ptr(), c_id.as_ptr()) };
        Self {
            tray: Arc::new(Mutex::new(TrayWrapper(tray))),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
        }
    }

    pub fn menu(&mut self, menu: Menu) -> &mut Self {
        let text = CString::new(menu.text).unwrap();
        let id = CString::new(menu.id).unwrap();
        let sender = self.event_sender.clone();

        unsafe {
            let tray = self.tray.lock().unwrap();
            bind::system_tray_add_menu_item(
                tray.0,
                text.as_ptr(),
                Some(Self::menu_callback),
                Box::into_raw(Box::new((sender, id))) as *mut _,
            );
        }
        self
    }

    extern "C" fn menu_callback(user_data: *mut std::ffi::c_void) {
        let (sender, id) =
            unsafe { *Box::from_raw(user_data as *mut (mpsc::Sender<Event>, CString)) };
        sender.send(Event::Menu(id.into_string().unwrap())).unwrap();
    }

    pub fn icon(&mut self, icon_data: &[u8], icon_format: &str) -> &mut Self {
        let format = CString::new(icon_format).unwrap();
        unsafe {
            let tray = self.tray.lock().unwrap();
            bind::system_tray_set_icon(
                tray.0,
                icon_data.as_ptr(),
                icon_data.len(),
                format.as_ptr(),
            );
        }
        self
    }

    pub fn handle_event<F: Fn(Event) -> EventHandleReturn + Send + 'static>(
        &self,
        handle_function: F,
    ) {
        let receiver = self.event_receiver.clone();
        let tray_clone = self.clone(); // SystemTrayのクローンを作成
        thread::spawn(move || {
            let receiver = receiver.lock().unwrap();
            for event in receiver.iter() {
                let result = handle_function(event);
                match result {
                    EventHandleReturn::Continue => continue,
                    EventHandleReturn::Break => {
                        tray_clone.stop();
                        break;
                    }
                }
            }
        });
    }

    pub fn run(&self) {
        let tray = self.tray.lock().unwrap();
        unsafe {
            bind::system_tray_run(tray.0);
        }
    }

    pub fn stop(&self) {
        // system_tray_exitはグローバルな関数なので、trayのロックは不要
        unsafe {
            bind::system_tray_exit();
        }
    }

    pub fn event_sender(&self) -> EventSender {
        EventSender::from(self)
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new("default", "default")
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        let tray = self.tray.lock().unwrap();
        unsafe {
            bind::system_tray_delete(tray.0);
        }
    }
}
