#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bind {
    include!(concat!(env!("OUT_DIR"), "/qt6-bind.rs"));
}
#[derive(Debug, thiserror::Error)]
pub enum Qt6Error {}
#[derive(Debug)]
pub struct QtApp {
    // TODO: QtApplication構造体を作成する
}
impl Default for QtApp {
    fn default() -> Self {
        Self {}
    }
}
impl QtApp {
    pub fn new() -> Self {}
    pub fn with_id(self, id: &str) -> Self {}
    pub fn run(&self) -> Result<(), Qt6Error> {
        // TODO: 起動プロセスを作成する(処理をブロック)
        Ok(())
    }
    pub fn start(&self) -> Result<QtAppInstance, Qt6Error> {
        // TODO: 開始プロセスを作成する(処理をブロックせず、後からイベントなどを受け取れるようにする)
    }
}
#[derive(Debug)]
pub struct QtAppInstance {
    // TODO: QtAppに処理を送信したりするための機能を作成したい
}

#[derive(Debug)]
pub enum QtAppEvent{
    // TODO: QtAppのイベントを作成する
}
pub struct QtAppEventListener{
    // TODO: イベントリスナを作成する。イベントに応じて処理をできるようにしてほしいです。
}
pub struct QtAppTray{
    // TODO: トレイアイコンを表示することをできるようにしてほしいです。
    // イベント等も実装してください。また、メニューも設定してほしいです
}