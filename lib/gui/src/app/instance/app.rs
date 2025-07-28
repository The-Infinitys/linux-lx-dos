use crate::app::event::Event;

pub struct AppInstance;

impl AppInstance {
    pub fn send_event(&self, _event: Event) {
        // TODO: Implement event sending
    }

    pub fn exit(&self) {
        // TODO: Implement application exit
    }

    pub fn suspend(&self) {
        // TODO: Implement application suspend
    }
}
