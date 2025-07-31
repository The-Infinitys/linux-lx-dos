// src/app/messages.rs

/// システムトレイからメインアプリケーションへ送るメッセージ
///
/// `Clone` と `Copy` を派生させることで、メッセージを値として簡単に渡せるようにします。
#[derive(Debug, Clone, Copy)]
pub enum TrayMessage {
    /// ウィンドウを開く、または既存のウィンドウを前面に表示するメッセージ
    OpenWindow,
    /// アプリケーションを終了するメッセージ
    QuitApp,
}
