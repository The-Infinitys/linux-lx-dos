use std::fmt;
use std::path::Path;

pub mod event;
pub mod instance;
pub mod tray;
pub mod window;
pub mod notification;
use crate::app::event::Event;
use crate::app::notification::Notification;
use crate::app::tray::Tray;
use crate::app::window::builder::WindowBuilder;

pub struct App {
    id: String,
    icon_data: Option<Vec<u8>>,
    tray: Option<Tray>,
    event_handler: Box<dyn Fn(Event) + Send + 'static>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: "com.example.app".to_string(),
            icon_data: None,
            tray: None,
            event_handler: Box::new(|_event| {}),
        }
    }
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("id", &self.id)
            .field("icon_data", &self.icon_data.is_some())
            .field("tray", &self.tray)
            .field("event_handler", &"<function>")
            .finish()
    }
}

impl App {
    /// 新しいアプリケーションを生成する
    pub fn new() -> Self {
        Self::default()
    }

    /// アプリケーションにidを設定する。必須
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    /// アプリケーションにトレイを設定する
    pub fn with_tray(mut self, tray: Tray) -> Self {
        self.tray = Some(tray);
        self
    }

    /// ファイルを追加する
    pub fn add_file(&mut self, _path: &Path, _content: &[u8]) {
        // TODO: Implement add_file (Qt resource system or similar)
    }

    /// アイコンを設定する
    pub fn set_icon(&mut self, _path: &Path) {
        // TODO: Implement reading icon from path and setting icon_data
        // For now, just store the path (or load it if feasible)
    }

    /// アイコンを設定する
    pub fn with_icon(mut self, icon: &[u8]) -> Self {
        self.icon_data = Some(icon.to_vec());
        self
    }

    /// 新しいウィンドウビルダーを作成する
    pub fn new_window_builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// 新しいトレイを作成する
    pub fn new_tray() -> Tray {
        Tray::new()
    }

    /// 新しい通知を作成する
    pub fn new_notification() -> Notification {
        Notification::new()
    }

    /// アプリケーションを開始する。スレッドを停止しない。
    pub fn start(&mut self) -> instance::app::AppInstance {
        // Qt関連のコードは後回し
        instance::app::AppInstance
    }

    /// アプリケーションを開始する。スレッドはアプリが停止するまでそこで停止する
    pub fn run(&self) {
        // Qt関連のコードは後回し
    }

    /// イベントハンドラを設定する
    pub fn handle_event<F>(&mut self, handler: F)
    where
        F: Fn(Event) + Send + 'static,
    {
        self.event_handler = Box::new(handler);
    }

    // 内部イベントディスパッチャ
    fn _dispatch_event(&self, event: Event) {
        let handler = &self.event_handler;
        handler(event);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // Qt関連のクリーンアップは後回し
    }
}
