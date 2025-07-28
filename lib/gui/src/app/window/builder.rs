use crate::app::window::element::Element;
use crate::app::window::Window;
pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: "New Window".to_string(),
            width: 800,
            height: 600,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn add_element(&mut self, _element: Element) -> Self {
        todo!()
    }
    pub fn build(self) -> Window {
        todo!()
    }
}
