pub enum WindowEvent {
    Open,
    Close,
    Resize { width: u32, height: u32 },
    Minimize,
    Maximize,
    Restore,
    Focus,
    Blur,
}
