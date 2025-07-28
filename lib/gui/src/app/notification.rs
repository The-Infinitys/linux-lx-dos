// src/app/notification.rs
use std::path::Path;
use std::fmt;

/// Represents a desktop notification.
pub struct Notification {
    title: String,
    body: String,
    icon_path: Option<String>,
    icon_data: Option<Vec<u8>>,
}

impl Default for Notification {
    /// Creates a new default notification.
    fn default() -> Self {
        Self {
            title: "Notification".to_string(),
            body: String::new(),
            icon_path: None,
            icon_data: None,
        }
    }
}

impl fmt::Debug for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Notification")
            .field("title", &self.title)
            .field("body", &self.body)
            .field("icon_path", &self.icon_path)
            .field("icon_data", &self.icon_data.is_some())
            .finish()
    }
}

impl Notification {
    /// Creates a new notification builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the title of the notification.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the body text of the notification.
    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// Sets the icon from a file path.
    pub fn with_icon_path(mut self, path: &Path) -> Self {
        self.icon_path = Some(path.to_string_lossy().into_owned());
        self
    }

    /// Sets the icon from raw image data.
    pub fn with_icon_data(mut self, data: &[u8]) -> Self {
        self.icon_data = Some(data.to_vec());
        self
    }

    /// Shows the notification to the user.
    ///
    /// This is a placeholder and will be implemented with a backend later.
    pub fn show(&self) {
        // In a real application, this would call into the backend
        // to display a native notification.
        println!("Showing Notification:");
        println!("  Title: {}", self.title);
        println!("  Body: {}", self.body);
        if let Some(path) = &self.icon_path {
            println!("  Icon Path: {}", path);
        }
        if self.icon_data.is_some() {
            println!("  Icon data is set.");
        }
    }
}
