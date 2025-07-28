use std::path::Path;

pub mod event;
pub mod image;
pub mod tray;
pub mod window;
pub mod instance;


#[derive(Debug)]
pub struct App {
    id: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: "com.example.app".to_string(),
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

    /// ファイルを追加する
    pub fn add_file(&mut self, _path: &Path, _content: &[u8]) {
        // TODO: Implement add_file
    }

    /// アイコンを設定する
    pub fn set_icon(&mut self, _path: &Path) {
        // TODO: Implement with_icon
    }

    /// アイコンを設定する
    pub fn with_icon(&mut self, _icon: &[u8]) {
        // TODO: Implement with_icon
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

