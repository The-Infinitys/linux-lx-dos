use std::path::Path;

pub struct Tray {
    icon_path: Option<String>,
    icon_data: Option<Vec<u8>>,
    menu_items: Vec<(String, String)>,
}

impl Default for Tray {
    fn default() -> Self {
        Self {
            icon_path: None,
            icon_data: None,
            menu_items: Vec::new(),
        }
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
}
