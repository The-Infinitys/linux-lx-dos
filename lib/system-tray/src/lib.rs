mod bind;
mod error;
pub use error::SystemTrayError as Error;
pub struct SystemTray {
    // TODO: 必要な要素を判断し、入れておく
}
pub enum Event {
    Click,
    Menu(String), // idを表す文字列
}
pub struct EventSender {}
impl EventSender {
    pub fn send(&self, event: Event) -> Result<(), Error> {
        Ok(()) // SystemTrayが破棄されていた場合などにエラーが出るようにしてほしいです。
    }
}
impl From<&SystemTray> for EventSender {
    fn from(value: &SystemTray) -> Self {
        Self {}
    }
}
pub struct Menu {
    // TODO: 必要な要素を判断し、入れておく。
}
impl Menu {
    pub fn new(context: String, id: String) -> Self {
        Self {}
    }
}
impl SystemTray {
    pub fn new() -> Self {
        todo!()
    }
    pub fn menu(self, menu: Menu) -> Self {
        self
    }
    pub fn icon(self, icon_data: &[u8], icon_format: &str) -> Self {
        self
    }
    pub fn handle_event<F: Fn(Event)>(handle_function: F) {}
    pub fn run(&mut self) {} //スレッドを停止し、システムトレイを開始する
    pub fn start(&mut self) {} // スレッドを停止せずに、システムトレイを開始する。
    pub fn stop(&mut self) {} // システムトレイを停止する。
    pub fn send_event(event: Event) {}
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
        self.stop();
    }
}
