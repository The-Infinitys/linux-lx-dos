use std::fmt;
pub mod builder;
pub mod element;
use crate::app::event::WindowEvent;
use builder::WindowBuilder;

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    event_handler: Box<dyn Fn(WindowEvent) + Send + 'static>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "New Window".to_string(),
            width: 800,
            height: 600,
            event_handler: Box::new(|_event| {}),
        }
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("title", &self.title)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("event_handler", &"<function>")
            .finish()
    }
}

impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// イベントハンドラを設定する
    pub fn handle_event<F>(&mut self, handler: F)
    where
        F: Fn(WindowEvent) + Send + 'static,
    {
        self.event_handler = Box::new(handler);
    }

    // 内部イベントディスパッチャ
    fn _dispatch_event(&self, event: WindowEvent) {
        let handler = &self.event_handler;
        handler(event);
    }
}
