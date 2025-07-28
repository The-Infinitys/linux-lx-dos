pub mod window;

pub enum Event {
    App(AppEvent),
    Window(WindowEvent),
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

pub enum AppEvent {
    Exit,
    Suspend,
    Resume,
}

pub enum WindowEvent {
    Open,
    Close,
    Resize { width: u32, height: u32 },
    Minimize,
    Maximize,
    Restore,
}

pub enum MouseEvent {
    Click { x: i32, y: i32, button: MouseButton },
    Move { x: i32, y: i32 },
    Scroll { delta_x: i32, delta_y: i32 },
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub enum KeyboardEvent {
    KeyDown(Key),
    KeyUp(Key),
}

pub enum Key {
    // TODO: Add more keys as needed
    Escape,
    Delete,
    BackSpace,
    Enter,
    Space,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftShift,
    RightShift,
    Command, // 広義的な名称について、要検討。Windowsキーと呼ばれている場合もあるらしい
    CapsLock,
    Char(char), // 普通の文字記号 例:(a, c, d, !, %, ...)
    Function(u8) // ファンクションキー
}
