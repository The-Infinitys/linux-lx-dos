pub mod builder;
pub mod element;
use builder::WindowBuilder;

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }
}
