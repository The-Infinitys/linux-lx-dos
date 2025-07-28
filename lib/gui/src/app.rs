use std::path::Path;

pub mod event;
pub mod image;
pub mod tray;
pub mod window;
pub mod instance;


#[derive(Debug)]
pub struct App {
    id: String,
    icon_data: Option<Vec<u8>>,
    tray: Option<crate::app::tray::Tray>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: "com.example.app".to_string(),
            icon_data: None,
            tray: None,
        }
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
    pub fn with_tray(mut self, tray: crate::app::tray::Tray) -> Self {
        self.tray = Some(tray);
        self
    }

    /// ファイルを追加する
    pub fn add_file(&mut self, _path: &Path, _content: &[u8]) {
        // TODO: Implement add_file
    }

    /// アイコンを設定する
    pub fn set_icon(&mut self, path: &Path) {
        // TODO: Implement reading icon from path and setting icon_data
        // For now, just store the path (or load it if feasible)
    }

    /// アイコンを設定する
    pub fn with_icon(&mut self, icon: &[u8]) {
        self.icon_data = Some(icon.to_vec());
    }

    /// アプリケーションを開始する。スレッドを停止しない。
    pub fn start(&self) -> instance::app::AppInstance {
        todo!()
    }

    /// アプリケーションを開始する。スレッドはアプリが停止するまでそこで停止する
    pub fn run(&self) {
        todo!()
    }
}

