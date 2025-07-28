use crate::app::event::ElementEvent;
use std::fmt;

pub struct Element {
    pub kind: ElementKind,
    _properties: Vec<ElementProperty>,
    event_handler: Box<dyn Fn(ElementEvent) + Send + 'static>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            kind: ElementKind::Widget, // Default to a generic widget
            _properties: Vec::new(),
            event_handler: Box::new(|_event| {}),
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Element")
            .field("kind", &self.kind)
            .field("_properties", &self._properties)
            .field("event_handler", &"<function>")
            .finish()
    }
}
#[derive(Debug)]
pub enum ElementProperty {
    Text(String),
    Color(u8, u8, u8, u8),
    // Add more properties as needed
}
#[derive(Debug)]
pub enum ElementKind {
    Button,
    Label,
    TextInput,
    Widget,
    // Add more element types as needed
}

impl Element {
    pub fn property(&mut self, _property: ElementProperty) {
        todo!()
    }
    pub fn add_element() {
        todo!()
    }

    /// イベントハンドラを設定する
    pub fn handle_event<F>(&mut self, handler: F)
    where
        F: Fn(ElementEvent) + Send + 'static,
    {
        self.event_handler = Box::new(handler);
    }

    // 内部イベントディスパッチャ
    fn _dispatch_event(&self, event: ElementEvent) {
        let handler = &self.event_handler;
        handler(event);
    }
}
