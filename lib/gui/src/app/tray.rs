use crate::app::event::TrayEvent;
use std::fmt;
use std::path::Path;

pub struct Tray {
    icon_path: Option<String>,
    icon_data: Option<Vec<u8>>,
    menu_items: Vec<(String, String)>,
    event_handler: Box<dyn Fn(TrayEvent) + Send + 'static>,
}

impl Default for Tray {
    fn default() -> Self {
        Self {
            icon_path: None,
            icon_data: None,
            menu_items: Vec::new(),
            event_handler: Box::new(|_event| {}),
        }
    }
}

impl fmt::Debug for Tray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tray")
            .field("icon_path", &self.icon_path)
            .field("icon_data", &self.icon_data)
            .field("menu_items", &self.menu_items)
            .field("event_handler", &"<function>")
            .finish()
    }
}

impl Tray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_icon_path(mut self, path: &Path) -> Self {
        self.icon_path = Some(path.to_string_lossy().into_owned());
        self
    }

    pub fn with_icon_data(mut self, data: &[u8]) -> Self {
        self.icon_data = Some(data.to_vec());
        self
    }

    pub fn add_menu_item(mut self, text: &str, id: &str) -> Self {
        self.menu_items.push((text.to_string(), id.to_string()));
        self
    }

    /// イベントハンドラを設定する
    pub fn handle_event<F>(&mut self, handler: F)
    where
        F: Fn(TrayEvent) + Send + 'static,
    {
        self.event_handler = Box::new(handler);
    }

    // 内部イベントディスパッチャ
    fn _dispatch_event(&self, event: TrayEvent) {
        let handler = &self.event_handler;
        handler(event);
    }
}
